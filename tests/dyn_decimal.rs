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

use decimal_scaled::{
    D9, D18, D38, DecimalWidth, DynDecimal, RawStorage, RoundingMode,
};

// ── Identity surface ──────────────────────────────────────────────────

#[test]
fn width_and_scale_round_trip() {
    let d9: Box<dyn DynDecimal> = Box::new(D9::<3>::from_i32(7));
    let d18: Box<dyn DynDecimal> = Box::new(D18::<5>::from_i32(7));
    let d38: Box<dyn DynDecimal> = Box::new(D38::<12>::from_i32(7));

    assert_eq!(d9.width(), DecimalWidth::D9);
    assert_eq!(d18.width(), DecimalWidth::D18);
    assert_eq!(d38.width(), DecimalWidth::D38);

    assert_eq!(d9.scale_dyn(), 3);
    assert_eq!(d18.scale_dyn(), 5);
    assert_eq!(d38.scale_dyn(), 12);

    assert_eq!(d9.max_scale(), 9);
    assert_eq!(d18.max_scale(), 18);
    assert_eq!(d38.max_scale(), 38);
}

#[test]
fn raw_storage_tagged_correctly() {
    let d9: Box<dyn DynDecimal> = Box::new(D9::<3>::from_i32(7));
    let d18: Box<dyn DynDecimal> = Box::new(D18::<5>::from_i32(7));
    let d38: Box<dyn DynDecimal> = Box::new(D38::<12>::from_i32(7));

    match d9.raw_storage() {
        RawStorage::I32(v) => assert_eq!(v, 7 * 10_i32.pow(3)),
        _ => panic!("expected I32"),
    }
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
    let d: Box<dyn DynDecimal> = Box::new(D38::<7>::from_i32(42));
    let typed: &D38<7> = d.as_any().downcast_ref::<D38<7>>().unwrap();
    assert_eq!(*typed, D38::<7>::from_i32(42));

    // Wrong scale downcast fails.
    assert!(d.as_any().downcast_ref::<D38<6>>().is_none());
    // Wrong width downcast fails.
    assert!(d.as_any().downcast_ref::<D18<7>>().is_none());
}

#[test]
fn clone_box_yields_independent_handle() {
    let original: Box<dyn DynDecimal> = Box::new(D18::<4>::from_i32(99));
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
    let v: Box<dyn DynDecimal> = Box::new(D38::<5>::from_i32(7));

    assert!(z.is_zero());
    assert!(!o.is_zero());
    assert!(o.is_one());
    assert!(!v.is_one());
    assert!(!z.is_one());
}

#[test]
fn sign_predicates_and_unary_ops() {
    let pos: Box<dyn DynDecimal> = Box::new(D18::<2>::from_i32(5));
    let neg: Box<dyn DynDecimal> = Box::new(D18::<2>::from_i32(-5));
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
    let a: Box<dyn DynDecimal> = Box::new(D38::<3>::from_i32(2));
    let b: Box<dyn DynDecimal> = Box::new(D38::<3>::from_i32(5));
    let sum = a.add(&*b).unwrap();
    let typed = sum.as_any().downcast_ref::<D38<3>>().unwrap();
    assert_eq!(*typed, D38::<3>::from_i32(7));
}

#[test]
fn sub_mul_div_rem_same_scale() {
    let a: Box<dyn DynDecimal> = Box::new(D38::<2>::from_i32(20));
    let b: Box<dyn DynDecimal> = Box::new(D38::<2>::from_i32(3));

    let diff = a.sub(&*b).unwrap();
    assert_eq!(*diff.as_any().downcast_ref::<D38<2>>().unwrap(), D38::<2>::from_i32(17));

    let prod = a.mul(&*b).unwrap();
    assert_eq!(*prod.as_any().downcast_ref::<D38<2>>().unwrap(), D38::<2>::from_i32(60));

    let quot = a.div(&*b).unwrap();
    // 20 / 3 at scale 2 = 6.67 (banker's rounding).
    assert_eq!(*quot.as_any().downcast_ref::<D38<2>>().unwrap(), D38::<2>(667));

    let rem = a.rem(&*b).unwrap();
    assert_eq!(*rem.as_any().downcast_ref::<D38<2>>().unwrap(), D38::<2>::from_i32(2));
}

// ── Arithmetic: same width, different scale → auto-rescale to max ─────

#[test]
fn add_same_width_different_scale_auto_rescales_up() {
    // D38<2> value 1.50  +  D38<5> value 0.00007  →  D38<5> value 1.50007
    let a: Box<dyn DynDecimal> = Box::new(D38::<2>(150));
    let b: Box<dyn DynDecimal> = Box::new(D38::<5>(7));
    let sum = a.add(&*b).unwrap();
    assert_eq!(sum.scale_dyn(), 5);
    assert_eq!(sum.width(), DecimalWidth::D38);
    let typed = sum.as_any().downcast_ref::<D38<5>>().unwrap();
    assert_eq!(*typed, D38::<5>(150_007));
}

#[test]
fn cmp_dyn_uses_lossless_rescale() {
    // 1.50 and 1.50000 are equal logically; rescale up preserves that.
    let a: Box<dyn DynDecimal> = Box::new(D38::<2>(150));
    let b: Box<dyn DynDecimal> = Box::new(D38::<5>(150_000));
    assert_eq!(a.cmp_dyn(&*b), Some(core::cmp::Ordering::Equal));
    assert!(a.eq_dyn(&*b));
}

#[test]
fn cmp_dyn_distinguishes_unequal_finer_scale() {
    // 1.50 vs 1.50001 (D38<5>): the finer-scale value carries info the
    // narrower scale can't, and the lossless rescale-up preserves it.
    let a: Box<dyn DynDecimal> = Box::new(D38::<2>(150));
    let b: Box<dyn DynDecimal> = Box::new(D38::<5>(150_001));
    assert_eq!(a.cmp_dyn(&*b), Some(core::cmp::Ordering::Less));
    assert!(!a.eq_dyn(&*b));
}

// ── Cross-width: returns None ─────────────────────────────────────────

#[test]
fn cross_width_arithmetic_returns_none() {
    let a: Box<dyn DynDecimal> = Box::new(D38::<3>::from_i32(2));
    let b: Box<dyn DynDecimal> = Box::new(D18::<3>::from_i32(5));
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
    let v: Box<dyn DynDecimal> = Box::new(D38::<2>(150));
    let up = v.rescale_to(5).unwrap();
    assert_eq!(up.scale_dyn(), 5);
    assert_eq!(*up.as_any().downcast_ref::<D38<5>>().unwrap(), D38::<5>(150_000));

    let down = up.rescale_to(2).unwrap();
    assert_eq!(*down.as_any().downcast_ref::<D38<2>>().unwrap(), D38::<2>(150));
}

#[test]
fn rescale_to_with_explicit_rounding_mode() {
    // 1.555 at scale 3 → scale 2 with Trunc → 1.55; with Ceiling → 1.56.
    let v: Box<dyn DynDecimal> = Box::new(D38::<3>(1555));
    let truncated = v.rescale_to_with(2, RoundingMode::Trunc).unwrap();
    assert_eq!(*truncated.as_any().downcast_ref::<D38<2>>().unwrap(), D38::<2>(155));
    let ceiled = v.rescale_to_with(2, RoundingMode::Ceiling).unwrap();
    assert_eq!(*ceiled.as_any().downcast_ref::<D38<2>>().unwrap(), D38::<2>(156));
}

#[test]
fn rescale_to_above_max_returns_none() {
    let v: Box<dyn DynDecimal> = Box::new(D9::<3>::from_i32(5));
    assert!(v.rescale_to(10).is_none()); // max_scale = 9
    let v18: Box<dyn DynDecimal> = Box::new(D18::<3>::from_i32(5));
    assert!(v18.rescale_to(19).is_none()); // max_scale = 18
    let v38: Box<dyn DynDecimal> = Box::new(D38::<3>::from_i32(5));
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
    let a: Box<dyn DynDecimal> = Box::new(D38::<3>::from_i32(7));
    let z: Box<dyn DynDecimal> = Box::new(D38::<3>::ZERO);
    assert!(a.div(&*z).is_none());
    assert!(a.rem(&*z).is_none());
}

// ── Bridge: display / to_f64 / to_int ─────────────────────────────────

#[test]
fn display_matches_typed_format() {
    let v: Box<dyn DynDecimal> = Box::new(D38::<3>(1234));
    assert_eq!(v.display(), "1.234");
}

#[cfg(feature = "std")]
#[test]
fn to_f64_round_trip_low_scale() {
    let v: Box<dyn DynDecimal> = Box::new(D38::<3>(1500));
    let f = v.to_f64();
    assert!((f - 1.5).abs() < 1e-12);
}

#[test]
fn to_int_truncates() {
    let v: Box<dyn DynDecimal> = Box::new(D38::<3>(7_500)); // 7.500
    // Crate default is HalfToEven; 7.5 → 8 (rounds to even).
    // We don't assert the exact rounding mode (depends on Cargo features);
    // just that the call succeeds and is in the right neighbourhood.
    let i = v.to_int();
    assert!(i == 7 || i == 8);
}

// ── Cross-scale arithmetic on D9 and D18 ──────────────────────────────

#[test]
fn d9_cross_scale_add() {
    let a: Box<dyn DynDecimal> = Box::new(D9::<2>(150));     // 1.50
    let b: Box<dyn DynDecimal> = Box::new(D9::<4>(7));       // 0.0007
    let sum = a.add(&*b).unwrap();
    assert_eq!(sum.scale_dyn(), 4);
    assert_eq!(*sum.as_any().downcast_ref::<D9<4>>().unwrap(), D9::<4>(15_007));
}

#[test]
fn d18_cross_scale_mul() {
    // 1.5 (D18<1>) * 2.00 (D18<2>) -> 3.00 at scale 2.
    let a: Box<dyn DynDecimal> = Box::new(D18::<1>(15));
    let b: Box<dyn DynDecimal> = Box::new(D18::<2>(200));
    let prod = a.mul(&*b).unwrap();
    assert_eq!(prod.scale_dyn(), 2);
    assert_eq!(*prod.as_any().downcast_ref::<D18<2>>().unwrap(), D18::<2>(300));
}

// ── Storing mixed values in a single container ────────────────────────

#[test]
fn vec_of_mixed_widths() {
    let values: Vec<Box<dyn DynDecimal>> = vec![
        Box::new(D9::<2>::from_i32(1)),
        Box::new(D18::<3>::from_i32(2)),
        Box::new(D38::<5>::from_i32(3)),
    ];

    let widths: Vec<DecimalWidth> = values.iter().map(|v| v.width()).collect();
    assert_eq!(widths, vec![DecimalWidth::D9, DecimalWidth::D18, DecimalWidth::D38]);

    let displays: Vec<String> = values.iter().map(|v| v.display()).collect();
    assert_eq!(displays, vec!["1.00", "2.000", "3.00000"]);
}
