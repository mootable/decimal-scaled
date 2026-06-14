// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Macro-generated `rescale` / `rescale_with` for all decimal widths.
//!
//! The body lives in `rescale_with`, which takes an explicit
//! `RoundingMode`. The no-arg `rescale` delegates to it with the
//! crate's `DEFAULT_ROUNDING_MODE`, which is `HalfToEven` unless a
//! `rounding-*` Cargo feature selects something else.
//!
//! - The *native* arm emits `rescale` / `rescale_with` as `const fn`,
//! since primitive integer arithmetic is `const`.
//! - The *wide* arm emits them as ordinary `fn`: the wide integer's `Div` / `Rem`
//! operators are not `const`, so the wide rescale path cannot be a
//! `const fn`. The semantics are otherwise identical.

/// Emits `rescale` (no-arg, uses `DEFAULT_ROUNDING_MODE`) and
/// `rescale_with` (explicit mode) methods for `$Type<SCALE>` with
/// storage `$Storage`.
macro_rules! decl_decimal_rescale {
    // Wide storage. Not `const` — the wide integer's `Div`/`Rem`
    // operators are not const fns.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Rescales to `TARGET_SCALE` using the crate's default
            /// rounding mode (`HalfToEven`, or whatever a `rounding-*`
            /// Cargo feature selects). Delegates to [`Self::rescale_with`].
            #[inline]
            #[must_use]
            pub fn rescale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.rescale_with::<TARGET_SCALE>($crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Builder-style alias for [`Self::rescale`].
            ///
            /// Returns a new value at `TARGET_SCALE` using the crate's
            /// default rounding mode. Use [`Self::rescale_with`] when
            /// you need to pass an explicit [`RoundingMode`].
            ///
            /// [`RoundingMode`]: $crate::support::rounding::RoundingMode
            #[inline]
            #[must_use]
            pub fn with_scale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.rescale::<TARGET_SCALE>()
            }

            /// Rescales to `TARGET_SCALE` using the supplied rounding
            /// mode.
            ///
            /// - `TARGET_SCALE == SCALE`: bit-identity.
            /// - `TARGET_SCALE > SCALE`: scale-up multiplies by
            /// `10^(TARGET - SCALE)`; lossless; panics on overflow.
            /// - `TARGET_SCALE < SCALE`: scale-down divides by
            /// `10^(SCALE - TARGET)` with the requested rounding rule.
            #[inline]
            #[must_use]
            pub fn rescale_with<const TARGET_SCALE: u32>(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Type<TARGET_SCALE> {
                if TARGET_SCALE == SCALE {
                    return $Type::<TARGET_SCALE>::from_bits(self.0);
                }
                let ten = <$Storage>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let one = <$Storage>::from_str_radix("1", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let zero = <$Storage>::from_str_radix("0", 10)
                    .expect("wide decimal: invalid base-10 literal");
                if TARGET_SCALE > SCALE {
                    let shift = TARGET_SCALE - SCALE;
                    let multiplier = ten.pow(shift);
                    let result = match self.0.checked_mul(multiplier) {
                        Some(v) => v,
                        None => panic!(concat!(stringify!($Type), "::rescale: scale-up overflow")),
                    };
                    return $Type::<TARGET_SCALE>::from_bits(result);
                }
                let shift = SCALE - TARGET_SCALE;
                let divisor = ten.pow(shift);
                let raw = self.0;
                let quotient = raw / divisor;
                let remainder = raw % divisor;
                if remainder == zero {
                    return $Type::<TARGET_SCALE>::from_bits(quotient);
                }
                let abs_rem = remainder.unsigned_abs();
                let half = divisor.unsigned_abs() >> 1;
                let non_negative = !raw.is_negative();
                let bits = match mode {
                    $crate::support::rounding::RoundingMode::HalfToEven => {
                        if abs_rem < half {
                            quotient
                        } else if abs_rem > half {
                            if non_negative {
                                quotient + one
                            } else {
                                quotient - one
                            }
                        } else if !quotient.bit(0) {
                            quotient
                        } else if non_negative {
                            quotient + one
                        } else {
                            quotient - one
                        }
                    }
                    $crate::support::rounding::RoundingMode::HalfAwayFromZero => {
                        if abs_rem < half {
                            quotient
                        } else if non_negative {
                            quotient + one
                        } else {
                            quotient - one
                        }
                    }
                    $crate::support::rounding::RoundingMode::HalfTowardZero => {
                        if abs_rem > half {
                            if non_negative {
                                quotient + one
                            } else {
                                quotient - one
                            }
                        } else {
                            quotient
                        }
                    }
                    $crate::support::rounding::RoundingMode::Trunc => quotient,
                    $crate::support::rounding::RoundingMode::Floor => {
                        if non_negative {
                            quotient
                        } else {
                            quotient - one
                        }
                    }
                    $crate::support::rounding::RoundingMode::Ceiling => {
                        if non_negative {
                            quotient + one
                        } else {
                            quotient
                        }
                    }
                };
                $Type::<TARGET_SCALE>::from_bits(bits)
            }
        }
    };

}

pub(crate) use decl_decimal_rescale;
