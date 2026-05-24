//! `div_widen_scale` — decimal division by the widen-then-divide method.
//!
//! Divides `a / b` for two same-`SCALE` decimals stored as `Int<N>`. The
//! logical quotient is `(a / 10^SCALE) / (b / 10^SCALE) = a / b`, but to
//! keep `SCALE` fractional digits the numerator is first scaled up by
//! `10^SCALE` (`a * mult`). Scaling can overflow `Int<N>`, so the kernel
//! widens to the next-up work width `Int<W>` before dividing — except on a
//! value-gated fast path, where a leading-zero count proves `a * mult` fits
//! `Int<N>` exactly and the divide stays narrow.
//!
//! # Generic over the storage and work widths `(N, W)`
//!
//! The kernel is parameterised over the storage limb count `N` and the
//! next-up work limb count `W` (`Int<W>` covers `a * 10^SCALE` without
//! overflow). The pre-computed scale multiplier `mult = 10^SCALE` is
//! supplied by the caller (the policy evaluates the per-tier `multiplier()`
//! const fn so the `leading_zeros()` collapses at compile time). `W` is a
//! *work* width, not an algorithm distinction.

use crate::int::types::Int;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Widen-then-divide decimal division kernel for storage `Int<N>` and work
/// width `Int<W>`. `mult` is the pre-computed `10^SCALE` scale multiplier.
///
/// A fast path skips the widen step when `a * mult` provably fits `Int<N>`
/// (via leading-zero counts); otherwise the numerator is widened to
/// `Int<W>`, divided by `b`, rounded under `mode`, and narrowed back to
/// `Int<N>` (debug panic / release wrap on overflow).
#[inline]
pub(crate) fn div_widen_scale<const N: usize, const W: usize>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    let lz_n = a.unsigned_abs().leading_zeros();
    let lz_m = mult.leading_zeros();
    if lz_n + lz_m > <Int<N>>::BITS {
        // Fast path: `a * mult` fits `Int<N>`.
        let n: Int<N> = a.wrapping_mul(mult);
        let result = crate::macros::arithmetic::round_with_mode_wide!(
            n, b, Int<N>, mode
        );
        return result;
    }
    // Slow path: widen numerator, divide in `Int<W>`.
    let b_wide: Int<W> = b.resize::<Int<W>>();
    let n: Int<W> = a.widen_mul::<Int<W>>(mult);
    let result = crate::macros::arithmetic::round_with_mode_wide!(
        n, b_wide, Int<W>, mode
    );
    crate::macros::arithmetic::narrow_or_panic!(
        result, Int<N>, Int<W>,
        "attempt to divide with overflow"
    )
}
