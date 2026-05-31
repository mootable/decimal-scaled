// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pow10` policy — `10^exp` is **TABLE-FIRST**.
//!
//! Producing `10^exp` is a one-op "constant generation" with two real
//! algorithms, so it gets the canonical policy shape
//! (`docs/ARCHITECTURE.md` → "Policy file structure"):
//!
//! * [`Algorithm::Table`] — the DEFAULT: a baked-`static` lookup from the
//!   generated POW10 table ([`super::table::pow10_limbs`]), zero-extended
//!   into the work integer. A `&'static [u64]` read, no arithmetic.
//! * [`Algorithm::Function`] — the out-of-range FALLBACK: the
//!   square-and-multiply `W::TEN.pow(exp)`, used ONLY when `exp` is beyond
//!   the (feature-gated) baked range.
//!
//! [`select`] picks Table whenever the entry exists, else Function. It is a
//! `const fn` keyed only on `exp`, so for a **const** `exp` the whole verdict
//! const-folds to one arm with no runtime branch (the policy is free); for a
//! **runtime** `exp` it is one in-range check.
//!
//! Two doors over the same policy (the const/slice dual-door idea, here
//! const-`EXP` vs runtime-`exp`):
//!
//! * [`dispatch`] — generic over the work integer `W: BigInt`, `#[inline]`.
//!   A const-known `exp` folds to the baked entry (Table) or a const
//!   `TEN.pow` (Function); a runtime `exp` branches on the in-range check.
//!   The everyday door for kernels that hold a `W`.
//! * [`dispatch_int`] — a `const fn` returning `Int<N>`, for the const-`EXP`
//!   sites that previously wrote `const { Int::<N>::TEN.pow(EXP) }`. Those
//!   baked the value via the *Function* (square-and-multiply) at compile
//!   time; routing them here makes the value **table-sourced** instead, while
//!   still folding to a compile-time constant.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;

/// The two `10^exp` algorithms — Table (the baked lookup, default) and the
/// square-and-multiply Function (out-of-range fallback). No `Default` variant.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Table,
    Function,
}

/// Pick the algorithm for `10^exp`: Table when the baked entry exists,
/// else Function. `const fn`, so a const `exp` folds the verdict.
#[inline]
const fn select(exp: u32) -> Algorithm {
    match super::table::pow10_limbs(exp) {
        Some(_) => Algorithm::Table,
        None => Algorithm::Function,
    }
}

/// `10^exp` in the work integer `W`, **table-first**. `#[inline]`: a
/// const-known `exp` folds to the baked entry or a const `W::TEN.pow`; a
/// runtime `exp` does the in-range check. The generic (`W: BigInt`) door —
/// what kernels holding a work integer call (replaces the old `pow10_in`).
#[inline]
pub(crate) fn dispatch<W: BigInt>(exp: u32) -> W {
    match select(exp) {
        // `select` returned Table, so the entry is present.
        Algorithm::Table => {
            super::table::limbs_to_w::<W>(super::table::pow10_limbs(exp).unwrap())
        }
        Algorithm::Function => W::TEN.pow(exp),
    }
}

/// `10^exp` as `Int<N>`, **table-first**, in a `const fn` — the const-`EXP`
/// door. Replaces `const { Int::<N>::TEN.pow(EXP) }` so the value is sourced
/// from the baked table (Table) rather than recomputed by square-and-multiply
/// (Function) at compile time; out-of-range still folds via the const
/// `Int::<N>::TEN.pow`. Zero-extends the narrowest-fit table limbs into
/// `[u64; N]` (the entry never exceeds `N` for an in-range `10^exp` that fits
/// `Int<N>`).
#[inline]
pub(crate) const fn dispatch_int<const N: usize>(exp: u32) -> Int<N> {
    match super::table::pow10_limbs(exp) {
        Some(limbs) => {
            let mut arr = [0u64; N];
            let mut i = 0;
            while i < limbs.len() {
                arr[i] = limbs[i];
                i += 1;
            }
            Int::<N>::from_limbs(arr)
        }
        None => Int::<N>::TEN.pow(exp),
    }
}

/// `10^exp` as `i128`, **table-first** — the narrow-`i128` door for the
/// D18/D38 hardware paths. Sources the value from the baked table (its low
/// one or two u64 limbs) when in range, else the const `10i128.pow(exp)`.
/// `const fn`, so a const `exp` folds. Valid only where `10^exp` fits `i128`
/// (`exp <= 38`), which every narrow caller guarantees; a larger `exp`
/// overflows exactly as the original `10i128.pow(exp)` did.
#[inline]
pub(crate) const fn dispatch_i128(exp: u32) -> i128 {
    match super::table::pow10_limbs(exp) {
        Some(limbs) if limbs.len() <= 2 => {
            let lo = limbs[0] as u128;
            let hi = if limbs.len() == 2 { limbs[1] as u128 } else { 0 };
            (lo | (hi << 64)) as i128
        }
        // Beyond `i128` range (or beyond the table): the square-and-multiply
        // Function — overflows identically to the prior `10i128.pow(exp)`.
        _ => 10i128.pow(exp),
    }
}
