// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer cubing policy — the sqr-then-mul algorithm matcher.
//!
//! `Uint<N>::cube` and `Int<N>::cube` delegate to [`dispatch`], which follows
//! the canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
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
//! The optimal form of `x³` is `sqr(x) · x` — two limb operations rather
//! than three sequential multiplies. The squaring step uses the half-product
//! kernel (see [`crate::int::policy::sqr`]), so the total limb-multiply count
//! is `N(N+1)/2 + N²`. No cheaper form exists below two multiplications.
//! `cube_via_mul` routes through `wrapping_cube` which already implements
//! this two-step form.
//!
//! The `ByValue` arm of [`Select`] is present for canonical-shape
//! uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` is **not** `const fn`. `Uint<N>::wrapping_cube` is not `const fn`.
//! `Int<N>::cube` delegates to `Int<N>::wrapping_cube` (which IS `const fn`)
//! directly rather than through this dispatcher. The `ByValue` arm returns
//! the default algorithm tag without invoking the fn pointer.

use crate::int::types::Uint;

// ── 1. the real cubing algorithm — NAMED, no `Default` ───────────────

/// The cubing algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `cube_` function
/// prefix (`cube_via_mul` → `ViaMul`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`cube_via_mul`] — sqr-then-multiply sequence routed through
    /// [`Uint::wrapping_cube`]: `x² · x`. Uses the half-product squaring
    /// kernel for the first step. Result is `x³` modulo `2^BITS`.
    ViaMul,
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
/// sqr-then-multiply is width-independent so `ViaMul` wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::ViaMul)
}

// ── algorithm fn: thin delegation to wrapping_cube ───────────────────

/// Sqr-then-multiply integer cube for `Uint<N>`.
///
/// Delegates to [`Uint::wrapping_cube`] which implements `x² · x` via
/// `wrapping_sqr` (half-product squaring kernel) then `wrapping_mul`.
/// Result is `x³` modulo `2^BITS`.
#[inline]
pub(crate) fn cube_via_mul<const N: usize>(x: Uint<N>) -> Uint<N> {
    x.wrapping_cube()
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Integer cubing dispatcher for `Uint<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Not `const fn`: `Uint<N>::wrapping_cube` is not `const fn`. `Int<N>::cube`
/// delegates to `Int<N>::wrapping_cube` (which IS `const fn`) directly rather
/// than through this dispatcher.
#[inline]
pub(crate) fn dispatch<const N: usize>(x: Uint<N>) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&x),
    };
    match algo {
        Algorithm::ViaMul => cube_via_mul(x),
    }
}
