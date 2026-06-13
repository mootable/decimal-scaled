// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Angle-conversion schoolbook reference kernels -- to_degrees /
//! to_radians.
//!
//! Naive textbook realisations registered as unrouted `Schoolbook` arms
//! of the angle-conversion policies. Correctness reference + A/B
//! microbench partner for the routed `MulPiRatio` kernels; `select`
//! never routes here.
//!
//! Identities, dispatched DOWN to the `Int<N>` work int:
//! - to_degrees(x) = x * 180 / pi  (multiply by the integer 180, divide
//!   by pi at the working scale).
//! - to_radians(x) = x * pi / 180  (multiply by pi, divide by 180).
//!
//! Wide path uses the `WideTrigCore::mul` + the `pi` constant + the
//! `lit(180)` literal binding; narrow path uses the `Fixed` work int and
//! `wide_pi`. NEVER calls a decimal `*_strict_with` on its own value.
//! Identical composition + narrowing as the routed kernel, so it matches
//! bit-exactly.

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
    let w = SCALE + C::GUARD;
    let v = C::to_work(raw);
    let r = C::div(v * C::lit(180), C::pi::<SCALE>(w), w);
    C::round_to_storage_with(r, w, SCALE, mode)
}

/// Schoolbook to_radians for a wide tier -- x * pi / 180.
#[inline]
#[must_use]
pub(crate) fn to_radians_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w = SCALE + C::GUARD;
    let v = C::to_work(raw);
    let r = C::mul(v, C::pi::<SCALE>(w), w) / C::lit(180);
    C::round_to_storage_with(r, w, SCALE, mode)
}

// -- Narrow tier -- Int<2> storage, math in the 256-bit Fixed ---------

#[inline]
#[must_use]
fn to_degrees_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    to_fixed(raw)
        .mul_u128(180)
        .div(wide_pi(w), w)
        .round_to_i128_with(w, SCALE, mode)
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
    let w = SCALE + STRICT_GUARD;
    to_fixed(raw)
        .mul(wide_pi(w), w)
        .div_small(180)
        .round_to_i128_with(w, SCALE, mode)
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

// -- Unit tests: each schoolbook is bit-exact against the routed kernel.
//
// Reference correctness (skill 7): delta == 0 against the routed
// MulPiRatio kernel at every input, scale, tier and mode.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::D;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
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
                    d38(raw).to_degrees_strict_with(mode).0,
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
                    d38(raw).to_radians_strict_with(mode).0,
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
            for &u in &INPUTS9 {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        to_degrees_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).to_degrees_strict_with(mode).0,
                        "D57 to_degrees schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        to_radians_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).to_radians_strict_with(mode).0,
                        "D57 to_radians schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }
    }
}
