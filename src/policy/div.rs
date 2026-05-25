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
//! directly in a `ComputeInt` limb buffer and divides via the int layer's
//! width-agnostic `div_rem`. So `dispatch` carries no work-width parameter;
//! it adds only `where Int<N>: ComputeInt` for the scratch buffer.
//!
//! The `10^SCALE` multiplier is evaluated here via `Int::<N>::TEN.pow(SCALE)`
//! (folds at compile time per `(N, SCALE)`) and threaded into the kernel.

use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real division algorithms — NAMED, no `Default` ─────────────

/// The division algorithms this policy chooses between. Variants are the
/// CamelCase of each kernel fn's name minus the `div_` prefix
/// (`div_widen_scale` → `WidenScale`, `div_schoolbook` → `Schoolbook`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`crate::algos::div::div_native::div_native`] — hardware-`i128`
    /// scale-then-divide for narrow storage (`N <= 2`, D18 / D38): widens
    /// both operands to `i128` (lossless for `N <= 2`) and computes
    /// `(a * 10^SCALE) / b` via the shared `mg_divide` hardware kernel (its
    /// own `i128` fast path + `256`-bit fallback). At `N == 1` (D18) the
    /// rescale is an `i128 / u64` schoolbook divide (two `divq`). Routed at
    /// `N == 1` and `N == 2` -- microbench: native beats widen at both.
    Native,
    /// [`crate::algos::div::div_widen_scale::div_widen_scale`] — forms
    /// `a * 10^SCALE` in a `2N`-limb scratch buffer, divides by `b` via the
    /// int layer's `div_rem`, rounds, and rebuilds `Int<N>`. The generic
    /// default for every band except `N == 2`.
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
/// `SCALE`. Total over the key; `Native` at `N == 2`, `WidenScale`
/// otherwise.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    // Both narrow bands take the hardware scale-then-divide:
    //   * D18 (`N == 1`, i64 storage): the scaled numerator `a * 10^SCALE`
    //     fits `i128` and the divisor `b` fits `u64`, so the rescale is an
    //     `i128 / u64` schoolbook divide (two `divq`) -- microbench: native
    //     beats widen 2.0-2.1x (s6 / s18).
    //   * D38 (`N == 2`, i128 storage): the shared `i128` / 256-bit kernel
    //     beats forming a `2N`-limb scratch numerator -- microbench: native
    //     beats widen 1.32-1.47x (s6 / s18).
    // `N >= 3` cannot use the i128 path and keeps the generic widen-scale
    // kernel.
    match N {
        1 | 2 => Select::ByAlgorithm(Algorithm::Native),
        _ => Select::ByAlgorithm(Algorithm::WidenScale),
    }
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Decimal division dispatch for storage `Int<N>` and decimal `SCALE`.
///
/// The `const { select }` block folds away at every concrete `N`, leaving a
/// direct call to the chosen kernel. The `2N`-wide scaled numerator lives
/// in the kernel's `ComputeInt` scratch buffer, so no work-width type is
/// named here.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Int<N>: ComputeInt,
{
    // 10^SCALE in Int<N> storage, forced to fold at compile time per
    // (N, SCALE) via the `const` block — a bare `TEN.pow(SCALE)` call runs
    // the int pow square-and-multiply at RUNTIME (the exponent reaches the
    // method as a plain `u32`), so the `const {}` is load-bearing.
    let mult: Int<N> = const { <Int<N>>::TEN.pow(SCALE) };
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a, &b),
    };
    match algo {
        Algorithm::Native => {
            crate::algos::div::div_native::div_native::<N, SCALE>(a, b, mult, mode)
        }
        Algorithm::WidenScale => {
            crate::algos::div::div_widen_scale::div_widen_scale::<N>(a, b, mult, mode)
        }
        Algorithm::Schoolbook => {
            crate::algos::div::div_schoolbook::div_schoolbook::<N>(a, b, mult, mode)
        }
    }
}
