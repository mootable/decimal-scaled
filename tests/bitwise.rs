//! Bitwise operations on `D38`'s underlying `i128` storage.
//!
//! Every operator and method here delegates directly to the equivalent
//! `i128` intrinsic on the raw storage field. They operate on the
//! **raw storage bits**, not the logical decimal value.
//!
//! # Storage-not-value semantic
//!
//! `D38<SCALE>` stores its value as `raw * 10^(-SCALE)`, so a logical
//! value of `1.0` at `SCALE = 12` has raw storage `10^12`, not `1`.
//! Bitwise operations see that raw integer, not the logical decimal.
//!
//! ```ignore
//! use decimal_scaled::D38s12;
//! // D38s12::ONE.to_bits() == 1_000_000_000_000 (= 10^12), NOT 1.
//! // count_ones() returns the popcount of 10^12, which is 21.
//! assert_eq!(D38s12::ONE.count_ones(), 21);
//! ```
//!
//! For predictable bit-pattern test data, construct values with
//! [`D38::from_bits`], which sets the raw `i128` directly.
//!
//! # Operator semantics
//!
//! - `Shr` is **arithmetic** (sign-extending), matching `i128`'s default.
//! Negative values remain negative after a right shift.
//! - [`D38::unsigned_shr`] is the **logical** (zero-fill) right shift:
//! the storage is reinterpreted as `u128`, shifted, then cast back.
//! - `Not` (`!self`) flips every bit of the underlying `i128`.
//! - `Shl` and `Shr` panic in debug builds when the shift amount is >= 128
//! (standard Rust integer-shift overflow contract).
//!
//! # `no_std` compatibility
//!
//! All items in this module are pure `i128` or `u128` operations and
//! require neither `std` nor `alloc`. They compile under
//! `--no-default-features`.

// -- Tests ------------------------------------------------------------

use decimal_scaled::{D38, D38s12};

// --- BitAnd / BitOr / BitXor ------------------------------------

#[test]
fn bitand_clears_bits() {
    // raw-bit boundary; from_bits not ONE
    let a = D38s12::from_bits(0xF0);
    let b = D38s12::from_bits(0x0F);
    assert_eq!(a & b, D38s12::from_bits(0x00));
}

#[test]
fn bitand_assign_in_place() {
    let mut a = D38s12::from_bits(0xFF);
    a &= D38s12::from_bits(0x0F);
    assert_eq!(a, D38s12::from_bits(0x0F));
}

#[test]
fn bitor_sets_bits() {
    // raw-bit boundary; from_bits not ONE
    let zero = D38s12::ZERO;
    let one_lsb = D38s12::from_bits(1);
    assert_eq!(zero | one_lsb, one_lsb);
}

#[test]
fn bitor_assign_in_place() {
    let mut a = D38s12::from_bits(0xF0);
    a |= D38s12::from_bits(0x0F);
    assert_eq!(a, D38s12::from_bits(0xFF));
}

#[test]
fn bitxor_toggles_bits() {
    let a = D38s12::from_bits(0b1100);
    let b = D38s12::from_bits(0b1010);
    assert_eq!(a ^ b, D38s12::from_bits(0b0110));
}

#[test]
fn bitxor_assign_in_place() {
    let mut a = D38s12::from_bits(0xFF);
    a ^= D38s12::from_bits(0x0F);
    assert_eq!(a, D38s12::from_bits(0xF0));
}

#[test]
fn bitxor_self_is_zero() {
    let a = D38s12::from_bits(0xDEAD_BEEF_i128);
    assert_eq!(a ^ a, D38s12::ZERO);
}

// --- Shl / Shr ---------------------------------------------------

#[test]
fn shl_doubles_lsb() {
    // raw-bit boundary; from_bits(1) not ONE
    assert_eq!(D38s12::from_bits(1) << 1u32, D38s12::from_bits(2));
}

#[test]
fn shr_halves_lsb() {
    // raw-bit boundary; from_bits not ONE
    assert_eq!(D38s12::from_bits(2) >> 1u32, D38s12::from_bits(1));
}

#[test]
fn shr_is_sign_extending() {
    // -1 raw is all-ones; arithmetic shr preserves all-ones.
    assert_eq!(D38s12::from_bits(-1) >> 1u32, D38s12::from_bits(-1));
}

#[test]
fn shr_negative_stays_negative() {
    // -8 raw >> 1 = -4 raw under arithmetic shift.
    assert_eq!(D38s12::from_bits(-8) >> 1u32, D38s12::from_bits(-4));
}

#[test]
fn shl_assign_in_place() {
    let mut a = D38s12::from_bits(1);
    a <<= 4u32;
    assert_eq!(a, D38s12::from_bits(16));
}

#[test]
fn shr_assign_in_place() {
    let mut a = D38s12::from_bits(16);
    a >>= 2u32;
    assert_eq!(a, D38s12::from_bits(4));
}

// --- Not ---------------------------------------------------------

#[test]
fn not_zero_is_neg_one() {
    // raw-bit boundary; from_bits(-1) not -ONE
    assert_eq!(!D38s12::ZERO, D38s12::from_bits(-1));
}

#[test]
fn not_neg_one_is_zero() {
    assert_eq!(!D38s12::from_bits(-1), D38s12::ZERO);
}

#[test]
fn not_is_self_inverse() {
    let a = D38s12::from_bits(0xCAFE);
    assert_eq!(!!a, a);
}

// --- unsigned_shr ------------------------------------------------

#[test]
fn unsigned_shr_zero_fills_negative() {
    // -1 raw is all-ones; logical shr by 1 clears the top bit, so
    // the result is i128::MAX.
    assert_eq!(
        D38s12::from_bits(-1).unsigned_shr(1),
        D38s12::from_bits(i128::MAX)
    );
}

#[test]
fn unsigned_shr_positive_matches_arithmetic_shr() {
    // For non-negative inputs, arithmetic and logical shifts agree.
    let a = D38s12::from_bits(0xFF);
    assert_eq!(a.unsigned_shr(4), a >> 4u32);
    assert_eq!(a.unsigned_shr(4), D38s12::from_bits(0x0F));
}

#[test]
fn unsigned_shr_zero_amount_identity() {
    let a = D38s12::from_bits(-42);
    assert_eq!(a.unsigned_shr(0), a);
}

// --- rotate_left / rotate_right ---------------------------------

#[test]
fn rotate_left_low_bits() {
    // 0b111 rotate_left 1 = 0b1110 = 14.
    assert_eq!(
        D38s12::from_bits(0b111).rotate_left(1),
        D38s12::from_bits(0b1110)
    );
}

#[test]
fn rotate_right_low_bit_wraps_to_top() {
    // 1 rotate_right 1 = top bit set = i128::MIN raw.
    assert_eq!(
        D38s12::from_bits(1).rotate_right(1),
        D38s12::from_bits(i128::MIN)
    );
}

#[test]
fn rotate_left_full_width_is_identity() {
    let a = D38s12::from_bits(0xDEAD_BEEF_i128);
    assert_eq!(a.rotate_left(128), a);
}

#[test]
fn rotate_right_round_trip() {
    let a = D38s12::from_bits(0xCAFE_F00D_i128);
    assert_eq!(a.rotate_left(13).rotate_right(13), a);
}

// --- leading_zeros / trailing_zeros -----------------------------

#[test]
fn leading_zeros_lsb_is_127() {
    // raw-bit boundary; from_bits(1) not ONE
    assert_eq!(D38s12::from_bits(1).leading_zeros(), 127);
}

#[test]
fn leading_zeros_zero_is_128() {
    assert_eq!(D38s12::ZERO.leading_zeros(), 128);
}

#[test]
fn leading_zeros_neg_one_is_zero() {
    assert_eq!(D38s12::from_bits(-1).leading_zeros(), 0);
}

#[test]
fn trailing_zeros_eight_is_three() {
    assert_eq!(D38s12::from_bits(8).trailing_zeros(), 3);
}

#[test]
fn trailing_zeros_zero_is_128() {
    assert_eq!(D38s12::ZERO.trailing_zeros(), 128);
}

#[test]
fn trailing_zeros_one_is_zero() {
    assert_eq!(D38s12::from_bits(1).trailing_zeros(), 0);
}

// --- count_ones / count_zeros -----------------------------------

#[test]
fn count_ones_pattern() {
    // 0b101 has two ones.
    assert_eq!(D38s12::from_bits(0b101).count_ones(), 2);
}

#[test]
fn count_zeros_pattern() {
    // 0b101 has 128 - 2 = 126 zeros (in i128 storage).
    assert_eq!(D38s12::from_bits(0b101).count_zeros(), 126);
}

#[test]
fn count_ones_zero_is_zero() {
    assert_eq!(D38s12::ZERO.count_ones(), 0);
}

#[test]
fn count_ones_neg_one_is_128() {
    // -1 raw is all-ones.
    assert_eq!(D38s12::from_bits(-1).count_ones(), 128);
}

#[test]
fn count_zeros_complement_relation() {
    // count_ones + count_zeros == 128 for every value.
    let a = D38s12::from_bits(0xDEAD_BEEF_CAFE_i128);
    assert_eq!(a.count_ones() + a.count_zeros(), 128);
}

// --- is_power_of_two / next_power_of_two ------------------------

#[test]
fn is_power_of_two_true_for_eight() {
    assert!(D38s12::from_bits(8).is_power_of_two());
}

#[test]
fn is_power_of_two_false_for_seven() {
    assert!(!D38s12::from_bits(7).is_power_of_two());
}

#[test]
fn is_power_of_two_false_for_zero() {
    assert!(!D38s12::ZERO.is_power_of_two());
}

#[test]
fn is_power_of_two_false_for_negative() {
    // Negative i128 has the sign bit set; reinterpreted as u128 the
    // popcount is more than one, so not a power of two.
    assert!(!D38s12::from_bits(-1).is_power_of_two());
}

#[test]
fn is_power_of_two_storage_not_value_semantic() {
    // D38s12::ONE has storage 10^12 = 2^12 * 5^12, not a power of
    // two. Documents the storage-not-value semantic.
    assert!(!D38s12::ONE.is_power_of_two());
}

#[test]
fn next_power_of_two_seven_is_eight() {
    assert_eq!(
        D38s12::from_bits(7).next_power_of_two(),
        D38s12::from_bits(8)
    );
}

#[test]
fn next_power_of_two_eight_is_eight() {
    // Already a power of two -- stays put.
    assert_eq!(
        D38s12::from_bits(8).next_power_of_two(),
        D38s12::from_bits(8)
    );
}

#[test]
fn next_power_of_two_one_is_one() {
    assert_eq!(
        D38s12::from_bits(1).next_power_of_two(),
        D38s12::from_bits(1)
    );
}

// --- Const-generic exercise at a non-default scale --------------

#[test]
fn ops_work_at_scale_six() {
    type D6 = D38<6>;
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
