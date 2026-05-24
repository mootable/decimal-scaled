//! Neg policy — the per-`(N, SCALE)` algorithm matcher for decimal
//! negation.
//!
//! `D<Int<N>, SCALE>` negation delegates to [`NegPolicy::neg_impl`],
//! which forwards to the one shared [`dispatch`] function. `dispatch`
//! follows the canonical policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"), mirroring [`crate::policy::add`]:
//!
//! 1. an [`Algorithm`] enum — the real negation algorithm, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (neg has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Why there is only one algorithm
//!
//! Decimal negation requires no rescaling: the sign of the stored integer
//! flips and the scale is unchanged. The single algorithm
//! (`neg_int_layer`) delegates to `Int<N>`'s `checked_neg` / `wrapping_neg`
//! following Rust's standard integer-overflow contract (debug panics on
//! `-MIN`, release wraps). There is no crossover threshold, no work-width
//! widening, and no value-dependent split. `ByValue` is present for
//! canonical-shape uniformity; `select` never returns it.

use crate::int::types::Int;

// ── 1. the real negation algorithm — NAMED, no `Default` ──────────────

/// The negation algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `neg_` function
/// prefix (`neg_int_layer` → `IntLayer`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`neg_int_layer`](crate::algos::neg::neg_int_layer::neg_int_layer) —
    /// delegates directly to `Int<N>`'s checked/wrapping
    /// neg, applying Rust's standard integer-overflow contract at the decimal
    /// layer. Sign flip requires no rescaling.
    IntLayer,
    /// Naive schoolbook reference: delegates to the same
    /// [`neg_int_layer`](crate::algos::neg::neg_int_layer::neg_int_layer)
    /// kernel. This variant documents the seam and stays unrouted by `select`.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The neg picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the negation algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `IntLayer` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    Select::ByAlgorithm(Algorithm::IntLayer)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal negation dispatcher for storage `Int<N>` and decimal `SCALE`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N, SCALE>() }` (folds per monomorphisation; dead arms
/// are eliminated in release) then dispatches exhaustively over
/// [`Algorithm`].
///
/// Not `const fn`: `neg_int_layer` branches on `cfg!(debug_assertions)`,
/// which is not permitted in `const fn`.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(a: Int<N>) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::IntLayer,
    };
    match algo {
        Algorithm::IntLayer | Algorithm::Schoolbook => {
            crate::algos::neg::neg_int_layer::neg_int_layer(a)
        }
    }
}

// ── per-type `NegPolicy` trait ────────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for unary `-`.
pub(crate) trait NegPolicy: Sized {
    /// Negate `self`, applying Rust's standard integer-overflow contract
    /// (panic in debug, wrap in release).
    fn neg_impl(self) -> Self;
}

impl<const N: usize, const SCALE: u32> NegPolicy for crate::D<Int<N>, SCALE> {
    #[inline]
    fn neg_impl(self) -> Self {
        Self(dispatch::<N, SCALE>(self.0))
    }
}
