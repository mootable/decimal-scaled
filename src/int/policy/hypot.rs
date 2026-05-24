// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer hypotenuse policy -- the per-`N` algorithm matcher for
//! `round(sqrt(a^2 + b^2))`.
//!
//! `Int<N>::hypot` delegates to [`dispatch`], which follows the canonical
//! policy shape (see `docs/ARCHITECTURE.md` -> "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum -- the real hypot algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict -- a settled algorithm or "the value decides"
//!    (`hypot` has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` -- no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on `N`, the `const { ... }`
//! block folds per monomorphisation and the unchosen arm is
//! dead-arm-eliminated in release: each concrete `Int<N>` compiles to a
//! direct call to one kernel, no runtime branch.
//!
//! # Algorithm
//!
//! One algorithm ([`crate::int::algos::hypot::hypot_pythagoras`]): form
//! `a^2 + b^2` in a limb scratch buffer, take the floor root via the Newton
//! slice `isqrt`, round. `select` returns [`Algorithm::Pythagoras`] at every
//! `N`; the `Schoolbook` arm is the canonical benchmarkable seam pointing at
//! the same kernel (the per-width root choice is the `isqrt` policy's job,
//! not hypot's).
//!
//! Returns [`None`] from the kernel on true overflow (the rounded root does
//! not fit the signed range of `Int<N>`); the type method maps that to its
//! `Option` return.

use crate::int::algos::hypot::hypot_pythagoras::hypot_pythagoras;
use crate::int::algos::hypot::hypot_u128_fast::hypot_u128_fast;
use crate::int::types::work_scratch::WorkingInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// -- 1. the real hypot algorithms -- NAMED, no `Default` --------------

/// The integer hypotenuse algorithms this policy chooses between. Variants
/// are the CamelCase of each kernel fn's name minus the `hypot_` prefix --
/// strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`hypot_pythagoras`] -- `round(sqrt(a^2 + b^2))` forming the radicand
    /// in a limb scratch and taking the floor root through the Newton slice
    /// `isqrt`. The width-agnostic path; selected for the wide tiers.
    Pythagoras,
    /// [`hypot_u128_fast`] -- native-`u128` fast path for the narrow tiers
    /// (`N <= 3`): when `a^2 + b^2` fits a `u128` it floors the root in u128
    /// (f64 seed + exact-remainder round, no multi-precision `div_rem`),
    /// falling back to [`hypot_pythagoras`] for radicands that don't fit.
    /// Bit-identical to Pythagoras; the `hypot_ab` microbench measured it
    /// 2.1-3.1x faster at D18/D38/D57.
    U128Fast,
}

// -- 2. the const verdict ----------------------------------------------

/// A settled algorithm, or "the value decides". The hypot picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// -- 3. the matcher: const, keyed on `N`, total over the key -----------

/// Pick the integer hypot algorithm for storage limb count `N`. Total over
/// the key. Narrow tiers (`N <= 3`: D18/D38/D57) take the native-u128 fast
/// path (`hypot_ab` microbench: 2.1-3.1x faster, bit-identical); the wider
/// tiers — whose radicand never fits a u128, so the fast path would always
/// fall back and only pay its fit-guard — stay on the generic Pythagoras.
const fn select<const N: usize>() -> Select<N> {
    match N {
        1 | 2 | 3 => Select::ByAlgorithm(Algorithm::U128Fast),
        _ => Select::ByAlgorithm(Algorithm::Pythagoras),
    }
}

// -- 4. the shared dispatch: resolve the verdict, then dispatch --------

/// Integer hypotenuse dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release), then dispatches exhaustively over [`Algorithm`].
/// Returns [`None`] when the rounded root does not fit the signed range of
/// `Int<N>` (true overflow). Negative inputs are handled by squaring (the
/// sign drops out).
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode) -> Option<Int<N>>
where
    Int<N>: WorkingInt,
{
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a),
    };
    match algo {
        Algorithm::Pythagoras => hypot_pythagoras::<N>(a, b, mode),
        Algorithm::U128Fast => hypot_u128_fast::<N>(a, b, mode),
    }
}
