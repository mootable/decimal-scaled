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
        #[cfg(any(feature = "d115", feature = "wide"))]
        (6, 50..=60) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d153", feature = "wide"))]
        (8, 70..=82) => Select::ByAlgorithm(Algorithm::Tang),
        // Wide tiers — low-scale Tang rectangles, magnitude-gated. The N-way
        // width × scale A/B (`benches/micro/exp_wide_series_tang_ab.rs`, pinned,
        // operands at decimal scale built directly in the wide `Int<N>`) shows
        // Tang beats the Series Smith core at low scale only up to D462
        // (Int<24>): D307 ~1.0–1.3× and D462 ~1.1–1.6× faster than Series at
        // the bench-branch-compare SCALE 30. From D616 (Int<32>) UP, Tang is
        // tie-to-LOSS vs Series at every in-rectangle scale (D616 break-even,
        // D924 ~1.1× SLOWER, D1232 ~1.1–1.3× SLOWER) — the table-multiply on
        // the wider work integer costs as much as the Smith squarings it
        // elides, exactly as the prior wide-exp bench-trials found. So only
        // N=16/24 carry a Tang rectangle; N≥32 falls through to Series.
        //
        // Tang's `k·ln 2` reduction lifts the working scale by ~`|k|·log10 2`
        // digits, which for large `|x|` exceeds the work width — so Tang is
        // only VALID for small `|x|` (Series builds the result by squaring and
        // stays valid everywhere). Hence `ByValue`: Tang for `|x| < 100`,
        // Series above. The `_` arm keeps Series for scales above the rectangle
        // and for every tier N≥32.
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        (16, 0..=160) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        (24, 0..=235) => Select::ByValue(wide_tang_gate::<N, SCALE>),
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
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => {
            let r = raw.resize_to::<Int<6>>();
            let out = match SCALE {
                50..=60 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 128, 8, true, true, false>(r, mode),
                _ => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => {
            let r = raw.resize_to::<Int<8>>();
            let out = match SCALE {
                70..=82 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 128, 10, true, false, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        // Wide tiers — the low-scale Tang rectangles (`select` routes only the
        // in-rectangle SCALEs here, and only N=16/24: from N=32 up Tang is
        // tie-to-loss vs Series, so `select` never picks Tang there). One
        // config per tier: M=128, G=30, the directed + external-extra shape
        // (`DIRECTED, EXTERNAL_EXTRA`) — Ziv escalation for the directed modes
        // and base-guard widening for the large `|k|` the 2^k reassembly
        // amplifies (single-shot would be wrong for large-x inputs whose
        // `|k|·log10 2` exceeds the guard).
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 128, 30, true, true, false>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
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
