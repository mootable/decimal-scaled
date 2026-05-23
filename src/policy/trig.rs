//! Trigonometric policy — the per-`(N, SCALE)` algorithm matchers.
//!
//! `D<Int<N>, SCALE>::sin_strict_with(mode)` (and the cos / tan / atan /
//! asin / acos / atan2 / hyperbolic / angle-conversion siblings) delegate
//! to [`TrigPolicy`], which resolves each family's canonical
//! [`Algorithm`](forward::Algorithm)/`select` verdict (the `sqrt` exemplar
//! shape — see `docs/ARCHITECTURE.md` → "Policy file structure", mirrored
//! by [`crate::policy::exp`] / [`crate::policy::ln`]):
//!
//! 1. a per-family `Algorithm` enum — the real trig algorithms, no
//!    `Default` variant;
//! 2. a `Select` verdict — a settled algorithm or "the value decides"
//!    (the trig families have no value split, so `ByValue` is never
//!    returned);
//! 3. a `const fn select` keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # The four families and their algorithms
//!
//! - **forward (sin / cos / tan / atan)** — `Series` (the macro-emitted
//!   Taylor-on-reduced-residue core: `wide_kernel` on the wide tiers,
//!   `fixed_d38` on the narrow tier, the narrow-GUARD `lookup_*` slots at
//!   the low bands) and `Tang` (Tang 1991 table-driven argument reduction
//!   + residual Taylor, on the benched mid/deep SCALE bands).
//! - **inverse (asin / acos / atan2)** — `Atan` (atan-of-ratio with
//!   half-angle reduction / quadrant dispatch; the wide tiers compose the
//!   inherent `*_strict_with` shells, the D57 18..=22 band uses the
//!   narrow-GUARD lookup, D38 borrows D57 / runs `fixed_d38`).
//! - **hyperbolic (sinh / cosh / tanh)** — `ExpIdentity` (the `(eˣ, e⁻ˣ)`
//!   identity over `exp`; the benched SCALE bands divert to the
//!   Tang-routed `lookup_*_hyper` kernels).
//! - **asinh / acosh / atanh and the angle conversions** delegate to the
//!   inherent `*_strict_with` shells everywhere (one algorithm, no bands).
//!
//! # Why the kernels are threaded per-tier
//!
//! Same reasoning as [`crate::policy::exp`] / [`crate::policy::ln`]: each
//! algorithm has SCALE-/width-specific kernel realisations today (narrow
//! `Fixed`-256 in `trig::fixed_d38`, wide per-tier `wide_trig_<tier>`
//! cores in `trig::wide_kernel`, Tang/narrow-GUARD bands in the
//! `trig::lookup_*` kernels). Collapsing those kernel *bodies* to one
//! generic-over-work-width core needs the macro-emitted core to lift to a
//! generic `W` — the **4.1 genericisation prerequisite** recorded in
//! `phase4/migration_trig.md`, not a matcher concern. The matcher here is
//! canonical: the algorithm *choice* per cell is expressed once via
//! `Algorithm`/`select`, and each per-tier impl binds the concrete kernel
//! that realises the chosen algorithm at that width — the same "thread the
//! work-specifics in from the per-tier impl" shape the `sqrt` pilot uses
//! for `W`.

use crate::algos::trig;
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

pub(crate) trait TrigPolicy: Sized {
    fn sin_impl(self, mode: RoundingMode) -> Self;
    fn sin_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn cos_impl(self, mode: RoundingMode) -> Self;
    fn cos_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn tan_impl(self, mode: RoundingMode) -> Self;
    fn tan_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atan_impl(self, mode: RoundingMode) -> Self;
    fn atan_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn asin_impl(self, mode: RoundingMode) -> Self;
    fn asin_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn acos_impl(self, mode: RoundingMode) -> Self;
    fn acos_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self;
    fn atan2_with_impl(self, other: Self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Hyperbolic family ──────────────────────────────────────────
    fn sinh_impl(self, mode: RoundingMode) -> Self;
    fn sinh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn cosh_impl(self, mode: RoundingMode) -> Self;
    fn cosh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn tanh_impl(self, mode: RoundingMode) -> Self;
    fn tanh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn asinh_impl(self, mode: RoundingMode) -> Self;
    fn asinh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn acosh_impl(self, mode: RoundingMode) -> Self;
    fn acosh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atanh_impl(self, mode: RoundingMode) -> Self;
    fn atanh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Angle conversions ─────────────────────────────────────────
    fn to_degrees_impl(self, mode: RoundingMode) -> Self;
    fn to_degrees_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn to_radians_impl(self, mode: RoundingMode) -> Self;
    fn to_radians_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ══════════════════════════════════════════════════════════════════════
// Per-family matchers
//
// Each family below is its own per-function policy (an `Algorithm` enum +
// a `Select` verdict + a `const fn select<N, SCALE>` + a `resolve`), in
// the canonical `sqrt`/`exp`/`ln` shape. The forward family carries the
// `Series`/`Tang` split; the inverse and hyperbolic families carry a
// single algorithm each (the per-band kernels are realisations of that one
// algorithm, picked by a const-folding `match SCALE` inside the tier
// impl). The wide-tier `TrigPolicy` impls below resolve these verdicts and
// match them against the tier's concrete kernels.
// ══════════════════════════════════════════════════════════════════════

/// Forward family — sin / cos / tan / atan.
pub(crate) mod forward {
    use crate::int::types::Int;

    /// The forward-trig algorithms. Variants are the CamelCase of each
    /// kernel's method name minus the function prefix (`sin_series` →
    /// `Series`, `sin_tang_with_taylor` → `Tang`) — strict 1:1 with the
    /// method family. The function prefix is implicit from the family.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Algorithm {
        /// `*_series` — the macro-emitted Taylor-on-reduced-residue core.
        /// The generic default; realised by `trig::wide_kernel` (wide),
        /// `trig::fixed_d38` (narrow), and the narrow-GUARD `lookup_*`
        /// slots at the low bands (a smaller `GUARD` over the same core).
        Series,
        /// `*_tang_with_taylor` — Tang (1991) table-driven argument
        /// reduction + residual Taylor. Selected on the benched mid/deep
        /// SCALE bands; realised by the `trig::lookup_*` kernels.
        ///
        /// Every cell that selects `Tang` lives at N ≥ 3, so the variant,
        /// its `select` arms, and its dispatch arms are gated with the
        /// wide tiers (the policy stays exhaustive in both configs).
        #[cfg(feature = "_wide-support")]
        Tang,
    }

    /// A settled algorithm, or "the value decides". `ByValue` is part of
    /// the canonical shape for uniformity; the forward family never
    /// returns it (the choice is fully determined by `(N, SCALE)`).
    #[derive(Clone, Copy)]
    pub(crate) enum Select<const N: usize> {
        ByAlgorithm(Algorithm),
        #[allow(dead_code)]
        ByValue(fn(&Int<N>) -> Algorithm),
    }

    /// Pick the sin / cos / atan algorithm for storage limb count `N` and
    /// decimal `SCALE`. Total over the key; the `_` arm is the generic
    /// `Series` default. Reproduces the legacy `(W, SCALE)` triplet
    /// routing: the Tang bands divert to `Tang`; every other cell —
    /// including the narrow-GUARD low bands realised as a `Series` kernel
    /// — runs `Series`.
    ///
    /// Used by sin, cos, and atan (their Tang bands coincide). `tan` has
    /// no D57 44..=56 Tang band and routes through [`select_tan`].
    pub(crate) const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
        match (N, SCALE) {
            // D57 (`Int<3>`) Tang band.
            #[cfg(any(feature = "d57", feature = "wide"))]
            (3, 44..=56) => Select::ByAlgorithm(Algorithm::Tang),
            // D153 (`Int<8>`) Tang band.
            #[cfg(any(feature = "d153", feature = "wide"))]
            (8, 70..=82) => Select::ByAlgorithm(Algorithm::Tang),
            // D307 (`Int<16>`) Tang band.
            #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
            (16, 140..=160) => Select::ByAlgorithm(Algorithm::Tang),
            // D462 (`Int<24>`) Tang band.
            #[cfg(any(feature = "d462", feature = "x-wide"))]
            (24, 225..=235) => Select::ByAlgorithm(Algorithm::Tang),
            // Everything else — generic Series (incl. the D57 18..=22
            // narrow-GUARD band, realised as a Series kernel).
            _ => Select::ByAlgorithm(Algorithm::Series),
        }
    }

    /// `tan`-specific matcher. Identical to [`select`] except the D57
    /// 44..=56 cell stays `Series` (tan has no Tang band there — the
    /// policy wires only sin/cos/atan to the D57 44..=56 Tang kernel;
    /// tan at 44..=56 falls to the generic `wide_kernel`).
    pub(crate) const fn select_tan<const N: usize, const SCALE: u32>() -> Select<N> {
        match (N, SCALE) {
            #[cfg(any(feature = "d153", feature = "wide"))]
            (8, 70..=82) => Select::ByAlgorithm(Algorithm::Tang),
            #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
            (16, 140..=160) => Select::ByAlgorithm(Algorithm::Tang),
            #[cfg(any(feature = "d462", feature = "x-wide"))]
            (24, 225..=235) => Select::ByAlgorithm(Algorithm::Tang),
            _ => Select::ByAlgorithm(Algorithm::Series),
        }
    }

    /// Resolve the sin/cos/atan verdict to a concrete `Algorithm`.
    #[inline]
    pub(crate) fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
        match const { select::<N, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(f) => f(raw),
        }
    }

    /// Resolve the tan verdict to a concrete `Algorithm`.
    #[inline]
    pub(crate) fn resolve_tan<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
        match const { select_tan::<N, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(f) => f(raw),
        }
    }
}

/// Inverse family — asin / acos / atan2.
pub(crate) mod inverse {
    use crate::int::types::Int;

    /// The inverse-trig algorithms. One method today: `*_atan` /
    /// `*_atan_with_sqrt` — atan-of-ratio (`atan(x/√(1−x²))`) with
    /// half-angle reduction (asin/acos), quadrant dispatch (atan2). The
    /// per-band lookups are realisations of this one algorithm.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Algorithm {
        /// `asin_atan_with_sqrt` / `acos_atan_with_sqrt` / `atan2_atan` —
        /// the atan-of-ratio composition. Realised by the inherent
        /// `*_strict_with` shells (wide), the `lookup_*_inverse` kernels
        /// (D57 18..=22 band), `borrow_d57` (D38 with D57 present), and
        /// `fixed_d38` (D38 without D57).
        Atan,
    }

    /// A settled algorithm, or "the value decides". `ByValue` is part of
    /// the canonical shape; the inverse family never returns it.
    #[derive(Clone, Copy)]
    pub(crate) enum Select<const N: usize> {
        ByAlgorithm(Algorithm),
        #[allow(dead_code)]
        ByValue(fn(&Int<N>) -> Algorithm),
    }

    /// The inverse family is one algorithm at every `(N, SCALE)` — the
    /// per-band kernel is a realisation of `Atan`, picked inside the tier
    /// impl. Total over the key. The `match (N, SCALE)` form is kept (over
    /// the simpler bare body) to stay parallel with the multi-arm
    /// `forward::select` and to document the matcher key.
    #[allow(clippy::match_single_binding)]
    pub(crate) const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
        match (N, SCALE) {
            _ => Select::ByAlgorithm(Algorithm::Atan),
        }
    }

    /// Resolve the inverse verdict to a concrete `Algorithm`.
    #[inline]
    pub(crate) fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
        match const { select::<N, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(f) => f(raw),
        }
    }
}

/// Hyperbolic family — sinh / cosh / tanh.
pub(crate) mod hyper {
    use crate::int::types::Int;

    /// The hyperbolic algorithms. One method: `*_exp_identity` — the
    /// `(eˣ, e⁻ˣ)` identity over `exp`. The benched SCALE bands divert
    /// the inner `exp` through the Tang-routed `lookup_*_hyper` kernels; a
    /// band is a GUARD + exp-kernel choice (`const`-of-SCALE), not a
    /// separate algorithm.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Algorithm {
        /// `sinh_exp_identity` / `cosh_exp_identity` / `tanh_exp_identity`
        /// — the `(eˣ, e⁻ˣ)` identity over `exp`. Realised by the
        /// inherent `*_strict_with` shells (default) and the
        /// `lookup_*_hyper` kernels (benched bands), and by `fixed_d38`
        /// on the narrow tier.
        ExpIdentity,
    }

    /// A settled algorithm, or "the value decides". `ByValue` is part of
    /// the canonical shape; the hyperbolic family never returns it.
    #[derive(Clone, Copy)]
    pub(crate) enum Select<const N: usize> {
        ByAlgorithm(Algorithm),
        #[allow(dead_code)]
        ByValue(fn(&Int<N>) -> Algorithm),
    }

    /// The hyperbolic family is one algorithm at every `(N, SCALE)` — the
    /// per-band kernel is a realisation of `ExpIdentity`, picked inside
    /// the tier impl. Total over the key. The `match (N, SCALE)` form is
    /// kept (over the simpler bare body) to stay parallel with the
    /// multi-arm `forward::select` and to document the matcher key.
    #[allow(clippy::match_single_binding)]
    pub(crate) const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
        match (N, SCALE) {
            _ => Select::ByAlgorithm(Algorithm::ExpIdentity),
        }
    }

    /// Resolve the hyperbolic verdict to a concrete `Algorithm`.
    #[inline]
    pub(crate) fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
        match const { select::<N, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(f) => f(raw),
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// Narrow tier (D18) — widen-to-D38 work width, then the chosen algorithm
//
// The narrow tier has no wide storage of its own; it widens into the D38
// `Fixed` work width and runs there (the `widen_to_work` strategy — a
// policy concern, not an algorithm). The forward family uses dedicated
// `widen_to_d38` kernels; the hyperbolics, inverse hyperbolics, and angle
// conversions widen via the `TryInto` shape the inherent shells use.
// ══════════════════════════════════════════════════════════════════════

/// Emits the narrow-tier `TrigPolicy` impl that widens to D38, calls the
/// D38 method, then narrows back.
macro_rules! impl_narrow_trig {
    ($T:ident,
     $sin_s:path, $sin_w:path,
     $cos_s:path, $cos_w:path,
     $tan_s:path, $tan_w:path,
     $atan_s:path, $atan_w:path,
     $asin_s:path, $asin_w:path,
     $acos_s:path, $acos_w:path,
     $atan2_s:path, $atan2_w:path
    ) => {
        impl<const SCALE: u32> TrigPolicy for $T<SCALE> {
            #[inline]
            fn sin_impl(self, mode: RoundingMode) -> Self {
                $sin_s(self, mode)
            }
            #[inline]
            fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $sin_w(self, wd, mode)
            }
            #[inline]
            fn cos_impl(self, mode: RoundingMode) -> Self {
                $cos_s(self, mode)
            }
            #[inline]
            fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $cos_w(self, wd, mode)
            }
            #[inline]
            fn tan_impl(self, mode: RoundingMode) -> Self {
                $tan_s(self, mode)
            }
            #[inline]
            fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $tan_w(self, wd, mode)
            }
            #[inline]
            fn atan_impl(self, mode: RoundingMode) -> Self {
                $atan_s(self, mode)
            }
            #[inline]
            fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $atan_w(self, wd, mode)
            }
            #[inline]
            fn asin_impl(self, mode: RoundingMode) -> Self {
                $asin_s(self, mode)
            }
            #[inline]
            fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $asin_w(self, wd, mode)
            }
            #[inline]
            fn acos_impl(self, mode: RoundingMode) -> Self {
                $acos_s(self, mode)
            }
            #[inline]
            fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $acos_w(self, wd, mode)
            }
            #[inline]
            fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
                $atan2_s(self, other, mode)
            }
            #[inline]
            fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self {
                $atan2_w(self, other, wd, mode)
            }

            // Hyperbolics and angle conversions widen → D38 → narrow.
            #[inline]
            fn sinh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::sinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn sinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::sinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn cosh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::cosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn cosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::cosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn tanh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::tanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn tanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::tanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn asinh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::asinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn asinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::asinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn acosh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::acosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn acosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::acosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn atanh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::atanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn atanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::atanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn to_degrees_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict_with(mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_degrees"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn to_degrees_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_approx_with(wd, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_degrees"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn to_radians_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_strict_with(mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_radians"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn to_radians_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_approx_with(wd, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_radians"),
                            SCALE,
                        )
                    })
            }
        }
    };
}

impl_narrow_trig!(
    D18,
    trig::widen_to_d38::sin_strict_d18,
    trig::widen_to_d38::sin_with_d18,
    trig::widen_to_d38::cos_strict_d18,
    trig::widen_to_d38::cos_with_d18,
    trig::widen_to_d38::tan_strict_d18,
    trig::widen_to_d38::tan_with_d18,
    trig::widen_to_d38::atan_strict_d18,
    trig::widen_to_d38::atan_with_d18,
    trig::widen_to_d38::asin_strict_d18,
    trig::widen_to_d38::asin_with_d18,
    trig::widen_to_d38::acos_strict_d18,
    trig::widen_to_d38::acos_with_d18,
    trig::widen_to_d38::atan2_strict_d18,
    trig::widen_to_d38::atan2_with_d18
);

// ══════════════════════════════════════════════════════════════════════
// D38 — narrow `Fixed`-256 kernels (`fixed_d38`), with the inverse family
// borrowing D57 when it is available.
//
// N==2 always selects `Series` (forward) / `Atan` (inverse) /
// `ExpIdentity` (hyper); each `match algo` is exhaustive over the gated
// real variants and dead-arm-eliminated. The forward family runs the
// bespoke `fixed_d38` series kernel directly (it beats the widen-and-back
// path ~2× since the 0.4.2 MG-routed `Fixed` primitives). The inverse
// family borrows D57 when present (the wide_kernel atan is ~2× faster than
// the `fixed_d38` adaptive-halvings path; asin/acos/atan2 compose atan, so
// they inherit the gap) and runs `fixed_d38` without D57.
// ══════════════════════════════════════════════════════════════════════

/// D38 hyperbolic + angle-conversion methods share one `Fixed` core
/// regardless of whether the forward / inverse trig path borrows D57.
macro_rules! d38_hyperbolic_and_angle {
    () => {
        #[inline]
        fn sinh_impl(self, mode: RoundingMode) -> Self {
            Self(match hyper::resolve::<2, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => trig::fixed_d38::sinh_strict::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        fn sinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::sinh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn cosh_impl(self, mode: RoundingMode) -> Self {
            Self(match hyper::resolve::<2, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => trig::fixed_d38::cosh_strict::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        fn cosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::cosh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn tanh_impl(self, mode: RoundingMode) -> Self {
            Self(match hyper::resolve::<2, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => trig::fixed_d38::tanh_strict::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        fn tanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::tanh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn asinh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::asinh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn asinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::asinh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn acosh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::acosh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn acosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::acosh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn atanh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::atanh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn atanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::atanh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn to_degrees_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_degrees_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn to_degrees_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_degrees_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn to_radians_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_radians_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn to_radians_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_radians_with(self.0, SCALE, wd, mode))
        }
    };
}

/// D38 forward family — always `Series` on the `fixed_d38` kernel. The
/// gated `Tang` arm is dead-arm-eliminated (N==2 never selects it; it
/// forwards to the series kernel so the `match` stays exhaustive).
macro_rules! d38_forward_fixed {
    () => {
        #[inline]
        fn sin_impl(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<2, SCALE>(&self.0) {
                forward::Algorithm::Series => trig::fixed_d38::sin_strict::<SCALE>(self.0, mode),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => trig::fixed_d38::sin_strict::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::sin_with::<SCALE>(self.0, wd, mode))
        }
        #[inline]
        fn cos_impl(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<2, SCALE>(&self.0) {
                forward::Algorithm::Series => trig::fixed_d38::cos_strict::<SCALE>(self.0, mode),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => trig::fixed_d38::cos_strict::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::cos_with::<SCALE>(self.0, wd, mode))
        }
        #[inline]
        fn tan_impl(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve_tan::<2, SCALE>(&self.0) {
                forward::Algorithm::Series => trig::fixed_d38::tan_strict::<SCALE>(self.0, mode),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => trig::fixed_d38::tan_strict::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::tan_with::<SCALE>(self.0, wd, mode))
        }
    };
}

// D38 with D57 present — forward via `fixed_d38`, inverse borrows D57.
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for D38<SCALE> {
    d38_forward_fixed!();

    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::borrow_d57::atan_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::atan_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::borrow_d57::asin_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::asin_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::borrow_d57::acos_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::acos_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => {
                trig::borrow_d57::atan2_strict::<SCALE>(self.0, other.0, mode)
            }
        })
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::atan2_strict::<SCALE>(self.0, other.0, mode))
    }

    d38_hyperbolic_and_angle!();
}

// D38 without D57 — forward + inverse both on `fixed_d38`.
#[cfg(not(any(feature = "d57", feature = "wide")))]
impl<const SCALE: u32> TrigPolicy for D38<SCALE> {
    d38_forward_fixed!();

    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::fixed_d38::atan_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::atan_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::fixed_d38::asin_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::asin_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::fixed_d38::acos_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::acos_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => {
                trig::fixed_d38::atan2_strict::<SCALE>(self.0, other.0, mode)
            }
        })
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::atan2_with::<SCALE>(self.0, other.0, wd, mode))
    }

    d38_hyperbolic_and_angle!();
}

// ══════════════════════════════════════════════════════════════════════
// Wide tiers — the canonical matcher.
//
// `*_impl` / `*_with_impl` resolve the family verdict and match it against
// the tier's concrete kernels. `std` == `non_std` for every trig method —
// every trig kernel is pure-integer; the only std machinery is the
// wide-kernel constant cache, invisible to the policy — so there is no
// per-mode arm; the const verdict folds identically in both configs.
//
// The forward family (sin / cos / tan / atan) routes `Series` to the
// tier's `wide_kernel` (or the narrow-GUARD `lookup_*` at the low band)
// and `Tang` to the tier's Tang `lookup_*` band kernel. The inverse and
// hyperbolic families carry one algorithm each; their per-band kernels are
// realisations of that algorithm picked by a const-folding `match SCALE`.
// ══════════════════════════════════════════════════════════════════════

/// The inverse + asinh/acosh/atanh + angle-conversion methods that
/// delegate to the inherent `*_strict_with` shells with no per-band
/// override (the wide-tier default tail).
macro_rules! wide_trig_inverse_inherent {
    ($N:literal) => {
        #[inline]
        fn asin_impl(self, mode: RoundingMode) -> Self {
            match inverse::resolve::<$N, SCALE>(&self.0) {
                inverse::Algorithm::Atan => self.asin_strict_with(mode),
            }
        }
        #[inline]
        fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.asin_strict_with(mode)
        }
        #[inline]
        fn acos_impl(self, mode: RoundingMode) -> Self {
            match inverse::resolve::<$N, SCALE>(&self.0) {
                inverse::Algorithm::Atan => self.acos_strict_with(mode),
            }
        }
        #[inline]
        fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.acos_strict_with(mode)
        }
        #[inline]
        fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
            match inverse::resolve::<$N, SCALE>(&self.0) {
                inverse::Algorithm::Atan => self.atan2_strict_with(other, mode),
            }
        }
        #[inline]
        fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
            self.atan2_strict_with(other, mode)
        }
    };
}

/// asinh / acosh / atanh + angle conversions — inherent shells, no bands.
macro_rules! wide_trig_extra_inherent {
    () => {
        #[inline]
        fn asinh_impl(self, mode: RoundingMode) -> Self {
            self.asinh_strict_with(mode)
        }
        #[inline]
        fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.asinh_strict_with(mode)
        }
        #[inline]
        fn acosh_impl(self, mode: RoundingMode) -> Self {
            self.acosh_strict_with(mode)
        }
        #[inline]
        fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.acosh_strict_with(mode)
        }
        #[inline]
        fn atanh_impl(self, mode: RoundingMode) -> Self {
            self.atanh_strict_with(mode)
        }
        #[inline]
        fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.atanh_strict_with(mode)
        }
        #[inline]
        fn to_degrees_impl(self, mode: RoundingMode) -> Self {
            self.to_degrees_strict_with(mode)
        }
        #[inline]
        fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.to_degrees_strict_with(mode)
        }
        #[inline]
        fn to_radians_impl(self, mode: RoundingMode) -> Self {
            self.to_radians_strict_with(mode)
        }
        #[inline]
        fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.to_radians_strict_with(mode)
        }
    };
}

/// Hyperbolics — inherent `*_strict_with` shells with no per-band
/// override (`ExpIdentity` realised by the inherent composition).
macro_rules! wide_trig_hyper_inherent {
    ($N:literal) => {
        #[inline]
        fn sinh_impl(self, mode: RoundingMode) -> Self {
            match hyper::resolve::<$N, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => self.sinh_strict_with(mode),
            }
        }
        #[inline]
        fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.sinh_strict_with(mode)
        }
        #[inline]
        fn cosh_impl(self, mode: RoundingMode) -> Self {
            match hyper::resolve::<$N, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => self.cosh_strict_with(mode),
            }
        }
        #[inline]
        fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.cosh_strict_with(mode)
        }
        #[inline]
        fn tanh_impl(self, mode: RoundingMode) -> Self {
            match hyper::resolve::<$N, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => self.tanh_strict_with(mode),
            }
        }
        #[inline]
        fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.tanh_strict_with(mode)
        }
    };
}

/// Forward family for a wide tier with **no** Tang band: every forward
/// cell is `Series` on the tier `wide_kernel`. The `Tang` arm is
/// unreachable (no `select` arm yields it at this `N`) and forwards to the
/// series kernel so the `match` stays exhaustive and dead-arm-eliminated.
macro_rules! wide_trig_forward_series {
    ($N:literal, $sin:path, $cos:path, $tan:path, $atan:path) => {
        #[inline]
        fn sin_impl(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => $sin(self.0, mode, SCALE),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => $sin(self.0, mode, SCALE),
            })
        }
        #[inline]
        fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            Self($sin(self.0, mode, SCALE))
        }
        #[inline]
        fn cos_impl(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => $cos(self.0, mode, SCALE),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => $cos(self.0, mode, SCALE),
            })
        }
        #[inline]
        fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            Self($cos(self.0, mode, SCALE))
        }
        #[inline]
        fn tan_impl(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve_tan::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => $tan(self.0, mode, SCALE),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => $tan(self.0, mode, SCALE),
            })
        }
        #[inline]
        fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            Self($tan(self.0, mode, SCALE))
        }
        #[inline]
        fn atan_impl(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => $atan(self.0, mode, SCALE),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => $atan(self.0, mode, SCALE),
            })
        }
        #[inline]
        fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            Self($atan(self.0, mode, SCALE))
        }
    };
}

// ── D57 — forward Tang band at 44..=56 (sin/cos/atan), narrow-GUARD
// Series band at 18..=22 (sin/cos/tan/atan); inverse + hyper divert
// 18..=22 to their lookup kernels. ─────────────────────────────────────
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D57<SCALE> {
    // Forward family — `Series` runs the 18..=22 narrow-GUARD lookup or
    // the generic `wide_kernel`; `Tang` runs the 44..=56 band kernel
    // (sin/cos/atan only — tan has no 44..=56 Tang band).
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_sincos::sin_strict::<SCALE>(self.0, mode),
                _ => trig::wide_kernel::sin_strict_d57(self.0, mode, SCALE),
            },
            forward::Algorithm::Tang => {
                trig::lookup_d57_s44_56_sincos::sin_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sin_impl(mode)
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_sincos::cos_strict::<SCALE>(self.0, mode),
                _ => trig::wide_kernel::cos_strict_d57(self.0, mode, SCALE),
            },
            forward::Algorithm::Tang => {
                trig::lookup_d57_s44_56_sincos::cos_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cos_impl(mode)
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_sincos::tan_strict::<SCALE>(self.0, mode),
                _ => trig::wide_kernel::tan_strict_d57(self.0, mode, SCALE),
            },
            // tan has no D57 Tang band; the arm is dead-arm-eliminated
            // (forwards to the generic kernel for exhaustiveness).
            forward::Algorithm::Tang => trig::wide_kernel::tan_strict_d57(self.0, mode, SCALE),
        })
    }
    #[inline]
    fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tan_impl(mode)
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_atan::atan_strict::<SCALE>(self.0, mode),
                _ => trig::wide_kernel::atan_strict_d57(self.0, mode, SCALE),
            },
            forward::Algorithm::Tang => {
                trig::lookup_d57_s44_56_atan::atan_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan_impl(mode)
    }

    // Inverse family — `Atan` realised by the 18..=22 lookup or the
    // inherent shell.
    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<3, SCALE>(&self.0) {
            inverse::Algorithm::Atan => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_inverse::asin_strict::<SCALE>(self.0, mode),
                _ => return self.asin_strict_with(mode),
            },
        })
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asin_impl(mode)
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<3, SCALE>(&self.0) {
            inverse::Algorithm::Atan => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_inverse::acos_strict::<SCALE>(self.0, mode),
                _ => return self.acos_strict_with(mode),
            },
        })
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acos_impl(mode)
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<3, SCALE>(&self.0) {
            inverse::Algorithm::Atan => match SCALE {
                18..=22 => {
                    trig::lookup_d57_s18_22_inverse::atan2_strict::<SCALE>(self.0, other.0, mode)
                }
                _ => return self.atan2_strict_with(other, mode),
            },
        })
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan2_impl(other, mode)
    }

    // Hyperbolics — `ExpIdentity` realised by the 18..=22 lookup or the
    // inherent shell.
    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<3, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_hyper::sinh_strict::<SCALE>(self.0, mode),
                _ => return self.sinh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sinh_impl(mode)
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<3, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_hyper::cosh_strict::<SCALE>(self.0, mode),
                _ => return self.cosh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cosh_impl(mode)
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<3, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                18..=22 => trig::lookup_d57_s18_22_hyper::tanh_strict::<SCALE>(self.0, mode),
                _ => return self.tanh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tanh_impl(mode)
    }

    wide_trig_extra_inherent!();
}

// ── D76 — width default (no bands) ─────────────────────────────────────
#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D76<SCALE> {
    wide_trig_forward_series!(
        4,
        trig::wide_kernel::sin_strict_d76,
        trig::wide_kernel::cos_strict_d76,
        trig::wide_kernel::tan_strict_d76,
        trig::wide_kernel::atan_strict_d76
    );
    wide_trig_inverse_inherent!(4);
    wide_trig_hyper_inherent!(4);
    wide_trig_extra_inherent!();
}

// ── D115 — forward via wide_kernel; sinh/cosh/tanh divert SCALE
// 50..=60 to the Tang-style hyper lookup. ──────────────────────────────
#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D115<SCALE> {
    wide_trig_forward_series!(
        6,
        trig::wide_kernel::sin_strict_d115,
        trig::wide_kernel::cos_strict_d115,
        trig::wide_kernel::tan_strict_d115,
        trig::wide_kernel::atan_strict_d115
    );
    wide_trig_inverse_inherent!(6);

    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<6, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                50..=60 => trig::lookup_d115_s57_hyper::sinh_strict::<SCALE>(self.0, mode),
                _ => return self.sinh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sinh_impl(mode)
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<6, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                50..=60 => trig::lookup_d115_s57_hyper::cosh_strict::<SCALE>(self.0, mode),
                _ => return self.cosh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cosh_impl(mode)
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<6, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                50..=60 => trig::lookup_d115_s57_hyper::tanh_strict::<SCALE>(self.0, mode),
                _ => return self.tanh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tanh_impl(mode)
    }

    wide_trig_extra_inherent!();
}

// ── D153 — forward sin/cos/tan/atan divert SCALE 70..=82 (Tang);
// sinh/cosh/tanh divert the same band. ─────────────────────────────────
#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D153<SCALE> {
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::sin_strict_d153(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d153_s70_82_sincos::sin_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sin_impl(mode)
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::cos_strict_d153(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d153_s70_82_sincos::cos_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cos_impl(mode)
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::tan_strict_d153(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d153_s70_82_sincos::tan_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tan_impl(mode)
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::atan_strict_d153(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d153_s70_82_atan::atan_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan_impl(mode)
    }

    wide_trig_inverse_inherent!(8);

    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<8, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                70..=82 => trig::lookup_d153_s70_82_hyper::sinh_strict::<SCALE>(self.0, mode),
                _ => return self.sinh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sinh_impl(mode)
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<8, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                70..=82 => trig::lookup_d153_s70_82_hyper::cosh_strict::<SCALE>(self.0, mode),
                _ => return self.cosh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cosh_impl(mode)
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<8, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                70..=82 => trig::lookup_d153_s70_82_hyper::tanh_strict::<SCALE>(self.0, mode),
                _ => return self.tanh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tanh_impl(mode)
    }

    wide_trig_extra_inherent!();
}

// ── D230 — width default (no bands) ────────────────────────────────────
#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D230<SCALE> {
    wide_trig_forward_series!(
        12,
        trig::wide_kernel::sin_strict_d230,
        trig::wide_kernel::cos_strict_d230,
        trig::wide_kernel::tan_strict_d230,
        trig::wide_kernel::atan_strict_d230
    );
    wide_trig_inverse_inherent!(12);
    wide_trig_hyper_inherent!(12);
    wide_trig_extra_inherent!();
}

// ── D307 — forward sin/cos/tan/atan divert SCALE 140..=160 (Tang);
// sinh/cosh/tanh divert the same band. ─────────────────────────────────
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D307<SCALE> {
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::sin_strict_d307(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d307_s140_160_sincos::sin_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sin_impl(mode)
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::cos_strict_d307(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d307_s140_160_sincos::cos_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cos_impl(mode)
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::tan_strict_d307(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d307_s140_160_sincos::tan_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tan_impl(mode)
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::atan_strict_d307(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d307_s140_160_atan::atan_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan_impl(mode)
    }

    wide_trig_inverse_inherent!(16);

    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<16, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                140..=160 => trig::lookup_d307_s140_160_hyper::sinh_strict::<SCALE>(self.0, mode),
                _ => return self.sinh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sinh_impl(mode)
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<16, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                140..=160 => trig::lookup_d307_s140_160_hyper::cosh_strict::<SCALE>(self.0, mode),
                _ => return self.cosh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cosh_impl(mode)
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<16, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                140..=160 => trig::lookup_d307_s140_160_hyper::tanh_strict::<SCALE>(self.0, mode),
                _ => return self.tanh_strict_with(mode),
            },
        })
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tanh_impl(mode)
    }

    wide_trig_extra_inherent!();
}

// ── D462 — forward sin/cos/tan/atan divert SCALE 225..=235 (Tang);
// the hyperbolics keep the inherent shells (Tang hyper slot lost here). ─
#[cfg(any(feature = "d462", feature = "x-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D462<SCALE> {
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::sin_strict_d462(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d462_s225_235_sincos::sin_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.sin_impl(mode)
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::cos_strict_d462(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d462_s225_235_sincos::cos_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.cos_impl(mode)
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::tan_strict_d462(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d462_s225_235_sincos::tan_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.tan_impl(mode)
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => trig::wide_kernel::atan_strict_d462(self.0, mode, SCALE),
            forward::Algorithm::Tang => {
                trig::lookup_d462_s225_235_atan::atan_strict::<SCALE>(self.0, mode)
            }
        })
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan_impl(mode)
    }

    wide_trig_inverse_inherent!(24);
    wide_trig_hyper_inherent!(24);
    wide_trig_extra_inherent!();
}

// ── D616 — width default (no bands) ────────────────────────────────────
#[cfg(any(feature = "d616", feature = "x-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D616<SCALE> {
    wide_trig_forward_series!(
        32,
        trig::wide_kernel::sin_strict_d616,
        trig::wide_kernel::cos_strict_d616,
        trig::wide_kernel::tan_strict_d616,
        trig::wide_kernel::atan_strict_d616
    );
    wide_trig_inverse_inherent!(32);
    wide_trig_hyper_inherent!(32);
    wide_trig_extra_inherent!();
}

// ── D924 — width default (no bands) ────────────────────────────────────
#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D924<SCALE> {
    wide_trig_forward_series!(
        48,
        trig::wide_kernel::sin_strict_d924,
        trig::wide_kernel::cos_strict_d924,
        trig::wide_kernel::tan_strict_d924,
        trig::wide_kernel::atan_strict_d924
    );
    wide_trig_inverse_inherent!(48);
    wide_trig_hyper_inherent!(48);
    wide_trig_extra_inherent!();
}

// ── D1232 — width default (no bands) ───────────────────────────────────
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D1232<SCALE> {
    wide_trig_forward_series!(
        64,
        trig::wide_kernel::sin_strict_d1232,
        trig::wide_kernel::cos_strict_d1232,
        trig::wide_kernel::tan_strict_d1232,
        trig::wide_kernel::atan_strict_d1232
    );
    wide_trig_inverse_inherent!(64);
    wide_trig_hyper_inherent!(64);
    wide_trig_extra_inherent!();
}
