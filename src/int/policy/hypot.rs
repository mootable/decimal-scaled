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
//! # Algorithm selection
//!
//! Both algorithms are valid at every `N`. The production kernel
//! ([`crate::int::algos::hypot::hypot_isqrt`]) takes the floor root via the
//! Newton slice `isqrt`; the [`Schoolbook`](Algorithm::Schoolbook)
//! reference uses the division-free bitwise `isqrt`. The Newton path wins
//! everywhere, so `select` returns [`Algorithm::Isqrt`] for every `N`; the
//! `Schoolbook` arm is the registered benchmarkable seam.
//!
//! Returns [`None`] from the kernel on true overflow (the rounded root does
//! not fit the signed range of `Int<N>`); the type method maps that to its
//! `Option` return.

use crate::int::algos::hypot::hypot_isqrt::hypot_isqrt;
use crate::int::algos::hypot::hypot_schoolbook::hypot_schoolbook;
use crate::int::types::work_scratch::WorkScratch;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// -- 1. the real hypot algorithms -- NAMED, no `Default` --------------

/// The integer hypotenuse algorithms this policy chooses between. Variants
/// are the CamelCase of each kernel fn's name minus the `hypot_` prefix --
/// strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`hypot_isqrt`] -- `round(sqrt(a^2 + b^2))` taking the floor root
    /// through the Newton slice `isqrt`. The generic default for every `N`.
    Isqrt,
    /// [`hypot_schoolbook`] -- same arithmetic, floor root via the
    /// division-free, float-free bitwise schoolbook `isqrt`. The
    /// correctness baseline / benchmarkable seam; numerically identical.
    #[allow(dead_code)]
    Schoolbook,
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
/// the key; [`Algorithm::Isqrt`] wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    let _ = N; // key accepted for uniformity; one algorithm at every width
    Select::ByAlgorithm(Algorithm::Isqrt)
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
    Int<N>: WorkScratch,
{
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a),
    };
    match algo {
        Algorithm::Isqrt => hypot_isqrt::<N>(a, b, mode),
        Algorithm::Schoolbook => hypot_schoolbook::<N>(a, b, mode),
    }
}
