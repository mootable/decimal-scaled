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
/// derives from it via [`work_scratch`] — ONE feature-gated const + one
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
/// proven `288 = work_scratch(4)` at xx-wide). Kernels expand in limbs
/// rather than a work *type* `Int<2N>`/`Int<4N>` (unnameable from `N` on
/// stable; see the algorithim-optimiser skill §5). The exact per-`N`
/// alternatives ([`work_scratch_n`], the `exact-scratch` impls) live in
/// `crate::int::types::work_scratch`.
pub(crate) const fn work_scratch(mult: usize) -> usize {
    mult * MAX_WORK_N + (MAX_WORK_N + 1) / 2
}

/// Exact per-`N` work-scratch budget: `mult·n + ceil(n/2)`, the same
/// formula as [`work_scratch`] but for a *specific* limb count `n` rather
/// than the build-max. Used by the `exact-scratch-nightly` blanket
/// [`WorkScratch`] impl, where it appears as a `generic_const_exprs` array
/// length confined to that impl block.
///
/// [`WorkScratch`]: crate::int::types::work_scratch::WorkScratch
#[cfg(feature = "exact-scratch-nightly")]
pub(crate) const fn work_scratch_n(mult: usize, n: usize) -> usize {
    mult * n + (n + 1) / 2
}

/// `a == 0`.
#[inline]
pub(crate) const fn is_zero(a: &[u64]) -> bool {
    let mut i = 0;
    while i < a.len() {
        if a[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Fixed-width specialisation of [`is_zero`]. `L` const at callsite, lets
/// LLVM unroll for small `L`.
#[inline]
pub(crate) const fn is_zero_fixed<const L: usize>(a: &[u64; L]) -> bool {
    let mut i = 0;
    while i < L {
        if a[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// `a == b` for two limb slices of possibly different lengths.
#[inline]
pub(crate) const fn eq(a: &[u64], b: &[u64]) -> bool {
    let n = if a.len() > b.len() { a.len() } else { b.len() };
    let mut i = 0;
    while i < n {
        let av = if i < a.len() { a[i] } else { 0 };
        let bv = if i < b.len() { b[i] } else { 0 };
        if av != bv {
            return false;
        }
        i += 1;
    }
    true
}

/// Three-way comparison `-1`/`0`/`1`.
#[inline]
pub(crate) const fn cmp(a: &[u64], b: &[u64]) -> i32 {
    let n = if a.len() > b.len() { a.len() } else { b.len() };
    let mut i = n;
    while i > 0 {
        i -= 1;
        let av = if i < a.len() { a[i] } else { 0 };
        let bv = if i < b.len() { b[i] } else { 0 };
        if av < bv {
            return -1;
        }
        if av > bv {
            return 1;
        }
    }
    0
}

/// Fixed-width specialisation of [`cmp`] — both operands the same `L`; no
/// length-difference handling needed.
#[inline]
pub(crate) const fn cmp_fixed<const L: usize>(a: &[u64; L], b: &[u64; L]) -> i32 {
    let mut i = L;
    while i > 0 {
        i -= 1;
        if a[i] < b[i] {
            return -1;
        }
        if a[i] > b[i] {
            return 1;
        }
    }
    0
}

/// Cross-width unsigned magnitude comparison of two little-endian limb
/// slices of possibly different lengths. Returns `-1` / `0` / `1` for
/// `a < b` / `a == b` / `a > b`. The surplus high limbs of the longer
/// slice must all be zero for the magnitudes to be equal there; any
/// non-zero surplus limb makes that side the larger. No widening copy is
/// made — the slices are compared in place. Const.
#[inline]
pub(crate) const fn cmp_cross(a: &[u64], b: &[u64]) -> i32 {
    let la = a.len();
    let lb = b.len();
    let max = if la > lb { la } else { lb };
    let mut i = max;
    while i > 0 {
        i -= 1;
        let av = if i < la { a[i] } else { 0 };
        let bv = if i < lb { b[i] } else { 0 };
        if av < bv {
            return -1;
        }
        if av > bv {
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
pub(crate) const fn bit_len(a: &[u64]) -> u32 {
    let mut i = a.len();
    while i > 0 {
        i -= 1;
        if a[i] != 0 {
            return (i as u32) * 64 + (64 - a[i].leading_zeros());
        }
    }
    0
}

/// Fixed-width specialisation of [`bit_len`]: significant bits of the
/// non-negative magnitude held in `a` (`0` for zero).
#[inline]
pub(crate) const fn bit_len_fixed<const L: usize>(a: &[u64; L]) -> u32 {
    let mut i = L;
    while i > 0 {
        i -= 1;
        if a[i] != 0 {
            return (i as u32) * 64 + (64 - a[i].leading_zeros());
        }
    }
    0
}

/// `a += b`, returns carry out. `a.len() >= b.len()`.
#[inline]
pub(crate) const fn add_assign(a: &mut [u64], b: &[u64]) -> bool {
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < a.len() {
        let bv = if i < b.len() { b[i] } else { 0 };
        let (s1, c1) = a[i].overflowing_add(bv);
        let (s2, c2) = s1.overflowing_add(carry);
        a[i] = s2;
        carry = (c1 as u64) + (c2 as u64);
        i += 1;
    }
    carry != 0
}

/// Fixed-width specialisation of [`add_assign`] — both operands the same
/// `L`.
#[inline]
pub(crate) const fn add_assign_fixed<const L: usize>(a: &mut [u64; L], b: &[u64; L]) -> bool {
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < L {
        let (s1, c1) = a[i].overflowing_add(b[i]);
        let (s2, c2) = s1.overflowing_add(carry);
        a[i] = s2;
        carry = (c1 as u64) + (c2 as u64);
        i += 1;
    }
    carry != 0
}

/// `a -= b`, returns borrow out. `a.len() >= b.len()`.
#[inline]
pub(crate) const fn sub_assign(a: &mut [u64], b: &[u64]) -> bool {
    let mut borrow: u64 = 0;
    let mut i = 0;
    while i < a.len() {
        let bv = if i < b.len() { b[i] } else { 0 };
        let (d1, b1) = a[i].overflowing_sub(bv);
        let (d2, b2) = d1.overflowing_sub(borrow);
        a[i] = d2;
        borrow = (b1 as u64) + (b2 as u64);
        i += 1;
    }
    borrow != 0
}

/// Fixed-width specialisation of [`sub_assign`].
#[inline]
pub(crate) const fn sub_assign_fixed<const L: usize>(a: &mut [u64; L], b: &[u64; L]) -> bool {
    let mut borrow: u64 = 0;
    let mut i = 0;
    while i < L {
        let (d1, b1) = a[i].overflowing_sub(b[i]);
        let (d2, b2) = d1.overflowing_sub(borrow);
        a[i] = d2;
        borrow = (b1 as u64) + (b2 as u64);
        i += 1;
    }
    borrow != 0
}

/// Fixed-width specialisation of [`shl`]. `L` const, but `shift` is still
/// runtime — bounds checks vanish, the inner loop trip count is known.
#[inline]
pub(crate) const fn shl_fixed<const L: usize>(a: &[u64; L], shift: u32, out: &mut [u64; L]) {
    let mut z = 0;
    while z < L {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit = shift % 64;
    let mut i = 0;
    while i < L {
        let dst = i + limb_shift;
        if dst < L {
            if bit == 0 {
                out[dst] |= a[i];
            } else {
                out[dst] |= a[i] << bit;
                if dst + 1 < L {
                    out[dst + 1] |= a[i] >> (64 - bit);
                }
            }
        }
        i += 1;
    }
}

/// Fixed-width specialisation of [`shr`].
#[inline]
pub(crate) const fn shr_fixed<const L: usize>(a: &[u64; L], shift: u32, out: &mut [u64; L]) {
    let mut z = 0;
    while z < L {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit = shift % 64;
    let mut i = limb_shift;
    while i < L {
        let dst = i - limb_shift;
        if dst < L {
            if bit == 0 {
                out[dst] |= a[i];
            } else {
                out[dst] |= a[i] >> bit;
                if dst >= 1 {
                    out[dst - 1] |= a[i] << (64 - bit);
                }
            }
        }
        i += 1;
    }
}

/// `out = a << shift`. `out` is zeroed then filled.
pub(crate) const fn shl(a: &[u64], shift: u32, out: &mut [u64]) {
    let mut z = 0;
    while z < out.len() {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit = shift % 64;
    let mut i = 0;
    while i < a.len() {
        let dst = i + limb_shift;
        if dst < out.len() {
            if bit == 0 {
                out[dst] |= a[i];
            } else {
                out[dst] |= a[i] << bit;
                if dst + 1 < out.len() {
                    out[dst + 1] |= a[i] >> (64 - bit);
                }
            }
        }
        i += 1;
    }
}

/// `out = a >> shift`. `out` is zeroed then filled.
pub(crate) const fn shr(a: &[u64], shift: u32, out: &mut [u64]) {
    let mut z = 0;
    while z < out.len() {
        out[z] = 0;
        z += 1;
    }
    let limb_shift = (shift / 64) as usize;
    let bit = shift % 64;
    let mut i = limb_shift;
    while i < a.len() {
        let dst = i - limb_shift;
        if dst < out.len() {
            if bit == 0 {
                out[dst] |= a[i];
            } else {
                out[dst] |= a[i] >> bit;
                if dst >= 1 {
                    out[dst - 1] |= a[i] << (64 - bit);
                }
            }
        }
        i += 1;
    }
}

/// Single-bit left shift in place; returns the bit shifted out.
#[inline]
pub(crate) const fn shl1(a: &mut [u64]) -> u64 {
    let mut carry: u64 = 0;
    let mut i = 0;
    while i < a.len() {
        let new_carry = a[i] >> 63;
        a[i] = (a[i] << 1) | carry;
        carry = new_carry;
        i += 1;
    }
    carry
}

/// `true` if every limb above index 0 is zero — fits a single u64.
#[inline]
pub(crate) const fn fit_one(a: &[u64]) -> bool {
    let mut i = 1;
    while i < a.len() {
        if a[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Signed three-way compare for u64-limb magnitudes with signs.
#[inline]
pub(crate) const fn scmp(a_neg: bool, a: &[u64], b_neg: bool, b: &[u64]) -> i32 {
    match (a_neg, b_neg) {
        (true, false) => -1,
        (false, true) => 1,
        _ => cmp(a, b),
    }
}
