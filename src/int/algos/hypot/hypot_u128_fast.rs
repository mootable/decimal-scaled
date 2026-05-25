// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `hypot_u128_fast` -- integer hypotenuse `round(sqrt(a² + b²))` with a
//! native-`u128` fast path for the common small-operand case, falling back to
//! the exact wide [`hypot_pythagoras`] kernel otherwise.
//!
//! Background. [`hypot_pythagoras`] forms the exact radicand `n = a² + b²`,
//! takes `q = floor(sqrt(n))` via the width-agnostic Newton slice `isqrt`
//! (which runs the runtime `div_rem` dispatcher once per iteration), then
//! rounds using the exact remainder `n - q²` against `q`. The Newton divide
//! loop dominates the cost. But when both operands fit ~63 bits — the
//! overwhelmingly common D38 (`Int<2>`) input — the radicand `n = a² + b²`
//! fits a single `u128`, and `floor(sqrt(n))` plus the exact remainder can be
//! had with a hardware-`f64` seed and one Newton refinement entirely in
//! `u128`, with NO multi-precision divide. The round step is the SAME exact
//! remainder test (`n - q² > q` for the half modes, `n - q² != 0` for
//! ceiling), so the delivered value is bit-identical to [`hypot_pythagoras`]
//! for every input and every [`RoundingMode`].
//!
//! Correctly rounded. `q = isqrt_u128(n)` is the exact floor root of the
//! exact integer radicand; the exact midpoint between `q` and `q+1` is
//! `q + 1/2`, whose square `q² + q + 1/4` is never an integer, so an integer
//! `n` can never land exactly on a half-way tie. Hence `n - q² > q`
//! (equivalently `n >= q² + q + 1`) is the exact "above half-way" predicate
//! and the three half-* modes coincide — matching [`hypot_pythagoras`].
//!
//! Cite: the floor-root-plus-exact-remainder rounding structure is the
//! integer specialisation of the correctly-rounded `sqrt(x²+y²)` device in
//! C. F. Borges, "An Improved Algorithm for hypot(a,b)", arXiv:1904.09481
//! (2019) — there in floating point with an FMA remainder; here the
//! remainder is exact because the radicand is an exact integer.

use crate::int::algos::hypot::hypot_pythagoras::hypot_pythagoras;
use crate::int::algos::support::limbs::fit_one;
use crate::int::types::work_scratch::ComputeInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// `floor(sqrt(n))` for a `u128`, exact. Seeds from the hardware `f64` sqrt
/// (when `std`) or a 1-bit bracket (`no_std`), then refines with the integer
/// Newton averaging step and a final ±1 floor correction so the result is the
/// exact floor regardless of seed rounding.
#[inline]
fn isqrt_u128(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    // Seed.
    #[cfg(feature = "std")]
    let mut x: u128 = {
        // f64 sqrt of n is within a relative 2^-52 of the true root; for a
        // 128-bit n that is a handful of ULPs, well inside the Newton basin.
        let approx = (n as f64).sqrt();
        // Clamp to the representable u128 range; cast truncates toward zero.
        if approx.is_finite() && approx >= 1.0 {
            approx as u128
        } else {
            1
        }
    };
    #[cfg(not(feature = "std"))]
    let mut x: u128 = {
        // Classical pure-integer seed `2^ceil(bits/2)` — a safe over-estimate.
        let bits = 128 - n.leading_zeros();
        1u128 << ((bits + 1) / 2)
    };
    if x == 0 {
        x = 1;
    }
    // Newton averaging: x ← (x + n/x)/2, monotone-decreasing once x ≥ root.
    // Converges to floor or floor+1 from any over-estimate; the loop stops
    // when it stops decreasing.
    loop {
        let y = (x + n / x) >> 1;
        if y >= x {
            break;
        }
        x = y;
    }
    // Floor correction: the seed may have been an under-estimate (f64 round
    // down on a near-perfect square), so walk x to the exact floor.
    while x > 0 && x > n / x {
        x -= 1;
    }
    while (x + 1) <= n / (x + 1) {
        // (x+1)² ≤ n  ⟺  x+1 ≤ n/(x+1) when (x+1) divides cleanly; guard
        // against overflow by comparing via division rather than squaring.
        x += 1;
    }
    x
}

/// `round(sqrt(a² + b²))` with the `u128` fast path. `N` is the storage limb
/// count of the `Int<N>` operands. Bit-identical to
/// [`hypot_pythagoras`] for every input and [`RoundingMode`]; returns
/// [`None`] on the same true-overflow condition.
#[inline]
#[must_use]
#[allow(dead_code)]
pub(crate) fn hypot_u128_fast<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode) -> Option<Int<N>>
where
    Int<N>: ComputeInt,
{
    let ma = a.unsigned_abs();
    let mb = b.unsigned_abs();
    let la = ma.as_limbs();
    let lb = mb.as_limbs();

    // Fast path only when each magnitude fits a single u64 limb; then each
    // square fits u128 and the sum fits u128 iff it does not overflow bit 128.
    if fit_one(la) && fit_one(lb) {
        let av = la[0] as u128;
        let bv = lb[0] as u128;
        // a² and b² each ≤ (2^64-1)² < 2^128; their sum can carry into bit
        // 128. Detect that and fall through to the exact wide path.
        let asq = av * av;
        let bsq = bv * bv;
        if let Some(n) = asq.checked_add(bsq) {
            if n == 0 {
                return Some(Int::<N>::ZERO);
            }
            let q = isqrt_u128(n);
            let rem = n - q * q; // exact remainder, n - floor(sqrt n)²
            let halfway_round_up = rem > q;
            let diff_nonzero = rem != 0;
            let bump = match mode {
                RoundingMode::HalfToEven
                | RoundingMode::HalfAwayFromZero
                | RoundingMode::HalfTowardZero => halfway_round_up,
                RoundingMode::Trunc | RoundingMode::Floor => false,
                RoundingMode::Ceiling => diff_nonzero,
            };
            let result = q + (bump as u128);
            // Fit check: must be < 2^(64N-1). For N ≥ 2 a 128-bit result
            // always fits the signed range (2^127-1 ≤ MAX); for N == 1 it must
            // be < 2^63. Guard the N == 1 (and any N where 128 bits is the
            // whole width) case.
            let hi = (result >> 64) as u64;
            let lo = result as u64;
            if N == 1 {
                if hi != 0 || (lo >> 63) != 0 {
                    return None;
                }
                let mut out = [0u64; N];
                out[0] = lo;
                return Some(Int::<N>::from_limbs(out));
            }
            // N ≥ 2: write the low two limbs, zero the rest. 2^127-1 fits.
            let mut out = [0u64; N];
            out[0] = lo;
            if N >= 2 {
                out[1] = hi;
            }
            return Some(Int::<N>::from_limbs(out));
        }
    }

    // Fallback: exact wide kernel.
    hypot_pythagoras::<N>(a, b, mode)
}

#[cfg(test)]
mod tests {
    use super::hypot_u128_fast;
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

    /// `hypot_u128_fast` MUST equal `hypot_pythagoras` bit-for-bit on every
    /// input and every rounding mode — both the fast and fallback arms.
    fn diff_at<const N: usize>()
    where
        Int<N>: crate::int::types::work_scratch::ComputeInt,
    {
        let mut s = 0xDEAD_BEEF_CAFE_F00du64 ^ (N as u64);
        for _ in 0..300 {
            // Build operands at a mix of magnitudes: small (fast-path), full
            // single-limb (fast-path edge), and multi-limb (fallback).
            let mut la = [0u64; N];
            let mut lb = [0u64; N];
            let shape = mix(&mut s) % 3;
            match shape {
                0 => {
                    // small ~32-bit
                    la[0] = mix(&mut s) & 0xFFFF_FFFF;
                    lb[0] = mix(&mut s) & 0xFFFF_FFFF;
                }
                1 => {
                    // full single-limb (up to 64 bits) — fast-path carry edge
                    la[0] = mix(&mut s);
                    lb[0] = mix(&mut s);
                }
                _ => {
                    // multi-limb — exercises the fallback (clear top sign bit)
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

    #[test]
    fn hypot_u128_fast_matches_pythagoras() {
        diff_at::<1>();
        diff_at::<2>();
        diff_at::<3>();
        diff_at::<4>();
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
