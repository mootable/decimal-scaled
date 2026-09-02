// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `cbrt_newton_with_table_seed` — bespoke cube-root kernel for the
//! `(D57, SCALE == 20)` cell.
//!
//! The generic D57 cbrt kernel works in `Int<12>` because
//! `MAX_SCALE = 57` forces `mag · 10^(2·SCALE)` to span up to `~10^171`,
//! which overflows narrower work integers. At `SCALE = 20` the radicand
//! is bounded by `mag · 10^40 ≤ 10^57 · 10^40 = 10^97` which fits
//! `Int<6>` (~`10^115`) — half the limb count of the generic `Int<12>`
//! path.
//!
//! Newton iteration cost scales `O(L²)` per `n / (x · x)` step (one wide
//! `mul` plus a Knuth `div` on operands of limb count `L`), so dropping
//! from `L = 12` to `L = 6` shrinks each iteration ~4×.
//!
//! # `f64`-bridge Newton seed
//!
//! The floor cube root is taken via [`Int::icbrt`] (the int `icbrt`
//! policy's seeded Newton limb kernel). The seed — the only
//! `std`/`no_std` divergence — is encapsulated in the cross-algorithm
//! seed leaf that kernel calls: an `f64::cbrt`-derived ~53-bit
//! over-estimate under `std`, the classical 1-bit seed under `no_std`.
//! Both return the exact `⌊∛n⌋`, so this body is cfg-free.
//!
//! Result is bit-for-bit identical to [`crate::algos::cbrt::cbrt_newton`]
//! under every [`RoundingMode`] value; only the work-integer width
//! (`Int<6>` vs the generic `Int<12>`) and the seed source change. See
//! [`crate::algos::cbrt::cbrt_newton`] for the Newton + half-step
//! rounding algorithm.
//!
//! NOT feature-gated: referenced by the feature-independent `cbrt` policy (as
//! a kept reference seam), so it compiles in every build.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

const SCALE: u32 = 20;

/// `D57<20>` cube-root kernel. The floor cube root is taken via the
/// integer wide-kernel surface ([`Int::icbrt`] → the int `icbrt` policy);
/// the `f64::cbrt`-vs-classical seed std/no_std choice is encapsulated in
/// the seed leaf the kernel calls, so this body is cfg-free. The
/// half-step rounding mirrors [`crate::algos::cbrt::cbrt_newton`]
/// exactly; the result is bit-identical to the generic path under every
/// [`RoundingMode`] value, only the iteration count differs.
#[inline]
#[must_use]
pub(crate) fn cbrt_newton_with_table_seed(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    if raw == Int::<3>::ZERO {
        return Int::<3>::ZERO;
    }
    let zero = Int::<6>::ZERO;
    let one = Int::<6>::ONE;
    let widened: Int<6> = raw.resize_to::<Int<6>>();
    let is_negative = widened < zero;
    let magnitude = if is_negative { -widened } else { widened };
    let radicand: Int<6> =
        magnitude * const { crate::consts::pow10::dispatch_int::<6>(2 * SCALE) };

    let root: Int<6> = radicand.icbrt();

    // ── single half-step round (same logic as cbrt_newton). ──────────
    let eight_radicand = radicand << 3u32;
    let doubled_midpoint = root + root + one;
    let cube = doubled_midpoint * doubled_midpoint * doubled_midpoint;
    let halfway_geq = eight_radicand >= cube;
    let halfway_gt = eight_radicand > cube;
    let tie = halfway_geq && !halfway_gt;
    let two_root = root + root;
    let eight_root_cubed = if root == zero { zero } else { two_root * two_root * two_root };
    let residual_nonzero = eight_radicand > eight_root_cubed;
    // Last decimal digit of the (non-negative) root magnitude `root`.
    let root_mod_10 = (root % Int::<6>::TEN).as_i128() as u8;
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && root_mod_10 & 1 == 1),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => is_negative && residual_nonzero,
        RoundingMode::Ceiling => !is_negative && residual_nonzero,
        // `root` is the magnitude, so away-from-zero is a bump either sign.
        RoundingMode::AwayFromZero => residual_nonzero,
        RoundingMode::ZeroFiveUp => residual_nonzero && matches!(root_mod_10, 0 | 5),
    };
    let root = if bump { root + one } else { root };
    let signed_root = if is_negative { -root } else { root };
    signed_root.resize_to::<Int<3>>()
}
