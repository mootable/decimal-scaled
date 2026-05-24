// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Negate policy — the default-delegating algorithm matcher for integer
//! two's-complement negation.
//!
//! `Int<N>::wrapping_neg` and the `Neg` operator delegate to [`dispatch`],
//! which follows the canonical policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real negation algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Int<N>` compiles to a
//! direct call to the two's-complement bitwise-NOT-plus-one kernel, with
//! no runtime branch.
//!
//! # Why there is only one algorithm
//!
//! Integer two's-complement negation is width-independent: the bitwise-NOT
//! followed by a carry-propagating increment is the unique correct choice at
//! every limb count `N`. There is no crossover threshold, no value-dependent
//! split, and no work-width widening (work width == storage width). The
//! `ByValue` arm of [`Select`] is present for canonical-shape uniformity;
//! `select` never returns it.
//!
//! # Why `dispatch` is `const fn`
//!
//! `Int<N>::wrapping_neg` is itself `const fn` and is called from const
//! contexts across the crate (e.g. `abs`, `wrapping_div`, `wrapping_rem`,
//! `from_mag_limbs`). `dispatch` therefore must also be `const fn`. A
//! pure-`ByAlgorithm` dispatch can satisfy this: the `ByValue` arm simply
//! returns the default algorithm tag without invoking the fn pointer
//! (calling a fn pointer is not permitted in `const fn`; merely matching
//! the variant is fine).

use crate::int::algos::neg::neg_twos_complement::neg_twos_complement;
use crate::int::types::Int;

// ── 1. the real negation algorithm — NAMED, no `Default` ─────────────

/// The negation algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `neg_` function
/// prefix (`neg_twos_complement` → `TwosComplement`) — strict 1:1 with
/// the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`neg_twos_complement`] — bitwise-NOT-plus-one over the `N` limbs,
    /// the canonical two's-complement negation. `MIN` negates to itself,
    /// matching the primitive signed integer contract.
    TwosComplement,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The neg picker always
/// returns `ByAlgorithm`: the choice is fully determined by `N` (which
/// is constant, and the same algorithm wins at every `N`). `ByValue` is
/// part of the canonical shape for uniformity across functions; `select`
/// never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `N`, total over the key ──────────

/// Pick the negation algorithm for storage limb count `N`. Total over the
/// key; negation is width-independent so `TwosComplement` wins at every
/// `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::TwosComplement)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Wrapping integer negation dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: `Int<N>::wrapping_neg` is itself `const fn` and is
/// called from `const` contexts across the crate. The `ByValue` arm
/// returns the default algorithm tag without invoking the fn pointer,
/// satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(a: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // neg is always ByAlgorithm; fall through to the default if
        // the arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::TwosComplement,
    };
    match algo {
        Algorithm::TwosComplement => neg_twos_complement(a),
    }
}
