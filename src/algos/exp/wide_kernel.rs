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
//!
//! # Why this file stays per-tier (no `BigInt` collapse)
//!
//! See the matching note in [`crate::algos::ln::wide_kernel`] — the
//! same reasoning applies. `core::exp_fixed`, `core::to_work`,
//! `core::round_to_storage_with`, and `core::GUARD` are emitted
//! per-tier by `decl_wide_transcendental!` against a tier-specific
//! work integer `W` and tier-specific constant tables. The wrappers
//! below stay per-tier; collapsing them would force a larger refactor
//! (either the macro itself or a new `WideTrigCore` trait) for no
//! shape-saving in the 3-line wrapper body.

#[allow(unused_imports)]
use crate::support::rounding::RoundingMode;

/// Emits an `exp_strict_<tier>(raw, mode, scale)` free function for
/// one wide tier. See module docs.
#[allow(unused_macros)]
macro_rules! decl_exp_kernel {
    ($name:ident, $Storage:ty, $core_path:path, $tier_label:literal) => {
        /// Wide-tier `exp_strict` kernel. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            let zero = <$Storage>::from_str_radix("0", 10).expect(concat!(
                "exp_strict_",
                $tier_label,
                ": invalid base-10 literal"
            ));
            if raw == zero {
                // `Dxx<SCALE>::ONE` raw is `10^SCALE`; build it from
                // the storage type via the same path the macro uses.
                let ten = <$Storage>::from_str_radix("10", 10).expect(concat!(
                    "exp_strict_",
                    $tier_label,
                    ": invalid base-10 literal"
                ));
                return ten.pow(scale);
            }
            core::round_to_storage_directed(core::GUARD, scale, mode, |guard| {
                core::exp_fixed(core::to_work_w(raw, guard), scale + guard)
            })
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d57,
    crate::int::types::Int<3>,
    crate::types::widths::wide_trig_d57,
    "D57"
);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d76,
    crate::int::types::Int<4>,
    crate::types::widths::wide_trig_d76,
    "D76"
);

#[cfg(any(feature = "d115", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d115,
    crate::int::types::Int<6>,
    crate::types::widths::wide_trig_d115,
    "D115"
);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d153,
    crate::int::types::Int<8>,
    crate::types::widths::wide_trig_d153,
    "D153"
);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_exp_kernel!(
    exp_strict_d230,
    crate::int::types::Int<12>,
    crate::types::widths::wide_trig_d230,
    "D230"
);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_exp_kernel!(
    exp_strict_d307,
    crate::int::types::Int<16>,
    crate::types::widths::wide_trig_d307,
    "D307"
);

#[cfg(any(feature = "d462", feature = "x-wide"))]
decl_exp_kernel!(
    exp_strict_d462,
    crate::int::types::Int<24>,
    crate::types::widths::wide_trig_d462,
    "D462"
);

#[cfg(any(feature = "d616", feature = "x-wide"))]
decl_exp_kernel!(
    exp_strict_d616,
    crate::int::types::Int<32>,
    crate::types::widths::wide_trig_d616,
    "D616"
);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
decl_exp_kernel!(
    exp_strict_d924,
    crate::int::types::Int<48>,
    crate::types::widths::wide_trig_d924,
    "D924"
);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
decl_exp_kernel!(
    exp_strict_d1232,
    crate::int::types::Int<64>,
    crate::types::widths::wide_trig_d1232,
    "D1232"
);
