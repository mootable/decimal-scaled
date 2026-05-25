//! `div_schoolbook` -- naive schoolbook decimal division reference,
//! generic over the storage width `N` only.
//!
//! Computes `a / b` for two same-`SCALE` decimals stored as `Int<N>`.
//! The logical quotient is `(a / 10^SCALE) / (b / 10^SCALE) = a / b`,
//! but to retain `SCALE` fractional digits the numerator is first scaled
//! up by `10^SCALE` before dividing.
//!
//! This is the unambiguous schoolbook reference: it forms the scaled
//! numerator `|a| * 10^SCALE` in a `2N`-limb scratch buffer and divides by
//! `|b|` via the int layer's width-agnostic `div_rem`, with no leading-zero
//! narrow shortcut. Since decimal division's divisor is the runtime operand
//! `b` (not `10^SCALE`), there is no MG / Newton path to forgo here — both
//! this reference and
//! [`div_widen_scale`](super::div_widen_scale::div_widen_scale) divide via
//! the same int-layer engine. The kernel exists as an explicit
//! benchmarkable seam, named 1:1 with the policy's `Schoolbook` variant.
//!
//! All integer arithmetic dispatches DOWN to the int layer; this fn never
//! calls a decimal method on its own value.

use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Naive schoolbook decimal division for storage `Int<N>`. `mult` is the
/// pre-computed `10^SCALE` multiplier (same convention as
/// [`div_widen_scale`](super::div_widen_scale::div_widen_scale)).
///
/// Forms the scaled numerator and divides via the int layer, rounding under
/// `mode`. Requires `Int<N>: ComputeInt`. Panics on a zero divisor.
#[inline]
pub(crate) fn div_schoolbook<const N: usize>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Int<N>: ComputeInt,
{
    // The scaled-numerator-then-int-divide pipeline is the schoolbook
    // reference; `div_widen_scale` is the same pipeline (decimal division
    // has no MG / Newton arm to drop), so delegate to it.
    super::div_widen_scale::div_widen_scale::<N>(a, b, mult, mode)
}
