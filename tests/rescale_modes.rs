//! Coverage for `macros/rescale.rs` — `rescale_with(mode)` on every
//! decimal width × every rounding mode, plus the scale-up overflow panic
//! path on D9 (the easiest tier to overflow at scale-up).

use decimal_scaled::{D18, D38, RoundingMode};

const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

// `1.5050` rescaled 4→2 leaves a residual of exactly HALF (kept `1.50`, rest
// `50`), so each mode's tie rule decides the last digit — the per-mode contract
// these raw expectations pin (kept digits 150, the bump per mode):
const TIE_POS: [i64; 6] = [150, 151, 150, 150, 150, 151];
const TIE_NEG: [i64; 6] = [-150, -151, -150, -150, -151, -150];

#[test]
fn d18_rescale_with_all_modes() {

    let v = D18::<4>::from_bits(decimal_scaled::Int::<1>::from(15050_i64));
    let neg = D18::<4>::from_bits(decimal_scaled::Int::<1>::from(-15050_i64));
    for (i, m) in ALL_MODES.into_iter().enumerate() {
        let r: D18<2> = v.rescale_with::<2>(m);
        assert_eq!(r.to_bits(), decimal_scaled::Int::<1>::from(TIE_POS[i]), "{m:?} +tie");
        let r: D18<2> = neg.rescale_with::<2>(m);
        assert_eq!(r.to_bits(), decimal_scaled::Int::<1>::from(TIE_NEG[i]), "{m:?} -tie");
    }
    // Identity scale
    let r: D18<4> = v.rescale_with::<4>(RoundingMode::HalfToEven);
    assert_eq!(r.to_bits(), 15050);
}

#[test]
fn d38_rescale_with_all_modes() {

    let v = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(15050_i128).unwrap());
    let neg = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(-15050_i128).unwrap());
    for (i, m) in ALL_MODES.into_iter().enumerate() {
        let r: D38<2> = v.rescale_with::<2>(m);
        let want = decimal_scaled::Int::<2>::try_from(i128::from(TIE_POS[i])).unwrap();
        assert_eq!(r.to_bits(), want, "{m:?} +tie");
        let r: D38<2> = neg.rescale_with::<2>(m);
        let want = decimal_scaled::Int::<2>::try_from(i128::from(TIE_NEG[i])).unwrap();
        assert_eq!(r.to_bits(), want, "{m:?} -tie");
    }
    // Identity scale
    let r: D38<4> = v.rescale_with::<4>(RoundingMode::HalfToEven);
    assert_eq!(r.to_bits(), 15050);
}

// ─── Wide-tier rescale ─────────────────────────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn d76_rescale_with_all_modes() {
    use decimal_scaled::D76;

    let v: D76<4> = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(15050_i128).unwrap()).into();
    let neg: D76<4> = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(-15050_i128).unwrap()).into();
    for (i, m) in ALL_MODES.into_iter().enumerate() {
        let r: D76<2> = v.rescale_with::<2>(m);
        let want: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(i128::from(TIE_POS[i])).unwrap()).into();
        assert_eq!(r, want, "{m:?} +tie");
        let r: D76<2> = neg.rescale_with::<2>(m);
        let want: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(i128::from(TIE_NEG[i])).unwrap()).into();
        assert_eq!(r, want, "{m:?} -tie");
    }
    // Identity scale
    let r: D76<4> = v.rescale_with::<4>(RoundingMode::HalfToEven);
    assert_eq!(r, v);
    // with_scale path
    let _: D76<2> = v.with_scale::<2>();
}

#[cfg(feature = "wide")]
#[test]
#[should_panic(expected = "rescale: scale-up overflow")]
fn d76_rescale_up_overflow_panics() {
    use decimal_scaled::D76;
    let v = D76::<0>::MAX;
    let _: D76<75> = v.rescale::<75>();
}
