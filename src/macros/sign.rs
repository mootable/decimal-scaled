//! Macro-generated sign-aware inherent methods (`abs`, `signum`,
//! `is_positive`, `is_negative`) for the decimal widths.
//!
//! The native arm delegates to the primitive integer intrinsics. The
//! `wide` arm delegates to `bnum`'s equivalents: `signum` on a `bnum`
//! integer returns the storage type (not a primitive `i32`), and
//! integer literals cannot be compared against `bnum` values, so the
//! bodies are spelled with `is_positive()` / `is_negative()` instead.

/// Emits `abs`, `signum`, `is_positive`, `is_negative` for a decimal
/// type with the given storage.
macro_rules! decl_decimal_sign_methods {
    // Wide (bnum-backed) storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Returns the absolute value of `self`.
            ///
            /// Note: `abs(MIN)` overflows (because |MIN| has no positive
            /// counterpart in two's complement). Debug builds panic;
            /// release builds wrap.
            #[inline]
            #[must_use]
            pub const fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns the sign of `self` encoded as a scaled `Self`:
            /// `-ONE`, `ZERO`, or `+ONE`.
            #[inline]
            #[must_use]
            pub fn signum(self) -> Self {
                if self.0.is_positive() {
                    Self::ONE
                } else if self.0.is_negative() {
                    Self(-Self::multiplier())
                } else {
                    Self::ZERO
                }
            }

            /// Returns `true` if `self` is strictly greater than zero.
            #[inline]
            #[must_use]
            pub const fn is_positive(self) -> bool {
                self.0.is_positive()
            }

            /// Returns `true` if `self` is strictly less than zero.
            #[inline]
            #[must_use]
            pub const fn is_negative(self) -> bool {
                self.0.is_negative()
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Returns the absolute value of `self`.
            ///
            /// Note: `abs(MIN)` overflows (because |MIN| has no positive
            /// counterpart in two's complement). Debug builds panic;
            /// release builds wrap.
            #[inline]
            #[must_use]
            pub const fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns the sign of `self` encoded as a scaled `Self`:
            /// `-ONE`, `ZERO`, or `+ONE`.
            #[inline]
            #[must_use]
            pub fn signum(self) -> Self {
                match self.0.signum() {
                    1 => Self::ONE,
                    -1 => Self(-Self::multiplier()),
                    _ => Self::ZERO,
                }
            }

            /// Returns `true` if `self` is strictly greater than zero.
            #[inline]
            #[must_use]
            pub const fn is_positive(self) -> bool {
                self.0 > 0
            }

            /// Returns `true` if `self` is strictly less than zero.
            #[inline]
            #[must_use]
            pub const fn is_negative(self) -> bool {
                self.0 < 0
            }
        }
    };
}

pub(crate) use decl_decimal_sign_methods;
