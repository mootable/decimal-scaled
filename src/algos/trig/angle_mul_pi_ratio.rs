// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Angle-conversion `MulPiRatio` kernels -- to_degrees / to_radians.
//!
//! The routed realisations of `to_degrees`/`to_radians` for the wide
//! tiers: multiply by the exact `180/pi` (resp. `pi/180`) ratio in the
//! guard-digit work integer, then round correctly to storage. These are
//! the `MulPiRatio` algorithm named by
//! `crate::policy::to_degrees` / `crate::policy::to_radians`; the policy `MulPiRatio`
//! arm routes DOWN to them directly (no inversion through the inherent
//! `*_strict_with` shell).
//!
//! Identities, dispatched DOWN to the `Int<N>` work int via the tier core:
//! - to_degrees(x) = x * 180 / pi
//! - to_radians(x) = x * pi / 180
//!
//! These never call a decimal `*_strict_with` on their own value.

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::support::rounding::RoundingMode;

/// `MulPiRatio` to_degrees for a wide tier -- `x * 180 / pi` in the
/// guard-digit work integer, correctly rounded to storage.
#[inline]
#[must_use]
pub(crate) fn to_degrees_mul_pi_ratio<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w = SCALE + C::GUARD;
    let v = C::to_work(raw);
    // `x * 180/π`: multiply by the exact oracle `deg_per_rad` factor
    // instead of dividing by `π` (the divide was the main cost).
    let r = C::mul(v, C::deg_per_rad::<SCALE>(w), w);
    C::round_to_storage_with(r, w, SCALE, mode)
}

/// `MulPiRatio` to_radians for a wide tier -- `x * pi / 180` in the
/// guard-digit work integer, correctly rounded to storage.
#[inline]
#[must_use]
pub(crate) fn to_radians_mul_pi_ratio<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w = SCALE + C::GUARD;
    let v = C::to_work(raw);
    // `x * π/180`: multiply by the exact oracle `rad_per_deg` factor.
    let r = C::mul(v, C::rad_per_deg::<SCALE>(w), w);
    C::round_to_storage_with(r, w, SCALE, mode)
}

// ── Unit tests: the MulPiRatio kernel is bit-exact against the routed
// `*_strict_with` shell at every input, scale, tier and mode (skill 7).
#[cfg(test)]
mod tests {
    use super::*;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::D;
        use crate::int::types::Int;
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
        fn to_degrees_to_radians_mul_pi_ratio_match_routed() {
            for &u in &INPUTS9 {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        to_degrees_mul_pi_ratio::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).to_degrees_strict_with(mode).0,
                        "D57 to_degrees MulPiRatio != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        to_radians_mul_pi_ratio::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).to_radians_strict_with(mode).0,
                        "D57 to_radians MulPiRatio != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }
    }
}
