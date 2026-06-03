//! To-degrees policy — the per-(N, SCALE) algorithm matcher for
//! radians-to-degrees angle conversion.
//!
//! `D<Int<N>, SCALE>::to_degrees_strict_with(mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`. Narrow tiers (N=1 widened to Int<2>, N=2) run the 256-bit
//! `Fixed` `trig_series_2limb` kernel; wide tiers run the tier-generic
//! `angle_mul_pi_ratio` kernel over their `WideTrigCore` core, reached by
//! a `match N` with `resize_to` bridges (identity at the matched `N`,
//! like the `sqrt` `MgDivide` arm). One computation everywhere.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    MulPiRatio,
    #[allow(dead_code)]
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
    Select::ByAlgorithm(Algorithm::MulPiRatio)
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&raw),
    };
    match algo {
        Algorithm::MulPiRatio => mul_pi_ratio_routed::<N, SCALE>(raw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
fn mul_pi_ratio_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => crate::algos::trig::trig_series_2limb::to_degrees_strict::<SCALE>(
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
        _ => crate::algos::trig::trig_series_2limb::to_degrees_strict::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook_narrow::<SCALE>(
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
        _ => crate::algos::trig::angle_schoolbook::to_degrees_schoolbook_narrow::<SCALE>(
            raw.resize_to::<Int<2>>(), mode).resize_to::<Int<N>>(),
    }
}
