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

use crate::int::types::compute_int::Limb;

/// `out = a · b` schoolbook. `out.len() >= a.len() + b.len()` and `out`
/// must be zeroed by the caller.
///
/// Inner step uses the native `u64 × u64 → u128` widening mul
/// (`MUL` + `UMULH` on x86-64 / aarch64).
pub(crate) const fn mul_schoolbook(a: &[u64], b: &[u64], out: &mut [u64]) {
    let mut i = 0;
    while i < a.len() {
        if a[i] != 0 {
            let mut carry: u64 = 0;
            let mut j = 0;
            while j < b.len() {
                if b[j] != 0 || carry != 0 {
                    let prod = (a[i] as u128) * (b[j] as u128);
                    let prod_lo = prod as u64;
                    let prod_hi = (prod >> 64) as u64;
                    let idx = i + j;
                    let (s1, c1) = out[idx].overflowing_add(prod_lo);
                    let (s2, c2) = s1.overflowing_add(carry);
                    out[idx] = s2;
                    carry = prod_hi + (c1 as u64) + (c2 as u64);
                }
                j += 1;
            }
            let mut idx = i + b.len();
            while carry != 0 && idx < out.len() {
                let (s, c) = out[idx].overflowing_add(carry);
                out[idx] = s;
                carry = c as u64;
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
    a: &[u64; L],
    b: &[u64; L],
    out: &mut [u64; D],
) {
    debug_assert!(D >= 2 * L, "mul_schoolbook_fixed: D must be ≥ 2·L");
    let mut i = 0;
    while i < L {
        let ai = a[i];
        if ai != 0 {
            let mut carry: u64 = 0;
            let mut j = 0;
            while j < L {
                let v = (ai as u128) * (b[j] as u128) + (out[i + j] as u128) + (carry as u128);
                out[i + j] = v as u64;
                carry = (v >> 64) as u64;
                j += 1;
            }
            // Final row carry, propagated until exhausted or end of
            // `out`. Worst-case unbounded chain when out[i + L ..]
            // is all-ones; ordinarily exits after 1 iteration.
            let mut idx = i + L;
            let mut c = carry;
            while c != 0 && idx < D {
                let v = (out[idx] as u128) + (c as u128);
                out[idx] = v as u64;
                c = (v >> 64) as u64;
                idx += 1;
            }
        }
        i += 1;
    }
}

/// `out = a · n` where `n` is a single u64 multiplier, `a` is a
/// fixed-width `L`-limb input, and `out` is a fixed-width `LP1 = L + 1`
/// limb output. `out` must be zeroed by the caller.
///
/// Specialisation of the n-by-1-word multi-precision multiply (Knuth,
/// TAOCP Vol 2 §4.3.1, Algorithm M with `n = 1`): every inner-loop step
/// is a single `u64 × u64 → u128` widening mul plus an accumulator-and-
/// carry fold, so the whole operation is `L` widening muls and `L` adds
/// with no cross-row carry chains. By contrast, [`mul_schoolbook_fixed`]
/// called with `b = [n, 0, ..., 0]` still runs the `L²` outer-product
/// loop (most iterations are short-circuited on `b[j] == 0`, but the
/// monomorphisation still emits the dead branches and the row
/// carry-propagation tail).
///
/// `LP1` must equal `L + 1`; the caller passes both because Rust stable
/// cannot express `L + 1` in a const generic position.
#[inline(always)]
pub(crate) const fn mul_schoolbook_into<const L: usize, const LP1: usize>(
    a: &[u64; L],
    n: u64,
    out: &mut [u64; LP1],
) {
    debug_assert!(LP1 == L + 1, "mul_schoolbook_into: LP1 must equal L + 1");
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < L {
        // p fits u128 with no overflow:
        //   (2^64 - 1)·(2^64 - 1) + (2^64 - 1) + (2^64 - 1)
        //   = 2^128 - 1
        let p = (a[i] as u128) * (n as u128) + (out[i] as u128) + (carry as u128);
        out[i] = p as u64;
        carry = (p >> 64) as u64;
        i += 1;
    }
    out[L] = carry;
}

/// `out = (a · b) mod 2^(64·N)` — the low `N` limbs of the schoolbook
/// product, with the high half never formed.
///
/// `out` must be zeroed by the caller. For each operand limb `a[i]`, the
/// inner loop runs only while `i + j < N`; products that would land in
/// limb `N` or above are exactly the bits above the width and are
/// dropped, including the final row carry. Bit-identical to the low `N`
/// limbs of [`mul_schoolbook_fixed`].
#[inline]
pub(crate) const fn mul_low_fixed<const N: usize>(a: &[u64; N], b: &[u64; N], out: &mut [u64; N]) {
    let mut i = 0;
    while i < N {
        let ai = a[i];
        if ai != 0 {
            let mut carry: u64 = 0;
            let mut j = 0;
            // Stop once `i + j` reaches `N`: those partial products lie
            // entirely above `2^(64·N)` and drop out of the result.
            while j < N - i {
                let v = (ai as u128) * (b[j] as u128) + (out[i + j] as u128) + (carry as u128);
                out[i + j] = v as u64;
                carry = (v >> 64) as u64;
                j += 1;
            }
            // The final row carry would land in limb `i + (N - i) = N`,
            // which is above the width — discarded.
        }
        i += 1;
    }
}

/// `out = (a · b) mod 2^(64·N)` — the truncated-low schoolbook, generic over
/// the limb type `L` (the [`Limb`] axis). For `L = u64` it is base-2^64 over
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
/// The carry merge `hi.add_carries(c1, c2)` never overflows: the product
/// high limb satisfies `hi ≤ L::MAX − 1` (maximal only when the low limb is
/// 1), and `c1`/`c2` are never both set (`c1` needs `acc + lo` to wrap to 0,
/// after which `+ carry` cannot wrap), so `hi + c1 + c2 ≤ L::MAX`.
///
/// [`LimbSize`]: crate::int::types::compute_int::LimbSize
#[inline]
pub(crate) fn mul_low_limb<const N: usize, L: Limb>(a: &[u64; N], b: &[u64; N], out: &mut [u64; N]) {
    let h = L::packed_len(N);
    // `[L; N]` covers `packed_len(N) ≤ N` for both limb types (stable Rust
    // cannot put `N/2` in an array-length position; only the low `h` are used).
    let mut ap = [L::ZERO; N];
    let mut bp = [L::ZERO; N];
    L::pack(a, &mut ap[..h]);
    L::pack(b, &mut bp[..h]);
    let mut acc = [L::ZERO; N];
    let mut i = 0;
    while i < h {
        let ai = ap[i];
        if ai != L::ZERO {
            let mut carry = L::ZERO;
            let mut j = 0;
            // Stop once `i + j` reaches `h`: those partials lie above
            // 2^(64·N) and drop out of the truncated-low result.
            while j < h - i {
                let (lo, hi) = ai.widening_mul(bp[j]);
                let idx = i + j;
                let (s1, c1) = acc[idx].overflowing_add(lo);
                let (s2, c2) = s1.overflowing_add(carry);
                acc[idx] = s2;
                carry = hi.add_carries(c1, c2);
                j += 1;
            }
        }
        i += 1;
    }
    L::unpack(&acc[..h], out);
}
