//! Add policy — the per-`(N, SCALE)` algorithm matcher for decimal addition.
//!
//! `D<Int<N>, SCALE>` addition delegates to [`AddPolicy::add_impl`], which
//! forwards to the one shared [`dispatch`] function. `dispatch` follows the
//! canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
//!
//! 1. an [`Algorithm`] enum — the real addition algorithm, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (add has no value split, so `ByValue` is never returned);
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
//! Decimal addition does not require rescaling: both operands share the same
//! `SCALE`, so the storage-level sum is the answer. The single algorithm
//! (`add_int_layer`) delegates to `Int<N>`'s `checked_add` / `wrapping_add`
//! following Rust's standard integer-overflow contract (debug panics, release
//! wraps). There is no crossover threshold, no work-width widening, and no
//! value-dependent split. `ByValue` is present for canonical-shape
//! uniformity; `select` never returns it.

use crate::int::types::Int;

// ── 1. the real addition algorithm — NAMED, no `Default` ─────────────

/// The addition algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `add_` function
/// prefix (`add_int_layer` → `IntLayer`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`add_int_layer`](crate::algos::add::add_int_layer::add_int_layer) —
    /// delegates directly to `Int<N>`'s checked/wrapping
    /// add, applying Rust's standard integer-overflow contract at the decimal
    /// layer. Same-SCALE addition needs no rescaling.
    IntLayer,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The add picker always
/// returns `ByAlgorithm`: the choice is fully determined by `(N, SCALE)`
/// (which is constant, and the same algorithm wins at every combination).
/// `ByValue` is part of the canonical shape for uniformity; `select` never
/// returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the addition algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `IntLayer` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE; // key accepted for uniformity; unused — one algorithm
    Select::ByAlgorithm(Algorithm::IntLayer)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal addition dispatcher for storage `Int<N>` and decimal `SCALE`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N, SCALE>() }` (folds per monomorphisation; dead arms
/// are eliminated in release) then dispatches exhaustively over
/// [`Algorithm`].
///
/// Not `const fn`: `add_int_layer` branches on `cfg!(debug_assertions)`,
/// which is not permitted in `const fn`. This matches the existing
/// non-`const` `Add` operator on `D<Int<N>, SCALE>`.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(a: Int<N>, b: Int<N>) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::IntLayer,
    };
    match algo {
        Algorithm::IntLayer => crate::algos::add::add_int_layer::add_int_layer(a, b),
    }
}

// ── per-type `AddPolicy` trait ────────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for `+`.
pub(crate) trait AddPolicy: Sized {
    /// Add `rhs` to `self`, applying Rust's standard integer-overflow
    /// contract (panic in debug, wrap in release).
    fn add_impl(self, rhs: Self) -> Self;
}

impl<const N: usize, const SCALE: u32> AddPolicy for crate::D<Int<N>, SCALE> {
    #[inline]
    fn add_impl(self, rhs: Self) -> Self {
        Self(dispatch::<N, SCALE>(self.0, rhs.0))
    }
}
