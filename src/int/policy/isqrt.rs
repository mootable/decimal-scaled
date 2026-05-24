// SPDX-FileCopyrightText: 2026 John Moxley

// SPDX-License-Identifier: MIT OR Apache-2.0



//! Integer square-root policy â€” the native-vs-Newton algorithm matcher.

//!

//! `Uint<N>::isqrt` delegates to [`dispatch`], which follows the canonical

//! policy shape (see `docs/ARCHITECTURE.md` â†’ "Policy file structure"):

//!

//! 1. an [`Algorithm`] enum â€” the real isqrt algorithms, no `Default`

//!    variant;

//! 2. a [`Select`] verdict â€” a settled algorithm or "the value decides";

//! 3. a `const fn` [`select`] keyed on `N`, total over the key;

//! 4. dispatch via an inline `const { select::<N>() }` block, then an

//!    **exhaustive** `match algo` â€” no `_`, no panic.

//!

//! Because `select` is `const` and keyed only on the const generic `N`,

//! the `const { â€¦ }` block folds per monomorphisation and the unchosen arm

//! is dead-arm-eliminated in release: each concrete `Uint<N>` compiles to a

//! direct call to the chosen kernel, no runtime branch.

//!

//! # Algorithm selection

//!

//! The two algorithms correspond directly to the arms of the existing

//! const-`N` ladder in [`crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed`], which

//! this policy formalises:

//!

//! - **`N âˆˆ {1, 2}`** â†’ [`isqrt_native`]: single hardware instruction via

//!   `u64::isqrt` (`N == 1`) or `u128::isqrt` (`N == 2`). The fastest path

//!   at these widths; genuinely width-bespoke (no generic form).

//! - **`N >= 3`** â†’ [`isqrt_newton`]: width-agnostic Newton iteration with a

//!   hardware-`f64::sqrt` seed over u64 limbs â€” one algorithm serving every

//!   wider int. Today's `limbs_isqrt_u64` (now in `int/algos/roots.rs`).

//!

//! The `ByValue` arm of [`Select`] is present for canonical-shape uniformity;

//! `select` never returns it (the choice is fully determined by `N`).

//!

//! # Const-ness

//!

//! `dispatch` is **not** `const fn`. The `Newton` arm calls

//! [`isqrt_newton`], which performs Newton iteration and is not

//! const-evaluable. The `Native` arm could in principle be `const`, but

//! because the policy must accommodate both arms a single `const fn` is not

//! possible. `Uint<N>::isqrt` is therefore not `const fn`.



use crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed;
use crate::int::algos::isqrt::isqrt_schoolbook::isqrt_schoolbook as isqrt_schoolbook_kernel;

use crate::int::types::Uint;



// â”€â”€ 1. the real isqrt algorithms â€” NAMED, no `Default` â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€



/// The integer square-root algorithms this policy chooses between. Variants

/// are the CamelCase of each kernel fn's name minus the `isqrt_` function

/// prefix â€” strict 1:1 with the kernel fns.

///

/// Names follow RULES Â§4: `isqrt_native` â†’ `Native`, `isqrt_newton` â†’

/// `Newton`.

#[derive(Clone, Copy, PartialEq, Eq)]

enum Algorithm {

    /// [`isqrt_native`] â€” hardware `u64::isqrt` (`N == 1`) or

    /// `u128::isqrt` (`N == 2`). Width-bespoke; const-split inside the fn.

    Native,

    /// [`isqrt_newton`] â€” width-agnostic Newton iteration with a

    /// hardware-`f64::sqrt` seed over u64 limbs. Serves every `N >= 3`.

    Newton,
    /// [`isqrt_schoolbook`] -- two-bits-at-a-time bitwise reference
    /// implementation; pure integer, no division, no floating-point seed.
    /// Serves any `N` as a generic reference baseline.
    #[allow(dead_code)]
    Schoolbook,
}



// â”€â”€ 2. the verdict â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€



/// A settled algorithm, or "the value decides". The isqrt picker always

/// returns `ByAlgorithm`: the choice is fully determined by `N`. `ByValue`

/// is part of the canonical shape for uniformity across functions; `select`

/// never returns it.

#[derive(Clone, Copy)]

enum Select<const N: usize> {

    ByAlgorithm(Algorithm),

    #[allow(dead_code)]

    ByValue(fn(&Uint<N>) -> Algorithm),

}



// â”€â”€ 3. the matcher: const, keyed on `N`, total over the key â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€



/// Pick the isqrt algorithm for storage limb count `N`. Total over the key.

///

/// - `N âˆˆ {1, 2}` â†’ [`Algorithm::Native`] (hardware single-instruction path).

/// - `N >= 3` (the `_` arm) â†’ [`Algorithm::Newton`] (generic limb Newton).

const fn select<const N: usize>() -> Select<N> {

    match N {

        1 | 2 => Select::ByAlgorithm(Algorithm::Native),

        _ => Select::ByAlgorithm(Algorithm::Newton),

    }

}



// â”€â”€ algorithm fns: thin delegations to the existing kernels â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€



/// Native hardware integer square root for `Uint<N>` where `N âˆˆ {1, 2}`.

///

/// Delegates to [`isqrt_mag_fixed`] which const-splits on `N` internally:

/// `N == 1` â†’ `u64::isqrt`, `N == 2` â†’ `u128::isqrt`. Both are single

/// hardware instructions on modern ISAs.

#[inline]

pub(crate) fn isqrt_native<const N: usize>(x: Uint<N>) -> Uint<N> {

    let mut out = [0u64; N];

    isqrt_mag_fixed::<N>(x.as_limbs(), &mut out);

    Uint::<N>::from_limbs(out)

}



/// Newton integer square root for `Uint<N>` where `N >= 3`.

///

/// Delegates to [`isqrt_mag_fixed`] which routes to

/// [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`] for `N >= 3`: Newton

/// iteration with a hardware-`f64::sqrt` seed over the u64 limbs.

#[inline]

pub(crate) fn isqrt_newton<const N: usize>(x: Uint<N>) -> Uint<N> {

    let mut out = [0u64; N];

    isqrt_mag_fixed::<N>(x.as_limbs(), &mut out);

    Uint::<N>::from_limbs(out)

}



/// Schoolbook two-bits-at-a-time integer square root for `Uint<N>`.
///
/// Delegates to
/// [`isqrt_schoolbook_kernel`][`crate::int::algos::isqrt::isqrt_schoolbook::isqrt_schoolbook`]:
/// classical bitwise digit-by-digit algorithm; no division, no float seed.
/// Serves any `N` as a generic reference baseline.
#[allow(dead_code)]
#[inline]
pub(crate) fn isqrt_schoolbook_policy<const N: usize>(x: Uint<N>) -> Uint<N> {
    let mut out = [0u64; N];
    isqrt_schoolbook_kernel(x.as_limbs(), &mut out);
    Uint::<N>::from_limbs(out)
}

// â”€â”€ 4. the dispatcher: fold the verdict, then dispatch â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€



/// Integer square-root dispatcher for `Uint<N>`.

///

/// Resolves the compile-time algorithm verdict via

/// `const { select::<N>() }` (folds per monomorphisation; dead arms are

/// eliminated in release) then dispatches exhaustively over [`Algorithm`].

///

/// Not `const fn`: the `Newton` arm delegates to

/// [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`] (Newton iteration, not

/// const-evaluable).

#[inline]

pub(crate) fn dispatch<const N: usize>(x: Uint<N>) -> Uint<N> {

    let algo = match const { select::<N>() } {

        Select::ByAlgorithm(a) => a,

        Select::ByValue(f) => f(&x),

    };

    match algo {

        Algorithm::Native => isqrt_native::<N>(x),

        Algorithm::Newton => isqrt_newton::<N>(x),

        Algorithm::Schoolbook => isqrt_schoolbook_policy::<N>(x),

    }

}

