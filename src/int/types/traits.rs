//! The `FixedInt` / `FixedIntConvert` traits — the integer-layer parity
//! surface.
//!
//! These mirror the decimal traits ([`crate::DecimalArithmetic`] /
//! [`crate::DecimalConvert`]) method-for-method where the operation
//! makes sense on a fixed-width integer, plus the optimisable named
//! functions the decimal kernels lean on (`sqr`, `cube`, `root_int`,
//! `isqrt`, `bit_length`, `leading_zeros`). One trait carries the
//! arithmetic and optimisable surface; the companion
//! [`FixedIntConvert`] carries the limb/primitive bridges. Both are
//! implemented for [`Int<N>`] and [`Uint<N>`] with static dispatch and
//! `#[inline]` bodies, so generic width-aware code pays no runtime cost.
//!
//! Methods that only make sense on one signedness (`abs` / `signum` /
//! `is_negative` for signed; `isqrt` / `root_int` for the natural
//! domain of an unsigned magnitude) stay on the inherent impls rather
//! than the shared trait, keeping the trait the genuine common surface.

use super::{Int, Uint};

/// Arithmetic + optimisable surface shared by every fixed-width integer
/// type, mirroring the decimal [`crate::DecimalArithmetic`] method
/// names. Static dispatch only.
pub trait FixedInt:
    Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + core::fmt::Debug
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitXor<Output = Self>
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shr<u32, Output = Self>
{
    /// Number of 64-bit limbs.
    const LIMBS: usize;
    /// Bit width (`LIMBS * 64`).
    const BITS: usize;
    /// The additive identity.
    const ZERO: Self;
    /// The multiplicative identity.
    const ONE: Self;

    // ── Predicates ────────────────────────────────────────────────────

    fn is_zero(self) -> bool;
    fn is_one(self) -> bool;

    // ── Wrapping arithmetic ──────────────────────────────────────────

    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;

    // ── Checked arithmetic ───────────────────────────────────────────

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    // ── Integer-exponent powers ──────────────────────────────────────

    fn pow(self, exp: u32) -> Self;
    fn wrapping_pow(self, exp: u32) -> Self;
    fn checked_pow(self, exp: u32) -> Option<Self>;

    // ── Optimisable named functions ──────────────────────────────────

    /// `self²` (wrapping), via the dedicated half-product kernel.
    fn sqr(self) -> Self;
    /// `self³` (wrapping), `sqr` then one multiply.
    fn cube(self) -> Self;
    /// Bit length: `0` for zero, else `floor(log2|self|) + 1`.
    fn bit_length(self) -> u32;
    /// Leading zero bits in the `BITS`-wide representation.
    fn leading_zeros(self) -> u32;

    // ── Reductions (defaults) ────────────────────────────────────────

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

/// Limb / primitive conversion surface, mirroring the decimal
/// [`crate::DecimalConvert`] round-trip + integer-conversion methods.
/// `from_limbs` / `as_limbs` are the `from_bits` / `to_bits`
/// counterparts; `resize_to` is the cross-width bridge.
pub trait FixedIntConvert: FixedInt {
    /// Raw little-endian limb array type (`[u64; LIMBS]`).
    type Limbs: Copy;

    /// Constructs from raw little-endian limbs.
    fn from_limbs(limbs: Self::Limbs) -> Self;

    /// Returns the raw little-endian limbs by value.
    fn to_limbs(self) -> Self::Limbs;
}

// ── Uint<N> ─────────────────────────────────────────────────────────

impl<const N: usize> FixedInt for Uint<N> {
    const LIMBS: usize = N;
    const BITS: usize = N * 64;
    const ZERO: Self = Uint::<N>::ZERO;
    const ONE: Self = Uint::<N>::ONE;

    #[inline]
    fn is_zero(self) -> bool {
        Uint::is_zero(&self)
    }
    #[inline]
    fn is_one(self) -> bool {
        Uint::is_one(&self)
    }

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Uint::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        Uint::wrapping_sub(self, rhs)
    }
    #[inline]
    fn wrapping_mul(self, rhs: Self) -> Self {
        Uint::wrapping_mul(self, rhs)
    }

    #[inline]
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Uint::checked_add(self, rhs)
    }
    #[inline]
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Uint::checked_sub(self, rhs)
    }
    #[inline]
    fn checked_mul(self, rhs: Self) -> Option<Self> {
        Uint::checked_mul(self, rhs)
    }

    #[inline]
    fn pow(self, exp: u32) -> Self {
        Uint::pow(self, exp)
    }
    #[inline]
    fn wrapping_pow(self, exp: u32) -> Self {
        Uint::wrapping_pow(self, exp)
    }
    #[inline]
    fn checked_pow(self, exp: u32) -> Option<Self> {
        Uint::checked_pow(self, exp)
    }

    #[inline]
    fn sqr(self) -> Self {
        self.wrapping_sqr()
    }
    #[inline]
    fn cube(self) -> Self {
        self.wrapping_cube()
    }
    #[inline]
    fn bit_length(self) -> u32 {
        Uint::bit_length(&self)
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        Uint::leading_zeros(&self)
    }
}

impl<const N: usize> FixedIntConvert for Uint<N> {
    type Limbs = [u64; N];

    #[inline]
    fn from_limbs(limbs: [u64; N]) -> Self {
        Uint::from_limbs(limbs)
    }
    #[inline]
    fn to_limbs(self) -> [u64; N] {
        *self.as_limbs()
    }
}

// ── Int<N> ──────────────────────────────────────────────────────────

impl<const N: usize> FixedInt for Int<N> {
    const LIMBS: usize = N;
    const BITS: usize = N * 64;
    const ZERO: Self = Int::<N>::ZERO;
    const ONE: Self = Int::<N>::ONE;

    #[inline]
    fn is_zero(self) -> bool {
        Int::is_zero(&self)
    }
    #[inline]
    fn is_one(self) -> bool {
        Int::is_one(&self)
    }

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        Int::wrapping_add(self, rhs)
    }
    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self {
        Int::wrapping_sub(self, rhs)
    }
    #[inline]
    fn wrapping_mul(self, rhs: Self) -> Self {
        Int::wrapping_mul(self, rhs)
    }

    #[inline]
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Int::checked_add(self, rhs)
    }
    #[inline]
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Int::checked_sub(self, rhs)
    }
    #[inline]
    fn checked_mul(self, rhs: Self) -> Option<Self> {
        Int::checked_mul(self, rhs)
    }

    #[inline]
    fn pow(self, exp: u32) -> Self {
        Int::wrapping_pow(self, exp)
    }
    #[inline]
    fn wrapping_pow(self, exp: u32) -> Self {
        Int::wrapping_pow(self, exp)
    }
    #[inline]
    fn checked_pow(self, exp: u32) -> Option<Self> {
        Int::checked_pow(self, exp)
    }

    #[inline]
    fn sqr(self) -> Self {
        self.wrapping_sqr()
    }
    #[inline]
    fn cube(self) -> Self {
        self.wrapping_cube()
    }
    #[inline]
    fn bit_length(self) -> u32 {
        Int::bit_length(&self)
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        Int::leading_zeros(&self)
    }
}

impl<const N: usize> FixedIntConvert for Int<N> {
    type Limbs = [u64; N];

    #[inline]
    fn from_limbs(limbs: [u64; N]) -> Self {
        Int::from_limbs(limbs)
    }
    #[inline]
    fn to_limbs(self) -> [u64; N] {
        *self.as_limbs()
    }
}
