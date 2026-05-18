//! Cube-root policy — same cascade shape as [`crate::policy::sqrt`].

use crate::algos::cbrt;
use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

/// Per-width policy: which kernel a `Dxx<SCALE>` uses for
/// `cbrt_strict_with`. See [`crate::policy`] module docs for the
/// cascade structure.
pub(crate) trait CbrtPolicy: Sized {
    /// Cube root under the supplied rounding mode. Sign is preserved.
    fn cbrt_impl(self, mode: RoundingMode) -> Self;
}

// ── Narrow tier — width override: widen → D38 ───────────────────────

impl<const SCALE: u32> CbrtPolicy for D9<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        cbrt::widen_to_d38::cbrt_d9(self, mode)
    }
}

impl<const SCALE: u32> CbrtPolicy for D18<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        cbrt::widen_to_d38::cbrt_d18(self, mode)
    }
}

// ── D38 — width override: hand-tuned 384-bit cube root ─────────────

impl<const SCALE: u32> CbrtPolicy for D38<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::mg_divide_d38::cbrt(self.0, SCALE, mode))
    }
}

// ── Wide tiers — width default: generic_wide ───────────────────────

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D57<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        // Scale-range slot reserved for D57<20> tuning (none yet).
        Self(cbrt::generic_wide::cbrt_d57(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D76<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d76(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D115<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d115(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D153<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d153(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D230<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d230(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D307<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d307(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d462", feature = "x-wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D462<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d462(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d616", feature = "x-wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D616<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d616(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D924<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d924(self.0, SCALE, mode))
    }
}

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::core_type::D1232<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        Self(cbrt::generic_wide::cbrt_d1232(self.0, SCALE, mode))
    }
}
