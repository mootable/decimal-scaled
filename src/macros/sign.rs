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
            /// `abs(MIN)` overflows: `|MIN|` has no positive counterpart
            /// in two's-complement. Because this is a `const fn`, it
            /// takes the stricter `const` overflow contract — it
            /// *always* panics on overflow (an unconditional
            /// `checked_neg(…).expect(…)`), so in a `const` context the
            /// overflowing case is a compile-time evaluation error,
            /// profile-independent, matching `std`'s `const` integer
            /// arithmetic. At runtime it panics in both debug and
            /// release.
            #[inline]
            #[must_use]
            pub const fn abs(self) -> Self {
                // `Int<N>::abs` is modular (`MIN.abs() == MIN`); the
                // contract is applied here via `checked_neg`. `const`
                // can't branch on `cfg!(debug_assertions)`, so this is
                // the always-error form.
                if self.0.is_negative() {
                    match self.0.checked_neg() {
                        Some(v) => Self(v),
                        None => panic!("attempt to compute the absolute value with overflow"),
                    }
                } else {
                    self
                }
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
