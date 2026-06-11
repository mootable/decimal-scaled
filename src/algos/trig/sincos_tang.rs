// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic Tang-style table-driven `sin_strict` / `cos_strict`
//! kernel (the deep-`SCALE` sincos band).
//!
//! Both sin and cos share one reduction; the requested component is
//! selected at the end. Tang 1991 two-stage argument reduction with a
//! `(sin(c_j), cos(c_j))` table plus a short addition-formula reassembly:
//!
//! ```text
//! x = k·(π/2) + r,    k = round(x · 2/π),    |r| ≤ π/4
//! r = c_j + δ,        c_j = j·π/(4M),        |δ| ≤ π/(8M)
//!
//! sin(r) = sin(c_j)·cos(δ) + cos(c_j)·sin(δ)
//! cos(r) = cos(c_j)·cos(δ) − sin(c_j)·sin(δ)
//! ```
//!
//! Then the quadrant `k mod 4` permutes `(sin(r), cos(r))` into
//! `(sin(x), cos(x))`:
//!
//! | k mod 4 | sin(x)  | cos(x)  |
//! | ------- | ------- | ------- |
//! |    0    |  sin(r) |  cos(r) |
//! |    1    |  cos(r) | −sin(r) |
//! |    2    | −sin(r) | −cos(r) |
//! |    3    | −cos(r) |  sin(r) |
//!
//! With `M = 512` the residual `|δ| ≤ π/4096 ≈ 7.7·10⁻⁴`, so the
//! `sin(δ)` / `cos(δ)` Taylor series converge in ~6-8 terms each. The
//! `(sin(c_j), cos(c_j))` entries are baked binary fixed-point consts
//! (`sincos_tang_table`); `C::sincos_table_entry` converts the one
//! indexed slot to the working scale per lookup — no runtime table.
//!
//! ## Layering
//!
//! An **algorithm function** (`docs/ARCHITECTURE.md` → "Layering
//! direction"): it computes only through the [`WideTrigCore`] trait
//! surface; it never calls a method on a decimal type. `policy::trig`
//! (the forward family) calls the `*_tang_with_taylor` wrappers *down*.
//!
//! Collapses the per-tier D57 44..=56 sincos Tang
//! kernel into one generic over `C: WideTrigCore`, the `SCALE`, and the
//! table size `M`.

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::support::rounding::RoundingMode;

/// Which component the caller wants out of the shared kernel. Both sin
/// and cos share every stage of the reduction; the selector only picks
/// which of the two final quadrant-permuted values to return.
#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

/// Shared Tang `sin_strict` / `cos_strict` kernel for a wide tier —
/// generic over `C`, the `SCALE`, and the table size `M`.
#[inline]
#[must_use]
fn sin_cos_strict<C: WideTrigCore, const SCALE: u32, const M: u32>(
    raw: C::Storage,
    mode: RoundingMode,
    which: Which,
) -> C::Storage
where
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    // sin(0) = 0, cos(0) = 1 short-circuits — matches `wide_kernel`.
    if raw == C::storage_zero() {
        return match which {
            Which::Sin => C::storage_zero(),
            Which::Cos => C::storage_one(SCALE),
        };
    }

    let w = SCALE + C::GUARD;
    let v_w = C::to_work(raw);
    let one_w = C::one(w);
    let pow10_w = one_w;
    let pi_w = C::pi::<SCALE>(w);
    let half_pi_w = C::half_pi::<SCALE>(w);

    // Stage 1: x = k·(π/2) + r, |r| ≤ π/4 + small rounding slack.
    let k = C::round_to_nearest_int(C::div_cached(v_w, half_pi_w, pow10_w), w);
    let k_half_pi = if k >= 0 {
        half_pi_w * C::lit(k as u128)
    } else {
        -(half_pi_w * C::lit((-k) as u128))
    };
    let r = v_w - k_half_pi;

    // Stage 2: r = c_j_signed · π/(4M) + δ, |δ| ≤ π/(8M).
    let four_m = C::lit((4 * M) as u128);
    let j_signed = C::round_to_nearest_int(C::div_cached(r * four_m, pi_w, pow10_w), w);
    let cj_signed_w = if j_signed >= 0 {
        (pi_w * C::lit(j_signed as u128)) / four_m
    } else {
        -((pi_w * C::lit((-j_signed) as u128)) / four_m)
    };
    let delta = r - cj_signed_w;

    // Table lookup: stored entries are for |j_signed|. Apply
    // sin(-c) = -sin(c), cos(-c) = cos(c) on the way out.
    let j_abs = j_signed.unsigned_abs() as u32;
    debug_assert!(j_abs <= M, "sin_cos_strict tang: table index {j_abs} > M={M}");
    let j_idx = if j_abs > M { M as usize } else { j_abs as usize };
    let (sin_cj_abs, cos_cj) = C::sincos_table_entry::<SCALE>(w, j_idx, M);
    let sin_cj = if j_signed < 0 { -sin_cj_abs } else { sin_cj_abs };

    // Stage 3: small-residual Taylor for sin(δ) and cos(δ).
    let delta2 = C::mul(delta, delta, w);

    // sin(δ) = δ − δ³/3! + δ⁵/5! − …
    let sin_delta = {
        let mut sum = delta;
        let mut term = delta;
        let mut k_term: u128 = 1;
        loop {
            term = C::mul(term, delta2, w) / C::lit((2 * k_term) * (2 * k_term + 1));
            if term == C::zero() {
                break;
            }
            if k_term % 2 == 1 {
                sum = sum - term;
            } else {
                sum = sum + term;
            }
            k_term += 1;
            if k_term > 200 {
                break;
            }
        }
        sum
    };

    // cos(δ) = 1 − δ²/2! + δ⁴/4! − δ⁶/6! + …
    let cos_delta = {
        let mut sum = one_w;
        let mut term = one_w;
        let mut k_term: u128 = 1;
        loop {
            term = C::mul(term, delta2, w) / C::lit((2 * k_term - 1) * (2 * k_term));
            if term == C::zero() {
                break;
            }
            if k_term % 2 == 1 {
                sum = sum - term;
            } else {
                sum = sum + term;
            }
            k_term += 1;
            if k_term > 200 {
                break;
            }
        }
        sum
    };

    // Stage 4: addition formula to lift (sin δ, cos δ) onto r.
    let sin_r = C::mul(sin_cj, cos_delta, w) + C::mul(cos_cj, sin_delta, w);
    let cos_r = C::mul(cos_cj, cos_delta, w) - C::mul(sin_cj, sin_delta, w);

    // Stage 5: quadrant permutation. `k mod 4` selects the signed
    // permutation of (sin(r), cos(r)) that becomes (sin(x), cos(x)).
    let quadrant = ((k % 4) + 4) % 4;
    let (sin_x, cos_x) = match quadrant {
        0 => (sin_r, cos_r),
        1 => (cos_r, -sin_r),
        2 => (-sin_r, -cos_r),
        3 => (-cos_r, sin_r),
        _ => unreachable!(),
    };

    let result = match which {
        Which::Sin => sin_x,
        Which::Cos => cos_x,
    };
    // Near-tie escape — see `wide_trig_core::tan_series` / the asin(3e-60)
    // family: a fixed-w single shot cannot see a deciding digit below w.
    // Clear-of-band residuals keep the single-shot cost; the band falls to
    // the Ziv-escalating generic kernel (rare).
    match crate::algos::support::wide_trig_core::round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
        result, w, SCALE, mode, C::storage_max(), C::storage_min(),
    ) {
        Some(st) => st,
        None => match which {
            Which::Sin => crate::algos::support::wide_trig_core::sin_series::<C, SCALE>(raw, mode),
            Which::Cos => crate::algos::support::wide_trig_core::cos_series::<C, SCALE>(raw, mode),
        },
    }
}

/// Tang `sin_strict` for a wide tier — generic over `C`, `SCALE`, `M`.
#[inline]
#[must_use]
pub(crate) fn sin_tang_with_taylor<C: WideTrigCore, const SCALE: u32, const M: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    sin_cos_strict::<C, SCALE, M>(raw, mode, Which::Sin)
}

/// Tang `cos_strict` for a wide tier — generic over `C`, `SCALE`, `M`.
#[inline]
#[must_use]
pub(crate) fn cos_tang_with_taylor<C: WideTrigCore, const SCALE: u32, const M: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    sin_cos_strict::<C, SCALE, M>(raw, mode, Which::Cos)
}
