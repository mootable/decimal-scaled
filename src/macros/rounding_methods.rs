// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Macro-generated rounding-related methods (floor, ceil, round,
//! trunc, fract) for decimal types.
//!
//! `floor` / `ceil` / `trunc` / `fract` are storage-agnostic — they
//! only use `div_euclid`, multiply, subtract and negate, which both
//! primitive integers and the wide integers support with identical
//! spelling — so they live in a shared `@common` arm. Only `round`
//! differs: it needs the half-LSB threshold and a sign test, which a
//! a wide integer cannot express against the `0` / `2` literals, so it
//! is written inline per front-end arm.

/// Emits `floor`, `ceil`, `round` (half-away-from-zero), `trunc`,
/// `fract` for `$Type<SCALE>`.
macro_rules! decl_decimal_rounding_methods {
    // Wide storage.
    (wide $Type:ident) => {
        $crate::macros::rounding_methods::decl_decimal_rounding_methods!(@common $Type);

        impl<const SCALE: u32> $Type<SCALE> {
            /// Round to the nearest integer (half-away-from-zero).
            #[inline]
            #[must_use]
            pub fn round(self) -> Self {
                let m = Self::multiplier();
                // `m` (= 10^SCALE) is positive, so `>> 1` is `m / 2`.
                let half = m >> 1u32;
                let bias = if self.0.is_negative() { -half } else { half };
                Self((self.0 + bias) / m * m)
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident) => {
        $crate::macros::rounding_methods::decl_decimal_rounding_methods!(@common $Type);

        impl<const SCALE: u32> $Type<SCALE> {
            /// Round to the nearest integer (half-away-from-zero).
            #[inline]
            #[must_use]
            pub fn round(self) -> Self {
                let m = Self::multiplier();
                let half = m / 2;
                let bias = if self.0 >= 0 { half } else { -half };
                Self((self.0 + bias) / m * m)
            }
        }
    };

    // Shared: floor / ceil / trunc / fract.
    (@common $Type:ident) => {
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
