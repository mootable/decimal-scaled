//! `sqrt_newton` — Newton integer square root of `raw · 10^SCALE`, with a
//! single round step, computed directly over `u64` limbs.
//!
//! For a `D<Int<N>, SCALE>` value with raw storage `r`, the logical value
//! is `r / 10^SCALE`, so the square-root raw storage is
//! `round(sqrt(r · 10^SCALE))`. The radicand `|r| · 10^SCALE` is formed in
//! a local limb scratch buffer (it spans up to `2N` limbs, which is why a
//! wider *type* used to be threaded in), the exact integer square root is
//! taken via the int layer's width-agnostic slice kernel
//! ([`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`]), and a single
//! round-to-nearest step lands the result on the type's last representable
//! place. Within 0.5 ULP under any of the six rounding modes.
//!
//! # Generic over the storage width only
//!
//! The kernel is generic over the storage limb count `N` and does the
//! work-width arithmetic in limbs — there is **no** `W = Int<2N>` work
//! *type* (which stable Rust cannot name from `N`), and therefore no
//! per-tier work-width binding in the policy. The integer work dispatches
//! *down* to the int layer's slice kernels: `isqrt_newton` for the root and
//! [`crate::int::algos::mul::mul_schoolbook::mul_schoolbook`] for the
//! products, with the `limbs` primitives for the rest.
//!
//! Returns `Int::<N>::ZERO` for `raw <= 0` (saturate-not-panic).

use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
use crate::int::algos::support::limbs::{cmp_cross, is_zero, sub_assign};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Limb scratch budget — matches the int root kernels' `SCRATCH_LIMBS`
/// (288 u64 = 18432 bits), covering the widest radicand (`2 · Int<64>`).
const SCRATCH: usize = 288;

/// Significant limb length of `a` (index of the highest non-zero limb + 1),
/// clamped to at least 1 so zero has length 1.
#[inline]
fn sig_len(a: &[u64]) -> usize {
    let mut l = a.len();
    while l > 1 && a[l - 1] == 0 {
        l -= 1;
    }
    l
}

/// Newton integer square-root kernel, computed in limbs.
///
/// `N` is the storage limb count backing `D<Int<N>, SCALE>`. The radicand
/// `|raw| · 10^scale` is built in scratch and rooted via the int layer's
/// `isqrt_newton`; the result is rounded and returned as `Int<N>`.
#[inline]
#[must_use]
pub(crate) fn sqrt_newton<const N: usize>(raw: Int<N>, scale: u32, mode: RoundingMode) -> Int<N> {
    if raw <= Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }

    // ── radicand n = |raw| · 10^scale, in limb scratch ──────────────────
    let mut n = [0u64; SCRATCH];
    n[..N].copy_from_slice(raw.unsigned_abs().as_limbs());
    let mut nl = sig_len(&n[..N]);
    {
        let mut tmp = [0u64; SCRATCH];
        for _ in 0..scale {
            let out = nl + 1;
            for t in tmp[..out].iter_mut() {
                *t = 0;
            }
            mul_schoolbook(&n[..nl], &[10u64], &mut tmp[..out]);
            n[..out].copy_from_slice(&tmp[..out]);
            nl = sig_len(&n[..out]);
        }
    }

    // ── q = floor(sqrt(n)) via the int slice kernel ─────────────────────
    let mut q = [0u64; SCRATCH];
    isqrt_newton(&n[..nl], &mut q[..nl]);
    let ql = sig_len(&q[..nl]);

    // ── diff = n - q²  (q² ≤ n, so diff fits in nl limbs) ───────────────
    let mut qsq = [0u64; SCRATCH];
    mul_schoolbook(&q[..ql], &q[..ql], &mut qsq[..(2 * ql).min(SCRATCH)]);
    let mut diff = [0u64; SCRATCH];
    diff[..nl].copy_from_slice(&n[..nl]);
    sub_assign(&mut diff[..nl], &qsq[..nl]);

    // ── single round step (matches the BigInt-generic kernel exactly) ───
    // halfway_round_up: remainder past the lower root exceeds the root
    // (diff > q); diff_nonzero: any remainder at all.
    let halfway_round_up = cmp_cross(&diff[..nl], &q[..ql]) > 0;
    let diff_nonzero = !is_zero(&diff[..nl]);
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    if bump {
        // q += 1 (carry stays within ql+1 limbs).
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

    // ── narrow the root to Int<N> (positive; fits by construction) ──────
    let mut out = [0u64; N];
    out.copy_from_slice(&q[..N]);
    Int::<N>::from_limbs(out)
}
