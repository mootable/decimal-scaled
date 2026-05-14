//! Macro-generated `from_f64_lossy` / `to_f64_lossy` / `to_f32_lossy`
//! for narrow decimal widths, with rounding-mode-aware variants.
//!
//! Two `from_f64` surfaces are emitted:
//!
//! - `from_f64_lossy(value)` — uses
//!   `crate::rounding::DEFAULT_ROUNDING_MODE` (controlled by the
//!   `rounding-*` Cargo features; HalfToEven by default).
//! - `from_f64_lossy_with(value, mode)` — explicit `RoundingMode`.
//!
//! Output saturation policy is uniform across modes: NaN -> ZERO,
//! +Infinity -> MAX, -Infinity -> MIN, finite out-of-range -> MAX/MIN
//! by sign. Float methods are `std`-only.

/// Emits `from_f64_lossy(value)`, `from_f64_lossy_with(value, mode)`,
/// `to_f64_lossy(self)`, `to_f32_lossy(self)` for a decimal type.
macro_rules! decl_decimal_float_bridge {
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an `f64` using the crate default rounding
            /// mode (HalfToEven, or whichever a `rounding-*` Cargo
            /// feature selects). NaN -> ZERO, +Infinity -> MAX,
            /// -Infinity -> MIN, out-of-range -> saturate by sign.
            #[cfg(all(feature = "std", not(feature = "strict")))]
            #[inline]
            #[must_use]
            pub fn from_f64_lossy(value: f64) -> Self {
                Self::from_f64_lossy_with(value, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Constructs from an `f64` using the supplied rounding
            /// mode. Saturation policy as in [`Self::from_f64_lossy`].
            #[cfg(all(feature = "std", not(feature = "strict")))]
            #[inline]
            #[must_use]
            pub fn from_f64_lossy_with(value: f64, mode: $crate::rounding::RoundingMode) -> Self {
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
                let rounded = match mode {
                    $crate::rounding::RoundingMode::HalfToEven => scaled.round_ties_even(),
                    $crate::rounding::RoundingMode::HalfAwayFromZero => scaled.round(),
                    $crate::rounding::RoundingMode::HalfTowardZero => {
                        // Round to nearest, ties toward zero. Bias the
                        // value by a half-LSB toward zero and then round.
                        if scaled >= 0.0 {
                            (scaled - 0.5).ceil()
                        } else {
                            (scaled + 0.5).floor()
                        }
                    }
                    $crate::rounding::RoundingMode::Trunc => scaled.trunc(),
                    $crate::rounding::RoundingMode::Floor => scaled.floor(),
                    $crate::rounding::RoundingMode::Ceiling => scaled.ceil(),
                };
                Self(rounded as $Storage)
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
