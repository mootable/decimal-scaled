// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `expm1` policy — the per-(N, SCALE) algorithm matcher.
//!
//! `D<Int<N>, SCALE>::expm1_strict_with(mode)` delegates to [`dispatch`];
//! `expm1_approx_with` delegates to [`dispatch_with`]. Canonical
//! matcher-only shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `policy::log1p`:
//!
//! 1. an [`Algorithm`] enum — Series / WithExp, no `Default`;
//! 2. a [`Select`] verdict;
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via `const { select::<N, SCALE>() }`, then an exhaustive
//!    `match algo` — no `_`, no panic.
//!
//! The choice here is VALUE-dependent, so [`select`] returns the
//! [`Select::ByValue`] arm at every cell and [`classify`] reads the
//! argument. See [`classify`] for the region and why it is a validity
//! wall rather than a tuning knob.
//!
//! The width axis — which work integer each tier runs the chosen kernel
//! in — lives in the routing fns below, like `policy::log1p`'s: never in
//! the [`Select`] verdict, never on [`Algorithm`], never in a `dispatch`
//! signature (the BigRule). Narrow (`N <= 2`) computes at [`WZiv`] and
//! narrows to storage; each wide tier computes at its own `Core::W`
//! (Series) or `Core::Wexp` (WithExp — the large-argument regime, which
//! needs the width; see [`expm1_with_exp_g`]). Both reach the SAME generic
//! kernels — no per-tier algorithm copy.
//!
//! # Kept, unrouted alternatives
//!
//! `algos::expm1` holds two further generic kernels that this matcher does
//! NOT route — `expm1_halving` (binary halving + the `E*(E+2)` doubling
//! recurrence) and `expm1_reduced` (`k*ln 2` reduction + the
//! `((P+E)<<k) - P` reassembly). Both are CORRECT over regions that
//! overlap the two routed arms entirely, so choosing between them is an
//! OPTIMALITY question, not a validity one — and optimality is decided by
//! measurement, which this pass deliberately does not attempt. They stay
//! as kept alternatives for a later `policy-mapper` race.

use crate::algos::expm1::expm1_series::{expm1_series_approx_g, expm1_series_g};
use crate::algos::expm1::expm1_with_exp::{expm1_with_exp_approx_g, expm1_with_exp_g};
use crate::algos::support::narrow_ziv::WZiv;
use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Working guard digits the narrow (D18 / D38) arm starts its Ziv
/// escalation at — the crate's narrow strict guard, shared with every
/// other narrow transcendental. Private to this file; the kernels take
/// it as an argument and never import it.
const NARROW_GUARD: u32 = crate::algos::ln::ln_series_2limb::STRICT_GUARD;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// The leading-term-dropped Taylor series `x + x²/2! + x³/3! + …`
    /// with no argument reduction —
    /// [`crate::algos::expm1::expm1_series`].
    Series,
    /// `e^x - 1` formed at the WORKING scale —
    /// [`crate::algos::expm1::expm1_with_exp`]. A composition, so the
    /// variant names the composition rather than a single kernel fn (the
    /// documented `ExpWithLn` precedent).
    WithExp,
}

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    #[allow(dead_code)]
    ByAlgorithm(Algorithm),
    ByValue(fn(&Int<N>) -> Algorithm),
}

/// Total over `(N, SCALE)`: the choice is value-dependent at every cell,
/// so every cell resolves to the same [`classify`] — monomorphised at
/// this cell's `SCALE`, which is what turns the region below into a
/// compile-time-known comparison bound.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    Select::ByValue(classify::<N, SCALE>)
}

/// Routes `x ∈ [-1, +1]` to [`Algorithm::Series`] and everything else to
/// [`Algorithm::WithExp`].
///
/// This is a **validity wall, not a tuning threshold.** The series
/// `x + x²/2! + x³/3! + …` carries NO range reduction, and it fails
/// outside the region in two different ways depending on the sign:
///
/// * `x < 0` — the series ALTERNATES, so its cancellation loss is
///   `max term / |sum|`. At `x = -1` that is `1 / |expm1(-1)| =
///   1/0.63212`, i.e. **0.66 bits** — under one bit, so the whole band is
///   safe. Past it the loss grows like `e^|x|` and eats the guard.
/// * `x > 0` — the terms peak at `m ≈ x` with value `≈ e^x/√(2πx)`, so
///   the intermediate reaches the SIZE OF THE RESULT and the work integer
///   must host `e^x` on top of the series' own `2·w`-digit product. Inside
///   `x ≤ 1` the peak term is bounded by `1`, so no such lift is needed.
///
/// Both failures are unbounded in the term count as well, and the series
/// stops at `SERIES_CAP` — returning a TRUNCATED, WRONG value rather than
/// a slow one. `WithExp` carries `exp`'s own `k·ln 2` range reduction and
/// its proven peak model, and is uniformly correct there.
///
/// The region is stated as a continuous condition on the series' own
/// convergence and is correct at every width and every scale; it is not
/// fitted to any benchmarked cell. Where inside the region the two
/// kernels actually cross on COST is a separate, un-benched question —
/// the wall may safely be moved once a `policy-mapper` sweep says where.
fn classify<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    let one = crate::consts::pow10::dispatch::<Int<N>>(SCALE);
    // `|x| <= 1`, in raw storage units.
    if *raw <= one && *raw >= -one {
        Algorithm::Series
    } else {
        Algorithm::WithExp
    }
}

#[inline]
fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(choose) => choose(raw),
    }
}

/// `expm1(x) = e^x - 1`, correctly rounded to `SCALE` under `mode`.
///
/// # Panics
///
/// Panics if the result overflows the storage range.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    routed::<N, SCALE>(raw, resolve::<N, SCALE>(&raw), mode)
}

/// `expm1` at a caller-chosen working guard — a single shot, no Ziv
/// escalation. Same routing as [`dispatch`].
///
/// # Panics
///
/// Panics if the result overflows the storage range.
#[inline]
#[must_use]
pub(crate) fn dispatch_with<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    approx_routed::<N, SCALE>(raw, working_digits, resolve::<N, SCALE>(&raw), mode)
}

// ── per-`Algorithm` delegations ─────────────────────────────────────
//
// Thin: each picks the chosen kernel and passes the width's work
// integer, base guard and storage bounds. No computation lives here.

/// Narrow (`N <= 2`) strict arm: compute at [`WZiv`] over `Int<2>`
/// storage, then fit back to `Int<N>`. `WZiv` (`Int<24>`, 1536 bits) is
/// already the widest work integer the narrow tiers have, so BOTH arms
/// run there — the wide split between `C::W` and `C::Wexp` has no narrow
/// analogue.
#[inline]
fn narrow_strict<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    algo: Algorithm,
    mode: RoundingMode,
) -> Int<N> {
    let raw_narrow = raw.resize_to::<Int<2>>();
    let out = match algo {
        Algorithm::Series => expm1_series_g::<Int<2>, WZiv, SCALE>(
            raw_narrow,
            NARROW_GUARD,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
        Algorithm::WithExp => expm1_with_exp_g::<Int<2>, WZiv, SCALE>(
            raw_narrow,
            NARROW_GUARD,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
    };
    super::narrow_checked::<N>(out, "expm1_strict", SCALE)
}

/// Narrow (`N <= 2`) approx arm. See [`narrow_strict`].
#[inline]
fn narrow_approx<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    working_digits: u32,
    algo: Algorithm,
    mode: RoundingMode,
) -> Int<N> {
    let raw_narrow = raw.resize_to::<Int<2>>();
    let out = match algo {
        Algorithm::Series => expm1_series_approx_g::<Int<2>, WZiv, SCALE>(
            raw_narrow,
            working_digits,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
        Algorithm::WithExp => expm1_with_exp_approx_g::<Int<2>, WZiv, SCALE>(
            raw_narrow,
            working_digits,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
    };
    super::narrow_checked::<N>(out, "expm1_approx", SCALE)
}

/// Wide strict arm at the tier `C`. Each kernel sources its own width
/// from the tier: `Series` runs at `C::W`, `WithExp` at the wider
/// `C::Wexp` because it owns the large-argument regime where `e^x`'s
/// internal peak grows.
#[cfg(feature = "_wide-support")]
#[inline]
fn wide_strict<C: crate::algos::support::wide_trig_core::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    algo: Algorithm,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    match algo {
        Algorithm::Series => {
            crate::algos::expm1::expm1_series::expm1_series::<C, SCALE>(raw, mode)
        }
        Algorithm::WithExp => {
            crate::algos::expm1::expm1_with_exp::expm1_with_exp::<C, SCALE>(raw, mode)
        }
    }
}

/// Wide approx arm at the tier `C`. See [`wide_strict`].
#[cfg(feature = "_wide-support")]
#[inline]
fn wide_approx<C: crate::algos::support::wide_trig_core::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    working_digits: u32,
    algo: Algorithm,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    match algo {
        Algorithm::Series => crate::algos::expm1::expm1_series::expm1_series_approx::<C, SCALE>(
            raw,
            working_digits,
            mode,
        ),
        Algorithm::WithExp => crate::algos::expm1::expm1_with_exp::expm1_with_exp_approx::<C, SCALE>(
            raw,
            working_digits,
            mode,
        ),
    }
}

// ── width routing ───────────────────────────────────────────────────
//
// `match N` const-folds per monomorphisation to a single direct call at
// exactly one work integer; the unchosen arms are dead-arm-eliminated.

#[inline]
fn routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    algo: Algorithm,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 | 2 => narrow_strict::<N, SCALE>(raw, algo, mode),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => wide_strict::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => wide_strict::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => wide_strict::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => wide_strict::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => wide_strict::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => wide_strict::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => wide_strict::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => wide_strict::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => wide_strict::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => wide_strict::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), algo, mode).resize_to::<Int<N>>(),
        _ => narrow_strict::<N, SCALE>(raw, algo, mode),
    }
}

#[inline]
fn approx_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    working_digits: u32,
    algo: Algorithm,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 | 2 => narrow_approx::<N, SCALE>(raw, working_digits, algo, mode),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => wide_approx::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => wide_approx::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => wide_approx::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => wide_approx::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => wide_approx::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => wide_approx::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => wide_approx::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => wide_approx::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => wide_approx::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => wide_approx::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), working_digits, algo, mode).resize_to::<Int<N>>(),
        _ => narrow_approx::<N, SCALE>(raw, working_digits, algo, mode),
    }
}
