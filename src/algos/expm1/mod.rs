// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `expm1` algorithm family — CANDIDATES, none wired.
//!
//! Four drafted approaches to a correctly-rounded `expm1(x) = e^x - 1`, all
//! generic over the work integer `S: BigInt` and all sharing one signature
//!
//! ```text
//! fn expm1_<variant>_fixed<S: BigInt>(v_w: S, w: u32) -> Option<S>
//! ```
//!
//! — the working-scale `expm1`, `None` = out of range, mirroring
//! `exp_generic::try_exp_fixed` so the eventual policy wrapper applies the
//! overflow policy once.
//!
//! | variant | reduction | reassembly | strongest where |
//! |---|---|---|---|
//! | [`expm1_series`] | none | none | small `\|x\|`, best accuracy near 0 |
//! | [`expm1_halving`] | binary `v >> n` | `E <- E*(E + 2)` | `x <= 0` (error-CONTRACTING), small `x > 0` |
//! | [`expm1_reduced`] | `k*ln 2` | `((P + E) << k) - P` | large positive `x` (flat peak) |
//! | [`expm1_via_exp`] | `exp_fixed`'s | exact `- 10^w` | reference baseline / widest domain |
//!
//! The design derivation — the reassembly identity and its error analysis, the
//! no-cancellation lemma, the Ziv strategy and the per-candidate validity
//! inequalities — is in `research/2026_08_31_expm1_algorithm_design.md`.
//!
//! Two results from that derivation are load-bearing for whoever wires this:
//!
//! 1. **The reassembly cannot cancel.** For `k != 0`, `|expm1(x)| >= 0.2928`, so
//!    the final `- 1` discards at most 1.77 bits. The one real cancellation in
//!    the pipeline is the REDUCTION `x - k*ln 2`, paid for by the guard lift.
//! 2. **The deep-negative representative is `1 - 10^w`, never `-10^w`** — see
//!    [`expm1_support::just_above_minus_one`]. A bare `-10^w` makes the walkers'
//!    `never_exact` rule bump the magnitude, so `Floor` returns `-1 - 1 ULP`:
//!    the wrong side, and out of storage range at `SCALE = D`.
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
pub(crate) mod expm1_support;
pub(crate) mod expm1_via_exp;

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
    use super::expm1_via_exp::expm1_via_exp_fixed;
    use crate::algos::exp::exp_generic as eg;
    use crate::int::types::Int;

    type S = Int<24>;

    const W: u32 = 60;
    /// Loose agreement window, in working units (see the module docs).
    const TOL: i128 = 1_000_000;

    fn at(units: i128, exp10: u32) -> S {
        eg::lit::<S>(units) * eg::pow10::<S>(exp10)
    }

    fn close(a: S, b: S, what: &str) {
        let d = a - b;
        let d = if d < S::ZERO { -d } else { d };
        assert!(
            d <= eg::lit::<S>(TOL),
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
            let v = at(units, exp10);
            let base = expm1_via_exp_fixed::<S>(v, W).expect("via_exp in range");
            close(
                expm1_series_fixed::<S>(v, W).expect("series in band"),
                base,
                name,
            );
            close(
                expm1_halving_fixed::<S>(v, W).expect("halving in range"),
                base,
                name,
            );
            close(
                expm1_reduced_fixed::<S>(v, W).expect("reduced in range"),
                base,
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
            let v = at(units, exp10);
            let base = expm1_via_exp_fixed::<S>(v, W).expect("via_exp in range");
            close(
                expm1_halving_fixed::<S>(v, W).expect("halving in range"),
                base,
                name,
            );
            close(
                expm1_reduced_fixed::<S>(v, W).expect("reduced in range"),
                base,
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
        assert_eq!(expm1_via_exp_fixed::<S>(S::ZERO, W), Some(S::ZERO));
    }

    /// The deep-negative tail must land ONE working unit above `-1`, never on
    /// `-10^w`: a bare `-10^w` leaves a zero residual, which the walkers'
    /// `never_exact` rule reads as "further from zero" and bumps, so `Floor`
    /// would return `-1 - 1 ULP` (the wrong side, and out of storage range at
    /// `SCALE = D`).
    #[test]
    fn deep_negative_lands_just_above_minus_one() {
        let want = eg::lit::<S>(1) - eg::one::<S>(W);
        // x = -500: e^-500 is far below 10^-60.
        let v = at(-500, W);
        for (got, name) in [
            (expm1_series_fixed::<S>(v, W), "series"),
            (expm1_halving_fixed::<S>(v, W), "halving"),
            (expm1_reduced_fixed::<S>(v, W), "reduced"),
            (expm1_via_exp_fixed::<S>(v, W), "via_exp"),
        ] {
            assert_eq!(got, Some(want), "{name}: deep-negative representative");
        }
    }
}
