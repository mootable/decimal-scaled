// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `to_int_quotient_remainder` — round `raw / 10^SCALE` to a whole
//! number with two typed full-width divides, then narrow to `i64`.
//!
//! This is the shape that shipped in 0.5.x, lifted out of the
//! `decl_decimal_int_conversion_methods!` macro (where it was emitted
//! twelve times) into one generic kernel. The arithmetic is unchanged;
//! what changed is that the eight-mode rounding match it carried inline
//! is gone, replaced by the one shared decider
//! [`should_bump`](crate::support::rounding::should_bump).
//!
//! # Why the substitution is bit-identical
//!
//! The inline match compared `|r|` against `half = 10^SCALE >> 1`;
//! `should_bump` takes a three-way comparison of `|r|` against
//! `|m| − |r|`. For an **even** divisor the two agree exactly —
//! `|r| < m/2 ⟺ 2|r| < m ⟺ |r| < m − |r|`, and likewise for `>` and
//! `=` — and `m = 10^SCALE` is even for every `SCALE >= 1`. `SCALE == 0`
//! makes `m = 1`, so the remainder is always zero and the early return
//! fires before any comparison is formed. The `|m| − |r|` form is also
//! what every other rounding site in the crate passes, so this kernel
//! now agrees with them by construction rather than by inspection.
//!
//! The other two inputs map just as directly. `should_bump` reads the
//! half-to-even tie break off `q_mod_10 & 1`; the inline match read it
//! off `quotient.bit(0)`. Ten is even, so the last decimal digit of a
//! magnitude and the magnitude itself share a parity, and two's
//! complement negation preserves bit 0 — so both express the parity of
//! `|q|`. `ZeroFiveUp` wants the digit itself, which is the same
//! `|q| % 10` the inline match computed with a full-width `%`; here it
//! comes from [`limbs_mod_10`], one `u64 % 10` per limb.
//!
//! # Why the divisor is passed in
//!
//! `10^SCALE` is `Self::multiplier()` at the call site — the exact value
//! and the exact overflow semantics this method has always had. The
//! crate caps no tier's `SCALE` at its `MAX_SCALE`, so deriving the
//! divisor here instead (from the baked POW10 table, keyed on the const
//! `SCALE`) would move where an out-of-range `SCALE` fails. Taking it as
//! an operand keeps that behaviour exactly where it was.

use crate::algos::to_int::narrow_saturating::narrow_saturating_i64;
use crate::int::types::Int;
use crate::support::rounding::{limbs_mod_10, should_bump, RoundingMode};

/// Rounds `raw / divisor` to a whole number under `mode` and narrows it
/// to `i64`, saturating out of range. `divisor` is `10^SCALE` and is
/// always positive.
#[inline]
pub(crate) fn to_int_quotient_remainder<const N: usize>(
    raw: Int<N>,
    divisor: Int<N>,
    mode: RoundingMode,
) -> i64 {
    let quotient = raw / divisor;
    let remainder = raw % divisor;
    // An exact value needs no rounding — and this is the contract
    // `should_bump` relies on to make `AwayFromZero` an unconditional
    // `true` and `ZeroFiveUp` a bare digit test.
    if remainder == Int::<N>::ZERO {
        return narrow_saturating_i64(quotient);
    }

    let abs_remainder = remainder.unsigned_abs();
    // `divisor` is `10^SCALE` and always positive, so `unsigned_abs()`
    // is the value itself.
    let divisor_mag = divisor.unsigned_abs();
    let remainder_cmp = abs_remainder.cmp(&(divisor_mag - abs_remainder));
    let q_mod_10 = limbs_mod_10(quotient.unsigned_abs().as_limbs());
    // Sign of the true result: the divisor is positive, so it is the
    // sign of `raw`. A zero `raw` has a zero remainder and returned
    // above.
    let result_positive = !raw.is_negative();

    let rounded = if should_bump(mode, remainder_cmp, q_mod_10, result_positive) {
        // The bump is always one step AWAY from zero in the result's
        // direction, which is what `should_bump`'s contract promises.
        if result_positive {
            quotient + Int::<N>::ONE
        } else {
            quotient - Int::<N>::ONE
        }
    } else {
        quotient
    };
    narrow_saturating_i64(rounded)
}
