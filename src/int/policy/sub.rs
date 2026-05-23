// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Subtract policy — the default-delegating algorithm matcher for integer
//! subtraction.
//!
//! `Int<N>::wrapping_sub` and the `Sub` operator delegate to [`dispatch`],
//! which follows the canonical policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real subtraction algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Int<N>` compiles to a
//! direct call to the ripple-borrow limb kernel, no runtime branch.
//!
//! # Why there is only one algorithm
//!
//! Integer subtraction is width-independent: the ripple-borrow accumulator in
//! [`crate::int::algos::support::limbs::sub_assign_fixed`] is the unique correct
//! choice at every limb count `N`. There is no crossover threshold, no
//! value-dependent split, and no work-width widening (work width ==
//! storage width). The `ByValue` arm of [`Select`] is present for
//! canonical-shape uniformity; `select` never returns it.
//!
//! # Why `dispatch` is `const fn`
//!
//! `Int<N>::wrapping_sub` is itself `const fn` and is called from const
//! contexts across the crate (e.g. `checked_sub`, `overflowing_sub`,
//! `div_euclid`). `dispatch` therefore must also be `const fn`. A
//! pure-`ByAlgorithm` dispatch can satisfy this: the `ByValue` arm simply
//! returns the default algorithm tag without invoking the fn pointer
//! (calling a fn pointer is not permitted in `const fn`; merely matching
//! the variant is fine).

use crate::int::algos::support::limbs::sub_assign_fixed;
use crate::int::types::Int;

// ── 1. the real subtraction algorithm — NAMED, no `Default` ──────────

/// The subtraction algorithms this policy chooses between. The single
/// variant is the CamelCase of the kernel fn's name minus the `sub_`
/// function prefix (`sub_ripple_borrow` → `RippleBorrow`) — strict 1:1
/// with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`sub_ripple_borrow`] — ripple-borrow accumulator over the `N` limbs,
    /// delegating to [`crate::int::algos::support::limbs::sub_assign_fixed`].
    RippleBorrow,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The sub picker always
/// returns `ByAlgorithm`: the choice is fully determined by `N` (which
/// is constant, and the same algorithm wins at every `N`). `ByValue` is
/// part of the canonical shape for uniformity across functions; `select`
/// never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `N`, total over the key ──────────

/// Pick the subtraction algorithm for storage limb count `N`. Total over
/// the key; subtraction is width-independent so `RippleBorrow` wins at
/// every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::RippleBorrow)
}

// ── algorithm fn: thin delegation to the limb kernel ─────────────────

/// Ripple-borrow integer subtraction for `Int<N>`. Wraps
/// [`sub_assign_fixed`], discarding the borrow-out so the result wraps
/// modulo `2^BITS` (two's-complement wrapping semantics).
#[inline]
pub(crate) const fn sub_ripple_borrow<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    let mut limbs = *a.as_limbs();
    sub_assign_fixed(&mut limbs, b.as_limbs());
    Int::<N>::from_limbs(limbs)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Wrapping integer subtraction dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: `Int<N>::wrapping_sub` is itself `const fn` and is
/// called from `const` contexts across the crate. The `ByValue` arm
/// returns the default algorithm tag without invoking the fn pointer,
/// satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // sub is always ByAlgorithm; fall through to the default if
        // the arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::RippleBorrow,
    };
    match algo {
        Algorithm::RippleBorrow => sub_ripple_borrow(a, b),
    }
}
