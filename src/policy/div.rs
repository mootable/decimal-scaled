//! Div policy — the per-`(N, SCALE)` algorithm matcher for decimal
//! division.
//!
//! `D<Int<N>, SCALE>::div_with` delegates directly to the one shared
//! [`dispatch`] generic function, mirroring [`crate::policy::mul`] and the
//! canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
//!
//! 1. an [`Algorithm`] enum — the real division algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (div has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release.
//!
//! # Work width — expanded in limbs, no `Int<2N>` type
//!
//! Decimal division scales the numerator by `10^SCALE` (`a * 10^SCALE`),
//! which spans up to `2N` limbs, before dividing by `b`. Rather than thread
//! a work *type* `Int<2N>` (unnameable from `N` on stable), the
//! [`WidenScale`](Algorithm::WidenScale) kernel forms the scaled numerator
//! directly in a `WorkScratch` limb buffer and divides via the int layer's
//! width-agnostic `div_rem`. So `dispatch` carries no work-width parameter;
//! it adds only `where Int<N>: WorkScratch` for the scratch buffer.
//!
//! The `10^SCALE` multiplier is evaluated here via `Int::<N>::TEN.pow(SCALE)`
//! (folds at compile time per `(N, SCALE)`) and threaded into the kernel.

use crate::int::types::work_scratch::WorkScratch;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real division algorithms — NAMED, no `Default` ─────────────

/// The division algorithms this policy chooses between. Variants are the
/// CamelCase of each kernel fn's name minus the `div_` prefix
/// (`div_widen_scale` → `WidenScale`, `div_schoolbook` → `Schoolbook`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`crate::algos::div::div_widen_scale::div_widen_scale`] — forms
    /// `a * 10^SCALE` in a `2N`-limb scratch buffer, divides by `b` via the
    /// int layer's `div_rem`, rounds, and rebuilds `Int<N>`. The generic
    /// default at every `(N, SCALE)`.
    WidenScale,
    /// [`crate::algos::div::div_schoolbook::div_schoolbook`] — the
    /// unambiguous schoolbook reference (same int-layer divide; decimal
    /// division has no MG / Newton arm to drop). Unrouted by `select`; a
    /// real benchmarkable seam, used directly in tests.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The div picker always
/// returns `ByAlgorithm`; `ByValue` is part of the canonical shape for
/// uniformity and `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the division algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `WidenScale` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    Select::ByAlgorithm(Algorithm::WidenScale)
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Decimal division dispatch for storage `Int<N>` and decimal `SCALE`.
///
/// The `const { select }` block folds away at every concrete `N`, leaving a
/// direct call to the chosen kernel. The `2N`-wide scaled numerator lives
/// in the kernel's `WorkScratch` scratch buffer, so no work-width type is
/// named here.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Int<N>: WorkScratch,
{
    // 10^SCALE in Int<N> storage; folds at compile time per (N, SCALE).
    let mult: Int<N> = <Int<N>>::TEN.pow(SCALE);
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a, &b),
    };
    match algo {
        Algorithm::WidenScale => {
            crate::algos::div::div_widen_scale::div_widen_scale::<N>(a, b, mult, mode)
        }
        Algorithm::Schoolbook => {
            crate::algos::div::div_schoolbook::div_schoolbook::<N>(a, b, mult, mode)
        }
    }
}
