//! Tang-style table-driven `exp_strict` kernel for `D57<SCALE>` with
//! `SCALE ∈ 18..=22`.
//!
//! Same two-stage range reduction as the SCALE 45..=56 sibling
//! ([`super::lookup_d57_s45_56`]) — see Tang 1989 (ACM TOMS 16(4))
//! "Table-driven implementation of the exponential function in
//! IEEE floating-point arithmetic" for the underlying technique:
//!
//! ```text
//! e^v = 2^k · e^s,            s = v − k·ln 2,           |s| ≤ ln 2 / 2
//!     = 2^k · e^(c_j) · e^δ,  c_j = j · ln 2 / M,       j ∈ [0, M)
//!                              δ  = s − c_j,            |δ| ≤ ln 2 / (2M)
//! ```
//!
//! Per-call body collapses the post-stage-1 Taylor on `|s| ≤ ln(2)/2`
//! (which the narrow-GUARD `lookup_d57_s18_22` runs ~25 muls on) into
//! a short Taylor on `|δ| ≤ ln(2)/(2M)` after a table multiply.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 12` matches the sibling `lookup_d57_s18_22`
//!   accuracy budget — the table multiply and Taylor on δ together
//!   accumulate ≤ 15 LSB-of-w (vs the 12 LSB-of-w of the non-table
//!   path), well within the 0.5-LSB-of-storage strict contract for
//!   any `SCALE ≤ 22`.
//! - `M = 128` strikes the balance for the narrow regime: smaller
//!   table than the SCALE 45..=56 sibling's `M = 512` (less cold-
//!   start cost — ~128 entries × ~1 µs each at narrow w ≈ 130 µs
//!   one-time per thread per w), and the residual `|δ| ≤ ln(2)/256
//!   ≈ 2.7·10⁻³` still gives a ~6-term Taylor convergence at w = 32.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::types::widths::wide_trig_d57 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int192;

/// Narrow guard for the SCALE 18..=22 Tang-exp slot.
const GUARD_NARROW: u32 = 8;

/// Table size — power of two so the index quantisation step
/// `ln(2) / M` keeps the cheap integer-division path. Tuning rationale
/// in the module docs.
const M: u32 = 128;

#[cfg(feature = "std")]
::std::thread_local! {
    /// Per-thread cache of `exp(j · ln 2 / M)` tables keyed on the
    /// working scale `w`. One entry per distinct `w` (typically one
    /// per `SCALE` choice in 18..=22).
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

/// Tang-style `e^x` strict kernel for `D57<SCALE>` with `SCALE ∈ 18..=22`.
///
/// Two-stage range reduction + table multiply + short Taylor on δ.
/// See module docs for the algorithm; identical structure to the
/// SCALE 45..=56 sibling, tuned for the narrow-GUARD regime.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    if raw == Int192::ZERO {
        let ten: Int192 = crate::wide_int::wide_cast::<u128, Int192>(10);
        return ten.pow(SCALE);
    }

    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
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
    debug_assert!(j_idx < M, "exp_strict d57 s18..=22 tang: table index out of range");

    // Taylor on δ. |δ| ≤ ln(2)/256 ≈ 2.7·10⁻³, so δⁿ shrinks fast and
    // the loop exits on a zero term in ~6-7 iterations at w = 32.
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
        if n > 100 {
            break;
        }
    }

    // exp(s) = table[j_idx] · sum at working scale.
    let exp_cj = table_entry(w, j_idx as usize);
    let exp_s = core::mul_cached(exp_cj, sum, pow10_w);

    // Reassemble: exp(v) = 2^(k + k_adj) · exp(s).
    let k_total = k + k_adj;
    let result = if k_total >= 0 {
        let shift = k_total as u32;
        debug_assert!(
            core::bit_length(exp_s) + shift < core::W::BITS,
            "exp_strict d57 s18..=22 tang: result overflows the representable range",
        );
        exp_s << shift
    } else {
        let neg_k = (-k_total) as u32;
        if neg_k as u128 >= core::bit_length(exp_s) as u128 {
            core::zero()
        } else {
            exp_s >> neg_k
        }
    };

    core::round_to_storage_with(result, w, SCALE, mode)
}
