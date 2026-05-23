//! `sqrt_mg_divide` — D38 square-root kernel
//! (`mg_divide::sqrt_raw_with`).
//!
//! Captures the **width-bespoke specialisation** that has lived on D38
//! since before the algorithm library existed: a hand-tuned isqrt on a
//! 256-bit intermediate (`mul_u128_to_u256` + `isqrt_256`) tailored to
//! the `Int<2>` (`i128`/`u128`) storage. Strictly faster than the
//! generic Newton kernel widening `Int<2> → Int<4>` and running the
//! generic `isqrt`.
//!
//! Genuinely width-bespoke (Q4.1b): the body is `i128`/`u128`-specific
//! 256-bit arithmetic with a hardware-`u128::isqrt` fast arm, so it
//! cannot be made generic over the storage width without losing the
//! intrinsic-backed fast path. It serves `N == 2` only (the D18 `N == 1`
//! tier widens to `Int<2>` in the policy layer and reuses it).
//!
//! Signature mirrors [`crate::algos::sqrt::sqrt_newton`]: takes the raw
//! storage integer, the scale, and the rounding mode; returns the raw
//! storage integer of the square root.
//!
//! Negative-input saturation (`raw < 0 → 0`) is handled by the policy
//! caller, not here, mirroring the contract of
//! [`crate::algos::mg_divide::sqrt_raw_with`] which only accepts
//! non-negative input.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// D38 square-root kernel. Input `raw` must be non-negative; the policy
/// caller saturates negatives to zero before invoking this.
///
/// `Int<2>` entry point: bridges the decimal storage type to the `i128`
/// core ([`sqrt_mg_divide_raw`]) at the algorithm boundary. `i128` does
/// not escape this module.
#[inline]
#[must_use]
pub(crate) fn sqrt_mg_divide(raw: Int<2>, scale: u32, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(sqrt_mg_divide_raw(raw.as_i128(), scale, mode))
}

/// `i128` core of the D38 square-root kernel.
#[inline]
#[must_use]
fn sqrt_mg_divide_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    debug_assert!(
        raw >= 0,
        "sqrt_mg_divide: negative input — caller must saturate"
    );
    crate::algos::mg_divide::sqrt_raw_with(raw as u128, scale, mode) as i128
}
