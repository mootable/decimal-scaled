//! D38 floating-point power via widen → D56 inherent `powf_strict_with`
//! / `powf_approx_with` → narrow back.
//!
//! See `algos::ln::borrow_d56` for the broader rationale. `powf`
//! composes `exp(y · ln x)` internally; D56's inherent `powf_strict_with`
//! routes that composition through the `wide_trig_d56::{ln,exp}_fixed`
//! cores that the matching `ln`/`exp` borrow wrappers already use, so
//! D38 picks up the same 5.9×/3×+ ln/exp speedups in composed form
//! rather than calling `fixed_d38::powf_strict` (which routes through
//! the 256-bit `Fixed` intermediate `ln_fixed`/`exp_fixed` cores).
//!
//! Correctness: `powf` can overflow `D38::<S>` for large bases or
//! large exponents even when D56 holds the intermediate comfortably;
//! the bespoke `fixed_d38::powf_strict` panics on overflow via
//! `round_to_i128_with` returning `None`. These wrappers preserve that
//! panic semantic via the narrowing `TryFrom` failing and the
//! wrapping `expect(...)`. The non-positive-base saturation to `0` is
//! handled inside D56's `powf_strict_with` / `powf_approx_with`, so
//! the wrapper does not need a separate fast path.

use crate::core_type::{D38, D56};
use crate::rounding::RoundingMode;

/// D38 `base^exp` via widen → D56 inherent `powf_strict_with` →
/// narrow back. Strict working scale (`SCALE + GUARD` const-folded
/// inside D56).
#[inline]
#[must_use]
pub(crate) fn powf_strict<const SCALE: u32>(
    base: i128,
    exp: i128,
    mode: RoundingMode,
) -> i128 {
    let base_w: D56<SCALE> = D38::<SCALE>::from_bits(base).into();
    let exp_w: D56<SCALE> = D38::<SCALE>::from_bits(exp).into();
    let result = base_w.powf_strict_with(exp_w, mode);
    let narrowed: D38<SCALE> = result
        .try_into()
        .expect("powf kernel: result overflows the representable range");
    narrowed.0
}

/// D38 `base^exp` with caller-chosen `working_digits` via widen →
/// D56 inherent `powf_approx_with` → narrow back.
#[inline]
#[must_use]
pub(crate) fn powf_with<const SCALE: u32>(
    base: i128,
    exp: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    let base_w: D56<SCALE> = D38::<SCALE>::from_bits(base).into();
    let exp_w: D56<SCALE> = D38::<SCALE>::from_bits(exp).into();
    let result = base_w.powf_approx_with(exp_w, working_digits, mode);
    let narrowed: D38<SCALE> = result
        .try_into()
        .expect("powf kernel: result overflows the representable range");
    narrowed.0
}
