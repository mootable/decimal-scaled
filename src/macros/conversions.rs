//! Macro-generated `From` impls for primitive integer inputs.
//!
//! Each impl multiplies the input by `multiplier()` (= `10^SCALE`) and
//! stores the result in the width's native integer storage. Overflow
//! follows Rust's default integer arithmetic: debug-mode panic,
//! release-mode wrap. Users with overflow risk should reach for the
//! eventual `TryFrom` variants (Phase 3D pending).

/// Generates `From<$Src> for $Type<SCALE>` that scales the value by
/// `10^SCALE` and stores it in `$Storage`. The cast `value as $Storage`
/// happens first when the source is narrower than the storage, which
/// is the lossless case; the subsequent multiply is the overflow risk.
macro_rules! decl_from_primitive {
    // Wide storage: the primitive widens into wide
    // storage via the `WideInt` cast, then scales by `10^SCALE`.
    (wide $Type:ident, $Storage:ty, $Src:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src> for $Type<SCALE> {
            /// Constructs from an integer by scaling to `value * 10^SCALE`.
            /// Overflows follow the wide integer's default arithmetic semantics
            /// (debug-mode panic, release-mode wrap).
            #[inline]
            fn from(value: $Src) -> Self {
                let widened: $Storage = $crate::wide_int::wide_cast(value as i128);
                Self(widened * Self::multiplier())
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty, $Src:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src> for $Type<SCALE> {
            /// Constructs from an integer by scaling to `value * 10^SCALE`.
            /// Overflows follow Rust's default integer arithmetic semantics
            /// (debug-mode panic, release-mode wrap).
            #[inline]
            fn from(value: $Src) -> Self {
                Self((value as $Storage) * Self::multiplier())
            }
        }
    };
}

pub(crate) use decl_from_primitive;

/// Generates `From<$Src<SCALE>> for $Dest<SCALE>` for a lossless
/// widening conversion (e.g. D9 -> D18, D18 -> D38). `$SrcStorage`
/// must widen losslessly to `$DestStorage` via an `as` cast.
macro_rules! decl_cross_width_widening {
    // Widening *into* wide storage. The source storage is
    // a primitive integer or a narrower wide integer; either way
    // the `WideInt` cast performs the lossless widen.
    (wide $Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src<SCALE>> for $Dest<SCALE> {
            /// Widens a narrower decimal type to this wider one. The
            /// scale is unchanged; the storage is widened via
            /// the `WideInt` cast (lossless because the source domain is
            /// a subset of the destination).
            #[inline]
            fn from(value: $Src<SCALE>) -> Self {
                Self($crate::wide_int::wide_cast(value.to_bits()))
            }
        }
    };

    // Native-to-native widening.
    ($Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src<SCALE>> for $Dest<SCALE> {
            /// Widens a narrower decimal type to this wider one. The
            /// scale is unchanged and the storage is widened by an
            /// `as` cast (lossless because the source storage type is
            /// strictly narrower than the destination).
            #[inline]
            fn from(value: $Src<SCALE>) -> Self {
                Self(value.to_bits() as $DestStorage)
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
    // integer (e.g. D153 -> D76); the `WideInt` cast handles the bound
    // widening and the final narrowing cast in both cases.
    (wide $Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<$Src<SCALE>> for $Dest<SCALE> {
            type Error = $crate::error::ConvertError;
            /// Attempts to narrow a wider decimal type to this narrower
            /// one. Fails with `Overflow` when the source value exceeds
            /// the destination's `MIN..=MAX`. The scale is unchanged.
            #[inline]
            fn try_from(value: $Src<SCALE>) -> ::core::result::Result<Self, Self::Error> {
                let bits = value.to_bits();
                let dest_max: $SrcStorage = $crate::wide_int::wide_cast(<$DestStorage>::MAX);
                let dest_min: $SrcStorage = $crate::wide_int::wide_cast(<$DestStorage>::MIN);
                if bits > dest_max || bits < dest_min {
                    return ::core::result::Result::Err(
                        $crate::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self($crate::wide_int::wide_cast(bits)))
            }
        }
    };

    // Native-to-native narrowing.
    ($Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<$Src<SCALE>> for $Dest<SCALE> {
            type Error = $crate::error::ConvertError;
            /// Attempts to narrow a wider decimal type to this narrower
            /// one. Fails with `OutOfRange` when the source value
            /// exceeds the destination's `MIN..=MAX`. The scale is
            /// unchanged.
            #[inline]
            fn try_from(value: $Src<SCALE>) -> ::core::result::Result<Self, Self::Error> {
                let bits = value.to_bits();
                if bits > (<$DestStorage>::MAX as $SrcStorage)
                    || bits < (<$DestStorage>::MIN as $SrcStorage)
                {
                    return ::core::result::Result::Err(
                        $crate::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self(bits as $DestStorage))
            }
        }
    };
}

pub(crate) use decl_cross_width_narrowing;

/// Emits `TryFrom<i128> for $Type<SCALE>` returning
/// `Result<Self, ConvertError::Overflow>` after `checked_mul` by the
/// multiplier and a narrowing range-check against `$Storage`.
macro_rules! decl_try_from_i128 {
    // Wide storage. `i128` always widens losslessly into
    // the storage; the only failure mode is the `checked_mul` by the
    // multiplier overflowing the (still finite) wide storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<i128> for $Type<SCALE> {
            type Error = $crate::error::ConvertError;
            #[inline]
            fn try_from(value: i128) -> ::core::result::Result<Self, Self::Error> {
                let widened: $Storage = $crate::wide_int::wide_cast(value);
                let scaled = widened
                    .checked_mul(Self::multiplier())
                    .ok_or($crate::error::ConvertError::Overflow)?;
                ::core::result::Result::Ok(Self(scaled))
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<i128> for $Type<SCALE> {
            type Error = $crate::error::ConvertError;
            #[inline]
            fn try_from(value: i128) -> ::core::result::Result<Self, Self::Error> {
                let m: i128 = Self::multiplier() as i128;
                let scaled = value
                    .checked_mul(m)
                    .ok_or($crate::error::ConvertError::Overflow)?;
                if scaled > <$Storage>::MAX as i128 || scaled < <$Storage>::MIN as i128 {
                    return ::core::result::Result::Err(
                        $crate::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self(scaled as $Storage))
            }
        }
    };
}

pub(crate) use decl_try_from_i128;

/// Emits `TryFrom<u128> for $Type<SCALE>`. First narrows u128 to i128
/// (rejecting if the value exceeds i128::MAX) then delegates to the
/// i128 path.
macro_rules! decl_try_from_u128 {
    // Wide storage. `u128` always widens losslessly into
    // the (256-bit-or-wider) signed storage — including values above
    // `i128::MAX`, which the native path would have to reject — so the
    // only failure mode is the `checked_mul` overflowing the storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<u128> for $Type<SCALE> {
            type Error = $crate::error::ConvertError;
            #[inline]
            fn try_from(value: u128) -> ::core::result::Result<Self, Self::Error> {
                let widened: $Storage = <$Storage>::from_u128(value);
                let scaled = widened
                    .checked_mul(Self::multiplier())
                    .ok_or($crate::error::ConvertError::Overflow)?;
                ::core::result::Result::Ok(Self(scaled))
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<u128> for $Type<SCALE> {
            type Error = $crate::error::ConvertError;
            #[inline]
            fn try_from(value: u128) -> ::core::result::Result<Self, Self::Error> {
                let as_i128: i128 = i128::try_from(value)
                    .map_err(|_| $crate::error::ConvertError::Overflow)?;
                <Self as ::core::convert::TryFrom<i128>>::try_from(as_i128)
            }
        }
    };
}

pub(crate) use decl_try_from_u128;

/// Emits `TryFrom<f64> for $Type<SCALE>`. NaN / ±inf return
/// `NotFinite`; finite values whose scaled magnitude exceeds the
/// storage range return `Overflow`. Truncates toward zero (matches the
/// historical D38 behaviour). For rounding-mode-aware float
/// construction, use `from_f64_fast_with`.
macro_rules! decl_try_from_f64 {
    // Wide storage. The multiplier and storage bounds
    // round-trip through `f64` via the `WideInt` cast; the final
    // `f64` -> wide cast is also the `WideInt` cast.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<f64> for $Type<SCALE> {
            type Error = $crate::error::ConvertError;
            #[inline]
            fn try_from(value: f64) -> ::core::result::Result<Self, Self::Error> {
                if !value.is_finite() {
                    return ::core::result::Result::Err(
                        $crate::error::ConvertError::NotFinite,
                    );
                }
                let mult_f64: f64 = Self::multiplier().as_f64();
                let scaled = value * mult_f64;
                let storage_max_f64: f64 = <$Storage>::MAX.as_f64();
                let storage_min_f64: f64 = <$Storage>::MIN.as_f64();
                if !(storage_min_f64..storage_max_f64).contains(&scaled) {
                    return ::core::result::Result::Err(
                        $crate::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self(<$Storage>::from_f64(scaled)))
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<f64> for $Type<SCALE> {
            type Error = $crate::error::ConvertError;
            #[inline]
            fn try_from(value: f64) -> ::core::result::Result<Self, Self::Error> {
                if !value.is_finite() {
                    return ::core::result::Result::Err(
                        $crate::error::ConvertError::NotFinite,
                    );
                }
                let scaled = value * (Self::multiplier() as f64);
                let storage_max_f64 = <$Storage>::MAX as f64;
                let storage_min_f64 = <$Storage>::MIN as f64;
                if !(storage_min_f64..storage_max_f64).contains(&scaled) {
                    return ::core::result::Result::Err(
                        $crate::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self(scaled as $Storage))
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
    (wide $Type:ident, $Storage:ty) => {
        $crate::macros::conversions::decl_try_from_f32!($Type, $Storage);
    };
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<f32> for $Type<SCALE> {
            type Error = $crate::error::ConvertError;
            #[inline]
            fn try_from(value: f32) -> ::core::result::Result<Self, Self::Error> {
                <Self as ::core::convert::TryFrom<f64>>::try_from(value as f64)
            }
        }
    };
}

pub(crate) use decl_try_from_f32;
pub(crate) use decl_try_from_f64;

/// Emits the named integer constructors and `to_int_lossy` /
/// `to_int_lossy_with` on a decimal type. `$Storage` is the storage
/// integer; `$IntSrc` is the wider integer source for `from_int`
/// (typically `i64` for D18/D38 and `i32` for D9). `from_int` and
/// `from_i32` scale directly (they do not depend on a `From<iN>` impl
/// existing for the width).
macro_rules! decl_decimal_int_conversion_methods {
    // Wide storage. The rounding logic mirrors the native
    // arm but is carried in the wide storage type throughout; the
    // `i128` source widens via the `WideInt` cast, and the final
    // saturating narrow to `i64` also goes through the `WideInt` cast.
    (wide $Type:ident, $Storage:ty, $IntSrc:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an integer source, scaling by `10^SCALE`.
            /// Overflow follows the wide integer's default arithmetic semantics.
            #[inline]
            pub fn from_int(value: $IntSrc) -> Self {
                let widened: $Storage = $crate::wide_int::wide_cast(value as i128);
                Self(widened * Self::multiplier())
            }

            /// Constructs from an `i32`, scaling by `10^SCALE`.
            #[inline]
            pub fn from_i32(value: i32) -> Self {
                let widened: $Storage = $crate::wide_int::wide_cast(value as i128);
                Self(widened * Self::multiplier())
            }

            /// Converts to `i64` using the crate default rounding mode.
            /// Saturates to `i64::MAX` / `i64::MIN` when the rounded
            /// integer part falls outside `i64`'s range.
            #[inline]
            pub fn to_int_lossy(self) -> i64 {
                self.to_int_lossy_with($crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Converts to `i64` using the supplied rounding mode for the
            /// fractional discard step. Saturates to `i64::MAX` /
            /// `i64::MIN` when the rounded integer is out of `i64` range.
            #[inline]
            pub fn to_int_lossy_with(
                self,
                mode: $crate::rounding::RoundingMode,
            ) -> i64 {
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
                        $crate::rounding::RoundingMode::HalfToEven => {
                            if abs_rem < half {
                                quotient
                            } else if abs_rem > half {
                                if non_negative { quotient + one } else { quotient - one }
                            } else if !quotient.bit(0) {
                                quotient
                            } else if non_negative {
                                quotient + one
                            } else {
                                quotient - one
                            }
                        }
                        $crate::rounding::RoundingMode::HalfAwayFromZero => {
                            if abs_rem < half {
                                quotient
                            } else if non_negative {
                                quotient + one
                            } else {
                                quotient - one
                            }
                        }
                        $crate::rounding::RoundingMode::HalfTowardZero => {
                            if abs_rem > half {
                                if non_negative { quotient + one } else { quotient - one }
                            } else {
                                quotient
                            }
                        }
                        $crate::rounding::RoundingMode::Trunc => quotient,
                        $crate::rounding::RoundingMode::Floor => {
                            if non_negative { quotient } else { quotient - one }
                        }
                        $crate::rounding::RoundingMode::Ceiling => {
                            if non_negative { quotient + one } else { quotient }
                        }
                    }
                };
                let i64_max: $Storage = $crate::wide_int::wide_cast(i64::MAX);
                let i64_min: $Storage = $crate::wide_int::wide_cast(i64::MIN);
                if int_rounded > i64_max {
                    i64::MAX
                } else if int_rounded < i64_min {
                    i64::MIN
                } else {
                    $crate::wide_int::wide_cast::<_, i64>(int_rounded)
                }
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty, $IntSrc:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an integer at the widest supported source,
            /// scaling by `10^SCALE`. Overflow follows Rust's default
            /// integer arithmetic (debug panic, release wrap).
            #[inline]
            pub fn from_int(value: $IntSrc) -> Self {
                Self((value as $Storage) * Self::multiplier())
            }

            /// Constructs from an `i32`, scaling by `10^SCALE`.
            #[inline]
            pub fn from_i32(value: i32) -> Self {
                Self((value as $Storage) * Self::multiplier())
            }

            /// Converts to `i64` using the crate default rounding mode.
            /// Saturates to `i64::MAX` / `i64::MIN` when the integer part
            /// of the rounded value falls outside `i64`'s range.
            #[inline]
            pub fn to_int_lossy(self) -> i64 {
                self.to_int_lossy_with($crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Converts to `i64` using the supplied rounding mode for the
            /// fractional discard step. Saturates to `i64::MAX` /
            /// `i64::MIN` when the rounded integer is out of `i64` range.
            #[inline]
            pub fn to_int_lossy_with(
                self,
                mode: $crate::rounding::RoundingMode,
            ) -> i64 {
                let raw = self.0 as i128;
                let divisor = Self::multiplier() as i128;
                let quotient = raw / divisor;
                let remainder = raw % divisor;
                let int_rounded: i128 = if remainder == 0 {
                    quotient
                } else {
                    let abs_rem = remainder.unsigned_abs();
                    let half = (divisor / 2) as u128;
                    match mode {
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
                    }
                };
                if int_rounded > i64::MAX as i128 {
                    i64::MAX
                } else if int_rounded < i64::MIN as i128 {
                    i64::MIN
                } else {
                    int_rounded as i64
                }
            }
        }
    };
}

pub(crate) use decl_decimal_int_conversion_methods;
