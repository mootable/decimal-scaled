//! Log-base policy — the per-(N, SCALE) algorithm matcher for the
//! arbitrary-base decimal logarithm `log(self, base)`.
//!
//! `D<Int<N>, SCALE>::log_strict_with(base, mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`.
//!
//! # One algorithm — `LnDivide`
//!
//! `log(self, base) = ln(self) / ln(base)`. Every tier and scale uses the
//! same ratio. The narrow tiers route through the
//! `crate::algos::log::log_ln_divide` composition kernels (D18 widens to
//! Int<2>; D38 calls `ln::ln_series_2limb`); the wide tiers route through
//! the per-tier `wide_trig_<tier>::log_strict_with_kernel` /
//! `log_approx_with_kernel` free functions (emitted by
//! `decl_wide_transcendental!`), reached by a `match N` with `resize_to`
//! bridges (identity at the matched `N`). `Schoolbook` is the unrouted
//! naive `ln(x)/ln(b)` reference.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    LnDivide,
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
    Select::ByAlgorithm(Algorithm::LnDivide)
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&raw),
    };
    match algo {
        Algorithm::LnDivide => ln_divide_routed::<N, SCALE>(raw, braw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, braw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch_with<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&raw),
    };
    match algo {
        Algorithm::LnDivide => ln_divide_with_routed::<N, SCALE>(raw, braw, working_digits, mode),
        Algorithm::Schoolbook => schoolbook_with_routed::<N, SCALE>(raw, braw, working_digits, mode),
    }
}

#[inline]
fn ln_divide_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 => crate::algos::log::log_ln_divide::log_ln_divide_d18::<SCALE>(raw.resize_to::<Int<1>>(), braw.resize_to::<Int<1>>(), mode).resize_to::<Int<N>>(),
        2 => super::narrow_checked::<N>(crate::algos::log::log_ln_divide::log_ln_divide_d38::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode), "log_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::log::log_ln_divide::log_ln_divide_d38::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode), "log_strict", SCALE),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 => super::narrow_checked::<N>(crate::algos::log::log_schoolbook::log_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode), "log_strict", SCALE),
        2 => super::narrow_checked::<N>(crate::algos::log::log_schoolbook::log_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode), "log_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::log::log_schoolbook::log_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode), "log_strict", SCALE),
    }
}

#[inline]
fn ln_divide_with_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 => crate::algos::log::log_ln_divide::log_ln_divide_d18_approx::<SCALE>(raw.resize_to::<Int<1>>(), braw.resize_to::<Int<1>>(), working_digits, mode).resize_to::<Int<N>>(),
        2 => super::narrow_checked::<N>(crate::algos::log::log_ln_divide::log_ln_divide_d38_approx::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), working_digits, mode), "log_with", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => { if working_digits == crate::types::widths::wide_trig_d57::GUARD { crate::types::widths::wide_trig_d57::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode) } else { crate::types::widths::wide_trig_d57::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => { if working_digits == crate::types::widths::wide_trig_d76::GUARD { crate::types::widths::wide_trig_d76::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode) } else { crate::types::widths::wide_trig_d76::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => { if working_digits == crate::types::widths::wide_trig_d115::GUARD { crate::types::widths::wide_trig_d115::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode) } else { crate::types::widths::wide_trig_d115::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => { if working_digits == crate::types::widths::wide_trig_d153::GUARD { crate::types::widths::wide_trig_d153::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode) } else { crate::types::widths::wide_trig_d153::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => { if working_digits == crate::types::widths::wide_trig_d230::GUARD { crate::types::widths::wide_trig_d230::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode) } else { crate::types::widths::wide_trig_d230::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => { if working_digits == crate::types::widths::wide_trig_d307::GUARD { crate::types::widths::wide_trig_d307::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode) } else { crate::types::widths::wide_trig_d307::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => { if working_digits == crate::types::widths::wide_trig_d462::GUARD { crate::types::widths::wide_trig_d462::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode) } else { crate::types::widths::wide_trig_d462::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => { if working_digits == crate::types::widths::wide_trig_d616::GUARD { crate::types::widths::wide_trig_d616::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode) } else { crate::types::widths::wide_trig_d616::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => { if working_digits == crate::types::widths::wide_trig_d924::GUARD { crate::types::widths::wide_trig_d924::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode) } else { crate::types::widths::wide_trig_d924::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => { if working_digits == crate::types::widths::wide_trig_d1232::GUARD { crate::types::widths::wide_trig_d1232::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode) } else { crate::types::widths::wide_trig_d1232::log_approx_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), working_digits, mode) } }.resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::log::log_ln_divide::log_ln_divide_d38_approx::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), working_digits, mode), "log_with", SCALE),
    }
}

#[inline]
fn schoolbook_with_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 => super::narrow_checked::<N>(crate::algos::log::log_schoolbook::log_schoolbook_with(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "log_with", SCALE),
        2 => super::narrow_checked::<N>(crate::algos::log::log_schoolbook::log_schoolbook_with(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "log_with", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::log::log_schoolbook::log_schoolbook_with(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "log_with", SCALE),
    }
}
