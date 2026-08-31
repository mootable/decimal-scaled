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

/// The region wall at a LARGE working scale — where the artanh series'
/// term count (`≈2.1·w` inside the wall) and the 20 000-iteration
/// series cap are furthest apart, and where a mis-placed wall would
/// show as a truncated series rather than a slow one. D307<150> runs at
/// `w ≈ 160`; the argument set straddles both region edges and reaches
/// the domain edge `t → -1`.
#[test]
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
fn log1p_equals_ln_of_one_plus_t_d307_s150() {
    use crate::int::types::traits::BigInt;
    let unit = crate::consts::pow10::dispatch::<Int<16>>(150);
    let lsb = <Int<16> as BigInt>::ONE;
    let cases = [
        Int::<16>::ZERO,
        lsb,
        -lsb,
        unit >> 1,
        -(unit >> 1),
        (unit >> 1) + lsb,
        unit,
        unit + lsb,
        unit + unit,
        -(unit - lsb),
    ];
    for &t in &cases {
        for &mode in &MODES {
            let x = D::<Int<16>, 150>(t);
            let y = D::<Int<16>, 150>(t + unit);
            assert_eq!(
                x.log1p_strict_with(mode).to_bits(),
                y.ln_strict_with(mode).to_bits(),
                "D307<150> log1p != ln(1+t) at t={t:?} mode={mode:?}"
            );
        }
    }
}

/// The widest shipped tier at its top scale — D1232<1231>, `w ≈ 1241`,
/// the largest working scale the crate reaches. Inside the region wall
/// the artanh series needs `≈2.1·w ≈ 2 600` terms here, its worst case;
/// this is the case the wall's series-cap headroom argument rests on,
/// so both region edges are checked at the extreme. Kept to a small
/// argument set because each call is a full-width evaluation.
#[test]
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
fn log1p_equals_ln_of_one_plus_t_d1232_s1231() {
    use crate::int::types::traits::BigInt;
    let unit = crate::consts::pow10::dispatch::<Int<64>>(1231);
    let lsb = <Int<64> as BigInt>::ONE;
    // `t → -1` is bounded by the OUTPUT range, not the domain: at this
    // scale `|log1p(t)| < ~100` is all the storage holds, so the deepest
    // representable approach is `1 + t = 10^-20` (`log1p ≈ -46.05`).
    // Anything closer to -1 legitimately overflows and panics, exactly as
    // `ln` of the same argument does.
    let deepest_neg = -(unit - crate::consts::pow10::dispatch::<Int<64>>(1211));
    let cases = [
        lsb,           // tiny t — artanh region, the sub-resolution case
        -lsb,          // its negative mirror
        unit,          // upper region edge
        unit + lsb,    // just outside it
        -(unit >> 1),  // lower region edge
        deepest_neg,   // as close to the domain edge as the range allows
    ];
    for &t in &cases {
        for &mode in &[RoundingMode::HalfToEven, RoundingMode::Floor] {
            let x = D::<Int<64>, 1231>(t);
            let y = D::<Int<64>, 1231>(t + unit);
            assert_eq!(
                x.log1p_strict_with(mode).to_bits(),
                y.ln_strict_with(mode).to_bits(),
                "D1232<1231> log1p != ln(1+t) at mode={mode:?}"
            );
        }
    }
}

/// `S` is the scale every anchor above is stated at; assert `UNIT` is
/// consistent with it so a future scale change cannot silently
/// invalidate the baked expectations.
#[test]
fn oracle_unit_matches_the_stated_scale() {
    assert_eq!(UNIT, 10_i128.pow(S), "UNIT must be 10^S");
}
