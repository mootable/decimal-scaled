// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Remainder policy — the per-width algorithm matcher for signed integer
//! remainder.
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
//! the `const { … }` block folds per monomorphisation and the unchosen arms
//! are dead-arm-eliminated in release: each concrete `Int<N>` routes to a
//! single algorithm with no runtime branch on the const path.
//!
//! # The two algorithms and the width split
//!
//! Remainder keys purely on `N`:
//!
//! * **`N <= 2`** (`Int<1>` = D18, `Int<2>` = D38) → [`rem_native`]: the
//!   operand magnitude fits a single `u128`, so a hardware `u128 % u128`
//!   with the dividend's sign re-applied is correct and far cheaper than
//!   routing through the runtime division dispatcher. Microbenched at the
//!   dispatch seam: native beats the via-div-rem path ~1.85x at `N == 1`
//!   and ~3.0x at `N == 2`.
//! * **`N >= 3`** → [`rem_via_div_rem`]: the magnitude exceeds `u128`, so
//!   the remainder is the second output of a multi-limb divmod. The
//!   division policy's Knuth / Burnikel–Ziegler engine selection IS the
//!   remainder optimisation boundary at these widths. A remainder-only
//!   Knuth pass (skipping the quotient store) was microbenched against the
//!   full divmod and showed no win at any wide tier — the quotient store is
//!   negligible next to the multiply-subtract pass — so the wide band stays
//!   on the full-divmod path.
//!
//! The `ByValue` arm of [`Select`] is present for canonical-shape
//! uniformity; `select` never returns it (the split is by `N` alone, not by
//! operand value).
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

use crate::int::algos::rem::rem_native::rem_native;
use crate::int::algos::rem::rem_schoolbook::rem_schoolbook;
use crate::int::algos::rem::rem_via_div_rem::rem_via_div_rem;
use crate::int::types::Int;

// ── 1. the real remainder algorithm — NAMED, no `Default` ────────────

/// The remainder algorithms this policy chooses between. The single variant
/// is the CamelCase of the algorithm fn's name minus the `rem_` function
/// prefix (`rem_via_div_rem` → `ViaDivRem`) — strict 1:1 with the
/// algorithm fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`rem_native`] — hardware `u128 % u128` on the operand magnitudes,
    /// with the dividend's sign re-applied. Valid only for `N <= 2` (the
    /// magnitude must fit a single `u128`); routed for `Int<1>` (D18) and
    /// `Int<2>` (D38), where bypassing the runtime division dispatcher (no
    /// shape classification, no `[u64; 288]` Knuth scratch, no quotient
    /// buffer) wins decisively over [`Self::ViaDivRem`].
    Native,
    /// [`rem_via_div_rem`] — derives the remainder by delegating to
    /// [`crate::int::policy::div_rem::dispatch`] and taking the remainder
    /// output. Reuses the division policy's Knuth / Burnikel–Ziegler engine
    /// selection; the division policy IS the optimization boundary for this
    /// operation.
    ViaDivRem,
    /// [`rem_schoolbook`] — binary shift-subtract long division remainder,
    /// the naive reference baseline. Registered but unrouted: `select`
    /// never returns this variant; it exists for unit-test reachability
    /// and future routing experiments. `#[allow(dead_code)]` suppresses
    /// the compiler warning.
    #[allow(dead_code)]
    Schoolbook,
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
    match N {
        // N <= 2: magnitude fits a single u128 — the hardware `%` path
        // beats routing through the division dispatcher (microbenched:
        // `native` beats `via_div_rem` ~1.85x at N=1 / ~3.0x at N=2).
        1 | 2 => Select::ByAlgorithm(Algorithm::Native),
        // N >= 3: the magnitude exceeds u128; the division policy's Knuth /
        // Burnikel–Ziegler engine is the remainder optimisation boundary.
        // A remainder-only Knuth pass was microbenched and showed no win
        // over the full divmod at any wide tier (the quotient store is
        // negligible vs the multiply-subtract pass), so the wide band stays
        // on `ViaDivRem`.
        _ => Select::ByAlgorithm(Algorithm::ViaDivRem),
    }
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
        Algorithm::Native => rem_native(a, b),
        Algorithm::ViaDivRem => rem_via_div_rem(a, b),
        Algorithm::Schoolbook => rem_schoolbook(a, b),
    }
}


#[cfg(test)]
mod tests {
    use super::dispatch;
    use crate::int::types::Int;

    /// At the native/via-div-rem matcher boundary (`N == 2` native vs
    /// `N == 3` via-div-rem) the signed remainder must be identical to the
    /// truncating reference for the same values. Truncating-toward-zero:
    /// the remainder carries the dividend's sign.
    #[test]
    fn dispatch_matches_truncating_reference_across_boundary() {
        // (a, b, expected) — small enough to fit i128, hence representable
        // at N=2 and N=3 alike. Covers all four sign combinations and 0.
        let cases: &[(i128, i128, i128)] = &[
            (100, 7, 2),
            (-100, 7, -2),
            (100, -7, 2),
            (-100, -7, -2),
            (0, 5, 0),
            (5, 5, 0),
            (i128::MAX, 3, i128::MAX % 3),
            (i128::MIN + 1, 3, (i128::MIN + 1) % 3),
        ];
        for &(a, b, want) in cases {
            // N = 2 (native arm).
            let got2 = dispatch::<2>(Int::<2>::from_i128(a), Int::<2>::from_i128(b));
            assert_eq!(got2.to_i128(), want, "N=2 native rem({a}, {b})");
            // N = 3 (via-div-rem arm) — same value, wider storage.
            let got3 = dispatch::<3>(Int::<3>::from_i128(a), Int::<3>::from_i128(b));
            assert_eq!(got3.to_i128(), want, "N=3 via-div-rem rem({a}, {b})");
            assert_eq!(got2.to_i128(), got3.to_i128(), "boundary disagreement ({a}, {b})");
        }
    }
}
