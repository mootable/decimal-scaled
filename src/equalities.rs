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
//!   - `D128s12::from_int(5) == 5_i32` -> `true`
//!   - `D128s12::from_bits(5_500_000_000_000) == 5_i32` -> `false` (5.5 != 5)
//!   - `D128s12::from_bits(-1) == 0_u32` -> `false` (negative value)
//!
//! - **Floats** (`f32`, `f64`): equality holds iff `f` is finite and converts
//!   to and from `D128` losslessly relative to the f64 representation. NaN and
//!   ±inf always compare unequal. A `D128` value larger than `2^53` cannot
//!   match any `f64` exactly and will compare unequal except when the float's
//!   stored value happens to round-trip.
//!
//!   Note that f64 cannot represent decimals like `1.1` exactly; the nearest
//!   f64 to `1.1` is `1.1000000000000000888...`. The implementation treats
//!   that nearest f64 as equal to `D128s12::from_bits(1_100_000_000_000)`
//!   because the round-trip through `from_f64_lossy`/`to_f64_lossy` agrees.
//!   Callers who need true rational equality should convert and compare
//!   explicitly.
//!
//! Each impl provides both directions (`D128<S> == T` and `T == D128<S>`) so
//! comparisons are symmetric at the call site.

use crate::core_type::{D128, D32, D64};

// Cross-equality with primitive integer types is emitted by the
// `decl_eq_all_integers!` macro family — see
// `src/decimal_equalities_macro.rs`. The same surface is generated for
// every decimal width.
crate::macros::equalities::decl_eq_all_integers!(D128);
crate::macros::equalities::decl_eq_all_integers!(D64);
crate::macros::equalities::decl_eq_all_integers!(D32);

// Float equality requires the f64 bridge (`from_f64_lossy` /
// `to_f64_lossy`), which is only present when `std` is on and
// `strict` is off. Gate the float impls accordingly. Float impls are
// emitted for D128 only at present — the D32/D64 conversion bridge
// covers will follow in a later commit.
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D128, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D128, f64);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D64, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D64, f64);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D32, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D32, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_type::D128s12;

    // --- signed integers --------------------------------------------------

    #[test]
    fn eq_signed_exact_match() {
        assert!(D128s12::from_int(5) == 5_i32);
        assert!(5_i32 == D128s12::from_int(5));
        assert!(D128s12::from_int(-7) == -7_i64);
        assert!(D128s12::ZERO == 0_i8);
    }

    #[test]
    fn eq_signed_fractional_is_false() {
        let one_and_a_half = D128s12::from_bits(1_500_000_000_000);
        assert!(!(one_and_a_half == 1_i32));
        assert!(!(one_and_a_half == 2_i32));
    }

    #[test]
    fn eq_signed_one_lsb_is_false() {
        let just_above_zero = D128s12::from_bits(1);
        assert!(!(just_above_zero == 0_i32));
    }

    #[test]
    fn eq_i128_no_overflow_at_extremes() {
        let huge = i128::MAX / D128s12::multiplier();
        let d = D128s12::from_bits(huge * D128s12::multiplier());
        assert!(d == huge);
    }

    #[test]
    fn eq_i128_negative() {
        let d = D128s12::from_bits(-42_000_000_000_000);
        assert!(d == -42_i128);
        assert!(-42_i128 == d);
    }

    // --- unsigned integers ------------------------------------------------

    #[test]
    fn eq_unsigned_exact_match() {
        assert!(D128s12::from_int(5) == 5_u32);
        assert!(5_u64 == D128s12::from_int(5));
        assert!(D128s12::ZERO == 0_u8);
    }

    #[test]
    fn eq_unsigned_negative_is_false() {
        let neg = D128s12::from_int(-1);
        assert!(!(neg == 0_u32));
        assert!(!(neg == 1_u32));
    }

    #[test]
    fn eq_u128_large_value() {
        let n: u128 = 1_000_000_u128;
        let d = D128s12::from_bits((n as i128) * D128s12::multiplier());
        assert!(d == n);
    }

    #[test]
    fn eq_u128_out_of_d128_range_is_false() {
        // A u128 value larger than D128::MAX after scaling cannot match.
        let too_big: u128 = u128::MAX;
        let d = D128s12::MAX;
        assert!(!(d == too_big));
    }

    // --- floats -----------------------------------------------------------

    #[cfg(feature = "std")]
    #[test]
    fn eq_float_exact_representable() {
        // 1.5 is exactly representable in both f64 and D128s12.
        let d = D128s12::from_bits(1_500_000_000_000);
        assert!(d == 1.5_f64);
        assert!(1.5_f64 == d);
        assert!(d == 1.5_f32);
    }

    #[cfg(feature = "std")]
    #[test]
    fn eq_float_zero_and_one() {
        assert!(D128s12::ZERO == 0.0_f64);
        assert!(D128s12::ONE == 1.0_f64);
        assert!(D128s12::ZERO == 0.0_f32);
        assert!(D128s12::ONE == 1.0_f32);
    }

    #[cfg(feature = "std")]
    #[test]
    fn eq_float_nan_is_false() {
        assert!(!(D128s12::ZERO == f64::NAN));
        assert!(!(D128s12::ZERO == f32::NAN));
    }

    #[cfg(feature = "std")]
    #[test]
    fn eq_float_infinity_is_false() {
        assert!(!(D128s12::MAX == f64::INFINITY));
        assert!(!(D128s12::MIN == f64::NEG_INFINITY));
        assert!(!(D128s12::MAX == f32::INFINITY));
    }

    #[cfg(feature = "std")]
    #[test]
    fn eq_float_negative() {
        let d = D128s12::from_bits(-2_500_000_000_000);
        assert!(d == -2.5_f64);
        assert!(-2.5_f64 == d);
    }

    // --- D32 / D64 cross-equality (uses the macro just like D128) --------

    #[test]
    fn eq_d32_with_integer() {
        use crate::core_type::D32s2;
        let v = D32s2::from_bits(150); // 1.50
        assert!(!(v == 1_i32));
        assert!(!(v == 2_i32));
        let int = D32s2::from_bits(500); // 5.00
        assert!(int == 5_i32);
        assert!(5_i32 == int);
        assert!(int == 5_u8);
    }

    #[test]
    fn eq_d64_with_integer() {
        use crate::core_type::D64s9;
        let v = D64s9::from_bits(7_000_000_000); // 7.0
        assert!(v == 7_i64);
        assert!(v == 7_u64);
        let neg = D64s9::from_bits(-7_000_000_000);
        assert!(neg == -7_i32);
        assert!(!(neg == 7_u32));
    }
}
