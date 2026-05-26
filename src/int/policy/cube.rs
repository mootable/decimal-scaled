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
//! **Fused product-scanning candidate (`Comba`).**
//! [`crate::int::algos::cube::cube_fused_comba::cube_fused_comba`] forms `x³`
//! in a SINGLE product-scanning pass (the cube analogue of the symmetric comba
//! square) rather than materialising `x²` and re-multiplying. It is also
//! `const fn`, so it can be wired without de-const-ing the public API. The
//! `int_cube_eq_ab` N-way A/B shows it beats `x²·x` only at the narrow
//! `N == 2` tier (~1.11x, reproducible); at `N == 1` it ties, and from
//! `N >= 3` its `~N³/6` triple-product count loses to the schoolbook
//! `N(N+1)/2 + N²` and the gap GROWS with width (1.2x at N=3 → 13x at N=64).
//! `select` therefore routes `N == 2` to `Comba` and every other width to
//! `Schoolbook`.
//!
//! The `LimbSize` u128 packing used by [`crate::int::policy::sqr_low`] /
//! [`crate::int::policy::mul_low`] is NOT a cube candidate: those kernels are
//! **not `const fn`** (the `Limb` trait methods are not const), and `cube`'s
//! dispatch MUST stay `const fn` because `Int<N>::wrapping_cube` is `const fn`
//! and is reached from `const` contexts crate-wide.
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

use crate::int::algos::cube::cube_fused_comba::cube_fused_comba;
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
    /// modulo `2^BITS`. Wins at every width except `N == 2`.
    Schoolbook,
    /// [`cube_fused_comba`] — single fused product-scanning pass for `x³`
    /// (the cube analogue of the symmetric comba square). `const fn`. Wins at
    /// the narrow `N == 2` tier per the `int_cube_eq_ab` A/B; loses (and the
    /// gap grows with width) from `N >= 3`.
    Comba,
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

/// Pick the cubing algorithm for storage limb count `N`. Total over the key.
///
/// - `N == 2` → [`Algorithm::Comba`] (the fused product-scanning pass wins
///   ~1.11x at this narrow tier per the `int_cube_eq_ab` A/B).
/// - every other `N` (the `_` arm) → [`Algorithm::Schoolbook`] (`x²·x`; comba
///   ties at N=1 and loses by a widening margin from N>=3).
const fn select<const N: usize>() -> Select<N> {
    match N {
        2 => Select::ByAlgorithm(Algorithm::Comba),
        _ => Select::ByAlgorithm(Algorithm::Schoolbook),
    }
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
        Algorithm::Comba => cube_fused_comba(x),
    }
}
