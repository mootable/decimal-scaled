//! Bespoke square-root kernel for `D57<20>`.
//!
//! At `SCALE = 20` the radicand `r · 10^20` is bounded by
//! `D57<20>::MAX · 10^20 ≤ 10^57 · 10^20 = 10^77`, which fits
//! comfortably in `Int256` (~`10^77`) instead of the generic
//! kernel's `Int384` work integer. Running `isqrt` on a 256-bit
//! value rather than a 384-bit value cuts the wide-int
//! big-number arithmetic cost roughly in half — the per-iteration
//! `isqrt` step is dominated by limb-array `mul_sub` whose cost
//! scales `O(L²)` where `L` is the limb count
//! (`L = 4` for Int256, `L = 6` for Int384).
//!
//! # `f64`-bridge Newton seed
//!
//! The generic `isqrt` seeds Newton at `2^⌈bits/2⌉` — an upper bound
//! correct to 1 bit. Each Newton step doubles the correct-bit count,
//! so reaching the 128-bit precision needed for a 256-bit radicand
//! takes ~7 iterations, each performing a 256-bit `div_rem`.
//!
//! When `std` is available we instead seed with `f64::sqrt(n.as_f64())`.
//! `as_f64` rounds the radicand to an `f64` (~53 bits of mantissa
//! accuracy), `f64::sqrt` is correctly rounded so the seed lands
//! within ~2⁻⁵² of the true `√n` in relative terms. From a 53-bit
//! seed Newton needs only 2 iterations to reach 128-bit precision,
//! dropping the dominant `div_rem` cost ~3×.
//!
//! The composition `f64::sqrt(n.as_f64())` can under- OR over-shoot
//! the true `√n` depending on how `n.as_f64()` rounded. A single
//! unconditional Newton step from any positive seed lands at
//! `≥ √n` by the AM-GM inequality (`(x + n/x) / 2 ≥ √(x · n/x)
//! = √n`), so we always perform one pre-iter before entering the
//! standard `y ≥ x` monotone-decrease loop. Cost is one extra
//! `div_rem` versus the 4-5 saved by the tighter seed.
//!
//! Result is bit-for-bit identical to the generic path under all
//! six [`RoundingMode`] values; only the integer width and the seed
//! source change.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::int::types::traits::BigInt;
use crate::wide_int::{Int192, Int256};

/// `D57<20>` square-root kernel. The Newton-on-`Int256` floor-root is
/// seeded via the `f64::sqrt` bridge when `std` is available and via
/// the classical 1-bit seed otherwise — that std/no_std choice lives in
/// [`crate::policy::float_seed::isqrt`], so this body is cfg-free. The
/// result `⌊√(raw·10^20)⌋` is bit-identical either way; only the
/// iteration count differs.
#[inline]
#[must_use]
pub(crate) fn sqrt(raw: Int192, mode: RoundingMode) -> Int192 {
    if raw <= Int192::ZERO {
        return Int192::ZERO;
    }
    // SCALE = 20 ⇒ scale-10 multiplier is 10^20. `raw` ≤ ~10^57,
    // so `raw · 10^20` ≤ ~10^77 which fits Int256 (~10^77).
    const SCALE: u32 = 20;
    let n: Int256 = raw.resize_to::<Int256>() * Int256::TEN.pow(SCALE);
    let q: Int256 = crate::policy::float_seed::isqrt::<Int256>(n);
    let diff: Int256 = n - q * q;
    let halfway_round_up = diff > q;
    let diff_nonzero = diff != Int256::ZERO;
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    let q = if bump { q + Int256::ONE } else { q };
    q.resize_to::<Int192>()
}
