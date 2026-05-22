//! Macro-generated sign-aware inherent methods (`abs`, `signum`,
//! `is_positive`, `is_negative`) for the decimal widths.
//!
//! The methods delegate to the `Int<N>` storage's equivalents: `signum`
//! returns the storage type (not a primitive `i32`), and integer literals
//! cannot be compared against `Int<N>` values, so the bodies are spelled
//! with `is_positive()` / `is_negative()` instead.

/// Emits `abs`, `signum`, `is_positive`, `is_negative` for a decimal
/// type with the given storage.
macro_rules! decl_decimal_sign_methods {
    // Wide storage.
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

}

pub(crate) use decl_decimal_sign_methods;
