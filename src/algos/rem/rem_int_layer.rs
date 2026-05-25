// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_int_layer` -- decimal remainder via the `Int<N>` layer.

use crate::int::policy::div_rem::dispatch_into as div_rem_dispatch_into;
use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;

/// Decimal remainder via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: division by zero always panics; `MIN % -ONE`
/// panics in debug (with "overflow") and wraps in release (matching
/// `i128::wrapping_rem`). No rescaling needed -- same-SCALE operands share
/// the scale factor.
///
/// Resolves the remainder through the same Knuth / Burnikel–Ziegler engine
/// the `Rem` operator uses ([`crate::int::policy::div_rem`]), but calls the
/// dispatcher's [`dispatch_into`] entry with **exact `ComputeInt` scratch**
/// (`single_limbs`, sized `N + 2` per width) instead of the operator's
/// build-max `[u64; 288]` Knuth buffers. Both paths run the identical engine
/// on the identical operands — bit-identical result — but the operator must
/// stay build-max (it is blanket over all `N`, the `exact-scratch` wall),
/// whereas this concrete-`N` decimal kernel carries `Int<N>: ComputeInt` and
/// so sizes the normalised `u`/`v` buffers to the operand width, dropping the
/// 288-limb memset that dominated the wide-tier remainder (98% of the cost at
/// D57 … 12% at D1232; and a ~25x regression vs 0.4.4 at the narrow shift-
/// subtract `div_rem` fallback this path also avoids).
///
/// Only reached for `N >= 3` (the decimal `rem` policy routes `N <= 2` to
/// `rem_native`), so the narrow hardware-`%` path is untouched; every such
/// `N` is in the `exact-scratch` width list, so the `ComputeInt` bound
/// discharges at the concrete `N` and never cascades.
///
/// [`dispatch_into`]: crate::int::policy::div_rem::dispatch_into
#[inline]
pub(crate) fn rem_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N>
where
    Int<N>: ComputeInt,
{
    // Divide-by-zero panics in both modes. In debug, the `MIN % -ONE`
    // overflow must also panic, matching the primitive contract and the
    // prior `checked_rem` path; detect it with cheap comparisons (no divide)
    // and panic before the divide wraps it to zero.
    if cfg!(debug_assertions) && a == Int::<N>::MIN && b == -Int::<N>::ONE {
        panic!("attempt to calculate the remainder with overflow");
    }
    assert!(
        !b.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );

    // Truncating-toward-zero: the remainder carries the dividend's sign.
    let neg_r = a.is_negative();
    let mut quot = [0u64; N];
    let mut rem = [0u64; N];
    // Exact per-`N` Knuth scratch: `single_limbs` is `[u64; N + 2]`, covering
    // the normalised dividend `u` (`num.len() + 2`) and divisor `v`.
    let mut u = Int::<N>::single_limbs();
    let mut v = Int::<N>::single_limbs();
    div_rem_dispatch_into(
        a.unsigned_abs().as_limbs(),
        b.unsigned_abs().as_limbs(),
        &mut quot,
        &mut rem,
        u.as_mut(),
        v.as_mut(),
    );
    Int::<N>::from_mag_limbs(&rem, neg_r)
}
