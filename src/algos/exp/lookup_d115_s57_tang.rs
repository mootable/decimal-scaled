//! Tang-style table-driven `exp_strict` kernel for `D115<SCALE>` with
//! `SCALE = 57` (mid-storage, popular precision tier).
//!
//! Direct port of [`crate::algos::exp::lookup_d57_s18_22_tang`] adapted
//! for the `D115` storage tier (`Int384` storage, `Int2048` work
//! integer). See Tang 1989 (ACM TOMS 16(4)) "Table-driven
//! implementation of the exponential function in IEEE floating-point
//! arithmetic" for the underlying technique:
//!
//! ```text
//! e^v = 2^k · e^s,            s = v − k·ln 2,           |s| ≤ ln 2 / 2
//!     = 2^k · e^(c_j) · e^δ,  c_j = j · ln 2 / M,       j ∈ [0, M)
//!                              δ  = s − c_j,            |δ| ≤ ln 2 / (2M)
//! ```
//!
//! Per-call body collapses the post-stage-1 Taylor on `|s| ≤ ln(2)/2`
//! (which the macro-emitted `exp_fixed` runs Smith-style halving +
//! Taylor on) into a short Taylor on `|δ| ≤ ln(2)/(2M)` after a table
//! multiply.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 8` mirrors the D115 ln Tang slot. Error budget at
//!   `w = 65`: table multiply + table mul-back + Taylor on δ
//!   accumulate ≤ ~15 LSB-of-w `≈ 1.5·10⁻⁶⁴` in working units; storage
//!   half-ULP at SCALE=57 is `0.5·10⁻⁵⁷`, so the drift sits ~6 orders
//!   below half-ULP.
//! - `M = 128` matches the D57 sibling: per-thread memory cost
//!   `M · sizeof(W) = 128 · 256 B = ~32 KB` (W = Int2048). Fits L2.

#![cfg(any(feature = "d115", feature = "wide"))]

use crate::types::widths::wide_trig_d115 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int384;

/// Narrow guard for the Tang-style exp slot at SCALE = 57.
pub(crate) const GUARD_NARROW: u32 = 8;

/// Table size. Power of two so the index quantisation step `ln(2)/M`
/// keeps the cheap integer-division path.
const M: u32 = 128;

crate::policy::table_cache::decl_table_cache!(entry = core::W, compute = compute_table);

fn compute_table(w: u32) -> alloc::vec::Vec<core::W> {
    let mut out = alloc::vec::Vec::with_capacity(M as usize);
    let l2 = core::ln2(w);
    out.push(core::one(w)); // j = 0: exp(0) = 1.
    for j in 1..M {
        let cj_w = (l2 * core::lit(j as u128)) / core::lit(M as u128);
        out.push(core::exp_fixed(cj_w, w));
    }
    out
}

/// Tang-style `e^v_w` kernel on an already-lifted working value.
/// Shared so hyperbolic kernels can reuse a single lift.
#[must_use]
pub(crate) fn tang_exp_fixed(v_w: core::W, w: u32) -> core::W {
    let one_w = core::one(w);
    let pow10_w = one_w;
    let l2 = core::ln2(w);

    // Stage 1: v = k·ln 2 + s, |s| ≤ ln 2 / 2.
    let k = core::round_to_nearest_int(core::div_cached(v_w, l2, pow10_w), w);
    let k_l2 = if k >= 0 {
        l2 * core::lit(k as u128)
    } else {
        -(l2 * core::lit((-k) as u128))
    };
    let s = v_w - k_l2;

    // Stage 2: s = j_signed · (ln 2 / M) + δ, |δ| ≤ ln 2 / (2M).
    let j_signed = core::round_to_nearest_int(
        core::div_cached(s * core::lit(M as u128), l2, pow10_w),
        w,
    );
    let cj_signed_w = if j_signed >= 0 {
        (l2 * core::lit(j_signed as u128)) / core::lit(M as u128)
    } else {
        -((l2 * core::lit((-j_signed) as u128)) / core::lit(M as u128))
    };
    let delta = s - cj_signed_w;
    let (j_idx, k_adj) = if j_signed >= 0 {
        (j_signed as u32, 0i128)
    } else {
        ((j_signed + M as i128) as u32, -1i128)
    };
    debug_assert!(j_idx < M, "tang_exp_fixed d115 s57: table index out of range");

    // Taylor on δ. |δ| ≤ ln(2)/256 ≈ 2.7·10⁻³, ~7-8 iterations at w=65.
    let mut sum = one_w + delta;
    let mut term = delta;
    let mut n: u128 = 2;
    loop {
        term = core::mul_cached(term, delta, pow10_w) / core::lit(n);
        if term == core::zero() {
            break;
        }
        sum = sum + term;
        n += 1;
        if n > 200 {
            break;
        }
    }

    let exp_cj = table_entry(w, j_idx as usize);
    let exp_s = core::mul_cached(exp_cj, sum, pow10_w);

    let k_total = k + k_adj;
    if k_total >= 0 {
        let shift = k_total as u32;
        debug_assert!(
            core::bit_length(exp_s) + shift < core::W::BITS,
            "tang_exp_fixed d115 s57: result overflows the representable range",
        );
        exp_s << shift
    } else {
        let neg_k = (-k_total) as u32;
        if neg_k as u128 >= core::bit_length(exp_s) as u128 {
            core::zero()
        } else {
            exp_s >> neg_k
        }
    }
}

/// Tang-style `e^x` strict kernel for `D115<SCALE>` with `SCALE = 57`.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int384, mode: RoundingMode) -> Int384 {
    if raw == Int384::ZERO {
        let ten: Int384 = crate::wide_int::wide_cast::<u128, Int384>(10);
        return ten.pow(SCALE);
    }
    let w = SCALE + GUARD_NARROW;

    // Range-reduction error budget. `tang_exp_fixed` reassembles the
    // result as `exp_s · 2^k`, with `k ≈ v / ln 2`. That final shift is
    // exact, but it amplifies the working-scale rounding error of
    // `exp_s` by `2^k` — i.e. by `|k|·log10(2)` decimal digits. With the
    // fixed `GUARD_NARROW = 8` slot that amplification swamps the storage
    // half-ULP for large `|x|` (e.g. `x ≈ 116.8` gives `k ≈ 168`, so the
    // result needs ~50 integer digits and the 8-digit guard leaves the
    // fractional tail tens of orders of magnitude short).
    //
    // Widen the working scale by `extra ≈ ceil(|k|·log10(2)) + margin`
    // so the post-shift residual lands back inside the guard, matching
    // the adaptive lift the generic `core::exp_fixed` uses (see
    // `wide_transcendental.rs`; Muller, *Elementary Functions* 3rd ed.,
    // §11.1).
    let l2_w = core::ln2(w);
    let one_w = core::one(w);
    let v_w_probe = core::to_work_w(raw, GUARD_NARROW);
    let k = core::round_to_nearest_int(core::div_cached(v_w_probe, l2_w, one_w), w);
    let abs_k = k.unsigned_abs();
    let extra: u32 = if abs_k == 0 {
        0
    } else {
        // |k|·log10(2) = |k| · 30103 / 100000, rounded up; margin for
        // Taylor accumulation, the table multiply, and final narrowing.
        let digits = (abs_k * 30103 + 99_999) / 100_000;
        let capped = digits.min((core::W::BITS / 4) as u128) as u32;
        capped + 12 + (capped >> 2)
    };

    // Directed modes (Trunc/Floor/Ceiling) decide which side of a storage
    // grid line the *true* result falls. Near a grid line the working-scale
    // approximation can sit on the wrong side, so route the narrowing
    // through the shared Ziv escalation, which recomputes at a wider guard
    // until the directed answer stabilises. The `+ extra` lift for the
    // large-|x| range-reduction error budget is folded into every guard.
    core::round_to_storage_directed(GUARD_NARROW + extra, SCALE, mode, |guard| {
        tang_exp_fixed(core::to_work_w(raw, guard), SCALE + guard)
    })
}
