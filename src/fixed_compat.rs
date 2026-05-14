//! Compatibility layer for migrating call sites from the `fixed` crate
//! (`fixed::types::I64F64`, `I32F32`, `FixedI128`, etc.) to [`D128`].
//!
//! # Purpose
//!
//! The `fixed` crate's binary fixed-point types share a `from_num` /
//! `to_num` constructor/reader convention. This module provides the same
//! method names on [`D128`] so call sites can swap the underlying type
//! without renaming every occurrence:
//!
//! ```ignore
//! let d = Decimal::from_num(some_i32);
//! let f: f32 = d.to_num();
//! ```
//!
//! When `Decimal` is aliased to [`D128`], existing call sites compile
//! unchanged. New code should prefer the idiomatic [`From<T>`] /
//! [`num_traits::FromPrimitive`] / [`num_traits::ToPrimitive`] surface
//! instead.
//!
//! # Saturation policy
//!
//! Conversions never panic. Out-of-range inputs are saturated:
//!
//! - `NaN` maps to [`D128::ZERO`].
//! - `+Infinity` maps to [`D128::MAX`].
//! - `-Infinity` maps to [`D128::MIN`].
//! - Finite values outside the representable range saturate to `MAX` or `MIN`
//!   by sign.
//!
//! `D128`'s storage range (~+/-1.7e26 model units at `SCALE = 12`) is wider
//! than `I64F64`'s (~+/-9.2e18), so values that would have panicked in
//! `I64F64::from_num` may succeed here. That is correct behaviour.
//!
//! # Examples
//!
//! ```
//! use decimal_scaled::D128e12;
//!
//! // Constructor mirrors `I64F64::from_num`:
//! let d = D128e12::from_num(42_i32);
//! assert_eq!(d, D128e12::from(42_i32));
//!
//! // Reader mirrors `I64F64::to_num`:
//! let f: f32 = d.to_num();
//! assert_eq!(f, 42.0_f32);
//!
//! // Saturation: `f64::INFINITY` -> `D128::MAX` (not panic):
//! assert_eq!(D128e12::from_num(f64::INFINITY), D128e12::MAX);
//! ```

use num_traits::{Bounded, NumCast, ToPrimitive};

use crate::core_type::D128;

impl<const SCALE: u32> D128<SCALE> {
    /// Constructs a `D128<SCALE>` from any `T: ToPrimitive`.
    ///
    /// This is a compatibility alias for the idiomatic [`From<T>`] /
    /// [`num_traits::FromPrimitive`] surface. Routes through
    /// [`num_traits::NumCast::from`], which dispatches to the
    /// [`num_traits::FromPrimitive`] impl on `D128`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point when `T` is a float type;
    /// result may lose precision. For integer `T`, the conversion is Strict:
    /// all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Saturation policy
    ///
    /// - Float `NaN` maps to [`D128::ZERO`].
    /// - `+Infinity` maps to [`D128::MAX`].
    /// - `-Infinity` maps to [`D128::MIN`].
    /// - Finite out-of-range positive maps to [`D128::MAX`].
    /// - Finite out-of-range negative maps to [`D128::MIN`].
    /// - Never panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// assert_eq!(D128e12::from_num(42_i32), D128e12::from(42_i32));
    /// assert_eq!(D128e12::from_num(f64::INFINITY), D128e12::MAX);
    /// assert_eq!(D128e12::from_num(f64::NAN), D128e12::ZERO);
    /// ```
    pub fn from_num<T: ToPrimitive>(value: T) -> Self {
        // Determine sign and NaN status before consuming `value` through
        // NumCast. Integer signals (to_i128 / to_u128) are checked first so
        // that integer-typed inputs never route through f64 -- D128's storage
        // is wider than f64's mantissa and f64 sign-detection would lose
        // precision at large integer values.
        //
        // Three cases cover all ToPrimitive implementors:
        // - to_i128 returns Some(i): all signed primitives and unsigned
        //   primitives that fit in i128 (u8 through u64).
        // - to_i128 returns None but to_u128 returns Some: unsigned values
        //   exceeding i128::MAX; sign is non-negative.
        // - Both return None: input is f32 or f64; inspect to_f64 for NaN
        //   and sign classification.
        let int_signal = value.to_i128();
        let uint_signal = value.to_u128();
        let float_signal = if int_signal.is_none() && uint_signal.is_none() {
            value.to_f64()
        } else {
            None
        };
        // Early exit: NaN maps to ZERO. Only reachable on the float path.
        if let Some(f) = float_signal {
            if f.is_nan() {
                return Self::ZERO;
            }
        }
        if let Some(d) = <Self as NumCast>::from(value) {
            return d;
        }
        // NumCast returned None -- saturate by sign of the original input.
        // Prefer integer signals (lossless); fall back to float only for
        // genuinely float-typed inputs.
        if let Some(i) = int_signal {
            return if i < 0 { Self::MIN } else { Self::MAX };
        }
        if uint_signal.is_some() {
            // Unsigned-only representation cannot be negative.
            return Self::MAX;
        }
        match float_signal {
            Some(f) if f.is_sign_negative() => Self::MIN,
            Some(_) => Self::MAX,
            // No representation at all (exotic ToPrimitive impl). Default
            // to ZERO rather than picking a sign.
            None => Self::ZERO,
        }
    }

    /// Converts `self` to any `T: NumCast + Bounded`.
    ///
    /// This is a compatibility alias for the idiomatic
    /// [`num_traits::ToPrimitive`] / `to_X_lossy` surface. Routes through
    /// [`num_traits::NumCast::from`], which dispatches to the
    /// [`num_traits::ToPrimitive`] impl on `D128`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point when `T` is a float type;
    /// result may lose precision. For integer `T`, the conversion is Strict:
    /// all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Saturation policy
    ///
    /// - In-range conversions return the cast value unchanged.
    /// - Positive out-of-range maps to [`Bounded::max_value`] of `T`.
    /// - Negative out-of-range maps to [`Bounded::min_value`] of `T`.
    /// - Never panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// assert_eq!(D128e12::from(42_i32).to_num::<i32>(), 42_i32);
    /// assert_eq!(D128e12::MAX.to_num::<i32>(), i32::MAX);
    /// assert_eq!(D128e12::MIN.to_num::<i32>(), i32::MIN);
    /// ```
    pub fn to_num<T: NumCast + Bounded>(self) -> T {
        match T::from(self) {
            Some(t) => t,
            None => {
                // Saturate to T::MAX or T::MIN based on the sign of self.
                // Read sign directly from the raw i128 field to avoid a
                // Signed-trait dispatch round-trip.
                if self.0 >= 0 {
                    T::max_value()
                } else {
                    T::min_value()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core_type::{D128, D128e12};

    // from_num -- thin delegate over NumCast / FromPrimitive

    /// `from_num(i32)` matches the idiomatic `From<i32>` impl.
    #[test]
    fn from_num_i32_round_trip() {
        let d = D128e12::from_num(42_i32);
        assert_eq!(d, D128e12::from(42_i32));
        assert_eq!(d.to_num::<i32>(), 42_i32);
    }

    /// `from_num(i64)` matches `From<i64>`.
    #[test]
    fn from_num_i64_matches_from() {
        let d = D128e12::from_num(1_000_i64);
        assert_eq!(d, D128e12::from(1_000_i64));
    }

    /// `from_num(f64)` for an in-range value matches `from_f64_lossy`.
    #[test]
    fn from_num_f64_within_range() {
        let d = D128e12::from_num(1.5_f64);
        assert_eq!(d, D128e12::from_f64_lossy(1.5_f64));
    }

    /// `from_num(f64::INFINITY)` saturates to `MAX`.
    #[test]
    fn from_num_f64_inf_saturates_max() {
        assert_eq!(D128e12::from_num(f64::INFINITY), D128e12::MAX);
    }

    /// `from_num(f64::NEG_INFINITY)` saturates to `MIN`.
    #[test]
    fn from_num_f64_neg_inf_saturates_min() {
        assert_eq!(D128e12::from_num(f64::NEG_INFINITY), D128e12::MIN);
    }

    /// `from_num(f64::NAN)` returns `ZERO` (deterministic NaN policy).
    #[test]
    fn from_num_f64_nan_is_zero() {
        assert_eq!(D128e12::from_num(f64::NAN), D128e12::ZERO);
    }

    /// Finite out-of-range f64 saturates by sign.
    #[test]
    fn from_num_f64_finite_oor_saturates() {
        // 1e30 * 10^12 = 1e42 > i128::MAX ~1.7e38; positive -> MAX.
        assert_eq!(D128e12::from_num(1e30_f64), D128e12::MAX);
        // negative -> MIN.
        assert_eq!(D128e12::from_num(-1e30_f64), D128e12::MIN);
    }

    /// `from_num(f32::INFINITY)` saturates (validates f32 path).
    #[test]
    fn from_num_f32_inf_saturates() {
        assert_eq!(D128e12::from_num(f32::INFINITY), D128e12::MAX);
        assert_eq!(D128e12::from_num(f32::NEG_INFINITY), D128e12::MIN);
        assert_eq!(D128e12::from_num(f32::NAN), D128e12::ZERO);
    }

    // from_num -- wider range than I64F64

    /// At `SCALE = 12`, `D128`'s integer range is ~+/-1.7e14 model units.
    /// `I64F64`'s integer range is ~+/-9.2e9. A value of 1e10 is within
    /// D128's range but exceeds I64F64's representable bound -- this call
    /// must succeed without saturation.
    #[test]
    fn from_num_does_not_panic_on_wider_range_than_i64f64() {
        let v: i64 = 10_000_000_000_i64;
        let d = D128e12::from_num(v);
        // Round-trip: to_int_lossy must return the original value.
        assert_eq!(d.to_int_lossy(), v);
    }

    // to_num -- thin delegate over NumCast / ToPrimitive

    /// `D128::ONE.to_num::<f64>() == 1.0`.
    #[test]
    fn to_num_f64_lossy() {
        assert_eq!(D128e12::ONE.to_num::<f64>(), 1.0_f64);
        assert_eq!((-D128e12::ONE).to_num::<f64>(), -1.0_f64);
        assert_eq!(D128e12::ZERO.to_num::<f64>(), 0.0_f64);
    }

    /// `D128::ONE.to_num::<f32>() == 1.0`.
    #[test]
    fn to_num_f32_lossy() {
        assert_eq!(D128e12::ONE.to_num::<f32>(), 1.0_f32);
        assert_eq!((-D128e12::ONE).to_num::<f32>(), -1.0_f32);
    }

    /// `D128::from(42_i32).to_num::<i32>() == 42`.
    #[test]
    fn to_num_i32_in_range() {
        let d = D128e12::from(42_i32);
        assert_eq!(d.to_num::<i32>(), 42_i32);

        let neg = D128e12::from(-42_i32);
        assert_eq!(neg.to_num::<i32>(), -42_i32);
    }

    /// `D128::MAX.to_num::<i32>() == i32::MAX` (saturating positive).
    #[test]
    fn to_num_i32_out_of_range_saturates_max() {
        assert_eq!(D128e12::MAX.to_num::<i32>(), i32::MAX);
    }

    /// `D128::MIN.to_num::<i32>() == i32::MIN` (saturating negative).
    #[test]
    fn to_num_i32_out_of_range_saturates_min() {
        assert_eq!(D128e12::MIN.to_num::<i32>(), i32::MIN);
    }

    /// `to_num::<i64>()` saturates at i64 bounds.
    #[test]
    fn to_num_i64_saturates() {
        assert_eq!(D128e12::MAX.to_num::<i64>(), i64::MAX);
        assert_eq!(D128e12::MIN.to_num::<i64>(), i64::MIN);
        assert_eq!(D128e12::from(42_i64).to_num::<i64>(), 42_i64);
    }

    /// `to_num::<u32>()` returns 0 for negative values (saturates to
    /// u32::MIN = 0).
    #[test]
    fn to_num_u32_negative_saturates_to_zero() {
        // u32::MIN is 0, so negative D128 values saturate to 0.
        assert_eq!((-D128e12::ONE).to_num::<u32>(), u32::MIN);
        assert_eq!(D128e12::MIN.to_num::<u32>(), u32::MIN);
        // Positive out-of-range -> u32::MAX.
        assert_eq!(D128e12::MAX.to_num::<u32>(), u32::MAX);
    }

    /// Round-trip via from_num / to_num for representative i32 values.
    #[test]
    fn from_num_to_num_round_trip_i32() {
        for v in [0_i32, 1, -1, 42, -42, 1_000_000, -1_000_000] {
            let d = D128e12::from_num(v);
            assert_eq!(d.to_num::<i32>(), v);
        }
    }

    // Cross-scale exercise -- non-default SCALE

    /// Compat surface works at non-default SCALE.
    #[test]
    fn from_num_to_num_at_scale_6() {
        type D6 = D128<6>;
        let d = D6::from_num(7_i32);
        assert_eq!(d, D6::from(7_i32));
        assert_eq!(d.to_num::<i32>(), 7_i32);
    }

    // Integer-typed inputs must not route through f64 for sign detection.

    /// `from_num(i128::MAX)` saturates to `D128::MAX` via the i128 sign
    /// signal, not through a f64 round-trip. `i128::MAX * 10^12` overflows
    /// i128 storage, so NumCast::from returns None; the saturation fallback
    /// reads sign directly from i128.
    #[test]
    fn from_num_i128_max_saturates_via_int_signal() {
        assert_eq!(D128e12::from_num(i128::MAX), D128e12::MAX);
    }

    /// `from_num(i128::MIN)` saturates to `D128::MIN` via the i128 sign signal.
    #[test]
    fn from_num_i128_min_saturates_via_int_signal() {
        assert_eq!(D128e12::from_num(i128::MIN), D128e12::MIN);
    }

    /// `from_num(u128::MAX)` saturates to `D128::MAX` via the u128 sign
    /// signal. `to_i128` returns None for u128 > i128::MAX, so the u128
    /// fallback path is exercised here.
    #[test]
    fn from_num_u128_max_saturates_via_uint_signal() {
        assert_eq!(D128e12::from_num(u128::MAX), D128e12::MAX);
    }

    /// `from_num(u64::MAX)` succeeds without saturation -- u64::MAX fits
    /// in D128's storage at SCALE = 12.
    #[test]
    fn from_num_u64_max_succeeds_without_saturation() {
        let d = D128e12::from_num(u64::MAX);
        assert_eq!(d, D128e12::from(u64::MAX));
    }
}
