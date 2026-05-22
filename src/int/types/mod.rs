//! Const-generic fixed-width integers.
//!
//! A single `Uint<const LIMBS: usize>` / `Int<const LIMBS: usize>` pair
//! parameterised by the number of 64-bit little-endian limbs, replacing
//! the family of named `IntXXXX` / `UintXXXX` types. Bit width is
//! derived (`BITS = LIMBS * 64`); every width this crate uses
//! (256 … 16384 bits) is a clean multiple of 64, so the limb count is
//! the natural single parameter — it sidesteps the
//! `LIMBS = ceil(BITS / 64)` derivation that a `BITS`-parameterised type
//! cannot express on stable Rust.
//!
//! Storage is `[u64; LIMBS]`, matching the limb representation the
//! arithmetic primitives already use. Methods delegate to the
//! width-matched limb algorithms; because `LIMBS` is a compile-time
//! constant, the limb loops unroll and any `match LIMBS` arms const-fold
//! per monomorphisation — no runtime dispatch.

pub(crate) mod traits;
mod wide_compat;

pub use traits::BigInt;

use crate::int::algos::div::{div_rem_mag_fixed, isqrt_mag_fixed};
use crate::int::algos::mul::{limbs_mul_low_u64_fixed, limbs_sqr_low_u64_fixed};
use crate::int::limbs::{
    limbs_add_assign_u64_fixed, limbs_bit_len_u64_fixed, limbs_cmp_u64_fixed,
    limbs_divmod_dispatch_u64, limbs_divmod_u64, limbs_fmt_into_u64, limbs_is_zero_u64_fixed,
    limbs_mul_fast_u64, limbs_shl_u64_fixed, limbs_shr_u64_fixed, limbs_sub_assign_u64_fixed,
};
use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

/// Unsigned fixed-width integer of `N` little-endian 64-bit limbs.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Uint<const N: usize> {
    limbs: [u64; N],
}

/// Signed (two's-complement) fixed-width integer of `N` little-endian
/// 64-bit limbs.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Int<const N: usize> {
    limbs: [u64; N],
}

impl<const N: usize> Uint<N> {
    /// Number of 64-bit limbs.
    pub const LIMBS: usize = N;
    /// Bit width (`LIMBS * 64`). `u32` so it composes directly with the
    /// `leading_zeros` / `count_ones` `u32` surface and matches the
    /// historic named-type `BITS` constant.
    pub const BITS: u32 = (N as u32) * 64;

    /// Additive identity.
    pub const ZERO: Self = Self { limbs: [0; N] };
    /// Multiplicative identity.
    pub const ONE: Self = {
        let mut limbs = [0u64; N];
        limbs[0] = 1;
        Self { limbs }
    };
    /// Largest representable value (all limbs set).
    pub const MAX: Self = Self {
        limbs: [u64::MAX; N],
    };

    /// Constructs from raw little-endian limbs.
    #[inline]
    pub const fn from_limbs(limbs: [u64; N]) -> Self {
        Self { limbs }
    }

    /// Borrows the raw little-endian limbs.
    #[inline]
    pub const fn as_limbs(&self) -> &[u64; N] {
        &self.limbs
    }

    /// Wrapping addition (modulo `2^BITS`).
    #[inline]
    pub fn wrapping_add(mut self, rhs: Self) -> Self {
        limbs_add_assign_u64_fixed(&mut self.limbs, &rhs.limbs);
        self
    }

    /// Wrapping subtraction (modulo `2^BITS`).
    #[inline]
    pub fn wrapping_sub(mut self, rhs: Self) -> Self {
        limbs_sub_assign_u64_fixed(&mut self.limbs, &rhs.limbs);
        self
    }

    /// Wrapping multiplication (modulo `2^BITS`).
    ///
    /// Schoolbook multiply truncated to the low `N` limbs. Only the
    /// product limbs that land below `2^BITS` are kept, so no
    /// `[u64; 2*N]` output buffer is needed — the higher partial
    /// products are simply never written. Carries that would land at or
    /// above limb `N` are discarded, which is exactly the modular
    /// reduction.
    #[inline]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        limbs_mul_low_u64_fixed(&self.limbs, &rhs.limbs, &mut out);
        Self { limbs: out }
    }

    /// Wrapping square (`self²` modulo `2^BITS`). Named entry point for
    /// the open-coded `x * x` pattern. Uses the dedicated half-product
    /// squaring kernel (`limbs_sqr_low_u64_fixed`): each cross term is
    /// formed once and doubled, so the limb-multiply count is
    /// `N(N+1)/2` rather than the `N²` of a general multiply.
    #[inline]
    pub fn wrapping_sqr(self) -> Self {
        let mut out = [0u64; N];
        limbs_sqr_low_u64_fixed(&self.limbs, &mut out);
        Self { limbs: out }
    }

    /// Wrapping cube (`self³` modulo `2^BITS`). Named entry point for the
    /// open-coded `x * x * x` pattern; computed as `sqr` then one
    /// multiply — no cheaper form exists below two multiplies.
    #[inline]
    pub fn wrapping_cube(self) -> Self {
        self.wrapping_sqr().wrapping_mul(self)
    }

    /// Checked addition: `None` on overflow past `2^BITS`.
    #[inline]
    pub fn checked_add(mut self, rhs: Self) -> Option<Self> {
        let carry = limbs_add_assign_u64_fixed(&mut self.limbs, &rhs.limbs);
        if carry { None } else { Some(self) }
    }

    /// Checked subtraction: `None` if the result would be negative.
    #[inline]
    pub fn checked_sub(mut self, rhs: Self) -> Option<Self> {
        let borrow = limbs_sub_assign_u64_fixed(&mut self.limbs, &rhs.limbs);
        if borrow { None } else { Some(self) }
    }

    /// Checked multiplication: `None` if the true product does not fit
    /// `2^BITS`.
    #[inline]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let (a, b) = (&self.limbs, &rhs.limbs);
        let mut out = [0u64; N];
        let mut overflow = false;
        let mut i = 0;
        while i < N {
            let ai = a[i];
            if ai != 0 {
                let mut carry: u64 = 0;
                let mut j = 0;
                while j < N {
                    let prod = (ai as u128) * (b[j] as u128);
                    if i + j < N {
                        let v = prod + (out[i + j] as u128) + (carry as u128);
                        out[i + j] = v as u64;
                        carry = (v >> 64) as u64;
                    } else if prod != 0 || carry != 0 {
                        // Any product or carry landing at/above limb `N`
                        // means the true product exceeds the width.
                        overflow = true;
                        carry = 0;
                    }
                    j += 1;
                }
                if carry != 0 {
                    // Row carry would spill into limb `i + N >= N`.
                    overflow = true;
                }
            }
            i += 1;
        }
        if overflow {
            None
        } else {
            Some(Self { limbs: out })
        }
    }

    /// Wrapping division. Panics on a zero divisor, matching the
    /// behaviour of the primitive unsigned integer types.
    #[inline]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        assert!(!rhs.is_zero(), "attempt to divide by zero");
        let mut quot = [0u64; N];
        let mut rem = [0u64; N];
        limbs_divmod_u64(&self.limbs, &rhs.limbs, &mut quot, &mut rem);
        Self { limbs: quot }
    }

    /// Wrapping remainder. Panics on a zero divisor, matching the
    /// behaviour of the primitive unsigned integer types.
    #[inline]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        assert!(
            !rhs.is_zero(),
            "attempt to calculate the remainder with a divisor of zero"
        );
        let mut quot = [0u64; N];
        let mut rem = [0u64; N];
        limbs_divmod_u64(&self.limbs, &rhs.limbs, &mut quot, &mut rem);
        Self { limbs: rem }
    }

    /// Bitwise AND.
    #[inline]
    pub fn bitand(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = self.limbs[i] & rhs.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }

    /// Bitwise OR.
    #[inline]
    pub fn bitor(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = self.limbs[i] | rhs.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }

    /// Bitwise XOR.
    #[inline]
    pub fn bitxor(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = self.limbs[i] ^ rhs.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }

    /// Bitwise NOT (ones' complement).
    #[inline]
    pub fn not(self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = !self.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }

    /// Logical left shift by `shift` bits (modulo `2^BITS`).
    #[inline]
    pub fn shl(self, shift: u32) -> Self {
        let mut out = [0u64; N];
        limbs_shl_u64_fixed(&self.limbs, shift, &mut out);
        Self { limbs: out }
    }

    /// Logical right shift by `shift` bits.
    #[inline]
    pub fn shr(self, shift: u32) -> Self {
        let mut out = [0u64; N];
        limbs_shr_u64_fixed(&self.limbs, shift, &mut out);
        Self { limbs: out }
    }

    /// `true` when every limb is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        limbs_is_zero_u64_fixed(&self.limbs)
    }

    /// Bit length: `0` for zero, else `floor(log2(self)) + 1`
    /// (equivalently `BITS - leading_zeros`).
    #[inline]
    pub fn bit_length(&self) -> u32 {
        limbs_bit_len_u64_fixed(&self.limbs)
    }

    /// Number of leading zero bits in the `BITS`-wide representation.
    #[inline]
    pub fn leading_zeros(&self) -> u32 {
        (Self::BITS as u32) - self.bit_length()
    }

    /// `true` when the value equals one.
    #[inline]
    pub fn is_one(&self) -> bool {
        if N == 0 || self.limbs[0] != 1 {
            return false;
        }
        let mut i = 1;
        while i < N {
            if self.limbs[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Wrapping exponentiation by squaring (`self^exp` modulo `2^BITS`).
    /// `self^0 == 1`. Binary square-and-multiply using the dedicated
    /// squaring kernel; optimal for the small fixed exponents the root
    /// iteration needs (`k-1`, `k`).
    #[inline]
    pub fn wrapping_pow(self, mut exp: u32) -> Self {
        let mut acc = Self::ONE;
        let mut base = self;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = acc.wrapping_mul(base);
            }
            exp >>= 1;
            if exp > 0 {
                base = base.wrapping_sqr();
            }
        }
        acc
    }

    /// Exponentiation by squaring, returning `None` if the true power
    /// overflows `2^BITS`. `self^0 == 1`.
    #[inline]
    pub fn checked_pow(self, mut exp: u32) -> Option<Self> {
        let mut acc = Self::ONE;
        let mut base = self;
        loop {
            if exp & 1 == 1 {
                acc = acc.checked_mul(base)?;
            }
            exp >>= 1;
            if exp == 0 {
                break;
            }
            base = base.checked_mul(base)?;
        }
        Some(acc)
    }

    /// Exponentiation by squaring. Alias of [`Self::wrapping_pow`] for
    /// the common case where the caller has already bounded the result.
    #[inline]
    pub fn pow(self, exp: u32) -> Self {
        self.wrapping_pow(exp)
    }

    /// Integer square root: the largest `r` with `r² <= self`.
    /// Delegates to the const-`N` fast-arm (`isqrt_mag_fixed`): native
    /// `u64::isqrt` at `N == 1`, `u128::isqrt` at `N == 2`, and the shared
    /// limb isqrt (Newton with a hardware-sqrt seed) for wider `N`. All
    /// arms return the identical floor root.
    #[inline]
    pub fn isqrt(self) -> Self {
        let mut out = [0u64; N];
        isqrt_mag_fixed::<N>(&self.limbs, &mut out);
        Self { limbs: out }
    }

    /// Integer `k`th root: returns `(root, exact)` where
    /// `root = floor(self^(1/k))` and `exact` is `true` iff
    /// `root^k == self`. `k` must be `>= 1`.
    ///
    /// Brent–Zimmermann RootInt (Modern Computer Arithmetic §1.5.2): the
    /// integer projection of Newton's iteration
    /// `u = ((k-1)·s + m / s^(k-1)) / k`, started from an upper bound on
    /// the root and run until the monotone-decreasing sequence first
    /// fails to decrease (`u >= s`), at which point `s` is the floor
    /// root. The seed is the no_std-safe bit-length bound
    /// `2^ceil(bit_length / k)` — a clean upper bound since
    /// `(2^ceil(L/k))^k >= 2^L > m`. `k == 2` reuses the dedicated
    /// [`Self::isqrt`]; `k == 3` is the cube root.
    pub fn root_int(self, k: u32) -> (Self, bool) {
        debug_assert!(k >= 1, "root_int requires k >= 1");
        // Degenerate / trivial roots.
        if k == 1 {
            return (self, true);
        }
        if self.is_zero() {
            return (Self::ZERO, true);
        }
        if self.is_one() {
            return (Self::ONE, true);
        }
        if k == 2 {
            let r = self.isqrt();
            return (r, r.wrapping_sqr() == self);
        }

        // Seed: 2^ceil(bit_length / k) is an upper bound on the root.
        let len = self.bit_length();
        let seed_shift = len.div_ceil(k);
        // ceil(len/k) <= len for k >= 2, so the seed fits the width.
        let mut s = Self::ONE.shl(seed_shift);

        // Newton: s decreases monotonically while above the root.
        loop {
            // t = (k-1)*s + m / s^(k-1)
            let pow_km1 = s.wrapping_pow(k - 1);
            // pow_km1 is non-zero (s >= 1), so the divide is defined.
            let quot = self.wrapping_div(pow_km1);
            let mut t = Self::ZERO;
            let mut c = 0;
            while c < k - 1 {
                t = t.wrapping_add(s);
                c += 1;
            }
            t = t.wrapping_add(quot);
            // u = t / k
            let u = t.wrapping_div(Self::from_u64(k as u64));
            if u >= s {
                break;
            }
            s = u;
        }

        let exact = s.checked_pow(k).is_some_and(|p| p == self);
        (s, exact)
    }

    /// Constructs from a `u64`, zero-extending into the high limbs.
    #[inline]
    pub fn from_u64(value: u64) -> Self {
        let mut limbs = [0u64; N];
        if N > 0 {
            limbs[0] = value;
        }
        Self { limbs }
    }

    /// Builds from an unsigned 128-bit value (zero-extends the upper
    /// limbs).
    #[inline]
    pub const fn from_u128(v: u128) -> Self {
        Self::from_u128_bits(v)
    }

    /// Reinterprets an unsigned 128-bit value into the low 128 bits,
    /// zero-extending the rest. **Truncating** — for `Uint<1>` the high
    /// 64 bits of `v` are discarded; use [`Self::from_u128_checked`]
    /// (or `TryFrom`) when `v` may not fit.
    #[inline]
    pub const fn from_u128_bits(v: u128) -> Self {
        let mut limbs = [0u64; N];
        if N > 0 {
            limbs[0] = v as u64;
        }
        if N > 1 {
            limbs[1] = (v >> 64) as u64;
        }
        Self { limbs }
    }

    /// Exact value conversion from `u128`, or `None` if `v` does not fit
    /// `Uint<N>` (only possible for `N < 2`). For `N >= 2` every `u128` fits.
    #[inline]
    pub const fn from_u128_checked(v: u128) -> Option<Self> {
        if N >= 2 || v <= u64::MAX as u128 {
            Some(Self::from_u128_bits(v))
        } else {
            None
        }
    }

    /// Reinterprets the bit pattern as the signed sibling.
    #[inline]
    pub const fn cast_signed(self) -> Int<N> {
        Int::from_limbs(self.limbs)
    }

    /// Approximate `f64` value (positive; truncated toward zero on
    /// overflow).
    pub fn as_f64(self) -> f64 {
        let radix: f64 = 18_446_744_073_709_551_616.0; // 2^64
        let mut acc = 0.0f64;
        let mut i = N;
        while i > 0 {
            i -= 1;
            acc = acc * radix + self.limbs[i] as f64;
        }
        acc
    }

    /// Set-bit count across all limbs.
    #[inline]
    pub const fn count_ones(self) -> u32 {
        let mut total = 0;
        let mut i = 0;
        while i < N {
            total += self.limbs[i].count_ones();
            i += 1;
        }
        total
    }

    /// `true` when exactly one bit is set.
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
        self.count_ones() == 1
    }

    /// Smallest power of two `>= self` (`1` for zero), wrapping on
    /// overflow.
    pub fn next_power_of_two(self) -> Self {
        if self.is_zero() {
            return Self::ONE;
        }
        if self.is_power_of_two() {
            return self;
        }
        let bits = self.bit_length();
        let mut out = [0u64; N];
        if (bits as usize) < N * 64 {
            out[(bits / 64) as usize] = 1u64 << (bits % 64);
        }
        Self { limbs: out }
    }

    /// Parses an unsigned decimal string. Only base 10 is supported.
    pub const fn from_str_radix(s: &str, radix: u32) -> Result<Self, ()> {
        if radix != 10 {
            return Err(());
        }
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return Err(());
        }
        let mut acc = [0u64; N];
        let mut k = 0;
        while k < bytes.len() {
            let ch = bytes[k];
            if ch < b'0' || ch > b'9' {
                return Err(());
            }
            let d = (ch - b'0') as u64;
            let mut carry: u64 = d;
            let mut j = 0;
            while j < N {
                let p = (acc[j] as u128) * 10u128 + (carry as u128);
                acc[j] = p as u64;
                carry = (p >> 64) as u64;
                j += 1;
            }
            k += 1;
        }
        Ok(Self { limbs: acc })
    }
}

impl<const N: usize> Add for Uint<N> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

impl<const N: usize> Sub for Uint<N> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
}

impl<const N: usize> Mul for Uint<N> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.wrapping_mul(rhs)
    }
}

impl<const N: usize> BitAnd for Uint<N> {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Uint::bitand(self, rhs)
    }
}

impl<const N: usize> BitOr for Uint<N> {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Uint::bitor(self, rhs)
    }
}

impl<const N: usize> BitXor for Uint<N> {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Uint::bitxor(self, rhs)
    }
}

impl<const N: usize> Not for Uint<N> {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Uint::not(self)
    }
}

impl<const N: usize> Shl<u32> for Uint<N> {
    type Output = Self;
    #[inline]
    fn shl(self, shift: u32) -> Self {
        Uint::shl(self, shift)
    }
}

impl<const N: usize> Shr<u32> for Uint<N> {
    type Output = Self;
    #[inline]
    fn shr(self, shift: u32) -> Self {
        Uint::shr(self, shift)
    }
}

// Truncating unsigned division / remainder via the dispatching divmod
// (Knuth / Burnikel–Ziegler), matching the macro `$U` operators so the
// const-generic and named unsigned types share one divide algorithm.

impl<const N: usize> Div for Uint<N> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        let mut q = [0u64; N];
        let mut r = [0u64; N];
        limbs_divmod_dispatch_u64(&self.limbs, &rhs.limbs, &mut q, &mut r);
        Self { limbs: q }
    }
}

impl<const N: usize> Rem for Uint<N> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        let mut q = [0u64; N];
        let mut r = [0u64; N];
        limbs_divmod_dispatch_u64(&self.limbs, &rhs.limbs, &mut q, &mut r);
        Self { limbs: r }
    }
}

impl<const N: usize> PartialOrd for Uint<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Uint<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match limbs_cmp_u64_fixed(&self.limbs, &other.limbs) {
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl<const N: usize> Int<N> {
    /// Number of 64-bit limbs.
    pub const LIMBS: usize = N;
    /// Bit width (`LIMBS * 64`). `u32` so it composes directly with the
    /// `leading_zeros` / `count_ones` `u32` surface and matches the
    /// historic named-type `BITS` constant.
    pub const BITS: u32 = (N as u32) * 64;

    /// Additive identity.
    pub const ZERO: Self = Self { limbs: [0; N] };
    /// Multiplicative identity.
    pub const ONE: Self = {
        let mut limbs = [0u64; N];
        limbs[0] = 1;
        Self { limbs }
    };
    /// Most positive representable value (`2^(BITS-1) - 1`).
    pub const MAX: Self = {
        let mut limbs = [u64::MAX; N];
        limbs[N - 1] = i64::MAX as u64;
        Self { limbs }
    };
    /// Most negative representable value (`-2^(BITS-1)`).
    pub const MIN: Self = {
        let mut limbs = [0u64; N];
        limbs[N - 1] = 1u64 << 63;
        Self { limbs }
    };

    /// Constructs from raw little-endian two's-complement limbs.
    #[inline]
    pub const fn from_limbs(limbs: [u64; N]) -> Self {
        Self { limbs }
    }

    /// Borrows the raw little-endian limbs.
    #[inline]
    pub const fn as_limbs(&self) -> &[u64; N] {
        &self.limbs
    }

    /// `true` when every limb is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        limbs_is_zero_u64_fixed(&self.limbs)
    }

    /// `true` when the value is strictly negative (top bit set).
    #[inline]
    pub const fn is_negative(&self) -> bool {
        N > 0 && (self.limbs[N - 1] >> 63) == 1
    }

    /// `true` when the value is strictly positive (non-zero and the
    /// sign bit clear).
    #[inline]
    pub const fn is_positive(&self) -> bool {
        !self.is_negative() && !self.is_zero()
    }

    /// Two's-complement wrapping negation (`!self + 1`). `MIN` negates
    /// to itself, as with the primitive signed integers.
    #[inline]
    pub const fn wrapping_neg(self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = !self.limbs[i];
            i += 1;
        }
        let mut result = Self { limbs: out };
        let mut one = [0u64; N];
        if N > 0 {
            one[0] = 1;
        }
        limbs_add_assign_u64_fixed(&mut result.limbs, &one);
        result
    }

    /// Wrapping addition (modulo `2^BITS`). Identical bit pattern to the
    /// unsigned add — two's-complement makes signed and unsigned
    /// addition the same operation.
    #[inline]
    pub const fn wrapping_add(mut self, rhs: Self) -> Self {
        limbs_add_assign_u64_fixed(&mut self.limbs, &rhs.limbs);
        self
    }

    /// Wrapping subtraction (modulo `2^BITS`).
    #[inline]
    pub const fn wrapping_sub(mut self, rhs: Self) -> Self {
        limbs_sub_assign_u64_fixed(&mut self.limbs, &rhs.limbs);
        self
    }

    /// Wrapping multiplication (modulo `2^BITS`). The low `N` limbs of a
    /// two's-complement product are independent of the operand signs, so
    /// this is the same truncated schoolbook the unsigned type uses.
    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        limbs_mul_low_u64_fixed(&self.limbs, &rhs.limbs, &mut out);
        Self { limbs: out }
    }

    /// Absolute value (wrapping: `MIN.abs() == MIN`).
    #[inline]
    pub const fn abs(self) -> Self {
        if self.is_negative() {
            self.wrapping_neg()
        } else {
            self
        }
    }

    /// Sign: `-1`, `0`, or `1` as the value is negative, zero, or
    /// positive.
    #[inline]
    pub fn signum(&self) -> i32 {
        if self.is_zero() {
            0
        } else if self.is_negative() {
            -1
        } else {
            1
        }
    }

    /// Constructs from an `i64`, sign-extending into the high limbs.
    #[inline]
    pub const fn from_i64(value: i64) -> Self {
        // Negative values fill the upper limbs with all-ones so the
        // two's-complement representation matches at every width.
        let fill = if value < 0 { u64::MAX } else { 0 };
        let mut limbs = [fill; N];
        if N > 0 {
            limbs[0] = value as u64;
        }
        Self { limbs }
    }

    /// Constructs from an `i8` (always representable; sign-extends).
    #[inline]
    pub(crate) const fn from_i8(value: i8) -> Self {
        Self::from_i64(value as i64)
    }

    /// Constructs from an `i16` (always representable; sign-extends).
    #[inline]
    pub(crate) const fn from_i16(value: i16) -> Self {
        Self::from_i64(value as i64)
    }

    /// Constructs from an `i32` (always representable; sign-extends).
    #[inline]
    pub(crate) const fn from_i32(value: i32) -> Self {
        Self::from_i64(value as i64)
    }

    /// Constructs from a `u8` (always representable; zero-extends).
    #[inline]
    pub(crate) const fn from_u8(value: u8) -> Self {
        Self::from_u64_unsigned(value as u64)
    }

    /// Constructs from a `u16` (always representable; zero-extends).
    #[inline]
    pub(crate) const fn from_u16(value: u16) -> Self {
        Self::from_u64_unsigned(value as u64)
    }

    /// Constructs from a `u32` (always representable; zero-extends).
    #[inline]
    pub(crate) const fn from_u32(value: u32) -> Self {
        Self::from_u64_unsigned(value as u64)
    }

    /// Zero-extends an unsigned 64-bit value into limb 0. Internal helper
    /// for the unsigned `from_*` family; the public fitting check is in
    /// [`Self::try_from_u64`].
    #[inline]
    const fn from_u64_unsigned(value: u64) -> Self {
        let mut limbs = [0u64; N];
        if N > 0 {
            limbs[0] = value;
        }
        Self { limbs }
    }

    /// Exact conversion from a `u64`, or `None` if it does not fit
    /// `Int<N>`. Only `N == 1` (the `i64` floor) can fail, when the value
    /// exceeds `i64::MAX`; every wider tier holds all of `u64`.
    #[inline]
    pub(crate) const fn try_from_u64(value: u64) -> Option<Self> {
        if N >= 2 || value <= i64::MAX as u64 {
            Some(Self::from_u64_unsigned(value))
        } else {
            None
        }
    }

    /// Exact conversion from an `i128`, or `None` if it does not fit
    /// `Int<N>`. Only `N == 1` (64-bit storage) can fail; `N >= 2` holds
    /// every `i128`.
    #[inline]
    pub(crate) const fn try_from_i128(v: i128) -> Option<Self> {
        let mag = v.unsigned_abs();
        let built = Self::from_mag_limbs(&[mag as u64, (mag >> 64) as u64], v < 0);
        if N >= 2 || built.as_i128() == v {
            Some(built)
        } else {
            None
        }
    }

    /// Exact conversion from a `u128`, or `None` if it does not fit
    /// `Int<N>`. `N == 1` fails above `i64::MAX`; `N == 2` fails above
    /// `i128::MAX` (the sign bit); `N >= 3` holds every `u128`.
    #[inline]
    pub(crate) const fn try_from_u128(v: u128) -> Option<Self> {
        let built = Self::from_mag_limbs(&[v as u64, (v >> 64) as u64], false);
        if N >= 3 {
            Some(built)
        } else if built.is_negative() {
            // Magnitude landed in the sign bit of the N-limb storage.
            None
        } else if N >= 2 || built.as_i128() as u128 == v {
            Some(built)
        } else {
            None
        }
    }

    /// Lossless `i64` value, valid on the `N == 1` tier where `Int<N>`
    /// *is* an `i64`. The trait form is `From<Int<1>> for i64`.
    #[inline]
    pub(crate) const fn to_i64(self) -> i64 {
        self.as_i128() as i64
    }

    /// Lossless `i128` value, valid on the `N <= 2` tiers where every
    /// `Int<N>` fits an `i128`. The trait forms are `From<Int<1>>` /
    /// `From<Int<2>> for i128`.
    #[inline]
    pub(crate) const fn to_i128(self) -> i128 {
        self.as_i128()
    }

    /// Exact `i32` value, or `None` if out of range.
    #[inline]
    pub(crate) fn try_to_i32(self) -> Option<i32> {
        match self.to_i128_checked() {
            Some(v) if v >= i32::MIN as i128 && v <= i32::MAX as i128 => Some(v as i32),
            _ => None,
        }
    }

    /// Exact `u32` value, or `None` if negative or out of range.
    #[inline]
    pub(crate) fn try_to_u32(self) -> Option<u32> {
        match self.to_u128_checked() {
            Some(v) if v <= u32::MAX as u128 => Some(v as u32),
            _ => None,
        }
    }

    /// Exact `i64` value, or `None` if out of range.
    #[inline]
    pub(crate) fn try_to_i64(self) -> Option<i64> {
        match self.to_i128_checked() {
            Some(v) if v >= i64::MIN as i128 && v <= i64::MAX as i128 => Some(v as i64),
            _ => None,
        }
    }

    /// Exact `u64` value, or `None` if negative or out of range.
    #[inline]
    pub(crate) fn try_to_u64(self) -> Option<u64> {
        match self.to_u128_checked() {
            Some(v) if v <= u64::MAX as u128 => Some(v as u64),
            _ => None,
        }
    }

    /// Exact `i128` value, or `None` if out of range. Surface alias of
    /// [`Self::to_i128_checked`].
    #[inline]
    pub(crate) fn try_to_i128(self) -> Option<i128> {
        self.to_i128_checked()
    }

    /// Exact `u128` value, or `None` if negative or out of range. Surface
    /// alias of [`Self::to_u128_checked`].
    #[inline]
    pub(crate) fn try_to_u128(self) -> Option<u128> {
        self.to_u128_checked()
    }

    /// `true` when the value equals one.
    #[inline]
    pub fn is_one(&self) -> bool {
        if N == 0 || self.limbs[0] != 1 {
            return false;
        }
        let mut i = 1;
        while i < N {
            if self.limbs[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Most negative representable value (`-2^(BITS-1)`).
    #[inline]
    pub fn min_value() -> Self {
        let mut limbs = [0u64; N];
        if N > 0 {
            limbs[N - 1] = 1u64 << 63;
        }
        Self { limbs }
    }

    /// Most positive representable value (`2^(BITS-1) - 1`).
    #[inline]
    pub fn max_value() -> Self {
        let mut limbs = [u64::MAX; N];
        if N > 0 {
            limbs[N - 1] = u64::MAX >> 1;
        }
        Self { limbs }
    }

    /// Checked signed addition: `None` on two's-complement overflow.
    /// Overflow happens only when both operands share a sign and the
    /// result's sign differs from it.
    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let r = self.wrapping_add(rhs);
        let sa = self.is_negative();
        let sb = rhs.is_negative();
        let sr = r.is_negative();
        if sa == sb && sr != sa { None } else { Some(r) }
    }

    /// Checked signed subtraction: `None` on two's-complement overflow.
    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let r = self.wrapping_sub(rhs);
        let sa = self.is_negative();
        let sb = rhs.is_negative();
        let sr = r.is_negative();
        // Overflow when the operands' signs differ and the result takes
        // the subtrahend's sign.
        if sa != sb && sr != sa { None } else { Some(r) }
    }

    /// Checked signed multiplication: `None` if the true product does
    /// not fit the signed range. Computed via magnitudes so it reuses
    /// the unsigned overflow check, then re-signs.
    #[inline]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        if self.is_zero() || rhs.is_zero() {
            return Some(Self::ZERO);
        }
        let neg = self.is_negative() ^ rhs.is_negative();
        let ma = Uint::<N>::from_limbs(*self.abs().as_limbs());
        let mb = Uint::<N>::from_limbs(*rhs.abs().as_limbs());
        let prod = ma.checked_mul(mb)?;
        let signed = Self::from_limbs(*prod.as_limbs());
        if neg {
            let r = signed.wrapping_neg();
            // Negative magnitude must not exceed |MIN|; the round-trip
            // through wrapping_neg detects the single MIN-magnitude case.
            if r.is_negative() || r.is_zero() {
                Some(r)
            } else {
                None
            }
        } else if signed.is_negative() {
            // Positive magnitude landed in the sign bit → overflow.
            None
        } else {
            Some(signed)
        }
    }

    /// Wrapping exponentiation by squaring (`self^exp` modulo `2^BITS`).
    /// `self^0 == 1`. Uses the dedicated squaring kernel.
    #[inline]
    pub const fn wrapping_pow(self, mut exp: u32) -> Self {
        let mut acc = Self::ONE;
        let mut base = self;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = acc.wrapping_mul(base);
            }
            exp >>= 1;
            if exp > 0 {
                base = base.wrapping_sqr();
            }
        }
        acc
    }

    /// Exponentiation by squaring, returning `None` on signed overflow.
    #[inline]
    pub fn checked_pow(self, mut exp: u32) -> Option<Self> {
        let mut acc = Self::ONE;
        let mut base = self;
        loop {
            if exp & 1 == 1 {
                acc = acc.checked_mul(base)?;
            }
            exp >>= 1;
            if exp == 0 {
                break;
            }
            base = base.checked_mul(base)?;
        }
        Some(acc)
    }

    /// Wrapping square (`self²` modulo `2^BITS`) via the dedicated
    /// half-product squaring kernel. The low `N` limbs of a square are
    /// sign-independent, so the unsigned kernel applies directly.
    #[inline]
    pub const fn wrapping_sqr(self) -> Self {
        let mut out = [0u64; N];
        limbs_sqr_low_u64_fixed(&self.limbs, &mut out);
        Self { limbs: out }
    }

    /// Wrapping cube (`self³` modulo `2^BITS`): `sqr` then one multiply.
    #[inline]
    pub const fn wrapping_cube(self) -> Self {
        self.wrapping_sqr().wrapping_mul(self)
    }

    /// Bit length of the magnitude: `0` for zero, else
    /// `floor(log2|self|) + 1`.
    #[inline]
    pub fn bit_length(&self) -> u32 {
        limbs_bit_len_u64_fixed(self.abs().as_limbs())
    }

    /// Leading zero bits of the two's-complement representation, matching
    /// the primitive `iN::leading_zeros` contract. A negative value has its
    /// sign bit (the MSB) set, so it has zero leading zeros; a non-negative
    /// value's leading-zero count is `BITS - bit_length` (`BITS` for zero).
    #[inline]
    pub fn leading_zeros(&self) -> u32 {
        if self.is_negative() {
            0
        } else {
            (Self::BITS as u32) - self.bit_length()
        }
    }

    // ── BigInt / BigInt parity surface ─────────────────────────
    //
    // The methods below mirror the `decl_wide_int!`-generated `Int*`
    // surface so the const-generic `Int<N>` can satisfy the kernel-facing
    // `BigInt` / `BigInt` traits and the public `IntXXXX` API. Most
    // delegate to the existing inherent methods or the `Uint<N>` twin.

    /// Integer constant `10`, used by decimal-scale `10^scale`
    /// rescaling.
    pub const TEN: Self = {
        let mut limbs = [0u64; N];
        if N > 0 {
            limbs[0] = 10;
        }
        Self { limbs }
    };

    /// `|self|` as the unsigned twin. `MIN` maps to `2^(BITS-1)`.
    #[inline]
    pub const fn unsigned_abs(self) -> Uint<N> {
        Uint::from_limbs(*self.abs().as_limbs())
    }

    /// Two's-complement negation. Alias of [`Self::wrapping_neg`],
    /// matching the `decl_wide_int!` `negate` name.
    #[inline]
    pub fn negate(self) -> Self {
        self.wrapping_neg()
    }

    /// Truncating quotient and remainder `(self / rhs, self % rhs)` in a
    /// single divmod call. The quotient truncates toward zero and the
    /// remainder takes the sign of the dividend. Routes through the
    /// const-`N` fast-arm (`div_rem_mag_fixed`): native `u64` idiv at
    /// `N == 1`, native `u128` divide at `N == 2`, and the dispatching
    /// divmod (Knuth / Burnikel–Ziegler) for wider `N` — matching the
    /// `decl_wide_int!` `div_rem` so the const-generic and macro families
    /// share one divide algorithm. Panics on a zero divisor.
    #[inline]
    pub fn div_rem(self, rhs: Self) -> (Self, Self) {
        assert!(!rhs.is_zero(), "attempt to divide by zero");
        let neg_q = self.is_negative() ^ rhs.is_negative();
        let neg_r = self.is_negative();
        let mut quot = [0u64; N];
        let mut rem = [0u64; N];
        div_rem_mag_fixed::<N>(
            self.unsigned_abs().as_limbs(),
            rhs.unsigned_abs().as_limbs(),
            &mut quot,
            &mut rem,
        );
        let q = Self::from_mag_limbs(&quot, neg_q);
        let r = Self::from_mag_limbs(&rem, neg_r);
        (q, r)
    }

    /// Builds a signed value from a non-negative magnitude limb slice
    /// and a sign, truncating the magnitude into `N` limbs.
    #[inline]
    pub(crate) const fn from_mag_limbs(mag: &[u64], negative: bool) -> Self {
        let mut out = [0u64; N];
        let n = if mag.len() < N { mag.len() } else { N };
        let mut i = 0;
        while i < n {
            out[i] = mag[i];
            i += 1;
        }
        let v = Self { limbs: out };
        // Inherent `const` zero-check (avoids the non-const `BigInt`
        // trait method that name-resolution would otherwise pick here).
        if negative && !limbs_is_zero_u64_fixed(&v.limbs) {
            v.wrapping_neg()
        } else {
            v
        }
    }

    /// `true` if bit `idx` of the two's-complement representation is set.
    #[inline]
    pub const fn bit(self, idx: u32) -> bool {
        let limb = (idx / 64) as usize;
        if limb >= N {
            return self.is_negative();
        }
        (self.limbs[limb] >> (idx % 64)) & 1 == 1
    }

    /// Builds from a signed 128-bit value.
    #[inline]
    pub const fn from_i128(v: i128) -> Self {
        let mag = v.unsigned_abs();
        Self::from_mag_limbs(&[mag as u64, (mag >> 64) as u64], v < 0)
    }

    /// Reinterprets a signed 128-bit value into the low 128 bits of the
    /// storage, sign-extending the rest. **Truncating** — for `Int<1>`
    /// the high 64 bits of `v` are discarded; use [`Self::from_i128_checked`]
    /// (or `TryFrom`) when `v` may not fit. Identical to [`Self::from_i128`].
    #[inline]
    pub const fn from_i128_bits(v: i128) -> Self {
        let mag = v.unsigned_abs();
        Self::from_mag_limbs(&[mag as u64, (mag >> 64) as u64], v < 0)
    }

    /// Exact value conversion from `i128`, or `None` if `v` does not fit
    /// `Int<N>` (only possible for `N < 2`, where the storage is narrower
    /// than 128 bits). For `N >= 2` every `i128` fits.
    #[inline]
    pub const fn from_i128_checked(v: i128) -> Option<Self> {
        let bits = Self::from_i128_bits(v);
        if N >= 2 || bits.as_i128_bits() == v {
            Some(bits)
        } else {
            None
        }
    }

    /// Builds from an unsigned 128-bit value.
    #[inline]
    pub const fn from_u128(v: u128) -> Self {
        Self::from_mag_limbs(&[v as u64, (v >> 64) as u64], false)
    }

    /// Builds directly from the little-endian u64 limb array. Alias of
    /// [`Self::from_limbs`], matching the `decl_wide_int!` `from_limbs_le`
    /// name (the historic public surface).
    #[inline]
    pub const fn from_limbs_le(limbs: [u64; N]) -> Self {
        Self { limbs }
    }

    /// Returns the little-endian u64 limbs by value. Symmetric with
    /// [`Self::from_limbs_le`].
    #[inline]
    pub const fn limbs_le(self) -> [u64; N] {
        self.limbs
    }

    /// `self · (n as Self)` with the sign of `self`, panicking on
    /// overflow. Computes the n-by-1-word product (identical limb
    /// recurrence to the macro's `limbs_mul_u64_into`) and rejects a
    /// non-zero top carry — matching the `decl_wide_int!`
    /// `checked_mul_u64`.
    #[inline]
    pub fn checked_mul_u64(self, n: u64) -> Self {
        let mag = *self.unsigned_abs().as_limbs();
        let mut prod = [0u64; N];
        let mut carry: u64 = 0;
        let mut i = 0;
        while i < N {
            let p = (mag[i] as u128) * (n as u128) + (carry as u128);
            prod[i] = p as u64;
            carry = (p >> 64) as u64;
            i += 1;
        }
        if carry != 0 {
            panic!("Int: mul overflow");
        }
        let negative = self.is_negative();
        let r = Self::from_mag_limbs(&prod, negative);
        // `from_mag_limbs` only mishandles the `mag == 2^(BITS-1)` edge:
        // legal as MIN for `negative`, overflow otherwise.
        if !r.is_zero() && r.is_negative() != negative {
            panic!("Int: mul overflow");
        }
        r
    }

    /// Exact `i128` value, or `None` if it does not fit.
    pub fn to_i128_checked(self) -> Option<i128> {
        let negative = self.is_negative();
        let mag = *self.unsigned_abs().as_limbs();
        // First two u64 limbs make up the low u128; the rest must be 0.
        let mut i = 2;
        while i < N {
            if mag[i] != 0 {
                return None;
            }
            i += 1;
        }
        let lo = if N > 0 { mag[0] as u128 } else { 0 };
        let hi = if N > 1 { mag[1] as u128 } else { 0 };
        let lo_u128 = lo | (hi << 64);
        if negative {
            if lo_u128 <= (i128::MAX as u128) + 1 {
                Some((lo_u128 as i128).wrapping_neg())
            } else {
                None
            }
        } else if lo_u128 <= i128::MAX as u128 {
            Some(lo_u128 as i128)
        } else {
            None
        }
    }

    /// Exact `u128` value, or `None` if negative / too large.
    pub fn to_u128_checked(self) -> Option<u128> {
        if self.is_negative() {
            return None;
        }
        let mut i = 2;
        while i < N {
            if self.limbs[i] != 0 {
                return None;
            }
            i += 1;
        }
        let lo = if N > 0 { self.limbs[0] as u128 } else { 0 };
        let hi = if N > 1 { self.limbs[1] as u128 } else { 0 };
        Some(lo | (hi << 64))
    }

    /// Approximate `f64` value of `self` (lossy above 53 significant
    /// bits).
    pub fn to_f64(self) -> f64 {
        let mag = *self.unsigned_abs().as_limbs();
        let radix: f64 = 18_446_744_073_709_551_616.0; // 2^64
        let mut acc = 0.0f64;
        let mut i = N;
        while i > 0 {
            i -= 1;
            acc = acc * radix + mag[i] as f64;
        }
        if self.is_negative() { -acc } else { acc }
    }

    /// Approximate `f32` value of `self` (round-to-nearest; lossy above
    /// 24 significant bits). Routes through the `f64` accumulation to keep
    /// one summation path.
    pub fn to_f32(self) -> f32 {
        self.to_f64() as f32
    }

    /// Exact conversion from an `f64`, or `None` when `v` is NaN, ±inf,
    /// has a fractional part, or lies outside the `Int<N>` range.
    ///
    /// Not `const`: float classification (`is_finite` / `fract`) and the
    /// float→int `as` cast are not const-stable. The integer helpers it
    /// composes with stay `const`.
    pub(crate) fn try_from_f64(v: f64) -> Option<Self> {
        if !v.is_finite() || v.fract() != 0.0 {
            return None;
        }
        let negative = v < 0.0;
        let mut m = if negative { -v } else { v };
        let radix: f64 = 18_446_744_073_709_551_616.0; // 2^64
        let mut limbs = [0u64; N];
        let mut i = 0;
        while m >= 1.0 && i < N {
            let rem = m % radix;
            limbs[i] = rem as u64;
            m = (m - rem) / radix;
            i += 1;
        }
        if m >= 1.0 {
            // Magnitude needs more than `N` limbs — out of range.
            return None;
        }
        let built = Self::from_mag_limbs(&limbs, negative);
        // Reject the sign-bit overflow: a positive magnitude that landed
        // negative, or a negative one that is not the legal `MIN` edge.
        if built.is_zero() {
            Some(built)
        } else if built.is_negative() != negative {
            None
        } else {
            Some(built)
        }
    }

    /// Exact conversion from an `f32`, or `None` on NaN, ±inf, a
    /// fractional part, or out-of-range. Widens to `f64` (lossless for
    /// `f32`) and defers to [`Self::try_from_f64`]. Not `const` for the
    /// same float-op reason.
    pub(crate) fn try_from_f32(v: f32) -> Option<Self> {
        Self::try_from_f64(v as f64)
    }

    /// Builds from an `f64`, truncating toward zero. Saturates to
    /// `MIN` / `MAX` on out-of-range; non-finite maps to `ZERO`.
    pub fn from_f64(v: f64) -> Self {
        if !v.is_finite() {
            return Self::ZERO;
        }
        let negative = v < 0.0;
        let mut m = if negative { -v } else { v };
        let radix: f64 = 18_446_744_073_709_551_616.0; // 2^64
        let mut limbs = [0u64; N];
        let mut i = 0;
        while m >= 1.0 && i < N {
            let rem = m % radix;
            limbs[i] = rem as u64;
            m = (m - rem) / radix;
            i += 1;
        }
        if m >= 1.0 {
            return if negative {
                Self::min_value()
            } else {
                Self::max_value()
            };
        }
        Self::from_mag_limbs(&limbs, negative)
    }

    /// Parses a signed decimal magnitude from `s`. Accepts an optional
    /// leading `-`, then ASCII digits. Only `radix == 10` is supported;
    /// any other value returns `Err(())`.
    pub const fn from_str_radix(s: &str, radix: u32) -> Result<Self, ()> {
        if radix != 10 {
            return Err(());
        }
        let bytes = s.as_bytes();
        let (negative, start): (bool, usize) = if !bytes.is_empty() && bytes[0] == b'-' {
            (true, 1)
        } else {
            (false, 0)
        };
        if start >= bytes.len() {
            return Err(());
        }
        // acc = acc * 10 + d per digit, truncating into N limbs — the
        // same Horner recurrence the macro runs through `limbs_mul_u64`
        // + `limbs_add_assign_u64`, but the low-N-limb multiply-by-10 is
        // folded into one n-by-1-word pass (no `2*N` staging buffer).
        let mut acc = [0u64; N];
        let mut k = start;
        while k < bytes.len() {
            let ch = bytes[k];
            if ch < b'0' || ch > b'9' {
                return Err(());
            }
            let d = (ch - b'0') as u64;
            let mut carry: u64 = d;
            let mut j = 0;
            while j < N {
                let p = (acc[j] as u128) * 10u128 + (carry as u128);
                acc[j] = p as u64;
                carry = (p >> 64) as u64;
                j += 1;
            }
            k += 1;
        }
        Ok(Self::from_mag_limbs(&acc, negative))
    }

    // ── Named-type API parity (the `decl_wide_int!` `$S` surface) ─────
    //
    // The methods below complete the inherent surface the macro `Int*`
    // structs expose, so the `IntXXXX = Int<N>` aliases keep every call
    // site resolving. Behaviour-preserving ports of the macro bodies.

    /// Integer power: `self^exp` (wrapping on overflow). Alias of
    /// [`Self::wrapping_pow`], matching the macro's `pow` name.
    #[inline]
    pub const fn pow(self, exp: u32) -> Self {
        self.wrapping_pow(exp)
    }

    /// Integer square root of the magnitude (`floor(sqrt(|self|))`),
    /// returned non-negative. Matches the macro's signed `isqrt`.
    #[inline]
    pub fn isqrt(self) -> Self {
        Self::from_limbs(*self.unsigned_abs().isqrt().as_limbs())
    }

    /// Reinterprets the bit pattern as the unsigned sibling.
    #[inline]
    pub const fn cast_unsigned(self) -> Uint<N> {
        Uint::from_limbs(self.limbs)
    }

    /// Approximate `f64` value. Alias of [`Self::to_f64`], matching the
    /// macro's `as_f64` name.
    #[inline]
    pub fn as_f64(self) -> f64 {
        self.to_f64()
    }

    /// Count of set bits across the two's-complement representation.
    #[inline]
    pub const fn count_ones(self) -> u32 {
        let mut total = 0;
        let mut i = 0;
        while i < N {
            total += self.limbs[i].count_ones();
            i += 1;
        }
        total
    }

    /// Count of clear bits across the two's-complement representation.
    #[inline]
    pub const fn count_zeros(self) -> u32 {
        Self::BITS - self.count_ones()
    }

    /// Number of trailing zero bits; `BITS` for zero.
    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
        let mut i = 0;
        while i < N {
            if self.limbs[i] != 0 {
                return i as u32 * 64 + self.limbs[i].trailing_zeros();
            }
            i += 1;
        }
        Self::BITS
    }

    /// Checked negation: `None` exactly at `MIN` (whose negation
    /// overflows the signed range).
    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        if limbs_cmp_u64_fixed(&self.limbs, &Self::MIN.limbs) == 0 {
            None
        } else {
            Some(self.wrapping_neg())
        }
    }

    /// Checked division: `None` on a zero divisor.
    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if limbs_is_zero_u64_fixed(&rhs.limbs) {
            None
        } else {
            Some(self.wrapping_div(rhs))
        }
    }

    /// Checked remainder: `None` on a zero divisor, and `None` for the
    /// `MIN % -1` overflow case (the paired division `MIN / -1` overflows
    /// the signed range), matching the primitive integer contract.
    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if limbs_is_zero_u64_fixed(&rhs.limbs) {
            None
        } else if self.is_min_neg_one(rhs) {
            None
        } else {
            Some(self.wrapping_rem(rhs))
        }
    }

    /// `true` when `self == MIN` and `rhs == -1` — the remainder/division
    /// overflow case where `MIN / -1` exceeds the signed range.
    #[inline]
    const fn is_min_neg_one(self, rhs: Self) -> bool {
        limbs_cmp_u64_fixed(&self.limbs, &Self::MIN.limbs) == 0
            && limbs_cmp_u64_fixed(&rhs.wrapping_neg().limbs, &Self::ONE.limbs) == 0
    }

    /// Euclidean division: the quotient that leaves a non-negative
    /// remainder.
    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        let q = self.wrapping_div(rhs);
        let r = self.wrapping_rem(rhs);
        if r.is_negative() {
            if rhs.is_negative() {
                q.wrapping_add(Self::ONE)
            } else {
                q.wrapping_sub(Self::ONE)
            }
        } else {
            q
        }
    }

    /// Euclidean remainder — always non-negative.
    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        let r = self.wrapping_rem(rhs);
        if r.is_negative() {
            r.wrapping_add(rhs.abs())
        } else {
            r
        }
    }

    /// Wrapping addition paired with the two's-complement overflow flag.
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let r = self.wrapping_add(rhs);
        let sa = self.is_negative();
        let sb = rhs.is_negative();
        let sr = r.is_negative();
        (r, sa == sb && sr != sa)
    }

    /// Wrapping subtraction paired with the two's-complement overflow
    /// flag.
    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let r = self.wrapping_sub(rhs);
        let sa = self.is_negative();
        let sb = rhs.is_negative();
        let sr = r.is_negative();
        (r, sa != sb && sr != sa)
    }

    /// Wrapping negation paired with the overflow flag (`true` only at
    /// `MIN`).
    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let ov = limbs_cmp_u64_fixed(&self.limbs, &Self::MIN.limbs) == 0;
        (self.wrapping_neg(), ov)
    }

    /// Wrapping remainder paired with an overflow flag. The flag is `true`
    /// only for `MIN % -1` (whose paired division overflows the signed
    /// range), in which case the remainder is `0`; otherwise `false`.
    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        if self.is_min_neg_one(rhs) {
            (Self::ZERO, true)
        } else {
            (self.wrapping_rem(rhs), false)
        }
    }

    /// Saturating addition: clamps to `MIN` / `MAX` on overflow.
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(v) => v,
            None => {
                if self.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
        }
    }

    /// Saturating subtraction: clamps to `MIN` / `MAX` on overflow.
    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        match self.checked_sub(rhs) {
            Some(v) => v,
            None => {
                if self.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
        }
    }

    /// Saturating negation: `MIN` saturates to `MAX`.
    #[inline]
    pub const fn saturating_neg(self) -> Self {
        match self.checked_neg() {
            Some(v) => v,
            None => Self::MAX,
        }
    }

    /// Rotates the bits left by `n` (modulo `BITS`).
    #[inline]
    pub fn rotate_left(self, n: u32) -> Self {
        let bits = Self::BITS;
        let n = n % bits;
        if n == 0 {
            return self;
        }
        let u = self.cast_unsigned();
        Self::from_limbs(((u.shl(n)) | (u.shr(bits - n))).limbs)
    }

    /// Rotates the bits right by `n` (modulo `BITS`).
    #[inline]
    pub fn rotate_right(self, n: u32) -> Self {
        self.rotate_left(Self::BITS - (n % Self::BITS))
    }

    /// Truncating cast to `i128` (low 128 bits, sign-applied).
    #[inline]
    pub const fn as_i128(self) -> i128 {
        self.as_i128_bits()
    }

    /// Reinterprets the low 128 bits as `i128`, sign-applied.
    /// **Truncating** — for `Int<3+>` any value outside the `i128` range
    /// loses its high limbs; use [`Self::to_i128_checked`] (or `TryFrom`)
    /// when the value may not fit.
    #[inline]
    pub const fn as_i128_bits(self) -> i128 {
        let mag = *self.unsigned_abs().as_limbs();
        let lo = if N > 0 { mag[0] as u128 } else { 0 };
        let hi = if N > 1 { mag[1] as u128 } else { 0 };
        let combined = lo | (hi << 64);
        if self.is_negative() {
            (combined as i128).wrapping_neg()
        } else {
            combined as i128
        }
    }

    /// Widening / narrowing cast to any other [`BigInt`] type, via the
    /// shared magnitude + sign bridge. Matches the macro's
    /// `resize::<T>()` signature so the decimal-tier code that calls
    /// `storage.resize::<$Wider>()` resolves against `Int<N>`.
    #[inline]
    pub(crate) fn resize<T: crate::int::types::traits::BigInt>(self) -> T {
        let negative = self.is_negative();
        let mag = *self.unsigned_abs().as_limbs();
        T::from_mag_sign(&mag, negative)
    }

    /// Truncating division toward zero. Panics on a zero divisor.
    /// Matches the macro's `wrapping_div` (single-limb-aware
    /// `limbs_divmod_u64`, not the dispatching `div_rem`).
    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        if limbs_is_zero_u64_fixed(&rhs.limbs) {
            panic!("attempt to divide by zero");
        }
        let negative = self.is_negative() ^ rhs.is_negative();
        let mut q = [0u64; N];
        let mut r = [0u64; N];
        limbs_divmod_u64(
            self.unsigned_abs().as_limbs(),
            rhs.unsigned_abs().as_limbs(),
            &mut q,
            &mut r,
        );
        Self::from_mag_limbs(&q, negative)
    }

    /// Truncating remainder; result carries the sign of `self`. Panics
    /// on a zero divisor. Matches the macro's `wrapping_rem`.
    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        if limbs_is_zero_u64_fixed(&rhs.limbs) {
            panic!("attempt to calculate the remainder with a divisor of zero");
        }
        let mut q = [0u64; N];
        let mut r = [0u64; N];
        limbs_divmod_u64(
            self.unsigned_abs().as_limbs(),
            rhs.unsigned_abs().as_limbs(),
            &mut q,
            &mut r,
        );
        Self::from_mag_limbs(&r, self.is_negative())
    }

    /// Full `self · rhs` product widened into a `W: BigInt`, in one
    /// step (no double trip through the magnitude staging buffer). Used
    /// by the wide-tier `Mul` operator to compute
    /// `Storage * Storage → Wider`. Matches the macro's `widen_mul`.
    #[inline]
    pub(crate) fn widen_mul<W: crate::int::types::traits::BigInt>(self, rhs: Self) -> W {
        let negative = self.is_negative() ^ rhs.is_negative();
        let a = *self.unsigned_abs().as_limbs();
        let b = *rhs.unsigned_abs().as_limbs();
        // Full product spans 2·N limbs; the shared 288-limb magnitude
        // staging width (covers Int16384) bounds every instantiation.
        // Route through the equal-length multiply dispatcher: both
        // operands are `[u64; N]`, so this is the single site every wide
        // tier's full product flows through. The dispatcher base-cases to
        // schoolbook below `KARATSUBA_THRESHOLD_U64` (every shipped tier)
        // and engages the non-allocating Karatsuba kernel at or above it,
        // so one call lifts every width that crosses the threshold.
        let mut prod = [0u64; 288];
        limbs_mul_fast_u64(&a, &b, &mut prod[..2 * N]);
        W::from_mag_sign(&prod, negative)
    }
}

impl<const N: usize> PartialOrd for Int<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// `i128` interop for the 128-bit storage `Int<2>` (D38's backend).
// `Int<2>` *is* an `i128`, so the comparison is the direct `as_i128`
// value — no widening round-trip. This lets `to_bits()` results compare
// against `i128` literals without an explicit conversion at the call
// site. Deliberately `Int<2>`-only: for wider tiers an `i128` comparison
// would be a lossy narrowing and is not offered.
// `Int<2>` *is* a 128-bit integer, so the conversion to `i128` is exact
// (the trait form of `as_i128`). Enables `i128::from(int2)` / `.into()`.
impl From<Int<2>> for i128 {
    #[inline]
    fn from(v: Int<2>) -> i128 {
        v.as_i128()
    }
}

impl PartialEq<i128> for Int<2> {
    #[inline]
    fn eq(&self, other: &i128) -> bool {
        self.as_i128() == *other
    }
}
impl PartialEq<Int<2>> for i128 {
    #[inline]
    fn eq(&self, other: &Int<2>) -> bool {
        *self == other.as_i128()
    }
}
impl PartialOrd<i128> for Int<2> {
    #[inline]
    fn partial_cmp(&self, other: &i128) -> Option<Ordering> {
        self.as_i128().partial_cmp(other)
    }
}
impl PartialOrd<Int<2>> for i128 {
    #[inline]
    fn partial_cmp(&self, other: &Int<2>) -> Option<Ordering> {
        self.partial_cmp(&other.as_i128())
    }
}

// `i64` interop for the 64-bit storage `Int<1>` (D18's backend). `Int<1>`
// *is* an `i64`, so the bridge is the direct value (its `as_i128()` is the
// sign-extended `i64`) — letting `to_bits()` results compare against `i64`
// literals without an explicit conversion. Deliberately `Int<1>`-only.
impl From<Int<1>> for i64 {
    #[inline]
    fn from(v: Int<1>) -> i64 {
        v.as_i128() as i64
    }
}
// `Int<1>` is 64-bit, so widening to `i128` is exact.
impl From<Int<1>> for i128 {
    #[inline]
    fn from(v: Int<1>) -> i128 {
        v.as_i128()
    }
}
impl PartialEq<i64> for Int<1> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        self.as_i128() == *other as i128
    }
}
impl PartialEq<Int<1>> for i64 {
    #[inline]
    fn eq(&self, other: &Int<1>) -> bool {
        *self as i128 == other.as_i128()
    }
}
impl PartialOrd<i64> for Int<1> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.as_i128().partial_cmp(&(i128::from(*other)))
    }
}
impl PartialOrd<Int<1>> for i64 {
    #[inline]
    fn partial_cmp(&self, other: &Int<1>) -> Option<Ordering> {
        i128::from(*self).partial_cmp(&other.as_i128())
    }
}

impl<const N: usize> Ord for Int<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // Signed compare: a negative value is always less than a
        // non-negative one. When the signs agree the two's-complement
        // bit patterns order the same way as the unsigned magnitude
        // comparison of the limbs.
        match (self.is_negative(), other.is_negative()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => match limbs_cmp_u64_fixed(&self.limbs, &other.limbs) {
                -1 => Ordering::Less,
                1 => Ordering::Greater,
                _ => Ordering::Equal,
            },
        }
    }
}

impl<const N: usize> Add for Int<N> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

impl<const N: usize> Sub for Int<N> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
}

impl<const N: usize> Mul for Int<N> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.wrapping_mul(rhs)
    }
}

impl<const N: usize> BitAnd for Int<N> {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = self.limbs[i] & rhs.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }
}

impl<const N: usize> BitOr for Int<N> {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = self.limbs[i] | rhs.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }
}

impl<const N: usize> BitXor for Int<N> {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = self.limbs[i] ^ rhs.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }
}

impl<const N: usize> Not for Int<N> {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        let mut out = [0u64; N];
        let mut i = 0;
        while i < N {
            out[i] = !self.limbs[i];
            i += 1;
        }
        Self { limbs: out }
    }
}

impl<const N: usize> Shl<u32> for Int<N> {
    type Output = Self;
    #[inline]
    fn shl(self, shift: u32) -> Self {
        let mut out = [0u64; N];
        limbs_shl_u64_fixed(&self.limbs, shift, &mut out);
        Self { limbs: out }
    }
}

impl<const N: usize> Shr<u32> for Int<N> {
    type Output = Self;
    #[inline]
    fn shr(self, shift: u32) -> Self {
        // Arithmetic (sign-preserving) right shift — matches Rust's signed
        // `>>` and the prior macro `Int*` types the transcendental range
        // reduction relies on. Two's-complement: x >> s == !((!x) >> s) for x < 0.
        let neg = self.is_negative();
        let src = if neg { !self } else { self };
        let mut out = [0u64; N];
        limbs_shr_u64_fixed(&src.limbs, shift, &mut out);
        let shifted = Self { limbs: out };
        if neg { !shifted } else { shifted }
    }
}

impl<const N: usize> Neg for Int<N> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self.wrapping_neg()
    }
}

// ── Div / Rem ───────────────────────────────────────────────────────
//
// Truncating signed division / remainder, delegating to the dispatching
// `div_rem` so the operators share the macro types' divide algorithm
// (`limbs_divmod_dispatch_u64`: Knuth / Burnikel–Ziegler for multi-limb
// divisors). These supertraits are what `BigInt` requires.

impl<const N: usize> Div for Int<N> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        self.div_rem(rhs).0
    }
}

impl<const N: usize> Rem for Int<N> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        self.div_rem(rhs).1
    }
}

// ── Display / FromStr ───────────────────────────────────────────────
//
// Delegate to the same limb fmt / parse path the `decl_wide_int!` macro
// types use, so the const-generic surface round-trips identically.

impl<const N: usize> core::fmt::Display for Uint<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Stack scratch sized to the widest tier (256 limbs = Uint16384),
        // matching the `Int<N>` Display impl below.
        let mut buf = [0u8; 256 * 64];
        let s = limbs_fmt_into_u64(&self.limbs, 10, true, &mut buf);
        f.pad_integral(true, "", s)
    }
}

impl<const N: usize> core::fmt::Display for Int<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mag = *self.unsigned_abs().as_limbs();
        // Decimal needs <= BITS bytes (`BITS * log10(2) < BITS`); the
        // formatter writes from the buffer tail. A stack scratch sized
        // to the crate's widest tier (256 limbs = Int16384) keeps the
        // const-generic body `core`-clean — no `[u8; N*64]` const
        // expression and no `alloc` dependency, matching the macro's
        // per-width `[u8; $L * 64]` buffer.
        const MAX_DIGITS: usize = 256 * 64;
        let mut buf = [0u8; MAX_DIGITS];
        let s = limbs_fmt_into_u64(&mag, 10, true, &mut buf);
        f.pad_integral(!self.is_negative() || self.is_zero(), "", s)
    }
}

impl<const N: usize> core::str::FromStr for Int<N> {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_str_radix(s, 10)
    }
}

// ── Radix formatting (raw two's-complement bit pattern) ─────────────
//
// `LowerHex` / `UpperHex` / `Octal` / `Binary` print the raw limb bit
// pattern (not a signed magnitude), matching the macro `$S` impls.
// Stack scratch sized to the widest tier (256 limbs = Int16384), as in
// the `Display` impl above.

impl<const N: usize> core::fmt::LowerHex for Int<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buf = [0u8; 256 * 64];
        let s = limbs_fmt_into_u64(&self.limbs, 16, true, &mut buf);
        f.pad_integral(true, "0x", s)
    }
}

impl<const N: usize> core::fmt::UpperHex for Int<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buf = [0u8; 256 * 64];
        let s = limbs_fmt_into_u64(&self.limbs, 16, false, &mut buf);
        f.pad_integral(true, "0x", s)
    }
}

impl<const N: usize> core::fmt::Octal for Int<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buf = [0u8; 256 * 64];
        let s = limbs_fmt_into_u64(&self.limbs, 8, true, &mut buf);
        f.pad_integral(true, "0o", s)
    }
}

impl<const N: usize> core::fmt::Binary for Int<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buf = [0u8; 256 * 64];
        let s = limbs_fmt_into_u64(&self.limbs, 2, true, &mut buf);
        f.pad_integral(true, "0b", s)
    }
}

// ── Width conversion: widen (lossless) / narrow (fallible) ─────────
//
// `Uint<N>` and `Uint<M>` are different-sized stack types, so a value
// conversion builds a fresh `[u64; M]` — there is no heap allocation,
// and reinterpreting across widths via `transmute` would be unsound
// (different size and layout). `resize` writes each destination limb
// exactly once via `array::from_fn`; `widen` is the infallible extend
// and `narrow` the information-preserving truncation. Stable Rust
// cannot constrain `M >= N` / `M <= N` in the type system, so the
// direction is enforced by `debug_assert!` plus, for `narrow`, the
// `Option` return.

impl<const N: usize> Uint<N> {
    /// Resizes to `M` limbs: zero-extends when widening, drops the high
    /// limbs when narrowing. Direction-agnostic and infallible.
    ///
    /// Named `resize_n` (not `resize`) so the const-generic width bridge
    /// does not collide with the type-generic `Int::resize` the named-
    /// type API expects.
    #[inline]
    pub fn resize_n<const M: usize>(self) -> Uint<M> {
        Uint::from_limbs(core::array::from_fn(
            |i| if i < N { self.limbs[i] } else { 0 },
        ))
    }

    /// Widens to a wider `Uint<M>` (`M >= N`), zero-extending the new
    /// high limbs. Lossless.
    #[inline]
    pub fn widen<const M: usize>(self) -> Uint<M> {
        debug_assert!(M >= N, "widen requires M >= N");
        self.resize_n::<M>()
    }

    /// Narrows to a narrower `Uint<M>` (`M <= N`). Returns `None` if any
    /// discarded high limb is non-zero (the value does not fit `M`).
    #[inline]
    pub fn narrow<const M: usize>(self) -> Option<Uint<M>> {
        debug_assert!(M <= N, "narrow requires M <= N");
        let keep = if M < N { M } else { N };
        let mut i = keep;
        while i < N {
            if self.limbs[i] != 0 {
                return None;
            }
            i += 1;
        }
        Some(self.resize_n::<M>())
    }
}

impl<const N: usize> Int<N> {
    /// Resizes to `M` limbs: sign-extends when widening, drops the high
    /// limbs when narrowing. Direction-agnostic and infallible
    /// (narrowing may change the represented value).
    ///
    /// Named `resize_n` (not `resize`) so the const-generic width bridge
    /// does not collide with the type-generic `Int::resize` the named-
    /// type API expects (the magnitude-bridge cast over any `BigInt`).
    #[inline]
    pub fn resize_n<const M: usize>(self) -> Int<M> {
        let fill = if self.is_negative() { u64::MAX } else { 0 };
        Int::from_limbs(core::array::from_fn(|i| {
            if i < N { self.limbs[i] } else { fill }
        }))
    }

    /// Widens to a wider `Int<M>` (`M >= N`), sign-extending. Lossless.
    #[inline]
    pub fn widen<const M: usize>(self) -> Int<M> {
        debug_assert!(M >= N, "widen requires M >= N");
        self.resize_n::<M>()
    }

    /// Narrows to a narrower `Int<M>` (`1 <= M <= N`). Returns `None`
    /// unless every discarded high limb is a pure sign-extension of the
    /// narrowed value's top bit — i.e. the value fits `M` limbs as
    /// two's complement.
    #[inline]
    pub fn narrow<const M: usize>(self) -> Option<Int<M>> {
        debug_assert!(M >= 1 && M <= N, "narrow requires 1 <= M <= N");
        let sign_fill = if (self.limbs[M - 1] >> 63) == 1 {
            u64::MAX
        } else {
            0
        };
        let mut i = M;
        while i < N {
            if self.limbs[i] != sign_fill {
                return None;
            }
            i += 1;
        }
        Some(self.resize_n::<M>())
    }
}



// ── std-aligned primitive conversions ───────────────────────────────
// Infallible widening (the source always fits `Int<N>` / `Uint<N>` for
// N >= 1, judged against the `N == 1` = `i64` floor) is `From`. Fallible
// narrowing (the value may exceed the target range) is `TryFrom`,
// returning [`ConvertError::Overflow`]. Each impl is a thin one-line
// delegate to the inherent const base. `Into` / `TryInto` come for free.

// ── IN: primitive → Int<N> ───────────────────────────────────────────
macro_rules! int_from_signed {
    ($($prim:ty => $base:ident),+) => {$(
        impl<const N: usize> From<$prim> for Int<N> {
            #[inline]
            fn from(v: $prim) -> Self { Self::$base(v) }
        }
    )+};
}
int_from_signed!(i8 => from_i8, i16 => from_i16, i32 => from_i32, i64 => from_i64);

macro_rules! int_from_unsigned {
    ($($prim:ty => $base:ident),+) => {$(
        impl<const N: usize> From<$prim> for Int<N> {
            #[inline]
            fn from(v: $prim) -> Self { Self::$base(v) }
        }
    )+};
}
int_from_unsigned!(u8 => from_u8, u16 => from_u16, u32 => from_u32);

macro_rules! uint_from_unsigned {
    ($($prim:ty),+) => {$(
        impl<const N: usize> From<$prim> for Uint<N> {
            #[inline]
            fn from(v: $prim) -> Self { Self::from_u64(v as u64) }
        }
    )+};
}
uint_from_unsigned!(u8, u16, u32, u64);

// Fallible IN: `u64` can overflow `Int<1>`; `i128` / `u128` can overflow
// the narrow tiers (see the inherent `try_from_*`).
impl<const N: usize> TryFrom<u64> for Int<N> {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Self::try_from_u64(v).ok_or(crate::support::error::ConvertError::Overflow)
    }
}

impl<const N: usize> TryFrom<i128> for Int<N> {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: i128) -> Result<Self, Self::Error> {
        Self::try_from_i128(v).ok_or(crate::support::error::ConvertError::Overflow)
    }
}

impl<const N: usize> TryFrom<u128> for Int<N> {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: u128) -> Result<Self, Self::Error> {
        Self::try_from_u128(v).ok_or(crate::support::error::ConvertError::Overflow)
    }
}

// ── OUT: Int<N> → primitive ──────────────────────────────────────────
// `i64` / `i128` are always representable on the fitting tiers, so those
// are infallible `From` (declared above: `From<Int<1>> for i64`,
// `From<Int<1>>` / `From<Int<2>> for i128`). The std blanket
// `impl<T,U: From<T>> TryFrom<T> for U` then derives the matching
// `TryFrom` for those tiers, so a generic `TryFrom<Int<N>> for i64`/`i128`
// is omitted to avoid the coherence conflict. The remaining widths /
// targets are genuinely fallible:
impl<const N: usize> TryFrom<Int<N>> for i32 {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: Int<N>) -> Result<Self, Self::Error> {
        v.try_to_i32().ok_or(crate::support::error::ConvertError::Overflow)
    }
}

impl<const N: usize> TryFrom<Int<N>> for u32 {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: Int<N>) -> Result<Self, Self::Error> {
        v.try_to_u32().ok_or(crate::support::error::ConvertError::Overflow)
    }
}

impl<const N: usize> TryFrom<Int<N>> for u64 {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: Int<N>) -> Result<Self, Self::Error> {
        v.try_to_u64().ok_or(crate::support::error::ConvertError::Overflow)
    }
}

impl<const N: usize> TryFrom<Int<N>> for u128 {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: Int<N>) -> Result<Self, Self::Error> {
        v.try_to_u128().ok_or(crate::support::error::ConvertError::Overflow)
    }
}

impl<const N: usize> TryFrom<u128> for Uint<N> {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: u128) -> Result<Self, Self::Error> {
        Self::from_u128_checked(v).ok_or(crate::support::error::ConvertError::Overflow)
    }
}

// ── Floats ───────────────────────────────────────────────────────────
// Float → Int<N> is fallible (NaN, ±inf, non-integer, out-of-range), so
// `TryFrom`. There is no `From<Int<N>> for f64/f32`: that direction is
// lossy above the mantissa width and is offered only as the inherent
// `to_f64` / `to_f32` round-to-nearest methods.
impl<const N: usize> TryFrom<f64> for Int<N> {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        Self::try_from_f64(v).ok_or(crate::support::error::ConvertError::Overflow)
    }
}

impl<const N: usize> TryFrom<f32> for Int<N> {
    type Error = crate::support::error::ConvertError;
    #[inline]
    fn try_from(v: f32) -> Result<Self, Self::Error> {
        Self::try_from_f32(v).ok_or(crate::support::error::ConvertError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int::algos::mul::{limbs_mul_low_u64_fixed, limbs_mul_u64_fixed};

    #[test]
    fn from_i128_checked_detects_narrow_overflow() {
        // Int<2> (128-bit) holds every i128.
        assert_eq!(Int::<2>::from_i128_checked(i128::MAX), Some(Int::<2>::from_i128_bits(i128::MAX)));
        assert_eq!(Int::<2>::from_i128_checked(i128::MIN), Some(Int::<2>::from_i128_bits(i128::MIN)));
        // Int<1> (64-bit) holds only the i64 range.
        assert_eq!(Int::<1>::from_i128_checked(i64::MAX as i128 + 1), None);
        assert_eq!(Int::<1>::from_i128_checked(i64::MIN as i128 - 1), None);
        assert_eq!(
            Int::<1>::from_i128_checked(i64::MAX as i128),
            Some(Int::<1>::from_i64(i64::MAX))
        );
        // TryFrom mirrors it.
        assert!(<Int<1> as TryFrom<i128>>::try_from(i64::MAX as i128 + 1).is_err());
        assert_eq!(<Int<2> as TryFrom<i128>>::try_from(-5_i128), Ok(Int::<2>::from_i64(-5)));
    }

    #[test]
    fn from_u128_checked_detects_narrow_overflow() {
        assert_eq!(Uint::<2>::from_u128_checked(u128::MAX), Some(Uint::<2>::from_u128_bits(u128::MAX)));
        assert_eq!(Uint::<1>::from_u128_checked(u64::MAX as u128 + 1), None);
        assert_eq!(Uint::<1>::from_u128_checked(u64::MAX as u128), Some(Uint::<1>::from_u64(u64::MAX)));
        assert!(<Uint<1> as TryFrom<u128>>::try_from(u64::MAX as u128 + 1).is_err());
    }

    #[test]
    fn as_i128_bits_round_trips_at_edges() {
        // MIN/MAX of the 128-bit storage reinterpret exactly.
        assert_eq!(Int::<2>::MAX.as_i128_bits(), i128::MAX);
        assert_eq!(Int::<2>::MIN.as_i128_bits(), i128::MIN);
        assert_eq!(Int::<1>::from_i64(-123).as_i128_bits(), -123_i128);
    }

    #[test]
    fn from_traits_are_infallible_for_fitting_prims() {
        assert_eq!(Int::<1>::from(-7_i32), Int::<1>::from_i64(-7));
        assert_eq!(Int::<4>::from(42_i64), Int::<4>::from_i64(42));
        assert_eq!(Uint::<1>::from(7_u32), Uint::<1>::from_u64(7));
        assert_eq!(Uint::<4>::from(u64::MAX), Uint::<4>::from_u64(u64::MAX));
    }

    /// The `BigInt` / `BigInt` surface must compile and
    /// behave through a fully generic bound, for both signed and
    /// unsigned fixed-width integers.
    #[test]
    fn fixed_int_trait_surface() {
        fn exercises<T: BigInt>(seven: T, three: T) {
            assert_eq!(T::LIMBS as u32 * 64, T::BITS);
            assert!(T::ZERO.is_zero());
            assert!(T::ONE.is_one());
            assert!(!T::ZERO.is_one());

            // Operators via the supertrait bounds.
            let ten = seven + three;
            assert_eq!(ten - three, seven);
            assert_eq!(ten, seven.wrapping_add(three));
            assert_eq!(seven.wrapping_sub(three) + three, seven);

            // Bitwise / shift surface.
            let _ = (seven & three) | (seven ^ three);
            let _ = !T::ZERO;
            assert_eq!((T::ONE << 4) >> 4, T::ONE);

            // Checked arithmetic returns Some for in-range work.
            assert_eq!(seven.checked_add(three), Some(ten));
            assert!(seven.checked_mul(three).is_some());

            // Powers and optimisable functions agree with each other.
            assert_eq!(three.sqr(), three * three);
            assert_eq!(three.cube(), three * three * three);
            assert_eq!(three.pow(2), three.sqr());
            assert_eq!(three.wrapping_pow(3), three.cube());
            assert_eq!(three.checked_pow(2), Some(three.sqr()));

            // bit_length / leading_zeros consistency.
            assert_eq!(T::ONE.bit_length(), 1);
            assert_eq!(T::ZERO.bit_length(), 0);
            assert_eq!(T::ONE.leading_zeros(), (T::BITS as u32) - 1);

            // Reductions and the limb round-trip.
            let items = [T::ONE, T::ONE, T::ONE];
            assert_eq!(T::sum(items), three);
            assert_eq!(T::product([three, T::ONE]), three);
            assert_eq!(T::from_limbs(seven.to_limbs()), seven);
        }

        exercises(Int::<4>::from_i64(7), Int::<4>::from_i64(3));
        exercises(Int::<6>::from_i64(7), Int::<6>::from_i64(3));
    }

    /// The truncated low-`N` product must equal the low `N` limbs of
    /// the full `2N`-limb schoolbook product, across widths and edges.
    #[test]
    fn limbs_mul_low_matches_full_product_low_half() {
        fn check<const N: usize, const D: usize>(a: [u64; N], b: [u64; N]) {
            debug_assert!(D == 2 * N);
            let mut full = [0u64; D];
            limbs_mul_u64_fixed::<N, D>(&a, &b, &mut full);
            let mut low = [0u64; N];
            limbs_mul_low_u64_fixed::<N>(&a, &b, &mut low);
            let mut expected = [0u64; N];
            expected.copy_from_slice(&full[..N]);
            assert_eq!(low, expected, "low-half mismatch for {a:?} * {b:?}");
        }

        // Width 4 (256-bit): zero, one, MAX, cross-limb spans.
        check::<4, 8>([0, 0, 0, 0], [0, 0, 0, 0]);
        check::<4, 8>([1, 0, 0, 0], [u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
        check::<4, 8>([u64::MAX; 4], [u64::MAX; 4]);
        check::<4, 8>([0, 1, 0, 0], [0, 1, 0, 0]); // 2^64 * 2^64
        check::<4, 8>(
            [0xDEAD_BEEF, 0xCAFE_F00D, 0x1234, 0x5678_9ABC],
            [0xFEED_FACE, 0x0BAD_C0DE, 0x9999, 0x0000_0001],
        );
        // Width 2 and width 6 to exercise other monomorphisations.
        check::<2, 4>([u64::MAX, u64::MAX], [3, 0]);
        check::<6, 12>([7, 8, 9, 10, 11, 12], [1, 2, 3, 4, 5, 6]);
    }

    /// The dedicated squaring kernel must be bit-exact against the
    /// general truncated product `x · x` at every width and edge case.
    #[test]
    fn dedicated_sqr_matches_general_mul() {
        fn check<const N: usize>(x: [u64; N]) {
            let a = Uint::<N>::from_limbs(x);
            assert_eq!(
                a.wrapping_sqr(),
                a.wrapping_mul(a),
                "sqr != mul(self,self) for {x:?}"
            );
        }

        // Width 4: 0, 1, MAX, single-limb, cross-limb, full-width.
        check::<4>([0, 0, 0, 0]);
        check::<4>([1, 0, 0, 0]);
        check::<4>([u64::MAX; 4]);
        check::<4>([0x1234_5678, 0, 0, 0]);
        check::<4>([u64::MAX, u64::MAX, 0, 0]);
        check::<4>([0xDEAD_BEEF_CAFE_F00D, 0x0123_4567_89AB_CDEF, 0xFEDC, 0x99]);
        // Carry-heavy: every limb all-ones but the top.
        check::<4>([u64::MAX, u64::MAX, u64::MAX, 0]);
        // Other widths / monomorphisations.
        check::<1>([u64::MAX]);
        check::<2>([u64::MAX, u64::MAX]);
        check::<6>([7, 8, 9, 10, 11, 12]);
        check::<8>([u64::MAX, 1, u64::MAX, 2, u64::MAX, 3, u64::MAX, 4]);
        // A pseudo-random sweep across width 5.
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..64 {
            check::<5>([next(), next(), next(), next(), next()]);
        }
    }

    #[test]
    fn uint_sqr_cube_match_naive() {
        let x = Uint::<4>::from_limbs([123_456_789, 0, 0, 0]);
        assert_eq!(x.wrapping_sqr(), x.wrapping_mul(x));
        assert_eq!(x.wrapping_cube(), x.wrapping_mul(x).wrapping_mul(x));

        // A value spanning two limbs, to exercise cross-limb products.
        let y = Uint::<4>::from_limbs([0xDEAD_BEEF_CAFE, 0x1234_5678, 0, 0]);
        assert_eq!(y.wrapping_sqr(), y.wrapping_mul(y));
        assert_eq!(y.wrapping_cube(), y.wrapping_mul(y).wrapping_mul(y));
    }

    /// `root_int(k)` must return `floor(m^(1/k))` with the correct
    /// exactness flag: `root^k <= m < (root+1)^k`, and `exact` iff
    /// `root^k == m`. Checked against a brute-force reference for small
    /// `m` and `k in {2,3,5}` across widths, plus cross-checking k=2
    /// against the shipped isqrt and large perfect-power exactness.
    #[test]
    fn root_int_floor_and_exactness() {
        // Brute-force floor kth root of a small u128.
        fn brute(m: u128, k: u32) -> (u128, bool) {
            if m == 0 {
                return (0, true);
            }
            let mut r: u128 = 0;
            // Smallest r with (r+1)^k > m, capped to avoid overflow.
            while {
                let next = r + 1;
                next.checked_pow(k).is_some_and(|p| p <= m)
            } {
                r += 1;
            }
            (r, r.pow(k) == m)
        }

        fn check<const N: usize>(m: u128, k: u32) {
            let lo = (m & 0xFFFF_FFFF_FFFF_FFFF) as u64;
            let hi = (m >> 64) as u64;
            let mut limbs = [0u64; N];
            limbs[0] = lo;
            if N > 1 {
                limbs[1] = hi;
            }
            let n = Uint::<N>::from_limbs(limbs);
            let (root, exact) = n.root_int(k);
            let (eroot, eexact) = brute(m, k);
            let root_lo = root.as_limbs()[0] as u128
                | ((if N > 1 { root.as_limbs()[1] as u128 } else { 0 }) << 64);
            assert_eq!(root_lo, eroot, "root mismatch for m={m}, k={k}");
            assert_eq!(exact, eexact, "exact flag mismatch for m={m}, k={k}");

            // The defining bracket, computed in-width.
            let rk = root.pow(k);
            assert!(rk <= n, "root^k > m for m={m}, k={k}");
            let next = root.wrapping_add(Uint::<N>::ONE);
            // (root+1)^k overflowing the width still satisfies > m.
            match next.checked_pow(k) {
                Some(p) => assert!(p > n, "(root+1)^k <= m for m={m}, k={k}"),
                None => {}
            }
        }

        let samples: [u128; 14] = [
            0,
            1,
            2,
            7,
            8,
            9,
            26,
            27,
            28,
            1000,
            1023,
            1024,
            1_000_000,
            u64::MAX as u128,
        ];
        for &m in &samples {
            for k in [2u32, 3, 5] {
                check::<4>(m, k);
                check::<2>(m, k);
                check::<8>(m, k);
            }
        }

        // k=2 cross-check against shipped isqrt for a wide value.
        let big = Uint::<4>::from_limbs([0xFFFF_FFFF_FFFF_FFFF, 0x1234_5678, 0, 0]);
        assert_eq!(big.root_int(2).0, big.isqrt());

        // Large exact perfect cube: (2^40)^3 = 2^120.
        let base = Uint::<4>::ONE.shl(40);
        let cube = base.wrapping_cube();
        let (r, exact) = cube.root_int(3);
        assert_eq!(r, base);
        assert!(exact);
        // One less than a perfect cube is not exact and floors down.
        let (r2, exact2) = cube.wrapping_sub(Uint::<4>::ONE).root_int(3);
        assert_eq!(r2, base.wrapping_sub(Uint::<4>::ONE));
        assert!(!exact2);
    }

    #[test]
    fn uint_widen_zero_extends() {
        let a = Uint::<2>::from_limbs([7, 8]);
        let w = a.widen::<4>();
        assert_eq!(*w.as_limbs(), [7, 8, 0, 0]);
        assert_eq!(a.resize_n::<4>(), w);
    }

    #[test]
    fn uint_narrow_checks_dropped_limbs() {
        let fits = Uint::<4>::from_limbs([7, 8, 0, 0]);
        assert_eq!(*fits.narrow::<2>().unwrap().as_limbs(), [7, 8]);

        let too_big = Uint::<4>::from_limbs([7, 8, 1, 0]);
        assert!(too_big.narrow::<2>().is_none());

        // widen → narrow round-trips losslessly.
        let a = Uint::<2>::from_limbs([3, 4]);
        assert_eq!(a.widen::<4>().narrow::<2>().unwrap(), a);
    }

    #[test]
    fn int_widen_sign_extends() {
        // -1 is all-ones; sign-extension keeps it all-ones at any width.
        let neg = Int::<2>::from_i64(-1);
        assert_eq!(*neg.widen::<4>().as_limbs(), [u64::MAX; 4]);
        assert_eq!(neg.widen::<4>(), Int::<4>::from_i64(-1));
        // Positive values zero-extend.
        let pos = Int::<2>::from_i64(5);
        assert_eq!(*pos.widen::<4>().as_limbs(), [5, 0, 0, 0]);
    }

    #[test]
    fn int_narrow_requires_sign_consistency() {
        // Negative value whose dropped limbs are all the sign fill: fits.
        let neg = Int::<4>::from_i64(-1);
        assert_eq!(neg.narrow::<2>().unwrap(), Int::<2>::from_i64(-1));
        // Positive magnitude with a non-sign high limb: does not fit.
        let big = Int::<4>::from_limbs([0, 0, 1, 0]);
        assert!(big.narrow::<2>().is_none());
        // Negative top bit but a dropped limb that isn't all-ones: no fit.
        let weird = Int::<4>::from_limbs([1, 0, 0, u64::MAX]);
        assert!(weird.narrow::<2>().is_none());
        // Small value round-trips.
        let p = Int::<2>::from_i64(42);
        assert_eq!(p.widen::<4>().narrow::<2>().unwrap(), p);
    }

    #[test]
    fn consts_and_round_trip() {
        assert_eq!(Uint::<4>::LIMBS, 4);
        assert_eq!(Uint::<4>::BITS, 256);
        assert_eq!(*Uint::<4>::ZERO.as_limbs(), [0, 0, 0, 0]);
        assert_eq!(*Uint::<4>::ONE.as_limbs(), [1, 0, 0, 0]);
        assert_eq!(*Uint::<4>::MAX.as_limbs(), [u64::MAX; 4]);

        let v = Uint::<4>::from_limbs([7, 8, 9, 10]);
        assert_eq!(*v.as_limbs(), [7, 8, 9, 10]);
    }

    #[test]
    fn widths_have_expected_bits() {
        assert_eq!(Int::<4>::BITS, 256);
        assert_eq!(Int::<64>::BITS, 4096);
        assert_eq!(Uint::<16>::LIMBS, 16);
    }

    #[test]
    fn wrapping_sub_borrows_across_limbs() {
        // 2^64 - 1 ... computed as 0 - 1 wrapping, then check a clean borrow.
        let a = Uint::<4>::from_limbs([0, 1, 0, 0]);
        let d = a.wrapping_sub(Uint::<4>::ONE);
        assert_eq!(*d.as_limbs(), [u64::MAX, 0, 0, 0]);

        // 0 - 1 wraps to all-ones (modulo 2^256).
        let wrap = Uint::<4>::ZERO.wrapping_sub(Uint::<4>::ONE);
        assert_eq!(*wrap.as_limbs(), [u64::MAX; 4]);
    }

    #[test]
    fn unsigned_ordering() {
        let small = Uint::<4>::from_limbs([5, 0, 0, 0]);
        let big = Uint::<4>::from_limbs([0, 1, 0, 0]); // 2^64 > 5
        assert!(small < big);
        assert!(big > small);
        assert_eq!(small, small);
        assert!(Uint::<4>::ZERO < Uint::<4>::MAX);
        // round-trips through derived PartialOrd helpers
        assert_eq!(small.max(big), big);
    }

    #[test]
    fn wrapping_add_carries_across_limbs() {
        // (2^64 - 1) + 1 carries into the next limb.
        let a = Uint::<4>::from_limbs([u64::MAX, 0, 0, 0]);
        let sum = a.wrapping_add(Uint::<4>::ONE);
        assert_eq!(*sum.as_limbs(), [0, 1, 0, 0]);

        // All-ones + 1 wraps to zero (modulo 2^256).
        let wrap = Uint::<4>::MAX.wrapping_add(Uint::<4>::ONE);
        assert_eq!(*wrap.as_limbs(), [0, 0, 0, 0]);
    }

    #[test]
    fn uint_wrapping_mul_cross_limb_product() {
        // 2^64 * 2^64 = 2^128 — lands exactly in limb 2.
        let a = Uint::<4>::from_limbs([0, 1, 0, 0]);
        let p = a.wrapping_mul(a);
        assert_eq!(*p.as_limbs(), [0, 0, 1, 0]);

        // (2^64 - 1) * 3 = 3*2^64 - 3 = [u64::MAX - 2, 2, 0, 0].
        let m = Uint::<4>::from_limbs([u64::MAX, 0, 0, 0]);
        let three = Uint::<4>::from_limbs([3, 0, 0, 0]);
        let q = m.wrapping_mul(three);
        assert_eq!(*q.as_limbs(), [u64::MAX - 2, 2, 0, 0]);

        // Multiply by one is identity.
        let v = Uint::<4>::from_limbs([7, 8, 9, 10]);
        assert_eq!(v.wrapping_mul(Uint::<4>::ONE), v);
    }

    #[test]
    fn uint_wrapping_mul_truncates_modulo_width() {
        // 2^192 * 2^192 = 2^384, fully above 2^256 → wraps to zero.
        let hi = Uint::<4>::from_limbs([0, 0, 0, 1]);
        assert_eq!(hi.wrapping_mul(hi), Uint::<4>::ZERO);

        // MAX * MAX mod 2^256: (2^256 - 1)^2 = 2^512 - 2^257 + 1.
        // mod 2^256 that is 1 (since 2^512 and 2^257 are both 0 mod
        // 2^256). So the low limb is 1, the rest zero.
        let r = Uint::<4>::MAX.wrapping_mul(Uint::<4>::MAX);
        assert_eq!(*r.as_limbs(), [1, 0, 0, 0]);
    }

    #[test]
    fn uint_checked_add_sub_overflow() {
        assert_eq!(
            Uint::<4>::ONE.checked_add(Uint::<4>::ONE),
            Some(Uint::<4>::from_limbs([2, 0, 0, 0]))
        );
        // MAX + 1 overflows.
        assert_eq!(Uint::<4>::MAX.checked_add(Uint::<4>::ONE), None);

        assert_eq!(
            Uint::<4>::from_limbs([5, 0, 0, 0]).checked_sub(Uint::<4>::from_limbs([3, 0, 0, 0])),
            Some(Uint::<4>::from_limbs([2, 0, 0, 0]))
        );
        // 0 - 1 underflows.
        assert_eq!(Uint::<4>::ZERO.checked_sub(Uint::<4>::ONE), None);
    }

    #[test]
    fn uint_checked_mul_overflow() {
        // 2^64 * 2^64 = 2^128 fits in 256 bits.
        let a = Uint::<4>::from_limbs([0, 1, 0, 0]);
        assert_eq!(a.checked_mul(a), Some(Uint::<4>::from_limbs([0, 0, 1, 0])));
        // 2^192 * 2^192 overflows 256 bits.
        let hi = Uint::<4>::from_limbs([0, 0, 0, 1]);
        assert_eq!(hi.checked_mul(hi), None);
        // MAX * 2 overflows.
        assert_eq!(
            Uint::<4>::MAX.checked_mul(Uint::<4>::from_limbs([2, 0, 0, 0])),
            None
        );
    }

    #[test]
    fn uint_div_rem_with_remainder() {
        // 1000 / 7 = 142 r 6.
        let n = Uint::<4>::from_limbs([1000, 0, 0, 0]);
        let d = Uint::<4>::from_limbs([7, 0, 0, 0]);
        assert_eq!(*n.wrapping_div(d).as_limbs(), [142, 0, 0, 0]);
        assert_eq!(*n.wrapping_rem(d).as_limbs(), [6, 0, 0, 0]);

        // 2^128 / 2^64 = 2^64, remainder 0.
        let big = Uint::<4>::from_limbs([0, 0, 1, 0]);
        let by = Uint::<4>::from_limbs([0, 1, 0, 0]);
        assert_eq!(*big.wrapping_div(by).as_limbs(), [0, 1, 0, 0]);
        assert!(big.wrapping_rem(by).is_zero());
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn uint_div_by_zero_panics() {
        let _ = Uint::<4>::ONE.wrapping_div(Uint::<4>::ZERO);
    }

    #[test]
    fn uint_bitwise_ops() {
        let a = Uint::<4>::from_limbs([0b1100, 0xFF, 0, 0]);
        let b = Uint::<4>::from_limbs([0b1010, 0x0F, 0, 0]);
        assert_eq!(*(a & b).as_limbs(), [0b1000, 0x0F, 0, 0]);
        assert_eq!(*(a | b).as_limbs(), [0b1110, 0xFF, 0, 0]);
        assert_eq!(*(a ^ b).as_limbs(), [0b0110, 0xF0, 0, 0]);
        assert_eq!(*(!Uint::<4>::ZERO).as_limbs(), [u64::MAX; 4]);
    }

    #[test]
    fn uint_shifts() {
        let one = Uint::<4>::ONE;
        // 1 << 64 lands in limb 1.
        assert_eq!(*(one << 64).as_limbs(), [0, 1, 0, 0]);
        // 1 << 130 = limb 2 bit 2.
        assert_eq!(*(one << 130).as_limbs(), [0, 0, 0b100, 0]);
        // Right shift back.
        let v = Uint::<4>::from_limbs([0, 0, 0b100, 0]);
        assert_eq!(*(v >> 130).as_limbs(), [1, 0, 0, 0]);
        // Shift past the width drops everything.
        assert_eq!(one << 256, Uint::<4>::ZERO);
    }

    #[test]
    fn uint_is_zero_bitlen_leading_zeros() {
        assert!(Uint::<4>::ZERO.is_zero());
        assert!(!Uint::<4>::ONE.is_zero());
        assert_eq!(Uint::<4>::ZERO.bit_length(), 0);
        assert_eq!(Uint::<4>::ONE.bit_length(), 1);
        // 2^64 has bit length 65.
        let b = Uint::<4>::from_limbs([0, 1, 0, 0]);
        assert_eq!(b.bit_length(), 65);
        assert_eq!(b.leading_zeros(), 256 - 65);
        assert_eq!(Uint::<4>::ZERO.leading_zeros(), 256);
        assert_eq!(Uint::<4>::MAX.leading_zeros(), 0);
    }

    #[test]
    fn uint_operator_traits_delegate() {
        let a = Uint::<4>::from_limbs([10, 0, 0, 0]);
        let b = Uint::<4>::from_limbs([3, 0, 0, 0]);
        assert_eq!(*(a + b).as_limbs(), [13, 0, 0, 0]);
        assert_eq!(*(a - b).as_limbs(), [7, 0, 0, 0]);
        assert_eq!(*(a * b).as_limbs(), [30, 0, 0, 0]);
    }

    #[test]
    fn int_from_i64_sign_extends() {
        // Positive: only the low limb is set.
        assert_eq!(*Int::<4>::from_i64(5).as_limbs(), [5, 0, 0, 0]);
        // -1 sign-extends to all-ones.
        assert_eq!(*Int::<4>::from_i64(-1).as_limbs(), [u64::MAX; 4]);
        // -2 → low limb u64::MAX - 1, upper limbs all-ones.
        assert_eq!(
            *Int::<4>::from_i64(-2).as_limbs(),
            [u64::MAX - 1, u64::MAX, u64::MAX, u64::MAX]
        );
        assert!(Int::<4>::from_i64(-1).is_negative());
        assert!(Int::<4>::from_i64(1).is_positive());
        assert!(Int::<4>::from_i64(0).is_zero());
    }

    #[test]
    fn int_wrapping_neg_two_complement() {
        let five = Int::<4>::from_i64(5);
        let neg_five = five.wrapping_neg();
        assert_eq!(neg_five, Int::<4>::from_i64(-5));
        // Negating twice returns the original.
        assert_eq!(neg_five.wrapping_neg(), five);
        // -1 negates to 1.
        assert_eq!(Int::<4>::from_i64(-1).wrapping_neg(), Int::<4>::ONE);
        // Neg operator delegates.
        assert_eq!(-five, neg_five);
    }

    #[test]
    fn int_add_sub_mul_with_signs() {
        let a = Int::<4>::from_i64(7);
        let b = Int::<4>::from_i64(-3);
        // 7 + (-3) = 4
        assert_eq!(a.wrapping_add(b), Int::<4>::from_i64(4));
        // 7 - (-3) = 10
        assert_eq!(a.wrapping_sub(b), Int::<4>::from_i64(10));
        // 7 * (-3) = -21
        assert_eq!(a.wrapping_mul(b), Int::<4>::from_i64(-21));
        // (-3) * (-3) = 9
        assert_eq!(b.wrapping_mul(b), Int::<4>::from_i64(9));
        // operator delegation
        assert_eq!(a + b, Int::<4>::from_i64(4));
        assert_eq!(a - b, Int::<4>::from_i64(10));
        assert_eq!(a * b, Int::<4>::from_i64(-21));
    }

    #[test]
    fn int_mul_crosses_limbs_with_sign() {
        // 2^64 * (-1) = -2^64.
        let big = Int::<4>::from_i64(0).wrapping_add(Int::<4>::from_limbs([0, 1, 0, 0]));
        let neg = big.wrapping_mul(Int::<4>::from_i64(-1));
        assert_eq!(neg, big.wrapping_neg());
        // -2^64 should be [0, u64::MAX, u64::MAX, u64::MAX].
        assert_eq!(*neg.as_limbs(), [0, u64::MAX, u64::MAX, u64::MAX]);
    }

    #[test]
    fn int_abs_signum() {
        assert_eq!(Int::<4>::from_i64(-9).abs(), Int::<4>::from_i64(9));
        assert_eq!(Int::<4>::from_i64(9).abs(), Int::<4>::from_i64(9));
        assert_eq!(Int::<4>::from_i64(-9).signum(), -1);
        assert_eq!(Int::<4>::from_i64(9).signum(), 1);
        assert_eq!(Int::<4>::from_i64(0).signum(), 0);
    }

    #[test]
    fn int_from_i128_round_trips() {
        for v in [
            0i128,
            1,
            -1,
            42,
            -42,
            i64::MAX as i128,
            i64::MIN as i128,
            i128::MAX,
            i128::MIN,
            123_456_789_012_345_678,
        ] {
            let a = Int::<4>::from_i128(v);
            assert_eq!(a.to_i128_checked(), Some(v), "round-trip i128 {v}");
            let b = Int::<8>::from_i128(v);
            assert_eq!(b.to_i128_checked(), Some(v), "round-trip i128 {v} (N=8)");
        }
        // from_u128 of a value above i128::MAX is not representable as i128.
        let big = Int::<4>::from_u128(u128::MAX);
        assert_eq!(big.to_i128_checked(), None);
        assert_eq!(big.to_u128_checked(), Some(u128::MAX));
        // Negative has no u128.
        assert_eq!(Int::<4>::from_i128(-1).to_u128_checked(), None);
    }

    #[test]
    fn int_from_str_radix_and_display_round_trip() {
        let cases = [
            "0",
            "1",
            "-1",
            "42",
            "-42",
            "1000000000000000000000",
            "-340282366920938463463374607431768211455",
        ];
        for s in cases {
            let v = Int::<4>::from_str_radix(s, 10).unwrap();
            assert_eq!(format!("{v}"), s, "display round-trip {s}");
            // FromStr delegates to from_str_radix(_, 10).
            let v2: Int<4> = s.parse().unwrap();
            assert_eq!(v, v2);
        }
        // Non-base-10 and malformed input reject.
        assert!(Int::<4>::from_str_radix("10", 16).is_err());
        assert!(Int::<4>::from_str_radix("12x", 10).is_err());
        assert!(Int::<4>::from_str_radix("", 10).is_err());
        assert!(Int::<4>::from_str_radix("-", 10).is_err());
    }

    #[test]
    fn int_div_rem_signs_match_truncating() {
        // Truncating division: quotient truncates toward zero, remainder
        // carries the sign of the dividend.
        let cases: [(i128, i128); 6] = [
            (1000, 7),
            (-1000, 7),
            (1000, -7),
            (-1000, -7),
            (7, 1000),
            (-7, 1000),
        ];
        for (a, b) in cases {
            let (q, r) = Int::<4>::from_i128(a).div_rem(Int::<4>::from_i128(b));
            assert_eq!(q.to_i128_checked(), Some(a / b), "quot {a}/{b}");
            assert_eq!(r.to_i128_checked(), Some(a % b), "rem {a}%{b}");
        }
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn int_div_rem_by_zero_panics() {
        let _ = Int::<4>::ONE.div_rem(Int::<4>::ZERO);
    }

    #[test]
    fn int_bit_reads_twos_complement() {
        let v = Int::<4>::from_i128(0b1010);
        assert!(!v.bit(0));
        assert!(v.bit(1));
        assert!(!v.bit(2));
        assert!(v.bit(3));
        // Above the value's set bits: clear for positive.
        assert!(!v.bit(200));
        // Negative: high bits read as the sign extension (all ones).
        let neg = Int::<4>::from_i128(-1);
        assert!(neg.bit(0));
        assert!(neg.bit(255));
        assert!(neg.bit(1000)); // out of range → sign bit
    }

    #[test]
    fn int_checked_mul_u64_matches_wide_mul() {
        let v = Int::<4>::from_i128(123_456_789);
        assert_eq!(
            v.checked_mul_u64(1000),
            Int::<4>::from_i128(123_456_789_000)
        );
        // Sign preserved.
        let n = Int::<4>::from_i128(-123_456_789);
        assert_eq!(
            n.checked_mul_u64(1000),
            Int::<4>::from_i128(-123_456_789_000)
        );
        // Times zero / one.
        assert_eq!(v.checked_mul_u64(0), Int::<4>::ZERO);
        assert_eq!(v.checked_mul_u64(1), v);
    }

    #[test]
    #[should_panic(expected = "mul overflow")]
    fn int_checked_mul_u64_overflow_panics() {
        // max_value * 2 overflows the signed range.
        let _ = Int::<4>::max_value().checked_mul_u64(2);
    }

    #[test]
    fn int_div_rem_operators_match_div_rem() {
        // The Div / Rem operators must agree with the inherent div_rem,
        // which is what BigInt requires as supertraits.
        let a = Int::<4>::from_i128(-1000);
        let b = Int::<4>::from_i128(7);
        assert_eq!(a / b, Int::<4>::from_i128(-1000 / 7));
        assert_eq!(a % b, Int::<4>::from_i128(-1000 % 7));
        let (q, r) = a.div_rem(b);
        assert_eq!(a / b, q);
        assert_eq!(a % b, r);
    }

    #[test]
    fn int_wide_storage_surface() {
        use crate::int::types::traits::BigInt;
        fn exercises<T: BigInt>() {
            assert!(<T as BigInt>::ZERO == <T as BigInt>::from_i128(0));
            assert!(<T as BigInt>::ONE == <T as BigInt>::from_i128(1));
            assert!(<T as BigInt>::TEN == <T as BigInt>::from_i128(10));

            let twelve = <T as BigInt>::from_i128(12);
            let three = <T as BigInt>::from_i128(3);
            // pow / isqrt
            assert!(three.pow(3) == <T as BigInt>::from_i128(27));
            assert!(<T as BigInt>::from_i128(144).isqrt() == <T as BigInt>::from_i128(12));
            // div_rem
            let (q, r) = twelve.div_rem(<T as BigInt>::from_i128(5));
            assert!(q == <T as BigInt>::from_i128(2));
            assert!(r == <T as BigInt>::from_i128(2));
            // bit / leading_zeros
            assert!(twelve.bit(2) && twelve.bit(3) && !twelve.bit(0));
            assert!(<T as BigInt>::ONE.leading_zeros() == <T as BigInt>::BITS - 1);
            // checked_mul_u64 / f64 round-trips
            assert!(twelve.checked_mul_u64(10) == <T as BigInt>::from_i128(120));
            assert!(twelve.to_f64() == 12.0);
            assert!(<T as BigInt>::from_f64_val(7.9) == <T as BigInt>::from_i128(7));
        }
        exercises::<Int<4>>();
        exercises::<Int<8>>();

        // resize_to widens/narrows across the family.
        let v = Int::<4>::from_i128(-123_456_789);
        let w: Int<8> = BigInt::resize_to(v);
        assert_eq!(w.to_i128_checked(), Some(-123_456_789));
        let back: Int<4> = BigInt::resize_to(w);
        assert_eq!(back, v);
    }

    #[test]
    fn int_isqrt_matches_uint_magnitude() {
        use crate::int::types::traits::BigInt;
        // Signed isqrt is the magnitude isqrt (macro parity).
        let n = Int::<4>::from_i128(1_000_000_000_000);
        let r = BigInt::isqrt(n);
        assert_eq!(r, Int::<4>::from_i128(1_000_000));
        // Perfect square round-trip at width 8.
        let big = Int::<8>::from_i128(987_654_321);
        let sq = big.checked_mul(big).unwrap();
        assert_eq!(BigInt::isqrt(sq), big);
    }

    #[test]
    fn int_wideint_mag_sign_round_trips() {
        use crate::int::types::traits::BigInt;
        use crate::int::types::traits::MagSign;
        // to_mag_sign / from_mag_sign round-trip for signed values,
        // including the magnitude + sign split.
        for v in [0i128, 1, -1, 123_456_789_012_345_678, -987_654_321] {
            let a = Int::<4>::from_i128(v);
            let (mag, neg) = a.to_mag_sign();
            assert_eq!(neg, a.is_negative());
            assert_eq!(Int::<4>::from_mag_sign(&mag, neg), a, "mag/sign {v}");
        }
        // U128_LIMBS = ceil(N/2): even and odd N.
        assert_eq!(<Int<4> as BigInt>::U128_LIMBS, 2);
        assert_eq!(<Int<3> as BigInt>::U128_LIMBS, 2);
        assert_eq!(<Int<8> as BigInt>::U128_LIMBS, 4);

        // mag_into_u128 / from_mag_sign_u128 round-trip (the hot-path
        // buffer bypass), including the odd-N tail at N=3.
        let v3 = Int::<3>::from_i128(-170_141_183_460_469_231_731);
        let mut buf = [0u128; 2];
        let neg = v3.mag_into_u128(&mut buf);
        assert_eq!(Int::<3>::from_mag_sign_u128(&buf, neg), v3);

        let v4 = Int::<4>::from_i128(i128::MIN);
        let mut buf4 = [0u128; 2];
        let neg4 = v4.mag_into_u128(&mut buf4);
        assert_eq!(Int::<4>::from_mag_sign_u128(&buf4, neg4), v4);
    }

    #[test]
    fn int_to_from_f64_and_negate_ten() {
        assert_eq!(Int::<4>::from_i64(5).to_f64(), 5.0);
        assert_eq!(Int::<4>::from_i64(-5).to_f64(), -5.0);
        assert_eq!(Int::<4>::from_f64(42.9), Int::<4>::from_i64(42));
        assert_eq!(Int::<4>::from_f64(-42.9), Int::<4>::from_i64(-42));
        // Non-finite maps to ZERO.
        assert_eq!(Int::<4>::from_f64(f64::NAN), Int::<4>::ZERO);
        // negate is wrapping_neg; TEN const is 10.
        assert_eq!(Int::<4>::from_i64(5).negate(), Int::<4>::from_i64(-5));
        assert_eq!(Int::<4>::TEN, Int::<4>::from_i64(10));
        // from_limbs_le / limbs_le round-trip.
        let v = Int::<4>::from_i128(-9_876_543_210);
        assert_eq!(Int::<4>::from_limbs_le(v.limbs_le()), v);
    }

    #[test]
    fn int_signed_ordering() {
        let neg = Int::<4>::from_i64(-5);
        let zero = Int::<4>::ZERO;
        let pos = Int::<4>::from_i64(5);
        // Negative < zero < positive even though -5's limbs are larger
        // unsigned than 5's.
        assert!(neg < zero);
        assert!(zero < pos);
        assert!(neg < pos);
        // Two negatives compare by magnitude with sign accounted for.
        assert!(Int::<4>::from_i64(-10) < Int::<4>::from_i64(-1));
        assert_eq!(neg.max(pos), pos);
        assert_eq!(neg.min(pos), neg);
        assert_eq!(pos.cmp(&pos), Ordering::Equal);
    }
}

/// Feasibility proof for the unified narrow-tier divide path: the same
/// `widen_mul` → `div_wide_pow10_with` pipeline the wide tiers already
/// run must produce the correct `(a · b) / 10^scale` at the narrow
/// limb widths `N = 1` (`Int64`) and `N = 2` (`Int128`) that the
/// D18/D38-unify steps will rewire onto. This locks in the
/// `widen_mul::<wider>` then `div_wide_pow10_with::<wider, U128_LIMBS>`
/// composition before any decimal type is rewired; it is additive and
/// asserts only — no behaviour is changed here.
#[cfg(all(test, feature = "wide"))]
mod unified_mg_feasibility {
    use super::Int;
    use crate::algos::mg_divide::div_wide_pow10_with;
    use crate::int::types::traits::BigInt;
    use crate::support::rounding::RoundingMode;

    /// `(a · b) / 10^scale` through the unified pipeline, computed as
    /// `Int<N>::widen_mul::<Int<M>>` (full product into the wider type)
    /// then `div_wide_pow10_with::<Int<M>, LW>`. `LW` is the wider type's
    /// `U128_LIMBS` (`(M + 1) / 2`), supplied explicitly by each caller
    /// because a `<Int<M> as BigInt>::U128_LIMBS` expression cannot
    /// appear in a const-generic argument over a generic `M`. Returns the
    /// scaled wider-width quotient.
    fn scaled<const N: usize, const M: usize, const LW: usize>(
        a: Int<N>,
        b: Int<N>,
        scale: u32,
    ) -> Int<M>
    where
        Int<M>: BigInt,
    {
        let prod: Int<M> = a.widen_mul::<Int<M>>(b);
        div_wide_pow10_with::<Int<M>, LW>(prod, scale, RoundingMode::HalfToEven)
    }

    /// N = 2 → widen to N = 4, scale 5 (the plan's anchor case).
    #[test]
    fn n2_widen4_scale5() {
        let a = Int::<2>::from_i64(123456789);
        let b = Int::<2>::from_i64(987654321);
        let got = scaled::<2, 4, 2>(a, b, 5);
        assert_eq!(got, Int::<4>::from_i64(1219326311126));
    }

    /// N = 1 → widen to N = 2, scale 3 (the plan's anchor case).
    #[test]
    fn n1_widen2_scale3() {
        let a = Int::<1>::from_i64(123456);
        let b = Int::<1>::from_i64(654321);
        let got = scaled::<1, 2, 1>(a, b, 3);
        assert_eq!(got, Int::<2>::from_i64(80779853));
    }

    /// Round-trip: `(x · 10^k) / 10^k == x` at both narrow widths. The
    /// widen_mul forms `x · 10^k` exactly in the wider type and the MG
    /// divide undoes it with no residue.
    #[test]
    fn round_trip_mul_then_div() {
        // N = 1 → 2: x = 4242, k = 4.
        let x1 = 4242i64;
        let ten_pow_4 = Int::<1>::from_i64(10_000);
        let rt1 = scaled::<1, 2, 1>(Int::<1>::from_i64(x1), ten_pow_4, 4);
        assert_eq!(rt1, Int::<2>::from_i64(x1));

        // N = 2 → 4: x = 9_876_543_210, k = 7.
        let x2 = 9_876_543_210i64;
        let ten_pow_7 = Int::<2>::from_i64(10_000_000);
        let rt2 = scaled::<2, 4, 2>(Int::<2>::from_i64(x2), ten_pow_7, 7);
        assert_eq!(rt2, Int::<4>::from_i64(x2));
    }

    /// Scale-0 identity: callers short-circuit `scale == 0` as a no-op
    /// (`div_wide_pow10_with` is only ever invoked for `1..=38`), so the
    /// scaled value at scale 0 is exactly the full widen_mul product.
    /// This locks that contract for the narrow widths.
    #[test]
    fn scale_zero_is_widen_mul_identity() {
        // N = 1 → 2.
        let a1 = Int::<1>::from_i64(123456);
        let b1 = Int::<1>::from_i64(654321);
        let prod1: Int<2> = a1.widen_mul::<Int<2>>(b1);
        assert_eq!(prod1, Int::<2>::from_i64(123456 * 654321));

        // N = 2 → 4.
        let a2 = Int::<2>::from_i64(123456789);
        let b2 = Int::<2>::from_i64(987654321);
        let prod2: Int<4> = a2.widen_mul::<Int<4>>(b2);
        assert_eq!(prod2, Int::<4>::from_i64(123456789i64 * 987654321i64));
    }

    /// Max-operand case at each narrow width: the widest product the
    /// source width can hold must widen and scale without truncation.
    #[test]
    fn max_operand_each_width() {
        // N = 1: u64::MAX-ish operands. Use the largest i64 magnitudes
        // whose product the widen_mul (into N = 2) holds exactly, then
        // scale by 10^9 and check against the u128 reference.
        let a1 = Int::<1>::from_i64(i64::MAX);
        let b1 = Int::<1>::from_i64(i64::MAX);
        let got1 = scaled::<1, 2, 1>(a1, b1, 9);
        let exact1 = (i64::MAX as i128) * (i64::MAX as i128);
        let pow9 = 1_000_000_000i128;
        let q1 = exact1 / pow9;
        let r1 = exact1 % pow9;
        // HalfToEven bump when the remainder is past the halfway point.
        let half = pow9 / 2;
        let expect1 = if r1 > half || (r1 == half && (q1 & 1) == 1) {
            q1 + 1
        } else {
            q1
        };
        assert_eq!(got1, Int::<2>::from_i128(expect1));

        // N = 2: the most-positive signed operand (2^127 - 1) squared,
        // widened into N = 4, then scaled by 10^20. Compare against the
        // exact reference reconstructed from the full 256-bit product.
        let big = Int::<2>::MAX; // 2^127 - 1
        let got2 = scaled::<2, 4, 2>(big, big, 20);
        let prod2: Int<4> = big.widen_mul::<Int<4>>(big);
        // Exact (2^127 - 1)^2 / 10^20, HalfToEven, via the wider type's
        // own div_rem against 10^20 built in Int<4>.
        let pow20 = Int::<4>::TEN.pow(20);
        let (q2, r2) = prod2.div_rem(pow20);
        let half20 = pow20.div_rem(Int::<4>::from_i64(2)).0;
        let expect2 = match r2.cmp(&half20) {
            core::cmp::Ordering::Greater => q2.wrapping_add(Int::<4>::ONE),
            core::cmp::Ordering::Equal => {
                if (q2.as_limbs()[0] & 1) == 1 {
                    q2.wrapping_add(Int::<4>::ONE)
                } else {
                    q2
                }
            }
            core::cmp::Ordering::Less => q2,
        };
        assert_eq!(got2, expect2);
    }
}
