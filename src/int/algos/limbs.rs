//! Generic little-endian `u64` limb arithmetic.
//!
//! The integer layer's primitive bucket: the raw `&[u64]` slice
//! arithmetic that the const-generic `Int<N>` / `Uint<N>` types and the
//! width-matched [`crate::int::algos`] routines compose on. Every routine
//! treats its `&[u64]` slices as little-endian unsigned integers
//! (`limbs[0]` least significant); lengths are taken from the slices and
//! callers size the output buffers.
//!
//! The core routines are `const fn` so the integer types built on them
//! can expose `const` constructors and constants. Hardware has a native
//! `u64 × u64 → u128` widening multiply and a native `u128 / u64` divide,
//! so these limb kernels map directly onto machine instructions.
//!
//! The module path already says "limbs", so the routines drop the legacy
//! `limbs_` prefix and the `_u64` suffix; width stays the const-generic
//! `L` parameter on the `_fixed` siblings. The two multiply algorithms
//! ([`mul_schoolbook`] and [`mul_karatsuba`]) are kept *pure* — they each
//! perform one named algorithm; the schoolbook-vs-Karatsuba *choice*
//! lives in [`crate::int::policy::mul`].

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

/// `out = (x²) mod 2^(64·N)` — dedicated truncated squaring.
///
/// `out` must be zeroed by the caller. A square has symmetric cross terms
/// (`x_i·x_j == x_j·x_i`), so each `i < j` partial product is formed once
/// and added twice (the doubling), while the `i == j` diagonals `x_i²`
/// are added once. Limb-multiply count drops from `N²` (general mul) to
/// `N(N+1)/2`. As with [`mul_low_fixed`], any product whose column index
/// reaches `N` is above the width and is dropped, so only the low `N`
/// limbs are touched. Bit-identical to the low `N` limbs of `x · x`.
///
/// Each `add_at` folds a `(hi, lo)` pair into `out[col]` / `out[col+1]`
/// and propagates the carry through the remaining low limbs. The
/// cross-term doubling is realised by calling `add_at` twice rather than
/// a separate shift, which keeps the carry handling — the only fiddly
/// part of squaring — identical to the diagonal path.
#[inline]
pub(crate) const fn sqr_low_fixed<const N: usize>(x: &[u64; N], out: &mut [u64; N]) {
    // Fold `value` (a u128 partial product) into the low limbs starting
    // at `col`, propagating carry until exhausted or past the width.
    #[inline(always)]
    const fn add_at<const N: usize>(out: &mut [u64; N], col: usize, value: u128) {
        if col >= N {
            return;
        }
        let mut idx = col;
        let mut carry = value;
        while carry != 0 && idx < N {
            let v = (out[idx] as u128) + (carry & 0xFFFF_FFFF_FFFF_FFFF);
            out[idx] = v as u64;
            // Surviving carry = high 64 of this column's sum plus the
            // high 64 of the incoming value that has not been consumed.
            carry = (v >> 64) + (carry >> 64);
            idx += 1;
        }
    }

    let mut i = 0;
    while i < N {
        let xi = x[i] as u128;
        if xi != 0 {
            // Diagonal square at column 2i (added once).
            add_at::<N>(out, i + i, xi * xi);
            // Doubled cross terms x_i·x_j for j > i at column i+j.
            let mut j = i + 1;
            while i + j < N {
                let prod = xi * (x[j] as u128);
                add_at::<N>(out, i + j, prod);
                add_at::<N>(out, i + j, prod);
                j += 1;
            }
        }
        i += 1;
    }
}

// ── Karatsuba multiplication (pure kernel; the schoolbook-vs-Karatsuba
//    *choice* lives in `int::policy::mul`) ─────────────────────────────

/// Stack scratch for the non-allocating Karatsuba kernel, in u64 limbs.
///
/// Each recursion level on an `n`-limb operand carves three product
/// windows (`z0 ≈ 2·⌈n/2⌉`, `z2 ≈ 2·⌈n/2⌉`, `z1 ≈ 2·(⌈n/2⌉+1)`) plus
/// two `(⌈n/2⌉+1)`-limb sum windows off the front of the buffer, then
/// hands the tail to the three (sequential) child calls. That is
/// `S(n) ≤ 6n + O(1)` limbs per level; recursing on the halves gives a
/// geometric total `K(n) = S(n) + S(n/2) + … ≤ 2·S(n) ≤ 12n + O(log n)`.
/// For the widest equal-length multiply the crate performs — `n = 256`
/// limbs (Int<256>) — that bound is `≤ 12·256 ≈ 3072`; rounded up with
/// headroom. ~25 KiB on the stack, recursion depth `log2(256) = 8`.
const KARATSUBA_SCRATCH_LIMBS: usize = 3200;

/// Non-allocating recursive Karatsuba multiplication at u64 base.
///
/// Reference: Karatsuba & Ofman 1962, "Multiplication of Multidigit
/// Numbers on Automata" (Doklady Akad. Nauk SSSR 145, 293-294). Splits
/// both equal-length operands at half: `a = a₁·B + a₀`, `b = b₁·B + b₀`,
/// computes three half-width sub-products `z₀ = a₀·b₀`, `z₂ = a₁·b₁`,
/// `z₁ = (a₀+a₁)·(b₀+b₁) − z₀ − z₂`, then recombines as
/// `z₂·B² + z₁·B + z₀`.
///
/// All temporaries live in a single fixed `[u64; KARATSUBA_SCRATCH_LIMBS]`
/// stack buffer declared once at this public entry point; the recursion
/// carves disjoint windows out of it with `split_at_mut` (so the borrow
/// checker proves non-aliasing — no `unsafe`, no `Vec`, available in
/// `no_std`/no-alloc builds). The three child products run sequentially
/// and share the same scratch tail, which keeps the total at `~2·S(n)`.
///
/// Operands must be equal length. `out.len() >= 2 * a.len()` and `out`
/// must be zeroed by the caller. The crossover threshold against
/// [`mul_schoolbook`] is supplied by [`crate::int::policy::mul`].
pub(crate) fn mul_karatsuba(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        karatsuba_scratch_needed_th(a.len(), threshold) <= KARATSUBA_SCRATCH_LIMBS,
        "Karatsuba scratch overflow: n={} needs {} limbs, have {}",
        a.len(),
        karatsuba_scratch_needed_th(a.len(), threshold),
        KARATSUBA_SCRATCH_LIMBS,
    );
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Bench-only entry that drives the production [`karatsuba_rec`] over the
/// real fixed `[u64; KARATSUBA_SCRATCH_LIMBS]` stack scratch at an
/// arbitrary `threshold`, so the crossover sweep can time the kernel at
/// widths below the parked production threshold. `out` is zeroed here.
#[cfg(feature = "bench-alt")]
pub(crate) fn mul_karatsuba_forced(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        karatsuba_scratch_needed_th(a.len(), threshold) <= KARATSUBA_SCRATCH_LIMBS,
        "Karatsuba scratch overflow in forced bench entry"
    );
    for o in out.iter_mut() {
        *o = 0;
    }
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Test-only entry that drives the production [`karatsuba_rec`] at an
/// arbitrary `threshold`, sizing the scratch for the deeper recursion a
/// small threshold induces. Lets the correctness test exercise the real
/// split/recombine algebra at every width without depending on the
/// shipped threshold.
#[cfg(test)]
pub(crate) fn mul_karatsuba_with_threshold(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    let need = karatsuba_scratch_needed_th(a.len(), threshold);
    let mut scratch = alloc::vec![0u64; need];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Threshold-parameterised upper bound on the scratch (in u64 limbs) the
/// non-allocating Karatsuba recursion consumes for an `n`-limb
/// equal-length multiply. Mirrors the per-level carve in [`karatsuba_rec`]:
/// `S(n) = 2h + 2hi + 2(hi+1) + 2(hi+1)` with `h = n/2`, `hi = n - h`,
/// plus the geometric tail down the largest-child spine.
const fn karatsuba_scratch_needed_th(n: usize, threshold: usize) -> usize {
    if n < threshold {
        return 0;
    }
    let h = n / 2;
    let hi = n - h;
    let level = 2 * h + 2 * hi + (hi + 1) + (hi + 1) + 2 * (hi + 1);
    // The deepest child is the z1 product on `hi + 1`-limb operands (the
    // sum windows), larger than the z0/z2 children — size for it.
    level + karatsuba_scratch_needed_th(hi + 1, threshold)
}

/// One Karatsuba recursion level. `out` is pre-zeroed by the caller for
/// the `2·n`-limb window; `scratch` is the live tail of the entry buffer.
/// Children below `threshold` base-case to [`mul_schoolbook`].
fn karatsuba_rec(a: &[u64], b: &[u64], out: &mut [u64], scratch: &mut [u64], threshold: usize) {
    debug_assert!(
        threshold >= 4,
        "Karatsuba threshold must be >= 4 to terminate"
    );
    let n = a.len();
    if n < threshold {
        // `out` window pre-zeroed by the caller.
        mul_schoolbook(a, b, out);
        return;
    }
    let h = n / 2;
    let hi = n - h; // hi == h or h + 1 (n odd)
    let (a_lo, a_hi) = a.split_at(h);
    let (b_lo, b_hi) = b.split_at(h);

    // Carve this level's windows off the FRONT of scratch; the TAIL is
    // handed down to the (sequential) child calls. `split_at_mut` proves
    // disjointness — no aliasing, all safe Rust.
    let (z0, rest) = scratch.split_at_mut(2 * h);
    let (z2, rest) = rest.split_at_mut(2 * hi);
    let (sa, rest) = rest.split_at_mut(hi + 1);
    let (sb, rest) = rest.split_at_mut(hi + 1);
    let (z1, tail) = rest.split_at_mut(2 * (hi + 1));

    for v in z0.iter_mut() {
        *v = 0;
    }
    for v in z2.iter_mut() {
        *v = 0;
    }
    for v in z1.iter_mut() {
        *v = 0;
    }

    // z0 = a_lo · b_lo (both h limbs), z2 = a_hi · b_hi (both hi limbs).
    // The children run one at a time, so each may reuse `tail`.
    karatsuba_rec(a_lo, b_lo, z0, tail, threshold);
    karatsuba_rec_unbalanced(a_hi, b_hi, z2, tail, threshold);

    // sa = a_lo + a_hi, sb = b_lo + b_hi (each fits hi + 1 limbs with one
    // limb of carry headroom — sa, sb are pre-zeroed by the carve below).
    for v in sa.iter_mut() {
        *v = 0;
    }
    for v in sb.iter_mut() {
        *v = 0;
    }
    sa[..h].copy_from_slice(a_lo);
    sb[..h].copy_from_slice(b_lo);
    let _ = add_assign(sa, a_hi);
    let _ = add_assign(sb, b_hi);

    // z1 = sa · sb − z0 − z2  (sa, sb both hi + 1 limbs).
    karatsuba_rec_unbalanced(sa, sb, z1, tail, threshold);
    let _ = sub_assign(z1, z0);
    let _ = sub_assign(z1, z2);

    // Recombine into the pre-zeroed out: out = z0 + z2·B² + z1·B
    // (B = 2^(64·h)). z0 lands at offset 0, z2 at 2h, z1 at h — the
    // overlap-adds use carry-propagating add, not copy.
    out[..z0.len()].copy_from_slice(z0);
    let _ = add_assign(&mut out[2 * h..], z2);
    let _ = add_assign(&mut out[h..], z1);
}

/// Karatsuba child dispatch for the equal-length sub-products that may
/// drop below the threshold (or be the `hi + 1`-limb sum product). Both
/// operands are equal length here; route to the recursion above the
/// threshold, else zero the window and base-case to [`mul_schoolbook`].
fn karatsuba_rec_unbalanced(
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
    scratch: &mut [u64],
    threshold: usize,
) {
    debug_assert_eq!(a.len(), b.len());
    if a.len() >= threshold {
        karatsuba_rec(a, b, out, scratch, threshold);
    } else {
        for v in out.iter_mut() {
            *v = 0;
        }
        mul_schoolbook(a, b, out);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Pack a `[u128; N]` little-endian limb array into `[u64; 2*N]`.
    fn pack(limbs: &[u128]) -> alloc::vec::Vec<u64> {
        let mut out = alloc::vec![0u64; 2 * limbs.len()];
        for (i, &l) in limbs.iter().enumerate() {
            out[2 * i] = l as u64;
            out[2 * i + 1] = (l >> 64) as u64;
        }
        out
    }

    fn corpus() -> alloc::vec::Vec<alloc::vec::Vec<u128>> {
        alloc::vec![
            alloc::vec![0u128, 0, 0, 0],
            alloc::vec![1u128, 0, 0, 0],
            alloc::vec![u128::MAX, 0, 0, 0],
            alloc::vec![u128::MAX, u128::MAX, 0, 0],
            alloc::vec![u128::MAX, u128::MAX, u128::MAX, u128::MAX],
            alloc::vec![123u128, 456, 0, 0],
            alloc::vec![
                0x1234_5678_9abc_def0_fedc_ba98_7654_3210_u128,
                0xa5a5_a5a5_5a5a_5a5a_3c3c_3c3c_c3c3_c3c3,
                0,
                0,
            ],
        ]
    }

    /// `mul_karatsuba` matches `mul_schoolbook` on equal-length operands
    /// across the carry-stressing corpus at a forced low threshold (so the
    /// split/recombine algebra is exercised even at narrow widths).
    #[test]
    fn karatsuba_matches_schoolbook() {
        for a in corpus() {
            for b in corpus() {
                let a64 = pack(&a);
                let b64 = pack(&b);
                let n = a64.len().min(b64.len());
                let mut a_buf = alloc::vec![0u64; n];
                let mut b_buf = alloc::vec![0u64; n];
                a_buf.copy_from_slice(&a64[..n]);
                b_buf.copy_from_slice(&b64[..n]);
                let mut out_school = alloc::vec![0u64; 2 * n];
                let mut out_kara = alloc::vec![0u64; 2 * n];
                mul_schoolbook(&a_buf, &b_buf, &mut out_school);
                mul_karatsuba_with_threshold(&a_buf, &b_buf, &mut out_kara, 4);
                assert_eq!(out_kara, out_school, "Karatsuba mismatch at n={n}");
            }
        }
    }

    /// Non-allocating Karatsuba is bit-exact against the schoolbook oracle
    /// [`mul_schoolbook`] over a large seeded corpus across every width the
    /// crate multiplies, including odd, threshold-boundary, and the
    /// 256-limb maximum. The recursion is driven at small thresholds so
    /// the full split/recombine algebra is exercised even at the narrow
    /// widths. Commutativity (`a·b == b·a`) is asserted in the same pass.
    #[test]
    fn nonalloc_karatsuba_bit_exact_vs_schoolbook() {
        // SplitMix64 — Vigna 2014, public-domain reference algorithm.
        let mut state: u64 = 0x5EED_1234_ABCD_0F0F;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };

        const WIDTHS: &[usize] = &[
            1, 2, 4, 7, 8, 15, 16, 17, 31, 32, 33, 48, 64, 96, 128, 255, 256,
        ];
        const THRESHOLDS: &[usize] = &[4, 8, 16, 24, 256];

        let edge_fill = |buf: &mut [u64], kind: usize, next: &mut dyn FnMut() -> u64| match kind {
            0 => buf.iter_mut().for_each(|x| *x = 0),
            1 => buf.iter_mut().for_each(|x| *x = u64::MAX),
            2 => {
                buf.iter_mut().for_each(|x| *x = 0);
                if let Some(last) = buf.last_mut() {
                    *last = u64::MAX;
                }
            }
            3 => {
                buf.iter_mut().for_each(|x| *x = 0);
                buf[0] = u64::MAX;
            }
            _ => buf.iter_mut().for_each(|x| *x = next()),
        };

        for &n in WIDTHS {
            let random_pairs = if n <= 16 {
                400
            } else if n <= 64 {
                120
            } else {
                30
            };

            let mut pairs: alloc::vec::Vec<(alloc::vec::Vec<u64>, alloc::vec::Vec<u64>)> =
                alloc::vec::Vec::new();
            for ka in 0..5 {
                for kb in 0..5 {
                    let mut a = alloc::vec![0u64; n];
                    let mut b = alloc::vec![0u64; n];
                    edge_fill(&mut a, ka, &mut next);
                    edge_fill(&mut b, kb, &mut next);
                    pairs.push((a, b));
                }
            }
            for _ in 0..random_pairs {
                let mut a = alloc::vec![0u64; n];
                let mut b = alloc::vec![0u64; n];
                for x in a.iter_mut() {
                    *x = next();
                }
                for x in b.iter_mut() {
                    *x = next();
                }
                pairs.push((a, b));
            }

            for (a, b) in &pairs {
                let mut oracle = alloc::vec![0u64; 2 * n];
                mul_schoolbook(a, b, &mut oracle);

                for &th in THRESHOLDS {
                    let mut got = alloc::vec![0u64; 2 * n];
                    mul_karatsuba_with_threshold(a, b, &mut got, th);
                    assert_eq!(
                        got, oracle,
                        "non-alloc Karatsuba mismatch at n={n}, threshold={th}\na={a:?}\nb={b:?}"
                    );

                    let mut got_swapped = alloc::vec![0u64; 2 * n];
                    mul_karatsuba_with_threshold(b, a, &mut got_swapped, th);
                    assert_eq!(
                        got_swapped, oracle,
                        "non-alloc Karatsuba not commutative at n={n}, threshold={th}"
                    );
                }
            }
        }
    }

    /// The widest equal-length multiply (256 limbs, Int<256>) routes
    /// through the production [`mul_karatsuba`] entry — which declares the
    /// fixed `[u64; KARATSUBA_SCRATCH_LIMBS]` stack buffer — without
    /// tripping the scratch-overflow `debug_assert` and matches schoolbook.
    /// Guards the scratch sizing against future threshold drops that deepen
    /// the recursion.
    #[test]
    fn nonalloc_karatsuba_max_width_fits_fixed_scratch() {
        let mut state: u64 = 0xC0FF_EE00_1357_9BDF;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        assert!(
            karatsuba_scratch_needed_th(256, 8) <= KARATSUBA_SCRATCH_LIMBS,
            "fixed scratch too small for n=256 at a threshold of 8"
        );

        let n = 256;
        let mut a = alloc::vec![0u64; n];
        let mut b = alloc::vec![0u64; n];
        for x in a.iter_mut() {
            *x = next();
        }
        for x in b.iter_mut() {
            *x = next();
        }
        let mut oracle = alloc::vec![0u64; 2 * n];
        let mut got = alloc::vec![0u64; 2 * n];
        mul_schoolbook(&a, &b, &mut oracle);
        // Production entry: real fixed stack scratch, production threshold.
        mul_karatsuba(&a, &b, &mut got, super::super::super::policy::mul::karatsuba_threshold());
        assert_eq!(got, oracle, "max-width Karatsuba mismatch via fixed scratch");
    }

    /// `mul_schoolbook_fixed::<L, D>` matches `mul_schoolbook` at a
    /// representative set of compile-time `L` values covering every wide
    /// tier (D38..D1232).
    #[test]
    fn mul_schoolbook_fixed_matches_slice() {
        macro_rules! check {
            ($L:expr, $D:expr) => {{
                for a in corpus() {
                    for b in corpus() {
                        let a64 = pack(&a);
                        let b64 = pack(&b);
                        if a64.len() < $L || b64.len() < $L {
                            continue;
                        }
                        let mut a_arr = [0u64; $L];
                        let mut b_arr = [0u64; $L];
                        a_arr.copy_from_slice(&a64[..$L]);
                        b_arr.copy_from_slice(&b64[..$L]);
                        let mut out_slice = alloc::vec![0u64; $D];
                        let mut out_fixed = [0u64; $D];
                        mul_schoolbook(&a_arr, &b_arr, &mut out_slice);
                        mul_schoolbook_fixed::<$L, $D>(&a_arr, &b_arr, &mut out_fixed);
                        assert_eq!(
                            &out_slice[..],
                            &out_fixed[..],
                            "mul_schoolbook_fixed::<{}, {}> mismatch",
                            $L,
                            $D
                        );
                    }
                }
            }};
        }
        check!(2, 4);
        check!(4, 8);
        check!(8, 16);
        check!(16, 32);
        check!(24, 48);
        check!(32, 64);
        check!(48, 96);
        check!(64, 128);
    }

    /// `mul_schoolbook_into::<L, L+1>` matches `mul_schoolbook_fixed::<L, 2·L>`
    /// when the wider operand is `[n, 0, ..., 0]`, across L covering every
    /// wide tier from D38 (L=2) to D307 (L=16).
    #[test]
    fn mul_schoolbook_into_matches_fixed() {
        // SplitMix64 — Vigna 2014, public-domain reference algorithm.
        let mut state: u64 = 0xDEAD_BEEF_CAFE_F00D;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };

        macro_rules! check_into {
            ($L:expr, $LP1:expr, $D:expr) => {{
                for _ in 0..1000 {
                    let mut a = [0u64; $L];
                    for slot in a.iter_mut() {
                        *slot = next();
                    }
                    let n = next();

                    let mut out_into = [0u64; $LP1];
                    mul_schoolbook_into::<$L, $LP1>(&a, n, &mut out_into);

                    let mut b = [0u64; $L];
                    b[0] = n;
                    let mut out_fixed = [0u64; $D];
                    mul_schoolbook_fixed::<$L, $D>(&a, &b, &mut out_fixed);

                    assert_eq!(
                        &out_into[..],
                        &out_fixed[..$LP1],
                        "mul_schoolbook_into::<{}, {}> low limbs mismatch (a={:?}, n={:#x})",
                        $L,
                        $LP1,
                        a,
                        n
                    );
                    for (k, &limb) in out_fixed[$LP1..].iter().enumerate() {
                        assert_eq!(
                            limb,
                            0,
                            "mul_schoolbook_fixed high limb {} not zero",
                            $LP1 + k
                        );
                    }
                }
            }};
        }
        check_into!(2, 3, 4);
        check_into!(3, 4, 6);
        check_into!(4, 5, 8);
        check_into!(6, 7, 12);
        check_into!(8, 9, 16);
        check_into!(16, 17, 32);
    }

    /// `mul_low_fixed` matches the low `N` limbs of the full product.
    #[test]
    fn mul_low_matches_full_product_low_half() {
        const N: usize = 4;
        const D: usize = 8;
        for a in corpus() {
            for b in corpus() {
                let a64 = pack(&a);
                let b64 = pack(&b);
                let mut a_arr = [0u64; N];
                let mut b_arr = [0u64; N];
                a_arr.copy_from_slice(&a64[..N]);
                b_arr.copy_from_slice(&b64[..N]);
                let mut full = [0u64; D];
                mul_schoolbook_fixed::<N, D>(&a_arr, &b_arr, &mut full);
                let mut low = [0u64; N];
                mul_low_fixed::<N>(&a_arr, &b_arr, &mut low);
                assert_eq!(&full[..N], &low[..], "mul_low_fixed mismatch");
            }
        }
    }
}
