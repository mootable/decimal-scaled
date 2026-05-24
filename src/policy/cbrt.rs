//! Cube-root policy — the per-`(N, SCALE)` algorithm matcher.
//!
//! `D<Int<N>, SCALE>::cbrt_strict_with(mode)` delegates directly to the
//! one shared [`dispatch`] generic function. `dispatch` follows the
//! canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"), mirroring [`crate::policy::sqrt`]:
//!
//! 1. an [`Algorithm`] enum — the real cube-root algorithms, no
//!    `Default` variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (cbrt has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block,
//!    then an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Work width
//!
//! The `Newton` kernel forms the radicand `|raw| · 10^(2·SCALE)`, which
//! spans up to `4N` limbs. Rather than thread a work *type* `Int<4N>`
//! (unnameable from `N` on stable), `cbrt_newton` does that arithmetic
//! directly in limbs and calls the int layer's width-agnostic slice
//! `icbrt`. So the dispatch carries no work-width parameter and the policy
//! stays a pure `(N, SCALE)` matcher.

use crate::algos::cbrt;
use crate::int::types::traits::BigInt;
use crate::int::types::work_scratch::WorkScratch;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real cube-root algorithms — NAMED, no `Default` ────────────

/// The cube-root algorithms this policy chooses between. Variants are
/// the CamelCase of each kernel's name minus the `cbrt_` function prefix
/// (`cbrt_newton` → `Newton`, …) — strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`cbrt::cbrt_newton::cbrt_newton`] — Newton `icbrt` over a work
    /// width `W` covering `|raw| · 10^(2·SCALE)`. The generic default.
    Newton,
    /// [`cbrt::cbrt_mg_divide::cbrt_mg_divide`] — hand-tuned 384-bit
    /// cube root for the `Int<2>` storage (D38, and D18 widened to it).
    MgDivide,
    /// [`cbrt::cbrt_native::cbrt_native`] — `f64`-seeded `Int<6>`
    /// Newton bespoke for the `(D57, 20)` cell. Runs Newton directly in
    /// tight `Int<6>` values (two iterations from the `f64::cbrt` seed)
    /// rather than through the width-agnostic int `icbrt` policy, whose
    /// build-max scratch buffer churn dominated this small radicand.
    /// Microbench: ~2× faster than [`Self::NewtonWithTableSeed`] at
    /// `(D57, 20)`. Bit-identical to [`Self::Newton`] across all six
    /// modes.
    ///
    /// Gated with the kernel: the `(D57, 20)` cell only exists when D57
    /// is compiled in, so the variant, its `select` arm, and its
    /// dispatch arm are gated together (the policy stays exhaustive in
    /// both configs — see `docs/ARCHITECTURE.md` "Feature-flagging a
    /// variation").
    #[cfg(any(feature = "d57", feature = "wide"))]
    Native,
    /// [`cbrt::cbrt_newton_with_table_seed::cbrt_newton_with_table_seed`]
    /// — the prior `Int<6>` + int-`icbrt` arm for `(D57, 20)`. Superseded
    /// by [`Self::Native`] (the int-`icbrt` scratch churn made it ~2×
    /// slower); kept as an explicit benchmarkable reference seam, never
    /// selected by `select`.
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[allow(dead_code)]
    NewtonWithTableSeed,
    /// Schoolbook reference tag -- delegates to
    /// [`cbrt::cbrt_schoolbook::cbrt_schoolbook`], which uses the same
    /// `W::icbrt`-based pipeline as `cbrt_newton`. Exists as an explicit
    /// benchmarkable seam; never selected by `select` in production.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". `ByValue` is part of the
/// canonical shape for uniformity across functions; cbrt never returns it
/// (the choice is fully determined by `(N, SCALE)`).
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the cube-root algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; the `_` arm is the generic `Newton`
/// default (a real algorithm — there is no synthetic default variant).
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // D18 (`Int<1>`) — widened to `Int<2>` storage in the dispatch
        // and run through the hand-tuned 384-bit cube root there.
        (1, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // D38 (`Int<2>`) — hand-tuned 384-bit cube root.
        (2, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // (D57, SCALE == 20) — bespoke narrow-work + f64 seed. Gated
        // with the kernel; falls to `Newton` when D57 is not compiled in
        // (in which case the `(3, 20)` cell is unreachable anyway).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 20) => Select::ByAlgorithm(Algorithm::Native),
        // Everything else (all wide tiers, all other scales) — generic
        // Newton over the tier's work width.
        _ => Select::ByAlgorithm(Algorithm::Newton),
    }
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Shared cube-root dispatch for storage `Int<N>`, decimal `SCALE`, and
/// Newton work width `W`. Zero input returns `Int::<N>::ZERO`; the sign
/// of a non-zero input is preserved.
///
/// `W` is the double-bumped work width for the `Newton` arm
/// (`Int<4N>`), supplied by the caller because `Int<4N>` is not
/// computable from `N` on stable. The `MgDivide` / `NewtonWithTableSeed`
/// arms run at their own fixed widths (`Int<2>` / `Int<3>`); the
/// `resize_to` bridges are no-ops at the `N` those arms are selected for
/// and dead-arm-eliminated at every other `N`.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N>
where
    Int<N>: WorkScratch,
{
    if raw == Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&raw),
    };
    match algo {
        Algorithm::Newton => cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode),
        // D18 / D38: run on `Int<2>` storage, resize back to `Int<N>`.
        // (`resize_to` is identity at N==2 and a lossless widen at N==1.)
        Algorithm::MgDivide => {
            cbrt::cbrt_mg_divide::cbrt_mg_divide(raw.resize_to::<Int<2>>(), SCALE, mode)
                .resize_to::<Int<N>>()
        }
        // (D57, 20): the bespoke kernels work on `Int<3>` storage.
        #[cfg(any(feature = "d57", feature = "wide"))]
        Algorithm::Native => {
            cbrt::cbrt_native::cbrt_native(raw.resize_to::<Int<3>>(), mode)
                .resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d57", feature = "wide"))]
        Algorithm::NewtonWithTableSeed => {
            cbrt::cbrt_newton_with_table_seed::cbrt_newton_with_table_seed(
                raw.resize_to::<Int<3>>(),
                mode,
            )
            .resize_to::<Int<N>>()
        }
        Algorithm::Schoolbook => cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode),
    }
}
