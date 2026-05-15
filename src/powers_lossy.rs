//! Lossy (f64-bridge) powers methods for D38.
//!
//! Companion to powers_strict.rs. The plain methods here are the
//! f64-bridge variants, gated on std + (no strict feature or
//! no_strict set). When strict is on, the dispatcher in the
//! _strict file shadows these.

use crate::core_type::D38;

impl<const SCALE: u32> D38<SCALE> {

    /// Raises `self` to the power `exp` via the f64 bridge.
    ///
    /// Converts both operands to f64, calls `f64::powf`, then converts
    /// the result back. For integer exponents, prefer [`Self::pow`] or
    /// [`Self::powi`], which are bit-exact.
    ///
    /// NaN results map to `ZERO`; infinities clamp to `MAX` or `MIN`,
    /// following the saturate-vs-error policy of [`Self::from_f64_lossy`].
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// let two = D38s12::from_int(2);
    /// let three = D38s12::from_int(3);
    /// // 2^3 = 8, within f64 precision.
    /// assert!((two.powf(three).to_f64_lossy() - 8.0).abs() < 1e-9);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn powf(self, exp: D38<SCALE>) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().powf(exp.to_f64_lossy()))
    }

    /// Returns the square root of `self` via the f64 bridge.
    ///
    /// IEEE 754 mandates that `f64::sqrt` is correctly-rounded
    /// (round-to-nearest, ties-to-even). Combined with the deterministic
    /// `to_f64_lossy` / `from_f64_lossy` round-trip, this makes
    /// `D38::sqrt` bit-deterministic: the same input produces the same
    /// output bit-pattern on every IEEE-754-conformant platform.
    ///
    /// Negative inputs produce a NaN from `f64::sqrt`, which
    /// [`Self::from_f64_lossy`] maps to `ZERO` per the saturate-vs-error
    /// policy. No panic is raised for negative inputs.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// assert_eq!(D38s12::ZERO.sqrt(), D38s12::ZERO);
    /// // f64::sqrt(1.0) == 1.0 exactly, so the result is bit-exact.
    /// assert_eq!(D38s12::ONE.sqrt(), D38s12::ONE);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().sqrt())
    }

    /// Returns the cube root of `self` via the f64 bridge.
    ///
    /// `f64::cbrt` is defined for the entire real line, including
    /// negative inputs (`cbrt(-8.0) == -2.0`). The result is
    /// bit-deterministic across IEEE-754-conformant platforms because
    /// `f64::cbrt` is correctly-rounded.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// let neg_eight = D38s12::from_int(-8);
    /// let result = neg_eight.cbrt();
    /// assert!((result.to_f64_lossy() - (-2.0_f64)).abs() < 1e-9);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn cbrt(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().cbrt())
    }

    // Integer power variant family.


    /// Returns `sqrt(self^2 + other^2)` without intermediate overflow.
    ///
    /// The naive form `(self * self + other * other).sqrt()` overflows
    /// `i128` once either operand approaches `sqrt(D38::MAX)`. This
    /// method uses the scale trick to avoid that:
    ///
    /// ```text
    /// hypot(a, b) = max(|a|, |b|) * sqrt(1 + (min(|a|, |b|) / max(|a|, |b|))^2)
    /// ```
    ///
    /// The `min/max` ratio is in `[0, 1]`, so `ratio^2` is also in
    /// `[0, 1]` and cannot overflow. The outer multiply by `large` only
    /// overflows when the true hypotenuse genuinely exceeds `D38::MAX`,
    /// which matches `f64::hypot`'s contract.
    ///
    /// Both inputs are absolute-valued before processing, so
    /// `hypot(-a, b) == hypot(a, b)`.
    ///
    /// Edge cases: `hypot(0, 0) == 0` (bit-exact via the early return);
    /// `hypot(0, x) ~= |x|` and `hypot(x, 0) ~= |x|`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// let three = D38s12::from_int(3);
    /// let four = D38s12::from_int(4);
    /// // Pythagorean triple: hypot(3, 4) ~= 5.
    /// assert!((three.hypot(four).to_f64_lossy() - 5.0).abs() < 1e-9);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn hypot(self, other: Self) -> Self {
        let a = self.abs();
        let b = other.abs();
        let (large, small) = if a >= b { (a, b) } else { (b, a) };
        if large == Self::ZERO {
            // Both inputs are zero; large is the max of two non-negatives,
            // so this branch is only reached when both are zero.
            Self::ZERO
        } else {
            let ratio = small / large;
            // ratio^2 is in [0, 1]; ONE + ratio^2 is in [1, 2]; no overflow.
            // The outer sqrt is in [1, sqrt(2)]; the final multiply by large
            // only overflows when the true hypotenuse exceeds D38::MAX.
            let one_plus_sq = Self::ONE + ratio * ratio;
            large * one_plus_sq.sqrt()
        }
    }
}
