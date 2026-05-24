//! Accuracy gate for `hypot_strict_with` — the test that was missing when
//! the old scale-trick's +8..13 ULP double-rounding error went uncaught.
//!
//! Every expected value here is **externally validated**, not computed by
//! this crate:
//!
//! * **Pythagorean triples** — the true hypotenuse is an exact integer by
//!   construction (`3²+4²=5²`, `5²+12²=13²`, `8²+15²=17²`, `7²+24²=25²`,
//!   `20²+21²=29²`). At any `SCALE` the correctly-rounded raw is therefore
//!   `integer · 10^SCALE` with **no rounding** — we assert exact equality
//!   (`delta == 0`) across all six `RoundingMode`s and two `SCALE`s.
//! * **Non-perfect cases** — `hypot(1,1)=√2`, `hypot(2,3)=√13`,
//!   `hypot(123,456)=√209745`. The expected raw integers were computed
//!   offline with Python `mpmath` (v1.4.1, `mp.dps = 160`) as
//!   `nint(sqrt(a²+b²) · 10^SCALE)` and baked in below. The kernel's
//!   floor-root + half-step rounding was independently checked to equal
//!   that `nint` for every case here (no exact-half ties occur for an
//!   integer radicand), so the default `HalfToEven` mode must reproduce
//!   them exactly (`delta == 0`).
//!
//! Coverage: narrow tier D38 (`Int<2>`) and wide tier D57 (`Int<3>`).

#![cfg(all(feature = "wide", feature = "strict", not(feature = "fast")))]

use decimal_scaled::{Int, RoundingMode, D38, D57};

const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Pythagorean triples `(a, b, hypotenuse)` — hypotenuse is an exact
/// integer (math-validated, no rounding).
const TRIPLES: [(i64, i64, i64); 5] = [
    (3, 4, 5),
    (5, 12, 13),
    (8, 15, 17),
    (7, 24, 25),
    (20, 21, 29),
];

// ── D38 (narrow, Int<2>) ──────────────────────────────────────────────

#[test]
fn d38_pythagorean_triples_exact_all_modes_two_scales() {
    // SCALE 6
    for &(a, b, h) in &TRIPLES {
        let da = D38::<6>::try_from(a).unwrap();
        let db = D38::<6>::try_from(b).unwrap();
        let expected = D38::<6>::try_from(h).unwrap();
        for mode in ALL_MODES {
            let got = da.hypot_strict_with(db, mode);
            assert_eq!(
                got, expected,
                "D38<6> hypot({a},{b}) mode {mode:?}: got {got:?} expected exact {expected:?}",
            );
        }
    }
    // SCALE 19 (near the D38 fractional limit)
    for &(a, b, h) in &TRIPLES {
        let da = D38::<19>::try_from(a).unwrap();
        let db = D38::<19>::try_from(b).unwrap();
        let expected = D38::<19>::try_from(h).unwrap();
        for mode in ALL_MODES {
            let got = da.hypot_strict_with(db, mode);
            assert_eq!(
                got, expected,
                "D38<19> hypot({a},{b}) mode {mode:?}: got {got:?} expected exact {expected:?}",
            );
        }
    }
}

#[test]
fn d38_non_perfect_match_mpmath_oracle() {
    // Expected raws: Python mpmath 1.4.1, mp.dps=160,
    // nint(sqrt(a²+b²) · 10^SCALE).
    // SCALE 6:
    //   hypot(1,1)     = 1414214
    //   hypot(2,3)     = 3605551
    //   hypot(123,456) = 472297576
    let s6: [(i64, i64, i128); 3] = [(1, 1, 1414214), (2, 3, 3605551), (123, 456, 472297576)];
    for (a, b, raw) in s6 {
        let got = D38::<6>::try_from(a)
            .unwrap()
            .hypot_strict_with(D38::<6>::try_from(b).unwrap(), RoundingMode::HalfToEven);
        let expected = D38::<6>::from_bits(Int::<2>::try_from(raw).unwrap());
        assert_eq!(got, expected, "D38<6> hypot({a},{b}) vs mpmath oracle");
    }

    // SCALE 19:
    //   hypot(1,1)     = 14142135623730950488
    //   hypot(2,3)     = 36055512754639892931
    //   hypot(123,456) = 4722975756871932113221
    let s19: [(i64, i64, &str); 3] = [
        (1, 1, "14142135623730950488"),
        (2, 3, "36055512754639892931"),
        (123, 456, "4722975756871932113221"),
    ];
    for (a, b, raw) in s19 {
        let got = D38::<19>::try_from(a)
            .unwrap()
            .hypot_strict_with(D38::<19>::try_from(b).unwrap(), RoundingMode::HalfToEven);
        let expected = D38::<19>::from_bits(raw.parse::<Int<2>>().unwrap());
        assert_eq!(got, expected, "D38<19> hypot({a},{b}) vs mpmath oracle");
    }
}

// ── D57 (wide, Int<3>) ────────────────────────────────────────────────

#[test]
fn d57_pythagorean_triples_exact_all_modes_two_scales() {
    // SCALE 6
    for &(a, b, h) in &TRIPLES {
        let da: D57<6> = D38::<6>::try_from(a).unwrap().into();
        let db: D57<6> = D38::<6>::try_from(b).unwrap().into();
        let expected: D57<6> = D38::<6>::try_from(h).unwrap().into();
        for mode in ALL_MODES {
            let got = da.hypot_strict_with(db, mode);
            assert_eq!(
                got, expected,
                "D57<6> hypot({a},{b}) mode {mode:?}: got {got:?} expected exact {expected:?}",
            );
        }
    }
    // SCALE 30 (wide-tier scale beyond D38's range)
    for &(a, b, h) in &TRIPLES {
        let da = D57::<30>::try_from(a).unwrap();
        let db = D57::<30>::try_from(b).unwrap();
        let expected = D57::<30>::try_from(h).unwrap();
        for mode in ALL_MODES {
            let got = da.hypot_strict_with(db, mode);
            assert_eq!(
                got, expected,
                "D57<30> hypot({a},{b}) mode {mode:?}: got {got:?} expected exact {expected:?}",
            );
        }
    }
}

#[test]
fn d57_non_perfect_match_mpmath_oracle() {
    // Expected raws: Python mpmath 1.4.1, mp.dps=160,
    // nint(sqrt(a²+b²) · 10^30).
    //   hypot(1,1)     = 1414213562373095048801688724210
    //   hypot(2,3)     = 3605551275463989293119221267470
    //   hypot(123,456) = 472297575687193211322083847575167
    let s30: [(i64, i64, &str); 3] = [
        (1, 1, "1414213562373095048801688724210"),
        (2, 3, "3605551275463989293119221267470"),
        (123, 456, "472297575687193211322083847575167"),
    ];
    for (a, b, raw) in s30 {
        let got = D57::<30>::try_from(a)
            .unwrap()
            .hypot_strict_with(D57::<30>::try_from(b).unwrap(), RoundingMode::HalfToEven);
        let expected = D57::<30>::from_bits(raw.parse::<Int<3>>().unwrap());
        assert_eq!(got, expected, "D57<30> hypot({a},{b}) vs mpmath oracle");
    }
}
