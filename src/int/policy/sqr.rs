// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer squaring policy -- the half-product algorithm matcher.
//!
//! `Uint<N>::sqr` / `Uint<N>::wrapping_sqr` and the `Int<N>` siblings
//! delegate to [`dispatch`], which follows the canonical policy shape (see
//! `docs/ARCHITECTURE.md` -> "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum -- the real squaring algorithm(s), no `Default`
//!    variant;
//! 2. a [`Select`] verdict -- a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` -- no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { ... }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Uint<N>` compiles to a
//! direct call to the half-product squaring kernel, no runtime branch.
//!
//! # Algorithm
//!
//! The algorithm fn [`crate::int::algos::sqr::sqr_half_product::sqr_half_product`]
//! computes `x^2` via the const half-product kernel
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`]: it exploits symmetry to
//! form each cross term once and double it, halving the limb-multiply count
//! relative to a general `NxN` multiply. The layering points DOWN -- the
//! algorithm calls the kernel, never a squaring method on `Uint<N>`.
//!
//! A `Schoolbook` reference arm is registered for the naive `x*x` delegate
//! (via [`crate::int::algos::sqr::sqr_schoolbook::sqr_schoolbook`]). It is unrouted
//! (not returned by `select`) and marked `#[allow(dead_code)]` so the
//! exhaustive match stays warning-clean.
//!
//! The `ByValue` arm of [`Select`] is present for canonical-shape
//! uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` IS `const fn`: the algorithm fn computes via the const
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] kernel, so the type's
//! `const fn` `wrapping_sqr` can delegate through it. The `ByValue` arm
//! returns the default algorithm tag without invoking the fn pointer
//! (calling a fn pointer is not permitted in `const fn`; merely matching
//! the variant is fine).

use crate::int::algos::sqr::sqr_half_product::sqr_half_product;
use crate::int::algos::sqr::sqr_schoolbook::sqr_schoolbook;
use crate::int::types::Uint;

// -- 1. the real squaring algorithms -- NAMED, no `Default` ---------------

/// The squaring algorithms this policy chooses between. Variants are the
/// CamelCase of each kernel fn's name minus the `sqr_` function prefix --
/// strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`sqr_half_product`] -- half-product squaring via the const
    /// [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] kernel. Forms each cross
    /// term once and doubles it: `N(N+1)/2` limb-multiplies rather than
    /// `N^2`. Result is `x^2` modulo `2^BITS`.
    HalfProduct,
    /// [`sqr_schoolbook`] -- naive reference squaring via the const
    /// [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] kernel. Treats
    /// squaring as a general `x*x` multiply: `N^2` limb-multiplies, no
    /// symmetry exploitation. Bit-identical to `HalfProduct`; unrouted
    /// reference arm.
    #[allow(dead_code)]
    Schoolbook,
}

// -- 2. the verdict --------------------------------------------------------

/// A settled algorithm, or "the value decides". The sqr picker always
/// returns `ByAlgorithm`: the choice is fully determined by `N` (which is
/// constant, and the same algorithm wins at every `N`). `ByValue` is part
/// of the canonical shape for uniformity across functions; `select` never
/// returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Uint<N>) -> Algorithm),
}

// -- 3. the matcher: const, keyed on `N`, total over the key --------------

/// Pick the squaring algorithm for storage limb count `N`. Total over the
/// key; the half-product squaring kernel is width-independent so
/// `HalfProduct` wins at every `N`.
const fn select<const N: usize>() -> Select<N> {
    Select::ByAlgorithm(Algorithm::HalfProduct)
}

// -- 4. the dispatcher: fold the verdict, then dispatch --------------------

/// Integer squaring dispatcher for `Uint<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Must be `const fn`: `Int<N>::wrapping_sqr` is itself `const fn` and is
/// called from `const` contexts (and from `pow`/`cube`). The `ByValue` arm
/// returns the default algorithm tag without invoking the fn pointer,
/// satisfying the `const fn` constraint.
#[inline]
pub(crate) const fn dispatch<const N: usize>(x: Uint<N>) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        // sqr is always ByAlgorithm; fall through to the default if the
        // arm is reached (fn pointer calls are not allowed in const fn).
        Select::ByValue(_) => Algorithm::HalfProduct,
    };
    match algo {
        Algorithm::HalfProduct => sqr_half_product(x),
        Algorithm::Schoolbook => sqr_schoolbook(x),
    }
}
