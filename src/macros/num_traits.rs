//! Macro-generated `num_traits` impls for narrow decimal widths.
//!
//! Covers the foundational traits a generic numeric helper most often
//! needs: `Zero`, `One`, `Bounded`. The richer surfaces (`Signed`,
//! `Num`, `FromPrimitive`, `ToPrimitive`, `Checked*`) follow in later
//! sub-phases because each pulls in additional inherent methods on
//! the target type.

/// Emits `num_traits::Zero`, `num_traits::One`, and
/// `num_traits::Bounded` for a decimal type.
macro_rules! decl_decimal_num_traits_basics {
    ($Type:ident) => {
        impl<const SCALE: u32> ::num_traits::Zero for $Type<SCALE> {
            #[inline]
            fn zero() -> Self {
                Self::ZERO
            }
            #[inline]
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }

        impl<const SCALE: u32> ::num_traits::One for $Type<SCALE> {
            #[inline]
            fn one() -> Self {
                Self::ONE
            }
            #[inline]
            fn is_one(&self) -> bool {
                self.0 == Self::multiplier()
            }
        }

        impl<const SCALE: u32> ::num_traits::Bounded for $Type<SCALE> {
            #[inline]
            fn min_value() -> Self {
                Self::MIN
            }
            #[inline]
            fn max_value() -> Self {
                Self::MAX
            }
        }

        impl<const SCALE: u32> ::num_traits::Num for $Type<SCALE> {
            type FromStrRadixErr = $crate::core_type::ParseError;
            fn from_str_radix(
                s: &str,
                radix: u32,
            ) -> ::core::result::Result<Self, Self::FromStrRadixErr> {
                if radix != 10 {
                    return ::core::result::Result::Err(
                        $crate::core_type::ParseError::InvalidChar,
                    );
                }
                <Self as ::core::str::FromStr>::from_str(s)
            }
        }

        impl<const SCALE: u32> ::num_traits::CheckedAdd for $Type<SCALE> {
            #[inline]
            fn checked_add(&self, rhs: &Self) -> Option<Self> {
                <$Type<SCALE>>::checked_add(*self, *rhs)
            }
        }

        impl<const SCALE: u32> ::num_traits::CheckedSub for $Type<SCALE> {
            #[inline]
            fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                <$Type<SCALE>>::checked_sub(*self, *rhs)
            }
        }

        impl<const SCALE: u32> ::num_traits::CheckedMul for $Type<SCALE> {
            #[inline]
            fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                <$Type<SCALE>>::checked_mul(*self, *rhs)
            }
        }

        impl<const SCALE: u32> ::num_traits::CheckedDiv for $Type<SCALE> {
            #[inline]
            fn checked_div(&self, rhs: &Self) -> Option<Self> {
                <$Type<SCALE>>::checked_div(*self, *rhs)
            }
        }

        impl<const SCALE: u32> ::num_traits::CheckedRem for $Type<SCALE> {
            #[inline]
            fn checked_rem(&self, rhs: &Self) -> Option<Self> {
                <$Type<SCALE>>::checked_rem(*self, *rhs)
            }
        }

        impl<const SCALE: u32> ::num_traits::CheckedNeg for $Type<SCALE> {
            #[inline]
            fn checked_neg(&self) -> Option<Self> {
                <$Type<SCALE>>::checked_neg(*self)
            }
        }

        impl<const SCALE: u32> ::num_traits::Signed for $Type<SCALE> {
            #[inline]
            fn abs(&self) -> Self {
                <$Type<SCALE>>::abs(*self)
            }
            #[inline]
            fn abs_sub(&self, other: &Self) -> Self {
                if self <= other {
                    Self::ZERO
                } else {
                    *self - *other
                }
            }
            #[inline]
            fn signum(&self) -> Self {
                <$Type<SCALE>>::signum(*self)
            }
            #[inline]
            fn is_positive(&self) -> bool {
                <$Type<SCALE>>::is_positive(*self)
            }
            #[inline]
            fn is_negative(&self) -> bool {
                <$Type<SCALE>>::is_negative(*self)
            }
        }
    };
}

pub(crate) use decl_decimal_num_traits_basics;

/// Emits `num_traits::FromPrimitive`, `ToPrimitive`, and `NumCast` for
/// a decimal type.
///
/// - `decl_decimal_num_traits_conversions!(D38, i128)` — native
/// storage. `from_i64` / `from_u64` scale via an `as`-cast and
/// `checked_mul`; the `to_*` integer methods divide the raw storage
/// by `10^SCALE` and narrow with `TryFrom`.
/// - `decl_decimal_num_traits_conversions!(wide D76, I256)` — wide
/// storage. `from_i64` / `from_u64` widen via the `WideInt` cast; the
/// `to_*` methods divide the wide storage and narrow with the
/// wide-to-primitive `TryFrom` impls.
///
/// `from_i128` / `from_u128` / `from_f32` / `from_f64` delegate to the
/// width's `TryFrom` impls in both arms, and `NumCast` is fully
/// storage-agnostic, so those parts are shared via the `@numcast` arm.
macro_rules! decl_decimal_num_traits_conversions {
    // Wide storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::num_traits::FromPrimitive for $Type<SCALE> {
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                let widened: $Storage = <$Storage>::from_i128(n as i128);
                widened.checked_mul(Self::multiplier()).map(Self)
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                let widened: $Storage = <$Storage>::from_u128(n as u128);
                widened.checked_mul(Self::multiplier()).map(Self)
            }
            #[inline]
            fn from_i128(n: i128) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<i128>>::try_from(n).ok()
            }
            #[inline]
            fn from_u128(n: u128) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<u128>>::try_from(n).ok()
            }
            #[inline]
            fn from_f32(n: f32) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<f32>>::try_from(n).ok()
            }
            #[inline]
            fn from_f64(n: f64) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<f64>>::try_from(n).ok()
            }
        }

        impl<const SCALE: u32> ::num_traits::ToPrimitive for $Type<SCALE> {
            #[inline]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                (self.0 / Self::multiplier())
                    .to_i128_checked()
                    .and_then(|v| i64::try_from(v).ok())
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                (self.0 / Self::multiplier())
                    .to_u128_checked()
                    .and_then(|v| u64::try_from(v).ok())
            }
            #[inline]
            fn to_i128(&self) -> ::core::option::Option<i128> {
                (self.0 / Self::multiplier()).to_i128_checked()
            }
            #[inline]
            fn to_u128(&self) -> ::core::option::Option<u128> {
                (self.0 / Self::multiplier()).to_u128_checked()
            }
            #[inline]
            fn to_f32(&self) -> ::core::option::Option<f32> {
                ::core::option::Option::Some((*self).to_f32_fast())
            }
            #[inline]
            fn to_f64(&self) -> ::core::option::Option<f64> {
                ::core::option::Option::Some((*self).to_f64_fast())
            }
        }

        $crate::macros::num_traits::decl_decimal_num_traits_conversions!(@numcast $Type);
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::num_traits::FromPrimitive for $Type<SCALE> {
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                (n as $Storage).checked_mul(Self::multiplier()).map(Self)
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                // u64 may exceed the storage's positive range; widen
                // through i128, then narrow via the type's
                // range-checking `TryFrom<i128>`.
                <Self as ::core::convert::TryFrom<i128>>::try_from(n as i128).ok()
            }
            #[inline]
            fn from_i128(n: i128) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<i128>>::try_from(n).ok()
            }
            #[inline]
            fn from_u128(n: u128) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<u128>>::try_from(n).ok()
            }
            #[inline]
            fn from_f32(n: f32) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<f32>>::try_from(n).ok()
            }
            #[inline]
            fn from_f64(n: f64) -> ::core::option::Option<Self> {
                <Self as ::core::convert::TryFrom<f64>>::try_from(n).ok()
            }
        }

        impl<const SCALE: u32> ::num_traits::ToPrimitive for $Type<SCALE> {
            #[inline]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                i64::try_from(self.0 / Self::multiplier()).ok()
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                if self.0 < 0 {
                    return ::core::option::Option::None;
                }
                u64::try_from(self.0 / Self::multiplier()).ok()
            }
            #[inline]
            fn to_i128(&self) -> ::core::option::Option<i128> {
                i128::try_from(self.0 / Self::multiplier()).ok()
            }
            #[inline]
            fn to_u128(&self) -> ::core::option::Option<u128> {
                if self.0 < 0 {
                    return ::core::option::Option::None;
                }
                u128::try_from(self.0 / Self::multiplier()).ok()
            }
            #[inline]
            fn to_f32(&self) -> ::core::option::Option<f32> {
                ::core::option::Option::Some((*self).to_f32_fast())
            }
            #[inline]
            fn to_f64(&self) -> ::core::option::Option<f64> {
                ::core::option::Option::Some((*self).to_f64_fast())
            }
        }

        $crate::macros::num_traits::decl_decimal_num_traits_conversions!(@numcast $Type);
    };

    // Shared `NumCast` — fully storage-agnostic (dispatches through the
    // `ToPrimitive` / `FromPrimitive` trait methods only).
    (@numcast $Type:ident) => {
        impl<const SCALE: u32> ::num_traits::NumCast for $Type<SCALE> {
            #[inline]
            fn from<T: ::num_traits::ToPrimitive>(n: T) -> ::core::option::Option<Self> {
                use ::num_traits::FromPrimitive;
                // Read f64 early to distinguish integer vs fractional inputs.
                let f = n.to_f64();
                // Integer fast path: if `n` round-trips through i128 and
                // the f64 value matches, the input is integer-shaped —
                // take the lossless integer path (preserves precision for
                // i64/u64 values beyond f64's 2^53 exact-integer range).
                if let ::core::option::Option::Some(int) = n.to_i128() {
                    let take_int_path = match f {
                        ::core::option::Option::None => true,
                        ::core::option::Option::Some(fv) => {
                            fv.is_finite() && ((int as f64) == fv)
                        }
                    };
                    if take_int_path {
                        return <Self as FromPrimitive>::from_i128(int);
                    }
                }
                // Float path — preserves fractional information; `None`
                // for NaN / infinity / out-of-range.
                if let ::core::option::Option::Some(fv) = f {
                    return <Self as FromPrimitive>::from_f64(fv);
                }
                ::core::option::Option::None
            }
        }
    };
}

pub(crate) use decl_decimal_num_traits_conversions;
