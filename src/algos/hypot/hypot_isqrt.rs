//! `hypot_isqrt` — `sqrt(a² + b²)` via the int-layer slice `isqrt`.
//!
//! For two `D<Int<N>, SCALE>` values with raw storages `a` and `b`, the
//! hypotenuse raw storage is `round(sqrt(a² + b²))` — both operands carry
//! the same `10^SCALE` factor, so it divides out of the root and no rescale
//! is needed (contrast [`crate::algos::sqrt`], which forms `raw · 10^SCALE`).
//! The radicand `a² + b²` spans up to `2N` limbs; it is formed in a limb
//! scratch buffer, rooted via the int layer's width-agnostic slice kernel
//! ([`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`] — the same root
//! [`crate::algos::sqrt::sqrt_newton`] uses), and a single round step lands
//! the result on the last place. This routes the root **down** through the
//! integer layer instead of calling the decimal `sqrt` surface on the tier's
//! own value (the old inversion).
//!
//! # Generic over the storage width only
//!
//! Like [`crate::algos::sqrt::sqrt_newton`], the work-width arithmetic is
//! done in limbs — no `W = Int<2N>` work type. Returns [`None`] when the
//! rounded root does not fit `S` (true overflow); the caller maps that to
//! the out-of-range panic.
//!
//! Semantics preserved: `hypot(0, 0) = 0`; `hypot(0, x) = |x|`.

use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{add_assign, cmp_cross, is_zero, sub_assign};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

const SCRATCH: usize = 288;

#[inline]
fn sig_len(a: &[u64]) -> usize {
    let mut l = a.len();
    while l > 1 && a[l - 1] == 0 {
        l -= 1;
    }
    l
}

/// `sqrt(a² + b²)` via the int slice `isqrt`. `N` is the storage limb count
/// backing `D<Int<N>, SCALE>`. Returns [`None`] on true overflow (the rounded
/// root does not fit `Int<N>`).
#[inline]
#[must_use]
pub(crate) fn hypot_isqrt<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode) -> Option<Int<N>> {
    // ── n = a² + b² (magnitudes; sign drops out of squaring) ────────────
    let ma = a.unsigned_abs();
    let mb = b.unsigned_abs();
    let la = sig_len(ma.as_limbs());
    let lb = sig_len(mb.as_limbs());
    let mut n = [0u64; SCRATCH];
    mul_schoolbook(&ma.as_limbs()[..la], &ma.as_limbs()[..la], &mut n[..2 * la]);
    let mut bsq = [0u64; SCRATCH];
    mul_schoolbook(&mb.as_limbs()[..lb], &mb.as_limbs()[..lb], &mut bsq[..2 * lb]);
    let span = (2 * la).max(2 * lb) + 1;
    add_assign(&mut n[..span], &bsq[..2 * lb]);
    let nl = sig_len(&n[..span]);
    if nl == 1 && n[0] == 0 {
        return Some(Int::<N>::ZERO);
    }

    // ── q = floor(sqrt(n)) ──────────────────────────────────────────────
    let mut q = [0u64; SCRATCH];
    isqrt_newton(&n[..nl], &mut q[..nl]);
    let ql = sig_len(&q[..nl]);

    // ── diff = n - q²  (reuse `n` in place as the remainder) ────────────
    let mut qsq = [0u64; SCRATCH];
    mul_schoolbook(&q[..ql], &q[..ql], &mut qsq[..(2 * ql).min(SCRATCH)]);
    sub_assign(&mut n[..nl], &qsq[..nl]);
    let halfway_round_up = cmp_cross(&n[..nl], &q[..ql]) > 0;
    let diff_nonzero = !is_zero(&n[..nl]);
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    if bump {
        let mut i = 0;
        loop {
            let (v, c) = q[i].overflowing_add(1);
            q[i] = v;
            if !c {
                break;
            }
            i += 1;
        }
    }

    // ── fit check: positive magnitude must be < 2^(64N-1) (signed range) ─
    let qfl = sig_len(&q[..(N + 2).min(SCRATCH)]);
    if qfl > N || (qfl == N && (q[N - 1] >> 63) != 0) {
        return None;
    }
    let mut out = [0u64; N];
    out.copy_from_slice(&q[..N]);
    Some(Int::<N>::from_limbs(out))
}

#[cfg(test)]
mod tests {
    use super::hypot_isqrt;
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

    #[test]
    fn hypot_isqrt_pythagorean_3_4_5_all_modes() {
        let a = Int::<2>::from_i64(3);
        let b = Int::<2>::from_i64(4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_isqrt_pythagorean_5_12_13_all_modes() {
        let a = Int::<2>::from_i64(5);
        let b = Int::<2>::from_i64(12);
        let expected = Int::<2>::from_i64(13);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_isqrt_non_perfect_1_1() {
        let a = Int::<2>::from_i64(1);
        let b = Int::<2>::from_i64(1);
        assert_eq!(hypot_isqrt::<2>(a, b, RoundingMode::Trunc).unwrap().as_i128(), 1);
        assert_eq!(hypot_isqrt::<2>(a, b, RoundingMode::Ceiling).unwrap().as_i128(), 2);
        assert_eq!(hypot_isqrt::<2>(a, b, RoundingMode::HalfToEven).unwrap().as_i128(), 1);
    }

    #[test]
    fn hypot_isqrt_zero_zero() {
        let z = Int::<2>::from_i64(0);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(z, z, mode), Some(z), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_isqrt_zero_x_equals_abs_x() {
        let z = Int::<2>::from_i64(0);
        let x = Int::<2>::from_i64(42);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(z, x, mode), Some(x), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_isqrt_negative_inputs() {
        let a = Int::<2>::from_i64(-3);
        let b = Int::<2>::from_i64(-4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }
}
