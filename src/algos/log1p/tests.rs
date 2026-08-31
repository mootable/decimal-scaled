// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log1p` family tests — exercised through the public per-width
//! surface, so each case covers the matcher's region choice as well as
//! the kernel it lands on.
//!
//! Expected values come from the published decimal expansions of the
//! underlying logarithms, never from this crate's own output. The
//! `log1p == ln(1 + t)` cases are a CONSISTENCY identity between two
//! public entry points (both correctly rounded to the same real number,
//! so they must agree bit-for-bit), not an accuracy oracle.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
use crate::D;

const MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Scale the oracle anchors below are stated at.
const S: u32 = 20;
/// `1.0` at [`S`].
const UNIT: i128 = 10_i128.pow(20);

fn d38s20(raw: i128) -> D<Int<2>, 20> {
    D::<Int<2>, 20>(Int::<2>::from_i128(raw))
}

/// Correctly-rounded `log1p` at D38<20>, half-to-even, from the
/// published decimal expansions of the underlying logarithms:
///
/// - `ln 2   = 0.69314718055994530941 7232…` → `…942`
/// - `ln 1.5 = 0.40546510810816438197 8013…` → `…198`
/// - `ln 3   = 1.09861228866810969139 5245…` → `…140`
/// - `ln 10  = 2.30258509299404568401 7991…` → `…402`
///
/// The set straddles the matcher's region wall on purpose: `t = 0.5`
/// and `t = 1` are inside the artanh region, `t = 2` and `t = 9` are
/// outside it, `t = -0.5` sits exactly on the lower edge.
#[test]
fn log1p_matches_external_oracle_d38_s20() {
    const CASES: [(i128, i128); 5] = [
        // log1p(1) = ln 2 — artanh region, upper edge.
        (UNIT, 69_314_718_055_994_530_942),
        // log1p(0.5) = ln 1.5 — artanh region.
        (UNIT / 2, 40_546_510_810_816_438_198),
        // log1p(2) = ln 3 — WithLn region.
        (2 * UNIT, 109_861_228_866_810_969_140),
        // log1p(9) = ln 10 — WithLn region.
        (9 * UNIT, 230_258_509_299_404_568_402),
        // log1p(-0.5) = ln 0.5 = -ln 2 — artanh region, lower edge.
        (-UNIT / 2, -69_314_718_055_994_530_942),
    ];
    for &(t_raw, expected) in &CASES {
        assert_eq!(
            d38s20(t_raw)
                .log1p_strict_with(RoundingMode::HalfToEven)
                .to_bits()
                .as_i128(),
            expected,
            "log1p D38<20> HalfToEven at raw={t_raw}"
        );
    }
}

/// `log1p(0) = 0` exactly, in every mode — the one rational value the
/// function takes.
#[test]
fn log1p_of_zero_is_zero_in_every_mode() {
    for &mode in &MODES {
        assert_eq!(
            d38s20(0).log1p_strict_with(mode).to_bits().as_i128(),
            0,
            "log1p(0) mode={mode:?}"
        );
    }
}

/// The sub-ULP argument `t = 10^-20` at D38<20>: the true value
/// `t - t²/2 + …` sits `5·10^-21` of an LSB BELOW one LSB, so the
/// directed modes must split (`Floor`/`Trunc` down, `Ceiling` up) and
/// the nearest modes must round to 1. This only resolves if the Ziv
/// escalation actually walks past the base guard.
#[test]
fn log1p_sub_ulp_argument_splits_the_directed_modes() {
    const EXPECTED: [(RoundingMode, i128); 6] = [
        (RoundingMode::HalfToEven, 1),
        (RoundingMode::HalfAwayFromZero, 1),
        (RoundingMode::HalfTowardZero, 1),
        (RoundingMode::Trunc, 0),
        (RoundingMode::Floor, 0),
        (RoundingMode::Ceiling, 1),
    ];
    for &(mode, expected) in &EXPECTED {
        assert_eq!(
            d38s20(1).log1p_strict_with(mode).to_bits().as_i128(),
            expected,
            "log1p(1e-20) mode={mode:?}"
        );
    }
    // Mirror on the negative side: the magnitude exceeds one LSB by the
    // same `5·10^-21`, so `Floor` alone steps to -2.
    const EXPECTED_NEG: [(RoundingMode, i128); 6] = [
        (RoundingMode::HalfToEven, -1),
        (RoundingMode::HalfAwayFromZero, -1),
        (RoundingMode::HalfTowardZero, -1),
        (RoundingMode::Trunc, -1),
        (RoundingMode::Floor, -2),
        (RoundingMode::Ceiling, -1),
    ];
    for &(mode, expected) in &EXPECTED_NEG {
        assert_eq!(
            d38s20(-1).log1p_strict_with(mode).to_bits().as_i128(),
            expected,
            "log1p(-1e-20) mode={mode:?}"
        );
    }
}

/// The parity claim: `log1p(t)` and `ln(1 + t)` are the SAME correctly
/// rounded value at a shared scale. Spans both matcher regions and both
/// sides of each region edge, including the near-domain-edge arguments
/// where the artanh series would not converge (so a mis-placed region
/// wall fails here).
#[test]
fn log1p_equals_ln_of_one_plus_t_d38_s20() {
    const TS: [i128; 17] = [
        0,
        1,
        -1,
        UNIT / 1_000,
        -UNIT / 1_000,
        UNIT / 2,      // lower region edge (t = -1/2 mirrored)
        -UNIT / 2,     // lower region edge
        -UNIT / 2 - 1, // just outside the lower edge
        UNIT,          // upper region edge (t = 1)
        UNIT + 1,      // just outside the upper edge
        2 * UNIT,
        9 * UNIT,
        1_000 * UNIT,
        -(UNIT - 1),   // t = -0.999…9, |u| → 1
        -(UNIT / 10),
        -(9 * UNIT / 10),
        -(99 * UNIT / 100),
    ];
    for &t in &TS {
        for &mode in &MODES {
            assert_eq!(
                d38s20(t).log1p_strict_with(mode).to_bits().as_i128(),
                d38s20(t + UNIT).ln_strict_with(mode).to_bits().as_i128(),
                "log1p != ln(1+t) at t_raw={t} mode={mode:?}"
            );
        }
    }
}

/// The two kernels must agree wherever both are valid — the region wall
/// moves cost, never the value. Checked on the overlap band (both are
/// correct for `|t| ≤ 1/2`) by driving each kernel directly at the
/// narrow work integer.
#[test]
fn both_kernels_agree_inside_the_overlap_band() {
    use crate::algos::log1p::log1p_artanh::log1p_artanh_g;
    use crate::algos::log1p::log1p_with_ln::log1p_with_ln_g;
    use crate::algos::support::narrow_ziv::WZiv;
    const GUARD: u32 = 30;
    const TS: [i128; 9] = [
        0,
        1,
        -1,
        UNIT / 1_000,
        -UNIT / 1_000,
        UNIT / 3,
        -UNIT / 3,
        UNIT / 2,
        -UNIT / 2,
    ];
    for &t in &TS {
        for &mode in &MODES {
            let v = Int::<2>::from_i128(t);
            assert_eq!(
                log1p_artanh_g::<Int<2>, WZiv, 20>(
                    v,
                    GUARD,
                    Int::<2>::MAX,
                    Int::<2>::MIN,
                    mode
                ),
                log1p_with_ln_g::<Int<2>, WZiv, 20>(
                    v,
                    GUARD,
                    Int::<2>::MAX,
                    Int::<2>::MIN,
                    mode
                ),
                "artanh != with_ln at t_raw={t} mode={mode:?}"
            );
        }
    }
}

/// `_approx` at a guard well past the strict one reproduces the strict
/// result on well-conditioned inputs — the `*_approx` contract (same
/// value, caller-chosen working width). Spans both regions.
#[test]
fn log1p_approx_at_a_deep_guard_matches_strict() {
    const TS: [i128; 5] = [UNIT / 2, UNIT, 2 * UNIT, 9 * UNIT, -UNIT / 2];
    for &t in &TS {
        for &mode in &MODES {
            assert_eq!(
                d38s20(t).log1p_approx_with(60, mode).to_bits().as_i128(),
                d38s20(t).log1p_strict_with(mode).to_bits().as_i128(),
                "log1p_approx(60) != log1p_strict at t_raw={t} mode={mode:?}"
            );
        }
    }
}

/// Every `*_with(mode)` has a default-mode sibling that agrees with it.
#[test]
fn log1p_default_mode_siblings_agree() {
    let t = UNIT / 2;
    assert_eq!(
        d38s20(t).log1p_approx(45).to_bits().as_i128(),
        d38s20(t)
            .log1p_approx_with(45, crate::support::rounding::DEFAULT_ROUNDING_MODE)
            .to_bits()
            .as_i128(),
        "log1p_approx != log1p_approx_with(default mode)"
    );
    assert_eq!(
        d38s20(t).log1p_strict().to_bits().as_i128(),
        d38s20(t)
            .log1p_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
            .to_bits()
            .as_i128(),
        "log1p_strict != log1p_strict_with(default mode)"
    );
}

/// Domain: `t = -1` is out of domain (`1 + t == 0`, exactly `ln`'s
/// non-positive-argument case) and panics.
#[test]
#[should_panic(expected = "log1p: argument must be greater than -1")]
fn log1p_at_minus_one_panics() {
    let _ = d38s20(-UNIT).log1p_strict();
}

/// Domain: `t < -1` panics.
#[test]
#[should_panic(expected = "log1p: argument must be greater than -1")]
fn log1p_below_minus_one_panics() {
    let _ = d38s20(-2 * UNIT).log1p_strict();
}

/// The `_approx` surface carries the same domain guard.
#[test]
#[should_panic(expected = "log1p: argument must be greater than -1")]
fn log1p_approx_below_minus_one_panics() {
    let _ = d38s20(-2 * UNIT).log1p_approx(45);
}

/// D18 (`Int<1>`) routes through the same policy at its own storage
/// width and agrees with `ln(1 + t)` there too.
#[test]
fn log1p_equals_ln_of_one_plus_t_d18_s9() {
    const UNIT9: i128 = 1_000_000_000;
    for &t in &[
        0_i128,
        1,
        -1,
        UNIT9 / 2,
        -UNIT9 / 2,
        UNIT9,
        3 * UNIT9,
        -(UNIT9 - 1),
    ] {
        for &mode in &MODES {
            let x = D::<Int<1>, 9>(Int::<1>::from_i128(t));
            let y = D::<Int<1>, 9>(Int::<1>::from_i128(t + UNIT9));
            assert_eq!(
                x.log1p_strict_with(mode).to_bits().as_i128(),
                y.ln_strict_with(mode).to_bits().as_i128(),
                "D18 log1p != ln(1+t) at t_raw={t} mode={mode:?}"
            );
        }
    }
}

/// The wide arm runs the SAME generic kernels at the tier's own work
/// integer; it must agree with `ln(1 + t)` at that width as well.
#[test]
#[cfg(any(feature = "d57", feature = "wide"))]
fn log1p_equals_ln_of_one_plus_t_d57_s20() {
    let unit = Int::<3>::from_i128(UNIT);
    for &t in &[
        0_i128,
        1,
        -1,
        UNIT / 2,
        -UNIT / 2,
        UNIT,
        UNIT + 1,
        5 * UNIT,
        -(UNIT - 1),
    ] {
        for &mode in &MODES {
            let x = D::<Int<3>, 20>(Int::<3>::from_i128(t));
            let y = D::<Int<3>, 20>(Int::<3>::from_i128(t) + unit);
            assert_eq!(
                x.log1p_strict_with(mode).to_bits(),
                y.ln_strict_with(mode).to_bits(),
                "D57 log1p != ln(1+t) at t_raw={t} mode={mode:?}"
            );
        }
    }
}

/// The wide oracle anchors, at a different width than the D38 set — the
/// same external expansions, so the wide work integer is checked against
/// the literature and not against the narrow arm.
#[test]
#[cfg(any(feature = "d57", feature = "wide"))]
fn log1p_matches_external_oracle_d57_s20() {
    const CASES: [(i128, i128); 4] = [
        (UNIT, 69_314_718_055_994_530_942),
        (UNIT / 2, 40_546_510_810_816_438_198),
        (2 * UNIT, 109_861_228_866_810_969_140),
        (9 * UNIT, 230_258_509_299_404_568_402),
    ];
    for &(t_raw, expected) in &CASES {
        assert_eq!(
            D::<Int<3>, 20>(Int::<3>::from_i128(t_raw))
                .log1p_strict_with(RoundingMode::HalfToEven)
                .to_bits(),
            Int::<3>::from_i128(expected),
            "log1p D57<20> HalfToEven at raw={t_raw}"
        );
    }
}

/// `S` is the scale every anchor above is stated at; assert `UNIT` is
/// consistent with it so a future scale change cannot silently
/// invalidate the baked expectations.
#[test]
fn oracle_unit_matches_the_stated_scale() {
    assert_eq!(UNIT, 10_i128.pow(S), "UNIT must be 10^S");
}
