// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! To-radians policy — the per-(N, SCALE) algorithm matcher for
//! degrees-to-radians angle conversion.
//!
//! `D<Int<N>, SCALE>::to_radians_with(mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`. `select` names [`Algorithm::Schoolbook`] at every width,
//! because that is the identity the surface evaluates: the narrow tiers
//! (N=1 widened to Int<2>, N=2) form `mul(x, π) / 180` on the 256-bit
//! `Fixed` intermediate via `trig_series_2limb`, and the wide tiers form
//! the same expression over their `WideTrigCore` core via
//! `angle_schoolbook::to_radians_schoolbook`, reached by a `match N` with
//! `resize_to` bridges (identity at the matched `N`, like the `sqrt`
//! `MgDivide` arm). One computation everywhere.
//!
//! [`Algorithm::MulPiRatio`] stays registered as a kept alternative, not
//! as the verdict. It is a DIFFERENT algorithm rather than a spelling of
//! the same one: it multiplies by the `rad_per_deg` table constant
//! instead of scaling through `π` first, which gives up about
//! `log10(180)` digits of relative precision (see
//! `angle_mul_pi_ratio::to_radians_mul_pi_ratio`, which records the
//! measured 11-unit error at `D57<0>::to_radians(10^32)`). Naming it here
//! would be a precision regression, not a routing tweak.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `x · (π/180)` as ONE multiply against the `rad_per_deg` table
    /// constant. Registered and kept, but not named by `select`: it
    /// trades away about `log10(180)` digits of relative precision
    /// against [`Algorithm::Schoolbook`] — see the module docs.
    #[allow(dead_code)]
    MulPiRatio,
    /// `mul(x, π) / 180` — scale through `π` at the working scale first,
    /// then divide by the exact integer 180. What every width runs:
    /// `trig_series_2limb::to_radians` on the narrow tiers,
    /// `angle_schoolbook::to_radians_schoolbook` over the tier core on
    /// the wide ones.
    Schoolbook,
}

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE);
    // Schoolbook at every width — the identity every `to_radians` surface
    // evaluates today. Tier-independent, so no `match N` is needed: the
    // per-width realisation is picked inside `schoolbook_routed`.
    Select::ByAlgorithm(Algorithm::Schoolbook)
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(choose) => choose(&raw),
    };
    match algo {
        Algorithm::MulPiRatio => mul_pi_ratio_routed::<N, SCALE>(raw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
fn mul_pi_ratio_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => crate::algos::trig::trig_series_2limb::to_radians::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => crate::algos::trig::trig_series_2limb::to_radians::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        // The narrow realisation is `trig_series_2limb::to_radians`:
        // `to_fixed_w(x, STRICT_GUARD).mul(π).div_small(180)`, the same
        // Schoolbook identity the wide arms evaluate and the kernel the
        // D18/D38 surface has always run.
        1 | 2 => crate::algos::trig::trig_series_2limb::to_radians::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        // Widths no tier claims. Keeps the standalone narrow Schoolbook
        // kernel routed; it evaluates the same expression as the
        // `1 | 2` arm at the same guard width.
        _ => crate::algos::trig::angle_schoolbook::to_radians_schoolbook_narrow::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
    }
}
