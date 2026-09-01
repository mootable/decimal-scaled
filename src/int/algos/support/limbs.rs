// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Generic little-endian `u64` limb primitives.
//!
//! The integer layer's *shared* primitive bucket: the cross-cutting
//! `&[u64]` slice operations that several function families compose on
//! (comparison, equality, bit-length, ripple add/sub, shifts, single-limb
//! fit test, signed compare). Each is a low-level primitive — not itself a
//! dispatched function's headline algorithm — so it stays here rather than
//! in a per-function `int/algos/<fn>/` folder; the function-specific
//! algorithm kernels (multiply, squaring, the divide engines, the integer
//! roots) live in their own `int/algos/<fn>/` folders.
//!
//! Every routine treats its `&[u64]` slices as little-endian unsigned
//! integers (`limbs[0]` least significant); lengths are taken from the
//! slices and callers size the output buffers.
//!
//! The core routines are `const fn` so the integer types built on them
//! can expose `const` constructors and constants.

/// Widest decimal storage limb count `N` enabled by the build's width
/// features (D38=2, D307=16, D616=32, D1232=64). The work-scratch sizing
/// derives from it via [`max_n_limbs`] — ONE feature-gated const + one
/// `const fn`, no per-`Int<N>` impls.
#[cfg(any(feature = "xx-wide", feature = "d924", feature = "d1232"))]
pub(crate) const MAX_WORK_N: usize = 64;
#[cfg(all(
    not(any(feature = "xx-wide", feature = "d924", feature = "d1232")),
    any(feature = "x-wide", feature = "d462", feature = "d616")
))]
pub(crate) const MAX_WORK_N: usize = 32;
#[cfg(all(
    not(any(
        feature = "xx-wide", feature = "d924", feature = "d1232",
        feature = "x-wide", feature = "d462", feature = "d616"
    )),
    any(
        feature = "wide", feature = "d57", feature = "d76", feature = "d115",
        feature = "d153", feature = "d230", feature = "d307"
    )
))]
pub(crate) const MAX_WORK_N: usize = 16;
#[cfg(not(any(
    feature = "xx-wide", feature = "d924", feature = "d1232",
    feature = "x-wide", feature = "d462", feature = "d616",
    feature = "wide", feature = "d57", feature = "d76", feature = "d115",
    feature = "d153", feature = "d230", feature = "d307"
)))]
pub(crate) const MAX_WORK_N: usize = 2;

/// Fixed limb-scratch budget for a width-agnostic kernel whose work value
/// spans `mult·N` limbs: `mult = 2` for the 2N-family (`sqrt`/`hypot`/
/// `isqrt_newton`, radicand ≤ 2N), `mult = 4` for the 4N-family
/// (`cbrt`/`icbrt_newton`, radicand ≤ 4N). Sized
/// `mult·MAX_WORK_N + ceil(MAX_WORK_N/2)` — the work width plus a `0.5·N`
/// margin for the `work = n.len()+1` carry-limb sizing (reproducing the
/// proven `288 = max_n_limbs(4)` at xx-wide). Kernels expand in limbs
/// rather than a work *type* `Int<2N>`/`Int<4N>` (unnameable from `N` on
/// stable; see the algorithim-optimiser skill §5). The exact per-`N`
/// alternatives ([`n_limbs`], the `exact-scratch` impls) live in
/// `crate::int::types::max_n_limbs`.
pub(crate) const fn max_n_limbs(mult: usize) -> usize {
    mult * MAX_WORK_N + MAX_WORK_N.div_ceil(2)
}

/// Exact per-`N` work-scratch budget: `mult·limb_count + ceil(limb_count/2)`,
/// the same formula as [`max_n_limbs`] but for a *specific* `limb_count`
/// rather than the build-max. Used by the `exact-scratch-nightly` blanket
/// [`ComputeLimbs`] impl, where it appears as a `generic_const_exprs` array
/// length confined to that impl block.
///
/// [`ComputeLimbs`]: crate::int::types::compute_limbs::ComputeLimbs
#[cfg(feature = "exact-scratch-nightly")]
pub(crate) const fn n_limbs(mult: usize, limb_count: usize) -> usize {
    mult * limb_count + (limb_count + 1) / 2
}

/// `limbs == 0`.
#[inline]
pub(crate) const fn is_zero(limbs: &[u64]) -> bool {
    let mut i = 0;
    while i < limbs.len() {
        if limbs[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Fixed-width specialisation of [`is_zero`]. `L` const at callsite, lets
/// LLVM unroll for small `L`.
#[inline]
pub(crate) const fn is_zero_fixed<const L: usize>(limbs: &[u64; L]) -> bool {
    let mut i = 0;
    while i < L {
        if limbs[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// `lhs == rhs` for two limb slices of possibly different lengths.
#[inline]
pub(crate) const fn eq(lhs: &[u64], rhs: &[u64]) -> bool {
    let max_len = if lhs.len() > rhs.len() { lhs.len() } else { rhs.len() };
    let mut i = 0;
    while i < max_len {
        let lhs_limb = if i < lhs.len() { lhs[i] } else { 0 };
        let rhs_limb = if i < rhs.len() { rhs[i] } else { 0 };
        if lhs_limb != rhs_limb {
            return false;
        }
        i += 1;
    }
    true
}

/// Three-way comparison `-1`/`0`/`1`.
#[inline]
pub(crate) const fn cmp(lhs: &[u64], rhs: &[u64]) -> i32 {
    let max_len = if lhs.len() > rhs.len() { lhs.len() } else { rhs.len() };
    let mut i = max_len;
    while i > 0 {
        i -= 1;
        let lhs_limb = if i < lhs.len() { lhs[i] } else { 0 };
        let rhs_limb = if i < rhs.len() { rhs[i] } else { 0 };
        if lhs_limb < rhs_limb {
            return -1;
        }
        if lhs_limb > rhs_limb {
            return 1;
        }
    }
    0
}

/// Fixed-width specialisation of [`cmp`] — both operands the same `L`; no
/// length-difference handling needed.
#[inline]
pub(crate) const fn cmp_fixed<const L: usize>(lhs: &[u64; L], rhs: &[u64; L]) -> i32 {
    let mut i = L;
    while i > 0 {
        i -= 1;
        if lhs[i] < rhs[i] {
            return -1;
        }
        if lhs[i] > rhs[i] {
            return 1;
        }
    }
    0
}

/// Cross-width unsigned magnitude comparison of two little-endian limb
/// slices of possibly different lengths. Returns `-1` / `0` / `1` for
/// `lhs < rhs` / `lhs == rhs` / `lhs > rhs`. The surplus high limbs of the
/// longer slice must all be zero for the magnitudes to be equal there; any
/// non-zero surplus limb makes that side the larger. No widening copy is
/// made — the slices are compared in place. Const.
#[inline]
pub(crate) const fn cmp_cross(lhs: &[u64], rhs: &[u64]) -> i32 {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let max_len = if lhs_len > rhs_len { lhs_len } else { rhs_len };
    let mut i = max_len;
    while i > 0 {
        i -= 1;
        let lhs_limb = if i < lhs_len { lhs[i] } else { 0 };
        let rhs_limb = if i < rhs_len { rhs[i] } else { 0 };
        if lhs_limb < rhs_limb {
            return -1;
        }
        if lhs_limb > rhs_limb {
            return 1;
        }
    }
    0
}

/// Bit length of the UNSIGNED value the limbs represent (`0` for zero,
/// else `floor(log2)+1`). The limbs are read as a non-negative
/// little-endian magnitude; signed callers pass the magnitude limbs of
/// `|value|` (see `Int::bit_length`), so the result is the count of
/// significant bits, not a two's-complement bit count.
#[inline]
pub(crate) const fn bit_len(limbs: &[u64]) -> u32 {
    let mut i = limbs.len();
    while i > 0 {
        i -= 1;
        if limbs[i] != 0 {
            return (i as u32) * 64 + (64 - limbs[i].leading_zeros());
        }
    }
    0
}

/// Fixed-width specialisation of [`bit_len`]: significant bits of the
/// non-negative magnitude held in `limbs` (`0` for zero).
#[inline]
pub(crate) const fn bit_len_fixed<const L: usize>(limbs: &[u64; L]) -> u32 {
    let mut i = L;
    while i > 0 {
        i -= 1;
        if limbs[i] != 0 {
            return (i as u32) * 64 + (64 - limbs[i].leading_zeros());
        }
    }
    0
}

/// `lhs += rhs`, returns carry out. `lhs.len() >= rhs.len()`.
#[inline]
pub(crate) const fn add_assign(lhs: &mut [u64], rhs: &[u64]) -> bool {
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < lhs.len() {
        let rhs_limb = if i < rhs.len() { rhs[i] } else { 0 };
        let (sum1, carry1) = lhs[i].overflowing_add(rhs_limb);
        let (sum2, carry2) = sum1.overflowing_add(carry);
        lhs[i] = sum2;
        carry = (carry1 as u64) + (carry2 as u64);
        i += 1;
    }
    carry != 0
}

/// Fixed-width specialisation of [`add_assign`] — both operands the same
/// `L`.
#[inline]
pub(crate) const fn add_assign_fixed<const L: usize>(lhs: &mut [u64; L], rhs: &[u64; L]) -> bool {
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < L {
        let (sum1, carry1) = lhs[i].overflowing_add(rhs[i]);
        let (sum2, carry2) = sum1.overflowing_add(carry);
        lhs[i] = sum2;
        carry = (carry1 as u64) + (carry2 as u64);
        i += 1;
    }
    carry != 0
}

/// `lhs -= rhs`, returns borrow out. `lhs.len() >= rhs.len()`.
#[inline]
pub(crate) const fn sub_assign(lhs: &mut [u64], rhs: &[u64]) -> bool {
    let mut borrow: u64 = 0;
    let mut i = 0;
    while i < lhs.len() {
        let rhs_limb = if i < rhs.len() { rhs[i] } else { 0 };
        let (diff1, borrow1) = lhs[i].overflowing_sub(rhs_limb);
        let (diff2, borrow2) = diff1.overflowing_sub(borrow);
        lhs[i] = diff2;
        borrow = (borrow1 as u64) + (borrow2 as u64);
        i += 1;
    }
    borrow != 0
}

/// Fixed-width specialisation of [`sub_assign`].
#[inline]
pub(crate) const fn sub_assign_fixed<const L: usize>(lhs: &mut [u64; L], rhs: &[u64; L]) -> bool {
    let mut borrow: u64 = 0;
    let mut i = 0;
    while i < L {
        let (diff1, borrow1) = lhs[i].overflowing_sub(rhs[i]);
        let (diff2, borrow2) = diff1.overflowing_sub(borrow);
        lhs[i] = diff2;
        borrow = (borrow1 as u64) + (borrow2 as u64);
        i += 1;
    }
    borrow != 0
}

/// Fixed-width specialisation of [`shl`]. `L` const, but `shift` is still
/// runtime — bounds checks vanish, the inner loop trip count is known.
#[inline]
pub(crate) const fn shl_fixed<const L: usize>(limbs: &[u64; L], shift: u32, out: &mut [u64; L]) {
    let mut z = 0;
    while z < L {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    let mut i = 0;
    while i < L {
        let dst = i + limb_shift;
        if dst < L {
            if bit_shift == 0 {
                out[dst] |= limbs[i];
            } else {
                out[dst] |= limbs[i] << bit_shift;
                if dst + 1 < L {
                    out[dst + 1] |= limbs[i] >> (64 - bit_shift);
                }
            }
        }
        i += 1;
    }
}

/// Fixed-width specialisation of [`shr`].
#[inline]
pub(crate) const fn shr_fixed<const L: usize>(limbs: &[u64; L], shift: u32, out: &mut [u64; L]) {
    let mut z = 0;
    while z < L {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    let mut i = limb_shift;
    while i < L {
        let dst = i - limb_shift;
        if dst < L {
            if bit_shift == 0 {
                out[dst] |= limbs[i];
            } else {
                out[dst] |= limbs[i] >> bit_shift;
                if dst >= 1 {
                    out[dst - 1] |= limbs[i] << (64 - bit_shift);
                }
            }
        }
        i += 1;
    }
}

/// `out = limbs << shift`. `out` is zeroed then filled.
pub(crate) const fn shl(limbs: &[u64], shift: u32, out: &mut [u64]) {
    let mut z = 0;
    while z < out.len() {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    let mut i = 0;
    while i < limbs.len() {
        let dst = i + limb_shift;
        if dst < out.len() {
            if bit_shift == 0 {
                out[dst] |= limbs[i];
            } else {
                out[dst] |= limbs[i] << bit_shift;
                if dst + 1 < out.len() {
                    out[dst + 1] |= limbs[i] >> (64 - bit_shift);
                }
            }
        }
        i += 1;
    }
}

/// `out = limbs >> shift`. `out` is zeroed then filled.
pub(crate) const fn shr(limbs: &[u64], shift: u32, out: &mut [u64]) {
    let mut z = 0;
    while z < out.len() {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    let mut i = limb_shift;
    while i < limbs.len() {
        let dst = i - limb_shift;
        if dst < out.len() {
            if bit_shift == 0 {
                out[dst] |= limbs[i];
            } else {
                out[dst] |= limbs[i] >> bit_shift;
                if dst >= 1 {
                    out[dst - 1] |= limbs[i] << (64 - bit_shift);
                }
            }
        }
        i += 1;
    }
}

/// Single-bit left shift in place; returns the bit shifted out.
#[inline]
pub(crate) const fn shl1(limbs: &mut [u64]) -> u64 {
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < limbs.len() {
        let new_carry = limbs[i] >> 63;
        limbs[i] = (limbs[i] << 1) | carry;
        carry = new_carry;
        i += 1;
    }
    carry
}

/// `true` if every limb above index 0 is zero — fits a single u64.
#[inline]
pub(crate) const fn fit_one(limbs: &[u64]) -> bool {
    fit_k(limbs, 1)
}

/// `true` if every limb at or above index `limb_count` is zero — i.e. the
/// magnitude fits `limb_count` u64 limbs (`< 2^(64·limb_count)`). A slice
/// shorter than `limb_count` trivially fits. Generalises [`fit_one`]
/// (`fit_k(limbs, 1)`); `fit_k(limbs, 2)` is the "`< 2^128`" gate the
/// u128/u256 fast paths key on.
#[inline]
pub(crate) const fn fit_k(limbs: &[u64], limb_count: usize) -> bool {
    let mut i = limb_count;
    while i < limbs.len() {
        if limbs[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Signed three-way compare for u64-limb magnitudes with signs.
#[inline]
pub(crate) const fn scmp(lhs_is_negative: bool, lhs: &[u64], rhs_is_negative: bool,
    rhs: &[u64]) -> i32 {
    match (lhs_is_negative, rhs_is_negative) {
        (true, false) => -1,
        (false, true) => 1,
        _ => cmp(lhs, rhs),
    }
}
