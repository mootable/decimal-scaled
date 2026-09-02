// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `sqrt_newton` — Newton integer square root of `raw · 10^SCALE`, with a
//! single round step, computed directly over `u64` limbs.
//!
//! For a `D<Int<N>, SCALE>` value with raw storage `r`, the logical value
//! is `r / 10^SCALE`, so the square-root raw storage is
//! `round(sqrt(r · 10^SCALE))`. The radicand `|r| · 10^SCALE` is formed in
//! a local limb scratch buffer (it spans up to `2N` limbs), the exact integer square root is
//! taken via the int layer's width-agnostic slice kernel
//! ([`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`]), and a single
//! round-to-nearest step lands the result on the type's last representable
//! place. Within 0.5 ULP under any of the six rounding modes.
//!
//! # Generic over the storage width only
//!
//! The kernel is generic over the storage limb count `N` and does the
//! work-width arithmetic in limbs — there is **no** `W = Int<2N>` work
//! *type* (which stable Rust cannot name from `N`), and therefore no
//! per-tier work-width binding in the policy. The integer work dispatches
//! *down* to the int layer's slice kernels: `isqrt_newton` for the root and
//! the multiply matcher's slice door
//! [`crate::int::policy::mul::dispatch_slice`] for the products (so the
//! schoolbook-vs-Karatsuba choice is the matcher's, not hardcoded), with the
//! `limbs` primitives for the rest.
//!
//! Returns `Int::<N>::ZERO` for `raw <= 0` (saturate-not-panic).

use crate::int::policy::mul::dispatch_slice as mul_slice;
use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
use crate::int::algos::support::limbs::{cmp_cross, is_zero, sub_assign};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Significant limb length of `limbs` (index of the highest non-zero limb
/// + 1), clamped to at least 1 so zero has length 1.
#[inline]
fn sig_len(limbs: &[u64]) -> usize {
    let mut len = limbs.len();
    while len > 1 && limbs[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// Newton integer square-root kernel, computed in limbs.
///
/// `N` is the storage limb count backing `D<Int<N>, SCALE>`. The radicand
/// `|raw| · 10^scale` is built in scratch and rooted via the int layer's
/// `isqrt_newton`; the result is rounded and returned as `Int<N>`.
#[inline]
#[must_use]
pub(crate) fn sqrt_newton<const N: usize>(raw: Int<N>, scale: u32, mode: RoundingMode) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if raw <= Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }

    // ── radicand = |raw| · 10^scale, in limb scratch ────────────────────
    let mut radicand_buf = Limbs::<N>::double_buffered_u64();
    let radicand = radicand_buf.as_mut();
    radicand[..N].copy_from_slice(raw.unsigned_abs().as_limbs());
    let mut radicand_len = sig_len(&radicand[..N]);
    {
        let mut product_buf = Limbs::<N>::double_buffered_u64();
        let product = product_buf.as_mut();
        for _ in 0..scale {
            let product_len = radicand_len + 1;
            for limb in product[..product_len].iter_mut() {
                *limb = 0;
            }
            mul_slice(&radicand[..radicand_len], &[10u64], &mut product[..product_len]);
            radicand[..product_len].copy_from_slice(&product[..product_len]);
            radicand_len = sig_len(&radicand[..product_len]);
        }
    }

    // ── root = floor(sqrt(radicand)) via the int slice kernel ───────────
    let mut root_buf = Limbs::<N>::double_buffered_u64();
    let root = root_buf.as_mut();
    isqrt_newton(&radicand[..radicand_len], &mut root[..radicand_len]);
    let root_len = sig_len(&root[..radicand_len]);

    // ── diff = radicand - root²  (root² ≤ radicand, so diff fits in
    //    radicand_len limbs) ──────────────────────────────────────────────
    let mut root_sq_buf = Limbs::<N>::double_buffered_u64();
    let root_sq = root_sq_buf.as_mut();
    let root_sq_cap = root_sq.len();
    mul_slice(&root[..root_len], &root[..root_len], &mut root_sq[..(2 * root_len).min(root_sq_cap)]);
    let mut diff_buf = Limbs::<N>::double_buffered_u64();
    let diff = diff_buf.as_mut();
    diff[..radicand_len].copy_from_slice(&radicand[..radicand_len]);
    sub_assign(&mut diff[..radicand_len], &root_sq[..radicand_len]);

    // ── single round step (matches the BigInt-generic kernel exactly) ───
    // halfway_round_up: remainder past the lower root exceeds the root
    // (diff > root); diff_nonzero: any remainder at all.
    let halfway_round_up = cmp_cross(&diff[..radicand_len], &root[..root_len]) > 0;
    let diff_nonzero = !is_zero(&diff[..radicand_len]);
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        // The radicand is non-negative, so up IS away from zero.
        RoundingMode::Ceiling | RoundingMode::AwayFromZero => diff_nonzero,
        // The last decimal digit spans the whole `root_len`-limb root.
        RoundingMode::ZeroFiveUp => {
            diff_nonzero
                && matches!(crate::support::rounding::limbs_mod_10(&root[..root_len]), 0 | 5)
        }
    };
    if bump {
        // root += 1 (carry stays within root_len+1 limbs).
        let mut i = 0;
        loop {
            let (sum, carry) = root[i].overflowing_add(1);
            root[i] = sum;
            if !carry {
                break;
            }
            i += 1;
        }
    }

    // ── narrow the root to Int<N> (positive; fits by construction) ──────
    let mut root_limbs = [0u64; N];
    root_limbs.copy_from_slice(&root[..N]);
    Int::<N>::from_limbs(root_limbs)
}
