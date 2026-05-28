//! Exponential policy — the per-(N, SCALE) algorithm matcher (plus the
//! derived exp2).
//!
//! `D<Int<N>, SCALE>::exp_strict_with(mode)` delegates directly to the one
//! shared [`dispatch`] generic function — the canonical matcher-only
//! policy shape (see `docs/ARCHITECTURE.md`), mirrored from `sqrt`:
//!
//! 1. an [`Algorithm`] enum — Series / Tang / Schoolbook, no `Default`;
//! 2. a [`Select`] verdict;
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via `const { select::<N, SCALE>() }`, then an exhaustive
//!    `match algo` — no `_`, no panic.
//!
//! The narrow tiers run the 256-bit `Fixed` kernel (`exp_series_2limb`,
//! D18 widened to Int<2>); the wide tiers run the tier-generic `exp_series`
//! over `WideTrigCore`, or the per-tier `exp_tang` band kernel, reached by
//! a `match N` with `resize_to` bridges (identity at the matched `N`).
//!
//! exp2 is derived (`2^x = exp(x·ln2)` with an exact-power pin) and routes
//! DOWN to the narrow `exp_series_2limb::exp2_*` kernels or the wide
//! per-tier `wide_trig_<tier>::exp2_{strict,approx}_with_kernel` free fns —
//! never back through a sibling decimal policy.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Series,
    #[cfg(feature = "_wide-support")]
    Tang,
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
    match (N, SCALE) {
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 18..=22) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 45..=56) => Select::ByAlgorithm(Algorithm::Tang),
        // D76 (Int<4>): the full width × scale A/B
        // (`benches/micro/exp_wide_series_tang_ab.rs`) shows Tang beats the
        // Series squaring core at EVERY D76 scale, including the MAX scale
        // (s75), where the map ranks Series ~16× slower than Tang and every
        // Tang candidate is bit-identical to Series (zero validity failures).
        // The old gate stopped at s74, so the MAX scale `exp_D76_s75` cell —
        // the bench-branch-compare 6.41× regression — fell through to the
        // slow Series `_` arm (a Class-I single-cell boundary miss). Cover the
        // WHOLE D76 scale range (0..=75, the design max) through the
        // small-`|x|` value gate so no scale is left on the Series path.
        #[cfg(any(feature = "d76", feature = "wide"))]
        (4, 0..=75) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        // Narrow-wide tiers (N = 6/8/12/16/24) — low-scale Tang rectangles,
        // magnitude-gated. The width × scale A/B shows Tang beats the Series
        // squaring core at low/mid scale (the bench-branch-compare SCALE 30
        // regime); the crossover where Series catches up scales with width, so
        // the rectangles widen for the narrower tiers (D115/D153/D230) and
        // shrink for the wider ones (D307/D462, where Series already wins by
        // S/2). Tang's `k·ln 2` reduction lifts the working scale by
        // ~`|k|·log10 2` digits, which for large `|x|` exceeds the work width —
        // so Tang is only VALID for small `|x|` (Series builds the result by
        // squaring and stays valid everywhere). Hence `ByValue`: Tang for
        // `|x| < 100`, Series above.
        //
        // The WIDEST tiers (N >= 32: D616/D924/D1232) have NO Tang rectangle:
        // the A/B shows Series wins at EVERY scale there (including s0/s30),
        // and at D1232 max scale single-shot Tang is not even bit-identical to
        // Series (its `k·ln 2` lift overflows the guard). Those widths want the
        // orthogonal u128-square work, not Tang; they fall through to Series.
        #[cfg(any(feature = "d115", feature = "wide"))]
        (6, 0..=85) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        #[cfg(any(feature = "d153", feature = "wide"))]
        (8, 0..=110) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        #[cfg(any(feature = "d230", feature = "wide"))]
        (12, 0..=140) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        (16, 0..=60) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        (24, 0..=60) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        _ => Select::ByAlgorithm(Algorithm::Series),
    }
}

/// Value gate for the wide-tier low-scale Tang rectangles: Tang is correct
/// only while its `k·ln 2` working-scale lift fits the work width, i.e. for
/// small `|x|`. Route Tang for `|x| < 100`, else Series (always valid).
///
/// `|x| < 100` ⇔ `|raw| < 10^(SCALE+2)`, tested conservatively on the bit
/// length: `|raw| < 2^B ≤ 10^(SCALE+2)` when `B = ⌊(SCALE+2)·log2 10⌋`
/// (`log2 10 ≈ 3.32192`, taken as `332192/100000`, rounded DOWN so `2^B`
/// never exceeds `10^(SCALE+2)` — never routes an out-of-range value to Tang).
#[cfg(feature = "_wide-support")]
fn wide_tang_gate<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    let max_bits = (SCALE + 2) * 332_192 / 100_000;
    if BigInt::bit_length(*raw) <= max_bits {
        Algorithm::Tang
    } else {
        Algorithm::Series
    }
}

#[inline]
fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(raw),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match resolve::<N, SCALE>(&raw) {
        Algorithm::Series => series_routed::<N, SCALE>(raw, mode),
        #[cfg(feature = "_wide-support")]
        Algorithm::Tang => tang_routed::<N, SCALE>(raw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch_with<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    // Only the narrow tier honours caller working_digits (matching the
    // prior ExpPolicy routing, where wide exp_with_impl ignored it).
    match N {
        1 | 2 => super::narrow_checked::<N>(
            crate::algos::exp::exp_series_2limb::exp_with(
                raw.resize_to::<Int<2>>(),
                SCALE,
                working_digits,
                mode,
            ),
            "exp_with",
            SCALE,
        ),
        _ => {
            let _ = working_digits;
            dispatch::<N, SCALE>(raw, mode)
        }
    }
}

#[inline]
fn series_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_schoolbook::exp_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_schoolbook::exp_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
    }
}

#[cfg(feature = "_wide-support")]
#[inline]
fn tang_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => {
            let r = raw.resize_to::<Int<3>>();
            let out = match SCALE {
                18..=22 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 128, 8, false, false, false>(r, mode),
                45..=56 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 512, 30, false, false, false>(r, mode),
                _ => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        // D76 (Int<4>): full-range Tang, M=512 G=30, the directed +
        // external-extra shape <true,true,false> — bit-identical to Series
        // across the spread × all six modes at s0/s37/s74 in the width × scale
        // A/B. The `select` gate (small `|x|`) keeps it valid; large-`|x|`
        // falls through to Series.
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d76::Core, SCALE, 512, 30, true, true, false>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 512, 30, true, true, false>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 512, 30, true, true, false>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d230::Core, SCALE, 128, 30, true, true, false>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        // Narrow-wide tiers — the low-scale Tang rectangles (`select` routes
        // only the in-rectangle SCALEs here). The directed + external-extra
        // shape (`DIRECTED, EXTERNAL_EXTRA`) — Ziv escalation for the directed
        // modes and base-guard widening for the large `|k|` the 2^k reassembly
        // amplifies (single-shot would be wrong for large-x inputs whose
        // `|k|·log10 2` exceeds the guard). Table M tuned per tier from the A/B
        // (M=512 for D307, M=128 for D462). The widest tiers (N >= 32) have NO
        // Tang arm — `select` never routes them here and they would fall to the
        // `_` Series arm anyway; Series wins those at every scale.
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 512, 30, true, true, false>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d462::Core, SCALE, 128, 30, true, true, false>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        _ => series_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn exp2_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp2_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp2_strict", SCALE),
    }
}

#[inline]
#[must_use]
pub(crate) fn exp2_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, working_digits: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "exp2_with", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), working_digits, mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "exp2_with", SCALE),
    }
}
