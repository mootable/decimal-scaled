// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer cube-root policy — the native-vs-Newton algorithm matcher.
//!
//! `Uint<N>::icbrt` and `Int<N>::icbrt` delegate to [`dispatch`], which
//! follows the canonical policy shape (see `docs/ARCHITECTURE.md` â†’
//! "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real icbrt algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `N`, total over the key;
//! 4. dispatch via an inline `const { select::<N>() }` block, then an
//!    **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generic `N`,
//! the `const { â€¦ }` block folds per monomorphisation and the unchosen arm
//! is dead-arm-eliminated in release: each concrete `Uint<N>` compiles to a
//! direct call to the chosen kernel, no runtime branch.
//!
//! # Algorithm selection
//!
//! Unlike isqrt (which has a genuine hardware `u64::isqrt` / `u128::isqrt`
//! narrow path), there is **no native hardware cube root**: every width is
//! served by the one width-agnostic Newton kernel
//! ([`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`]) — Newton
//! iteration with a shared-library `f64::cbrt` seed over u64 limbs. The
//! `icbrt_ab` N-way A/B confirms this Newton kernel beats the bitwise
//! [`Algorithm::Schoolbook`] reference at EVERY width (12.9x at `N == 1`
//! growing to ~50x at `N == 64`), so `select` returns
//! [`Algorithm::Newton`] for all `N`.
//!
//! [`Algorithm::Schoolbook`] is the registered-but-unselected reference
//! baseline (kept per `docs/ARCHITECTURE.md` → "Keeping the alternatives").
//! The `ByValue` arm of [`Select`] is present for canonical-shape
//! uniformity; `select` never returns it.
//!
//! # Const-ness
//!
//! `dispatch` is **not** `const fn`. The `Newton` arm calls
//! [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`] which performs Newton
//! iteration and is not const-evaluable.
//!
//! Names follow RULES §4: `icbrt_newton` → `Newton`, `icbrt_schoolbook` →
//! `Schoolbook`.

use crate::int::algos::icbrt::icbrt_newton::icbrt_newton as icbrt_newton_kernel;
use crate::int::algos::icbrt::icbrt_schoolbook::icbrt_schoolbook as icbrt_schoolbook_kernel;
use crate::int::types::Uint;

// â”€â”€ 1. the real icbrt algorithms — NAMED, no `Default` â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// The integer cube-root algorithms this policy chooses between. Variants
/// are the CamelCase of each kernel fn's name minus the `icbrt_` function
/// prefix — strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`icbrt_newton`] — width-agnostic Newton iteration with a
    /// shared-library `f64::cbrt` seed over u64 limbs. Serves every `N`
    /// (there is no native hardware cube root). Delegates to
    /// [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`].
    Newton,
    /// [`icbrt_schoolbook`] -- bit-by-bit restoring cube root;
    /// pure integer, no division, no float seed. Serves any `N`. The
    /// registered-but-unselected reference baseline.
    #[allow(dead_code)]
    Schoolbook,
}

// â”€â”€ 2. the verdict â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// A settled algorithm, or "the value decides". The icbrt picker always
/// returns `ByAlgorithm(Newton)`: one algorithm serves every `N`. `ByValue`
/// is part of the canonical shape for uniformity across functions; `select`
/// never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Uint<N>) -> Algorithm),
}

// â”€â”€ 3. the matcher: const, keyed on `N`, total over the key â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Pick the icbrt algorithm for storage limb count `N`. Total over the key;
/// [`Algorithm::Newton`] wins at every `N` (the `icbrt_ab` A/B beats the
/// `Schoolbook` reference 12.9x–50x across the full width sweep).
const fn select<const N: usize>() -> Select<N> {
    let _ = N; // key accepted for uniformity; one algorithm at every width
    Select::ByAlgorithm(Algorithm::Newton)
}

// â”€â”€ algorithm fns â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Newton integer cube root for `Uint<N>` — serves every `N`.
///
/// Delegates to [`icbrt_newton_kernel`]: Newton iteration with a
/// shared-library `f64::cbrt` seed over u64 limbs; converges quadratically to
/// `floor(x^(1/3))`. For the narrow widths (`N <= 2`) the limb loop
/// terminates in a handful of iterations; there is no distinct hardware
/// cube-root path, so this one kernel is the icbrt for all widths.
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
        Algorithm::Newton => icbrt_newton::<N>(x),
        Algorithm::Schoolbook => icbrt_schoolbook_policy::<N>(x),
    }
}
