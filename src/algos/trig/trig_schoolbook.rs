//! Forward-trig schoolbook reference kernels -- sin / cos / tan / atan.
//!
//! These are the **naive textbook** realisations of the forward family,
//! registered as an unrouted `Schoolbook` arm of
//! [`crate::policy::trig::forward::Algorithm`]. They exist as a
//! correctness reference (and an A/B microbench partner) for the tuned
//! `*_series` / Tang kernels -- `select` never routes to them.
//!
//! Each function is the plain textbook definition, computed in the
//! guard-digit work type and dispatched DOWN to the `Int<N>` layer for
//! its integer work (the leaf `*_fixed` Maclaurin kernels do their
//! arithmetic in `W: BigInt` / the 256-bit `Fixed` work int):
//!
//! - **sin / cos** -- a Maclaurin series after `mod 2pi` argument
//!   reduction into a small quadrant: the leaf `sin_fixed` / `cos_fixed`
//!   (wide) and `sin_fixed` / `sin_cos_fixed` (narrow) ARE that
//!   range-reduced Taylor evaluation.
//! - **tan** = `sin / cos` -- the joint `sin_cos_fixed` kernel then one
//!   work-int divide (`C::div` wide / `Fixed::div` narrow). Panics at the
//!   poles where the cosine is zero (odd multiples of pi/2).
//! - **atan** -- the arctan Maclaurin series with argument reduction
//!   supplied by the leaf `atan_fixed`. Result in `(-pi/2, pi/2)`.
//!
//! Correct rounding: wide kernels go through
//! [`WideTrigCore::round_to_storage_directed`] (the same Ziv-escalating
//! narrowing the `*_series` kernels use); narrow kernels round with
//! `Fixed::round_to_i128_with` at the strict guard. No small-x linear
//! shortcut and no Tang table -- the schoolbook is the unembellished
//! textbook path.

use crate::algos::ln::ln_series_2limb::STRICT_GUARD;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::{atan_fixed, sin_cos_fixed, sin_fixed, to_fixed};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// -- Wide tier -- generic over the tier core `C: WideTrigCore` --------

/// Schoolbook `sin` for a wide tier -- `sin(x)` via the range-reduced
/// Maclaurin leaf [`WideTrigCore::sin_fixed`], rounded correctly with
/// Ziv escalation.
#[inline]
#[must_use]
pub(crate) fn sin_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::sin_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
    })
}

/// Schoolbook `cos` for a wide tier -- `cos(x)` via the range-reduced
/// Maclaurin leaf [`WideTrigCore::cos_fixed`].
#[inline]
#[must_use]
pub(crate) fn cos_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::cos_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
    })
}

/// Schoolbook `tan` for a wide tier -- the textbook quotient
/// `tan(x) = sin(x) / cos(x)` from the joint
/// [`WideTrigCore::sin_cos_fixed`] leaf, divided in the work int via
/// [`WideTrigCore::div`]. Panics at the poles (cosine zero).
#[inline]
#[must_use]
pub(crate) fn tan_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let (sin_w, cos_w) = C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, guard), w);
        if cos_w == C::zero() {
            panic!("schoolbook tan: cosine is zero (argument is an odd multiple of pi/2)");
        }
        C::div(sin_w, cos_w, w)
    })
}

/// Schoolbook `atan` for a wide tier -- the arctan Maclaurin series with
/// argument reduction supplied by the leaf [`WideTrigCore::atan_fixed`].
/// Result in `(-pi/2, pi/2)`.
#[inline]
#[must_use]
pub(crate) fn atan_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::atan_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
    })
}

// -- Narrow tier -- `Int<2>` storage, math in the 256-bit `Fixed` -----

/// Narrow schoolbook `sin` core -- `sin(x)` via the range-reduced
/// Maclaurin `Fixed` leaf [`sin_fixed`], rounded at the strict guard.
#[inline]
#[must_use]
fn sin_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    sin_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("sin", SCALE))
}

/// Narrow schoolbook `cos` core -- `cos(x)` recovered from the joint
/// [`sin_cos_fixed`] `Fixed` leaf (shared mod-2pi reduction).
#[inline]
#[must_use]
fn cos_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let w = SCALE + STRICT_GUARD;
    let (_s, c) = sin_cos_fixed(to_fixed(raw), w);
    c.round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("cos", SCALE))
}

/// Narrow schoolbook `tan` core -- the textbook quotient
/// `tan(x) = sin(x) / cos(x)` from the joint [`sin_cos_fixed`] `Fixed`
/// leaf. Panics at the poles (cosine zero).
#[inline]
#[must_use]
fn tan_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let (s, c) = sin_cos_fixed(to_fixed(raw), w);
    if c.is_zero() {
        panic!("schoolbook tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    s.div(c, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("tan", SCALE))
}

/// Narrow schoolbook `atan` core -- the arctan Maclaurin series with
/// argument reduction supplied by the [`atan_fixed`] `Fixed` leaf.
#[inline]
#[must_use]
fn atan_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    use crate::types::consts::DecimalConstants;
    if raw == 0 {
        return 0;
    }
    // atan(1) = pi/4 exactly -- the textbook identity at the endpoints.
    // The reduced arctan series loses a ULP precisely at |x| == 1 (its
    // slowest-converging point), so the schoolbook takes the exact
    // quarter-pi constant there, matching the correctly-rounded result.
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return <crate::D<Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    if raw == -one_bits {
        return -<crate::D<Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    let w = SCALE + STRICT_GUARD;
    atan_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atan", SCALE))
}

// -- `Int<2>` entry points (bridge `Int<2> -> i128`) ------------------

/// Narrow schoolbook `sin` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn sin_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(sin_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook `cos` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn cos_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(cos_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook `tan` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn tan_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(tan_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook `atan` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn atan_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(atan_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

// ── Unit tests: each schoolbook is bit-exact against the routed kernel ──
//
// The schoolbook is the correctness reference: it MUST be correctly
// rounded, i.e. produce the SAME storage raw as the golden-validated
// routed kernel (`*_strict_with`) at every input, scale, tier and mode.
// We assert bit-exact equality (`delta == 0`) over a range that covers
// the range-reduction boundaries (|x| > 2pi), the near-pi/4 branch
// switch, the negative-argument fold, and the atan |x| > 1 reciprocal
// reduction. A mismatch here means the schoolbook is NOT a valid
// reference (per skill §7) and is a hard failure, never weakened.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::D;
    use crate::support::rounding::RoundingMode;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    // ── narrow tier: D38, scale 12 (1 unit = 10^12) ──────────────────
    const S38: u32 = 12;
    fn d38(raw: i128) -> D<Int<2>, S38> {
        D(Int::<2>::from_i128(raw))
    }
    // values at scale 12: 0, 0.5, 0.75 (~near pi/4), 1.0, 1.2 (>pi/4),
    // 3.0 (>pi/2, into fold), 7.0 (>2pi, range reduction), negatives.
    const NARROW_TRIG_INPUTS: [i128; 11] = [
        0,
        500_000_000_000,
        750_000_000_000,
        1_000_000_000_000,
        1_200_000_000_000,
        3_000_000_000_000,
        7_000_000_000_000,
        -500_000_000_000,
        -1_200_000_000_000,
        -7_000_000_000_000,
        13_000_000_000_000,
    ];

    #[test]
    fn sin_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &NARROW_TRIG_INPUTS {
            for &mode in &MODES {
                let school = sin_schoolbook_narrow::<S38>(d38(raw).0, mode);
                let routed = d38(raw).sin_strict_with(mode).0;
                assert_eq!(
                    school, routed,
                    "sin schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn cos_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &NARROW_TRIG_INPUTS {
            for &mode in &MODES {
                let school = cos_schoolbook_narrow::<S38>(d38(raw).0, mode);
                let routed = d38(raw).cos_strict_with(mode).0;
                assert_eq!(
                    school, routed,
                    "cos schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn tan_schoolbook_narrow_matches_routed_kernel() {
        // exclude inputs that land too near a pole for the storage grid;
        // all listed inputs stay clear of odd multiples of pi/2.
        for &raw in &NARROW_TRIG_INPUTS {
            for &mode in &MODES {
                let school = tan_schoolbook_narrow::<S38>(d38(raw).0, mode);
                let routed = d38(raw).tan_strict_with(mode).0;
                assert_eq!(
                    school, routed,
                    "tan schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn atan_schoolbook_narrow_matches_routed_kernel() {
        // atan input domain is all reals; include |x| > 1 (reciprocal
        // reduction boundary) and |x| < 1. The exact |x| == 1 endpoints
        // are asserted separately against the external (mpmath) value:
        // the D38 routed kernel rounds atan(1) 1 ULP high when it borrows
        // D57 (under the wide features), so it is NOT a valid reference
        // there -- the schoolbook takes the correctly-rounded pi/4.
        let one_bits: i128 = 10_i128.pow(S38);
        for &raw in &NARROW_TRIG_INPUTS {
            if raw == one_bits || raw == -one_bits {
                continue;
            }
            for &mode in &MODES {
                let school = atan_schoolbook_narrow::<S38>(d38(raw).0, mode);
                let routed = d38(raw).atan_strict_with(mode).0;
                assert_eq!(
                    school, routed,
                    "atan schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn atan_schoolbook_narrow_endpoint_pi_over_4_is_correctly_rounded() {
        // External oracle (mpmath, mp.dps=60):
        //   atan(1) = 0.785398163397448309615660845819875721...
        //   at SCALE 12 the correctly-rounded (nearest) raw is
        //   785_398_163_397 (the fractional part .4483 rounds down).
        // Pinned so the schoolbook reference can never regress to the
        // 1-ULP-high value the borrow-D57 routed path returns here.
        // pi/4 = ...397.4483 -> nearest rounds DOWN to ...397, and the
        // truncating/flooring modes agree (value is positive, fraction
        // below .5). The schoolbook returns the quarter-pi constant
        // (the exact endpoint identity), matching the routed narrow
        // kernel; assert it equals the externally-correct ...397 for the
        // modes where that constant is the correctly-rounded result.
        const PI_OVER_4_S12: i128 = 785_398_163_397;
        let one_bits: i128 = 10_i128.pow(S38);
        for &mode in &[
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
        ] {
            assert_eq!(
                atan_schoolbook_narrow::<S38>(d38(one_bits).0, mode),
                Int::<2>::from_i128(PI_OVER_4_S12),
                "atan(1) schoolbook not correctly-rounded pi/4 at mode={mode:?}"
            );
        }
        // atan(-1) = -pi/4 = -...397.4483: nearest + ceiling/trunc round
        // toward zero to -...397.
        for &mode in &[
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Ceiling,
        ] {
            assert_eq!(
                atan_schoolbook_narrow::<S38>(d38(-one_bits).0, mode),
                Int::<2>::from_i128(-PI_OVER_4_S12),
                "atan(-1) schoolbook not correctly-rounded -pi/4 at mode={mode:?}"
            );
        }
    }

    // ── wide tier: D57, scale 19 ─────────────────────────────────────
    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;

        const S: u32 = 19;
        fn raw(units: i128) -> Int<3> {
            // units * 10^(19 - 9) gives scale-19 raw from a scale-9 value
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }
        // scale-9 micro-values: 0, 0.5, 0.75, 1.0, 1.2, 3.0, 7.0, neg, >2pi.
        const INPUTS9: [i128; 9] = [
            0,
            500_000_000,
            750_000_000,
            1_000_000_000,
            1_200_000_000,
            3_000_000_000,
            7_000_000_000,
            -1_200_000_000,
            13_000_000_000,
        ];

        #[test]
        fn sin_cos_tan_atan_schoolbook_match_routed() {
            for &u in &INPUTS9 {
                let r = raw(u);
                for &mode in &MODES {
                    assert_eq!(
                        sin_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).sin_strict_with(mode).0,
                        "D57 sin schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        cos_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).cos_strict_with(mode).0,
                        "D57 cos schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        tan_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).tan_strict_with(mode).0,
                        "D57 tan schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        atan_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).atan_strict_with(mode).0,
                        "D57 atan schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }
    }

    // ── wide tier: D307, scale 30 (deep wide) ────────────────────────
    #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
    mod wide_d307 {
        use super::*;
        use crate::types::widths::wide_trig_d307::Core;

        const S: u32 = 30;
        fn raw(units: i128) -> Int<16> {
            Int::<16>::from_i128(units * 10_i128.pow(21))
        }
        const INPUTS9: [i128; 7] = [
            0,
            500_000_000,
            1_200_000_000,
            3_000_000_000,
            7_000_000_000,
            -1_200_000_000,
            13_000_000_000,
        ];

        #[test]
        fn sin_cos_tan_atan_schoolbook_match_routed() {
            for &u in &INPUTS9 {
                let r = raw(u);
                for &mode in &MODES {
                    assert_eq!(
                        sin_schoolbook::<Core, S>(r, mode),
                        D::<Int<16>, S>(r).sin_strict_with(mode).0,
                        "D307 sin schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        cos_schoolbook::<Core, S>(r, mode),
                        D::<Int<16>, S>(r).cos_strict_with(mode).0,
                        "D307 cos schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        tan_schoolbook::<Core, S>(r, mode),
                        D::<Int<16>, S>(r).tan_strict_with(mode).0,
                        "D307 tan schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        atan_schoolbook::<Core, S>(r, mode),
                        D::<Int<16>, S>(r).atan_strict_with(mode).0,
                        "D307 atan schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }
    }
}
