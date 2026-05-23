//! Legacy `limbs_*_u64` name shim over the dissolved limb buckets.
//!
//! The integer layer's raw `&[u64]` limb arithmetic used to live here as
//! one ~3700-line module. Phase 4 (#79) fanned it out into its rightful
//! homes:
//!
//! - **generic limb arithmetic** → [`crate::int::algos::limbs`]
//!   (`is_zero`/`cmp`/`add_assign`/`mul_schoolbook`/`mul_karatsuba`/…);
//! - **division engines** → [`crate::int::algos::div`] (`div_rem`,
//!   `div_knuth`, `div_burnikel_ziegler_with_knuth`, the `Mg2By1` /
//!   `Mg3By2` reciprocal engines);
//! - **the divisor-shape + mul-algorithm pickers** →
//!   [`crate::int::policy::div`] / [`crate::int::policy::mul`];
//! - **integer square root** → [`crate::int::algos::roots`]
//!   (`isqrt_newton`);
//! - **radix-to-string formatting** → [`crate::support::int_fmt`]
//!   (`fmt_into`).
//!
//! This module survives only as a thin re-export so the historic
//! `limbs_*_u64` names (and the `crate::wide_int::limbs_*` paths in
//! `lib.rs` / the benches) keep resolving without churning their ~80 use
//! sites. New code should reach the kernels by their bare names in the
//! buckets above.

#[allow(unused_imports)]
pub(crate) use crate::int::algos::div::{
    div_burnikel_ziegler_with_knuth as limbs_divmod_bz_u64, div_knuth as limbs_divmod_knuth_u64,
    div_rem as limbs_divmod_u64,
};
#[allow(unused_imports)]
pub(crate) use crate::int::algos::limbs::{
    add_assign as limbs_add_assign_u64, add_assign_fixed as limbs_add_assign_u64_fixed,
    bit_len as limbs_bit_len_u64, bit_len_fixed as limbs_bit_len_u64_fixed, cmp as limbs_cmp_u64,
    cmp_cross as limbs_cmp_u64_cross, cmp_fixed as limbs_cmp_u64_fixed, eq as limbs_eq_u64,
    is_zero as limbs_is_zero_u64, is_zero_fixed as limbs_is_zero_u64_fixed,
    mul_low_fixed as limbs_mul_low_u64_fixed, mul_schoolbook as limbs_mul_u64,
    mul_schoolbook_fixed as limbs_mul_u64_fixed, mul_schoolbook_into as limbs_mul_u64_into,
    scmp as scmp_u64, shl as limbs_shl_u64, shl_fixed as limbs_shl_u64_fixed, shr as limbs_shr_u64,
    shr_fixed as limbs_shr_u64_fixed, sqr_low_fixed as limbs_sqr_low_u64_fixed,
    sub_assign as limbs_sub_assign_u64, sub_assign_fixed as limbs_sub_assign_u64_fixed,
};
#[allow(unused_imports)]
pub(crate) use crate::int::algos::roots::isqrt_newton as limbs_isqrt_u64;
#[allow(unused_imports)]
pub(crate) use crate::int::policy::div::div_rem_dispatch as limbs_divmod_dispatch_u64;
#[allow(unused_imports)]
pub(crate) use crate::int::policy::mul::mul_fast as limbs_mul_fast_u64;
#[allow(unused_imports)]
pub(crate) use crate::support::int_fmt::fmt_into as limbs_fmt_into_u64;

// The bench-only forced-threshold Karatsuba entry the `__bench_internals`
// shim in `lib.rs` reaches via `crate::wide_int`.
#[cfg(feature = "bench-alt")]
#[allow(unused_imports)]
pub(crate) use crate::int::algos::limbs::mul_karatsuba_forced as limbs_mul_karatsuba_u64_forced;

#[cfg(test)]
mod hint_tests {
    use crate::int::types::{Int, Uint};

    #[test]
    fn signed_add_sub_neg() {
        let a = Int::<4>::from_i128(5);
        let b = Int::<4>::from_i128(3);
        assert_eq!(a.wrapping_add(b), Int::<4>::from_i128(8));
        assert_eq!(a.wrapping_sub(b), Int::<4>::from_i128(2));
        assert_eq!(b.wrapping_sub(a), Int::<4>::from_i128(-2));
        assert_eq!(a.negate(), Int::<4>::from_i128(-5));
        assert_eq!(Int::<4>::ZERO.negate(), Int::<4>::ZERO);
    }

    #[test]
    fn signed_mul_div_rem() {
        let six = Int::<8>::from_i128(6);
        let two = Int::<8>::from_i128(2);
        let three = Int::<8>::from_i128(3);
        assert_eq!(six.wrapping_mul(three), Int::<8>::from_i128(18));
        assert_eq!(six.wrapping_div(two), three);
        assert_eq!(
            Int::<8>::from_i128(7).wrapping_rem(three),
            Int::<8>::from_i128(1)
        );
        assert_eq!(
            Int::<8>::from_i128(-7).wrapping_rem(three),
            Int::<8>::from_i128(-1)
        );
        assert_eq!(six.negate().wrapping_mul(three), Int::<8>::from_i128(-18));
    }

    #[test]
    fn checked_overflow() {
        assert_eq!(Int::<4>::MAX.checked_add(Int::<4>::ONE), None);
        assert_eq!(Int::<4>::MIN.checked_sub(Int::<4>::ONE), None);
        assert_eq!(Int::<4>::MIN.checked_neg(), None);
        assert_eq!(
            Int::<4>::from_i128(2).checked_add(Int::<4>::from_i128(3)),
            Some(Int::<4>::from_i128(5))
        );
    }

    #[test]
    fn from_str_and_pow() {
        let ten = Int::<16>::from_str_radix("10", 10).unwrap();
        assert_eq!(ten, Int::<16>::from_i128(10));
        assert_eq!(ten.pow(3), Int::<16>::from_i128(1000));
        let big = Int::<16>::from_str_radix("10", 10).unwrap().pow(40);
        let from_str =
            Int::<16>::from_str_radix("10000000000000000000000000000000000000000", 10).unwrap();
        assert_eq!(big, from_str);
        assert_eq!(
            Int::<4>::from_str_radix("-42", 10).unwrap(),
            Int::<4>::from_i128(-42)
        );
    }

    #[test]
    fn ordering_and_resize() {
        assert!(Int::<4>::from_i128(-1) < Int::<4>::ZERO);
        assert!(Int::<4>::MIN < Int::<4>::MAX);
        let v = Int::<4>::from_i128(-123_456_789);
        let wide: Int<16> = v.resize();
        let back: Int<4> = wide.resize();
        assert_eq!(back, v);
        assert_eq!(wide, Int::<16>::from_i128(-123_456_789));
    }

    #[test]
    fn isqrt_and_f64() {
        assert_eq!(Int::<8>::from_i128(144).isqrt(), Int::<8>::from_i128(12));
        assert_eq!(Int::<4>::from_i128(1_000_000).as_f64(), 1_000_000.0);
        assert_eq!(Int::<4>::from_f64(-2_500.0), Int::<4>::from_i128(-2500));
    }

    /// `Uint<4>` (the unsigned macro emission) supports the same
    /// bit/sign-manipulation surface as the signed sibling.
    #[test]
    fn uint256_is_zero_and_bit_helpers() {
        let zero = Uint::<4>::ZERO;
        let one = Uint::<4>::from_str_radix("1", 10).unwrap();
        let two = Uint::<4>::from_str_radix("2", 10).unwrap();
        assert!(zero.is_zero());
        assert!(!one.is_zero());
        assert!(one.is_power_of_two());
        assert!(two.is_power_of_two());
        let three = Uint::<4>::from_str_radix("3", 10).unwrap();
        assert!(!three.is_power_of_two());
        assert_eq!(zero.next_power_of_two(), one);
        assert_eq!(one.next_power_of_two(), one);
        let four = Uint::<4>::from_str_radix("4", 10).unwrap();
        assert_eq!(three.next_power_of_two(), four);
        assert_eq!(zero.count_ones(), 0);
        assert_eq!(one.count_ones(), 1);
        assert_eq!(zero.leading_zeros(), Uint::<4>::BITS);
        assert_eq!(one.leading_zeros(), Uint::<4>::BITS - 1);
    }

    #[test]
    fn uint256_parse_arithmetic_and_pow() {
        assert!(Uint::<4>::from_str_radix("10", 2).is_err());
        assert!(Uint::<4>::from_str_radix("1a", 10).is_err());
        let two = Uint::<4>::from_str_radix("2", 10).unwrap();
        let three = Uint::<4>::from_str_radix("3", 10).unwrap();
        let six = Uint::<4>::from_str_radix("6", 10).unwrap();
        let seven = Uint::<4>::from_str_radix("7", 10).unwrap();
        assert_eq!(three - two, Uint::<4>::from_str_radix("1", 10).unwrap());
        assert_eq!(six / two, three);
        assert_eq!(seven % three, Uint::<4>::from_str_radix("1", 10).unwrap());
        let five = Uint::<4>::from_str_radix("5", 10).unwrap();
        let four = Uint::<4>::from_str_radix("4", 10).unwrap();
        let one = Uint::<4>::from_str_radix("1", 10).unwrap();
        assert_eq!(five & four, four);
        assert_eq!(five | one, five);
        assert_eq!(five ^ four, one);
        let p10 = two.pow(10);
        assert_eq!(p10, Uint::<4>::from_str_radix("1024", 10).unwrap());
        let signed = three.cast_signed();
        assert_eq!(signed, Int::<4>::from_i128(3));
    }

    /// `Int::<4>::bit` reports the two's-complement bit at any index.
    #[test]
    fn signed_bit_and_trailing_zeros() {
        let v = Int::<4>::from_i128(0b1100);
        assert!(v.bit(2));
        assert!(v.bit(3));
        assert!(!v.bit(0));
        assert!(!v.bit(1));
        assert!(!v.bit(1000));
        let n = Int::<4>::from_i128(-1);
        assert!(n.bit(1000));
        assert_eq!(Int::<4>::from_i128(8).trailing_zeros(), 3);
        assert_eq!(Int::<4>::ZERO.trailing_zeros(), Int::<4>::BITS);
    }

    /// `Int::<4>::as_u128` returns the low 128 magnitude bits.
    #[test]
    fn as_u128_returns_low_magnitude_bits() {
        let src = Int::<4>::from_i128(123_456_789);
        let dst: u128 = src.as_u128();
        assert_eq!(dst, 123_456_789);
        let dst: u128 = Int::<4>::ZERO.as_u128();
        assert_eq!(dst, 0);
    }
}
