//! `mul_widen_divide` — decimal multiplication by the widen-then-divide
//! method.
//!
//! Multiplies `a * b` for two same-`SCALE` decimals stored as `Int<N>`. The
//! logical product is `(a / 10^SCALE) * (b / 10^SCALE)`, whose raw storage
//! is `a * b / 10^SCALE`. The full product spans up to twice the storage
//! width, so the kernel widens to the next-up work width `Int<W>` before
//! dividing by `10^SCALE` — except on a value-gated fast path, where a
//! leading-zero count proves `a * b` fits `Int<N>` exactly and the divide
//! stays narrow.
//!
//! # Generic over the storage and work widths, with explicit u128-limb counts
//!
//! The kernel is parameterised over the storage limb count `N`, the next-up
//! work limb count `W`, and — because stable Rust cannot derive an
//! associated const into const-generic argument position
//! (`generic_const_exprs` is nightly) — the two u128-limb counts the
//! magnitude-divide kernels need as const-generic arguments: `SL =
//! Int<N>::U128_LIMBS` and `WL = Int<W>::U128_LIMBS`. The policy supplies
//! all four as concrete literals from the per-tier macro, so the divide
//! buffers stay sized to the exact width. `W` is a *work* width, not an
//! algorithm distinction.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Widen-then-divide decimal multiplication kernel for storage `Int<N>` and
/// work width `Int<W>`. `SL` / `WL` are the u128-limb counts of `Int<N>` /
/// `Int<W>` (`(N + 1) / 2` and `(W + 1) / 2`), supplied by the caller
/// because stable Rust cannot compute them into const-generic position.
///
/// A fast path skips the widen step when `a * b` provably fits `Int<N>`
/// (via leading-zero counts); otherwise the product is formed in `Int<W>`,
/// divided by `10^SCALE` (MG divide for `SCALE <= 38`, Newton-reciprocal
/// above), and narrowed back to `Int<N>` (debug panic / release wrap on
/// overflow). `SCALE == 0` returns the product unscaled.
#[inline]
pub(crate) fn mul_widen_divide<
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
    let lz_a = a.unsigned_abs().leading_zeros();
    let lz_b = b.unsigned_abs().leading_zeros();
    if lz_a + lz_b > <Int<N>>::BITS {
        let n: Int<N> = a.wrapping_mul(b);
        let scaled = if SCALE == 0 {
            n
        } else if SCALE <= 38 {
            crate::algos::support::mg_divide::div_wide_pow10_with::<Int<N>, SL>(n, SCALE, mode)
        } else {
            crate::algos::support::newton_reciprocal::dispatch_wide_pow10_with::<Int<N>, SL>(
                n, SCALE, mode,
            )
        };
        return scaled;
    }
    let n: Int<W> = a.widen_mul::<Int<W>>(b);
    let scaled = if SCALE == 0 {
        n
    } else if SCALE <= 38 {
        crate::algos::support::mg_divide::div_wide_pow10_with::<Int<W>, WL>(n, SCALE, mode)
    } else {
        crate::algos::support::newton_reciprocal::dispatch_wide_pow10_with::<Int<W>, WL>(
            n, SCALE, mode,
        )
    };
    crate::macros::arithmetic::narrow_or_panic!(
        scaled, Int<N>, Int<W>,
        "attempt to multiply with overflow"
    )
}
