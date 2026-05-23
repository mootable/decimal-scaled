// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer exponentiation policy — the square-and-multiply algorithm matcher.
//!
//! `Uint<N>::pow` and `Int<N>::pow` delegate to [`dispatch`], which follows
//! the canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
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
//! # Why there is only one algorithm
//!
//! Binary square-and-multiply (exponentiation by squaring) is optimal for
//! the small fixed exponents `pow` is used with in this crate (root
//! iterations: `k-1`, `k` ≤ ~10). There is no width-specific crossover
//! and no value-split. The `ByValue` arm of [`Select`] is present for
//! canonical-shape uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` is **not** `const fn`. `Uint<N>::wrapping_pow` is not `const fn`
//! (it mutates a local `base` via non-const squaring). `Int<N>::pow` IS
//! `const fn` and continues to delegate to `Int<N>::wrapping_pow` directly
//! rather than through this dispatcher. The `ByValue` arm returns the default
//! algorithm tag without invoking the fn pointer.

use crate::int::types::Uint;

// ── 1. the real pow algorithm — NAMED, no `Default` ──────────────────

/// The exponentiation algorithms this policy chooses between. The single
/// variant is the CamelCase of the kernel fn's name minus the `pow_`
/// function prefix (`pow_square_and_multiply` → `SquareAndMultiply`) —
/// strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`pow_square_and_multiply`] — binary exponentiation by squaring using
    /// the dedicated squaring kernel for the square step and a truncating
    /// multiply for the multiply step. `self^0 == 1`; result wraps modulo
    /// `2^BITS`.
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

// ── algorithm fn: thin delegation to the wrapping_pow kernel ─────────

/// Binary exponentiation by squaring for `Uint<N>`.
///
/// Delegates to [`Uint::wrapping_pow`], which uses `wrapping_sqr` for the
/// square step and `wrapping_mul` for the multiply step. Result is
/// `self^exp` modulo `2^BITS`; `self^0 == 1`.
#[inline]
pub(crate) fn pow_square_and_multiply<const N: usize>(base: Uint<N>, exp: u32) -> Uint<N> {
    base.wrapping_pow(exp)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Integer exponentiation dispatcher for `Uint<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Not `const fn`: `Uint<N>::wrapping_pow` is not `const fn`. `Int<N>::pow`
/// (which IS `const fn`) delegates directly to `Int<N>::wrapping_pow`
/// rather than through this dispatcher. The `ByValue` arm returns the
/// default algorithm tag without invoking the fn pointer.
#[inline]
pub(crate) fn dispatch<const N: usize>(base: Uint<N>, exp: u32) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&base, exp),
    };
    match algo {
        Algorithm::SquareAndMultiply => pow_square_and_multiply(base, exp),
    }
}
