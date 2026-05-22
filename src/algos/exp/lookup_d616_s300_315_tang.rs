//! Tang-style table-driven `exp_strict` kernel for `D616<SCALE>` with
//! `SCALE ∈ 300..=315` — the mid-storage popular band centred on
//! `SCALE = 308`.
//!
//! Sibling to the D153 mid-storage Tang exp at
//! [`crate::algos::exp::lookup_d153_s70_82_tang`]. See Tang 1989,
//! "Table-driven implementation of the exponential function in IEEE
//! floating-point arithmetic" (ACM TOMS 16(4)).
//!
//! ```text
//! e^v = 2^k · e^s,            s = v − k·ln 2,           |s| ≤ ln 2 / 2
//!     = 2^k · e^(c_j) · e^δ,  c_j = j · ln 2 / M,       j ∈ [0, M)
//!                              δ  = s − c_j,            |δ| ≤ ln 2 / (2M)
//! ```
//!
//! With `M = 128` the residual `|δ| ≤ ln(2)/256 ≈ 2.7·10⁻³`, so the
//! Taylor on δ converges in `~p / log₁₀(1/|δ|) ≈ 318/2.57 ≈ 124` terms
//! at `w = SCALE + 10 ≤ 325`. The wide-tier macro `exp_fixed` already
//! runs adaptive Smith r/2^n with `n ≈ √p_bits`; the Tang lookup
//! replaces both the `k·ln 2` Stage 1 reduction *and* most of the
//! Smith squaring loop with a single table multiply.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 10` matches the sibling
//!   [`lookup_d616_s300_315_tang`] ln so the per-thread `pow10_w`
//!   cache slot is shared across neighbouring exp/ln/sinh/cosh/tanh
//!   calls.
//! - `M = 128` mirrors the D57 / D115 / D153 mid-storage Tang slots.
//!   Per-thread memory cost: `M · sizeof(W) = 128 · 1024 B ≈ 128 KB`
//!   for D616's Int8192 working integer. Larger than L1d (32 KB
//!   typical) but well inside L2 (256 KB-1 MB on modern x86).

#![cfg(any(feature = "d616", feature = "x-wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d616 as core;
use crate::wide_int::Int2048;

/// Narrow guard for the SCALE 300..=315 Tang-exp slot.
const GUARD_NARROW: u32 = 10;

/// Table size — power of two so the index quantisation step
/// `ln(2) / M` keeps the cheap integer-division path.
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

/// Tang-style `e^v_w` kernel on an already-lifted working value. Used
/// both by [`exp_strict`] and by the hyperbolic kernels in
/// [`crate::algos::trig::lookup_d616_s300_315_hyper`] which need a
/// shared `(exp(v), exp(-v))` pair without paying the to_work_w lift
/// twice.
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
    let j_signed =
        core::round_to_nearest_int(core::div_cached(s * core::lit(M as u128), l2, pow10_w), w);
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
    debug_assert!(
        j_idx < M,
        "tang_exp_fixed d616 s300..=315: table index out of range"
    );

    // Taylor on δ.
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
        if n > 400 {
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
            "tang_exp_fixed d616 s300..=315: result overflows the representable range",
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

/// Tang-style `e^x` strict kernel for `D616<SCALE>` with
/// `SCALE ∈ 300..=315`.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int2048, mode: RoundingMode) -> Int2048 {
    if raw == Int2048::ZERO {
        let ten: Int2048 = crate::wide_int::wide_cast::<u128, Int2048>(10);
        return ten.pow(SCALE);
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let result = tang_exp_fixed(v_w, w);
    core::round_to_storage_with(result, w, SCALE, mode)
}

/// Narrow guard used by the Tang exp kernel — exposed so the
/// hyperbolic kernels can lift their argument to the same working
/// width before calling [`tang_exp_fixed`].
pub(crate) const GUARD_FOR_HYPER: u32 = GUARD_NARROW;
