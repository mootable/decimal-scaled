//! `div_schoolbook` -- naive schoolbook decimal division reference.
//!
//! Computes `a / b` for two same-`SCALE` decimals stored as `Int<N>`.
//! The logical quotient is `(a / 10^SCALE) / (b / 10^SCALE) = a / b`,
//! but to retain `SCALE` fractional digits the numerator is first scaled
//! up by `10^SCALE` before dividing.
//!
//! This is the naive reference algorithm:
//!
//! 1. Widen `a` to `Int<W>` and multiply by `mult` (= `10^SCALE`) via
//!    `widen_mul` -- dispatches to the `Int<N>` integer layer.
//! 2. Widen `b` to `Int<W>` via `resize`.
//! 3. Divide the scaled numerator by the widened denominator via the
//!    `Int<W>` layer `div_rem`, rounding under `mode`.
//! 4. Narrow the quotient back to `Int<N>`.
//!
//! Unlike [`div_widen_scale`](super::div_widen_scale::div_widen_scale),
//! this kernel has no leading-zero fast path. It uses only `Int<N>`
//! methods (`widen_mul`, `resize`) and the int-layer `div_rem`,
//! making it the unambiguous schoolbook reference.
//!
//! The `mult` (= `10^SCALE`) is accepted from the caller using the same
//! convention as `div_widen_scale` -- the policy's per-tier `multiplier()`
//! const fn collapses at compile time; the schoolbook kernel receives the
//! already-computed value and widens it.
//!
//! All integer arithmetic dispatches DOWN to `Int<N>` / `Int<W>` operators
//! and methods; this fn never calls a decimal method on its own value.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Naive schoolbook decimal division for storage `Int<N>` and work width
/// `Int<W>`. `mult` is the pre-computed `10^SCALE` in `Int<N>` storage
/// width (same calling convention as
/// [`div_widen_scale`](super::div_widen_scale::div_widen_scale)).
///
/// Scales the numerator `a` by `mult` in `Int<W>` (via `widen_mul`),
/// then divides by `b` (widened to `Int<W>`) using the int-layer `div_rem`,
/// rounding under `mode`. No leading-zero fast path.
///
/// Panics on a zero divisor.
#[inline]
pub(crate) fn div_schoolbook<const N: usize, const W: usize>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    // Scale numerator: a * 10^SCALE, widened to Int<W>.
    // widen_mul dispatches to the Int<N> integer layer.
    let num: Int<W> = a.widen_mul::<Int<W>>(mult);

    // Widen denominator to the same work width via the Int<N> layer.
    let den: Int<W> = b.resize::<Int<W>>();

    // Divide via the Int<W> layer and round. round_with_mode_wide! calls
    // Int<W>::div_rem -- no decimal method is called on `a` or `b`.
    let scaled = crate::macros::arithmetic::round_with_mode_wide!(
        num, den, Int<W>, mode
    );

    crate::macros::arithmetic::narrow_or_panic!(
        scaled,
        Int<N>,
        Int<W>,
        "attempt to divide with overflow"
    )
}
