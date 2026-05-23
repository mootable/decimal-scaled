//! Natural-logarithm wide-tier kernels — per-tier free functions
//! lifted from the inherent `ln_strict` / `ln_strict_with` bodies that
//! `decl_wide_transcendental!` emits.
//!
//! Each function takes the raw storage integer plus the rounding mode
//! and returns the raw storage integer. The body is the same `w =
//! SCALE + GUARD`, `ln_fixed`, `round_to_storage_with` shape as the
//! existing macro-emitted methods, just lifted to a policy-callable
//! free function. The per-tier transcendental core lives at
//! `crate::types::widths::wide_trig_<tier>` (the module the
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
//!
//! # Why this file stays per-tier (no `BigInt` collapse)
//!
//! Unlike `crate::algos::sqrt::sqrt_newton` and
//! [`crate::algos::cbrt::generic_wide`], the per-tier wrappers here do
//! *not* collapse to a single generic function over `BigInt`.
//! Each wrapper calls `core::ln_fixed`, `core::to_work`,
//! `core::round_to_storage_with`, and `core::GUARD` from the per-tier
//! `crate::types::widths::wide_trig_<tier>` module — these are emitted by
//! the `decl_wide_transcendental!` macro and carry tier-specific
//! constants (pi/ln2 tables, work-integer type alias `W`, scale
//! bounds) that vary per tier. Generalising would require either
//! reworking the macro to emit generic-over-`W` functions or
//! introducing a `WideTrigCore` trait surface — both are larger
//! refactors than the 3-line shim body justifies. The per-tier core
//! module is already the natural carrier for those bindings; the
//! wrappers below stay thin and per-tier on purpose.

use crate::support::rounding::RoundingMode;

/// Emits a `ln_strict_<tier>(raw, mode)` free function for one wide
/// tier. `$Storage` is the type's raw storage integer; `$core_path` is
/// the per-tier transcendental core module path (e.g.
/// `crate::types::widths::wide_trig_d57`).
macro_rules! decl_ln_kernel {
    ($name:ident, $Storage:ty, $core_path:path, $tier_label:literal) => {
        /// Wide-tier `ln_strict` kernel. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            let zero = <$Storage>::from_str_radix("0", 10).expect(concat!(
                "ln_strict_",
                $tier_label,
                ": invalid base-10 literal"
            ));
            if raw <= zero {
                panic!(concat!($tier_label, "::ln: argument must be positive"));
            }
            core::round_to_storage_directed(core::GUARD, scale, mode, |guard| {
                core::ln_fixed(core::to_work_w(raw, guard), scale + guard)
            })
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d57,
    crate::int::types::Int<3>,
    crate::types::widths::wide_trig_d57,
    "D57"
);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d76,
    crate::int::types::Int<4>,
    crate::types::widths::wide_trig_d76,
    "D76"
);

#[cfg(any(feature = "d115", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d115,
    crate::int::types::Int<6>,
    crate::types::widths::wide_trig_d115,
    "D115"
);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d153,
    crate::int::types::Int<8>,
    crate::types::widths::wide_trig_d153,
    "D153"
);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_ln_kernel!(
    ln_strict_d230,
    crate::int::types::Int<12>,
    crate::types::widths::wide_trig_d230,
    "D230"
);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_ln_kernel!(
    ln_strict_d307,
    crate::int::types::Int<16>,
    crate::types::widths::wide_trig_d307,
    "D307"
);

#[cfg(any(feature = "d462", feature = "x-wide"))]
decl_ln_kernel!(
    ln_strict_d462,
    crate::int::types::Int<24>,
    crate::types::widths::wide_trig_d462,
    "D462"
);

#[cfg(any(feature = "d616", feature = "x-wide"))]
decl_ln_kernel!(
    ln_strict_d616,
    crate::int::types::Int<32>,
    crate::types::widths::wide_trig_d616,
    "D616"
);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
decl_ln_kernel!(
    ln_strict_d924,
    crate::int::types::Int<48>,
    crate::types::widths::wide_trig_d924,
    "D924"
);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
decl_ln_kernel!(
    ln_strict_d1232,
    crate::int::types::Int<64>,
    crate::types::widths::wide_trig_d1232,
    "D1232"
);
