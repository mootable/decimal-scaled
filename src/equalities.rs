//! `PartialEq` impls between `D128<SCALE>` and primitive numeric types.
//!
//! # Semantics
//!
//! - **Integers** (`i8`-`i128`, `u8`-`u128`, `isize`, `usize`): exact
//!   mathematical equality. `d == n` holds iff `d.to_bits() == n * 10^SCALE`,
//!   i.e. `d` represents the integer `n` with no fractional part. The
//!   comparison is computed without overflow by splitting `d.to_bits()` into
//!   quotient and remainder modulo `10^SCALE`; the value is equal to `n` iff
//!   the remainder is zero and the quotient equals `n`.
//!
//!   Examples:
//!   - `D128e12::from_int(5) == 5_i32` -> `true`
//!   - `D128e12::from_bits(5_500_000_000_000) == 5_i32` -> `false` (5.5 != 5)
//!   - `D128e12::from_bits(-1) == 0_u32` -> `false` (negative value)
//!
//! - **Floats** (`f32`, `f64`): equality holds iff `f` is finite and converts
//!   to and from `D128` losslessly relative to the f64 representation. NaN and
//!   ±inf always compare unequal. A `D128` value larger than `2^53` cannot
//!   match any `f64` exactly and will compare unequal except when the float's
//!   stored value happens to round-trip.
//!
//!   Note that f64 cannot represent decimals like `1.1` exactly; the nearest
//!   f64 to `1.1` is `1.1000000000000000888...`. The implementation treats
//!   that nearest f64 as equal to `D128e12::from_bits(1_100_000_000_000)`
//!   because the round-trip through `from_f64_lossy`/`to_f64_lossy` agrees.
//!   Callers who need true rational equality should convert and compare
//!   explicitly.
//!
//! Each impl provides both directions (`D128<S> == T` and `T == D128<S>`) so
//! comparisons are symmetric at the call site.

use crate::core_type::D128;

// ---------------------------------------------------------------------------
// Signed integer impls (i8 .. i64, isize). All widen losslessly to i128.
// ---------------------------------------------------------------------------

macro_rules! impl_eq_signed {
    ($t:ty) => {
        impl<const SCALE: u32> PartialEq<$t> for D128<SCALE> {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                let m = Self::multiplier();
                self.0 % m == 0 && self.0 / m == *other as i128
            }
        }

        impl<const SCALE: u32> PartialEq<D128<SCALE>> for $t {
            #[inline]
            fn eq(&self, other: &D128<SCALE>) -> bool {
                other == self
            }
        }
    };
}

impl_eq_signed!(i8);
impl_eq_signed!(i16);
impl_eq_signed!(i32);
impl_eq_signed!(i64);
impl_eq_signed!(isize);

// i128 is special-cased because the `*other as i128` cast in the macro is
// already exact, but multiplying `n * m` would overflow for large `n`. The
// quotient/remainder split avoids any multiplication.

impl<const SCALE: u32> PartialEq<i128> for D128<SCALE> {
    #[inline]
    fn eq(&self, other: &i128) -> bool {
        let m = Self::multiplier();
        self.0 % m == 0 && self.0 / m == *other
    }
}

impl<const SCALE: u32> PartialEq<D128<SCALE>> for i128 {
    #[inline]
    fn eq(&self, other: &D128<SCALE>) -> bool {
        other == self
    }
}

// ---------------------------------------------------------------------------
// Unsigned integer impls (u8 .. u64, usize). All widen losslessly to i128.
// A negative `D128` is never equal to an unsigned value.
// ---------------------------------------------------------------------------

macro_rules! impl_eq_unsigned {
    ($t:ty) => {
        impl<const SCALE: u32> PartialEq<$t> for D128<SCALE> {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                if self.0 < 0 {
                    return false;
                }
                let m = Self::multiplier();
                self.0 % m == 0 && self.0 / m == *other as i128
            }
        }

        impl<const SCALE: u32> PartialEq<D128<SCALE>> for $t {
            #[inline]
            fn eq(&self, other: &D128<SCALE>) -> bool {
                other == self
            }
        }
    };
}

impl_eq_unsigned!(u8);
impl_eq_unsigned!(u16);
impl_eq_unsigned!(u32);
impl_eq_unsigned!(u64);
impl_eq_unsigned!(usize);

// u128 covers a range that does not fit in i128. After the negativity guard
// the storage value is non-negative, so casting the quotient to u128 is safe
// and lossless.

impl<const SCALE: u32> PartialEq<u128> for D128<SCALE> {
    #[inline]
    fn eq(&self, other: &u128) -> bool {
        if self.0 < 0 {
            return false;
        }
        let m = Self::multiplier();
        if self.0 % m != 0 {
            return false;
        }
        (self.0 / m) as u128 == *other
    }
}

impl<const SCALE: u32> PartialEq<D128<SCALE>> for u128 {
    #[inline]
    fn eq(&self, other: &D128<SCALE>) -> bool {
        other == self
    }
}

// ---------------------------------------------------------------------------
// Float impls (f32, f64). Equality holds when the f64 representation of the
// float and the f64 view of the D128 agree exactly, and the value round-trips
// back to the same storage bits. NaN and ±inf are always unequal.
// ---------------------------------------------------------------------------

macro_rules! impl_eq_float {
    ($t:ty) => {
        impl<const SCALE: u32> PartialEq<$t> for D128<SCALE> {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                if !other.is_finite() {
                    return false;
                }
                let f = *other as f64;
                let from_f = D128::<SCALE>::from_f64_lossy(f);
                from_f.0 == self.0 && self.to_f64_lossy() == f
            }
        }

        impl<const SCALE: u32> PartialEq<D128<SCALE>> for $t {
            #[inline]
            fn eq(&self, other: &D128<SCALE>) -> bool {
                other == self
            }
        }
    };
}

impl_eq_float!(f32);
impl_eq_float!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_type::D128e12;

    // --- signed integers --------------------------------------------------

    #[test]
    fn eq_signed_exact_match() {
        assert!(D128e12::from_int(5) == 5_i32);
        assert!(5_i32 == D128e12::from_int(5));
        assert!(D128e12::from_int(-7) == -7_i64);
        assert!(D128e12::ZERO == 0_i8);
    }

    #[test]
    fn eq_signed_fractional_is_false() {
        let one_and_a_half = D128e12::from_bits(1_500_000_000_000);
        assert!(!(one_and_a_half == 1_i32));
        assert!(!(one_and_a_half == 2_i32));
    }

    #[test]
    fn eq_signed_one_lsb_is_false() {
        let just_above_zero = D128e12::from_bits(1);
        assert!(!(just_above_zero == 0_i32));
    }

    #[test]
    fn eq_i128_no_overflow_at_extremes() {
        let huge = i128::MAX / D128e12::multiplier();
        let d = D128e12::from_bits(huge * D128e12::multiplier());
        assert!(d == huge);
    }

    #[test]
    fn eq_i128_negative() {
        let d = D128e12::from_bits(-42_000_000_000_000);
        assert!(d == -42_i128);
        assert!(-42_i128 == d);
    }

    // --- unsigned integers ------------------------------------------------

    #[test]
    fn eq_unsigned_exact_match() {
        assert!(D128e12::from_int(5) == 5_u32);
        assert!(5_u64 == D128e12::from_int(5));
        assert!(D128e12::ZERO == 0_u8);
    }

    #[test]
    fn eq_unsigned_negative_is_false() {
        let neg = D128e12::from_int(-1);
        assert!(!(neg == 0_u32));
        assert!(!(neg == 1_u32));
    }

    #[test]
    fn eq_u128_large_value() {
        let n: u128 = 1_000_000_u128;
        let d = D128e12::from_bits((n as i128) * D128e12::multiplier());
        assert!(d == n);
    }

    #[test]
    fn eq_u128_out_of_d128_range_is_false() {
        // A u128 value larger than D128::MAX after scaling cannot match.
        let too_big: u128 = u128::MAX;
        let d = D128e12::MAX;
        assert!(!(d == too_big));
    }

    // --- floats -----------------------------------------------------------

    #[test]
    fn eq_float_exact_representable() {
        // 1.5 is exactly representable in both f64 and D128e12.
        let d = D128e12::from_bits(1_500_000_000_000);
        assert!(d == 1.5_f64);
        assert!(1.5_f64 == d);
        assert!(d == 1.5_f32);
    }

    #[test]
    fn eq_float_zero_and_one() {
        assert!(D128e12::ZERO == 0.0_f64);
        assert!(D128e12::ONE == 1.0_f64);
        assert!(D128e12::ZERO == 0.0_f32);
        assert!(D128e12::ONE == 1.0_f32);
    }

    #[test]
    fn eq_float_nan_is_false() {
        assert!(!(D128e12::ZERO == f64::NAN));
        assert!(!(D128e12::ZERO == f32::NAN));
    }

    #[test]
    fn eq_float_infinity_is_false() {
        assert!(!(D128e12::MAX == f64::INFINITY));
        assert!(!(D128e12::MIN == f64::NEG_INFINITY));
        assert!(!(D128e12::MAX == f32::INFINITY));
    }

    #[test]
    fn eq_float_negative() {
        let d = D128e12::from_bits(-2_500_000_000_000);
        assert!(d == -2.5_f64);
        assert!(-2.5_f64 == d);
    }
}
