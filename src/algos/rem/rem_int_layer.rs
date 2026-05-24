// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_int_layer` -- decimal remainder via the `Int<N>` layer.

use crate::int::types::Int;

/// Decimal remainder via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: division by zero always panics; `MIN % -ONE`
/// panics in debug (with "overflow") and wraps in release (matching
/// `i128::wrapping_rem`). No rescaling needed -- same-SCALE operands share
/// the scale factor.
///
/// Routes the actual remainder through the `Int<N>` `Rem` operator
/// ([`crate::int::policy::rem::dispatch`] -> `rem_via_div_rem` ->
/// [`crate::int::policy::div_rem::dispatch`], the Knuth / Burnikel-Ziegler
/// engine), NOT `Int::wrapping_rem` / `Int::checked_rem`. Those route to the
/// const single-algorithm `div_rem`, whose multi-limb fallback is an
/// `O(bit_len)` binary shift-subtract: for a wide divisor (`10^SCALE` with
/// `SCALE > 19`, e.g. a 32-limb `10^616` at D1232) that fallback runs
/// thousands of shift-subtract steps and dominated the wide-tier remainder
/// (a ~25x regression vs 0.4.4, whose `Rem` operator already used the Knuth
/// dispatcher). The operator path resolves the same remainder via Knuth in a
/// single multiply-subtract sweep. Bit-identical result for every operand
/// relationship; only the engine differs.
///
/// The operator is only reached for `N >= 3` (the decimal `rem` policy routes
/// `N <= 2` to `rem_native`), so the narrow hardware-`%` path is untouched.
#[inline]
pub(crate) fn rem_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    // Divide-by-zero panics in both modes (the operator path asserts on a
    // zero divisor). In debug, the `MIN % -ONE` overflow must also panic,
    // matching the primitive contract and the prior `checked_rem` path;
    // detect it with cheap comparisons (no divide) and panic before the
    // operator wraps it to zero.
    if cfg!(debug_assertions) && a == Int::<N>::MIN && b == -Int::<N>::ONE {
        panic!("attempt to calculate the remainder with overflow");
    }
    a % b
}
