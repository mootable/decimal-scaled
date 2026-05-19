//! Square-root policy — per-width cascade of (scale-range → width →
//! global) kernel choices.
//!
//! Each `Dxx<SCALE>::sqrt_strict_with(mode)` delegates to
//! [`SqrtPolicy::sqrt_impl`], which is implemented once per width
//! (generic over `SCALE`) in this file. The body of each impl follows
//! the same shape — a small `if matches!(SCALE, range)` cascade for
//! scale-range overrides, then the width's chosen default kernel:
//!
//! ```ignore
//! impl<const SCALE: u32> SqrtPolicy for D57<SCALE> {
//!     fn sqrt_impl(self, mode: RoundingMode) -> Self {
//!         // Scale-range overrides — listed top-down, first match wins.
//!         if matches!(SCALE, 20..=20) {
//!             return Self(algos::sqrt::lookup_d57_s20::sqrt(self.0, mode));
//!         }
//!         // Width default — what every non-overridden scale gets.
//!         Self(algos::sqrt::generic_wide::sqrt_d57(self.0, SCALE, mode))
//!     }
//! }
//! ```
//!
//! Three-level cascade in plain code:
//!
//! 1. **Global default** — for sqrt, "the generic wide isqrt over the
//!    matching work integer" (`algos::sqrt::generic_wide::sqrt_<tier>`).
//!    No single function name carries it because the kernel signature
//!    varies per storage tier; instead, every per-width policy's
//!    fall-through arm calls the matching tier-instantiation.
//! 2. **Width override** — a per-width impl's fall-through arm picks
//!    a different kernel. Captured here today:
//!    - `D9` / `D18` → [`algos::sqrt::widen_to_d38`] (delegate via D38).
//!    - `D38` → [`algos::sqrt::mg_divide_d38`] (hand-tuned 256-bit
//!      isqrt over `i128` storage).
//! 3. **Scale-range override** — a leading `if matches!(SCALE, range)`
//!    arm picks a bespoke kernel for one or more scales. `matches!`
//!    accepts range patterns (`20..=20`, `18..=22`, `(5 | 10)`), so
//!    one arm can cover many cells.
//!
//! All three levels const-fold per monomorphisation — every concrete
//! `Dxx<S>` compiles to a direct call to one kernel only. Zero
//! runtime dispatch cost.
//!
//! [`algos::sqrt::widen_to_d38`]: crate::algos::sqrt::widen_to_d38
//! [`algos::sqrt::mg_divide_d38`]: crate::algos::sqrt::mg_divide_d38

use crate::algos::sqrt;
use crate::types::widths::{D9, D18, D38};
use crate::support::rounding::RoundingMode;

/// Per-width policy: which kernel a `Dxx<SCALE>` uses for
/// `sqrt_strict_with`. See module docs for the cascade structure.
pub(crate) trait SqrtPolicy: Sized {
    /// Square root under the supplied rounding mode. Negative inputs
    /// saturate to zero (the policy implementor handles saturation).
    fn sqrt_impl(self, mode: RoundingMode) -> Self;
}

// ── Narrow tier — width override: widen → D38 ───────────────────────

impl<const SCALE: u32> SqrtPolicy for D9<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        if self.0 <= 0 {
            return Self(0);
        }
        // Width override: widen to D38, sqrt there, narrow back.
        sqrt::widen_to_d38::sqrt_d9(self, mode)
    }
}

impl<const SCALE: u32> SqrtPolicy for D18<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        if self.0 <= 0 {
            return Self(0);
        }
        // Width override: widen to D38, sqrt there, narrow back.
        sqrt::widen_to_d38::sqrt_d18(self, mode)
    }
}

// ── D38 — width override: hand-tuned mg_divide path ────────────────

impl<const SCALE: u32> SqrtPolicy for D38<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        if self.0 <= 0 {
            return Self(0);
        }
        // Width override: 256-bit isqrt tailored to i128 storage.
        Self(sqrt::mg_divide_d38::sqrt(self.0, SCALE, mode))
    }
}

// ── Wide tiers — width default: generic_wide; D57 has a scale slot ─

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D57<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        // Scale-range overrides — first match wins.
        if matches!(SCALE, 20..=20) {
            // SCALE_OVERRIDE(D57, 20): bespoke kernel slot, currently
            // a pass-through to `generic_wide` (no behavioural change).
            // Replace `lookup_d57_s20::sqrt` body with a tuned
            // implementation when ready.
            return Self(sqrt::lookup_d57_s20::sqrt(self.0, mode));
        }
        // Width default (global default for the wide-tier family).
        Self(sqrt::generic_wide::sqrt_d57(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D76<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d76(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D115<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d115(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D153<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d153(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D230<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d230(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D307<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d307(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d462", feature = "x-wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D462<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d462(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d616", feature = "x-wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D616<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d616(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D924<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d924(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D1232<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        Self(sqrt::generic_wide::sqrt_d1232(self.0, SCALE, mode))
    }
}
