// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_int_layer` -- decimal remainder via the `Int<N>` layer.

use crate::int::algos::div::div_knuth::div_knuth_into;
use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;

/// Decimal remainder via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: division by zero always panics; `MIN % -ONE`
/// panics in debug (with "overflow") and wraps in release (matching
/// `i128::wrapping_rem`). No rescaling needed -- same-SCALE operands share
/// the scale factor.
///
/// Resolves the remainder via the Knuth engine [`div_knuth_into`] with
/// **exact `ComputeInt` scratch** (`single_buffered_u64`, `N + 2` per width)
/// instead of the `Rem` operator's build-max `[u64; MAX_SINGLE_LIMBS]`
/// Knuth buffers. `div_knuth_into` routes a single-limb divisor to the
/// hardware path internally and Burnikel–Ziegler never engages at these
/// widths, so calling the engine directly is the matcher's identical
/// choice — bit-identical result — while sizing the normalised `u`/`v` to
/// the operand width drops the build-max memset that dominated the wide-tier
/// remainder (98% of the cost at D57 … 12% at D1232). The bare `Rem`
/// operator must stay build-max (blanket over all `N`, the `exact-scratch`
/// wall); this concrete-`N` decimal kernel carries `Int<N>: ComputeInt`.
///
/// Only reached for `N >= 3` (the decimal `rem` policy routes `N <= 2` to
/// `rem_native`), so the narrow hardware-`%` path is untouched; every such
/// `N` is in the `exact-scratch` width list, so the `ComputeInt` bound
/// discharges at the concrete `N` and never cascades.
///
/// [`div_knuth_into`]: crate::int::algos::div::div_knuth::div_knuth_into
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
    // Exact per-`N` Knuth scratch: `single_buffered_u64` is `[u64; N + 2]`, covering
    // the normalised dividend `u` (`num.len() + 2`) and divisor `v`.
    let mut u = Int::<N>::single_buffered_u64();
    let mut v = Int::<N>::single_buffered_u64();
    div_knuth_into(
        a.unsigned_abs().as_limbs(),
        b.unsigned_abs().as_limbs(),
        &mut quot,
        &mut rem,
        u.as_mut(),
        v.as_mut(),
    );
    Int::<N>::from_mag_limbs(&rem, neg_r)
}
