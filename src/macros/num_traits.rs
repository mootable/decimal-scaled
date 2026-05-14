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
                self.0 == 0
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
            type FromStrRadixErr = $crate::core_type::ParseD128Error;
            fn from_str_radix(
                s: &str,
                radix: u32,
            ) -> ::core::result::Result<Self, Self::FromStrRadixErr> {
                if radix != 10 {
                    return ::core::result::Result::Err(
                        $crate::core_type::ParseD128Error::InvalidChar,
                    );
                }
                <Self as ::core::str::FromStr>::from_str(s)
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
