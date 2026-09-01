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
//! # The algorithms and the width split
//!
//! Remainder keys purely on `N`:
//!
//! * **`N <= 2`** (`Int<1>` = D18, `Int<2>` = D38) → [`rem_native`]: the
//!   operand magnitude always fits a single `u128`, so a hardware
//!   `u128 % u128` with the dividend's sign re-applied is correct and far
//!   cheaper than routing through the runtime division dispatcher.
//!   Microbenched at the dispatch seam: native beats the via-div-rem path
//!   ~1.85x at `N == 1` and ~3.0x at `N == 2`.
//! * **`N >= 3`** → [`rem_small_fast`]: at the wide tiers the FULL-width
//!   magnitude exceeds `u128`, but the integer-remainder operands are
//!   frequently small (a bare integer, a scale-0 decimal). `rem_small_fast`
//!   takes the same hardware `u128 % u128` whenever BOTH magnitudes fit a
//!   single word — bypassing the `[u64; N]` quotient scratch and the
//!   `div_rem::dispatch` shape classifier — and falls back to the multi-limb
//!   divmod ([`rem_via_div_rem`]'s path) otherwise. This is a
//!   single-word fast path applied generically (one kernel, value-gated, valid
//!   at every `N`). Benched (`rem_kernel_ab`): `small_fast` beats the bare
//!   `via_div_rem` 1.1–2.6× on small operands (the scale-0 shape) at every
//!   wide tier and is at parity on full-width operands. The multi-limb
//!   divmod fallback IS the division policy's Knuth / Burnikel–Ziegler
//!   boundary; a remainder-only Knuth pass (skipping the quotient store) was
//!   microbenched and showed no win (the quotient store is negligible next
//!   to the multiply-subtract pass), so the fallback stays the full divmod.
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
use crate::int::algos::rem::rem_small_fast::rem_small_fast;
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
    /// [`rem_small_fast`] — the width-agnostic small-magnitude fast path:
    /// when both operand magnitudes fit a single `u128` it takes a hardware
    /// `u128 % u128` (no `[u64; N]` quotient scratch, no `div_rem::dispatch`
    /// shape classifier); otherwise it falls back to the same divmod
    /// [`Self::ViaDivRem`] takes. A single-word fast path applied
    /// generically. Routed for every wide tier (`N >= 3`): the integer
    /// remainder operands are frequently small (a bare integer, a scale-0
    /// decimal), where the full divmod setup dwarfs the divide. Benched
    /// (`rem_kernel_ab`): `small_fast` beats `via_div_rem` 1.1–2.6× on the
    /// small-operand (`s0_word` / `s0_u128`) cells at every width `N >= 3`,
    /// and is at parity on full-width operands (the fallback shares the
    /// magnitude conversion, so no extra sign-magnitude round trip is paid).
    SmallFast,
    /// [`rem_via_div_rem`] — derives the remainder by delegating to
    /// [`crate::int::policy::div_rem::dispatch`] and taking the remainder
    /// output. Reuses the division policy's Knuth / Burnikel–Ziegler engine
    /// selection; the division policy IS the optimization boundary for this
    /// operation. Registered but unrouted: [`Self::SmallFast`] wraps it with
    /// the small-operand fast path and wins or ties at every wide tier, so
    /// `select` routes `SmallFast` rather than this bare variant.
    /// `#[allow(dead_code)]` suppresses the unrouted-variant warning.
    #[allow(dead_code)]
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
/// the key: `Native` for `N <= 2` (the magnitude always fits a `u128`),
/// `SmallFast` for the wide tiers (a value-gated single-word fast path with
/// a multi-limb divmod fallback).
const fn select<const N: usize>() -> Select<N> {
    match N {
        // N <= 2: magnitude fits a single u128 — the hardware `%` path
        // beats routing through the division dispatcher (microbenched:
        // `native` beats `via_div_rem` ~1.85x at N=1 / ~3.0x at N=2).
        1 | 2 => Select::ByAlgorithm(Algorithm::Native),
        // N >= 3: the FULL-width magnitude exceeds u128, but the operands are
        // frequently small (a bare integer, a scale-0 decimal). `SmallFast`
        // takes a hardware `u128 % u128` whenever both magnitudes fit one
        // word and falls back to the same divmod otherwise — a
        // single-word fast path applied generically. Benched
        // (`rem_kernel_ab`): it beats the bare `via_div_rem` 1.1–2.6× on
        // small operands at every wide tier and ties on full-width operands,
        // so the wide band routes `SmallFast`. (A remainder-only Knuth pass
        // skipping the quotient store was separately microbenched and showed
        // no win — the quotient store is negligible vs the multiply-subtract
        // pass — so the fallback stays the full divmod.)
        _ => Select::ByAlgorithm(Algorithm::SmallFast),
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
pub(crate) fn dispatch<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        // rem is always ByAlgorithm; fall through to the default if
        // the arm is reached (fn pointer calls are not allowed in const fn,
        // but this outer fn is not const so reaching ByValue would be fine
        // at runtime — the arm is dead in practice).
        Select::ByValue(_) => Algorithm::SmallFast,
    };
    match algo {
        Algorithm::Native => rem_native(dividend, divisor),
        Algorithm::SmallFast => rem_small_fast(dividend, divisor),
        Algorithm::ViaDivRem => rem_via_div_rem(dividend, divisor),
        Algorithm::Schoolbook => rem_schoolbook(dividend, divisor),
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
        // (dividend, divisor, expected) — small enough to fit i128, hence representable
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
        for &(dividend, divisor, want) in cases {
            // N = 2 (native arm).
            let got2 = dispatch::<2>(Int::<2>::from_i128(dividend), Int::<2>::from_i128(divisor));
            assert_eq!(got2.to_i128(), want, "N=2 native rem({dividend}, {divisor})");
            // N = 3 (via-div-rem arm) — same value, wider storage.
            let got3 = dispatch::<3>(Int::<3>::from_i128(dividend), Int::<3>::from_i128(divisor));
            assert_eq!(got3.to_i128(), want, "N=3 via-div-rem rem({dividend}, {divisor})");
            assert_eq!(
                got2.to_i128(),
                got3.to_i128(),
                "boundary disagreement ({dividend}, {divisor})"
            );
        }
    }
}
