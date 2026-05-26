// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Per-term `÷10^w` divide bench — the divide shape the wide-tier
//! transcendentals (`exp`/`ln` Taylor terms) actually run, plus a
//! faithful port of the v0.4.4 u128-limb Knuth divide as a candidate
//! arm.
//!
//! The wide-tier strict transcendentals hold a value at working scale
//! `w = SCALE + GUARD` (`GUARD = 60`) and divide by `10^w` once per
//! Taylor term. That divisor is a *power of ten* whose effective limb
//! count is `ceil(w·log2(10)/64) ≈ w/19` — far NARROWER than the storage
//! tier. The dividend is the term product `a·b` (~`2w` digits). So the
//! real per-term divide is a `~2k / k` shape with `k ≈ w/19` limbs,
//! the SAME across every storage tier — NOT the full-storage-width
//! `2N / N` shape `div_kernel_ab` measures.
//!
//! This bench measures that real shape and ranks, in one `compare_all`
//! per `w`:
//!  - `dispatch` — the production routing (`int::policy::div_rem`),
//!  - `knuth`    — base-2⁶⁴ Knuth (`div_knuth`),
//!  - `u128`     — our base-2¹²⁸ Knuth (`div_knuth_u128_limb`,
//!                 `Mg3By2`-based `q̂` via a 256/128 software divide),
//!  - `v044_u128`— the v0.4.4 base-2¹²⁸ Knuth `q̂` scheme: ONE
//!                 precomputed MG 2-by-1 reciprocal of the top u128
//!                 divisor limb + a refinement loop (the divide
//!                 v0.4.4 shipped on its `[u128]` limb storage).
//!
//! All candidates produce the same exact quotient/remainder; the
//! correctness gate asserts it before timing.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench div_pow10_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    div_dispatch_slice, div_knuth_slice, div_knuth_u128_limb_slice,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

// ── operand builders: the real per-term shape ─────────────────────────

/// `10^w` as a little-endian u64 magnitude (leading-zero-trimmed).
fn pow10_limbs(w: u32) -> Vec<u64> {
    // Build by repeated ×10 in u64 limbs (no u128 pow that overflows at
    // w ≥ 39).
    let mut v = vec![1u64];
    for _ in 0..w {
        let mut carry: u128 = 0;
        for limb in v.iter_mut() {
            let acc = (*limb as u128) * 10 + carry;
            *limb = acc as u64;
            carry = acc >> 64;
        }
        if carry != 0 {
            v.push(carry as u64);
        }
    }
    v
}

/// A pseudo-random `used`-limb little-endian magnitude with the top
/// limb's MSB set.
fn fill(seed: u64, used: usize) -> Vec<u64> {
    let mut v = vec![0u64; used];
    for i in 0..used {
        v[i] = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add((i as u64).wrapping_mul(0x1357_9BDF))
            ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    }
    if used > 0 {
        v[used - 1] |= 0x8000_0000_0000_0000;
    }
    v
}

#[derive(Clone)]
struct Shape {
    label: &'static str,
    num: Vec<u64>,
    den: Vec<u64>,
}

/// The per-term shape for working scale `w`: divisor `10^w`, dividend a
/// `~2·den_n`-limb product-sized random value (the Taylor term `a·b`
/// before the rescale).
fn shape_for_w(w: u32) -> Shape {
    let den = pow10_limbs(w);
    let num_n = 2 * den.len();
    let num = fill(1009, num_n);
    Shape { label: "div_pow10", num, den }
}

// ── candidate runners ─────────────────────────────────────────────────

fn run_dispatch(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    div_dispatch_slice(&s.num, &s.den, &mut q, &mut r);
    r
}
fn run_knuth(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    div_knuth_slice(&s.num, &s.den, &mut q, &mut r);
    r
}
fn run_u128(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    div_knuth_u128_limb_slice(&s.num, &s.den, &mut q, &mut r);
    r
}
fn run_v044(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    v044_u128::divmod(&s.num, &s.den, &mut q, &mut r);
    r
}

// ── v0.4.4 base-2¹²⁸ Knuth (MG 2-by-1 q̂ + refinement) ─────────────────
//
// A faithful port of v0.4.4's `limbs_divmod_knuth` (which ran on `[u128]`
// limbs) to the 0.5.0 `&[u64]` slice contract: pack to u128, run the
// exact same q̂ scheme — ONE precomputed MG 2-by-1 reciprocal of the top
// u128 divisor limb + the v[n-2] refinement loop — then unpack. This is
// the candidate that answers "was v0.4.4's u128 divide faster than ours
// at the per-term ÷10^w shape?" against our `Mg3By2`-based `u128` engine.
mod v044_u128 {
    /// 128×128→256 widening multiply (native split).
    #[inline]
    fn mul_128(a: u128, b: u128) -> (u128, u128) {
        let a_lo = a as u64 as u128;
        let a_hi = a >> 64;
        let b_lo = b as u64 as u128;
        let b_hi = b >> 64;
        let ll = a_lo * b_lo;
        let lh = a_lo * b_hi;
        let hl = a_hi * b_lo;
        let hh = a_hi * b_hi;
        let (mid, c1) = lh.overflowing_add(hl);
        let (lo, c2) = ll.overflowing_add(mid << 64);
        let hi = hh + (mid >> 64) + ((c1 as u128) << 64) + (c2 as u128);
        (hi, lo)
    }

    /// 2-by-1 bit-recovery divide `(high·B + low)/d`, `high < d`. Used
    /// only for the reciprocal setup.
    fn div_2_by_1(high: u128, low: u128, d: u128) -> (u128, u128) {
        let mut r = high;
        let mut q: u128 = 0;
        let mut i = 128;
        while i > 0 {
            i -= 1;
            let r_top = r >> 127;
            r = (r << 1) | ((low >> i) & 1);
            q <<= 1;
            if r_top == 1 || r >= d {
                r = r.wrapping_sub(d);
                q |= 1;
            }
        }
        (q, r)
    }

    /// MG 2-by-1 invariant divisor on u128 limbs (v0.4.4 `MG2by1`).
    #[derive(Clone, Copy)]
    struct Mg2By1U128 {
        d: u128,
        v: u128,
    }
    impl Mg2By1U128 {
        #[inline]
        fn new(d: u128) -> Self {
            let (v, _r) = div_2_by_1(!d, u128::MAX, d);
            Self { d, v }
        }
        #[inline]
        fn div_rem(&self, u1: u128, u0: u128) -> (u128, u128) {
            let (vu1_hi, vu1_lo) = mul_128(self.v, u1);
            let (q0, c_lo) = vu1_lo.overflowing_add(u0);
            let (q1, _) = vu1_hi.overflowing_add(u1);
            let (q1, _) = q1.overflowing_add(c_lo as u128);
            let q1 = q1.wrapping_add(1);
            let r = u0.wrapping_sub(q1.wrapping_mul(self.d));
            let (q1, r) = if r > q0 {
                (q1.wrapping_sub(1), r.wrapping_add(self.d))
            } else {
                (q1, r)
            };
            if r >= self.d {
                (q1.wrapping_add(1), r.wrapping_sub(self.d))
            } else {
                (q1, r)
            }
        }
    }

    fn eff(s: &[u64]) -> usize {
        let mut n = s.len();
        while n > 0 && s[n - 1] == 0 {
            n -= 1;
        }
        n
    }

    /// Pack even-length u64 slice into u128 limbs (little-endian).
    fn pack(src: &[u64], dst: &mut [u128]) {
        for (k, slot) in dst.iter_mut().enumerate() {
            let lo = src[2 * k] as u128;
            let hi = if 2 * k + 1 < src.len() { src[2 * k + 1] as u128 } else { 0 };
            *slot = lo | (hi << 64);
        }
    }
    fn unpack(src: &[u128], dst: &mut [u64]) {
        for (k, &limb) in src.iter().enumerate() {
            if 2 * k < dst.len() {
                dst[2 * k] = limb as u64;
            }
            if 2 * k + 1 < dst.len() {
                dst[2 * k + 1] = (limb >> 64) as u64;
            }
        }
    }

    /// v0.4.4-faithful u128-limb Knuth D. Falls back to the production
    /// base-2⁶⁴ Knuth for odd / single-limb / `num < den` shapes (those
    /// have no exact u128 packing) so the candidate stays valid over the
    /// whole input set.
    pub(super) fn divmod(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
        for q in quot.iter_mut() {
            *q = 0;
        }
        for r in rem.iter_mut() {
            *r = 0;
        }
        let n64 = eff(den);
        assert!(n64 > 0, "v044_u128: divide by zero");
        let top64 = eff(num);
        if top64 < n64 {
            let copy = num.len().min(rem.len());
            rem[..copy].copy_from_slice(&num[..copy]);
            return;
        }
        if n64 < 2 || n64 % 2 != 0 {
            decimal_scaled::__bench_internals::div_knuth_slice(num, den, quot, rem);
            return;
        }

        // Normalise so the top u64 limb's MSB is set (also normalises the
        // top u128 limb's bit 127).
        let shift = den[n64 - 1].leading_zeros();
        let mut u64buf = vec![0u64; top64 + 2];
        let mut v64buf = vec![0u64; n64];
        if shift == 0 {
            u64buf[..top64].copy_from_slice(&num[..top64]);
            v64buf[..n64].copy_from_slice(&den[..n64]);
        } else {
            let mut carry = 0u64;
            for i in 0..top64 {
                let val = num[i];
                u64buf[i] = (val << shift) | carry;
                carry = val >> (64 - shift);
            }
            u64buf[top64] = carry;
            carry = 0;
            for i in 0..n64 {
                let val = den[i];
                v64buf[i] = (val << shift) | carry;
                carry = val >> (64 - shift);
            }
        }
        let mut u_len64 = if u64buf[top64] != 0 { top64 + 1 } else { top64 };
        u_len64 += u_len64 & 1;
        let n128 = n64 / 2;
        let u_len128 = u_len64 / 2;
        let mut u = vec![0u128; u_len128 + 1];
        let mut v = vec![0u128; n128];
        pack(&u64buf[..u_len64], &mut u[..u_len128]);
        pack(&v64buf[..n64], &mut v[..n128]);

        let m128 = u_len128 - n128;
        let v_top = v[n128 - 1];
        let v_below = v[n128 - 2];
        let mg_top = Mg2By1U128::new(v_top);

        let mut jp1 = m128 + 1;
        while jp1 > 0 {
            jp1 -= 1;
            let j = jp1;
            let jn = j + n128;
            let u_top = u[jn];
            let u_next = u[jn - 1];

            let (mut q_hat, mut r_hat) = if u_top > v_top {
                (u128::MAX, u128::MAX)
            } else if u_top == v_top {
                let (r, of) = u_next.overflowing_add(v_top);
                (u128::MAX, if of { u128::MAX } else { r })
            } else {
                mg_top.div_rem(u_top, u_next)
            };

            loop {
                let (hi, lo) = mul_128(q_hat, v_below);
                if hi < r_hat || (hi == r_hat && lo <= u[jn - 2]) {
                    break;
                }
                q_hat = q_hat.wrapping_sub(1);
                let (nr, of) = r_hat.overflowing_add(v_top);
                if of {
                    break;
                }
                r_hat = nr;
            }

            // D4: u[j..=j+n128] -= q̂·v, u128 carry-merge (the proven
            // single-u128-carry pattern: carry accumulates q̂·v[i]'s high
            // word plus the borrow from the previous limb).
            let mut carry: u128 = 0;
            for i in 0..n128 {
                let (p_hi, p_lo) = mul_128(q_hat, v[i]);
                let (acc_lo, k) = p_lo.overflowing_add(carry);
                let acc_hi = p_hi + (k as u128);
                let (res, b) = u[j + i].overflowing_sub(acc_lo);
                u[j + i] = res;
                carry = acc_hi + (b as u128);
            }
            let (s2, b1) = u[jn].overflowing_sub(carry);
            u[jn] = s2;
            if b1 {
                q_hat = q_hat.wrapping_sub(1);
                let mut c: u128 = 0;
                for i in 0..n128 {
                    let (s1, c1) = u[j + i].overflowing_add(v[i]);
                    let (s2, c2) = s1.overflowing_add(c);
                    u[j + i] = s2;
                    c = (c1 as u128) + (c2 as u128);
                }
                u[jn] = u[jn].wrapping_add(c);
            }
            if 2 * j < quot.len() {
                quot[2 * j] = q_hat as u64;
            }
            if 2 * j + 1 < quot.len() {
                quot[2 * j + 1] = (q_hat >> 64) as u64;
            }
        }

        // Unpack + denormalise remainder.
        let mut r64 = vec![0u64; n64];
        unpack(&u[..n128], &mut r64);
        if shift == 0 {
            let copy = n64.min(rem.len());
            rem[..copy].copy_from_slice(&r64[..copy]);
        } else {
            for i in 0..n64 {
                if i < rem.len() {
                    let lo = r64[i] >> shift;
                    let hi = if i + 1 < n64 { r64[i + 1] << (64 - shift) } else { 0 };
                    rem[i] = lo | hi;
                }
            }
        }
    }
}

fn compare_w(c: &mut Criterion, w: u32) {
    let s = shape_for_w(w);
    // Correctness gate: every candidate agrees with the production
    // dispatcher before timing.
    let mut q0 = vec![0u64; s.num.len()];
    let mut r0 = vec![0u64; s.num.len()];
    div_dispatch_slice(&s.num, &s.den, &mut q0, &mut r0);
    {
        let mut q = vec![0u64; s.num.len()];
        let mut r = vec![0u64; s.num.len()];
        div_knuth_slice(&s.num, &s.den, &mut q, &mut r);
        assert_eq!(q, q0, "knuth quot mismatch w={w}");
        assert_eq!(r, r0, "knuth rem mismatch w={w}");
    }
    {
        let mut q = vec![0u64; s.num.len()];
        let mut r = vec![0u64; s.num.len()];
        div_knuth_u128_limb_slice(&s.num, &s.den, &mut q, &mut r);
        assert_eq!(q, q0, "u128 quot mismatch w={w}");
        assert_eq!(r, r0, "u128 rem mismatch w={w}");
    }
    {
        let mut q = vec![0u64; s.num.len()];
        let mut r = vec![0u64; s.num.len()];
        v044_u128::divmod(&s.num, &s.den, &mut q, &mut r);
        assert_eq!(q, q0, "v044 quot mismatch w={w}");
        assert_eq!(r, r0, "v044 rem mismatch w={w}");
    }
    compare_all(
        c,
        &format!("div_pow10/w{w}"),
        |s: &Shape| format!("{}_{}limb_den", s.label, s.den.len()),
        vec![s],
        vec![
            ("dispatch", run_dispatch as fn(Shape) -> Vec<u64>),
            ("knuth", run_knuth),
            ("u128", run_u128),
            ("v044_u128", run_v044),
        ],
    );
}

fn bench(c: &mut Criterion) {
    // Working scales `w = SCALE + GUARD` (GUARD = 60) for the wide tiers,
    // plus a couple of wider points to see the trend:
    //   w=64  (SCALE 4 + 60)   den ~ 4 limbs
    //   w=90  (SCALE 30 + 60)  den ~ 5 limbs   (the bbc exp@scale-30 cell)
    //   w=120                  den ~ 7 limbs
    //   w=180                  den ~ 10 limbs
    //   w=264 (golden cap)     den ~ 14 limbs
    //   w=360                  den ~ 19 limbs
    for &w in &[64u32, 90, 120, 180, 264, 360] {
        compare_w(c, w);
    }
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
