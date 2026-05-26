// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer sum-of-squares policy -- the per-`N` algorithm matcher for
//! `a^2 + b^2`.
//!
//! `Int<N>::sum_sq` delegates to [`dispatch`], which follows the canonical
//! policy shape (see `docs/ARCHITECTURE.md` -> "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum -- the real sum-of-squares algorithms, no
//!    `Default` variant;
//! 2. a [`Select`] verdict -- a settled algorithm or "the value decides"
//!    (`sum_sq` has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` -- no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on `N`, the `const { ... }`
//! block folds per monomorphisation and the unchosen arm is
//! dead-arm-eliminated in release: each concrete `Int<N>` compiles to a
//! direct call to one kernel, no runtime branch.
//!
//! # Algorithm
//!
//! Two algorithms, both forming `a^2 + b^2` in a limb scratch buffer and
//! fit-checking to `Int<N>` (bit-identical by construction — same value, same
//! `None`-on-overflow contract; they differ only in how each square is
//! formed):
//!
//! - [`Algorithm::Schoolbook`]
//!   ([`crate::int::algos::sum_sq::sum_sq_schoolbook`]) — each square via the
//!   general `mul_schoolbook(x, x, ..)` product (~L^2 limb-multiplies).
//! - [`Algorithm::Comba`] ([`crate::int::algos::sum_sq::sum_sq_comba`]) — each
//!   square via a symmetric product-scanning pass that forms each off-diagonal
//!   partial product `x_i*x_j` once and doubles it (~L^2/2 limb-multiplies).
//!
//! `select` keys the choice on `N`. The comba edge is asymptotic in the
//! operand limb-length, so it pays off once the squares are wide enough to
//! amortise the column-loop setup; the narrowest tier prefers schoolbook:
//!
//! - **`N <= 2`** -> [`Algorithm::Schoolbook`]: at one-to-two limbs the
//!   product is a handful of multiplies; comba's column machinery is pure
//!   overhead. The `sum_sq_ab` N-way A/B confirms a stable ~1.16x schoolbook
//!   win at `N == 2`.
//! - **`N >= 3`** -> [`Algorithm::Comba`]: the halved partial-product count
//!   wins, growing with width (the `sum_sq_ab` A/B shows stable comba wins at
//!   the wide tiers, ~1.3-1.75x at `N` in {16, 32, 64}; the `N` in {3..6} band
//!   is near parity but comba is the asymptotic winner and never materially
//!   loses there).
//!
//! Returns [`None`] from the kernel on true overflow (the sum does not fit
//! the signed range of `Int<N>`); the type method propagates that `Option`.

use crate::int::algos::sum_sq::sum_sq_comba::sum_sq_comba;
use crate::int::algos::sum_sq::sum_sq_schoolbook::sum_sq_schoolbook;
use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;

// -- 1. the real sum-of-squares algorithms -- NAMED, no `Default` ------

/// The integer sum-of-squares algorithms this policy chooses between.
/// Variants are the CamelCase of each kernel fn's name minus the `sum_sq_`
/// prefix -- strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`sum_sq_schoolbook`] -- `a^2 + b^2` with each square formed by the
    /// general `mul_schoolbook(x, x, ..)` product (~L^2 limb-mults). The
    /// narrow-tier winner (`N <= 2`).
    Schoolbook,
    /// [`sum_sq_comba`] -- `a^2 + b^2` with each square formed by a symmetric
    /// product-scanning pass (~L^2/2 limb-mults). Bit-identical to
    /// `Schoolbook`; the wide-tier winner (`N >= 3`), margin growing with
    /// width.
    Comba,
}

// -- 2. the const verdict ----------------------------------------------

/// A settled algorithm, or "the value decides". The sum-of-squares picker
/// always returns `ByAlgorithm`. `ByValue` is part of the canonical shape
/// for uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// -- 3. the matcher: const, keyed on `N`, total over the key -----------

/// Pick the integer sum-of-squares algorithm for storage limb count `N`.
/// Total over the key.
///
/// - `N <= 2` -> [`Algorithm::Schoolbook`] (narrow: comba's column setup is
///   pure overhead at one-to-two limbs).
/// - `N >= 3` (the `_` arm) -> [`Algorithm::Comba`] (the ~L^2/2 partial-
///   product count wins, growing with width).
///
/// Wired from the `sum_sq_ab` N-way dispatch-seam A/B (both kernels asserted
/// bit-identical at every cell, incl. the overflow `None` path).
const fn select<const N: usize>() -> Select<N> {
    match N {
        1 | 2 => Select::ByAlgorithm(Algorithm::Schoolbook),
        _ => Select::ByAlgorithm(Algorithm::Comba),
    }
}

// -- 4. the shared dispatch: resolve the verdict, then dispatch --------

/// Integer sum-of-squares dispatcher for `Int<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release), then dispatches exhaustively over [`Algorithm`].
/// Returns [`None`] when `a^2 + b^2` does not fit the signed range of
/// `Int<N>` (true overflow). The signs of the operands drop out of squaring.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize>(a: Int<N>, b: Int<N>) -> Option<Int<N>>
where
    Int<N>: ComputeInt,
{
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a),
    };
    match algo {
        Algorithm::Schoolbook => sum_sq_schoolbook::<N>(a, b),
        Algorithm::Comba => sum_sq_comba::<N>(a, b),
    }
}
