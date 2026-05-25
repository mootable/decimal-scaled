#![cfg(feature = "macros")]
//! Integration tests for the `d38!` proc-macro.
//!
//! See `macros/README.md` for the macro spec. This corpus covers
//! decimal literals (positive / negative, fractional, underscore
//! separators), scientific notation, the `scale N` and `rounded`
//! qualifiers, and the compile-time error path when a literal is
//! lossy without `rounded`.

use decimal_scaled::{D38, D38s0, D38s2, D38s5, D38s6, D38s12, d38};

// --- Auto-scale inference --------------------------------------------

#[test]
fn auto_scale_inference_simple() {
    let v = d38!(1.23);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(123_i128).unwrap()));
}

#[test]
fn auto_scale_inference_integer() {
    let v = d38!(123);
    assert_eq!(v, D38s0::from_bits(decimal_scaled::Int::<2>::try_from(123_i128).unwrap()));
}

#[test]
fn auto_scale_inference_zero_fractional() {
    let v = d38!(1.0);
    assert_eq!(v, D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from(10_i128).unwrap()));
}

#[test]
fn auto_scale_inference_trailing_zero_preserved() {
    let v = d38!(1.230);
    assert_eq!(v, D38::<3>::from_bits(decimal_scaled::Int::<2>::try_from(1230_i128).unwrap()));
}

#[test]
fn auto_scale_inference_leading_zero_after_dot() {
    let v = d38!(0.001);
    assert_eq!(v, D38::<3>::from_bits(decimal_scaled::Int::<2>::try_from(1_i128).unwrap()));
}

#[test]
fn auto_scale_inference_negative() {
    let v = d38!(-1.23);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(-123_i128).unwrap()));
}

#[test]
fn auto_scale_inference_with_underscores() {
    let v = d38!(1_234.567_89);
    assert_eq!(v, D38s5::from_bits(decimal_scaled::Int::<2>::try_from(123_456_789_i128).unwrap()));
}

// --- Scientific notation ---------------------------------------------

#[test]
fn sci_simple() {
    // 1.5e3 = 1500; mantissa scale 1, exponent 3 -> natural scale = max(0, 1-3) = 0.
    let v = d38!(1.5e3);
    assert_eq!(v, D38s0::from_bits(decimal_scaled::Int::<2>::try_from(1500_i128).unwrap()));
}

#[test]
fn sci_negative_exponent() {
    // 1.5e-3 = 0.0015; mantissa scale 1, exponent -3 -> natural scale = 4.
    let v = d38!(1.5e-3);
    assert_eq!(v, D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(15_i128).unwrap()));
}

#[test]
fn sci_zero_exponent() {
    let v = d38!(1.5e0);
    assert_eq!(v, D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from(15_i128).unwrap()));
}

#[test]
fn sci_integer_with_exponent() {
    // 1e6 = 1_000_000; mantissa scale 0, exponent 6 -> natural scale 0,
    // digits padded by 6 zeros.
    let v = d38!(1e6);
    assert_eq!(v, D38s0::from_bits(decimal_scaled::Int::<2>::try_from(1_000_000_i128).unwrap()));
}

#[test]
fn sci_trailing_zero_mantissa() {
    // 1.500e2 = 150.0; mantissa scale 3, exponent 2 -> natural scale 1.
    let v = d38!(1.500e2);
    assert_eq!(v, D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from(1500_i128).unwrap()));
}

#[test]
fn sci_negative_value() {
    let v = d38!(-2.5e-2);
    assert_eq!(v, D38::<3>::from_bits(decimal_scaled::Int::<2>::try_from(-25_i128).unwrap()));
}

// --- `scale N` qualifier ---------------------------------------------

#[test]
fn explicit_scale_up_pads_with_zeros() {
    let v = d38!(1.23, scale 4);
    assert_eq!(v, D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(12_300_i128).unwrap()));
}

#[test]
fn explicit_scale_equal_is_identity() {
    let v = d38!(1.23, scale 2);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(123_i128).unwrap()));
}

#[test]
fn explicit_scale_zero_on_integer() {
    let v = d38!(42, scale 0);
    assert_eq!(v, D38s0::from_bits(decimal_scaled::Int::<2>::try_from(42_i128).unwrap()));
}

#[test]
fn explicit_scale_with_sci_notation() {
    // 1.5e3 = 1500 (natural scale 0); explicit scale 5 -> pad with 5 zeros.
    let v = d38!(1.5e3, scale 5);
    assert_eq!(v, D38::<5>::from_bits(decimal_scaled::Int::<2>::try_from(150_000_000_i128).unwrap()));
}

// --- `rounded` qualifier ---------------------------------------------

#[test]
fn rounded_half_to_even_below_half() {
    // 1.234999 at scale 2 -> 1.23 (below half).
    let v = d38!(1.234999, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(123_i128).unwrap()));
}

#[test]
fn rounded_half_to_even_above_half() {
    // 1.235001 at scale 2 -> 1.24 (above half).
    let v = d38!(1.235001, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(124_i128).unwrap()));
}

#[test]
fn rounded_half_to_even_exact_half_ties_to_even() {
    // 1.235 at scale 2 -> 1.24 (tie, 4 is even).
    let v = d38!(1.235, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(124_i128).unwrap()));

    // 1.225 at scale 2 -> 1.22 (tie, 2 is even).
    let v = d38!(1.225, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(122_i128).unwrap()));
}

#[test]
fn rounded_negative_value() {
    let v = d38!(-1.235, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(-124_i128).unwrap()));
}

// --- More named precision tiers --------------------------------------

#[test]
fn explicit_scale_to_max_supported() {
    // v0.4.0 cap: MAX_SCALE for D38 is 37.
    let v = d38!(1, scale 37);
    assert_eq!(v.to_bits(), 10_i128.pow(37));
}

#[test]
fn primary_alias_d38s12() {
    let v = d38!(1.500_000_000_000);
    assert_eq!(v, D38s12::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_000_000_i128).unwrap()));
}

#[test]
fn financial_cents() {
    let v = d38!(0.50);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(50_i128).unwrap()));
}

#[test]
fn satoshi_grade() {
    // 0.123_456_78 at scale 8.
    let v = d38!(0.123_456_78);
    assert_eq!(v, D38::<8>::from_bits(decimal_scaled::Int::<2>::try_from(12_345_678_i128).unwrap()));
}

#[test]
fn small_value_via_sci() {
    let v = d38!(1e-9);
    assert_eq!(v, D38::<9>::from_bits(decimal_scaled::Int::<2>::try_from(1_i128).unwrap()));
}

#[test]
fn six_digit_micro() {
    let v = d38!(1.234_567, scale 6);
    assert_eq!(v, D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_234_567_i128).unwrap()));
}

// --- Inline expressions ---------------------------------------------

#[test]
fn expression_simple_arithmetic() {
    let v = d38!(10 * 12 + 3, scale 0);
    assert_eq!(v, D38s0::from_bits(decimal_scaled::Int::<2>::try_from(123_i128).unwrap()));
}

#[test]
fn expression_with_scale_factor() {
    // Scale 4 means each input unit becomes 10_000 bits.
    let v = d38!(5, scale 4);
    assert_eq!(v, D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(50_000_i128).unwrap()));
}

#[test]
fn expression_with_variable() {
    let x: i128 = 42;
    let v = d38!(x, scale 2);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(4_200_i128).unwrap()));
}

#[test]
fn expression_with_const() {
    const N: i128 = 100;
    let v = d38!(N + 23, scale 2);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(12_300_i128).unwrap()));
}

#[test]
fn expression_function_call() {
    fn produce() -> i128 {
        7
    }
    let v = d38!(produce() * 2, scale 0);
    assert_eq!(v, D38s0::from_bits(decimal_scaled::Int::<2>::try_from(14_i128).unwrap()));
}

#[test]
fn expression_negative_result() {
    let v = d38!(0 - 5, scale 2);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(-500_i128).unwrap()));
}

#[test]
fn expression_value_at_runtime() {
    // NB: the expression form of `d38!` is no longer `const`-evaluable —
    // D38's storage is `Int<2>`, whose `checked_mul`/`pow` are not `const fn`
    // (the literal form stays const). See research/2026_05_22_const_fn_arithmetic_followup.md.
    const N: i128 = 42;
    let v: D38s2 = d38!(N * 3, scale 2);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(12_600_i128).unwrap()));
}

// ── Per-width entry points: narrow tiers ──────────────────────────────

#[test]
fn d18_literal_inferred_scale() {
    use decimal_scaled::{D18s4, d18};
    let v = d18!(1234.5678);
    assert_eq!(v, D18s4::from_bits(decimal_scaled::Int::<1>::from(12_345_678_i64)));
}

#[test]
fn d18_explicit_scale_pad() {
    use decimal_scaled::{D18s9, d18};
    let v = d18!(1.5, scale 9);
    assert_eq!(v, D18s9::from_bits(decimal_scaled::Int::<1>::from(1_500_000_000_i64)));
}

#[test]
fn d18_expression_form() {
    use decimal_scaled::{D18s4, d18};
    let raw: i64 = 12345;
    let v = d18!(raw, scale 4);
    assert_eq!(v.to_bits(), 12345i64 * 10_000);
    let _: D18s4 = v;
}

// ── Per-width entry points: wide tiers ────────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn d76_literal_inferred_scale() {
    use decimal_scaled::{D76s2, d76};
    let v: D76s2 = d76!(1.23);
    assert_eq!(v.to_string(), "1.23");
}

#[cfg(feature = "wide")]
#[test]
fn d76_explicit_scale() {
    use decimal_scaled::{D76s12, d76};
    let v: D76s12 = d76!(0.000_000_000_001); // scale 12 inferred and matched
    assert_eq!(v.to_string(), "0.000000000001");
}

#[cfg(feature = "wide")]
#[test]
fn d153_literal_inferred() {
    use decimal_scaled::{D153s35, d153};
    // Use a value whose natural scale matches the alias.
    let v: D153s35 = d153!(2.71828182845904523536028747135266250);
    assert!(v.to_string().starts_with("2.71828182845904523536028747"));
}

#[cfg(feature = "wide")]
#[test]
fn d307_literal_inferred() {
    use decimal_scaled::{D307s35, d307};
    let v: D307s35 = d307!(3.14159265358979323846264338327950288);
    assert!(
        v.to_string()
            .starts_with("3.14159265358979323846264338327950288")
    );
}

// ── Per-scale wrapper macros ──────────────────────────────────────────

#[test]
fn d18_per_scale_wrapper_d18s12() {
    use decimal_scaled::{D18s12, d18s12};
    let v: D18s12 = d18s12!(1.5);
    assert_eq!(v, D18s12::from_bits(decimal_scaled::Int::<1>::from(1_500_000_000_000_i64)));
}

#[test]
fn d38_per_scale_wrapper_d38s12() {
    use decimal_scaled::{D38s12, d38s12};
    let v: D38s12 = d38s12!(1.5);
    assert_eq!(v, D38s12::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_000_000_i128).unwrap()));
}

#[test]
fn d38_per_scale_wrapper_forwards_qualifiers() {
    use decimal_scaled::{D38s2, d38s2};
    // Scale 2 is pre-baked; `rounded` is forwarded as a tail
    // qualifier.
    let v: D38s2 = d38s2!(1.234_567, rounded);
    assert_eq!(v, D38s2::from_bits(decimal_scaled::Int::<2>::try_from(123_i128).unwrap()));
}

#[cfg(feature = "wide")]
#[test]
fn d76_per_scale_wrapper_d76s35() {
    use decimal_scaled::{D76s35, d76s35};
    let v: D76s35 = d76s35!(1.5);
    // 1.5 at scale 35 = 1.5 * 10^35.
    assert_eq!(v.to_string(), "1.50000000000000000000000000000000000");
}

// ── Radix-prefixed literals ───────────────────────────────────────────

#[test]
fn d38_hex_prefix_integer() {
    use decimal_scaled::{D38s0, d38};
    // 0xFF == 255 — scale defaults to 0 for radix-prefixed integers.
    let v: D38s0 = d38!(0xFF);
    assert_eq!(v.to_bits(), 255);
}

#[test]
fn d38_oct_prefix_integer() {
    use decimal_scaled::{D38s0, d38};
    let v: D38s0 = d38!(0o755); // == 493
    assert_eq!(v.to_bits(), 493);
}

#[test]
fn d38_bin_prefix_integer() {
    use decimal_scaled::{D38s0, d38};
    let v: D38s0 = d38!(0b1010_0110); // == 166
    assert_eq!(v.to_bits(), 166);
}

#[test]
fn d38_hex_with_explicit_scale() {
    use decimal_scaled::{D38s2, d38};
    let v: D38s2 = d38!(0x7B, scale 2); // 123 at scale 2 = 1.23
    assert_eq!(v.to_string(), "1.23");
}

#[test]
fn d38_explicit_radix_qualifier() {
    use decimal_scaled::{D38s0, d38};
    let v: D38s0 = d38!(123, radix 8); // octal: 1*64 + 2*8 + 3 = 83
    assert_eq!(v.to_bits(), 83);
}

#[cfg(feature = "wide")]
#[test]
fn d76_hex_prefix() {
    use decimal_scaled::{D76s0, d76};
    let v: D76s0 = d76!(0xDEAD_BEEF);
    assert_eq!(v.to_string(), "3735928559");
}

// ── Fractional radix literals ─────────────────────────────────────────

#[test]
fn d38_hex_fractional_literal() {
    use decimal_scaled::{D38, d38};
    // 1.A3 in hex = 0x1A3 magnitude = 419. The decimal point split
    // gives 2 hex fractional digits. Scale must be supplied — the
    // bits ARE the parsed magnitude (no scale shift).
    let v: D38<2> = d38!(1.A3, radix 16, scale 2);
    assert_eq!(v.to_bits(), 419);
}

#[test]
fn d38_bin_fractional_literal() {
    use decimal_scaled::{D38, d38};
    // 11.0110 in binary = 0b110110 = 54. Stored at scale 4 → 0.0054.
    let v: D38<4> = d38!(11.0110, radix 2, scale 4);
    assert_eq!(v.to_bits(), 54);
}

#[test]
fn d38_oct_fractional_literal() {
    use decimal_scaled::{D38, d38};
    // 17.3 in octal: digits "173" in base 8 = 1*64 + 7*8 + 3 = 123.
    let v: D38<2> = d38!(17.3, radix 8, scale 2);
    assert_eq!(v.to_bits(), 123);
}

#[test]
fn d38_negative_hex_fractional() {
    use decimal_scaled::{D38, d38};
    let v: D38<2> = d38!(-1.A3, radix 16, scale 2);
    assert_eq!(v.to_bits(), -419);
}

#[test]
fn d38_hex_fractional_ident_dot_ident() {
    use decimal_scaled::{D38, d38};
    // `FF.AA` in radix 16: Rust tokenises both halves as idents
    // (no leading digit on either side). Magnitude in base 16 is
    // 0xFFAA = 65450.
    let v: D38<2> = d38!(FF.AA, radix 16, scale 2);
    assert_eq!(v.to_bits(), 65450);
}

#[test]
fn d38_hex_fractional_int_dot_ident() {
    use decimal_scaled::{D38, d38};
    // `7.AB` in radix 16: 7 is an Int literal, AB an Ident.
    // 0x7AB = 1963.
    let v: D38<3> = d38!(7.AB, radix 16, scale 3);
    assert_eq!(v.to_bits(), 1963);
}

#[test]
fn d38_hex_fractional_ident_dot_int() {
    use decimal_scaled::{D38, d38};
    // `AB.7` in radix 16: AB is an Ident, 7 is an Int literal.
    // 0xAB7 = 2743.
    let v: D38<3> = d38!(AB.7, radix 16, scale 3);
    assert_eq!(v.to_bits(), 2743);
}

#[test]
fn d38_hex_integer_only_radix() {
    use decimal_scaled::{D38, d38};
    // Bare IDENT (no prefix, no dot) under explicit `radix 16`.
    let v: D38<0> = d38!(BEEF, radix 16);
    assert_eq!(v.to_bits(), 0xBEEF);
}
