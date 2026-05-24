// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Compare policy — the default-delegating algorithm matcher for the signed
//! total-order comparison of `Int<N>`.
//!
//! The const comparison primitive `Int<N>::cmp_cross` (called by `Ord::cmp`
//! and `PartialOrd::partial_cmp`) delegates to [`dispatch`], which follows
//! the canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
//!
//! 1. an [`Algorithm`] enum — the real comparison algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Int<N>` compiles to a
//! direct call to the limbwise signed-comparison kernel, with no runtime
//! branch.
//!
//! # Why there is only one algorithm
//!
//! Signed comparison is width-independent: comparing the sign bits first,
//! then the unsigned limb magnitudes from most-significant to least (the
//! limbwise approach in [`cmp_limbwise`]) is the unique correct choice at
//! every limb count `N`. There is no crossover threshold, no value-dependent
//! split, and no work-width widening. The `ByValue` arm of [`Select`] is
//! present for canonical-shape uniformity; `select` never returns it.
//!
//! # Why `dispatch` is `const fn`
//!
//! `Int<N>::cmp_cross` is `const fn` and is called from const contexts
//! across the crate (e.g. `checked_neg`, `is_min_neg_one`). `dispatch`
//! therefore must also be `const fn`. A pure-`ByAlgorithm` dispatch can
//! satisfy this: the `ByValue` arm simply returns the default algorithm tag
//! without invoking the fn pointer (calling a fn pointer is not permitted in
//! `const fn`; merely matching the variant is fine).
//!
//! # Routed primitive
//!
//! The routed primitive is `Int<N>::cmp_cross<N>` — the same-width
//! specialisation of the const cross-width signed comparator. `Ord::cmp`
//! and `PartialOrd::partial_cmp` call `cmp_cross` directly and continue
//! to do so; their observable behaviour is unchanged. This policy wraps the
//! same-width path so the algorithm seam exists for future crossover work
//! (e.g. a SIMD limbwise path) and mirrors every other int policy file.

use crate::int::algos::cmp::cmp_limbwise::cmp_limbwise;
use crate::int::types::Int;
use core::cmp::Ordering;

// ── 1. the real comparison algorithm — NAMED, no `Default` ───────────

/// The comparison algorithms this policy chooses between. The single
/// variant is the CamelCase of the kernel fn's name minus the `cmp_`
/// function prefix (`cmp_limbwise` → `Limbwise`) — strict 1:1 with the
/// kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`cmp_limbwise`] — sign-first then unsigned-magnitude limbwise
    /// comparison, delegating to
    /// [`cmp_fixed`](crate::int::algos::support::limbs::cmp_fixed)
    /// (same-width path) or
    /// [`cmp_cross`](crate::int::algos::support::limbs::cmp_cross)
    /// (cross-width path, present for the generic form).
    Limbwise,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The cmp picker always
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

/// Pick the comparison algorithm for storage limb count `N`. Total over
/// the key; signed comparison is width-independent so `Limbwise` wins at
/// every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::Limbwise)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Signed integer comparison dispatcher for `Int<N>` (same-width).
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: the comparison primitive is `const fn` and is
/// called from `const` contexts across the crate. The `ByValue` arm
/// returns the default algorithm tag without invoking the fn pointer,
/// satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(a: Int<N>, b: Int<N>) -> Ordering {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // cmp is always ByAlgorithm; fall through to the default if
        // the arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::Limbwise,
    };
    match algo {
        Algorithm::Limbwise => cmp_limbwise(a, b),
    }
}
