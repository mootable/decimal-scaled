// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Trigonometric policy — the per-`(N, SCALE)` algorithm matchers.
//!
//! `D<Int<N>, SCALE>::sin_strict_with(mode)` (and the cos / tan / atan /
//! asin / acos / atan2 / hyperbolic / angle-conversion siblings) delegate
//! to the per-function `dispatch` fns at the end of this file, which
//! resolve each family's canonical
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
//!   `trig_series_2limb` on the narrow tier, the narrow-GUARD `lookup_*` slots at
//!   the low bands) and `Tang` (Tang 1991 table-driven argument reduction
//!   + residual Taylor, on the benched mid/deep SCALE bands).
//! - **inverse (asin / acos / atan2)** — `Atan` (atan-of-ratio with
//!   half-angle reduction / quadrant dispatch; the wide tiers compose the
//!   inherent `*_strict_with` shells, the D57 18..=22 band uses the
//!   narrow-GUARD lookup, D38 borrows D57 / runs `trig_series_2limb`).
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
//! `Fixed`-256 in `trig::trig_series_2limb`, the tier-generic `*_series` kernels
//! over `WideTrigCore` in `algos::support::wide_trig_core`, Tang/narrow-GUARD
//! bands in the `trig::lookup_*` kernels). Collapsing those kernel *bodies* to one
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
use crate::types::widths::D18;

// (TrigPolicy trait removed — see the per-function `dispatch` fns at end of file)

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
        /// The generic default; realised by the tier-generic `*_series`
        /// kernels in `algos::support::wide_trig_core` (wide),
        /// `trig::trig_series_2limb` (narrow), and the narrow-GUARD `lookup_*`
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
        /// `*_schoolbook` -- the naive textbook reference (Maclaurin
        /// series on the range-reduced residue for sin/cos, `sin/cos`
        /// for tan, the reduced arctan series for atan). UNROUTED:
        /// `select` never returns it; registered for the correctness
        /// reference + A/B microbench partner. Realised by the generic
        /// `trig_schoolbook::*_schoolbook` (wide) and
        /// `trig_schoolbook::*_schoolbook_narrow` (narrow) kernels.
        #[allow(dead_code)]
        Schoolbook,
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
            // D462 (`Int<24>`) — narrow-GUARD reclaim wins across the FULL
            // SCALE range (0..=461). At D462 the `Tang` arm realises as
            // `sincos_narrow::*_with_taylor` (GUARD=10) / `atan_narrow`
            // (GUARD=12) — a narrowed-GUARD Taylor reclaim that runs the
            // same `sin_fixed`/`cos_fixed`/`atan_fixed` cores but at the
            // band's smaller working width (vs the tier's default
            // GUARD=30). The 2026-05-28 bisection (`trig_wide_tang_bisect`)
            // showed narrow_g10/g12 wins by 1.0×–2.4× at every probed
            // s0/50/100/150/180/200/210/218/225/230/235/240/260/290/330/400/450
            // across sin/cos/tan/atan (3 inputs × 6 rounding modes,
            // bit-identical to Series), refuting the prior 225..=235
            // point-range gate (Audit Finding #2, untested). One inner
            // outlier (sin s230, Series +1.11×) is bench noise: neighbours
            // s225 / s235 / s240 all show narrow wins for sin.
            #[cfg(any(feature = "d462", feature = "x-wide"))]
            (24, 0..=461) => Select::ByAlgorithm(Algorithm::Tang),
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
            // D462 — same full-range narrow-GUARD reclaim as [`select`];
            // see that arm's comment for the bisection evidence.
            #[cfg(any(feature = "d462", feature = "x-wide"))]
            (24, 0..=461) => Select::ByAlgorithm(Algorithm::Tang),
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
        /// `trig_series_2limb` (D38 without D57).
        Atan,
        /// `*_schoolbook` -- the naive textbook reference (asin via
        /// `atan(x / sqrt(1 - x^2))` + half-angle reduction, acos via
        /// `pi/2 - asin`, atan2 via quadrant-resolved `atan(y/x)`).
        /// UNROUTED: `select` never returns it; registered for the
        /// correctness reference + A/B microbench partner. Realised by
        /// the generic `inverse_schoolbook::*_schoolbook` (wide) and
        /// `inverse_schoolbook::*_schoolbook_narrow` (narrow) kernels.
        #[allow(dead_code)]
        Schoolbook,
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
        /// `lookup_*_hyper` kernels (benched bands), and by `trig_series_2limb`
        /// on the narrow tier.
        ExpIdentity,
        /// `*_schoolbook` -- the naive textbook reference (the exp
        /// identity `(e^x +- e^-x)/2` for sinh/cosh, the ratio for
        /// tanh). UNROUTED: `select` never returns it; registered for
        /// the correctness reference + A/B microbench partner. Realised
        /// by the generic `hyper_schoolbook::*_schoolbook` (wide) and
        /// `hyper_schoolbook::*_schoolbook_narrow` (narrow) kernels.
        #[allow(dead_code)]
        Schoolbook,
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
// Forward-family SCALE-derived work-rung routing (sin / cos / tan)
//
// The wide tiers historically ran every forward-trig kernel in the
// TIER-FIXED work integer `C::W` regardless of scale — at SCALE 0 the
// working scale is only `GUARD = 30` digits yet D1232 computed sin in
// `Int<176>`. The helpers below key the work integer on the working
// scale instead: a const-folded rung match (shared selector
// `policy::work_rung::trig_rung`) monomorphises the ONE generic kernel
// (`wide_trig_core::{sin,cos,tan}_series_g`) at the narrowest valid
// `Int<K>`, with dead arms eliminated. A rung-INTERNAL second axis —
// NOT in the `select` verdict, NOT on `Algorithm` (the `policy::ln`
// Tang-rung shape).
//
// THE VALUE AXIS (trig-specific — `ln` has no analogue): the mod-τ
// range reduction loses one guard digit per integer digit of `|x|`
// (`r = v − q·τ` with `τ` correctly rounded ⇒ error ≈ `q = |x|/2π`
// working units), so a small rung is only valid where the argument's
// integer-digit count provably fits the rung's budget. The runtime
// magnitude gate below (`|x| < 10^D_BUDGET`, one bit-length compare —
// the irreducible ByValue residue) admits the continuous everyday
// region to the rung and routes everything larger to the EXACT
// pre-rung tier-width path (`sin_series` / `sincos_narrow` /
// `tan_series`), bit-identical to today at every out-of-budget value.
// Within the gate the rung value is bit-identical to the tier value by
// construction (width-agnostic integer ops; the only divergence
// surface is the Ziv escalation cap, budgeted analytically in
// `work_rung::TRIG_MARGIN` and walled by the golden gate).
// ══════════════════════════════════════════════════════════════════════
/// The const-folded work-rung match: one macro emits the 13-arm match
/// per kernel call so the ladder stays single-source (the
/// `policy::ln::tang_at_rung` shape). `$sel` is the policy-internal
/// rung selector (`trig_rung`, …) resolved in the caller's scope;
/// `$kernel` the rung-generic kernel (imported by the calling module).
#[cfg(feature = "_wide-support")]
macro_rules! rung_match {
    ($sel:ident, $C:ty, $SCALE:ident, $kernel:ident, [$($k:tt)*], $($arg:expr),+ $(,)?) => {
        match const { $sel::<$C, $SCALE>() } {
            crate::policy::work_rung::Rung::W3 => $kernel::<$C, crate::int::types::Int<3>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W4 => $kernel::<$C, crate::int::types::Int<4>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W6 => $kernel::<$C, crate::int::types::Int<6>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W8 => $kernel::<$C, crate::int::types::Int<8>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W12 => $kernel::<$C, crate::int::types::Int<12>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W16 => $kernel::<$C, crate::int::types::Int<16>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W24 => $kernel::<$C, crate::int::types::Int<24>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W32 => $kernel::<$C, crate::int::types::Int<32>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W48 => $kernel::<$C, crate::int::types::Int<48>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W64 => $kernel::<$C, crate::int::types::Int<64>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W96 => $kernel::<$C, crate::int::types::Int<96>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W128 => $kernel::<$C, crate::int::types::Int<128>, $($k)*>($($arg),+),
            crate::policy::work_rung::Rung::W176 => $kernel::<$C, crate::int::types::Int<176>, $($k)*>($($arg),+),
        }
    };
}

#[cfg(feature = "_wide-support")]
pub(crate) mod forward_rung {
    use super::super::work_rung::{trig_rung, D_BUDGET};
    use crate::algos::support::wide_trig_core::{
        atan_series_g, cos_series_g, sin_series_g, tan_series_g, WideTrigCore,
    };
    use crate::int::types::compute_limbs::ComputeLimbs;
    use crate::int::types::traits::BigInt;
    use crate::support::rounding::RoundingMode;

    /// `true` iff `|x| < ~10^BUDGET` — a rung's admitted magnitude
    /// region. Conservative bit-length test (`332_192/100_000 <
    /// log2(10)`): never admits a value at or beyond `10^(SCALE +
    /// BUDGET)` raw units, so an admitted argument's integer digits
    /// provably fit the rung's budget; the sliver it under-admits just
    /// below the boundary takes the (correct, slower) tier path. One
    /// compare against a compile-time constant. Shared by the forward /
    /// inverse / hyperbolic rung gates (each passes its family's
    /// budget).
    #[inline]
    pub(in crate::policy::trig) fn in_budget<C: WideTrigCore, const SCALE: u32, const BUDGET: u32>(
        raw: &C::Storage,
    ) -> bool {
        let bl = crate::algos::exp::exp_generic::bit_length::<C::Storage>(*raw) as u64;
        bl * 100_000 <= ((SCALE + BUDGET) as u64) * 332_192
    }

    /// `sin_strict` at the work rung; out-of-budget magnitudes fall to
    /// the tier-width narrow-GUARD kernel (at `GUARD = C::GUARD` that is
    /// value-identical to `sin_series` — same directed narrowing, same
    /// extremum adjust; the zero pin is an exact identity).
    #[inline]
    pub(crate) fn sin_strict<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage {
        if in_budget::<C, SCALE, D_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, sin_series_g, [SCALE, GUARD], raw, mode)
        } else {
            crate::algos::trig::sincos_narrow::sin_narrow_with_taylor::<C, SCALE, GUARD>(raw, mode)
        }
    }

    /// `cos_strict` at the work rung — see [`sin_strict`].
    #[inline]
    pub(crate) fn cos_strict<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage {
        if in_budget::<C, SCALE, D_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, cos_series_g, [SCALE, GUARD], raw, mode)
        } else {
            crate::algos::trig::sincos_narrow::cos_narrow_with_taylor::<C, SCALE, GUARD>(raw, mode)
        }
    }

    /// `tan_strict` at the work rung. `SUB_GUARD` selects the tier-GUARD
    /// Series probe shape (`tan_series`, lift minus the guard already
    /// paid) vs the narrow-band shape (full lift); the out-of-budget
    /// fallback is the matching pre-rung kernel, bit-identical to today.
    #[inline]
    pub(crate) fn tan_strict<
        C: WideTrigCore,
        const SCALE: u32,
        const GUARD: u32,
        const NEAR_POLE: bool,
        const SUB_GUARD: bool,
    >(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage
    where
        <C::W as BigInt>::Scratch: ComputeLimbs,
    {
        if in_budget::<C, SCALE, D_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, tan_series_g, [SCALE, GUARD, NEAR_POLE, SUB_GUARD], raw, mode)
        } else if SUB_GUARD {
            crate::algos::support::wide_trig_core::tan_series::<C, SCALE>(raw, mode)
        } else {
            crate::algos::trig::sincos_narrow::tan_narrow_with_taylor::<C, SCALE, GUARD, NEAR_POLE>(
                raw, mode,
            )
        }
    }

    /// `atan_strict` at the work rung. `DIRECTED` selects the tier-GUARD
    /// Ziv shape (`atan_series`) vs the narrow-band single-shot shape
    /// (`atan_narrow`); the out-of-budget fallback is the matching
    /// pre-rung kernel, bit-identical to today. atan's gate guards only
    /// REPRESENTATION (the lift of `|x|`'s integer digits into the rung
    /// — the reciprocal fold loses no precision per digit of `|x|`, so
    /// unlike sin/cos there is no reduction-error axis); `D_BUDGET`
    /// bounds it conservatively with the same continuous `|x| < ~10^8`
    /// region, budgeted inside `work_rung::TRIG_MARGIN`.
    #[inline]
    pub(crate) fn atan_strict<
        C: WideTrigCore,
        const SCALE: u32,
        const GUARD: u32,
        const DIRECTED: bool,
    >(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage {
        if in_budget::<C, SCALE, D_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, atan_series_g, [SCALE, GUARD, DIRECTED], raw, mode)
        } else if DIRECTED {
            crate::algos::support::wide_trig_core::atan_series::<C, SCALE>(raw, mode)
        } else {
            crate::algos::support::wide_trig_core::atan_narrow::<C, SCALE, GUARD>(raw, mode)
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// Inverse-family SCALE-derived work-rung routing (asin / acos / atan2)
//
// Same pattern as `forward_rung`: the const-folded rung match
// monomorphises the ONE rung-generic composition
// (`inverse_schoolbook::{asin,acos,atan2}_schoolbook_g`) at the
// narrowest valid `Int<K>`. The compositions are SINGLE-SHOT (one
// kernel evaluation at `w = SCALE + GUARD`, one narrowing, no Ziv
// escalation), so the rung only needs the base working width:
// `trig_rung`'s margin budgets it with room (intermediates stay
// ≤ 10^2w, covered by the ladder's bits-per-digit slack).
//
// THE VALUE AXIS: asin / acos have a bounded domain (|x| ≤ 1), so every
// in-domain argument is in-budget — the gate exists so an OUT-OF-DOMAIN
// magnitude beyond the rung's lift capacity still takes the tier path
// (reaching the identical domain panic there, never a wrapped lift).
// atan2 is the unbounded one: both operands are gated (the y·10^w /
// x·10^w divide numerators are the representation axis, exactly atan's
// — no precision loss per integer digit).
// ══════════════════════════════════════════════════════════════════════
#[cfg(feature = "_wide-support")]
pub(crate) mod inverse_rung {
    use super::super::work_rung::{trig_rung, D_BUDGET};
    use super::forward_rung::in_budget;
    use crate::algos::support::wide_trig_core::WideTrigCore;
    use crate::algos::trig::inverse_schoolbook::{
        acos_schoolbook_g, asin_schoolbook_g, atan2_schoolbook_g,
    };
    use crate::support::rounding::RoundingMode;

    /// `asin_strict` at the work rung; out-of-budget magnitudes (all out
    /// of domain) fall to the tier-width composition, bit-identical.
    #[inline]
    pub(crate) fn asin_strict<C: WideTrigCore, const SCALE: u32>(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage {
        if in_budget::<C, SCALE, D_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, asin_schoolbook_g, [SCALE], raw, mode)
        } else {
            crate::algos::trig::inverse_schoolbook::asin_schoolbook::<C, SCALE>(raw, mode)
        }
    }

    /// `acos_strict` at the work rung — see [`asin_strict`].
    #[inline]
    pub(crate) fn acos_strict<C: WideTrigCore, const SCALE: u32>(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage {
        if in_budget::<C, SCALE, D_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, acos_schoolbook_g, [SCALE], raw, mode)
        } else {
            crate::algos::trig::inverse_schoolbook::acos_schoolbook::<C, SCALE>(raw, mode)
        }
    }

    /// `atan2_strict` at the work rung; BOTH operands must be in budget
    /// (each is lifted to the rung and divides the other), else the
    /// tier-width composition runs, bit-identical.
    #[inline]
    pub(crate) fn atan2_strict<C: WideTrigCore, const SCALE: u32>(
        y_raw: C::Storage,
        x_raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage {
        if in_budget::<C, SCALE, D_BUDGET>(&y_raw) && in_budget::<C, SCALE, D_BUDGET>(&x_raw) {
            rung_match!(trig_rung, C, SCALE, atan2_schoolbook_g, [SCALE], y_raw, x_raw, mode)
        } else {
            crate::algos::trig::inverse_schoolbook::atan2_schoolbook::<C, SCALE>(y_raw, x_raw, mode)
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// Hyperbolic-family SCALE-derived work-rung routing (sinh / cosh / tanh)
//
// Same pattern as `forward_rung`, with the RESULT-MAGNITUDE value axis
// (`e^|x|` grows with `x`): the gate admits `|x| < 10^EXP_ARG_BUDGET`,
// bounding the result's integer-digit lift (`exp_result_int_digits`)
// and the exp kernel's internal `2^k` extension to what
// `work_rung::TRIG_MARGIN` budgets at the rung. The rung kernels
// (`hyper_schoolbook::*_schoolbook_g`) ALSO regime-split per Ziv probe
// on the exact `exp_peak_fits` model (the tier's own `hyper_fits_w`
// gate), lifting a too-deep probe to `C::Wexp` — so the analytic gate
// here governs only the fast path's hit-rate, never correctness.
// Out-of-budget magnitudes run the tier-width kernels, bit-identical.
// ══════════════════════════════════════════════════════════════════════
#[cfg(feature = "_wide-support")]
pub(crate) mod hyper_rung {
    use super::super::work_rung::{trig_rung, EXP_ARG_BUDGET};
    use super::forward_rung::in_budget;
    use crate::algos::support::wide_trig_core::WideTrigCore;
    use crate::algos::trig::hyper_schoolbook::{
        cosh_schoolbook_g, sinh_schoolbook_g, tanh_schoolbook_g,
    };
    use crate::int::types::compute_limbs::ComputeLimbs;
    use crate::int::types::traits::BigInt;
    use crate::support::rounding::RoundingMode;

    /// `sinh_strict` at the work rung; out-of-budget magnitudes fall to
    /// the tier-width exp-identity kernel, bit-identical.
    #[inline]
    pub(crate) fn sinh_strict<C: WideTrigCore, const SCALE: u32>(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage
    where
        <C::Wexp as BigInt>::Scratch: ComputeLimbs,
    {
        if in_budget::<C, SCALE, EXP_ARG_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, sinh_schoolbook_g, [SCALE], raw, mode)
        } else {
            crate::algos::trig::hyper_schoolbook::sinh_schoolbook::<C, SCALE>(raw, mode)
        }
    }

    /// `cosh_strict` at the work rung — see [`sinh_strict`].
    #[inline]
    pub(crate) fn cosh_strict<C: WideTrigCore, const SCALE: u32>(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage
    where
        <C::Wexp as BigInt>::Scratch: ComputeLimbs,
    {
        if in_budget::<C, SCALE, EXP_ARG_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, cosh_schoolbook_g, [SCALE], raw, mode)
        } else {
            crate::algos::trig::hyper_schoolbook::cosh_schoolbook::<C, SCALE>(raw, mode)
        }
    }

    /// `tanh_strict` at the work rung — see [`sinh_strict`].
    #[inline]
    pub(crate) fn tanh_strict<C: WideTrigCore, const SCALE: u32>(
        raw: C::Storage,
        mode: RoundingMode,
    ) -> C::Storage
    where
        <C::Wexp as BigInt>::Scratch: ComputeLimbs,
    {
        if in_budget::<C, SCALE, EXP_ARG_BUDGET>(&raw) {
            rung_match!(trig_rung, C, SCALE, tanh_schoolbook_g, [SCALE], raw, mode)
        } else {
            crate::algos::trig::hyper_schoolbook::tanh_schoolbook::<C, SCALE>(raw, mode)
        }
    }
}

// ══════════════════════════════════════════════════════════════════════
// D38 inverse-trig "borrow D57" dispatch strategy
//
// The D38 inverse-trig family (atan / asin / acos / atan2) is qualitatively
// faster routed through D57 than through D38's own `trig_series_2limb`
// adaptive-halvings path (~2× at SCALE 19; asin / acos / atan2 compose atan
// and inherit the gap). The strategy: widen D38 → D57, run the D57 kernel
// outside the SCALE 18..=22 lookup window (the lookup kernels cover that
// band), then narrow back to D38.
//
// Per the layering rule this is a dispatch *strategy* (`borrow_*`), not an
// algorithm — it carries no function prefix and lives in the policy layer,
// not `algos/`. It is consumed only by the D38-with-D57 `TrigPolicy` impl
// below.
//
// Correctness: every output here is bounded within `[-π, π]` (atan2) or
// smaller, so the narrowing `TryFrom` cannot fail on a representable input.
#[cfg(any(feature = "d57", feature = "wide"))]
mod borrow_d57 {
    use crate::algos::support::wide_trig_core;
    use crate::algos::trig;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;
    use crate::types::widths::wide_trig_d57;

    #[inline]
    fn narrow<const SCALE: u32>(raw_wide: Int<3>, op: &'static str) -> Int<2> {
        let wide = crate::D::<crate::int::types::Int<3>, SCALE>::from_bits(raw_wide);
        let r: crate::D<crate::int::types::Int<2>, SCALE> = wide.try_into().unwrap_or_else(|_| {
            panic!(
                "{op}: result out of range — produced {wide}, D38<{SCALE}> represents only |x| < 1.7e{}",
                38_i32 - SCALE as i32,
            )
        });
        r.0
    }

    #[inline]
    #[must_use]
    pub(crate) fn atan_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
        let widened: crate::D<crate::int::types::Int<3>, SCALE> = crate::D::<crate::int::types::Int<2>, SCALE>::from_bits(raw).into();
        let raw_wide = if matches!(SCALE, 18..=22) {
            wide_trig_core::atan_narrow::<wide_trig_d57::Core, SCALE, 10>(widened.0, mode)
        } else {
            wide_trig_core::atan_series::<wide_trig_d57::Core, SCALE>(widened.0, mode)
        };
        narrow::<SCALE>(raw_wide, "atan_strict")
    }

    #[inline]
    #[must_use]
    pub(crate) fn asin_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
        let widened: crate::D<crate::int::types::Int<3>, SCALE> = crate::D::<crate::int::types::Int<2>, SCALE>::from_bits(raw).into();
        let result_raw = if matches!(SCALE, 18..=22) {
            trig::inverse_tang_3limb_s18_22::asin_strict::<SCALE>(widened.0, mode)
        } else {
            widened.asin_strict_with(mode).0
        };
        narrow::<SCALE>(result_raw, "asin_strict")
    }

    #[inline]
    #[must_use]
    pub(crate) fn acos_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
        let widened: crate::D<crate::int::types::Int<3>, SCALE> = crate::D::<crate::int::types::Int<2>, SCALE>::from_bits(raw).into();
        let result_raw = if matches!(SCALE, 18..=22) {
            trig::inverse_tang_3limb_s18_22::acos_strict::<SCALE>(widened.0, mode)
        } else {
            widened.acos_strict_with(mode).0
        };
        narrow::<SCALE>(result_raw, "acos_strict")
    }

    #[inline]
    #[must_use]
    pub(crate) fn atan2_strict<const SCALE: u32>(
        y_raw: Int<2>,
        x_raw: Int<2>,
        mode: RoundingMode,
    ) -> Int<2> {
        let y_wide: crate::D<crate::int::types::Int<3>, SCALE> = crate::D::<crate::int::types::Int<2>, SCALE>::from_bits(y_raw).into();
        let x_wide: crate::D<crate::int::types::Int<3>, SCALE> = crate::D::<crate::int::types::Int<2>, SCALE>::from_bits(x_raw).into();
        let result_raw = if matches!(SCALE, 18..=22) {
            trig::inverse_tang_3limb_s18_22::atan2_strict::<SCALE>(y_wide.0, x_wide.0, mode)
        } else {
            y_wide.atan2_strict_with(x_wide, mode).0
        };
        narrow::<SCALE>(result_raw, "atan2_strict")
    }
}

// ══════════════════════════════════════════════════════════════════════
// Narrow tier (D18) — widen-to-D38 work width, then the chosen algorithm
//
// The narrow tier has no wide storage of its own; it widens into the D38
// `Fixed` work width and runs there (the `widen_to_work` strategy — a
// policy concern, not an algorithm). The forward family uses the
// `narrow_widen_*` helpers below; the hyperbolics, inverse hyperbolics,
// and angle conversions widen via the `TryInto` shape the inherent
// shells use.
// ══════════════════════════════════════════════════════════════════════

// ── widen → D38 → narrow dispatch helpers ───────────────────────────
//
// These are the `widen_to_work` dispatch strategy for the forward / inverse
// trig family: widen D18 → D38, run the hand-tuned `trig_series_2limb` kernel, narrow
// back. Per the layering rule they live in the policy layer, not `algos/`.

macro_rules! narrow_widen {
    ($name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        fn $name<const SCALE: u32>(v: $crate::D<$crate::int::types::Int<1>, SCALE>, mode: RoundingMode) -> $crate::D<$crate::int::types::Int<1>, SCALE> {
            let widened: $crate::D<$crate::int::types::Int<2>, SCALE> = v.into();
            let raw = trig::trig_series_2limb::$kernel::<SCALE>(widened.0, mode);
            $crate::D::<$crate::int::types::Int<2>, SCALE>::from_bits(raw).try_into().expect($err)
        }
    };
}

macro_rules! narrow_widen_with {
    ($name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        fn $name<const SCALE: u32>(
            v: $crate::D<$crate::int::types::Int<1>, SCALE>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> $crate::D<$crate::int::types::Int<1>, SCALE> {
            let widened: $crate::D<$crate::int::types::Int<2>, SCALE> = v.into();
            let raw = trig::trig_series_2limb::$kernel::<SCALE>(widened.0, working_digits, mode);
            $crate::D::<$crate::int::types::Int<2>, SCALE>::from_bits(raw).try_into().expect($err)
        }
    };
}

// `atan2` takes both `y` and `x`, widening each to D38 before delegating.
macro_rules! narrow_widen_binary {
    ($name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        fn $name<const SCALE: u32>(y: $crate::D<$crate::int::types::Int<1>, SCALE>, x: $crate::D<$crate::int::types::Int<1>, SCALE>, mode: RoundingMode) -> $crate::D<$crate::int::types::Int<1>, SCALE> {
            let y_wide: $crate::D<$crate::int::types::Int<2>, SCALE> = y.into();
            let x_wide: $crate::D<$crate::int::types::Int<2>, SCALE> = x.into();
            let raw = trig::trig_series_2limb::$kernel::<SCALE>(y_wide.0, x_wide.0, mode);
            $crate::D::<$crate::int::types::Int<2>, SCALE>::from_bits(raw).try_into().expect($err)
        }
    };
}

macro_rules! narrow_widen_binary_with {
    ($name:ident, $kernel:ident, $err:literal) => {
        #[inline]
        #[must_use]
        fn $name<const SCALE: u32>(
            y: $crate::D<$crate::int::types::Int<1>, SCALE>,
            x: $crate::D<$crate::int::types::Int<1>, SCALE>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> $crate::D<$crate::int::types::Int<1>, SCALE> {
            let y_wide: $crate::D<$crate::int::types::Int<2>, SCALE> = y.into();
            let x_wide: $crate::D<$crate::int::types::Int<2>, SCALE> = x.into();
            let raw = trig::trig_series_2limb::$kernel::<SCALE>(y_wide.0, x_wide.0, working_digits, mode);
            $crate::D::<$crate::int::types::Int<2>, SCALE>::from_bits(raw).try_into().expect($err)
        }
    };
}

narrow_widen!(sin_strict_d18, sin_strict, "sin_strict: result out of range");
narrow_widen_with!(sin_with_d18, sin_with, "sin_with: result out of range");
narrow_widen!(cos_strict_d18, cos_strict, "cos_strict: result out of range");
narrow_widen_with!(cos_with_d18, cos_with, "cos_with: result out of range");
narrow_widen!(tan_strict_d18, tan_strict, "tan_strict: result out of range");
narrow_widen_with!(tan_with_d18, tan_with, "tan_with: result out of range");
narrow_widen!(atan_strict_d18, atan_strict, "atan_strict: result out of range");
narrow_widen_with!(atan_with_d18, atan_with, "atan_with: result out of range");
narrow_widen!(asin_strict_d18, asin_strict, "asin_strict: result out of range");
narrow_widen_with!(asin_with_d18, asin_with, "asin_with: result out of range");
narrow_widen!(acos_strict_d18, acos_strict, "acos_strict: result out of range");
narrow_widen_with!(acos_with_d18, acos_with, "acos_with: result out of range");
narrow_widen_binary!(atan2_strict_d18, atan2_strict, "atan2_strict: result out of range");
narrow_widen_binary_with!(atan2_with_d18, atan2_with, "atan2_with: result out of range");

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
        impl<const SCALE: u32> $T<SCALE> {
            #[inline]
            pub(crate) fn policy_sin(self, mode: RoundingMode) -> Self {
                $sin_s(self, mode)
            }
            #[inline]
            pub(crate) fn policy_sin_with(self, wd: u32, mode: RoundingMode) -> Self {
                $sin_w(self, wd, mode)
            }
            #[inline]
            pub(crate) fn policy_cos(self, mode: RoundingMode) -> Self {
                $cos_s(self, mode)
            }
            #[inline]
            pub(crate) fn policy_cos_with(self, wd: u32, mode: RoundingMode) -> Self {
                $cos_w(self, wd, mode)
            }
            #[inline]
            pub(crate) fn policy_tan(self, mode: RoundingMode) -> Self {
                $tan_s(self, mode)
            }
            #[inline]
            pub(crate) fn policy_tan_with(self, wd: u32, mode: RoundingMode) -> Self {
                $tan_w(self, wd, mode)
            }
            #[inline]
            pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
                $atan_s(self, mode)
            }
            #[inline]
            pub(crate) fn policy_atan_with(self, wd: u32, mode: RoundingMode) -> Self {
                $atan_w(self, wd, mode)
            }
            #[inline]
            pub(crate) fn policy_asin(self, mode: RoundingMode) -> Self {
                $asin_s(self, mode)
            }
            #[inline]
            pub(crate) fn policy_asin_with(self, wd: u32, mode: RoundingMode) -> Self {
                $asin_w(self, wd, mode)
            }
            #[inline]
            pub(crate) fn policy_acos(self, mode: RoundingMode) -> Self {
                $acos_s(self, mode)
            }
            #[inline]
            pub(crate) fn policy_acos_with(self, wd: u32, mode: RoundingMode) -> Self {
                $acos_w(self, wd, mode)
            }
            #[inline]
            pub(crate) fn policy_atan2(self, other: Self, mode: RoundingMode) -> Self {
                $atan2_s(self, other, mode)
            }
            #[inline]
            pub(crate) fn policy_atan2_with(self, other: Self, wd: u32, mode: RoundingMode) -> Self {
                $atan2_w(self, other, wd, mode)
            }

            // Hyperbolics and angle conversions widen → D38 → narrow.
            #[inline]
            pub(crate) fn policy_sinh(self, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_sinh_with(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_cosh(self, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_cosh_with(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_tanh(self, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_tanh_with(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_asinh(self, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_asinh_with(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_acosh(self, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_acosh_with(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_atanh(self, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_atanh_with(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
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
            pub(crate) fn policy_to_degrees(self, mode: RoundingMode) -> Self {
                Self::from_bits(crate::policy::to_degrees::dispatch::<_, SCALE>(self.to_bits(), mode))
            }
            #[inline]
            pub(crate) fn policy_to_degrees_with(self, wd: u32, mode: RoundingMode) -> Self {
                let _ = wd;
                Self::from_bits(crate::policy::to_degrees::dispatch::<_, SCALE>(self.to_bits(), mode))
            }
            #[inline]
            pub(crate) fn policy_to_radians(self, mode: RoundingMode) -> Self {
                Self::from_bits(crate::policy::to_radians::dispatch::<_, SCALE>(self.to_bits(), mode))
            }
            #[inline]
            pub(crate) fn policy_to_radians_with(self, wd: u32, mode: RoundingMode) -> Self {
                let _ = wd;
                Self::from_bits(crate::policy::to_radians::dispatch::<_, SCALE>(self.to_bits(), mode))
            }
        }
    };
}

impl_narrow_trig!(
    D18,
    sin_strict_d18,
    sin_with_d18,
    cos_strict_d18,
    cos_with_d18,
    tan_strict_d18,
    tan_with_d18,
    atan_strict_d18,
    atan_with_d18,
    asin_strict_d18,
    asin_with_d18,
    acos_strict_d18,
    acos_with_d18,
    atan2_strict_d18,
    atan2_with_d18
);

// ══════════════════════════════════════════════════════════════════════
// D38 — narrow `Fixed`-256 kernels (`trig_series_2limb`), with the inverse family
// borrowing D57 when it is available.
//
// N==2 always selects `Series` (forward) / `Atan` (inverse) /
// `ExpIdentity` (hyper); each `match algo` is exhaustive over the gated
// real variants and dead-arm-eliminated. The forward family runs the
// bespoke `trig_series_2limb` series kernel directly (it beats the widen-and-back
// path ~2× since the 0.4.2 MG-routed `Fixed` primitives). The inverse
// family borrows D57 when present (the wide_kernel atan is ~2× faster than
// the `trig_series_2limb` adaptive-halvings path; asin/acos/atan2 compose atan, so
// they inherit the gap) and runs `trig_series_2limb` without D57.
// ══════════════════════════════════════════════════════════════════════

/// D38 hyperbolic + angle-conversion methods share one `Fixed` core
/// regardless of whether the forward / inverse trig path borrows D57.
macro_rules! d38_hyperbolic_and_angle {
    () => {
        #[inline]
        pub(crate) fn policy_sinh(self, mode: RoundingMode) -> Self {
            Self(match hyper::resolve::<2, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => trig::trig_series_2limb::sinh_strict::<SCALE>(self.0, mode),
                #[allow(dead_code)]
                hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::sinh_schoolbook_narrow::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_sinh_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::sinh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_cosh(self, mode: RoundingMode) -> Self {
            Self(match hyper::resolve::<2, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => trig::trig_series_2limb::cosh_strict::<SCALE>(self.0, mode),
                #[allow(dead_code)]
                hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::cosh_schoolbook_narrow::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_cosh_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::cosh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_tanh(self, mode: RoundingMode) -> Self {
            Self(match hyper::resolve::<2, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => trig::trig_series_2limb::tanh_strict::<SCALE>(self.0, mode),
                #[allow(dead_code)]
                hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::tanh_schoolbook_narrow::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_tanh_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::tanh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_asinh(self, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::asinh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_asinh_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::asinh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_acosh(self, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::acosh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_acosh_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::acosh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_atanh(self, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::atanh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_atanh_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::atanh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_to_degrees(self, mode: RoundingMode) -> Self {
            Self::from_bits(crate::policy::to_degrees::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
        #[inline]
        pub(crate) fn policy_to_degrees_with(self, wd: u32, mode: RoundingMode) -> Self {
            let _ = wd;
            Self::from_bits(crate::policy::to_degrees::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
        #[inline]
        pub(crate) fn policy_to_radians(self, mode: RoundingMode) -> Self {
            Self::from_bits(crate::policy::to_radians::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
        #[inline]
        pub(crate) fn policy_to_radians_with(self, wd: u32, mode: RoundingMode) -> Self {
            let _ = wd;
            Self::from_bits(crate::policy::to_radians::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
    };
}

/// D38 forward family — always `Series` on the `trig_series_2limb` kernel. The
/// gated `Tang` arm is dead-arm-eliminated (N==2 never selects it; it
/// forwards to the series kernel so the `match` stays exhaustive).
macro_rules! d38_forward_fixed {
    () => {
        #[inline]
        pub(crate) fn policy_sin(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<2, SCALE>(&self.0) {
                forward::Algorithm::Series => trig::trig_series_2limb::sin_strict::<SCALE>(self.0, mode),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => trig::trig_series_2limb::sin_strict::<SCALE>(self.0, mode),
                #[allow(dead_code)]
                forward::Algorithm::Schoolbook => trig::trig_schoolbook::sin_schoolbook_narrow::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_sin_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::sin_with::<SCALE>(self.0, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_cos(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<2, SCALE>(&self.0) {
                forward::Algorithm::Series => trig::trig_series_2limb::cos_strict::<SCALE>(self.0, mode),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => trig::trig_series_2limb::cos_strict::<SCALE>(self.0, mode),
                #[allow(dead_code)]
                forward::Algorithm::Schoolbook => trig::trig_schoolbook::cos_schoolbook_narrow::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_cos_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::cos_with::<SCALE>(self.0, wd, mode))
        }
        #[inline]
        pub(crate) fn policy_tan(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve_tan::<2, SCALE>(&self.0) {
                forward::Algorithm::Series => trig::trig_series_2limb::tan_strict::<SCALE>(self.0, mode),
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => trig::trig_series_2limb::tan_strict::<SCALE>(self.0, mode),
                #[allow(dead_code)]
                forward::Algorithm::Schoolbook => trig::trig_schoolbook::tan_schoolbook_narrow::<SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_tan_with(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::trig_series_2limb::tan_with::<SCALE>(self.0, wd, mode))
        }
    };
}

// D38 with D57 present — forward via `trig_series_2limb`, inverse borrows D57.
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    d38_forward_fixed!();

    #[inline]
    pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => borrow_d57::atan_strict::<SCALE>(self.0, mode),
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::trig_schoolbook::atan_schoolbook_narrow::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(borrow_d57::atan_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    pub(crate) fn policy_asin(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => borrow_d57::asin_strict::<SCALE>(self.0, mode),
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::asin_schoolbook_narrow::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_asin_with(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(borrow_d57::asin_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    pub(crate) fn policy_acos(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => borrow_d57::acos_strict::<SCALE>(self.0, mode),
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::acos_schoolbook_narrow::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_acos_with(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(borrow_d57::acos_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    pub(crate) fn policy_atan2(self, other: Self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => {
                borrow_d57::atan2_strict::<SCALE>(self.0, other.0, mode)
            }
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::atan2_schoolbook_narrow::<SCALE>(self.0, other.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan2_with(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        Self(borrow_d57::atan2_strict::<SCALE>(self.0, other.0, mode))
    }

    d38_hyperbolic_and_angle!();
}

// D38 without D57 — forward + inverse both on `trig_series_2limb`.
#[cfg(not(any(feature = "d57", feature = "wide")))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    d38_forward_fixed!();

    #[inline]
    pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::trig_series_2limb::atan_strict::<SCALE>(self.0, mode),
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::trig_schoolbook::atan_schoolbook_narrow::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan_with(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::trig_series_2limb::atan_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    pub(crate) fn policy_asin(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::trig_series_2limb::asin_strict::<SCALE>(self.0, mode),
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::asin_schoolbook_narrow::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_asin_with(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::trig_series_2limb::asin_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    pub(crate) fn policy_acos(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => trig::trig_series_2limb::acos_strict::<SCALE>(self.0, mode),
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::acos_schoolbook_narrow::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_acos_with(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::trig_series_2limb::acos_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    pub(crate) fn policy_atan2(self, other: Self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<2, SCALE>(&self.0) {
            inverse::Algorithm::Atan => {
                trig::trig_series_2limb::atan2_strict::<SCALE>(self.0, other.0, mode)
            }
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::atan2_schoolbook_narrow::<SCALE>(self.0, other.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan2_with(self, other: Self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::trig_series_2limb::atan2_with::<SCALE>(self.0, other.0, wd, mode))
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
#[allow(unused_macros)]
macro_rules! wide_trig_inverse_inherent {
    ($N:literal, $Core:ty) => {
        #[inline]
        pub(crate) fn policy_asin(self, mode: RoundingMode) -> Self {
            match inverse::resolve::<$N, SCALE>(&self.0) {
                inverse::Algorithm::Atan => Self(inverse_rung::asin_strict::<$Core, SCALE>(self.0, mode)),
                #[allow(dead_code)]
                inverse::Algorithm::Schoolbook => Self(crate::algos::trig::inverse_schoolbook::asin_schoolbook::<$Core, SCALE>(self.0, mode)),
            }
        }
        #[inline]
        pub(crate) fn policy_asin_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(inverse_rung::asin_strict::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_acos(self, mode: RoundingMode) -> Self {
            match inverse::resolve::<$N, SCALE>(&self.0) {
                inverse::Algorithm::Atan => Self(inverse_rung::acos_strict::<$Core, SCALE>(self.0, mode)),
                #[allow(dead_code)]
                inverse::Algorithm::Schoolbook => Self(crate::algos::trig::inverse_schoolbook::acos_schoolbook::<$Core, SCALE>(self.0, mode)),
            }
        }
        #[inline]
        pub(crate) fn policy_acos_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(inverse_rung::acos_strict::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_atan2(self, other: Self, mode: RoundingMode) -> Self {
            match inverse::resolve::<$N, SCALE>(&self.0) {
                inverse::Algorithm::Atan => Self(inverse_rung::atan2_strict::<$Core, SCALE>(self.0, other.0, mode)),
                #[allow(dead_code)]
                inverse::Algorithm::Schoolbook => Self(crate::algos::trig::inverse_schoolbook::atan2_schoolbook::<$Core, SCALE>(self.0, other.0, mode)),
            }
        }
        #[inline]
        pub(crate) fn policy_atan2_with(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
            Self(inverse_rung::atan2_strict::<$Core, SCALE>(self.0, other.0, mode))
        }
    };
}

/// asinh / acosh / atanh + angle conversions — inherent shells, no bands.
#[allow(unused_macros)]
macro_rules! wide_trig_extra_inherent {
    ($N:literal, $Core:ty) => {
        #[inline]
        pub(crate) fn policy_asinh(self, mode: RoundingMode) -> Self {
            Self(crate::algos::trig::hyper_schoolbook::asinh_schoolbook::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_asinh_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(crate::algos::trig::hyper_schoolbook::asinh_schoolbook::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_acosh(self, mode: RoundingMode) -> Self {
            Self(crate::algos::trig::hyper_schoolbook::acosh_schoolbook::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_acosh_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(crate::algos::trig::hyper_schoolbook::acosh_schoolbook::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_atanh(self, mode: RoundingMode) -> Self {
            Self(crate::algos::trig::hyper_schoolbook::atanh_schoolbook::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_atanh_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(crate::algos::trig::hyper_schoolbook::atanh_schoolbook::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_to_degrees(self, mode: RoundingMode) -> Self {
            Self::from_bits(crate::policy::to_degrees::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
        #[inline]
        pub(crate) fn policy_to_degrees_with(self, wd: u32, mode: RoundingMode) -> Self {
            let _ = wd;
            Self::from_bits(crate::policy::to_degrees::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
        #[inline]
        pub(crate) fn policy_to_radians(self, mode: RoundingMode) -> Self {
            Self::from_bits(crate::policy::to_radians::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
        #[inline]
        pub(crate) fn policy_to_radians_with(self, wd: u32, mode: RoundingMode) -> Self {
            let _ = wd;
            Self::from_bits(crate::policy::to_radians::dispatch::<_, SCALE>(self.to_bits(), mode))
        }
    };
}

/// Hyperbolics — inherent `*_strict_with` shells with no per-band
/// override (`ExpIdentity` realised by the inherent composition).
#[allow(unused_macros)]
macro_rules! wide_trig_hyper_inherent {
    ($N:literal, $Core:ty) => {
        #[inline]
        pub(crate) fn policy_sinh(self, mode: RoundingMode) -> Self {
            match hyper::resolve::<$N, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => Self(hyper_rung::sinh_strict::<$Core, SCALE>(self.0, mode)),
                #[allow(dead_code)]
                hyper::Algorithm::Schoolbook => Self(crate::algos::trig::hyper_schoolbook::sinh_schoolbook::<$Core, SCALE>(self.0, mode)),
            }
        }
        #[inline]
        pub(crate) fn policy_sinh_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(hyper_rung::sinh_strict::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_cosh(self, mode: RoundingMode) -> Self {
            match hyper::resolve::<$N, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => Self(hyper_rung::cosh_strict::<$Core, SCALE>(self.0, mode)),
                #[allow(dead_code)]
                hyper::Algorithm::Schoolbook => Self(crate::algos::trig::hyper_schoolbook::cosh_schoolbook::<$Core, SCALE>(self.0, mode)),
            }
        }
        #[inline]
        pub(crate) fn policy_cosh_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(hyper_rung::cosh_strict::<$Core, SCALE>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_tanh(self, mode: RoundingMode) -> Self {
            match hyper::resolve::<$N, SCALE>(&self.0) {
                hyper::Algorithm::ExpIdentity => Self(hyper_rung::tanh_strict::<$Core, SCALE>(self.0, mode)),
                #[allow(dead_code)]
                hyper::Algorithm::Schoolbook => Self(crate::algos::trig::hyper_schoolbook::tanh_schoolbook::<$Core, SCALE>(self.0, mode)),
            }
        }
        #[inline]
        pub(crate) fn policy_tanh_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(hyper_rung::tanh_strict::<$Core, SCALE>(self.0, mode))
        }
    };
}

/// Forward family for a wide tier with **no** Tang band: every forward
/// cell is `Series` on the tier `wide_kernel`. The `Tang` arm is
/// unreachable (no `select` arm yields it at this `N`) and forwards to the
/// series kernel so the `match` stays exhaustive and dead-arm-eliminated.
#[allow(unused_macros)]
macro_rules! wide_trig_forward_series {
    ($N:literal, $Core:ty) => {
        #[inline]
        pub(crate) fn policy_sin(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => {
                    forward_rung::sin_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode)
                }
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => {
                    forward_rung::sin_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode)
                }
                #[allow(dead_code)]
                forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::sin_schoolbook::<$Core, SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_sin_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(forward_rung::sin_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_cos(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => {
                    forward_rung::cos_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode)
                }
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => {
                    forward_rung::cos_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode)
                }
                #[allow(dead_code)]
                forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::cos_schoolbook::<$Core, SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_cos_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(forward_rung::cos_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_tan(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve_tan::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => {
                    forward_rung::tan_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode)
                }
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => {
                    forward_rung::tan_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode)
                }
                #[allow(dead_code)]
                forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::tan_schoolbook::<$Core, SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_tan_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(forward_rung::tan_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode))
        }
        #[inline]
        pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
            Self(match forward::resolve::<$N, SCALE>(&self.0) {
                forward::Algorithm::Series => {
                    forward_rung::atan_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true>(self.0, mode)
                }
                #[cfg(feature = "_wide-support")]
                forward::Algorithm::Tang => {
                    forward_rung::atan_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true>(self.0, mode)
                }
                #[allow(dead_code)]
                forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::atan_schoolbook::<$Core, SCALE>(self.0, mode),
            })
        }
        #[inline]
        pub(crate) fn policy_atan_with(self, _wd: u32, mode: RoundingMode) -> Self {
            Self(forward_rung::atan_strict::<$Core, SCALE, { <$Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true>(self.0, mode))
        }
    };
}

// ── D57 — forward Tang band at 44..=56 (sin/cos/atan), narrow-GUARD
// Series band at 18..=22 (sin/cos/tan/atan); inverse + hyper divert
// 18..=22 to their lookup kernels. ─────────────────────────────────────
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<3>, SCALE> {
    // Forward family — `Series` runs the 18..=22 narrow-GUARD lookup or
    // the generic `wide_kernel`; `Tang` runs the 44..=56 band kernel
    // (sin/cos/atan only — tan has no 44..=56 Tang band).
    #[inline]
    pub(crate) fn policy_sin(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                // Same narrow-GUARD reclaim as the D153/D307/D462 bands,
                // through the shared directed-aware generic (Ziv escalation
                // + the bounded-extremum adjust); the bespoke single-shot
                // D57 slot kept only for tan. GUARD=8 per the band's probe.
                // Both bands run at the SCALE-derived work rung.
                18..=22 => forward_rung::sin_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, 8>(self.0, mode),
                _ => forward_rung::sin_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, { <crate::types::widths::wide_trig_d57::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            },
            forward::Algorithm::Tang => {
                trig::sincos_tang::sin_tang_with_taylor::<crate::types::widths::wide_trig_d57::Core, SCALE, 512>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::sin_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sin_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sin(mode)
    }
    #[inline]
    pub(crate) fn policy_cos(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                // Shared directed-aware narrow-GUARD kernel — see the
                // matching `policy_sin` arm above.
                18..=22 => forward_rung::cos_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, 8>(self.0, mode),
                _ => forward_rung::cos_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, { <crate::types::widths::wide_trig_d57::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            },
            forward::Algorithm::Tang => {
                trig::sincos_tang::cos_tang_with_taylor::<crate::types::widths::wide_trig_d57::Core, SCALE, 512>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::cos_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cos_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cos(mode)
    }
    #[inline]
    pub(crate) fn policy_tan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                18..=22 => trig::sincos_tang_3limb_s18_22::tan_strict::<SCALE>(self.0, mode),
                _ => forward_rung::tan_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, { <crate::types::widths::wide_trig_d57::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode),
            },
            // tan has no D57 Tang band; the arm is dead-arm-eliminated
            // (forwards to the generic kernel for exhaustiveness).
            forward::Algorithm::Tang => forward_rung::tan_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, { <crate::types::widths::wide_trig_d57::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode),
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::tan_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tan(mode)
    }
    #[inline]
    pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<3, SCALE>(&self.0) {
            forward::Algorithm::Series => match SCALE {
                // Both bands run at the SCALE-derived work rung.
                18..=22 => forward_rung::atan_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, 10, false>(self.0, mode),
                _ => forward_rung::atan_strict::<crate::types::widths::wide_trig_d57::Core, SCALE, { <crate::types::widths::wide_trig_d57::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true>(self.0, mode),
            },
            forward::Algorithm::Tang => {
                trig::atan_tang_3limb_s44_56::atan_strict::<SCALE>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::atan_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_atan(mode)
    }

    // Inverse family — `Atan` realised by the 18..=22 lookup or the
    // inherent shell.
    #[inline]
    pub(crate) fn policy_asin(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<3, SCALE>(&self.0) {
            inverse::Algorithm::Atan => match SCALE {
                18..=22 => trig::inverse_tang_3limb_s18_22::asin_strict::<SCALE>(self.0, mode),
                _ => return Self(inverse_rung::asin_strict::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::asin_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_asin_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_asin(mode)
    }
    #[inline]
    pub(crate) fn policy_acos(self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<3, SCALE>(&self.0) {
            inverse::Algorithm::Atan => match SCALE {
                18..=22 => trig::inverse_tang_3limb_s18_22::acos_strict::<SCALE>(self.0, mode),
                _ => return Self(inverse_rung::acos_strict::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::acos_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_acos_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_acos(mode)
    }
    #[inline]
    pub(crate) fn policy_atan2(self, other: Self, mode: RoundingMode) -> Self {
        Self(match inverse::resolve::<3, SCALE>(&self.0) {
            inverse::Algorithm::Atan => match SCALE {
                18..=22 => {
                    trig::inverse_tang_3limb_s18_22::atan2_strict::<SCALE>(self.0, other.0, mode)
                }
                _ => return Self(inverse_rung::atan2_strict::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, other.0, mode)),
            },
            #[allow(dead_code)]
            inverse::Algorithm::Schoolbook => trig::inverse_schoolbook::atan2_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, other.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan2_with(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_atan2(other, mode)
    }

    // Hyperbolics — `ExpIdentity` realised by the 18..=22 lookup or the
    // inherent shell.
    #[inline]
    pub(crate) fn policy_sinh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<3, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                18..=22 => trig::hyper_exp_identity::sinh_exp_identity_with_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(hyper_rung::sinh_strict::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::sinh_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sinh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sinh(mode)
    }
    #[inline]
    pub(crate) fn policy_cosh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<3, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                18..=22 => trig::hyper_exp_identity::cosh_exp_identity_with_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(hyper_rung::cosh_strict::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::cosh_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cosh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cosh(mode)
    }
    #[inline]
    pub(crate) fn policy_tanh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<3, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                18..=22 => trig::hyper_exp_identity::tanh_exp_identity_with_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(hyper_rung::tanh_strict::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::tanh_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tanh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tanh(mode)
    }

    wide_trig_extra_inherent!(3, crate::types::widths::wide_trig_d57::Core);
}

// ── D76 — width default (no bands) ─────────────────────────────────────
#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<4>, SCALE> {
    wide_trig_forward_series!(4, crate::types::widths::wide_trig_d76::Core);
    wide_trig_inverse_inherent!(4, crate::types::widths::wide_trig_d76::Core);
    wide_trig_hyper_inherent!(4, crate::types::widths::wide_trig_d76::Core);
    wide_trig_extra_inherent!(4, crate::types::widths::wide_trig_d76::Core);
}

// ── D115 — forward via wide_kernel; sinh/cosh/tanh divert SCALE
// 50..=60 to the Tang-style hyper lookup. ──────────────────────────────
#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<6>, SCALE> {
    wide_trig_forward_series!(6, crate::types::widths::wide_trig_d115::Core);
    wide_trig_inverse_inherent!(6, crate::types::widths::wide_trig_d115::Core);

    #[inline]
    pub(crate) fn policy_sinh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<6, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                50..=60 => trig::hyper_exp_identity::sinh_exp_identity_with_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(crate::algos::trig::hyper_schoolbook::sinh_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::sinh_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sinh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sinh(mode)
    }
    #[inline]
    pub(crate) fn policy_cosh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<6, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                50..=60 => trig::hyper_exp_identity::cosh_exp_identity_with_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(crate::algos::trig::hyper_schoolbook::cosh_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::cosh_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cosh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cosh(mode)
    }
    #[inline]
    pub(crate) fn policy_tanh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<6, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                50..=60 => trig::hyper_exp_identity::tanh_exp_identity_with_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(crate::algos::trig::hyper_schoolbook::tanh_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::tanh_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tanh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tanh(mode)
    }

    wide_trig_extra_inherent!(6, crate::types::widths::wide_trig_d115::Core);
}

// ── D153 — forward sin/cos/tan/atan divert SCALE 70..=82 (Tang);
// sinh/cosh/tanh divert the same band. ─────────────────────────────────
#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<8>, SCALE> {
    #[inline]
    pub(crate) fn policy_sin(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::sin_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, { <crate::types::widths::wide_trig_d153::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::sin_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, 10>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::sin_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sin_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sin(mode)
    }
    #[inline]
    pub(crate) fn policy_cos(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::cos_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, { <crate::types::widths::wide_trig_d153::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::cos_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, 10>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::cos_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cos_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cos(mode)
    }
    #[inline]
    pub(crate) fn policy_tan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::tan_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, { <crate::types::widths::wide_trig_d153::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::tan_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, true, false>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::tan_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tan(mode)
    }
    #[inline]
    pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<8, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::atan_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, { <crate::types::widths::wide_trig_d153::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::atan_strict::<crate::types::widths::wide_trig_d153::Core, SCALE, 12, false>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::atan_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_atan(mode)
    }

    wide_trig_inverse_inherent!(8, crate::types::widths::wide_trig_d153::Core);

    #[inline]
    pub(crate) fn policy_sinh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<8, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                70..=82 => trig::hyper_exp_identity::sinh_exp_identity_with_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, 128, true>(self.0, mode),
                _ => return Self(hyper_rung::sinh_strict::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::sinh_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sinh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sinh(mode)
    }
    #[inline]
    pub(crate) fn policy_cosh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<8, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                70..=82 => trig::hyper_exp_identity::cosh_exp_identity_with_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, 128, true>(self.0, mode),
                _ => return Self(hyper_rung::cosh_strict::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::cosh_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cosh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cosh(mode)
    }
    #[inline]
    pub(crate) fn policy_tanh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<8, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                70..=82 => trig::hyper_exp_identity::tanh_exp_identity_with_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, 128, true>(self.0, mode),
                _ => return Self(hyper_rung::tanh_strict::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::tanh_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tanh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tanh(mode)
    }

    wide_trig_extra_inherent!(8, crate::types::widths::wide_trig_d153::Core);
}

// ── D230 — width default (no bands) ────────────────────────────────────
#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<12>, SCALE> {
    wide_trig_forward_series!(12, crate::types::widths::wide_trig_d230::Core);
    wide_trig_inverse_inherent!(12, crate::types::widths::wide_trig_d230::Core);
    wide_trig_hyper_inherent!(12, crate::types::widths::wide_trig_d230::Core);
    wide_trig_extra_inherent!(12, crate::types::widths::wide_trig_d230::Core);
}

// ── D307 — forward sin/cos/tan/atan divert SCALE 140..=160 (Tang);
// sinh/cosh/tanh divert the same band. ─────────────────────────────────
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<16>, SCALE> {
    #[inline]
    pub(crate) fn policy_sin(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::sin_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, { <crate::types::widths::wide_trig_d307::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::sin_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, 8>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::sin_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sin_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sin(mode)
    }
    #[inline]
    pub(crate) fn policy_cos(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::cos_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, { <crate::types::widths::wide_trig_d307::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::cos_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, 8>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::cos_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cos_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cos(mode)
    }
    #[inline]
    pub(crate) fn policy_tan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::tan_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, { <crate::types::widths::wide_trig_d307::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::tan_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, 8, true, false>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::tan_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tan(mode)
    }
    #[inline]
    pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<16, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::atan_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, { <crate::types::widths::wide_trig_d307::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::atan_strict::<crate::types::widths::wide_trig_d307::Core, SCALE, 10, false>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::atan_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_atan(mode)
    }

    wide_trig_inverse_inherent!(16, crate::types::widths::wide_trig_d307::Core);

    #[inline]
    pub(crate) fn policy_sinh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<16, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                140..=160 => trig::hyper_exp_identity::sinh_exp_identity_with_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(hyper_rung::sinh_strict::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::sinh_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sinh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sinh(mode)
    }
    #[inline]
    pub(crate) fn policy_cosh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<16, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                140..=160 => trig::hyper_exp_identity::cosh_exp_identity_with_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(hyper_rung::cosh_strict::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::cosh_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cosh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cosh(mode)
    }
    #[inline]
    pub(crate) fn policy_tanh(self, mode: RoundingMode) -> Self {
        Self(match hyper::resolve::<16, SCALE>(&self.0) {
            hyper::Algorithm::ExpIdentity => match SCALE {
                140..=160 => trig::hyper_exp_identity::tanh_exp_identity_with_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 8, 128, false>(self.0, mode),
                _ => return Self(hyper_rung::tanh_strict::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode)),
            },
            #[allow(dead_code)]
            hyper::Algorithm::Schoolbook => trig::hyper_schoolbook::tanh_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tanh_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tanh(mode)
    }

    wide_trig_extra_inherent!(16, crate::types::widths::wide_trig_d307::Core);
}

// ── D462 — forward sin/cos/tan/atan divert SCALE 225..=235 (Tang);
// the hyperbolics keep the inherent shells (Tang hyper slot lost here). ─
#[cfg(any(feature = "d462", feature = "x-wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<24>, SCALE> {
    #[inline]
    pub(crate) fn policy_sin(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::sin_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, { <crate::types::widths::wide_trig_d462::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::sin_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, 10>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::sin_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_sin_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_sin(mode)
    }
    #[inline]
    pub(crate) fn policy_cos(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::cos_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, { <crate::types::widths::wide_trig_d462::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::cos_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, 10>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::cos_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_cos_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_cos(mode)
    }
    #[inline]
    pub(crate) fn policy_tan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve_tan::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::tan_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, { <crate::types::widths::wide_trig_d462::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true, true>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::tan_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, 10, false, false>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::tan_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_tan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_tan(mode)
    }
    #[inline]
    pub(crate) fn policy_atan(self, mode: RoundingMode) -> Self {
        Self(match forward::resolve::<24, SCALE>(&self.0) {
            forward::Algorithm::Series => forward_rung::atan_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, { <crate::types::widths::wide_trig_d462::Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD }, true>(self.0, mode),
            forward::Algorithm::Tang => {
                forward_rung::atan_strict::<crate::types::widths::wide_trig_d462::Core, SCALE, 12, false>(self.0, mode)
            }
            #[allow(dead_code)]
            forward::Algorithm::Schoolbook => crate::algos::trig::trig_schoolbook::atan_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(self.0, mode),
        })
    }
    #[inline]
    pub(crate) fn policy_atan_with(self, _wd: u32, mode: RoundingMode) -> Self {
        self.policy_atan(mode)
    }

    wide_trig_inverse_inherent!(24, crate::types::widths::wide_trig_d462::Core);
    wide_trig_hyper_inherent!(24, crate::types::widths::wide_trig_d462::Core);
    wide_trig_extra_inherent!(24, crate::types::widths::wide_trig_d462::Core);
}

// ── D616 — width default (no bands) ────────────────────────────────────
#[cfg(any(feature = "d616", feature = "x-wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<32>, SCALE> {
    wide_trig_forward_series!(32, crate::types::widths::wide_trig_d616::Core);
    wide_trig_inverse_inherent!(32, crate::types::widths::wide_trig_d616::Core);
    wide_trig_hyper_inherent!(32, crate::types::widths::wide_trig_d616::Core);
    wide_trig_extra_inherent!(32, crate::types::widths::wide_trig_d616::Core);
}

// ── D924 — width default (no bands) ────────────────────────────────────
#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<48>, SCALE> {
    wide_trig_forward_series!(48, crate::types::widths::wide_trig_d924::Core);
    wide_trig_inverse_inherent!(48, crate::types::widths::wide_trig_d924::Core);
    wide_trig_hyper_inherent!(48, crate::types::widths::wide_trig_d924::Core);
    wide_trig_extra_inherent!(48, crate::types::widths::wide_trig_d924::Core);
}

// ── D1232 — width default (no bands) ───────────────────────────────────
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<64>, SCALE> {
    wide_trig_forward_series!(64, crate::types::widths::wide_trig_d1232::Core);
    wide_trig_inverse_inherent!(64, crate::types::widths::wide_trig_d1232::Core);
    wide_trig_hyper_inherent!(64, crate::types::widths::wide_trig_d1232::Core);
    wide_trig_extra_inherent!(64, crate::types::widths::wide_trig_d1232::Core);
}

// ── Per-function generic dispatch fns (matcher-only seam) ─────────────
use crate::int::types::traits::BigInt as _;
use crate::int::types::Int;

#[inline]
#[must_use]
pub(crate) fn sin_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sin(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn sin_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sin_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn cos_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cos(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn cos_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cos_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn tan_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tan(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn tan_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tan_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn atan_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn atan_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn asin_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asin(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn asin_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asin_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn acos_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acos(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn acos_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acos_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn sinh_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sinh(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn sinh_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_sinh_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn cosh_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cosh(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn cosh_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_cosh_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn tanh_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tanh(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn tanh_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_tanh_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn asinh_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asinh(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn asinh_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_asinh_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn acosh_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acosh(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn acosh_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_acosh_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn atanh_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atanh(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn atanh_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atanh_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn to_degrees_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_degrees(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn to_degrees_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_degrees_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn to_radians_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_radians(mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn to_radians_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_to_radians_with(wd, mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn atan2_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, other: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_atan2(crate::D::<crate::int::types::Int<1>, SCALE>(other.resize_to::<crate::int::types::Int<1>>()), mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan2(crate::D::<crate::int::types::Int<2>, SCALE>(other.resize_to::<crate::int::types::Int<2>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_atan2(crate::D::<crate::int::types::Int<3>, SCALE>(other.resize_to::<crate::int::types::Int<3>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_atan2(crate::D::<crate::int::types::Int<4>, SCALE>(other.resize_to::<crate::int::types::Int<4>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_atan2(crate::D::<crate::int::types::Int<6>, SCALE>(other.resize_to::<crate::int::types::Int<6>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_atan2(crate::D::<crate::int::types::Int<8>, SCALE>(other.resize_to::<crate::int::types::Int<8>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_atan2(crate::D::<crate::int::types::Int<12>, SCALE>(other.resize_to::<crate::int::types::Int<12>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_atan2(crate::D::<crate::int::types::Int<16>, SCALE>(other.resize_to::<crate::int::types::Int<16>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_atan2(crate::D::<crate::int::types::Int<24>, SCALE>(other.resize_to::<crate::int::types::Int<24>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_atan2(crate::D::<crate::int::types::Int<32>, SCALE>(other.resize_to::<crate::int::types::Int<32>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_atan2(crate::D::<crate::int::types::Int<48>, SCALE>(other.resize_to::<crate::int::types::Int<48>>()), mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_atan2(crate::D::<crate::int::types::Int<64>, SCALE>(other.resize_to::<crate::int::types::Int<64>>()), mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan2(crate::D::<crate::int::types::Int<2>, SCALE>(other.resize_to::<crate::int::types::Int<2>>()), mode).0.resize_to::<Int<N>>(),
    }
}

#[inline]
#[must_use]
pub(crate) fn atan2_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, other: Int<N>, wd: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 => crate::D::<crate::int::types::Int<1>, SCALE>(raw.resize_to::<crate::int::types::Int<1>>()).policy_atan2_with(crate::D::<crate::int::types::Int<1>, SCALE>(other.resize_to::<crate::int::types::Int<1>>()), wd, mode).0.resize_to::<Int<N>>(),
        2 => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan2_with(crate::D::<crate::int::types::Int<2>, SCALE>(other.resize_to::<crate::int::types::Int<2>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).policy_atan2_with(crate::D::<crate::int::types::Int<3>, SCALE>(other.resize_to::<crate::int::types::Int<3>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).policy_atan2_with(crate::D::<crate::int::types::Int<4>, SCALE>(other.resize_to::<crate::int::types::Int<4>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).policy_atan2_with(crate::D::<crate::int::types::Int<6>, SCALE>(other.resize_to::<crate::int::types::Int<6>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).policy_atan2_with(crate::D::<crate::int::types::Int<8>, SCALE>(other.resize_to::<crate::int::types::Int<8>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).policy_atan2_with(crate::D::<crate::int::types::Int<12>, SCALE>(other.resize_to::<crate::int::types::Int<12>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).policy_atan2_with(crate::D::<crate::int::types::Int<16>, SCALE>(other.resize_to::<crate::int::types::Int<16>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).policy_atan2_with(crate::D::<crate::int::types::Int<24>, SCALE>(other.resize_to::<crate::int::types::Int<24>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).policy_atan2_with(crate::D::<crate::int::types::Int<32>, SCALE>(other.resize_to::<crate::int::types::Int<32>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).policy_atan2_with(crate::D::<crate::int::types::Int<48>, SCALE>(other.resize_to::<crate::int::types::Int<48>>()), wd, mode).0.resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).policy_atan2_with(crate::D::<crate::int::types::Int<64>, SCALE>(other.resize_to::<crate::int::types::Int<64>>()), wd, mode).0.resize_to::<Int<N>>(),
        _ => crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).policy_atan2_with(crate::D::<crate::int::types::Int<2>, SCALE>(other.resize_to::<crate::int::types::Int<2>>()), wd, mode).0.resize_to::<Int<N>>(),
    }
}

// ── `checked_*` hop dispatchers ───────────────────────────────────────
//
// The `checked_<fn>_strict_with` surface for the methods whose WIDE
// `*_strict_with` shells are inline kernel compositions (emitted by
// `decl_wide_transcendental!`), not policy-dispatch delegates: sinh,
// cosh, tanh, asin, acos, atan2, asinh, acosh, atanh, to_degrees,
// to_radians. Bit-identity with the default form is the contract, so
// each arm hops to the SAME inherent `*_strict_with` shell the default
// surface runs — never a parallel kernel route.
//
// The `N == 1` arm runs the D38 shell and re-applies the D18
// `try_into` fit as [`super::narrow_fit`], so a result that fits the
// D38 work width but not D18 storage is an exact `None` (the condition
// the default D18 shell panics on). Out-of-range detection INSIDE the
// D38 / wide shells is not yet threaded through (see
// `research/checked_wide_shell_patch.md`): those still panic,
// identically to the default form.

macro_rules! checked_hop_dispatch {
    ($name:ident, $method:ident) => {
        #[inline]
        #[must_use]
        pub(crate) fn $name<const N: usize, const SCALE: u32>(
            raw: Int<N>,
            mode: RoundingMode,
        ) -> Option<Int<N>> {
            match N {
                1 => super::narrow_fit::<N>(
                    crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).$method(mode).0,
                ),
                2 => Some(crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d57", feature = "wide"))]
                3 => Some(crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d76", feature = "wide"))]
                4 => Some(crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d115", feature = "wide"))]
                6 => Some(crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d153", feature = "wide"))]
                8 => Some(crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d230", feature = "wide"))]
                12 => Some(crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
                16 => Some(crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d462", feature = "x-wide"))]
                24 => Some(crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d616", feature = "x-wide"))]
                32 => Some(crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d924", feature = "xx-wide"))]
                48 => Some(crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).$method(mode).0.resize_to::<Int<N>>()),
                #[cfg(any(feature = "d1232", feature = "xx-wide"))]
                64 => Some(crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).$method(mode).0.resize_to::<Int<N>>()),
                _ => super::narrow_fit::<N>(
                    crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).$method(mode).0,
                ),
            }
        }
    };
}

checked_hop_dispatch!(checked_sinh_dispatch, sinh_strict_with);
checked_hop_dispatch!(checked_cosh_dispatch, cosh_strict_with);
checked_hop_dispatch!(checked_tanh_dispatch, tanh_strict_with);
checked_hop_dispatch!(checked_asin_dispatch, asin_strict_with);
checked_hop_dispatch!(checked_acos_dispatch, acos_strict_with);
checked_hop_dispatch!(checked_asinh_dispatch, asinh_strict_with);
checked_hop_dispatch!(checked_acosh_dispatch, acosh_strict_with);
checked_hop_dispatch!(checked_atanh_dispatch, atanh_strict_with);
checked_hop_dispatch!(checked_to_degrees_dispatch, to_degrees_strict_with);
checked_hop_dispatch!(checked_to_radians_dispatch, to_radians_strict_with);

/// Binary (`y.atan2(x)`) sibling of the [`checked_hop_dispatch!`]
/// emissions; same hop shape, second operand threaded through.
#[inline]
#[must_use]
pub(crate) fn checked_atan2_dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    other: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 => super::narrow_fit::<N>(
            crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).atan2_strict_with(crate::D::<crate::int::types::Int<2>, SCALE>(other.resize_to::<crate::int::types::Int<2>>()), mode).0,
        ),
        2 => Some(crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).atan2_strict_with(crate::D::<crate::int::types::Int<2>, SCALE>(other.resize_to::<crate::int::types::Int<2>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::D::<crate::int::types::Int<3>, SCALE>(raw.resize_to::<crate::int::types::Int<3>>()).atan2_strict_with(crate::D::<crate::int::types::Int<3>, SCALE>(other.resize_to::<crate::int::types::Int<3>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::D::<crate::int::types::Int<4>, SCALE>(raw.resize_to::<crate::int::types::Int<4>>()).atan2_strict_with(crate::D::<crate::int::types::Int<4>, SCALE>(other.resize_to::<crate::int::types::Int<4>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::D::<crate::int::types::Int<6>, SCALE>(raw.resize_to::<crate::int::types::Int<6>>()).atan2_strict_with(crate::D::<crate::int::types::Int<6>, SCALE>(other.resize_to::<crate::int::types::Int<6>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::D::<crate::int::types::Int<8>, SCALE>(raw.resize_to::<crate::int::types::Int<8>>()).atan2_strict_with(crate::D::<crate::int::types::Int<8>, SCALE>(other.resize_to::<crate::int::types::Int<8>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::D::<crate::int::types::Int<12>, SCALE>(raw.resize_to::<crate::int::types::Int<12>>()).atan2_strict_with(crate::D::<crate::int::types::Int<12>, SCALE>(other.resize_to::<crate::int::types::Int<12>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::D::<crate::int::types::Int<16>, SCALE>(raw.resize_to::<crate::int::types::Int<16>>()).atan2_strict_with(crate::D::<crate::int::types::Int<16>, SCALE>(other.resize_to::<crate::int::types::Int<16>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::D::<crate::int::types::Int<24>, SCALE>(raw.resize_to::<crate::int::types::Int<24>>()).atan2_strict_with(crate::D::<crate::int::types::Int<24>, SCALE>(other.resize_to::<crate::int::types::Int<24>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::D::<crate::int::types::Int<32>, SCALE>(raw.resize_to::<crate::int::types::Int<32>>()).atan2_strict_with(crate::D::<crate::int::types::Int<32>, SCALE>(other.resize_to::<crate::int::types::Int<32>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::D::<crate::int::types::Int<48>, SCALE>(raw.resize_to::<crate::int::types::Int<48>>()).atan2_strict_with(crate::D::<crate::int::types::Int<48>, SCALE>(other.resize_to::<crate::int::types::Int<48>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::D::<crate::int::types::Int<64>, SCALE>(raw.resize_to::<crate::int::types::Int<64>>()).atan2_strict_with(crate::D::<crate::int::types::Int<64>, SCALE>(other.resize_to::<crate::int::types::Int<64>>()), mode).0.resize_to::<Int<N>>()),
        _ => super::narrow_fit::<N>(
            crate::D::<crate::int::types::Int<2>, SCALE>(raw.resize_to::<crate::int::types::Int<2>>()).atan2_strict_with(crate::D::<crate::int::types::Int<2>, SCALE>(other.resize_to::<crate::int::types::Int<2>>()), mode).0,
        ),
    }
}

#[cfg(test)]
mod forward_rung_tests {
    //! Light anchor tests for the forward-trig work-rung routing (the
    //! `policy::ln` test shape): the rung-routed public path must equal
    //! the tier-width kernel bit-for-bit on representative values —
    //! in-budget low-scale anchors AND an out-of-budget magnitude that
    //! exercises the gate's tier fallback. Routing/threshold choices are
    //! NOT pinned (policy tests stay light); the golden gate is the
    //! correctness wall.

    #[cfg(feature = "d307")]
    const ALL_MODES: [crate::support::rounding::RoundingMode; 6] = [
        crate::support::rounding::RoundingMode::HalfToEven,
        crate::support::rounding::RoundingMode::HalfAwayFromZero,
        crate::support::rounding::RoundingMode::HalfTowardZero,
        crate::support::rounding::RoundingMode::Trunc,
        crate::support::rounding::RoundingMode::Floor,
        crate::support::rounding::RoundingMode::Ceiling,
    ];

    /// D307<0> rung anchors: the gate admits |x| < ~10^8 and the rung
    /// match folds to Int<12>; the values must equal the tier-width
    /// Series kernel across the everyday inputs and the near-π-multiple
    /// 3141 (4 integer digits, in-budget — the reduction-precision axis).
    #[test]
    #[cfg(feature = "d307")]
    fn d307_s0_rung_matches_tier_kernel() {
        type Core = crate::types::widths::wide_trig_d307::Core;
        const G: u32 =
            <Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD;
        for v in ["1", "-1", "2", "3141", "-1570"] {
            let x: crate::D307<0> = v.parse().unwrap();
            for mode in ALL_MODES {
                assert_eq!(
                    super::forward_rung::sin_strict::<Core, 0, G>(x.to_bits(), mode),
                    crate::algos::support::wide_trig_core::sin_series::<Core, 0>(x.to_bits(), mode),
                    "sin({v}) mode {mode:?}"
                );
                assert_eq!(
                    super::forward_rung::tan_strict::<Core, 0, G, true, true>(x.to_bits(), mode),
                    crate::algos::support::wide_trig_core::tan_series::<Core, 0>(x.to_bits(), mode),
                    "tan({v}) mode {mode:?}"
                );
                assert_eq!(
                    super::forward_rung::atan_strict::<Core, 0, G, true>(x.to_bits(), mode),
                    crate::algos::support::wide_trig_core::atan_series::<Core, 0>(x.to_bits(), mode),
                    "atan({v}) mode {mode:?}"
                );
            }
        }
    }

    /// D307 Tang-band shape anchor: the single-shot narrow-GUARD rung
    /// path (`DIRECTED = false`, band GUARD 10) must equal the tier
    /// `atan_narrow` band kernel bit-for-bit.
    #[test]
    #[cfg(feature = "d307")]
    fn d307_band_atan_rung_matches_tier_narrow() {
        type Core = crate::types::widths::wide_trig_d307::Core;
        for v in ["1", "-1", "2", "3141"] {
            let x: crate::D307<150> = v.parse().unwrap();
            for mode in ALL_MODES {
                assert_eq!(
                    super::forward_rung::atan_strict::<Core, 150, 10, false>(x.to_bits(), mode),
                    crate::algos::support::wide_trig_core::atan_narrow::<Core, 150, 10>(x.to_bits(), mode),
                    "atan({v}) band mode {mode:?}"
                );
            }
        }
    }

    /// Out-of-budget magnitude (10^9 rad > the 10^8 gate): the rung path
    /// must take the tier fallback and still equal the tier kernel.
    #[test]
    #[cfg(feature = "d307")]
    fn d307_s0_out_of_budget_falls_back_to_tier() {
        type Core = crate::types::widths::wide_trig_d307::Core;
        const G: u32 =
            <Core as crate::algos::support::wide_trig_core::WideTrigCore>::GUARD;
        let x: crate::D307<0> = "1000000000".parse().unwrap();
        for mode in ALL_MODES {
            assert_eq!(
                super::forward_rung::sin_strict::<Core, 0, G>(x.to_bits(), mode),
                crate::algos::support::wide_trig_core::sin_series::<Core, 0>(x.to_bits(), mode),
                "sin(1e9) mode {mode:?}"
            );
            assert_eq!(
                super::forward_rung::atan_strict::<Core, 0, G, true>(x.to_bits(), mode),
                crate::algos::support::wide_trig_core::atan_series::<Core, 0>(x.to_bits(), mode),
                "atan(1e9) mode {mode:?}"
            );
        }
    }

    /// Inverse-family rung anchors (D307<19>): the rung-routed asin /
    /// acos / atan2 must equal the tier-width compositions bit-for-bit
    /// across the domain spread (incl. the |x| > 1/2 half-angle branch
    /// and the ±1 endpoints), and the out-of-budget atan2 magnitude must
    /// take the tier fallback.
    #[test]
    #[cfg(feature = "d307")]
    fn d307_inverse_rung_matches_tier_kernels() {
        type Core = crate::types::widths::wide_trig_d307::Core;
        for v in ["0", "0.25", "0.6", "0.9", "1", "-0.6", "-1"] {
            let x: crate::D307<19> = v.parse().unwrap();
            for mode in ALL_MODES {
                assert_eq!(
                    super::inverse_rung::asin_strict::<Core, 19>(x.to_bits(), mode),
                    crate::algos::trig::inverse_schoolbook::asin_schoolbook::<Core, 19>(x.to_bits(), mode),
                    "asin({v}) mode {mode:?}"
                );
                assert_eq!(
                    super::inverse_rung::acos_strict::<Core, 19>(x.to_bits(), mode),
                    crate::algos::trig::inverse_schoolbook::acos_schoolbook::<Core, 19>(x.to_bits(), mode),
                    "acos({v}) mode {mode:?}"
                );
            }
        }
        for (y, x) in [
            ("1", "1"),
            ("1", "-1"),
            ("-1", "1"),
            ("-1", "-1"),
            ("0.5", "2"),
            ("2", "0.5"),
            ("1", "0"),
            ("0", "1"),
            ("1000000000", "1"),
        ] {
            let yd: crate::D307<19> = y.parse().unwrap();
            let xd: crate::D307<19> = x.parse().unwrap();
            for mode in ALL_MODES {
                assert_eq!(
                    super::inverse_rung::atan2_strict::<Core, 19>(yd.to_bits(), xd.to_bits(), mode),
                    crate::algos::trig::inverse_schoolbook::atan2_schoolbook::<Core, 19>(yd.to_bits(), xd.to_bits(), mode),
                    "atan2({y}, {x}) mode {mode:?}"
                );
            }
        }
    }

    /// Hyperbolic-family rung anchors (D307<19>): the rung-routed sinh /
    /// cosh / tanh must equal the tier-width exp-identity kernels
    /// bit-for-bit across the everyday spread (incl. the in-budget edge
    /// 9.9 and the tanh near-saturation shape), and the out-of-budget
    /// |x| = 50 must take the tier fallback.
    #[test]
    #[cfg(feature = "d307")]
    fn d307_hyper_rung_matches_tier_kernels() {
        type Core = crate::types::widths::wide_trig_d307::Core;
        for v in ["0", "0.3", "1", "1.5", "9.9", "-1", "-9.9", "50"] {
            let x: crate::D307<19> = v.parse().unwrap();
            for mode in ALL_MODES {
                assert_eq!(
                    super::hyper_rung::sinh_strict::<Core, 19>(x.to_bits(), mode),
                    crate::algos::trig::hyper_schoolbook::sinh_schoolbook::<Core, 19>(x.to_bits(), mode),
                    "sinh({v}) mode {mode:?}"
                );
                assert_eq!(
                    super::hyper_rung::cosh_strict::<Core, 19>(x.to_bits(), mode),
                    crate::algos::trig::hyper_schoolbook::cosh_schoolbook::<Core, 19>(x.to_bits(), mode),
                    "cosh({v}) mode {mode:?}"
                );
                assert_eq!(
                    super::hyper_rung::tanh_strict::<Core, 19>(x.to_bits(), mode),
                    crate::algos::trig::hyper_schoolbook::tanh_schoolbook::<Core, 19>(x.to_bits(), mode),
                    "tanh({v}) mode {mode:?}"
                );
            }
        }
    }
}
