// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! To-degrees policy — the per-(N, SCALE) algorithm matcher for
//! radians-to-degrees angle conversion.
//!
//! `D<Int<N>, SCALE>::to_degrees_with(mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`. The narrow and wide tiers run genuinely different algorithms
//! here, and `select` names each where it runs: the narrow tiers (N=1
//! widened to Int<2>, N=2) form `mul(x, 180) / π` on the 256-bit `Fixed`
//! intermediate via `trig_series_2limb` — that is
//! [`Algorithm::Schoolbook`], not a ratio multiply — while the wide tiers
//! multiply by the exact `deg_per_rad` constant over their
//! `WideTrigCore` core via `angle_mul_pi_ratio::to_degrees_mul_pi_ratio`
//! — [`Algorithm::MulPiRatio`]. Both are reached by a `match N` with
//! `resize_to` bridges (identity at the matched `N`, like the `sqrt`
//! `MgDivide` arm).
//!
//! The sibling `policy::to_radians` names `Schoolbook` at EVERY width
//! instead, because there the ratio constant `π/180 < 1` costs about
//! `log10(180)` digits of relative precision. No such swap happened
//! here: the wide `to_degrees` shell's own former inline body WAS the
//! `deg_per_rad` multiply, expression for expression, graded by
//! `to_degrees_mul_pi_ratio_matches_the_inherent_shell_across_the_surface`.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `x · (180/π)` as ONE multiply against the `deg_per_rad` table
    /// constant. What the wide tiers run.
    MulPiRatio,
    /// `mul(x, 180) / π` — scale by the exact integer 180 at the working
    /// scale, then divide by `π`. What the narrow tiers run, via
    /// `trig_series_2limb::to_degrees`.
    Schoolbook,
}

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    match N {
        // The `Fixed`-256 kernel, which divides by `π` rather than
        // multiplying by `deg_per_rad`: Schoolbook is the name of what
        // runs. There is no narrow `deg_per_rad` kernel to name.
        1 | 2 => Select::ByAlgorithm(Algorithm::Schoolbook),
        // Wide tiers: the ratio multiply the wide shell has always run.
        _ => Select::ByAlgorithm(Algorithm::MulPiRatio),
    }
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
        1 | 2 => crate::algos::trig::trig_series_2limb::to_degrees::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => crate::algos::trig::trig_series_2limb::to_degrees::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        // The narrow realisation is `trig_series_2limb::to_degrees`:
        // `to_fixed_w(x, STRICT_GUARD).mul_u128(180).div(π)`, the same
        // Schoolbook identity the wide arms evaluate and the kernel the
        // D18/D38 surface has always run.
        1 | 2 => crate::algos::trig::trig_series_2limb::to_degrees::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        // Widths no tier claims. Keeps the standalone narrow Schoolbook
        // kernel routed; it evaluates the same expression as the
        // `1 | 2` arm at the same guard width.
        _ => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook_narrow::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
    }
}
