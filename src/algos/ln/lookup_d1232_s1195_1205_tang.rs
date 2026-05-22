//! Tang-style table-driven `ln_strict` kernel for `D1232<SCALE>` with
//! `SCALE ∈ 1195..=1205` — the deep-storage band approaching the MAX
//! end of D1232 (`MAX_SCALE = 1231`).
//!
//! Sibling to the D1232 mid-band Tang ln at
//! [`crate::algos::ln::lookup_d1232_s610_620_tang`]. See Tang 1990,
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
//! converges in `log₁₀(10⁻¹²¹⁵) / log₁₀(1.5·10⁻⁵) ≈ 252 pair-terms`
//! at `w = SCALE + 10 = 1205..1215`. Compare to the non-table
//! wide-kernel path which runs ~380 artanh muls plus several wide
//! `sqrt_fixed` calls for Brent's argument-halving — at Int16384
//! working width each sqrt is a full-width divmod inside Newton, so
//! the table-win is the elimination of all wide sqrts plus a chunk
//! of the artanh series.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 10` — error budget: ~252 muls × 0.5 LSB-of-w +
//!   table-entry rounding ≈ 126 LSB-of-w. At `w = SCALE + 10`, that's
//!   `126·10⁻¹⁰` in storage units — many orders of magnitude below
//!   half a storage ULP for any `SCALE ≤ 1205`.
//! - `M = 128` matches the D1232 mid-band sibling slot. Per-thread
//!   memory cost: `(M+1) · sizeof(W) = 129·2048 B = ~264 KB` for
//!   D1232's Int16384 working integer. Larger than L1d (32 KB) and
//!   typically larger than L2 (256 KB-1 MB on modern x86) — the
//!   table is built once per thread per w, then iterated through L2
//!   /L3 during artanh.

#![cfg(any(feature = "d1232", feature = "xx-wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d1232 as core;
use crate::wide_int::Int4096;

/// Narrow guard for the Tang-style ln slot at SCALE 1195..=1205. See
/// module docs for the derivation and headroom.
const GUARD_NARROW: u32 = 10;

/// Table size — number of `ln(1 + i/M)` entries per working scale.
const M: u32 = 128;

crate::policy::table_cache::decl_table_cache!(entry = core::W, compute = compute_table);

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
/// `SCALE ∈ 1195..=1205`. Panics if `raw <= 0`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: Int4096, mode: RoundingMode) -> Int4096 {
    if raw <= Int4096::ZERO {
        panic!("D1232::ln: argument must be positive");
    }
    // Directed modes decide which side of a storage grid line the true
    // value falls; near a grid line (e.g. ln(1 + 10^-SCALE), whose value
    // sits ~SCALE digits below the unit) the working-scale approximation
    // can land on the wrong side. Route through the shared Ziv escalation;
    // nearest modes narrow once.
    core::round_to_storage_directed(GUARD_NARROW, SCALE, mode, |guard| {
        ln_value(core::to_work_w(raw, guard), SCALE + guard)
    })
}

/// Tang-style `ln(v)` for a working-scale value `v_w` (`= x · 10^w`),
/// returned at the same working scale `w`. Shared across guard widths so
/// the Ziv escalation can re-evaluate at a wider scale.
fn ln_value(v_w: core::W, w: u32) -> core::W {
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
        return if k >= 0 {
            core::ln2(w) * core::lit(k as u128)
        } else if k < 0 {
            -(core::ln2(w) * core::lit((-k) as u128))
        } else {
            core::zero()
        };
    }

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
    k_ln2 + ln_m
}
