//! Tang-style table-driven `sin_strict` + `cos_strict` + `tan_strict`
//! kernel for `D57<SCALE>` with `SCALE ∈ 18..=22`.
//!
//! Same shape as the SCALE 44..=56 sibling
//! ([`super::lookup_d57_s44_56_sincos`]) — see Tang 1991 ACM TOMS 17(4)
//! "Table-driven implementation of the sin/cos function" for the
//! underlying technique. Tuned for the narrow-GUARD regime: smaller
//! table (M = 64 vs M = 512) since the per-thread cold-start cost is
//! more pressing at this scale tier, and the residual `|δ| ≤ π/(8M)`
//! is still small enough for ~5-term Taylor convergence at narrow w.
//!
//! ## Algorithm
//!
//! ```text
//! x = k·(π/2) + r,    k = round(x · 2/π),    |r| ≤ π/4
//! r = c_j + δ,        c_j = j·π/(4M),        |δ| ≤ π/(8M)
//!
//! sin(r) = sin(c_j)·cos(δ) + cos(c_j)·sin(δ)
//! cos(r) = cos(c_j)·cos(δ) − sin(c_j)·sin(δ)
//! ```
//!
//! Then quadrant `k mod 4` permutes (sin, cos) of `r` into (sin, cos)
//! of `x`.
//!
//! With `M = 64` the residual `|δ| ≤ π/512 ≈ 6.1·10⁻³`, so `|δ²| ≤
//! 3.8·10⁻⁵`. The sin/cos Taylor series converge in ~5 terms each at
//! `w = SCALE + 8 = 26..30`.
//!
//! ## Memory
//!
//! Per-thread: `2·(M+1)·sizeof(W) = (M+1)·256 B = ~16 KB` at M=64.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::wide_int::Int192;

/// Narrow guard matches the non-Tang sin/cos kernel.
const GUARD_NARROW: u32 = 8;

/// Table size — smaller than the SCALE 44..=56 sibling's `M = 512`
/// because narrow-tier cold-start matters more here. See module docs.
const M: u32 = 64;

type Entry = (core::W, core::W);

crate::policy::table_cache::decl_table_cache!(entry = Entry, compute = compute_table);

fn compute_table(w: u32) -> alloc::vec::Vec<Entry> {
    let mut out = alloc::vec::Vec::with_capacity((M + 1) as usize);
    let pi_w = core::pi(w);
    let step_denom = core::lit((4 * M) as u128);
    out.push((core::zero(), core::one(w))); // j=0: sin(0)=0, cos(0)=1.
    for j in 1..=M {
        let cj_w = (pi_w * core::lit(j as u128)) / step_denom;
        out.push(core::sin_cos_fixed(cj_w, w));
    }
    out
}

#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

#[must_use]
fn sin_cos_fixed_tang(v_w: core::W, w: u32) -> (core::W, core::W) {
    let one_w = core::one(w);
    let pow10_w = one_w;
    let pi_w = core::pi(w);
    let half_pi_w = core::half_pi(w);

    // Stage 1: x = k·(π/2) + r, |r| ≤ π/4 + slack.
    let k = core::round_to_nearest_int(core::div_cached(v_w, half_pi_w, pow10_w), w);
    let k_half_pi = if k >= 0 {
        half_pi_w * core::lit(k as u128)
    } else {
        -(half_pi_w * core::lit((-k) as u128))
    };
    let r = v_w - k_half_pi;

    // Stage 2: r = c_j_signed·π/(4M) + δ, |δ| ≤ π/(8M).
    let four_m = core::lit((4 * M) as u128);
    let j_signed = core::round_to_nearest_int(core::div_cached(r * four_m, pi_w, pow10_w), w);
    let cj_signed_w = if j_signed >= 0 {
        (pi_w * core::lit(j_signed as u128)) / four_m
    } else {
        -((pi_w * core::lit((-j_signed) as u128)) / four_m)
    };
    let delta = r - cj_signed_w;

    let j_abs = j_signed.unsigned_abs() as u32;
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

    let delta2 = core::mul_cached(delta, delta, pow10_w);

    // sin(δ) = δ − δ³/3! + …
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
            if k_term > 100 {
                break;
            }
        }
        sum
    };

    // cos(δ) = 1 − δ²/2! + …
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
            if k_term > 100 {
                break;
            }
        }
        sum
    };

    let sin_r =
        core::mul_cached(sin_cj, cos_delta, pow10_w) + core::mul_cached(cos_cj, sin_delta, pow10_w);
    let cos_r =
        core::mul_cached(cos_cj, cos_delta, pow10_w) - core::mul_cached(sin_cj, sin_delta, pow10_w);

    let quadrant = ((k % 4) + 4) % 4;
    match quadrant {
        0 => (sin_r, cos_r),
        1 => (cos_r, -sin_r),
        2 => (-sin_r, -cos_r),
        3 => (-cos_r, sin_r),
        _ => unreachable!(),
    }
}

#[inline]
#[must_use]
pub(crate) fn sin_cos_strict<const SCALE: u32>(
    raw: Int192,
    mode: RoundingMode,
    which: Which,
) -> Int192 {
    if raw == Int192::ZERO {
        return match which {
            Which::Sin => Int192::ZERO,
            Which::Cos => {
                let ten: Int192 = crate::wide_int::wide_cast::<u128, Int192>(10);
                ten.pow(SCALE)
            }
        };
    }

    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let (sin_x, cos_x) = sin_cos_fixed_tang(v_w, w);
    let result = match which {
        Which::Sin => sin_x,
        Which::Cos => cos_x,
    };
    core::round_to_storage_with(result, w, SCALE, mode)
}

#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Sin)
}

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Cos)
}

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    if raw == Int192::ZERO {
        return Int192::ZERO;
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let (sin_x, cos_x) = sin_cos_fixed_tang(v_w, w);
    if cos_x == core::zero() {
        panic!("D57::tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let r = core::div(sin_x, cos_x, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
