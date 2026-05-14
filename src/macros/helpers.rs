//! Macro-generated helper methods: min/max/clamp/recip/copysign.
//!
//! All work uniformly across integer storage widths.

/// Emits `min`, `max`, `clamp`, `recip`, `copysign` for `$Type<SCALE>`.
macro_rules! decl_decimal_helpers {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// The lesser of `self` and `other`.
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            /// The greater of `self` and `other`.
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            /// Restrict `self` to the closed interval `[lo, hi]`.
            /// Panics if `lo > hi`.
            #[inline]
            #[must_use]
            pub fn clamp(self, lo: Self, hi: Self) -> Self {
                Self(self.0.clamp(lo.0, hi.0))
            }

            /// Multiplicative inverse: `ONE / self`. Panics on `self == ZERO`.
            #[inline]
            #[must_use]
            pub fn recip(self) -> Self {
                Self::ONE / self
            }

            /// Magnitude of `self` with the sign of `sign`. Zero sign is
            /// treated as positive (the storage type has no negative zero).
            #[inline]
            #[must_use]
            pub fn copysign(self, sign: Self) -> Self {
                let mag = self.0.abs();
                if sign.0 < 0 { Self(-mag) } else { Self(mag) }
            }
        }
    };
}

pub(crate) use decl_decimal_helpers;
