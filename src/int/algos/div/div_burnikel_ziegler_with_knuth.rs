// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Burnikel–Ziegler outer chunking, recursing to Knuth as its base case.
//!
//! [`div_burnikel_ziegler_with_knuth`] — Burnikel–Ziegler outer chunking
//! that recurses to [`crate::int::algos::div::div_knuth::div_knuth`] as its
//! base case (hence the `_with_` hybrid name). The threshold above which
//! this engine is chosen lives in [`crate::int::policy::div_rem`].

use crate::int::algos::div::div_knuth::div_knuth;
use crate::int::algos::div::SCRATCH_LIMBS;

/// Burnikel–Ziegler outer chunking, u64 base, recursing to [`div_knuth`]
/// as the base case.
pub(crate) fn div_burnikel_ziegler_with_knuth(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
) {
    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "div_burnikel_ziegler_with_knuth: divide by zero");

    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }

    if n < crate::int::policy::div_rem::BZ_THRESHOLD || top < 2 * n {
        div_knuth(num, den, quot, rem);
        return;
    }

    bz_chunk_core(num, den, quot, rem, n, top);
}

/// The chunking core — divides the already-stripped `num` (effective
/// `top` limbs) by `den` (effective `n` limbs) by splitting the dividend
/// into `n`-limb blocks and Knuth-dividing each (block ‖ running carry)
/// by the divisor. Callers pass the stripped effective shape `(n, top)`;
/// the public entry above applies the engagement guard first.
///
/// Split out so a bench seam can drive it below the production
/// engagement threshold (where the public entry would otherwise short to
/// Knuth), without an engagement branch in the timed path.
pub(crate) fn bz_chunk_core(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
    n: usize,
    top: usize,
) {
    for q in quot.iter_mut() {
        *q = 0;
    }
    for r in rem.iter_mut() {
        *r = 0;
    }

    let chunks = top.div_ceil(n);
    let mut carry = [0u64; SCRATCH_LIMBS];
    let mut buf = [0u64; SCRATCH_LIMBS];
    let mut q_chunk = [0u64; SCRATCH_LIMBS];
    let mut r_chunk = [0u64; SCRATCH_LIMBS];

    let mut idx = chunks;
    while idx > 0 {
        idx -= 1;
        let lo = idx * n;
        let hi = ((idx + 1) * n).min(top);
        buf.fill(0);
        let chunk_len = hi - lo;
        buf[..chunk_len].copy_from_slice(&num[lo..lo + chunk_len]);
        buf[chunk_len..chunk_len + n].copy_from_slice(&carry[..n]);
        let buf_len = chunk_len + n;
        div_knuth(
            &buf[..buf_len],
            &den[..n],
            &mut q_chunk[..buf_len],
            &mut r_chunk[..n],
        );
        let store_end = (lo + n).min(quot.len());
        let store_len = store_end.saturating_sub(lo);
        quot[lo..lo + store_len].copy_from_slice(&q_chunk[..store_len]);
        carry[..n].copy_from_slice(&r_chunk[..n]);
    }
    let rem_n = n.min(rem.len());
    rem[..rem_n].copy_from_slice(&carry[..rem_n]);
}

/// Forced chunking entry for the crossover microbench: strips the operand
/// shapes then runs [`bz_chunk_core`] **unconditionally**, ignoring the
/// production engagement guard, so the Knuth-vs-BZ crossover can be timed
/// at sub-threshold widths. Not used in production routing.
#[cfg(feature = "bench-alt")]
pub(crate) fn bz_chunk_core_forced(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
) {
    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "bz_chunk_core_forced: divide by zero");
    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }
    bz_chunk_core(num, den, quot, rem, n, top);
}
