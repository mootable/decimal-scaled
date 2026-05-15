//! Macro-generated integer-flavoured methods shared by every decimal
//! width: Euclidean division / remainder, floor / ceil division,
//! absolute difference, midpoint, and the `is_zero` predicate.
//!
//! `div_euclid` / `rem_euclid` / `abs_diff` / `midpoint` are spelled
//! identically for primitive-integer and wide storage and live in a
//! shared `@common` arm. `div_floor` / `div_ceil` / `is_zero` compare
//! against the integer literal `0`, which a wide-integer value cannot, so
//! they are written per front-end arm.

/// Emits `div_euclid`, `rem_euclid`, `div_floor`, `div_ceil`,
/// `abs_diff`, `midpoint`, and `is_zero` for a decimal type.
///
/// - `decl_decimal_int_methods!(D38, i128)` — *native* storage.
/// - `decl_decimal_int_methods!(wide D76, I256)` — *wide* storage.
macro_rules! decl_decimal_int_methods {
    // Wide storage.
    (wide $Type:ident, $Storage:ty) => {
        $crate::macros::int_methods::decl_decimal_int_methods!(@common $Type, $Storage);

        impl<const SCALE: u32> $Type<SCALE> {
            /// Floor-rounded division: `floor(self / rhs)` as an integer
            /// multiple of `ONE`. Panics on `rhs == ZERO`.
            #[inline]
            #[must_use]
            pub fn div_floor(self, rhs: Self) -> Self {
                let q = self.0 / rhs.0;
                let r = self.0 % rhs.0;
                let zero = <$Storage>::from_str_radix("0", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let one = <$Storage>::from_str_radix("1", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let raw = if r != zero && (r ^ rhs.0).is_negative() {
                    q - one
                } else {
                    q
                };
                Self(raw * Self::multiplier())
            }

            /// Ceil-rounded division: `ceil(self / rhs)` as an integer
            /// multiple of `ONE`. Panics on `rhs == ZERO`.
            #[inline]
            #[must_use]
            pub fn div_ceil(self, rhs: Self) -> Self {
                let q = self.0 / rhs.0;
                let r = self.0 % rhs.0;
                let zero = <$Storage>::from_str_radix("0", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let one = <$Storage>::from_str_radix("1", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let raw = if r != zero && !(r ^ rhs.0).is_negative() {
                    q + one
                } else {
                    q
                };
                Self(raw * Self::multiplier())
            }

            /// `true` if `self` is the additive identity.
            #[inline]
            #[must_use]
            pub fn is_zero(self) -> bool {
                self == Self::ZERO
            }

            /// Returns `true` for any non-zero value. A fixed-point
            /// decimal has no subnormals, so zero is the only value that
            /// is not "normal".
            #[inline]
            #[must_use]
            pub fn is_normal(self) -> bool {
                self != Self::ZERO
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        $crate::macros::int_methods::decl_decimal_int_methods!(@common $Type, $Storage);

        impl<const SCALE: u32> $Type<SCALE> {
            /// Floor-rounded division: `floor(self / rhs)` as an integer
            /// multiple of `ONE`. Panics on `rhs == ZERO`.
            ///
            /// Inlined rather than using `i128::div_floor`, which is
            /// still unstable for signed types.
            #[inline]
            #[must_use]
            pub fn div_floor(self, rhs: Self) -> Self {
                let q = self.0 / rhs.0;
                let r = self.0 % rhs.0;
                // XOR of sign bits detects a remainder/divisor sign mismatch.
                let raw = if r != 0 && (r ^ rhs.0) < 0 { q - 1 } else { q };
                Self(raw * Self::multiplier())
            }

            /// Ceil-rounded division: `ceil(self / rhs)` as an integer
            /// multiple of `ONE`. Panics on `rhs == ZERO`.
            #[inline]
            #[must_use]
            pub fn div_ceil(self, rhs: Self) -> Self {
                let q = self.0 / rhs.0;
                let r = self.0 % rhs.0;
                let raw = if r != 0 && (r ^ rhs.0) >= 0 { q + 1 } else { q };
                Self(raw * Self::multiplier())
            }

            /// `true` if `self` is the additive identity.
            #[inline]
            #[must_use]
            pub const fn is_zero(self) -> bool {
                self.0 == 0
            }

            /// Returns `true` for any non-zero value. A fixed-point
            /// decimal has no subnormals, so zero is the only value that
            /// is not "normal".
            #[inline]
            #[must_use]
            pub const fn is_normal(self) -> bool {
                self.0 != 0
            }
        }
    };

    // Shared: div_euclid / rem_euclid / abs_diff / midpoint, plus the
    // float-shape predicates that are constant for a fixed-point type.
    (@common $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Euclidean division: the quotient as an integer multiple of
            /// `ONE`, chosen so the remainder is non-negative. Panics on
            /// `rhs == ZERO`.
            #[inline]
            #[must_use]
            pub fn div_euclid(self, rhs: Self) -> Self {
                Self(self.0.div_euclid(rhs.0) * Self::multiplier())
            }

            /// Euclidean remainder: `self - rhs * self.div_euclid(rhs)`,
            /// always non-negative when `rhs != ZERO`. Both operands
            /// share the scale, so no rescaling is needed. Panics on
            /// `rhs == ZERO`.
            #[inline]
            #[must_use]
            pub fn rem_euclid(self, rhs: Self) -> Self {
                Self(self.0.rem_euclid(rhs.0))
            }

            /// Absolute difference `|self - rhs|`. Computed as
            /// `max - min` so the subtraction is always non-negative.
            #[inline]
            #[must_use]
            pub fn abs_diff(self, rhs: Self) -> Self {
                Self(self.0.max(rhs.0) - self.0.min(rhs.0))
            }

            /// Midpoint of `self` and `rhs` without intermediate
            /// overflow, rounding toward negative infinity. Uses the
            /// branch-free `(a & b) + ((a ^ b) >> 1)` identity, which is
            /// overflow-free and storage-agnostic.
            #[inline]
            #[must_use]
            pub fn midpoint(self, rhs: Self) -> Self {
                Self((self.0 & rhs.0) + ((self.0 ^ rhs.0) >> 1u32))
            }

            /// Always `false` — a fixed-point decimal has no NaN.
            #[inline]
            #[must_use]
            pub const fn is_nan(self) -> bool {
                false
            }

            /// Always `false` — a fixed-point decimal has no infinity.
            #[inline]
            #[must_use]
            pub const fn is_infinite(self) -> bool {
                false
            }

            /// Always `true` — every fixed-point decimal value is finite.
            #[inline]
            #[must_use]
            pub const fn is_finite(self) -> bool {
                true
            }

            /// `self * a + b`. Mirrors the `f64::mul_add` call shape so
            /// f64-generic numeric code can monomorphise to a decimal
            /// type; there is no hardware FMA — the multiply uses the
            /// type's `Mul` and the add uses its `Add`.
            #[inline]
            #[must_use]
            pub fn mul_add(self, a: Self, b: Self) -> Self {
                self * a + b
            }
        }
    };
}

pub(crate) use decl_decimal_int_methods;
