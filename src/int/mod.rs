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

pub(crate) mod limbs;

use crate::wide_int::{
    limbs_add_assign_u64_fixed, limbs_bit_len_u64_fixed, limbs_cmp_u64_fixed,
    limbs_divmod_u64, limbs_is_zero_u64_fixed, limbs_shl_u64_fixed, limbs_shr_u64_fixed,
    limbs_sub_assign_u64_fixed,
};
use limbs::mul::limbs_mul_low_u64_fixed;
use core::cmp::Ordering;
use core::ops::{
    Add, BitAnd, BitOr, BitXor, Mul, Neg, Not, Shl, Shr, Sub,
};

/// Unsigned fixed-width integer of `N` little-endian 64-bit limbs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Uint<const N: usize> {
    limbs: [u64; N],
}

/// Signed (two's-complement) fixed-width integer of `N` little-endian
/// 64-bit limbs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Int<const N: usize> {
    limbs: [u64; N],
}

impl<const N: usize> Uint<N> {
    /// Number of 64-bit limbs.
    pub const LIMBS: usize = N;
    /// Bit width (`LIMBS * 64`).
    pub const BITS: usize = N * 64;

    /// Additive identity.
    pub const ZERO: Self = Self { limbs: [0; N] };
    /// Multiplicative identity.
    pub const ONE: Self = {
        let mut limbs = [0u64; N];
        limbs[0] = 1;
        Self { limbs }
    };
    /// Largest representable value (all limbs set).
    pub const MAX: Self = Self { limbs: [u64::MAX; N] };

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
    /// the open-coded `x * x` pattern. Currently delegates to
    /// [`Self::wrapping_mul`]; a dedicated half-product squaring kernel
    /// (≈ half the limb products) is the planned optimisation, gated by
    /// the per-width baseline benchmark before it lands.
    #[inline]
    pub fn wrapping_sqr(self) -> Self {
        self.wrapping_mul(self)
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
        if overflow { None } else { Some(Self { limbs: out }) }
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
        assert!(!rhs.is_zero(), "attempt to calculate the remainder with a divisor of zero");
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
    /// Bit width (`LIMBS * 64`).
    pub const BITS: usize = N * 64;

    /// Additive identity.
    pub const ZERO: Self = Self { limbs: [0; N] };
    /// Multiplicative identity.
    pub const ONE: Self = {
        let mut limbs = [0u64; N];
        limbs[0] = 1;
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
    pub fn is_zero(&self) -> bool {
        limbs_is_zero_u64_fixed(&self.limbs)
    }

    /// `true` when the value is strictly negative (top bit set).
    #[inline]
    pub fn is_negative(&self) -> bool {
        N > 0 && (self.limbs[N - 1] >> 63) == 1
    }

    /// `true` when the value is strictly positive (non-zero and the
    /// sign bit clear).
    #[inline]
    pub fn is_positive(&self) -> bool {
        !self.is_negative() && !self.is_zero()
    }

    /// Two's-complement wrapping negation (`!self + 1`). `MIN` negates
    /// to itself, as with the primitive signed integers.
    #[inline]
    pub fn wrapping_neg(self) -> Self {
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

    /// Wrapping multiplication (modulo `2^BITS`). The low `N` limbs of a
    /// two's-complement product are independent of the operand signs, so
    /// this is the same truncated schoolbook the unsigned type uses.
    #[inline]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        let mut out = [0u64; N];
        limbs_mul_low_u64_fixed(&self.limbs, &rhs.limbs, &mut out);
        Self { limbs: out }
    }

    /// Absolute value (wrapping: `MIN.abs() == MIN`).
    #[inline]
    pub fn abs(self) -> Self {
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
    pub fn from_i64(value: i64) -> Self {
        // Negative values fill the upper limbs with all-ones so the
        // two's-complement representation matches at every width.
        let fill = if value < 0 { u64::MAX } else { 0 };
        let mut limbs = [fill; N];
        if N > 0 {
            limbs[0] = value as u64;
        }
        Self { limbs }
    }
}

impl<const N: usize> PartialOrd for Int<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl<const N: usize> Neg for Int<N> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self.wrapping_neg()
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
    #[inline]
    pub fn resize<const M: usize>(self) -> Uint<M> {
        Uint::from_limbs(core::array::from_fn(|i| if i < N { self.limbs[i] } else { 0 }))
    }

    /// Widens to a wider `Uint<M>` (`M >= N`), zero-extending the new
    /// high limbs. Lossless.
    #[inline]
    pub fn widen<const M: usize>(self) -> Uint<M> {
        debug_assert!(M >= N, "widen requires M >= N");
        self.resize::<M>()
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
        Some(self.resize::<M>())
    }
}

impl<const N: usize> Int<N> {
    /// Resizes to `M` limbs: sign-extends when widening, drops the high
    /// limbs when narrowing. Direction-agnostic and infallible
    /// (narrowing may change the represented value).
    #[inline]
    pub fn resize<const M: usize>(self) -> Int<M> {
        let fill = if self.is_negative() { u64::MAX } else { 0 };
        Int::from_limbs(core::array::from_fn(|i| if i < N { self.limbs[i] } else { fill }))
    }

    /// Widens to a wider `Int<M>` (`M >= N`), sign-extending. Lossless.
    #[inline]
    pub fn widen<const M: usize>(self) -> Int<M> {
        debug_assert!(M >= N, "widen requires M >= N");
        self.resize::<M>()
    }

    /// Narrows to a narrower `Int<M>` (`1 <= M <= N`). Returns `None`
    /// unless every discarded high limb is a pure sign-extension of the
    /// narrowed value's top bit — i.e. the value fits `M` limbs as
    /// two's complement.
    #[inline]
    pub fn narrow<const M: usize>(self) -> Option<Int<M>> {
        debug_assert!(M >= 1 && M <= N, "narrow requires 1 <= M <= N");
        let sign_fill = if (self.limbs[M - 1] >> 63) == 1 { u64::MAX } else { 0 };
        let mut i = M;
        while i < N {
            if self.limbs[i] != sign_fill {
                return None;
            }
            i += 1;
        }
        Some(self.resize::<M>())
    }
}

// ── Named aliases ──────────────────────────────────────────────────
// Preserve the existing surface so the const-generic types can be
// introduced without renaming every call site at once. Limb counts
// match `decl_wide_int!`: bit_width / 64.

pub type Uint192 = Uint<3>;
pub type Uint256 = Uint<4>;
pub type Uint384 = Uint<6>;
pub type Uint512 = Uint<8>;
pub type Uint768 = Uint<12>;
pub type Uint1024 = Uint<16>;
pub type Uint2048 = Uint<32>;
pub type Uint4096 = Uint<64>;

pub type Int192 = Int<3>;
pub type Int256 = Int<4>;
pub type Int384 = Int<6>;
pub type Int512 = Int<8>;
pub type Int768 = Int<12>;
pub type Int1024 = Int<16>;
pub type Int2048 = Int<32>;
pub type Int4096 = Int<64>;

#[cfg(test)]
mod tests {
    use super::limbs::mul::{limbs_mul_low_u64_fixed, limbs_mul_u64_fixed};
    use super::*;

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

    #[test]
    fn uint_widen_zero_extends() {
        let a = Uint::<2>::from_limbs([7, 8]);
        let w = a.widen::<4>();
        assert_eq!(*w.as_limbs(), [7, 8, 0, 0]);
        assert_eq!(a.resize::<4>(), w);
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
    fn aliases_have_expected_widths() {
        assert_eq!(Int256::BITS, 256);
        assert_eq!(Int4096::BITS, 4096);
        assert_eq!(Uint1024::LIMBS, 16);
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
        assert_eq!(
            a.checked_mul(a),
            Some(Uint::<4>::from_limbs([0, 0, 1, 0]))
        );
        // 2^192 * 2^192 overflows 256 bits.
        let hi = Uint::<4>::from_limbs([0, 0, 0, 1]);
        assert_eq!(hi.checked_mul(hi), None);
        // MAX * 2 overflows.
        assert_eq!(Uint::<4>::MAX.checked_mul(Uint::<4>::from_limbs([2, 0, 0, 0])), None);
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
