// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `expm1` algorithm family — `expm1(x) = e^x - 1`, total over the argument.
//!
//! Four generic kernels; `crate::policy::expm1` routes TWO of them
//! ([`expm1_series`] and [`expm1_with_exp`]) on a value-dependent validity wall
//! and supplies each width's work integer and guard. [`expm1_halving`] and
//! [`expm1_reduced`] are correct over regions that overlap the routed pair
//! entirely, so choosing between them is an OPTIMALITY question — they stay as
//! kept alternatives for a later measured race, per
//! `docs/ARCHITECTURE.md` → "Keeping the alternatives".
//!
//! All four are generic over the work integer `S: BigInt` and share one
//! working-scale signature
//!
//! ```text
//! fn expm1_<variant>_fixed<S: BigInt>(working_value: S, working_scale: u32) -> Option<S>
//! ```
//!
//! — the working-scale `expm1`, `None` = out of range, mirroring
//! `exp_generic::try_exp_fixed` so the eventual policy wrapper applies the
//! overflow policy once.
//!
//! | variant | reduction | reassembly | status |
//! |---|---|---|---|
//! | [`expm1_series`] | none | none | **ROUTED** for `\|x\| <= 1` |
//! | [`expm1_with_exp`] | `exp_fixed`'s | exact `- 10^w` | **ROUTED** for `\|x\| > 1` |
//! | [`expm1_halving`] | binary `v >> n` | `E <- E*(E + 2)` | kept — contracting for `x <= 0`, no `ln 2` |
//! | [`expm1_reduced`] | `k*ln 2` | `((P + E) << k) - P` | kept — flat peak for large positive `x` |
//!
//! Each candidate's reduction and reassembly are the table above; its validity
//! inequality is stated in its own module docs, and the Ziv strategy in
//! [`expm1_support`]. The three results a wirer cannot reconstruct from those
//! alone are recorded here in full:
//!
//! 1. **The reassembly cannot cancel.** For `k != 0`, `|expm1(x)| >= 0.2928`, so
//!    the final `- 1` discards at most 1.77 bits. The one real cancellation in
//!    the pipeline is the REDUCTION `x - k*ln 2`, paid for by the guard lift.
//! 2. **The deep-negative representative is `1 - 10^w`, never `-10^w`** — see
//!    [`expm1_generic::just_above_minus_one`]. A bare `-10^w` makes the walkers'
//!    `never_exact` rule bump the magnitude, so `Floor` returns `-1 - 1 ULP`:
//!    the wrong side. (It is representable, so this is a silently WRONG
//!    value rather than a panic - which is why only a test catches it.)
//! 3. **The strict wrapper must pass `never_exact = FALSE`** — the opposite of
//!    `exp`. That flag asserts "when the residual reads zero, the TRUE magnitude
//!    is LARGER"; it is sound for `exp` only because `exp > 0` everywhere, so a
//!    positive neglected tail always increases the magnitude. `expm1` changes
//!    sign, and on the negative half a positive tail moves the value TOWARD
//!    zero — the opposite of what the walker's `bump` can express. The two bands
//!    where the side IS analytically known are handled before the walker sees
//!    them (the near-min pin, and the `1 - 10^w` representative above), so what
//!    remains is genuine Table-Maker's-Dilemma residue with no known side.
//!    Derivation in the research doc, section 4.4.

pub(crate) mod expm1_halving;
pub(crate) mod expm1_reduced;
pub(crate) mod expm1_series;
pub(crate) mod expm1_generic;
pub(crate) mod expm1_with_exp;

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Applies the family's overflow policy to a kernel's `Option` verdict —
/// "detect once in the kernel, the wrapper applies the policy".
///
/// `None` means the result cannot be produced in the work integer at this
/// working scale; a fixed-width decimal has no infinity, so the contract is a
/// PANIC, uniform across every tier and scale and in both debug and release.
#[inline]
pub(crate) fn checked<S>(value: Option<S>, method: &str, scale: u32) -> S {
    value.unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale(method, scale))
}

/// Directed-rounding post-adjust for the sub-resolution band near `x = 0` —
/// the mirror image of [`log1p`'s `adjust_near_zero`](crate::algos::log1p::adjust_near_zero),
/// reflected because `expm1` bends the opposite way.
///
/// Convexity gives `e^x > 1 + x` strictly for every `x != 0`, i.e.
///
/// ```text
/// expm1(x) - x = x²/2 + x³/6 + … > 0        for every x != 0
/// ```
///
/// — `expm1(x) > x` STRICTLY, for both signs (the `x²/2` term dominates and is
/// positive either way). And `e^x - 1` is transcendental for algebraic
/// `x != 0` (Lindemann–Weierstrass), so the value never lands exactly on a
/// storage grid line.
///
/// For a tiny `x` the excess `expm1(x) - x ≈ x²/2` can sit far below any
/// REACHABLE working scale (`x = 10^-SCALE` leaves it at ~`10^-2·SCALE`, past
/// the Ziv precision horizon at the wide tiers), so the kernel rounds to
/// exactly the linear term `x` and an upward mode then keeps `x` though the
/// true value is strictly above it.
///
/// Because `expm1(x) > x`, a CORRECT upward result can never equal `x`, so
/// `rounded == raw` is unambiguously the sub-resolution undershoot — step UP one
/// LSB. `expm1(0) = 0` is exact and excluded; nearest modes (the fraction is
/// `0⁺`, so they round to `x` anyway) and `Floor` (`x` IS the correct floor)
/// are already right. `Ceiling` steps up for both signs; `Trunc` (toward zero)
/// steps up only for `x < 0`, since for `x > 0` truncation moves DOWN and `x`
/// is then the correct answer.
///
/// A no-op unless the result is exactly `raw`, so every cell whose deciding
/// digit the walker actually reaches passes through untouched.
///
/// # Scope — the STRICT series path no longer uses this
///
/// Testing `rounded == raw` reaches only the ONE grid point where the value
/// lands on its own linear term. The value lands on DEEPER partial sums just
/// as often, whenever the argument makes `x^j/j!` terminate — `x = -3e-152`
/// reaches the 3rd, `x = -3e-86` the 5th — and this test is blind to every one
/// of them. No fixed number of such tests would do, because the run of
/// exactly-representable terms is unbounded for a suitably composite
/// coefficient.
///
/// [`expm1_series_g`](super::expm1_series::expm1_series_g) therefore threads
/// the series' own tail sign into the walker instead, which is exact at every
/// depth. What remains here is the two callers that cannot use it:
///
/// * the `_approx` single-shot paths, which run no Ziv walker at all, and
///   whose contract is explicitly not correct rounding;
/// * [`expm1_with_exp_g`](super::expm1_with_exp::expm1_with_exp_g), where it is
///   a provable no-op — the policy routes `|x| > 1` there, and
///   `expm1(x) - x = x²/2 + ...` exceeds half an ULP by orders of magnitude
///   across that whole region, so `rounded == raw` never holds. It is left in
///   place rather than removed so that path stays byte-for-byte unchanged.
#[inline]
pub(crate) fn adjust_near_zero<St: BigInt>(rounded: St, raw: St, mode: RoundingMode) -> St {
    if crate::support::rounding::is_nearest_mode(mode) {
        return rounded;
    }
    if raw == <St as BigInt>::ZERO {
        return rounded; // expm1(0) = 0 is exact
    }
    if rounded != raw {
        return rounded; // only the sub-resolution linear-term undershoot
    }
    match mode {
        RoundingMode::Ceiling => rounded + <St as BigInt>::ONE,
        RoundingMode::Trunc if raw < <St as BigInt>::ZERO => rounded + <St as BigInt>::ONE,
        _ => rounded,
    }
}

/// The one `expm1` test that cannot live in `decimal-scale-test`: it reads
/// `types::log_exp::STRICT_GUARD`, which is crate-private with no public
/// equivalent. Everything else from the old `tests.rs` is now
/// `decimal-scale-test/tests/api/expm1.rs`.
#[cfg(test)]
mod crate_internal_tests {
    use crate::int::types::Int;
    use crate::D;

    /// `1.0` at scale 20.
    const UNIT: i128 = 10_i128.pow(20);

    fn d38s20(raw: i128) -> D<Int<2>, 20> {
        D::<Int<2>, 20>(Int::<2>::from_i128(raw))
    }

    /// `_approx` at the strict guard must return the strict answer (the
    /// documented redirect every `*_approx_with` carries), and a looser guard
    /// must still land within a few ULP of it.
    ///
    /// STAYS CRATE-INTERNAL: the redirect is defined AT `STRICT_GUARD`, and that
    /// constant is not exported. Substituting its literal value would stop the
    /// test tracking the constant, which is the only thing it is really pinning.
    #[test]
    fn expm1_approx_redirects_at_strict_guard_and_stays_close_below_it() {
        for arg in [UNIT / 2, -UNIT / 2, 2 * UNIT] {
            let strict = d38s20(arg).expm1_strict().to_bits().as_i128();
            assert_eq!(
                d38s20(arg)
                    .expm1_approx(crate::types::log_exp::STRICT_GUARD)
                    .to_bits()
                    .as_i128(),
                strict,
                "expm1_approx at the strict guard must redirect to strict, arg {arg}"
            );
            let loose = d38s20(arg).expm1_approx(12).to_bits().as_i128();
            assert!(
                (loose - strict).abs() <= 2,
                "expm1_approx(12) drifted more than 2 ULP from strict at {arg}"
            );
        }
    }
}

#[cfg(test)]
mod candidate_agreement_tests {
    //! Cross-candidate smoke agreement. NOT a correctness gate — the golden
    //! suite is that, once a candidate is wired and the strict wrapper exists.
    //! These only catch gross breakage in a candidate (a wrong reassembly, a
    //! dropped reduction), so the tolerance is deliberately loose: the design's
    //! own error bounds are a few hundred working units for the direct series
    //! and `~1.5*2^n` (`~8e3` at `w = 60`) for the `exp` route, so `10^6`
    //! working units out of `10^60` separates "these agree" from "one of them
    //! is broken" without pinning either kernel's exact rounding.

    use super::expm1_halving::expm1_halving_fixed;
    use super::expm1_reduced::expm1_reduced_fixed;
    use super::expm1_series::expm1_series_fixed;
    use super::expm1_with_exp::expm1_with_exp_fixed;
    use crate::algos::exp::exp_generic as eg;
    use crate::int::types::Int;

    type S = Int<24>;

    const W: u32 = 60;
    /// Loose agreement window, in working units (see the module docs).
    const TOL: i128 = 1_000_000;

    fn at(units: i128, exp10: u32) -> S {
        eg::lit::<S>(units) * eg::pow10::<S>(exp10)
    }

    fn close(lhs: S, rhs: S, what: &str) {
        let difference = lhs - rhs;
        let difference = if difference < S::ZERO { -difference } else { difference };
        assert!(
            difference <= eg::lit::<S>(TOL),
            "{what}: candidates disagree by more than the smoke tolerance"
        );
    }

    /// Every candidate must agree on a spread of in-band arguments spanning
    /// both signs, the reduction boundary (`|x| ~ ln2/2`) and the `k != 0`
    /// reassembly.
    #[test]
    fn candidates_agree_on_the_common_band() {
        // x = 0.25, -0.25, 0.5, -0.5 at scale W (values * 10^W).
        for (units, exp10, name) in [
            (25i128, W - 2, "0.25"),
            (-25i128, W - 2, "-0.25"),
            (5i128, W - 1, "0.5"),
            (-5i128, W - 1, "-0.5"),
        ] {
            let working_value = at(units, exp10);
            let baseline = expm1_with_exp_fixed::<S>(working_value, W).expect("via_exp in range");
            close(
                expm1_series_fixed::<S>(working_value, W).expect("series in band"),
                baseline,
                name,
            );
            close(
                expm1_halving_fixed::<S>(working_value, W).expect("halving in range"),
                baseline,
                name,
            );
            close(
                expm1_reduced_fixed::<S>(working_value, W).expect("reduced in range"),
                baseline,
                name,
            );
        }
    }

    /// Outside the direct band the reducing candidates must still agree with
    /// the baseline — this is what exercises the `k != 0` reassembly in both
    /// shift directions.
    #[test]
    fn reducing_candidates_agree_outside_the_direct_band() {
        for (units, exp10, name) in [
            (3i128, W, "3"),
            (-3i128, W, "-3"),
            (12i128, W, "12"),
            (-12i128, W, "-12"),
        ] {
            let working_value = at(units, exp10);
            let baseline = expm1_with_exp_fixed::<S>(working_value, W).expect("via_exp in range");
            close(
                expm1_halving_fixed::<S>(working_value, W).expect("halving in range"),
                baseline,
                name,
            );
            close(
                expm1_reduced_fixed::<S>(working_value, W).expect("reduced in range"),
                baseline,
                name,
            );
        }
    }

    /// `expm1(0) = 0` exactly, in every candidate — the one exact case.
    #[test]
    fn zero_is_exact_in_every_candidate() {
        assert_eq!(expm1_series_fixed::<S>(S::ZERO, W), Some(S::ZERO));
        assert_eq!(expm1_halving_fixed::<S>(S::ZERO, W), Some(S::ZERO));
        assert_eq!(expm1_reduced_fixed::<S>(S::ZERO, W), Some(S::ZERO));
        assert_eq!(expm1_with_exp_fixed::<S>(S::ZERO, W), Some(S::ZERO));
    }

    /// The deep-negative tail must land ONE working unit above `-1`, never on
    /// `-10^w`: a bare `-10^w` leaves a zero residual, which the walkers'
    /// `never_exact` rule reads as "further from zero" and bumps, so `Floor`
    /// would return `-1 - 1 ULP` - the wrong side, and representable, so a
    /// silently wrong value rather than a panic.
    #[test]
    fn deep_negative_lands_just_above_minus_one() {
        let want = eg::lit::<S>(1) - eg::one::<S>(W);
        // x = -2000. The magnitude must clear the regime classifier's
        // BIT-LENGTH test, which is a sufficient (hence conservative)
        // condition: at `w = 60` it needs `bit_length(v) >= 210`, i.e.
        // `|x| >~ 820`. A smaller argument like -500 is mathematically just as
        // deep (`e^-500 ~ 1e-218`, far under `10^-60`) but sits one bit under
        // that threshold, so `expm1_series_fixed` declines it as out-of-BAND
        // instead — correct behaviour, wrong test.
        let working_value = at(-2000, W);
        for (got, name) in [
            (expm1_series_fixed::<S>(working_value, W), "series"),
            (expm1_halving_fixed::<S>(working_value, W), "halving"),
            (expm1_reduced_fixed::<S>(working_value, W), "reduced"),
            (expm1_with_exp_fixed::<S>(working_value, W), "via_exp"),
        ] {
            assert_eq!(got, Some(want), "{name}: deep-negative representative");
        }
    }
}
