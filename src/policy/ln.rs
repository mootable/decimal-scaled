//! Natural-logarithm policy — narrow-tier only (D9 / D18 / D38).
//!
//! Wide-tier `ln_strict` remains macro-emitted today (it already
//! delegates to a per-tier `ln_fixed` core function); migrating those
//! through the policy trait is deferred to a separate effort because
//! the per-tier core modules are `pub(super)` and need their access
//! widened first.
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
