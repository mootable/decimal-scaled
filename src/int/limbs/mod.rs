//! Raw limb primitives and the named-type re-exports.
//!
//! The integer layer's primitive bucket (absorbed from the former
//! `src/wide_int/`): the raw `&[u64]` slice arithmetic that the
//! const-generic `Int<N>` / `Uint<N>` types and the width-matched
//! [`crate::int::algos`] compose on. The named signed `Int*` / unsigned
//! `Uint*` two's-complement integers from 192 to 16384 bits are now
//! `pub type` aliases over `Int<N>` / `Uint<N>` (defined in
//! [`crate::int::types`]) and re-exported here so the historic
//! `crate::wide_int::IntXXXX` paths keep resolving. The module depends
//! on nothing else in the crate and is structured so it can later be
//! lifted into a standalone crate.
//!
//! # Structure
//!
//! - **Slice primitives** — the actual arithmetic, written over `&[u64]`
//!   limb slices (little-endian, `limbs[0]` least significant).
//!   Operating on slices sidesteps the const-generic return-type problem
//!   a widening multiply would otherwise hit; hardware's native
//!   `u64 × u64 → u128` widening multiply and `u128 / u64` divide map
//!   directly onto these kernels. The core routines are `const fn` so
//!   the integer types built on them can expose `const` constructors and
//!   constants.
//! - The named `Uint* / Int*` aliases over the const-generic types.

// On `no_std` the f64 inherent methods (`floor` / `sqrt`) used by the
// `from_f64` / integer-sqrt seed paths are unavailable; pull them in via
// `num_traits::Float` (libm-backed). Under `std` the inherent methods win,
// so this import is gated out to avoid an unused-import warning and to keep
// the std float path bit-for-bit unchanged.
#[cfg(not(feature = "std"))]
use num_traits::Float as _;

// ─────────────────────────────────────────────────────────────────────
// u64 limb primitives — unsigned limb-array arithmetic.
//
// Every routine treats its `&[u64]` slices as little-endian unsigned
// integers (`limbs[0]` least significant); lengths are taken from the
// slices and callers size the output buffers. The core routines are
// `const fn` so the integer types built on them can expose `const`
// constructors and constants.
//
// Hardware has a native `u64 × u64 → u128` widening multiply and a
// native `u128 / u64` divide, so these limb kernels map directly onto
// machine instructions.
// ─────────────────────────────────────────────────────────────────────

/// `a == 0`.
#[inline]
pub(crate) const fn limbs_is_zero_u64(a: &[u64]) -> bool {
    let mut i = 0;
    while i < a.len() {
        if a[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Fixed-width specialisation of [`limbs_is_zero_u64`]. `L` const at
/// callsite, lets LLVM unroll for small `L`.
#[inline]
pub(crate) const fn limbs_is_zero_u64_fixed<const L: usize>(a: &[u64; L]) -> bool {
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
pub(crate) const fn limbs_eq_u64(a: &[u64], b: &[u64]) -> bool {
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
pub(crate) const fn limbs_cmp_u64(a: &[u64], b: &[u64]) -> i32 {
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

/// Fixed-width specialisation of [`limbs_cmp_u64`] — both operands
/// the same `L`; no length-difference handling needed.
#[inline]
pub(crate) const fn limbs_cmp_u64_fixed<const L: usize>(a: &[u64; L], b: &[u64; L]) -> i32 {
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
pub(crate) const fn limbs_cmp_u64_cross(a: &[u64], b: &[u64]) -> i32 {
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
pub(crate) const fn limbs_bit_len_u64(a: &[u64]) -> u32 {
    let mut i = a.len();
    while i > 0 {
        i -= 1;
        if a[i] != 0 {
            return (i as u32) * 64 + (64 - a[i].leading_zeros());
        }
    }
    0
}

/// Fixed-width specialisation of [`limbs_bit_len_u64`]: significant bits
/// of the non-negative magnitude held in `a` (`0` for zero).
#[inline]
pub(crate) const fn limbs_bit_len_u64_fixed<const L: usize>(a: &[u64; L]) -> u32 {
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
pub(crate) const fn limbs_add_assign_u64(a: &mut [u64], b: &[u64]) -> bool {
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

/// Fixed-width specialisation of [`limbs_add_assign_u64`] — both
/// operands the same `L`.
#[inline]
pub(crate) const fn limbs_add_assign_u64_fixed<const L: usize>(
    a: &mut [u64; L],
    b: &[u64; L],
) -> bool {
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
pub(crate) const fn limbs_sub_assign_u64(a: &mut [u64], b: &[u64]) -> bool {
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

/// Fixed-width specialisation of [`limbs_sub_assign_u64`].
#[inline]
pub(crate) const fn limbs_sub_assign_u64_fixed<const L: usize>(
    a: &mut [u64; L],
    b: &[u64; L],
) -> bool {
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

/// Fixed-width specialisation of [`limbs_shl_u64`]. `L` const, but
/// `shift` is still runtime — bounds checks vanish, the inner loop
/// trip count is known.
#[inline]
pub(crate) const fn limbs_shl_u64_fixed<const L: usize>(
    a: &[u64; L],
    shift: u32,
    out: &mut [u64; L],
) {
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

/// Fixed-width specialisation of [`limbs_shr_u64`].
#[inline]
pub(crate) const fn limbs_shr_u64_fixed<const L: usize>(
    a: &[u64; L],
    shift: u32,
    out: &mut [u64; L],
) {
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
pub(crate) const fn limbs_shl_u64(a: &[u64], shift: u32, out: &mut [u64]) {
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
pub(crate) const fn limbs_shr_u64(a: &[u64], shift: u32, out: &mut [u64]) {
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
const fn limbs_shl1_u64(a: &mut [u64]) -> u64 {
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
const fn limbs_fit_one_u64(a: &[u64]) -> bool {
    let mut i = 1;
    while i < a.len() {
        if a[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// `out = a · b` schoolbook. `out.len() >= a.len() + b.len()` and
/// `out` must be zeroed by the caller.
///
/// Inner step uses the native `u64 × u64 → u128` widening mul
/// (`MUL` + `UMULH` on x86-64 / aarch64), avoiding the 4-way
/// `mul_128` decomposition every u128 schoolbook step pays.
pub(crate) const fn limbs_mul_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
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

/// Fixed-width specialisation of [`limbs_mul_u64`]: the operand
/// limb-count `L` and output limb-count `D = 2·L` are both
/// compile-time constants, so the slice indirection and loop-bound
/// checks vanish and LLVM can unroll the inner loop (and, for small
/// `L`, the outer one too).
///
/// Same algorithm and same output as [`limbs_mul_u64`]; faster only
/// when both operands have known-equal length (the common case for
/// wide-tier `widen_mul` where both operands are an `Int{N}` of the
/// tier's storage width).
#[inline]
pub(crate) const fn limbs_mul_u64_fixed<const L: usize, const D: usize>(
    a: &[u64; L],
    b: &[u64; L],
    out: &mut [u64; D],
) {
    debug_assert!(D >= 2 * L, "limbs_mul_u64_fixed: D must be ≥ 2·L");
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
/// fixed-width `L`-limb input, and `out` is a fixed-width
/// `LP1 = L + 1` limb output. `out` must be zeroed by the caller.
///
/// Specialisation of the n-by-1-word multi-precision multiply
/// (Knuth, TAOCP Vol 2 §4.3.1, Algorithm M with `n = 1`):
/// every inner-loop step is a single `u64 × u64 → u128` widening
/// mul plus an accumulator-and-carry fold, so the whole operation
/// is `L` widening muls and `L` adds with no cross-row carry
/// chains. By contrast, [`limbs_mul_u64_fixed`] called with
/// `b = [n, 0, ..., 0]` still runs the `L²` outer-product loop
/// (most iterations are short-circuited on `b[j] == 0`, but the
/// monomorphisation still emits the dead branches and the row
/// carry-propagation tail).
///
/// `LP1` must equal `L + 1`; the caller passes both because Rust
/// stable cannot express `L + 1` in a const generic position.
#[inline(always)]
pub(crate) const fn limbs_mul_u64_into<const L: usize, const LP1: usize>(
    a: &[u64; L],
    n: u64,
    out: &mut [u64; LP1],
) {
    debug_assert!(LP1 == L + 1, "limbs_mul_u64_into: LP1 must equal L + 1");
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

/// `quot = num / den`, `rem = num % den`, u64 limbs.
///
/// Hardware fast paths:
/// - both fit a single u64 → one native `u64 / u64`
/// - divisor fits a single u64 → native `u128 / u64` per dividend limb
/// - otherwise → bit shift-subtract (only reached when divisor is
///   multi-limb; the dispatcher routes those to Knuth instead)
pub(crate) const fn limbs_divmod_u64(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    let mut z = 0;
    while z < quot.len() {
        quot[z] = 0;
        z += 1;
    }
    z = 0;
    while z < rem.len() {
        rem[z] = 0;
        z += 1;
    }

    let den_one_limb = limbs_fit_one_u64(den);

    // Fast path A: both fit a single u64 → hardware divide.
    if den_one_limb && limbs_fit_one_u64(num) {
        if !quot.is_empty() {
            quot[0] = num[0] / den[0];
        }
        if !rem.is_empty() {
            rem[0] = num[0] % den[0];
        }
        return;
    }

    // Fast path B: divisor fits a single u64 — schoolbook base-2^64
    // long divide using the native u128/u64 hardware divide. Note this
    // is the SAME shape as the u128 path's Fast B (which manually
    // splits u128 into 64-bit halves), but here it's one hardware
    // divide per limb instead of two.
    if den_one_limb {
        let d = den[0];
        let mut r: u64 = 0;
        let mut top = num.len();
        while top > 0 && num[top - 1] == 0 {
            top -= 1;
        }
        let mut i = top;
        while i > 0 {
            i -= 1;
            let acc = ((r as u128) << 64) | (num[i] as u128);
            let q = (acc / (d as u128)) as u64;
            r = (acc % (d as u128)) as u64;
            if i < quot.len() {
                quot[i] = q;
            }
        }
        if !rem.is_empty() {
            rem[0] = r;
        }
        return;
    }

    // General path: binary shift-subtract. Only reached for multi-limb
    // divisors when the dispatcher isn't routing to Knuth (i.e. in
    // const contexts where Knuth isn't available).
    let bits = limbs_bit_len_u64(num);
    let mut i = bits;
    while i > 0 {
        i -= 1;
        limbs_shl1_u64(rem);
        let bit = (num[(i / 64) as usize] >> (i % 64)) & 1;
        rem[0] |= bit;
        limbs_shl1_u64(quot);
        if limbs_cmp_u64(rem, den) >= 0 {
            limbs_sub_assign_u64(rem, den);
            quot[0] |= 1;
        }
    }
}

/// Scratch capacity for the runtime u64-limb kernels — 144 u64 limbs
/// (9216 bits), matching the u128 path's 72-limb scratch.
// 288 u64 limbs = 18432 bits — covers the widest work integer in
// the crate (Int<256> used by D1232 cbrt, 256 u64 limbs) with isqrt
// scratch slack.
const SCRATCH_LIMBS_U64: usize = 288;

/// Karatsuba threshold for the u64-base multiplier: the operand
/// limb-count at or above which [`limbs_mul_fast_u64`] routes through
/// the non-allocating Karatsuba kernel instead of schoolbook.
///
/// The dispatcher [`limbs_mul_fast_u64`] is the single site every
/// equal-length wide multiply flows through (via the `Int<N>` widening
/// product), so one threshold governs the crossover for every tier from
/// one place. Set at **256 u64 limbs** — above the widest equal-length
/// multiply the crate emits (D1232 storage = 64 limbs; the widest
/// transcendental work-int is 192–256 limbs). At this setting every
/// shipped tier base-cases to the LLVM-unrolled schoolbook
/// [`limbs_mul_u64`], so the kernel is reachable and correct without
/// changing the product behaviour of any shipped width.
///
/// Why above the shipped widths: a focused u64 microbench
/// (`examples/karabench_u64.rs`) of the non-alloc kernel against
/// schoolbook at the wide-tier storage widths — L = 24 (D462), 32
/// (D616), 48 (D924), 64 (D1232) — shows schoolbook at break-even or
/// faster across the whole band, at every recursion base case tried
/// (8…32). The asymptotic 3·(n/2)² limb-mul saving does not yet outrun
/// the recombine (carve + zero + add/sub) overhead because the
/// schoolbook leaf keeps both `u64 × u64 → u128` multiplier ports
/// saturated. The crossover lands beyond the widest shipped multiply.
///
/// NEEDS-BENCH: the 256 value is the spec/architecture default, not a
/// tuned crossover. It must be re-swept on the pinned GHA bench
/// (`benches/int_ops_micro.rs`, `mul_crossover`, plus the per-tier wide
/// `mul` cells) before being lowered to engage any shipped tier; the
/// local microbench above is unpinned and noisy and only establishes
/// that no shipped width is a clear win today.
///
/// Must be `>= 4`: the recursion's z1 sum product runs on `⌈n/2⌉ + 1`
/// limbs, which only strictly shrinks below `n` once `n >= 4`, so a
/// threshold below 4 would fail to terminate.
pub(crate) const KARATSUBA_THRESHOLD_U64: usize = 256;

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
/// Numbers on Automata" (Doklady Akad. Nauk SSSR 145, 293-294).
/// Splits both equal-length operands at half: `a = a₁·B + a₀`,
/// `b = b₁·B + b₀`, computes three half-width sub-products
/// `z₀ = a₀·b₀`, `z₂ = a₁·b₁`, `z₁ = (a₀+a₁)·(b₀+b₁) − z₀ − z₂`,
/// then recombines as `z₂·B² + z₁·B + z₀`.
///
/// All temporaries live in a single fixed `[u64; KARATSUBA_SCRATCH_LIMBS]`
/// stack buffer declared once at this public entry point; the recursion
/// carves disjoint windows out of it with `split_at_mut` (so the borrow
/// checker proves non-aliasing — no `unsafe`, no `Vec`, available in
/// `no_std`/no-alloc builds). The three child products run sequentially
/// and share the same scratch tail, which keeps the total at `~2·S(n)`.
///
/// Operands must be equal length. `out.len() >= 2 * a.len()` and `out`
/// must be zeroed by the caller.
pub(crate) fn limbs_mul_karatsuba_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        karatsuba_scratch_needed(a.len()) <= KARATSUBA_SCRATCH_LIMBS,
        "Karatsuba scratch overflow: n={} needs {} limbs, have {}",
        a.len(),
        karatsuba_scratch_needed(a.len()),
        KARATSUBA_SCRATCH_LIMBS,
    );
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(a, b, out, &mut scratch, KARATSUBA_THRESHOLD_U64);
}

/// Bench-only entry that drives the production [`karatsuba_rec`] over
/// the real fixed `[u64; KARATSUBA_SCRATCH_LIMBS]` stack scratch at an
/// arbitrary `threshold`, so the crossover sweep can time the kernel at
/// widths below the parked [`KARATSUBA_THRESHOLD_U64`]. `out` is zeroed
/// here. Mirrors the production [`limbs_mul_karatsuba_u64`] exactly bar
/// the threshold parameter.
#[cfg(feature = "bench-alt")]
pub(crate) fn limbs_mul_karatsuba_u64_forced(
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
    threshold: usize,
) {
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
/// small threshold induces. Lets the correctness test exercise the
/// real split/recombine algebra at every width without depending on the
/// shipped [`KARATSUBA_THRESHOLD_U64`].
#[cfg(test)]
pub(crate) fn limbs_mul_karatsuba_u64_with_threshold(
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
    threshold: usize,
) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    let need = karatsuba_scratch_needed_th(a.len(), threshold);
    let mut scratch = alloc::vec![0u64; need];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Upper bound on the scratch (in u64 limbs) the non-allocating
/// Karatsuba recursion consumes for an `n`-limb equal-length multiply
/// at the production threshold. Mirrors the per-level carve below:
/// `S(n) = 2h + 2hi + 2(hi+1) + 2(hi+1)` with `h = n/2`, `hi = n - h`,
/// plus the geometric tail down the largest-child spine.
const fn karatsuba_scratch_needed(n: usize) -> usize {
    karatsuba_scratch_needed_th(n, KARATSUBA_THRESHOLD_U64)
}

/// Threshold-parameterised form of [`karatsuba_scratch_needed`] so the
/// correctness test can size scratch for the deeper recursion a small
/// test threshold induces.
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
/// the `2·n`-limb window; `scratch` is the live tail of the entry
/// buffer. Children below `threshold` base-case to schoolbook.
///
/// `threshold` is `KARATSUBA_THRESHOLD_U64` in production; it is a
/// parameter only so the correctness test can force deeper recursion at
/// small widths to exercise the split/recombine algebra.
fn karatsuba_rec(a: &[u64], b: &[u64], out: &mut [u64], scratch: &mut [u64], threshold: usize) {
    debug_assert!(
        threshold >= 4,
        "Karatsuba threshold must be >= 4 to terminate"
    );
    let n = a.len();
    if n < threshold {
        // `out` window pre-zeroed by the caller.
        limbs_mul_u64(a, b, out);
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
    let _ = limbs_add_assign_u64(sa, a_hi);
    let _ = limbs_add_assign_u64(sb, b_hi);

    // z1 = sa · sb − z0 − z2  (sa, sb both hi + 1 limbs).
    karatsuba_rec_unbalanced(sa, sb, z1, tail, threshold);
    let _ = limbs_sub_assign_u64(z1, z0);
    let _ = limbs_sub_assign_u64(z1, z2);

    // Recombine into the pre-zeroed out: out = z0 + z2·B² + z1·B
    // (B = 2^(64·h)). z0 lands at offset 0, z2 at 2h, z1 at h — the
    // overlap-adds use carry-propagating add, not copy.
    out[..z0.len()].copy_from_slice(z0);
    let _ = limbs_add_assign_u64(&mut out[2 * h..], z2);
    let _ = limbs_add_assign_u64(&mut out[h..], z1);
}

/// Karatsuba child dispatch for the equal-length sub-products that may
/// drop below the threshold (or be the `hi + 1`-limb sum product). Both
/// operands are equal length here; route to the recursion above the
/// threshold, else zero the window and base-case to schoolbook.
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
        limbs_mul_u64(a, b, out);
    }
}

/// Equal-length u64 multiplier dispatcher. Picks the non-allocating
/// Karatsuba kernel at or above the threshold; otherwise schoolbook.
/// Both operands are assumed to be the same length (the common
/// `widen_mul` case).
pub(crate) fn limbs_mul_fast_u64(a: &[u64], b: &[u64], out: &mut [u64]) {
    if a.len() == b.len() && a.len() >= KARATSUBA_THRESHOLD_U64 {
        for o in out.iter_mut() {
            *o = 0;
        }
        limbs_mul_karatsuba_u64(a, b, out);
        return;
    }
    limbs_mul_u64(a, b, out);
}

/// Original heap-allocating Karatsuba (four `Vec`s per recursion level),
/// retained compiled-out as the reference implementation the
/// non-allocating kernel above replaces. Its per-level allocation
/// overhead pushed the u64-base crossover past every shipped width;
/// kept only for documentation / cross-checking.
#[cfg(any())]
#[cfg(feature = "alloc")]
pub(crate) fn limbs_mul_karatsuba_u64_alloc(a: &[u64], b: &[u64], out: &mut [u64]) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    let n = a.len();
    if n < KARATSUBA_THRESHOLD_U64 {
        limbs_mul_u64(a, b, out);
        return;
    }
    let h = n / 2;
    let hi_len = n - h;
    let (a_lo, a_hi) = a.split_at(h);
    let (b_lo, b_hi) = b.split_at(h);

    let mut z0 = alloc::vec![0u64; 2 * h];
    limbs_mul_karatsuba_padded_u64_alloc(a_lo, b_lo, &mut z0);

    let mut z2 = alloc::vec![0u64; 2 * hi_len];
    limbs_mul_karatsuba_padded_u64_alloc(a_hi, b_hi, &mut z2);

    let sum_len = core::cmp::max(h, hi_len) + 1;
    let mut sum_a = alloc::vec![0u64; sum_len];
    let mut sum_b = alloc::vec![0u64; sum_len];
    sum_a[..h].copy_from_slice(a_lo);
    sum_b[..h].copy_from_slice(b_lo);
    let _ = limbs_add_assign_u64(&mut sum_a[..], a_hi);
    let _ = limbs_add_assign_u64(&mut sum_b[..], b_hi);

    let mut z1 = alloc::vec![0u64; 2 * sum_len];
    limbs_mul_karatsuba_padded_u64_alloc(&sum_a, &sum_b, &mut z1);
    let _ = limbs_sub_assign_u64(&mut z1[..], &z0);
    let _ = limbs_sub_assign_u64(&mut z1[..], &z2);

    for o in out.iter_mut().take(2 * n) {
        *o = 0;
    }
    let z0_take = core::cmp::min(z0.len(), out.len());
    out[..z0_take].copy_from_slice(&z0[..z0_take]);
    let z2_take = core::cmp::min(z2.len(), out.len().saturating_sub(2 * h));
    if z2_take > 0 {
        out[2 * h..2 * h + z2_take].copy_from_slice(&z2[..z2_take]);
    }
    let z1_take = core::cmp::min(z1.len(), out.len().saturating_sub(h));
    if z1_take > 0 {
        let _ = limbs_add_assign_u64(&mut out[h..h + z1_take], &z1[..z1_take]);
    }
}

/// Reference padded helper for [`limbs_mul_karatsuba_u64_alloc`].
#[cfg(any())]
#[cfg(feature = "alloc")]
fn limbs_mul_karatsuba_padded_u64_alloc(a: &[u64], b: &[u64], out: &mut [u64]) {
    if a.len() == b.len() && a.len() >= KARATSUBA_THRESHOLD_U64 {
        limbs_mul_karatsuba_u64_alloc(a, b, out);
    } else {
        for o in out.iter_mut() {
            *o = 0;
        }
        limbs_mul_u64(a, b, out);
    }
}

/// Möller–Granlund 2-by-1 invariant divisor at u64 base.
///
/// Reference: Möller & Granlund (2011), Algorithm 4.
///
/// The u64 base implementation is compact because the doubled type
/// (u128) is *native* — each q̂ step is a single `u128` op rather than
/// a software 256-bit decomposition.
#[derive(Clone, Copy)]
pub(crate) struct MG2by1U64 {
    d: u64,
    v: u64,
}

impl MG2by1U64 {
    /// `d` must be normalised: `d >> 63 == 1`.
    #[inline]
    pub(crate) const fn new(d: u64) -> Self {
        debug_assert!(d >> 63 == 1, "MG2by1U64::new: divisor must be normalised");
        // v = floor((B² - 1 - d·B) / d) where B = 2^64.
        // Numerator high = !d (= B-1-d), low = u64::MAX (= B-1).
        // High < d for normalised d, so native u128/u128 with the
        // divisor cast to u128 returns a quotient fitting u64.
        let num = ((!d as u128) << 64) | (u64::MAX as u128);
        let v = (num / (d as u128)) as u64;
        Self { d, v }
    }

    /// Divide `(u1·B + u0)` by `d`. Requires `u1 < d`.
    #[inline]
    pub(crate) const fn div_rem(&self, u1: u64, u0: u64) -> (u64, u64) {
        debug_assert!(
            u1 < self.d,
            "MG2by1U64::div_rem: high word must be < divisor"
        );
        // (q1, q0) = v·u1 + ⟨u1, u0⟩ as u128
        let q128 = (self.v as u128)
            .wrapping_mul(u1 as u128)
            .wrapping_add(((u1 as u128) << 64) | (u0 as u128));
        let mut q1 = (q128 >> 64) as u64;
        let q0 = q128 as u64;
        q1 = q1.wrapping_add(1);
        let mut r = u0.wrapping_sub(q1.wrapping_mul(self.d));
        if r > q0 {
            q1 = q1.wrapping_sub(1);
            r = r.wrapping_add(self.d);
        }
        if r >= self.d {
            q1 = q1.wrapping_add(1);
            r = r.wrapping_sub(self.d);
        }
        (q1, r)
    }
}

/// Möller–Granlund 3-by-2 invariant divisor at u64 base.
///
/// Divides `(n2·B² + n1·B + n0)` by `(d1·B + d0)` for a normalised
/// 2-limb divisor (`d1`'s top bit set) using *two* limbs of divisor
/// information, returning a quotient that is exactly correct in one
/// pass — no refinement loop is needed in the Knuth Algorithm D
/// caller. Compared to [`MG2by1U64`] + the historic Knuth refinement
/// loop, the 3-by-2 form trades:
///
/// - +1 hardware multiply per call (the d0·q step), against
/// - up to 2 refinement-loop iterations per quotient limb saved.
///
/// Net win at every Knuth call with `n ≥ 2` divisor limbs.
///
/// Reference: Möller & Granlund 2011, Algorithm 5 (the divide) and
/// Algorithm 6 (the reciprocal precompute). `MG2by1U64` is the 2-by-1
/// cousin used by `limbs_divmod_knuth_u64`'s q̂ estimator.
#[derive(Clone, Copy)]
pub(crate) struct MG3by2U64 {
    d1: u64,
    d0: u64,
    /// Reciprocal of the top divisor limb (same formula as MG2by1U64::v).
    dinv: u64,
}

impl MG3by2U64 {
    /// Setup. `d1` must be normalised (`d1 >> 63 == 1`).
    ///
    /// Computes the *3-by-2* invariant reciprocal, which differs from
    /// the [`MG2by1U64`] 2-by-1 reciprocal by an extra refinement step
    /// that accounts for `d0`. Without that refinement the algorithm
    /// fails on inputs where the divisor's low limb is large enough
    /// to push the q estimate over by more than the corrections can
    /// recover (test case: `n=(B-2, B-1, B-1)`, `d=(B-1, B-1)`, where
    /// the naive 2-by-1 reciprocal hands back q=0 instead of B-1).
    ///
    /// Reference: Möller & Granlund 2011, Algorithm 6 (the
    /// reciprocal refinement that accounts for `d0`).
    #[inline]
    pub(crate) const fn new(d1: u64, d0: u64) -> Self {
        debug_assert!(
            d1 >> 63 == 1,
            "MG3by2U64::new: top divisor limb must be normalised"
        );
        // Step 1: 2-by-1 reciprocal of d1 alone.
        let num = ((!d1 as u128) << 64) | (u64::MAX as u128);
        let mut v = (num / (d1 as u128)) as u64;

        // Step 2: refine for d0. `p = d1·v + d0` (mod B). If the sum
        // overflows, v was over-estimated → decrement.
        let mut p = d1.wrapping_mul(v).wrapping_add(d0);
        if p < d0 {
            v = v.wrapping_sub(1);
            let mask = if p >= d1 { u64::MAX } else { 0 };
            p = p.wrapping_sub(d1);
            v = v.wrapping_add(mask);
            p = p.wrapping_sub(mask & d1);
        }

        // Step 3: account for d0·v. `(t1, t0) = d0·v`; check if
        // `p + t1` overflows; one or two more decrements may be
        // required.
        let prod = (d0 as u128) * (v as u128);
        let t1 = (prod >> 64) as u64;
        let t0 = prod as u64;
        let (new_p, carry) = p.overflowing_add(t1);
        let _p_final = new_p;
        if carry {
            v = v.wrapping_sub(1);
            if new_p >= d1 && (new_p > d1 || t0 >= d0) {
                v = v.wrapping_sub(1);
            }
        }

        Self { d1, d0, dinv: v }
    }

    /// Divide `(n2·B² + n1·B + n0)` by `(d1·B + d0)`. Requires
    /// `(n2, n1) < (d1, d0)` so the quotient fits a single u64.
    /// Returns `(q, r1, r0)` where the remainder is `r1·B + r0`.
    ///
    /// Algorithm decomposition (Möller & Granlund 2011, Algorithm 5):
    /// 1. Initial `q` from a 2-by-1 divide of `(n2, n1)` by `d1` via
    ///    the precomputed reciprocal `dinv`.
    /// 2. Subtract `(q·d1, q·d0)` from `(n1, n0)`; this stages the
    ///    candidate remainder `(r1, r0)`.
    /// 3. Two add-back corrections that fire 0–1 times each: the
    ///    first catches `q` over by 1, the second catches the rare
    ///    `q` over by 2 (only possible if the initial 2-by-1 reciprocal
    ///    over-shot).
    #[inline]
    pub(crate) const fn div_rem(&self, n2: u64, n1: u64, n0: u64) -> (u64, u64, u64) {
        debug_assert!(
            n2 < self.d1 || (n2 == self.d1 && n1 < self.d0),
            "MG3by2U64::div_rem: numerator high pair must be < divisor"
        );

        // Step 1: q estimate from (n2, n1) / d1 via dinv.
        // (q_hi, q_lo) = n2 * dinv + (n2, n1) — overflow into a 257th
        // bit is fine, the mask-based correction (step 4a) recovers
        // from it without needing to materialise the lost bit.
        let prod = (n2 as u128)
            .wrapping_mul(self.dinv as u128)
            .wrapping_add(((n2 as u128) << 64) | (n1 as u128));
        let mut q = (prod >> 64) as u64;
        let q_lo = prod as u64;

        // Step 2a: r1 = n1 - q·d1 (mod B).
        let mut r1 = n1.wrapping_sub(q.wrapping_mul(self.d1));

        // Step 2b: (r1, r0) = (r1, n0) - (d1, d0).
        let r256 = (((r1 as u128) << 64) | (n0 as u128))
            .wrapping_sub(((self.d1 as u128) << 64) | (self.d0 as u128));
        r1 = (r256 >> 64) as u64;
        let mut r0 = r256 as u64;

        // Step 2c: (r1, r0) -= d0·q (mod B²).
        let t = (self.d0 as u128).wrapping_mul(q as u128);
        let r256 = (((r1 as u128) << 64) | (r0 as u128)).wrapping_sub(t);
        r1 = (r256 >> 64) as u64;
        r0 = r256 as u64;

        // Step 3: q += 1; provisional.
        q = q.wrapping_add(1);

        // Step 4a: first conditional correction.
        // If r1 >= q_lo (in u64 numeric), the provisional q was over
        // by 1; decrement q and add (d1, d0) back to the remainder.
        // Branchless via mask.
        let mask = if r1 >= q_lo { u64::MAX } else { 0 };
        q = q.wrapping_add(mask); // adds u64::MAX = -1.
        let add = ((mask & self.d1) as u128) << 64 | ((mask & self.d0) as u128);
        let r256 = (((r1 as u128) << 64) | (r0 as u128)).wrapping_add(add);
        r1 = (r256 >> 64) as u64;
        r0 = r256 as u64;

        // Step 4b: final correction (rare).
        // If (r1, r0) >= (d1, d0), q was *still* off by 1; bump q and
        // subtract the divisor once more.
        if r1 > self.d1 || (r1 == self.d1 && r0 >= self.d0) {
            q = q.wrapping_add(1);
            let r256 = (((r1 as u128) << 64) | (r0 as u128))
                .wrapping_sub(((self.d1 as u128) << 64) | (self.d0 as u128));
            r1 = (r256 >> 64) as u64;
            r0 = r256 as u64;
        }

        (q, r1, r0)
    }
}

/// Runtime divide dispatcher at u64 base.
pub(crate) fn limbs_divmod_dispatch_u64(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
) {
    const BZ_THRESHOLD_U64: usize = 16; // doubled from u128 path's 8.

    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "limbs_divmod_dispatch_u64: divide by zero");

    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }

    // Single-limb divisor: defer to const limbs_divmod_u64 (its Fast B
    // is one hardware u128/u64 per dividend limb — already optimal).
    if n == 1 {
        limbs_divmod_u64(num, den, quot, rem);
        return;
    }

    if n >= BZ_THRESHOLD_U64 && top >= 2 * n {
        limbs_divmod_bz_u64(num, den, quot, rem);
    } else {
        limbs_divmod_knuth_u64(num, den, quot, rem);
    }
}

/// Knuth Algorithm D at base 2^64.
///
/// Every limb is a u64 and the q̂ estimator uses [`MG2by1U64`]. The
/// multiply-subtract pass uses native `u64 × u64 → u128`, which keeps
/// the carry-merge to a single layer.
pub(crate) fn limbs_divmod_knuth_u64(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    for q in quot.iter_mut() {
        *q = 0;
    }
    for r in rem.iter_mut() {
        *r = 0;
    }

    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "limbs_divmod_knuth_u64: divide by zero");

    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }
    if top < n {
        let copy_n = num.len().min(rem.len());
        let mut i = 0;
        while i < copy_n {
            rem[i] = num[i];
            i += 1;
        }
        return;
    }

    let shift = den[n - 1].leading_zeros();
    let mut u = [0u64; SCRATCH_LIMBS_U64];
    let mut v = [0u64; SCRATCH_LIMBS_U64];
    debug_assert!(top < SCRATCH_LIMBS_U64 && n <= SCRATCH_LIMBS_U64);

    if shift == 0 {
        u[..top].copy_from_slice(&num[..top]);
        u[top] = 0;
        v[..n].copy_from_slice(&den[..n]);
    } else {
        let mut carry: u64 = 0;
        for i in 0..top {
            let val = num[i];
            u[i] = (val << shift) | carry;
            carry = val >> (64 - shift);
        }
        u[top] = carry;
        carry = 0;
        for i in 0..n {
            let val = den[i];
            v[i] = (val << shift) | carry;
            carry = val >> (64 - shift);
        }
    }

    let m_plus_n = if u[top] != 0 { top + 1 } else { top };
    debug_assert!(m_plus_n >= n);
    let m = m_plus_n - n;

    // Knuth Algorithm D requires a multi-limb divisor. Single-limb
    // divisors have a much faster hardware divide path; route them
    // out here so the hot loop below can assume n >= 2 and lose the
    // per-iteration n=1 dispatch.
    if n == 1 {
        limbs_divmod_u64(num, den, quot, rem);
        return;
    }

    // MG 2-by-1 q̂ estimator (Möller-Granlund 2011 Algorithm 4) +
    // inner refinement against v[n-2]. The 3-by-2 estimator was
    // re-benched post u64 migration: its per-q̂ setup cost
    // (extra multiplies vs the 2-by-1's one) outweighs the
    // refinement loop's near-zero iteration count on decimal
    // divisors, so 2-by-1 + while-loop still wins at the widest
    // tiers.
    let v_top = v[n - 1];
    let v_below = v[n - 2];
    let mg_top = MG2by1U64::new(v_top);

    let mut j_plus_one = m + 1;
    while j_plus_one > 0 {
        j_plus_one -= 1;
        let j = j_plus_one;

        let jn = j + n;
        let u_top = u[jn];
        let u_next = u[jn - 1];

        // MG 2-by-1 q̂ + (r_hat). Cheap-check `u_top > v_top` first
        // — strict greater → q̂ = MAX, no MG call, no refinement
        // possible — then handle the `u_top == v_top` edge inline,
        // then the common u_top < v_top case via the 2-by-1
        // estimator.
        let (mut q_hat, mut r_hat) = if u_top > v_top {
            (u64::MAX, u64::MAX)
        } else if u_top == v_top {
            let (r, of) = u_next.overflowing_add(v_top);
            (u64::MAX, if of { u64::MAX } else { r })
        } else {
            mg_top.div_rem(u_top, u_next)
        };

        // Refinement against v[n-2]. Tightens q̂ from off-by-2 to
        // off-by-1; the post-subtract `final_borrow` check below
        // catches the remaining off-by-1 case. Almost never fires
        // on decimal divisors but cheap when it doesn't.
        loop {
            let prod = (q_hat as u128) * (v_below as u128);
            let hi = (prod >> 64) as u64;
            let lo = prod as u64;
            let rhs_lo = u[jn - 2];
            let rhs_hi = r_hat;
            if hi < rhs_hi || (hi == rhs_hi && lo <= rhs_lo) {
                break;
            }
            q_hat = q_hat.wrapping_sub(1);
            let (new_r, of) = r_hat.overflowing_add(v_top);
            if of {
                break;
            }
            r_hat = new_r;
        }

        // D4. u[j..=j+n] -= q̂ · v[0..n]
        let mut mul_carry: u64 = 0;
        let mut borrow: u64 = 0;
        for i in 0..n {
            let prod = (q_hat as u128) * (v[i] as u128);
            let prod_lo = prod as u64;
            let prod_hi = (prod >> 64) as u64;
            let (s_prod, c1) = prod_lo.overflowing_add(mul_carry);
            let new_mul_carry = prod_hi + (c1 as u64);
            let (s1, b1) = u[j + i].overflowing_sub(s_prod);
            let (s2, b2) = s1.overflowing_sub(borrow);
            u[j + i] = s2;
            borrow = (b1 as u64) + (b2 as u64);
            mul_carry = new_mul_carry;
        }
        let (s1, b1) = u[j + n].overflowing_sub(mul_carry);
        let (s2, b2) = s1.overflowing_sub(borrow);
        u[j + n] = s2;
        let final_borrow = (b1 as u64) + (b2 as u64);

        if final_borrow != 0 {
            q_hat = q_hat.wrapping_sub(1);
            let mut carry: u64 = 0;
            for i in 0..n {
                let (s1, c1) = u[j + i].overflowing_add(v[i]);
                let (s2, c2) = s1.overflowing_add(carry);
                u[j + i] = s2;
                carry = (c1 as u64) + (c2 as u64);
            }
            u[j + n] = u[j + n].wrapping_add(carry);
        }

        if j < quot.len() {
            quot[j] = q_hat;
        }
    }

    if shift == 0 {
        let copy_n = n.min(rem.len());
        rem[..copy_n].copy_from_slice(&u[..copy_n]);
    } else {
        for i in 0..n {
            if i < rem.len() {
                let lo = u[i] >> shift;
                let hi_into_lo = if i + 1 < n {
                    u[i + 1] << (64 - shift)
                } else {
                    0
                };
                rem[i] = lo | hi_into_lo;
            }
        }
    }
}

/// Burnikel–Ziegler outer chunking, u64 base.
pub(crate) fn limbs_divmod_bz_u64(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    const BZ_THRESHOLD_U64: usize = 16;

    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "limbs_divmod_bz_u64: divide by zero");

    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }

    if n < BZ_THRESHOLD_U64 || top < 2 * n {
        limbs_divmod_knuth_u64(num, den, quot, rem);
        return;
    }

    for q in quot.iter_mut() {
        *q = 0;
    }
    for r in rem.iter_mut() {
        *r = 0;
    }

    let chunks = top.div_ceil(n);
    let mut carry = [0u64; SCRATCH_LIMBS_U64];
    let mut buf = [0u64; SCRATCH_LIMBS_U64];
    let mut q_chunk = [0u64; SCRATCH_LIMBS_U64];
    let mut r_chunk = [0u64; SCRATCH_LIMBS_U64];

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
        limbs_divmod_knuth_u64(
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

/// `out = floor(sqrt(n))`. Newton iteration on top of the runtime
/// divide dispatcher.
///
/// History: this routine previously called the *const* [`limbs_divmod_u64`]
/// per iteration, which routes multi-limb divisors through the
/// O(bits²) shift-subtract path. At Int<64> (n=64 u64 limbs) that
/// dominates wall time — Newton converges in ~log₂(b) ≈ 12 iterations,
/// each one a `~65k`-limb-op divmod. Switching to
/// [`limbs_divmod_dispatch_u64`] gets Knuth-base-2⁶⁴ per iteration
/// (~`~32²` = 1024 limb-ops), worth ~40× on D307 sqrt.
pub(crate) fn limbs_isqrt_u64(n: &[u64], out: &mut [u64]) {
    for o in out.iter_mut() {
        *o = 0;
    }
    let bits = limbs_bit_len_u64(n);
    if bits == 0 {
        return;
    }
    if bits <= 1 {
        out[0] = 1;
        return;
    }
    let work = n.len() + 1;
    debug_assert!(work <= SCRATCH_LIMBS_U64, "isqrt scratch overflow");
    let mut x = [0u64; SCRATCH_LIMBS_U64];

    // Initial guess. The classical seed is a single bit at position
    // `ceil(bits/2)` — one bit of accuracy, costing one Newton step per
    // doubling of accuracy (≈ `log2(bits/2)` iterations at any width).
    //
    // The hardware-`f64::sqrt` seed below lifts that to ~53 correct
    // bits in one go: extract the top 64 bits of `n` (which fits the
    // f64 mantissa with 11 bits of headroom), take the hardware sqrt,
    // and shift the result back to the correct magnitude. For Int<8>
    // (D76 sqrt input) this drops the Newton iteration count from ~8
    // to ~3, with each saved iteration eliminating one full
    // `limbs_divmod_dispatch_u64` call (the dominant cost).
    //
    // Hasselgren's trick — see Crandall & Pomerance 2005, "Prime
    // Numbers: A Computational Perspective" §9.2.1 — credits the
    // f64-bootstrap idea to T. Hasselgren in the GMP mailing list
    // archives; the implementation here is a from-first-principles
    // limb-array variant.
    if bits >= 8 {
        // Extract top 64 bits of `n` as a u64, aligned so the leading
        // 1 sits at position 63 (or as close as `n` allows).
        let shift = bits - 64.min(bits);
        // shift == 0 happens iff bits < 64 → n fits one limb.
        // Otherwise top_u64 = (n >> shift) & ((1<<64)-1).
        let limb_idx = (shift / 64) as usize;
        let bit_off = shift % 64;
        let top_u64: u64 = if bit_off == 0 {
            n[limb_idx]
        } else {
            let lo = n[limb_idx] >> bit_off;
            let hi = if limb_idx + 1 < n.len() {
                n[limb_idx + 1].checked_shl(64 - bit_off).unwrap_or(0)
            } else {
                0
            };
            lo | hi
        };
        // Hardware sqrt on the top 64 bits. f64 mantissa carries 53
        // bits; the low 11 of top_u64 are lost — accepted, the Newton
        // loop refines the seed to full precision.
        let seed_f64 = (top_u64 as f64).sqrt();
        // True sqrt(n) = sqrt(top * 2^shift) = seed_f64 * 2^(shift / 2).
        // If shift is odd we add a √2 factor:
        // seed_f64 * √2 * 2^((shift - 1) / 2).
        let (seed_f64, half_shift) = if (shift & 1) == 1 {
            (seed_f64 * core::f64::consts::SQRT_2, (shift - 1) / 2)
        } else {
            (seed_f64, shift / 2)
        };
        // Convert to an integer over-estimate. f64's sqrt is
        // correctly rounded but the `as u128` cast truncates toward
        // zero; ceil and add a 1-ULP safety margin so the placed
        // seed is a strict over-estimate of the true sqrt. Newton's
        // iteration only converges monotonically down to
        // floor(sqrt(n)) from a strict over-estimate; under-shoot
        // would cause the iteration to oscillate and the
        // `y >= x` exit check to terminate on the wrong side.
        // `core::f64::ceil` is std-only but we depend on std elsewhere
        // (the `as u128` cast already brings the libm float→int
        // conversion in). Use a no_std-safe manual ceil:
        // `(x as u128) + (if x.fract() != 0.0 { 1 } else { 0 })`.
        let truncated = seed_f64 as u128;
        let frac_nonzero = (truncated as f64) != seed_f64;
        let seed_int: u128 = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        // Place seed_int at bit position `half_shift` in x. seed_int
        // is at most ~53 bits set (the f64 mantissa) + 2, so the
        // shifted value occupies at most 2 u64 limbs.
        let seed_limb_idx = (half_shift / 64) as usize;
        let seed_bit_off = half_shift % 64;
        let shifted: u128 = seed_int << seed_bit_off;
        let seed_lo = shifted as u64;
        let seed_hi = (shifted >> 64) as u64;
        if seed_limb_idx < work {
            x[seed_limb_idx] |= seed_lo;
        }
        if seed_limb_idx + 1 < work {
            x[seed_limb_idx + 1] |= seed_hi;
        }
        // Newton needs a non-zero divisor. Empty seed (would only
        // happen on a tiny input — `bits >= 8` rules most of those
        // out, but defend the invariant anyway).
        if limbs_is_zero_u64(&x[..work]) {
            x[0] = 1;
        }
    } else {
        // Tiny n: fall back to the classical 1-bit seed.
        let e = bits.div_ceil(2);
        x[(e / 64) as usize] |= 1u64 << (e % 64);
    }

    loop {
        let mut q = [0u64; SCRATCH_LIMBS_U64];
        let mut r = [0u64; SCRATCH_LIMBS_U64];
        limbs_divmod_dispatch_u64(n, &x[..work], &mut q[..work], &mut r[..work]);
        limbs_add_assign_u64(&mut q[..work], &x[..work]);
        let mut y = [0u64; SCRATCH_LIMBS_U64];
        limbs_shr_u64(&q[..work], 1, &mut y[..work]);
        if limbs_cmp_u64(&y[..work], &x[..work]) >= 0 {
            break;
        }
        x = y;
    }
    let copy_len = if out.len() < work { out.len() } else { work };
    out[..copy_len].copy_from_slice(&x[..copy_len]);
}

/// `limbs /= radix` in place, returning the remainder. `radix` must
/// be a u64 (so the per-limb divide stays inside `u128 / u64`).
fn limbs_div_small_u64(limbs: &mut [u64], radix: u64) -> u64 {
    let mut rem: u64 = 0;
    for limb in limbs.iter_mut().rev() {
        let acc = ((rem as u128) << 64) | (*limb as u128);
        *limb = (acc / (radix as u128)) as u64;
        rem = (acc % (radix as u128)) as u64;
    }
    rem
}

/// `10^19` — the largest power of ten that fits in a `u64`
/// (`10^19 < 2^64 < 10^20`). Dividing the magnitude by this constant
/// peels off 19 decimal digits per full-width pass instead of one.
const POW10_19: u64 = 10_000_000_000_000_000_000;
/// Number of decimal digits emitted per `POW10_19` chunk.
const POW10_19_DIGITS: usize = 19;

/// Format a u64 limb slice into `buf` in the given radix (`2..=16`).
///
/// For the decimal radix this peels 19 digits per full-width divide by
/// dividing the magnitude by `10^19` (the largest power of ten below
/// `2^64`) and emitting the 19-digit `u64` remainder with cheap native
/// arithmetic. The expensive `O(limbs)` full-width small-divide then
/// runs once per 19 digits rather than once per digit. The other radixes
/// (2 / 8 / 16) keep the one-divide-per-digit loop.
pub(crate) fn limbs_fmt_into_u64<'a>(
    limbs: &[u64],
    radix: u64,
    lower: bool,
    buf: &'a mut [u8],
) -> &'a str {
    if limbs_is_zero_u64(limbs) {
        let last = buf.len() - 1;
        buf[last] = b'0';
        return core::str::from_utf8(&buf[last..]).unwrap();
    }
    let mut work = [0u64; SCRATCH_LIMBS_U64];
    work[..limbs.len()].copy_from_slice(limbs);
    let wl = limbs.len();
    let mut pos = buf.len();

    if radix == 10 {
        // Peel one 19-digit base-10^19 chunk per full-width divide.
        loop {
            let chunk = limbs_div_small_u64(&mut work[..wl], POW10_19);
            if limbs_is_zero_u64(&work[..wl]) {
                // Most-significant chunk: emit without leading-zero pad.
                let mut v = chunk;
                loop {
                    pos -= 1;
                    buf[pos] = b'0' + (v % 10) as u8;
                    v /= 10;
                    if v == 0 {
                        break;
                    }
                }
                break;
            }
            // Interior chunk: always exactly 19 zero-padded digits.
            let mut v = chunk;
            for _ in 0..POW10_19_DIGITS {
                pos -= 1;
                buf[pos] = b'0' + (v % 10) as u8;
                v /= 10;
            }
        }
        return core::str::from_utf8(&buf[pos..]).unwrap();
    }

    let digits: &[u8] = if lower {
        b"0123456789abcdef"
    } else {
        b"0123456789ABCDEF"
    };
    while !limbs_is_zero_u64(&work[..wl]) {
        let r = limbs_div_small_u64(&mut work[..wl], radix);
        pos -= 1;
        buf[pos] = digits[r as usize];
    }
    core::str::from_utf8(&buf[pos..]).unwrap()
}

/// Signed three-way compare for u64-limb magnitudes with signs.
#[inline]
pub(crate) const fn scmp_u64(a_neg: bool, a: &[u64], b_neg: bool, b: &[u64]) -> i32 {
    match (a_neg, b_neg) {
        (true, false) => -1,
        (false, true) => 1,
        _ => limbs_cmp_u64(a, b),
    }
}

// ─────────────────────────────────────────────────────────────────────
// End of u64 primitives.
// ─────────────────────────────────────────────────────────────────────



// The named wide-integer type family. Every width is now the
// const-generic `Int<N>` / `Uint<N>`, re-exported here from
// `int::types` so every `crate::wide_int::IntXXXX` path (the `lib.rs`
// re-exports, the `I*` / `U*` short aliases below, and the kernel
// shims) keeps resolving with no change at the use sites. The kernels
// run on `Int<N>` through the blanket `BigInt` impl in
// `int::types::wide_compat`. Some widths are unused in low-feature
// builds (e.g. only `Int<4>` is reached by D38 under default
// features), so the re-export carries `allow(unused_imports)`.

// Short aliases used by the decimal-tier macros (replacing the former
// `crate::wide` re-export shim). The signed alias is exposed at each
// width where it backs storage *or* serves as the next-width mul/div
// widening step; the unsigned alias only where `Display`'s magnitude
// path needs it. The feature gates mirror the call-site features.
// Tier aliases — each width gets an `I*` (signed) alias when it
// backs storage or serves as a mul/div widening step for some tier,
// and a matching `U*` (unsigned) when `Display`'s magnitude path
// needs it.

#[cfg(test)]
mod hint_tests {
    use super::*;
    use crate::int::types::{Int, Uint};

    #[test]
    fn signed_add_sub_neg() {
        let a = Int::<4>::from_i128(5);
        let b = Int::<4>::from_i128(3);
        assert_eq!(a.wrapping_add(b), Int::<4>::from_i128(8));
        assert_eq!(a.wrapping_sub(b), Int::<4>::from_i128(2));
        assert_eq!(b.wrapping_sub(a), Int::<4>::from_i128(-2));
        assert_eq!(a.negate(), Int::<4>::from_i128(-5));
        assert_eq!(Int::<4>::ZERO.negate(), Int::<4>::ZERO);
    }

    #[test]
    fn signed_mul_div_rem() {
        let six = Int::<8>::from_i128(6);
        let two = Int::<8>::from_i128(2);
        let three = Int::<8>::from_i128(3);
        assert_eq!(six.wrapping_mul(three), Int::<8>::from_i128(18));
        assert_eq!(six.wrapping_div(two), three);
        assert_eq!(
            Int::<8>::from_i128(7).wrapping_rem(three),
            Int::<8>::from_i128(1)
        );
        assert_eq!(
            Int::<8>::from_i128(-7).wrapping_rem(three),
            Int::<8>::from_i128(-1)
        );
        assert_eq!(six.negate().wrapping_mul(three), Int::<8>::from_i128(-18));
    }

    #[test]
    fn checked_overflow() {
        assert_eq!(Int::<4>::MAX.checked_add(Int::<4>::ONE), None);
        assert_eq!(Int::<4>::MIN.checked_sub(Int::<4>::ONE), None);
        assert_eq!(Int::<4>::MIN.checked_neg(), None);
        assert_eq!(
            Int::<4>::from_i128(2).checked_add(Int::<4>::from_i128(3)),
            Some(Int::<4>::from_i128(5))
        );
    }

    #[test]
    fn from_str_and_pow() {
        let ten = Int::<16>::from_str_radix("10", 10).unwrap();
        assert_eq!(ten, Int::<16>::from_i128(10));
        assert_eq!(ten.pow(3), Int::<16>::from_i128(1000));
        let big = Int::<16>::from_str_radix("10", 10).unwrap().pow(40);
        let from_str =
            Int::<16>::from_str_radix("10000000000000000000000000000000000000000", 10).unwrap();
        assert_eq!(big, from_str);
        assert_eq!(
            Int::<4>::from_str_radix("-42", 10).unwrap(),
            Int::<4>::from_i128(-42)
        );
    }

    #[test]
    fn ordering_and_resize() {
        assert!(Int::<4>::from_i128(-1) < Int::<4>::ZERO);
        assert!(Int::<4>::MIN < Int::<4>::MAX);
        let v = Int::<4>::from_i128(-123_456_789);
        let wide: Int<16> = v.resize();
        let back: Int<4> = wide.resize();
        assert_eq!(back, v);
        assert_eq!(wide, Int::<16>::from_i128(-123_456_789));
    }

    #[test]
    fn isqrt_and_f64() {
        assert_eq!(Int::<8>::from_i128(144).isqrt(), Int::<8>::from_i128(12));
        assert_eq!(Int::<4>::from_i128(1_000_000).as_f64(), 1_000_000.0);
        assert_eq!(Int::<4>::from_f64(-2_500.0), Int::<4>::from_i128(-2500));
    }

    /// `Uint<4>` (the unsigned macro emission) supports the same
    /// bit/sign-manipulation surface as the signed sibling. Methods
    /// here are reachable through the wide decimal types but not always
    /// exercised by name; verify the contracts directly.
    #[test]
    fn uint256_is_zero_and_bit_helpers() {
        let zero = Uint::<4>::ZERO;
        let one = Uint::<4>::from_str_radix("1", 10).unwrap();
        let two = Uint::<4>::from_str_radix("2", 10).unwrap();
        assert!(zero.is_zero());
        assert!(!one.is_zero());
        assert!(one.is_power_of_two());
        assert!(two.is_power_of_two());
        let three = Uint::<4>::from_str_radix("3", 10).unwrap();
        assert!(!three.is_power_of_two());
        // next_power_of_two(0) == 1
        assert_eq!(zero.next_power_of_two(), one);
        // next_power_of_two(1) == 1 (already power of two)
        assert_eq!(one.next_power_of_two(), one);
        // next_power_of_two(3) == 4
        let four = Uint::<4>::from_str_radix("4", 10).unwrap();
        assert_eq!(three.next_power_of_two(), four);
        // count_ones / leading_zeros
        assert_eq!(zero.count_ones(), 0);
        assert_eq!(one.count_ones(), 1);
        assert_eq!(zero.leading_zeros(), Uint::<4>::BITS);
        assert_eq!(one.leading_zeros(), Uint::<4>::BITS - 1);
    }

    #[test]
    fn uint256_parse_arithmetic_and_pow() {
        // from_str_radix only accepts radix 10.
        assert!(Uint::<4>::from_str_radix("10", 2).is_err());
        // Non-digit byte rejected.
        assert!(Uint::<4>::from_str_radix("1a", 10).is_err());
        // Arithmetic: 3 - 2 = 1, 6 / 2 = 3, 7 % 3 = 1, 3·3 = 9.
        let two = Uint::<4>::from_str_radix("2", 10).unwrap();
        let three = Uint::<4>::from_str_radix("3", 10).unwrap();
        let six = Uint::<4>::from_str_radix("6", 10).unwrap();
        let seven = Uint::<4>::from_str_radix("7", 10).unwrap();
        assert_eq!(three - two, Uint::<4>::from_str_radix("1", 10).unwrap());
        assert_eq!(six / two, three);
        assert_eq!(seven % three, Uint::<4>::from_str_radix("1", 10).unwrap());
        // BitAnd / BitOr / BitXor
        let five = Uint::<4>::from_str_radix("5", 10).unwrap(); // 101
        let four = Uint::<4>::from_str_radix("4", 10).unwrap(); // 100
        let one = Uint::<4>::from_str_radix("1", 10).unwrap(); // 001
        assert_eq!(five & four, four); // 100
        assert_eq!(five | one, five); // 101
        assert_eq!(five ^ four, one); // 001
        // pow: 2^10 = 1024
        let p10 = two.pow(10);
        assert_eq!(p10, Uint::<4>::from_str_radix("1024", 10).unwrap());
        // cast_signed round-trip
        let signed = three.cast_signed();
        assert_eq!(signed, Int::<4>::from_i128(3));
    }

    /// `Int::<4>::bit` reports the two's-complement bit at any index;
    /// indices past the storage width return the sign bit.
    #[test]
    fn signed_bit_and_trailing_zeros() {
        let v = Int::<4>::from_i128(0b1100);
        assert!(v.bit(2));
        assert!(v.bit(3));
        assert!(!v.bit(0));
        assert!(!v.bit(1));
        // Out-of-range bit returns the sign — non-negative for v.
        assert!(!v.bit(1000));
        // Negative input: sign bit returns true past the storage.
        let n = Int::<4>::from_i128(-1);
        assert!(n.bit(1000));
        // trailing_zeros
        assert_eq!(Int::<4>::from_i128(8).trailing_zeros(), 3);
        assert_eq!(Int::<4>::ZERO.trailing_zeros(), Int::<4>::BITS);
    }
}

#[cfg(test)]
mod slice_tests {
    use super::*;
    use crate::int::types::{Int, Uint};

    // ── u64-primitive equivalence + self-consistency ──────

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

    /// `limbs_mul_u64` is self-consistent with the divide engine across
    /// the carry-stressing corpus: `(a·b) / b == a` and `(a·b) % b == 0`
    /// whenever `b != 0`. Together with the Karatsuba and Knuth/BZ
    /// cross-checks below this pins the schoolbook multiply.
    #[test]
    fn limbs_mul_u64_round_trips_through_divide() {
        for a in corpus() {
            for b in corpus() {
                let a64 = pack(&a);
                let b64 = pack(&b);
                if limbs_is_zero_u64(&b64) {
                    continue;
                }
                let mut prod = alloc::vec![0u64; a64.len() + b64.len()];
                limbs_mul_u64(&a64, &b64, &mut prod);

                let mut q = alloc::vec![0u64; prod.len()];
                let mut r = alloc::vec![0u64; b64.len() + 1];
                limbs_divmod_dispatch_u64(&prod, &b64, &mut q, &mut r);

                assert!(limbs_is_zero_u64(&r), "remainder non-zero");
                assert_eq!(&q[..a64.len()], &a64[..], "quotient != a");
            }
        }
    }

    /// `limbs_mul_karatsuba_u64` matches `limbs_mul_u64` on equal-length
    /// operands across the carry-stressing corpus. Proves the recursive
    /// split + recombine algebra holds for the worst-case inputs at the
    /// production threshold.
    #[test]
    fn limbs_mul_karatsuba_u64_matches_schoolbook() {
        for a in corpus() {
            for b in corpus() {
                let a64 = pack(&a);
                let b64 = pack(&b);
                let n = a64.len().min(b64.len());
                if n < super::KARATSUBA_THRESHOLD_U64 {
                    continue;
                }
                let mut a_buf = alloc::vec![0u64; n];
                let mut b_buf = alloc::vec![0u64; n];
                a_buf.copy_from_slice(&a64[..n]);
                b_buf.copy_from_slice(&b64[..n]);
                let mut out_school = alloc::vec![0u64; 2 * n];
                let mut out_kara = alloc::vec![0u64; 2 * n];
                limbs_mul_u64(&a_buf, &b_buf, &mut out_school);
                limbs_mul_karatsuba_u64(&a_buf, &b_buf, &mut out_kara);
                assert_eq!(out_kara, out_school, "Karatsuba mismatch at n={n}");
            }
        }
    }

    /// Non-allocating Karatsuba is bit-exact against the schoolbook
    /// oracle [`limbs_mul_u64`] over a large seeded corpus across every
    /// width the crate multiplies, including odd, threshold-boundary,
    /// and the 256-limb maximum. The recursion is driven at small
    /// thresholds so the full split/recombine algebra is exercised even
    /// at the narrow widths (the production threshold would otherwise
    /// base-case them straight to schoolbook).
    ///
    /// Edge magnitudes (all-zero, all-ones, single high/low limb) sit
    /// alongside uniform-random pairs to stress maximal carry
    /// propagation. Integer multiply is exact, so the assertion is
    /// byte-for-byte over the full `2n`-limb output with no tolerance.
    /// Commutativity (`a·b == b·a`) is asserted in the same pass.
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

        // Widths span the shipped tiers plus odd and threshold-boundary
        // sizes; 256 is the widest equal-length multiply (Int<256>).
        const WIDTHS: &[usize] = &[
            1, 2, 4, 7, 8, 15, 16, 17, 31, 32, 33, 48, 64, 96, 128, 255, 256,
        ];
        // Drive the recursion at several thresholds: 4 forces the
        // deepest sensible recursion (a threshold below 4 cannot shrink
        // the `hi + 1`-limb z1 sum product and would not terminate),
        // 8/16/24 exercise mixed depths, and 256 mirrors the
        // schoolbook-dominant production regime.
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
            // Build the operand-pair set once per width: every edge×edge
            // combination plus a batch of uniform-random pairs. Sizes
            // are modest per width so the whole test stays fast.
            let random_pairs = if n <= 16 {
                400
            } else if n <= 64 {
                120
            } else {
                30
            };

            let mut pairs: alloc::vec::Vec<(alloc::vec::Vec<u64>, alloc::vec::Vec<u64>)> =
                alloc::vec::Vec::new();
            // 5 edge kinds: 0=zero,1=ones,2=hi-limb,3=lo-limb,4=random.
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
                limbs_mul_u64(a, b, &mut oracle);

                for &th in THRESHOLDS {
                    let mut got = alloc::vec![0u64; 2 * n];
                    limbs_mul_karatsuba_u64_with_threshold(a, b, &mut got, th);
                    assert_eq!(
                        got, oracle,
                        "non-alloc Karatsuba mismatch at n={n}, threshold={th}\na={a:?}\nb={b:?}"
                    );

                    // Commutativity: b·a must equal a·b.
                    let mut got_swapped = alloc::vec![0u64; 2 * n];
                    limbs_mul_karatsuba_u64_with_threshold(b, a, &mut got_swapped, th);
                    assert_eq!(
                        got_swapped, oracle,
                        "non-alloc Karatsuba not commutative at n={n}, threshold={th}"
                    );
                }
            }
        }
    }

    /// The widest equal-length multiply (256 limbs, Int<256>) routes
    /// through the production [`limbs_mul_karatsuba_u64`] entry — which
    /// declares the fixed `[u64; KARATSUBA_SCRATCH_LIMBS]` stack buffer —
    /// without tripping the scratch-overflow `debug_assert` and matches
    /// schoolbook. Guards the scratch sizing against future threshold
    /// drops that deepen the recursion.
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
        // Scratch sizing must hold for the deepest recursion the kernel
        // can be tuned to — threshold as low as the documented floor.
        assert!(
            super::karatsuba_scratch_needed_th(256, 8) <= super::KARATSUBA_SCRATCH_LIMBS,
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
        limbs_mul_u64(&a, &b, &mut oracle);
        // Production entry: real fixed stack scratch, production threshold.
        limbs_mul_karatsuba_u64(&a, &b, &mut got);
        assert_eq!(
            got, oracle,
            "max-width Karatsuba mismatch via fixed scratch"
        );
    }

    /// `limbs_mul_u64_fixed::<L, D>` matches `limbs_mul_u64` at
    /// a representative set of compile-time `L` values covering
    /// every wide tier (D38..D1232). Each L gets its own
    /// monomorphisation; the test confirms the unrolled-by-LLVM
    /// fixed-array path produces the same output as the slice
    /// path for every shape in the carry-stressing corpus.
    #[test]
    fn limbs_mul_u64_fixed_matches_slice() {
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
                        limbs_mul_u64(&a_arr, &b_arr, &mut out_slice);
                        limbs_mul_u64_fixed::<$L, $D>(&a_arr, &b_arr, &mut out_fixed);
                        assert_eq!(
                            &out_slice[..],
                            &out_fixed[..],
                            "limbs_mul_u64_fixed::<{}, {}> mismatch",
                            $L, $D
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

    /// `limbs_mul_u64_into::<L, L+1>` matches `limbs_mul_u64_fixed::<L, 2·L>`
    /// when the wider operand is `[n, 0, ..., 0]`, across L covering every
    /// wide tier from D38 (L=2) to D307 (L=16). 1000 random (a, n) pairs
    /// per L from a deterministic SplitMix64 stream — no run-to-run drift,
    /// regression-friendly. Tail-zero limbs from the wide product are
    /// asserted alongside the leading `L + 1` so any spurious write past
    /// the truncated output is caught.
    #[test]
    fn limbs_mul_u64_into_matches_fixed() {
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
                    limbs_mul_u64_into::<$L, $LP1>(&a, n, &mut out_into);

                    let mut b = [0u64; $L];
                    b[0] = n;
                    let mut out_fixed = [0u64; $D];
                    limbs_mul_u64_fixed::<$L, $D>(&a, &b, &mut out_fixed);

                    assert_eq!(
                        &out_into[..],
                        &out_fixed[..$LP1],
                        "limbs_mul_u64_into::<{}, {}> low limbs mismatch \
                         (a={:?}, n={:#x})",
                        $L,
                        $LP1,
                        a,
                        n
                    );
                    for (k, &limb) in out_fixed[$LP1..].iter().enumerate() {
                        assert_eq!(
                            limb,
                            0,
                            "limbs_mul_u64_fixed high limb {} not zero \
                             — single-multiplier product must fit L+1 limbs",
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

    /// Verify the Euclidean identity `num == q·den + r` with
    /// `0 <= r < den` reconstructs across the corpus, where `q`/`r`
    /// come from `limbs_divmod_u64`.
    #[test]
    fn limbs_divmod_u64_satisfies_identity() {
        for num in corpus() {
            for den in corpus() {
                let n64 = pack(&num);
                let d64 = pack(&den);
                if limbs_is_zero_u64(&d64) {
                    continue;
                }
                let mut q64 = alloc::vec![0u64; n64.len()];
                let mut r64 = alloc::vec![0u64; n64.len()];
                limbs_divmod_u64(&n64, &d64, &mut q64, &mut r64);

                // recon = q·den + r, then compare to num.
                let mut recon = alloc::vec![0u64; q64.len() + d64.len() + 1];
                limbs_mul_u64(&q64, &d64, &mut recon);
                let _ = limbs_add_assign_u64(&mut recon, &r64);
                assert_eq!(&recon[..n64.len()], &n64[..], "q·den + r != num");
                assert!(recon[n64.len()..].iter().all(|&x| x == 0), "recon overflow");
                assert!(limbs_cmp_u64(&r64, &d64) < 0, "remainder >= divisor");
            }
        }
    }

    /// `limbs_divmod_knuth_u64` agrees with the dispatch path
    /// (`limbs_divmod_u64` / `limbs_divmod_dispatch_u64`) on the corpus.
    #[test]
    fn limbs_divmod_knuth_u64_matches_dispatch() {
        for num in corpus() {
            for den in corpus() {
                let n64 = pack(&num);
                let d64 = pack(&den);
                // Knuth requires a genuinely multi-limb normalised divisor.
                let mut dn = d64.len();
                while dn > 0 && d64[dn - 1] == 0 {
                    dn -= 1;
                }
                if dn < 2 {
                    continue;
                }
                let mut q_ref = alloc::vec![0u64; n64.len()];
                let mut r_ref = alloc::vec![0u64; n64.len()];
                limbs_divmod_dispatch_u64(&n64, &d64, &mut q_ref, &mut r_ref);

                let mut q_knuth = alloc::vec![0u64; n64.len()];
                let mut r_knuth = alloc::vec![0u64; n64.len()];
                limbs_divmod_knuth_u64(&n64, &d64, &mut q_knuth, &mut r_knuth);

                assert_eq!(q_knuth, q_ref, "knuth q mismatch");
                assert_eq!(r_knuth, r_ref, "knuth r mismatch");
            }
        }
    }

    /// `MG3by2U64` matches the `limbs_divmod_u64` oracle on a
    /// representative corpus. Tests the corner cases that historically
    /// broke MG 3-by-2 implementations: numerator near divisor (so the
    /// initial q estimate may overshoot by 2), minimal normalised d1
    /// (= B/2), maximal d1, and the paper's worst-case corner where r1
    /// must compare against q_lo without underflow.
    #[test]
    fn mg3by2_u64_matches_reference() {
        let cases: &[(u64, u64, u64, u64, u64)] = &[
            // (n2, n1, n0, d1, d0) — d1 normalised, (n2, n1) < (d1, d0).
            // Minimal normalised d1 = B/2.
            (0, 0, 1, 1u64 << 63, 0),
            (0, 1, 0, 1u64 << 63, 0),
            ((1u64 << 63) - 1, u64::MAX, u64::MAX, 1u64 << 63, 1),
            // Maximal d1 = B-1.
            (u64::MAX - 1, u64::MAX, u64::MAX, u64::MAX, u64::MAX),
            (0, 0, 1, u64::MAX, 1),
            // Mid-range divisor; numerator just under (d1, d0).
            (
                0xc0ffee,
                0xdead_beef,
                0xface_b00c,
                (1u64 << 63) | 0xc0ffee_u64,
                0xdead_beef_face_b00c,
            ),
            // Small numerator vs large divisor (quotient = 0).
            (0, 1, 2, (1u64 << 63) | 1, 2),
            // Numerator = divisor (quotient = 1, remainder = 0). Need to
            // express (d1, d0, 0) carefully: n2 = 0, then we'd violate
            // the precondition. Skip; this is a degenerate corner the
            // Knuth caller never hits.
        ];
        for &(n2, n1, n0, d1, d0) in cases {
            assert!(d1 >> 63 == 1, "d1 not normalised: {d1:#x}");
            assert!(
                n2 < d1 || (n2 == d1 && n1 < d0),
                "test precondition (n2, n1) < (d1, d0) violated"
            );
            let mg = MG3by2U64::new(d1, d0);
            let (q, r1, r0) = mg.div_rem(n2, n1, n0);

            // Reference: 3-limb numerator / 2-limb divisor via
            // limbs_divmod_u64. The function requires
            // `rem.len() >= num.len()` so size both at 3.
            let num = alloc::vec![n0, n1, n2];
            let den = alloc::vec![d0, d1];
            let mut q_ref = alloc::vec![0u64; 3];
            let mut r_ref = alloc::vec![0u64; 3];
            limbs_divmod_u64(&num, &den, &mut q_ref, &mut r_ref);

            assert_eq!(
                q_ref[0], q,
                "MG3by2 q mismatch for n=({n2:#x},{n1:#x},{n0:#x}) d=({d1:#x},{d0:#x})"
            );
            assert_eq!(
                q_ref[1], 0,
                "MG3by2 q higher limb non-zero — precondition violated"
            );
            assert_eq!(
                q_ref[2], 0,
                "MG3by2 q higher limb non-zero — precondition violated"
            );
            assert_eq!(r_ref[0], r0, "MG3by2 r0 mismatch");
            assert_eq!(r_ref[1], r1, "MG3by2 r1 mismatch");
        }
    }

    /// `MG2by1U64` matches a reference 2-by-1 divide.
    #[test]
    fn mg2by1_u64_matches_reference() {
        let cases: &[(u64, u64, u64)] = &[
            (0, 1, 1u64 << 63),
            (0, u64::MAX, 1u64 << 63),
            ((1u64 << 63) - 1, u64::MAX, 1u64 << 63),
            (0, 1, u64::MAX),
            (u64::MAX - 1, u64::MAX, u64::MAX),
            (12345, 67890, (1u64 << 63) | 0xdead_beef_u64),
            (u64::MAX - 1, 0, u64::MAX),
        ];
        for &(u1, u0, d) in cases {
            assert!(d >> 63 == 1);
            assert!(u1 < d);
            let mg = MG2by1U64::new(d);
            let (q, r) = mg.div_rem(u1, u0);
            // Reference: ((u1 as u128) << 64 | u0 as u128) / (d as u128)
            let num = ((u1 as u128) << 64) | (u0 as u128);
            let exp_q = (num / (d as u128)) as u64;
            let exp_r = (num % (d as u128)) as u64;
            assert_eq!(
                (q, r),
                (exp_q, exp_r),
                "MG u64 mismatch for {u1:#x}, {u0:#x}, d={d:#x}"
            );
        }
    }

    /// `limbs_divmod_knuth_u64` agrees with the dispatch path
    /// (`limbs_divmod_dispatch_u64`) on a battery of representative
    /// shapes — single-limb divisors, multi-limb divisors, zero
    /// remainders, partial overflows in the q̂ refinement step.
    #[test]
    fn knuth_matches_canonical_divmod() {
        let cases: &[(&[u64], &[u64])] = &[
            // Simple
            (&[42], &[7]),
            (&[u64::MAX, 0], &[2]),
            // Multi-limb numerator, single-limb denominator.
            (&[1, 1, 0, 0], &[3]),
            // Multi-limb both — three-limb numerator by two-limb den.
            (&[u64::MAX, u64::MAX, 1, 0], &[5, 9]),
            // Three-limb both.
            (&[u64::MAX, u64::MAX, u64::MAX, 0], &[1, 2, 3]),
            // Numerator < denominator — quotient zero, remainder = num.
            (&[100, 0, 0], &[200, 0, 1]),
            // Equal high limbs (forces the u_top ≥ v_top branch).
            (&[0, 0, u64::MAX, u64::MAX], &[1, 2, u64::MAX]),
        ];
        for (num, den) in cases {
            let mut q_canon = [0u64; 8];
            let mut r_canon = [0u64; 8];
            limbs_divmod_dispatch_u64(num, den, &mut q_canon, &mut r_canon);
            let mut q_knuth = [0u64; 8];
            let mut r_knuth = [0u64; 8];
            limbs_divmod_knuth_u64(num, den, &mut q_knuth, &mut r_knuth);
            assert_eq!(
                q_canon, q_knuth,
                "quotient mismatch on {:?} / {:?}",
                num, den
            );
            assert_eq!(
                r_canon, r_knuth,
                "remainder mismatch on {:?} / {:?}",
                num, den
            );
        }
    }

    /// `limbs_divmod_bz_u64` agrees with the dispatch path on
    /// medium-and-large operands. Recursion engages only above the
    /// `BZ_THRESHOLD_U64` limb cutoff.
    #[test]
    fn bz_matches_canonical_divmod() {
        // Builds a 40-limb dividend with a 20-limb divisor — well above
        // BZ_THRESHOLD_U64 so the recursive path is exercised.
        let mut num = [0u64; 40];
        for (i, slot) in num.iter_mut().enumerate() {
            *slot = (i as u64)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(i as u64);
        }
        let mut den = [0u64; 20];
        for (i, slot) in den.iter_mut().enumerate() {
            *slot = ((i + 1) as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        }
        let mut q_canon = [0u64; 40];
        let mut r_canon = [0u64; 40];
        limbs_divmod_knuth_u64(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_bz = [0u64; 40];
        let mut r_bz = [0u64; 40];
        limbs_divmod_bz_u64(&num, &den, &mut q_bz, &mut r_bz);
        assert_eq!(q_canon, q_bz, "BZ quotient mismatch");
        assert_eq!(r_canon, r_bz, "BZ remainder mismatch");
    }

    /// `Int::<4>::as_u128` returns the low 128 magnitude bits — the
    /// truncating `Int<4> → u128` cast.
    #[test]
    fn as_u128_returns_low_magnitude_bits() {
        let src = Int::<4>::from_i128(123_456_789);
        let dst: u128 = src.as_u128();
        assert_eq!(dst, 123_456_789);
        // Casting ZERO yields 0.
        let dst: u128 = Int::<4>::ZERO.as_u128();
        assert_eq!(dst, 0);
    }

    /// Knuth's q̂-cap path fires when `u_top >= v_top` in the
    /// per-quotient-limb loop. We engineer a dividend whose normalised
    /// top limb equals the normalised divisor top so the cap (`q̂ =
    /// u64::MAX`, plus the subsequent multiply-subtract correction)
    /// runs, then verify the resulting quotient matches the dispatch
    /// path (`limbs_divmod_dispatch_u64`).
    #[test]
    fn knuth_q_hat_cap_branch_matches_canonical() {
        // num top limb == den top limb; div quotient's first chunk hits
        // the cap. Picking the divisor's top close to u64::MAX tightens
        // the normalisation shift.
        let num: [u64; 4] = [0, 0, u64::MAX, u64::MAX >> 1];
        let den: [u64; 3] = [1, 2, u64::MAX >> 1];
        let mut q_canon = [0u64; 4];
        let mut r_canon = [0u64; 4];
        limbs_divmod_dispatch_u64(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_knuth = [0u64; 4];
        let mut r_knuth = [0u64; 4];
        limbs_divmod_knuth_u64(&num, &den, &mut q_knuth, &mut r_knuth);
        assert_eq!(q_canon, q_knuth);
        assert_eq!(r_canon, r_knuth);
    }

    /// `limbs_divmod_bz_u64` with a numerator that has trailing zero
    /// limbs strips them off in its top-non-zero scan before deciding
    /// whether to recurse.
    #[test]
    fn bz_strips_numerator_trailing_zeros() {
        // 32-limb buffer but only the low half is non-zero; den is 20
        // limbs. BZ should recognise top < 2*n and fall back to Knuth.
        let mut num = [0u64; 32];
        for slot in &mut num[..16] {
            *slot = 0xCAFE_F00D;
        }
        let mut den = [0u64; 20];
        den[0] = 7;
        let mut q_canon = [0u64; 32];
        let mut r_canon = [0u64; 32];
        limbs_divmod_knuth_u64(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_bz = [0u64; 32];
        let mut r_bz = [0u64; 32];
        limbs_divmod_bz_u64(&num, &den, &mut q_bz, &mut r_bz);
        assert_eq!(q_canon, q_bz);
        assert_eq!(r_canon, r_bz);
    }
}
