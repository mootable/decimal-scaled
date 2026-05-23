//! Tang-style table-driven `ln_strict` kernel for `D924<SCALE>` with
//! `SCALE ∈ 455..=465` — the mid-storage popular band centred on
//! `SCALE = 461 ≈ MAX_SCALE / 2` (`MAX_SCALE = 923`).
//!
//! Sibling to the D616 mid-storage Tang ln at
//! [`crate::algos::ln::lookup_d616_s300_315_tang`]. See Tang 1990,
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
//! converges in `log₁₀(10⁻ʷ) / log₁₀(1.5·10⁻⁵) ≈ w/5` pair-terms.
//! At `w = SCALE + 8 = 463..473` that's ~93-95 pair-terms — still a
//! substantial win over the canonical `wide_kernel::ln_strict_d924`
//! whose Brent argument-halving runs 4 wide `sqrt_fixed` calls against
//! the `Int12288` working integer plus an artanh series at full
//! `SCALE + GUARD = 30` width.
//!
//! ## Tuning
//!
//! - `GUARD_NARROW = 8` — matches the D462 sibling slot. Error budget:
//!   ~95 muls × 0.5 LSB-of-w + table-entry rounding ≈ 50 LSB-of-w. At
//!   `w = SCALE + 8`, that's `50·10⁻⁸` in storage units — many orders
//!   of magnitude below half a storage ULP for any `SCALE ≤ 465`.
//! - `M = 128` matches the rest of the Tang slot family (D57 / D115 /
//!   D153 / D307 / D462 / D616). Per-thread memory cost:
//!   `(M + 1) · sizeof(W) = 129 · 1536 B ≈ 198 KB` for D924's Int12288
//!   working integer. Larger than L1d (32 KB typical) and L2 (256 KB-
//!   1 MB on modern x86) — relies on L3 + sequential prefetch. The
//!   crossover with `wide_kernel::ln_strict_d924`'s 4 wide sqrts +
//!   full-width artanh tail is still well in favour of the table at
//!   this working width per the D616 measurement template.

#![cfg(any(feature = "d924", feature = "xx-wide"))]

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d924 as core;

/// Narrow guard for the Tang-style ln slot at SCALE 455..=465. Matches
/// the D462 sibling — error budget is dominated by the artanh-series
/// term count (proportional to `w`), and at this working width the
/// 8-digit narrow guard already buys >40 LSB-of-w headroom.
pub(crate) const GUARD_NARROW: u32 = 8;

/// Table size — `M = 128` matches the rest of the Tang slot family.
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

/// Tang-style `ln(x)` strict kernel for `D924<SCALE>` with
/// `SCALE ∈ 455..=465`. Panics if `raw <= 0`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: Int<48>, mode: RoundingMode) -> Int<48> {
    if raw <= Int::<48>::ZERO {
        panic!("D924::ln: argument must be positive");
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
    let i_i128 = crate::int::types::traits::BigInt::to_i128(i_raw);
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
