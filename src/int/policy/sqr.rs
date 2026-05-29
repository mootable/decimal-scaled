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
//! Two const squaring algorithms, both bit-identical (each computes `x^2`
//! modulo `2^BITS` exactly), selected per `N` by a **benched crossover band**:
//!
//! - [`crate::int::algos::sqr::sqr_half_product::sqr_half_product`] computes
//!   `x^2` via the const comba half-product kernel
//!   [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`]: it exploits
//!   symmetry to form each cross term once and double it, ≈`N^2/4`
//!   limb-multiplies. It is the default everywhere outside the band.
//! - [`crate::int::algos::sqr::sqr_schoolbook::sqr_schoolbook`] computes the
//!   full `x*x` truncated to the low `N` limbs via the unrolled fixed-width
//!   [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] kernel
//!   (≈`N^2/2` limb-multiplies, no symmetry). Despite the higher multiply
//!   count it WINS in the mid-width band: the comba's variable per-column
//!   inner-loop bound defeats the unroller exactly where `mul_low_fixed`
//!   still unrolls cleanly.
//!
//! The `sqr_full_ab` A/B (`half_product` vs `schoolbook` across `N = 2..64`,
//! both const, the only candidates this `const fn` dispatch can route to;
//! three independent core-pinned runs) localizes the crossover:
//!
//! | N           | winner       | margin            |
//! |-------------|--------------|-------------------|
//! | 2, 3, 4, 6  | ~tie         | <= 1.10x (noise)  |
//! | 8, 10, 12   | schoolbook   | 1.15 .. 1.38x     |
//! | 14, 16, ..  | half_product | 1.46 .. 1.98x     |
//!
//! So the `Schoolbook` band is [`SCHOOLBOOK_LO`]`..=`[`SCHOOLBOOK_HI`]
//! (`8..=13`, placing the upper edge between the N=12 schoolbook win and the
//! N=14 half_product win). The sub-N=8 ties stay on `HalfProduct` — the
//! algorithmically-fewer-multiplies default, no flip on noise. N=8 (D153)
//! and N=12 (D230) are real storage tiers, so the band recovers a real win
//! on the square-and-multiply (`pow`/`cube`) and transcendental paths.
//!
//! The layering points DOWN — each algorithm calls the kernel, never a
//! squaring method on `Uint<N>`.
//!
//! ## What this policy does NOT route to (the const wall)
//!
//! [`dispatch`] is `const fn` (`Uint<N>::wrapping_sqr` is `const fn` and feeds
//! `pow`/`cube`/const contexts), so it can only choose between **const**
//! kernels. The u128-packed truncated-low square
//! [`crate::int::algos::sqr::sqr_low_limb::sqr_low_limb`] is NOT `const fn`
//! (the [`crate::int::types::compute_limbs::Limb`] trait methods are not const),
//! so the `LimbSize` (u64 / u128) axis is INELIGIBLE here. That axis — where
//! the `sqr_full_ab` map shows `u128` overtaking both const arms at `N >= 32`
//! — is owned by the separate **non-const** policy
//! [`crate::int::policy::sqr_low`] (benched via `sqr_low_u128_ab`), which
//! drives `BigInt::wrapping_sqr_low_u128`. This file deliberately does not
//! duplicate that axis.
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
    /// [`sqr_schoolbook`] -- full `x*x` truncated to the low `N` limbs via the
    /// const unrolled [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`]
    /// kernel: `N^2` limb-multiplies, no symmetry exploitation. Bit-identical
    /// to `HalfProduct`. ROUTED in the benched mid-width band
    /// [`SCHOOLBOOK_LO`]`..=`[`SCHOOLBOOK_HI`], where the fixed-width unroll
    /// beats the comba despite the higher multiply count.
    Schoolbook,
}

// -- 2. the verdict --------------------------------------------------------

/// A settled algorithm, or "the value decides". The sqr picker always
/// returns `ByAlgorithm`: the choice is fully determined by `N` (constant per
/// monomorphisation) via the benched crossover band in [`select`]. `ByValue`
/// is part of the canonical shape for uniformity across functions; `select`
/// never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Uint<N>) -> Algorithm),
}

// -- policy data: the benched crossover band ------------------------------

/// Inclusive lower edge of the [`Algorithm::Schoolbook`] band. Below this the
/// `sqr_full_ab` map is a statistical tie (`N = 2/3/4/6`, <= 1.10x both
/// directions across runs), so those `N` stay on the fewer-multiplies
/// [`Algorithm::HalfProduct`] default. File-private policy DATA.
const SCHOOLBOOK_LO: usize = 8;

/// Inclusive upper edge of the [`Algorithm::Schoolbook`] band. The const A/B
/// shows `schoolbook` winning at `N = 8/10/12` and `half_product` winning at
/// `N >= 14`, so the band ends at `13` (between the N=12 schoolbook win and
/// the N=14 half_product win). File-private policy DATA.
const SCHOOLBOOK_HI: usize = 13;

// -- 3. the matcher: const, keyed on `N`, total over the key --------------

/// Pick the squaring algorithm for storage limb count `N`. Total over the
/// key: the benched mid-width band [`SCHOOLBOOK_LO`]`..=`[`SCHOOLBOOK_HI`]
/// takes the full unrolled [`Algorithm::Schoolbook`] (it beats the comba
/// there despite forming twice the partial products — the comba's variable
/// inner-loop bound defeats the unroller in that band); every other width
/// takes the symmetric [`Algorithm::HalfProduct`].
const fn select<const N: usize>() -> Select<N> {
    if N >= SCHOOLBOOK_LO && N <= SCHOOLBOOK_HI {
        Select::ByAlgorithm(Algorithm::Schoolbook)
    } else {
        Select::ByAlgorithm(Algorithm::HalfProduct)
    }
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
