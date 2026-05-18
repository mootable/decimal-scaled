//! Natural-logarithm policy.
//!
//! Narrow tier (D9 / D18 / D38) routes the `Fixed` 256-bit
//! intermediate kernels; wide tier (D56 .. D1231) routes the per-tier
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

// ── D38 — width override: hand-tuned `Fixed`-intermediate ln ───────

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

#[cfg(any(feature = "d56", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D56<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        // Wide-tier core has no runtime-guard `ln_fixed`; delegate
        // to the strict path. See module docs.
        Self(ln::wide_kernel::ln_strict_d56(self.0, mode, SCALE))
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

#[cfg(any(feature = "d114", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D114<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d114(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d114(self.0, mode, SCALE))
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

#[cfg(any(feature = "d461", feature = "x-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D461<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d461(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d461(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d615", feature = "x-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D615<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d615(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d615(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d923", feature = "xx-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D923<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d923(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d923(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
impl<const SCALE: u32> LnPolicy for crate::core_type::D1231<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d1231(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::wide_kernel::ln_strict_d1231(self.0, mode, SCALE))
    }
}
