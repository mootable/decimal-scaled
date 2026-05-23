// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer exponentiation policy — the square-and-multiply algorithm matcher.
//!
//! `Uint<N>::pow` / `Uint<N>::wrapping_pow` and the `Int<N>` siblings
//! delegate to [`dispatch`], which follows the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real pow algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Uint<N>` compiles to a
//! direct call to the square-and-multiply kernel, no runtime branch.
//!
//! # Algorithm
//!
//! The algorithm fn
//! [`crate::int::algos::pow::pow_square_and_multiply::pow_square_and_multiply`]
//! is the binary square-and-multiply loop computing via the const
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] (square step) and
//! [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] (multiply step) kernels.
//! Binary exponentiation by squaring is optimal for the small fixed
//! exponents `pow` is used with in this crate (root iterations: `k-1`,
//! `k` ≤ ~10). There is no width-specific crossover and no value-split. The
//! layering points DOWN — the algorithm calls the kernels, never a
//! pow/sqr/mul method on `Uint<N>`. The `ByValue` arm of [`Select`] is
//! present for canonical-shape uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` IS `const fn`: the algorithm fn computes via const kernels,
//! so the type's `const fn` `wrapping_pow` can delegate through it. The
//! `ByValue` arm returns the default algorithm tag without invoking the fn
//! pointer (calling a fn pointer is not permitted in `const fn`; merely
//! matching the variant is fine).

use crate::int::algos::pow::pow_square_and_multiply::pow_square_and_multiply;
use crate::int::types::Uint;

// ── 1. the real pow algorithm — NAMED, no `Default` ──────────────────

/// The exponentiation algorithms this policy chooses between. The single
/// variant is the CamelCase of the algorithm fn's name minus the `pow_`
/// function prefix (`pow_square_and_multiply` → `SquareAndMultiply`) —
/// strict 1:1 with the fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`pow_square_and_multiply`] — binary exponentiation by squaring
    /// using the const [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] kernel
    /// for the square step and the const truncated
    /// [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] kernel for the multiply
    /// step. `self^0 == 1`; result wraps modulo `2^BITS`.
    SquareAndMultiply,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The pow picker always
/// returns `ByAlgorithm`: the choice is fully determined by `N` (which is
/// constant, and the same algorithm wins at every `N`). `ByValue` is part of
/// the canonical shape for uniformity across functions; `select` never
/// returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Uint<N>, u32) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `N`, total over the key ──────────

/// Pick the pow algorithm for storage limb count `N`. Total over the key;
/// square-and-multiply is width-independent so `SquareAndMultiply` wins at
/// every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::SquareAndMultiply)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Integer exponentiation dispatcher for `Uint<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: `Int<N>::wrapping_pow` is itself `const fn`. The
/// `ByValue` arm returns the default algorithm tag without invoking the fn
/// pointer, satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(base: Uint<N>, exp: u32) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // pow is always ByAlgorithm; fall through to the default if the
        // arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::SquareAndMultiply,
    };
    match algo {
        Algorithm::SquareAndMultiply => pow_square_and_multiply(base, exp),
    }
}
