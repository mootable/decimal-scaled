// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: k*ln2 range reduction + the 2^k reassembly identity, not wired

//! `expm1` by the classic `k*ln 2` range reduction and the `2^k` reassembly.
//!
//! `v = k*ln 2 + s` with `k = round(v / ln 2)` and `|s| <= ln2/2`, so
//! `e^v = 2^k * e^s` and, with `E = expm1(s)` and `P = 10^w`,
//!
//! ```text
//! expm1(v) = 2^k * (1 + E) - 1
//! ```
//!
//! # The reassembly identity — derived
//!
//! Two forms are available:
//!
//! ```text
//! (i)  naive   R = ((P + E) << k) - P
//! (ii) split   R = (2^k - 1)*P + (E << k)
//! ```
//!
//! For `k >= 0` they are **bit-identical**:
//!
//! ```text
//! ((P + E) << k) - P  =  P*2^k + E*2^k - P  =  (2^k - 1)*P + (E << k)
//! ```
//!
//! Every operation on both sides is an exact integer shift / add / subtract at
//! ONE shared working scale, so neither rounds. The floating-point reason to
//! prefer the split form — that forming `1 + E` renormalises and discards `E`'s
//! low bits — has no analogue in fixed point: `P + E` is an exact integer
//! addition that keeps every bit of `E`. Form (i) is therefore used: one shift,
//! one subtract, provably the same integer.
//!
//! For `k < 0`, form (i) applies ONE rounded shift where the split form applies
//! two (and its `round_shr(E, |k|)` flushes a small `E` to zero for the same
//! information loss with worse bookkeeping), so form (i) is used there too —
//! with a half-to-even `round_div` rather than a truncating `>>`.
//!
//! # Why the `- P` cannot cancel catastrophically
//!
//! **Lemma.** For `k != 0` and `|s| <= ln2/2`, `|expm1(v)| >= 0.2928`.
//!
//! *Proof.* `k >= 1`: `e^v = 2^k e^s >= 2*e^{-ln2/2} = sqrt(2)`, so
//! `expm1(v) >= sqrt(2) - 1 = 0.4142`. `k <= -1`:
//! `e^v = 2^k e^s <= (1/2)*e^{ln2/2} = 1/sqrt(2)`, so
//! `expm1(v) <= 1/sqrt(2) - 1 = -0.2929`.
//!
//! The larger operand of the final subtraction is at most `sqrt(2)*P` (`k >= 1`)
//! or `P` (`k <= -1`) and the result is at least `0.2929*P`, so the subtraction
//! discards at most `log2(1/0.2929) = 1.77` bits — uniformly, at every `k` and
//! every scale. **Cancellation can only occur at `k = 0`, where there is no
//! reassembly at all** (the kernel returns `E` directly).
//!
//! # Where cancellation DOES enter
//!
//! Not the reassembly — the REDUCTION. `s = v - k*ln 2` subtracts two
//! quantities of size `|k|*ln 2` to leave `|s| <= ln2/2`, losing `log2(2|k|)`
//! bits, and with `ln2_w` rounded to half a unit the error is
//! `d_s <= |k|/2 + 1` working units. Since `dE/ds = e^s ~ 1` and the
//! reassembly multiplies by `2^k`,
//!
//! ```text
//! d_result ~ 2^k * (|k|/2 + 1)   working units
//! ```
//!
//! so correct rounding needs `guard > k*log10 2 + log10|k|` — i.e. the guard
//! must absorb the result's integer-digit count. That is exactly the
//! `extra_digits` lift `exp_fixed` provisions, mirrored here.
//!
//! # Validity
//!
//! * writing `ext` for `extended_working_scale`,
//!   `max(2*ext*log2(10), ext*log2(10) + |k|) + 64 < BITS` — the `exp_fixed`
//!   wall verbatim (the squaring peak, and the `<< k` reassembly peak).
//! * `|k| < BITS`, guaranteed by the [`super::expm1_generic::Regime`] pre-gate.
//!
//! Unlike `expm1_halving`, the growth is applied ONCE at the end, so the
//! doubling chain stays on `~P` and the peak does not carry `e^v`. That is what
//! makes this the candidate for large positive `v`.

#![allow(dead_code)]

use super::expm1_halving::expm1_doubling_core;
use super::expm1_generic as sup;
use crate::algos::exp::exp_generic as eg;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `expm1(v)` for a `working_value` at `working_scale`, by `k*ln 2`
/// reduction. `None` = cannot be produced in `S` at this `working_scale` (the
/// `try_exp_fixed` contract).
pub(crate) fn expm1_reduced_fixed<S: BigInt>(working_value: S, working_scale: u32) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    if working_value == S::ZERO {
        return Some(S::ZERO);
    }
    match sup::regime::<S>(working_value, working_scale) {
        sup::Regime::Overflow => return None,
        sup::Regime::MinusOne => return Some(sup::just_above_minus_one::<S>(working_scale)),
        sup::Regime::Fits => {}
    }

    // The range-reduction quotient, at the caller's scale. `Regime::Fits`
    // bounds `|v|` so this quotient stays well inside `i128`.
    let one_at_working_scale = eg::one::<S>(working_scale);
    let ln2_at_working_scale =
        crate::consts::ln2_by_working_scale::<S>(working_scale, RoundingMode::HalfToEven);
    let k = eg::round_to_nearest_int(
        eg::div_cached::<S>(working_value, ln2_at_working_scale, one_at_working_scale),
        working_scale);

    // Guard lift, `exp_fixed`'s shape: `ceil(|k|*log10 2)` digits for the
    // reduction error `2^k*(|k|/2 + 1)`, plus the flat slack that also covers
    // the doubling chain's `2^n`.
    let abs_k = k.unsigned_abs();
    let k_digits = abs_k.saturating_mul(30_103).div_ceil(100_000).min(u32::MAX as u128) as u32;
    let extra_digits = sup::extra_digits::<S>(k_digits);

    let extended_working_scale = working_scale.checked_add(extra_digits)?;
    // Peak, exactly `exp_fixed`'s two terms: the symmetric doubling product
    // spans twice the extended scale's digits, and the `<< k` reassembly lifts
    // an extended-scale value by `|k|` bits.
    let squaring_bits = 2 * sup::scale_bits(extended_working_scale);
    let reassembly_bits = sup::scale_bits(extended_working_scale)
        .saturating_add(abs_k.min(u64::MAX as u128) as u64);
    if !sup::peak_fits::<S>(squaring_bits.max(reassembly_bits)) {
        return None;
    }

    let extended_working_value = if extra_digits == 0 {
        working_value
    } else {
        working_value * eg::pow10::<S>(extra_digits)
    };
    let ln2_at_extended_scale =
        crate::consts::ln2_by_working_scale::<S>(extended_working_scale, RoundingMode::HalfToEven);
    let reduced_arg = extended_working_value - eg::scale_by_k::<S>(ln2_at_extended_scale, k);

    // `|s| <= ln2/2`, so the doubling core only needs the precision-driven
    // depth (no band levels).
    let expm1_of_reduced_arg = expm1_doubling_core::<S>(
        reduced_arg,
        extended_working_scale,
        sup::halving_levels(extended_working_scale));

    let one_at_extended_scale = eg::one::<S>(extended_working_scale);
    // exact: `e^s * 10^ext`, every bit of the reduced expm1 kept
    let lifted = one_at_extended_scale + expm1_of_reduced_arg;
    let reassembled = if k >= 0 {
        let shift = k as u32;
        // The reassembly must not wrap: `wrapping` arithmetic below it would
        // truncate a genuinely out-of-range result into a small wrong value.
        if eg::bit_length::<S>(lifted) + shift >= <S as BigInt>::BITS {
            return None;
        }
        (lifted << shift) - one_at_extended_scale
    } else {
        let shift_bits = abs_k;
        if shift_bits >= eg::bit_length::<S>(lifted) as u128 {
            // `2^k * e^s` is below the working resolution: `expm1(v)` sits
            // strictly between `-1` and `-1 + 10^-w`. Return the value one
            // working unit above `-1`, never a bare `-P` — see
            // `expm1_generic::just_above_minus_one`.
            return Some(sup::just_above_minus_one::<S>(working_scale));
        }
        // One half-to-even rounded shift (<= 1/2 working unit), not a
        // truncating `>>`.
        eg::round_div::<S>(lifted, S::ONE << (shift_bits as u32)) - one_at_extended_scale
    };

    let expm1_value = if extra_digits == 0 {
        reassembled
    } else {
        eg::round_div_pow10::<S>(reassembled, extra_digits)
    };
    Some(expm1_value)
}
