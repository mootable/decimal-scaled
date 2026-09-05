// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Burnikel–Ziegler recursive fast-division, base-casing to Knuth.
//!
//! [`div_burnikel_ziegler_with_knuth`] — the recursive divide-and-conquer
//! division of Burnikel & Ziegler, "Fast Recursive Division" (MPI-I-98-1-022,
//! 1998). The two mutually-recursive halves `D_{2n/1n}` ([`div_2n_1n`]) and
//! `D_{3n/2n}` ([`div_3n_2n`]) each halve the divisor width per round-trip
//! until they reach the Knuth base case
//! ([`crate::int::algos::div::div_knuth::div_knuth`]) — hence the `_with_`
//! hybrid name. The threshold above which this engine is chosen lives in
//! [`crate::int::policy::div_rem`].
//!
//! The asymptotic edge of the recursion is that each `D_{3n/2n}` step does its
//! back-correction with ONE multiply (`Q̂ · B_lo`) rather than Knuth's per-limb
//! multiply-subtract sweep, so a sub-quadratic multiply lifts the whole divide
//! below `O(n²)`. The back-multiply here routes to the int multiply layer's
//! slice kernel ([`mul_schoolbook`]); the recursion stays pure
//! divide-and-conquer regardless, base-casing to Knuth at
//! [`BZ_BASECASE`]-and-below or any odd width.

use crate::int::algos::div::div_knuth::div_knuth;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{add_assign, cmp, sub_assign};
use crate::int::types::compute_limbs::MAX_QUADRUPLE_LIMBS;

/// Recursion base-case width, in u64 limbs: a `D_{2n/1n}` sub-problem whose
/// divisor is at most this many limbs — or any odd-width divisor the half-split
/// can't cleanly halve — bottoms out on a single [`div_knuth`] pass instead of
/// recursing. This is the non-recursive floor of the strictly-decreasing `n`
/// measure (see the recursion guard in [`div_2n_1n`]). Benched against Knuth on
/// the wide `2n`/`n` shape (`div_kernel_ab`): the recursion's per-step overhead
/// (two sub-divides + one back-multiply + correction loop) only begins to pay
/// off once the divisor is wide enough that the avoided `O(n²)` Knuth
/// multiply-subtract dominates — which, with the schoolbook back-multiply
/// (Karatsuba engages only at `N ≥ 128`), is at the high end of the working
/// widths BZ is routed at, if at all. A base of `16` keeps the recursion engaged
/// for any divisor wide enough to reach BZ so the kernel IS the true recursion
/// (not a single Knuth pass in disguise).
const BZ_BASECASE: usize = 16;

/// Build-max scratch width for the recursive buffers, in u64 limbs.
///
/// This is the width-erased slice engine's build-max blanket: BZ is reached
/// through the slice [`crate::int::policy::div_rem::dispatch`] (not a
/// concrete-`N` caller), so it takes bare runtime-length `&[u64]` — there is no
/// `N` to size against. The divide it must handle is NOT bounded by the storage
/// tiers: the decimal `÷10^w` rescale and the wide-transcendental slice roots
/// (`isqrt_newton` / `icbrt_newton` / `barrett_reciprocal`) present **working**
/// numerators that exceed the widest storage width — at the supported build the
/// effective `divisor_len` reaches ~67 limbs (≥ `BZ_THRESHOLD`, so BZ DOES
/// engage there) over a `~2·divisor_len` ≈ 134-limb dividend. Every recursive
/// sub-buffer (`u`, the `2n` block dividend, the `3s` sub-dividend, the `2s+2`
/// correction window) is bounded by that widest working dividend, which
/// [`MAX_QUADRUPLE_LIMBS`] (`4·MAX_WORK_N + ⌈MAX_WORK_N/2⌉`, the build-max
/// `quad_buffered` blanket — `288` at xx-wide) covers with margin.
///
/// Using the `MAX_QUADRUPLE_LIMBS` blanket here is architectural-review
/// **Class-B-SANCTIONED**: this is one of the structurally-`N`-less slice-divide
/// engines (`docs/ARCHITECTURE.md` → "Work-width scratch", the width-erased
/// slice-divide engines), so the `ComputeLimbs` build-max blanket is the
/// correct sizing — exactly the `max_quadruple_limbs()` family the chunking
/// predecessor used. A frozen literal or a `MAX_WORK_N`-derived storage-width
/// bound (the original undersizing defect) is forbidden.
const BZ_MAX: usize = MAX_QUADRUPLE_LIMBS;

/// Burnikel–Ziegler recursive fast-division entry. Strips the operand shapes,
/// applies the engagement guard (the same
/// `n < BZ_THRESHOLD || dividend_len < 2·n` short-circuit to Knuth the policy
/// threshold encodes), then runs the recursion. The chosen-engine `match` in
/// [`crate::int::policy::div_rem::dispatch`] only reaches here for a divisor of
/// at least `BZ_THRESHOLD` effective limbs whose dividend is at least twice as
/// wide.
pub(crate) fn div_burnikel_ziegler_with_knuth(
    dividend: &[u64],
    divisor: &[u64],
    quotient: &mut [u64],
    remainder: &mut [u64],
) {
    let mut n = divisor.len();
    while n > 0 && divisor[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "div_burnikel_ziegler_with_knuth: divide by zero");

    let mut dividend_len = dividend.len();
    while dividend_len > 0 && dividend[dividend_len - 1] == 0 {
        dividend_len -= 1;
    }

    if n < crate::int::policy::div_rem::BZ_THRESHOLD || dividend_len < 2 * n {
        div_knuth(dividend, divisor, quotient, remainder);
        return;
    }

    bz_recursive_core(dividend, divisor, quotient, remainder, n, dividend_len);
}

/// The recursive core. Callers pass the stripped effective shape
/// `(n, dividend_len)`; the public entry above applies the engagement guard
/// first.
///
/// Normalises the divisor to a full top limb (Knuth's shift), runs the
/// block-recursive divide ([`div_blocks`]) so the dividend is consumed in
/// `n`-limb blocks each handled by the recursive [`div_2n_1n`], then
/// de-normalises the remainder. Split out so a bench seam can drive it below
/// the production engagement threshold without an engagement branch in the
/// timed path.
pub(crate) fn bz_recursive_core(
    dividend: &[u64],
    divisor: &[u64],
    quotient: &mut [u64],
    remainder: &mut [u64],
    n: usize,
    dividend_len: usize,
) {
    for slot in quotient.iter_mut() {
        *slot = 0;
    }
    for slot in remainder.iter_mut() {
        *slot = 0;
    }

    // Normalise so the divisor's top limb has its high bit set (Knuth's
    // shift); the recursion's `D_{3n/2n}` quotient estimate relies on a
    // normalised divisor exactly as Knuth does.
    let shift = divisor[n - 1].leading_zeros();

    let mut v = [0u64; BZ_MAX];
    let mut u = [0u64; BZ_MAX + 2];
    debug_assert!(n <= BZ_MAX && dividend_len + 1 < BZ_MAX + 2);

    if shift == 0 {
        v[..n].copy_from_slice(&divisor[..n]);
        u[..dividend_len].copy_from_slice(&dividend[..dividend_len]);
    } else {
        let mut carry: u64 = 0;
        for i in 0..n {
            let limb = divisor[i];
            v[i] = (limb << shift) | carry;
            carry = limb >> (64 - shift);
        }
        carry = 0;
        for i in 0..dividend_len {
            let limb = dividend[i];
            u[i] = (limb << shift) | carry;
            carry = limb >> (64 - shift);
        }
        u[dividend_len] = carry;
    }
    // The normalised dividend occupies `dividend_len + 1` limbs (the extra
    // carry limb).
    let u_len = dividend_len + 1;

    // Block-recursive divide of the normalised `u` by the normalised `v[..n]`.
    let mut r_norm = [0u64; BZ_MAX];
    div_blocks(&u[..u_len], &v[..n], n, quotient, &mut r_norm[..n]);

    // De-normalise the remainder (shift right by `shift`).
    if shift == 0 {
        let copy_len = n.min(remainder.len());
        remainder[..copy_len].copy_from_slice(&r_norm[..copy_len]);
    } else {
        for i in 0..n {
            if i < remainder.len() {
                let lo = r_norm[i] >> shift;
                let hi_into_lo = if i + 1 < n {
                    r_norm[i + 1] << (64 - shift)
                } else {
                    0
                };
                remainder[i] = lo | hi_into_lo;
            }
        }
    }
}

/// Block-recursive driver: divide a (normalised) dividend `u` by a
/// (normalised) `n`-limb divisor `v`, producing the full quotient in
/// `quotient` and the `n`-limb remainder in `remainder`.
///
/// Burnikel–Ziegler's outer loop: walk the dividend in `n`-limb blocks from the
/// most-significant end, maintaining a `running_remainder` of `n` limbs
/// (`< v`). Each block forms the `2n`-limb dividend `running_remainder ‖ block`
/// and runs ONE [`div_2n_1n`] (the recursive `D_{2n/1n}`), whose `n`-limb
/// quotient is that block's quotient digits and whose `n`-limb remainder
/// carries into the next block. For the `2n`/`n` `div` shape there is exactly
/// one block beyond the initial zero `running_remainder`; a wider dividend
/// generalises to `⌈u_len/n⌉` blocks.
fn div_blocks(u: &[u64], v: &[u64], n: usize, quotient: &mut [u64], remainder: &mut [u64]) {
    let u_len = u.len();
    let blocks = u_len.div_ceil(n);

    let mut running_remainder = [0u64; BZ_MAX]; // running n-limb remainder (< v)
    let mut dividend = [0u64; BZ_MAX]; // running_remainder ‖ block, 2n limbs
    let mut q_block = [0u64; BZ_MAX]; // n-limb quotient digits

    let mut idx = blocks;
    while idx > 0 {
        idx -= 1;
        let block_start = idx * n;
        let block_end = ((idx + 1) * n).min(u_len);
        let block_len = block_end - block_start;

        // Build the `2n`-limb dividend = running_remainder·2^(64n) + block. The
        // low `block_len` limbs are the block; limbs `[block_len, n)` (if any)
        // are zero; the high `n` limbs are `running_remainder`.
        dividend[..block_len]
            .copy_from_slice(&u[block_start..block_start + block_len]);
        for slot in dividend[block_len..n].iter_mut() {
            *slot = 0;
        }
        dividend[n..2 * n].copy_from_slice(&running_remainder[..n]);

        div_2n_1n(&dividend[..2 * n], v, n, &mut q_block[..n],
            &mut running_remainder[..n]);

        let store_end = (block_start + n).min(quotient.len());
        let store_len = store_end.saturating_sub(block_start);
        if store_len > 0 {
            quotient[block_start..block_start + store_len]
                .copy_from_slice(&q_block[..store_len]);
        }
    }

    let copy_len = n.min(remainder.len());
    remainder[..copy_len].copy_from_slice(&running_remainder[..copy_len]);
}

/// `D_{2n/1n}` — divide a `2n`-limb dividend `a` by an `n`-limb (normalised)
/// divisor `b`, writing the `n`-limb quotient to `q` and the `n`-limb
/// remainder to `r`. Precondition: `a < b · 2^(64n)` (the quotient fits `n`
/// limbs), guaranteed by the block driver seeding each block with the previous
/// `< b` remainder.
///
/// **Recursion guard (Constitution runaway-recursion rule).** The
/// strictly-decreasing well-founded measure is the divisor limb count `n`: the
/// recursion descends `n → s = n/2` via two [`div_3n_2n`] calls (each of which
/// recurses back through `D_{2n/1n}` at width `s < n`), and bottoms out —
/// non-recursively — on a single [`div_knuth`] pass once `n <= BZ_BASECASE` or
/// `n` is odd (so `n/2` would not cleanly tile). `n` strictly decreases and
/// cannot grow, so the descent terminates; `div_knuth` never re-enters this
/// engine.
fn div_2n_1n(a: &[u64], b: &[u64], n: usize, q: &mut [u64], r: &mut [u64]) {
    // Base case: a single Knuth pass. Reached at the floor width or any odd
    // width the half-split can't tile. div_knuth zeroes q/r itself.
    if n <= BZ_BASECASE || n % 2 == 1 {
        div_knuth(a, b, q, r);
        return;
    }

    let s = n / 2; // half-width

    // a (little-endian) = a0 (low s) ‖ a1 ‖ a2 ‖ a3 (high s). The high `3s`
    // limbs (a1‖a2‖a3) form the first `D_{3n/2n}` dividend; a0 the second.
    let mut q1 = [0u64; BZ_MAX]; // s-limb high quotient
    let mut r1 = [0u64; BZ_MAX]; // 2s = n-limb intermediate remainder

    // First half: (a's high 3s limbs) / b → q1 (s limbs), r1 (2s = n limbs).
    div_3n_2n(&a[s..4 * s], b, s, &mut q1[..s], &mut r1[..2 * s]);

    // Second half: (r1 ‖ a0) / b → q0 written straight into q's low half,
    // remainder into r.
    let mut dividend2 = [0u64; BZ_MAX];
    dividend2[..s].copy_from_slice(&a[..s]); // a0 (low s)
    dividend2[s..3 * s].copy_from_slice(&r1[..2 * s]); // r1 (high 2s)
    div_3n_2n(&dividend2[..3 * s], b, s, &mut q[..s], r);

    // Quotient high half = q1.
    q[s..2 * s].copy_from_slice(&q1[..s]);
}

/// `D_{3n/2n}` — divide a `3s`-limb dividend `a` by a `2s`-limb (normalised)
/// divisor `b`, writing the `s`-limb quotient to `q` and the `2s`-limb
/// remainder to `r`. Precondition `a < b · 2^(64s)` (the quotient fits `s`
/// limbs).
///
/// Burnikel–Ziegler's `D_{3n/2n}` (their §3): estimate the quotient from the
/// top `2s`/`s` division (recursing through [`div_2n_1n`]), then a single
/// `Q̂ · B_lo` back-multiply and at most two `+B` corrections. This is the step
/// that trades Knuth's per-limb multiply-subtract for one multiply.
fn div_3n_2n(a: &[u64], b: &[u64], s: usize, q: &mut [u64], r: &mut [u64]) {
    // b = b_lo (low s) ‖ b_hi (high s).
    let b_lo = &b[..s];
    let b_hi = &b[s..2 * s];
    // a (little-endian) = a3 (low s) ‖ a2 (mid s) ‖ a1 (high s).
    let a3 = &a[..s]; // low s limbs
    let a_top2 = &a[s..3 * s]; // high 2s limbs (a2‖a1)
    let a1 = &a[2 * s..3 * s]; // highest s limbs

    let mut q_hat = [0u64; BZ_MAX]; // s-limb quotient estimate
    // r1 = remainder of (a_top2 / b_hi), an s-limb value (< b_hi); a spare
    // high limb holds the saturate branch's carry.
    let mut r1 = [0u64; BZ_MAX];

    if cmp(a1, b_hi) >= 0 {
        // Saturate: q_hat = 2^(64s) - 1 (all ones). The D_{2n/1n} precondition
        // a < b·2^(64s) gives a1 <= b_hi, so `>=` means a1 == b_hi and
        //   r1 = a_top2 - q_hat·b_hi = a_top2 + b_hi - b_hi·2^(64s) = a2 + b_hi
        // (an s-limb value plus a possible carry limb).
        for slot in q_hat[..s].iter_mut() {
            *slot = u64::MAX;
        }
        r1[..s].copy_from_slice(&a[s..2 * s]); // a2 = a_top2's low s limbs
        if add_assign(&mut r1[..s], b_hi) {
            r1[s] = 1;
        }
    } else {
        // (a_top2) / b_hi → q_hat (s), r1 (s).
        div_2n_1n(a_top2, b_hi, s, &mut q_hat[..s], &mut r1[..s]);
    }

    // D = q_hat · b_lo (s × s → up to 2s limbs).
    let mut d = [0u64; BZ_MAX];
    mul_schoolbook(&q_hat[..s], b_lo, &mut d[..2 * s]);

    // R = r1·2^(64s) + a3 - D. r1 is an s-limb value (+1 carry limb in the
    // saturate branch), so r1·2^(64s)+a3 occupies up to 2s+1 limbs. Assemble
    // in a 2s+2-limb window (the +2 holds the transient borrow during the
    // correction) and subtract D (2s limbs).
    let mut rr = [0u64; BZ_MAX];
    rr[..s].copy_from_slice(a3); // low s limbs = a3
    rr[s..2 * s + 1].copy_from_slice(&r1[..s + 1]); // high limbs = r1 (+carry)
    let _ = sub_assign(&mut rr[..2 * s + 2], &d[..2 * s]);

    // Correction: while R < 0 (a borrow wrapped the limbs above the 2s window
    // to all-ones), Q̂ -= 1 and R += B (2s limbs). Burnikel–Ziegler bound: at
    // most 2 iterations.
    let mut corrections = 0usize;
    while rr[2 * s] != 0 || rr[2 * s + 1] != 0 {
        let _ = sub_assign(&mut q_hat[..s], &[1]); // Q̂ -= 1
        let _ = add_assign(&mut rr[..2 * s + 2], &b[..2 * s]); // R += B
        corrections += 1;
        debug_assert!(
            corrections <= 2,
            "div_3n_2n: more than 2 BZ corrections (s={s})"
        );
        if corrections > 2 {
            break;
        }
    }

    q[..s].copy_from_slice(&q_hat[..s]);
    r[..2 * s].copy_from_slice(&rr[..2 * s]);
}

/// Forced recursive entry for the crossover microbench: strips the operand
/// shapes then runs [`bz_recursive_core`] **unconditionally**, ignoring the
/// production engagement guard, so the Knuth-vs-BZ crossover can be timed at
/// sub-threshold widths. Not used in production routing.
#[cfg(feature = "bench-alt")]
pub(crate) fn bz_chunk_core_forced(dividend: &[u64], divisor: &[u64], quotient: &mut [u64],
    remainder: &mut [u64]) {
    let mut n = divisor.len();
    while n > 0 && divisor[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "bz_chunk_core_forced: divide by zero");
    let mut dividend_len = dividend.len();
    while dividend_len > 0 && dividend[dividend_len - 1] == 0 {
        dividend_len -= 1;
    }
    bz_recursive_core(dividend, divisor, quotient, remainder, n, dividend_len);
}

// The differentials below all drive the recursion at the widest WORKING widths
// (`divisor_len` up to ~69), so they require the xx-wide divide
// scratch and are gated on it — gating the whole module keeps the narrow default
// build free of unused-import / dead-helper warnings.
#[cfg(all(test, feature = "xx-wide"))]
mod tests {
    use super::{bz_recursive_core, div_burnikel_ziegler_with_knuth};
    use crate::int::algos::div::div_knuth::div_knuth;

    // A small deterministic PRNG so the differential is reproducible without a
    // dependency. xorshift64.
    fn rng(seed: u64) -> impl FnMut() -> u64 {
        let mut state = seed | 1;
        move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        }
    }

    // Build a `(dividend, divisor)` pair: `divisor` exactly `divisor_len`
    // nonzero-top limbs, `dividend` exactly `dividend_len` limbs (top limb
    // forced nonzero). The recursion + Knuth both strip leading zeros, so the
    // EFFECTIVE shape is `(dividend_len, divisor_len)`.
    fn make(next: &mut impl FnMut() -> u64, dividend_len: usize, divisor_len: usize)
        -> (Vec<u64>, Vec<u64>) {
        let mut dividend = vec![0u64; dividend_len];
        let mut divisor = vec![0u64; divisor_len];
        for slot in dividend.iter_mut() {
            *slot = next();
        }
        for slot in divisor.iter_mut() {
            *slot = next();
        }
        dividend[dividend_len - 1] |= 0x8000_0000_0000_0000;
        divisor[divisor_len - 1] |= 0x8000_0000_0000_0000;
        (dividend, divisor)
    }

    // Drive the forced recursive core and assert bit-identity with the Knuth
    // reference (the exact-quotient oracle for an integer divide).
    fn assert_recursive_matches_knuth(dividend: &[u64], divisor: &[u64], label: &str) {
        let len = dividend.len();
        let mut quotient_reference = vec![0u64; len + 1];
        let mut remainder_reference = vec![0u64; len + 1];
        div_knuth(dividend, divisor, &mut quotient_reference, &mut remainder_reference);

        // Strip the divisor / dividend to their effective lengths, then run the
        // recursive core directly (bypasses the engagement guard so the
        // recursion is exercised at every width, not only routed ones).
        let mut divisor_len = divisor.len();
        while divisor_len > 0 && divisor[divisor_len - 1] == 0 {
            divisor_len -= 1;
        }
        let mut dividend_len = dividend.len();
        while dividend_len > 0 && dividend[dividend_len - 1] == 0 {
            dividend_len -= 1;
        }
        let mut quotient_bz = vec![0u64; len + 1];
        let mut remainder_bz = vec![0u64; len + 1];
        bz_recursive_core(dividend, divisor, &mut quotient_bz, &mut remainder_bz,
            divisor_len, dividend_len);

        assert_eq!(quotient_bz, quotient_reference, "BZ recursive quot mismatch [{label}]");
        assert_eq!(
            remainder_bz[..divisor_len],
            remainder_reference[..divisor_len],
            "BZ recursive rem mismatch [{label}]"
        );
    }

    // REGRESSION GUARD: the WORKING-WIDTH shapes that overflowed the (formerly
    // storage-sized) recursive scratch and panicked the D924/D1232 wide-
    // transcendental golden cells (`index out of bounds: len 134 idx 134`).
    // Production presents a divide at WORKING widths exceeding storage — the
    // `÷10^w` rescale + the wide-transcendental slice roots give an effective
    // `divisor_len` up to ~67 over a `~2·divisor_len` dividend — and BZ engages
    // there (`divisor_len ≥ BZ_THRESHOLD`). These are the exact 2n/n shapes that
    // arise in practice (134/67, 138/69, 96/48) plus neighbours, each
    // bit-identical to Knuth. xx-wide so the build-max recursive scratch is the
    // widest tier.
    #[test]
    fn bz_recursive_matches_knuth_working_width_2n_over_n() {
        let mut next = rng(0x5DEE_CE66_D1B5_4A32);
        // The named regression shapes (divisor_len ≥ 65, dividend = 2·divisor_len).
        for &(dividend_len, divisor_len) in
            &[(134usize, 67usize), (138, 69), (96, 48), (130, 65)]
        {
            for _ in 0..40 {
                let (dividend, divisor) = make(&mut next, dividend_len, divisor_len);
                assert_recursive_matches_knuth(&dividend, &divisor,
                    &format!("2n/n {dividend_len}/{divisor_len}"));
            }
        }
    }

    // The PRODUCTION ROUTED path: the public entry (engagement guard intact)
    // at `divisor_len ≥ BZ_THRESHOLD(65)`, `dividend ≥ 2·divisor_len` — exactly
    // where the matcher routes to BZ — must equal Knuth. Confirms the routed
    // engine, not just the forced core.
    #[test]
    fn bz_routed_entry_matches_knuth_at_engagement_widths() {
        let mut next = rng(0xA0761D64_78BD_642F);
        for &(dividend_len, divisor_len) in
            &[(134usize, 67usize), (138, 69), (130, 65), (128, 64)]
        {
            let (dividend, divisor) = make(&mut next, dividend_len, divisor_len);
            let len = dividend.len();
            let mut quotient_reference = vec![0u64; len + 1];
            let mut remainder_reference = vec![0u64; len + 1];
            div_knuth(&dividend, &divisor, &mut quotient_reference,
                &mut remainder_reference);
            let mut quotient_bz = vec![0u64; len + 1];
            let mut remainder_bz = vec![0u64; len + 1];
            div_burnikel_ziegler_with_knuth(&dividend, &divisor, &mut quotient_bz,
                &mut remainder_bz);
            assert_eq!(quotient_bz, quotient_reference,
                "routed BZ quot mismatch {dividend_len}/{divisor_len}");
            assert_eq!(
                remainder_bz[..divisor_len],
                remainder_reference[..divisor_len],
                "routed BZ rem mismatch {dividend_len}/{divisor_len}"
            );
        }
    }

    // A spread of shapes across the recursion regime — even/odd divisors,
    // exact-2n and ragged dividends, narrow through the widest working width —
    // each bit-identical to Knuth. Exercises every recursion path (even split,
    // odd base-case, saturate branch, the ≤2 corrections). xx-wide for the
    // widest scratch.
    #[test]
    fn bz_recursive_matches_knuth_spread() {
        let mut next = rng(0x2545_F491_4F6C_DD1D);
        for &divisor_len in &[17usize, 24, 32, 33, 48, 64, 65, 67, 69] {
            for &multiple in &[2usize, 3] {
                let dividend_len = multiple * divisor_len;
                for _ in 0..30 {
                    let (dividend, divisor) = make(&mut next, dividend_len, divisor_len);
                    assert_recursive_matches_knuth(&dividend, &divisor,
                        &format!("spread {dividend_len}/{divisor_len}"));
                    // Ragged dividend (dividend_len - a few limbs) to vary the
                    // block split.
                    if dividend_len > divisor_len + 1 {
                        let (dividend2, divisor2) =
                            make(&mut next, dividend_len - 1, divisor_len);
                        assert_recursive_matches_knuth(
                            &dividend2,
                            &divisor2,
                            &format!("ragged {}/{divisor_len}", dividend_len - 1),
                        );
                    }
                }
            }
        }
    }
}
