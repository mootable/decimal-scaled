// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log1p` policy — the per-(N, SCALE) algorithm matcher.
//!
//! `D<Int<N>, SCALE>::log1p_strict_with(mode)` delegates to [`dispatch`];
//! `log1p_approx_with` delegates to [`dispatch_with`]. Canonical
//! matcher-only shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `policy::ln`:
//!
//! 1. an [`Algorithm`] enum — Artanh / WithLn, no `Default`;
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
//! in — lives in the routing fns below, like `policy::ln`'s Tang rung:
//! never in the [`Select`] verdict, never on [`Algorithm`], never in a
//! `dispatch` signature (the BigRule). Narrow (`N <= 2`) computes at
//! [`WZiv`] and narrows to storage; each wide tier computes at its own
//! `Core::W`. Both reach the SAME generic kernels — no per-tier
//! algorithm copy.

use crate::algos::log1p::log1p_artanh::{log1p_artanh_approx_g, log1p_artanh_g};
use crate::algos::log1p::log1p_with_ln::{log1p_with_ln_approx_g, log1p_with_ln_g};
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
    /// `2·artanh(t / (2 + t))` — [`crate::algos::log1p::log1p_artanh`].
    Artanh,
    /// `ln` of the exactly-formed `1 + t` —
    /// [`crate::algos::log1p::log1p_with_ln`]. A composition, so the
    /// variant names the composition rather than a single kernel fn
    /// (the documented `ExpWithLn` precedent).
    WithLn,
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

/// Routes `t ∈ [-1/2, +1]` to [`Algorithm::Artanh`] and everything else
/// to [`Algorithm::WithLn`].
///
/// This is a **validity wall, not a tuning threshold.** The artanh
/// kernel evaluates `u + u³/3 + u⁵/5 + …` with `u = t / (2 + t)` and NO
/// range reduction, so its convergence ratio is `u²`. The region above
/// is exactly the preimage of `|u| ≤ 1/3` — a ratio of at most `1/9`,
/// i.e. at most `≈2.1·w` terms at working scale `w`. With `w` bounded by
/// the Ziv precision horizon (~1264 digits) that is ~2 600 terms,
/// comfortably inside the kernel's 20 000-iteration series cap. Outside
/// the region `|u| → 1` as `t → -1` or `t → ∞`, the term count grows
/// without bound and the series hits that cap — returning a truncated,
/// WRONG value rather than a slow one. `WithLn` carries `ln`'s own
/// multi-level sqrt reduction and is uniformly correct there.
///
/// The region is stated as a continuous condition on the series ratio
/// and is correct at every width and every scale; it is not fitted to
/// any benchmarked cell. Where inside the region the two kernels
/// actually cross on COST is a separate, un-benched question — the
/// wall may safely be narrowed once a `policy-mapper` sweep says where.
fn classify<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    let one = crate::consts::pow10::dispatch::<Int<N>>(SCALE);
    // `t <= 1` and `t >= -1/2`, in raw storage units.
    if *raw <= one && *raw >= -(one >> 1) {
        Algorithm::Artanh
    } else {
        Algorithm::WithLn
    }
}

#[inline]
fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(raw),
    }
}

/// `log1p(t) = ln(1 + t)`, correctly rounded to `SCALE` under `mode`.
///
/// # Panics
///
/// Panics if `t <= -1`, or if the result overflows the storage range.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    routed::<N, SCALE>(raw, resolve::<N, SCALE>(&raw), mode)
}

/// `log1p` at a caller-chosen working guard — a single shot, no Ziv
/// escalation. Same routing as [`dispatch`].
///
/// # Panics
///
/// Panics if `t <= -1`, or if the result overflows the storage range.
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
/// storage, then fit back to `Int<N>`.
#[inline]
fn narrow_strict<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    algo: Algorithm,
    mode: RoundingMode,
) -> Int<N> {
    let v = raw.resize_to::<Int<2>>();
    let out = match algo {
        Algorithm::Artanh => log1p_artanh_g::<Int<2>, WZiv, SCALE>(
            v,
            NARROW_GUARD,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
        Algorithm::WithLn => log1p_with_ln_g::<Int<2>, WZiv, SCALE>(
            v,
            NARROW_GUARD,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
    };
    super::narrow_checked::<N>(out, "log1p_strict", SCALE)
}

/// Narrow (`N <= 2`) approx arm. See [`narrow_strict`].
#[inline]
fn narrow_approx<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    working_digits: u32,
    algo: Algorithm,
    mode: RoundingMode,
) -> Int<N> {
    let v = raw.resize_to::<Int<2>>();
    let out = match algo {
        Algorithm::Artanh => log1p_artanh_approx_g::<Int<2>, WZiv, SCALE>(
            v,
            working_digits,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
        Algorithm::WithLn => log1p_with_ln_approx_g::<Int<2>, WZiv, SCALE>(
            v,
            working_digits,
            Int::<2>::MAX,
            Int::<2>::MIN,
            mode,
        ),
    };
    super::narrow_checked::<N>(out, "log1p_approx", SCALE)
}

/// Wide strict arm at the tier `C` — the tier supplies the work integer
/// `C::W`, the base guard and the storage bounds.
#[cfg(feature = "_wide-support")]
#[inline]
fn wide_strict<C: crate::algos::support::wide_trig_core::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    algo: Algorithm,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    match algo {
        Algorithm::Artanh => {
            crate::algos::log1p::log1p_artanh::log1p_artanh::<C, SCALE>(raw, mode)
        }
        Algorithm::WithLn => {
            crate::algos::log1p::log1p_with_ln::log1p_with_ln::<C, SCALE>(raw, mode)
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
{
    match algo {
        Algorithm::Artanh => crate::algos::log1p::log1p_artanh::log1p_artanh_approx::<C, SCALE>(
            raw,
            working_digits,
            mode,
        ),
        Algorithm::WithLn => crate::algos::log1p::log1p_with_ln::log1p_with_ln_approx::<C, SCALE>(
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
