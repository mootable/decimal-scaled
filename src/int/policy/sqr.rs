// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer squaring policy — the via-mul algorithm matcher.
//!
//! `Uint<N>::sqr` and `Int<N>::sqr` delegate to [`dispatch`], which follows
//! the canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
//!
//! 1. an [`Algorithm`] enum — the real squaring algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Uint<N>` compiles to a
//! direct call to the half-product squaring kernel, no runtime branch.
//!
//! # Algorithm
//!
//! The dedicated squaring kernel [`crate::int::algos::limbs::sqr_low_fixed`]
//! already exists: it exploits symmetry to form each cross term once and
//! double it, halving the limb-multiply count relative to a general
//! `N×N` multiply. `sqr_via_mul` is a thin wrapper that routes through
//! `wrapping_sqr` so the seam is one consistent place.
//!
//! The `ByValue` arm of [`Select`] is present for canonical-shape
//! uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` is **not** `const fn`. `Uint<N>::wrapping_sqr` is not `const fn`.
//! `Int<N>::wrapping_sqr` IS `const fn` and `Int<N>::sqr` delegates there
//! directly rather than through this dispatcher. The `ByValue` arm returns
//! the default algorithm tag without invoking the fn pointer.

use crate::int::types::Uint;

// ── 1. the real squaring algorithm — NAMED, no `Default` ─────────────

/// The squaring algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `sqr_` function
/// prefix (`sqr_via_mul` → `ViaMul`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`sqr_via_mul`] — half-product squaring via
    /// [`crate::int::algos::limbs::sqr_low_fixed`], routed through
    /// [`Uint::wrapping_sqr`]. Forms each cross term once and doubles it:
    /// `N(N+1)/2` limb-multiplies rather than `N²`. Result is `self²`
    /// modulo `2^BITS`.
    ViaMul,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The sqr picker always
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

/// Pick the squaring algorithm for storage limb count `N`. Total over the
/// key; the half-product squaring kernel is width-independent so `ViaMul`
/// wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::ViaMul)
}

// ── algorithm fn: thin delegation to wrapping_sqr ────────────────────

/// Half-product integer square for `Uint<N>`.
///
/// Delegates to [`Uint::wrapping_sqr`] which routes to
/// [`crate::int::algos::limbs::sqr_low_fixed`]: each cross term is formed
/// once and doubled, so the limb-multiply count is `N(N+1)/2` rather than
/// the `N²` of a general multiply. Result is `x²` modulo `2^BITS`.
#[inline]
pub(crate) fn sqr_via_mul<const N: usize>(x: Uint<N>) -> Uint<N> {
    x.wrapping_sqr()
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Integer squaring dispatcher for `Uint<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Not `const fn`: `Uint<N>::wrapping_sqr` is not `const fn`. `Int<N>::sqr`
/// delegates to `Int<N>::wrapping_sqr` (which IS `const fn`) directly rather
/// than through this dispatcher.
#[inline]
pub(crate) fn dispatch<const N: usize>(x: Uint<N>) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&x),
    };
    match algo {
        Algorithm::ViaMul => sqr_via_mul(x),
    }
}
