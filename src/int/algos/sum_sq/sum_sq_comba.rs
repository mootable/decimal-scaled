// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! UNBENCHED CANDIDATE — not wired into any policy; coordinator benches + selects.
//!
//! `sum_sq_comba` -- the integer sum of squares `a² + b²`, forming each
//! square with a dedicated FULL-WIDTH product-scanning (comba) squaring pass
//! instead of a general `x·x` schoolbook multiply.
//!
//! Background. The shared radicand former
//! [`crate::int::algos::sum_sq::sum_sq_schoolbook::sum_sq_radicand`] computes
//! `a²` and `b²` via `mul_schoolbook(x, x, ..)`, which forms every partial
//! product `x_i·x_j` for all `i, j` — `≈ L²` limb-multiplies per square. A
//! square has symmetric cross terms (`x_i·x_j == x_j·x_i`), so for each
//! output column only the pairs `i ≤ j` need be formed: each `i < j` once and
//! doubled, the `i == j` diagonal once. That is `≈ L²/2` limb-multiplies —
//! half the work — and is exactly the trick the crate's truncated low-`N`
//! squaring kernel
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] already uses; this
//! candidate lifts it to the FULL `2L`-limb product needed by the radicand.
//! Bit-identical to the schoolbook radicand by construction.
//!
//! The result is byte-for-byte the same value as `sum_sq_radicand`, so the
//! `sum_sq` method form built on it ([`sum_sq_comba`]) is a drop-in for
//! [`crate::int::algos::sum_sq::sum_sq_schoolbook::sum_sq_schoolbook`].

use crate::int::algos::sum_sq::sum_sq_schoolbook::sig_len;
use crate::int::algos::support::limbs::add_assign;
use crate::int::types::work_scratch::WorkingInt;
use crate::int::types::Int;

/// Full-width product-scanning (comba) square: `out = x²` over the
/// little-endian limb slice `x[..l]`, writing the full `2l` result limbs.
/// `out.len() >= 2*l` and `out` must be zeroed by the caller for the high
/// limbs it does not reach (this routine overwrites `out[..2*l]`).
///
/// For each output column `col` in `0..2*l-1`, sum every partial product
/// `x_i·x_j` with `i + j == col` and `i ≤ j`: the off-diagonal `i < j` terms
/// contribute twice (symmetry), the diagonal `i == j` once. A running
/// 128-bit-plus-overflow accumulator threads the inter-column carry, so there
/// is no per-product carry walk. Same partial products as the low-`N`
/// [`sqr_low_fixed`](crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed),
/// only carried out to the full width.
#[inline]
fn sqr_full(x: &[u64], l: usize, out: &mut [u64]) {
    let mut acc: u128 = 0;
    let mut hi: u64 = 0;
    let ncol = 2 * l; // columns 0 ..= 2l-2 are emitted; the final carry lands in 2l-1.
    let mut col = 0;
    while col < ncol {
        // Pairs (i, j) with i ≤ j, i + j == col, and both in 0..l.
        // i ranges from max(0, col-(l-1)) to floor(col/2).
        let lo_i = if col >= l { col - (l - 1) } else { 0 };
        let mut i = lo_i;
        while 2 * i <= col {
            let j = col - i;
            // j ≤ l-1 holds because i ≥ col-(l-1) ⇒ j = col-i ≤ l-1.
            let p = (x[i] as u128) * (x[j] as u128);
            let reps = if i == j { 1 } else { 2 };
            let mut r = 0;
            while r < reps {
                let (s, c) = acc.overflowing_add(p);
                acc = s;
                hi += c as u64;
                r += 1;
            }
            i += 1;
        }
        out[col] = acc as u64;
        acc = (acc >> 64) + ((hi as u128) << 64);
        hi = 0;
        col += 1;
    }
}

/// Form `a² + b²` (on the magnitude slices `ma` / `mb`) into `out` via the
/// comba square, returning its significant limb length. Drop-in for
/// [`crate::int::algos::sum_sq::sum_sq_schoolbook::sum_sq_radicand`]: same
/// contract, same result, the squares formed by [`sqr_full`].
#[inline]
#[allow(dead_code)]
pub(crate) fn sum_sq_radicand_comba<const N: usize>(ma: &[u64], mb: &[u64], out: &mut [u64]) -> usize
where
    Int<N>: WorkingInt,
{
    let la = sig_len(ma);
    let lb = sig_len(mb);
    // a² into `out` (zeroed by the caller); b² into its own scratch.
    sqr_full(ma, la, &mut out[..2 * la]);
    let mut bsq_buf = Int::<N>::work2();
    let bsq = bsq_buf.as_mut();
    sqr_full(mb, lb, &mut bsq[..2 * lb]);
    let span = (2 * la).max(2 * lb) + 1;
    add_assign(&mut out[..span], &bsq[..2 * lb]);
    sig_len(&out[..span])
}

/// `a² + b²` as an `Int<N>`, or [`None`] on true overflow. Drop-in for
/// [`crate::int::algos::sum_sq::sum_sq_schoolbook::sum_sq_schoolbook`].
#[inline]
#[must_use]
#[allow(dead_code)]
pub(crate) fn sum_sq_comba<const N: usize>(a: Int<N>, b: Int<N>) -> Option<Int<N>>
where
    Int<N>: WorkingInt,
{
    let ma = a.unsigned_abs();
    let mb = b.unsigned_abs();
    let mut n_buf = Int::<N>::work2();
    let n = n_buf.as_mut();
    let nl = sum_sq_radicand_comba::<N>(ma.as_limbs(), mb.as_limbs(), n);
    if nl > N || (nl == N && (n[N - 1] >> 63) != 0) {
        return None;
    }
    let mut out = [0u64; N];
    out.copy_from_slice(&n[..N]);
    Some(Int::<N>::from_limbs(out))
}

#[cfg(test)]
mod tests {
    use super::sum_sq_comba;
    use crate::int::algos::sum_sq::sum_sq_schoolbook::sum_sq_schoolbook;
    use crate::int::types::Int;

    /// SplitMix64 step — deterministic spread for the differential check.
    fn mix(s: &mut u64) -> u64 {
        *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *s;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn rand_int<const N: usize>(s: &mut u64) -> Int<N> {
        let mut limbs = [0u64; N];
        for limb in limbs.iter_mut() {
            *limb = mix(s);
        }
        // Clear the top bit so the magnitude stays comfortably positive; the
        // value is then interpreted signed by Int, which is fine — the
        // candidate and reference both run on `unsigned_abs`.
        limbs[N - 1] &= i64::MAX as u64;
        Int::<N>::from_limbs(limbs)
    }

    /// `sum_sq_comba` MUST equal `sum_sq_schoolbook` bit-for-bit on every
    /// input, including the overflow (`None`) cases.
    fn diff_at<const N: usize>()
    where
        Int<N>: crate::int::types::work_scratch::WorkingInt,
    {
        let mut s = 0x1234_5678_9ABC_DEFu64 ^ (N as u64);
        for _ in 0..400 {
            // Shrink one operand sometimes to exercise unequal lengths.
            let mut a = rand_int::<N>(&mut s);
            let mut b = rand_int::<N>(&mut s);
            if mix(&mut s) & 1 == 0 {
                a = Int::<N>::from_limbs({
                    let mut l = *a.as_limbs();
                    for k in 1..N {
                        l[k] = 0;
                    }
                    l[0] &= 0xFFFF_FFFF; // ~32-bit operand
                    l
                });
            }
            if mix(&mut s) & 1 == 0 {
                b = Int::<N>::ZERO;
            }
            assert_eq!(
                sum_sq_comba::<N>(a, b),
                sum_sq_schoolbook::<N>(a, b),
                "N={N} a={:?} b={:?}",
                a.as_limbs(),
                b.as_limbs()
            );
        }
        // Explicit small cases.
        let three = Int::<N>::from_i64(3);
        let four = Int::<N>::from_i64(4);
        assert_eq!(sum_sq_comba::<N>(three, four), sum_sq_schoolbook::<N>(three, four));
        let z = Int::<N>::ZERO;
        assert_eq!(sum_sq_comba::<N>(z, z), sum_sq_schoolbook::<N>(z, z));
        // Overflow case: MAX, MAX.
        assert_eq!(sum_sq_comba::<N>(Int::<N>::MAX, Int::<N>::MAX), sum_sq_schoolbook::<N>(Int::<N>::MAX, Int::<N>::MAX));
    }

    #[test]
    fn sum_sq_comba_matches_schoolbook() {
        diff_at::<2>();
        diff_at::<3>();
        diff_at::<4>();
        diff_at::<6>();
        diff_at::<8>();
    }
}
