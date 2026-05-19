//! Arithmetic operator overloads for [`D38`].
//!
//! All operators work directly on the raw `i128` storage value.
//! Addition, subtraction, and negation require no rescaling because the
//! scale factor commutes with those operations. Multiplication and division
//! each require one rescaling step to keep the result in `value * 10^SCALE`
//! form; remainder requires none because both operands share the same scale.
//!
//! # Overflow policy
//!
//! Default operator semantics match Rust's `i128` arithmetic: **panics on
//! overflow in debug builds; wraps two's-complement in release builds.**
//! This is the standard Rust integer arithmetic contract. Explicit-overflow
//! variants (`checked_*`, `wrapping_*`, `saturating_*`, `overflowing_*`)
//! are not provided in this module.
//!
//! # Mul / Div algorithm
//!
//! `Mul` / `MulAssign` use a 256-bit widening intermediate followed by a
//! Moller-Granlund 2011 magic-number divide (see the `mg_divide` module).
//! This replaces a naive `(self.0 * rhs.0) / multiplier()` approach that
//! would silently overflow `i128` at operand magnitudes beyond
//! `sqrt(i128::MAX)`. With the widening approach the operand range covers
//! the full `i128` storage range; the only overflow possible is on the
//! final `i128` quotient.
//!
//! `Div` / `DivAssign` widen the numerator `a * 10^SCALE` to 256 bits via
//! the same schoolbook multiply, then divide by `b` using a hand-rolled
//! binary long-divide. MG-style magic does not apply because the divisor is
//! variable rather than a known power of ten.
//!
//! Both paths preserve panic-debug / wrap-release semantics for the final
//! `i128` result. The intermediate 256-bit calculation never observably
//! overflows.

use core::ops::{Div, DivAssign, Mul, MulAssign};

use crate::types::widths::D38;

impl<const SCALE: u32> Mul for D38<SCALE> {
    type Output = Self;

    /// Multiply two values, rescaling the result back to `value * 10^S` form.
    ///
    /// The raw product `a.0 * b.0` has units `10^(2S)`, so it must be
    /// divided by `10^S` to restore the canonical scale. This is done via
    /// a 256-bit widening intermediate and a Moller-Granlund magic-number
    /// divide (see the `mg_divide` module), which avoids the intermediate
    /// overflow that would occur with a naive `i128` multiply at large
    /// operand magnitudes.
    ///
    /// # Panics
    ///
    /// Panics in debug builds when the final rescaled quotient overflows
    /// `i128::MAX`. In release builds the result wraps two's-complement.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D38s12;
    ///
    /// let a = D38s12::from_bits(1_500_000_000_000); // 1.5
    /// let b = D38s12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!((a * b).to_bits(), 3_000_000_000_000); // 3.0
    /// ```
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        match crate::algos::mg_divide::mul_div_pow10::<SCALE>(self.0, rhs.0) {
            Some(q) => Self(q),
            None => Self(panic_or_wrap_mul::<SCALE>(self.0, rhs.0)),
        }
    }
}

impl<const SCALE: u32> MulAssign for D38<SCALE> {
    /// Multiply `self` by `rhs` in place.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const SCALE: u32> Div for D38<SCALE> {
    type Output = Self;

    /// Divide `self` by `rhs`, rescaling the numerator to keep the result
    /// in `value * 10^S` form.
    ///
    /// The numerator `self.0` is widened to 256 bits and multiplied by
    /// `10^SCALE` before dividing by `rhs.0`. This avoids the intermediate
    /// overflow that would occur with a naive `(self.0 * 10^S) / rhs.0`
    /// approach at large dividend magnitudes.
    ///
    /// # Panics
    ///
    /// Panics on division by zero (matching `i128 /`). Also panics in debug
    /// builds when the final quotient overflows `i128`; wraps in release
    /// builds.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D38s12;
    ///
    /// let a = D38s12::from_bits(3_000_000_000_000); // 3.0
    /// let b = D38s12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!((a / b).to_bits(), 1_500_000_000_000); // 1.5
    /// ```
    #[inline]
    fn div(self, rhs: Self) -> Self {
        // Match the panic message from `i128 /`.
        assert!(rhs.0 != 0, "attempt to divide by zero");
        match crate::algos::mg_divide::div_pow10_div::<SCALE>(self.0, rhs.0) {
            Some(q) => Self(q),
            None => Self(panic_or_wrap_div::<SCALE>(self.0, rhs.0)),
        }
    }
}

impl<const SCALE: u32> DivAssign for D38<SCALE> {
    /// Divide `self` by `rhs` in place.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const SCALE: u32> D38<SCALE> {
    /// Multiply two values of the same scale, rounding the
    /// scale-narrowing divide by `10^SCALE` according to `mode`.
    ///
    /// The default `Mul` operator delegates to this with the
    /// crate-default rounding mode; call `mul_with` directly when
    /// you need a non-default rounding rule (e.g.
    /// `HalfAwayFromZero` for commercial rounding, `Floor`/`Ceiling`
    /// for one-sided bracketing).
    ///
    /// # Panics
    ///
    /// Panics in debug builds when the rescaled quotient overflows
    /// `i128`. Wraps two's-complement in release builds.
    ///
    /// # Precision
    ///
    /// Strict: integer-only arithmetic. Within 0.5 ULP for the half-*
    /// family; directed rounding otherwise.
    #[inline]
    #[must_use]
    pub fn mul_with(self, rhs: Self, mode: crate::support::rounding::RoundingMode) -> Self {
        match crate::algos::mg_divide::mul_div_pow10_with::<SCALE>(self.0, rhs.0, mode) {
            Some(q) => Self(q),
            None => Self(panic_or_wrap_mul::<SCALE>(self.0, rhs.0)),
        }
    }

    /// Divide two values of the same scale, rounding the final
    /// divide step according to `mode`.
    ///
    /// The default `Div` operator delegates to this with the
    /// crate-default rounding mode.
    ///
    /// # Panics
    ///
    /// Panics on division by zero (matching `i128 /`). Panics in debug
    /// builds when the quotient overflows `i128`; wraps in release.
    ///
    /// # Precision
    ///
    /// Strict: integer-only arithmetic. Within 0.5 ULP for the half-*
    /// family; directed rounding otherwise.
    #[inline]
    #[must_use]
    pub fn div_with(self, rhs: Self, mode: crate::support::rounding::RoundingMode) -> Self {
        assert!(rhs.0 != 0, "attempt to divide by zero");
        match crate::algos::mg_divide::div_pow10_div_with::<SCALE>(self.0, rhs.0, mode) {
            Some(q) => Self(q),
            None => Self(panic_or_wrap_div::<SCALE>(self.0, rhs.0)),
        }
    }
}

// Overflow fallback helpers for Mul and Div.
//
// The widening multiply/divide paths return `None` when the final `i128`
// quotient overflows. These helpers reproduce Rust's standard integer
// overflow contract: panic in debug builds, wrap two's-complement in
// release builds. The wrapping form re-derives the result from the
// original operands using `wrapping_*` intrinsics so it matches the naive
// form's behavior at operand magnitudes where the naive form was correct.

/// Emit a debug panic or return the wrapping result for a Mul overflow.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline(always)]
#[allow(clippy::arithmetic_side_effects)]
fn panic_or_wrap_mul<const SCALE: u32>(a: i128, b: i128) -> i128 {
    #[cfg(debug_assertions)]
    {
        let _ = (a, b);
        panic!("attempt to multiply with overflow");
    }
    #[cfg(not(debug_assertions))]
    {
        a.wrapping_mul(b).wrapping_div(D38::<SCALE>::multiplier())
    }
}

/// Emit a debug panic or return the wrapping result for a Div overflow.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline(always)]
#[allow(clippy::arithmetic_side_effects)]
fn panic_or_wrap_div<const SCALE: u32>(a: i128, b: i128) -> i128 {
    #[cfg(debug_assertions)]
    {
        let _ = (a, b);
        panic!("attempt to divide with overflow");
    }
    #[cfg(not(debug_assertions))]
    {
        a.wrapping_mul(D38::<SCALE>::multiplier()).wrapping_div(b)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::widths::D38s12;

    /// ZERO + ZERO == ZERO.
    #[test]
    fn add_zero_to_zero_is_zero() {
        assert_eq!(D38s12::ZERO + D38s12::ZERO, D38s12::ZERO);
    }

    /// ZERO - ZERO == ZERO.
    #[test]
    fn sub_zero_from_zero_is_zero() {
        assert_eq!(D38s12::ZERO - D38s12::ZERO, D38s12::ZERO);
    }

    /// -ZERO == ZERO.
    #[test]
    fn neg_zero_is_zero() {
        assert_eq!(-D38s12::ZERO, D38s12::ZERO);
    }

    /// AddAssign mutates in place.
    #[test]
    fn add_assign_zero() {
        let mut v = D38s12::ZERO;
        v += D38s12::ZERO;
        assert_eq!(v, D38s12::ZERO);
    }

    /// SubAssign mutates in place.
    #[test]
    fn sub_assign_zero() {
        let mut v = D38s12::ZERO;
        v -= D38s12::ZERO;
        assert_eq!(v, D38s12::ZERO);
    }

    /// Canonical claim: `(a + b) - b == a` for representative values.
    /// At SCALE = 12, `a = 1.5 mm` is bits `1_500_000_000_000`, `b =
    /// 0.25 mm` is bits `250_000_000_000`.
    #[test]
    fn add_sub_round_trip_canonical_claim() {
        let a = D38s12::from_bits(1_500_000_000_000);
        let b = D38s12::from_bits(250_000_000_000);
        assert_eq!((a + b) - b, a);
    }

    /// Round-trip with a negative `a` to exercise sign handling.
    #[test]
    fn add_sub_round_trip_negative() {
        let a = D38s12::from_bits(-7_321_654_987_000);
        let b = D38s12::from_bits(42_000_000_000_000);
        assert_eq!((a + b) - b, a);
    }

    /// `ONE + ONE` is the scaled bit-pattern `2 * 10^12`.
    #[test]
    fn one_plus_one_is_two_in_scaled_bits() {
        let two = D38s12::ONE + D38s12::ONE;
        // 2 * 10^12 = 2_000_000_000_000
        assert_eq!(two.to_bits(), 2_000_000_000_000);
    }

    /// `-ONE + ONE == ZERO` -- additive inverse property.
    #[test]
    fn neg_one_plus_one_is_zero() {
        assert_eq!(-D38s12::ONE + D38s12::ONE, D38s12::ZERO);
    }

    /// Default policy: overflow panics in debug builds. Locks the
    /// debug-vs-release split documented in module docs. The matching
    /// release-build wrap behaviour is delegated to the toolchain's
    /// `i128 +` semantics; testing it here would require a release-
    /// only test gate that's overkill for a base-ops slice.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "overflow")]
    fn add_overflow_panics_in_debug() {
        let _ = D38s12::MAX + D38s12::ONE;
    }

    /// Default policy: underflow panics in debug builds.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "overflow")]
    fn sub_underflow_panics_in_debug() {
        let _ = D38s12::MIN - D38s12::ONE;
    }

    /// Default policy: `-MIN` panics in debug builds (i128::MIN has
    /// no positive counterpart in two's-complement).
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "overflow")]
    fn neg_min_panics_in_debug() {
        let _ = -D38s12::MIN;
    }

    /// AddAssign with non-zero values.
    #[test]
    fn add_assign_accumulates() {
        let mut v = D38s12::from_bits(100);
        v += D38s12::from_bits(250);
        assert_eq!(v.to_bits(), 350);
        v += D38s12::from_bits(-50);
        assert_eq!(v.to_bits(), 300);
    }

    /// SubAssign with non-zero values.
    #[test]
    fn sub_assign_accumulates() {
        let mut v = D38s12::from_bits(1000);
        v -= D38s12::from_bits(250);
        assert_eq!(v.to_bits(), 750);
    }

    // ── Mul / Div / Rem ──

    /// `ONE * ONE == ONE` -- multiplicative identity.
    #[test]
    fn mul_one_one_is_one() {
        assert_eq!(D38s12::ONE * D38s12::ONE, D38s12::ONE);
    }

    /// `ONE / ONE == ONE`.
    #[test]
    fn div_one_one_is_one() {
        assert_eq!(D38s12::ONE / D38s12::ONE, D38s12::ONE);
    }

    /// `ZERO % ONE == ZERO`.
    #[test]
    fn rem_zero_one_is_zero() {
        assert_eq!(D38s12::ZERO % D38s12::ONE, D38s12::ZERO);
    }

    /// `ZERO * x == ZERO` for representative non-trivial `x`.
    #[test]
    fn mul_zero_is_zero() {
        let x = D38s12::from_bits(1_500_000_000_000); // 1.5
        assert_eq!(D38s12::ZERO * x, D38s12::ZERO);
        assert_eq!(x * D38s12::ZERO, D38s12::ZERO);
    }

    /// `ONE * x == x` for representative `x` (left and right identity).
    #[test]
    fn mul_one_is_identity() {
        let x = D38s12::from_bits(1_500_000_000_000); // 1.5
        assert_eq!(D38s12::ONE * x, x);
        assert_eq!(x * D38s12::ONE, x);

        let y = D38s12::from_bits(-7_321_654_987_000); // -7.321...
        assert_eq!(D38s12::ONE * y, y);
        assert_eq!(y * D38s12::ONE, y);
    }

    /// `x / ONE == x`.
    #[test]
    fn div_one_is_identity() {
        let x = D38s12::from_bits(1_500_000_000_000);
        assert_eq!(x / D38s12::ONE, x);

        let y = D38s12::from_bits(-7_321_654_987_000);
        assert_eq!(y / D38s12::ONE, y);
    }

    /// `x / x == ONE` for non-zero x.
    #[test]
    fn div_self_is_one() {
        let x = D38s12::from_bits(1_500_000_000_000); // 1.5
        assert_eq!(x / x, D38s12::ONE);

        let y = D38s12::from_bits(-7_321_654_987_000);
        assert_eq!(y / y, D38s12::ONE);

        // ONE / ONE already covered; a smaller value to exercise the
        // promotion path.
        let small = D38s12::from_bits(1); // 1 LSB
        assert_eq!(small / small, D38s12::ONE);
    }

    /// `(x * 7) % x == 0` -- multiple-of property.
    #[test]
    fn rem_multiple_is_zero() {
        let x = D38s12::from_bits(3_500_000_000_000); // 3.5
        let seven = D38s12::ONE + D38s12::ONE + D38s12::ONE + D38s12::ONE
            + D38s12::ONE + D38s12::ONE + D38s12::ONE; // 7
        assert_eq!((x * seven) % x, D38s12::ZERO);
    }

    /// `x % x == ZERO` for non-zero x.
    #[test]
    fn rem_self_is_zero() {
        let x = D38s12::from_bits(1_500_000_000_000);
        assert_eq!(x % x, D38s12::ZERO);

        let y = D38s12::from_bits(-7_321_654_987_000);
        assert_eq!(y % y, D38s12::ZERO);
    }

    /// **Headline claim**: `1.1 + 2.2 == 3.3` exactly. This is the
    /// distinguishing property versus binary floats (`f64`'s
    /// `1.1 + 2.2 == 3.3000000000000003`). Uses known scaled
    /// bit-patterns rather than the (not-yet-shipped) `FromStr`.
    #[test]
    fn one_point_one_plus_two_point_two_equals_three_point_three() {
        let one_point_one = D38s12::from_bits(1_100_000_000_000); // 1.1
        let two_point_two = D38s12::from_bits(2_200_000_000_000); // 2.2
        let three_point_three = D38s12::from_bits(3_300_000_000_000); // 3.3
        assert_eq!(one_point_one + two_point_two, three_point_three);
    }

    /// `(a * b) / b == a` round-trip for representative non-trivial values.
    /// At SCALE = 12, picking moderate operands keeps `a.0 * b.0` well
    /// inside the i128 boundary.
    #[test]
    fn mul_round_trip_canonical_claim() {
        // a = 1.5, b = 2.5 -> a * b = 3.75; (3.75 / 2.5) == 1.5
        let a = D38s12::from_bits(1_500_000_000_000);
        let b = D38s12::from_bits(2_500_000_000_000);
        let product = a * b;
        assert_eq!(product, D38s12::from_bits(3_750_000_000_000));
        assert_eq!(product / b, a);

        // Negative-operand round-trip.
        let c = D38s12::from_bits(-7_321_654_987_000);
        let d = D38s12::from_bits(13_000_000_000); // 0.013
        let cd = c * d;
        assert_eq!(cd / d, c);
    }

    /// In-place MulAssign matches `Mul`.
    #[test]
    fn mul_assign_matches_mul() {
        let a = D38s12::from_bits(1_500_000_000_000);
        let b = D38s12::from_bits(2_500_000_000_000);
        let mut x = a;
        x *= b;
        assert_eq!(x, a * b);
    }

    /// In-place DivAssign matches `Div`.
    #[test]
    fn div_assign_matches_div() {
        let a = D38s12::from_bits(3_750_000_000_000);
        let b = D38s12::from_bits(2_500_000_000_000);
        let mut x = a;
        x /= b;
        assert_eq!(x, a / b);
    }

    /// In-place RemAssign matches `Rem`.
    #[test]
    fn rem_assign_matches_rem() {
        let a = D38s12::from_bits(7_500_000_000_000);
        let b = D38s12::from_bits(2_000_000_000_000); // 2.0
        let mut x = a;
        x %= b;
        assert_eq!(x, a % b);
    }

    /// `Mul` is commutative under canonical equality.
    #[test]
    fn mul_is_commutative() {
        let a = D38s12::from_bits(1_500_000_000_000);
        let b = D38s12::from_bits(2_500_000_000_000);
        assert_eq!(a * b, b * a);
    }

    /// `Mul` rescales correctly: 0.5 * 0.5 == 0.25 (bit-exact).
    #[test]
    fn mul_subunit_rescales_exactly() {
        let half = D38s12::from_bits(500_000_000_000); // 0.5
        let quarter = D38s12::from_bits(250_000_000_000); // 0.25
        assert_eq!(half * half, quarter);
    }

    /// `Div` rescales correctly: 0.5 / 2 == 0.25.
    #[test]
    fn div_rescales_exactly() {
        let half = D38s12::from_bits(500_000_000_000); // 0.5
        let two = D38s12::from_bits(2_000_000_000_000); // 2.0
        let quarter = D38s12::from_bits(250_000_000_000); // 0.25
        assert_eq!(half / two, quarter);
    }

    /// `Rem` matches `i128 %` truncated-toward-zero semantics.
    /// 5.5 % 2.0 == 1.5 (since 5.5 = 2 * 2.0 + 1.5).
    #[test]
    fn rem_truncates_toward_zero() {
        let a = D38s12::from_bits(5_500_000_000_000);
        let b = D38s12::from_bits(2_000_000_000_000);
        let expected = D38s12::from_bits(1_500_000_000_000);
        assert_eq!(a % b, expected);

        // Negative dividend keeps the sign of the dividend (matches i128 %).
        let neg = D38s12::from_bits(-5_500_000_000_000);
        let neg_expected = D38s12::from_bits(-1_500_000_000_000);
        assert_eq!(neg % b, neg_expected);
    }

    /// Default policy: Mul overflow panics in debug. The product
    /// `MAX * 2` overflows the FINAL i128 quotient (256-bit
    /// intermediate doesn't matter -- the result still can't fit).
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "overflow")]
    fn mul_overflow_panics_in_debug() {
        let two = D38s12::from_bits(2_000_000_000_000);
        let _ = D38s12::MAX * two;
    }

    /// Widening multiply correctness: at operand magnitudes above the
    /// naive form's `sqrt(i128::MAX)` ~= 1.3e19 boundary, the widening
    /// multiply produces the correct result.
    ///
    /// Operands chosen around +/- 5e22. Expected result hand-computed:
    /// `(5e22 * 3e22) / 10^12 = 1.5e33`, which fits comfortably inside
    /// the final i128 range (i128::MAX ~= 1.7e38).
    #[test]
    fn mul_wide_operands_match_widened_form() {
        let a = D38s12::from_bits(50_000_000_000_000_000_000_000);
        let b = D38s12::from_bits(30_000_000_000_000_000_000_000);
        let expected = D38s12::from_bits(1_500_000_000_000_000_000_000_000_000_000_000);
        assert_eq!(a * b, expected);
        // Symmetric.
        assert_eq!(b * a, expected);
    }

    /// Signed round-trip at wide operand magnitudes: `(a * b) / b == a`.
    #[test]
    fn mul_div_wide_round_trip() {
        let a = D38s12::from_bits(50_000_000_000_000_000_000_000);
        let b = D38s12::from_bits(30_000_000_000_000_000_000_000);
        let prod = a * b;
        // Round-trip: prod / b should recover a.
        assert_eq!(prod / b, a);
    }

    /// Sign handling at wide operand magnitudes: mixed and same signs.
    #[test]
    fn mul_wide_negative_signs() {
        let a = D38s12::from_bits(50_000_000_000_000_000_000_000);
        let b = D38s12::from_bits(30_000_000_000_000_000_000_000);
        let neg_a = -a;
        let neg_b = -b;
        let pos_prod = a * b;
        let neg_prod = -pos_prod;
        assert_eq!(neg_a * b, neg_prod);
        assert_eq!(a * neg_b, neg_prod);
        assert_eq!(neg_a * neg_b, pos_prod);
    }

    /// Widening divide correctness at large dividend magnitudes.
    #[test]
    fn div_wide_dividend_correct() {
        // a = 10^22 raw (~10^10 in scaled value at SCALE=12)
        let a = D38s12::from_bits(10_i128.pow(22));
        // b = 2 raw (sub-LSB; effectively divides by 2 * 10^-12)
        let b = D38s12::from_bits(2);
        // Expected: (a.0 * 10^12) / b.0 = (10^34) / 2 = 5e33.
        let expected = D38s12::from_bits(5 * 10_i128.pow(33));
        assert_eq!(a / b, expected);
    }

    /// Widening divide round-trip: forces the numerator widening path
    /// because `a * 10^12` exceeds `i128::MAX`.
    #[test]
    fn div_wide_round_trip_exact() {
        // a = 10^27 raw: a * 10^12 = 10^39 > i128::MAX (1.7e38).
        // Divide by b = 100 raw: q = 10^39 / 100 = 10^37, which fits i128.
        let a = D38s12::from_bits(10_i128.pow(27));
        let b = D38s12::from_bits(100);
        let q = a / b;
        // q = (10^27 * 10^12) / 100 = 10^37 raw.
        let expected = D38s12::from_bits(10_i128.pow(37));
        assert_eq!(q, expected);
    }

    /// Div at SCALE = 0: scale-narrowing step is `a / b`, rounded per
    /// the crate-default mode (HalfToEven by default).
    #[test]
    fn div_scale_zero_matches_i128_div() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        type D0 = crate::types::widths::D38<0>;
        let a = D0::from_bits(15);
        let b = D0::from_bits(4);
        // 15 / 4 = 3.75 -> 4 under HalfToEven (no tie at .75).
        assert_eq!(a / b, D0::from_bits(4));
        assert_eq!((-a) / b, D0::from_bits(-4));
        // Exact divide is unchanged.
        let c = D0::from_bits(16);
        assert_eq!(c / b, D0::from_bits(4));
    }

    /// Mul at SCALE = 0: reduces to plain `i128 *`.
    #[test]
    fn mul_scale_zero_matches_i128_mul() {
        type D0 = crate::types::widths::D38<0>;
        let a = D0::from_bits(7);
        let b = D0::from_bits(11);
        assert_eq!(a * b, D0::from_bits(77));
        assert_eq!((-a) * b, D0::from_bits(-77));
    }

    /// Default policy: division by zero panics.
    #[test]
    #[should_panic]
    fn div_by_zero_panics() {
        let _ = D38s12::ONE / D38s12::ZERO;
    }

    /// Default policy: remainder with zero divisor panics.
    #[test]
    #[should_panic]
    fn rem_by_zero_panics() {
        let _ = D38s12::ONE % D38s12::ZERO;
    }

    // ── Math methods ──

    // ── abs ──

    /// `abs(0) == 0`.
    #[test]
    fn abs_zero_is_zero() {
        assert_eq!(D38s12::ZERO.abs(), D38s12::ZERO);
    }

    /// `abs(positive) == positive`.
    #[test]
    fn abs_positive_is_self() {
        let x = D38s12::from_bits(1_500_000_000_000); // 1.5
        assert_eq!(x.abs(), x);
    }

    /// `abs(negative) == positive(magnitude)`.
    #[test]
    fn abs_negative_is_positive() {
        let neg = D38s12::from_bits(-1_500_000_000_000);
        let pos = D38s12::from_bits(1_500_000_000_000);
        assert_eq!(neg.abs(), pos);
    }

    /// `abs(MIN)` panics in debug builds (no positive counterpart in
    /// two's-complement). Locks the panic-debug policy.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "overflow")]
    fn abs_min_panics_in_debug() {
        let _ = D38s12::MIN.abs();
    }

    // ── signum ──

    /// `signum(0) == ZERO` (no sign for zero).
    #[test]
    fn signum_zero_is_zero() {
        assert_eq!(D38s12::ZERO.signum(), D38s12::ZERO);
    }

    /// `signum(positive) == ONE`.
    #[test]
    fn signum_positive_is_one() {
        let x = D38s12::from_bits(1_500_000_000_000);
        assert_eq!(x.signum(), D38s12::ONE);

        // Smallest positive (1 LSB).
        let tiny = D38s12::from_bits(1);
        assert_eq!(tiny.signum(), D38s12::ONE);
    }

    /// `signum(negative) == -ONE`.
    #[test]
    fn signum_negative_is_neg_one() {
        let x = D38s12::from_bits(-1_500_000_000_000);
        assert_eq!(x.signum(), -D38s12::ONE);

        let tiny_neg = D38s12::from_bits(-1);
        assert_eq!(tiny_neg.signum(), -D38s12::ONE);
    }

    // ── floor ──

    /// `floor(2.5) == 2.0` (positive fractional rounds down).
    #[test]
    fn floor_positive_fractional_rounds_down() {
        let x = D38s12::from_bits(2_500_000_000_000);
        let expected = D38s12::from_bits(2_000_000_000_000);
        assert_eq!(x.floor(), expected);
    }

    /// `floor(-2.5) == -3.0` (negative fractional rounds toward
    /// negative infinity, NOT toward zero -- this is the key sign
    /// distinction from `trunc`).
    #[test]
    fn floor_negative_fractional_rounds_down_toward_neg_inf() {
        let x = D38s12::from_bits(-2_500_000_000_000);
        let expected = D38s12::from_bits(-3_000_000_000_000);
        assert_eq!(x.floor(), expected);

        // Smaller fractional part: -0.5 -> -1.0
        let small_neg = D38s12::from_bits(-500_000_000_000);
        let small_expected = D38s12::from_bits(-1_000_000_000_000);
        assert_eq!(small_neg.floor(), small_expected);
    }

    /// `floor(integer) == integer` (already at an integer boundary).
    #[test]
    fn floor_integer_unchanged() {
        let two = D38s12::from_bits(2_000_000_000_000);
        assert_eq!(two.floor(), two);

        let neg_two = D38s12::from_bits(-2_000_000_000_000);
        assert_eq!(neg_two.floor(), neg_two);

        assert_eq!(D38s12::ZERO.floor(), D38s12::ZERO);
    }

    // ── ceil ──

    /// `ceil(2.5) == 3.0`.
    #[test]
    fn ceil_positive_fractional_rounds_up() {
        let x = D38s12::from_bits(2_500_000_000_000);
        let expected = D38s12::from_bits(3_000_000_000_000);
        assert_eq!(x.ceil(), expected);
    }

    /// `ceil(-2.5) == -2.0`. Sign distinction from `floor`: ceiling
    /// rounds toward positive infinity.
    #[test]
    fn ceil_negative_fractional_rounds_up_toward_pos_inf() {
        let x = D38s12::from_bits(-2_500_000_000_000);
        let expected = D38s12::from_bits(-2_000_000_000_000);
        assert_eq!(x.ceil(), expected);

        // -0.5 -> 0
        let small_neg = D38s12::from_bits(-500_000_000_000);
        assert_eq!(small_neg.ceil(), D38s12::ZERO);
    }

    /// `ceil(integer) == integer`.
    #[test]
    fn ceil_integer_unchanged() {
        let two = D38s12::from_bits(2_000_000_000_000);
        assert_eq!(two.ceil(), two);

        let neg_two = D38s12::from_bits(-2_000_000_000_000);
        assert_eq!(neg_two.ceil(), neg_two);

        assert_eq!(D38s12::ZERO.ceil(), D38s12::ZERO);
    }

    // ── round ──

    /// Half-away-from-zero locked policy. Property test asserting:
    /// - 2.5 -> 3.0 (positive half rounds up)
    /// - 2.4 -> 2.0
    /// - 2.6 -> 3.0
    /// - -2.5 -> -3.0 (negative half rounds away from zero, i.e. down)
    /// - -2.4 -> -2.0
    /// - -2.6 -> -3.0
    #[test]
    fn round_half_away_from_zero() {
        // Positive halves
        let two_point_five = D38s12::from_bits(2_500_000_000_000);
        assert_eq!(two_point_five.round(), D38s12::from_bits(3_000_000_000_000));

        let two_point_four = D38s12::from_bits(2_400_000_000_000);
        assert_eq!(two_point_four.round(), D38s12::from_bits(2_000_000_000_000));

        let two_point_six = D38s12::from_bits(2_600_000_000_000);
        assert_eq!(two_point_six.round(), D38s12::from_bits(3_000_000_000_000));

        // Negative halves -- away from zero == toward neg infinity
        let neg_two_point_five = D38s12::from_bits(-2_500_000_000_000);
        assert_eq!(neg_two_point_five.round(), D38s12::from_bits(-3_000_000_000_000));

        let neg_two_point_four = D38s12::from_bits(-2_400_000_000_000);
        assert_eq!(neg_two_point_four.round(), D38s12::from_bits(-2_000_000_000_000));

        let neg_two_point_six = D38s12::from_bits(-2_600_000_000_000);
        assert_eq!(neg_two_point_six.round(), D38s12::from_bits(-3_000_000_000_000));

        // Zero
        assert_eq!(D38s12::ZERO.round(), D38s12::ZERO);
    }

    // ── trunc / fract ──

    /// `trunc` drops the fractional part (rounds toward zero), unlike
    /// `floor` which rounds toward negative infinity.
    #[test]
    fn trunc_drops_fractional() {
        // Positive
        let x = D38s12::from_bits(2_500_000_000_000);
        assert_eq!(x.trunc(), D38s12::from_bits(2_000_000_000_000));

        // Negative -- key sign distinction: trunc(-2.5) == -2.0
        // (floor(-2.5) would be -3.0)
        let neg = D38s12::from_bits(-2_500_000_000_000);
        assert_eq!(neg.trunc(), D38s12::from_bits(-2_000_000_000_000));

        // Zero
        assert_eq!(D38s12::ZERO.trunc(), D38s12::ZERO);

        // Already-integer values
        let two = D38s12::from_bits(2_000_000_000_000);
        assert_eq!(two.trunc(), two);
    }

    /// `fract` keeps only the fractional part. Sign matches the sign
    /// of `self` (because `trunc` rounds toward zero).
    #[test]
    fn fract_keeps_only_fractional() {
        let x = D38s12::from_bits(2_500_000_000_000);
        assert_eq!(x.fract(), D38s12::from_bits(500_000_000_000));

        // Negative: fract preserves dividend sign
        let neg = D38s12::from_bits(-2_500_000_000_000);
        assert_eq!(neg.fract(), D38s12::from_bits(-500_000_000_000));

        // Integer values have zero fract
        let two = D38s12::from_bits(2_000_000_000_000);
        assert_eq!(two.fract(), D38s12::ZERO);

        assert_eq!(D38s12::ZERO.fract(), D38s12::ZERO);
    }

    /// Identity: `trunc(x) + fract(x) == x` for any `x`.
    #[test]
    fn trunc_plus_fract_equals_self() {
        let cases = [
            D38s12::from_bits(2_500_000_000_000),
            D38s12::from_bits(-2_500_000_000_000),
            D38s12::from_bits(7_321_654_987_000),
            D38s12::from_bits(-7_321_654_987_000),
            D38s12::ZERO,
            D38s12::ONE,
            -D38s12::ONE,
            D38s12::from_bits(1), // sub-LSB fractional
            D38s12::from_bits(-1),
        ];
        for x in cases {
            assert_eq!(x.trunc() + x.fract(), x, "failed for {:?}", x);
        }
    }

    // ── min / max / clamp ──

    /// Basic min/max/clamp on representative values.
    #[test]
    fn min_max_clamp_basic() {
        let a = D38s12::from_bits(1_000_000_000_000); // 1.0
        let b = D38s12::from_bits(2_000_000_000_000); // 2.0
        let c = D38s12::from_bits(3_000_000_000_000); // 3.0

        assert_eq!(a.min(b), a);
        assert_eq!(b.min(a), a);
        assert_eq!(a.max(b), b);
        assert_eq!(b.max(a), b);

        // clamp inside range -- pass through
        assert_eq!(b.clamp(a, c), b);
        // clamp below lo
        assert_eq!(D38s12::ZERO.clamp(a, c), a);
        // clamp above hi
        let four = D38s12::from_bits(4_000_000_000_000);
        assert_eq!(four.clamp(a, c), c);

        // Negative values
        let neg_a = -a;
        let neg_b = -b;
        assert_eq!(neg_a.min(neg_b), neg_b); // -2.0 < -1.0
        assert_eq!(neg_a.max(neg_b), neg_a);
    }

    // ── recip ──

    /// `recip(2.0) == 0.5`, `recip(0.5) == 2.0`.
    #[test]
    fn recip_inverts_known_values() {
        let two = D38s12::from_bits(2_000_000_000_000);
        let half = D38s12::from_bits(500_000_000_000);
        assert_eq!(two.recip(), half);
        assert_eq!(half.recip(), two);

        // recip of ONE is ONE
        assert_eq!(D38s12::ONE.recip(), D38s12::ONE);

        // recip of -ONE is -ONE
        assert_eq!((-D38s12::ONE).recip(), -D38s12::ONE);
    }

    /// `recip(ZERO)` panics (division by zero).
    #[test]
    #[should_panic]
    fn recip_zero_panics() {
        let _ = D38s12::ZERO.recip();
    }

    // ── copysign ──

    /// Magnitude of self, sign of `sign` arg.
    #[test]
    fn copysign_basic() {
        let pos = D38s12::from_bits(1_500_000_000_000);
        let neg = D38s12::from_bits(-1_500_000_000_000);

        // copysign(pos, pos) == pos
        assert_eq!(pos.copysign(pos), pos);
        // copysign(pos, neg) == neg
        assert_eq!(pos.copysign(neg), neg);
        // copysign(neg, pos) == pos
        assert_eq!(neg.copysign(pos), pos);
        // copysign(neg, neg) == neg
        assert_eq!(neg.copysign(neg), neg);
    }

    /// `copysign(x, ZERO)` -- zero is treated as positive (no negative
    /// zero in i128). This locks the v1 policy.
    #[test]
    fn copysign_zero() {
        let neg = D38s12::from_bits(-1_500_000_000_000);
        let pos = D38s12::from_bits(1_500_000_000_000);

        // sign == ZERO -> positive magnitude
        assert_eq!(neg.copysign(D38s12::ZERO), pos);
        assert_eq!(pos.copysign(D38s12::ZERO), pos);

        // self == ZERO -> result == ZERO regardless of sign
        assert_eq!(D38s12::ZERO.copysign(neg), D38s12::ZERO);
        assert_eq!(D38s12::ZERO.copysign(pos), D38s12::ZERO);
    }

    // ── div_euclid / rem_euclid ──

    /// Positive operands match plain integer division.
    /// 5.0 / 2.0 = 2.5; div_euclid -> floor = 2.0; rem_euclid -> 1.0.
    #[test]
    fn div_euclid_positive() {
        let a = D38s12::from_bits(5_000_000_000_000); // 5.0
        let b = D38s12::from_bits(2_000_000_000_000); // 2.0

        let q = a.div_euclid(b);
        assert_eq!(q, D38s12::from_bits(2_000_000_000_000)); // 2

        let r = a.rem_euclid(b);
        assert_eq!(r, D38s12::from_bits(1_000_000_000_000)); // 1

        // Identity: q*b + r == a
        assert_eq!(q * b + r, a);
    }

    /// Negative dividend: -5.0 div_euclid 2.0 -> -3.0 (Euclidean, with
    /// non-negative remainder).
    #[test]
    fn div_euclid_negative_dividend() {
        let a = D38s12::from_bits(-5_000_000_000_000); // -5.0
        let b = D38s12::from_bits(2_000_000_000_000); // 2.0

        let q = a.div_euclid(b);
        // -5 = -3*2 + 1, so quotient = -3, rem = 1
        assert_eq!(q, D38s12::from_bits(-3_000_000_000_000));

        let r = a.rem_euclid(b);
        assert_eq!(r, D38s12::from_bits(1_000_000_000_000));

        // Identity: q*b + r == a
        assert_eq!(q * b + r, a);
    }

    /// Negative divisor: 5.0 div_euclid -2.0 -> -2.0 (Euclidean keeps
    /// remainder non-negative).
    #[test]
    fn div_euclid_negative_divisor() {
        let a = D38s12::from_bits(5_000_000_000_000); // 5.0
        let b = D38s12::from_bits(-2_000_000_000_000); // -2.0

        let q = a.div_euclid(b);
        assert_eq!(q, D38s12::from_bits(-2_000_000_000_000)); // -2

        let r = a.rem_euclid(b);
        assert_eq!(r, D38s12::from_bits(1_000_000_000_000)); // 1 (non-negative!)

        // Identity: q*b + r == a
        assert_eq!(q * b + r, a);
    }

    /// Property: `(a.div_euclid(b)) * b + a.rem_euclid(b) == a` for
    /// representative sign combinations.
    #[test]
    fn rem_euclid_consistency_with_div_euclid() {
        let cases: &[(i128, i128)] = &[
            (5_000_000_000_000, 2_000_000_000_000),
            (-5_000_000_000_000, 2_000_000_000_000),
            (5_000_000_000_000, -2_000_000_000_000),
            (-5_000_000_000_000, -2_000_000_000_000),
            (7_321_654_987_000, 13_000_000_000),
            (-7_321_654_987_000, 13_000_000_000),
        ];
        for (a_bits, b_bits) in cases {
            let a = D38s12::from_bits(*a_bits);
            let b = D38s12::from_bits(*b_bits);
            let q = a.div_euclid(b);
            let r = a.rem_euclid(b);
            assert_eq!(q * b + r, a, "failed for a={}, b={}", a_bits, b_bits);
            // Remainder must be non-negative (Euclidean property)
            assert!(r.0 >= 0, "rem_euclid returned negative for a={}, b={}: {}",
                    a_bits, b_bits, r.0);
        }
    }

    // ── div_floor / div_ceil ──

    /// `div_floor` rounds toward negative infinity. Positive operands
    /// match plain truncating div.
    #[test]
    fn div_floor_basic() {
        // 5.0 / 2.0 -> floor(2.5) = 2.0
        let a = D38s12::from_bits(5_000_000_000_000);
        let b = D38s12::from_bits(2_000_000_000_000);
        assert_eq!(a.div_floor(b), D38s12::from_bits(2_000_000_000_000));

        // -5.0 / 2.0 -> floor(-2.5) = -3.0
        let neg_a = D38s12::from_bits(-5_000_000_000_000);
        assert_eq!(neg_a.div_floor(b), D38s12::from_bits(-3_000_000_000_000));

        // -5.0 / -2.0 -> floor(2.5) = 2.0 (sign distinction from div_euclid)
        let neg_b = D38s12::from_bits(-2_000_000_000_000);
        assert_eq!(neg_a.div_floor(neg_b), D38s12::from_bits(2_000_000_000_000));

        // 5.0 / -2.0 -> floor(-2.5) = -3.0
        // (div_euclid here would be -2 because rem must be >= 0.)
        assert_eq!(a.div_floor(neg_b), D38s12::from_bits(-3_000_000_000_000));
    }

    /// `div_ceil` rounds toward positive infinity.
    #[test]
    fn div_ceil_basic() {
        // 5.0 / 2.0 -> ceil(2.5) = 3.0
        let a = D38s12::from_bits(5_000_000_000_000);
        let b = D38s12::from_bits(2_000_000_000_000);
        assert_eq!(a.div_ceil(b), D38s12::from_bits(3_000_000_000_000));

        // -5.0 / 2.0 -> ceil(-2.5) = -2.0
        let neg_a = D38s12::from_bits(-5_000_000_000_000);
        assert_eq!(neg_a.div_ceil(b), D38s12::from_bits(-2_000_000_000_000));

        // 4.0 / 2.0 -> exact -> 2.0
        let four = D38s12::from_bits(4_000_000_000_000);
        assert_eq!(four.div_ceil(b), D38s12::from_bits(2_000_000_000_000));
    }

    // ── abs_diff ──

    /// `abs_diff` is commutative and non-negative.
    #[test]
    fn abs_diff_commutative() {
        let a = D38s12::from_bits(5_000_000_000_000); // 5.0
        let b = D38s12::from_bits(2_000_000_000_000); // 2.0
        let expected = D38s12::from_bits(3_000_000_000_000); // 3.0

        assert_eq!(a.abs_diff(b), expected);
        assert_eq!(b.abs_diff(a), expected);

        // Negative operands
        let neg_a = -a;
        let neg_b = -b;
        // |(-5) - (-2)| = |-3| = 3
        assert_eq!(neg_a.abs_diff(neg_b), expected);
        assert_eq!(neg_b.abs_diff(neg_a), expected);

        // Mixed sign: |5 - (-2)| = 7
        let seven = D38s12::from_bits(7_000_000_000_000);
        assert_eq!(a.abs_diff(neg_b), seven);
        assert_eq!(neg_b.abs_diff(a), seven);
    }

    /// `abs_diff(x, x) == 0` and `abs_diff(x, 0) == abs(x)`.
    #[test]
    fn abs_diff_zero() {
        let x = D38s12::from_bits(1_500_000_000_000);
        assert_eq!(x.abs_diff(x), D38s12::ZERO);
        assert_eq!(x.abs_diff(D38s12::ZERO), x.abs());

        let neg = -x;
        assert_eq!(neg.abs_diff(D38s12::ZERO), x);
    }

    // ── midpoint ──

    /// Midpoint of two representative values.
    #[test]
    fn midpoint_basic() {
        let a = D38s12::from_bits(1_000_000_000_000); // 1.0
        let b = D38s12::from_bits(3_000_000_000_000); // 3.0
        assert_eq!(a.midpoint(b), D38s12::from_bits(2_000_000_000_000)); // 2.0

        // Negative
        let neg_a = -a;
        let neg_b = -b;
        assert_eq!(neg_a.midpoint(neg_b), D38s12::from_bits(-2_000_000_000_000));

        // Mixed sign: midpoint(-1, 1) == 0
        assert_eq!(neg_a.midpoint(a), D38s12::ZERO);
    }

    /// Midpoint near MAX must not overflow (the whole point of using
    /// `i128::midpoint` over `(a + b) / 2`).
    #[test]
    fn midpoint_no_overflow_at_max() {
        // (MAX + MAX) / 2 == MAX, but a naive (a+b)/2 would overflow.
        // i128::midpoint handles this without intermediate overflow.
        assert_eq!(D38s12::MAX.midpoint(D38s12::MAX), D38s12::MAX);
        assert_eq!(D38s12::MIN.midpoint(D38s12::MIN), D38s12::MIN);
        // midpoint(MIN, MAX) -- delegates to i128::midpoint. The
        // Rust 1.95 stabilised implementation rounds the average
        // toward zero for signed integers (so MIN + MAX = -1 averages
        // to 0, not -1). Just assert it doesn't overflow / panic.
        let mid = D38s12::MIN.midpoint(D38s12::MAX);
        assert!(mid.0 == 0 || mid.0 == -1,
                "midpoint(MIN, MAX) should be 0 or -1, got {}", mid.0);
    }

    // ── Float-shape compat predicates ──

    #[test]
    fn is_nan_always_false() {
        assert!(!D38s12::ZERO.is_nan());
        assert!(!D38s12::ONE.is_nan());
        assert!(!D38s12::MAX.is_nan());
        assert!(!D38s12::MIN.is_nan());
    }

    #[test]
    fn is_infinite_always_false() {
        assert!(!D38s12::ZERO.is_infinite());
        assert!(!D38s12::MAX.is_infinite());
        assert!(!D38s12::MIN.is_infinite());
    }

    #[test]
    fn is_finite_always_true() {
        assert!(D38s12::ZERO.is_finite());
        assert!(D38s12::ONE.is_finite());
        assert!(D38s12::MAX.is_finite());
        assert!(D38s12::MIN.is_finite());
    }

    #[test]
    fn is_normal_zero_is_false() {
        assert!(!D38s12::ZERO.is_normal());
    }

    #[test]
    fn is_normal_nonzero_is_true() {
        assert!(D38s12::ONE.is_normal());
        assert!((-D38s12::ONE).is_normal());
        assert!(D38s12::from_bits(1).is_normal()); // smallest positive
        assert!(D38s12::from_bits(-1).is_normal()); // smallest negative
        assert!(D38s12::MAX.is_normal());
        assert!(D38s12::MIN.is_normal());
    }

    /// is_zero / is_positive / is_negative resolve in the foundation
    /// slice (cheap predicates).
    #[test]
    fn is_zero_predicates() {
        assert!(D38s12::ZERO.is_zero());
        assert!(!D38s12::ZERO.is_positive());
        assert!(!D38s12::ZERO.is_negative());

        assert!(!D38s12::from_bits(1).is_zero());
        assert!(D38s12::from_bits(1).is_positive());
        assert!(!D38s12::from_bits(1).is_negative());

        assert!(!D38s12::from_bits(-1).is_zero());
        assert!(!D38s12::from_bits(-1).is_positive());
        assert!(D38s12::from_bits(-1).is_negative());
    }
}
