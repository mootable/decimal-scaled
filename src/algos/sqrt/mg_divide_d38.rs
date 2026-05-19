//! D38 square-root kernel — `mg_divide::sqrt_raw_with`.
//!
//! Captures the **width-level specialisation** that has lived on D38
//! since before the algorithm library existed: a hand-tuned isqrt on a
//! 256-bit intermediate (`mul2` + `isqrt_256`) tailored to the `u128`
//! storage. Strictly faster than the generic wide kernel (which would
//! widen `i128 → Int512` and use the generic `Int512::isqrt`).
//!
//! Signature mirrors the generic kernels: takes the raw storage
//! integer, the scale, and the rounding mode; returns the raw storage
//! integer of the square root.
//!
//! Negative-input saturation (`raw < 0 → 0`) is handled by the policy
//! caller, not here, mirroring the contract of
//! [`crate::algos::mg_divide::sqrt_raw_with`] which only accepts non-negative
//! input.

use crate::support::rounding::RoundingMode;

/// D38 square-root kernel. Input `raw` must be non-negative; the
/// policy caller saturates negatives to zero before invoking this.
#[inline]
#[must_use]
pub(crate) fn sqrt(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    debug_assert!(raw >= 0, "mg_divide_d38::sqrt: negative input — caller must saturate");
    crate::algos::mg_divide::sqrt_raw_with(raw as u128, scale, mode) as i128
}
