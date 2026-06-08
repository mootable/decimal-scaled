//! Sub policy — the per-`(N, SCALE)` algorithm matcher for decimal
//! subtraction.
//!
//! `D<Int<N>, SCALE>` subtraction delegates to [`SubPolicy::sub_impl`],
//! which forwards to the one shared [`dispatch`] function. `dispatch`
//! follows the canonical policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"), mirroring [`crate::policy::add`]:
//!
//! 1. an [`Algorithm`] enum — the real subtraction algorithm, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (sub has no value split, so `ByValue` is never returned);
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
//! Decimal subtraction, like addition, requires no rescaling when both
//! operands share the same `SCALE`. The single algorithm (`sub_int_layer`)
//! delegates to `Int<N>`'s `checked_sub` and panics on overflow in both
//! debug and release (the default operator never silently wraps a wrong
//! number). There is no crossover threshold, no work-width widening, and no
//! value-dependent split. `ByValue` is present for canonical-shape
//! uniformity; `select` never returns it.

use crate::int::types::Int;

// ── 1. the real subtraction algorithm — NAMED, no `Default` ───────────

/// The subtraction algorithms this policy chooses between. The single
/// variant is the CamelCase of the kernel fn's name minus the `sub_`
/// function prefix (`sub_int_layer` → `IntLayer`) — strict 1:1 with the
/// kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`sub_int_layer`](crate::algos::sub::sub_int_layer::sub_int_layer) —
    /// delegates directly to `Int<N>`'s checked/wrapping
    /// sub, applying Rust's standard integer-overflow contract at the decimal
    /// layer. Same-SCALE subtraction needs no rescaling.
    IntLayer,
    /// Naive schoolbook reference: delegates to the same
    /// [`sub_int_layer`](crate::algos::sub::sub_int_layer::sub_int_layer)
    /// kernel. This variant documents the seam and stays unrouted by `select`.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The sub picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the subtraction algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `IntLayer` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    Select::ByAlgorithm(Algorithm::IntLayer)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal subtraction dispatcher for storage `Int<N>` and decimal `SCALE`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N, SCALE>() }` (folds per monomorphisation; dead arms
/// are eliminated in release) then dispatches exhaustively over
/// [`Algorithm`].
///
/// Not `const fn`: matches the existing non-`const` `Sub` operator on
/// `D<Int<N>, SCALE>`.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(a: Int<N>, b: Int<N>) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::IntLayer,
    };
    match algo {
        Algorithm::IntLayer | Algorithm::Schoolbook => {
            crate::algos::sub::sub_int_layer::sub_int_layer(a, b)
        }
    }
}

// ── per-type `SubPolicy` trait ────────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for `-`.
pub(crate) trait SubPolicy: Sized {
    /// Subtract `rhs` from `self`, panicking on overflow in both debug and
    /// release.
    fn sub_impl(self, rhs: Self) -> Self;
}

impl<const N: usize, const SCALE: u32> SubPolicy for crate::D<Int<N>, SCALE> {
    #[inline]
    fn sub_impl(self, rhs: Self) -> Self {
        Self(dispatch::<N, SCALE>(self.0, rhs.0))
    }
}
