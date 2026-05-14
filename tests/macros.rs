#![cfg(feature = "macros")]

use decimal_scaled::{D128s12, d128};

#[test]
fn it_can_parse_standard_decimal() {
    let num = d128!();
    assert_eq!(num, D128s12::from_bits(0));
}
