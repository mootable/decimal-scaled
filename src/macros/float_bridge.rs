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
///
/// - `decl_decimal_float_bridge!(D32, i32)` — *native* storage; the
///   `f64` <-> storage conversions use `as`-casts.
/// - `decl_decimal_float_bridge!(wide D256, I256)` — *wide* storage;
///   the conversions use `bnum::cast::As`. The experimental `f16` /
///   `f128` entry points route through `f64` for wide storage (`bnum`
///   only provides `f32` / `f64` casts), so they are lossier on the
///   wide tier than on D128-and-narrower.
macro_rules! decl_decimal_float_bridge {
    // Wide (bnum-backed) storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from an `f64` using the crate default rounding
            /// mode. NaN -> ZERO, +Infinity -> MAX, -Infinity -> MIN,
            /// out-of-range -> saturate by sign.
            #[cfg(feature = "std")]
            #[inline]
            #[must_use]
            pub fn from_f64_lossy(value: f64) -> Self {
                Self::from_f64_lossy_with(value, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Constructs from an `f64` using the supplied rounding
            /// mode. Saturation policy as in [`Self::from_f64_lossy`].
            #[cfg(feature = "std")]
            #[inline]
            #[must_use]
            pub fn from_f64_lossy_with(value: f64, mode: $crate::rounding::RoundingMode) -> Self {
                if value.is_nan() {
                    return Self::ZERO;
                }
                if value.is_infinite() {
                    return if value > 0.0 { Self::MAX } else { Self::MIN };
                }
                let mult_f64: f64 = ::bnum::cast::As::as_(Self::multiplier());
                let scaled = value * mult_f64;
                let storage_max_f64: f64 = ::bnum::cast::As::as_(<$Storage>::MAX);
                let storage_min_f64: f64 = ::bnum::cast::As::as_(<$Storage>::MIN);
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
                Self(::bnum::cast::As::as_(rounded))
            }

            /// Converts to `f64` by dividing the raw storage by
            /// `10^SCALE`. Lossy: an `f64` mantissa cannot hold the full
            /// wide-storage precision.
            #[inline]
            #[must_use]
            pub fn to_f64_lossy(self) -> f64 {
                let raw_f64: f64 = ::bnum::cast::As::as_(self.0);
                let mult_f64: f64 = ::bnum::cast::As::as_(Self::multiplier());
                raw_f64 / mult_f64
            }

            /// Converts to `f32` via `f64`, then narrows.
            #[inline]
            #[must_use]
            pub fn to_f32_lossy(self) -> f32 {
                self.to_f64_lossy() as f32
            }

            /// Construct from an `f16` using the crate default rounding
            /// mode. Routes through `f64`. Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f16_lossy(value: f16) -> Self {
                Self::from_f64_lossy(value as f64)
            }

            /// Convert to `f16` (lossy). Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f16_lossy(self) -> f16 {
                self.to_f64_lossy() as f16
            }

            /// Construct from an `f128` using the crate default rounding
            /// mode. For wide storage this routes through `f64` (`bnum`
            /// provides no `f128` cast), so it is lossier than the
            /// D128-and-narrower path. Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128_lossy(value: f128) -> Self {
                Self::from_f128_lossy_with(value, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Construct from an `f128` using the supplied rounding mode.
            /// Routes through `f64` for wide storage.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128_lossy_with(
                value: f128,
                mode: $crate::rounding::RoundingMode,
            ) -> Self {
                Self::from_f64_lossy_with(value as f64, mode)
            }

            /// Convert to `f128`. Routes through `f64` for wide storage.
            /// Nightly + `experimental-floats`.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f128_lossy(self) -> f128 {
                self.to_f64_lossy() as f128
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
            pub fn from_f64_lossy(value: f64) -> Self {
                Self::from_f64_lossy_with(value, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Constructs from an `f64` using the supplied rounding
            /// mode. Saturation policy as in [`Self::from_f64_lossy`].
            #[cfg(feature = "std")]
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
            /// Available in `no_std` because the `as f64` cast and float
            /// division are part of `core`.
            #[inline]
            #[must_use]
            pub fn to_f64_lossy(self) -> f64 {
                (self.0 as f64) / (Self::multiplier() as f64)
            }

            /// Converts to `f32` via `f64`, then narrows. `no_std`-safe.
            #[inline]
            #[must_use]
            pub fn to_f32_lossy(self) -> f32 {
                self.to_f64_lossy() as f32
            }

            /// Construct from an `f16` using the crate default rounding
            /// mode. Available only on nightly with the
            /// `experimental-floats` feature.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f16_lossy(value: f16) -> Self {
                Self::from_f64_lossy(value as f64)
            }

            /// Convert to `f16` (lossy; f16 has only a 10-bit mantissa).
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f16_lossy(self) -> f16 {
                self.to_f64_lossy() as f16
            }

            /// Construct from an `f128` using the crate default rounding
            /// mode. Available only on nightly with the
            /// `experimental-floats` feature. The intermediate is `f128`
            /// directly (no lossy `as f64` step) for the maximum
            /// precision available before narrowing to the storage.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128_lossy(value: f128) -> Self {
                Self::from_f128_lossy_with(value, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Construct from an `f128` using the supplied rounding mode.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn from_f128_lossy_with(
                value: f128,
                mode: $crate::rounding::RoundingMode,
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
                    $crate::rounding::RoundingMode::HalfToEven => scaled.round_ties_even(),
                    $crate::rounding::RoundingMode::HalfAwayFromZero => scaled.round(),
                    $crate::rounding::RoundingMode::HalfTowardZero => {
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

            /// Convert to `f128`. Lossless when the storage fits in the
            /// f128 mantissa (113 bits), which holds for D128 and
            /// narrower; D256 / D512 / D1024 narrowing is lossy.
            #[cfg(all(feature = "std", feature = "experimental-floats"))]
            #[inline]
            #[must_use]
            pub fn to_f128_lossy(self) -> f128 {
                (self.0 as f128) / (Self::multiplier() as f128)
            }
        }
    };
}

pub(crate) use decl_decimal_float_bridge;
