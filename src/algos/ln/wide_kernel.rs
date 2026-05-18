//! Natural-logarithm wide-tier kernels — per-tier free functions
//! lifted from the inherent `ln_strict` / `ln_strict_with` bodies that
//! `decl_wide_transcendental!` emits.
//!
//! Each function takes the raw storage integer plus the rounding mode
//! and returns the raw storage integer. The body is the same `w =
//! SCALE + GUARD`, `ln_fixed`, `round_to_storage_with` shape as the
//! existing macro-emitted methods, just lifted to a policy-callable
//! free function. The per-tier transcendental core lives at
//! `crate::core_type::wide_trig_<tier>` (the module the
//! `decl_wide_transcendental!` macro emits next to each `Dxx` struct).
//!
//! Panics if `raw <= 0` — matches the long-standing `ln_strict`
//! contract for the wide tiers.
//!
//! The wide-tier core doesn't ship a runtime-`working_digits` variant
//! of `ln_fixed`; the strict path with `SCALE + GUARD` is the only
//! widely-validated kernel. `LnPolicy::ln_with_impl` for wide tiers
//! therefore ignores the requested working digits and delegates to the
//! strict path; this is intentional and documented at the policy call
//! site.

use crate::rounding::RoundingMode;

/// Emits a `ln_strict_<tier>(raw, mode)` free function for one wide
/// tier. `$Storage` is the type's raw storage integer; `$core_path` is
/// the per-tier transcendental core module path (e.g.
/// `crate::core_type::wide_trig_d56`).
macro_rules! decl_ln_kernel {
    ($name:ident, $Storage:ty, $core_path:path, $tier_label:literal) => {
        /// Wide-tier `ln_strict` kernel. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            let zero = <$Storage>::from_str_radix("0", 10)
                .expect(concat!("ln_strict_", $tier_label, ": invalid base-10 literal"));
            if raw <= zero {
                panic!(concat!($tier_label, "::ln: argument must be positive"));
            }
            let w = scale + core::GUARD;
            let r = core::ln_fixed(core::to_work(raw), w);
            core::round_to_storage_with(r, w, scale, mode)
        }
    };
}

#[cfg(any(feature = "d56", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d56,
    crate::wide_int::I192,
    crate::core_type::wide_trig_d56,
    "D56"
);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d76,
    crate::wide_int::I256,
    crate::core_type::wide_trig_d76,
    "D76"
);

#[cfg(any(feature = "d114", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d114,
    crate::wide_int::I384,
    crate::core_type::wide_trig_d114,
    "D114"
);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d153,
    crate::wide_int::I512,
    crate::core_type::wide_trig_d153,
    "D153"
);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d230,
    crate::wide_int::I768,
    crate::core_type::wide_trig_d230,
    "D230"
);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_ln_kernel!(
    ln_strict_d307,
    crate::wide_int::I1024,
    crate::core_type::wide_trig_d307,
    "D307"
);

#[cfg(any(feature = "d461", feature = "x-wide"))]
decl_ln_kernel!(
    ln_strict_d461,
    crate::wide_int::I1536,
    crate::core_type::wide_trig_d461,
    "D461"
);

#[cfg(any(feature = "d615", feature = "x-wide"))]
decl_ln_kernel!(
    ln_strict_d615,
    crate::wide_int::I2048,
    crate::core_type::wide_trig_d615,
    "D615"
);

#[cfg(any(feature = "d923", feature = "xx-wide"))]
decl_ln_kernel!(
    ln_strict_d923,
    crate::wide_int::I3072,
    crate::core_type::wide_trig_d923,
    "D923"
);

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
decl_ln_kernel!(
    ln_strict_d1231,
    crate::wide_int::I4096,
    crate::core_type::wide_trig_d1231,
    "D1231"
);
