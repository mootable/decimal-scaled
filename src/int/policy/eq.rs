// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Equality policy — the default-delegating algorithm matcher for the signed
//! equality test of `Int<N>`.
//!
//! The const equality primitive (called by `PartialEq::eq`) delegates to
//! [`dispatch`], which follows the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real equality algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`, the
//! `const { … }` block folds per monomorphisation, and the FINAL compiled
//! code carries no runtime branch: LLVM discards every arm the constant
//! discriminant doesn't select, so each concrete `Int<N>` still compiles to
//! a direct call to the limbwise equality kernel. What does NOT fold away is
//! the compile-time mono-item cost: `match algo` switches on an enum
//! discriminant, which rustc's mono collector cannot prune the way it prunes
//! a `match N` on the const generic itself — every arm's kernel is
//! instantiated (MIR generated, `required_consts` evaluated) for every `N`
//! this policy is monomorphised at, in debug AND release
//! ([[reference_mono_collector_prunes_int_switch_not_enum]]). One extra
//! unrouted arm is a bounded, accepted cost here (this is the ALREADY-SHIPPED
//! shape — `Schoolbook` has paid it since before `XorFold` existed); it would
//! need revisiting only if this policy's item count itself became a measured
//! problem, the same way `ln::tang_at_rung` did.
//!
//! # Why there is only one ROUTED algorithm
//!
//! Integer equality is width-independent: a limb-by-limb comparison over the
//! `N`-limb two's-complement representation (the limbwise approach in
//! [`eq_limbwise`]) is the unique correct choice at every limb count `N`.
//! There is no crossover threshold, no value-dependent split, and no
//! work-width widening — so `select` always returns [`Algorithm::Limbwise`]
//! and never reaches [`Algorithm::XorFold`], the registered-but-unrouted
//! candidate kept per `docs/ARCHITECTURE.md` → "Keeping the alternatives"
//! until a benchmark pass decides whether it wins. The `ByValue` arm of
//! [`Select`] is present for canonical-shape uniformity; `select` never
//! returns it either.
//!
//! # Why `dispatch` is `const fn`
//!
//! `PartialEq::eq` for `Int<N>` (via `cmp_cross`) is `const fn`-compatible
//! and equality is tested in const contexts. `dispatch` therefore must also
//! be `const fn`. A pure-`ByAlgorithm` dispatch satisfies this: the
//! `ByValue` arm returns the default algorithm tag without invoking the fn
//! pointer (calling a fn pointer is not permitted in `const fn`; merely
//! matching the variant is fine).
//!
//! # Routed primitive
//!
//! The routed primitive is same-width two's-complement limb equality. The
//! `PartialEq<Int<M>> for Int<N>` impl calls `cmp_cross` (which handles the
//! cross-width general case); this policy provides the same-width seam so
//! the algorithm choice is recorded in one place and the dispatch is
//! separately microbenchable. `PartialEq` observable behaviour is unchanged.

use crate::int::algos::eq::eq_limbwise::eq_limbwise;
use crate::int::algos::eq::eq_xor_fold::eq_xor_fold;
use crate::int::types::Int;

// ── 1. the real equality algorithm — NAMED, no `Default` ─────────────

/// The equality algorithms this policy chooses between. Variants are the
/// CamelCase of each kernel fn's name minus the `eq_` function prefix
/// (`eq_limbwise` → `Limbwise`, `eq_xor_fold` → `XorFold`) — strict 1:1
/// with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`eq_limbwise`] — limb-by-limb two's-complement equality test,
    /// delegating to
    /// [`cmp_fixed`](crate::int::algos::support::limbs::cmp_fixed)
    /// (reusing the comparison kernel's
    /// equal-length path). Returns `true` iff all `N` limbs match.
    Limbwise,
    /// Schoolbook reference tag — delegates to the same [`eq_limbwise`]
    /// kernel. Limb-by-limb equality IS the schoolbook equality algorithm;
    /// this variant exists as an explicit, benchmarkable seam.
    #[allow(dead_code)]
    Schoolbook,
    /// [`eq_xor_fold`] — a genuinely distinct kernel: an OR-fold of
    /// per-limb XORs rather than a reuse of the comparison kernel. Bit-
    /// identical to [`eq_limbwise`] for every input (`docs/ARCHITECTURE.md`
    /// → "Keeping the alternatives"). Registered but **unrouted**:
    /// `select` never returns it. Kept per `int_cube_eq_ab`'s bench seam
    /// (`src/lib.rs`) until a measurement pass decides whether it wins.
    #[allow(dead_code)]
    XorFold,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The eq picker always
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

/// Pick the equality algorithm for storage limb count `N`. Total over the
/// key; equality is width-independent so `Limbwise` wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::Limbwise)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Integer equality dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: the equality primitive is `const fn`-compatible and
/// is called from `const` contexts across the crate. The `ByValue` arm
/// returns the default algorithm tag without invoking the fn pointer,
/// satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(lhs: Int<N>, rhs: Int<N>) -> bool {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        // eq is always ByAlgorithm; fall through to the default if
        // the arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::Limbwise,
    };
    match algo {
        Algorithm::Limbwise => eq_limbwise(lhs, rhs),
        Algorithm::Schoolbook => eq_limbwise(lhs, rhs),
        Algorithm::XorFold => eq_xor_fold(lhs, rhs),
    }
}
