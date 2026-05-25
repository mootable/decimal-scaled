// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer sum-of-squares policy -- the per-`N` algorithm matcher for
//! `a^2 + b^2`.
//!
//! `Int<N>::sum_sq` delegates to [`dispatch`], which follows the canonical
//! policy shape (see `docs/ARCHITECTURE.md` -> "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum -- the real sum-of-squares algorithms, no
//!    `Default` variant;
//! 2. a [`Select`] verdict -- a settled algorithm or "the value decides"
//!    (`sum_sq` has no value split, so `ByValue` is never returned);
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
//! One algorithm
//! ([`crate::int::algos::sum_sq::sum_sq_schoolbook`]): form `a^2 + b^2` in a
//! limb scratch buffer via the schoolbook product and fit-check to `Int<N>`.
//! `select` returns [`Algorithm::Schoolbook`] at every `N`.
//!
//! Returns [`None`] from the kernel on true overflow (the sum does not fit
//! the signed range of `Int<N>`); the type method propagates that `Option`.

use crate::int::algos::sum_sq::sum_sq_schoolbook::sum_sq_schoolbook;
use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;

// -- 1. the real sum-of-squares algorithms -- NAMED, no `Default` ------

/// The integer sum-of-squares algorithms this policy chooses between.
/// Variants are the CamelCase of each kernel fn's name minus the `sum_sq_`
/// prefix -- strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`sum_sq_schoolbook`] -- `a^2 + b^2` via the schoolbook product into
    /// a limb scratch buffer. The sole sum-of-squares algorithm.
    Schoolbook,
}

// -- 2. the const verdict ----------------------------------------------

/// A settled algorithm, or "the value decides". The sum-of-squares picker
/// always returns `ByAlgorithm`. `ByValue` is part of the canonical shape
/// for uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// -- 3. the matcher: const, keyed on `N`, total over the key -----------

/// Pick the integer sum-of-squares algorithm for storage limb count `N`.
/// Total over the key; [`Algorithm::Schoolbook`] wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    let _ = N; // key accepted for uniformity; one algorithm at every width
    Select::ByAlgorithm(Algorithm::Schoolbook)
}

// -- 4. the shared dispatch: resolve the verdict, then dispatch --------

/// Integer sum-of-squares dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release), then dispatches exhaustively over [`Algorithm`].
/// Returns [`None`] when `a^2 + b^2` does not fit the signed range of
/// `Int<N>` (true overflow). The signs of the operands drop out of squaring.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize>(a: Int<N>, b: Int<N>) -> Option<Int<N>>
where
    Int<N>: ComputeInt,
{
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a),
    };
    match algo {
        Algorithm::Schoolbook => sum_sq_schoolbook::<N>(a, b),
    }
}
