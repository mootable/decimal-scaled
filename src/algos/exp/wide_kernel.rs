//! Exponential wide-tier kernels — per-tier free functions lifted
//! from the inherent `exp_strict` / `exp_strict_with` bodies that
//! `decl_wide_transcendental!` emits.
//!
//! Mirrors [`crate::algos::ln::wide_kernel`] — see that module's docs
//! for the broader scheme. `raw == 0` short-circuits to the type's
//! `ONE` raw representation (`10^SCALE`) rather than running the
//! Taylor series.
//!
//! The wide-tier core does not ship a runtime-`working_digits` variant
//! of `exp_fixed`; the strict `SCALE + GUARD` path is the only widely-
//! validated kernel. `ExpPolicy::exp_with_impl` for wide tiers
//! therefore ignores the requested working digits and delegates to the
//! strict path; this is intentional and documented at the policy call
//! site.

use crate::rounding::RoundingMode;

/// Emits an `exp_strict_<tier>(raw, mode, scale)` free function for
/// one wide tier. See module docs.
macro_rules! decl_exp_kernel {
    ($name:ident, $Storage:ty, $core_path:path, $tier_label:literal) => {
        /// Wide-tier `exp_strict` kernel. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            let zero = <$Storage>::from_str_radix("0", 10)
                .expect(concat!("exp_strict_", $tier_label, ": invalid base-10 literal"));
            if raw == zero {
                // `Dxx<SCALE>::ONE` raw is `10^SCALE`; build it from
                // the storage type via the same path the macro uses.
                let ten = <$Storage>::from_str_radix("10", 10)
                    .expect(concat!("exp_strict_", $tier_label, ": invalid base-10 literal"));
                return ten.pow(scale);
            }
            let w = scale + core::GUARD;
            let r = core::exp_fixed(core::to_work(raw), w);
            core::round_to_storage_with(r, w, scale, mode)
        }
    };
}

#[cfg(any(feature = "d56", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d56,
    crate::wide_int::I192,
    crate::core_type::wide_trig_d56,
    "D56"
);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d76,
    crate::wide_int::I256,
    crate::core_type::wide_trig_d76,
    "D76"
);

#[cfg(any(feature = "d114", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d114,
    crate::wide_int::I384,
    crate::core_type::wide_trig_d114,
    "D114"
);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d153,
    crate::wide_int::I512,
    crate::core_type::wide_trig_d153,
    "D153"
);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d230,
    crate::wide_int::I768,
    crate::core_type::wide_trig_d230,
    "D230"
);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_exp_kernel!(
    exp_strict_d307,
    crate::wide_int::I1024,
    crate::core_type::wide_trig_d307,
    "D307"
);

#[cfg(any(feature = "d461", feature = "x-wide"))]
decl_exp_kernel!(
    exp_strict_d461,
    crate::wide_int::I1536,
    crate::core_type::wide_trig_d461,
    "D461"
);

#[cfg(any(feature = "d615", feature = "x-wide"))]
decl_exp_kernel!(
    exp_strict_d615,
    crate::wide_int::I2048,
    crate::core_type::wide_trig_d615,
    "D615"
);

#[cfg(any(feature = "d923", feature = "xx-wide"))]
decl_exp_kernel!(
    exp_strict_d923,
    crate::wide_int::I3072,
    crate::core_type::wide_trig_d923,
    "D923"
);

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
decl_exp_kernel!(
    exp_strict_d1231,
    crate::wide_int::I4096,
    crate::core_type::wide_trig_d1231,
    "D1231"
);
