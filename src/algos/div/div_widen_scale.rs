//! `div_widen_scale` — decimal division by the widen-then-divide method,
//! generic over the storage width `N` only.
//!
//! Divides `a / b` for two same-`SCALE` decimals stored as `Int<N>`. The
//! logical quotient is `(a / 10^SCALE) / (b / 10^SCALE) = a / b`, but to
//! keep `SCALE` fractional digits the numerator is first scaled up by
//! `10^SCALE` (`a * 10^SCALE`). Scaling can overflow `Int<N>`, so the
//! scaled numerator spans up to `2N` limbs and is formed in a limb
//! **scratch buffer** rather than a work *type* `Int<2N>`.
//!
//! # Generic over the storage width only — no `Int<2N>` work type
//!
//! Following the `sqrt`/`cbrt`/`hypot` template, the kernel is generic over
//! `N` alone:
//!
//! 1. form `|a| * 10^SCALE` (`2N` u64 limbs) in a [`WorkingDecimal::work2`]
//!    buffer via the int slice multiply;
//! 2. divide it by `|b|` via the int layer's width-agnostic divide
//!    ([`crate::int::algos::div::div_fixed::div_rem_mag_slice`], which
//!    fronts the divisor-shape policy — Knuth / single-limb fast paths),
//!    rounding under `mode`;
//! 3. rebuild the signed `Int<N>` quotient (debug panic / release wrap on
//!    overflow).
//!
//! The divisor here is the runtime operand `b`, not `10^SCALE`, so the MG
//! magic-divide does not apply — the int-layer `div_rem` (with its own
//! hardware fast paths) is the right engine, exactly as the prior
//! `Int<W>::div_rem` path used.
//!
//! All integer arithmetic dispatches DOWN to the int layer; this fn never
//! calls a decimal method on its own value.

use crate::int::algos::div::div_fixed::div_rem_mag_slice;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::types::work_scratch::WorkingDecimal;
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

/// Compare `2*rem` against `divisor` (little-endian magnitudes), returning
/// the ordering of `rem` vs `divisor - rem` (the rounding half-comparison).
#[inline]
fn cmp_double_vs(rem: &[u64], divisor: &[u64]) -> core::cmp::Ordering {
    let mut two_r = [0u64; 80];
    let mut carry: u64 = 0;
    for (i, &r) in rem.iter().enumerate() {
        let v = ((r as u128) << 1) | carry as u128;
        two_r[i] = v as u64;
        carry = (v >> 64) as u64;
    }
    let mut len = rem.len();
    if carry != 0 {
        two_r[len] = carry;
        len += 1;
    }
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
/// debug-panicking on overflow.
#[inline]
fn apply_sign<const N: usize>(out: [u64; N], neg: bool, msg: &str) -> Int<N> {
    let mag = Int::<N>::from_limbs(out);
    if cfg!(debug_assertions) && mag.is_negative() && !(neg && mag == Int::<N>::MIN) {
        panic!("{msg}");
    }
    if neg {
        mag.wrapping_neg()
    } else {
        mag
    }
}

/// Widen-then-divide decimal division kernel, generic over `N`. Requires
/// `Int<N>: WorkingDecimal` for the `2N`-limb scaled-numerator scratch.
///
/// `mult` is the pre-computed `10^SCALE` multiplier in `Int<N>` storage
/// (the policy evaluates the per-type `multiplier()` const so it folds at
/// compile time). Forms `|a| * mult` in scratch, divides by `|b|` via the
/// int layer, rounds under `mode`, and rebuilds the signed quotient.
///
/// Panics on a zero divisor.
#[inline]
pub(crate) fn div_widen_scale<const N: usize>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Int<N>: WorkingDecimal,
{
    if b == Int::<N>::ZERO {
        panic!("attempt to divide by zero");
    }
    let neg = a.is_negative() != b.is_negative();
    let a_mag = *a.unsigned_abs().as_limbs();
    let m_mag = *mult.as_limbs(); // mult >= 0
    let b_mag = *b.unsigned_abs().as_limbs();
    let al = sig_len(&a_mag);
    let ml = sig_len(&m_mag);
    let bl = sig_len(&b_mag);

    // Scaled numerator |a| * 10^SCALE (up to 2N u64 limbs) in scratch.
    let mut num_buf = Int::<N>::work2();
    let num = num_buf.as_mut();
    let nlen = (al + ml).min(num.len());
    for slot in num[..nlen].iter_mut() {
        *slot = 0;
    }
    mul_schoolbook(&a_mag[..al], &m_mag[..ml], &mut num[..nlen]);
    let ntop = sig_len(&num[..nlen]);

    // q = num / b, r = num % b (magnitudes, via the int layer).
    let mut quot_buf = Int::<N>::work2();
    let quot = quot_buf.as_mut();
    let mut rem_buf = Int::<N>::work2();
    let rem = rem_buf.as_mut();
    let qlen = ntop.max(1);
    for slot in quot[..qlen].iter_mut() {
        *slot = 0;
    }
    for slot in rem[..bl.max(1)].iter_mut() {
        *slot = 0;
    }
    div_rem_mag_slice(&num[..ntop], &b_mag[..bl], &mut quot[..qlen], &mut rem[..bl.max(1)]);

    // Round per `mode`: compare remainder against b - remainder.
    let rl = sig_len(&rem[..bl.max(1)]);
    let rem_nonzero = !(rl == 1 && rem[0] == 0);
    if rem_nonzero {
        let cmp_r = cmp_double_vs(&rem[..bl.max(1)], &b_mag[..bl]);
        let q_is_odd = (quot[0] & 1) != 0;
        if should_bump(mode, cmp_r, q_is_odd, !neg) {
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
    apply_sign::<N>(out, neg, "attempt to divide with overflow")
}
