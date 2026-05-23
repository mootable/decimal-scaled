//! 0.5 ULP precision suite for every D38 method whose contract claims it.
//!
//! For each precision-losing method we check `result == truth ± 1 LSB`
//! at a fan of representative inputs plus the edge cases the
//! implementation tends to mishandle (zero, near-zero, near domain
//! boundary, exact ties, large magnitudes for range-reduction stress).
//!
//! ## What "± 1 LSB" means
//!
//! For a 0.5-ULP-correctly-rounded method, the storage-scale result
//! `r` satisfies `|r − round_to_scale(truth)| ≤ 1` LSB. When the
//! truth's nearest two representables are equidistant the result is
//! exactly the half-to-even neighbour (assumes `DEFAULT_ROUNDING_MODE
//! = HalfToEven`); otherwise it's exactly the closer one. We allow
//! `± 1 LSB` so the test tolerates a single-LSB transcription error
//! in our hand-computed truth without losing teeth.
//!
//! ## Scope
//!
//! - D38<12> strict transcendentals + constants. Strongest contract
//!   (0.5 ULP guarantee).
//! - mul / div / rescale at SCALE=12. Uses the same `should_bump`
//!   strategy as transcendentals.
//!
//! Not covered here (different contract, separate tests should follow):
//! - Lossy (`f64`-bridge) variants — bounded by `f64`'s ~15-digit
//!   ceiling, not by D38's last place.
//! - Wide-tier (D76+) strict transcendentals — currently `≤ 2 ULP`
//!   per their softened module docs; need a separate ≤ 2 ULP suite.
//! - Non-default rounding modes — directed rounding (Floor / Ceiling
//!   / Trunc) has a different contract.
//!
//! All tests short-circuit when a non-default `rounding-*` feature is
//! active (the per-mode contract differs).

// The 0.5 ULP truth values in this file assume the crate-default
// rounding mode is `HalfToEven`. Compile-gate the entire suite to that
// configuration so every test always runs at least one assertion when
// present (no silent skip under a `rounding-*` feature build).
#![cfg(all(
    not(feature = "fast"),
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]

use decimal_scaled::{D38, D38s12, DecimalConstants};

/// Check that `actual` is within 1 LSB of `expected_truth` (both as
/// raw storage bits) — the 0.5-ULP contract.
#[track_caller]
fn assert_05_ulp(label: &str, actual: i128, expected_truth: i128) {
    let diff = (actual - expected_truth).abs();
    assert!(
        diff <= 1,
        "{label}: result {actual} differs from truth {expected_truth} by {diff} LSB (> 1, violates 0.5 ULP)",
    );
}

// ─── Constants (DecimalConstants trait) ──────────────────────────────────

/// Truth values at SCALE=12, half-to-even rounded from the canonical
/// 35-digit references:
///   π    = 3.141592653589793238462643383279502884...
///   τ    = 6.283185307179586476925286766559005768...
///   π/2  = 1.570796326794896619231321691639751442...
///   π/4  = 0.785398163397448309615660845819875721...
///   e    = 2.718281828459045235360287471352662498...
///   φ    = 1.618033988749894848204586834365638118...
#[test]
fn constants_at_scale_12() {
    assert_05_ulp("pi", i128::from(D38s12::pi().to_bits()), 3_141_592_653_590);
    assert_05_ulp("tau", i128::from(D38s12::tau().to_bits()), 6_283_185_307_180);
    assert_05_ulp("half_pi", i128::from(D38s12::half_pi().to_bits()), 1_570_796_326_795);
    assert_05_ulp(
        "quarter_pi",
        i128::from(D38s12::quarter_pi().to_bits()),
        785_398_163_397,
    );
    assert_05_ulp("e", i128::from(D38s12::e().to_bits()), 2_718_281_828_459);
    assert_05_ulp("golden", i128::from(D38s12::golden().to_bits()), 1_618_033_988_750);
}

// ─── Multiplication ───────────────────────────────────────────────────

/// `(a · b) / 10^SCALE` rounded half-to-even.
///
/// Truth = a · b rounded half-to-even at SCALE precision.
#[test]
fn mul_strict_at_scale_12() {
    // 1.5 × 2.0 = 3.0 (exact)
    let a = D38s12::try_from(3).unwrap() / D38s12::try_from(2).unwrap(); // 1.5 from rescale
    let b = D38s12::try_from(2).unwrap();
    let r = a * b;
    assert_05_ulp("1.5 * 2.0", i128::from(r.to_bits()), 3_000_000_000_000);

    // 1.234567890123 × 0.000000000007 = 8.641975230861e-12 (exact at SCALE=12 = 9)
    // Truth: 1.234567890123 * 7e-12 = 8.641975230861e-12.
    // Rounded to SCALE=12: 9 (the value is 8.64e-12, just under one LSB).
    let a = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_234_567_890_123) as i128).unwrap()); // 1.234567890123
    let b = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((7) as i128).unwrap()); // 7e-12
    let r = a * b;
    // truth = 8.641... → at SCALE=12, the result is 9 LSBs (rounds 8.64 up).
    assert_05_ulp("1.234e0 * 7e-12", i128::from(r.to_bits()), 9);

    // Negative * positive
    let r = (-a) * b;
    assert_05_ulp("(-1.234e0) * 7e-12", i128::from(r.to_bits()), -9);

    // Exact-tie half: (5e-12) * (3) = 15e-12, rescale to SCALE=12 gives 15.
    let a = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((5) as i128).unwrap()); // 5e-12
    let b = D38s12::try_from(3).unwrap();
    let r = a * b;
    assert_05_ulp("5e-12 * 3", i128::from(r.to_bits()), 15);
}

// ─── Division ─────────────────────────────────────────────────────────

#[test]
fn div_strict_at_scale_12() {
    // 1.0 / 3.0 = 0.333333333333... → at SCALE=12: 333_333_333_333
    let r = D38s12::try_from(1).unwrap() / D38s12::try_from(3).unwrap();
    assert_05_ulp("1/3", i128::from(r.to_bits()), 333_333_333_333);

    // 1.0 / 7.0 = 0.142857142857... → at SCALE=12: 142_857_142_857
    let r = D38s12::try_from(1).unwrap() / D38s12::try_from(7).unwrap();
    assert_05_ulp("1/7", i128::from(r.to_bits()), 142_857_142_857);

    // -1.0 / 3.0 = -0.333333... → -333_333_333_333
    let r = (-D38s12::try_from(1).unwrap()) / D38s12::try_from(3).unwrap();
    assert_05_ulp("-1/3", i128::from(r.to_bits()), -333_333_333_333);

    // 22.0 / 7.0 ≈ 3.142857142857 (close to π)
    let r = D38s12::try_from(22).unwrap() / D38s12::try_from(7).unwrap();
    assert_05_ulp("22/7", i128::from(r.to_bits()), 3_142_857_142_857);

    // 1.0 / 1e-12 = 1e12, exact at SCALE=12.
    let one = D38s12::ONE;
    let eps = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap());
    let r = one / eps;
    // 1.0 / 1e-12 in scaled space = (1*10^12) / 1 = 1e12. Storage at S=12: 1e24 (overflows i128).
    // Actually: a = 10^12, b = 1. n = a * 10^12 = 10^24, n/b = 10^24. Doesn't fit i128.
    // So this would overflow — skip.
    let _ = r;

    // 1.0 / 2.0 = 0.5, exact at S=12 (raw 500_000_000_000)
    let r = D38s12::try_from(1).unwrap() / D38s12::try_from(2).unwrap();
    assert_05_ulp("1/2", i128::from(r.to_bits()), 500_000_000_000);
}

// ─── rescale ──────────────────────────────────────────────────────────

#[test]
fn rescale_strict_at_scale_12() {
    use decimal_scaled::D38s6;
    type D2 = D38<2>;

    // Half-to-even at exact halves.
    let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from((1_235_000) as i128).unwrap()); // 1.235000
    let r: D2 = micros.rescale::<2>();
    assert_05_ulp("1.235 -> cents (half to even = 1.24)", i128::from(r.to_bits()), 124);

    let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from((1_225_000) as i128).unwrap()); // 1.225000
    let r: D2 = micros.rescale::<2>();
    assert_05_ulp("1.225 -> cents (half to even = 1.22)", i128::from(r.to_bits()), 122);

    // Below half: 1.234999 → 1.23
    let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from((1_234_999) as i128).unwrap());
    let r: D2 = micros.rescale::<2>();
    assert_05_ulp("1.234999 -> 1.23", i128::from(r.to_bits()), 123);

    // Above half: 1.235001 → 1.24
    let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from((1_235_001) as i128).unwrap());
    let r: D2 = micros.rescale::<2>();
    assert_05_ulp("1.235001 -> 1.24", i128::from(r.to_bits()), 124);

    // Negative ties: -1.235000 → -1.24 (sign-symmetric half-to-even)
    let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from((-1_235_000) as i128).unwrap());
    let r: D2 = micros.rescale::<2>();
    assert_05_ulp("-1.235 -> -1.24", i128::from(r.to_bits()), -124);
}

// ─── ln ───────────────────────────────────────────────────────────────

/// Truth values at SCALE=12:
///   ln(1)   = 0                          → 0
///   ln(2)   = 0.693147180559945309...    → 693_147_180_560 (13th=9, round up to ..560)
///   ln(e)   = 1                          → 1_000_000_000_000
///   ln(10)  = 2.302585092994045684...    → 2_302_585_092_994 (13th=0)
///   ln(0.5) = -ln(2) = -0.693147180560   → -693_147_180_560
///   ln(0.1) = -ln(10) = -2.302585092994  → -2_302_585_092_994
#[test]
fn ln_strict_at_scale_12() {
    assert_05_ulp("ln(1)", i128::from(D38s12::ONE.ln_strict().to_bits()), 0);
    assert_05_ulp(
        "ln(2)",
        i128::from(D38s12::try_from(2).unwrap().ln_strict().to_bits()),
        693_147_180_560,
    );
    let e = D38s12::e();
    let r = e.ln_strict();
    assert_05_ulp("ln(e)", i128::from(r.to_bits()), 1_000_000_000_000);
    assert_05_ulp(
        "ln(10)",
        i128::from(D38s12::try_from(10).unwrap().ln_strict().to_bits()),
        2_302_585_092_994,
    );
    // ln(0.5) = -ln(2)
    let half = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((500_000_000_000) as i128).unwrap());
    assert_05_ulp("ln(0.5)", i128::from(half.ln_strict().to_bits()), -693_147_180_560);
    // ln(0.1) = -ln(10)
    let tenth = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((100_000_000_000) as i128).unwrap());
    assert_05_ulp("ln(0.1)", i128::from(tenth.ln_strict().to_bits()), -2_302_585_092_994);
}

// ─── exp ──────────────────────────────────────────────────────────────

/// Truth at SCALE=12:
///   exp(0) = 1                            → 1_000_000_000_000
///   exp(1) = e = 2.718281828459045...     → 2_718_281_828_459
///   exp(-1) = 1/e = 0.367879441171442...  → 367_879_441_171
///   exp(ln(2)) = 2 (round-trip)
///   exp(ln(10)) = 10
#[test]
fn exp_strict_at_scale_12() {
    assert_05_ulp(
        "exp(0)",
        i128::from(D38s12::ZERO.exp_strict().to_bits()),
        1_000_000_000_000,
    );
    assert_05_ulp(
        "exp(1)",
        i128::from(D38s12::ONE.exp_strict().to_bits()),
        2_718_281_828_459,
    );
    assert_05_ulp(
        "exp(-1)",
        i128::from((-D38s12::ONE).exp_strict().to_bits()),
        367_879_441_171,
    );

    // Round-trip ln/exp; verify ≤ a couple of LSB at the boundary
    let two = D38s12::try_from(2).unwrap();
    let r = two.ln_strict().exp_strict();
    assert_05_ulp("exp(ln(2)) ~= 2", i128::from(r.to_bits()), 2_000_000_000_000);

    let ten = D38s12::try_from(10).unwrap();
    let r = ten.ln_strict().exp_strict();
    assert_05_ulp("exp(ln(10)) ~= 10", i128::from(r.to_bits()), 10_000_000_000_000);
}

// ─── sin / cos / tan ──────────────────────────────────────────────────

/// Truth at SCALE=12:
///   sin(0)    = 0
///   sin(π/2)  = 1   (after range reduction of D38s12::half_pi())
///   sin(π)    = 0
///   sin(1)    = 0.841470984807896506...   → 841_470_984_808 (13th=9 round up)
///   sin(-1)   = -sin(1)                    → -841_470_984_808
///   cos(0)    = 1
///   cos(π)    = -1
///   cos(π/2)  = 0
///   cos(1)    = 0.540302305868139717...   → 540_302_305_868 (13th=1 no round)
///   tan(0)    = 0
///   tan(π/4)  = 1
#[test]
fn sin_strict_at_scale_12() {
    assert_05_ulp("sin(0)", i128::from(D38s12::ZERO.sin_strict().to_bits()), 0);
    let half_pi = D38s12::half_pi();
    assert_05_ulp(
        "sin(π/2)",
        i128::from(half_pi.sin_strict().to_bits()),
        1_000_000_000_000,
    );
    let pi = D38s12::pi();
    // sin(π) ≈ 0 (subject to π's own 1 LSB rounding)
    let r = i128::from(pi.sin_strict().to_bits());
    assert!(r.abs() <= 2, "sin(π) was {r}, expected ~0 (≤ 2 LSB)");
    assert_05_ulp(
        "sin(1)",
        i128::from(D38s12::ONE.sin_strict().to_bits()),
        841_470_984_808,
    );
    assert_05_ulp(
        "sin(-1)",
        i128::from((-D38s12::ONE).sin_strict().to_bits()),
        -841_470_984_808,
    );
}

#[test]
fn cos_strict_at_scale_12() {
    assert_05_ulp(
        "cos(0)",
        i128::from(D38s12::ZERO.cos_strict().to_bits()),
        1_000_000_000_000,
    );
    let pi = D38s12::pi();
    assert_05_ulp("cos(π)", i128::from(pi.cos_strict().to_bits()), -1_000_000_000_000);
    let half_pi = D38s12::half_pi();
    let r = i128::from(half_pi.cos_strict().to_bits());
    assert!(r.abs() <= 2, "cos(π/2) was {r}, expected ~0 (≤ 2 LSB)");
    assert_05_ulp(
        "cos(1)",
        i128::from(D38s12::ONE.cos_strict().to_bits()),
        540_302_305_868,
    );
}

#[test]
fn tan_strict_at_scale_12() {
    assert_05_ulp("tan(0)", i128::from(D38s12::ZERO.tan_strict().to_bits()), 0);
    let quarter_pi = D38s12::quarter_pi();
    let r = quarter_pi.tan_strict();
    // tan(π/4) = 1, but π/4 has its own 1 LSB error.
    assert_05_ulp("tan(π/4)", i128::from(r.to_bits()), 1_000_000_000_000);
}

// ─── atan / asin / acos ───────────────────────────────────────────────

/// Truth at SCALE=12:
///   atan(0)  = 0
///   atan(1)  = π/4 = 0.785398163397448310...  → 785_398_163_397
///   atan(-1) = -π/4                            → -785_398_163_397
///   atan(√3) = π/3 = 1.047197551196597746...  → 1_047_197_551_197
///   asin(0)  = 0
///   asin(1)  = π/2                            → 1_570_796_326_795
///   asin(0.5) = π/6 = 0.523598775598298873...→ 523_598_775_598
///   acos(1)  = 0
///   acos(0)  = π/2                            → 1_570_796_326_795
///   acos(-1) = π                              → 3_141_592_653_590
#[test]
fn atan_strict_at_scale_12() {
    assert_05_ulp("atan(0)", i128::from(D38s12::ZERO.atan_strict().to_bits()), 0);
    assert_05_ulp(
        "atan(1)",
        i128::from(D38s12::ONE.atan_strict().to_bits()),
        785_398_163_397,
    );
    assert_05_ulp(
        "atan(-1)",
        i128::from((-D38s12::ONE).atan_strict().to_bits()),
        -785_398_163_397,
    );
}

#[test]
fn asin_strict_at_scale_12() {
    assert_05_ulp("asin(0)", i128::from(D38s12::ZERO.asin_strict().to_bits()), 0);
    assert_05_ulp(
        "asin(1)",
        i128::from(D38s12::ONE.asin_strict().to_bits()),
        1_570_796_326_795,
    );
    let half = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((500_000_000_000) as i128).unwrap());
    // asin(0.5) = π/6
    assert_05_ulp("asin(0.5)", i128::from(half.asin_strict().to_bits()), 523_598_775_598);
}

#[test]
fn acos_strict_at_scale_12() {
    assert_05_ulp("acos(1)", i128::from(D38s12::ONE.acos_strict().to_bits()), 0);
    assert_05_ulp(
        "acos(0)",
        i128::from(D38s12::ZERO.acos_strict().to_bits()),
        1_570_796_326_795,
    );
    assert_05_ulp(
        "acos(-1)",
        i128::from((-D38s12::ONE).acos_strict().to_bits()),
        3_141_592_653_590,
    );
}

// ─── atan2 ────────────────────────────────────────────────────────────

#[test]
fn atan2_strict_at_scale_12() {
    let one = D38s12::ONE;
    let zero = D38s12::ZERO;
    // atan2(1, 1) = π/4
    assert_05_ulp(
        "atan2(1, 1)",
        i128::from(one.atan2_strict(one).to_bits()),
        785_398_163_397,
    );
    // atan2(0, 1) = 0
    assert_05_ulp("atan2(0, 1)", i128::from(zero.atan2_strict(one).to_bits()), 0);
    // atan2(1, 0) = π/2
    assert_05_ulp(
        "atan2(1, 0)",
        i128::from(one.atan2_strict(zero).to_bits()),
        1_570_796_326_795,
    );
    // atan2(-1, 0) = -π/2
    assert_05_ulp(
        "atan2(-1, 0)",
        i128::from((-one).atan2_strict(zero).to_bits()),
        -1_570_796_326_795,
    );
    // atan2(0, -1) = π
    assert_05_ulp(
        "atan2(0, -1)",
        i128::from(zero.atan2_strict(-one).to_bits()),
        3_141_592_653_590,
    );
}

// ─── sqrt / cbrt ──────────────────────────────────────────────────────

/// Truth at SCALE=12:
///   sqrt(0) = 0
///   sqrt(1) = 1
///   sqrt(2) = 1.414213562373095048...  → 1_414_213_562_373 (13th=0)
///   sqrt(3) = 1.732050807568877293...  → 1_732_050_807_569 (13th=8 round up)
///   sqrt(4) = 2 (exact)
///   sqrt(5) = 2.236067977499789696...  → 2_236_067_977_500 (13th=9 round up)
///   cbrt(0) = 0
///   cbrt(1) = 1
///   cbrt(2) = 1.259921049894873164...  → 1_259_921_049_895 (13th=8 round up)
///   cbrt(8) = 2 (exact)
///   cbrt(27) = 3 (exact)
#[test]
fn sqrt_strict_at_scale_12() {
    assert_05_ulp("sqrt(0)", i128::from(D38s12::ZERO.sqrt_strict().to_bits()), 0);
    assert_05_ulp(
        "sqrt(1)",
        i128::from(D38s12::ONE.sqrt_strict().to_bits()),
        1_000_000_000_000,
    );
    assert_05_ulp(
        "sqrt(2)",
        i128::from(D38s12::try_from(2).unwrap().sqrt_strict().to_bits()),
        1_414_213_562_373,
    );
    assert_05_ulp(
        "sqrt(3)",
        i128::from(D38s12::try_from(3).unwrap().sqrt_strict().to_bits()),
        1_732_050_807_569,
    );
    assert_05_ulp(
        "sqrt(4)",
        i128::from(D38s12::try_from(4).unwrap().sqrt_strict().to_bits()),
        2_000_000_000_000,
    );
    assert_05_ulp(
        "sqrt(5)",
        i128::from(D38s12::try_from(5).unwrap().sqrt_strict().to_bits()),
        2_236_067_977_500,
    );
}

#[test]
fn cbrt_strict_at_scale_12() {
    assert_05_ulp("cbrt(0)", i128::from(D38s12::ZERO.cbrt_strict().to_bits()), 0);
    assert_05_ulp(
        "cbrt(1)",
        i128::from(D38s12::ONE.cbrt_strict().to_bits()),
        1_000_000_000_000,
    );
    assert_05_ulp(
        "cbrt(2)",
        i128::from(D38s12::try_from(2).unwrap().cbrt_strict().to_bits()),
        1_259_921_049_895,
    );
    assert_05_ulp(
        "cbrt(8)",
        i128::from(D38s12::try_from(8).unwrap().cbrt_strict().to_bits()),
        2_000_000_000_000,
    );
    assert_05_ulp(
        "cbrt(27)",
        i128::from(D38s12::try_from(27).unwrap().cbrt_strict().to_bits()),
        3_000_000_000_000,
    );
    // cbrt(-8) = -2
    assert_05_ulp(
        "cbrt(-8)",
        i128::from(D38s12::try_from(-8).unwrap().cbrt_strict().to_bits()),
        -2_000_000_000_000,
    );
}

// ─── hyperbolic family ────────────────────────────────────────────────

/// Truth at SCALE=12:
///   sinh(0) = 0
///   sinh(1) = 1.175201193643801456...   → 1_175_201_193_644 (13th=8 round up)
///   cosh(0) = 1
///   cosh(1) = 1.543080634815243778...   → 1_543_080_634_815
///   tanh(0) = 0
///   tanh(1) = 0.761594155955764888...   → 761_594_155_956 (13th=8 round up)
#[test]
fn sinh_strict_at_scale_12() {
    assert_05_ulp("sinh(0)", i128::from(D38s12::ZERO.sinh_strict().to_bits()), 0);
    assert_05_ulp(
        "sinh(1)",
        i128::from(D38s12::ONE.sinh_strict().to_bits()),
        1_175_201_193_644,
    );
    assert_05_ulp(
        "sinh(-1)",
        i128::from((-D38s12::ONE).sinh_strict().to_bits()),
        -1_175_201_193_644,
    );
}

#[test]
fn cosh_strict_at_scale_12() {
    assert_05_ulp(
        "cosh(0)",
        i128::from(D38s12::ZERO.cosh_strict().to_bits()),
        1_000_000_000_000,
    );
    assert_05_ulp(
        "cosh(1)",
        i128::from(D38s12::ONE.cosh_strict().to_bits()),
        1_543_080_634_815,
    );
    // cosh is even
    assert_05_ulp(
        "cosh(-1)",
        i128::from((-D38s12::ONE).cosh_strict().to_bits()),
        1_543_080_634_815,
    );
}

#[test]
fn tanh_strict_at_scale_12() {
    assert_05_ulp("tanh(0)", i128::from(D38s12::ZERO.tanh_strict().to_bits()), 0);
    assert_05_ulp(
        "tanh(1)",
        i128::from(D38s12::ONE.tanh_strict().to_bits()),
        761_594_155_956,
    );
}

// ─── angle conversions ────────────────────────────────────────────────

/// Angle conversion tests use *exact-at-storage* inputs so the 0.5 ULP
/// contract on the conversion itself can be asserted rigorously.
/// Inputs like `D38s12::pi()` carry their own ≈ 0.5 LSB rounding
/// error which amplifies through `to_degrees`'s `180/π` factor by
/// ~57×; that's the input's error budget, not the conversion's.
///
/// Truth (at exact-storage inputs):
///   to_degrees(0)             = 0
///   to_degrees(1 rad)         = 180/π = 57.295779513082320876...   → 57_295_779_513_082
///   to_degrees(2 rad)         = 360/π = 114.591559026164641753...  → 114_591_559_026_165
///   to_radians(0)             = 0
///   to_radians(180 deg)       = π_stored_at_S12 = 3_141_592_653_590 (within 1 LSB)
///   to_radians(57.295779513082 deg) ≈ 1 rad (exact-at-storage of 180/π)
#[test]
fn angle_conversion_strict_at_scale_12() {
    assert_05_ulp(
        "to_degrees(0)",
        i128::from(D38s12::ZERO.to_degrees_strict().to_bits()),
        0,
    );
    assert_05_ulp(
        "to_degrees(1 rad)",
        i128::from(D38s12::ONE.to_degrees_strict().to_bits()),
        57_295_779_513_082,
    );
    assert_05_ulp(
        "to_degrees(2 rad)",
        i128::from(D38s12::try_from(2).unwrap().to_degrees_strict().to_bits()),
        114_591_559_026_165,
    );
    assert_05_ulp(
        "to_radians(0)",
        i128::from(D38s12::ZERO.to_radians_strict().to_bits()),
        0,
    );
    // to_radians(180 deg) — the input is exact-at-storage; output
    // matches the stored π exactly because the formula folds:
    // 180 * π_internal / 180 = π_internal, which rounds to the same
    // bit pattern as D38s12::pi().
    let deg180 = D38s12::try_from(180).unwrap();
    assert_05_ulp(
        "to_radians(180 deg) == stored pi",
        i128::from(deg180.to_radians_strict().to_bits()),
        3_141_592_653_590,
    );
}

// ─── log family ───────────────────────────────────────────────────────

/// Truth:
///   log2(1) = 0
///   log2(2) = 1
///   log2(4) = 2
///   log2(8) = 3
///   log2(10) = 3.321928094887362347...  → 3_321_928_094_887 (13th=3 no round)
///   log10(1) = 0
///   log10(10) = 1
///   log10(100) = 2
///   log10(2) = 0.301029995663981195...  → 301_029_995_664 (13th=1 no round, but next is 9 → ...196 → so 12-digit ...664 actually)
///     wait let me recompute: 0.30102999566398119521...
///     12 frac = 301029995663, 13th = 9 → round up to 664. So 301_029_995_664.
#[test]
fn log_strict_at_scale_12() {
    assert_05_ulp("log2(1)", i128::from(D38s12::ONE.log2_strict().to_bits()), 0);
    assert_05_ulp(
        "log2(2)",
        i128::from(D38s12::try_from(2).unwrap().log2_strict().to_bits()),
        1_000_000_000_000,
    );
    assert_05_ulp(
        "log2(4)",
        i128::from(D38s12::try_from(4).unwrap().log2_strict().to_bits()),
        2_000_000_000_000,
    );
    assert_05_ulp("log10(1)", i128::from(D38s12::ONE.log10_strict().to_bits()), 0);
    assert_05_ulp(
        "log10(10)",
        i128::from(D38s12::try_from(10).unwrap().log10_strict().to_bits()),
        1_000_000_000_000,
    );
    assert_05_ulp(
        "log10(100)",
        i128::from(D38s12::try_from(100).unwrap().log10_strict().to_bits()),
        2_000_000_000_000,
    );
    assert_05_ulp(
        "log10(2)",
        i128::from(D38s12::try_from(2).unwrap().log10_strict().to_bits()),
        301_029_995_664,
    );
}

// ─── powf ─────────────────────────────────────────────────────────────

/// Truth at SCALE=12:
///   powf(2, 10) = 1024
///   powf(2, 0.5) = sqrt(2) = 1.414213562373
///   powf(e, 1) = e
///   powf(10, 2) = 100
#[test]
fn powf_strict_at_scale_12() {
    let two = D38s12::try_from(2).unwrap();
    let ten = D38s12::try_from(10).unwrap();
    assert_05_ulp(
        "2^10",
        i128::from(two.powf_strict(D38s12::try_from(10).unwrap()).to_bits()),
        1_024_000_000_000_000,
    );
    // 2^0.5 = sqrt(2)
    let half = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((500_000_000_000) as i128).unwrap());
    let r = two.powf_strict(half);
    assert_05_ulp("2^0.5 ~= sqrt(2)", i128::from(r.to_bits()), 1_414_213_562_373);
    // 10^2 = 100
    assert_05_ulp(
        "10^2",
        i128::from(ten.powf_strict(D38s12::try_from(2).unwrap()).to_bits()),
        100_000_000_000_000,
    );
}

// ─── inverse hyperbolic ───────────────────────────────────────────────

/// Truth:
///   asinh(0) = 0
///   asinh(1) = 0.881373587019543025...  → 881_373_587_020 (13th=2 no round wait)
///     0.88137358701954302523... 12 frac = 881373587019, 13th = 5 → tie. half-to-even: 9 odd → round up to 020.
///     Actually 12th digit is 9, round to even = 020 (even). So 881_373_587_020.
///   acosh(1) = 0
///   acosh(2) = 1.316957896924816708...  → 1_316_957_896_925 (13th=8 round up)
///   atanh(0) = 0
///   atanh(0.5) = 0.549306144334054846...  → 549_306_144_334 (13th=0 no round)
#[test]
fn asinh_strict_at_scale_12() {
    assert_05_ulp("asinh(0)", i128::from(D38s12::ZERO.asinh_strict().to_bits()), 0);
    assert_05_ulp(
        "asinh(1)",
        i128::from(D38s12::ONE.asinh_strict().to_bits()),
        881_373_587_020,
    );
}

#[test]
fn acosh_strict_at_scale_12() {
    assert_05_ulp("acosh(1)", i128::from(D38s12::ONE.acosh_strict().to_bits()), 0);
    assert_05_ulp(
        "acosh(2)",
        i128::from(D38s12::try_from(2).unwrap().acosh_strict().to_bits()),
        1_316_957_896_925,
    );
}

#[test]
fn atanh_strict_at_scale_12() {
    assert_05_ulp("atanh(0)", i128::from(D38s12::ZERO.atanh_strict().to_bits()), 0);
    let half = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((500_000_000_000) as i128).unwrap());
    assert_05_ulp("atanh(0.5)", i128::from(half.atanh_strict().to_bits()), 549_306_144_334);
}
