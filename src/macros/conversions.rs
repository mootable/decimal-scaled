// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Macro-generated conversions between primitive types and the decimal
//! widths.
//!
//! Every primitive-int → decimal conversion is **fallible**: scaling the
//! input by `multiplier()` (= `10^SCALE`) can overflow the destination
//! storage (near a width's top scale even tiny inputs exceed the range),
//! so each is a `TryFrom` returning [`ConvertError::Overflow`] rather
//! than silently wrapping.
//!
//! - **`TryFrom<$Src>` for the primitive integers** — emitted by
//! [`decl_try_from_primitive!`]. Each delegates to the width's
//! `TryFrom<i128>` via the lossless `$Src -> i128` widen (every
//! `i8..=u64` source fits `i128`).
//! - **`TryFrom<i128>` / `TryFrom<u128>`** — emitted by
//! [`decl_try_from_i128!`] / [`decl_try_from_u128!`]: a `checked_mul`
//! by the multiplier with a range-check against the storage.
//! - **`TryFrom<i64>` / `TryFrom<u64>`** — emitted by
//! [`decl_try_from_i64!`] / [`decl_try_from_u64!`] for the 64-bit
//! storage tier ([`D18`]) only, where `value * 10^SCALE` need not fit
//! `i64`; the wider tiers get their `i64` / `u64` `TryFrom` from
//! [`decl_try_from_primitive!`].
//! - **`TryFrom<f64>` / `TryFrom<f32>`** — emitted by
//! [`decl_try_from_f64!`] / [`decl_try_from_f32!`]: non-finite inputs
//! return [`ConvertError::NotFinite`], finite-but-out-of-range return
//! [`ConvertError::Overflow`], and in-range values round to the type's
//! scale via the crate-default `RoundingMode`.
//!
//! Plus [`decl_decimal_int_conversion_methods!`] which emits
//! `to_int` / `to_int_with` on each width.
//!
//! [`ConvertError::Overflow`]: $crate::support::error::ConvertError::Overflow
//! [`ConvertError::NotFinite`]: $crate::support::error::ConvertError::NotFinite

/// Generates `TryFrom<$Src> for $Type<SCALE>` for a primitive integer
/// source, scaling the value by `10^SCALE` into `$Storage`. Returns
/// `Err(ConvertError::Overflow)` when the scaled magnitude exceeds the
/// storage's representable range — scaling can overflow even for tiny
/// inputs near a width's top scale, so the conversion is fallible.
///
/// The body delegates to the width's `TryFrom<i128>` via the lossless
/// `$Src -> i128` widen: every `i8..=u64` source fits `i128`, so the
/// single `as i128` cast cannot lose information and the `checked_mul`
/// + range-check live in one place ([`decl_try_from_i128!`]). `$Storage`
/// is matched for call-site symmetry with the other conversion macros
/// but is unused in the delegating body.
macro_rules! decl_try_from_primitive {
    (wide $Type:ident, $Storage:ty, $Src:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<$Src> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            /// Constructs from an integer by scaling to `value * 10^SCALE`.
            /// Returns `Err(ConvertError::Overflow)` when the scaled
            /// magnitude exceeds the storage range.
            #[inline]
            fn try_from(value: $Src) -> ::core::result::Result<Self, Self::Error> {
                <Self as ::core::convert::TryFrom<i128>>::try_from(value as i128)
            }
        }
    };

}

pub(crate) use decl_try_from_primitive;

/// Generates `From<$Src<SCALE>> for $Dest<SCALE>` for a lossless
/// widening conversion (e.g. D18, D18 -> D38). `$SrcStorage`
/// must widen losslessly to `$DestStorage` via an `as` cast.
macro_rules! decl_cross_width_widening {
    // Widening *into* wide storage. The source storage is
    // a primitive integer or a narrower wide integer; either way
    // the `BigInt` cast performs the lossless widen.
    (wide $Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::From<$Src<SCALE>> for $Dest<SCALE> {
            /// Widens a narrower decimal type to this wider one. The
            /// scale is unchanged; the storage is widened via
            /// the `BigInt` cast (lossless because the source domain is
            /// a subset of the destination).
            #[inline]
            fn from(value: $Src<SCALE>) -> Self {
                Self(value.to_bits().resize::<$DestStorage>())
            }
        }
    };

}

pub(crate) use decl_cross_width_widening;

/// Generates `TryFrom<$Src<SCALE>> for $Dest<SCALE>` for a fallible
/// narrowing conversion. Returns
/// `Err(ConvertError::OutOfRange)` when the source value exceeds
/// the destination's representable range; otherwise returns the
/// narrowed value bit-for-bit (same logical decimal value).
macro_rules! decl_cross_width_narrowing {
    // Narrowing *from* wide storage. The destination may
    // be a primitive integer (e.g. D76 -> D38) or a narrower wide integer
    // integer (e.g. D153 -> D76); the `BigInt` cast handles the bound
    // widening and the final narrowing cast in both cases.
    (wide $Dest:ident, $DestStorage:ty, $Src:ident, $SrcStorage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<$Src<SCALE>> for $Dest<SCALE> {
            type Error = $crate::support::error::ConvertError;
            /// Attempts to narrow a wider decimal type to this narrower
            /// one. Fails with `Overflow` when the source value exceeds
            /// the destination's `MIN..=MAX`. The scale is unchanged.
            #[inline]
            fn try_from(value: $Src<SCALE>) -> ::core::result::Result<Self, Self::Error> {
                let bits = value.to_bits();
                let dest_max: $SrcStorage = <$DestStorage>::MAX.resize::<$SrcStorage>();
                let dest_min: $SrcStorage = <$DestStorage>::MIN.resize::<$SrcStorage>();
                if bits > dest_max || bits < dest_min {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                ::core::result::Result::Ok(Self(bits.resize::<$DestStorage>()))
            }
        }
    };

}

pub(crate) use decl_cross_width_narrowing;

/// Emits `TryFrom<i128> for $Type<SCALE>` returning
/// `Result<Self, ConvertError::Overflow>` after `checked_mul` by the
/// multiplier and a narrowing range-check against `$Storage`.
macro_rules! decl_try_from_i128 {
    // Wide storage. For storage ≥ 128-bit (`Int<2>`+) `i128` widens
    // losslessly, so the only failure mode is the `checked_mul` by the
    // multiplier overflowing the (still finite) wide storage. For the
    // 64-bit `Int<1>` (D18) storage the cast can truncate, so the
    // round-trip check below rejects out-of-range `i128` inputs (it is a
    // no-op for the wider tiers, where `as_i128()` always recovers `value`).
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<i128> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: i128) -> ::core::result::Result<Self, Self::Error> {
                let widened: $Storage = <$Storage>::from_i128(value);
                if widened.as_i128() != value {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                let scaled = widened
                    .checked_mul(Self::multiplier())
                    .ok_or($crate::support::error::ConvertError::Overflow)?;
                ::core::result::Result::Ok(Self(scaled))
            }
        }
    };

}

pub(crate) use decl_try_from_i128;

/// Emits `TryFrom<u128> for $Type<SCALE>`. First narrows u128 to i128
/// (rejecting if the value exceeds `i128::MAX`) then delegates to the
/// i128 path.
macro_rules! decl_try_from_u128 {
    // Wide storage. `u128` always widens losslessly into
    // the (256-bit-or-wider) signed storage — including values above
    // `i128::MAX`, which the native path would have to reject — so the
    // only failure mode is the `checked_mul` overflowing the storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<u128> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: u128) -> ::core::result::Result<Self, Self::Error> {
                let widened: $Storage = <$Storage>::from_u128(value);
                // For storage wider than 128 bits this always holds; for the
                // 128-bit `Int<2>` (D38) a `u128` above `i128::MAX` lands in
                // the sign bit, so a negative result means the input did not
                // fit the signed storage's positive range — reject as overflow.
                // (This also catches the 64-bit `Int<1>` (D18) case where the
                // low word's high bit was set.)
                if widened.is_negative() {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                // The `is_negative` test misses truncations that land on a
                // small non-negative value (e.g. `2^64` → 0 in `Int<1>`), so a
                // round-trip back to `u128` confirms nothing was dropped. No-op
                // for storage wide enough to hold every `u128`.
                if widened.as_u128() != value {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                let scaled = widened
                    .checked_mul(Self::multiplier())
                    .ok_or($crate::support::error::ConvertError::Overflow)?;
                ::core::result::Result::Ok(Self(scaled))
            }
        }
    };

}

pub(crate) use decl_try_from_u128;

/// Emits `TryFrom<i64> for $Type<SCALE>`, delegating to the width's
/// `TryFrom<i128>` path via the lossless `i64 -> i128` widen.
///
/// This is the fallible integer surface for the 64-bit-storage tier
/// ([`D18`]): `i64 * 10^SCALE` overflows `i64` storage for every
/// `SCALE >= 1`. Wider tiers (D38+) get their `TryFrom<i64>` from
/// [`decl_try_from_primitive!`]; this macro is only wired up for D18 so
/// no `(Src, Dest)` pair gets two impls.
macro_rules! decl_try_from_i64 {
    // Storage-agnostic: forwards `i64` through the width's `TryFrom<i128>`.
    // `$Storage` is matched for call-site symmetry but unused.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<i64> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            /// Constructs from an `i64` by scaling to `value * 10^SCALE`.
            /// Returns `Err(ConvertError::Overflow)` when the scaled
            /// magnitude exceeds the storage range.
            #[inline]
            fn try_from(value: i64) -> ::core::result::Result<Self, Self::Error> {
                <Self as ::core::convert::TryFrom<i128>>::try_from(value as i128)
            }
        }
    };
}

pub(crate) use decl_try_from_i64;

/// Emits `TryFrom<u64> for $Type<SCALE>`, delegating to the width's
/// `TryFrom<u128>` path via the lossless `u64 -> u128` widen.
///
/// `u64::MAX` exceeds `i64::MAX`, so even at `SCALE == 0` a `u64` need
/// not fit the signed 64-bit storage of [`D18`]; the conversion is
/// fallible for every `(u64, Int<1>, SCALE)` cell. Only wired up for D18
/// (wider tiers get their `TryFrom<u64>` from
/// [`decl_try_from_primitive!`]).
macro_rules! decl_try_from_u64 {
    // Storage-agnostic: forwards `u64` through the width's `TryFrom<u128>`.
    // `$Storage` is matched for call-site symmetry but unused.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<u64> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            /// Constructs from a `u64` by scaling to `value * 10^SCALE`.
            /// Returns `Err(ConvertError::Overflow)` when the value or its
            /// scaled magnitude exceeds the signed storage range.
            #[inline]
            fn try_from(value: u64) -> ::core::result::Result<Self, Self::Error> {
                <Self as ::core::convert::TryFrom<u128>>::try_from(value as u128)
            }
        }
    };
}

pub(crate) use decl_try_from_u64;

/// Emits `TryFrom<f64> for $Type<SCALE>`. NaN / ±inf return
/// `NotFinite`; finite values whose scaled magnitude exceeds the
/// storage range return `Overflow`.
///
/// The in-range value is **rounded to the type's scale using the
/// crate-default [`RoundingMode`]** (the compile-time `rounding-*`
/// feature selection; `HalfToEven` by default). This is the standard
/// fallible `f64 -> D` surface and is lossy, unlike the int
/// `TryFrom<i128>` path which is exact-or-`Overflow`. For an explicit
/// rounding mode use the inherent `from_f64_with(value, mode)`, which
/// saturates on overflow rather than erroring.
///
/// Under `no_std` (no float-rounding intrinsics) the value is truncated
/// toward zero; the rounding-aware path is `std`-only.
///
/// [`RoundingMode`]: $crate::support::rounding::RoundingMode
macro_rules! decl_try_from_f64 {
    // Wide storage. The multiplier and storage bounds
    // round-trip through `f64` via the `BigInt` cast; the final
    // `f64` -> wide cast is also the `BigInt` cast.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<f64> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: f64) -> ::core::result::Result<Self, Self::Error> {
                if !value.is_finite() {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::NotFinite,
                    );
                }
                let mult_f64: f64 = Self::multiplier().as_f64();
                let scaled = value * mult_f64;
                let storage_max_f64: f64 = <$Storage>::MAX.as_f64();
                let storage_min_f64: f64 = <$Storage>::MIN.as_f64();
                if !(storage_min_f64..storage_max_f64).contains(&scaled) {
                    return ::core::result::Result::Err(
                        $crate::support::error::ConvertError::Overflow,
                    );
                }
                // Round the scaled value to the nearest integer using the
                // crate-default rounding mode, then store. The rounding
                // uses the libm-free `f64` helpers, so it behaves
                // identically with or without `std` / `libm`.
                let rounded = match $crate::support::rounding::DEFAULT_ROUNDING_MODE {
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
                ::core::result::Result::Ok(Self(<$Storage>::from_f64(rounded)))
            }
        }
    };

}

/// Emits `TryFrom<f32> for $Type<SCALE>` by delegating to the
/// `TryFrom<f64>` path via a widening cast. The body is storage-
/// agnostic — it only forwards — so a single arm serves every width;
/// the `wide` token is accepted and ignored for call-site symmetry
/// with the other conversion macros.
macro_rules! decl_try_from_f32 {
    // Storage-agnostic: forwards `f32` through the width's `TryFrom<f64>`.
    // `$Storage` is matched for call-site symmetry but unused.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::convert::TryFrom<f32> for $Type<SCALE> {
            type Error = $crate::support::error::ConvertError;
            #[inline]
            fn try_from(value: f32) -> ::core::result::Result<Self, Self::Error> {
                <Self as ::core::convert::TryFrom<f64>>::try_from(value as f64)
            }
        }
    };
}

pub(crate) use decl_try_from_f32;
pub(crate) use decl_try_from_f64;

/// Emits the `to_int` / `to_int_with` integer readers on a decimal
/// type. `$Storage` is the storage integer. Construction from an
/// integer is the fallible `TryFrom<iN>` surface (see
/// [`decl_try_from_primitive!`] / [`decl_try_from_i128!`]); this macro
/// only emits the to-integer direction.
macro_rules! decl_decimal_int_conversion_methods {
    // Wide storage. The rounding logic is carried in the wide storage
    // type throughout; the final saturating narrow to `i64` goes through
    // the `BigInt` cast.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Converts to `i64` using the crate default rounding mode.
            /// Saturates to `i64::MAX` / `i64::MIN` when the rounded
            /// integer part falls outside `i64`'s range.
            #[inline]
            pub fn to_int(self) -> i64 {
                self.to_int_with($crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Converts to `i64` using the supplied rounding mode for the
            /// fractional discard step. Saturates to `i64::MAX` /
            /// `i64::MIN` when the rounded integer is out of `i64` range.
            #[inline]
            pub fn to_int_with(self, mode: $crate::support::rounding::RoundingMode) -> i64 {
                let zero = <$Storage>::from_str_radix("0", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let one = <$Storage>::from_str_radix("1", 10)
                    .expect("wide decimal: invalid base-10 literal");
                let raw = self.0;
                let divisor = Self::multiplier();
                let quotient = raw / divisor;
                let remainder = raw % divisor;
                let int_rounded: $Storage = if remainder == zero {
                    quotient
                } else {
                    let abs_rem = remainder.unsigned_abs();
                    // `divisor` is `10^SCALE` and always positive, so
                    // `unsigned_abs()` is the value itself; `>> 1` is
                    // the half-LSB threshold.
                    let half = divisor.unsigned_abs() >> 1;
                    let non_negative = !raw.is_negative();
                    match mode {
                        $crate::support::rounding::RoundingMode::HalfToEven => {
                            if abs_rem < half {
                                quotient
                            } else if abs_rem > half {
                                if non_negative {
                                    quotient + one
                                } else {
                                    quotient - one
                                }
                            } else if !quotient.bit(0) {
                                quotient
                            } else if non_negative {
                                quotient + one
                            } else {
                                quotient - one
                            }
                        }
                        $crate::support::rounding::RoundingMode::HalfAwayFromZero => {
                            if abs_rem < half {
                                quotient
                            } else if non_negative {
                                quotient + one
                            } else {
                                quotient - one
                            }
                        }
                        $crate::support::rounding::RoundingMode::HalfTowardZero => {
                            if abs_rem > half {
                                if non_negative {
                                    quotient + one
                                } else {
                                    quotient - one
                                }
                            } else {
                                quotient
                            }
                        }
                        $crate::support::rounding::RoundingMode::Trunc => quotient,
                        $crate::support::rounding::RoundingMode::Floor => {
                            if non_negative {
                                quotient
                            } else {
                                quotient - one
                            }
                        }
                        $crate::support::rounding::RoundingMode::Ceiling => {
                            if non_negative {
                                quotient + one
                            } else {
                                quotient
                            }
                        }
                    }
                };
                let i64_max: $Storage = <$Storage>::from_i128(i64::MAX as i128);
                let i64_min: $Storage = <$Storage>::from_i128(i64::MIN as i128);
                if int_rounded > i64_max {
                    i64::MAX
                } else if int_rounded < i64_min {
                    i64::MIN
                } else {
                    int_rounded.as_i128() as i64
                }
            }
        }
    };

}

pub(crate) use decl_decimal_int_conversion_methods;
