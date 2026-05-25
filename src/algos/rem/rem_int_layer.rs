// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_int_layer` -- decimal remainder via the `Int<N>` layer.

use crate::int::algos::div::div_knuth::div_knuth_into;
use crate::int::policy::div_rem::{select_for_limbs, Algorithm};
use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;

/// Decimal remainder via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: division by zero always panics; `MIN % -ONE`
/// panics in debug (with "overflow") and wraps in release (matching
/// `i128::wrapping_rem`). No rescaling needed -- same-SCALE operands share
/// the scale factor.
///
/// Routes on the divide matcher's verdict
/// ([`select_for_limbs`](crate::int::policy::div_rem::select_for_limbs)) and
/// resolves the remainder via the chosen engine with **exact `ComputeInt`
/// scratch** (`single_buffered_u64`, `N + 2` per width) instead of the `Rem`
/// operator's build-max `[u64; MAX_SINGLE_LIMBS]` Knuth buffers. The balanced
/// `a % b` shape never presents the wide `num ≥ 2·den` form the u128 /
/// Burnikel–Ziegler engines require, so every verdict resolves to a correct
/// Knuth divide ([`div_knuth_into`] routes a single-limb divisor to the
/// hardware path internally) — but consulting the matcher (rather than
/// hardcoding the engine) means a future engine the matcher picks for this
/// shape reaches the kernel instead of being silently bypassed. Sizing the
/// normalised `u`/`v` to the operand width drops the build-max memset that
/// dominated the wide-tier remainder (98% of the cost at D57 … 12% at
/// D1232). The bare `Rem` operator must stay build-max (blanket over all `N`,
/// the `exact-scratch` wall); this concrete-`N` decimal kernel carries
/// `Int<N>: ComputeInt`.
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
    let a_abs = a.unsigned_abs();
    let b_abs = b.unsigned_abs();
    let mut quot = [0u64; N];
    let mut rem = [0u64; N];
    // Exact per-`N` Knuth scratch: `single_buffered_u64` is `[u64; N + 2]`, covering
    // the normalised dividend `u` (`num.len() + 2`) and divisor `v`.
    let mut u = Int::<N>::single_buffered_u64();
    let mut v = Int::<N>::single_buffered_u64();
    // Exhaustive over the verdict: the balanced shape only ever yields `Rem`
    // or `Knuth`, both correct via `div_knuth_into`; the wide-only engines are
    // unreachable here but listed so adding an engine forces a decision.
    match select_for_limbs(a_abs.as_limbs(), b_abs.as_limbs()) {
        Algorithm::Rem
        | Algorithm::Knuth
        | Algorithm::KnuthU128Limb
        | Algorithm::BurnikelZieglerWithKnuth
        | Algorithm::Schoolbook => div_knuth_into(
            a_abs.as_limbs(),
            b_abs.as_limbs(),
            &mut quot,
            &mut rem,
            u.as_mut(),
            v.as_mut(),
        ),
    }
    Int::<N>::from_mag_limbs(&rem, neg_r)
}
