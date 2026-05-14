//! Macro-generated rounding-related methods (floor, ceil, round,
//! trunc, fract) for decimal types.
//!
//! All five methods work uniformly on integer storage by exploiting
//! integer division and `div_euclid` semantics. No widening or
//! float arithmetic is needed.

/// Emits `floor`, `ceil`, `round` (half-away-from-zero), `trunc`,
/// `fract` for `$Type<SCALE>`. Storage type is inferred via
/// `Self::multiplier()`.
macro_rules! decl_decimal_rounding_methods {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Largest integer multiple of `ONE` less than or equal to
            /// `self` (toward negative infinity).
            #[inline]
            #[must_use]
            pub fn floor(self) -> Self {
                let m = Self::multiplier();
                Self(self.0.div_euclid(m) * m)
            }

            /// Smallest integer multiple of `ONE` greater than or equal
            /// to `self` (toward positive infinity).
            #[inline]
            #[must_use]
            pub fn ceil(self) -> Self {
                let m = Self::multiplier();
                Self(-((-self.0).div_euclid(m)) * m)
            }

            /// Round to the nearest integer (half-away-from-zero).
            #[inline]
            #[must_use]
            pub fn round(self) -> Self {
                let m = Self::multiplier();
                let half = m / 2;
                let bias = if self.0 >= 0 { half } else { -half };
                Self((self.0 + bias) / m * m)
            }

            /// Drop the fractional part (toward zero).
            #[inline]
            #[must_use]
            pub fn trunc(self) -> Self {
                let m = Self::multiplier();
                Self(self.0 / m * m)
            }

            /// Return only the fractional part: `self - self.trunc()`.
            #[inline]
            #[must_use]
            pub fn fract(self) -> Self {
                let m = Self::multiplier();
                Self(self.0 - (self.0 / m * m))
            }
        }
    };
}

pub(crate) use decl_decimal_rounding_methods;
