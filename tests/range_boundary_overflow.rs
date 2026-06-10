//! Overflow-contract pins at the range boundary for the exact-power pins.
//!
//! `exp2(127)` at D38<0> is `2^127 = i128::MAX + 1` — out of the decimal
//! range by exactly one. The exact-power pin's positive-`k` overflow is
//! PROOF of out-of-range and must panic under EVERY mode (debug and
//! release): the `exp(k·ln 2)` composition it used to defer to sits a hair
//! below the true integer, so Floor / Trunc directed-rounded the
//! approximation back INSIDE the range while the nearest modes panicked —
//! a mode-dependent overflow contract violation. The in-range neighbour
//! must stay exact for every mode. Same proof for `powf`'s positive
//! integer-power pin.

use core::str::FromStr;
use decimal_scaled::RoundingMode::{
    self, Ceiling, Floor, HalfAwayFromZero, HalfToEven, HalfTowardZero, Trunc,
};
use decimal_scaled::D38;

const MODES: [RoundingMode; 6] = [
    HalfToEven,
    HalfAwayFromZero,
    HalfTowardZero,
    Trunc,
    Floor,
    Ceiling,
];

#[test]
fn exp2_out_by_one_panics_every_mode() {
    let v = D38::<0>::from_str("127").unwrap();
    for mode in MODES {
        let r = std::panic::catch_unwind(|| v.exp2_strict_with(mode));
        assert!(r.is_err(), "exp2(127) D38<0> {mode:?}: expected panic");
    }
}

#[test]
fn exp2_boundary_neighbour_is_exact_every_mode() {
    // 2^126 — the largest in-range power of two at scale 0.
    let v = D38::<0>::from_str("126").unwrap();
    let want = "85070591730234615865843651857942052864";
    for mode in MODES {
        assert_eq!(v.exp2_strict_with(mode).to_string(), want, "{mode:?}");
    }
}

#[test]
fn powf_integer_power_out_by_factor_panics_every_mode() {
    // 4^64 = 2^128 — past the decimal range; the pin's overflow is proof.
    let b = D38::<0>::from_str("4").unwrap();
    let e = D38::<0>::from_str("64").unwrap();
    for mode in MODES {
        let r = std::panic::catch_unwind(|| b.powf_strict_with(e, mode));
        assert!(r.is_err(), "powf(4, 64) D38<0> {mode:?}: expected panic");
    }
}

#[test]
fn powf_integer_power_boundary_neighbour_is_exact_every_mode() {
    // 4^63 = 2^126 — in range, exactly representable.
    let b = D38::<0>::from_str("4").unwrap();
    let e = D38::<0>::from_str("63").unwrap();
    let want = "85070591730234615865843651857942052864";
    for mode in MODES {
        assert_eq!(b.powf_strict_with(e, mode).to_string(), want, "{mode:?}");
    }
}
