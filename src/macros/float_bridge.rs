//! Macro-generated `from_f64_lossy` / `to_f64_lossy` / `to_f32_lossy`
//! for narrow decimal widths.
//!
//! These methods bridge between the integer storage and IEEE-754
//! floats. They are gated on `cfg(all(feature = "std", not(feature =
//! "strict")))` because strict mode bans f64 transcendentals — but the
//! plain `as f64` cast and float arithmetic are part of `core` for
//! IEEE-754, so the gate is mainly to keep the strict surface
//! integer-only by convention.

/// Emits `from_f64_lossy`, `to_f64_lossy`, `to_f32_lossy` on a decimal
/// type with the given storage. `$Storage` must support `as f64` /
/// `f64 as $Storage` casts.
macro_rules! decl_decimal_float_bridge {
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an `f64`, saturating on non-finite or
            /// out-of-range inputs. NaN maps to ZERO.
            #[cfg(all(feature = "std", not(feature = "strict")))]
            #[inline]
            #[must_use]
            pub fn from_f64_lossy(value: f64) -> Self {
                if value.is_nan() {
                    return Self::ZERO;
                }
                if value.is_infinite() {
                    return if value > 0.0 { Self::MAX } else { Self::MIN };
                }
                let scaled = value * (Self::multiplier() as f64);
                let storage_max_f64 = <$Storage>::MAX as f64;
                let storage_min_f64 = <$Storage>::MIN as f64;
                if scaled >= storage_max_f64 {
                    return Self::MAX;
                }
                if scaled < storage_min_f64 {
                    return Self::MIN;
                }
                Self(scaled as $Storage)
            }

            /// Converts to `f64` by dividing the raw storage by `10^SCALE`.
            #[cfg(all(feature = "std", not(feature = "strict")))]
            #[inline]
            #[must_use]
            pub fn to_f64_lossy(self) -> f64 {
                (self.0 as f64) / (Self::multiplier() as f64)
            }

            /// Converts to `f32` via `f64`, then narrows.
            #[cfg(all(feature = "std", not(feature = "strict")))]
            #[inline]
            #[must_use]
            pub fn to_f32_lossy(self) -> f32 {
                self.to_f64_lossy() as f32
            }
        }
    };
}

pub(crate) use decl_decimal_float_bridge;
