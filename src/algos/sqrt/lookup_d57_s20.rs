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
//! Algorithm: identical to [`super::generic_wide::sqrt`] but
//! parameterised at the narrower `SqrtWide = Int256` width.
//! Result is bit-for-bit identical to the generic path under all
//! six [`RoundingMode`] values; the only difference is the integer
//! width the kernel runs in.
//!
//! See [`super::generic_wide`] for the underlying algorithm and
//! `n − q²` half-step rounding logic.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::wide_int::{Int192, Int256, WideStorage};

/// `D57<20>` square-root kernel. Runs the generic isqrt over a
/// narrower `Int256` work integer instead of `Int384`.
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
    let q: Int256 = n.isqrt();
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
