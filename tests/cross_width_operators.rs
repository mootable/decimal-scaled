//! Cross-width operator overloading: `a op b` where the operands differ in
//! storage width, SCALE, or both.
//!
//! The contract under test:
//! - result type is the WIDER storage width at the LEFT operand's SCALE;
//! - the width promotion is lossless;
//! - a finer-scaled right-hand side is rescaled into the left's SCALE;
//! - compound assignment keeps the left's width AND scale in both directions,
//!   and a narrowing result that does not fit panics with Rust's standard
//!   overflow wording rather than failing to compile.

use decimal_scaled::{D18, D38};

fn n(v: i64) -> D18<2> {
    D18::<2>::try_from(v).unwrap()
}
fn w(v: i64) -> D38<2> {
    D38::<2>::try_from(v).unwrap()
}

// ---------------------------------------------------------------- value ops

#[test]
fn value_ops_promote_to_the_wider_width_in_both_directions() {
    // narrow op wide, and wide op narrow, both land on the wider type
    let s: D38<2> = n(12) + w(4);
    assert_eq!(s, w(16));
    let s: D38<2> = w(12) + n(4);
    assert_eq!(s, w(16));

    let d: D38<2> = n(12) - w(4);
    assert_eq!(d, w(8));
    let d: D38<2> = w(12) - n(4);
    assert_eq!(d, w(8));

    let m: D38<2> = n(12) * w(4);
    assert_eq!(m, w(48));
    let m: D38<2> = w(12) * n(4);
    assert_eq!(m, w(48));

    let q: D38<2> = n(12) / w(4);
    assert_eq!(q, w(3));
    let q: D38<2> = w(12) / n(4);
    assert_eq!(q, w(3));

    let r: D38<2> = n(13) % w(4);
    assert_eq!(r, w(1));
    let r: D38<2> = w(13) % n(4);
    assert_eq!(r, w(1));
}

#[test]
fn result_scale_is_the_left_operands_not_the_wider_operands() {
    // left is the NARROW operand -> result takes the narrow operand's SCALE
    let a = D18::<2>::try_from(5i64).unwrap();
    let b = D38::<6>::try_from(7i64).unwrap();
    let r: D38<2> = a + b;
    assert_eq!(r, D38::<2>::try_from(12i64).unwrap());

    // left is the WIDE operand -> result takes the wide operand's SCALE
    let r: D38<6> = b + a;
    assert_eq!(r, D38::<6>::try_from(12i64).unwrap());
}

#[test]
fn operand_order_changes_the_result_type_but_not_the_value() {
    let a = D18::<2>::try_from(5i64).unwrap();
    let b = D38::<6>::try_from(7i64).unwrap();
    let left: D38<2> = a + b;
    let right: D38<6> = b + a;
    // different types, same number
    assert_eq!(left.to_string(), "12.00");
    assert_eq!(right.to_string(), "12.000000");
}

#[test]
fn finer_scaled_rhs_is_rescaled_into_the_left_scale() {
    // 0.005 at SCALE 6 added to SCALE 2 must round at the target scale,
    // not silently keep digits the target cannot hold.
    let a = D18::<2>::try_from(1i64).unwrap();
    let b = D38::<6>::from_bits(decimal_scaled::Int::<2>::from(5_000i64)); // 0.005000
    let r: D38<2> = a + b;
    // 1.005 -> HalfToEven at SCALE 2 -> 1.00
    assert_eq!(r.to_string(), "1.00");
}

#[test]
fn negative_and_zero_operands() {
    let r: D38<2> = n(-12) + w(4);
    assert_eq!(r, w(-8));
    let r: D38<2> = n(0) + w(-4);
    assert_eq!(r, w(-4));
    let r: D38<2> = n(-12) * w(-4);
    assert_eq!(r, w(48));
    let r: D38<2> = n(0) * w(99);
    assert_eq!(r, w(0));
}

#[test]
fn scale_zero_operands() {
    let a = D18::<0>::try_from(7i64).unwrap();
    let b = D38::<0>::try_from(2i64).unwrap();
    let r: D38<0> = a + b;
    assert_eq!(r, D38::<0>::try_from(9i64).unwrap());
    let r: D38<0> = a % b;
    assert_eq!(r, D38::<0>::try_from(1i64).unwrap());
}

// -------------------------------------------------------- compound assigns

#[test]
fn widening_compound_assign_keeps_left_width_and_scale() {
    let mut acc = D38::<2>::try_from(10i64).unwrap();
    acc += n(5);
    assert_eq!(acc, w(15));
    acc -= n(3);
    assert_eq!(acc, w(12));
    acc *= n(2);
    assert_eq!(acc, w(24));
    acc /= n(4);
    assert_eq!(acc, w(6));
    acc %= n(4);
    assert_eq!(acc, w(2));
}

#[test]
fn narrowing_compound_assign_works_when_the_value_fits() {
    let mut acc = n(10);
    acc += w(5);
    assert_eq!(acc, n(15));
    acc -= w(3);
    assert_eq!(acc, n(12));
    acc *= w(2);
    assert_eq!(acc, n(24));
    acc /= w(4);
    assert_eq!(acc, n(6));
    acc %= w(4);
    assert_eq!(acc, n(2));
}

#[test]
fn narrowing_compound_assign_across_scales() {
    let mut acc = D18::<2>::try_from(10i64).unwrap();
    acc += D38::<6>::try_from(7i64).unwrap();
    assert_eq!(acc, D18::<2>::try_from(17i64).unwrap());
}

#[test]
fn narrowing_compound_assign_of_a_negative_result() {
    let mut acc = n(3);
    acc -= w(10);
    assert_eq!(acc, n(-7));
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn narrowing_compound_assign_overflow_panics_like_normal_arithmetic() {
    let big = w(i64::MAX) * w(1_000);
    let mut small = n(1);
    small += big;
}

#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn narrowing_compound_assign_multiply_overflow_panics() {
    // fits D18<2> comfortably; the product fits the D38 compute width but
    // NOT the D18 result width, so the narrowing step is what overflows.
    let mut small = n(1_000_000_000_000_000);
    small *= w(1_000_000);
}

// ------------------------------------------------------------- comparisons

#[test]
fn comparisons_already_span_width_and_scale() {
    let a = D18::<2>::try_from(5i64).unwrap();
    let b = D38::<6>::try_from(7i64).unwrap();
    let c = D38::<6>::try_from(5i64).unwrap();
    assert!(a < b);
    assert!(b > a);
    assert_eq!(a, c);
    assert_ne!(a, b);
}
