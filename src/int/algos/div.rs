//! Width-matched limb division primitives.
//!
//! Re-exported from [`crate::int::limbs`]; see [`super`] for the
//! re-export rationale. The divisor-dispatch / Knuth-D / Burnikel-
//! Ziegler stack stays where it is — the integer layer routes through
//! [`limbs_divmod_u64`] (single-limb fast paths) and the dispatcher
//! for the multi-limb case.

#[allow(unused_imports)]
pub(crate) use crate::int::limbs::{
    limbs_divmod_dispatch_u64, limbs_divmod_u64, limbs_isqrt_u64,
};

/// Const-`N` fast-arm divmod over little-endian u64 magnitude limbs.
///
/// `num`, `den`, `quot`, `rem` are all `N`-limb magnitudes (sign handling
/// is the caller's; this is an unsigned division of the magnitudes). The
/// quotient and remainder are written into `quot` / `rem`.
///
/// Because `N` is a compile-time constant, the `if N == …` ladder
/// const-folds per monomorphisation:
///
/// * `N == 1` lowers to a single native `u64` `/` + `%` (the hardware
///   `idiv`), with no slice bookkeeping or limb-loop overhead.
/// * `N == 2` widens to native `u128` `/` + `%`, letting the compiler
///   emit its 128-bit divide (`__udivti3` / `__divti3` on the signed
///   side) directly instead of routing through the generic limb path.
/// * `N >= 3` falls through to the shared, width-agnostic
///   [`limbs_divmod_dispatch_u64`] (Knuth-D / Burnikel–Ziegler).
///
/// All three arms are behaviour-identical: truncating (Euclidean on
/// non-negative magnitudes) division, `quot = num / den`,
/// `rem = num - quot·den`. The divisor must be non-zero (the caller
/// guards this before delegating).
#[inline]
pub(crate) fn div_rem_mag_fixed<const N: usize>(
    num: &[u64; N],
    den: &[u64; N],
    quot: &mut [u64; N],
    rem: &mut [u64; N],
) {
    if N == 1 {
        // Native u64 path → hardware idiv.
        let n0 = num[0];
        let d0 = den[0];
        quot[0] = n0 / d0;
        rem[0] = n0 % d0;
    } else if N == 2 {
        // Native u128 path → compiler-emitted 128-bit divide.
        let n = (num[0] as u128) | ((num[1] as u128) << 64);
        let d = (den[0] as u128) | ((den[1] as u128) << 64);
        let q = n / d;
        let r = n % d;
        quot[0] = q as u64;
        quot[1] = (q >> 64) as u64;
        rem[0] = r as u64;
        rem[1] = (r >> 64) as u64;
    } else {
        limbs_divmod_dispatch_u64(num, den, quot, rem);
    }
}

/// Const-`N` fast-arm integer square root over little-endian u64
/// magnitude limbs. Writes `floor(sqrt(n))` into `out`.
///
/// Mirrors [`div_rem_mag_fixed`]: `N == 1` uses the native `u64::isqrt`,
/// `N == 2` uses `u128::isqrt`, and `N >= 3` falls through to the shared
/// [`limbs_isqrt_u64`] (Newton with a hardware-`f64::sqrt` seed). All
/// arms return the identical floor square root.
#[inline]
pub(crate) fn isqrt_mag_fixed<const N: usize>(n: &[u64; N], out: &mut [u64; N]) {
    if N == 1 {
        out[0] = n[0].isqrt();
    } else if N == 2 {
        let v = (n[0] as u128) | ((n[1] as u128) << 64);
        let r = v.isqrt();
        out[0] = r as u64;
        out[1] = (r >> 64) as u64;
    } else {
        limbs_isqrt_u64(n, out);
    }
}

#[cfg(test)]
mod fast_arm_tests {
    use super::{div_rem_mag_fixed, isqrt_mag_fixed};
    use crate::int::limbs::{limbs_divmod_dispatch_u64, limbs_isqrt_u64};

    /// The `N == 1` and `N == 2` native fast arms must agree limb-for-limb
    /// with the generic dispatch path over the divmod edge cases:
    /// divide-by-one, equal operands, max/zero numerator, and assorted
    /// mid-range values.
    #[test]
    fn fast_arm_div_rem_matches_generic() {
        // N == 1: every interesting (num, den) over u64.
        let vals1: [u64; 8] = [
            0,
            1,
            2,
            7,
            u64::MAX,
            u64::MAX - 1,
            0x8000_0000_0000_0000,
            123_456_789,
        ];
        for &num in &vals1 {
            for &den in &vals1 {
                if den == 0 {
                    continue;
                }
                let mut fq = [0u64; 1];
                let mut fr = [0u64; 1];
                div_rem_mag_fixed::<1>(&[num], &[den], &mut fq, &mut fr);
                let mut gq = [0u64; 1];
                let mut gr = [0u64; 1];
                limbs_divmod_dispatch_u64(&[num], &[den], &mut gq, &mut gr);
                assert_eq!(fq, gq, "N=1 quot mismatch {num}/{den}");
                assert_eq!(fr, gr, "N=1 rem mismatch {num}%{den}");
                // And against the host u64 directly.
                assert_eq!(fq[0], num / den);
                assert_eq!(fr[0], num % den);
            }
        }

        // N == 2: u128 edge cases.
        let vals2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            u128::MAX - 1,
            1u128 << 127,
            (1u128 << 64) - 1,
            1u128 << 64,
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        let to_limbs = |v: u128| [v as u64, (v >> 64) as u64];
        for &num in &vals2 {
            for &den in &vals2 {
                if den == 0 {
                    continue;
                }
                let n = to_limbs(num);
                let d = to_limbs(den);
                let mut fq = [0u64; 2];
                let mut fr = [0u64; 2];
                div_rem_mag_fixed::<2>(&n, &d, &mut fq, &mut fr);
                let mut gq = [0u64; 2];
                let mut gr = [0u64; 2];
                limbs_divmod_dispatch_u64(&n, &d, &mut gq, &mut gr);
                assert_eq!(fq, gq, "N=2 quot mismatch {num}/{den}");
                assert_eq!(fr, gr, "N=2 rem mismatch {num}%{den}");
                assert_eq!(fq, to_limbs(num / den));
                assert_eq!(fr, to_limbs(num % den));
            }
        }
    }

    /// The native isqrt fast arms must match the generic limb isqrt
    /// (and the host integer `isqrt`) across boundary values.
    #[test]
    fn fast_arm_isqrt_matches_generic() {
        let vals1: [u64; 9] = [
            0,
            1,
            2,
            3,
            4,
            15,
            16,
            u64::MAX,
            (u32::MAX as u64) * (u32::MAX as u64),
        ];
        for &v in &vals1 {
            let mut f = [0u64; 1];
            isqrt_mag_fixed::<1>(&[v], &mut f);
            let mut g = [0u64; 1];
            limbs_isqrt_u64(&[v], &mut g);
            assert_eq!(f, g, "N=1 isqrt mismatch sqrt({v})");
            assert_eq!(f[0], v.isqrt());
        }

        let vals2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            (1u128 << 64) - 1,
            1u128 << 64,
            1u128 << 126,
            (u64::MAX as u128) * (u64::MAX as u128),
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        for &v in &vals2 {
            let n = [v as u64, (v >> 64) as u64];
            let mut f = [0u64; 2];
            isqrt_mag_fixed::<2>(&n, &mut f);
            let mut g = [0u64; 2];
            limbs_isqrt_u64(&n, &mut g);
            assert_eq!(f, g, "N=2 isqrt mismatch sqrt({v})");
            let r = v.isqrt();
            assert_eq!(f, [r as u64, (r >> 64) as u64]);
        }
    }
}
