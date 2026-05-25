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
    let quarter_pi_bits = i128::from(D::quarter_pi().to_bits());
    (-quarter_pi_bits + half)..=(quarter_pi_bits - half)
}

// ─── D38<19> property blocks ───────────────────────────────────────

proptest! {
    #![proptest_config(proptest_config("exp_of_ln_roundtrip"))]
    #[test]
    fn exp_of_ln_roundtrip(raw in positive_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let y = x.ln_strict().exp_strict();
        let xb = i128::from(x.to_bits());
        let yb = i128::from(y.to_bits());
        let tol = round_trip_tol(xb);
        prop_assert_within!("exp(ln(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("ln_of_exp_roundtrip"))]
    #[test]
    fn ln_of_exp_roundtrip(raw in ln_exp_domain()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let y = x.exp_strict().ln_strict();
        let xb = i128::from(x.to_bits());
        let yb = i128::from(y.to_bits());
        let tol = round_trip_tol(xb);
        prop_assert_within!("ln(exp(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("sin2_plus_cos2_is_one"))]
    #[test]
    fn sin2_plus_cos2_is_one(raw in real_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let s = x.sin_strict();
        let c = x.cos_strict();
        let sum = s * s + c * c;
        let one = D::from_bits(decimal_scaled::Int::<2>::try_from(ONE).unwrap());
        let tol = round_trip_tol(ONE);
        prop_assert_within!("sin²+cos²",
                            i128::from(sum.to_bits()), i128::from(one.to_bits()),
                            (i128::from(sum.to_bits()) - i128::from(one.to_bits())).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("sqrt_squared_roundtrip"))]
    #[test]
    fn sqrt_squared_roundtrip(raw in nonneg_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let r = x.sqrt_strict();
        let back = r * r;
        let xb = i128::from(x.to_bits());
        let bb = i128::from(back.to_bits());
        let tol = round_trip_tol(xb);
        prop_assert_within!("sqrt(x)²", xb, bb, (xb - bb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("cbrt_cubed_roundtrip"))]
    #[test]
    fn cbrt_cubed_roundtrip(raw in real_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let r = x.cbrt_strict();
        let back = r * r * r;
        let xb = i128::from(x.to_bits());
        let bb = i128::from(back.to_bits());
        let tol = round_trip_tol(xb);
        prop_assert_within!("cbrt(x)³", xb, bb, (xb - bb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("atan_of_tan_roundtrip"))]
    #[test]
    fn atan_of_tan_roundtrip(raw in open_quarter_pi()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let y = x.tan_strict().atan_strict();
        let xb = i128::from(x.to_bits());
        let yb = i128::from(y.to_bits());
        let tol = round_trip_tol(xb);
        prop_assert_within!("atan(tan(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("tanh_atanh_roundtrip"))]
    #[test]
    fn tanh_atanh_roundtrip(raw in open_interval_neg1_to_1()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let y = x.atanh_strict().tanh_strict();
        let xb = i128::from(x.to_bits());
        let yb = i128::from(y.to_bits());
        let tol = round_trip_tol(xb);
        prop_assert_within!("tanh(atanh(x))", xb, yb, (xb - yb).abs(), tol);
    }
}

proptest! {
    #![proptest_config(proptest_config("sin_odd_symmetry"))]
    #[test]
    fn sin_odd_symmetry(raw in real_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let lhs = x.sin_strict();
        let rhs = -((-x).sin_strict());
        prop_assert_within!("sin(-x)=-sin(x)", i128::from(lhs.to_bits()), i128::from(rhs.to_bits()),
                            (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs(),
                            SYMMETRY_LSB_TOL);
    }
}

proptest! {
    #![proptest_config(proptest_config("cos_even_symmetry"))]
    #[test]
    fn cos_even_symmetry(raw in real_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let lhs = x.cos_strict();
        let rhs = (-x).cos_strict();
        prop_assert_within!("cos(-x)=cos(x)", i128::from(lhs.to_bits()), i128::from(rhs.to_bits()),
                            (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs(),
                            SYMMETRY_LSB_TOL);
    }
}

proptest! {
    #![proptest_config(proptest_config("atan_odd_symmetry"))]
    #[test]
    fn atan_odd_symmetry(raw in real_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let lhs = x.atan_strict();
        let rhs = -((-x).atan_strict());
        prop_assert_within!("atan(-x)=-atan(x)", i128::from(lhs.to_bits()), i128::from(rhs.to_bits()),
                            (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs(),
                            SYMMETRY_LSB_TOL);
    }
}

proptest! {
    #![proptest_config(proptest_config("cbrt_odd_symmetry"))]
    #[test]
    fn cbrt_odd_symmetry(raw in real_x()) {
        let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
        let lhs = x.cbrt_strict();
        let rhs = -((-x).cbrt_strict());
        prop_assert_within!("cbrt(-x)=-cbrt(x)", i128::from(lhs.to_bits()), i128::from(rhs.to_bits()),
                            (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs(),
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
            let n = N::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let w: W = n.into();
            let nb = i128::from(n.sqrt_strict().to_bits());
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
            let n = N::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let w: W = n.into();
            let nb = i128::from(n.exp_strict().to_bits());
            let wb = w.exp_strict().to_bits().to_i128_checked()
                .expect("D76<19>::exp fits i128 at moderate x");
            let diff = (wb - nb).abs();
            prop_assert!(diff <= CROSS_TIER_LSB_TOL,
                         "D76<19>::exp vs D38<19>::exp diff={diff} (raw={raw})");
        }
    }
}

// ─── Hard-input category strategies ────────────────────────────────
//
// One strategy per category from `scripts/gen_hard_inputs.py`. The
// proptest harness has no mpmath oracle at runtime, so we still
// assert against round-trip / symmetry identities — the *category*
// determines the input distribution, not the assertion.
//
// Each block runs `CASES` cases with a deterministic per-block seed
// label, identical hermetic-CI setup to the blocks above.
//
// References per category live in `scripts/gen_hard_inputs.py`'s
// module docstring (lines 25-52). Tolerances re-use the file-level
// `round_trip_tol` / `SYMMETRY_LSB_TOL` budgets so adding a new
// category does not weaken the existing contract.

mod hard_inputs {
    use super::{CASES, RELATIVE_TOL_INV, ROUND_TRIP_FLOOR_LSB, SYMMETRY_LSB_TOL};
    use decimal_scaled::{D38, DecimalConstants};
    use proptest::prelude::*;

    type D = D38<19>;
    type Bits = i128;

    const ONE: Bits = 10i128.pow(19);

    fn config(label: &'static str) -> ProptestConfig {
        ProptestConfig {
            cases: CASES,
            max_shrink_iters: 256,
            failure_persistence: None,
            source_file: Some(label),
            ..ProptestConfig::default()
        }
    }

    fn round_trip_tol(reference: i128) -> i128 {
        let rel = reference.unsigned_abs() / (RELATIVE_TOL_INV as u128);
        ROUND_TRIP_FLOOR_LSB + (rel as i128)
    }

    // ─── Category 1: half-ULP-tie boundaries ──────────────────────
    //
    // Lefèvre/Muller (1998); Muller (2016) §10. We cannot detect
    // half-tie cases without an oracle, but uniform sampling at a
    // very fine granularity in a narrow window around an interesting
    // anchor concentrates probes near boundaries.

    fn near_unit() -> impl Strategy<Value = Bits> {
        // [1 - 10^-6, 1 + 10^-6] in storage units.
        let delta = ONE / 1_000_000;
        (ONE - delta)..=(ONE + delta)
    }

    proptest! {
        #![proptest_config(config("hard_tie_sqrt_roundtrip"))]
        #[test]
        fn hard_tie_sqrt_roundtrip(raw in near_unit()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let r = x.sqrt_strict();
            let back = r * r;
            let xb = i128::from(x.to_bits());
            let bb = i128::from(back.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - bb).abs();
            prop_assert!(diff <= tol,
                "hard_tie sqrt(x)²: x={xb} back={bb} diff={diff} > tol={tol}");
        }
    }

    // ─── Category 2: catastrophic cancellation ────────────────────
    //
    // Goldberg (1991) §3; Higham (2002) §1.7.
    //   ln(1+ε), exp(tiny), cos(tiny) ≈ 1 - x²/2, sin(tiny) ≈ x.

    fn tiny_around_zero() -> impl Strategy<Value = Bits> {
        // |x| <= 10^-6 — exp/cos/sin lose >12 leading digits of
        // information here.
        let bound = ONE / 1_000_000;
        (-bound)..=bound
    }

    fn ln_just_above_one() -> impl Strategy<Value = Bits> {
        // Inputs of the form 1 ± tiny, where ln(1+ε) ≈ ε.
        let bound = ONE / 1_000_000;
        (ONE - bound)..=(ONE + bound)
    }

    proptest! {
        #![proptest_config(config("hard_canc_exp_of_ln"))]
        #[test]
        fn hard_canc_exp_of_ln(raw in ln_just_above_one()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let y = x.ln_strict().exp_strict();
            let xb = i128::from(x.to_bits());
            let yb = i128::from(y.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - yb).abs();
            prop_assert!(diff <= tol,
                "hard_canc exp(ln(x)) near 1: x={xb} y={yb} diff={diff} > tol={tol}");
        }
    }

    proptest! {
        #![proptest_config(config("hard_canc_pythag_tiny"))]
        #[test]
        fn hard_canc_pythag_tiny(raw in tiny_around_zero()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let s = x.sin_strict();
            let c = x.cos_strict();
            let sum = s * s + c * c;
            let one = D::from_bits(decimal_scaled::Int::<2>::try_from(ONE).unwrap());
            let tol = round_trip_tol(ONE);
            let diff = (i128::from(sum.to_bits()) - i128::from(one.to_bits())).abs();
            prop_assert!(diff <= tol,
                "hard_canc sin²+cos² tiny: sum={} one={} diff={diff} > tol={tol}",
                i128::from(sum.to_bits()), i128::from(one.to_bits()));
        }
    }

    // ─── Category 3: range-reduction breakpoints ──────────────────
    //
    // Payne & Hanek (1983); Muller (2016) §11. Sample within ±a few
    // LSBs of k·π/2 for sin/cos; ±LSBs of k·π/4 for tan.

    fn near_half_pi_multiples() -> impl Strategy<Value = Bits> {
        // Pre-compute k·π/2 for k = -4..=4 and probe ±100 storage LSBs.
        let half_pi = i128::from(D::half_pi().to_bits());
        (-4i64..=4i64).prop_flat_map(move |k| {
            let center = (k as i128) * half_pi;
            (-100i128..=100i128).prop_map(move |d| center + d)
        })
    }

    fn near_quarter_pi_odd_multiples() -> impl Strategy<Value = Bits> {
        // Odd k for k·π/4 — these are the tan(45°)-style breakpoints
        // (and the safe_to_case helper rejects π/2 poles).
        let quarter_pi = i128::from(D::quarter_pi().to_bits());
        (prop::sample::select(vec![-7i64, -5, -3, -1, 1, 3, 5, 7])).prop_flat_map(move |k| {
            let center = (k as i128) * quarter_pi;
            (-50i128..=50i128).prop_map(move |d| center + d)
        })
    }

    proptest! {
        #![proptest_config(config("hard_rred_sin_symmetry"))]
        #[test]
        fn hard_rred_sin_symmetry(raw in near_half_pi_multiples()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let lhs = x.sin_strict();
            let rhs = -((-x).sin_strict());
            let diff = (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs();
            prop_assert!(diff <= SYMMETRY_LSB_TOL,
                "hard_rred sin(-x)=-sin(x): lhs={} rhs={} diff={diff}",
                i128::from(lhs.to_bits()), i128::from(rhs.to_bits()));
        }
    }

    proptest! {
        #![proptest_config(config("hard_rred_pythag"))]
        #[test]
        fn hard_rred_pythag(raw in near_quarter_pi_odd_multiples()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let s = x.sin_strict();
            let c = x.cos_strict();
            let sum = s * s + c * c;
            let one = D::from_bits(decimal_scaled::Int::<2>::try_from(ONE).unwrap());
            let tol = round_trip_tol(ONE);
            let diff = (i128::from(sum.to_bits()) - i128::from(one.to_bits())).abs();
            prop_assert!(diff <= tol,
                "hard_rred sin²+cos² near k·π/4: sum={} one={} diff={diff} > tol={tol}",
                i128::from(sum.to_bits()), i128::from(one.to_bits()));
        }
    }

    // ─── Category 4: removable singularity / asymptote stress ─────
    //
    // Kahan archive "Branch cuts" (1987). ln near 0+, sqrt near 0+,
    // atan at large |x|.

    fn ln_near_zero() -> impl Strategy<Value = Bits> {
        // (0, 10^-3] — sqrt(ε) and ln(ε) blow up the derivative.
        1i128..=(ONE / 1000)
    }

    fn huge_real() -> impl Strategy<Value = Bits> {
        // |x| up to 10^6 — atan saturates to ±π/2 well before then.
        let bound = ONE * 1_000_000;
        (-bound)..=bound
    }

    proptest! {
        #![proptest_config(config("hard_asym_exp_of_ln_small"))]
        #[test]
        fn hard_asym_exp_of_ln_small(raw in ln_near_zero()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let y = x.ln_strict().exp_strict();
            let xb = i128::from(x.to_bits());
            let yb = i128::from(y.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - yb).abs();
            prop_assert!(diff <= tol,
                "hard_asym exp(ln(small)): x={xb} y={yb} diff={diff} > tol={tol}");
        }
    }

    proptest! {
        #![proptest_config(config("hard_asym_atan_odd_huge"))]
        #[test]
        fn hard_asym_atan_odd_huge(raw in huge_real()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let lhs = x.atan_strict();
            let rhs = -((-x).atan_strict());
            let diff = (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs();
            prop_assert!(diff <= SYMMETRY_LSB_TOL,
                "hard_asym atan(-x)=-atan(x) huge: lhs={} rhs={} diff={diff}",
                i128::from(lhs.to_bits()), i128::from(rhs.to_bits()));
        }
    }

    // ─── Category 5: inverse-identity round-trip stress ───────────
    //
    // Brent & Zimmermann (2010) §4.2. atan(tan(x)) on (-π/4, π/4),
    // sqrt(x²), cbrt(x³). The existing blocks above cover these on
    // uniform domains; here we tilt the strategies toward the
    // hardest sub-intervals.

    fn quarter_pi_inner() -> impl Strategy<Value = Bits> {
        // (-π/4 + ε, π/4 - ε) — sample heavily near the boundary.
        let quarter_pi = i128::from(D::quarter_pi().to_bits());
        let margin = ONE / 1000;
        let band = ONE / 100;
        let lo = quarter_pi - band;
        let hi = quarter_pi - margin;
        prop::sample::select(vec![1i64, -1])
            .prop_flat_map(move |s| (lo..=hi).prop_map(move |b| (s as i128) * b))
    }

    fn squares_of_inputs() -> impl Strategy<Value = Bits> {
        // x² for x ∈ small or large — exposes sqrt's branch decision.
        (1i128..=(ONE * 100)).prop_map(|raw| {
            // square as i128: result is (raw * raw) / ONE.
            // For raw up to 10^21, raw*raw can overflow i128 (max ~1.7e38).
            // Restrict to raw² fitting i128 with margin.
            let safe = if raw > ONE / 10 { raw } else { raw };
            (safe.saturating_mul(safe)) / ONE
        })
    }

    proptest! {
        #![proptest_config(config("hard_inv_atan_of_tan"))]
        #[test]
        fn hard_inv_atan_of_tan(raw in quarter_pi_inner()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let y = x.tan_strict().atan_strict();
            let xb = i128::from(x.to_bits());
            let yb = i128::from(y.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - yb).abs();
            prop_assert!(diff <= tol,
                "hard_inv atan(tan(x)): x={xb} y={yb} diff={diff} > tol={tol}");
        }
    }

    proptest! {
        #![proptest_config(config("hard_inv_sqrt_of_square"))]
        #[test]
        fn hard_inv_sqrt_of_square(raw in squares_of_inputs()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let r = x.sqrt_strict();
            let back = r * r;
            let xb = i128::from(x.to_bits());
            let bb = i128::from(back.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - bb).abs();
            prop_assert!(diff <= tol,
                "hard_inv sqrt(x)² at square: x={xb} back={bb} diff={diff} > tol={tol}");
        }
    }

    // ─── Category 6: perfect-power ± ULP for roots ────────────────
    //
    // Brent & Zimmermann (2010) §3.5 / §3.6. sqrt(n² ± 1) and
    // cbrt(n³ ± 1) exercise the correctly-rounded branch decision.

    fn perfect_squares_jittered() -> impl Strategy<Value = Bits> {
        // n² · ONE ± δ, for δ in a small jitter window, n in [1, 1000].
        (1i64..=1000i64).prop_flat_map(|n| {
            let center = (n as i128) * (n as i128) * ONE;
            (-3i128..=3i128).prop_map(move |d| center + d)
        })
    }

    fn perfect_cubes_jittered() -> impl Strategy<Value = Bits> {
        // n³ · ONE ± δ, for δ in [-3, 3], n in [-100, 100] excluding 0.
        prop::sample::select((-100i64..=100i64).filter(|&n| n != 0).collect::<Vec<_>>())
            .prop_flat_map(|n| {
                let center = (n as i128) * (n as i128) * (n as i128) * ONE;
                (-3i128..=3i128).prop_map(move |d| center + d)
            })
    }

    proptest! {
        #![proptest_config(config("hard_pp_sqrt_roundtrip"))]
        #[test]
        fn hard_pp_sqrt_roundtrip(raw in perfect_squares_jittered()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let r = x.sqrt_strict();
            let back = r * r;
            let xb = i128::from(x.to_bits());
            let bb = i128::from(back.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - bb).abs();
            prop_assert!(diff <= tol,
                "hard_pp sqrt(n² ± δ): x={xb} back={bb} diff={diff} > tol={tol}");
        }
    }

    proptest! {
        #![proptest_config(config("hard_pp_cbrt_symmetry"))]
        #[test]
        fn hard_pp_cbrt_symmetry(raw in perfect_cubes_jittered()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let lhs = x.cbrt_strict();
            let rhs = -((-x).cbrt_strict());
            let diff = (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs();
            prop_assert!(diff <= SYMMETRY_LSB_TOL,
                "hard_pp cbrt(-x)=-cbrt(x) at cube: lhs={} rhs={} diff={diff}",
                i128::from(lhs.to_bits()), i128::from(rhs.to_bits()));
        }
    }

    // ─── Category 7: constant edges ────────────────────────────────
    //
    // IEEE 754-2019 §9; Muller (2016) §9. Sample inputs ±a few LSBs
    // around named constants (π, e, ln 2, etc.).

    fn near_pi_constants() -> impl Strategy<Value = Bits> {
        let pi = i128::from(D::pi().to_bits());
        let half_pi = i128::from(D::half_pi().to_bits());
        let quarter_pi = i128::from(D::quarter_pi().to_bits());
        prop::sample::select(vec![pi, half_pi, quarter_pi, -pi, -half_pi, -quarter_pi])
            .prop_flat_map(|c| (-5i128..=5i128).prop_map(move |d| c + d))
    }

    fn near_e_unit() -> impl Strategy<Value = Bits> {
        let e = i128::from(D::e().to_bits());
        prop::sample::select(vec![e, -e, ONE, -ONE, 2 * ONE])
            .prop_flat_map(|c| (-5i128..=5i128).prop_map(move |d| c + d))
    }

    proptest! {
        #![proptest_config(config("hard_const_pythag"))]
        #[test]
        fn hard_const_pythag(raw in near_pi_constants()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let s = x.sin_strict();
            let c = x.cos_strict();
            let sum = s * s + c * c;
            let one = D::from_bits(decimal_scaled::Int::<2>::try_from(ONE).unwrap());
            let tol = round_trip_tol(ONE);
            let diff = (i128::from(sum.to_bits()) - i128::from(one.to_bits())).abs();
            prop_assert!(diff <= tol,
                "hard_const sin²+cos² near π const: sum={} one={} diff={diff} > tol={tol}",
                i128::from(sum.to_bits()), i128::from(one.to_bits()));
        }
    }

    proptest! {
        #![proptest_config(config("hard_const_exp_of_ln"))]
        #[test]
        fn hard_const_exp_of_ln(raw in near_e_unit()) {
            // Only positive inputs hit ln's domain.
            prop_assume!(raw > 0);
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let y = x.ln_strict().exp_strict();
            let xb = i128::from(x.to_bits());
            let yb = i128::from(y.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - yb).abs();
            prop_assert!(diff <= tol,
                "hard_const exp(ln(x)) near e: x={xb} y={yb} diff={diff} > tol={tol}");
        }
    }

    // ─── Category 8: argument-halving cascade edge (atan) ─────────
    //
    // ALGORITHMS.md per-width halving count table. Sample x near
    // ±tan(0.35 / 2^k) for k = 0..5.

    fn atan_halving_anchors() -> impl Strategy<Value = Bits> {
        // Pre-computed tan(0.35 · 2^(-k)) at scale 19 for k = 0..5.
        // The exact decimals don't matter for proptest — we just
        // need a deterministic roster that lives in the halving band.
        // tan(0.35) ≈ 0.3650; tan(0.175) ≈ 0.1771; tan(0.0875) ≈ 0.0877; …
        let anchors_pos: Vec<Bits> = vec![
            3_650_817_511_434_127_092, // tan(0.35)
            1_771_239_555_181_148_572, // tan(0.175)
            877_022_257_637_478_437,   // tan(0.0875)
            437_733_213_829_148_988,   // tan(0.04375)
            218_770_525_637_316_891,   // tan(0.021875)
            109_376_991_958_419_141,   // tan(0.0109375)
        ];
        let anchors: Vec<Bits> = anchors_pos.iter().flat_map(|&v| [v, -v]).collect();
        prop::sample::select(anchors).prop_flat_map(|c| (-5i128..=5i128).prop_map(move |d| c + d))
    }

    proptest! {
        #![proptest_config(config("hard_halv_atan_odd"))]
        #[test]
        fn hard_halv_atan_odd(raw in atan_halving_anchors()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let lhs = x.atan_strict();
            let rhs = -((-x).atan_strict());
            let diff = (i128::from(lhs.to_bits()) - i128::from(rhs.to_bits())).abs();
            prop_assert!(diff <= SYMMETRY_LSB_TOL,
                "hard_halv atan(-x)=-atan(x): lhs={} rhs={} diff={diff}",
                i128::from(lhs.to_bits()), i128::from(rhs.to_bits()));
        }
    }

    // ─── Category 9: Stage-2 argument reduction edge for exp ──────
    //
    // Tang (1989). exp(x) is reduced to exp(r) where r = x - k·ln2.
    // Sample x near k·ln2 for k in a small integer range.

    fn near_k_ln2() -> impl Strategy<Value = Bits> {
        // ln(2) at scale 19, integer-rounded.
        //
        // We restrict to non-negative k because the round-trip
        // identity `ln(exp(-k·ln2))` is intrinsically lossy at the
        // scale-19 storage: for x more negative than about
        // -13·ln2, exp(x) underflows below 10^-9, leaving < scale
        // significant digits, and the subsequent ln re-amplifies
        // the relative error well past the `round_trip_tol` budget.
        // The audit's known-failing arrays in `ulp_strict_golden.rs`
        // own the negative-x exp/ln precision holes; the proptest
        // strategy stays inside the well-conditioned half.
        const LN2_S19: i128 = 6_931_471_805_599_453_094;
        (0i64..=30i64).prop_flat_map(|k| {
            let center = (k as i128) * LN2_S19;
            (-5i128..=5i128).prop_map(move |d| center + d)
        })
    }

    proptest! {
        #![proptest_config(config("hard_exp_stage2_roundtrip"))]
        #[test]
        fn hard_exp_stage2_roundtrip(raw in near_k_ln2()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            // exp(x) might overflow at |x| > ~44 in storage; the
            // strategy keeps |k| <= 30 so we're well inside.
            let y = x.exp_strict().ln_strict();
            let xb = i128::from(x.to_bits());
            let yb = i128::from(y.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - yb).abs();
            prop_assert!(diff <= tol,
                "hard_exp ln(exp(k·ln2)): x={xb} y={yb} diff={diff} > tol={tol}");
        }
    }

    // ─── Category 10: Tang-lookup band edges (ln / exp) ───────────
    //
    // Tang (1989); Gal & Bachelis (1991). The shipped lookup bands
    // are scale-keyed; at D38<19> the kernel does not enter the
    // Tang slot. We still exercise the lookup-index breakpoints
    // T_i = 1 + i/N for small N, where the shipped ln kernel's
    // table-driven branch (if any) would route differently.

    fn tang_lookup_anchors() -> impl Strategy<Value = Bits> {
        // T_i = 1 + i/128 for i = 0..127 at scale 19.
        (0i64..128).prop_flat_map(|i| {
            let center = ONE + (i as i128) * (ONE / 128);
            (-3i128..=3i128).prop_map(move |d| center + d)
        })
    }

    proptest! {
        #![proptest_config(config("hard_tang_exp_of_ln"))]
        #[test]
        fn hard_tang_exp_of_ln(raw in tang_lookup_anchors()) {
            let x = D::from_bits(decimal_scaled::Int::<2>::try_from(raw).unwrap());
            let y = x.ln_strict().exp_strict();
            let xb = i128::from(x.to_bits());
            let yb = i128::from(y.to_bits());
            let tol = round_trip_tol(xb);
            let diff = (xb - yb).abs();
            prop_assert!(diff <= tol,
                "hard_tang exp(ln(T_i + δ)): x={xb} y={yb} diff={diff} > tol={tol}");
        }
    }
}
