// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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
//! * **`N == 1`** (D18) → `rem_native`: the storage value is one `i64`, so the
//!   remainder is a single hardware `idiv`, and skipping the generic
//!   unpack-to-magnitude / compare / signed-repack is worth more than that
//!   setup costs.
//! * **`N >= 2`** → `rem_int_layer`: unpacks to unsigned magnitudes and runs
//!   the const-`N` divmod, detecting the `MIN % -ONE` overflow up front, with
//!   two value-gated fast paths ahead of it (`|a| < |b|` returns the dividend;
//!   single-128-bit-word operands take one `u128 %`).
//!
//! The split is at `N == 1`, not at `N <= 2`, because the hardware runs out
//! one width earlier than the storage does. `i64 %` is an instruction;
//! `i128 %` is not — x86-64 has no 128-bit divide, so `N == 2`'s "native"
//! arm is really the `__modti3` / `__udivmodti4` soft-call. Once both arms
//! are paying for a soft-call, `rem_int_layer` wins on the strength of its
//! fast paths. See `rem_native`'s header for the measurement.
//!
//! Both follow the same overflow contract: a zero divisor and the
//! `MIN % -ONE` overflow both panic in BOTH debug and release (the default
//! operator never silently wraps to `0`). `ByValue` is present for
//! canonical-shape uniformity; `select` never returns it.

use crate::int::types::Int;

// ── 1. the real remainder algorithm — NAMED, no `Default` ─────────────

/// The remainder algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `rem_` function
/// prefix (`rem_int_layer` → `IntLayer`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`rem_native`](crate::algos::rem::rem_native::rem_native) — primitive
    /// `%` on the narrow storage value (`i64 %` at `N == 1`, `i128 %` at
    /// `N == 2`), the same-SCALE remainder needing no rescaling. Valid for
    /// `N <= 2`; **routed at `N == 1` (D18) only**, where the `%` really is
    /// one `idiv` and skipping [`Self::IntLayer`]'s unpack / compare / repack
    /// pays for itself.
    ///
    /// Kept but UNROUTED at `N == 2`: there the `%` is an `i128` soft-call,
    /// not an instruction, and it loses to [`Self::IntLayer`] at every scale
    /// and operand class measured. `rem_native`'s header carries the numbers.
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
/// `SCALE`. Total over the key: `Native` at `N == 1`, `IntLayer` at every
/// `N >= 2`. The choice does not vary with `SCALE`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    // `N == 1` (D18) is the ONLY width whose remainder is a hardware
    // instruction: the storage is one `i64`, so `rem_native` lowers to a
    // single `idiv`, and skipping the int-layer unpack-to-magnitude /
    // compare / repack is worth more than that setup costs.
    //
    // `N >= 2` keeps `IntLayer`. At `N == 2` (D38) `rem_native` is an
    // `i128 %`, and x86-64 has NO 128-bit divide instruction (`div r/m64`
    // is 128÷64→64 and traps when the quotient overflows), so it lowers to
    // the `__modti3` / `__udivmodti4` soft-call -- NOT a primitive `%`.
    // `rem_int_layer` reaches that same soft-call at worst, and skips the
    // divide entirely whenever `|a| < |b|`. See `rem_native`'s header for
    // the measurement and for why the division side of the crate
    // (`div_native`) already routes around the identical soft-call.
    match N {
        1 => Select::ByAlgorithm(Algorithm::Native),
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
/// Not `const fn`: matches the existing non-`const` `Rem` operator on
/// `D<Int<N>, SCALE>`.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    dividend: Int<N>,
    divisor: Int<N>
) -> Int<N>
where
    crate::int::types::compute_limbs::Limbs<N>: crate::int::types::compute_limbs::ComputeLimbs,
{
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(_) => Algorithm::IntLayer,
    };
    match algo {
        Algorithm::Native => crate::algos::rem::rem_native::rem_native(dividend, divisor),
        Algorithm::IntLayer | Algorithm::Schoolbook => {
            crate::algos::rem::rem_int_layer::rem_int_layer(dividend, divisor)
        }
    }
}

// ── per-type `RemPolicy` trait ────────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for `%`.
pub(crate) trait RemPolicy: Sized {
    /// Remainder of `self % rhs`, panicking on the `MIN % -ONE` overflow and
    /// on a zero divisor in both debug and release.
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
