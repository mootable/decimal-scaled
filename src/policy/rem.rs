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
//! # The two algorithms and the width split
//!
//! Decimal remainder requires no rescaling when both operands share the same
//! `SCALE`: because both carry the same `10^SCALE` factor, the storage-level
//! remainder is the answer. The split is purely on storage width `N`:
//!
//! * **`N <= 2`** (D18 / D38) → `rem_native`: the storage value fits a single
//!   hardware integer, so the remainder is a direct primitive `%` (`i64 %` at
//!   `N == 1`, `i128 %` at `N == 2`). Microbenched at the dispatch seam:
//!   native beats `rem_int_layer` ~1.5x at D18 and ~8x at D38 by skipping the
//!   generic unpack-to-magnitude / divmod / signed-repack.
//! * **`N >= 3`** → `rem_int_layer`: the magnitude exceeds a single hardware
//!   integer, so it delegates to `Int<N>`'s `checked_rem` / `wrapping_rem`
//!   following Rust's standard integer-overflow contract.
//!
//! Both follow the same overflow contract (debug panics on `MIN % -ONE` or a
//! zero divisor, release wraps / panics). `ByValue` is present for
//! canonical-shape uniformity; `select` never returns it.

use crate::int::types::Int;

// ── 1. the real remainder algorithm — NAMED, no `Default` ─────────────

/// The remainder algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `rem_` function
/// prefix (`rem_int_layer` → `IntLayer`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`rem_native`](crate::algos::rem::rem_native::rem_native) — hardware
    /// primitive `%` on the narrow storage value (`i64 %` at `N == 1`,
    /// `i128 %` at `N == 2`), the same-SCALE remainder needing no rescaling.
    /// Valid only for `N <= 2`; routed for D18 / D38, where bypassing the
    /// generic unpack / divmod / repack of [`Self::IntLayer`] wins decisively
    /// (microbench: native beats int-layer ~1.5x at D18, ~8x at D38).
    Native,
    /// [`rem_int_layer`](crate::algos::rem::rem_int_layer::rem_int_layer) —
    /// delegates directly to `Int<N>`'s checked/wrapping
    /// rem, applying Rust's standard integer-overflow contract at the decimal
    /// layer. Same-SCALE remainder needs no rescaling. The generic default
    /// for `N >= 3`.
    IntLayer,
    /// Naive schoolbook reference: delegates to the same
    /// [`rem_int_layer`](crate::algos::rem::rem_int_layer::rem_int_layer)
    /// kernel. This variant documents the seam and stays unrouted by `select`.
    #[allow(dead_code)]
    Schoolbook,
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
    // Narrow storage (`N <= 2`): the storage value fits a single hardware
    // integer, so the remainder is a direct primitive `%` -- far cheaper
    // than the generic int-layer unpack-to-magnitude / divmod / repack
    // (microbench: native beats int-layer ~1.5x at D18, ~8x at D38).
    // `N >= 3` exceeds a single hardware integer and keeps `IntLayer`.
    match N {
        1 | 2 => Select::ByAlgorithm(Algorithm::Native),
        _ => Select::ByAlgorithm(Algorithm::IntLayer),
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
/// Not `const fn`: both `rem_native` and `rem_int_layer` branch on
/// `cfg!(debug_assertions)`, which is not permitted in `const fn`.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(a: Int<N>, b: Int<N>) -> Int<N>
where
    crate::int::types::compute_limbs::Limbs<N>: crate::int::types::compute_limbs::ComputeLimbs,
{
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::IntLayer,
    };
    match algo {
        Algorithm::Native => crate::algos::rem::rem_native::rem_native(a, b),
        Algorithm::IntLayer | Algorithm::Schoolbook => {
            crate::algos::rem::rem_int_layer::rem_int_layer(a, b)
        }
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

impl<const N: usize, const SCALE: u32> RemPolicy for crate::D<Int<N>, SCALE>
where
    crate::int::types::compute_limbs::Limbs<N>: crate::int::types::compute_limbs::ComputeLimbs,
{
    #[inline]
    fn rem_impl(self, rhs: Self) -> Self {
        Self(dispatch::<N, SCALE>(self.0, rhs.0))
    }
}
