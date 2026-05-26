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
use crate::int::types::compute_int::ComputeInt;
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
    /// [`cbrt::cbrt_native_fast_d57::cbrt_native_fast_a`] — `f64`-seeded
    /// Newton run directly in a tight, concrete `Int<W>` (the work width `W`
    /// is chosen per `(N, SCALE)` cell in the dispatch arm to just cover
    /// `mag · 10^(2·SCALE)`), rather than through the width-agnostic int
    /// `icbrt` policy, whose build-max scratch buffer churn dominated the
    /// small mid-scale radicands. The seed is the 0.4.4 **full-radicand**
    /// f64 cbrt seed (commit routing the Native cells to `cbrt_native_fast_a`):
    /// a tight seed that cuts the Newton divide count, vs the earlier shipped
    /// top-64-bits seed which over-shot ∛n by ~2.5× and regressed cbrt@D57/D76.
    /// Routed cells: `(D57,20)`, `(D76,35)`, `(D115,57)`, `(D153,75)`,
    /// `(D230,115)`, `(D307,150)`. Microbench (`root_kernel_ab`): the
    /// PRE-change top-bits-seed Native was 1.1–2.0× faster than the generic
    /// slice [`Self::Newton`]; the full-radicand-seed win margin is to be
    /// re-confirmed by a fresh `root_kernel_ab` N-way compare on a quiet
    /// machine. Bit-identical to [`Self::Newton`] across all six modes (the
    /// rounding tail is shared); the seed falls back to the top-bits path
    /// past the f64 range.
    ///
    /// Gated with the kernel: each routed `(N, SCALE)` cell only exists
    /// when its tier is compiled in, so the variant, its `select` arms,
    /// and its dispatch arms are gated together (the policy stays
    /// exhaustive in both configs — see `docs/ARCHITECTURE.md`
    /// "Feature-flagging a variation").
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
        // Mid-wide tiers (D57/D76/D115/D153, N = 3/4/6/8) — bespoke
        // f64-seeded Newton in a tight, concrete `Int<W>` with `W = 3N`
        // (covering `mag · 10^(2·SCALE)`: the magnitude is ≤ 64N bits and
        // `10^(2·SCALE)` adds ≤ 128N more for SCALE ≤ the tier's digit
        // capacity, so 192N bits = 3N limbs always suffice). Routed by `N`
        // (all scales) because the build-max slice scratch the generic
        // `Newton` kernel zeroes per Newton iteration dominates these
        // mid-scale radicands. Microbench (`root_kernel_ab`, bbc scales):
        // native beats the generic slice 3.95× (D57<20>, via `fast_a`),
        // 1.92× (D76<20>), 1.32× (D115<25>), 1.12× (D153<25>); at the tier
        // max scale (D76<76>, W=3N) native still beats slice 2.06×. For
        // N ≥ 12 (D230/D307…) the slice is at/ahead at the bbc scales, so
        // those tiers keep their existing high-scale `Native` arms below and
        // otherwise fall to the generic `Newton` `_` arm.
        //
        // Gated with the kernel; each tier's cells are unreachable when the
        // tier isn't built (the policy stays exhaustive in both configs).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, _) // D57,  W=9
        | (4, _) // D76,  W=12
        | (6, _) // D115, W=18
        | (8, _) // D153, W=24
        | (12, 115) // D230<115>, W=25 (kept high-scale cell)
        | (16, 150) // D307<150>, W=32 (kept high-scale cell)
        => Select::ByAlgorithm(Algorithm::Native),
        // Everything else (N ≥ 12 wide tiers, other scales) — generic Newton
        // over the int layer's width-agnostic slice `icbrt`.
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
    Int<N>: ComputeInt,
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
        // Native tight-`Int<W>` arm: pick the literal work width `W` for
        // this `(N, SCALE)` cell, then run Newton directly in `Int<W>`.
        // The `(N, SCALE)` match is const-foldable (both are const
        // generics), so each monomorphisation keeps exactly one arm and
        // the rest are dead-arm-eliminated in release. The `_ => Newton`
        // fallback never fires for a cell `select` routed to `Native`.
        #[cfg(any(feature = "d57", feature = "wide"))]
        // Native cells use the 0.4.4-style full-radicand f64 cbrt seed
        // (`cbrt_native_fast_a`): a tight seed (vs the shipped top-64-bits
        // seed that over-shoots ∛n by ~2.5×) cuts the Newton divide count,
        // recovering the cbrt_D57/D76 regression. Bit-identical (the rounding
        // tail is shared); falls back to the top-bits seed past the f64 range.
        Algorithm::Native => match (N, SCALE) {
            // Mid-wide tiers routed by N (all scales), W = 3N.
            (3, _) => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 9>(raw, const { Int::<9>::TEN.pow(2 * SCALE) }, mode),
            (4, _) => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 12>(raw, const { Int::<12>::TEN.pow(2 * SCALE) }, mode),
            (6, _) => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 18>(raw, const { Int::<18>::TEN.pow(2 * SCALE) }, mode),
            (8, _) => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 24>(raw, const { Int::<24>::TEN.pow(2 * SCALE) }, mode),
            // Kept high-scale cells for the N ≥ 12 tiers.
            (12, 115) => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 25>(raw, const { Int::<25>::TEN.pow(2 * SCALE) }, mode),
            (16, 150) => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 32>(raw, const { Int::<32>::TEN.pow(2 * SCALE) }, mode),
            _ => cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode),
        },
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
