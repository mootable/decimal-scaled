//! Mul policy — the per-`(N, SCALE)` algorithm matcher for decimal
//! multiplication.
//!
//! `D<Int<N>, SCALE>::mul_with` delegates directly to the one shared
//! [`dispatch`] generic function, which follows the canonical policy shape
//! (see `docs/ARCHITECTURE.md` → "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real multiply algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (mul has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Work width — expanded in limbs, no `Int<2N>` type
//!
//! Decimal multiplication forms `a * b`, which spans up to `2N` limbs,
//! before dividing by `10^SCALE`. Rather than thread a work *type* `Int<2N>`
//! (unnameable from `N` on stable), the [`WidenDivide`](Algorithm::WidenDivide)
//! kernel does that arithmetic directly in a `WorkScratch` limb buffer and
//! divides via the shared MG / Newton magnitude-slice cores. So `dispatch`
//! carries no work-width parameter and the policy stays a pure `(N, SCALE)`
//! matcher; it adds only `where Int<N>: WorkScratch` for the scratch buffer.
//!
//! # Why there is only one selected algorithm
//!
//! Decimal multiply has two internal paths (a fast path when the product
//! fits `Int<N>`, and a widening path), but both are implementation details
//! of the single `mul_widen_divide` algorithm. `Schoolbook` is an unrouted
//! benchmarkable reference seam (no MG / Newton, plain int `div_rem`).

use crate::int::types::work_scratch::WorkScratch;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real multiply algorithms — NAMED, no `Default` ─────────────

/// The multiply algorithms this policy chooses between. Variants are the
/// CamelCase of each kernel fn's name minus the `mul_` prefix
/// (`mul_widen_divide` → `WidenDivide`, `mul_schoolbook` → `Schoolbook`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`crate::algos::mul::mul_widen_divide::mul_widen_divide`] — forms
    /// `a * b` in a `2N`-limb scratch buffer, divides by `10^SCALE` via the
    /// MG / Newton magnitude cores, rebuilds `Int<N>`. A leading-zero fast
    /// path keeps the divide narrow when the product fits `Int<N>`. The
    /// generic default at every `(N, SCALE)`.
    WidenDivide,
    /// [`crate::algos::mul::mul_schoolbook::mul_schoolbook`] — naive
    /// reference: full magnitude product then a plain int-layer `div_rem`
    /// by `10^SCALE`, no MG / Newton and no leading-zero fast path. Unrouted
    /// by `select`; a real benchmarkable seam, used directly in tests.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The mul picker always
/// returns `ByAlgorithm`; `ByValue` is part of the canonical shape for
/// uniformity and `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the multiply algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `WidenDivide` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    Select::ByAlgorithm(Algorithm::WidenDivide)
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Decimal multiply dispatch for storage `Int<N>` and decimal `SCALE`.
///
/// The `const { select }` block folds away at every concrete `N`, leaving a
/// direct call to the chosen kernel. `dispatch` delegates *down* to the
/// generic-over-`N` kernel; the `2N`-wide product lives in the kernel's
/// `WorkScratch` scratch buffer, so no work-width type is named here.
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
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a, &b),
    };
    match algo {
        Algorithm::WidenDivide => {
            crate::algos::mul::mul_widen_divide::mul_widen_divide::<N, SCALE>(a, b, mode)
        }
        Algorithm::Schoolbook => {
            crate::algos::mul::mul_schoolbook::mul_schoolbook::<N, SCALE>(a, b, mode)
        }
    }
}
