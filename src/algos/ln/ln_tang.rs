// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic Tang-style table-driven `ln_strict` kernel.
//!
//! Tang 1990, "Table-driven implementation of the logarithm function in
//! IEEE floating-point arithmetic" (ACM TOMS 16(4) 378-400).
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
//! `|t²| < 1.5·10⁻⁵`; the artanh series `2·(t + t³/3 + t⁵/5 + ...)`
//! converges fast. The table `ln(1 + i/M)` (one `ln_fixed` per slot) is
//! memoised per thread per working scale by the tier's `Core`.
//!
//! ## Layering
//!
//! This is an **algorithm function** (`docs/ARCHITECTURE.md` →
//! "Layering direction"): it computes only through the
//! [`WideTrigCore`] trait surface (which forwards *down* into the
//! per-tier guard-digit kernels) and `BigInt` arithmetic on the work
//! integer. It never calls a method on a decimal type. The thirteen
//! `policy::ln` Tang arms call *down* to [`ln_tang`]; the type's
//! `ln_strict` method delegates *down* through the policy.
//!
//! This collapses the thirteen per-tier Tang `ln_strict`
//! kernels — structurally identical bar the `core` module
//! (`wide_trig_d*`), the storage `Int<N>`, the narrow guard
//! (`GUARD = 8` or `10`) and the artanh-series iteration cap — into one
//! generic over `C: WideTrigCore`.

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Table size — number of `ln(1 + i/M)` entries per working scale. The
/// `i = 0` slot is `ln(1) = 0`, the `i = M` slot is `ln(2)`. Every
/// shipped tier uses `M = 128`.
const M: u32 = 128;

/// Tang-style `ln(v)` for a working-scale value `v_w` (`= x · 10^w`),
/// returned at the same working scale `w`. Generic over the tier `C`
/// and the artanh-series iteration cap `CAP` (a safety net; the loop
/// terminates on a zero term far sooner). Shared across guard widths so
/// the Ziv escalation can re-evaluate at a wider scale.
#[inline]
fn ln_value<C: WideTrigCore, const CAP: u128>(v_w: C::W, w: u32) -> C::W {
    let one_w = C::one(w);
    let pow10_w = one_w;
    let two_w = one_w + one_w;

    // Stage 1: v = 2^k · m, m ∈ [1, 2). k from bit-shifts.
    let mut k: i32 = C::bit_length(v_w) as i32 - C::bit_length(one_w) as i32;
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

    // Stage 2: pick i. Boundary `m = 1` short-circuits: ln(m) = 0, so
    // ln(v) = k · ln(2).
    if m_w == one_w {
        return if k >= 0 {
            C::ln2(w) * C::lit(k as u128)
        } else if k < 0 {
            -(C::ln2(w) * C::lit((-k) as u128))
        } else {
            C::zero()
        };
    }

    // i ∈ [0, M); when m = 2 exactly (rare boundary post-rounding),
    // clamp to M-1 so the table lookup stays in range, then the
    // residual t handles the remaining tiny piece.
    let i_raw = ((m_w - one_w) * C::lit(M as u128)) / one_w;
    let i_i128 = BigInt::to_i128(i_raw);
    let i_idx = if i_i128 >= M as i128 {
        (M - 1) as usize
    } else {
        i_i128 as usize
    };

    let f_i = one_w + (one_w * C::lit(i_idx as u128)) / C::lit(M as u128);

    // Stage 3: t = (m - f_i) / (m + f_i). |t| < 1/(2M + 1).
    let t = C::div_cached(m_w - f_i, m_w + f_i, pow10_w);

    // Artanh series: 2 · (t + t³/3 + t⁵/5 + ...).
    let t2 = C::mul(t, t, w);
    let mut sum = t;
    let mut term = t;
    let mut j: u128 = 1;
    loop {
        term = C::mul(term, t2, w);
        let contrib = term / C::lit(2 * j + 1);
        if contrib == C::zero() {
            break;
        }
        sum = sum + contrib;
        j += 1;
        if j > CAP {
            break;
        }
    }
    let ln_m = sum + sum + C::ln_table_entry(w, i_idx);

    // Final: ln(v) = k · ln(2) + ln(m).
    let k_ln2 = if k >= 0 {
        C::ln2(w) * C::lit(k as u128)
    } else {
        -(C::ln2(w) * C::lit((-k) as u128))
    };
    k_ln2 + ln_m
}

/// Tier-generic Tang-style `ln(x)` strict kernel. Panics if `raw <= 0`.
///
/// - `C` — the per-tier [`WideTrigCore`] marker (`wide_trig_d*::Core`).
/// - `SCALE` — the decimal storage scale.
/// - `GUARD` — the narrow guard for this band (`8` or `10`).
/// - `CAP` — the artanh-series iteration safety cap.
/// - `DIRECTED` — `true` routes the final narrowing through the shared
///   directed-rounding Ziv escalation (the dominant shape, used by every
///   band where a near-grid-line directed decision can flip a storage
///   LSB); `false` narrows once with `round_to_storage_with` (the
///   D57<18..=22> band, whose original kernel rounded the working
///   approximation directly).
#[inline]
#[must_use]
pub(crate) fn ln_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const GUARD: u32,
    const CAP: u128,
    const DIRECTED: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw <= C::storage_zero() {
        panic!("wide-tier ln: argument must be positive");
    }
    if DIRECTED {
        // Directed modes decide which side of a storage grid line the
        // true value falls; near a grid line the working-scale
        // approximation can land on the wrong side. Route through the
        // shared Ziv escalation; nearest modes narrow once.
        C::round_to_storage_directed(GUARD, SCALE, mode, &mut |guard| {
            ln_value::<C, CAP>(C::to_work_w(raw, guard), SCALE + guard)
        })
    } else {
        let w = SCALE + GUARD;
        let r = ln_value::<C, CAP>(C::to_work_w(raw, GUARD), w);
        C::round_to_storage_with(r, w, SCALE, mode)
    }
}
