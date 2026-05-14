//! Bitwise operations on `D128`'s underlying `i128` storage.
//!
//! Every operator and method here delegates directly to the equivalent
//! `i128` intrinsic on the raw storage field. They operate on the
//! **raw storage bits**, not the logical decimal value.
//!
//! # Storage-not-value semantic
//!
//! `D128<SCALE>` stores its value as `raw * 10^(-SCALE)`, so a logical
//! value of `1.0` at `SCALE = 12` has raw storage `10^12`, not `1`.
//! Bitwise operations see that raw integer, not the logical decimal.
//!
//! ```ignore
//! use decimal_scaled::D128e12;
//! // D128e12::ONE.to_bits() == 1_000_000_000_000 (= 10^12), NOT 1.
//! // count_ones() returns the popcount of 10^12, which is 21.
//! assert_eq!(D128e12::ONE.count_ones(), 21);
//! ```
//!
//! For predictable bit-pattern test data, construct values with
//! [`D128::from_bits`], which sets the raw `i128` directly.
//!
//! # Operator semantics
//!
//! - `Shr` is **arithmetic** (sign-extending), matching `i128`'s default.
//!   Negative values remain negative after a right shift.
//! - [`D128::unsigned_shr`] is the **logical** (zero-fill) right shift:
//!   the storage is reinterpreted as `u128`, shifted, then cast back.
//! - `Not` (`!self`) flips every bit of the underlying `i128`.
//! - `Shl` and `Shr` panic in debug builds when the shift amount is >= 128
//!   (standard Rust integer-shift overflow contract).
//!
//! # `no_std` compatibility
//!
//! All items in this module are pure `i128` or `u128` operations and
//! require neither `std` nor `alloc`. They compile under
//! `--no-default-features`.

use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

use crate::core_type::D128;

// -- BitAnd -----------------------------------------------------------

impl<const SCALE: u32> BitAnd for D128<SCALE> {
    type Output = Self;

    /// Bitwise AND of the underlying `i128` storage values.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// let a = D128e12::from_bits(0b1100);
    /// let b = D128e12::from_bits(0b1010);
    /// assert_eq!(a & b, D128e12::from_bits(0b1000));
    /// ```
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl<const SCALE: u32> BitAndAssign for D128<SCALE> {
    /// In-place bitwise AND of the underlying `i128` storage.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

// -- BitOr ------------------------------------------------------------

impl<const SCALE: u32> BitOr for D128<SCALE> {
    type Output = Self;

    /// Bitwise OR of the underlying `i128` storage values.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// let a = D128e12::from_bits(0b1100);
    /// let b = D128e12::from_bits(0b1010);
    /// assert_eq!(a | b, D128e12::from_bits(0b1110));
    /// ```
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl<const SCALE: u32> BitOrAssign for D128<SCALE> {
    /// In-place bitwise OR of the underlying `i128` storage.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// -- BitXor -----------------------------------------------------------

impl<const SCALE: u32> BitXor for D128<SCALE> {
    type Output = Self;

    /// Bitwise XOR of the underlying `i128` storage values.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// let a = D128e12::from_bits(0b1100);
    /// let b = D128e12::from_bits(0b1010);
    /// assert_eq!(a ^ b, D128e12::from_bits(0b0110));
    /// ```
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl<const SCALE: u32> BitXorAssign for D128<SCALE> {
    /// In-place bitwise XOR of the underlying `i128` storage.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

// -- Shl --------------------------------------------------------------

impl<const SCALE: u32> Shl<u32> for D128<SCALE> {
    type Output = Self;

    /// Left-shift the underlying `i128` storage by `n` bits.
    ///
    /// Operates on raw bits, not the logical decimal value.
    /// Delegates to `i128 << n`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Panics
    ///
    /// Panics in debug builds when `n >= 128`; wraps modulo 128 in
    /// release builds (standard Rust integer-shift contract).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// assert_eq!(D128e12::from_bits(1) << 3u32, D128e12::from_bits(8));
    /// ```
    #[inline]
    fn shl(self, n: u32) -> Self {
        Self(self.0 << n)
    }
}

impl<const SCALE: u32> ShlAssign<u32> for D128<SCALE> {
    /// In-place left-shift of the underlying `i128` storage by `n` bits.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn shl_assign(&mut self, n: u32) {
        self.0 <<= n;
    }
}

// -- Shr --------------------------------------------------------------

impl<const SCALE: u32> Shr<u32> for D128<SCALE> {
    type Output = Self;

    /// Arithmetic (sign-extending) right-shift of the underlying `i128`
    /// storage by `n` bits.
    ///
    /// Negative values remain negative: the vacated high bits are filled
    /// with the sign bit (ones for negative, zero for non-negative).
    /// Use [`D128::unsigned_shr`] for a logical (zero-fill) right shift.
    ///
    /// Operates on raw bits, not the logical decimal value.
    /// Delegates to `i128 >> n`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Panics
    ///
    /// Panics in debug builds when `n >= 128`; wraps modulo 128 in
    /// release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// // Arithmetic shift: -8 >> 1 == -4.
    /// assert_eq!(D128e12::from_bits(-8) >> 1u32, D128e12::from_bits(-4));
    /// ```
    #[inline]
    fn shr(self, n: u32) -> Self {
        Self(self.0 >> n)
    }
}

impl<const SCALE: u32> ShrAssign<u32> for D128<SCALE> {
    /// In-place arithmetic (sign-extending) right-shift of the underlying
    /// `i128` storage by `n` bits.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    #[inline]
    fn shr_assign(&mut self, n: u32) {
        self.0 >>= n;
    }
}

// -- Not --------------------------------------------------------------

impl<const SCALE: u32> Not for D128<SCALE> {
    type Output = Self;

    /// Bitwise complement of the underlying `i128` storage (flip every
    /// bit).
    ///
    /// `!D128::ZERO` produces `D128::from_bits(-1)` (all-ones in two's
    /// complement).
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// assert_eq!(!D128e12::ZERO, D128e12::from_bits(-1));
    /// assert_eq!(!D128e12::from_bits(-1), D128e12::ZERO);
    /// ```
    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

// -- Methods ----------------------------------------------------------

impl<const SCALE: u32> D128<SCALE> {
    /// Logical (zero-fill) right shift of the underlying `i128` storage
    /// by `n` bits.
    ///
    /// Unlike the [`Shr`] operator, which is arithmetic (sign-extending),
    /// this method reinterprets storage as `u128` for the shift, so the
    /// vacated high bits are always filled with zeros regardless of sign.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Panics
    ///
    /// Panics in debug builds when `n >= 128`; wraps modulo 128 in
    /// release builds.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D128e12;
    /// // -1 raw is all-ones. Arithmetic shr keeps it all-ones;
    /// // unsigned_shr clears the top bit, giving i128::MAX.
    /// let neg_one = D128e12::from_bits(-1);
    /// assert_eq!(neg_one >> 1u32, neg_one);                          // sign-extending
    /// assert_eq!(neg_one.unsigned_shr(1), D128e12::from_bits(i128::MAX)); // zero-fill
    /// ```
    #[inline]
    pub const fn unsigned_shr(self, n: u32) -> Self {
        // Reinterpret storage as u128, shift, then cast back. The
        // round-trip is bit-exact under two's complement.
        Self(((self.0 as u128) >> n) as i128)
    }

    /// Rotate the underlying `i128` storage left by `n` bits. Bits
    /// shifted off the high end wrap into the low end.
    ///
    /// Operates on raw bits, not the logical decimal value.
    /// Delegates to [`i128::rotate_left`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// // 0b111 rotated left by 1 = 0b1110.
    /// assert_eq!(D128e12::from_bits(0b111).rotate_left(1), D128e12::from_bits(0b1110));
    /// ```
    #[inline]
    pub const fn rotate_left(self, n: u32) -> Self {
        Self(self.0.rotate_left(n))
    }

    /// Rotate the underlying `i128` storage right by `n` bits. Bits
    /// shifted off the low end wrap into the high end.
    ///
    /// Operates on raw bits, not the logical decimal value.
    /// Delegates to [`i128::rotate_right`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// // 1 rotated right by 1 wraps the low bit to the top: i128::MIN.
    /// assert_eq!(D128e12::from_bits(1).rotate_right(1), D128e12::from_bits(i128::MIN));
    /// ```
    #[inline]
    pub const fn rotate_right(self, n: u32) -> Self {
        Self(self.0.rotate_right(n))
    }

    /// Number of leading zero bits in the underlying `i128` storage.
    ///
    /// Returns `128` for storage value `0`, `127` for `1`, and `0` for
    /// `from_bits(-1)` (all-ones). Delegates to [`i128::leading_zeros`].
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// assert_eq!(D128e12::from_bits(1).leading_zeros(), 127);
    /// assert_eq!(D128e12::ZERO.leading_zeros(), 128);
    /// assert_eq!(D128e12::from_bits(-1).leading_zeros(), 0);
    /// ```
    #[inline]
    pub const fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    /// Number of trailing zero bits in the underlying `i128` storage.
    ///
    /// Returns `128` for storage value `0` and `3` for `from_bits(8)`.
    /// Delegates to [`i128::trailing_zeros`].
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// assert_eq!(D128e12::from_bits(8).trailing_zeros(), 3);
    /// assert_eq!(D128e12::ZERO.trailing_zeros(), 128);
    /// ```
    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }

    /// Population count: number of `1` bits set in the underlying `i128`
    /// storage.
    ///
    /// Note the storage-not-value semantic: `D128e12::ONE.count_ones()`
    /// returns the popcount of `10^12` (= 21), not `1`. Use
    /// [`D128::from_bits`] when you need a predictable bit pattern.
    ///
    /// Delegates to [`i128::count_ones`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// assert_eq!(D128e12::from_bits(0b101).count_ones(), 2);
    /// ```
    #[inline]
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Number of `0` bits in the underlying `i128` storage. Always equal
    /// to `128 - self.count_ones()`.
    ///
    /// Delegates to [`i128::count_zeros`].
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// // 0b101 has 2 ones and 126 zeros in 128-bit storage.
    /// assert_eq!(D128e12::from_bits(0b101).count_zeros(), 126);
    /// ```
    #[inline]
    pub const fn count_zeros(self) -> u32 {
        self.0.count_zeros()
    }

    /// Returns `true` if the underlying `i128` storage is a power of two
    /// (exactly one bit set and the value is positive).
    ///
    /// Implemented by reinterpreting the storage as `u128` and delegating
    /// to [`u128::is_power_of_two`]. Negative `i128` values always return
    /// `false` because the sign bit being set means more than one bit is
    /// set in the `u128` view.
    ///
    /// Note the storage-not-value semantic: `D128e12::ONE.is_power_of_two()`
    /// returns `false` because storage is `10^12 = 2^12 * 5^12`, not a
    /// single-bit value.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// assert!(D128e12::from_bits(8).is_power_of_two());
    /// assert!(!D128e12::from_bits(7).is_power_of_two());
    /// assert!(!D128e12::from_bits(-1).is_power_of_two());
    /// ```
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
        // Reinterpret as u128 for the canonical "exactly one bit set"
        // check. Negative i128 has the sign bit set, so as u128 the
        // popcount may be more than one; the check correctly returns false.
        (self.0 as u128).is_power_of_two()
    }

    /// Smallest power of two greater than or equal to the underlying
    /// `i128` storage, treating the storage as `u128`.
    ///
    /// Delegates to [`u128::next_power_of_two`] over the unsigned
    /// reinterpretation, then casts the result back to `i128`. If the
    /// next power of two exceeds `i128::MAX` the raw bit pattern wraps
    /// into the negative `i128` range.
    ///
    /// Operates on raw bits, not the logical decimal value.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Panics
    ///
    /// Panics in debug builds when the next power of two overflows
    /// `u128::MAX`, matching [`u128::next_power_of_two`] semantics.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    /// assert_eq!(D128e12::from_bits(7).next_power_of_two(), D128e12::from_bits(8));
    /// assert_eq!(D128e12::from_bits(8).next_power_of_two(), D128e12::from_bits(8));
    /// ```
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        Self((self.0 as u128).next_power_of_two() as i128)
    }
}

// -- Tests ------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_type::D128e12;

    // --- BitAnd / BitOr / BitXor ------------------------------------

    #[test]
    fn bitand_clears_bits() {
        // raw-bit boundary; from_bits not ONE
        let a = D128e12::from_bits(0xF0);
        let b = D128e12::from_bits(0x0F);
        assert_eq!(a & b, D128e12::from_bits(0x00));
    }

    #[test]
    fn bitand_assign_in_place() {
        let mut a = D128e12::from_bits(0xFF);
        a &= D128e12::from_bits(0x0F);
        assert_eq!(a, D128e12::from_bits(0x0F));
    }

    #[test]
    fn bitor_sets_bits() {
        // raw-bit boundary; from_bits not ONE
        let zero = D128e12::ZERO;
        let one_lsb = D128e12::from_bits(1);
        assert_eq!(zero | one_lsb, one_lsb);
    }

    #[test]
    fn bitor_assign_in_place() {
        let mut a = D128e12::from_bits(0xF0);
        a |= D128e12::from_bits(0x0F);
        assert_eq!(a, D128e12::from_bits(0xFF));
    }

    #[test]
    fn bitxor_toggles_bits() {
        let a = D128e12::from_bits(0b1100);
        let b = D128e12::from_bits(0b1010);
        assert_eq!(a ^ b, D128e12::from_bits(0b0110));
    }

    #[test]
    fn bitxor_assign_in_place() {
        let mut a = D128e12::from_bits(0xFF);
        a ^= D128e12::from_bits(0x0F);
        assert_eq!(a, D128e12::from_bits(0xF0));
    }

    #[test]
    fn bitxor_self_is_zero() {
        let a = D128e12::from_bits(0xDEAD_BEEF_i128);
        assert_eq!(a ^ a, D128e12::ZERO);
    }

    // --- Shl / Shr ---------------------------------------------------

    #[test]
    fn shl_doubles_lsb() {
        // raw-bit boundary; from_bits(1) not ONE
        assert_eq!(D128e12::from_bits(1) << 1u32, D128e12::from_bits(2));
    }

    #[test]
    fn shr_halves_lsb() {
        // raw-bit boundary; from_bits not ONE
        assert_eq!(D128e12::from_bits(2) >> 1u32, D128e12::from_bits(1));
    }

    #[test]
    fn shr_is_sign_extending() {
        // -1 raw is all-ones; arithmetic shr preserves all-ones.
        assert_eq!(D128e12::from_bits(-1) >> 1u32, D128e12::from_bits(-1));
    }

    #[test]
    fn shr_negative_stays_negative() {
        // -8 raw >> 1 = -4 raw under arithmetic shift.
        assert_eq!(D128e12::from_bits(-8) >> 1u32, D128e12::from_bits(-4));
    }

    #[test]
    fn shl_assign_in_place() {
        let mut a = D128e12::from_bits(1);
        a <<= 4u32;
        assert_eq!(a, D128e12::from_bits(16));
    }

    #[test]
    fn shr_assign_in_place() {
        let mut a = D128e12::from_bits(16);
        a >>= 2u32;
        assert_eq!(a, D128e12::from_bits(4));
    }

    // --- Not ---------------------------------------------------------

    #[test]
    fn not_zero_is_neg_one() {
        // raw-bit boundary; from_bits(-1) not -ONE
        assert_eq!(!D128e12::ZERO, D128e12::from_bits(-1));
    }

    #[test]
    fn not_neg_one_is_zero() {
        assert_eq!(!D128e12::from_bits(-1), D128e12::ZERO);
    }

    #[test]
    fn not_is_self_inverse() {
        let a = D128e12::from_bits(0xCAFE);
        assert_eq!(!!a, a);
    }

    // --- unsigned_shr ------------------------------------------------

    #[test]
    fn unsigned_shr_zero_fills_negative() {
        // -1 raw is all-ones; logical shr by 1 clears the top bit, so
        // the result is i128::MAX.
        assert_eq!(
            D128e12::from_bits(-1).unsigned_shr(1),
            D128e12::from_bits(i128::MAX)
        );
    }

    #[test]
    fn unsigned_shr_positive_matches_arithmetic_shr() {
        // For non-negative inputs, arithmetic and logical shifts agree.
        let a = D128e12::from_bits(0xFF);
        assert_eq!(a.unsigned_shr(4), a >> 4u32);
        assert_eq!(a.unsigned_shr(4), D128e12::from_bits(0x0F));
    }

    #[test]
    fn unsigned_shr_zero_amount_identity() {
        let a = D128e12::from_bits(-42);
        assert_eq!(a.unsigned_shr(0), a);
    }

    // --- rotate_left / rotate_right ---------------------------------

    #[test]
    fn rotate_left_low_bits() {
        // 0b111 rotate_left 1 = 0b1110 = 14.
        assert_eq!(
            D128e12::from_bits(0b111).rotate_left(1),
            D128e12::from_bits(0b1110)
        );
    }

    #[test]
    fn rotate_right_low_bit_wraps_to_top() {
        // 1 rotate_right 1 = top bit set = i128::MIN raw.
        assert_eq!(
            D128e12::from_bits(1).rotate_right(1),
            D128e12::from_bits(i128::MIN)
        );
    }

    #[test]
    fn rotate_left_full_width_is_identity() {
        let a = D128e12::from_bits(0xDEAD_BEEF_i128);
        assert_eq!(a.rotate_left(128), a);
    }

    #[test]
    fn rotate_right_round_trip() {
        let a = D128e12::from_bits(0xCAFE_F00D_i128);
        assert_eq!(a.rotate_left(13).rotate_right(13), a);
    }

    // --- leading_zeros / trailing_zeros -----------------------------

    #[test]
    fn leading_zeros_lsb_is_127() {
        // raw-bit boundary; from_bits(1) not ONE
        assert_eq!(D128e12::from_bits(1).leading_zeros(), 127);
    }

    #[test]
    fn leading_zeros_zero_is_128() {
        assert_eq!(D128e12::ZERO.leading_zeros(), 128);
    }

    #[test]
    fn leading_zeros_neg_one_is_zero() {
        assert_eq!(D128e12::from_bits(-1).leading_zeros(), 0);
    }

    #[test]
    fn trailing_zeros_eight_is_three() {
        assert_eq!(D128e12::from_bits(8).trailing_zeros(), 3);
    }

    #[test]
    fn trailing_zeros_zero_is_128() {
        assert_eq!(D128e12::ZERO.trailing_zeros(), 128);
    }

    #[test]
    fn trailing_zeros_one_is_zero() {
        assert_eq!(D128e12::from_bits(1).trailing_zeros(), 0);
    }

    // --- count_ones / count_zeros -----------------------------------

    #[test]
    fn count_ones_pattern() {
        // 0b101 has two ones.
        assert_eq!(D128e12::from_bits(0b101).count_ones(), 2);
    }

    #[test]
    fn count_zeros_pattern() {
        // 0b101 has 128 - 2 = 126 zeros (in i128 storage).
        assert_eq!(D128e12::from_bits(0b101).count_zeros(), 126);
    }

    #[test]
    fn count_ones_zero_is_zero() {
        assert_eq!(D128e12::ZERO.count_ones(), 0);
    }

    #[test]
    fn count_ones_neg_one_is_128() {
        // -1 raw is all-ones.
        assert_eq!(D128e12::from_bits(-1).count_ones(), 128);
    }

    #[test]
    fn count_zeros_complement_relation() {
        // count_ones + count_zeros == 128 for every value.
        let a = D128e12::from_bits(0xDEAD_BEEF_CAFE_i128);
        assert_eq!(a.count_ones() + a.count_zeros(), 128);
    }

    // --- is_power_of_two / next_power_of_two ------------------------

    #[test]
    fn is_power_of_two_true_for_eight() {
        assert!(D128e12::from_bits(8).is_power_of_two());
    }

    #[test]
    fn is_power_of_two_false_for_seven() {
        assert!(!D128e12::from_bits(7).is_power_of_two());
    }

    #[test]
    fn is_power_of_two_false_for_zero() {
        assert!(!D128e12::ZERO.is_power_of_two());
    }

    #[test]
    fn is_power_of_two_false_for_negative() {
        // Negative i128 has the sign bit set; reinterpreted as u128 the
        // popcount is more than one, so not a power of two.
        assert!(!D128e12::from_bits(-1).is_power_of_two());
    }

    #[test]
    fn is_power_of_two_storage_not_value_semantic() {
        // D128e12::ONE has storage 10^12 = 2^12 * 5^12, not a power of
        // two. Documents the storage-not-value semantic.
        assert!(!D128e12::ONE.is_power_of_two());
    }

    #[test]
    fn next_power_of_two_seven_is_eight() {
        assert_eq!(
            D128e12::from_bits(7).next_power_of_two(),
            D128e12::from_bits(8)
        );
    }

    #[test]
    fn next_power_of_two_eight_is_eight() {
        // Already a power of two -- stays put.
        assert_eq!(
            D128e12::from_bits(8).next_power_of_two(),
            D128e12::from_bits(8)
        );
    }

    #[test]
    fn next_power_of_two_one_is_one() {
        assert_eq!(
            D128e12::from_bits(1).next_power_of_two(),
            D128e12::from_bits(1)
        );
    }

    // --- Const-generic exercise at a non-default scale --------------

    #[test]
    fn ops_work_at_scale_six() {
        type D6 = D128<6>;
        let a = D6::from_bits(0b1100);
        let b = D6::from_bits(0b1010);
        assert_eq!(a & b, D6::from_bits(0b1000));
        assert_eq!(a | b, D6::from_bits(0b1110));
        assert_eq!(a ^ b, D6::from_bits(0b0110));
        assert_eq!(D6::from_bits(1) << 3u32, D6::from_bits(8));
        assert_eq!(D6::from_bits(8) >> 3u32, D6::from_bits(1));
        assert_eq!(!D6::ZERO, D6::from_bits(-1));
        assert_eq!(D6::from_bits(8).count_ones(), 1);
        assert!(D6::from_bits(8).is_power_of_two());
    }
}
