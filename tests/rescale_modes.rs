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

#[test]
fn d18_rescale_with_all_modes() {
    type D18_4 = D18<4>;
    type D18_2 = D18<2>;
    let v = D18_4::from_bits(15050);
    let neg = D18_4::from_bits(-15050);
    for m in ALL_MODES {
        let r: D18_2 = v.rescale_with::<2>(m);
        let _ = r;
        let r: D18_2 = neg.rescale_with::<2>(m);
        let _ = r;
    }
    // Identity scale
    let r: D18_4 = v.rescale_with::<4>(RoundingMode::HalfToEven);
    assert_eq!(r.to_bits(), 15050);
}

#[test]
fn d38_rescale_with_all_modes() {
    type D38_4 = D38<4>;
    type D38_2 = D38<2>;
    let v = D38_4::from_bits(decimal_scaled::Int::<2>::from_i128(15050));
    let neg = D38_4::from_bits(decimal_scaled::Int::<2>::from_i128(-15050));
    for m in ALL_MODES {
        let r: D38_2 = v.rescale_with::<2>(m);
        let _ = r;
        let r: D38_2 = neg.rescale_with::<2>(m);
        let _ = r;
    }
    // Identity scale
    let r: D38_4 = v.rescale_with::<4>(RoundingMode::HalfToEven);
    assert_eq!(r.to_bits(), 15050);
}

// ─── Wide-tier rescale ─────────────────────────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn d76_rescale_with_all_modes() {
    use decimal_scaled::D76;
    type D76_4 = D76<4>;
    type D76_2 = D76<2>;
    let v: D76_4 = D38::<4>::from_bits(decimal_scaled::Int::<2>::from_i128(15050)).into();
    let neg: D76_4 = D38::<4>::from_bits(decimal_scaled::Int::<2>::from_i128(-15050)).into();
    for m in ALL_MODES {
        let r: D76_2 = v.rescale_with::<2>(m);
        let _ = r;
        let r: D76_2 = neg.rescale_with::<2>(m);
        let _ = r;
    }
    // Identity scale
    let r: D76_4 = v.rescale_with::<4>(RoundingMode::HalfToEven);
    assert_eq!(r, v);
    // with_scale path
    let _: D76_2 = v.with_scale::<2>();
}

#[cfg(feature = "wide")]
#[test]
#[should_panic(expected = "rescale: scale-up overflow")]
fn d76_rescale_up_overflow_panics() {
    use decimal_scaled::D76;
    let v = D76::<0>::MAX;
    let _: D76<75> = v.rescale::<75>();
}
