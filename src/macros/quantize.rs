// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Macro-generated `quantize` / `quantize_with` for all decimal widths.
//!
//! `quantize` sets the quantum: it changes `SCALE` at a fixed storage width.
//! [`requantize`](crate::macros::requantize) moves both axes at once.
//!
//! The body lives in `quantize_with`, which takes an explicit
//! `RoundingMode`. The no-arg `quantize` delegates to it with the
//! crate's `DEFAULT_ROUNDING_MODE`, which is `HalfToEven` unless a
//! `rounding-*` Cargo feature selects something else.
//!
//! The methods are ordinary `fn`, not `const fn`: the wide integer's
//! `Div` / `Rem` operators are not `const`.
//!
//! # The `rescale` alias
//!
//! This operation shipped in 0.5.0 as `rescale` — the name the decimal
//! arithmetic specification marks as deprecated in favour of `quantize`.
//! `rescale` / `rescale_with` are kept as delegating aliases and are
//! removed in 0.6.0.

/// Emits `quantize` (no-arg, uses `DEFAULT_ROUNDING_MODE`) and
/// `quantize_with` (explicit mode) methods for `$Type<SCALE>` with
/// storage `$Storage`, plus the deprecated `rescale` aliases.
macro_rules! decl_decimal_quantize {
    // Wide storage. Not `const` — the wide integer's `Div`/`Rem`
    // operators are not const fns.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Quantizes to `TARGET_SCALE` using the crate's default
            /// rounding mode (`HalfToEven`, or whatever a `rounding-*`
            /// Cargo feature selects). Delegates to [`Self::quantize_with`].
            #[inline]
            #[must_use]
            pub fn quantize<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.quantize_with::<TARGET_SCALE>($crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Builder-style alias for [`Self::quantize`].
            ///
            /// Returns a new value at `TARGET_SCALE` using the crate's
            /// default rounding mode. Use [`Self::quantize_with`] when
            /// you need to pass an explicit [`RoundingMode`].
            ///
            /// [`RoundingMode`]: $crate::support::rounding::RoundingMode
            #[inline]
            #[must_use]
            pub fn with_scale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.quantize::<TARGET_SCALE>()
            }

            /// Quantizes to `TARGET_SCALE` using the supplied rounding
            /// mode.
            ///
            /// - `TARGET_SCALE == SCALE`: bit-identity.
            /// - `TARGET_SCALE > SCALE`: scale-up multiplies by
            /// `10^(TARGET - SCALE)`; lossless; panics on overflow.
            /// - `TARGET_SCALE < SCALE`: scale-down divides by
            /// `10^(SCALE - TARGET)` with the requested rounding rule.
            #[inline]
            #[must_use]
            pub fn quantize_with<const TARGET_SCALE: u32>(
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
                    let scaled_up = match self.0.checked_mul(multiplier) {
                        Some(scaled) => scaled,
                        None => panic!(concat!(stringify!($Type), "::quantize: scale-up overflow")),
                    };
                    return $Type::<TARGET_SCALE>::from_bits(scaled_up);
                }
                let shift = SCALE - TARGET_SCALE;
                let divisor = ten.pow(shift);
                let raw = self.0;
                let quotient = raw / divisor;
                let remainder = raw % divisor;
                if remainder == zero {
                    return $Type::<TARGET_SCALE>::from_bits(quotient);
                }
                let abs_remainder = remainder.unsigned_abs();
                let half = divisor.unsigned_abs() >> 1;
                let is_non_negative = !raw.is_negative();
                let bits = match mode {
                    $crate::support::rounding::RoundingMode::HalfToEven => {
                        if abs_remainder < half {
                            quotient
                        } else if abs_remainder > half {
                            if is_non_negative {
                                quotient + one
                            } else {
                                quotient - one
                            }
                        } else if !quotient.bit(0) {
                            quotient
                        } else if is_non_negative {
                            quotient + one
                        } else {
                            quotient - one
                        }
                    }
                    $crate::support::rounding::RoundingMode::HalfAwayFromZero => {
                        if abs_remainder < half {
                            quotient
                        } else if is_non_negative {
                            quotient + one
                        } else {
                            quotient - one
                        }
                    }
                    $crate::support::rounding::RoundingMode::HalfTowardZero => {
                        if abs_remainder > half {
                            if is_non_negative {
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
                        if is_non_negative {
                            quotient
                        } else {
                            quotient - one
                        }
                    }
                    $crate::support::rounding::RoundingMode::Ceiling => {
                        if is_non_negative {
                            quotient + one
                        } else {
                            quotient
                        }
                    }
                };
                $Type::<TARGET_SCALE>::from_bits(bits)
            }

            /// Deprecated alias for [`Self::quantize`].
            ///
            /// Delegates unchanged; removed in 0.6.0.
            #[inline]
            #[must_use]
            #[deprecated(
                since = "0.5.1",
                note = "renamed to `quantize`; `rescale` is removed in 0.6.0"
            )]
            pub fn rescale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.quantize::<TARGET_SCALE>()
            }

            /// Deprecated alias for [`Self::quantize_with`].
            ///
            /// Delegates unchanged; removed in 0.6.0.
            #[inline]
            #[must_use]
            #[deprecated(
                since = "0.5.1",
                note = "renamed to `quantize_with`; `rescale_with` is removed in 0.6.0"
            )]
            pub fn rescale_with<const TARGET_SCALE: u32>(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Type<TARGET_SCALE> {
                self.quantize_with::<TARGET_SCALE>(mode)
            }
        }
    };

}

pub(crate) use decl_decimal_quantize;
