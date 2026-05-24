//! `mul_schoolbook` -- naive schoolbook decimal multiplication reference.
//!
//! Computes `a * b` for two same-`SCALE` decimals stored as `Int<N>`.
//! The logical product is `(a / 10^SCALE) * (b / 10^SCALE)`, whose raw
//! storage value is `a * b / 10^SCALE`.
//!
//! This is the naive reference algorithm:
//!
//! 1. Widen both operands to `Int<W>` (the next-up work width).
//! 2. Form the full product `a * b` via `widen_mul` (dispatches to the
//!    `Int<N>` integer layer).
//! 3. Divide the product by `10^SCALE` via the `Int<W>` layer `div_rem`,
//!    rounding under `mode`.
//! 4. Narrow the quotient back to `Int<N>`.
//!
//! Unlike [`mul_widen_divide`](super::mul_widen_divide::mul_widen_divide),
//! this kernel has no leading-zero fast path and does not use the
//! MG-divide or Newton-reciprocal acceleration: it uses only `widen_mul`
//! and `div_rem` from the `Int<W>` layer, making it the unambiguous
//! schoolbook reference.
//!
//! All integer arithmetic dispatches DOWN to `Int<N>` / `Int<W>` operators
//! and methods; this fn never calls a decimal method on its own value.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Naive schoolbook decimal multiplication for storage `Int<N>` and work
/// width `Int<W>`. `SL` / `WL` are the u128-limb counts of `Int<N>` /
/// `Int<W>` (`(N + 1) / 2` and `(W + 1) / 2`), mirroring the
/// `mul_widen_divide` signature so the policy can thread the same const
/// params.
///
/// Forms the full product `a * b` in `Int<W>` via `widen_mul`, then
/// divides by `10^SCALE` using the int-layer `div_rem`, rounding under
/// `mode`. No MG-divide, no Newton-reciprocal, no leading-zero fast path.
/// `SCALE == 0` returns the narrowed product unscaled.
#[inline]
pub(crate) fn mul_schoolbook<
    const N: usize,
    const W: usize,
    const SL: usize,
    const WL: usize,
    const SCALE: u32,
>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    // Full-width product via the Int<N> widen_mul (dispatches to the
    // integer layer; no decimal method is called on `a` or `b`).
    let product: Int<W> = a.widen_mul::<Int<W>>(b);

    if SCALE == 0 {
        // No scaling needed; narrow straight back.
        return crate::macros::arithmetic::narrow_or_panic!(
            product,
            Int<N>,
            Int<W>,
            "attempt to multiply with overflow"
        );
    }

    // Build the scale divisor 10^SCALE in the work width.
    // Use Int<W>::TEN.pow(SCALE) -- routes through the Int<W> layer,
    // never re-enters a decimal policy.
    let divisor: Int<W> = <Int<W>>::TEN.pow(SCALE);

    // Divide via the Int<W> layer div_rem and round.
    let scaled = crate::macros::arithmetic::round_with_mode_wide!(
        product, divisor, Int<W>, mode
    );

    crate::macros::arithmetic::narrow_or_panic!(
        scaled,
        Int<N>,
        Int<W>,
        "attempt to multiply with overflow"
    )
}
