// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Angle-conversion schoolbook kernels -- to_degrees / to_radians.
//!
//! Textbook realisations of both angle conversions, registered as the
//! `Schoolbook` arms of the angle-conversion policies. The two
//! directions have OPPOSITE routing status, so read the one you need:
//!
//! - [`to_radians_schoolbook`] is ROUTED, and is production code.
//!   `policy::to_radians::select` names `Schoolbook` at every width, so
//!   this is the wide-tier `to_radians` kernel every wide cell runs (the
//!   narrow tiers evaluate the same identity through
//!   `trig_series_2limb::to_radians`). It is preferred over the
//!   `MulPiRatio` alternative on PRECISION: multiplying by the sub-unit
//!   `rad_per_deg` constant gives up about log10(180) digits, because a
//!   constant below 1 held at the working scale spends its leading
//!   digits on zeros -- see `angle_mul_pi_ratio::to_radians_mul_pi_ratio`.
//! - [`to_degrees_schoolbook`] is UNROUTED. `policy::to_degrees::select`
//!   names `MulPiRatio` at the wide tiers, and its narrow `Schoolbook`
//!   arm runs `trig_series_2limb::to_degrees`. This direction is a kept
//!   correctness reference + A/B microbench partner.
//!
//! Identities, dispatched DOWN to the `Int<N>` work int:
//! - to_degrees(x) = x * 180 / pi  (multiply by the integer 180, divide
//!   by pi at the working scale).
//! - to_radians(x) = x * pi / 180  (multiply by pi, divide by 180).
//!
//! Wide path uses the `WideTrigCore::mul` + the `pi` constant + the
//! `lit(180)` literal binding; narrow path uses the `Fixed` work int and
//! `wide_pi`. NEVER calls a decimal `*_strict_with` on its own value.
//!
//! [`to_degrees_schoolbook`] does NOT match the routed `MulPiRatio`
//! kernel in general. Dividing by `pi` is about log10(180/pi) digits
//! weaker than multiplying by the `deg_per_rad` constant, which lands it
//! 1 ULP low at the directed-mode boundaries -- e.g.
//! `D57<0>::to_degrees(10^28)` under `Ceiling`, where the routed kernel
//! is correct and this one is not. The divergence is magnitude-driven,
//! so the small-input equivalence tests below agree while the general
//! claim does not hold; see `angle_mul_pi_ratio` for the worked case.

use crate::algos::ln::ln_series_2limb::STRICT_GUARD;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::{to_fixed, wide_pi};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Schoolbook to_degrees for a wide tier -- x * 180 / pi.
#[inline]
#[must_use]
pub(crate) fn to_degrees_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let working_scale = SCALE + C::GUARD;
    let working_value = C::to_work(raw);
    let degrees = C::div(
        working_value * C::lit(180),
        C::pi::<SCALE>(working_scale),
        working_scale,
    );
    C::round_to_storage_with(degrees, working_scale, SCALE, mode)
}

/// Schoolbook to_radians for a wide tier -- x * pi / 180.
#[inline]
#[must_use]
pub(crate) fn to_radians_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let working_scale = SCALE + C::GUARD;
    let working_value = C::to_work(raw);
    let radians =
        C::mul(working_value, C::pi::<SCALE>(working_scale), working_scale) / C::lit(180);
    C::round_to_storage_with(radians, working_scale, SCALE, mode)
}

// -- Narrow tier -- Int<2> storage, math in the 256-bit Fixed ---------

#[inline]
#[must_use]
fn to_degrees_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let working_scale = SCALE + STRICT_GUARD;
    to_fixed(raw)
        .mul_u128(180)
        .div(wide_pi(working_scale), working_scale)
        .round_to_i128_with(working_scale, SCALE, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("to_degrees", SCALE)
        })
}

#[inline]
#[must_use]
fn to_radians_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let working_scale = SCALE + STRICT_GUARD;
    to_fixed(raw)
        .mul(wide_pi(working_scale), working_scale)
        .div_small(180)
        .round_to_i128_with(working_scale, SCALE, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("to_radians", SCALE)
        })
}

/// Narrow schoolbook to_degrees for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn to_degrees_schoolbook_narrow<const SCALE: u32>(
    raw: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    Int::<2>::from_i128(to_degrees_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook to_radians for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn to_radians_schoolbook_narrow<const SCALE: u32>(
    raw: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    Int::<2>::from_i128(to_radians_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

// -- Unit tests: each schoolbook against the routed kernel on a
// small-magnitude input set, at one scale per tier, over all eight
// rounding modes.
//
// These pin agreement where it holds; they are NOT a general
// equivalence claim. `to_degrees_schoolbook`'s divergence from the
// routed `MulPiRatio` kernel is magnitude-driven and appears well above
// these inputs (module docs), so a passing run here says nothing about
// the top of a tier's range.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::D;

    const MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    const S38: u32 = 12;
    fn d38(raw: i128) -> D<Int<2>, S38> {
        D(Int::<2>::from_i128(raw))
    }
    const INPUTS: [i128; 7] = [
        0,
        1_000_000_000_000,
        90_000_000_000_000,
        180_000_000_000_000,
        -1_000_000_000_000,
        -90_000_000_000_000,
        -180_000_000_000_000,
    ];

    #[test]
    fn to_degrees_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    to_degrees_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).to_degrees_with(mode).0,
                    "to_degrees schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn to_radians_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    to_radians_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).to_radians_with(mode).0,
                    "to_radians schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;

        const S: u32 = 19;
        fn raw9(units: i128) -> Int<3> {
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }
        const INPUTS9: [i128; 5] = [
            0,
            1_000_000_000,
            45_000_000_000,
            -1_000_000_000,
            -45_000_000_000,
        ];

        #[test]
        fn to_degrees_to_radians_schoolbook_match_routed() {
            for &units in &INPUTS9 {
                let raw = raw9(units);
                for &mode in &MODES {
                    assert_eq!(
                        to_degrees_schoolbook::<Core, S>(raw, mode),
                        D::<Int<3>, S>(raw).to_degrees_with(mode).0,
                        "D57 to_degrees schoolbook != routed at units={units} mode={mode:?}"
                    );
                    assert_eq!(
                        to_radians_schoolbook::<Core, S>(raw, mode),
                        D::<Int<3>, S>(raw).to_radians_with(mode).0,
                        "D57 to_radians schoolbook != routed at units={units} mode={mode:?}"
                    );
                }
            }
        }
    }
}
