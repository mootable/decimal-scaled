// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Base-2⁶⁴ schoolbook (long) multiplication kernels.
//!
//! The schoolbook outer-product multiply and its fixed-width / single-word
//! / truncated-low specialisations. Each is a *pure* kernel performing one
//! named algorithm; the schoolbook-vs-Karatsuba *choice* lives in
//! [`crate::int::policy::mul`]. Every routine treats its `&[u64]` slices as
//! little-endian unsigned integers (`limbs[0]` least significant) and the
//! caller sizes/zeroes the output buffers.
//!
//! Inner step uses the native `u64 × u64 → u128` widening multiply
//! (`MUL` + `UMULH` on x86-64 / aarch64).

use crate::int::types::compute_limbs::{ComputeLimbs, Limb, Limbs};

/// `out = lhs · rhs` schoolbook. `out.len() >= lhs.len() + rhs.len()` and
/// `out` must be zeroed by the caller.
///
/// Inner step uses the native `u64 × u64 → u128` widening mul
/// (`MUL` + `UMULH` on x86-64 / aarch64).
pub(crate) const fn mul_schoolbook(lhs: &[u64], rhs: &[u64], out: &mut [u64]) {
    let mut i = 0;
    while i < lhs.len() {
        if lhs[i] != 0 {
            let mut carry: u64 = 0;
            let mut j = 0;
            while j < rhs.len() {
                if rhs[j] != 0 || carry != 0 {
                    let prod = (lhs[i] as u128) * (rhs[j] as u128);
                    let prod_lo = prod as u64;
                    let prod_hi = (prod >> 64) as u64;
                    let idx = i + j;
                    let (sum1, carry1) = out[idx].overflowing_add(prod_lo);
                    let (sum2, carry2) = sum1.overflowing_add(carry);
                    out[idx] = sum2;
                    carry = prod_hi + (carry1 as u64) + (carry2 as u64);
                }
                j += 1;
            }
            let mut idx = i + rhs.len();
            while carry != 0 && idx < out.len() {
                let (sum, carried) = out[idx].overflowing_add(carry);
                out[idx] = sum;
                carry = carried as u64;
                idx += 1;
            }
        }
        i += 1;
    }
}

/// Fixed-width specialisation of [`mul_schoolbook`]: the operand
/// limb-count `L` and output limb-count `D = 2·L` are both compile-time
/// constants, so the slice indirection and loop-bound checks vanish and
/// LLVM can unroll the inner loop (and, for small `L`, the outer one
/// too).
///
/// Same algorithm and same output as [`mul_schoolbook`]; faster only when
/// both operands have known-equal length (the common case for wide-tier
/// `widen_mul` where both operands are an `Int<N>` of the tier's storage
/// width).
#[inline]
pub(crate) const fn mul_schoolbook_fixed<const L: usize, const D: usize>(
    lhs: &[u64; L],
    rhs: &[u64; L],
    out: &mut [u64; D],
) {
    debug_assert!(D >= 2 * L, "mul_schoolbook_fixed: D must be ≥ 2·L");
    let mut i = 0;
    while i < L {
        let lhs_limb = lhs[i];
        if lhs_limb != 0 {
            let mut carry: u64 = 0;
            let mut j = 0;
            while j < L {
                let acc = (lhs_limb as u128) * (rhs[j] as u128)
                    + (out[i + j] as u128) + (carry as u128);
                out[i + j] = acc as u64;
                carry = (acc >> 64) as u64;
                j += 1;
            }
            // Final row carry, propagated until exhausted or end of
            // `out`. Worst-case unbounded chain when out[i + L ..]
            // is all-ones; ordinarily exits after 1 iteration.
            let mut idx = i + L;
            let mut tail_carry = carry;
            while tail_carry != 0 && idx < D {
                let acc = (out[idx] as u128) + (tail_carry as u128);
                out[idx] = acc as u64;
                tail_carry = (acc >> 64) as u64;
                idx += 1;
            }
        }
        i += 1;
    }
}

/// `out = multiplicand · multiplier` where `multiplier` is a single u64,
/// `multiplicand` is a fixed-width `L`-limb input, and `out` is a
/// fixed-width `LP1 = L + 1` limb output. `out` must be zeroed by the caller.
///
/// Specialisation of the n-by-1-word multi-precision multiply (Knuth,
/// TAOCP Vol 2 §4.3.1, Algorithm M with `n = 1`): every inner-loop step
/// is a single `u64 × u64 → u128` widening mul plus an accumulator-and-
/// carry fold, so the whole operation is `L` widening muls and `L` adds
/// with no cross-row carry chains. By contrast, [`mul_schoolbook_fixed`]
/// called with `rhs = [multiplier, 0, ..., 0]` still runs the `L²`
/// outer-product loop (most iterations are short-circuited on
/// `rhs[j] == 0`, but the monomorphisation still emits the dead branches
/// and the row carry-propagation tail).
///
/// `LP1` must equal `L + 1`; the caller passes both because Rust stable
/// cannot express `L + 1` in a const generic position.
#[inline(always)]
pub(crate) const fn mul_schoolbook_into<const L: usize, const LP1: usize>(
    multiplicand: &[u64; L],
    multiplier: u64,
    out: &mut [u64; LP1],
) {
    debug_assert!(LP1 == L + 1, "mul_schoolbook_into: LP1 must equal L + 1");
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < L {
        // acc fits u128 with no overflow:
        //   (2^64 - 1)·(2^64 - 1) + (2^64 - 1) + (2^64 - 1)
        //   = 2^128 - 1
        let acc = (multiplicand[i] as u128) * (multiplier as u128)
            + (out[i] as u128) + (carry as u128);
        out[i] = acc as u64;
        carry = (acc >> 64) as u64;
        i += 1;
    }
    out[L] = carry;
}

/// `out = (lhs · rhs) mod 2^(64·N)` — the low `N` limbs of the schoolbook
/// product, with the high half never formed.
///
/// `out` must be zeroed by the caller. For each operand limb `lhs[i]`, the
/// inner loop runs only while `i + j < N`; products that would land in
/// limb `N` or above are exactly the bits above the width and are
/// dropped, including the final row carry. Bit-identical to the low `N`
/// limbs of [`mul_schoolbook_fixed`].
#[inline]
pub(crate) const fn mul_low_fixed<const N: usize>(lhs: &[u64; N], rhs: &[u64; N],
    out: &mut [u64; N]) {
    let mut i = 0;
    while i < N {
        let lhs_limb = lhs[i];
        if lhs_limb != 0 {
            let mut carry: u64 = 0;
            let mut j = 0;
            // Stop once `i + j` reaches `N`: those partial products lie
            // entirely above `2^(64·N)` and drop out of the result.
            while j < N - i {
                let acc = (lhs_limb as u128) * (rhs[j] as u128)
                    + (out[i + j] as u128) + (carry as u128);
                out[i + j] = acc as u64;
                carry = (acc >> 64) as u64;
                j += 1;
            }
            // The final row carry would land in limb `i + (N - i) = N`,
            // which is above the width — discarded.
        }
        i += 1;
    }
}

/// `out = (lhs · rhs) mod 2^(64·N)` — the truncated-low schoolbook, generic
/// over the limb type `L` (the [`Limb`] axis). For `L = u64` it is base-2^64 over
/// `N` limbs; for `L = u128` it packs the operands into `N/2` u128 limbs
/// (`limb = lo | hi << 64`) and runs base-2^128 — half the limb count, so
/// ~1/4 the partial products at the cost of a wider 128×128→256 inner step —
/// then unpacks. Bit-identical low `N` u64 limbs either way.
///
/// ONE kernel for both widths: the matcher's [`LimbSize`] verdict picks `L`
/// (a const-folded `match` → `mul_low_limb::<N, u64>` / `::<N, u128>`), so
/// there is no per-limb-type copy. The `u128` arm requires **even `N`**
/// (`L::packed_len` halves it); callers gate on that. Scratch is `[L; N]`
/// (the value's own width — `packed_len(N) ≤ N`), not a build-max blanket.
///
/// The carry merge `hi.add_carries(carry1, carry2)` never overflows: the
/// product high limb satisfies `hi ≤ L::MAX − 1` (maximal only when the low
/// limb is 1), and `carry1`/`carry2` are never both set (`carry1` needs
/// `acc + lo` to wrap to 0, after which `+ carry` cannot wrap), so
/// `hi + carry1 + carry2 ≤ L::MAX`.
///
/// [`LimbSize`]: crate::int::types::compute_limbs::LimbSize
#[inline]
pub(crate) fn mul_low_limb<const N: usize, L: Limb>(lhs: &[u64; N], rhs: &[u64; N],
    out: &mut [u64; N]) {
    let h = L::packed_len(N);
    // `[L; N]` covers `packed_len(N) ≤ N` for both limb types (stable Rust
    // cannot put `N/2` in an array-length position; only the low `h` are used).
    let mut lhs_packed = [L::ZERO; N];
    let mut rhs_packed = [L::ZERO; N];
    L::pack(lhs, &mut lhs_packed[..h]);
    L::pack(rhs, &mut rhs_packed[..h]);
    // `rhs_len` = `rhs`'s live packed-limb count. The inner loop need only run
    // over `rhs`'s significant limbs; its zero high limbs contribute only a
    // carry, replicated bit-identically by the carry tail below. Skipping them
    // turns a full-width multiply of a SMALL operand — the common shape in the
    // wide transcendental series (terms shrink) and the working-scale lift —
    // into one scaled by `rhs`'s magnitude. Gated to wide `N`: for the narrow
    // tiers the operands are dense, so the scan is pure overhead and the const
    // folds it away.
    let rhs_len = if N >= 16 {
        let mut len = h;
        while len > 0 && rhs_packed[len - 1] == L::ZERO {
            len -= 1;
        }
        len
    } else {
        h
    };
    let mut acc = [L::ZERO; N];
    let mut i = 0;
    while i < h {
        let lhs_limb = lhs_packed[i];
        if lhs_limb != L::ZERO {
            let mut carry = L::ZERO;
            let mut j = 0;
            // Stop once `i + j` reaches `h` (partials above 2^(64·N) drop out
            // of the truncated-low result) OR once `rhs`'s significant limbs
            // are exhausted (`rhs_len`); the residual carry is propagated by
            // the tail.
            let jmax = (h - i).min(rhs_len);
            while j < jmax {
                let (lo, hi) = lhs_limb.widening_mul(rhs_packed[j]);
                let idx = i + j;
                let (sum1, carry1) = acc[idx].overflowing_add(lo);
                let (sum2, carry2) = sum1.overflowing_add(carry);
                acc[idx] = sum2;
                carry = hi.add_carries(carry1, carry2);
                j += 1;
            }
            // Carry tail over the zero-`rhs` region — bit-identical to running
            // the inner loop with `rhs_packed[j] == 0` (the multiply yields 0,
            // leaving only `acc[idx] += carry` and its overflow into the next
            // limb).
            let mut idx = i + jmax;
            while idx < h && carry != L::ZERO {
                let (sum, carried) = acc[idx].overflowing_add(carry);
                acc[idx] = sum;
                carry = L::ZERO.add_carries(false, carried);
                idx += 1;
            }
        }
        i += 1;
    }
    L::unpack(&acc[..h], out);
}

/// `out = lhs · rhs` — the FULL `2·N`-u64 schoolbook product, generic over the
/// limb type `L` (the [`Limb`] axis). The full-product sibling of
/// [`mul_low_limb`]: for `L = u64` it is base-2^64 over `N` limbs (bit-identical
/// to [`mul_schoolbook_fixed`]); for `L = u128` it packs each operand into `N/2`
/// u128 limbs (`limb = lo | hi << 64`) and runs base-2^128 — half the limb count,
/// so ~1/4 the partial products at the cost of a wider 128×128→256 inner step —
/// then unpacks. Bit-identical `2·N` u64 limbs either way.
///
/// ONE kernel for both widths: the matcher's [`LimbSize`] verdict picks `L` (a
/// const-folded `match` in [`crate::int::policy::mul`]), so there is no
/// per-limb-type copy. The `u128` arm requires **even `N`** (`L::packed_len`
/// halves it); the caller gates on that.
///
/// The accumulator is the value's OWN `2·N`-u64-width scratch in limb type `L`
/// ([`Limb::double`] → `Int<N>::double_{u64,u128}`): exactly `2·h` `L`-limbs
/// (`2·N` u64 / `N` u128), per-`N`-exact — NOT a build-max blanket. `out.len()`
/// must be `>= 2·N` and is written in full (the kernel zeroes its accumulator).
///
/// [`LimbSize`]: crate::int::types::compute_limbs::LimbSize
#[inline]
pub(crate) fn mul_full_limb<const N: usize, L: Limb>(lhs: &[u64; N], rhs: &[u64; N],
    out: &mut [u64])
where
    Limbs<N>: ComputeLimbs,
{
    let h = L::packed_len(N); // operand packed length (N for u64, N/2 for u128)
    let d = 2 * h; // full-product length in L-limbs (2N u64 / N u128)
    // Operand packs: the value's OWN `single` width in limb type `L`
    // ([`Limb::single`] → `N` u64 / `⌈N/2⌉` u128), from the same `ComputeLimbs`
    // family the accumulator below draws on. At `L = u128` that is the `h = N/2`
    // limbs actually live, where `[L::ZERO; N]` zeroed `N` of them — `16N` bytes
    // per operand for `8N` live. At `L = u64` it is `N` either way.
    let mut lhs_buf = L::single::<Limbs<N>>();
    let mut rhs_buf = L::single::<Limbs<N>>();
    let lhs_packed = lhs_buf.as_mut();
    let rhs_packed = rhs_buf.as_mut();
    L::pack(lhs, &mut lhs_packed[..h]);
    L::pack(rhs, &mut rhs_packed[..h]);
    // Accumulator: the value's own 2N-u64-width buffer in limb type `L`
    // (= 2h L-limbs exactly), freshly zeroed.
    let mut acc_buf = L::double::<Limbs<N>>();
    let acc = acc_buf.as_mut();
    let mut i = 0;
    while i < h {
        let lhs_limb = lhs_packed[i];
        if lhs_limb != L::ZERO {
            let mut carry = L::ZERO;
            let mut j = 0;
            while j < h {
                let (lo, hi) = lhs_limb.widening_mul(rhs_packed[j]);
                let idx = i + j;
                let (sum1, carry1) = acc[idx].overflowing_add(lo);
                let (sum2, carry2) = sum1.overflowing_add(carry);
                acc[idx] = sum2;
                carry = hi.add_carries(carry1, carry2);
                j += 1;
            }
            // Final row carry, propagated into the high half until exhausted.
            // The first add absorbs the full `L`-limb carry; thereafter the
            // propagated carry is at most one (a single-limb add).
            let mut idx = i + h;
            while carry != L::ZERO && idx < d {
                let (sum, carried) = acc[idx].overflowing_add(carry);
                acc[idx] = sum;
                carry = if carried { L::ONE } else { L::ZERO };
                idx += 1;
            }
        }
        i += 1;
    }
    L::unpack(&acc[..d], &mut out[..2 * N]);
}
