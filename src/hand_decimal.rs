//! `D256H` — a hand-rolled 256-bit fixed-point decimal type.
//!
//! `D256H<const SCALE: u32>` is the hand-rolled counterpart to the
//! `bnum`-backed [`D256`](crate::D256): same shape, same scale range
//! (`MAX_SCALE = 76`), but its storage is the in-tree [`HInt256`]
//! signed 256-bit integer rather than `bnum::I256`. The two are kept
//! side by side so the backends can be benchmarked head to head — see
//! `benches/wide_handrolled_vs_bnum.rs`.
//!
//! This is a focused "core decimal" surface: constructors / accessors,
//! the constants, the arithmetic operators and their `checked_*`
//! forms, `Display` / `Debug`, ordering, and `From` for the primitive
//! integers. It deliberately does not mirror `D256`'s full
//! conversion / transcendental surface — it exists to measure the
//! hand-rolled arithmetic core.
//!
//! Gated behind the `d256` / `wide` Cargo features, exactly like
//! `D256`.

#![cfg(any(feature = "d256", feature = "wide"))]

use crate::wide_int::HInt256;

/// Hand-rolled 256-bit scaled fixed-point decimal.
///
/// A logical value `v` is stored as `v * 10^SCALE` in the underlying
/// [`HInt256`]. `SCALE` ranges `0..=76`; `10^77` overflows the signed
/// 256-bit storage.
///
/// # Precision
///
/// N/A: type definition, no arithmetic performed.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D256H<const SCALE: u32>(pub(crate) HInt256);

impl<const SCALE: u32> D256H<SCALE> {
    /// The additive identity (logical value `0`).
    pub const ZERO: Self = Self(HInt256::ZERO);

    /// The largest representable value (storage `2^255 − 1`).
    pub const MAX: Self = Self(HInt256::MAX);

    /// The smallest representable value (storage `−2^255`).
    pub const MIN: Self = Self(HInt256::MIN);

    /// The decimal scale of this type, equal to the const-generic
    /// `SCALE` parameter. One LSB of storage represents `10^-SCALE`.
    pub const SCALE: u32 = SCALE;

    /// The largest legal `SCALE` for this width: `10^76 < 2^255 < 10^77`.
    pub const MAX_SCALE: u32 = 76;

    /// Returns `10^SCALE` as raw two-limb storage — the factor that
    /// converts a logical integer value to its storage representation,
    /// equal to the raw storage of [`Self::ONE`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    #[must_use]
    pub fn multiplier() -> [u128; 2] {
        HInt256::pow10(SCALE).0
    }

    /// The multiplicative identity (logical value `1`), stored as
    /// `10^SCALE`.
    #[inline]
    #[must_use]
    pub fn one() -> Self {
        Self(HInt256::pow10(SCALE))
    }

    /// Returns the decimal scale of this value, equal to [`Self::SCALE`].
    #[inline]
    #[must_use]
    pub const fn scale(self) -> u32 {
        SCALE
    }

    /// Constructs from the raw two-limb storage (`limbs[0]` least
    /// significant), interpreted as a two's-complement signed integer
    /// equal to `value * 10^SCALE`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    #[must_use]
    pub const fn from_raw(limbs: [u128; 2]) -> Self {
        Self(HInt256(limbs))
    }

    /// Returns the raw two-limb storage.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    #[must_use]
    pub const fn to_raw(self) -> [u128; 2] {
        (self.0).0
    }

    /// Constructs from a signed integer, scaling by `10^SCALE`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    #[must_use]
    pub fn from_int(value: i128) -> Self {
        // Sign-extend `value` into a 256-bit two's-complement integer.
        let v = if value < 0 {
            HInt256([value as u128, u128::MAX])
        } else {
            HInt256([value as u128, 0])
        };
        // Scale by 10^SCALE: `dec_mul` with scale 0 is a plain
        // (non-rescaling) 256-bit multiply.
        Self(v.dec_mul(HInt256::pow10(SCALE), 0))
    }

    /// `true` if this value is the additive identity.
    #[inline]
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// `true` if this value is strictly negative.
    #[inline]
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// `true` if this value is strictly positive.
    #[inline]
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    /// `self + rhs`, or `None` on overflow.
    #[inline]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    /// `self - rhs`, or `None` on overflow.
    #[inline]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    /// `-self`, or `None` for `MIN`.
    #[inline]
    #[must_use]
    pub fn checked_neg(self) -> Option<Self> {
        self.0.checked_neg().map(Self)
    }

    /// `self * rhs`, or `None` if the rescaled product does not fit the
    /// signed 256-bit storage.
    #[inline]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let r = self.0.dec_mul(rhs.0, SCALE);
        // dec_mul truncates the 512-bit product into 256 bits; detect
        // the overflow by re-deriving and comparing the sign.
        let expected_negative = self.0.is_negative() ^ rhs.0.is_negative();
        if !r.is_zero() && r.is_negative() != expected_negative {
            None
        } else {
            Some(Self(r))
        }
    }

    /// `self / rhs`, or `None` if `rhs` is zero or the result does not
    /// fit the signed 256-bit storage.
    #[inline]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.0.is_zero() {
            return None;
        }
        let r = self.0.dec_div(rhs.0, SCALE);
        let expected_negative = self.0.is_negative() ^ rhs.0.is_negative();
        if !r.is_zero() && r.is_negative() != expected_negative {
            None
        } else {
            Some(Self(r))
        }
    }
}

impl<const SCALE: u32> Default for D256H<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

// ── Arithmetic operators ─────────────────────────────────────────────
//
// Overflow policy mirrors `D256` / the primitive integers: debug builds
// panic, release builds wrap.

impl<const SCALE: u32> core::ops::Add for D256H<SCALE> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        #[cfg(debug_assertions)]
        {
            self.checked_add(rhs).expect("D256H::add: overflow")
        }
        #[cfg(not(debug_assertions))]
        {
            Self(self.0.wrapping_add(rhs.0))
        }
    }
}

impl<const SCALE: u32> core::ops::AddAssign for D256H<SCALE> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const SCALE: u32> core::ops::Sub for D256H<SCALE> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        #[cfg(debug_assertions)]
        {
            self.checked_sub(rhs).expect("D256H::sub: overflow")
        }
        #[cfg(not(debug_assertions))]
        {
            Self(self.0.wrapping_sub(rhs.0))
        }
    }
}

impl<const SCALE: u32> core::ops::SubAssign for D256H<SCALE> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const SCALE: u32> core::ops::Neg for D256H<SCALE> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.negate())
    }
}

impl<const SCALE: u32> core::ops::Mul for D256H<SCALE> {
    type Output = Self;
    /// Multiplies two values of the same scale: the full 256×256→512
    /// product divided back by `10^SCALE`.
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.dec_mul(rhs.0, SCALE))
    }
}

impl<const SCALE: u32> core::ops::MulAssign for D256H<SCALE> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const SCALE: u32> core::ops::Div for D256H<SCALE> {
    type Output = Self;
    /// Divides two values of the same scale: the numerator widened by
    /// `10^SCALE` and divided by the denominator.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    #[inline]
    fn div(self, rhs: Self) -> Self {
        if rhs.0.is_zero() {
            panic!("D256H::div: division by zero");
        }
        Self(self.0.dec_div(rhs.0, SCALE))
    }
}

impl<const SCALE: u32> core::ops::DivAssign for D256H<SCALE> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const SCALE: u32> core::ops::Rem for D256H<SCALE> {
    type Output = Self;
    /// Remainder of two values at the same scale. Both share the scale
    /// factor, so the storage-level remainder is the answer.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        if rhs.0.is_zero() {
            panic!("D256H::rem: division by zero");
        }
        Self(self.0.rem(rhs.0))
    }
}

impl<const SCALE: u32> core::ops::RemAssign for D256H<SCALE> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

// ── Formatting ───────────────────────────────────────────────────────

#[cfg(feature = "alloc")]
impl<const SCALE: u32> core::fmt::Display for D256H<SCALE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (int_part, frac_part) = self.0.magnitude_divmod_pow10(SCALE);
        if self.0.is_negative() {
            f.write_str("-")?;
        }
        let int_str = crate::wide_int::u256_decimal_string(int_part);
        if SCALE == 0 {
            return f.write_str(&int_str);
        }
        let frac_str = crate::wide_int::u256_decimal_string(frac_part);
        write!(f, "{int_str}.{frac_str:0>width$}", width = SCALE as usize)
    }
}

#[cfg(feature = "alloc")]
impl<const SCALE: u32> core::fmt::Debug for D256H<SCALE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "D256H<{SCALE}>({self})")
    }
}

// ── From the primitive integers ──────────────────────────────────────

macro_rules! d256h_from_int {
    ($($t:ty),*) => {$(
        impl<const SCALE: u32> core::convert::From<$t> for D256H<SCALE> {
            /// Constructs by scaling the integer to `value * 10^SCALE`.
            #[inline]
            fn from(value: $t) -> Self {
                Self::from_int(value as i128)
            }
        }
    )*};
}
d256h_from_int!(i8, i16, i32, i64, u8, u16, u32);

/// Curated scale aliases, mirroring the `D256s*` set.
pub type D256Hs0 = D256H<0>;
/// 1 LSB = 10^-2 (cents).
pub type D256Hs2 = D256H<2>;
/// 1 LSB = 10^-6 (ppm).
pub type D256Hs6 = D256H<6>;
/// 1 LSB = 10^-12 (pico; financial standard).
pub type D256Hs12 = D256H<12>;
/// 1 LSB = 10^-18 (atto).
pub type D256Hs18 = D256H<18>;
/// 1 LSB = 10^-35 (matches `SCALE_REF`).
pub type D256Hs35 = D256H<35>;
/// 1 LSB = 10^-76. Maximum supported scale.
pub type D256Hs76 = D256H<76>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_and_constants() {
        assert!(D256Hs2::ZERO.is_zero());
        assert_eq!(D256Hs2::one().to_raw(), [100, 0]);
        assert_eq!(D256Hs2::multiplier(), [100, 0]);
        assert_eq!(D256Hs2::SCALE, 2);
        assert_eq!(D256Hs2::MAX_SCALE, 76);
        assert_eq!(D256Hs2::ZERO.scale(), 2);
        // round-trip
        let v = D256Hs12::from_raw([123_456_789, 0]);
        assert_eq!(v.to_raw(), [123_456_789, 0]);
        // 76-digit ceiling multiplier is in range.
        let _ = D256Hs76::multiplier();
    }

    #[test]
    fn arithmetic() {
        type D = D256H<6>;
        let one = D::one();
        let two: D = 2i32.into();
        let three: D = 3i32.into();
        let six: D = 6i32.into();
        assert_eq!(one + two, three);
        assert_eq!(three - one, two);
        assert_eq!(-one + one, D::ZERO);
        assert_eq!(two * three, six);
        assert_eq!(six / two, three);
        assert_eq!(six % two, D::ZERO);
        // fractional: 1.5 * 1.5 == 2.25 at scale 6.
        let half = D::from_raw([D::multiplier()[0] / 2, 0]);
        let one_half = one + half;
        let product = one_half * one_half;
        assert_eq!(product.to_raw(), [2_250_000, 0]);
        // assign forms
        let mut v = one;
        v += two;
        assert_eq!(v, three);
        v *= two;
        assert_eq!(v, six);
        v /= two;
        assert_eq!(v, three);
    }

    #[test]
    fn overflow_and_div_by_zero() {
        type D = D256H<2>;
        assert_eq!(D::MAX.checked_add(D::one()), None);
        assert_eq!(D::MIN.checked_sub(D::one()), None);
        assert_eq!(D::MIN.checked_neg(), None);
        assert_eq!(D::one().checked_div(D::ZERO), None);
        assert_eq!(D::one().checked_add(D::one()), Some(D::from_int(2)));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn display() {
        type D = D256H<12>;
        assert_eq!(alloc::format!("{}", D::one()), "1.000000000000");
        assert_eq!(alloc::format!("{}", -D::one()), "-1.000000000000");
        assert_eq!(alloc::format!("{}", D::ZERO), "0.000000000000");
        let int_only: D256H<0> = D256H::<0>::from_int(-42);
        assert_eq!(alloc::format!("{}", int_only), "-42");
        assert_eq!(alloc::format!("{:?}", D::one()), "D256H<12>(1.000000000000)");
    }

    #[test]
    fn ordering() {
        type D = D256H<6>;
        assert!(D::from_int(-1) < D::ZERO);
        assert!(D::ZERO < D::from_int(1));
        assert!(D::MIN < D::MAX);
        assert!(D::from_int(5) > D::from_int(3));
    }
}
