#![cfg(feature = "macros")]

use decimal_scaled::{D128e12, i128s};

#[test]
fn it_can_parse_standard_decimal() {
    let num = i128s!();
    assert_eq!(num, D128e12::from_bits(0));
}
