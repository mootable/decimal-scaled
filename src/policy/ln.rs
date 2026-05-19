//! Natural-logarithm policy (plus `log` / `log2` / `log10`).
//!
//! Narrow tier (D9 / D18 / D38) routes the `Fixed` 256-bit
//! intermediate kernels; wide tier (D57 .. D1232) routes the per-tier
//! kernels in [`crate::algos::ln::wide_kernel`] that wrap each tier's
//! macro-emitted `wide_trig_<tier>::ln_fixed` core. The wide-tier
//! macro does not ship a runtime-`working_digits` variant of
//! `ln_fixed`, so the wide-tier `_with_impl` methods ignore the
//! caller-supplied digits and fall through to the strict path. This
//! trade-off keeps `*_approx_with` / `*_with` working on wide tiers
//! (correct but no faster than `*_strict_with`); promoting it to a
//! true runtime-guard kernel is a follow-up.
//!
//! The trait carries the four-variant matrix as two methods per
//! function — `*_impl` (strict, const-folded working scale) and
//! `*_with_impl` (caller-chosen working digits) — each taking an
//! explicit rounding mode. The no-mode variants live in the typed
//! method shells and delegate here with
//! [`crate::support::rounding::DEFAULT_ROUNDING_MODE`].
//!
//! Functions covered: `ln`, `log` (variable base), `log2`, `log10`.

use crate::algos::ln;
use crate::types::widths::{D9, D18, D38};
use crate::support::rounding::RoundingMode;

/// Per-width policy for natural log and the log family. See module
/// docs.
pub(crate) trait LnPolicy: Sized {
    // ── Natural log ────────────────────────────────────────────────

    /// Strict natural log under the supplied rounding mode. Working
    /// scale is `SCALE + STRICT_GUARD` (const-folded).
    fn ln_impl(self, mode: RoundingMode) -> Self;

    /// Natural log with caller-chosen `working_digits` above the
    /// storage scale, under the supplied rounding mode.
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Log with chosen base ───────────────────────────────────────

    /// `log_base(self)` under the supplied rounding mode (strict
    /// guard).
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self;

    /// `log_base(self)` with caller-chosen guard digits.
    fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Base-2 log ─────────────────────────────────────────────────

    fn log2_impl(self, mode: RoundingMode) -> Self;
    fn log2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Base-10 log ────────────────────────────────────────────────

    fn log10_impl(self, mode: RoundingMode) -> Self;
    fn log10_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ── Narrow tier — width override: widen → D38 ───────────────────────
//
// D9 / D18 widen into D38 for every log-family method; the narrow
// strict tests verify this widen-narrow path. `log` / `log2` / `log10`
// for D9 / D18 widen, call D38's method, then narrow back via
// `TryInto` — identical to the shape `decl_strict_transcendental!`
// already uses in the macro.

macro_rules! impl_log_widen {
    ($T:ident, $ln_strict:path, $ln_with:path) => {
        impl<const SCALE: u32> LnPolicy for $T<SCALE> {
            #[inline]
            fn ln_impl(self, mode: RoundingMode) -> Self {
                $ln_strict(self, mode)
            }
            #[inline]
            fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                $ln_with(self, working_digits, mode)
            }
            #[inline]
            fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                let wbase: D38<SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide.log_strict_with(wbase, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::log"), SCALE,
                    ))
            }
            #[inline]
            fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                let wbase: D38<SCALE> = base.into();
                ::core::convert::TryInto::try_into(
                    wide.log_approx_with(wbase, working_digits, mode),
                )
                .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                    concat!(stringify!($T), "::log"), SCALE,
                ))
            }
            #[inline]
            fn log2_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::log2"), SCALE,
                    ))
            }
            #[inline]
            fn log2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::log2"), SCALE,
                    ))
            }
            #[inline]
            fn log10_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::log10"), SCALE,
                    ))
            }
            #[inline]
            fn log10_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::log10"), SCALE,
                    ))
            }
        }
    };
}

impl_log_widen!(D9, ln::widen_to_d38::ln_strict_d9, ln::widen_to_d38::ln_with_d9);
impl_log_widen!(D18, ln::widen_to_d38::ln_strict_d18, ln::widen_to_d38::ln_with_d18);

// ── D38 — width override ───────────────────────────────────────────
//
// When D57 is available, D38's ln/log family routes through
// `borrow_d57` — widen to D57, call D57's wide_kernel, narrow back.
// The D57 kernel is 2-4× faster than D38's bespoke `Fixed` 256-bit
// path at matched precision. Without `d57` / `wide` the implementation
// falls back to the `Fixed` kernels in `algos::ln::fixed_d38`.
//
// `*_with_impl`: D57's wide_kernel has no runtime-`working_digits`
// variant, so the borrow path collapses to the strict kernel.

// D38 — use the in-tree `Fixed`-256 `ln_fixed` directly. See the
// `crate::policy::exp` comment for the same routing change rationale:
// once the MG-routed Fixed primitives ship the bespoke `Fixed`
// kernel beats the borrow_d57 round trip.
impl<const SCALE: u32> LnPolicy for D38<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::ln_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::ln_with(self.0, SCALE, working_digits, mode))
    }
    #[inline]
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log_strict::<SCALE>(self.0, base.0, mode))
    }
    #[inline]
    fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log_with(self.0, base.0, SCALE, working_digits, mode))
    }
    #[inline]
    fn log2_impl(self, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log2_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn log2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log2_with(self.0, SCALE, working_digits, mode))
    }
    #[inline]
    fn log10_impl(self, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log10_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn log10_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log10_with(self.0, SCALE, working_digits, mode))
    }
}

// ── Wide tiers — width default ─────────────────────────────────────
//
// Wide tiers route `ln` through `wide_kernel::ln_strict_<tier>`; the
// log family methods (`log`, `log2`, `log10`) keep the inherent
// `*_strict_with` shells emitted by `decl_wide_transcendental!` since
// they compose `ln_fixed` / `ln2` / `ln10` from the per-tier core in a
// way that doesn't have a free-function equivalent in
// `algos::ln::wide_kernel` today. The `*_with_impl` collapses to
// strict; see module docs.
//
// `impl_wide_ln!` emits the cross-cutting `LnPolicy` impl: `ln_impl`
// via `wide_kernel`, the log family via the inherent shells.

macro_rules! impl_wide_ln {
    ($T:ident, $ln:path) => {
        impl<const SCALE: u32> LnPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn ln_impl(self, mode: RoundingMode) -> Self {
                Self($ln(self.0, mode, SCALE))
            }
            #[inline]
            fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                Self($ln(self.0, mode, SCALE))
            }
            #[inline]
            fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
                self.log_strict_with(base, mode)
            }
            #[inline]
            fn log_with_impl(self, base: Self, _working_digits: u32, mode: RoundingMode) -> Self {
                self.log_strict_with(base, mode)
            }
            #[inline]
            fn log2_impl(self, mode: RoundingMode) -> Self {
                self.log2_strict_with(mode)
            }
            #[inline]
            fn log2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                self.log2_strict_with(mode)
            }
            #[inline]
            fn log10_impl(self, mode: RoundingMode) -> Self {
                self.log10_strict_with(mode)
            }
            #[inline]
            fn log10_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                self.log10_strict_with(mode)
            }
        }
    };
}

// D57 — bespoke arm so `ln_impl` can divert SCALE ∈ 18..=22 through
// the narrow-GUARD lookup before falling back to `wide_kernel`.
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::types::widths::D57<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(ln::lookup_d57_s18_22_tang::ln_strict::<SCALE>(self.0, mode));
        }
        Self(ln::wide_kernel::ln_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(ln::lookup_d57_s18_22_tang::ln_strict::<SCALE>(self.0, mode));
        }
        Self(ln::wide_kernel::ln_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
        self.log_strict_with(base, mode)
    }
    #[inline]
    fn log_with_impl(self, base: Self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.log_strict_with(base, mode)
    }
    #[inline]
    fn log2_impl(self, mode: RoundingMode) -> Self {
        self.log2_strict_with(mode)
    }
    #[inline]
    fn log2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.log2_strict_with(mode)
    }
    #[inline]
    fn log10_impl(self, mode: RoundingMode) -> Self {
        self.log10_strict_with(mode)
    }
    #[inline]
    fn log10_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.log10_strict_with(mode)
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl_wide_ln!(D76, ln::wide_kernel::ln_strict_d76);

// D115 — bespoke arm so `ln_impl` can divert SCALE = 57 through the
// Tang-style narrow-GUARD lookup before falling back to `wide_kernel`.
#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> LnPolicy for crate::types::widths::D115<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 50..=60) {
            return Self(ln::lookup_d115_s57_tang::ln_strict::<SCALE>(self.0, mode));
        }
        Self(ln::wide_kernel::ln_strict_d115(self.0, mode, SCALE))
    }
    #[inline]
    fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 50..=60) {
            return Self(ln::lookup_d115_s57_tang::ln_strict::<SCALE>(self.0, mode));
        }
        Self(ln::wide_kernel::ln_strict_d115(self.0, mode, SCALE))
    }
    #[inline]
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
        self.log_strict_with(base, mode)
    }
    #[inline]
    fn log_with_impl(self, base: Self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.log_strict_with(base, mode)
    }
    #[inline]
    fn log2_impl(self, mode: RoundingMode) -> Self {
        self.log2_strict_with(mode)
    }
    #[inline]
    fn log2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.log2_strict_with(mode)
    }
    #[inline]
    fn log10_impl(self, mode: RoundingMode) -> Self {
        self.log10_strict_with(mode)
    }
    #[inline]
    fn log10_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.log10_strict_with(mode)
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl_wide_ln!(D153, ln::wide_kernel::ln_strict_d153);

#[cfg(any(feature = "d230", feature = "wide"))]
impl_wide_ln!(D230, ln::wide_kernel::ln_strict_d230);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl_wide_ln!(D307, ln::wide_kernel::ln_strict_d307);

#[cfg(any(feature = "d462", feature = "x-wide"))]
impl_wide_ln!(D462, ln::wide_kernel::ln_strict_d462);

#[cfg(any(feature = "d616", feature = "x-wide"))]
impl_wide_ln!(D616, ln::wide_kernel::ln_strict_d616);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl_wide_ln!(D924, ln::wide_kernel::ln_strict_d924);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl_wide_ln!(D1232, ln::wide_kernel::ln_strict_d1232);
