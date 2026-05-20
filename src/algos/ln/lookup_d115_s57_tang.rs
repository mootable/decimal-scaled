//! Tang-style table-driven `ln_strict` kernel for `D115<SCALE>` with
//! `SCALE = 57` (mid-storage, popular precision tier).
//!
//! Direct port of [`crate::algos::ln::lookup_d57_s18_22_tang`] adapted
//! for the `D115` storage tier (`Int384` storage, `Int2048` work
//! integer). See Tang 1990, "Table-driven implementation of the
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
//! converges in `~p / log₁₀(1/t²) ≈ p / 5` pair-terms; at the
//! `GUARD_NARROW = 8` working scale `w = SCALE + 8 = 65`, that's
//! ~13 pair-terms. Compare to the macro-emitted `ln_fixed` which runs
//! Brent's multi-level sqrt + a long artanh — eliminating the wide
//! sqrt steps and shortening the artanh loop is the source of the
//! speedup.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 8` — the table multiply + table addition + Taylor
//!   on `|t| < 4·10⁻³` accumulate ≤ ~15 LSB-of-w of half-to-even
//!   rounded drift. At `w = 65` that's `15·10⁻⁶⁵ ≈ 1.5·10⁻⁶⁴` in
//!   working units; storage half-ULP is `0.5·10⁻⁵⁷ = 5·10⁻⁵⁸`, so the
//!   accumulated drift sits ~6 orders of magnitude below half-ULP.
//! - `M = 128` mirrors the D57 ln Tang slot. Per-thread memory cost:
//!   `(M + 1)·sizeof(W) = 129 · 256 B = ~33 KB` (W = Int2048). Fits
//!   L2; cold-start seed is `M · ln_fixed(w = 65) ≈ 128 · ~10 µs =
//!   ~1.3 ms`, paid once per thread per scale.

#![cfg(any(feature = "d115", feature = "wide"))]

use crate::types::widths::wide_trig_d115 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int384;

/// Narrow guard for the Tang-style ln slot at SCALE = 57.
pub(crate) const GUARD_NARROW: u32 = 8;

/// Table size — matches the D57 sibling. Power of two so the index
/// quantisation step `1/M` keeps the cheap integer-division path.
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
/// canonical `ln_fixed` kernel. `i ∈ [0, M]`.
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

/// Tang-style `ln(x)` strict kernel for `D115<SCALE>` with
/// `SCALE = 57`. Panics if `raw <= 0`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: Int384, mode: RoundingMode) -> Int384 {
    if raw <= Int384::ZERO {
        panic!("D115::ln: argument must be positive");
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let one_w = core::one(w);
    let pow10_w = one_w;
    let two_w = one_w + one_w;

    // Stage 1: v = 2^k · m, m ∈ [1, 2).
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

    // Boundary: m == 1 means ln(m) = 0 exactly.
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

    // Stage 2: pick table index i ∈ [0, M).
    let i_raw = ((m_w - one_w) * core::lit(M as u128)) / one_w;
    let i_i128 = crate::wide_int::wide_cast::<core::W, i128>(i_raw);
    let i_idx = if i_i128 >= M as i128 { (M - 1) as usize } else { i_i128 as usize };

    let f_i = one_w + (one_w * core::lit(i_idx as u128)) / core::lit(M as u128);

    // Stage 3: t = (m - f_i) / (m + f_i), |t| < 1/(2M + 1).
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
        if j > 200 {
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
