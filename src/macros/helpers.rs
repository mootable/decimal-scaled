//! Macro-generated helper methods: min/max/clamp/recip/copysign.
//!
//! `min` / `max` / `clamp` / `recip` are identical across native and
//! wide storage (both surfaces support `Ord::min` etc. and the `/`
//! operator). Only `copysign` differs: a hand-rolled wide integer cannot be
//! compared against the `0` literal, so the `wide` arm uses
//! `is_negative()` instead.

/// Emits `min`, `max`, `clamp`, `recip`, `copysign` for `$Type<SCALE>`.
macro_rules! decl_decimal_helpers {
    // Wide storage.
    (wide $Type:ident) => {
        $crate::macros::helpers::decl_decimal_helpers!(@common $Type);

        impl<const SCALE: u32> $Type<SCALE> {
            /// Magnitude of `self` with the sign of `sign`. Zero sign is
            /// treated as positive (the storage type has no negative zero).
            #[inline]
            #[must_use]
            pub fn copysign(self, sign: Self) -> Self {
                let mag = self.0.abs();
                if sign.0.is_negative() { Self(-mag) } else { Self(mag) }
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident) => {
        $crate::macros::helpers::decl_decimal_helpers!(@common $Type);

        impl<const SCALE: u32> $Type<SCALE> {
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

    // Shared: min / max / clamp / recip.
    (@common $Type:ident) => {
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
        }
    };
}

pub(crate) use decl_decimal_helpers;
