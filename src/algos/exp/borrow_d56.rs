//! D38 exponential via widen → D56 wide_kernel → narrow back.
//!
//! See [`crate::algos::ln::borrow_d56`] for the broader rationale.
//!
//! Correctness note for `exp`: unlike ln/sin/cos/atan, `exp(D38<S>::MAX)`
//! may overflow D38<S> even when D56 holds it comfortably. The current
//! `fixed_d38::exp_strict` panics on overflow via `round_to_i128_with`
//! returning `None`; this wrapper preserves that panic semantic via the
//! narrowing `TryFrom` failing and the wrapping `expect(...)`.

use crate::core_type::{D38, D56};
use crate::rounding::RoundingMode;

/// D38 exponential via widen → D56 wide_kernel → narrow back. Strict
/// working scale.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = super::wide_kernel::exp_strict_d56(widened.0, mode, SCALE);
    let narrowed: D38<SCALE> = D56::<SCALE>::from_bits(raw_wide)
        .try_into()
        .expect("exp kernel: result overflows the representable range");
    narrowed.0
}
