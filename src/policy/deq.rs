// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Decimal equality policy — the per-`(N, M, S1, S2)` algorithm matcher
//! for `PartialEq<D<Int<M>, S2>> for D<Int<N>, S1>`.
//!
//! See `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! `D<Int<N>, S1>::eq` delegates to [`deq_dispatch`], which follows the
//! canonical policy shape:
//!
//! 1. an [`Algorithm`] enum — the real equality algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (equality choice is fully determined by the const generics, so
//!    `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(S1, S2)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, S1, S2>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `(N, S1, S2)` compiles
//! to a direct call to one kernel, no runtime branch.
//!
//! # Why two algorithms
//!
//! Equality is derived from [`crate::policy::dcmp::dcmp_dispatch`]: two
//! values are equal when their comparison is `Equal`. When `S1 == S2` the
//! same-scale shortcut applies (plain cross-width integer compare); when
//! `S1 != S2` the cross-scale path is taken. This mirrors the two
//! `dcmp_dispatch` algorithms.
//!
//! # Naming collision avoidance
//!
//! The integer policy module `src/int/policy/` contains `eq` (and `cmp`)
//! policies for the `Int<N>` type. This file is `deq` (decimal eq) in
//! `src/policy/` to avoid any name collision with those int-tier modules.

use crate::int::types::Int;
use crate::policy::dcmp::dcmp_dispatch;

// ── 1. the real equality algorithms — NAMED, no `Default` ────────────

/// The equality algorithms this policy chooses between. Variants are the
/// CamelCase of each kernel name, strict 1:1 with the kernel functions.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `eq_same_scale` — plain cross-width integer equal. Selected when both
    /// operands share the same decimal scale (`S1 == S2`): the comparison
    /// collapses to a same-denominatgor integer equality on raw storage.
    SameScale,
    /// `eq_scaled_diff` — cross-width, cross-scale equality. Selected when
    /// `S1 != S2`: delegates to the comparison path, testing for `Equal`.
    ScaledDiff,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The equality picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(S1, S2)`, total over the key ───

/// Pick the equality algorithm for decimal scales `S1` and `S2` with
/// storage limb count `N`. Total over the key:
/// - `S1 == S2` → `SameScale` (plain cross-width int equal);
/// - `S1 != S2` → `ScaledDiff` (cross-scale compare-for-Equal).
const fn select<const N: usize, const S1: u32, const S2: u32>() -> Select<N> {
    if S1 == S2 {
        Select::ByAlgorithm(Algorithm::SameScale)
    } else {
        Select::ByAlgorithm(Algorithm::ScaledDiff)
    }
}

// ── algorithm kernels ─────────────────────────────────────────────────

/// Cross-width same-scale equality: plain integer equal on raw storage.
#[inline]
const fn eq_same_scale<const N: usize, const M: usize>(a: Int<N>, b: Int<M>) -> bool {
    // Same scale: values are equal iff storage is equal (after cross-width
    // sign-extension). Delegate to the cross-scale comparator keyed at
    // d=0 — or equivalently, call cmp_cross and test for Equal.
    use core::cmp::Ordering;
    matches!(a.cmp_cross(b), Ordering::Equal)
}

/// Cross-width cross-scale equality: test that the comparison is `Equal`.
#[inline]
const fn eq_scaled_diff<const N: usize, const M: usize, const S1: u32, const S2: u32>(
    a: Int<N>,
    b: Int<M>,
) -> bool {
    use core::cmp::Ordering;
    matches!(dcmp_dispatch::<N, M, S1, S2>(a, b), Ordering::Equal)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal equality dispatcher for storage `Int<N>` / `Int<M>`, scales
/// `S1` / `S2`. Const-folds to a direct call to one kernel per
/// monomorphisation.
///
/// `const fn`: both kernels are `const fn`, so the dispatcher is too.
#[inline]
#[must_use]
pub(crate) const fn deq_dispatch<
    const N: usize,
    const M: usize,
    const S1: u32,
    const S2: u32,
>(
    a: Int<N>,
    b: Int<M>,
) -> bool {
    let algo = match const { select::<N, S1, S2>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::SameScale,
    };
    match algo {
        Algorithm::SameScale => eq_same_scale::<N, M>(a, b),
        Algorithm::ScaledDiff => eq_scaled_diff::<N, M, S1, S2>(a, b),
    }
}
