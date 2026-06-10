// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Hypot policy — the per-`(N, SCALE)` algorithm matcher for
//! `sqrt(self² + other²)` (hypotenuse without intermediate overflow).
//!
//! See `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! `D<Int<N>, SCALE>::hypot_strict_with(other, mode)` delegates directly to
//! the one shared [`dispatch`] generic function. `dispatch` follows the
//! canonical policy shape (mirroring [`crate::policy::sqrt`]):
//!
//! 1. an [`Algorithm`] enum — the real hypot algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (`hypot` has no value split so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Algorithm
//!
//! The single algorithm ([`crate::algos::hypot::hypot_pythagoras`]) forms the
//! radicand `a² + b²` in a limb scratch buffer and takes the root via the
//! int layer's width-agnostic slice `isqrt`. The root goes **down** to the
//! integer layer; the kernel never calls the decimal `sqrt` surface on the
//! tier's own value. No work-width parameter — the policy is a pure
//! `(N, SCALE)` matcher (see [`crate::policy::sqrt`] for the limb-expansion
//! rationale).
//!
//! `hypot(0, 0) = 0` (bit-exact); `hypot(0, x) = |x|` (`isqrt(x²) = |x|`).

use crate::algos::hypot;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real hypot algorithm — NAMED, no `Default` ────────────────

/// The hypot algorithms this policy chooses between. The single variant is
/// the CamelCase of the kernel name minus the `hypot_` prefix — strict 1:1
/// with the kernel computation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`hypot::hypot_pythagoras::hypot_pythagoras`] — `round(sqrt(a² + b²))`.
    /// Both operands share `10^SCALE` (it cancels out of the root), so this
    /// dispatches DOWN to the integer-tier hypot on the raw storages. The
    /// sole hypot algorithm.
    Pythagoras,
    /// Benchmarkable reference seam — delegates to the same
    /// [`hypot::hypot_pythagoras::hypot_pythagoras`] kernel. `select` never
    /// returns this variant.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The hypot picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the hypot algorithm for storage limb count `N` and decimal `SCALE`.
/// Total over the key; `Pythagoras` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE); // keys accepted for uniformity; one algorithm
    Select::ByAlgorithm(Algorithm::Pythagoras)
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Shared hypot dispatch for storage `Int<N>`, decimal `SCALE`. Both
/// operands carry the same `10^SCALE`, so it cancels out of the root and
/// this is exactly integer hypot on the raw storages (dispatched DOWN to the
/// int tier, which forms `a² + b²` in a limb scratch buffer). No work-width
/// parameter; `SCALE` only labels the out-of-range panic. Negative inputs
/// are handled by squaring (sign drops out).
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    // Both operands carry the same `10^SCALE`, so it divides out of the
    // root; `SCALE` is used only to label the out-of-range panic.
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a),
    };
    match algo {
        Algorithm::Pythagoras => hypot::hypot_pythagoras::hypot_pythagoras::<N>(a, b, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale("hypot", SCALE)
            }),
        Algorithm::Schoolbook => hypot::hypot_pythagoras::hypot_pythagoras::<N>(a, b, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale("hypot", SCALE)
            }),
    }
}
