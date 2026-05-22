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
            type FromStrRadixErr = $crate::types::widths::ParseError;
            fn from_str_radix(
                s: &str,
                radix: u32,
            ) -> ::core::result::Result<Self, Self::FromStrRadixErr> {
                if radix != 10 {
                    return ::core::result::Result::Err(
                        $crate::types::widths::ParseError::InvalidChar,
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

        // Saturating, never-panicking `from_num` / `to_num` bridge
        // pair — single `NumCast`-style entry point that any width's
        // `T: ToPrimitive` / `T: NumCast + Bounded` caller can use
        // without dispatching on storage width.
        //
        // Float NaN maps to ZERO; +/-Infinity and finite-out-of-range
        // saturate to MAX / MIN by sign of the source.
        //
        // Requires the type's `::num_traits::NumCast` impl to be
        // available, which `decl_decimal_num_traits_conversions!`
        // emits for every width.
        impl<const SCALE: u32> $Type<SCALE> {
            /// Saturating `T → Self` via [`num_traits::NumCast`].
            /// Out-of-range / `±Infinity` saturate to `MAX` / `MIN`;
            /// `NaN` maps to [`Self::ZERO`]. See the module-level docs.
            #[must_use]
            pub fn from_num<T: ::num_traits::ToPrimitive>(value: T) -> Self {
                let int_signal = value.to_i128();
                let uint_signal = value.to_u128();
                let float_signal = if int_signal.is_none() && uint_signal.is_none() {
                    value.to_f64()
                } else {
                    None
                };
                if let Some(f) = float_signal
                    && f.is_nan()
                {
                    return Self::ZERO;
                }
                if let Some(d) = <Self as ::num_traits::NumCast>::from(value) {
                    return d;
                }
                if let Some(i) = int_signal {
                    return if i < 0 { Self::MIN } else { Self::MAX };
                }
                if uint_signal.is_some() {
                    return Self::MAX;
                }
                match float_signal {
                    Some(f) if f.is_sign_negative() => Self::MIN,
                    Some(_) => Self::MAX,
                    None => Self::ZERO,
                }
            }

            /// Saturating `Self → T` via [`num_traits::NumCast`].
            /// Out-of-range targets saturate to `T::max_value()` /
            /// `T::min_value()`. Never panics.
            #[must_use]
            pub fn to_num<T: ::num_traits::NumCast + ::num_traits::Bounded>(self) -> T {
                match T::from(self) {
                    ::core::option::Option::Some(t) => t,
                    ::core::option::Option::None => {
                        if self >= Self::ZERO {
                            T::max_value()
                        } else {
                            T::min_value()
                        }
                    }
                }
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
/// storage. `from_i64` / `from_u64` widen via the `BigInt` cast; the
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
                // `u64` can exceed a narrow wide-storage's positive range
                // (e.g. `Int<1>` / D18), so route through the range-checking
                // `TryFrom<u128>` rather than an unchecked widen.
                <Self as ::core::convert::TryFrom<u128>>::try_from(n as u128).ok()
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
                // A negative value has no `u64` representation. Guard the sign
                // before the truncating divide, which would otherwise map a
                // small negative (e.g. -1 LSB) to integer-part `0`.
                if self.0.is_negative() {
                    return ::core::option::Option::None;
                }
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
                // Negative values have no `u128` representation; guard the
                // sign before the truncating divide (which maps small
                // negatives to integer-part `0`).
                if self.0.is_negative() {
                    return ::core::option::Option::None;
                }
                (self.0 / Self::multiplier()).to_u128_checked()
            }
            #[inline]
            fn to_f32(&self) -> ::core::option::Option<f32> {
                ::core::option::Option::Some((*self).to_f32())
            }
            #[inline]
            fn to_f64(&self) -> ::core::option::Option<f64> {
                ::core::option::Option::Some((*self).to_f64())
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
