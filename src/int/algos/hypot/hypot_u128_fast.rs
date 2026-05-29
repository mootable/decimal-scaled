// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `hypot_u128_fast` -- integer hypotenuse `round(sqrt(a² + b²))` with a
//! native scalar fast path for the common small-operand case, falling back to
//! the exact wide [`hypot_pythagoras`] kernel otherwise.
//!
//! Background. [`hypot_pythagoras`] forms the exact radicand `n = a² + b²`,
//! takes `q = floor(sqrt(n))` via the width-agnostic Newton slice `isqrt`
//! (which runs the runtime `div_rem` dispatcher once per iteration), then
//! rounds using the exact remainder `n - q²` against `q`. The Newton divide
//! loop dominates the cost. But when both operands fit a FIXED scalar width
//! the whole computation -- radicand, floor root, exact remainder -- can be
//! done in fixed `u128`/`u256` scalar arithmetic with NO multi-precision
//! `div_rem` dispatcher:
//!
//! - both operands `< 2^64` (`fit_one`): the radicand `a² + b²` fits a single
//!   `u128`, rooted by [`isqrt_u128`] (an f64-seeded `u128` Newton);
//! - both operands `< 2^128` (`fit_k(_, 2)`): each square fits a `u256` and
//!   the sum fits a `u256` (with a carry guard), rooted by [`isqrt_u256`] (an
//!   f64-seeded Newton whose only divide is a FIXED `u256 / u128` long
//!   division -- a scalar routine, not the multi-precision dispatcher). This
//!   covers ALL of D38 (`Int<2>`) and the entire low magnitude band of every
//!   wider tier, which is exactly the decimal `s >= 19` slow band the wide
//!   `fit_one`-only gate used to cliff into [`hypot_pythagoras`] for.
//!
//! The round step is the SAME exact remainder test in both arms (`n - q² > q`
//! for the half modes, `n - q² != 0` for ceiling), so the delivered value is
//! bit-identical to [`hypot_pythagoras`] for every input and every
//! [`RoundingMode`]; operands beyond `2^128` still take the exact wide path.
//!
//! Correctly rounded. `q = floor(sqrt(n))` is the exact floor root of the
//! exact integer radicand; the exact midpoint between `q` and `q+1` is
//! `q + 1/2`, whose square `q² + q + 1/4` is never an integer, so an integer
//! `n` can never land exactly on a half-way tie. Hence `n - q² > q`
//! (equivalently `n >= q² + q + 1`) is the exact "above half-way" predicate
//! and the three half-* modes coincide -- matching [`hypot_pythagoras`].
//!
//! Cite: the floor-root-plus-exact-remainder rounding structure is the
//! integer specialisation of the correctly-rounded `sqrt(x²+y²)` device in
//! C. F. Borges, "An Improved Algorithm for hypot(a,b)", arXiv:1904.09481
//! (2019) -- there in floating point with an FMA remainder; here the
//! remainder is exact because the radicand is an exact integer.

use crate::int::algos::hypot::hypot_pythagoras::hypot_pythagoras;
use crate::int::algos::support::limbs::{fit_k, fit_one};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// `floor(sqrt(n))` for a `u128`, exact.
///
/// Seeds from the shared seed library's `u128` over-estimate
/// ([`sqrt_seed_u128`]), then runs the integer Newton averaging step
/// `x <- (x + n/x)/2`. From a guaranteed over-estimate that recurrence is
/// monotone-decreasing and lands on `floor(sqrt(n))` or `floor+1`; a single
/// down-correction `if` then pins the exact floor.
///
/// An f64-`sqrt` seed of the *full* `n` was tried but rejected: for `n` near
/// `2^128` the f64 root carries an absolute error of order `2^12`, and when
/// that lands on the *under* side it is no longer an over-estimate, so Newton
/// terminates early and the floor correction degrades to a linear `x -= 1`
/// walk -- thousands of `u128` divides on the worst inputs. The library seed
/// is a guaranteed over-estimate, so the descent is always the fast quadratic
/// one and the std/no_std paths are identical.
///
/// [`sqrt_seed_u128`]: crate::algo_x_support::seed::sqrt_seed_u128
#[inline]
fn isqrt_u128(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let bits = 128 - n.leading_zeros();
    let mut x: u128 = crate::algo_x_support::seed::sqrt_seed_u128(n, bits);
    // Newton averaging: x <- (x + n/x)/2. Monotone-decreasing from an
    // over-estimate; stop when it stops decreasing (x is then floor or floor+1).
    loop {
        let y = (x + n / x) >> 1;
        if y >= x {
            break;
        }
        x = y;
    }
    // Single floor correction: x is floor or floor+1, so at most one step.
    if x > n / x {
        x -= 1;
    }
    x
}

/// Full `128 × 128 -> 256` product `x · x`, as a little-endian `[u64; 4]`.
///
/// Delegates to the shared fixed-width square leaf
/// [`sqr_low_fixed`](crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed): `x`
/// (`< 2^128`) zero-extended to four `u64` limbs has an exact `< 2^256` square,
/// so its *full* product is exactly the low four limbs of the `N = 4` square.
/// `sqr_low_fixed` is a `const fn` over `&[u64; N]` / `&mut [u64; N]` -- pure
/// fixed-width stack arithmetic, NO allocation and NO `ComputeInt` slice
/// scratch -- so the no-allocation hot-path constraint is preserved while the
/// squaring logic stays single-sourced in the leaf (not re-hand-rolled here).
#[inline]
fn sq_u256(x: u128) -> [u64; 4] {
    let xw = [x as u64, (x >> 64) as u64, 0, 0];
    let mut out = [0u64; 4];
    crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed::<4>(&xw, &mut out);
    out
}

/// `a + b` for two `u256` (`[u64; 4]`) values, returning the 257-bit sum as
/// `([u64; 4], carry)` where `carry` is the bit-256 overflow.
#[inline]
fn add_u256(a: [u64; 4], b: [u64; 4]) -> ([u64; 4], bool) {
    let mut out = [0u64; 4];
    let mut carry = 0u128;
    let mut i = 0;
    while i < 4 {
        let s = a[i] as u128 + b[i] as u128 + carry;
        out[i] = s as u64;
        carry = s >> 64;
        i += 1;
    }
    (out, carry != 0)
}

/// `n / d` for a `u256` dividend `n` (`[u64; 4]`) and a `u128` divisor `d >= 1`,
/// returning the quotient as a `u128`. The caller guarantees the quotient fits
/// a `u128` (in the Newton loop `x` is an over-estimate of `sqrt(n)`, so
/// `n / x <= x <= 2^128`). FIXED scalar long division with `u64` base digits,
/// using the *hardware* `u128 / u64` divide for each quotient-digit estimate
/// (NOT a bit-by-bit loop, and NOT the multi-precision `div_rem` dispatcher).
///
/// Strategy. When `d` fits one `u64` limb, do a straight 4-digit base-`2^64`
/// long division by the `u64` divisor (four hardware `u128 / u64` steps). When
/// `d` is a genuine 2-limb divisor, fall back to Knuth Algorithm D over the
/// two normalised divisor limbs ([`knuth_d_256_by_128`]). Both are fixed,
/// branch-light scalar routines.
#[inline]
fn div_u256_by_u128(n: [u64; 4], d: u128) -> u128 {
    let d_hi = (d >> 64) as u64;
    if d_hi == 0 {
        // Single-limb divisor: classic base-2^64 long division, MSB limb first.
        let d0 = d as u64; // != 0 (caller guarantees d >= 1)
        let mut q = [0u64; 4];
        let mut rem: u64 = 0;
        let mut i = 4;
        while i > 0 {
            i -= 1;
            let cur = ((rem as u128) << 64) | n[i] as u128; // < d0·2^64 (rem < d0)
            q[i] = (cur / d0 as u128) as u64;
            rem = (cur % d0 as u128) as u64;
        }
        // quotient fits u128 by the caller's invariant (top two limbs zero).
        (q[0] as u128) | ((q[1] as u128) << 64)
    } else {
        knuth_d_256_by_128(n, d)
    }
}

/// Knuth Algorithm D for a `u256 / u128` with a genuine 2-limb (normalised)
/// divisor, quotient known to fit `u128`. Base `2^64`; the quotient-digit
/// estimate uses one hardware `u128 / u64` divide per digit with the standard
/// two-term correction. A fixed scalar routine (no recursion, no dispatcher).
#[inline]
fn knuth_d_256_by_128(n: [u64; 4], d: u128) -> u128 {
    // Normalise so the divisor's top limb has its MSB set. `d_hi != 0` here,
    // so its u64 leading-zero count is in 0..=63 -- the correct normalising
    // shift (taking it on the full u128 would count the 64 high zero bits and
    // over-shift the divisor).
    let d_hi = (d >> 64) as u64;
    let shift = d_hi.leading_zeros(); // 0..=63
    let dn: u128 = d << shift;
    let v1 = (dn >> 64) as u64; // top limb, MSB set
    let v0 = dn as u64;

    // Shifted dividend, 5 limbs (the shift can spill into a 5th).
    let mut u = [0u64; 5];
    if shift == 0 {
        u[0] = n[0];
        u[1] = n[1];
        u[2] = n[2];
        u[3] = n[3];
    } else {
        let s = shift;
        u[0] = n[0] << s;
        u[1] = (n[1] << s) | (n[0] >> (64 - s));
        u[2] = (n[2] << s) | (n[1] >> (64 - s));
        u[3] = (n[3] << s) | (n[2] >> (64 - s));
        u[4] = n[3] >> (64 - s);
    }

    // Two quotient digits (q1 at 2^64, q0 at 2^0); higher digits are zero by
    // the caller's "quotient fits u128" guarantee.
    let mut q = [0u64; 2];
    let mut j = 2; // process positions j = 1, 0 (dividend limbs [j+2..j])
    while j > 0 {
        j -= 1;
        // Estimate qhat = (u[j+2]·2^64 + u[j+1]) / v1, capped to a u64 digit.
        let num = ((u[j + 2] as u128) << 64) | u[j + 1] as u128;
        let mut qhat = num / v1 as u128;
        let mut rhat = num % v1 as u128;
        // Cap qhat at the base digit max (b - 1) so `qhat·v0` stays < 2^128 for
        // the correction test below (standard Knuth-D guard).
        if qhat >= (1u128 << 64) {
            qhat = (1u128 << 64) - 1;
            rhat = num - qhat * v1 as u128;
        }
        // Correct the over-estimate (at most twice): qhat·v0 > rhat·b + u[j].
        while rhat < (1u128 << 64) && qhat * v0 as u128 > (rhat << 64) + u[j] as u128 {
            qhat -= 1;
            rhat += v1 as u128;
        }
        // Multiply-subtract qhat·(v1:v0) from u[j..j+3]. The product spans 3
        // limbs (qhat·v0 at limb j, qhat·v1 at limb j+1), formed via two
        // u64×u64 -> u128 products so nothing truncates.
        let qh = qhat as u64;
        let m0 = qh as u128 * v0 as u128; // weight 2^(64j)
        let m1 = qh as u128 * v1 as u128; // weight 2^(64(j+1))
        let p_lo = m0 as u64;
        let p_mid = ((m0 >> 64) + (m1 as u64 as u128)) as u128; // limb j+1 column
        let p_hi = (p_mid >> 64) + (m1 >> 64); // limb j+2 column
        let p_mid = p_mid as u64;
        let p_hi = p_hi as u64;
        // subtract (p_hi:p_mid:p_lo) at limbs j, j+1, j+2.
        let mut borrow: i128 = 0;
        let s0 = u[j] as i128 - p_lo as i128 - borrow;
        u[j] = s0 as u64;
        borrow = if s0 < 0 { 1 } else { 0 };
        let s1 = u[j + 1] as i128 - p_mid as i128 - borrow;
        u[j + 1] = s1 as u64;
        borrow = if s1 < 0 { 1 } else { 0 };
        let s2 = u[j + 2] as i128 - p_hi as i128 - borrow;
        u[j + 2] = s2 as u64;
        borrow = if s2 < 0 { 1 } else { 0 };
        // If we over-subtracted, add the divisor back once and decrement qhat.
        let mut qd = qh;
        if borrow != 0 {
            qd = qd.wrapping_sub(1);
            let a0 = u[j] as u128 + v0 as u128;
            u[j] = a0 as u64;
            let a1 = u[j + 1] as u128 + v1 as u128 + (a0 >> 64);
            u[j + 1] = a1 as u64;
            u[j + 2] = u[j + 2].wrapping_add((a1 >> 64) as u64);
        }
        q[j] = qd;
    }
    (q[0] as u128) | ((q[1] as u128) << 64)
}

/// `floor(sqrt(n))` for a `u256` radicand `n` (`[u64; 4]`, `n < 2^256`), exact.
/// Returns the floor root as a `u128` (it is `< 2^128`). Same f64-seeded
/// over-estimate Newton structure as [`isqrt_u128`], but `n / x` is the fixed
/// scalar [`div_u256_by_u128`] (hardware `u128 / u64` digit estimates).
#[inline]
fn isqrt_u256(n: [u64; 4]) -> u128 {
    // Bit length of the 256-bit magnitude.
    let bits: u32 = if n[3] != 0 {
        192 + (64 - n[3].leading_zeros())
    } else if n[2] != 0 {
        128 + (64 - n[2].leading_zeros())
    } else if n[1] != 0 {
        64 + (64 - n[1].leading_zeros())
    } else {
        64 - n[0].leading_zeros()
    };
    if bits == 0 {
        return 0;
    }
    // Over-estimate seed of sqrt(n) from the shared, proven seed library. Its
    // `sqrt_seed(&[u64], bits, out)` leaf is `&[u64]`-generic and width-agnostic
    // BY DESIGN (its doc: "decimal-side consumers can reuse them unchanged"), so
    // pass the radicand slice + its bit length directly. The library guarantees
    // a strict over-estimate (its std body does a double `+1`, so the placed
    // seed is `>= sqrt(n)`), which the Newton loop below relies on. `out` is one
    // limb wider than the root (which is `< 2^128`, two limbs) so the rare
    // over-estimate that touches `2^128` still fits; we read it back as a u128,
    // saturating to `u128::MAX` if the headroom limb is non-zero.
    let mut seed_out = [0u64; 3];
    crate::algo_x_support::seed::sqrt_seed(&n, bits, &mut seed_out);
    let mut x: u128 = if seed_out[2] != 0 {
        u128::MAX
    } else {
        (seed_out[0] as u128) | ((seed_out[1] as u128) << 64)
    };
    // Guard: an over-estimate must be >= 1 (the library forces a non-zero seed,
    // but keep the Newton pre-loop invariant explicit).
    if x == 0 {
        x = 1;
    }
    // Over-estimate guarantee (the project's "debug_assert caps" defense): the
    // downward-monotone Newton loop needs `x >= floor(sqrt(n))`. The library
    // seed satisfies the stronger `x² >= n` (a strict over-estimate) -- assert
    // that for every non-saturated seed so a future seed regression PANICS with
    // a location in the gates. The exception is the saturation cap: when the
    // over-estimate touches `2^128` we cap `x = u128::MAX = 2^128-1`, and for
    // `n` just above `(2^128-1)²` (reachable on `N >= 3`, e.g. one operand near
    // `2^128` and a small other) the true floor IS `2^128-1`, so `x` is the
    // CORRECT seed even though `x² < n`. `x == u128::MAX` is trivially
    // `>= floor(sqrt(n))` for any `n < 2^256`, so admit it. `x` is a `u128`
    // (`<= 2^128-1`) so `x²` fits a `u256` exactly -- compare against `n`.
    debug_assert!(
        x == u128::MAX || cmp_u256(sq_u256(x), n) >= 0,
        "isqrt_u256 seed under-estimate: seed={x} n={n:?}"
    );
    // Newton averaging: x <- (x + n/x)/2, monotone-decreasing from the
    // over-estimate. n/x is the fixed u256/u128 scalar division.
    loop {
        let nx = div_u256_by_u128(n, x);
        // (x + n/x)/2 -- both < 2^129 here only transiently; x <= 2^128 and
        // nx <= x, so x + nx <= 2^129. Compute the average without overflow.
        let y = avg_u128(x, nx);
        if y >= x {
            break;
        }
        x = y;
    }
    // Single floor correction: x is floor or floor+1.
    if div_u256_by_u128(n, x) < x {
        x -= 1;
    }
    x
}

/// `(a + b) / 2` for two `u128` without overflowing the intermediate sum
/// (`a + b` can reach `2^129`). Standard average-without-overflow identity.
#[inline]
fn avg_u128(a: u128, b: u128) -> u128 {
    (a & b) + ((a ^ b) >> 1)
}

/// Three-way compare of two `u256` (`[u64; 4]`) values: `-1`/`0`/`1` for
/// `a < b` / `a == b` / `a > b`. Most-significant limb first.
#[inline]
fn cmp_u256(a: [u64; 4], b: [u64; 4]) -> i32 {
    let mut i = 4;
    while i > 0 {
        i -= 1;
        if a[i] != b[i] {
            return if a[i] < b[i] { -1 } else { 1 };
        }
    }
    0
}

/// `round(sqrt(a² + b²))` with the scalar fast paths. `N` is the storage limb
/// count of the `Int<N>` operands. Bit-identical to [`hypot_pythagoras`] for
/// every input and [`RoundingMode`]; returns [`None`] on the same true-overflow
/// condition.
#[inline]
#[must_use]
#[allow(dead_code)]
pub(crate) fn hypot_u128_fast<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode) -> Option<Int<N>>
where
    Limbs<N>: ComputeLimbs,
{
    let ma = a.unsigned_abs();
    let mb = b.unsigned_abs();
    let la = ma.as_limbs();
    let lb = mb.as_limbs();

    // -- u128 fast path: both magnitudes fit one u64 limb -----------------
    // Then each square fits u128 and the sum fits u128 iff no carry past bit
    // 128 (checked). Kept as a distinct branch because the hardware u128
    // divide in `isqrt_u128` is cheaper than the u256/u128 long division.
    if fit_one(la) && fit_one(lb) {
        let av = la[0] as u128;
        let bv = lb[0] as u128;
        let asq = av * av;
        let bsq = bv * bv;
        if let Some(n) = asq.checked_add(bsq) {
            if n == 0 {
                return Some(Int::<N>::ZERO);
            }
            let q = isqrt_u128(n);
            let rem = n - q * q; // exact remainder, n - floor(sqrt n)²
            return finish::<N>(q, rem != 0, rem > q, mode);
        }
        // sum carried past bit 128: fall through to the u256 / wide paths.
    }

    // -- u256 fast path: both magnitudes fit two u64 limbs (< 2^128) -------
    // Each square is < 2^256 and the sum is < 2^257; when it stays < 2^256
    // (no bit-256 carry) the floor root fits a u128 and the whole computation
    // is fixed scalar u256 arithmetic (no multi-precision div_rem). Covers all
    // of D38 and the entire low magnitude band of every wider tier -- the
    // decimal `s >= 19` slow band the `fit_one`-only gate cliffed on.
    if fit_k(la, 2) && fit_k(lb, 2) {
        let av = (la[0] as u128) | ((la[1] as u128) << 64);
        let bv = (lb[0] as u128) | ((lb[1] as u128) << 64);
        let (n, carry) = add_u256(sq_u256(av), sq_u256(bv));
        if !carry {
            // n < 2^256, so q = floor(sqrt(n)) < 2^128.
            if n == [0u64; 4] {
                return Some(Int::<N>::ZERO);
            }
            let q = isqrt_u256(n);
            // exact remainder rem = n - q². q² < 2^256 (q < 2^128), so the
            // subtraction stays in u256; compare against q for the round.
            let qsq = sq_u256(q);
            let (rem, _b) = sub_u256(n, qsq); // rem = n - q² >= 0, < 2·q+1 < 2^129
            // rem < 2^129 but actually <= 2q < 2^129; fit a u128? rem can be up
            // to 2q which may reach 2^129-2 (> u128) when q ~ 2^128. Compare via
            // the 256-bit value directly.
            let rem_nonzero = rem != [0u64; 4];
            let rem_gt_q = cmp_u256_u128(rem, q) > 0;
            return finish::<N>(q, rem_nonzero, rem_gt_q, mode);
        }
        // bit-256 carry (operands near 2^128): fall through to the wide path.
    }

    // Fallback: exact wide kernel.
    hypot_pythagoras::<N>(a, b, mode)
}

/// `a - b` for two `u256` (`[u64; 4]`), returning `(diff, borrow)`. Used for
/// the exact remainder `n - q²` (where `n >= q²`, so `borrow` is `false`).
#[inline]
fn sub_u256(a: [u64; 4], b: [u64; 4]) -> ([u64; 4], bool) {
    let mut out = [0u64; 4];
    let mut borrow = 0i128;
    let mut i = 0;
    while i < 4 {
        let d = a[i] as i128 - b[i] as i128 - borrow;
        if d < 0 {
            out[i] = (d + (1i128 << 64)) as u64;
            borrow = 1;
        } else {
            out[i] = d as u64;
            borrow = 0;
        }
        i += 1;
    }
    (out, borrow != 0)
}

/// Compare a `u256` (`[u64; 4]`) against a `u128` `q`: `-1`/`0`/`1`. The high
/// two limbs of the u256 decide first; only when they are zero does the low
/// 128 bits compare against `q`.
#[inline]
fn cmp_u256_u128(a: [u64; 4], q: u128) -> i32 {
    if a[3] != 0 || a[2] != 0 {
        return 1; // a >= 2^128 > q
    }
    let lo = (a[0] as u128) | ((a[1] as u128) << 64);
    if lo < q {
        -1
    } else if lo > q {
        1
    } else {
        0
    }
}

/// Apply the rounding bump to the floor root `q` and pack it into an `Int<N>`,
/// or [`None`] on true overflow (the rounded root does not fit the signed
/// range of `Int<N>`). Shared by both scalar fast arms; the predicates
/// `diff_nonzero` / `halfway_round_up` are the SAME exact-remainder tests
/// [`hypot_pythagoras`] uses, so the result is bit-identical.
#[inline]
fn finish<const N: usize>(q: u128, diff_nonzero: bool, halfway_round_up: bool, mode: RoundingMode) -> Option<Int<N>> {
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    // q < 2^128 and the bump is at most +1, so the rounded value is <= 2^128.
    // When q == u128::MAX (= 2^128-1) and bump, the sum is exactly 2^128, which
    // does NOT fit a u128 -- use `overflowing_add` so the bit-128 carry lands in
    // `top_overflow` instead of wrapping `result` to 0 (a silent wrong answer).
    let (result, carried) = q.overflowing_add(bump as u128);
    let hi = (result >> 64) as u64;
    let lo = result as u64;
    // Fit check: must be < 2^(64N-1) (signed range). For N == 1 the whole
    // value must be < 2^63; for N == 2 it must be < 2^127; for N >= 3 a
    // 128-bit (here <= 2^128) value always fits, BUT result can reach 2^128
    // exactly (q = 2^128-1, bump) -- that needs bit 128, i.e. a third limb.
    let top_overflow = carried as u128; // bit 128 (1 iff the sum == 2^128)
    match N {
        1 => {
            if hi != 0 || (lo >> 63) != 0 {
                return None;
            }
            let mut out = [0u64; N];
            out[0] = lo;
            Some(Int::<N>::from_limbs(out))
        }
        2 => {
            // signed range of Int<2> is < 2^127.
            if top_overflow != 0 || (hi >> 63) != 0 {
                return None;
            }
            let mut out = [0u64; N];
            out[0] = lo;
            out[1] = hi;
            Some(Int::<N>::from_limbs(out))
        }
        _ => {
            // N >= 3: result <= 2^128 fits the signed range; write low two
            // limbs plus the bit-128 carry into limb 2 if present.
            let mut out = [0u64; N];
            out[0] = lo;
            out[1] = hi;
            if top_overflow != 0 {
                out[2] = 1;
            }
            Some(Int::<N>::from_limbs(out))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{cmp_u256, hypot_u128_fast, isqrt_u256, sq_u256};
    use crate::int::algos::hypot::hypot_pythagoras::hypot_pythagoras;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    fn mix(s: &mut u64) -> u64 {
        *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *s;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// `sq_u256` must equal the true 256-bit square. Reference via a clean
    /// 2-limb schoolbook (x split into 64-bit limbs, full convolution into a
    /// [u64; 4] with explicit u128 carries) -- an independent assembly from the
    /// production column code.
    #[test]
    fn sq_u256_matches_reference() {
        let mut s = 0x1234_5678_9ABC_DEF0u64;
        // Hand checks against externally (Python) computed limbs.
        assert_eq!(sq_u256(0), [0, 0, 0, 0]);
        assert_eq!(sq_u256(1), [1, 0, 0, 0]);
        // (2^64-1)² = 2^128 - 2^65 + 1.
        assert_eq!(sq_u256(u64::MAX as u128), [1, u64::MAX - 1, 0, 0]);
        // (2^128-1)² = 2^256 - 2^129 + 1: [1, 0, 2^64-2, 2^64-1].
        assert_eq!(sq_u256(u128::MAX), [1, 0, u64::MAX - 1, u64::MAX]);
        // The low 128 bits of the exact square must equal the wrapping square,
        // and the round-trip isqrt(sq) == x pins the full value for every x.
        for _ in 0..5000 {
            let x = (mix(&mut s) as u128) | ((mix(&mut s) as u128) << 64);
            let q = sq_u256(x);
            let low = (q[0] as u128) | ((q[1] as u128) << 64);
            assert_eq!(low, x.wrapping_mul(x), "low128 x={x}");
            assert_eq!(isqrt_u256(q), x, "roundtrip x={x}");
        }
    }

    /// `isqrt_u256` must equal the exact floor sqrt for u256 radicands.
    #[test]
    fn isqrt_u256_exact_floor() {
        let mut s = 0xDEAD_F00D_u64;
        for _ in 0..3000 {
            // Build a u128 root r, square it, optionally add a small delta, and
            // confirm isqrt recovers floor(sqrt(r² + delta)) = r for delta in
            // [0, 2r].
            let r = ((mix(&mut s) as u128) | ((mix(&mut s) as u128) << 64)) >> (mix(&mut s) % 64);
            if r == 0 {
                continue;
            }
            let rsq = sq_u256(r);
            // perfect square: floor sqrt == r.
            assert_eq!(isqrt_u256(rsq), r, "perfect square r={r}");
            // r² + delta for delta in [0, 2r] still has floor sqrt == r
            // (since (r+1)² = r² + 2r + 1). Use delta = r (in range, < 2r+1).
            let (rsq_plus, carry) = super::add_u256(rsq, [r as u64, (r >> 64) as u64, 0, 0]);
            if !carry {
                assert_eq!(isqrt_u256(rsq_plus), r, "r²+r, r={r}");
            }
            // r² - 1 (when r >= 1) has floor sqrt == r-1.
            let (rsq_minus, borrow) = super::sub_u256(rsq, [1, 0, 0, 0]);
            if !borrow {
                assert_eq!(isqrt_u256(rsq_minus), r - 1, "r²-1, r={r}");
            }
        }
    }

    /// Reference floor-sqrt of a `u256` (`[u64; 4]`), computed bit-by-bit and
    /// completely independently of the production Newton path: build the root
    /// one bit at a time from the top, accepting a trial bit iff its square
    /// does not exceed `n`. O(256·squaring) but exact -- the test oracle.
    fn isqrt_u256_ref(n: [u64; 4]) -> u128 {
        // n < 2^256 so the floor root is < 2^128.
        let mut root: u128 = 0;
        let mut bit = 127i32;
        while bit >= 0 {
            let trial = root | (1u128 << bit);
            // accept iff trial² <= n.
            if cmp_u256(sq_u256(trial), n) <= 0 {
                root = trial;
            }
            bit -= 1;
        }
        root
    }

    /// TARGETED ADVERSARIAL over-estimate test. Uniform-random inputs cannot
    /// reach the load-bearing failure mode of the seed scaling -- a perfect-
    /// square top window paired with a large shift and maximal low bits (density
    /// ~2^-32). Attack it directly: for shifts across the real u256-arm range
    /// (`bits >= ~129`, so `s` from 64 to 191, BOTH parities) and perfect-square
    /// tops `top = k²` (incl. the boundary `k = 2^31`, `k = 2^32-1`, and random
    /// `k` in `[2^31, 2^32)`), build `n = top·2^s + (2^s - 1)` (maximal low bits)
    /// and assert (a) the library seed is a TRUE over-estimate (`seed² >= n`) and
    /// (b) `isqrt_u256(n)` equals the independent bit-by-bit reference floor.
    #[test]
    fn isqrt_u256_adversarial_perfect_square_tops() {
        // Recompute the production seed exactly as `isqrt_u256` does, so the
        // over-estimate assertion exercises the same library call/buffer.
        fn seed_of(n: [u64; 4], bits: u32) -> u128 {
            let mut out = [0u64; 3];
            crate::algo_x_support::seed::sqrt_seed(&n, bits, &mut out);
            if out[2] != 0 {
                u128::MAX
            } else {
                (out[0] as u128) | ((out[1] as u128) << 64)
            }
        }
        fn bit_len_u256(n: [u64; 4]) -> u32 {
            if n[3] != 0 {
                192 + (64 - n[3].leading_zeros())
            } else if n[2] != 0 {
                128 + (64 - n[2].leading_zeros())
            } else if n[1] != 0 {
                64 + (64 - n[1].leading_zeros())
            } else {
                64 - n[0].leading_zeros()
            }
        }

        let mut s = 0x5EED_A11A_C0DE_u64;
        // perfect-square roots k (so top = k² is an exact square): the two
        // boundary values + a spread of random k in [2^31, 2^32).
        let mut ks: Vec<u64> = vec![1u64 << 31, (1u64 << 32) - 1];
        for _ in 0..32 {
            let k = (mix(&mut s) % (1u64 << 31)) + (1u64 << 31); // [2^31, 2^32)
            ks.push(k);
        }
        // also a few small perfect-square tops near the top-window minimum.
        ks.extend_from_slice(&[(1u64 << 31) + 1, (1u64 << 31) + 7, (1u64 << 32) - 2]);

        for &k in &ks {
            let top = (k as u128) * (k as u128); // top = k², a perfect square < 2^64
            for shift in 64u32..=191 {
                // n = top·2^shift + (2^shift - 1): maximal low bits.
                // top < 2^64, shift <= 191 -> top·2^shift < 2^255, fits u256.
                let mut n = [0u64; 4];
                // place top << shift
                let limb = (shift / 64) as usize;
                let off = shift % 64;
                let lo = (top << off) as u128; // low 128 bits at limb..limb+2
                let hi = if off == 0 { 0u128 } else { top >> (128 - off) };
                let pieces = [lo as u64, (lo >> 64) as u64, hi as u64];
                for (idx, &pc) in pieces.iter().enumerate() {
                    if limb + idx < 4 {
                        n[limb + idx] = pc;
                    }
                }
                // OR in (2^shift - 1): all bits below `shift` set.
                let full_limbs = (shift / 64) as usize;
                for l in n.iter_mut().take(full_limbs) {
                    *l = u64::MAX;
                }
                if off != 0 && full_limbs < 4 {
                    n[full_limbs] |= (1u64 << off) - 1;
                }

                let bits = bit_len_u256(n);
                // (a) the seed must be a genuine over-estimate: seed² >= n.
                let seed = seed_of(n, bits);
                assert!(
                    cmp_u256(sq_u256(seed), n) >= 0,
                    "seed under-estimate: k={k} shift={shift} seed={seed} n={n:?}"
                );
                // (b) isqrt matches the independent bit-by-bit reference floor.
                let got = isqrt_u256(n);
                let want = isqrt_u256_ref(n);
                assert_eq!(got, want, "floor mismatch: k={k} shift={shift} n={n:?}");
            }
        }
    }

    /// SATURATION-ZONE regression: when the library over-estimate touches
    /// `2^128` the seed is capped to `u128::MAX = 2^128-1`, and for `n` just
    /// above `(2^128-1)²` the true floor IS `2^128-1` -- so the capped seed is
    /// correct yet `seed² < n`. The over-estimate `debug_assert` must admit this
    /// (it asserts `x == u128::MAX || x² >= n`); a naive `x² >= n` would PANIC
    /// here in debug even though the result is right. This case is unreachable
    /// from uniform-random inputs, so it gets a direct attack. Asserts no debug
    /// panic and that `isqrt_u256` matches the bit-by-bit reference floor;
    /// cross-checks the full `hypot_u128_fast::<3>` against `hypot_pythagoras`.
    #[test]
    fn isqrt_u256_saturation_zone_floor_2pow128_minus_1() {
        let max = u128::MAX; // 2^128 - 1, the largest u128 floor root
        let maxsq = sq_u256(max); // (2^128-1)²
        // n = (2^128-1)² + d, d small: floor(sqrt(n)) is still 2^128-1 for all
        // d in [0, 2·(2^128-1)] (since (2^128-1)² <= n < 2^256 = (2^128)²).
        for d in [0u128, 1, 2, 3, 7, 1000, max] {
            let (n, carry) = super::add_u256(maxsq, [d as u64, (d >> 64) as u64, 0, 0]);
            assert!(!carry, "n must stay < 2^256 (d={d})");
            // (a) no debug panic on the capped seed; (b) exact floor.
            let got = isqrt_u256(n);
            let want = isqrt_u256_ref(n);
            assert_eq!(want, max, "reference floor must be 2^128-1 (d={d})");
            assert_eq!(got, want, "isqrt_u256 saturation-zone floor (d={d}) n={n:?}");
        }
        // A few more values near 2^256 generally (top bits set, large floor).
        let mut s = 0xCAFE_5A7Eu64;
        for _ in 0..500 {
            // n near 2^256: set the top limb high, fill the rest randomly.
            let n = [mix(&mut s), mix(&mut s), mix(&mut s), mix(&mut s) | (1u64 << 63)];
            assert_eq!(isqrt_u256(n), isqrt_u256_ref(n), "near-2^256 floor n={n:?}");
        }

        // Full-kernel cross-check on the reachable N>=3 saturation input:
        // hypot(2^128-1, 1) -> u256 arm, n = (2^128-1)² + 1, floor = 2^128-1.
        let big = {
            let mut l = [0u64; 3];
            l[0] = u64::MAX;
            l[1] = u64::MAX; // 2^128 - 1
            Int::<3>::from_limbs(l)
        };
        let one = Int::<3>::from_i64(1);
        for mode in ALL_MODES {
            assert_eq!(
                hypot_u128_fast::<3>(big, one, mode),
                hypot_pythagoras::<3>(big, one, mode),
                "hypot saturation-zone mismatch mode={mode:?}"
            );
        }
    }

    /// `hypot_u128_fast` MUST equal `hypot_pythagoras` bit-for-bit on every
    /// input and every rounding mode -- the u128 fast arm, the NEW u256 fast
    /// arm, and the wide fallback.
    fn diff_at<const N: usize>()
    where
        Limbs<N>: crate::int::types::compute_limbs::ComputeLimbs,
    {
        let mut s = 0xDEAD_BEEF_CAFE_F00D_u64 ^ (N as u64);
        for _ in 0..400 {
            let mut la = [0u64; N];
            let mut lb = [0u64; N];
            let shape = mix(&mut s) % 5;
            match shape {
                0 => {
                    // small ~32-bit (u128 fast arm)
                    la[0] = mix(&mut s) & 0xFFFF_FFFF;
                    lb[0] = mix(&mut s) & 0xFFFF_FFFF;
                }
                1 => {
                    // full single-limb (u128 fast arm, carry edge)
                    la[0] = mix(&mut s);
                    lb[0] = mix(&mut s);
                }
                2 if N >= 2 => {
                    // two-limb operands (< 2^128) -- the NEW u256 fast arm.
                    la[0] = mix(&mut s);
                    la[1] = mix(&mut s);
                    lb[0] = mix(&mut s);
                    lb[1] = mix(&mut s);
                }
                3 if N >= 2 => {
                    // two-limb with the TOP bit of limb1 set on one operand
                    // (near 2^128 -- exercises the bit-256-carry fall-through).
                    la[0] = mix(&mut s);
                    la[1] = mix(&mut s) | (1u64 << 63);
                    lb[0] = mix(&mut s);
                    lb[1] = mix(&mut s) | (1u64 << 63);
                }
                _ => {
                    // multi-limb -- exercises the fallback (clear top sign bit)
                    for k in 0..N {
                        la[k] = mix(&mut s);
                        lb[k] = mix(&mut s);
                    }
                    la[N - 1] &= i64::MAX as u64;
                    lb[N - 1] &= i64::MAX as u64;
                }
            }
            let a = Int::<N>::from_limbs(la);
            let b = Int::<N>::from_limbs(lb);
            for mode in ALL_MODES {
                assert_eq!(
                    hypot_u128_fast::<N>(a, b, mode),
                    hypot_pythagoras::<N>(a, b, mode),
                    "N={N} mode={mode:?} a={:?} b={:?}",
                    a.as_limbs(),
                    b.as_limbs()
                );
            }
        }
        // Explicit pythagorean triples and edges.
        let checks: [(i64, i64); 6] = [(3, 4), (5, 12), (8, 15), (1, 1), (0, 0), (0, 42)];
        for (av, bv) in checks {
            let a = Int::<N>::from_i64(av);
            let b = Int::<N>::from_i64(bv);
            for mode in ALL_MODES {
                assert_eq!(
                    hypot_u128_fast::<N>(a, b, mode),
                    hypot_pythagoras::<N>(a, b, mode),
                    "explicit N={N} mode={mode:?} a={av} b={bv}"
                );
            }
        }
    }

    // Int<4> radicand (a²+b² → isqrt of ~8 limbs) exceeds the narrow build's
    // int-kernel scratch; runs where the wide tiers are real.
    #[cfg(feature = "_wide-support")]
    #[test]
    fn hypot_u128_fast_matches_pythagoras() {
        diff_at::<1>();
        diff_at::<2>();
        diff_at::<3>();
        diff_at::<4>();
    }

    /// Two-limb operand A/B at the narrow default build (`Int<2>`): the new
    /// u256 fast arm must match Pythagoras for D38-range operands.
    #[test]
    fn hypot_u128_fast_two_limb_matches_pythagoras_n2() {
        diff_at::<2>();
    }

    /// Perfect-square radicand: remainder is exactly zero, no mode bumps.
    #[test]
    fn hypot_u128_fast_perfect_square() {
        let a = Int::<2>::from_i64(3);
        let b = Int::<2>::from_i64(4);
        for mode in ALL_MODES {
            assert_eq!(hypot_u128_fast::<2>(a, b, mode).unwrap().as_i128(), 5);
        }
    }
}
