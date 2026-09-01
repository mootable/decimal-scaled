//! The `checked_<fn>_strict` / `_strict_with` pairs for `log1p` and `expm1`
//! (`src/types/checked_transcendentals.rs`).
//!
//! `docs/ARCHITECTURE.md` ("Overflow & domain behaviour") states the
//! invariant: every `<fn>_strict` / `<fn>_strict_with` has a `checked_`
//! sibling returning `Option`. `log1p` is the pair with teeth — it carries a
//! genuine domain panic at `t <= -1`, so the checked form is the only
//! non-panicking route for a caller that cannot guarantee the domain.
//! `expm1` is total over its argument, so its pair completes the surface
//! rather than guarding a domain.
//!
//! Both assertions matter in each test: that the checked form returns `None`
//! exactly where the default form panics, and that where it returns `Some` it
//! is bit-identical to the unchecked sibling — a `checked_` that disagreed
//! with its sibling would be worse than none at all.

mod from_checked_transcendentals {
    use decimal_scaled::{D38, RoundingMode};

    const MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Ceiling,
        RoundingMode::Floor,
        RoundingMode::Trunc,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// `-1/2` at scale 12 — inside the domain, and on the negative side
    /// where `log1p` returns a negative result.
    fn minus_half() -> D38<12> {
        D38::<12>::ZERO - (D38::<12>::ONE / D38::<12>::try_from(2i64).unwrap())
    }

    // ── log1p: the domain wall ────────────────────────────────────────

    #[test]
    fn checked_log1p_is_none_at_and_below_the_domain_wall() {
        // `log1p(t) = ln(1 + t)`: undefined at `t = -1` (ln 0) and below.
        for units in [-1i64, -2, -5, -1_000] {
            let t = D38::<12>::try_from(units).unwrap();
            assert_eq!(t.checked_log1p_strict(), None, "t = {units}");
            for mode in MODES {
                assert_eq!(
                    t.checked_log1p_strict_with(mode),
                    None,
                    "t = {units}, mode {mode:?}"
                );
            }
        }
    }

    #[test]
    fn checked_log1p_agrees_with_the_unchecked_sibling_in_domain() {
        let inputs = [D38::<12>::ZERO, D38::<12>::ONE, minus_half()];
        for t in inputs {
            for mode in MODES {
                let checked = t.checked_log1p_strict_with(mode);
                assert_eq!(
                    checked,
                    Some(t.log1p_strict_with(mode)),
                    "t = {t:?}, mode {mode:?}"
                );
                assert!(checked.is_some(), "t = {t:?} is in domain, mode {mode:?}");
            }
            assert_eq!(t.checked_log1p_strict(), Some(t.log1p_strict()));
        }
    }

    #[test]
    fn checked_log1p_is_exact_at_zero() {
        // `log1p(0) = ln(1) = 0`, the one exact point.
        assert_eq!(
            D38::<12>::ZERO.checked_log1p_strict(),
            Some(D38::<12>::ZERO)
        );
    }

    // ── expm1: total, so always `Some` ────────────────────────────────

    #[test]
    fn checked_expm1_agrees_with_the_unchecked_sibling() {
        let inputs = [
            D38::<12>::ZERO,
            D38::<12>::ONE,
            minus_half(),
            D38::<12>::try_from(-1i64).unwrap(),
            D38::<12>::try_from(3i64).unwrap(),
        ];
        for x in inputs {
            for mode in MODES {
                let checked = x.checked_expm1_strict_with(mode);
                assert_eq!(
                    checked,
                    Some(x.expm1_strict_with(mode)),
                    "x = {x:?}, mode {mode:?}"
                );
                assert!(checked.is_some(), "expm1 is total, x = {x:?}");
            }
            assert_eq!(x.checked_expm1_strict(), Some(x.expm1_strict()));
        }
    }

    #[test]
    fn checked_expm1_is_exact_at_zero() {
        // `expm1(0) = e^0 - 1 = 0`.
        assert_eq!(
            D38::<12>::ZERO.checked_expm1_strict(),
            Some(D38::<12>::ZERO)
        );
    }

    // ── the same contract at a wide tier ──────────────────────────────
    //
    // The pairs are emitted by ONE generic `impl` over `(N, SCALE)`, so a
    // wide tier exercises the same source. Checked anyway: the domain wall
    // is built from `10^SCALE` at the tier's own width, which is where a
    // width-dependent mistake would show.

    #[cfg(feature = "wide")]
    mod wide {
        use super::MODES;
        use decimal_scaled::D57;

        #[test]
        fn checked_log1p_domain_wall_holds_at_d57() {
            for units in [-1i64, -2, -1_000] {
                let t = D57::<12>::try_from(units).unwrap();
                assert_eq!(t.checked_log1p_strict(), None, "t = {units}");
                for mode in MODES {
                    assert_eq!(
                        t.checked_log1p_strict_with(mode),
                        None,
                        "t = {units}, mode {mode:?}"
                    );
                }
            }
        }

        #[test]
        fn checked_pairs_agree_with_their_siblings_at_d57() {
            let inputs = [D57::<12>::ZERO, D57::<12>::ONE];
            for v in inputs {
                for mode in MODES {
                    assert_eq!(
                        v.checked_log1p_strict_with(mode),
                        Some(v.log1p_strict_with(mode)),
                        "log1p v = {v:?}, mode {mode:?}"
                    );
                    assert_eq!(
                        v.checked_expm1_strict_with(mode),
                        Some(v.expm1_strict_with(mode)),
                        "expm1 v = {v:?}, mode {mode:?}"
                    );
                }
            }
        }
    }
}
