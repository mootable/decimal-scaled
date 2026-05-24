//! `mul_schoolbook` -- naive schoolbook decimal multiplication reference,
//! generic over the storage width `N` only.
//!
//! Computes `a * b` for two same-`SCALE` decimals stored as `Int<N>`.
//! The logical product is `(a / 10^SCALE) * (b / 10^SCALE)`, whose raw
//! storage value is `a * b / 10^SCALE`.
//!
//! This is the naive reference algorithm — no leading-zero fast path:
//!
//! 1. Form the full magnitude product `|a| * |b|` (`2N` u64 limbs) in a
//!    [`WorkScratch::work2`] buffer via the int layer's slice
//!    [`crate::int::algos::mul::mul_schoolbook::mul_schoolbook`].
//! 2. Build `10^SCALE` in the same limb domain and divide the product by
//!    it using the int layer's width-agnostic divide
//!    ([`crate::int::algos::div::div_fixed::div_rem_mag_slice`]),
//!    rounding under `mode`.
//! 3. Rebuild the signed `Int<N>` quotient.
//!
//! Unlike [`mul_widen_divide`](super::mul_widen_divide::mul_widen_divide),
//! this kernel has no leading-zero fast path and does not use the
//! MG-divide or Newton-reciprocal acceleration: it divides via the plain
//! int-layer `div_rem`, making it the unambiguous schoolbook reference and
//! a real benchmarkable seam.
//!
//! All integer arithmetic dispatches DOWN to the int layer; this fn never
//! calls a decimal method on its own value.

use crate::int::algos::div::div_fixed::div_rem_mag_slice;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook as mul_slice;
use crate::int::types::work_scratch::WorkScratch;
use crate::int::types::Int;
use crate::support::rounding::{should_bump, RoundingMode};

/// Significant limb length (highest non-zero limb index + 1, min 1).
#[inline]
fn sig_len(a: &[u64]) -> usize {
    let mut l = a.len();
    while l > 1 && a[l - 1] == 0 {
        l -= 1;
    }
    l
}

/// Naive schoolbook decimal multiplication, generic over `N`. Requires
/// `Int<N>: WorkScratch` for the `2N`-limb product scratch.
///
/// Forms the full magnitude product in the scratch buffer, then divides by
/// `10^SCALE` using the plain int-layer `div_rem`, rounding under `mode`.
/// No MG-divide, no Newton-reciprocal, no leading-zero fast path.
/// `SCALE == 0` returns the narrowed product unscaled.
#[inline]
pub(crate) fn mul_schoolbook<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Int<N>: WorkScratch,
{
    let neg = a.is_negative() != b.is_negative();
    let a_mag = *a.unsigned_abs().as_limbs();
    let b_mag = *b.unsigned_abs().as_limbs();
    let al = sig_len(&a_mag);
    let bl = sig_len(&b_mag);

    // Full magnitude product in the work scratch (2N u64 limbs).
    let mut prod_buf = Int::<N>::work2();
    let prod = prod_buf.as_mut();
    let plen = (al + bl).min(prod.len());
    for slot in prod[..plen].iter_mut() {
        *slot = 0;
    }
    mul_slice(&a_mag[..al], &b_mag[..bl], &mut prod[..plen]);

    if SCALE == 0 {
        let mut out = [0u64; N];
        out.copy_from_slice(&prod[..N]);
        return apply_sign::<N>(out, neg, "attempt to multiply with overflow");
    }

    // Build 10^SCALE in a u64 limb buffer (iterative *10).
    let mut div_buf = Int::<N>::work2();
    let divisor = div_buf.as_mut();
    divisor[0] = 1;
    let mut dl = 1usize;
    for _ in 0..SCALE {
        let mut carry: u64 = 0;
        for limb in divisor[..dl].iter_mut() {
            let p = (*limb as u128) * 10u128 + carry as u128;
            *limb = p as u64;
            carry = (p >> 64) as u64;
        }
        if carry != 0 {
            divisor[dl] = carry;
            dl += 1;
        }
    }

    // q = prod / divisor, r = prod % divisor (magnitudes, via int layer).
    let ptop = sig_len(&prod[..plen]);
    let mut quot_buf = Int::<N>::work2();
    let quot = quot_buf.as_mut();
    let mut rem_buf = Int::<N>::work2();
    let rem = rem_buf.as_mut();
    for slot in quot[..ptop].iter_mut() {
        *slot = 0;
    }
    for slot in rem[..dl].iter_mut() {
        *slot = 0;
    }
    div_rem_mag_slice(&prod[..ptop], &divisor[..dl], &mut quot[..ptop], &mut rem[..dl]);

    // Round: compare remainder against divisor - remainder.
    let rl = sig_len(&rem[..dl]);
    let rem_nonzero = !(rl == 1 && rem[0] == 0);
    if rem_nonzero {
        // cmp_r = rem.cmp(divisor - rem), via comparing 2*rem to divisor.
        let cmp_r = cmp_double_vs(&rem[..dl], &divisor[..dl]);
        let q_is_odd = (quot[0] & 1) != 0;
        if should_bump(mode, cmp_r, q_is_odd, !neg) {
            // quot += 1
            let mut carry: u64 = 1;
            for limb in quot.iter_mut() {
                let (s, c) = limb.overflowing_add(carry);
                *limb = s;
                if !c {
                    carry = 0;
                    break;
                }
            }
            let _ = carry;
        }
    }

    let mut out = [0u64; N];
    out.copy_from_slice(&quot[..N]);
    apply_sign::<N>(out, neg, "attempt to multiply with overflow")
}

/// Compare `2*rem` against `divisor` (both little-endian magnitudes),
/// returning the ordering of `rem` vs `divisor - rem`.
#[inline]
fn cmp_double_vs(rem: &[u64], divisor: &[u64]) -> core::cmp::Ordering {
    // Build 2*rem in a local buffer wide enough (divisor.len()+1).
    let mut two_r = [0u64; 80];
    let mut carry: u64 = 0;
    for (i, &r) in rem.iter().enumerate() {
        let v = (r as u128) << 1 | carry as u128;
        two_r[i] = v as u64;
        carry = (v >> 64) as u64;
    }
    let mut len = rem.len();
    if carry != 0 {
        two_r[len] = carry;
        len += 1;
    }
    // Compare two_r[..len] vs divisor (little-endian).
    let dl = divisor.len();
    let maxl = len.max(dl);
    let mut k = maxl;
    while k > 0 {
        k -= 1;
        let lhs = if k < len { two_r[k] } else { 0 };
        let rhs = if k < dl { divisor[k] } else { 0 };
        if lhs != rhs {
            return if lhs > rhs {
                core::cmp::Ordering::Greater
            } else {
                core::cmp::Ordering::Less
            };
        }
    }
    core::cmp::Ordering::Equal
}

/// Rebuild a signed `Int<N>` from magnitude limbs `out` and sign `neg`,
/// debug-panicking on overflow (matching the old `narrow_or_panic!`).
#[inline]
fn apply_sign<const N: usize>(out: [u64; N], neg: bool, msg: &str) -> Int<N> {
    let mag = Int::<N>::from_limbs(out);
    if cfg!(debug_assertions) {
        // `from_limbs` reinterprets bits as two's complement; if the top
        // bit is set the magnitude exceeds the signed range.
        if mag.is_negative() {
            // The sole representable case is exactly Int<N>::MIN with neg.
            if !(neg && mag == Int::<N>::MIN) {
                panic!("{msg}");
            }
        }
    }
    if neg {
        mag.wrapping_neg()
    } else {
        mag
    }
}
