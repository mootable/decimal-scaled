// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Decimal comparison policy — the per-`(N, M, S1, S2)` algorithm matcher
//! for `PartialOrd<D<Int<M>, S2>> for D<Int<N>, S1>` and `Ord for
//! D<Int<N>, S>`.
//!
//! See `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! `D<Int<N>, S1>` comparison delegates to [`dcmp_dispatch`], which
//! follows the canonical policy shape:
//!
//! 1. an [`Algorithm`] enum — the real comparison algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (comparison choice is fully determined by the const generics, so
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
//! When `S1 == S2` the operands share the same scale, so no cross-scale
//! multiply is needed — the comparison is a plain cross-width integer
//! compare (`cmp_cross_same_scale`). When `S1 != S2` the logical values
//! `a/10^S1` and `b/10^S2` have different denominators; the higher-scale
//! (more decimal digits) operand is compared against the lower-scale
//! operand scaled by `10^|S1−S2|` via the `Int` cross-scale comparator
//! (`cmp_cross_scaled_diff`). The `S1 == S2` branch is const-foldable
//! and collapses to a plain int compare in the common same-scale case.
//!
//! # Naming collision avoidance
//!
//! The integer policy module `src/int/policy/` contains `cmp` and `eq`
//! policies for the `Int<N>` type. This file is `dcmp` (decimal cmp) in
//! `src/policy/` to avoid any name collision with those int-tier modules.

use crate::int::types::Int;

// ── 1. the real comparison algorithms — NAMED, no `Default` ──────────

/// The comparison algorithms this policy chooses between. Variants are the
/// CamelCase of each kernel name minus the `cmp_cross_` prefix, strict 1:1
/// with the kernel functions.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `cmp_cross_same_scale` — plain cross-width integer compare. Selected
    /// when both operands share the same decimal scale (`S1 == S2`): the
    /// scale denominators cancel and the comparison is a simple integer
    /// order on the raw storage values.
    SameScale,
    /// `cmp_cross_scaled_diff` — cross-width, cross-scale compare. Selected
    /// when `S1 != S2`: the higher-scale operand is compared against the
    /// lower-scale operand multiplied by `10^|S1−S2|`, keeping the
    /// comparison overflow-free by scaling the larger value DOWN rather
    /// than the smaller one up.
    ScaledDiff,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The comparison picker always
/// returns `ByAlgorithm`: the choice is fully determined by the const scales.
/// `ByValue` is part of the canonical shape for uniformity; `select` never
/// returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(S1, S2)`, total over the key ───

/// Pick the comparison algorithm for decimal scales `S1` and `S2` with
/// storage limb count `N`. Total over the key:
/// - `S1 == S2` → `SameScale` (plain cross-width int compare);
/// - `S1 != S2` → `ScaledDiff` (cross-scale multiply-compare).
///
/// The `S1 == S2` branch const-folds away in the common same-scale case —
/// the monomorphisation sees only `SameScale` and the dead `ScaledDiff` arm
/// is eliminated in release.
const fn select<const N: usize, const S1: u32, const S2: u32>() -> Select<N> {
    if S1 == S2 {
        Select::ByAlgorithm(Algorithm::SameScale)
    } else {
        Select::ByAlgorithm(Algorithm::ScaledDiff)
    }
}

// ── algorithm kernels ─────────────────────────────────────────────────

/// Cross-width same-scale comparison: delegates to `Int::cmp_cross`.
#[inline]
const fn cmp_cross_same_scale<const N: usize, const M: usize>(
    lhs: Int<N>,
    rhs: Int<M>,
) -> core::cmp::Ordering {
    lhs.cmp_cross(rhs)
}

/// Cross-width cross-scale comparison. `lhs` has scale `S1`, `rhs` has `S2`.
/// Logical values are `lhs/10^S1` and `rhs/10^S2`. Scales to the lower
/// operand's denominator (dividing the higher-scale storage) so the comparison
/// is overflow-free and uses only `Int::cmp_cross_scaled`.
#[inline]
const fn cmp_cross_scaled_diff<const N: usize, const M: usize, const S1: u32, const S2: u32>(
    lhs: Int<N>,
    rhs: Int<M>,
) -> core::cmp::Ordering {
    // S1 > S2: compare lhs vs rhs·10^(S1−S2) — equivalently, scale DOWN
    // `lhs`'s counterpart: `cmp_cross_scaled(lhs, rhs, S1−S2)` means `lhs` vs
    // `rhs` scaled up by `10^d`, matching `lhs/10^S1` vs `rhs/10^S2` after
    // ×10^S1.
    if S1 > S2 {
        lhs.cmp_cross_scaled(rhs, S1 - S2)
    } else {
        // S2 > S1: compare rhs vs lhs·10^(S2−S1) → reverse.
        rhs.cmp_cross_scaled(lhs, S2 - S1).reverse()
    }
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal comparison dispatcher for storage `Int<N>` / `Int<M>`, scales
/// `S1` / `S2`. Const-folds to a direct call to one kernel per
/// monomorphisation.
///
/// `const fn`: both kernels are `const fn` (they delegate to `Int`'s const
/// comparison primitives), so the dispatcher is also `const fn`.
#[inline]
#[must_use]
pub(crate) const fn dcmp_dispatch<
    const N: usize,
    const M: usize,
    const S1: u32,
    const S2: u32,
>(
    lhs: Int<N>,
    rhs: Int<M>,
) -> core::cmp::Ordering {
    let algo = match const { select::<N, S1, S2>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(_) => Algorithm::SameScale,
    };
    match algo {
        Algorithm::SameScale => cmp_cross_same_scale::<N, M>(lhs, rhs),
        Algorithm::ScaledDiff => cmp_cross_scaled_diff::<N, M, S1, S2>(lhs, rhs),
    }
}
