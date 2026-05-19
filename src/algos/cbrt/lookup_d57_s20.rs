//! Bespoke cube-root kernel for `D57<20>`.
//!
//! The generic D57 cbrt kernel works in `Int768` (12 limbs) because
//! `MAX_SCALE = 57` forces `mag · 10^(2·SCALE)` to span up to
//! `~10^171` which overflows `Int512` (~`10^154`). At `SCALE = 20`
//! the radicand is bounded by `mag · 10^40 ≤ 10^57 · 10^40 = 10^97`
//! which fits `Int384` (~`10^115`) — half the limb count of the
//! generic Int768 path.
//!
//! Newton iteration cost scales `O(L²)` per `n / (x · x)` step
//! (one wide `mul` plus a Knuth `div` on operands of limb count `L`),
//! so dropping from `L = 12` to `L = 6` shrinks each iteration ~4×.
//!
//! # `f64`-bridge Newton seed
//!
//! The generic kernel seeds Newton at `1 << ⌈sig_bits/3⌉`, accurate
//! to 1 bit. Reaching 128-bit precision takes ~7 iterations of
//! `n / (x · x)` (one wide mul + one wide div each). When `std` is
//! available we seed with `f64::cbrt(n.as_f64())` — `as_f64`
//! preserves 53 bits of mantissa, `f64::cbrt` is correctly rounded,
//! so the seed lands within ~2⁻⁵² of the true `∛n` in relative
//! terms. From there Newton needs only 2 iterations to reach the
//! ~110-bit precision required by `Int192` storage.
//!
//! `n.as_f64()` can round either direction, so the seed may
//! over- OR under-shoot. The standard Newton cube-root iter
//! `(2x + n/x²)/3` is monotone-decreasing only from above
//! `∛n` (AM-GM-like inequality holds, but loosely). One
//! unconditional Newton pre-step ensures convergence from any
//! positive seed since it lifts to within a small factor of `∛n`
//! and the subsequent monotone-decrease loop then settles
//! to `⌊∛n⌋`.
//!
//! Result is bit-for-bit identical to the generic kernel under all
//! six [`RoundingMode`] values. See [`super::generic_wide`] for the
//! Newton + half-step rounding algorithm.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::wide_int::{Int192, Int384, WideStorage};

const SCALE: u32 = 20;

/// Newton `icbrt` over `Int384` seeded via the `f64::cbrt` bridge.
///
/// Returns `⌊∛n⌋` for `n > 0`. `n.as_f64()` + `f64::cbrt` lands a
/// seed within ~2⁻⁵² relative error of the true `∛n`. One
/// unconditional Newton step from any positive seed lifts to
/// `≥ ⌈∛n⌉` (for the cube-root iter, AM-GM on `(x, x, n/x²)`
/// gives `(2x + n/x²)/3 ≥ ∛n`); the monotone-decrease loop then
/// settles on `⌊∛n⌋`.
#[cfg(feature = "std")]
#[inline]
fn icbrt_f64_seeded(n: Int384) -> Int384 {
    let seed_f64 = n.as_f64().cbrt();
    let seed = Int384::from_f64(seed_f64);
    let x0 = if seed <= Int384::ZERO { Int384::ONE } else { seed };
    let three = Int384::from_i128(3);
    // Unconditional first Newton step. AM-GM ⇒ result ≥ ⌈∛n⌉.
    let mut x = (x0 + x0 + n / (x0 * x0)) / three;
    if x <= Int384::ZERO {
        x = Int384::ONE;
    }
    loop {
        let y = (x + x + n / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// `D57<20>` cube-root kernel. Runs Newton in `Int384` with an
/// `f64::cbrt` seed when `std` is available; falls back to the
/// generic 1-bit-seed path on `no_std`.
#[inline]
#[must_use]
pub(crate) fn cbrt(raw: Int192, mode: RoundingMode) -> Int192 {
    #[cfg(not(feature = "std"))]
    {
        return super::generic_wide::cbrt::<Int192, Int384>(raw, SCALE, mode);
    }
    #[cfg(feature = "std")]
    {
        if raw == Int192::ZERO {
            return Int192::ZERO;
        }
        let zero = Int384::ZERO;
        let one = Int384::ONE;
        let widened: Int384 = raw.resize_to::<Int384>();
        let negative = widened < zero;
        let mag = if negative { -widened } else { widened };
        let n: Int384 = mag * Int384::TEN.pow(2 * SCALE);

        let q = icbrt_f64_seeded(n);

        // ── Rounding (same logic as generic_wide). ────────────────
        let eight_n = n << 3u32;
        let t = q + q + one;
        let cube = t * t * t;
        let halfway_geq = eight_n >= cube;
        let halfway_gt = eight_n > cube;
        let tie = halfway_geq && !halfway_gt;
        let two_q = q + q;
        let eight_q_cubed = if q == zero { zero } else { two_q * two_q * two_q };
        let residual_nonzero = eight_n > eight_q_cubed;
        let q_is_odd = (q % (one + one)) != zero;
        let bump = match mode {
            RoundingMode::HalfToEven => halfway_gt || (tie && q_is_odd),
            RoundingMode::HalfAwayFromZero => halfway_geq,
            RoundingMode::HalfTowardZero => halfway_gt,
            RoundingMode::Trunc => false,
            RoundingMode::Floor => negative && residual_nonzero,
            RoundingMode::Ceiling => !negative && residual_nonzero,
        };
        let q = if bump { q + one } else { q };
        let signed = if negative { -q } else { q };
        signed.resize_to::<Int192>()
    }
}

// Suppress dead_code: `WideStorage` import is what the generic kernel
// resolves to during monomorphisation when the no-std fallback path
// is compiled.
const _: fn() = || {
    let _: fn(Int192, RoundingMode) -> Int192 = cbrt;
    let _ = <Int384 as WideStorage>::BITS;
};
