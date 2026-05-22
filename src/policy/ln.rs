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
use crate::policy::triplet::{policy_triplet, wtag};
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D9, D18, D38};

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
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::log"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                let wbase: D38<SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide.log_approx_with(
                    wbase,
                    working_digits,
                    mode,
                ))
                .unwrap_or_else(|_| {
                    crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::log"),
                        SCALE,
                    )
                })
            }
            #[inline]
            fn log2_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::log2"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn log2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::log2"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn log10_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::log10"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn log10_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::log10"),
                            SCALE,
                        )
                    })
            }
        }
    };
}

impl_log_widen!(
    D9,
    ln::widen_to_d38::ln_strict_d9,
    ln::widen_to_d38::ln_with_d9
);
impl_log_widen!(
    D18,
    ln::widen_to_d38::ln_strict_d18,
    ln::widen_to_d38::ln_with_d18
);

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
        Self(ln::fixed_d38::log_with(
            self.0,
            base.0,
            SCALE,
            working_digits,
            mode,
        ))
    }
    #[inline]
    fn log2_impl(self, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log2_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn log2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log2_with(
            self.0,
            SCALE,
            working_digits,
            mode,
        ))
    }
    #[inline]
    fn log10_impl(self, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log10_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn log10_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::log10_with(
            self.0,
            SCALE,
            working_digits,
            mode,
        ))
    }
}

// ── Wide tiers — width default ─────────────────────────────────────
//
// `ln_impl` / `ln_with_impl` share ONE base arm table per width via the
// `policy_triplet!` free fns (the `_with` form drops `working_digits`,
// matching today's verbatim-copy behaviour). `std` is identical to
// `base` for ln — the only std machinery is the wide-kernel constant
// cache (a later hoist concern, not visible to the policy). The log
// family (`log`, `log2`, `log10`) keeps the inherent `*_strict_with`
// shells emitted by `decl_wide_transcendental!`, which compose
// `ln_fixed` / `ln2` / `ln10` with no raw-storage free-fn equivalent.
//
// `impl_wide_ln!` takes the per-width base arm table as a token block
// and emits the triplet plus the cross-cutting `LnPolicy` impl.

macro_rules! impl_wide_ln {
    (
        $T:ident, $Storage:ty,
        $base_fn:ident, $std_fn:ident, $no_std_fn:ident,
        recv = $recv:ident, mode = $kmode:ident,
        base = { $( ($w:pat, $s:pat) => $base_expr:expr ),* $(,)? }
    ) => {
        policy_triplet! {
            storage   = $Storage,
            base_fn   = $base_fn,
            std_fn    = $std_fn,
            no_std_fn = $no_std_fn,
            recv      = $recv,
            mode      = $kmode,
            params    = {},
            base      = { $( ($w, $s) => $base_expr ),* },
            std       = {},
        }

        impl<const SCALE: u32> LnPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn ln_impl(self, mode: RoundingMode) -> Self {
                #[cfg(feature = "std")]
                { Self($std_fn::<{ wtag::$T }, SCALE>(self.0, mode)) }
                #[cfg(not(feature = "std"))]
                { Self($no_std_fn::<{ wtag::$T }, SCALE>(self.0, mode)) }
            }
            #[inline]
            fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                #[cfg(feature = "std")]
                { Self($std_fn::<{ wtag::$T }, SCALE>(self.0, mode)) }
                #[cfg(not(feature = "std"))]
                { Self($no_std_fn::<{ wtag::$T }, SCALE>(self.0, mode)) }
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

// D57 — bespoke arm so `ln` can divert SCALE in 18..=22 through the
// narrow-GUARD lookup before falling back to `wide_kernel`.
#[cfg(any(feature = "d57", feature = "wide"))]
impl_wide_ln! {
    D57, crate::wide_int::Int192,
    ln_d57_base, ln_d57_std, ln_d57_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D57, 18..=22) => ln::lookup_d57_s18_22_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D57, _)       => ln::wide_kernel::ln_strict_d57(raw, mode, SCALE)
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl_wide_ln! {
    D76, crate::wide_int::Int256,
    ln_d76_base, ln_d76_std, ln_d76_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D76, _) => ln::wide_kernel::ln_strict_d76(raw, mode, SCALE)
    }
}

// D115 — bespoke arm so `ln` can divert SCALE in 50..=60 through the
// Tang-style narrow-GUARD lookup before falling back to `wide_kernel`.
#[cfg(any(feature = "d115", feature = "wide"))]
impl_wide_ln! {
    D115, crate::wide_int::Int384,
    ln_d115_base, ln_d115_std, ln_d115_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D115, 50..=60) => ln::lookup_d115_s57_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D115, _)       => ln::wide_kernel::ln_strict_d115(raw, mode, SCALE)
    }
}

// D153 — bespoke arm so `ln` can divert SCALE in 70..=82 (the
// mid-storage band centred on SCALE = 76) through the Tang-style
// narrow-GUARD lookup before falling back to `wide_kernel`. See
// [`crate::algos::ln::lookup_d153_s70_82_tang`] for the algorithm.
#[cfg(any(feature = "d153", feature = "wide"))]
impl_wide_ln! {
    D153, crate::wide_int::Int512,
    ln_d153_base, ln_d153_std, ln_d153_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D153, 70..=82) => ln::lookup_d153_s70_82_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D153, _)       => ln::wide_kernel::ln_strict_d153(raw, mode, SCALE)
    }
}

// D230 — bespoke arm so `ln` can divert SCALE in 110..=120 (the
// popular mid-storage band centred on `SCALE = 115 ≈ MAX_SCALE / 2`)
// through the Tang-style narrow-GUARD lookup before falling back to
// `wide_kernel`. See [`crate::algos::ln::lookup_d230_s110_120_tang`].
#[cfg(any(feature = "d230", feature = "wide"))]
impl_wide_ln! {
    D230, crate::wide_int::Int768,
    ln_d230_base, ln_d230_std, ln_d230_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D230, 110..=120) => ln::lookup_d230_s110_120_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D230, _)         => ln::wide_kernel::ln_strict_d230(raw, mode, SCALE)
    }
}

// D307 — bespoke arm so `ln` can divert SCALE in 140..=160 (the
// popular mid-band centred on the half-MAX point) OR SCALE in 285..=295
// (the deep-storage band approaching MAX_SCALE = 306) through the
// Tang-style narrow-GUARD lookup before falling back to `wide_kernel`.
// See [`crate::algos::ln::lookup_d307_s140_160_tang`] and
// [`crate::algos::ln::lookup_d307_s285_295_tang`] for the algorithm.
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl_wide_ln! {
    D307, crate::wide_int::Int1024,
    ln_d307_base, ln_d307_std, ln_d307_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D307, 140..=160) => ln::lookup_d307_s140_160_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D307, 285..=295) => ln::lookup_d307_s285_295_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D307, _)         => ln::wide_kernel::ln_strict_d307(raw, mode, SCALE)
    }
}

// D462 — bespoke arm so `ln` can divert SCALE in 225..=235 (the
// popular mid-storage band centred on `SCALE = 230 = MAX_SCALE / 2`)
// through the Tang-style narrow-GUARD lookup before falling back to
// `wide_kernel`. See [`crate::algos::ln::lookup_d462_s225_235_tang`].
#[cfg(any(feature = "d462", feature = "x-wide"))]
impl_wide_ln! {
    D462, crate::wide_int::Int1536,
    ln_d462_base, ln_d462_std, ln_d462_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D462, 225..=235) => ln::lookup_d462_s225_235_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D462, _)         => ln::wide_kernel::ln_strict_d462(raw, mode, SCALE)
    }
}

// D616 — bespoke arm so `ln` can divert SCALE in 300..=315 (the
// mid-storage band centred on SCALE = 308) OR SCALE in 585..=595 (the
// deep-storage band approaching MAX_SCALE = 615) through the Tang-
// style narrow-GUARD lookup before falling back to `wide_kernel`. See
// [`crate::algos::ln::lookup_d616_s300_315_tang`] and
// [`crate::algos::ln::lookup_d616_s585_595_tang`] for the algorithm.
#[cfg(any(feature = "d616", feature = "x-wide"))]
impl_wide_ln! {
    D616, crate::wide_int::Int2048,
    ln_d616_base, ln_d616_std, ln_d616_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D616, 300..=315) => ln::lookup_d616_s300_315_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D616, 585..=595) => ln::lookup_d616_s585_595_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D616, _)         => ln::wide_kernel::ln_strict_d616(raw, mode, SCALE)
    }
}

// D924 — bespoke arm so `ln` can divert SCALE in 455..=465 (the
// popular mid-storage band centred on `SCALE = 461 ≈ MAX_SCALE / 2`)
// OR SCALE in 895..=905 (the deep-storage band approaching MAX_SCALE
// = 923) through the Tang-style narrow-GUARD lookup before falling
// back to `wide_kernel`. See
// [`crate::algos::ln::lookup_d924_s455_465_tang`] and
// [`crate::algos::ln::lookup_d924_s895_905_tang`].
#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl_wide_ln! {
    D924, crate::wide_int::Int3072,
    ln_d924_base, ln_d924_std, ln_d924_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D924, 455..=465) => ln::lookup_d924_s455_465_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D924, 895..=905) => ln::lookup_d924_s895_905_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D924, _)         => ln::wide_kernel::ln_strict_d924(raw, mode, SCALE)
    }
}

// D1232 — bespoke arm so `ln` can divert SCALE in 610..=620 (the
// mid-storage band centred on SCALE = 615) OR SCALE in 1195..=1205
// (the deep-storage band approaching MAX_SCALE = 1231) through the
// Tang-style narrow-GUARD lookup before falling back to `wide_kernel`.
// See [`crate::algos::ln::lookup_d1232_s610_620_tang`] and
// [`crate::algos::ln::lookup_d1232_s1195_1205_tang`].
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl_wide_ln! {
    D1232, crate::wide_int::Int4096,
    ln_d1232_base, ln_d1232_std, ln_d1232_no_std,
    recv = raw, mode = mode,
    base = {
        (wtag::D1232, 610..=620)   => ln::lookup_d1232_s610_620_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D1232, 1195..=1205) => ln::lookup_d1232_s1195_1205_tang::ln_strict::<SCALE>(raw, mode),
        (wtag::D1232, _)           => ln::wide_kernel::ln_strict_d1232(raw, mode, SCALE)
    }
}
