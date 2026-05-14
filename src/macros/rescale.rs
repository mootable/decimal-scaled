//! Macro-generated `rescale` / `rescale_with` for all decimal widths.
//!
//! The body lives in `rescale_with` (a `const fn` that takes an
//! explicit `RoundingMode`). The no-arg `rescale` delegates to it with
//! the crate's `DEFAULT_ROUNDING_MODE`, which is `HalfToEven` unless a
//! `rounding-*` Cargo feature selects something else.

/// Emits `rescale` (no-arg, uses `DEFAULT_ROUNDING_MODE`) and
/// `rescale_with` (explicit mode) methods for `$Type<SCALE>` with
/// storage `$Storage`.
macro_rules! decl_decimal_rescale {
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Rescales to `TARGET_SCALE` using the crate's default
            /// rounding mode (`HalfToEven`, or whatever a
            /// `rounding-*` Cargo feature selects).
            ///
            /// Delegates to [`Self::rescale_with`]; see that method for
            /// scale-up / scale-down semantics and the overflow policy.
            #[inline]
            #[must_use]
            pub const fn rescale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.rescale_with::<TARGET_SCALE>($crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Rescales to `TARGET_SCALE` using the supplied rounding
            /// mode.
            ///
            /// - `TARGET_SCALE == SCALE`: bit-identity.
            /// - `TARGET_SCALE > SCALE`: scale-up multiplies by
            ///   `10^(TARGET - SCALE)`; lossless; panics on overflow.
            /// - `TARGET_SCALE < SCALE`: scale-down divides by
            ///   `10^(SCALE - TARGET)` with the requested rounding rule.
            #[inline]
            #[must_use]
            pub const fn rescale_with<const TARGET_SCALE: u32>(
                self,
                mode: $crate::rounding::RoundingMode,
            ) -> $Type<TARGET_SCALE> {
                if TARGET_SCALE == SCALE {
                    return $Type::<TARGET_SCALE>(self.0);
                }
                if TARGET_SCALE > SCALE {
                    let shift = TARGET_SCALE - SCALE;
                    let multiplier = (10 as $Storage).pow(shift);
                    let result = match self.0.checked_mul(multiplier) {
                        Some(v) => v,
                        None => panic!(concat!(stringify!($Type), "::rescale: scale-up overflow")),
                    };
                    return $Type::<TARGET_SCALE>(result);
                }
                let shift = SCALE - TARGET_SCALE;
                let divisor = (10 as $Storage).pow(shift);
                let raw = self.0;
                let quotient = raw / divisor;
                let remainder = raw % divisor;
                if remainder == 0 {
                    return $Type::<TARGET_SCALE>(quotient);
                }
                let abs_rem = remainder.unsigned_abs();
                let half = (divisor / 2) as _;
                let bits = match mode {
                    $crate::rounding::RoundingMode::HalfToEven => {
                        if abs_rem < half {
                            quotient
                        } else if abs_rem > half {
                            if raw >= 0 { quotient + 1 } else { quotient - 1 }
                        } else if quotient % 2 == 0 {
                            quotient
                        } else if raw >= 0 {
                            quotient + 1
                        } else {
                            quotient - 1
                        }
                    }
                    $crate::rounding::RoundingMode::HalfAwayFromZero => {
                        if abs_rem < half {
                            quotient
                        } else if raw >= 0 {
                            quotient + 1
                        } else {
                            quotient - 1
                        }
                    }
                    $crate::rounding::RoundingMode::HalfTowardZero => {
                        if abs_rem > half {
                            if raw >= 0 { quotient + 1 } else { quotient - 1 }
                        } else {
                            quotient
                        }
                    }
                    $crate::rounding::RoundingMode::Trunc => quotient,
                    $crate::rounding::RoundingMode::Floor => {
                        if raw >= 0 { quotient } else { quotient - 1 }
                    }
                    $crate::rounding::RoundingMode::Ceiling => {
                        if raw >= 0 { quotient + 1 } else { quotient }
                    }
                };
                $Type::<TARGET_SCALE>(bits)
            }
        }
    };
}

pub(crate) use decl_decimal_rescale;
