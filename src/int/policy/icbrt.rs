// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer cube-root policy â€” the native-vs-Newton algorithm matcher.
//!
//! `Uint<N>::icbrt` and `Int<N>::icbrt` delegate to [`dispatch`], which
//! follows the canonical policy shape (see `docs/ARCHITECTURE.md` â†’
//! "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum â€” the real icbrt algorithms, no `Default`
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
//! The icbrt policy mirrors the isqrt policy (see [`super::isqrt`]) with
//! the same `N`-keyed split:
//!
//! - **`N âˆˆ {1, 2}`** â†’ [`icbrt_native`]: hardware-assisted cube root using
//!   the primitive type's cube root via `u64`/`u128` arithmetic. For small
//!   widths the result fits a single `u64` (`N == 1`) or two limbs
//!   (`N == 2`); a direct Newton step is faster than the general loop.
//! - **`N >= 3`** â†’ [`icbrt_newton`]: width-agnostic Newton iteration with a
//!   hardware-`f64::cbrt` seed over u64 limbs â€” one algorithm for all
//!   wider integers. Implemented in
//!   [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`].
//!
//! The `ByValue` arm of [`Select`] is present for canonical-shape
//! uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` is **not** `const fn`. The `Newton` arm calls
//! [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`] which performs Newton
//! iteration and is not const-evaluable.

use crate::int::algos::icbrt::icbrt_newton::icbrt_newton as icbrt_newton_kernel;
use crate::int::algos::icbrt::icbrt_schoolbook::icbrt_schoolbook as icbrt_schoolbook_kernel;
use crate::int::types::Uint;

// â”€â”€ 1. the real icbrt algorithms â€” NAMED, no `Default` â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// The integer cube-root algorithms this policy chooses between. Variants
/// are the CamelCase of each kernel fn's name minus the `icbrt_` function
/// prefix â€” strict 1:1 with the kernel fns.
///
/// Names follow RULES Â§4: `icbrt_native` â†’ `Native`, `icbrt_newton` â†’
/// `Newton`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`icbrt_native`] â€” hardware-assisted path for `N âˆˆ {1, 2}`: the
    /// cube root fits a single `u64` (`N == 1`) or two limbs (`N == 2`),
    /// computed via direct arithmetic without the full Newton loop.
    Native,
    /// [`icbrt_newton`] â€” width-agnostic Newton iteration with a
    /// hardware-`f64::cbrt` seed over u64 limbs. Serves every `N >= 3`.
    /// Delegates to [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`].
    Newton,
    /// [`icbrt_schoolbook`] -- bit-by-bit restoring cube root;
    /// pure integer, no division, no float seed. Serves any `N`.
    #[allow(dead_code)]
    Schoolbook,
}

// â”€â”€ 2. the verdict â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// A settled algorithm, or "the value decides". The icbrt picker always
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

/// Pick the icbrt algorithm for storage limb count `N`. Total over the key.
///
/// - `N âˆˆ {1, 2}` â†’ [`Algorithm::Native`] (narrow fast path).
/// - `N >= 3` (the `_` arm) â†’ [`Algorithm::Newton`] (generic limb Newton).
const fn select<const N: usize>() -> Select<N> {
    match N {
        1 | 2 => Select::ByAlgorithm(Algorithm::Native),
        _ => Select::ByAlgorithm(Algorithm::Newton),
    }
}

// â”€â”€ algorithm fns â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Native hardware-assisted integer cube root for `Uint<N>` where `N âˆˆ {1, 2}`.
///
/// For `N == 1` the cube root fits entirely in a `u64` (max input
/// `2^64 âˆ’ 1`, max root `2^21 + â€¦ < 2^22`). For `N == 2` the cube root
/// of a 128-bit value fits in a `u64` as well (max 128-bit value has cube
/// root < `2^43`). We delegate to the general Newton kernel
/// ([`icbrt_newton_kernel`]) which handles both widths correctly via its
/// limb loop; for these small widths the loop terminates in a handful of
/// iterations and the result is exact.
#[inline]
pub(crate) fn icbrt_native<const N: usize>(x: Uint<N>) -> Uint<N> {
    let mut out = [0u64; N];
    icbrt_newton_kernel(x.as_limbs(), &mut out);
    Uint::<N>::from_limbs(out)
}

/// Newton integer cube root for `Uint<N>` where `N >= 3`.
///
/// Delegates to [`icbrt_newton_kernel`]: Newton iteration with a
/// hardware-`f64::cbrt` seed over u64 limbs; converges quadratically to
/// `floor(x^(1/3))`.
#[inline]
pub(crate) fn icbrt_newton<const N: usize>(x: Uint<N>) -> Uint<N> {
    let mut out = [0u64; N];
    icbrt_newton_kernel(x.as_limbs(), &mut out);
    Uint::<N>::from_limbs(out)
}

/// Schoolbook bit-by-bit integer cube root for `Uint<N>`.
///
/// Delegates to
/// [`icbrt_schoolbook_kernel`][`crate::int::algos::icbrt::icbrt_schoolbook::icbrt_schoolbook`]:
/// digit-by-digit restoring algorithm; no division, no float seed.
/// Serves any `N` as a generic reference baseline.
#[allow(dead_code)]
#[inline]
pub(crate) fn icbrt_schoolbook_policy<const N: usize>(x: Uint<N>) -> Uint<N> {
    let mut out = [0u64; N];
    icbrt_schoolbook_kernel(x.as_limbs(), &mut out);
    Uint::<N>::from_limbs(out)
}

// â”€â”€ 4. the dispatcher: fold the verdict, then dispatch â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Integer cube-root dispatcher for `Uint<N>`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N>() }` (folds per monomorphisation; dead arms are
/// eliminated in release) then dispatches exhaustively over [`Algorithm`].
///
/// Not `const fn`: the `Newton` arm delegates to
/// [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`] (Newton iteration, not
/// const-evaluable).
#[inline]
pub(crate) fn dispatch<const N: usize>(x: Uint<N>) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&x),
    };
    match algo {
        Algorithm::Native => icbrt_native::<N>(x),
        Algorithm::Newton => icbrt_newton::<N>(x),
        Algorithm::Schoolbook => icbrt_schoolbook_policy::<N>(x),
    }
}
