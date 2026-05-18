//! D38 exponential / base-2 exponential via widen → D56 wide_kernel
//! → narrow back.
//!
//! See `algos::ln::borrow_d56` for the broader rationale.
//!
//! Correctness note for `exp` / `exp2`: unlike ln/sin/cos/atan,
//! `exp(D38::<S>::MAX)` may overflow `D38::<S>` even when D56 holds
//! it comfortably. The bespoke `fixed_d38::exp_strict` panics on
//! overflow via `round_to_i128_with` returning `None`; these wrappers
//! preserve that panic semantic via the narrowing `TryFrom` failing
//! and the wrapping `expect(...)`.

// `crate::algos::ln::borrow_d56` is referenced in the module docs as a
// plain code span rather than an intra-doc link because both modules
// are pub(crate); intra-doc links to private items break the
// `RUSTDOCFLAGS=-D warnings` doc build (the docs.yml workflow uses
// this flag to catch broken intra-doc links).

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

/// D38 base-2 exponential via widen → D56 inherent `exp2_strict_with`
/// → narrow back. D56's `exp2_strict_with` shares the
/// `wide_trig_d56::exp_fixed` core that the borrow `exp_strict`
/// uses, so this picks up the same speedup.
#[inline]
#[must_use]
pub(crate) fn exp2_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result = widened.exp2_strict_with(mode);
    let narrowed: D38<SCALE> = result
        .try_into()
        .expect("exp2 kernel: result overflows the representable range");
    narrowed.0
}
