//! Macro-generated sign-aware inherent methods (`abs`, `signum`,
//! `is_positive`, `is_negative`) for narrow decimal widths.
//!
//! These methods work uniformly across `i32` / `i64` / `i128` storage
//! by delegating to the storage type's primitive intrinsics.

/// Emits `abs`, `signum`, `is_positive`, `is_negative` for a decimal
/// type with the given storage.
macro_rules! decl_decimal_sign_methods {
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
