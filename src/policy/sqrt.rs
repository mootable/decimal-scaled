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
use crate::policy::triplet::{policy_triplet, wtag};
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
// ── Wide tiers — base/std/no_std triplet keyed on `match (W, SCALE)` ─
//
// Each width emits the triplet free fns (`sqrt_dNN_{base,std,no_std}`)
// under its feature gate; the trait method const-folds the std-vs-no_std
// select. The only `std` override is `(D57, 20)`: the f64-seeded `isqrt`
// path inside `lookup_d57_s20::sqrt`. Both arms name the same kernel
// today (its own internal `#[cfg]` does the right thing) so behaviour is
// byte-identical; the file split is deferred.

macro_rules! sqrt_wide_default {
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

        impl<const SCALE: u32> SqrtPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn sqrt_impl(self, mode: RoundingMode) -> Self {
                #[cfg(feature = "std")]
                { Self($std_fn::<{ wtag::$T }, SCALE>(self.0, mode)) }
                #[cfg(not(feature = "std"))]
                { Self($no_std_fn::<{ wtag::$T }, SCALE>(self.0, mode)) }
            }
        }
    };
}

// D57 — width default `generic_wide::sqrt_d57`, with the bespoke
// `(D57, 20)` cell and its f64-seeded `std` override.
#[cfg(any(feature = "d57", feature = "wide"))]
policy_triplet! {
    storage   = crate::wide_int::Int192,
    base_fn   = sqrt_d57_base,
    std_fn    = sqrt_d57_std,
    no_std_fn = sqrt_d57_no_std,
    recv      = raw,
    mode      = mode,
    params    = {},
    base      = {
        (wtag::D57, 20) => sqrt::lookup_d57_s20::sqrt(raw, mode),
        (wtag::D57, _)  => sqrt::generic_wide::sqrt_d57(raw, SCALE, mode)
    },
    std       = {
        (wtag::D57, 20) => sqrt::lookup_d57_s20::sqrt(raw, mode),
    },
}

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> SqrtPolicy for crate::types::widths::D57<SCALE> {
    #[inline]
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        { Self(sqrt_d57_std::<{ wtag::D57 }, SCALE>(self.0, mode)) }
        #[cfg(not(feature = "std"))]
        { Self(sqrt_d57_no_std::<{ wtag::D57 }, SCALE>(self.0, mode)) }
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
sqrt_wide_default!(D76, crate::wide_int::Int256, sqrt_d76_base, sqrt_d76_std, sqrt_d76_no_std, sqrt::generic_wide::sqrt_d76);

#[cfg(any(feature = "d115", feature = "wide"))]
sqrt_wide_default!(D115, crate::wide_int::Int384, sqrt_d115_base, sqrt_d115_std, sqrt_d115_no_std, sqrt::generic_wide::sqrt_d115);

#[cfg(any(feature = "d153", feature = "wide"))]
sqrt_wide_default!(D153, crate::wide_int::Int512, sqrt_d153_base, sqrt_d153_std, sqrt_d153_no_std, sqrt::generic_wide::sqrt_d153);

#[cfg(any(feature = "d230", feature = "wide"))]
sqrt_wide_default!(D230, crate::wide_int::Int768, sqrt_d230_base, sqrt_d230_std, sqrt_d230_no_std, sqrt::generic_wide::sqrt_d230);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
sqrt_wide_default!(D307, crate::wide_int::Int1024, sqrt_d307_base, sqrt_d307_std, sqrt_d307_no_std, sqrt::generic_wide::sqrt_d307);

#[cfg(any(feature = "d462", feature = "x-wide"))]
sqrt_wide_default!(D462, crate::wide_int::Int1536, sqrt_d462_base, sqrt_d462_std, sqrt_d462_no_std, sqrt::generic_wide::sqrt_d462);

#[cfg(any(feature = "d616", feature = "x-wide"))]
sqrt_wide_default!(D616, crate::wide_int::Int2048, sqrt_d616_base, sqrt_d616_std, sqrt_d616_no_std, sqrt::generic_wide::sqrt_d616);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
sqrt_wide_default!(D924, crate::wide_int::Int3072, sqrt_d924_base, sqrt_d924_std, sqrt_d924_no_std, sqrt::generic_wide::sqrt_d924);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
sqrt_wide_default!(D1232, crate::wide_int::Int4096, sqrt_d1232_base, sqrt_d1232_std, sqrt_d1232_no_std, sqrt::generic_wide::sqrt_d1232);
