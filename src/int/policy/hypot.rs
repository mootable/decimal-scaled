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
//! - [`crate::int::algos::hypot::hypot_u128_fast`] -- a value-gated wrapper
//!   with TWO scalar fast arms: when both operands fit a single `u64` limb the
//!   radicand fits a `u128`, rooted in `u128`; when both fit two `u64` limbs
//!   (`< 2^128`) the radicand fits a `u256`, rooted in fixed `u256`/`u128`
//!   scalar arithmetic (Newton whose only divide is a fixed `u256 / u128`
//!   long division -- no multi-precision `div_rem`). Both round with the exact
//!   remainder; operands beyond `2^128` fall through to `hypot_pythagoras`,
//!   paying only the cheap `fit_k` guard. Bit-identical to `hypot_pythagoras`
//!   for every input and `RoundingMode`.
//!
//! `select` returns [`Algorithm::U128Fast`] at **every** `N`. The `hypot_ab`
//! microbench (per-width, per-shape; pinned core 22) measured `u128_fast`
//! ~3.5x faster than `pythagoras` on single-`u64`-limb operands (`single`) and
//! ~2.0-2.5x faster on two-`u64`-limb operands (`two` -- the decimal `s >= 19`
//! slow band the old `fit_one`-only gate cliffed into Pythagoras) at every tier
//! from `N=2` through `N=64`, and statistically TIED on full-width
//! (`multi`/`skew`) operands where the fast branch falls through. So the fast
//! path strictly dominates the plain Pythagoras path at every width;
//! `Pythagoras` is the kept-but-unselected fallback (and remains the kernel
//! `u128_fast` itself delegates to). The per-width root choice inside the
//! fallback is the `isqrt` policy's job, not hypot's.
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
    /// [`hypot_u128_fast`] -- native scalar fast path with two arms: when both
    /// operands fit a single `u64` limb the radicand fits a `u128` (rooted in
    /// `u128`); when both fit two `u64` limbs (`< 2^128`) the radicand fits a
    /// `u256` (rooted in fixed `u256`/`u128` scalar Newton, no multi-precision
    /// `div_rem`). Both exact-remainder round; operands beyond `2^128` fall
    /// back to [`hypot_pythagoras`] (paying only the cheap `fit_k` guard).
    /// Bit-identical to Pythagoras; the `hypot_ab` microbench measured it
    /// ~3.5x faster on single-limb and ~2.0-2.5x on two-limb operands at every
    /// width (`N=2..=64`) and tied on full-width operands -- selected at every
    /// tier.
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
/// the key. Every tier takes the native scalar fast path
/// ([`Algorithm::U128Fast`]): the `hypot_ab` microbench (pinned, per-shape)
/// measured it ~3.5x faster than [`Algorithm::Pythagoras`] on single-`u64`-limb
/// operands and ~2.0-2.5x faster on two-`u64`-limb operands (`< 2^128`) --
/// across `N=2..=64` -- and statistically tied on full-width operands (it just
/// falls through to Pythagoras, paying only the cheap `fit_k` guard). So it
/// strictly dominates the plain Pythagoras path at every width; `Pythagoras`
/// is the kept fallback, never selected directly.
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
