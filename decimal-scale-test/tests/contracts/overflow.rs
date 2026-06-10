//! Overflow contracts: the tier-invariant strict-transcendental panic rule
//! and the hypot structural edges the golden set cannot express.
//! Migrated from `tests/hypot_edge_cases.rs` and
//! `tests/transcendental_overflow_uniform.rs`.

#[cfg(all(feature = "wide", feature = "strict", not(feature = "fast")))]
mod from_hypot_edge_cases {
    //! Structural edge-case gate for `hypot_strict_with` — the non-value
    //! assertions the correctly-rounded golden set cannot express.
    //!
    //! The numeric VALUE coverage for `hypot` (Pythagorean triples, the
    //! non-perfect `√` cases, and the adversarial seam/band-edge inputs) now
    //! lives in the shared golden set under `tests/golden/hypot_d*_s*.txt`
    //! and is checked, bit-exact across all six rounding modes, by
    //! `tests/ulp_strict_golden.rs` — the single source of truth. What
    //! remains here are the cases golden's `(input, floor, cls)` →
    //! `delta == 0` format cannot carry:
    //!
    //!   * `hypot(0, 0) = 0` bit-exact at every tier;
    //!   * `hypot(0, x) = |x|` (including the negative-operand mirror);
    //!   * the storage-overflow contract: `hypot(MAX, 0) = MAX` (fits, no
    //!     panic) versus `hypot(MAX, MAX) ≈ MAX·√2` (out of range → panic).
    //!
    //! A golden cell can only pin a representable result; the overflow case
    //! has no representable answer, so it is asserted here as a panic.

    use decimal_scaled::{RoundingMode, D18, D307, D38, D57};

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    #[test]
    fn hypot_zero_zero_is_zero_bit_exact_all_tiers_all_modes() {
        for mode in ALL_MODES {
            assert_eq!(
                D18::<6>::ZERO.hypot_strict_with(D18::<6>::ZERO, mode),
                D18::<6>::ZERO,
                "D18 hypot(0,0) mode {mode:?}",
            );
            assert_eq!(
                D38::<6>::ZERO.hypot_strict_with(D38::<6>::ZERO, mode),
                D38::<6>::ZERO,
                "D38 hypot(0,0) mode {mode:?}",
            );
            assert_eq!(
                D57::<6>::ZERO.hypot_strict_with(D57::<6>::ZERO, mode),
                D57::<6>::ZERO,
                "D57 hypot(0,0) mode {mode:?}",
            );
            assert_eq!(
                D307::<30>::ZERO.hypot_strict_with(D307::<30>::ZERO, mode),
                D307::<30>::ZERO,
                "D307 hypot(0,0) mode {mode:?}",
            );
        }
    }

    #[test]
    fn hypot_zero_x_is_abs_x_all_tiers_all_modes() {
        // hypot(0, x) = |x| exactly, and hypot(0, -x) = |x|.
        for &x in &[3i64, 7, 42, 100] {
            for mode in ALL_MODES {
                let d38 = D38::<6>::from(x);
                let d38n = D38::<6>::from(-x);
                assert_eq!(
                    D38::<6>::ZERO.hypot_strict_with(d38, mode),
                    d38,
                    "D38 hypot(0,{x}) mode {mode:?}",
                );
                assert_eq!(
                    D38::<6>::ZERO.hypot_strict_with(d38n, mode),
                    d38,
                    "D38 hypot(0,-{x}) mode {mode:?} (= |{x}|)",
                );
                let d307 = D307::<30>::from(x);
                assert_eq!(
                    D307::<30>::ZERO.hypot_strict_with(d307, mode),
                    d307,
                    "D307 hypot(0,{x}) mode {mode:?}",
                );
            }
        }
    }

    #[test]
    fn hypot_near_max_does_not_overflow_when_in_range() {
        // a near MAX, b = 0 -> hypot = a exactly, must not panic or overflow.
        let a = D38::<0>::MAX;
        assert_eq!(a.hypot_strict_with(D38::<0>::ZERO, RoundingMode::HalfToEven), a);
    }

    #[test]
    #[should_panic(expected = "hypot: result out of range")]
    fn hypot_overflow_panics_d38() {
        // a = b = MAX magnitude: hypot ~= MAX·√2 exceeds the type range.
        let m = D38::<0>::MAX;
        let _ = m.hypot_strict_with(m, RoundingMode::HalfToEven);
    }
}

#[cfg(all(not(feature = "fast"), not(any(feature = "rounding-half-away-from-zero", feature = "rounding-half-toward-zero", feature = "rounding-trunc", feature = "rounding-floor", feature = "rounding-ceiling"))))]
mod from_transcendental_overflow_uniform {
    //! Tier-invariant strict-transcendental overflow contract.
    //!
    //! `docs/ARCHITECTURE.md` → "Overflow & domain behaviour — one contract,
    //! invariant across tier and scale": a strict transcendental whose result
    //! is out of the storage range PANICS — identically at every width and
    //! scale, in BOTH debug and release. There is no ∞/NaN in a fixed-width
    //! decimal, so a wrapped/saturated value would be a silent wrong number;
    //! the default fails loudly instead.
    //!
    //! The historic bug this guards: a sufficiently large argument made the
    //! internal `exp` working-width arithmetic WRAP (`wrapping_sqr_low_u128`
    //! truncates to the low bits → an overflowed square collapses to 0), and
    //! the post-narrowing fit check — seeing only the small wrapped value —
    //! never fired, so `D18<0>::from(349).exp_strict()` returned `0` instead
    //! of panicking, while neighbouring arguments and the wide tiers panicked.
    //! `exp_generic::exp_fixed` now rejects such an argument up front, so the
    //! contract is uniform. Each `#[should_panic]` below fires in debug AND
    //! release because the panic route (`overflow_panic_with_scale` / the
    //! kernel's own `panic!`) is NOT gated behind `cfg!(debug_assertions)` —
    //! that gating is the separate i64-style ARITHMETIC contract, out of scope
    //! here.

    // The strict surface is the default build; skip under a non-default
    // rounding mode or the f64-bridge `fast` path (same gate as the sibling
    // `narrow_strict_transcendentals` suite).
    use decimal_scaled::{D18, D38};

    // ── Narrow tier (D18, i64 storage) ─────────────────────────────────────
    //
    // e^349 ≈ 10^151, far beyond i64. Pre-fix this RETURNED 0 (the internal
    // squaring wrapped the work integer to a value that survived the storage
    // fit check). Must panic, uniform with every other tier.

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_exp_far_overflow_panics() {
        let _ = D18::<0>::from(349).exp_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_exp2_far_overflow_panics() {
        // 2^400 ≈ 2.6e120, far beyond i64. Pre-fix returned 0.
        let _ = D18::<0>::from(400).exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_cosh_far_overflow_panics() {
        let _ = D18::<0>::from(500).cosh_strict();
    }

    // ── Mid tier (D38, i128 storage) ───────────────────────────────────────
    //
    // e^349 ≈ 10^151, far beyond i128. Pre-fix this RETURNED 0.

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp_far_overflow_panics() {
        let _ = D38::<0>::from(349).exp_strict();
    }

    // ── Wide tier (feature-gated) ──────────────────────────────────────────
    //
    // The wide tiers were the half that ALWAYS panicked (the contract the
    // narrow tier failed to match); the same far-out-of-range argument that
    // wrapped a wide work integer (e.g. D57 exp(1000) → 0 pre-fix) now panics
    // too, proving the fix closes the wrap on every tier — narrow AND wide.

    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide {
        use decimal_scaled::D57;

        #[test]
        #[should_panic(expected = "result out of range")]
        fn wide_d57_exp_far_overflow_panics() {
            // e^1000 ≈ 10^434, far beyond D57 storage AND its Wexp work integer.
            // Pre-fix this RETURNED 0 (the Wexp squaring wrapped).
            let _ = D57::<0>::from(1000).exp_strict();
        }

        #[test]
        #[should_panic(expected = "result out of range")]
        fn wide_d57_cosh_overflow_panics() {
            let _ = D57::<0>::from(140).cosh_strict();
        }
    }

    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    mod widest {
        use decimal_scaled::D1232;

        #[test]
        #[should_panic(expected = "result out of range")]
        fn widest_d1232_cosh_far_overflow_panics() {
            // cosh(5000) ≈ e^5000/2 ≈ 10^2171, far beyond D1232 storage. Pre-fix
            // this RETURNED a wrapped multi-limb value.
            let _ = D1232::<0>::from(5000).cosh_strict();
        }
    }

    // ── D76 storage-narrow gap (peak-margin regression guard) ──────────────
    //
    // D76 is the SOLE tier whose exp work integer `Wexp == W == Int<16>`
    // (1024 bits): a result overflowing the work integer cannot lift any
    // wider, so `exp_generic`'s peak gate IS the last line for it. The peak
    // model's flat margin was over-large (+512 bits ≈ half D76's work budget)
    // and false-panicked IN-RANGE band-edge cosh/sinh at D76<0> (those
    // in-range cells are covered by `ulp_strict_golden` band_edges); the
    // margin is now a small +64 slack. This pins the COUNTERPART case the
    // smaller margin newly routes through the storage-narrowing path: a cosh
    // whose result overflows STORAGE but whose internal squaring peak still
    // fits Int<16> — cosh(180) ≈ 7e77 is 78 digits (> D76's 76-digit storage)
    // while its internal peak is ≈ 764 bits (< 1024). It must still PANIC,
    // caught by the narrowing fit check (`round_to_storage_with_g`), not the
    // work-int peak gate — keeping the out-of-range contract uniform.
    #[cfg(any(feature = "d76", feature = "wide"))]
    mod d76_gap {
        use decimal_scaled::D76;

        #[test]
        #[should_panic(expected = "result out of range")]
        fn d76_cosh_storage_overflow_panics() {
            let _ = D76::<0>::from(180).cosh_strict();
        }
    }

    // ── In-range cells must still return a value (no false-positive panic) ──
    //
    // The guard must reject ONLY genuinely out-of-range results. These cells
    // sit just inside the storage edge and must compute normally.

    #[test]
    fn narrow_d18_exp_in_range_returns_value() {
        // e^43 ≈ 4.7e18 < i64::MAX ≈ 9.2e18.
        let _ = D18::<0>::from(43).exp_strict();
    }

    #[test]
    fn mid_d38_exp_in_range_returns_value() {
        // e^88 ≈ 1.65e38 < i128::MAX ≈ 1.70e38 — the last in-range integer arg.
        let _ = D38::<0>::from(88).exp_strict();
    }
}
