//! Core type definition: [`I128`] and the concrete scale aliases
//! [`I128s0`] through [`I128s38`].
//!
//! `I128<const SCALE: u32>` is a `#[repr(transparent)]` newtype around
//! `i128`. The stored integer equals `actual_value * 10^SCALE`.
//!
//! The `#[repr(transparent)]` annotation is load-bearing: it guarantees
//! the same ABI as a bare `i128`, so `from_bits` / `to_bits` round-trips
//! are exact and the type is safe to embed in C-ABI plugin payloads.

/// Scaled fixed-point decimal with 128-bit storage.
///
/// `SCALE` is the base-10 exponent. A logical value `v` is stored as
/// `v * 10^SCALE` in the underlying `i128`. For example, with `SCALE = 12`
/// the number `1.5` is stored as `i128(1_500_000_000_000)`.
///
/// # Precision
///
/// N/A: type definition, no arithmetic performed.
///
/// # Determinism
///
/// All arithmetic is integer arithmetic on `i128`. The same inputs produce
/// the same bit-pattern on every platform.
///
/// # Equality and ordering
///
/// `Hash`, `Eq`, and `Ord` are derived from `i128`. Two `I128<S>` values
/// are equal if and only if their underlying `i128` fields are bit-equal.
/// This works because the scale is fixed at compile time -- each logical
/// value has exactly one representation.
///
/// # Const-generic scale
///
/// The const generic allows scale variants (`I128<9>`, `I128<6>`, etc.)
/// as trivial type aliases without duplicating any method implementations.
/// Mixed-scale arithmetic is deliberately not provided; callers convert
/// explicitly.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I128<const SCALE: u32>(pub i128);

// Manual `Debug` is implemented in `display.rs` and renders via `Display`
// so the canonical decimal string is shown rather than the raw i128.

/// `Default` returns `ZERO`, matching `i128::default() == 0`.
///
/// This lets `#[derive(Default)]` work correctly on structs that contain
/// `I128<S>` fields.
impl<const SCALE: u32> Default for I128<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

// Scale aliases: I128s0 through I128s38.
//
// Each alias names a specific SCALE value. The const-generic impl block
// makes every method generic, so adding aliases is purely additive.
//
// Representable integer range is approximately `i128::MAX / 10^SCALE`.
// `i128::MAX` is approximately 1.7e38.
//
// SCALE = 0 is supported (mg_divide special-cases it to plain i128
// arithmetic). SCALE = 38 is the upper bound: 10^38 < i128::MAX, but
// 10^39 overflows. The math constants (pi, tau, e, golden) have a
// 35-digit reference in consts.rs (SCALE_REF = 35); at SCALE > 35
// they are zero-extended and gain no real precision.

/// Scale alias: `I128<0>`. 1 LSB = 1 (thin `i128` wrapper, no rescale).
/// Range ~+/-1.7e38.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s0 = I128<0>;

/// Scale alias: `I128<1>`. 1 LSB = 10^-1 (1 decimal digit).
/// Range ~+/-1.7e37.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s1 = I128<1>;

/// Scale alias: `I128<2>`. 1 LSB = 10^-2 (cents). Range ~+/-1.7e36.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s2 = I128<2>;

/// Scale alias: `I128<3>`. 1 LSB = 10^-3 (thousandths; 1 mm at m units).
/// Range ~+/-1.7e35.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s3 = I128<3>;

/// Scale alias: `I128<4>`. 1 LSB = 10^-4 (basis points). Range ~+/-1.7e34.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s4 = I128<4>;

/// Scale alias: `I128<5>`. 1 LSB = 10^-5. Range ~+/-1.7e33.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s5 = I128<5>;

/// Scale alias: `I128<6>`. 1 LSB = 10^-6 (1 um at mm units; ppm).
/// Range ~+/-1.7e32.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s6 = I128<6>;

/// Scale alias: `I128<7>`. 1 LSB = 10^-7. Range ~+/-1.7e31.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s7 = I128<7>;

/// Scale alias: `I128<8>`. 1 LSB = 10^-8 (satoshi-grade). Range ~+/-1.7e30.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s8 = I128<8>;

/// Scale alias: `I128<9>`. 1 LSB = 10^-9 (1 nm at mm units; ppb).
/// Range ~+/-1.7e29.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s9 = I128<9>;

/// Scale alias: `I128<10>`. 1 LSB = 10^-10. Range ~+/-1.7e28.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s10 = I128<10>;

/// Scale alias: `I128<11>`. 1 LSB = 10^-11. Range ~+/-1.7e27.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s11 = I128<11>;

/// Scale alias: `I128<12>`. 1 LSB = 10^-12 (1 pm at mm units).
/// Range ~+/-1.7e14 model units.
///
/// This is the primary concrete alias for general use. At `SCALE = 12`:
/// - 1 LSB is `10^-12` model units.
/// - The representable integer range is approximately +/-1.7e14 model units.
/// - Squared-component operations (e.g. dot products) overflow beyond
///   roughly 13,000 km at mm units.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s12 = I128<12>;

/// Scale alias: `I128<13>`. 1 LSB = 10^-13. Range ~+/-1.7e25.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s13 = I128<13>;

/// Scale alias: `I128<14>`. 1 LSB = 10^-14. Range ~+/-1.7e24.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s14 = I128<14>;

/// Scale alias: `I128<15>`. 1 LSB = 10^-15 (femto). Range ~+/-1.7e23.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s15 = I128<15>;

/// Scale alias: `I128<16>`. 1 LSB = 10^-16. Range ~+/-1.7e22.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s16 = I128<16>;

/// Scale alias: `I128<17>`. 1 LSB = 10^-17. Range ~+/-1.7e21.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s17 = I128<17>;

/// Scale alias: `I128<18>`. 1 LSB = 10^-18 (atto; high-precision scientific).
/// Range ~+/-1.7e20.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s18 = I128<18>;

/// Scale alias: `I128<19>`. 1 LSB = 10^-19. Range ~+/-1.7e19.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s19 = I128<19>;

/// Scale alias: `I128<20>`. 1 LSB = 10^-20. Range ~+/-1.7e18.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s20 = I128<20>;

/// Scale alias: `I128<21>`. 1 LSB = 10^-21 (zepto). Range ~+/-1.7e17.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s21 = I128<21>;

/// Scale alias: `I128<22>`. 1 LSB = 10^-22. Range ~+/-1.7e16.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s22 = I128<22>;

/// Scale alias: `I128<23>`. 1 LSB = 10^-23. Range ~+/-1.7e15.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s23 = I128<23>;

/// Scale alias: `I128<24>`. 1 LSB = 10^-24 (yocto). Range ~+/-1.7e14.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s24 = I128<24>;

/// Scale alias: `I128<25>`. 1 LSB = 10^-25. Range ~+/-1.7e13.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s25 = I128<25>;

/// Scale alias: `I128<26>`. 1 LSB = 10^-26. Range ~+/-1.7e12.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s26 = I128<26>;

/// Scale alias: `I128<27>`. 1 LSB = 10^-27. Range ~+/-1.7e11.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s27 = I128<27>;

/// Scale alias: `I128<28>`. 1 LSB = 10^-28. Range ~+/-1.7e10.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s28 = I128<28>;

/// Scale alias: `I128<29>`. 1 LSB = 10^-29. Range ~+/-1.7e9.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s29 = I128<29>;

/// Scale alias: `I128<30>`. 1 LSB = 10^-30. Range ~+/-1.7e8.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s30 = I128<30>;

/// Scale alias: `I128<31>`. 1 LSB = 10^-31. Range ~+/-1.7e7.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s31 = I128<31>;

/// Scale alias: `I128<32>`. 1 LSB = 10^-32. Range ~+/-1.7e6.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s32 = I128<32>;

/// Scale alias: `I128<33>`. 1 LSB = 10^-33. Range ~+/-1.7e5.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s33 = I128<33>;

/// Scale alias: `I128<34>`. 1 LSB = 10^-34. Range ~+/-1.7e4.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s34 = I128<34>;

/// Scale alias: `I128<35>`. 1 LSB = 10^-35. Range ~+/-1.7e3.
///
/// Matches `SCALE_REF` in `consts.rs`: the math constants `pi`, `tau`,
/// `e`, and `golden` are stored at this reference scale internally, so
/// at `SCALE = 35` they round-trip without precision loss.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s35 = I128<35>;

/// Scale alias: `I128<36>`. 1 LSB = 10^-36. Range ~+/-170.
///
/// The math constants (`pi`, `tau`, `e`, `golden`) are stored at a
/// 35-digit reference. Above `SCALE = 35` they are scaled up from that
/// reference, so trailing digits are zero-extended rather than
/// meaningfully precise.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s36 = I128<36>;

/// Scale alias: `I128<37>`. 1 LSB = 10^-37. Range ~+/-17.
///
/// Math constants lose precision above `SCALE = 35`; see `I128s36`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s37 = I128<37>;

/// Scale alias: `I128<38>`. 1 LSB = 10^-38. Range ~+/-1.7
/// (sub-unit dimensionless ratios).
///
/// This is the maximum supported scale. `10^38 < i128::MAX < 10^39`, so
/// `SCALE = 39` is not supported (`multiplier()` would overflow). Math
/// constants lose precision above `SCALE = 35`; see `I128s36`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type I128s38 = I128<38>;

/// Error returned by `<I128<SCALE> as FromStr>::from_str` when the input
/// string is not a valid canonical decimal literal.
///
/// Each variant identifies one specific failure mode so callers can surface
/// a precise diagnostic.
///
/// # Variants
///
/// - [`Empty`](Self::Empty): the input is empty after trimming.
/// - [`SignOnly`](Self::SignOnly): the input is a bare `-` or `+` with no digits.
/// - [`LeadingZero`](Self::LeadingZero): the integer part has a redundant
///   leading zero (e.g. `01.5`). A bare `0` or `0.x` is permitted; `00`, `01`,
///   and `01.5` are not.
/// - [`OverlongFractional`](Self::OverlongFractional): the fractional part has
///   more digits than `SCALE`, which would silently discard precision.
/// - [`ScientificNotation`](Self::ScientificNotation): the input contains a
///   scientific-notation `e` or `E` exponent. Callers that need this form must
///   strip the exponent themselves.
/// - [`InvalidChar`](Self::InvalidChar): the input contains a character that is
///   not a digit, sign, or decimal point.
/// - [`OutOfRange`](Self::OutOfRange): the parsed magnitude exceeds `I128::MAX`
///   or `I128::MIN` after scaling by `10^SCALE`.
/// - [`MissingDigits`](Self::MissingDigits): a decimal point has no digit on
///   one side (e.g. `.5` or `5.`). The required form is `0.5` or `5.0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseDecimalError {
    /// Input string is empty.
    Empty,
    /// Input is `-` or `+` with no following digits.
    SignOnly,
    /// Integer part has a redundant leading zero (e.g. `01.5`).
    LeadingZero,
    /// Fractional part has more digits than `SCALE`.
    OverlongFractional,
    /// Input uses scientific notation (`1e3`); not accepted.
    ScientificNotation,
    /// Input contains a character that is not a digit, sign, or dot.
    InvalidChar,
    /// Parsed value exceeds `I128::MAX` or `I128::MIN` after scaling.
    OutOfRange,
    /// Decimal point with no digit on one side (e.g. `.5` or `5.`).
    MissingDigits,
}

impl<const SCALE: u32> I128<SCALE> {
    /// Constructs a `I128<SCALE>` from a raw `i128` bit pattern.
    ///
    /// The integer is interpreted directly as the internal storage:
    /// `raw` represents the logical value `raw * 10^(-SCALE)`. This is the
    /// inverse of [`Self::to_bits`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    /// // Raw 1_500_000_000_000 represents the logical value 1.5.
    /// let v = I128s12::from_bits(1_500_000_000_000);
    /// assert_eq!(v.to_bits(), 1_500_000_000_000);
    /// ```
    #[inline]
    pub const fn from_bits(raw: i128) -> Self {
        Self(raw)
    }

    /// Returns the raw `i128` storage value.
    ///
    /// The returned integer encodes the logical value `self * 10^SCALE`.
    /// This is the inverse of [`Self::from_bits`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    /// assert_eq!(I128s12::ONE.to_bits(), 1_000_000_000_000_i128);
    /// ```
    #[inline]
    pub const fn to_bits(self) -> i128 {
        self.0
    }

    /// Returns `10^SCALE`, the factor that converts a logical integer value
    /// to its storage representation.
    ///
    /// This equals the bit pattern of [`Self::ONE`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Overflow
    ///
    /// `10^SCALE` overflows `i128` at `SCALE >= 39`. For practical scales
    /// (`SCALE <= 38`) this is within range. Calling with an overflowing
    /// scale panics at compile time when the const item is evaluated.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::I128s12;
    /// assert_eq!(I128s12::multiplier(), 1_000_000_000_000_i128);
    /// ```
    #[inline]
    pub const fn multiplier() -> i128 {
        // i128::pow is const-stable on current Rust toolchains, so calling
        // it inside a const fn works on stable. A const-item initialiser
        // of the form `10i128.pow(SCALE)` at the item level would require
        // the nightly generic_const_exprs feature; the const fn avoids that.
        10i128.pow(SCALE)
    }

    /// The additive identity. Stored as `i128(0)`.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    pub const ZERO: Self = Self(0);

    /// The multiplicative identity. Stored as `i128(10^SCALE)`.
    ///
    /// At `SCALE = 12` the raw value is `1_000_000_000_000`.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    pub const ONE: Self = Self(Self::multiplier());

    /// The largest representable value: `I128(i128::MAX)`.
    ///
    /// In logical terms this is `i128::MAX / 10^SCALE`. At `SCALE = 12`
    /// that is approximately `1.7e14` model units. Arithmetic that overflows
    /// this bound panics in debug builds and wraps in release builds.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    pub const MAX: Self = Self(i128::MAX);

    /// The smallest representable value: `I128(i128::MIN)`.
    ///
    /// Mirror of [`Self::MAX`]. Note that `-MIN` panics in debug builds
    /// because `i128::MIN` has no positive two's-complement counterpart.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    pub const MIN: Self = Self(i128::MIN);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `from_bits` / `to_bits` round-trip is exact.
    #[test]
    fn from_bits_to_bits_round_trip() {
        let raw: i128 = 1_500_000_000_000;
        let v: I128s12 = I128s12::from_bits(raw);
        assert_eq!(v.to_bits(), raw);
    }

    /// `ZERO` has raw bit value 0.
    #[test]
    fn zero_is_zero_bits() {
        assert_eq!(I128s12::ZERO.to_bits(), 0);
    }

    /// Two instances with identical raw bits compare equal.
    #[test]
    fn equal_by_underlying_bits() {
        assert_eq!(
            I128s12::from_bits(42_000_000_000_000),
            I128s12::from_bits(42_000_000_000_000)
        );
        assert_ne!(I128s12::from_bits(42), I128s12::from_bits(43));
    }

    /// Ord is derived from i128: smaller bits compare less.
    #[test]
    fn ord_by_underlying_bits() {
        assert!(I128s12::from_bits(1) < I128s12::from_bits(2));
        assert!(I128s12::from_bits(-1) < I128s12::from_bits(0));
    }

    /// `multiplier()` returns 10^SCALE. At SCALE = 12 that is 10^12.
    #[test]
    fn multiplier_is_ten_to_scale() {
        assert_eq!(I128s12::multiplier(), 1_000_000_000_000_i128);
    }

    /// `ONE` has bit pattern 10^SCALE so that the logical value is 1.
    #[test]
    fn one_has_scaled_bit_pattern() {
        assert_eq!(I128s12::ONE.to_bits(), 1_000_000_000_000_i128);
    }

    /// `MAX` is `i128::MAX`.
    #[test]
    fn max_is_i128_max() {
        assert_eq!(I128s12::MAX.to_bits(), i128::MAX);
    }

    /// `MIN` is `i128::MIN`.
    #[test]
    fn min_is_i128_min() {
        assert_eq!(I128s12::MIN.to_bits(), i128::MIN);
    }

    /// `ONE` is not equal to `ZERO`.
    #[test]
    fn one_is_not_zero() {
        assert_ne!(I128s12::ONE, I128s12::ZERO);
        assert!(I128s12::ONE.is_positive());
    }

    /// `multiplier()` works correctly at non-default scales.
    #[test]
    fn multiplier_at_other_scales() {
        type D6 = super::I128<6>;
        assert_eq!(D6::multiplier(), 1_000_000_i128);
        assert_eq!(D6::ONE.to_bits(), 1_000_000_i128);

        type D0 = super::I128<0>;
        assert_eq!(D0::multiplier(), 1_i128);
        assert_eq!(D0::ONE.to_bits(), 1_i128);
    }
}
