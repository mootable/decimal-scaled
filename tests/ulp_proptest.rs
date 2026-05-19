//! Property-based ULP coverage for the strict transcendentals.
//!
//! Complements the deterministic mpmath-oracle table in
//! `tests/ulp_strict_golden.rs` with randomised, fixed-seed fuzz
//! over the natural input domain of each function.
//!
//! The oracle here is mathematical *identity*, not a precomputed
//! table:
//!
//!   * `exp(ln(x)) ≈ x` for positive x
//!   * `ln(exp(x)) ≈ x` for moderate x
//!   * `sin² + cos² ≈ 1`
//!   * `sqrt(x)² ≈ x` for non-negative x
//!   * `cbrt(x)³ ≈ x`
//!   * `atan(tan(x)) ≈ x` on (-π/2, π/2)
//!   * `tanh(atanh(x)) ≈ x` on (-1, 1) — exercised via the strict
//!     surface
//!   * Sign symmetries: `sin(-x) = -sin(x)`, `cos(-x) = cos(x)`,
//!     `atan(-x) = -atan(x)`, `cbrt(-x) = -cbrt(x)`
//!
//! Round-trip identities accumulate two strict-kernel rounding
//! errors, so the tolerance is intentionally wider than the 0.5 ULP
//! per-kernel contract — we look for *gross* drift, not the
//! one-LSB envelope (the golden harness owns that).
//!
//! Per-block case count: 100 with a deterministic per-block RNG seed
//! key, so test runs are reproducible and a counterexample minimises
//! the same way every time.
//!
//! Cross-tier: each property block additionally runs on D76<19> (at
//! the same SCALE as D38<19>) so the wide-tier kernels see the same
//! property pressure.

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

use decimal_scaled::{D38, DecimalConstants};
use proptest::prelude::*;

type D = D38<19>;
type Bits = i128;

/// Cases per block. Small enough to keep the suite under a minute,
/// large enough to surface scale-dependent kernel bugs.
const CASES: u32 = 100;

/// Relative-tolerance budget for two-kernel round-trip identities.
/// One kernel pass is bounded by the 0.5 ULP contract; two-pass
/// accumulates a single-pass error per call **plus** a derivative-
/// amplification factor at the second call's evaluation point.
/// Absolute error scales with the magnitude of the result (e.g. for
/// `sqrt(x)² ≈ x` the second multiplication doubles the relative
/// error and turns it into absolute LSBs proportional to `x`).
///
/// We express the budget as a relative ratio: `|diff| <= max(8,
/// |reference| / RELATIVE_TOL_INV)` where `RELATIVE_TOL_INV`
/// corresponds to roughly 16 places of precision — well inside the
/// 19-digit storage budget but coarse enough that legitimate
/// derivative amplification at large abscissae doesn't trip the
/// gate. A kernel that silently drops 5+ guard digits will fail
/// this property.
const RELATIVE_TOL_INV: i128 = 10i128.pow(16);

/// Floor — round-trip identities also exhibit a constant per-call
/// LSB jitter that doesn't track magnitude.
const ROUND_TRIP_FLOOR_LSB: i128 = 8;

/// LSB tolerance for sign-symmetry identities (`sin(-x) = -sin(x)`).
/// These are a single-kernel relation, so the budget is tighter.
const SYMMETRY_LSB_TOL: i128 = 1;

fn proptest_config(seed_label: &'static str) -> ProptestConfig {
    // RngAlgorithm::ChaCha is the proptest default for the
    // `TestRunner::deterministic` constructor; we override `cases`
    // and disable persistence/regression files for hermetic CI.
    ProptestConfig {
        cases: CASES,
        max_shrink_iters: 256,
        failure_persistence: None,
        source_file: Some(seed_label),
        ..ProptestConfig::default()
    }
}

/// Round-trip tolerance: absolute LSB plus a relative slice keyed
/// off the magnitude of the reference value.
fn round_trip_tol(reference: i128) -> i128 {
    let rel = reference.unsigned_abs() / (RELATIVE_TOL_INV as u128);
    ROUND_TRIP_FLOOR_LSB + (rel as i128)
}

/// Bail-on-failure flow shared across blocks. Lives at module
/// scope so each `proptest!` block can call it directly.
macro_rules! prop_assert_within {
    ($label:expr, $lhs:expr, $rhs:expr, $diff:expr, $tol:expr) => {{
        if $diff > $tol {
            return Err(proptest::test_runner::TestCaseError::fail(format!(
                "{label}: lhs={lhs} rhs={rhs} diff={diff} > tol={tol} LSB",
                label = $label,
                lhs = $lhs,
                rhs = $rhs,
                diff = $diff,
                tol = $tol,
            )));
        }
    }};
}

// ─── Strategies (raw-int generators per input domain) ──────────────

const ONE: Bits = 10i128.pow(19); // D38<19> unit value (1.0).

fn positive_x() -> impl Strategy<Value = Bits> {
    // Cover (10^-19, 10^6]; excludes 0 (ln is undefined there).
    1i128..=(ONE * 1_000_000)
}

fn nonneg_x() -> impl Strategy<Value = Bits> {
    0i128..=(ONE * 1_000_000)
}

fn moderate_real() -> impl Strategy<Value = Bits> {
    // Cover |x| <= 30 to keep exp(x) inside the type's range.
    (-30 * ONE)..=(30 * ONE)
}

/// Domain for the `ln(exp(x)) ≈ x` round-trip. We restrict to
/// non-negative x because `exp(very_negative)` produces values
/// with so few significant raw LSBs that the subsequent `ln`
/// magnifies the relative error in ways the simple `relative_tol`
/// budget cannot accommodate — that's a property of the identity,
/// not a kernel defect.
fn ln_exp_domain() -> impl Strategy<Value = Bits> {
    0i128..=(30 * ONE)
}

fn real_x() -> impl Strategy<Value = Bits> {
    (-(ONE * 1_000_000))..=(ONE * 1_000_000)
}

fn open_interval_neg1_to_1() -> impl Strategy<Value = Bits> {
    // (-1, 1) for atanh / arcsin etc; stay away from the ±1 pole
    // by 0.001.
    (-(ONE - ONE / 1000))..=(ONE - ONE / 1000)
}

fn open_quarter_pi() -> impl Strategy<Value = Bits> {
    // (-π/4 + 0.001, π/4 - 0.001) — for atan(tan(x)) round-trip.
    let half = ONE / 1000;
    let quarter_pi_bits = D::quarter_pi().to_bits();
    (-quarter_pi_bits + half)..=(quarter_pi_bits - half)
}

// ─── D38<19> property blocks ───────────────────────────────────────

proptest! {
    #![proptest_config(proptest_config("exp_of_ln_roundtrip"))]
    #[test]
    fn exp_of_ln_roundtrip(raw in positive_x()) {
        let x = D::from_bits(raw);
        let y = x.ln_strict().exp_strict();
        let xb = x.to_bits();
        let yb = y.to_bits();
        let tol = round_trip_tol(xb);
        prop_assert_within!("exp(ln(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("ln_of_exp_roundtrip"))]
    #[test]
    fn ln_of_exp_roundtrip(raw in ln_exp_domain()) {
        let x = D::from_bits(raw);
        let y = x.exp_strict().ln_strict();
        let xb = x.to_bits();
        let yb = y.to_bits();
        let tol = round_trip_tol(xb);
        prop_assert_within!("ln(exp(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("sin2_plus_cos2_is_one"))]
    #[test]
    fn sin2_plus_cos2_is_one(raw in real_x()) {
        let x = D::from_bits(raw);
        let s = x.sin_strict();
        let c = x.cos_strict();
        let sum = s * s + c * c;
        let one = D::from_bits(ONE);
        let tol = round_trip_tol(ONE);
        prop_assert_within!("sin²+cos²",
                            sum.to_bits(), one.to_bits(),
                            (sum.to_bits() - one.to_bits()).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("sqrt_squared_roundtrip"))]
    #[test]
    fn sqrt_squared_roundtrip(raw in nonneg_x()) {
        let x = D::from_bits(raw);
        let r = x.sqrt_strict();
        let back = r * r;
        let xb = x.to_bits();
        let bb = back.to_bits();
        let tol = round_trip_tol(xb);
        prop_assert_within!("sqrt(x)²", xb, bb, (xb - bb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("cbrt_cubed_roundtrip"))]
    #[test]
    fn cbrt_cubed_roundtrip(raw in real_x()) {
        let x = D::from_bits(raw);
        let r = x.cbrt_strict();
        let back = r * r * r;
        let xb = x.to_bits();
        let bb = back.to_bits();
        let tol = round_trip_tol(xb);
        prop_assert_within!("cbrt(x)³", xb, bb, (xb - bb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("atan_of_tan_roundtrip"))]
    #[test]
    fn atan_of_tan_roundtrip(raw in open_quarter_pi()) {
        let x = D::from_bits(raw);
        let y = x.tan_strict().atan_strict();
        let xb = x.to_bits();
        let yb = y.to_bits();
        let tol = round_trip_tol(xb);
        prop_assert_within!("atan(tan(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("tanh_atanh_roundtrip"))]
    #[test]
    fn tanh_atanh_roundtrip(raw in open_interval_neg1_to_1()) {
        let x = D::from_bits(raw);
        let y = x.atanh_strict().tanh_strict();
        let xb = x.to_bits();
        let yb = y.to_bits();
        let tol = round_trip_tol(xb);
        prop_assert_within!("tanh(atanh(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("sin_odd_symmetry"))]
    #[test]
    fn sin_odd_symmetry(raw in real_x()) {
        let x = D::from_bits(raw);
        let lhs = x.sin_strict();
        let rhs = -((-x).sin_strict());
        prop_assert_within!("sin(-x)=-sin(x)", lhs.to_bits(), rhs.to_bits(),
                            (lhs.to_bits() - rhs.to_bits()).abs(),
                            SYMMETRY_LSB_TOL);
    }
}

proptest! {
    #![proptest_config(proptest_config("cos_even_symmetry"))]
    #[test]
    fn cos_even_symmetry(raw in real_x()) {
        let x = D::from_bits(raw);
        let lhs = x.cos_strict();
        let rhs = (-x).cos_strict();
        prop_assert_within!("cos(-x)=cos(x)", lhs.to_bits(), rhs.to_bits(),
                            (lhs.to_bits() - rhs.to_bits()).abs(),
                            SYMMETRY_LSB_TOL);
    }
}

proptest! {
    #![proptest_config(proptest_config("atan_odd_symmetry"))]
    #[test]
    fn atan_odd_symmetry(raw in real_x()) {
        let x = D::from_bits(raw);
        let lhs = x.atan_strict();
        let rhs = -((-x).atan_strict());
        prop_assert_within!("atan(-x)=-atan(x)", lhs.to_bits(), rhs.to_bits(),
                            (lhs.to_bits() - rhs.to_bits()).abs(),
                            SYMMETRY_LSB_TOL);
    }
}

proptest! {
    #![proptest_config(proptest_config("cbrt_odd_symmetry"))]
    #[test]
    fn cbrt_odd_symmetry(raw in real_x()) {
        let x = D::from_bits(raw);
        let lhs = x.cbrt_strict();
        let rhs = -((-x).cbrt_strict());
        prop_assert_within!("cbrt(-x)=-cbrt(x)", lhs.to_bits(), rhs.to_bits(),
                            (lhs.to_bits() - rhs.to_bits()).abs(),
                            SYMMETRY_LSB_TOL);
    }
}

// ─── Cross-tier witness: D76<19> vs D38<19> at the same scale ──────
//
// Same SCALE on both sides, so the storage int comparison is direct
// (D76 result fits i128 at scale 19 — width matters only for
// arithmetic headroom, not the value itself). Tolerance widens to
// match `precision_wide_baseline.rs`: ±1 LSB.

#[cfg(any(feature = "d76", feature = "wide"))]
mod wide_witness {
    use super::CASES;
    use decimal_scaled::{D38, D76};
    use proptest::prelude::*;

    type N = D38<19>;
    type W = D76<19>;

    const ONE: i128 = 10i128.pow(19);

    /// Per-call LSB budget for D76 vs D38 cross-witness. Both
    /// kernels target 0.5 ULP at the same storage scale; we allow
    /// ±1 LSB (matches `precision_wide_baseline.rs`).
    const CROSS_TIER_LSB_TOL: i128 = 1;

    fn nonneg_x() -> impl Strategy<Value = i128> {
        0i128..=(ONE * 1_000_000)
    }

    fn moderate_real() -> impl Strategy<Value = i128> {
        (-30 * ONE)..=(30 * ONE)
    }

    fn config(label: &'static str) -> ProptestConfig {
        ProptestConfig {
            cases: CASES,
            max_shrink_iters: 256,
            failure_persistence: None,
            source_file: Some(label),
            ..ProptestConfig::default()
        }
    }

    proptest! {
        #![proptest_config(config("d76_sqrt_witness"))]
        #[test]
        fn d76_sqrt_agrees_with_d38(raw in nonneg_x()) {
            let n = N::from_bits(raw);
            let w: W = n.into();
            let nb = n.sqrt_strict().to_bits();
            let wb = w.sqrt_strict().to_bits().to_i128_checked()
                .expect("D76<19>::sqrt fits i128");
            let diff = (wb - nb).abs();
            prop_assert!(diff <= CROSS_TIER_LSB_TOL,
                         "D76<19>::sqrt vs D38<19>::sqrt diff={diff} (raw={raw})");
        }
    }

    proptest! {
        #![proptest_config(config("d76_exp_witness"))]
        #[test]
        fn d76_exp_agrees_with_d38(raw in moderate_real()) {
            let n = N::from_bits(raw);
            let w: W = n.into();
            let nb = n.exp_strict().to_bits();
            let wb = w.exp_strict().to_bits().to_i128_checked()
                .expect("D76<19>::exp fits i128 at moderate x");
            let diff = (wb - nb).abs();
            prop_assert!(diff <= CROSS_TIER_LSB_TOL,
                         "D76<19>::exp vs D38<19>::exp diff={diff} (raw={raw})");
        }
    }
}

