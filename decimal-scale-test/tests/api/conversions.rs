//! Public conversion surface.
//! Migrated from `tests/widen_narrow_default.rs`.

#[cfg(feature = "wide")]
mod from_widen_narrow_default {
    //! Coverage of the `widen()` / `narrow()` hop methods on the legacy
    //! D38 / D76 / D153 / D307 tiers — now updated to step through the
    //! comprehensive ladder (D38 → D57 → D76 → D115 → D153 → D230 → D307
    //! → D462) rather than skipping straight to the next power-of-two
    //! width. Plus the per-tier `Default` impl coverage.

    use decimal_scaled::{D18, D38, D57, D76, D115};

    #[test]
    fn d38_widen_to_d57() {
        let a = D38::<12>::from(7);
        let w: D57<12> = a.widen();
        let expected: D57<12> = a.into();
        assert_eq!(w, expected);
    }

    #[test]
    fn d76_narrow_to_d57_in_range() {
        // D38 -> D57 -> D76 widens losslessly, then D76.narrow() back to
        // D57 should recover the value.
        let small: D57<12> = D38::<12>::from(7).into();
        let w: D76<12> = small.widen();
        let n: D57<12> = w.narrow().unwrap();
        assert_eq!(n.to_bits().to_string(), small.to_bits().to_string());
    }

    #[test]
    fn d76_narrow_to_d57_out_of_range_errors() {
        // D76<0>::MAX is way above D57<0>::MAX
        let w = D76::<0>::MAX;
        let r: Result<D57<0>, _> = w.narrow();
        assert!(r.is_err());
    }

    #[test]
    fn d76_widen_to_d115() {
        let a: D76<6> = D38::<6>::from(7).into();
        let b: D115<6> = a.widen();
        let n: D76<6> = b.narrow().unwrap();
        assert_eq!(n, a);
    }

    #[test]
    fn defaults_per_tier() {
        assert_eq!(D76::<2>::default(), D76::<2>::ZERO);
        #[cfg(feature = "x-wide")]
        {
            use decimal_scaled::{D153, D307};
            assert_eq!(D153::<2>::default(), D153::<2>::ZERO);
            assert_eq!(D307::<2>::default(), D307::<2>::ZERO);
        }
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d153_widen_to_d230_then_d307() {
        use decimal_scaled::{D153, D230, D307};
        let a: D153<6> = D76::<6>::from(7).widen().widen(); // D76 -> D115 -> D153
        let b: D230<6> = a.widen();
        let n: D153<6> = b.narrow().unwrap();
        assert_eq!(n, a);

        let c: D307<6> = b.widen();
        let n: D230<6> = c.narrow().unwrap();
        assert_eq!(n, b);

        // Out-of-range narrow stays the same shape.
        let big_153 = D153::<0>::MAX;
        let r: Result<D115<0>, _> = big_153.narrow();
        assert!(r.is_err());
        let big_307 = D307::<0>::MAX;
        let r: Result<D230<0>, _> = big_307.narrow();
        assert!(r.is_err());
    }

    // ─── Const-generic `widen_n` / `narrow_n` sugar (story 1.2.2) ──────────

    #[test]
    fn widen_n_d18_to_d38_lossless() {
        // D18 (Int<1>) → D38 (Int<2>), same scale, exact.
        let a = D18::<9>::from(7);
        let w: D38<9> = a.widen_n::<2>();
        // Same logical value: widening sign-extends, scale unchanged.
        assert_eq!(i128::from(w.to_bits()), i128::from(a.to_bits()));
    }

    #[test]
    fn widen_n_is_const() {
        // The const-generic sugar must be usable in const context — this is
        // the property that distinguishes it from the trait `From` widen.
        const A: D18<2> = D18::<2>::from_bits(decimal_scaled::Int::<1>::from_limbs([700]));
        const W: D38<2> = A.widen_n::<2>();
        assert_eq!(i128::from(W.to_bits()), 700);
    }

    #[test]
    fn narrow_n_d38_to_d18_in_range_and_out() {
        // In range: D38 value that fits Int<1> narrows back exactly.
        let a = D38::<2>::from(7);
        let n: Option<D18<2>> = a.narrow_n::<1>();
        assert!(n.is_some());
        assert_eq!(n.unwrap().to_bits(), 700);

        // Out of range: D38::MAX cannot fit Int<1> → None.
        let big = D38::<0>::MAX;
        let n: Option<D18<0>> = big.narrow_n::<1>();
        assert!(n.is_none());
    }

    #[test]
    fn narrow_n_const_is_const() {
        const A: D38<0> = D38::<0>::from_bits(decimal_scaled::Int::<2>::from_limbs([5, 0]));
        const N: Option<D18<0>> = A.narrow_n::<1>();
        assert!(N.is_some());
        assert_eq!(N.unwrap().to_bits(), 5);
    }

    // COMPILE-TIME LOCK — D18 (`Int<1>`) is the narrowest decimal storage,
    // so it has no neighbour `narrow()` method (only `widen()`). The line
    // below, if uncommented, must fail to compile (no such method) — this
    // pins the "nothing narrower than Int<1>" contract.
    //
    //   let _ = D18::<2>::try_from(1).unwrap().narrow();   // E0599: no method `narrow`
    //
    // The const-generic `narrow_n::<0>()` likewise has no meaning: the int
    // base's `try_narrow` debug-asserts `1 <= M`, so a width-0 storage is
    // rejected rather than silently produced.

    #[cfg(feature = "x-wide")]
    #[test]
    fn cross_width_narrowing_d76_to_d18_d9() {
        // Cross-tier TryFrom skips multiple rungs in one hop; this isn't
        // the `.narrow()` chain (which steps once) — it's the From /
        // TryFrom matrix that's been comprehensive since 0.2.5.
        let w: D76<2> = D38::<2>::from(7).into();
        let n18: D18<2> = w.try_into().unwrap();
        assert_eq!(n18.to_bits(), 700);

        // Out of range.
        let big = D76::<2>::MAX;
        let r: Result<D18<2>, _> = big.try_into();
        assert!(r.is_err());
    }
}
