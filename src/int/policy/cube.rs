// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer cubing policy — the sqr-then-multiply algorithm matcher.
//!
//! `Uint<N>::cube` / `Uint<N>::wrapping_cube` and the `Int<N>` siblings
//! delegate to [`dispatch`], which follows the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real cubing algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Uint<N>` compiles to a
//! direct call to the sqr-then-multiply sequence, no runtime branch.
//!
//! # Algorithm
//!
//! The optimal form of `x³` is `x²·x` — two limb operations rather than
//! three sequential multiplies; no cheaper form exists below two
//! multiplications. The algorithm fn
//! [`crate::int::algos::cube::cube_schoolbook::cube_schoolbook`] computes
//! the square with the const half-product kernel
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] and the final multiply with
//! the const truncated kernel [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`],
//! so the total limb-multiply count is `N(N+1)/2 + N²`. The layering points
//! DOWN — the algorithm calls the kernels, never a cube/sqr/mul method on
//! `Uint<N>`.
//!
//! The `ByValue` arm of [`Select`] is present for canonical-shape
//! uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` IS `const fn`: the algorithm fn computes via const kernels,
//! so the type's `const fn` `wrapping_cube` can delegate through it. The
//! `ByValue` arm returns the default algorithm tag without invoking the fn
//! pointer (calling a fn pointer is not permitted in `const fn`; merely
//! matching the variant is fine).

use crate::int::algos::cube::cube_schoolbook::cube_schoolbook;
use crate::int::types::Uint;

// ── 1. the real cubing algorithm — NAMED, no `Default` ───────────────

/// The cubing algorithms this policy chooses between. The single variant
/// is the CamelCase of the algorithm fn's name minus the `cube_` function
/// prefix (`cube_schoolbook` → `Schoolbook`) — strict 1:1 with the fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`cube_schoolbook`] — sqr-then-multiply sequence `x²·x` via the
    /// const [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] and
    /// [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] kernels. Result is `x³`
    /// modulo `2^BITS`.
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The cube picker always
/// returns `ByAlgorithm`: the choice is fully determined by `N` (which is
/// constant, and the same algorithm wins at every `N`). `ByValue` is part
/// of the canonical shape for uniformity across functions; `select` never
/// returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Uint<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `N`, total over the key ──────────

/// Pick the cubing algorithm for storage limb count `N`. Total over the key;
/// sqr-then-multiply is width-independent so `Schoolbook` wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::Schoolbook)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Integer cubing dispatcher for `Uint<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: `Int<N>::wrapping_cube` is itself `const fn`. The
/// `ByValue` arm returns the default algorithm tag without invoking the fn
/// pointer, satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(x: Uint<N>) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // cube is always ByAlgorithm; fall through to the default if the
        // arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::Schoolbook,
    };
    match algo {
        Algorithm::Schoolbook => cube_schoolbook(x),
    }
}
