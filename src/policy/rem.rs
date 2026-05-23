//! Rem policy — the per-`(N, SCALE)` algorithm matcher for decimal
//! remainder.
//!
//! `D<Int<N>, SCALE>` remainder delegates to [`RemPolicy::rem_impl`],
//! which forwards to the one shared [`dispatch`] function. `dispatch`
//! follows the canonical policy shape (see `docs/ARCHITECTURE.md` →
//! "Policy file structure"), mirroring [`crate::policy::add`]:
//!
//! 1. an [`Algorithm`] enum — the real remainder algorithm, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (rem has no value split, so `ByValue` is never returned);
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
//! Decimal remainder, like addition, requires no rescaling when both
//! operands share the same `SCALE`: because both carry the same `10^SCALE`
//! factor, the storage-level remainder is the answer. The single algorithm
//! (`rem_int_layer`) delegates to `Int<N>`'s `checked_rem` / `wrapping_rem`
//! following Rust's standard integer-overflow contract (debug panics on
//! `MIN % -ONE` or division by zero, release wraps / panics). There is no
//! crossover threshold, no work-width widening, and no value-dependent
//! split. `ByValue` is present for canonical-shape uniformity; `select`
//! never returns it.

use crate::int::types::Int;

// ── 1. the real remainder algorithm — NAMED, no `Default` ─────────────

/// The remainder algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `rem_` function
/// prefix (`rem_int_layer` → `IntLayer`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`rem_int_layer`] — delegates directly to `Int<N>`'s checked/wrapping
    /// rem, applying Rust's standard integer-overflow contract at the decimal
    /// layer. Same-SCALE remainder needs no rescaling.
    IntLayer,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The rem picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the remainder algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `IntLayer` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    Select::ByAlgorithm(Algorithm::IntLayer)
}

// ── algorithm fn ─────────────────────────────────────────────────────

/// Decimal remainder via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: division by zero always panics; `MIN % -ONE`
/// panics in debug (with "overflow") and wraps in release (matching
/// `i128::wrapping_rem`). No rescaling needed — same-SCALE operands share
/// the scale factor.
#[inline]
fn rem_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    if cfg!(debug_assertions) {
        // `checked_rem` returns `None` for both divide-by-zero and
        // `MIN % -ONE`. `wrapping_rem` panics for divide-by-zero and returns
        // the wrapped value for `MIN % -ONE`, so a non-panicking return means
        // we have an `MIN % -ONE` overflow.
        match a.checked_rem(b) {
            Some(v) => v,
            None => {
                let _ = a.wrapping_rem(b);
                panic!("attempt to calculate the remainder with overflow");
            }
        }
    } else {
        a.wrapping_rem(b)
    }
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal remainder dispatcher for storage `Int<N>` and decimal `SCALE`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N, SCALE>() }` (folds per monomorphisation; dead arms
/// are eliminated in release) then dispatches exhaustively over
/// [`Algorithm`].
///
/// Not `const fn`: `rem_int_layer` branches on `cfg!(debug_assertions)`,
/// which is not permitted in `const fn`.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(a: Int<N>, b: Int<N>) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::IntLayer,
    };
    match algo {
        Algorithm::IntLayer => rem_int_layer(a, b),
    }
}

// ── per-type `RemPolicy` trait ────────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for `%`.
pub(crate) trait RemPolicy: Sized {
    /// Remainder of `self % rhs`, applying Rust's standard integer-overflow
    /// contract (panic in debug, wrap in release; divide-by-zero always
    /// panics).
    fn rem_impl(self, rhs: Self) -> Self;
}

impl<const N: usize, const SCALE: u32> RemPolicy for crate::D<Int<N>, SCALE> {
    #[inline]
    fn rem_impl(self, rhs: Self) -> Self {
        Self(dispatch::<N, SCALE>(self.0, rhs.0))
    }
}
