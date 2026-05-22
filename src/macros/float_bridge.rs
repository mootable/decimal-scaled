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
//! by sign. Float methods are `std`-only.

/// Emits `from_f64(value)`, `from_f64_with(value, mode)`,
/// `to_f64(self)`, `to_f32(self)` for a decimal type.
///
/// - `decl_decimal_float_bridge!(D18, i64)` — *native* storage; the
/// `f64` <-> storage conversions use `as`-casts.
/// - `decl_decimal_float_bridge!(wide D76, I256)` — *wide* storage;
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
            #[cfg(feature = "std")]
            #[inline]
            #[must_use]
            pub fn from_f64(value: f64) -> Self {
                Self::from_f64_with(value, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Constructs from an `f64` using the supplied rounding
            /// mode. Saturation policy as in [`Self::from_f64`].
            #[cfg(feature = "std")]
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
                    $crate::support::rounding::RoundingMode::HalfToEven => scaled.round_ties_even(),
                    $crate::support::rounding::RoundingMode::HalfAwayFromZero => scaled.round(),
                    $crate::support::rounding::RoundingMode::HalfTowardZero => {
                        if scaled >= 0.0 {
                            (scaled - 0.5).ceil()
                        } else {
                            (scaled + 0.5).floor()
                        }
                    }
                    $crate::support::rounding::RoundingMode::Trunc => scaled.trunc(),
                    $crate::support::rounding::RoundingMode::Floor => scaled.floor(),
                    $crate::support::rounding::RoundingMode::Ceiling => scaled.ceil(),
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

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an `f64` using the crate default rounding
            /// mode (HalfToEven, or whichever a `rounding-*` Cargo
            /// feature selects). NaN -> ZERO, +Infinity -> MAX,
            /// -Infinity -> MIN, out-of-range -> saturate by sign.
            #[cfg(feature = "std")]
            #[inline]
            #[must_use]
            pub fn from_f64(value: f64) -> Self {
                Self::from_f64_with(value, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Constructs from an `f64` using the supplied rounding
            /// mode. Saturation policy as in [`Self::from_f64`].
            #[cfg(feature = "std")]
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
                    $crate::support::rounding::RoundingMode::HalfToEven => scaled.round_ties_even(),
                    $crate::support::rounding::RoundingMode::HalfAwayFromZero => scaled.round(),
                    $crate::support::rounding::RoundingMode::HalfTowardZero => {
                        // Round to nearest, ties toward zero. Bias the
                        // value by a half-LSB toward zero and then round.
                        if scaled >= 0.0 {
                            (scaled - 0.5).ceil()
                        } else {
                            (scaled + 0.5).floor()
                        }
                    }
                    $crate::support::rounding::RoundingMode::Trunc => scaled.trunc(),
                    $crate::support::rounding::RoundingMode::Floor => scaled.floor(),
                    $crate::support::rounding::RoundingMode::Ceiling => scaled.ceil(),
                };
                Self(rounded as $Storage)
            }

            /// Converts to `f64` by dividing the raw storage by `10^SCALE`.
            /// Available in `no_std` because the `as f64` cast and float
            /// division are part of `core`.
            #[inline]
            #[must_use]
            pub fn to_f64(self) -> f64 {
                (self.0 as f64) / (Self::multiplier() as f64)
            }

            /// Converts to `f32` via `f64`, then narrows. `no_std`-safe.
            #[inline]
            #[must_use]
            pub fn to_f32(self) -> f32 {
                self.to_f64() as f32
            }

            /// Construct from an `f16` using the crate default rounding
            /// mode. Available only on nightly with the
            /// `experimental-floats` feature.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f16(value: f16) -> Self {
                Self::from_f64(value as f64)
            }

            /// Convert to `f16` (lossy; f16 has only a 10-bit mantissa).
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f16(self) -> f16 {
                self.to_f64() as f16
            }

            /// Construct from an `f128` using the crate default rounding
            /// mode. Available only on nightly with the
            /// `experimental-floats` feature. The intermediate is `f128`
            /// directly (no lossy `as f64` step) for the maximum
            /// precision available before narrowing to the storage.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128(value: f128) -> Self {
                Self::from_f128_with(value, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Construct from an `f128` using the supplied rounding mode.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128_with(
                value: f128,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if value.is_nan() {
                    return Self::ZERO;
                }
                if value.is_infinite() {
                    return if value > 0.0 { Self::MAX } else { Self::MIN };
                }
                let scaled = value * (Self::multiplier() as f128);
                let storage_max_f128 = <$Storage>::MAX as f128;
                let storage_min_f128 = <$Storage>::MIN as f128;
                if scaled >= storage_max_f128 {
                    return Self::MAX;
                }
                if scaled < storage_min_f128 {
                    return Self::MIN;
                }
                let rounded = match mode {
                    $crate::support::rounding::RoundingMode::HalfToEven => scaled.round_ties_even(),
                    $crate::support::rounding::RoundingMode::HalfAwayFromZero => scaled.round(),
                    $crate::support::rounding::RoundingMode::HalfTowardZero => {
                        if scaled >= 0.0 {
                            (scaled - 0.5).ceil()
                        } else {
                            (scaled + 0.5).floor()
                        }
                    }
                    $crate::support::rounding::RoundingMode::Trunc => scaled.trunc(),
                    $crate::support::rounding::RoundingMode::Floor => scaled.floor(),
                    $crate::support::rounding::RoundingMode::Ceiling => scaled.ceil(),
                };
                Self(rounded as $Storage)
            }

            /// Convert to `f128`. Lossless when the storage fits in the
            /// f128 mantissa (113 bits), which holds for D38 and
            /// narrower; D76 / D153 / D307 narrowing is lossy.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f128(self) -> f128 {
                (self.0 as f128) / (Self::multiplier() as f128)
            }
        }
    };
}

pub(crate) use decl_decimal_float_bridge;
