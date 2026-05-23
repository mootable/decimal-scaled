// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Remainder policy — the default-delegating algorithm matcher for signed
//! integer remainder.
//!
//! The `Rem` operator for `Int<N>` delegates to [`dispatch`], which follows
//! the canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
//!
//! 1. an [`Algorithm`] enum — the real remainder algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { … }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Int<N>` routes to the
//! `via_div_rem` algorithm, with no runtime branch on the const path.
//!
//! # Why there is only one algorithm
//!
//! Signed remainder is derived from division: the remainder is exactly the
//! second output of a divmod call, so the unique algorithm is to delegate
//! to the division policy and take the remainder half. There is no crossover
//! threshold that applies to remainder alone, and no value-dependent split
//! that the division policy does not already capture. The `ByValue` arm of
//! [`Select`] is present for canonical-shape uniformity; `select` never
//! returns it.
//!
//! # Why `dispatch` is NOT `const fn` (and `wrapping_rem` is)
//!
//! The single algorithm (`rem_via_div_rem`) delegates to
//! [`crate::int::policy::div_rem::dispatch`], which is NOT `const fn`
//! (the division dispatcher classifies divisor shapes at runtime — a
//! `ByValue` value-matcher whose fn pointer cannot be called in a `const`
//! context). Therefore `dispatch` here cannot be `const fn` either.
//!
//! `Int<N>::wrapping_rem` **is** `const fn` and preserves that property by
//! NOT routing through this non-const dispatcher: it calls
//! [`crate::int::algos::div::div_rem`] (the const single-algorithm fast path)
//! directly, exactly as it did before this policy file was introduced. Only
//! the selection SEAM (the `Algorithm`/`Select`/`select` shape) is shared;
//! `wrapping_rem` keeps its existing const-safe implementation. This design
//! matches the existing const-ness contract exactly — no const path is
//! removed, no non-const path gains const.
//!
//! # Routed primitive
//!
//! The `Rem for Int<N>` operator routes through `dispatch` (non-const).
//! `Int<N>::wrapping_rem` (const) routes directly through
//! [`crate::int::algos::div::div_rem`] and is not altered.

use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
use crate::int::types::Int;

// ── 1. the real remainder algorithm — NAMED, no `Default` ────────────

/// The remainder algorithms this policy chooses between. The single variant
/// is the CamelCase of the algorithm fn's name minus the `rem_` function
/// prefix (`rem_via_div_rem` → `ViaDivRem`) — strict 1:1 with the
/// algorithm fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`rem_via_div_rem`] — derives the remainder by delegating to
    /// [`crate::int::policy::div_rem::dispatch`] and taking the remainder
    /// output. Reuses the division policy's Knuth / Burnikel–Ziegler engine
    /// selection; the division policy IS the optimization boundary for this
    /// operation.
    ViaDivRem,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The rem picker always
/// returns `ByAlgorithm`: the choice is fully determined by `N` (which is
/// constant, and the same algorithm wins at every `N`). `ByValue` is part
/// of the canonical shape for uniformity across functions; `select` never
/// returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `N`, total over the key ──────────

/// Pick the remainder algorithm for storage limb count `N`. Total over
/// the key; remainder always delegates to the division policy, so
/// `ViaDivRem` wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::ViaDivRem)
}

// ── algorithm fn: delegation to the division policy ───────────────────

/// Signed remainder via the division policy for `Int<N>`. Strips the
/// operand signs, calls [`div_rem_dispatch`] on the unsigned magnitudes,
/// re-applies the dividend's sign to the remainder (truncating-toward-zero
/// semantics), and returns the signed result.
///
/// Delegates to [`crate::int::policy::div_rem::dispatch`] — the single
/// site the division optimization boundary lives at — rather than
/// reimplementing the Knuth / Burnikel–Ziegler engine selection inline.
/// `div_rem_dispatch` is NOT `const fn` (its value-matcher invokes a fn
/// pointer at runtime), so this algorithm fn is not `const fn` either.
/// The caller (`dispatch`) is correspondingly non-const.
///
/// Panics on a zero divisor, matching the `Rem` operator contract.
#[inline]
pub(crate) fn rem_via_div_rem<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    assert!(
        !b.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    let neg_r = a.is_negative();
    let mut quot = [0u64; N];
    let mut rem = [0u64; N];
    div_rem_dispatch(
        a.unsigned_abs().as_limbs(),
        b.unsigned_abs().as_limbs(),
        &mut quot,
        &mut rem,
    );
    Int::<N>::from_mag_limbs(&rem, neg_r)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Signed integer remainder dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// NOT `const fn`: the single algorithm delegates to
/// [`crate::int::policy::div_rem::dispatch`], which is a runtime-shape
/// value-matcher and therefore cannot be `const fn`. This matches the
/// existing non-const `Rem for Int<N>` operator path. `Int<N>::wrapping_rem`
/// (which IS `const fn`) does NOT route through this dispatcher; it calls the
/// const [`crate::int::algos::div::div_rem`] fast path directly, preserving
/// its existing const-ness. The `ByValue` arm returns the default algorithm
/// tag without invoking the fn pointer — no fn pointer call occurs in the
/// const-select block.
#[inline]
pub(crate) fn dispatch<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // rem is always ByAlgorithm; fall through to the default if
        // the arm is reached (fn pointer calls are not allowed in const fn,
        // but this outer fn is not const so reaching ByValue would be fine
        // at runtime — the arm is dead in practice).
        Select::ByValue(_) => Algorithm::ViaDivRem,
    };
    match algo {
        Algorithm::ViaDivRem => rem_via_div_rem(a, b),
    }
}
