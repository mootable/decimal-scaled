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
//! Result is bit-for-bit identical to the generic kernel under all
//! six [`RoundingMode`] values. See [`super::generic_wide`] for the
//! Newton + half-step rounding algorithm.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::wide_int::{Int192, Int384, WideStorage};

/// `D57<20>` cube-root kernel. Runs Newton in `Int384` instead of
/// the generic Int768.
#[inline]
#[must_use]
pub(crate) fn cbrt(raw: Int192, mode: RoundingMode) -> Int192 {
    super::generic_wide::cbrt::<Int192, Int384>(raw, 20, mode)
}

// Suppress dead_code: `WideStorage` import is what the generic kernel
// resolves to during monomorphisation, but our local function only
// names `Int384`; the trait bound is satisfied transitively.
const _: fn() = || {
    let _: fn(Int192, RoundingMode) -> Int192 = cbrt;
    let _ = <Int384 as WideStorage>::BITS;
};
