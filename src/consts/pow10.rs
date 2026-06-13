// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `pow10` — `10^exp` in a work integer, **TABLE-FIRST**.
//!
//! Producing `10^exp` is a one-op "constant generation": the baked POW10
//! table ([`super::table::pow10_limbs`]) holds every in-range power as
//! `&'static [u64]` limbs; beyond the (feature-gated) baked range the
//! square-and-multiply `W::TEN.pow(exp)` is the fallback. The only decision
//! is "is `exp` in the baked range", which the `Option` from a single table
//! lookup already answers — so each door **matches that lookup directly**,
//! with no `select`/`Algorithm` pre-lookup that would only re-fetch the same
//! entry. `const fn`, so a const `exp` folds to the entry and a runtime `exp`
//! is one in-range branch then the single read.
//!
//! Three doors over the one table, by result type:
//!
//! * [`dispatch`] — generic over the work integer `W: BigInt`, `#[inline]`.
//!   The everyday door for kernels that hold a `W`: a const-known `exp` folds
//!   to the baked entry (zero-extended into `W`) or a const `TEN.pow`; a
//!   runtime `exp` is one in-range branch then the single table read.
//! * [`dispatch_int`] — a `const fn` returning `Int<N>`, for the const-`EXP`
//!   sites needing a `const { Int::<N>::TEN.pow(EXP) }` value:
//!   **table-sourced** while still folding to a compile-time constant.
//! * [`dispatch_i128`] — the narrow-`i128` door for the D18/D38 hardware
//!   paths (the value's low one or two limbs).

use crate::int::types::traits::BigInt;
use crate::int::types::Int;

/// `10^exp` in the work integer `W`, **table-first**. `#[inline]`: a
/// const-known `exp` folds to the baked entry or a const `W::TEN.pow`; a
/// runtime `exp` is one in-range branch then the single table read. The
/// generic (`W: BigInt`) door — what kernels holding a work integer call.
///
/// Matches the baked entry directly — no `select` pre-lookup that would
/// re-fetch the same entry — mirroring [`dispatch_int`] / [`dispatch_i128`].
#[inline]
pub(crate) fn dispatch<W: BigInt>(exp: u32) -> W {
    match super::table::pow10_limbs(exp) {
        // Baked entry present (in range): zero-extend the limbs into `W`.
        Some(limbs) => super::table::limbs_to_w::<W>(limbs),
        // Out of the baked range: square-and-multiply fallback.
        None => W::TEN.pow(exp),
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
