//! The [`DecimalArithmetic`] trait — type info, operators, sign,
//! pow / overflow-variant arithmetic, and reductions shared by every
//! decimal width.
//!
//! Split out of the original [`crate::Decimal`] trait alongside
//! [`crate::DecimalConvert`], [`crate::DecimalTranscendental`], and
//! [`crate::DecimalConstants`]; `Decimal` is now a marker supertrait
//! that requires all four. Callers who only need arithmetic (not
//! conversions or transcendentals) can target this narrower bound:
//!
//! ```ignore
//! use decimal_scaled::DecimalArithmetic;
//!
//! fn dot<T: DecimalArithmetic + Copy>(a: &[T], b: &[T]) -> T {
//!     a.iter().zip(b).map(|(x, y)| (*x) * (*y))
//!         .fold(T::ZERO, |acc, p| acc + p)
//! }
//! ```
//!
//! See `crate::decimal_trait` for the full scope rationale.

/// Arithmetic surface shared by every decimal width: type info,
/// constants, operators, sign methods, integer-shape predicates,
/// integer-style division helpers, integer-exponent powers, the
/// checked / wrapping / saturating / overflowing variants of every
/// operator, and reductions (`sum` / `product`).
///
/// See module-level docs for usage; see [`crate::DecimalConvert`]
/// for the conversion surface and [`crate::Decimal`] for the marker
/// supertrait that combines all four halves.
pub trait DecimalArithmetic:
    Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Default
    + core::fmt::Debug
    + core::fmt::Display
    + core::hash::Hash
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Rem<Output = Self>
    + core::ops::Neg<Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::ops::RemAssign
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitXor<Output = Self>
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shr<u32, Output = Self>
{
    /// Underlying integer storage type (e.g. `i128` for `D38<SCALE>`).
    type Storage: Copy + PartialEq + Eq;

    /// The decimal scale of this type.
    const SCALE: u32;

    /// The maximum legal `SCALE` for this width.
    const MAX_SCALE: u32;

    /// The additive identity.
    const ZERO: Self;

    /// The multiplicative identity.
    const ONE: Self;

    /// The largest representable value.
    const MAX: Self;

    /// The smallest representable value.
    const MIN: Self;

    /// Returns `10^SCALE`.
    fn multiplier() -> Self::Storage;

    // ── Sign ──────────────────────────────────────────────────────────

    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;

    // ── Integer-shape predicates (always-const for decimals) ────────

    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;

    // ── Integer methods ──────────────────────────────────────────────

    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn div_floor(self, rhs: Self) -> Self;
    fn div_ceil(self, rhs: Self) -> Self;
    fn abs_diff(self, rhs: Self) -> Self;
    fn midpoint(self, rhs: Self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;

    // ── Integer-exponent powers ──────────────────────────────────────

    fn pow(self, exp: u32) -> Self;
    fn powi(self, exp: i32) -> Self;
    fn checked_pow(self, exp: u32) -> Option<Self>;
    fn wrapping_pow(self, exp: u32) -> Self;
    fn saturating_pow(self, exp: u32) -> Self;
    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    // ── Overflow-variant arithmetic ─────────────────────────────────

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_neg(self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_neg(self) -> Self;
    fn wrapping_rem(self, rhs: Self) -> Self;

    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;
    fn saturating_div(self, rhs: Self) -> Self;
    fn saturating_neg(self) -> Self;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    // ── Default reductions ───────────────────────────────────────────

    #[inline]
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    #[inline]
    fn is_one(self) -> bool {
        self == Self::ONE
    }

    #[inline]
    fn is_normal(self) -> bool {
        !self.is_zero()
    }

    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().fold(Self::ZERO, |acc, x| acc + x)
    }

    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().fold(Self::ONE, |acc, x| acc * x)
    }
}
