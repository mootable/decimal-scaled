// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `num_traits`-bridge methods on every decimal width.
//!
//! `from_num` / `to_num` are saturating, never-panicking constructors
//! and readers that thread the input through the [`num_traits::NumCast`]
//! ecosystem, dispatching to the width's [`num_traits::FromPrimitive`] /
//! [`num_traits::ToPrimitive`] impls.
//!
//! Idiomatic call sites should prefer the direct surface — `From<T>` /
//! `TryFrom<T>` for construction, `from_f64` / `to_f64` for the float
//! bridge — for readability and stricter overflow handling. The `from_num` /
//! `to_num` pair is provided for code that needs a single saturating
//! `NumCast`-style entry point regardless of input type.
//!
//! # Saturation policy
//!
//! Conversions never panic. Out-of-range inputs are saturated:
//!
//! - `NaN` maps to [`D38::ZERO`].
//! - `+Infinity` maps to [`D38::MAX`].
//! - `-Infinity` maps to [`D38::MIN`].
//! - Finite values outside the representable range saturate to `MAX` or
//!   `MIN` by sign.
//!
//! # Examples
//!
//! ```
//! use decimal_scaled::D38s12;
//!
//! // `from_num` routes any `T: ToPrimitive` through `NumCast`:
//! let d = D38s12::from_num(42_i32);
//! assert_eq!(d, D38s12::from(42_i32));
//!
//! // `to_num` returns any `T: NumCast + Bounded`, saturating on
//! // out-of-range targets.
//! let f: f32 = d.to_num();
//! assert_eq!(f, 42.0_f32);
//! assert_eq!(D38s12::from_num(f64::INFINITY), D38s12::MAX);
//! ```

// `from_num` / `to_num` are emitted by
// `decl_decimal_num_traits_basics!` in `src/macros/num_traits.rs`,
// so every width (D18 / D38 / D57 / D76 / D115 / D153 / D230 /
// D307 / D462 / D616 / D924 / D1232) gets the same surface. Tests
// below cover the D38 path; the macro emits the same code for the
// wide widths and `tests/macros_num_traits.rs` covers them.

#[cfg(test)]
mod tests {
    use crate::types::widths::D38s12;

    // from_num — thin delegate over NumCast / FromPrimitive.

    /// `from_num(i32)` matches the idiomatic `From<i32>` impl.
    #[test]
    fn from_num_i32_round_trip() {
        let d = D38s12::from_num(42_i32);
        assert_eq!(d, D38s12::from(42_i32));
        assert_eq!(d.to_num::<i32>(), 42_i32);
    }

    /// `from_num(i64)` matches `From<i64>`.
    #[test]
    fn from_num_i64_matches_from() {
        let d = D38s12::from_num(1_000_i64);
        assert_eq!(d, D38s12::from(1_000_i64));
    }

    /// `from_num(f64)` for an in-range value matches `from_f64`.
    #[test]
    fn from_num_f64_within_range() {
        let d = D38s12::from_num(1.5_f64);
        assert_eq!(d, D38s12::from_f64(1.5_f64));
    }

    /// `from_num(f64::INFINITY)` saturates to `MAX`.
    #[test]
    fn from_num_f64_inf_saturates_max() {
        assert_eq!(D38s12::from_num(f64::INFINITY), D38s12::MAX);
    }

    /// `from_num(f64::NEG_INFINITY)` saturates to `MIN`.
    #[test]
    fn from_num_f64_neg_inf_saturates_min() {
        assert_eq!(D38s12::from_num(f64::NEG_INFINITY), D38s12::MIN);
    }

    /// `from_num(f64::NAN)` returns `ZERO` (deterministic NaN policy).
    #[test]
    fn from_num_f64_nan_is_zero() {
        assert_eq!(D38s12::from_num(f64::NAN), D38s12::ZERO);
    }

    /// Finite out-of-range f64 saturates by sign.
    #[test]
    fn from_num_f64_finite_oor_saturates() {
        // 1e30 * 10^12 = 1e42 > i128::MAX ~1.7e38; positive → MAX.
        assert_eq!(D38s12::from_num(1e30_f64), D38s12::MAX);
        // negative → MIN.
        assert_eq!(D38s12::from_num(-1e30_f64), D38s12::MIN);
    }

    /// `from_num(f32::INFINITY)` saturates (validates f32 path).
    #[test]
    fn from_num_f32_inf_saturates() {
        assert_eq!(D38s12::from_num(f32::INFINITY), D38s12::MAX);
        assert_eq!(D38s12::from_num(f32::NEG_INFINITY), D38s12::MIN);
        assert_eq!(D38s12::from_num(f32::NAN), D38s12::ZERO);
    }

    /// `from_num` accepts values past i64's range that still fit
    /// `D38<SCALE>`'s storage — at `SCALE = 12`, D38's integer range is
    /// roughly ±1.7e14 model units.
    #[test]
    fn from_num_does_not_saturate_for_wider_than_i64_decimal_range() {
        let v: i64 = 10_000_000_000_i64;
        let d = D38s12::from_num(v);
        assert_eq!(d.to_int(), v);
    }

    // to_num — thin delegate over NumCast / ToPrimitive.

    /// `D38::ONE.to_num::<f64>() == 1.0`.
    #[test]
    fn to_num_f64_lossy() {
        assert_eq!(D38s12::ONE.to_num::<f64>(), 1.0_f64);
        assert_eq!((-D38s12::ONE).to_num::<f64>(), -1.0_f64);
        assert_eq!(D38s12::ZERO.to_num::<f64>(), 0.0_f64);
    }

    /// `D38::ONE.to_num::<f32>() == 1.0`.
    #[test]
    fn to_num_f32_lossy() {
        assert_eq!(D38s12::ONE.to_num::<f32>(), 1.0_f32);
        assert_eq!((-D38s12::ONE).to_num::<f32>(), -1.0_f32);
    }

    /// `D38::from(42_i32).to_num::<i32>() == 42`.
    #[test]
    fn to_num_i32_in_range() {
        let d = D38s12::from(42_i32);
        assert_eq!(d.to_num::<i32>(), 42_i32);

        let neg = D38s12::from(-42_i32);
        assert_eq!(neg.to_num::<i32>(), -42_i32);
    }

    /// `D38::MAX.to_num::<i32>() == i32::MAX` (saturating positive).
    #[test]
    fn to_num_i32_out_of_range_saturates_max() {
        assert_eq!(D38s12::MAX.to_num::<i32>(), i32::MAX);
    }

    /// `D38::MIN.to_num::<i32>() == i32::MIN` (saturating negative).
    #[test]
    fn to_num_i32_out_of_range_saturates_min() {
        assert_eq!(D38s12::MIN.to_num::<i32>(), i32::MIN);
    }

    /// `to_num::<i64>()` saturates at i64 bounds.
    #[test]
    fn to_num_i64_saturates() {
        assert_eq!(D38s12::MAX.to_num::<i64>(), i64::MAX);
        assert_eq!(D38s12::MIN.to_num::<i64>(), i64::MIN);
        assert_eq!(D38s12::from(42_i64).to_num::<i64>(), 42_i64);
    }

    /// `to_num::<u32>()` returns 0 for negative values (saturates to
    /// `u32::MIN = 0`).
    #[test]
    fn to_num_u32_negative_saturates_to_zero() {
        assert_eq!((-D38s12::ONE).to_num::<u32>(), u32::MIN);
        assert_eq!(D38s12::MIN.to_num::<u32>(), u32::MIN);
        // Positive out-of-range → u32::MAX.
        assert_eq!(D38s12::MAX.to_num::<u32>(), u32::MAX);
    }

    /// Round-trip via from_num / to_num for representative i32 values.
    #[test]
    fn from_num_to_num_round_trip_i32() {
        for v in [0_i32, 1, -1, 42, -42, 1_000_000, -1_000_000] {
            let d = D38s12::from_num(v);
            assert_eq!(d.to_num::<i32>(), v);
        }
    }

    // Cross-scale exercise — non-default SCALE.

    /// Compat surface works at non-default SCALE.
    #[test]
    fn from_num_to_num_at_scale_6() {
        type D6 = crate::D<crate::int::types::Int<2>, 6>;
        let d = D6::from_num(7_i32);
        assert_eq!(d, D6::from(7_i32));
        assert_eq!(d.to_num::<i32>(), 7_i32);
    }

    // Integer-typed inputs must not route through f64 for sign
    // detection.

    /// `from_num(i128::MAX)` saturates to `D38::MAX` via the i128 sign
    /// signal, not through a f64 round-trip. `i128::MAX * 10^12`
    /// overflows i128 storage, so `NumCast::from` returns `None`; the
    /// saturation fallback reads sign directly from i128.
    #[test]
    fn from_num_i128_max_saturates_via_int_signal() {
        assert_eq!(D38s12::from_num(i128::MAX), D38s12::MAX);
    }

    /// `from_num(i128::MIN)` saturates to `D38::MIN` via the i128 sign
    /// signal.
    #[test]
    fn from_num_i128_min_saturates_via_int_signal() {
        assert_eq!(D38s12::from_num(i128::MIN), D38s12::MIN);
    }

    /// `from_num(u128::MAX)` saturates to `D38::MAX` via the u128 sign
    /// signal. `to_i128` returns None for u128 > i128::MAX, so the u128
    /// fallback path is exercised here.
    #[test]
    fn from_num_u128_max_saturates_via_uint_signal() {
        assert_eq!(D38s12::from_num(u128::MAX), D38s12::MAX);
    }

    /// `from_num(u64::MAX)` succeeds without saturation — u64::MAX fits
    /// in D38's storage at `SCALE = 12`.
    #[test]
    fn from_num_u64_max_succeeds_without_saturation() {
        let d = D38s12::from_num(u64::MAX);
        assert_eq!(d, D38s12::from(u64::MAX));
    }
}
