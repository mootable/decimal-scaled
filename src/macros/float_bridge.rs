//! Macro-generated `from_f64` / `to_f64` / `to_f32`
//! for narrow decimal widths, with rounding-mode-aware variants.
//!
//! Two `from_f64` surfaces are emitted:
//!
//! - `from_f64(value)` — uses
//! `crate::support::rounding::DEFAULT_ROUNDING_MODE` (controlled by the
//! `rounding-*` Cargo features; `HalfToEven` by default).
//! - `from_f64_with(value, mode)` — explicit `RoundingMode`.
//!
//! Output saturation policy is uniform across modes: NaN -> ZERO,
//! +Infinity -> MAX, -Infinity -> MIN, finite out-of-range -> MAX/MIN
//! by sign. The rounding step uses the libm-free `f64` rounding helpers
//! in `crate::support::rounding`, so `from_f64` / `to_f64` remain
//! available in `no_std` without `libm`.

/// Emits `from_f64(value)`, `from_f64_with(value, mode)`,
/// `to_f64(self)`, `to_f32(self)` for a decimal type.
///
/// - `decl_decimal_float_bridge!(D18, i64)` — *native* storage; the
/// `f64` <-> storage conversions use `as`-casts.
/// - `decl_decimal_float_bridge!(wide D76, Int<4>)` — *wide* storage;
/// the conversions use the `BigInt` cast. The experimental `f16` /
/// `f128` entry points route through `f64` for wide storage (the wide integer
/// only provides `f32` / `f64` casts), so they are lossier on the
/// wide tier than on D38-and-narrower.
macro_rules! decl_decimal_float_bridge {
    // Wide storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an `f64` using the crate default rounding
            /// mode. NaN -> ZERO, +Infinity -> MAX, -Infinity -> MIN,
            /// out-of-range -> saturate by sign.
            #[inline]
            #[must_use]
            pub fn from_f64(value: f64) -> Self {
                Self::from_f64_with(value, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Constructs from an `f64` using the supplied rounding
            /// mode. Saturation policy as in [`Self::from_f64`].
            #[inline]
            #[must_use]
            pub fn from_f64_with(
                value: f64,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if value.is_nan() {
                    return Self::ZERO;
                }
                if value.is_infinite() {
                    return if value > 0.0 { Self::MAX } else { Self::MIN };
                }
                let mult_f64: f64 = Self::multiplier().as_f64();
                let scaled = value * mult_f64;
                let storage_max_f64: f64 = <$Storage>::MAX.as_f64();
                let storage_min_f64: f64 = <$Storage>::MIN.as_f64();
                if scaled >= storage_max_f64 {
                    return Self::MAX;
                }
                if scaled < storage_min_f64 {
                    return Self::MIN;
                }
                let rounded = match mode {
                    $crate::support::rounding::RoundingMode::HalfToEven => {
                        $crate::support::rounding::round_half_even_f64(scaled)
                    }
                    $crate::support::rounding::RoundingMode::HalfAwayFromZero => {
                        $crate::support::rounding::round_half_away_f64(scaled)
                    }
                    $crate::support::rounding::RoundingMode::HalfTowardZero => {
                        $crate::support::rounding::round_half_toward_zero_f64(scaled)
                    }
                    $crate::support::rounding::RoundingMode::Trunc => {
                        $crate::support::rounding::trunc_f64(scaled)
                    }
                    $crate::support::rounding::RoundingMode::Floor => {
                        $crate::support::rounding::floor_f64(scaled)
                    }
                    $crate::support::rounding::RoundingMode::Ceiling => {
                        $crate::support::rounding::ceil_f64(scaled)
                    }
                };
                Self(<$Storage>::from_f64(rounded))
            }

            /// Converts to `f64` by dividing the raw storage by
            /// `10^SCALE`. Lossy: an `f64` mantissa cannot hold the full
            /// wide-storage precision.
            #[inline]
            #[must_use]
            pub fn to_f64(self) -> f64 {
                let raw_f64: f64 = self.0.as_f64();
                let mult_f64: f64 = Self::multiplier().as_f64();
                raw_f64 / mult_f64
            }

            /// Converts to `f32` via `f64`, then narrows.
            #[inline]
            #[must_use]
            pub fn to_f32(self) -> f32 {
                self.to_f64() as f32
            }

            /// Construct from an `f16` using the crate default rounding
            /// mode. Routes through `f64`. Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f16(value: f16) -> Self {
                Self::from_f64(value as f64)
            }

            /// Convert to `f16` (lossy). Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f16(self) -> f16 {
                self.to_f64() as f16
            }

            /// Construct from an `f128` using the crate default rounding
            /// mode. For wide storage this routes through `f64` (the wide integer
            /// provides no `f128` cast), so it is lossier than the
            /// D38-and-narrower path. Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128(value: f128) -> Self {
                Self::from_f128_with(value, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Construct from an `f128` using the supplied rounding mode.
            /// Routes through `f64` for wide storage.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128_with(
                value: f128,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                Self::from_f64_with(value as f64, mode)
            }

            /// Convert to `f128`. Routes through `f64` for wide storage.
            /// Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f128(self) -> f128 {
                self.to_f64() as f128
            }
        }
    };

}

pub(crate) use decl_decimal_float_bridge;
