//! Cube-root policy — same cascade shape as [`crate::policy::sqrt`].

use crate::algos::cbrt;
use crate::policy::triplet::{policy_triplet, wtag};
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

/// Per-width policy: which kernel a `Dxx<SCALE>` uses for
/// `cbrt_strict_with`. See [`crate::policy`] module docs for the
/// cascade structure.
pub(crate) trait CbrtPolicy: Sized {
    /// Cube root under the supplied rounding mode. Sign is preserved.
    fn cbrt_impl(self, mode: RoundingMode) -> Self;
}

// ── Narrow tier — width override: widen → D38 ───────────────────────

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

// ── Wide tiers — base/std/no_std triplet keyed on `match (W, SCALE)` ─
//
// Mirror of `crate::policy::sqrt`. The only `std` override is the
// f64-seeded `(D57, 20)` cell (`lookup_d57_s20::cbrt`); both arms point
// at the same kernel today so behaviour is byte-identical.

macro_rules! cbrt_wide_default {
    ($T:ident, $Storage:ty, $base_fn:ident, $std_fn:ident, $no_std_fn:ident, $kernel:path) => {
        policy_triplet! {
            storage   = $Storage,
            base_fn   = $base_fn,
            std_fn    = $std_fn,
            no_std_fn = $no_std_fn,
            recv      = raw,
            mode      = mode,
            params    = {},
            base      = { (wtag::$T, _) => $kernel(raw, SCALE, mode) },
            std       = {},
        }

        impl<const SCALE: u32> CbrtPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn cbrt_impl(self, mode: RoundingMode) -> Self {
                #[cfg(feature = "std")]
                {
                    Self($std_fn::<{ wtag::$T }, SCALE>(self.0, mode))
                }
                #[cfg(not(feature = "std"))]
                {
                    Self($no_std_fn::<{ wtag::$T }, SCALE>(self.0, mode))
                }
            }
        }
    };
}

// D57 — width default `generic_wide::cbrt_d57`, with the bespoke
// `(D57, 20)` cell and its f64-seeded `std` override.
#[cfg(any(feature = "d57", feature = "wide"))]
policy_triplet! {
    storage   = crate::int::types::Int<3>,
    base_fn   = cbrt_d57_base,
    std_fn    = cbrt_d57_std,
    no_std_fn = cbrt_d57_no_std,
    recv      = raw,
    mode      = mode,
    params    = {},
    base      = {
        (wtag::D57, 20) => cbrt::lookup_d57_s20::cbrt(raw, mode),
        (wtag::D57, _)  => cbrt::generic_wide::cbrt_d57(raw, SCALE, mode)
    },
    std       = {
        (wtag::D57, 20) => cbrt::lookup_d57_s20::cbrt(raw, mode),
    },
}

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> CbrtPolicy for crate::types::widths::D57<SCALE> {
    #[inline]
    fn cbrt_impl(self, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(cbrt_d57_std::<{ wtag::D57 }, SCALE>(self.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(cbrt_d57_no_std::<{ wtag::D57 }, SCALE>(self.0, mode))
        }
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
cbrt_wide_default!(
    D76,
    crate::int::types::Int<4>,
    cbrt_d76_base,
    cbrt_d76_std,
    cbrt_d76_no_std,
    cbrt::generic_wide::cbrt_d76
);

#[cfg(any(feature = "d115", feature = "wide"))]
cbrt_wide_default!(
    D115,
    crate::int::types::Int<6>,
    cbrt_d115_base,
    cbrt_d115_std,
    cbrt_d115_no_std,
    cbrt::generic_wide::cbrt_d115
);

#[cfg(any(feature = "d153", feature = "wide"))]
cbrt_wide_default!(
    D153,
    crate::int::types::Int<8>,
    cbrt_d153_base,
    cbrt_d153_std,
    cbrt_d153_no_std,
    cbrt::generic_wide::cbrt_d153
);

#[cfg(any(feature = "d230", feature = "wide"))]
cbrt_wide_default!(
    D230,
    crate::int::types::Int<12>,
    cbrt_d230_base,
    cbrt_d230_std,
    cbrt_d230_no_std,
    cbrt::generic_wide::cbrt_d230
);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
cbrt_wide_default!(
    D307,
    crate::int::types::Int<16>,
    cbrt_d307_base,
    cbrt_d307_std,
    cbrt_d307_no_std,
    cbrt::generic_wide::cbrt_d307
);

#[cfg(any(feature = "d462", feature = "x-wide"))]
cbrt_wide_default!(
    D462,
    crate::int::types::Int<24>,
    cbrt_d462_base,
    cbrt_d462_std,
    cbrt_d462_no_std,
    cbrt::generic_wide::cbrt_d462
);

#[cfg(any(feature = "d616", feature = "x-wide"))]
cbrt_wide_default!(
    D616,
    crate::int::types::Int<32>,
    cbrt_d616_base,
    cbrt_d616_std,
    cbrt_d616_no_std,
    cbrt::generic_wide::cbrt_d616
);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
cbrt_wide_default!(
    D924,
    crate::int::types::Int<48>,
    cbrt_d924_base,
    cbrt_d924_std,
    cbrt_d924_no_std,
    cbrt::generic_wide::cbrt_d924
);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
cbrt_wide_default!(
    D1232,
    crate::int::types::Int<64>,
    cbrt_d1232_base,
    cbrt_d1232_std,
    cbrt_d1232_no_std,
    cbrt::generic_wide::cbrt_d1232
);
