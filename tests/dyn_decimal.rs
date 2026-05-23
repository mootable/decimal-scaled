//! Behavioural tests for the [`DynDecimal`] runtime-polymorphic façade.
//!
//! Covers:
//! - Construction by coercion from each shipped width.
//! - Identity surface (`width`, `scale_dyn`, `max_scale`, `raw_storage`).
//! - Downcast back to the typed surface via `as_any`.
//! - Same-width same-scale arithmetic returns the typed result.
//! - Same-width different-scale arithmetic auto-rescales to the wider
//!   scale.
//! - Cross-width arithmetic returns `None`.
//! - `rescale_to` to legal / illegal scales.
//! - Overflow paths return `None` instead of panicking.
//! - `eq_dyn` / `cmp_dyn` honour the same width + lossless rescale rule.
//! - `display` / `to_f64` / `to_int` bridge the typed surface.

#![cfg(feature = "dyn")]

use decimal_scaled::{D18, D38, DecimalWidth, DynDecimal, RawStorage, RoundingMode};

// ── Identity surface ──────────────────────────────────────────────────

#[test]
fn width_and_scale_round_trip() {
    let d18: Box<dyn DynDecimal> = Box::new(D18::<5>::try_from(7).unwrap());
    let d38: Box<dyn DynDecimal> = Box::new(D38::<12>::try_from(7).unwrap());

    assert_eq!(d18.width(), DecimalWidth::D18);
    assert_eq!(d38.width(), DecimalWidth::D38);

    assert_eq!(d18.scale_dyn(), 5);
    assert_eq!(d38.scale_dyn(), 12);

    assert_eq!(d18.max_scale(), 18);
    assert_eq!(d38.max_scale(), 38);
}

#[test]
fn raw_storage_tagged_correctly() {
    let d18: Box<dyn DynDecimal> = Box::new(D18::<5>::try_from(7).unwrap());
    let d38: Box<dyn DynDecimal> = Box::new(D38::<12>::try_from(7).unwrap());

    match d18.raw_storage() {
        RawStorage::I64(v) => assert_eq!(v, 7 * 10_i64.pow(5)),
        _ => panic!("expected I64"),
    }
    match d38.raw_storage() {
        RawStorage::I128(v) => assert_eq!(v, 7 * 10_i128.pow(12)),
        _ => panic!("expected I128"),
    }
}

#[test]
fn as_any_downcasts_to_concrete_type() {
    let d: Box<dyn DynDecimal> = Box::new(D38::<7>::try_from(42).unwrap());
    let typed: &D38<7> = d.as_any().downcast_ref::<D38<7>>().unwrap();
    assert_eq!(*typed, D38::<7>::try_from(42).unwrap());

    // Wrong scale downcast fails.
    assert!(d.as_any().downcast_ref::<D38<6>>().is_none());
    // Wrong width downcast fails.
    assert!(d.as_any().downcast_ref::<D18<7>>().is_none());
}

#[test]
fn clone_box_yields_independent_handle() {
    let original: Box<dyn DynDecimal> = Box::new(D18::<4>::try_from(99).unwrap());
    let cloned = original.clone_box();
    assert!(original.eq_dyn(&*cloned));
    assert_eq!(original.scale_dyn(), cloned.scale_dyn());
    assert_eq!(original.width(), cloned.width());
}

// ── Predicates ────────────────────────────────────────────────────────

#[test]
fn zero_and_one_predicates() {
    let z: Box<dyn DynDecimal> = Box::new(D38::<5>::ZERO);
    let o: Box<dyn DynDecimal> = Box::new(D38::<5>::ONE);
    let v: Box<dyn DynDecimal> = Box::new(D38::<5>::try_from(7).unwrap());

    assert!(z.is_zero());
    assert!(!o.is_zero());
    assert!(o.is_one());
    assert!(!v.is_one());
    assert!(!z.is_one());
}

#[test]
fn sign_predicates_and_unary_ops() {
    let pos: Box<dyn DynDecimal> = Box::new(D18::<2>::try_from(5).unwrap());
    let neg: Box<dyn DynDecimal> = Box::new(D18::<2>::try_from(-5).unwrap());
    let z: Box<dyn DynDecimal> = Box::new(D18::<2>::ZERO);

    assert!(pos.is_positive());
    assert!(!pos.is_negative());
    assert!(neg.is_negative());
    assert!(!neg.is_positive());
    assert!(!z.is_positive() && !z.is_negative());

    let abs = neg.abs();
    assert!(abs.eq_dyn(&*pos));

    let signed = pos.signum();
    let one_box: Box<dyn DynDecimal> = Box::new(D18::<2>::ONE);
    assert!(signed.eq_dyn(&*one_box));

    let flipped = pos.neg();
    assert!(flipped.eq_dyn(&*neg));
}

// ── Arithmetic: same width, same scale ────────────────────────────────

#[test]
fn add_same_width_same_scale() {
    let a: Box<dyn DynDecimal> = Box::new(D38::<3>::try_from(2).unwrap());
    let b: Box<dyn DynDecimal> = Box::new(D38::<3>::try_from(5).unwrap());
    let sum = a.add(&*b).unwrap();
    let typed = sum.as_any().downcast_ref::<D38<3>>().unwrap();
    assert_eq!(*typed, D38::<3>::try_from(7).unwrap());
}

#[test]
fn sub_mul_div_rem_same_scale() {
    let a: Box<dyn DynDecimal> = Box::new(D38::<2>::try_from(20).unwrap());
    let b: Box<dyn DynDecimal> = Box::new(D38::<2>::try_from(3).unwrap());

    let diff = a.sub(&*b).unwrap();
    assert_eq!(
        *diff.as_any().downcast_ref::<D38<2>>().unwrap(),
        D38::<2>::try_from(17).unwrap()
    );

    let prod = a.mul(&*b).unwrap();
    assert_eq!(
        *prod.as_any().downcast_ref::<D38<2>>().unwrap(),
        D38::<2>::try_from(60).unwrap()
    );

    let quot = a.div(&*b).unwrap();
    // 20 / 3 at scale 2 = 6.6666… → rounds to 6.67 (667 at S=2), matching
    // the `/` operator; dyn div rounds via checked_div, it does not truncate.
    assert_eq!(
        *quot.as_any().downcast_ref::<D38<2>>().unwrap(),
        D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((667) as i128).unwrap())
    );

    let rem = a.rem(&*b).unwrap();
    assert_eq!(
        *rem.as_any().downcast_ref::<D38<2>>().unwrap(),
        D38::<2>::try_from(2).unwrap()
    );
}

// ── Arithmetic: same width, different scale → auto-rescale to max ─────

#[test]
fn add_same_width_different_scale_auto_rescales_up() {
    // D38<2> value 1.50  +  D38<5> value 0.00007  →  D38<5> value 1.50007
    let a: Box<dyn DynDecimal> =
        Box::new(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap()));
    let b: Box<dyn DynDecimal> =
        Box::new(D38::<5>::from_bits(decimal_scaled::Int::<2>::try_from((7) as i128).unwrap()));
    let sum = a.add(&*b).unwrap();
    assert_eq!(sum.scale_dyn(), 5);
    assert_eq!(sum.width(), DecimalWidth::D38);
    let typed = sum.as_any().downcast_ref::<D38<5>>().unwrap();
    assert_eq!(
        *typed,
        D38::<5>::from_bits(decimal_scaled::Int::<2>::try_from((150_007) as i128).unwrap())
    );
}

#[test]
fn cmp_dyn_uses_lossless_rescale() {
    // 1.50 and 1.50000 are equal logically; rescale up preserves that.
    let a: Box<dyn DynDecimal> =
        Box::new(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap()));
    let b: Box<dyn DynDecimal> =
        Box::new(D38::<5>::from_bits(decimal_scaled::Int::<2>::try_from((150_000) as i128).unwrap()));
    assert_eq!(a.cmp_dyn(&*b), Some(core::cmp::Ordering::Equal));
    assert!(a.eq_dyn(&*b));
}

#[test]
fn cmp_dyn_distinguishes_unequal_finer_scale() {
    // 1.50 vs 1.50001 (D38<5>): the finer-scale value carries info the
    // narrower scale can't, and the lossless rescale-up preserves it.
    let a: Box<dyn DynDecimal> =
        Box::new(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap()));
    let b: Box<dyn DynDecimal> =
        Box::new(D38::<5>::from_bits(decimal_scaled::Int::<2>::try_from((150_001) as i128).unwrap()));
    assert_eq!(a.cmp_dyn(&*b), Some(core::cmp::Ordering::Less));
    assert!(!a.eq_dyn(&*b));
}

// ── Cross-width: returns None ─────────────────────────────────────────

#[test]
fn cross_width_arithmetic_returns_none() {
    let a: Box<dyn DynDecimal> = Box::new(D38::<3>::try_from(2).unwrap());
    let b: Box<dyn DynDecimal> = Box::new(D18::<3>::try_from(5).unwrap());
    assert!(a.add(&*b).is_none());
    assert!(a.sub(&*b).is_none());
    assert!(a.mul(&*b).is_none());
    assert!(a.div(&*b).is_none());
    assert!(a.rem(&*b).is_none());
    assert!(!a.eq_dyn(&*b));
    assert_eq!(a.cmp_dyn(&*b), None);
}

// ── Rescale ───────────────────────────────────────────────────────────

#[test]
fn rescale_to_within_range() {
    let v: Box<dyn DynDecimal> =
        Box::new(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap()));
    let up = v.rescale_to(5).unwrap();
    assert_eq!(up.scale_dyn(), 5);
    assert_eq!(
        *up.as_any().downcast_ref::<D38<5>>().unwrap(),
        D38::<5>::from_bits(decimal_scaled::Int::<2>::try_from((150_000) as i128).unwrap())
    );

    let down = up.rescale_to(2).unwrap();
    assert_eq!(
        *down.as_any().downcast_ref::<D38<2>>().unwrap(),
        D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap())
    );
}

#[test]
fn rescale_to_with_explicit_rounding_mode() {
    // 1.555 at scale 3 → scale 2 with Trunc → 1.55; with Ceiling → 1.56.
    let v: Box<dyn DynDecimal> =
        Box::new(D38::<3>::from_bits(decimal_scaled::Int::<2>::try_from((1555) as i128).unwrap()));
    let truncated = v.rescale_to_with(2, RoundingMode::Trunc).unwrap();
    assert_eq!(
        *truncated.as_any().downcast_ref::<D38<2>>().unwrap(),
        D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((155) as i128).unwrap())
    );
    let ceiled = v.rescale_to_with(2, RoundingMode::Ceiling).unwrap();
    assert_eq!(
        *ceiled.as_any().downcast_ref::<D38<2>>().unwrap(),
        D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((156) as i128).unwrap())
    );
}

#[test]
fn rescale_to_above_max_returns_none() {
    let v18: Box<dyn DynDecimal> = Box::new(D18::<3>::try_from(5).unwrap());
    assert!(v18.rescale_to(19).is_none()); // max_scale = 18
    let v38: Box<dyn DynDecimal> = Box::new(D38::<3>::try_from(5).unwrap());
    assert!(v38.rescale_to(39).is_none()); // max_scale = 38
}

#[test]
fn rescale_overflow_returns_none() {
    // D38::MAX rescaled up by even one digit overflows.
    let v: Box<dyn DynDecimal> = Box::new(D38::<0>::MAX);
    assert!(v.rescale_to(1).is_none());
}

// ── Overflow paths return None ────────────────────────────────────────

#[test]
fn add_overflow_returns_none() {
    let a: Box<dyn DynDecimal> = Box::new(D38::<0>::MAX);
    let b: Box<dyn DynDecimal> = Box::new(D38::<0>::ONE);
    assert!(a.add(&*b).is_none());
}

#[test]
fn div_by_zero_returns_none() {
    let a: Box<dyn DynDecimal> = Box::new(D38::<3>::try_from(7).unwrap());
    let z: Box<dyn DynDecimal> = Box::new(D38::<3>::ZERO);
    assert!(a.div(&*z).is_none());
    assert!(a.rem(&*z).is_none());
}

// ── Bridge: display / to_f64 / to_int ─────────────────────────────────

#[test]
fn display_matches_typed_format() {
    let v: Box<dyn DynDecimal> =
        Box::new(D38::<3>::from_bits(decimal_scaled::Int::<2>::try_from((1234) as i128).unwrap()));
    assert_eq!(v.display(), "1.234");
}

#[cfg(feature = "std")]
#[test]
fn to_f64_round_trip_low_scale() {
    let v: Box<dyn DynDecimal> =
        Box::new(D38::<3>::from_bits(decimal_scaled::Int::<2>::try_from((1500) as i128).unwrap()));
    let f = v.to_f64();
    assert!((f - 1.5).abs() < 1e-12);
}

#[test]
fn to_int_truncates() {
    let v: Box<dyn DynDecimal> =
        Box::new(D38::<3>::from_bits(decimal_scaled::Int::<2>::try_from((7_500) as i128).unwrap())); // 7.500
    // Crate default is HalfToEven; 7.5 → 8 (rounds to even).
    // We don't assert the exact rounding mode (depends on Cargo features);
    // just that the call succeeds and is in the right neighbourhood.
    let i = v.to_int();
    assert!(i == 7 || i == 8);
}

// ── Cross-scale arithmetic on D18 ────────────────────────────────────

#[test]
fn d18_cross_scale_mul() {
    // 1.5 (D18<1>) * 2.00 (D18<2>) -> 3.00 at scale 2.
    let a: Box<dyn DynDecimal> = Box::new(D18::<1>::from_bits(decimal_scaled::Int::<1>::from((15) as i64)));
    let b: Box<dyn DynDecimal> = Box::new(D18::<2>::from_bits(decimal_scaled::Int::<1>::from((200) as i64)));
    let prod = a.mul(&*b).unwrap();
    assert_eq!(prod.scale_dyn(), 2);
    assert_eq!(
        *prod.as_any().downcast_ref::<D18<2>>().unwrap(),
        D18::<2>::from_bits(decimal_scaled::Int::<1>::from((300) as i64))
    );
}

// ── Storing mixed values in a single container ────────────────────────

#[test]
fn vec_of_mixed_widths() {
    let values: Vec<Box<dyn DynDecimal>> = vec![
        Box::new(D18::<3>::try_from(2).unwrap()),
        Box::new(D38::<5>::try_from(3).unwrap()),
    ];

    let widths: Vec<DecimalWidth> = values.iter().map(|v| v.width()).collect();
    assert_eq!(widths, vec![DecimalWidth::D18, DecimalWidth::D38]);

    let displays: Vec<String> = values.iter().map(|v| v.display()).collect();
    assert_eq!(displays, vec!["2.000", "3.00000"]);
}

// ── 0.5.0 tier-table audit: dyn round-trip + widen-scale ──────────────

/// After the 0.5.0 rewrite (D9 removed, `Int<N>` storage, native backend
/// gone) the dyn tier tables must still: box each shipped narrow tier as
/// `dyn DynDecimal`, recover the value bit-exact through `as_any`, and
/// widen the scale via `rescale_to` to land on the SAME tier at the new
/// scale with the storage scaled by exactly `10^(target - source)`.
#[test]
fn dyn_round_trip_and_widen_scale_across_tiers() {
    // D18 (Int<1> / I64): box, recover bit-exact, then widen 2 -> 6.
    let d18_src = D18::<2>::try_from(7).unwrap();
    let d18_box: Box<dyn DynDecimal> = Box::new(d18_src);
    assert_eq!(d18_box.width(), DecimalWidth::D18);
    // Bit-exact recovery through the dyn boundary.
    assert_eq!(*d18_box.as_any().downcast_ref::<D18<2>>().unwrap(), d18_src);
    // Widen-scale: same tier, new scale, storage scaled by 10^4 (exact).
    let d18_wide = d18_box.rescale_to(6).unwrap();
    assert_eq!(d18_wide.width(), DecimalWidth::D18);
    assert_eq!(d18_wide.scale_dyn(), 6);
    assert_eq!(
        *d18_wide.as_any().downcast_ref::<D18<6>>().unwrap(),
        D18::<6>::try_from(7).unwrap()
    );
    match d18_wide.raw_storage() {
        RawStorage::I64(v) => assert_eq!(v, 7 * 10_i64.pow(6)),
        _ => panic!("D18 must tag as RawStorage::I64"),
    }

    // D38 (Int<2> / I128): box, recover bit-exact, then widen 5 -> 20.
    let d38_src = D38::<5>::try_from(7).unwrap();
    let d38_box: Box<dyn DynDecimal> = Box::new(d38_src);
    assert_eq!(d38_box.width(), DecimalWidth::D38);
    assert_eq!(*d38_box.as_any().downcast_ref::<D38<5>>().unwrap(), d38_src);
    let d38_wide = d38_box.rescale_to(20).unwrap();
    assert_eq!(d38_wide.width(), DecimalWidth::D38);
    assert_eq!(d38_wide.scale_dyn(), 20);
    assert_eq!(
        *d38_wide.as_any().downcast_ref::<D38<20>>().unwrap(),
        D38::<20>::try_from(7).unwrap()
    );
    match d38_wide.raw_storage() {
        RawStorage::I128(v) => assert_eq!(v, 7 * 10_i128.pow(20)),
        _ => panic!("D38 must tag as RawStorage::I128"),
    }

    // Rescale above the tier's MAX_SCALE returns None (no panic).
    assert!(d18_box.rescale_to(d18_box.max_scale() + 1).is_none());
    assert!(d38_box.rescale_to(d38_box.max_scale() + 1).is_none());

    // Widened operands round-trip the wider-scale rule through `add`:
    // D38<5> + D38<20> = D38<20>, value 14.
    let sum = d38_box.add(&*d38_wide).unwrap();
    assert_eq!(sum.width(), DecimalWidth::D38);
    assert_eq!(sum.scale_dyn(), 20);
    assert_eq!(
        *sum.as_any().downcast_ref::<D38<20>>().unwrap(),
        D38::<20>::try_from(14).unwrap()
    );
}
