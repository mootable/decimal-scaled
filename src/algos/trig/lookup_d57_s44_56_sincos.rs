//! Bespoke `sin_strict` + `cos_strict` kernel slot for `D57<SCALE>`
//! with `SCALE ∈ 44..=56`.
//!
//! At deep storage scales the wide-tier `sin_cos_fixed` evaluates a
//! Taylor series on a reduced argument `r ∈ [0, π/4]`. The series
//! converges quadratically in the *argument squared*, but at `w = SCALE
//! + GUARD = 74..=87` the wide-int `Int<16>` `mul`/`div` cost per term
//! dominates: ~30 rounded muls per call, each running the full Knuth
//! divide. The strict `cos_strict` path additionally pays one wide
//! `sqrt_fixed` (for the Pythagorean identity that recovers cos).
//!
//! This kernel collapses both `sin` and `cos` onto a single table-driven
//! reduction:
//!
//! ```text
//! x = k·(π/2) + r,    k = round(x · 2/π),    |r| ≤ π/4
//! r = c_j + δ,        c_j = j·π/(4M),        |δ| ≤ π/(8M)
//!
//! sin(r) = sin(c_j)·cos(δ) + cos(c_j)·sin(δ)
//! cos(r) = cos(c_j)·cos(δ) − sin(c_j)·sin(δ)
//! ```
//!
//! Then quadrant `k mod 4` permutes (sin, cos) of the residual `r`
//! into (sin, cos) of `x`:
//!
//! | k mod 4 | sin(x)  | cos(x)  |
//! | ------- | ------- | ------- |
//! |    0    |  sin(r) |  cos(r) |
//! |    1    |  cos(r) | −sin(r) |
//! |    2    | −sin(r) | −cos(r) |
//! |    3    | −cos(r) |  sin(r) |
//!
//! With `M = 512` the residual `|δ| ≤ π/4096 ≈ 7.7·10⁻⁴`, so the
//! `sin(δ)` / `cos(δ)` Taylor series converge in ~6-8 terms each (vs
//! the ~30 the generic path runs on `r ∈ [0, π/4]`). The table stores
//! both `sin(c_j)` and `cos(c_j)` per non-negative `j` — sin and cos
//! share one reduction, so one table buys both.
//!
//! The slot is exposed through [`crate::policy::trig::TrigPolicy`] only
//! for `SCALE ∈ 44..=56`; lower scales keep using the generic
//! [`super::wide_kernel::sin_strict_d57`] / `cos_strict_d57` which are
//! already cheap there (smaller `w`, fewer Taylor terms, faster Knuth
//! dispatch).
//!
//! ## Correctness
//!
//! Error budget at working scale `w` (in LSB-of-`w`):
//!
//! - Stage 1 reduction `x − k·(π/2)`: 1 mul + 1 div + 1 sub → ≤ 2 LSB.
//! - Stage 2 sub-reduction `r − c_j`: 1 mul + 1 div for j, 1 mul for
//!   `c_j`, 1 sub → ≤ 2 LSB.
//! - Two small-residual Taylor series (sin δ + cos δ): ~16 rounded
//!   muls combined → ≤ 8 LSB.
//! - Four `mul_cached`s for the addition-formula reassembly: ≤ 2 LSB.
//! - One outer add + sign flip: ≤ 0.5 LSB.
//! - Table lookups (`sin(c_j)`, `cos(c_j)`): precomputed via
//!   `sin_cos_fixed` at the same `w`, ≤ 1 LSB each after rounding.
//!
//! Total ≤ ~15 LSB-of-`w` = ~15·10⁻³⁰ at storage scale. The strict
//! contract requires ≤ 0.5 LSB-of-storage = 0.5·10⁻ᴿᴱ — a margin of
//! 28+ orders of magnitude even at `SCALE = 57`.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::int::types::Int;

/// Table size — number of `(sin(c_j), cos(c_j))` entries per working
/// scale, with `c_j = j · π / (4M)` and `j ∈ [0, M]`. Power of two so
/// the index quantisation step `π/(4M)` keeps the cheap integer-
/// division path. Larger M shrinks the post-table residual `|δ| ≤
/// π/(8M)` and so shaves Taylor iterations.
///
/// Mirrors the tuning from the D57 atan + exp lookups (see
/// [`crate::algos::trig::lookup_d57_s44_56_atan::M`] and
/// [`crate::algos::exp::lookup_d57_s45_56::M`]): same `Int<16>`-wide
/// work integer, same Knuth-dispatch arithmetic cost per slot, same
/// per-thread memoisation pattern. `M = 512` strikes the same balance
/// here — the post-table Taylor remainders are small enough that each
/// of sin/cos converges in ~6-8 iterations, against a one-off cold-
/// start table seed of `(M+1) · sin_cos_fixed(w)` calls (~25 ms at
/// `SCALE = 57`).
///
/// Per-thread memory cost: `2·(M+1)·sizeof(W) = (M+1) · 256 B` for the
/// `Int<16>` wide-tier trig core, so ~128 KB at M = 512.
const M: u32 = 512;

/// One entry of the per-thread table: `(sin(c_j), cos(c_j))` at the
/// table's working scale.
type Entry = (core::W, core::W);

crate::policy::table_cache::decl_table_cache!(entry = Entry, compute = compute_table);

/// Build the `(sin(c_j), cos(c_j))` table at working scale `w` using
/// the canonical `sin_cos_fixed` kernel (one call per slot, paid once
/// per thread per `w`). `j ∈ [0, M]`; the j=M slot stores
/// `(sin(π/4), cos(π/4))` and is needed because rounding can round a
/// residual up to exactly the table boundary.
fn compute_table(w: u32) -> alloc::vec::Vec<Entry> {
    let mut out = alloc::vec::Vec::with_capacity((M + 1) as usize);
    let pi_w = core::pi(w);
    let step_denom = core::lit((4 * M) as u128);
    // j = 0: sin(0) = 0, cos(0) = 1.
    out.push((core::zero(), core::one(w)));
    for j in 1..=M {
        // c_j = j · π / (4M), computed at working scale.
        let cj_w = (pi_w * core::lit(j as u128)) / step_denom;
        out.push(core::sin_cos_fixed(cj_w, w));
    }
    out
}

/// Sin/cos selector — which component the caller wants out of the
/// shared kernel. Both sin and cos share every stage of the
/// reduction; the selector only picks which of the two final
/// quadrant-permuted values to return.
#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

/// Shared `sin_strict` / `cos_strict` kernel for `D57<SCALE>` with
/// `SCALE ∈ 44..=56`.
///
/// Stages:
/// 1. Reduce `x = k·(π/2) + r` with `|r| ≤ π/4` via
///    `k = round(x · 2/π)`.
/// 2. Sub-reduce `r = c_j + δ` with `c_j = j·π/(4M)`, `|δ| ≤ π/(8M)`,
///    via `j = round(r · 4M/π)`. `j ∈ [-M, M]`; the table is keyed
///    on `|j|` using `sin(-c) = -sin(c)`, `cos(-c) = cos(c)`.
/// 3. Evaluate `sin(δ)` and `cos(δ)` by short Taylor series (~6-8
///    terms each at `M = 512`, w ≤ 87).
/// 4. Reconstruct `sin(r) = sin(c_j)·cos(δ) + cos(c_j)·sin(δ)` and
///    `cos(r) = cos(c_j)·cos(δ) − sin(c_j)·sin(δ)`.
/// 5. Apply the quadrant permutation `k mod 4` to map (sin(r),
///    cos(r)) to (sin(x), cos(x)) and return the requested component.
#[inline]
#[must_use]
pub(crate) fn sin_cos_strict<const SCALE: u32>(
    raw: Int<3>,
    mode: RoundingMode,
    which: Which,
) -> Int<3> {
    // sin(0) = 0, cos(0) = 1 short-circuits — match `wide_kernel`.
    if raw == Int::<3>::ZERO {
        return match which {
            Which::Sin => Int::<3>::ZERO,
            // D57::<SCALE>::ONE raw is 10^SCALE in storage units.
            Which::Cos => {
                let ten: Int<3> = Int::<3>::from_u128(10);
                ten.pow(SCALE)
            }
        };
    }

    let w = SCALE + core::GUARD;
    let v_w = core::to_work(raw);
    let one_w = core::one(w);
    let pow10_w = one_w;
    let pi_w = core::pi(w);
    let half_pi_w = core::half_pi(w);

    // Stage 1: x = k·(π/2) + r, |r| ≤ π/4 + small rounding slack.
    // k = round(x / (π/2)).
    let k = core::round_to_nearest_int(core::div_cached(v_w, half_pi_w, pow10_w), w);
    // k_signed · (π/2) at working scale.
    let k_half_pi = if k >= 0 {
        half_pi_w * core::lit(k as u128)
    } else {
        -(half_pi_w * core::lit((-k) as u128))
    };
    let r = v_w - k_half_pi;

    // Stage 2: r = c_j_signed · π/(4M) + δ, |δ| ≤ π/(8M).
    // j_signed = round(r · 4M / π).
    let four_m = core::lit((4 * M) as u128);
    let j_signed = core::round_to_nearest_int(core::div_cached(r * four_m, pi_w, pow10_w), w);
    // c_j_signed at working scale.
    let cj_signed_w = if j_signed >= 0 {
        (pi_w * core::lit(j_signed as u128)) / four_m
    } else {
        -((pi_w * core::lit((-j_signed) as u128)) / four_m)
    };
    let delta = r - cj_signed_w;

    // Table lookup: stored entries are for |j_signed|. Apply
    // sin(-c) = -sin(c), cos(-c) = cos(c) on the way out.
    let j_abs = j_signed.unsigned_abs() as u32;
    debug_assert!(
        j_abs <= M,
        "sin_cos_strict d57 s44..=56: table index {j_abs} > M={M}"
    );
    let j_idx = if j_abs > M {
        M as usize
    } else {
        j_abs as usize
    };
    let (sin_cj_abs, cos_cj) = table_entry(w, j_idx);
    let sin_cj = if j_signed < 0 {
        -sin_cj_abs
    } else {
        sin_cj_abs
    };

    // Stage 3: small-residual Taylor for sin(δ) and cos(δ).
    //
    // For M = 512, |δ| ≤ π/(8·512) ≈ 7.7·10⁻⁴, so |δ²| ≤ ~6·10⁻⁷.
    // Each pair of sin terms shrinks by |δ|² / ((2k)(2k+1)); the loop
    // exits on a zero term in ~6-8 iterations at w ≤ 87. Same for cos.
    //
    // Inlined here (rather than calling the macro-private
    // `sin_taylor` / `cos_taylor`) so the cached `pow10_w` is reused
    // across all iterations of both series.
    let delta2 = core::mul_cached(delta, delta, pow10_w);

    // sin(δ) = δ − δ³/3! + δ⁵/5! − …
    let sin_delta = {
        let mut sum = delta;
        let mut term = delta;
        let mut k_term: u128 = 1;
        loop {
            term = core::mul_cached(term, delta2, pow10_w)
                / core::lit((2 * k_term) * (2 * k_term + 1));
            if term == core::zero() {
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
            term = core::mul_cached(term, delta2, pow10_w)
                / core::lit((2 * k_term - 1) * (2 * k_term));
            if term == core::zero() {
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
    //   sin(r) = sin(c_j)·cos(δ) + cos(c_j)·sin(δ)
    //   cos(r) = cos(c_j)·cos(δ) − sin(c_j)·sin(δ)
    let sin_r =
        core::mul_cached(sin_cj, cos_delta, pow10_w) + core::mul_cached(cos_cj, sin_delta, pow10_w);
    let cos_r =
        core::mul_cached(cos_cj, cos_delta, pow10_w) - core::mul_cached(sin_cj, sin_delta, pow10_w);

    // Stage 5: quadrant permutation. `k mod 4` selects which signed
    // permutation of (sin(r), cos(r)) becomes (sin(x), cos(x)).
    //
    // Rust's `%` follows the sign of the dividend; normalise to a
    // non-negative residue in [0, 4).
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
    core::round_to_storage_with(result, w, SCALE, mode)
}

/// Thin entry shim — `sin_strict` for `D57<SCALE>` with
/// `SCALE ∈ 44..=56`. See [`sin_cos_strict`] for the algorithm.
#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    sin_cos_strict::<SCALE>(raw, mode, Which::Sin)
}

/// Thin entry shim — `cos_strict` for `D57<SCALE>` with
/// `SCALE ∈ 44..=56`. See [`sin_cos_strict`] for the algorithm.
#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    sin_cos_strict::<SCALE>(raw, mode, Which::Cos)
}
