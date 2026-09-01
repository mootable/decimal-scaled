// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Signed remainder via the division policy.
//!
//! [`rem_via_div_rem`] is the width-agnostic remainder algorithm selected by
//! the remainder policy [`crate::int::policy::rem::dispatch`]. It derives the
//! remainder by delegating to [`crate::int::policy::div_rem::dispatch`] — the
//! single division optimization boundary — and taking the remainder half.
//! NOT `const fn`: the division dispatcher is a runtime-shape value-matcher.

use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
use crate::int::types::Int;

/// Signed remainder via the division policy for `Int<N>`. Strips the
/// operand signs, calls [`div_rem_dispatch`] on the unsigned magnitudes,
/// re-applies the dividend's sign to the remainder (truncating-toward-zero
/// semantics), and returns the signed result.
///
/// Delegates to [`crate::int::policy::div_rem::dispatch`] — the single
/// site the division optimization boundary lives at — rather than
/// reimplementing the Knuth / Burnikel–Ziegler engine selection inline.
/// `div_rem_dispatch` is NOT `const fn` (its value-matcher invokes a fn
/// pointer at runtime), so this algorithm fn is not `const fn` either.
/// The caller (`dispatch`) is correspondingly non-const.
///
/// Panics on a zero divisor, matching the `Rem` operator contract.
#[inline]
pub(crate) fn rem_via_div_rem<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> Int<N> {
    assert!(
        !divisor.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    let remainder_is_negative = dividend.is_negative();
    let mut quotient = [0u64; N];
    let mut remainder = [0u64; N];
    div_rem_dispatch(
        dividend.unsigned_abs().as_limbs(),
        divisor.unsigned_abs().as_limbs(),
        &mut quotient,
        &mut remainder,
    );
    Int::<N>::from_mag_limbs(&remainder, remainder_is_negative)
}
