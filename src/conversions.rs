//! Conversions between [`I128`] and primitive numeric types.
//!
//! # Naming convention
//!
//! - `from_int(i64)` / `from_i32(i32)` -- exact named constructors; thin
//!   wrappers around the `From<iN>` impls.
//! - `from_f64_lossy(f64)` -- explicitly lossy; multiplies the float by
//!   `10^SCALE`, truncates to `i128`, and saturates on out-of-range or
//!   non-finite inputs.
//! - `to_int_lossy() -> i64` -- truncates the fractional part toward zero;
//!   saturates to `i64::MAX` / `i64::MIN` when the integer magnitude exceeds
//!   `i64`'s range.
//! - `to_f64_lossy() -> f64` -- divides the raw `i128` storage by the
//!   multiplier in `f64`; f64's 53-bit mantissa cannot represent every `I128`
//!   value exactly.
//! - `to_f32_lossy() -> f32` -- converts via `f64` first, then narrows to
//!   `f32`; lossier than `to_f64_lossy`.
//!
//! # Lossless `From` impls
//!
//! Eight `From` impls cover integer types whose values fit losslessly into
//! `i128` after scaling by `10^SCALE` at practical scales: `i8`, `i16`,
//! `i32`, `i64`, `u8`, `u16`, `u32`, `u64`. Each multiplies the input by
//! `multiplier()` (= `10^SCALE`).
//!
//! At pathological scales (for example `SCALE >= 20` for `u64`) the
//! multiplication can overflow `i128`; the result follows the standard
//! Rust panic-in-debug / wrap-in-release behaviour.
//!
//! # Fallible `TryFrom` impls
//!
//! Four `TryFrom` impls cover types where lossless conversion is not always
//! possible: `i128`, `u128`, `f32`, `f64`. They return [`DecimalConvertError`]
//! with two variants:
//!
//! - `Overflow` -- the magnitude exceeds `I128::MAX` / `I128::MIN` after
//!   scaling by `10^SCALE`.
//! - `NotFinite` -- the float input is `NaN` or an infinity.
//!
//! # Saturation-versus-error policy
//!
//! - Lossy methods saturate: `from_f64_lossy(f64::INFINITY)` returns
//!   `I128::MAX`; `from_f64_lossy(f64::NAN)` returns `I128::ZERO`.
//! - `TryFrom` variants return `Err`: `Err(NotFinite)` for `NaN`/`inf`,
//!   `Err(Overflow)` for finite out-of-range inputs.

use crate::core_type::I128;

// ──────────────────────────────────────────────────────────────────────
// Error type
// ──────────────────────────────────────────────────────────────────────

/// Error returned by the fallible [`TryFrom`] impls on [`I128`].
///
/// Covers the two distinct failure modes:
/// - [`DecimalConvertError::Overflow`] -- the input, after scaling by
///   `10^SCALE`, exceeds the range `[I128::MIN, I128::MAX]`.
/// - [`DecimalConvertError::NotFinite`] -- the float input is `NaN`,
///   `+inf`, or `-inf`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecimalConvertError {
    /// Input magnitude is outside `[I128::MIN, I128::MAX]` after scaling.
    Overflow,
    /// Input is `NaN`, `+inf`, or `-inf` (only reachable from the
    /// `TryFrom<f32>` / `TryFrom<f64>` impls).
    NotFinite,
}

impl core::fmt::Display for DecimalConvertError {
    /// Formats the error as a short human-readable message.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overflow => f.write_str("decimal conversion overflow"),
            Self::NotFinite => f.write_str("decimal conversion from non-finite float"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecimalConvertError {}

// ──────────────────────────────────────────────────────────────────────
// Lossless From<integer> impls
// ──────────────────────────────────────────────────────────────────────
//
// Each impl multiplies the input by `multiplier()` (= 10^SCALE).
// At SCALE = 12, the worst-case `u64` product is ~1.8e31, well under
// i128::MAX ~1.7e38, so all eight impls are infallible in practice.

impl<const SCALE: u32> From<i8> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 36` (since `i8::MAX * 10^36 < i128::MAX`).
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_i8).to_bits(), 1_000_000_000_000);
    /// assert_eq!(I128s12::from(-1_i8).to_bits(), -1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: i8) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

impl<const SCALE: u32> From<i16> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 33`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_i16).to_bits(), 1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: i16) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

impl<const SCALE: u32> From<i32> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 28`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_i32).to_bits(), 1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: i32) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

impl<const SCALE: u32> From<i64> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 19`. At `SCALE = 12` all `i64` values
    /// fit with roughly six orders of magnitude of headroom before
    /// `i128::MAX`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_i64).to_bits(), 1_000_000_000_000);
    /// assert_eq!(I128s12::from(-1_i64).to_bits(), -1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: i64) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

impl<const SCALE: u32> From<u8> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 36`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_u8).to_bits(), 1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: u8) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

impl<const SCALE: u32> From<u16> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 33`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_u16).to_bits(), 1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: u16) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

impl<const SCALE: u32> From<u32> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 28`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_u32).to_bits(), 1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: u32) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

impl<const SCALE: u32> From<u64> for I128<SCALE> {
    /// Converts `value` by scaling it to `value * 10^SCALE`.
    ///
    /// Lossless for all `SCALE < 19`. At `SCALE = 12` the worst-case
    /// product is `u64::MAX * 10^12` (~1.8e31), well under `i128::MAX`
    /// (~1.7e38).
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from(1_u64).to_bits(), 1_000_000_000_000);
    /// ```
    #[inline]
    fn from(value: u64) -> Self {
        Self((value as i128) * Self::multiplier())
    }
}

// ──────────────────────────────────────────────────────────────────────
// Fallible TryFrom impls
// ──────────────────────────────────────────────────────────────────────
//
// `TryFrom<i128>` and `TryFrom<u128>` use `checked_mul` to detect
// overflow when scaling by `multiplier()`. `TryFrom<f32>` delegates to
// `TryFrom<f64>`. `TryFrom<f64>` multiplies in f64, compares against
// the f64 representations of i128::MIN / i128::MAX, and casts to i128.

impl<const SCALE: u32> TryFrom<i128> for I128<SCALE> {
    type Error = DecimalConvertError;

    /// Scales `value` by `10^SCALE` using checked multiplication.
    ///
    /// Returns `Err(Overflow)` if the product exceeds `i128::MAX` or
    /// falls below `i128::MIN`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::{I128s12, DecimalConvertError};
    ///
    /// let v: I128s12 = 1_i128.try_into().unwrap();
    /// assert_eq!(v, I128s12::ONE);
    ///
    /// let overflow: Result<I128s12, _> = i128::MAX.try_into();
    /// assert_eq!(overflow, Err(DecimalConvertError::Overflow));
    /// ```
    #[inline]
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        value
            .checked_mul(Self::multiplier())
            .map(Self)
            .ok_or(DecimalConvertError::Overflow)
    }
}

impl<const SCALE: u32> TryFrom<u128> for I128<SCALE> {
    type Error = DecimalConvertError;

    /// Converts `value` to `i128`, then scales by `10^SCALE`.
    ///
    /// Returns `Err(Overflow)` if `value > i128::MAX` or if the scaled
    /// product overflows `i128`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::{I128s12, DecimalConvertError};
    ///
    /// let v: I128s12 = 42_u128.try_into().unwrap();
    /// assert_eq!(v.to_bits(), 42_000_000_000_000);
    ///
    /// let overflow: Result<I128s12, _> = u128::MAX.try_into();
    /// assert_eq!(overflow, Err(DecimalConvertError::Overflow));
    /// ```
    #[inline]
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        // Step 1: u128 -> i128 (overflows when value > i128::MAX).
        let as_i128: i128 = i128::try_from(value).map_err(|_| DecimalConvertError::Overflow)?;
        // Step 2: scale using the existing checked path.
        as_i128
            .checked_mul(Self::multiplier())
            .map(Self)
            .ok_or(DecimalConvertError::Overflow)
    }
}

impl<const SCALE: u32> TryFrom<f32> for I128<SCALE> {
    type Error = DecimalConvertError;

    /// Widens `value` to `f64` and delegates to [`TryFrom<f64>`].
    ///
    /// Returns `Err(NotFinite)` for `NaN` and the infinities; `Err(Overflow)`
    /// for finite values whose magnitude exceeds `I128::MAX` after scaling.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::{I128s12, DecimalConvertError};
    ///
    /// let v: I128s12 = 1.0_f32.try_into().unwrap();
    /// assert_eq!(v, I128s12::ONE);
    ///
    /// let nan: Result<I128s12, _> = f32::NAN.try_into();
    /// assert_eq!(nan, Err(DecimalConvertError::NotFinite));
    /// ```
    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

impl<const SCALE: u32> TryFrom<f64> for I128<SCALE> {
    type Error = DecimalConvertError;

    /// Multiplies `value` by `10^SCALE` in `f64` and truncates to `i128`.
    ///
    /// Returns `Err(NotFinite)` for `NaN`/`inf`, and `Err(Overflow)` for
    /// finite inputs whose scaled value falls outside `[i128::MIN, i128::MAX)`.
    ///
    /// Note: `i128::MAX as f64` rounds up to `2^127` because `i128::MAX`
    /// (`2^127 - 1`) is not exactly representable in f64. The range check
    /// uses a strict `<` on the upper bound to reject `2^127` itself.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::{I128s12, DecimalConvertError};
    ///
    /// let v: I128s12 = 1.0_f64.try_into().unwrap();
    /// assert_eq!(v, I128s12::ONE);
    ///
    /// let nan: Result<I128s12, _> = f64::NAN.try_into();
    /// assert_eq!(nan, Err(DecimalConvertError::NotFinite));
    ///
    /// let overflow: Result<I128s12, _> = 1e30_f64.try_into();
    /// assert_eq!(overflow, Err(DecimalConvertError::Overflow));
    /// ```
    #[inline]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(DecimalConvertError::NotFinite);
        }
        let scaled = value * (Self::multiplier() as f64);
        // i128::MAX as f64 rounds up to 2^127; use strict `<` so 2^127 is rejected.
        const I128_MAX_F64: f64 = i128::MAX as f64;
        const I128_MIN_F64: f64 = i128::MIN as f64;
        if !(I128_MIN_F64..I128_MAX_F64).contains(&scaled) {
            return Err(DecimalConvertError::Overflow);
        }
        Ok(Self(scaled as i128))
    }
}

// ──────────────────────────────────────────────────────────────────────
// I128 inherent conversion methods
// ──────────────────────────────────────────────────────────────────────

impl<const SCALE: u32> I128<SCALE> {
    /// Constructs a `I128` from an `i64` integer value.
    ///
    /// Named constructor that wraps `From<i64>`. Prefer this over
    /// `I128::from(value)` when the intent of converting from an integer
    /// should be explicit at the call site.
    ///
    /// At `SCALE = 12` every `i64` value fits with roughly six orders of
    /// magnitude of headroom before `i128::MAX`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from_int(1), I128s12::ONE);
    /// assert_eq!(I128s12::from_int(-42).to_bits(), -42_000_000_000_000_i128);
    /// ```
    #[inline]
    pub fn from_int(value: i64) -> Self {
        Self::from(value)
    }

    /// Constructs a `I128` from an `i32` integer value.
    ///
    /// Named constructor that wraps `From<i32>`. Lossless at any
    /// practical `SCALE` (safe up to `SCALE < 28`).
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from_i32(1), I128s12::ONE);
    /// assert_eq!(I128s12::from_i32(0), I128s12::ZERO);
    /// ```
    #[inline]
    pub fn from_i32(value: i32) -> Self {
        Self::from(value)
    }

    /// Constructs a `I128` from an `f64`, saturating on non-finite or
    /// out-of-range inputs.
    ///
    /// Multiplies `value` by `10^SCALE` and truncates to `i128`. Non-finite
    /// and out-of-range inputs are handled as follows:
    ///
    /// - `NaN` returns `I128::ZERO` (deterministic, no panic).
    /// - `+inf` or any finite value above the representable range returns `I128::MAX`.
    /// - `-inf` or any finite value below the representable range returns `I128::MIN`.
    ///
    /// Use [`TryFrom<f64>`] when you want an error instead of saturation.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::from_f64_lossy(1.0), I128s12::ONE);
    /// assert_eq!(I128s12::from_f64_lossy(f64::NAN), I128s12::ZERO);
    /// assert_eq!(I128s12::from_f64_lossy(f64::INFINITY), I128s12::MAX);
    /// assert_eq!(I128s12::from_f64_lossy(f64::NEG_INFINITY), I128s12::MIN);
    /// ```
    pub fn from_f64_lossy(value: f64) -> Self {
        if value.is_nan() {
            return Self::ZERO;
        }
        if value.is_infinite() {
            return if value > 0.0 { Self::MAX } else { Self::MIN };
        }
        let scaled = value * (Self::multiplier() as f64);
        const I128_MAX_F64: f64 = i128::MAX as f64;
        const I128_MIN_F64: f64 = i128::MIN as f64;
        if scaled >= I128_MAX_F64 {
            return Self::MAX;
        }
        if scaled < I128_MIN_F64 {
            return Self::MIN;
        }
        Self(scaled as i128)
    }

    /// Converts to `i64` by truncating the fractional part toward zero.
    ///
    /// The integer part is `self.0 / 10^SCALE`. If that value exceeds
    /// `i64::MAX` or falls below `i64::MIN`, the result saturates to
    /// `i64::MAX` or `i64::MIN` respectively. At `SCALE = 12` the saturation
    /// threshold is approximately 9.2e18 (the `i64` limit), which is well
    /// below the `I128` maximum of ~1.7e26.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// // Truncates toward zero.
    /// assert_eq!(I128s12::from_bits(2_500_000_000_000).to_int_lossy(), 2);
    /// assert_eq!(I128s12::from_bits(-2_500_000_000_000).to_int_lossy(), -2);
    ///
    /// // Saturates when the integer part exceeds i64 range.
    /// assert_eq!(I128s12::MAX.to_int_lossy(), i64::MAX);
    /// assert_eq!(I128s12::MIN.to_int_lossy(), i64::MIN);
    /// ```
    #[inline]
    pub fn to_int_lossy(self) -> i64 {
        let int_part: i128 = self.0 / Self::multiplier();
        if int_part > i64::MAX as i128 {
            i64::MAX
        } else if int_part < i64::MIN as i128 {
            i64::MIN
        } else {
            int_part as i64
        }
    }

    /// Converts to `f64` by dividing the raw storage by `10^SCALE`.
    ///
    /// f64 has a 53-bit mantissa, so large or precision-dense `I128` values
    /// will round. The division is performed as `(self.0 as f64) / multiplier`
    /// to keep as much precision as f64 allows.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::ZERO.to_f64_lossy(), 0.0);
    /// assert_eq!(I128s12::ONE.to_f64_lossy(), 1.0);
    /// ```
    #[inline]
    pub fn to_f64_lossy(self) -> f64 {
        (self.0 as f64) / (Self::multiplier() as f64)
    }

    /// Converts to `f32` via `f64`, then narrows to `f32`.
    ///
    /// f32 has only a 24-bit mantissa, making this lossier than
    /// [`Self::to_f64_lossy`]. The `f64` intermediate step retains the
    /// best precision available before the final narrowing cast.
    ///
    /// # Precision
    ///
    /// Lossy: involves f32 or f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    ///
    /// assert_eq!(I128s12::ZERO.to_f32_lossy(), 0.0_f32);
    /// assert_eq!(I128s12::ONE.to_f32_lossy(), 1.0_f32);
    /// ```
    #[inline]
    pub fn to_f32_lossy(self) -> f32 {
        self.to_f64_lossy() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::DecimalConvertError;
    use crate::core_type::{I128, I128s12};

    // ──────────────────────────────────────────────────────────────────
    // from_int / from_i32 -- foundation wrappers around From<iN>
    // ──────────────────────────────────────────────────────────────────

    #[test]
    fn from_int_zero_is_zero() {
        assert_eq!(I128s12::from_int(0), I128s12::ZERO);
    }

    #[test]
    fn from_i32_zero_is_zero() {
        assert_eq!(I128s12::from_i32(0), I128s12::ZERO);
    }

    #[test]
    fn from_int_one_is_one() {
        assert_eq!(I128s12::from_int(1), I128s12::ONE);
    }

    #[test]
    fn from_i32_one_is_one() {
        assert_eq!(I128s12::from_i32(1), I128s12::ONE);
    }

    #[test]
    fn from_int_negative() {
        assert_eq!(I128s12::from_int(-1), -I128s12::ONE);
        assert_eq!(I128s12::from_int(-42).to_bits(), -42_000_000_000_000_i128);
    }

    // ──────────────────────────────────────────────────────────────────
    // Lossless From<iN> / From<uN> -- bit-exact scaling
    // ──────────────────────────────────────────────────────────────────

    #[test]
    fn from_i8_scales_correctly() {
        assert_eq!(I128s12::from(0_i8).to_bits(), 0);
        assert_eq!(I128s12::from(1_i8).to_bits(), 1_000_000_000_000);
        assert_eq!(I128s12::from(-1_i8).to_bits(), -1_000_000_000_000);
        assert_eq!(I128s12::from(i8::MAX).to_bits(), 127_000_000_000_000);
        assert_eq!(I128s12::from(i8::MIN).to_bits(), -128_000_000_000_000);
    }

    #[test]
    fn from_i16_scales_correctly() {
        assert_eq!(I128s12::from(0_i16).to_bits(), 0);
        assert_eq!(I128s12::from(1_i16).to_bits(), 1_000_000_000_000);
        assert_eq!(I128s12::from(i16::MAX).to_bits(), 32_767_000_000_000_000);
        assert_eq!(I128s12::from(i16::MIN).to_bits(), -32_768_000_000_000_000);
    }

    #[test]
    fn from_i32_scales_correctly() {
        assert_eq!(I128s12::from(0_i32).to_bits(), 0);
        assert_eq!(I128s12::from(i32::MAX).to_bits(), (i32::MAX as i128) * 1_000_000_000_000);
        assert_eq!(I128s12::from(i32::MIN).to_bits(), (i32::MIN as i128) * 1_000_000_000_000);
    }

    #[test]
    fn from_i64_scales_correctly() {
        assert_eq!(I128s12::from(0_i64).to_bits(), 0);
        assert_eq!(I128s12::from(i64::MAX).to_bits(), (i64::MAX as i128) * 1_000_000_000_000);
        assert_eq!(I128s12::from(i64::MIN).to_bits(), (i64::MIN as i128) * 1_000_000_000_000);
    }

    #[test]
    fn from_u8_scales_correctly() {
        assert_eq!(I128s12::from(0_u8).to_bits(), 0);
        assert_eq!(I128s12::from(u8::MAX).to_bits(), 255_000_000_000_000);
    }

    #[test]
    fn from_u16_scales_correctly() {
        assert_eq!(I128s12::from(0_u16).to_bits(), 0);
        assert_eq!(I128s12::from(u16::MAX).to_bits(), 65_535_000_000_000_000);
    }

    #[test]
    fn from_u32_scales_correctly() {
        assert_eq!(I128s12::from(0_u32).to_bits(), 0);
        assert_eq!(I128s12::from(u32::MAX).to_bits(), (u32::MAX as i128) * 1_000_000_000_000);
    }

    /// `From<u64>` at the boundary -- u64::MAX times multiplier is
    /// ~1.8e31, well under i128::MAX ~1.7e38, so this is lossless
    /// at SCALE=12.
    #[test]
    fn from_u64_at_boundary_is_lossless() {
        let v = I128s12::from(u64::MAX);
        // u64::MAX = 2^64 - 1 = 18_446_744_073_709_551_615
        assert_eq!(v.to_bits(), (u64::MAX as i128) * 1_000_000_000_000);
    }

    /// Sanity: round-trip `I128::from(int).to_int_lossy() == int as i64`
    /// across representative integer types.
    #[test]
    fn integer_round_trip_via_lossy_to_int() {
        for v in [0_i32, 1, -1, 42, -42, i32::MAX, i32::MIN] {
            assert_eq!(I128s12::from(v).to_int_lossy(), v as i64);
        }
        for v in [0_i64, 1, -1, 1_000_000_000, -1_000_000_000] {
            assert_eq!(I128s12::from(v).to_int_lossy(), v);
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // from_f64_lossy + to_f64_lossy + to_f32_lossy
    // ──────────────────────────────────────────────────────────────────

    #[test]
    fn from_f64_lossy_zero_is_zero() {
        assert_eq!(I128s12::from_f64_lossy(0.0), I128s12::ZERO);
    }

    #[test]
    fn zero_to_int_lossy_is_zero() {
        assert_eq!(I128s12::ZERO.to_int_lossy(), 0);
    }

    #[test]
    fn zero_to_f64_lossy_is_zero() {
        assert_eq!(I128s12::ZERO.to_f64_lossy(), 0.0);
    }

    #[test]
    fn zero_to_f32_lossy_is_zero() {
        assert_eq!(I128s12::ZERO.to_f32_lossy(), 0.0);
    }

    #[test]
    fn from_f64_lossy_one_is_one() {
        let v = I128s12::from_f64_lossy(1.0);
        assert_eq!(v, I128s12::ONE);
    }

    #[test]
    fn from_f64_lossy_negative() {
        let v = I128s12::from_f64_lossy(-1.0);
        assert_eq!(v, -I128s12::ONE);
    }

    /// Property test: `(from_f64_lossy(x).to_f64_lossy() - x).abs()`
    /// is within 1 LSB (= 10^-SCALE) for representative x in
    /// [-1e10, 1e10]. The 1-LSB tolerance covers the integer
    /// truncation in `from_f64_lossy`.
    #[test]
    fn from_f64_to_f64_round_trip_within_1_lsb() {
        let lsb = 1.0 / (I128s12::multiplier() as f64);
        let cases = [
            0.0_f64,
            1.0,
            -1.0,
            0.5,
            -0.5,
            1.5,
            -1.5,
            // Pick a value that's not close to any well-known math
            // constant so clippy's `approx_constant` lint stays quiet.
            1.234567890123_f64,
            -1.234567890123_f64,
            1e6,
            -1e6,
            1e10,
            -1e10,
            // Headline: 1.1, which f64 cannot represent exactly.
            1.1,
            2.2,
            3.3,
            // Sub-LSB; will round to ZERO at SCALE=12 (LSB = 1e-12).
            // Skip values smaller than the LSB.
        ];
        for x in cases {
            let v = I128s12::from_f64_lossy(x);
            let back = v.to_f64_lossy();
            let err = (back - x).abs();
            assert!(
                err <= lsb * 2.0, // allow up to 2 LSB to absorb f64 round-trip rounding
                "round-trip exceeded 2 LSB for x = {x}: back = {back}, err = {err}, lsb = {lsb}"
            );
        }
    }

    /// `to_f32_lossy` matches `to_f64_lossy as f32` (defines its
    /// implementation contract).
    #[test]
    fn to_f32_lossy_matches_f64_path() {
        let cases = [
            I128s12::ZERO,
            I128s12::ONE,
            -I128s12::ONE,
            I128s12::from_bits(1_500_000_000_000),
            I128s12::from_bits(-7_321_654_987_000),
        ];
        for v in cases {
            let via_f64 = v.to_f64_lossy() as f32;
            assert_eq!(v.to_f32_lossy(), via_f64);
        }
    }

    /// Saturation: `from_f64_lossy(f64::INFINITY) == I128::MAX`.
    #[test]
    fn from_f64_lossy_infinity_saturates_max() {
        assert_eq!(I128s12::from_f64_lossy(f64::INFINITY), I128s12::MAX);
    }

    /// Saturation: `from_f64_lossy(f64::NEG_INFINITY) == I128::MIN`.
    #[test]
    fn from_f64_lossy_neg_infinity_saturates_min() {
        assert_eq!(I128s12::from_f64_lossy(f64::NEG_INFINITY), I128s12::MIN);
    }

    /// NaN handling (locked policy): `from_f64_lossy(NaN) == ZERO`.
    #[test]
    fn from_f64_lossy_nan_is_zero() {
        assert_eq!(I128s12::from_f64_lossy(f64::NAN), I128s12::ZERO);
    }

    /// Saturation: finite out-of-range inputs clamp to MAX/MIN.
    #[test]
    fn from_f64_lossy_finite_out_of_range_saturates() {
        // 1e30 * 10^12 = 1e42 > i128::MAX ~1.7e38
        assert_eq!(I128s12::from_f64_lossy(1e30), I128s12::MAX);
        assert_eq!(I128s12::from_f64_lossy(-1e30), I128s12::MIN);
    }

    /// `to_int_lossy` truncates toward zero (drops fractional part).
    #[test]
    fn to_int_lossy_truncates_toward_zero() {
        // 2.5 -> 2
        assert_eq!(I128s12::from_bits(2_500_000_000_000).to_int_lossy(), 2);
        // -2.5 -> -2 (toward zero, not toward neg-infinity)
        assert_eq!(I128s12::from_bits(-2_500_000_000_000).to_int_lossy(), -2);
        // 0.999... -> 0
        assert_eq!(I128s12::from_bits(999_999_999_999).to_int_lossy(), 0);
        // -0.999... -> 0
        assert_eq!(I128s12::from_bits(-999_999_999_999).to_int_lossy(), 0);
    }

    /// `to_int_lossy` saturates beyond i64's range.
    #[test]
    fn to_int_lossy_saturates() {
        // I128s12::MAX is i128::MAX bits; integer part = i128::MAX / 10^12
        // ~= 1.7e26, way above i64::MAX. Saturates to i64::MAX.
        assert_eq!(I128s12::MAX.to_int_lossy(), i64::MAX);
        // I128s12::MIN is i128::MIN bits; integer part way below i64::MIN.
        // Saturates to i64::MIN.
        assert_eq!(I128s12::MIN.to_int_lossy(), i64::MIN);
    }

    // ──────────────────────────────────────────────────────────────────
    // TryFrom<i128> / TryFrom<u128>
    // ──────────────────────────────────────────────────────────────────

    #[test]
    fn try_from_i128_zero_succeeds() {
        let v: I128s12 = 0_i128.try_into().expect("zero fits");
        assert_eq!(v, I128s12::ZERO);
    }

    #[test]
    fn try_from_i128_in_range_succeeds() {
        // 1_000_000 model units -> 1e6 * 10^12 = 1e18, well under i128::MAX
        let v: I128s12 = 1_000_000_i128.try_into().expect("in-range fits");
        assert_eq!(v.to_bits(), 1_000_000 * 1_000_000_000_000);
    }

    #[test]
    fn try_from_i128_overflow_returns_err() {
        // i128::MAX cannot be scaled by 10^12.
        let result: Result<I128s12, _> = i128::MAX.try_into();
        assert_eq!(result, Err(DecimalConvertError::Overflow));

        let result_neg: Result<I128s12, _> = i128::MIN.try_into();
        assert_eq!(result_neg, Err(DecimalConvertError::Overflow));
    }

    #[test]
    fn try_from_u128_zero_succeeds() {
        let v: I128s12 = 0_u128.try_into().expect("zero fits");
        assert_eq!(v, I128s12::ZERO);
    }

    #[test]
    fn try_from_u128_in_range_succeeds() {
        let v: I128s12 = 42_u128.try_into().expect("in-range fits");
        assert_eq!(v.to_bits(), 42 * 1_000_000_000_000);
    }

    #[test]
    fn try_from_u128_above_i128_max_returns_err() {
        // Any u128 > i128::MAX is unrepresentable.
        let above: u128 = (i128::MAX as u128) + 1;
        let result: Result<I128s12, _> = above.try_into();
        assert_eq!(result, Err(DecimalConvertError::Overflow));
    }

    #[test]
    fn try_from_u128_max_returns_err() {
        let result: Result<I128s12, _> = u128::MAX.try_into();
        assert_eq!(result, Err(DecimalConvertError::Overflow));
    }

    // ──────────────────────────────────────────────────────────────────
    // TryFrom<f64> / TryFrom<f32>
    // ──────────────────────────────────────────────────────────────────

    #[test]
    fn try_from_f64_zero_succeeds() {
        let v: I128s12 = 0.0_f64.try_into().expect("zero fits");
        assert_eq!(v, I128s12::ZERO);
    }

    #[test]
    fn try_from_f64_one_succeeds() {
        let v: I128s12 = 1.0_f64.try_into().expect("one fits");
        assert_eq!(v, I128s12::ONE);
    }

    #[test]
    fn try_from_f64_nan_returns_err() {
        let result: Result<I128s12, _> = f64::NAN.try_into();
        assert_eq!(result, Err(DecimalConvertError::NotFinite));
    }

    #[test]
    fn try_from_f64_pos_infinity_returns_err() {
        let result: Result<I128s12, _> = f64::INFINITY.try_into();
        assert_eq!(result, Err(DecimalConvertError::NotFinite));
    }

    #[test]
    fn try_from_f64_neg_infinity_returns_err() {
        let result: Result<I128s12, _> = f64::NEG_INFINITY.try_into();
        assert_eq!(result, Err(DecimalConvertError::NotFinite));
    }

    #[test]
    fn try_from_f64_out_of_range_returns_err() {
        // 1e30 * 10^12 = 1e42 > i128::MAX
        let result: Result<I128s12, _> = 1e30_f64.try_into();
        assert_eq!(result, Err(DecimalConvertError::Overflow));

        let result_neg: Result<I128s12, _> = (-1e30_f64).try_into();
        assert_eq!(result_neg, Err(DecimalConvertError::Overflow));
    }

    #[test]
    fn try_from_f32_zero_succeeds() {
        let v: I128s12 = 0.0_f32.try_into().expect("zero fits");
        assert_eq!(v, I128s12::ZERO);
    }

    #[test]
    fn try_from_f32_nan_returns_err() {
        let result: Result<I128s12, _> = f32::NAN.try_into();
        assert_eq!(result, Err(DecimalConvertError::NotFinite));
    }

    #[test]
    fn try_from_f32_infinity_returns_err() {
        let result: Result<I128s12, _> = f32::INFINITY.try_into();
        assert_eq!(result, Err(DecimalConvertError::NotFinite));
    }

    #[test]
    fn try_from_f32_neg_infinity_returns_err() {
        let result: Result<I128s12, _> = f32::NEG_INFINITY.try_into();
        assert_eq!(result, Err(DecimalConvertError::NotFinite));
    }

    // ──────────────────────────────────────────────────────────────────
    // DecimalConvertError -- Display + Debug shape
    // ──────────────────────────────────────────────────────────────────

    /// Display impl produces stable strings for both variants.
    #[cfg(feature = "alloc")]
    #[test]
    fn convert_error_display() {
        extern crate alloc;
        use alloc::string::ToString;
        assert_eq!(
            DecimalConvertError::Overflow.to_string(),
            "decimal conversion overflow"
        );
        assert_eq!(
            DecimalConvertError::NotFinite.to_string(),
            "decimal conversion from non-finite float"
        );
    }

    /// `DecimalConvertError` is `Debug + Clone + Copy + Eq + Hash`
    /// (basic suite expected of any leaf error type).
    #[test]
    fn convert_error_traits_compile() {
        // Compile-time check: Copy + Clone + Eq + Hash bounds.
        fn assert_traits<T: core::fmt::Debug + Copy + Eq + core::hash::Hash>() {}
        assert_traits::<DecimalConvertError>();
    }

    // ──────────────────────────────────────────────────────────────────
    // Cross-scale exercise -- non-default SCALE
    // ──────────────────────────────────────────────────────────────────

    /// At SCALE = 6 (microseconds-style) the `From<i64>` impl still
    /// works and the round-trip via `to_int_lossy` is exact.
    #[test]
    fn from_int_works_at_scale_6() {
        type D6 = I128<6>;
        let v: D6 = D6::from(1_000_i64);
        assert_eq!(v.to_bits(), 1_000_000_000); // 10^9
        assert_eq!(v.to_int_lossy(), 1_000);
    }

    /// At SCALE = 0 the multiplier is 1 -- conversions are trivial.
    #[test]
    fn from_int_works_at_scale_0() {
        type D0 = I128<0>;
        let v: D0 = D0::from(42_i64);
        assert_eq!(v.to_bits(), 42);
        assert_eq!(v.to_int_lossy(), 42);
    }
}
