//! Macro-generated `rescale` / `rescale_with` for all decimal widths.
//!
//! Each width's storage type is the only thing that varies; the
//! algorithm (scale-up: multiply by `10^diff`; scale-down: divide and
//! apply rounding mode) is identical. `apply_rounding` lives in
//! `src/rounding.rs` and is generic over the storage type via the
//! integer arithmetic operators.

/// Emits `rescale` and `rescale_with` methods for `$Type<SCALE>` with
/// storage `$Storage`. `$Storage` must support `pow`, `checked_mul`,
/// `unsigned_abs`, `/`, `%`, and the comparison operators.
macro_rules! decl_decimal_rescale {
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Rescales to `TARGET_SCALE` using round-half-to-even.
            #[inline]
            #[must_use]
            pub const fn rescale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
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
                let bits = if abs_rem < half {
                    quotient
                } else if abs_rem > half {
                    if raw >= 0 { quotient + 1 } else { quotient - 1 }
                } else if quotient % 2 == 0 {
                    quotient
                } else if raw >= 0 {
                    quotient + 1
                } else {
                    quotient - 1
                };
                $Type::<TARGET_SCALE>(bits)
            }

            /// Rescales to `TARGET_SCALE` using the supplied rounding
            /// mode.
            #[inline]
            #[must_use]
            pub fn rescale_with<const TARGET_SCALE: u32>(
                self,
                mode: $crate::rounding::RoundingMode,
            ) -> $Type<TARGET_SCALE> {
                if TARGET_SCALE == SCALE {
                    return $Type::<TARGET_SCALE>::from_bits(self.0);
                }
                if TARGET_SCALE > SCALE {
                    let shift = TARGET_SCALE - SCALE;
                    let multiplier = (10 as $Storage).pow(shift);
                    let result = self
                        .0
                        .checked_mul(multiplier)
                        .expect(concat!(stringify!($Type), "::rescale_with: scale-up overflow"));
                    return $Type::<TARGET_SCALE>::from_bits(result);
                }
                let shift = SCALE - TARGET_SCALE;
                let divisor = (10 as $Storage).pow(shift);
                let raw = self.0;
                let quotient = raw / divisor;
                let remainder = raw % divisor;
                if remainder == 0 {
                    return $Type::<TARGET_SCALE>::from_bits(quotient);
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
                $Type::<TARGET_SCALE>::from_bits(bits)
            }
        }
    };
}

pub(crate) use decl_decimal_rescale;
