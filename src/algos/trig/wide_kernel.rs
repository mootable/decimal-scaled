//! Trigonometric wide-tier kernels — per-tier free functions lifted
//! from the inherent `sin_strict` / `cos_strict` / `tan_strict` /
//! `atan_strict` bodies (and their `*_with` siblings) that
//! `decl_wide_transcendental!` emits.
//!
//! Mirrors [`crate::algos::ln::wide_kernel`] — see that module's docs
//! for the broader scheme. Currently exposes the canonical forward and
//! inverse kernels the policy routes to (`sin`, `cos`, `tan`, `atan`);
//! `asin` / `acos` / `atan2` for the wide tiers remain macro-emitted
//! on inherent methods today.
//!
//! `cos_strict` historically had two paths inside the macro shell:
//! `sin_cos_fixed` (recovers `cos` via the Pythagorean identity,
//! shared Taylor evaluation with `sin`) and `sin_fixed(arg + π/2)`.
//! The standalone `cos_strict` kernel now uses a dedicated
//! `cos_fixed` (cofunction identity `cos(x) = sin(π/2 − x)` — one
//! `sin_fixed`, no sqrt). `sin_cos_fixed` is reserved for
//! `sin_cos_strict` where both outputs are wanted (one Taylor +
//! one sqrt vs two Taylors).
//!
//! The wide-tier core does not ship runtime-`working_digits` variants
//! of `sin_fixed` / `cos_fixed` / `atan_fixed`; the strict
//! `SCALE + GUARD` path is the only widely-validated kernel.
//! `TrigPolicy::sin_with_impl` / `cos_with_impl` / `tan_with_impl` /
//! `atan_with_impl` for wide tiers therefore ignore the requested
//! working digits and delegate to the strict path; this is intentional
//! and documented at the policy call site.
//!
//! # Why this file stays per-tier (no `BigInt` collapse)
//!
//! See the matching note in [`crate::algos::ln::wide_kernel`] — the
//! same reasoning applies, with the extra wrinkle that the trig
//! kernels share four cores (`sin_fixed`, `sin_cos_fixed`,
//! `atan_fixed`, plus `core::div` / `core::zero`) all emitted per-tier
//! by `decl_wide_transcendental!` against a tier-specific work integer
//! `W` and tier-specific pi tables. Collapsing the wrappers below
//! would not save any kernel logic — it would only delete four lines
//! per tier while forcing either a macro refactor or a new
//! `WideTrigCore` trait. The wrappers stay per-tier on purpose.

use crate::support::rounding::RoundingMode;

/// Emits four `<fn>_strict_<tier>(raw, mode, scale)` free functions
/// (sin / cos / tan / atan) for one wide tier. See module docs.
macro_rules! decl_trig_kernel {
    (
        $sin_name:ident,
        $cos_name:ident,
        $tan_name:ident,
        $atan_name:ident,
        $Storage:ty,
        $core_path:path,
        $tier_label:literal
    ) => {
        /// Wide-tier `sin_strict` kernel.
        #[inline]
        #[must_use]
        pub(crate) fn $sin_name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            core::round_to_storage_directed(core::GUARD, scale, mode, |guard| {
                core::sin_fixed(core::to_work_w(raw, guard), scale + guard)
            })
        }

        /// Wide-tier `cos_strict` kernel — standalone `cos_fixed`
        /// path via the cofunction identity `cos(x) = sin(π/2 − x)`.
        /// One `sin_fixed` evaluation, no sqrt. The `sin_cos_fixed`
        /// path is reserved for `sin_cos_strict` where both outputs
        /// are wanted.
        #[inline]
        #[must_use]
        pub(crate) fn $cos_name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            core::round_to_storage_directed(core::GUARD, scale, mode, |guard| {
                core::cos_fixed(core::to_work_w(raw, guard), scale + guard)
            })
        }

        /// Wide-tier `tan_strict` kernel. Panics at odd multiples of
        /// π/2 where the cosine is zero.
        ///
        /// Near a pole the range-reduced residue folds toward ±π/2 and
        /// `cos(r) → 0`, so dividing `sin(r)/cos(r)` at the fixed
        /// working scale amplifies the per-op rounding error by
        /// `1/cos(r) ≈ |tan|`. The base `GUARD` covers a few magnitude
        /// digits, but extreme near-pole inputs need the working scale
        /// lifted by ~log10(|tan|) extra digits (sized from a base-width
        /// probe quotient) to hold 0.5 ULP at storage. See
        /// [`crate::algos::trig::near_pole_tan`].
        #[inline]
        #[must_use]
        pub(crate) fn $tan_name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            let w0 = scale + core::GUARD;
            let (sin0, cos0) = core::sin_cos_fixed(core::to_work(raw), w0);
            if cos0 == core::zero() {
                panic!(concat!(
                    $tier_label,
                    "::tan: cosine is zero (argument is an odd multiple of pi/2)"
                ));
            }
            let probe = core::div(sin0, cos0, w0);
            let extra =
                crate::algos::trig::near_pole_tan::tan_extra_digits(core::bit_length(probe), w0)
                    .saturating_sub(core::GUARD);
            if extra == 0 {
                return core::round_to_storage_with(probe, w0, scale, mode);
            }
            let w = w0 + extra;
            let (sin_w, cos_w) = core::sin_cos_fixed(core::to_work_w(raw, core::GUARD + extra), w);
            let r = core::div(sin_w, cos_w, w);
            core::round_to_storage_with(r, w, scale, mode)
        }

        /// Wide-tier `atan_strict` kernel — result in `(−π/2, π/2)`.
        #[inline]
        #[must_use]
        pub(crate) fn $atan_name(raw: $Storage, mode: RoundingMode, scale: u32) -> $Storage {
            use $core_path as core;
            core::round_to_storage_directed(core::GUARD, scale, mode, |guard| {
                core::atan_fixed(core::to_work_w(raw, guard), scale + guard)
            })
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
decl_trig_kernel!(
    sin_strict_d57,
    cos_strict_d57,
    tan_strict_d57,
    atan_strict_d57,
    crate::wide_int::I192,
    crate::types::widths::wide_trig_d57,
    "D57"
);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_trig_kernel!(
    sin_strict_d76,
    cos_strict_d76,
    tan_strict_d76,
    atan_strict_d76,
    crate::wide_int::I256,
    crate::types::widths::wide_trig_d76,
    "D76"
);

#[cfg(any(feature = "d115", feature = "wide"))]
decl_trig_kernel!(
    sin_strict_d115,
    cos_strict_d115,
    tan_strict_d115,
    atan_strict_d115,
    crate::wide_int::I384,
    crate::types::widths::wide_trig_d115,
    "D115"
);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_trig_kernel!(
    sin_strict_d153,
    cos_strict_d153,
    tan_strict_d153,
    atan_strict_d153,
    crate::wide_int::I512,
    crate::types::widths::wide_trig_d153,
    "D153"
);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_trig_kernel!(
    sin_strict_d230,
    cos_strict_d230,
    tan_strict_d230,
    atan_strict_d230,
    crate::wide_int::I768,
    crate::types::widths::wide_trig_d230,
    "D230"
);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_trig_kernel!(
    sin_strict_d307,
    cos_strict_d307,
    tan_strict_d307,
    atan_strict_d307,
    crate::wide_int::I1024,
    crate::types::widths::wide_trig_d307,
    "D307"
);

#[cfg(any(feature = "d462", feature = "x-wide"))]
decl_trig_kernel!(
    sin_strict_d462,
    cos_strict_d462,
    tan_strict_d462,
    atan_strict_d462,
    crate::wide_int::I1536,
    crate::types::widths::wide_trig_d462,
    "D462"
);

#[cfg(any(feature = "d616", feature = "x-wide"))]
decl_trig_kernel!(
    sin_strict_d616,
    cos_strict_d616,
    tan_strict_d616,
    atan_strict_d616,
    crate::wide_int::I2048,
    crate::types::widths::wide_trig_d616,
    "D616"
);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
decl_trig_kernel!(
    sin_strict_d924,
    cos_strict_d924,
    tan_strict_d924,
    atan_strict_d924,
    crate::wide_int::I3072,
    crate::types::widths::wide_trig_d924,
    "D924"
);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
decl_trig_kernel!(
    sin_strict_d1232,
    cos_strict_d1232,
    tan_strict_d1232,
    atan_strict_d1232,
    crate::wide_int::I4096,
    crate::types::widths::wide_trig_d1232,
    "D1232"
);
