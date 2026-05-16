//! The [`Decimal`] trait — the width-generic surface shared by every
//! decimal type in this crate.
//!
//! Implemented by every width: `D9`, `D18`, `D38`, `D76`, `D153`, `D307`
//! (each via the shared [`crate::macros::basics::decl_decimal_basics!`]
//! macro). Lets downstream code write helpers that work across widths,
//! e.g.
//!
//! ```ignore
//! use decimal_scaled::Decimal;
//!
//! fn average<D: Decimal>(values: &[D]) -> D {
//!     D::sum(values.iter().copied()) / D::from_i32(values.len() as i32)
//! }
//! ```
//!
//! # Scope
//!
//! The trait carries the surface that has an identical *signature* on
//! every width:
//!
//! - **Type information**: `Storage`, `SCALE`, `MAX_SCALE`.
//! - **Constants**: `ZERO`, `ONE`, `MAX`, `MIN`, plus the
//!   `multiplier()` factor.
//! - **Round-trip**: `from_bits` / `to_bits` / `scale`.
//! - **Arithmetic operators**: as `Add` / `Sub` / `Mul` / `Div` /
//!   `Rem` / `Neg` supertrait bounds (also `*Assign`), reachable
//!   through plain `+` / `-` / etc.
//! - **Bitwise operators**: `BitAnd` / `BitOr` / `BitXor` / `Not` /
//!   `Shl<u32>` / `Shr<u32>` (as supertrait bounds).
//! - **Sign**: `abs`, `signum`, `is_positive`, `is_negative`.
//! - **Integer methods**: `div_euclid`, `rem_euclid`, `div_floor`,
//!   `div_ceil`, `abs_diff`, `midpoint`, `mul_add`, plus the
//!   float-shape predicates `is_nan`, `is_infinite`, `is_finite`.
//! - **Integer-exponent powers**: `pow`, `powi`, plus the four
//!   overflow-variant siblings (`checked_pow`, `wrapping_pow`,
//!   `saturating_pow`, `overflowing_pow`).
//! - **Overflow-variant arithmetic**: `checked_*`, `wrapping_*`,
//!   `saturating_*`, `overflowing_*` of `add` / `sub` / `mul` / `div`
//!   / `rem` / `neg`.
//! - **Integer conversion**: `from_i32`, `to_int`, `to_int_with`.
//! - **Float bridge** (when `feature = "std"`): `from_f64`,
//!   `from_f64_with`, `to_f64`, `to_f32`.
//! - **Default reductions**: `is_zero`, `is_one`, `is_normal`, `sum`,
//!   `product`.
//!
//! # Out of scope
//!
//! Some methods are deliberately not on the trait, because their
//! signature varies per width or the trait can't represent it:
//!
//! - **Rescale** (`rescale<TARGET>` / `rescale_with`) takes a
//!   `const`-generic target `SCALE` parameter. Const-generic trait
//!   methods aren't stable.
//! - **`from_int`** takes a different source integer per width
//!   (`i32` for `D9`, `i64` for `D18` / `D38`, `i128` for the wide
//!   tiers). Use [`Self::from_i32`] when you need a width-generic
//!   integer constructor, and accept the narrower input range.
//! - **Transcendentals** (`ln` / `exp` / `sin` / …) are gated by the
//!   `strict` / `fast` features and live as inherent methods on the
//!   concrete types. Reach for those directly when needed.
//! - **Mathematical constants** (`pi`, `tau`, `e`, …) live on
//!   [`crate::DecimalConsts`], a separate trait that every width also
//!   implements.
//!
//! For most users the concrete type (e.g. `D38<12>` or its alias
//! `D38s12`) is the canonical surface. Reach for [`Decimal`] only when
//! writing code that must work across widths.

use crate::rounding::RoundingMode;

/// Scaled fixed-point decimal type with a compile-time `SCALE` and a
/// fixed-width integer `Storage`.
///
/// See the module-level documentation for the full surface and what's
/// intentionally not on the trait.
///
/// # Precision
///
/// N/A: this is a trait definition; no arithmetic is performed.
pub trait Decimal:
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

    /// The decimal scale of this type, equal to the const-generic
    /// parameter. One LSB of storage represents `10^-SCALE`.
    const SCALE: u32;

    /// The maximum legal `SCALE` for this width. Equal to the largest
    /// `k` such that `10^k` fits in `Self::Storage`. For example, 38
    /// for `D38`, 76 for `D76`.
    const MAX_SCALE: u32;

    /// The additive identity (logical value `0`).
    const ZERO: Self;

    /// The multiplicative identity (logical value `1`).
    const ONE: Self;

    /// The largest representable value (storage equal to
    /// `Self::Storage::MAX`).
    const MAX: Self;

    /// The smallest representable value (storage equal to
    /// `Self::Storage::MIN`).
    const MIN: Self;

    /// Returns `10^SCALE`, the factor that converts a logical integer
    /// to its storage representation.
    fn multiplier() -> Self::Storage;

    /// Constructs from a raw storage value.
    fn from_bits(raw: Self::Storage) -> Self;

    /// Returns the raw storage value.
    fn to_bits(self) -> Self::Storage;

    /// Returns the decimal scale of this value (equal to
    /// [`Self::SCALE`]; provided for ergonomic method-call syntax).
    fn scale(self) -> u32;

    // ── Sign ──────────────────────────────────────────────────────────

    /// Absolute value. `Self::MIN.abs()` panics in debug, wraps to
    /// `Self::MIN` in release (mirroring the storage type).
    fn abs(self) -> Self;

    /// `+ONE`, `ZERO`, or `-ONE` according to the sign of `self`.
    fn signum(self) -> Self;

    /// `true` when `self > ZERO`.
    fn is_positive(self) -> bool;

    /// `true` when `self < ZERO`.
    fn is_negative(self) -> bool;

    // ── Integer-shape predicates ─────────────────────────────────────
    //
    // A fixed-point decimal is always a finite, non-NaN integer in its
    // storage representation. These predicates exist so generic
    // numeric code that branches on the float predicates also works
    // here; they return constants.

    /// Always `false` — fixed-point decimals cannot represent NaN.
    fn is_nan(self) -> bool;

    /// Always `false` — fixed-point decimals cannot represent infinity.
    fn is_infinite(self) -> bool;

    /// Always `true` — every fixed-point decimal value is finite.
    fn is_finite(self) -> bool;

    // ── Integer methods ──────────────────────────────────────────────

    /// Euclidean division: integer quotient (rounded so the remainder
    /// is non-negative) scaled back into `Self`. Panics on `rhs == ZERO`.
    fn div_euclid(self, rhs: Self) -> Self;

    /// Euclidean remainder, non-negative when `rhs != ZERO`. Panics on
    /// `rhs == ZERO`.
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Floor-rounded division (toward `-∞`). Panics on `rhs == ZERO`.
    fn div_floor(self, rhs: Self) -> Self;

    /// Ceil-rounded division (toward `+∞`). Panics on `rhs == ZERO`.
    fn div_ceil(self, rhs: Self) -> Self;

    /// `|self - rhs|`, computed without intermediate overflow.
    fn abs_diff(self, rhs: Self) -> Self;

    /// Midpoint of `self` and `rhs` (rounding toward `-∞`), computed
    /// without intermediate overflow.
    fn midpoint(self, rhs: Self) -> Self;

    /// `self * a + b` — mirrors the `f64::mul_add` call shape so
    /// f64-generic numeric code can monomorphise to a decimal type.
    fn mul_add(self, a: Self, b: Self) -> Self;

    // ── Integer-exponent powers ──────────────────────────────────────

    /// `self^exp` via square-and-multiply. Overflow follows `Mul`
    /// (debug panic, release wrap).
    fn pow(self, exp: u32) -> Self;

    /// Signed integer exponent. Negative `exp` produces
    /// `ONE / self.pow(|exp|)`; the divide truncates at the type's
    /// scale.
    fn powi(self, exp: i32) -> Self;

    /// `Some(self^exp)`, or `None` if any intermediate step overflows.
    fn checked_pow(self, exp: u32) -> Option<Self>;

    /// Wrapping `pow`. Each multiplication step wraps in the storage
    /// type.
    fn wrapping_pow(self, exp: u32) -> Self;

    /// Saturating `pow` — clamps to `MAX` / `MIN` on overflow, with
    /// the sign matching the mathematical result.
    fn saturating_pow(self, exp: u32) -> Self;

    /// `(self^exp, overflowed)` — the wrapping result paired with a
    /// boolean.
    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    // ── Overflow-variant arithmetic ─────────────────────────────────

    /// `Some(self + rhs)`, or `None` if the sum overflows.
    fn checked_add(self, rhs: Self) -> Option<Self>;
    /// `Some(self - rhs)`, or `None` if the difference overflows.
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    /// `Some(self * rhs)`, or `None` if the scaled product overflows.
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    /// `Some(self / rhs)`, or `None` if `rhs == ZERO` or the quotient
    /// overflows the storage.
    fn checked_div(self, rhs: Self) -> Option<Self>;
    /// `Some(-self)`, or `None` when `self == MIN`.
    fn checked_neg(self) -> Option<Self>;
    /// `Some(self % rhs)`, or `None` on divide-by-zero / `MIN % -ONE`.
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    /// Two's-complement wrapping `+`.
    fn wrapping_add(self, rhs: Self) -> Self;
    /// Two's-complement wrapping `-`.
    fn wrapping_sub(self, rhs: Self) -> Self;
    /// Wrapping `*` — intermediate widens for overflow detection, the
    /// final narrowing wraps.
    fn wrapping_mul(self, rhs: Self) -> Self;
    /// Wrapping `/` — **panics on `rhs == ZERO`**, matching
    /// `i128::wrapping_div`.
    fn wrapping_div(self, rhs: Self) -> Self;
    /// Wrapping `-self`; `MIN.wrapping_neg() == MIN`.
    fn wrapping_neg(self) -> Self;
    /// Wrapping `%` — **panics on `rhs == ZERO`**.
    fn wrapping_rem(self, rhs: Self) -> Self;

    /// Saturating `+`.
    fn saturating_add(self, rhs: Self) -> Self;
    /// Saturating `-`.
    fn saturating_sub(self, rhs: Self) -> Self;
    /// Saturating `*` — sign of the saturated bound matches the
    /// mathematical product.
    fn saturating_mul(self, rhs: Self) -> Self;
    /// Saturating `/` — divide-by-zero saturates to `MAX` / `MIN`
    /// according to the sign of `self`.
    fn saturating_div(self, rhs: Self) -> Self;
    /// Saturating `-self` — `MIN.saturating_neg() == MAX`.
    fn saturating_neg(self) -> Self;

    /// Overflowing `+`.
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    /// Overflowing `-`.
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    /// Overflowing `*`.
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    /// Overflowing `/` — `overflowed` is `true` on out-of-range or
    /// divide-by-zero.
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    /// Overflowing `-self` — `overflowed` is `true` iff `self == MIN`.
    fn overflowing_neg(self) -> (Self, bool);
    /// Overflowing `%`.
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    // ── Integer conversion ───────────────────────────────────────────

    /// Construct from an `i32`, scaling by `10^SCALE`. Width-generic
    /// integer constructor; for wider source integers use the
    /// concrete type's `from_int` (whose `IntSrc` parameter varies
    /// per width).
    fn from_i32(value: i32) -> Self;

    /// Convert to `i64` using the crate-default rounding mode, with
    /// saturating overflow on out-of-range integer parts.
    fn to_int(self) -> i64;

    /// Convert to `i64` using the supplied rounding mode for the
    /// fractional discard step. Saturating overflow on out-of-range
    /// integer parts.
    fn to_int_with(self, mode: RoundingMode) -> i64;

    // ── Float bridge (lossy) ─────────────────────────────────────────

    /// Construct from `f64` using the crate-default rounding mode.
    /// `NaN` saturates to `ZERO`; `±∞` saturates to `MAX` / `MIN`.
    #[cfg(feature = "std")]
    fn from_f64(value: f64) -> Self;

    /// Construct from `f64` using the supplied rounding mode.
    #[cfg(feature = "std")]
    fn from_f64_with(value: f64, mode: RoundingMode) -> Self;

    /// Convert to `f64`. Lossy when the storage magnitude exceeds
    /// `f64`'s ~15-digit exact range.
    #[cfg(feature = "std")]
    fn to_f64(self) -> f64;

    /// Convert to `f32`. Lossy.
    #[cfg(feature = "std")]
    fn to_f32(self) -> f32;

    // ── Default reductions ───────────────────────────────────────────

    /// `true` if this value is the additive identity.
    #[inline]
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// `true` if this value is the multiplicative identity.
    #[inline]
    fn is_one(self) -> bool {
        self == Self::ONE
    }

    /// `true` for every non-zero value (a fixed-point decimal has no
    /// subnormals).
    #[inline]
    fn is_normal(self) -> bool {
        !self.is_zero()
    }

    /// Sums an iterator of decimals of this width, starting from
    /// `ZERO`. Width-generic convenience for `iter.fold(ZERO, +)`.
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().fold(Self::ZERO, |acc, x| acc + x)
    }

    /// Multiplies an iterator of decimals of this width, starting from
    /// `ONE`. Width-generic convenience for `iter.fold(ONE, *)`.
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().fold(Self::ONE, |acc, x| acc * x)
    }
}

// The `Decimal` trait impl for every width is emitted by the
// `decl_decimal_basics!` macro in `src/macros/basics.rs`.
