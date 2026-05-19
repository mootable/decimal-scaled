//! Tang-style table-driven `exp_strict` kernel for `D153<SCALE>` with
//! `SCALE ∈ 70..=82` — the mid-storage popular band centred on
//! `SCALE = 76`.
//!
//! Sibling to the D57 narrow-tier Tang exp at
//! [`crate::algos::exp::lookup_d57_s18_22_tang`]. See Tang 1989,
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
//! Taylor on δ converges in ~25 terms at `w = SCALE + 10 ≤ 92`. The
//! cross-cutting `exp_fixed` already runs adaptive Smith r/2^n with
//! `n ≈ √p_bits`; the Tang lookup replaces both the `k·ln 2` Stage 1
//! reduction *and* most of the post-reduction squaring loop with a
//! single table multiply.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 10` matches the sibling `lookup_d153_s70_82_tang`
//!   for ln, so the per-thread `pow10_w` cache is shared across the
//!   neighbouring exp/ln/sinh/cosh/tanh calls.
//! - `M = 128` mirrors the D57 narrow Tang exp slot. Per-thread memory
//!   cost: `M · sizeof(W) = 128·128 B = ~16 KB` for D153's Int1024.

#![cfg(any(feature = "d153", feature = "wide"))]

use crate::types::widths::wide_trig_d153 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int512;

/// Narrow guard for the SCALE 70..=82 Tang-exp slot.
const GUARD_NARROW: u32 = 10;

/// Table size — power of two so the index quantisation step
/// `ln(2) / M` keeps the cheap integer-division path.
const M: u32 = 128;

#[cfg(feature = "std")]
::std::thread_local! {
    /// Per-thread cache of `exp(j · ln 2 / M)` tables keyed on the
    /// working scale `w`.
    static TABLE_CACHE: ::core::cell::RefCell<alloc::vec::Vec<(u32, alloc::vec::Vec<core::W>)>> =
        const { ::core::cell::RefCell::new(alloc::vec::Vec::new()) };
}

#[cfg(feature = "std")]
fn table_entry(w: u32, j_idx: usize) -> core::W {
    TABLE_CACHE.with(|c| {
        {
            let cache = c.borrow();
            for (cw, tbl) in cache.iter() {
                if *cw == w {
                    return tbl[j_idx];
                }
            }
        }
        let tbl = compute_table(w);
        let entry = tbl[j_idx];
        c.borrow_mut().push((w, tbl));
        entry
    })
}

#[cfg(not(feature = "std"))]
fn table_entry(w: u32, j_idx: usize) -> core::W {
    compute_table(w)[j_idx]
}

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
/// [`crate::algos::trig::lookup_d153_s70_82_hyper`] which need a
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
    debug_assert!(j_idx < M, "tang_exp_fixed d153 s70..=82: table index out of range");

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
            "tang_exp_fixed d153 s70..=82: result overflows the representable range",
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

/// Tang-style `e^x` strict kernel for `D153<SCALE>` with
/// `SCALE ∈ 70..=82`.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int512, mode: RoundingMode) -> Int512 {
    if raw == Int512::ZERO {
        let ten: Int512 = crate::wide_int::wide_cast::<u128, Int512>(10);
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
