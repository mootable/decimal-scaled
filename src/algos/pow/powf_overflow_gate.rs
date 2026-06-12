// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Width-generic analytic storage-overflow gate for the wide-tier
//! `powf` composition `x^y = exp(y·ln x)` — the wide sibling of the
//! narrow [`crate::algos::pow::powf_series_2limb`] `powf_overflow_gate`
//! (the `Fixed`/i128 form), generalised over any [`BigInt`] work
//! integer and any storage width.
//!
//! # Why the composition needs a pre-gate of its own
//!
//! The wide `powf` shell sizes its working lift from the result's
//! integer-digit count (`k_lift = digits(e^{y·ln x})`). For a
//! deep-overflow cell that count is in the hundreds, pushing the
//! working scale `w = SCALE + GUARD + k_lift` past the work integer's
//! safe working-scale ceiling (`~W::BITS/8` decimal digits — the bound
//! the Tang table reconstruction `slot_hi · 10^w` is sized for). Past
//! that ceiling the `ln` kernel's table product silently WRAPS,
//! returning a near-zero garbage `ln x`; the exp argument collapses,
//! every downstream overflow gate sees a tiny in-range argument, and
//! the deep band returns garbage near 1 instead of the contractual
//! "result out of range" panic (the `1.5^1000.5` D76 defect). The gate
//! classifies the provable overflow analytically BEFORE any lifted
//! working-scale arithmetic exists to wrap.

use crate::algos::exp::exp_generic as eg;
use crate::int::types::traits::BigInt;

/// Returns `true` when `x^y = e^arg` provably overflows a signed
/// storage integer of `storage_bits` bits at decimal scale `scale` —
/// i.e. the cell is contractually out of range and the caller must
/// panic (or signal `None` on a `checked_` surface) instead of running
/// the lifted composition.
///
/// `arg_w` is the composition argument `y·ln x` as a working-scale
/// value (`arg · 10^w`) in the work integer `S` — computed at the
/// CANONICAL working scale `w = SCALE + GUARD`, which every tier's work
/// integer carries in-contract (the gate must run before any
/// result-sized lift).
///
/// Derivation (a SUFFICIENT condition — a representable cell can never
/// fire):
///
/// * the storage holds magnitudes `< 2^(BITS−1) < 10^D` for
///   `D = ⌊BITS·30103/100000⌋ + 1` (30103/100000 over-approximates
///   log10 2), so the stored value `e^arg · 10^scale` overflows once
///   `e^arg ≥ 10^(D−scale)`, i.e. `arg ≥ (D−scale)·ln 10`;
/// * the threshold uses `D+1` (one full margin digit ≈ 2.3 in `arg`,
///   dwarfing the few-ULP-at-`w` error of the `ln`/`mul` that formed
///   `arg_w`) and `2.302586 > ln 10`, so a fired cell satisfies the
///   true bound with room;
/// * cells between the true edge and this threshold keep a small
///   `k_lift` (≤ the tier's own digit count), so their lifted working
///   scale stays inside the work integer's ceiling and the existing
///   storage-narrowing fit check raises the same contractual panic.
///
/// `arg < 0` (`x^y < 1`) never overflows; `w < 6` (no real caller —
/// every shell forms `w ≥ GUARD = 30`) skips the gate for totality.
#[inline]
pub(crate) fn powf_overflow_gate_g<S: BigInt>(
    arg_w: S,
    w: u32,
    storage_bits: u32,
    scale: u32,
) -> bool {
    if arg_w <= S::ZERO || w < 6 {
        return false;
    }
    // D = ⌊BITS·log10(2)⌋ + 1 (over-approximated), +1 margin digit.
    let d = (storage_bits as u64) * 30_103 / 100_000 + 2;
    // Every tier's max SCALE is below its digit count, so `d - scale`
    // is positive; saturate for totality.
    let digits_left = d.saturating_sub(scale as u64).max(1);
    // thr = (D+1−scale)·2.302586 · 10^w = (digits_left·2_302_586) · 10^(w−6).
    let thr = eg::lit::<S>((digits_left as u128 * 2_302_586) as i128)
        * eg::pow10::<S>(w - 6);
    arg_w >= thr
}
