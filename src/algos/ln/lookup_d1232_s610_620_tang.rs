//! Tang-style table-driven `ln_strict` kernel for `D1232<SCALE>` with
//! `SCALE ∈ 610..=620` — the mid-storage popular band centred on
//! `SCALE = 615` (half of `MAX_SCALE = 1231`).
//!
//! Deepest sibling of the per-width Tang ln stack:
//! [`crate::algos::ln::lookup_d616_s300_315_tang`] is the next step up
//! the ladder. See Tang 1990, "Table-driven implementation of the
//! logarithm function in IEEE floating-point arithmetic" (ACM TOMS
//! 16(4) 378-400).
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
//! converges in `log₁₀(10⁻⁶³⁰) / log₁₀(1.5·10⁻⁵) ≈ 130 pair-terms` at
//! `w = SCALE + 8 ≤ 628`. Compare to the wide-tier macro `ln_fixed`
//! which runs Brent's argument-halving (4 wide `sqrt_fixed`s on a
//! 2048-byte working integer) plus the long artanh on `|m - 1| < 1`
//! — replacing the 4 wide sqrts and shortening the artanh tail is
//! the source of the speedup at D1232's working width.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 8` — error budget: ~130 muls × 0.5 LSB-of-w +
//!   table-entry rounding ≤ ~66 LSB-of-w. At `w = SCALE + 8`, that's
//!   `66·10⁻⁸` in storage units — many orders of magnitude below
//!   half a storage ULP for any `SCALE ≤ 620`.
//! - `M = 128` matches the D57 / D115 / D153 / D307 / D462 / D616
//!   Tang slots. Per-thread memory cost: `(M + 1) · sizeof(W) =
//!   129 · 2048 B ≈ 264 KB` for D1232's Int16384 working integer.
//!   Beyond L1d (32-64 KB), into L2 (256 KB-1 MB) on modern x86. A
//!   smaller `M` (e.g. 64) would halve the footprint but double the
//!   `|t|` bound and extend the artanh tail; bench-trial confirms
//!   `M = 128` still wins despite the larger table.

#![cfg(any(feature = "d1232", feature = "xx-wide"))]

use crate::types::widths::wide_trig_d1232 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int4096;

/// Narrow guard for the Tang-style ln slot at SCALE 610..=620. See
/// module docs for the derivation and headroom.
const GUARD_NARROW: u32 = 8;

/// Table size — number of `ln(1 + i/M)` entries per working scale.
const M: u32 = 128;

#[cfg(feature = "std")]
::std::thread_local! {
    /// Per-thread cache of `(ln_table[i] = ln(1 + i/M))` tables keyed
    /// on the working scale `w`. Table index range is `[0, M]`
    /// inclusive so the `m = 2` boundary case can land at `i = M`
    /// without going out of range.
    static TABLE_CACHE: ::core::cell::RefCell<alloc::vec::Vec<(u32, alloc::vec::Vec<core::W>)>> =
        const { ::core::cell::RefCell::new(alloc::vec::Vec::new()) };
}

#[cfg(feature = "std")]
fn table_entry(w: u32, i_idx: usize) -> core::W {
    TABLE_CACHE.with(|c| {
        {
            let cache = c.borrow();
            for (cw, tbl) in cache.iter() {
                if *cw == w {
                    return tbl[i_idx];
                }
            }
        }
        let tbl = compute_table(w);
        let entry = tbl[i_idx];
        c.borrow_mut().push((w, tbl));
        entry
    })
}

#[cfg(not(feature = "std"))]
fn table_entry(w: u32, i_idx: usize) -> core::W {
    compute_table(w)[i_idx]
}

/// Build the `ln(1 + i/M)` table at working scale `w` using the
/// canonical `ln_fixed` kernel (one call per slot, paid once per
/// thread per `w`).
fn compute_table(w: u32) -> alloc::vec::Vec<core::W> {
    let mut out = alloc::vec::Vec::with_capacity((M + 1) as usize);
    let one_w = core::one(w);
    out.push(core::zero()); // ln(1) = 0.
    for i in 1..=M {
        let scaled = (one_w * core::lit(i as u128)) / core::lit(M as u128);
        let f_i = one_w + scaled;
        out.push(core::ln_fixed(f_i, w));
    }
    out
}

/// Tang-style `ln(x)` strict kernel for `D1232<SCALE>` with
/// `SCALE ∈ 610..=620`. Panics if `raw <= 0`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: Int4096, mode: RoundingMode) -> Int4096 {
    if raw <= Int4096::ZERO {
        panic!("D1232::ln: argument must be positive");
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

    // Stage 2: pick i. Boundary `m = 1` short-circuits.
    if m_w == one_w {
        let r = if k >= 0 {
            core::ln2(w) * core::lit(k as u128)
        } else if k < 0 {
            -(core::ln2(w) * core::lit((-k) as u128))
        } else {
            core::zero()
        };
        return core::round_to_storage_with(r, w, SCALE, mode);
    }

    let i_raw = ((m_w - one_w) * core::lit(M as u128)) / one_w;
    let i_i128 = crate::wide_int::wide_cast::<core::W, i128>(i_raw);
    let i_idx = if i_i128 >= M as i128 { (M - 1) as usize } else { i_i128 as usize };

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
        if j > 400 {
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
