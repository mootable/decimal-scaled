//! The `BigInt` trait — the single integer-layer surface.
//!
//! One trait carries everything the decimal kernels and the wide-tier
//! algorithms need from a fixed-width integer: the arithmetic +
//! optimisable surface (mirroring [`crate::DecimalArithmetic`]), the limb
//! round-trip bridge, the kernel-facing storage operations (`TEN`,
//! `isqrt`, `resize_to`, `div_rem`, `f64` bridges), and the
//! magnitude/sign cast. It is implemented for [`Int<N>`] only (the
//! `Uint<N>` magnitude type is used directly, never through this trait),
//! with static dispatch and `#[inline]` bodies, so generic width-aware
//! code pays no runtime cost.
//!
//! Methods that only make sense on one signedness (`abs` / `signum` /
//! `is_negative`) stay on the inherent impls rather than this shared
//! surface.

use super::Int;

/// 288 u64 limbs = 18432 bits — the shared `to_mag_sign` staging width
/// (covers `Int<256>` plus `isqrt` scratch slack).
const MAG_LIMBS: usize = 288;

/// The unified big-integer trait — the single common surface for every
/// fixed-width integer type. Static dispatch only.
pub trait BigInt:
    Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + core::fmt::Debug
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Rem<Output = Self>
    + core::ops::Neg<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitXor<Output = Self>
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u32, Output = Self>
    + core::ops::Shr<u32, Output = Self>
    + MagSign
{
    /// Raw little-endian limb array type (`[u64; LIMBS]`).
    type Limbs: Copy;

    /// Number of 64-bit limbs.
    const LIMBS: usize;
    /// Bit width (`LIMBS * 64`), as `u32` for the kernels' bit-length math.
    const BITS: u32;
    /// The additive identity.
    const ZERO: Self;
    /// The multiplicative identity.
    const ONE: Self;
    /// Integer constant `10` — the `10^scale` rescaling every kernel does.
    const TEN: Self;
    /// Number of u128 limbs needed to hold this type's full magnitude
    /// (`(L + 1) / 2`). Hot-path divide / rescale callers pass this as a
    /// `const N` to size their magnitude stack buffer to the exact width.
    const U128_LIMBS: usize = 1;

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

    // ── Wide-kernel surface ──────────────────────────────────────────

    /// Exact integer square root.
    fn isqrt(self) -> Self;
    /// Widening / narrowing cast to a sibling integer type.
    fn resize_to<T: BigInt>(self) -> T;
    /// Truncating quotient and remainder `(self / rhs, self % rhs)`.
    fn div_rem(self, rhs: Self) -> (Self, Self);
    /// `true` if bit `idx` of the two's-complement representation is set.
    fn bit(self, idx: u32) -> bool;
    /// Builds the value from a signed 128-bit integer.
    fn from_i128(v: i128) -> Self;
    /// The value as a signed 128-bit integer (truncating to the low 128
    /// bits; the inverse of [`BigInt::from_i128`]).
    fn to_i128(self) -> i128;
    /// `self * n` for an unsigned 64-bit multiplier (panics on overflow,
    /// matching `Mul`-operator semantics).
    fn checked_mul_u64(self, n: u64) -> Self;
    /// Nearest-`f64` value of `self` (lossy above 53 significant bits).
    fn to_f64(self) -> f64;
    /// Truncating conversion from `f64` (saturating on out-of-range).
    fn from_f64_val(v: f64) -> Self;

    // ── Limb round-trip bridge ───────────────────────────────────────

    /// Constructs from raw little-endian limbs.
    fn from_limbs(limbs: Self::Limbs) -> Self;
    /// Returns the raw little-endian limbs by value.
    fn to_limbs(self) -> Self::Limbs;

    // ── Magnitude / sign bridge (`to_mag_sign` / `from_mag_sign` come ──
    //    from the `MagSign` supertrait) ──────────────────────────────

    /// Writes the magnitude into a caller-supplied u128 limb buffer
    /// (little-endian) and returns the sign; zero-pads `dst`. Default
    /// wraps `to_mag_sign`; concrete types override with a direct copy.
    #[inline]
    fn mag_into_u128(self, dst: &mut [u128]) -> bool {
        let (mag, neg) = self.to_mag_sign();
        let n_pairs = (MAG_LIMBS / 2).min(dst.len());
        let mut i = 0;
        while i < n_pairs {
            dst[i] = (mag[2 * i] as u128) | ((mag[2 * i + 1] as u128) << 64);
            i += 1;
        }
        while i < dst.len() {
            dst[i] = 0;
            i += 1;
        }
        neg
    }
    /// Rebuilds `Self` from a u128-limb magnitude and a sign. Default
    /// unpacks each u128 into a u64 pair and routes through
    /// `from_mag_sign`; concrete types override with a direct copy.
    #[inline]
    fn from_mag_sign_u128(mag: &[u128], negative: bool) -> Self {
        let mut out = [0u64; MAG_LIMBS];
        let n_pairs = mag.len().min(MAG_LIMBS / 2);
        let mut i = 0;
        while i < n_pairs {
            out[2 * i] = mag[i] as u64;
            out[2 * i + 1] = (mag[i] >> 64) as u64;
            i += 1;
        }
        Self::from_mag_sign(&out, negative)
    }

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

/// Minimal magnitude/sign bridge used only by [`wide_cast`] — lets a
/// primitive (`u128` / `i128` / …) be a cast endpoint without
/// implementing the full [`BigInt`] surface. Temporary scaffolding:
/// retired together with `wide_cast` in the upcoming `resize`
/// simplification (direct two's-complement sign-extend).
pub(crate) trait MagSign: Copy {
    fn to_mag_sign(self) -> ([u64; MAG_LIMBS], bool);
    fn from_mag_sign(mag: &[u64], negative: bool) -> Self;
}

/// `MagSign` for a signed primitive — magnitude split into low/high u64
/// limbs (the high limb is zero for widths ≤ 64 bits).
macro_rules! impl_magsign_signed_prim {
    ($($t:ty),*) => {$(
        impl MagSign for $t {
            #[inline]
            fn to_mag_sign(self) -> ([u64; MAG_LIMBS], bool) {
                let mut out = [0u64; MAG_LIMBS];
                let mag = self.unsigned_abs() as u128;
                out[0] = mag as u64;
                out[1] = (mag >> 64) as u64;
                (out, self < 0)
            }
            #[inline]
            fn from_mag_sign(mag: &[u64], negative: bool) -> $t {
                let lo = mag.first().copied().unwrap_or(0) as u128;
                let hi = mag.get(1).copied().unwrap_or(0) as u128;
                let combined = lo | (hi << 64);
                let m = combined as $t;
                if negative { m.wrapping_neg() } else { m }
            }
        }
    )*};
}
impl_magsign_signed_prim!(i8, i16, i32, i64, i128);

impl MagSign for u128 {
    #[inline]
    fn to_mag_sign(self) -> ([u64; MAG_LIMBS], bool) {
        let mut out = [0u64; MAG_LIMBS];
        out[0] = self as u64;
        out[1] = (self >> 64) as u64;
        (out, false)
    }
    #[inline]
    fn from_mag_sign(mag: &[u64], _negative: bool) -> u128 {
        let lo = mag.first().copied().unwrap_or(0) as u128;
        let hi = mag.get(1).copied().unwrap_or(0) as u128;
        lo | (hi << 64)
    }
}

/// `Int<N>` bridges through its full `BigInt` magnitude/sign impl.
impl<const N: u32> MagSign for Int<N> {
    #[inline]
    fn to_mag_sign(self) -> ([u64; MAG_LIMBS], bool) {
        let mut out = [0u64; MAG_LIMBS];
        let mag = *self.unsigned_abs().as_limbs();
        // N never exceeds MAG_LIMBS for any width the crate instantiates.
        out[..N].copy_from_slice(&mag);
        (out, self.is_negative())
    }
    #[inline]
    fn from_mag_sign(mag: &[u64], negative: bool) -> Self {
        Self::from_mag_limbs(mag, negative)
    }
}

/// Widening / narrowing cast between any two `MagSign` types via the
/// shared magnitude + sign representation.
#[inline]
pub(crate) fn wide_cast<S: MagSign, T: MagSign>(src: S) -> T {
    let (mag, negative) = src.to_mag_sign();
    T::from_mag_sign(&mag, negative)
}

// ── BigInt for Int<N> ───────────────────────────────────────────────

impl<const N: u32> BigInt for Int<N> {
    type Limbs = [u64; N];

    const LIMBS: usize = N;
    const BITS: u32 = (N * 64) as u32;
    const ZERO: Self = Int::<N>::ZERO;
    const ONE: Self = Int::<N>::ONE;
    const TEN: Self = Int::<N>::TEN;
    const U128_LIMBS: usize = (N + 1) / 2;

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

    #[inline]
    fn isqrt(self) -> Self {
        // Magnitude isqrt, matching the macro's signed `isqrt`.
        Self::from_limbs(*self.unsigned_abs().isqrt().as_limbs())
    }

    #[inline]
    fn resize_to<T: BigInt>(self) -> T {
        // Hands this type's own N-limb magnitude straight to
        // `T::from_mag_sign`, skipping the 288-u64 staging buffer.
        let negative = self.is_negative();
        let mag = *self.unsigned_abs().as_limbs();
        T::from_mag_sign(&mag, negative)
    }

    #[inline]
    fn div_rem(self, rhs: Self) -> (Self, Self) {
        Int::div_rem(self, rhs)
    }

    #[inline]
    fn bit(self, idx: u32) -> bool {
        Int::bit(self, idx)
    }

    #[inline]
    fn from_i128(v: i128) -> Self {
        Int::from_i128(v)
    }

    #[inline]
    fn to_i128(self) -> i128 {
        self.as_i128()
    }

    #[inline]
    fn checked_mul_u64(self, n: u64) -> Self {
        Int::checked_mul_u64(self, n)
    }

    #[inline]
    fn to_f64(self) -> f64 {
        Int::to_f64(self)
    }

    #[inline]
    fn from_f64_val(v: f64) -> Self {
        Int::from_f64(v)
    }

    #[inline]
    fn from_limbs(limbs: [u64; N]) -> Self {
        Int::from_limbs(limbs)
    }
    #[inline]
    fn to_limbs(self) -> [u64; N] {
        *self.as_limbs()
    }

    /// Direct u64 → u128 limb pack into the caller's `dst` buffer. Only
    /// this type's own `N` u64 limbs are read (= `(N + 1) / 2` u128
    /// limbs); the rest of `dst` is zero-filled.
    #[inline]
    fn mag_into_u128(self, dst: &mut [u128]) -> bool {
        let mag = *self.unsigned_abs().as_limbs();
        let n_full_pairs = N / 2;
        let dst_len = dst.len();
        let mut i = 0;
        let m = n_full_pairs.min(dst_len);
        while i < m {
            dst[i] = (mag[2 * i] as u128) | ((mag[2 * i + 1] as u128) << 64);
            i += 1;
        }
        // Odd-N tail: one u64 promoted with zero high half.
        if (N & 1) == 1 && i < dst_len && i < <Self as BigInt>::U128_LIMBS {
            dst[i] = mag[2 * i] as u128;
            i += 1;
        }
        while i < dst_len {
            dst[i] = 0;
            i += 1;
        }
        self.is_negative()
    }

    /// Direct u128 → u64 limb unpack into this type's magnitude. Only
    /// `(N + 1) / 2` u128 limbs are read; excess is silently dropped.
    #[inline]
    fn from_mag_sign_u128(mag: &[u128], negative: bool) -> Self {
        let u128_limbs = (N + 1) / 2;
        let mut out = [0u64; N];
        let m = u128_limbs.min(mag.len());
        let n_full_pairs = N / 2;
        let copy_pairs = n_full_pairs.min(m);
        let mut i = 0;
        while i < copy_pairs {
            let v = mag[i];
            out[2 * i] = v as u64;
            out[2 * i + 1] = (v >> 64) as u64;
            i += 1;
        }
        // Odd-N tail: only the low u64 of mag[i] survives.
        if (N & 1) == 1 && i < m {
            out[2 * i] = mag[i] as u64;
        }
        Self::from_mag_limbs(&out, negative)
    }
}
