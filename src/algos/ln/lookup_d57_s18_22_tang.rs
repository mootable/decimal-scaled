//! Tang-style table-driven `ln_strict` kernel for `D57<SCALE>` with
//! `SCALE ∈ 18..=22`.
//!
//! Sibling to the Tang-style exp at
//! [`crate::algos::exp::lookup_d57_s18_22_tang`]. See Tang 1990,
//! "Table-driven implementation of the logarithm function in IEEE
//! floating-point arithmetic" (ACM TOMS 16(4) 378-400).
//!
//! ## Algorithm
//!
//! ```text
//! v = 2^k · m,                m ∈ [1, 2)
//! i = floor((m - 1) · M),     i ∈ [0, M)
//! f_i = 1 + i / M             (table-indexed boundary)
//! L_i = ln(f_i)               (table entry)
//! t = (m - f_i) / (m + f_i)   (|t| < 1 / (2M + 1))
//! ln(m) = L_i + 2 · artanh(t) (= L_i + ln((1 + t) / (1 - t)) the
//!                              identity reformulated as a series)
//! ln(v) = k · ln(2) + ln(m)
//! ```
//!
//! With `M = 128` the residual `|t| < 1/257 ≈ 3.9·10⁻³`, so
//! `|t²| < 1.5·10⁻⁵`. The artanh series `2·(t + t³/3 + t⁵/5 + ...)`
//! converges in `log₁₀(10⁻²⁸) / log₁₀(1.5·10⁻⁵) ≈ 6 pair-terms` at
//! `w = SCALE + 8 = 26..30`. Compare to the non-table narrow-GUARD
//! kernel which runs ~12 artanh muls plus 2 wide sqrt_fixed for
//! Brent's argument-halving — the table win is the elimination of
//! both wide sqrts (each ~1 µs) and half the artanh muls.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 8` matches the sibling
//!   [`crate::algos::ln::lookup_d57_s18_22`]. The error budget here
//!   is similar — ~3 muls + ~12 muls in artanh + table addition,
//!   total ~15 LSB-of-w. Still 6 orders of magnitude below
//!   half-storage-ULP at SCALE ≤ 22.
//! - `M = 128` mirrors the exp Tang slot's table size. Per-thread
//!   memory cost: `(M+1)·sizeof(W) = 129·128 B = ~16 KB`.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::wide_int::Int192;

/// Narrow guard for the Tang-style ln slot at SCALE 18..=22.
const GUARD_NARROW: u32 = 8;

/// Table size — number of `ln(1 + i/M)` entries per working scale.
/// Matches the exp Tang slot for consistency.
const M: u32 = 128;

crate::policy::table_cache::decl_table_cache!(entry = core::W, compute = compute_table);

/// Build the `ln(1 + i/M)` table at working scale `w` using the
/// canonical ln_fixed kernel (one call per slot, paid once per thread
/// per w). `i ∈ [0, M]`; the `i = 0` slot is exactly zero, the
/// `i = M` slot is `ln(2)` and is needed because rounding can round
/// the argument up to the boundary.
fn compute_table(w: u32) -> alloc::vec::Vec<core::W> {
    let mut out = alloc::vec::Vec::with_capacity((M + 1) as usize);
    let one_w = core::one(w);
    out.push(core::zero()); // ln(1) = 0.
    for i in 1..=M {
        // f_i = 1 + i/M, computed at working scale: (one_w * M + one_w * i) / M
        // = one_w * (M + i) / M. To avoid mul-then-div precision loss
        // we form (one_w + one_w · i / M) using exact integer ops:
        let scaled = (one_w * core::lit(i as u128)) / core::lit(M as u128);
        let f_i = one_w + scaled;
        out.push(core::ln_fixed(f_i, w));
    }
    out
}

/// Tang-style `ln(x)` strict kernel for `D57<SCALE>` with
/// `SCALE ∈ 18..=22`. Panics if `raw <= 0`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    if raw <= Int192::ZERO {
        panic!("D57::ln: argument must be positive");
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let one_w = core::one(w);
    let pow10_w = one_w;
    let two_w = one_w + one_w;

    // Stage 1: v = 2^k · m, m ∈ [1, 2). k from bit-shifts.
    let mut k: i32 = core::bit_length(v_w) as i32 - core::bit_length(one_w) as i32;
    let m_w = loop {
        let m = if k >= 0 {
            v_w >> (k as u32)
        } else {
            v_w << ((-k) as u32)
        };
        if m >= two_w {
            k += 1;
        } else if m < one_w {
            k -= 1;
        } else {
            break m;
        }
    };

    // Stage 2: pick i. `m ∈ [1, 2)`, so `(m - one_w) / (one_w / M) =
    // (m - 1) · M ∈ [0, M)`. Round down (truncate) via integer
    // division. Special-case the boundary `m = 1` to avoid pulling
    // in a table lookup when we know the answer is exact.
    if m_w == one_w {
        // ln(m) = 0, so ln(v) = k · ln(2).
        let r = if k >= 0 {
            core::ln2(w) * core::lit(k as u128)
        } else if k < 0 {
            -(core::ln2(w) * core::lit((-k) as u128))
        } else {
            core::zero()
        };
        return core::round_to_storage_with(r, w, SCALE, mode);
    }

    // i ∈ [0, M); when m = 2 exactly (rare boundary post-rounding),
    // clamp to M-1 so the table lookup stays in range, then the
    // residual t handles the remaining tiny piece.
    let i_raw = ((m_w - one_w) * core::lit(M as u128)) / one_w;
    let i_i128 = crate::wide_int::wide_cast::<core::W, i128>(i_raw);
    let i_idx = if i_i128 >= M as i128 {
        (M - 1) as usize
    } else {
        i_i128 as usize
    };

    let f_i = one_w + (one_w * core::lit(i_idx as u128)) / core::lit(M as u128);

    // Stage 3: t = (m - f_i) / (m + f_i). |t| < 1/(2M + 1).
    let t = core::div_cached(m_w - f_i, m_w + f_i, pow10_w);

    // Artanh series: 2 · (t + t³/3 + t⁵/5 + ...).
    let t2 = core::mul_cached(t, t, pow10_w);
    let mut sum = t;
    let mut term = t;
    let mut j: u128 = 1;
    loop {
        term = core::mul_cached(term, t2, pow10_w);
        let contrib = term / core::lit(2 * j + 1);
        if contrib == core::zero() {
            break;
        }
        sum = sum + contrib;
        j += 1;
        if j > 100 {
            break;
        }
    }
    let ln_m = sum + sum + table_entry(w, i_idx);

    // Final: ln(v) = k · ln(2) + ln(m).
    let k_ln2 = if k >= 0 {
        core::ln2(w) * core::lit(k as u128)
    } else {
        -(core::ln2(w) * core::lit((-k) as u128))
    };
    let r = k_ln2 + ln_m;
    core::round_to_storage_with(r, w, SCALE, mode)
}
