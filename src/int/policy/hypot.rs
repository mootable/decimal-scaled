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
//! Two algorithms:
//!
//! - [`crate::int::algos::hypot::hypot_pythagoras`] -- form `a^2 + b^2` in a
//!   limb scratch buffer, take the floor root via the Newton slice `isqrt`,
//!   round. The width-agnostic exact path.
//! - [`crate::int::algos::hypot::hypot_u128_fast`] -- a value-gated wrapper:
//!   when both operands fit a single `u64` limb the radicand fits a `u128`, so
//!   it roots in `u128` (integer Newton, no multi-precision `div_rem`) and
//!   rounds with the exact remainder; otherwise it falls through to
//!   `hypot_pythagoras`, paying only the cheap `fit_one` fit guard. It is
//!   bit-identical to `hypot_pythagoras` for every input and `RoundingMode`.
//!
//! `select` returns [`Algorithm::U128Fast`] at **every** `N`. The `hypot_ab`
//! microbench (per-width, per-shape; pinned core 22) measured `u128_fast`
//! 1.4-2.8x faster than `pythagoras` whenever both operands fit one limb (the
//! `single`-shape input) at every tier from `N=2` through `N=64`, and
//! statistically TIED with it on full-width (`multi`/`skew`) operands where the
//! fast branch falls through. So the fast path strictly dominates the plain
//! Pythagoras path at every width; `Pythagoras` is the kept-but-unselected
//! fallback (and remains the kernel `u128_fast` itself delegates to). The
//! per-width root choice inside the fallback is the `isqrt` policy's job, not
//! hypot's.
//!
//! Returns [`None`] from the kernel on true overflow (the rounded root does
//! not fit the signed range of `Int<N>`); the type method maps that to its
//! `Option` return.

use crate::int::algos::hypot::hypot_pythagoras::hypot_pythagoras;
use crate::int::algos::hypot::hypot_u128_fast::hypot_u128_fast;
use crate::int::types::compute_int::ComputeInt;
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
    /// `isqrt`. The width-agnostic exact path; the kept-but-unselected
    /// fallback (and the kernel [`U128Fast`](Algorithm::U128Fast) itself
    /// delegates to when the radicand exceeds a `u128`).
    Pythagoras,
    /// [`hypot_u128_fast`] -- native-`u128` fast path: when both operands fit
    /// a single `u64` limb the radicand `a^2 + b^2` fits a `u128`, so it
    /// floors the root in `u128` (integer-seeded Newton + exact-remainder
    /// round, no multi-precision `div_rem`), falling back to
    /// [`hypot_pythagoras`] otherwise (paying only the cheap `fit_one` guard).
    /// Bit-identical to Pythagoras; the `hypot_ab` microbench measured it
    /// 1.4-2.8x faster on single-limb operands at every width (`N=2..=64`) and
    /// tied on full-width operands -- so it is selected at every tier.
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
/// the key. Every tier takes the native-u128 fast path
/// ([`Algorithm::U128Fast`]): the `hypot_ab` microbench (pinned, per-shape)
/// measured it 1.4-2.8x faster than [`Algorithm::Pythagoras`] whenever both
/// operands fit one `u64` limb -- across `N=2..=64` -- and statistically tied
/// on full-width operands (it just falls through to Pythagoras, paying only
/// the cheap `fit_one` guard). So it strictly dominates the plain Pythagoras
/// path at every width; `Pythagoras` is the kept fallback, never selected
/// directly.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::U128Fast)
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
    Int<N>: ComputeInt,
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
