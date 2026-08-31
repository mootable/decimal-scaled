//! The deprecated 0.5.0 spellings still delegate to their replacements.
//!
//! `rescale` / `rescale_with` were renamed to `quantize` / `quantize_with`
//! in 0.5.1 and are removed in 0.6.0. Until then they must behave
//! identically, so these tests pin the delegation itself rather than the
//! arithmetic — `quantize`'s own suite already covers that.

use decimal_scaled::{D18, D38, RoundingMode};

#[test]
#[allow(deprecated)]
fn rescale_matches_quantize() {
    let a = D38::<6>::try_from(7i64).unwrap();
    assert_eq!(a.rescale::<2>(), a.quantize::<2>());
}

#[test]
#[allow(deprecated)]
fn rescale_with_matches_quantize_with_in_every_mode() {
    // 0.005 -> SCALE 2 is a tie, so the mode must survive the delegation.
    let a = D38::<6>::from_bits(decimal_scaled::Int::<2>::from(5_000i64));
    for mode in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        assert_eq!(a.rescale_with::<2>(mode), a.quantize_with::<2>(mode));
    }
}

#[test]
fn with_scale_matches_quantize() {
    // `with_scale` is the builder-style alias and is NOT deprecated; it now
    // delegates to `quantize`.
    let a = D18::<2>::try_from(3i64).unwrap();
    assert_eq!(a.with_scale::<5>(), a.quantize::<5>());
}

/// The `DynDecimal` facade carries the same rename, so its deprecated
/// spellings need the same delegation guarantee.
#[cfg(feature = "dyn")]
mod dyn_facade {
    use decimal_scaled::{D38, DynDecimal, RoundingMode};

    #[test]
    #[allow(deprecated)]
    fn rescale_to_matches_quantize_to() {
        let v: Box<dyn DynDecimal> = Box::new(D38::<2>::try_from(15i64).unwrap());
        let via_alias = v.rescale_to(5).unwrap();
        let via_quantize = v.quantize_to(5).unwrap();
        assert!(via_alias.eq_dyn(&*via_quantize));
    }

    #[test]
    #[allow(deprecated)]
    fn rescale_to_with_matches_quantize_to_with() {
        let v: Box<dyn DynDecimal> = Box::new(D38::<3>::try_from(1i64).unwrap());
        let via_alias = v.rescale_to_with(2, RoundingMode::Trunc).unwrap();
        let via_quantize = v.quantize_to_with(2, RoundingMode::Trunc).unwrap();
        assert!(via_alias.eq_dyn(&*via_quantize));
    }
}
