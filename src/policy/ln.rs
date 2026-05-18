//! Natural-logarithm policy.
//!
//! Narrow tier (D9 / D18 / D38) routes the `Fixed` 256-bit
//! intermediate kernels; wide tier (D57 .. D1232) routes the per-tier
//! kernels in [`crate::algos::ln::wide_kernel`] that wrap each tier's
//! macro-emitted `wide_trig_<tier>::ln_fixed` core. The wide-tier
//! macro does not ship a runtime-`working_digits` variant of
//! `ln_fixed`, so [`LnPolicy::ln_with_impl`] for the wide tiers
//! ignores the caller-supplied digits and falls through to the strict
//! path. This trade-off keeps `ln_approx_with` / `ln_with` working on
//! wide tiers (correct but no faster than `ln_strict_with`); promoting
//! it to a true runtime-guard kernel is a follow-up.
//!
//! The trait carries the four-variant matrix as two methods —
//! [`LnPolicy::ln_impl`] (strict, const-folded working scale) and
//! [`LnPolicy::ln_with_impl`] (caller-chosen working digits) — each
//! taking an explicit rounding mode. The no-mode variants
//! (`ln_strict`, `ln_approx`) live in the typed method shell and
//! delegate here with [`crate::rounding::DEFAULT_ROUNDING_MODE`].

use crate::algos::ln;
use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

/// Per-width policy for natural log. See module docs.
pub(crate) trait LnPolicy: Sized {
    /// Strict natural log under the supplied rounding mode. Working
    /// scale is `SCALE + STRICT_GUARD` (const-folded).
    fn ln_impl(self, mode: RoundingMode) -> Self;

    /// Natural log with caller-chosen `working_digits` above the
    /// storage scale, under the supplied rounding mode.
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ── Narrow tier — width override: widen → D38 ───────────────────────

impl<const SCALE: u32> LnPolicy for D9<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        ln::widen_to_d38::ln_strict_d9(self, mode)
    }
    #[inline]
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        ln::widen_to_d38::ln_with_d9(self, working_digits, mode)
    }
}

impl<const SCALE: u32> LnPolicy for D18<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        ln::widen_to_d38::ln_strict_d18(self, mode)
    }
    #[inline]
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        ln::widen_to_d38::ln_with_d18(self, working_digits, mode)
    }
}

// ── D38 — width override ───────────────────────────────────────────
//
// When D57 is available, D38's ln routes through `borrow_d57` —
// widen to D57, call D57's wide_kernel ln, narrow back. The D57
// kernel is 2-4× faster than D38's bespoke `Fixed` 256-bit path at
// matched precision (per per-scale survey v2). The hand-tuned
// `fixed_d38` kernel is retained as an alternate code path. Falls
// back to `fixed_d38` when D57 is gated out.
//
// `ln_with_impl`: D57's wide_kernel has no runtime-`working_digits`
// variant, so the borrow path collapses to the strict kernel
// (mirroring the wide-tier behaviour documented above).

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for D38<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::borrow_d57::ln_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        // D57 wide_kernel has no runtime-guard variant; delegate to
        // the strict path. See module docs.
        Self(ln::borrow_d57::ln_strict::<SCALE>(self.0, mode))
    }
}

#[cfg(not(any(feature = "d57", feature = "wide")))]
impl<const SCALE: u32> LnPolicy for D38<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::ln_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::ln_with(self.0, SCALE, working_digits, mode))
    }
}

// ── Wide tiers — width default: per-tier wide_kernel ────────────────
//
// `ln_with_impl` for wide tiers ignores `working_digits` and falls
// through to the strict path; see module-level docs for the rationale.

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D57<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        // Wide-tier core has no runtime-guard `ln_fixed`; delegate
        // to the strict path. See module docs.
        Self(ln::wide_kernel::ln_strict_d57(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D76<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d76(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d76(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D115<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d115(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d115(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D153<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d153(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d153(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D230<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d230(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d230(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D307<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d307(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d307(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d462", feature = "x-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D462<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d462(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d462(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d616", feature = "x-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D616<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d616(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d616(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D924<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d924(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d924(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D1232<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d1232(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d1232(self.0, mode, SCALE))
    }
}
