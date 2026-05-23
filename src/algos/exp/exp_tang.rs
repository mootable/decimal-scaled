// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic Tang-style table-driven `exp_strict` kernel.
//!
//! Tang 1989, "Table-driven implementation of the exponential function
//! in IEEE floating-point arithmetic" (ACM TOMS 16(4)):
//!
//! ```text
//! e^v = 2^k · e^s,            s = v − k·ln 2,           |s| ≤ ln 2 / 2
//!     = 2^k · e^(c_j) · e^δ,  c_j = j · ln 2 / M,       j ∈ [0, M)
//!                              δ  = s − c_j,            |δ| ≤ ln 2 / (2M)
//! ```
//!
//! A two-stage range reduction collapses the post-stage-1 Taylor into a
//! table multiply (`exp(c_j)` from a memoised `M`-entry table) plus a
//! short Taylor on the tiny remainder `δ`. The result is reassembled as
//! `2^(k+k_adj) · table[j] · e^δ`.
//!
//! ## Layering
//!
//! This is an **algorithm function** (`docs/ARCHITECTURE.md` →
//! "Layering direction"): it computes only through the
//! [`WideTrigCore`] trait surface and `BigInt` arithmetic on the work
//! integer; it never calls a method on a decimal type. `policy::exp`
//! calls [`exp_tang`] *down*; the type's `exp_strict` method delegates
//! *down* through the policy. The trig hyperbolic kernels reuse
//! [`tang_exp_fixed`] directly for their shared `(e^v, e^-v)` pair.
//!
//! Collapses the four per-tier `lookup_d57_s18_22_tang`,
//! `lookup_d57_s45_56`, `lookup_d115_s57_tang`, `lookup_d153_s70_82_tang`
//! kernels into one generic over `C: WideTrigCore`, the table size `M`,
//! and the per-band reduction/narrowing flags.

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::support::rounding::RoundingMode;

/// Tang-style `e^v_w` on an already-lifted working value `v_w` (`= x ·
/// 10^w`), returned at working scale `w`. Generic over the tier `C` and
/// the table size `M`.
///
/// `INTERNAL_EXTRA` selects the large-`|k|` mitigation. The final `2^k`
/// reassembly amplifies the reduction residual by `2^k ≈ 10^(|k|·log10
/// 2)` decimal digits, so a fixed narrow guard cannot cover an unbounded
/// `|k|`. When `true` the whole reduction runs at an extended working
/// scale `w + extra` (`extra = ceil(|k|·log10 2) + 12`) and the result
/// is narrowed back to `w` round-to-nearest; when `false` the body runs
/// at the caller-supplied `w` (the caller absorbs the `extra` lift in
/// its own guard, or the band's `|k|` is small enough not to need it).
/// This is the shared surface the trig hyperbolic kernels reuse.
#[must_use]
pub(crate) fn tang_exp_fixed<C: WideTrigCore, const M: u32, const INTERNAL_EXTRA: bool>(
    v_w: C::W,
    w: u32,
) -> C::W {
    // Stage 0 (INTERNAL_EXTRA only): size an extended working scale
    // `w_ext = w + extra` from `|k|` so the `2^k` reassembly does not
    // amplify the reduction residual past the storage LSB. Matches the
    // dynamic-margin reduction the generic `exp_fixed` uses (Muller,
    // *Elementary Functions* 3rd ed., §11.1). `k` is scale-invariant, so
    // reuse the value computed at `w` below.
    let k = {
        let one_w = C::one(w);
        C::round_to_nearest_int(C::div_cached(v_w, C::ln2(w), one_w), w)
    };

    let (w_ext, v_ext, extra) = if INTERNAL_EXTRA {
        let abs_k = if k < 0 { -k } else { k } as u128;
        let extra: u32 = if abs_k == 0 {
            0
        } else {
            let digits = ((abs_k * 30103 + 99_999) / 100_000) as u32;
            digits + 12
        };
        let v_ext = if extra == 0 {
            v_w
        } else {
            v_w * C::pow10(extra)
        };
        (w + extra, v_ext, extra)
    } else {
        (w, v_w, 0)
    };

    let one_w = C::one(w_ext);
    let pow10_w = one_w;
    let l2 = C::ln2(w_ext);

    // Stage 1: v = k·ln 2 + s, |s| ≤ ln 2 / 2.
    let k_l2 = if k >= 0 {
        l2 * C::lit(k as u128)
    } else {
        -(l2 * C::lit((-k) as u128))
    };
    let s = v_ext - k_l2;

    // Stage 2: s = j_signed · (ln 2 / M) + δ, |δ| ≤ ln 2 / (2M).
    let j_signed = C::round_to_nearest_int(C::div_cached(s * C::lit(M as u128), l2, pow10_w), w_ext);
    let cj_signed_w = if j_signed >= 0 {
        (l2 * C::lit(j_signed as u128)) / C::lit(M as u128)
    } else {
        -((l2 * C::lit((-j_signed) as u128)) / C::lit(M as u128))
    };
    let delta = s - cj_signed_w;
    let (j_idx, k_adj) = if j_signed >= 0 {
        (j_signed as u32, 0i128)
    } else {
        ((j_signed + M as i128) as u32, -1i128)
    };
    debug_assert!(j_idx < M, "tang_exp_fixed: table index out of range");

    // Taylor on δ.
    let mut sum = one_w + delta;
    let mut term = delta;
    let mut n: u128 = 2;
    loop {
        term = C::mul_cached(term, delta, pow10_w) / C::lit(n);
        if term == C::zero() {
            break;
        }
        sum = sum + term;
        n += 1;
        if n > 200 {
            break;
        }
    }

    let exp_cj = C::exp_table_entry(w_ext, j_idx as usize, M);
    let exp_s = C::mul_cached(exp_cj, sum, pow10_w);

    let k_total = k + k_adj;
    let scaled_at_w_ext = if k_total >= 0 {
        let shift = k_total as u32;
        debug_assert!(
            C::bit_length(exp_s) + shift < C::w_bits(),
            "tang_exp_fixed: result overflows the representable range",
        );
        exp_s << shift
    } else {
        let neg_k = (-k_total) as u32;
        if neg_k as u128 >= C::bit_length(exp_s) as u128 {
            C::zero()
        } else {
            exp_s >> neg_k
        }
    };

    if !INTERNAL_EXTRA || extra == 0 {
        scaled_at_w_ext
    } else {
        // Narrow the extended-scale result back to `w` round-to-nearest
        // (ties up via the `+ half` bias). `extra` is bounded so
        // `10^extra` stays well inside the working width.
        let p = C::pow10(extra);
        let half = p / C::lit(2);
        if scaled_at_w_ext >= C::zero() {
            (scaled_at_w_ext + half) / p
        } else {
            -((-scaled_at_w_ext + half) / p)
        }
    }
}

/// Tier-generic Tang-style `e^x` strict kernel.
///
/// - `C` — the per-tier [`WideTrigCore`] marker (`wide_trig_d*::Core`).
/// - `SCALE` — the decimal storage scale.
/// - `M` — the Tang table size (`128` or `512`).
/// - `GUARD` — the narrow guard for this band (`8`, `10`, or the tier's
///   canonical `30`).
/// - `DIRECTED` — route the final narrowing through the directed-rounding
///   Ziv escalation (`true`), else narrow once with
///   `round_to_storage_with` (`false`).
/// - `EXTERNAL_EXTRA` — compute the large-`|k|` working-scale lift `extra`
///   in this wrapper and fold it into the directed base guard (the D115
///   shape; requires `DIRECTED`).
/// - `INTERNAL_EXTRA` — let [`tang_exp_fixed`] do the `extra` lift +
///   narrow-back internally (the D153 shape).
#[inline]
#[must_use]
pub(crate) fn exp_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const M: u32,
    const GUARD: u32,
    const DIRECTED: bool,
    const EXTERNAL_EXTRA: bool,
    const INTERNAL_EXTRA: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }

    if !DIRECTED {
        // Single-shot narrowing (D57 18..=22 and 45..=56). Reduction runs
        // at the const-folded `w = SCALE + GUARD`.
        let w = SCALE + GUARD;
        let v_w = C::to_work_w(raw, GUARD);
        let result = tang_exp_fixed::<C, M, INTERNAL_EXTRA>(v_w, w);
        return C::round_to_storage_with(result, w, SCALE, mode);
    }

    let base_guard = if EXTERNAL_EXTRA {
        // The final `2^k` reassembly amplifies the working-scale rounding
        // error by `2^k` (≈ `|k|·log10 2` digits). Widen the base guard by
        // `extra` so the post-shift residual lands back inside the guard.
        let w = SCALE + GUARD;
        let one_w = C::one(w);
        let v_w_probe = C::to_work_w(raw, GUARD);
        let k = C::round_to_nearest_int(C::div_cached(v_w_probe, C::ln2(w), one_w), w);
        let abs_k = if k < 0 { -k } else { k } as u128;
        let extra: u32 = if abs_k == 0 {
            0
        } else {
            let digits = (abs_k * 30103 + 99_999) / 100_000;
            let capped = digits.min((C::w_bits() / 4) as u128) as u32;
            capped + 12 + (capped >> 2)
        };
        GUARD + extra
    } else {
        GUARD
    };

    // Directed modes decide which side of a storage grid line the true
    // result falls; near a grid line the working-scale approximation can
    // land on the wrong side, so route through the shared Ziv escalation.
    // Nearest modes narrow once.
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        tang_exp_fixed::<C, M, INTERNAL_EXTRA>(C::to_work_w(raw, guard), SCALE + guard)
    })
}
