//! Macro-generated conversions between primitive types and the decimal
//! widths.
//!
//! Two surfaces live here:
//!
//! - **Infallible `From<$Src>`** — emitted by [`decl_from_primitive!`]
//! for source types that always fit the destination. Multiplies the
//! input by `multiplier()` (= `10^SCALE`); overflow follows Rust's
//! default integer arithmetic (debug-mode panic, release-mode wrap).
//! - **Fallible `TryFrom<$Src>`** — emitted by [`decl_try_from_i128!`],
//! [`decl_try_from_u128!`], [`decl_try_from_f64!`], and
//! [`decl_try_from_f32!`] for sources where the scaled magnitude may
//! exceed the destination's range, or where the input may be
//! non-finite (`f32` / `f64`). Returns
//! [`ConvertError::Overflow`] / [`ConvertError::NotFinite`] instead of
//! panicking.
//!
//! Plus [`decl_decimal_int_conversion_methods!`] which emits
//! `from_int` / `from_i32` / `to_int` / `to_int_with` on each width.
//!
//! [`ConvertError::Overflow`]: $crate::support::error::ConvertError::Overflow
//! [`ConvertError::NotFinite`]: $crate::support::error::ConvertError::NotFinite

/// Generates `From<$Src> for $Type<SCALE>` that scales the value by
/// `10^SCALE` and stores it in `$Storage`. The cast `value as $Storage`
/// happens first when the source is narrower than the storage, which
/// is the lossless case; the subsequent multiply is the overflow risk.
macro_rules! decl_from_primitive {
    // Wide storage: the primitive widens into wide
    // storage via the `BigInt` cast, then scales by `10^SCALE`.
    (wide $Type:ident, $Storage:ty, $Src:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src> for $Type<SCALE> {
            /// Constructs from an integer by scaling to `value * 10^SCALE`.
            /// Overflows follow the wide integer's default arithmetic semantics
            /// (debug-mode panic, release-mode wrap).
            #[inline]
            fn from(value: $Src) -> Self {
                let widened: $Storage = $crate::int::types::traits::wide_cast(value as i128);
                Self(widened * Self::multiplier())
            }
        }
    };

}

pub(crate) use decl_from_primitive;

/// Generates `From<$Src<SCALE>> for $Dest<SCALE>` for a lossless
/// widening conversion (e.g. D18, D18 -> D38). `$SrcStorage`
/// must widen losslessly to `$DestStorage` via an `as` cast.
macro_rules! decl_cross_width_widening {
    // Widening *into* wide storage. The source storage is
    // a primitive integer or a narrower wide integer; either way
    // the `BigInt` cast performs the lossless widen.
    (wide $Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src<SCALE>> for $Dest<SCALE> {
            /// Widens a narrower decimal type to this wider one. The
            /// scale is unchanged; the storage is widened via
            /// the `BigInt` cast (lossless because the source domain is
            /// a subset of the destination).
            #[inline]
            fn from(value: $Src<SCALE>) -> Self {
                Self($crate::int::types::traits::wide_cast(value.to_bits()))
            }
        }
    };

}

pub(crate) use decl_cross_width_widening;

/// Generates `TryFrom<$Src<SCALE>> for $Dest<SCALE>` for a fallible
/// narrowing conversion. Returns
/// `Err(ConvertError::OutOfRange)` when the source value exceeds
/// the destination's representable range; otherwise returns the
/// narrowed value bit-for-bit (same logical decimal value).
macro_rules! decl_cross_width_narrowing {
    // Narrowing *from* wide storage. The destination may
    // be a primitive integer (e.g. D76 -> D38) or a narrower wide integer
    // integer (e.g. D153 -> D76); the `BigInt` cast handles the bound
    // widening and the final narrowing cast in both cases.
    (wide $Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<$Src<SCALE>> for $Dest<SCALE> {
            type Error = $crate::support::error::ConvertError;
            /// Attempts to narrow a wider decimal type to this narrower
            /// one. Fails with `Overflow` when the source value exceeds
            /// the destination's `MIN..=MAX`. The scale is unchanged.
            #[inline]
            fn try_from(value: $Src<SCALE>) -> ::core::result::Result<Self, Self::Error> {
                let bits = value.to_bits();
                let dest_max: $SrcStorage = $crate::int::types::traits::wide_cast(<$DestStorage>::MAX);
                let dest_min: $SrcStorage = $crate::int::types::traits::wide_cast(<$DestStorage>::MIN);
                if bits > dest_max || bits < dest_min {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self($crate::int::types::traits::wide_cast(bits)))
            }
        }
    };

}

pub(crate) use decl_cross_width_narrowing;

/// Emits `TryFrom<i128> for $Type<SCALE>` returning
/// `Result<Self, ConvertError::Overflow>` after `checked_mul` by the
/// multiplier and a narrowing range-check against `$Storage`.
macro_rules! decl_try_from_i128 {
    // Wide storage. For storage ≥ 128-bit (`Int<2>`+) `i128` widens
    // losslessly, so the only failure mode is the `checked_mul` by the
    // multiplier overflowing the (still finite) wide storage. For the
    // 64-bit `Int<1>` (D18) storage the cast can truncate, so the
    // round-trip check below rejects out-of-range `i128` inputs (it is a
    // no-op for the wider tiers, where `as_i128()` always recovers `value`).
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<i128> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: i128) -> ::core::result::Result<Self, Self::Error> {
                let widened: $Storage = $crate::int::types::traits::wide_cast(value);
                if $crate::int::types::traits::wide_cast::<$Storage, i128>(widened) != value {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                let scaled = widened
                    .checked_mul(Self::multiplier())
                    .ok_or($crate::support::error::ConvertError::Overflow)?;
                ::core::result::Result::Ok(Self(scaled))
            }
        }
    };

}

pub(crate) use decl_try_from_i128;

/// Emits `TryFrom<u128> for $Type<SCALE>`. First narrows u128 to i128
/// (rejecting if the value exceeds `i128::MAX`) then delegates to the
/// i128 path.
macro_rules! decl_try_from_u128 {
    // Wide storage. `u128` always widens losslessly into
    // the (256-bit-or-wider) signed storage — including values above
    // `i128::MAX`, which the native path would have to reject — so the
    // only failure mode is the `checked_mul` overflowing the storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<u128> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: u128) -> ::core::result::Result<Self, Self::Error> {
                let widened: $Storage = <$Storage>::from_u128(value);
                // For storage wider than 128 bits this always holds; for the
                // 128-bit `Int<2>` (D38) a `u128` above `i128::MAX` lands in
                // the sign bit, so a negative result means the input did not
                // fit the signed storage's positive range — reject as overflow.
                // (This also catches the 64-bit `Int<1>` (D18) case where the
                // low word's high bit was set.)
                if widened.is_negative() {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                // The `is_negative` test misses truncations that land on a
                // small non-negative value (e.g. `2^64` → 0 in `Int<1>`), so a
                // round-trip back to `u128` confirms nothing was dropped. No-op
                // for storage wide enough to hold every `u128`.
                if $crate::int::types::traits::wide_cast::<$Storage, u128>(widened) != value {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                let scaled = widened
                    .checked_mul(Self::multiplier())
                    .ok_or($crate::support::error::ConvertError::Overflow)?;
                ::core::result::Result::Ok(Self(scaled))
            }
        }
    };

}

pub(crate) use decl_try_from_u128;

/// Emits `TryFrom<f64> for $Type<SCALE>`. NaN / ±inf return
/// `NotFinite`; finite values whose scaled magnitude exceeds the
/// storage range return `Overflow`. Truncates toward zero (matches the
/// historical D38 behaviour). For rounding-mode-aware float
/// construction, use `from_f64_with`.
macro_rules! decl_try_from_f64 {
    // Wide storage. The multiplier and storage bounds
    // round-trip through `f64` via the `BigInt` cast; the final
    // `f64` -> wide cast is also the `BigInt` cast.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<f64> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: f64) -> ::core::result::Result<Self, Self::Error> {
                if !value.is_finite() {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::NotFinite,
                    );
                }
                let mult_f64: f64 = Self::multiplier().as_f64();
                let scaled = value * mult_f64;
                let storage_max_f64: f64 = <$Storage>::MAX.as_f64();
                let storage_min_f64: f64 = <$Storage>::MIN.as_f64();
                if !(storage_min_f64..storage_max_f64).contains(&scaled) {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self(<$Storage>::from_f64(scaled)))
            }
        }
    };

}

/// Emits `TryFrom<f32> for $Type<SCALE>` by delegating to the
/// `TryFrom<f64>` path via a widening cast. The body is storage-
/// agnostic — it only forwards — so a single arm serves every width;
/// the `wide` token is accepted and ignored for call-site symmetry
/// with the other conversion macros.
macro_rules! decl_try_from_f32 {
    // Storage-agnostic: forwards `f32` through the width's `TryFrom<f64>`.
    // `$Storage` is matched for call-site symmetry but unused.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<f32> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: f32) -> ::core::result::Result<Self, Self::Error> {
                <Self as ::core::convert::TryFrom<f64>>::try_from(value as f64)
            }
        }
    };
}

pub(crate) use decl_try_from_f32;
pub(crate) use decl_try_from_f64;

/// Emits the named integer constructors and `to_int` /
/// `to_int_with` on a decimal type. `$Storage` is the storage
/// integer; `$IntSrc` is the wider integer source for `from_int`
/// (typically `i64` for D18/D38). `from_int` and
/// `from_i32` scale directly (they do not depend on a `From<iN>` impl
/// existing for the width).
macro_rules! decl_decimal_int_conversion_methods {
    // Wide storage. The rounding logic mirrors the native
    // arm but is carried in the wide storage type throughout; the
    // `i128` source widens via the `BigInt` cast, and the final
    // saturating narrow to `i64` also goes through the `BigInt` cast.
    (wide $Type:ident, $Storage:ty, $IntSrc:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an integer source, scaling by `10^SCALE`.
            /// Overflow follows the wide integer's default arithmetic semantics.
            #[inline]
            pub fn from_int(value: $IntSrc) -> Self {
                let widened: $Storage = $crate::int::types::traits::wide_cast(value as i128);
                Self(widened * Self::multiplier())
            }

            /// Constructs from an `i32`, scaling by `10^SCALE`.
            #[inline]
            pub fn from_i32(value: i32) -> Self {
                let widened: $Storage = $crate::int::types::traits::wide_cast(value as i128);
                Self(widened * Self::multiplier())
            }

            /// Converts to `i64` using the crate default rounding mode.
            /// Saturates to `i64::MAX` / `i64::MIN` when the rounded
            /// integer part falls outside `i64`'s range.
            #[inline]
            pub fn to_int(self) -> i64 {
                self.to_int_with($crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Converts to `i64` using the supplied rounding mode for the
            /// fractional discard step. Saturates to `i64::MAX` /
            /// `i64::MIN` when the rounded integer is out of `i64` range.
            #[inline]
            pub fn to_int_with(self, mode: $crate::support::rounding::RoundingMode) -> i64 {
                let zero = <$Storage>::from_str_radix("0", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let one = <$Storage>::from_str_radix("1", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let raw = self.0;
                let divisor = Self::multiplier();
                let quotient = raw / divisor;
                let remainder = raw % divisor;
                let int_rounded: $Storage = if remainder == zero {
                    quotient
                } else {
                    let abs_rem = remainder.unsigned_abs();
                    // `divisor` is `10^SCALE` and always positive, so
                    // `unsigned_abs()` is the value itself; `>> 1` is
                    // the half-LSB threshold.
                    let half = divisor.unsigned_abs() >> 1;
                    let non_negative = !raw.is_negative();
                    match mode {
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
                    }
                };
                let i64_max: $Storage = $crate::int::types::traits::wide_cast(i64::MAX);
                let i64_min: $Storage = $crate::int::types::traits::wide_cast(i64::MIN);
                if int_rounded > i64_max {
                    i64::MAX
                } else if int_rounded < i64_min {
                    i64::MIN
                } else {
                    $crate::int::types::traits::wide_cast::<_, i64>(int_rounded)
                }
            }
        }
    };

}

pub(crate) use decl_decimal_int_conversion_methods;
