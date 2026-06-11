// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Add policy — the default-delegating algorithm matcher for integer addition.
//!
//! `Int<N>::wrapping_add` and the `Add` operator delegate to [`dispatch`],
//! which follows the canonical policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real addition algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Int<N>` compiles to a
//! direct call to the ripple-carry limb kernel, no runtime branch.
//!
//! # Why there is only one algorithm
//!
//! Integer addition is width-independent: the ripple-carry accumulator in
//! [`crate::int::algos::support::limbs::add_assign_fixed`] is the unique correct
//! choice at every limb count `N`. There is no crossover threshold, no
//! value-dependent split, and no work-width widening (work width ==
//! storage width). The `ByValue` arm of [`Select`] is present for
//! canonical-shape uniformity; `select` never returns it.
//!
//! # Why `dispatch` is `const fn`
//!
//! `Int<N>::wrapping_add` is itself `const fn` and is called from const
//! constructors and other `const fn` contexts across the crate. `dispatch`
//! therefore must also be `const fn`. A pure-`ByAlgorithm` dispatch can
//! satisfy this: the `ByValue` arm simply returns the default algorithm tag
//! without invoking the fn pointer (calling a fn pointer is not permitted in
//! `const fn`; merely matching the variant is fine).

use crate::int::algos::add::add_ripple_carry::add_ripple_carry;
use crate::int::types::Int;

// ── 1. the real addition algorithm — NAMED, no `Default` ─────────────

/// The addition algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `add_` function
/// prefix (`add_ripple_carry` → `RippleCarry`) — strict 1:1 with the
/// kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`add_ripple_carry`] — ripple-carry accumulator over the `N` limbs,
    /// delegating to [`crate::int::algos::support::limbs::add_assign_fixed`].
    RippleCarry,
    /// Schoolbook reference tag — delegates to the same [`add_ripple_carry`]
    /// kernel. Ripple-carry IS the schoolbook addition algorithm; this
    /// variant exists as an explicit, benchmarkable seam.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The add picker always
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

/// Pick the addition algorithm for storage limb count `N`. Total over the
/// key; addition is width-independent so `RippleCarry` wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::RippleCarry)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Wrapping integer addition dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: `Int<N>::wrapping_add` is itself `const fn` and is
/// called from `const` contexts across the crate. The `ByValue` arm
/// returns the default algorithm tag without invoking the fn pointer,
/// satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // add is always ByAlgorithm; fall through to the default if
        // the arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::RippleCarry,
    };
    match algo {
        Algorithm::RippleCarry => add_ripple_carry(a, b),
        Algorithm::Schoolbook => add_ripple_carry(a, b),
    }
}

/// Checked door over the SAME verdict: routes to the fused
/// single-pass checked kernel (`add_ripple_carry_checked` — ripple +
/// overflow verdict in one traversal, the `Option` returned directly).
/// `Int::checked_add` (and through it the decimal `+` operator's
/// panic-on-overflow contract) enters here; the layered
/// `wrapping_add`-then-sign-check shape it replaces measured ≈2× the
/// bare loop at 24 limbs in by-value moves between the layers.
#[inline]
pub(crate) const fn dispatch_checked<const N: usize>(a: Int<N>, b: Int<N>) -> Option<Int<N>> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::RippleCarry,
    };
    match algo {
        Algorithm::RippleCarry | Algorithm::Schoolbook => {
            crate::int::algos::add::add_ripple_carry::add_ripple_carry_checked(a, b)
        }
    }
}
