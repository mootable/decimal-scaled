//! Exponential policy — the per-`(N, SCALE)` algorithm matcher.
//!
//! `D<Int<N>, SCALE>::exp_strict_with(mode)` delegates to
//! [`ExpPolicy::exp_impl`], which resolves the canonical
//! [`Algorithm`]/[`select`] verdict (see `docs/ARCHITECTURE.md` →
//! "Policy file structure", mirrored from the `sqrt` exemplar):
//!
//! 1. an [`Algorithm`] enum — the real exponential algorithms, no
//!    `Default` variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (`exp` has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block,
//!    then an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Why the kernels are threaded per-tier
//!
//! The two surviving algorithms — `exp_series` (the direct fixed-point
//! Taylor with the Smith `r/2^n` halving-and-squaring) and `exp_tang`
//! (Tang two-stage table-driven range reduction) — each have a SCALE-/
//! width-specific realisation today: the narrow tiers run on the 256-bit
//! `Fixed` intermediate (`exp::fixed_d38`), the wide tiers on per-tier
//! macro-emitted `wide_trig_<tier>` cores (`exp::wide_kernel`), and the
//! Tang bands on the `lookup_*` kernels. Collapsing those per-tier kernel
//! *bodies* to one generic-over-work-width `exp_series` / `exp_tang`
//! needs the macro-emitted core to lift to a generic `W` (or a
//! `WideTrigCore` trait) — the **4.1 genericisation prerequisite**
//! recorded in `phase4/migration_explog.md` and `exp::wide_kernel`'s
//! module docs, not a matcher concern. The matcher here is canonical: the
//! algorithm *choice* per cell is expressed once via `Algorithm`/`select`,
//! and each per-tier impl binds the concrete kernel that realises the
//! chosen algorithm at that width — the same "thread the work-specifics
//! in from the per-tier impl" shape the `sqrt` pilot uses for `W`.
//!
//! Functions covered: `exp` (natural) and `exp2` (base-2). `exp2` derives
//! from the same `Series` / `Tang` algorithms with a base-2 range
//! reduction (it carries no own `Algorithm` variant).

use crate::algos::exp;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

/// Per-width policy: which kernel a `D<Int<N>, SCALE>` uses for the
/// exponential family.
pub(crate) trait ExpPolicy: Sized {
    /// `e^self` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn exp_impl(self, mode: RoundingMode) -> Self;

    /// `e^self` with caller-chosen working digits.
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    /// `2^self` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn exp2_impl(self, mode: RoundingMode) -> Self;

    /// `2^self` with caller-chosen working digits.
    fn exp2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ── 1. the real exponential algorithms — NAMED, no `Default` ──────────

/// The exponential algorithms this policy chooses between. Variants are
/// the CamelCase of each kernel's name minus the `exp_` function prefix
/// (`exp_series` → `Series`, `exp_tang` → `Tang`) — strict 1:1 with the
/// kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `exp_series` — the direct fixed-point Taylor kernel with the
    /// Smith `r/2^n` argument-halving-and-squaring trick. The generic
    /// default; realised by `exp::fixed_d38` (narrow) and
    /// `exp::wide_kernel` (wide).
    Series,
    /// `exp_tang` — Tang (1989) table-driven two-stage range reduction.
    /// Selected on the benched SCALE bands; realised by the
    /// `exp::lookup_*` kernels.
    ///
    /// Gated with the wide tiers: every cell that selects `Tang` lives
    /// at N ≥ 3, so the variant, its `select` arms, and its dispatch
    /// arms are gated together (the policy stays exhaustive in both
    /// configs).
    #[cfg(feature = "_wide-support")]
    Tang,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". `ByValue` is part of the
/// canonical shape for uniformity across functions; `exp` never returns
/// it (the choice is fully determined by `(N, SCALE)`; the `raw == 0`
/// short-circuit is a value-dependent *step* inside the kernel, not an
/// algorithm choice).
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the exponential algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; the `_` arm is the generic `Series`
/// default (a real algorithm — there is no synthetic default variant).
///
/// Reproduces the routing of the legacy per-tier `(W, SCALE)` triplet:
/// the Tang bands (D57 18..=22 & 45..=56, D115 50..=60, D153 70..=82)
/// divert to `Tang`; every other cell — including the narrow tiers
/// widened to the D38 work width and all wide tiers — runs `Series`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // D57 (`Int<3>`) Tang bands.
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 18..=22) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 45..=56) => Select::ByAlgorithm(Algorithm::Tang),
        // D115 (`Int<6>`) Tang band.
        #[cfg(any(feature = "d115", feature = "wide"))]
        (6, 50..=60) => Select::ByAlgorithm(Algorithm::Tang),
        // D153 (`Int<8>`) Tang band.
        #[cfg(any(feature = "d153", feature = "wide"))]
        (8, 70..=82) => Select::ByAlgorithm(Algorithm::Tang),
        // Everything else (narrow tiers via widen, all other wide
        // cells) — generic Series.
        _ => Select::ByAlgorithm(Algorithm::Series),
    }
}

// ── 4. the per-tier dispatch macros: resolve the verdict, then run ────
//
// Each tier impl resolves `select::<N, SCALE>()` once (const-folded) and
// matches the verdict against the tier's concrete kernels. The `Series`
// arm names the tier's series realisation; the `Tang` arm names the
// tier's Tang realisation (or is unreachable — and dead-arm-eliminated —
// on tiers with no Tang band, where it forwards to the series kernel so
// the `match` stays exhaustive without a synthetic default).

/// Resolve the `(N, SCALE)` verdict to a concrete `Algorithm`. Shared by
/// every tier's `exp_impl`.
#[inline]
fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(raw),
    }
}

// ── Narrow tier (D18) — widen-to-D38 work width, then Series ──────────
//
// The narrow tier has no wide storage of its own; it widens into the D38
// `Fixed` work width and runs `Series` there (the `widen_to_work`
// strategy — a policy concern, not an algorithm). `exp2` widens to D38
// and runs its base-2 `Series` reduction.

impl<const SCALE: u32> ExpPolicy for D18<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        // N==1 always selects Series.
        exp::widen_to_d38::exp_strict_d18(self, mode)
    }
    #[inline]
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        exp::widen_to_d38::exp_with_d18(self, working_digits, mode)
    }
    #[inline]
    fn exp2_impl(self, mode: RoundingMode) -> Self {
        let wide: D38<SCALE> = self.into();
        ::core::convert::TryInto::try_into(wide.exp2_strict_with(mode)).unwrap_or_else(|_| {
            crate::support::diagnostics::overflow_panic_with_scale("D18::exp2", SCALE)
        })
    }
    #[inline]
    fn exp2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        let wide: D38<SCALE> = self.into();
        ::core::convert::TryInto::try_into(wide.exp2_approx_with(working_digits, mode))
            .unwrap_or_else(|_| {
                crate::support::diagnostics::overflow_panic_with_scale("D18::exp2", SCALE)
            })
    }
}

// ── D38 — Series on the in-tree `Fixed`-256 kernel ─────────────────────
//
// The borrow_d57 round trip was retired once the 0.4.2 MG-routed
// `Fixed::mul` / `div_small` / `divmod_u256_by_pow10` fast paths made the
// D38-native kernel beat the widen-and-back path (~2× faster at SCALE 19
// on the GHA shared-runner pool). N==2 always selects Series.
impl<const SCALE: u32> ExpPolicy for D38<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(match resolve::<2, SCALE>(&self.0) {
            // N==2 only ever selects Series; the Tang arm is gated in
            // and dead-arm-eliminated (it forwards to Series so the
            // `match` stays exhaustive without a synthetic default).
            Algorithm::Series => exp::fixed_d38::exp_strict::<SCALE>(self.0, mode),
            #[cfg(feature = "_wide-support")]
            Algorithm::Tang => exp::fixed_d38::exp_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(match resolve::<2, SCALE>(&self.0) {
            Algorithm::Series => exp::fixed_d38::exp_with(self.0, SCALE, working_digits, mode),
            #[cfg(feature = "_wide-support")]
            Algorithm::Tang => exp::fixed_d38::exp_with(self.0, SCALE, working_digits, mode),
        })
    }
    #[inline]
    fn exp2_impl(self, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp2_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn exp2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp2_with(self.0, SCALE, working_digits, mode))
    }
}

// ── Wide tiers — Series default, Tang on the benched bands ─────────────
//
// `exp_impl` / `exp_with_impl` resolve the canonical verdict and match it
// against the tier's concrete kernels. `exp2_impl` / `exp2_with_impl`
// delegate to the inherent `exp2_strict_with` shell (base-2 reduction →
// the same `Series` core; no raw-storage free-fn equivalent today).
//
// `std` == `non_std` for `exp` — the only std machinery is the
// wide-kernel constant cache, invisible to the policy — so there is no
// per-mode arm; the const verdict folds identically in both configs.

/// Emit `impl ExpPolicy for D<Int<$N>, SCALE>` for a wide tier with **no**
/// Tang band: every cell is `Series`. The `Tang` arm is unreachable (no
/// `select` arm yields it at this `N`) and forwards to the series kernel
/// so the `match` stays exhaustive and dead-arm-eliminated.
macro_rules! exp_policy_wide_series {
    ($T:ident, $N:literal, $series:path) => {
        impl<const SCALE: u32> ExpPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn exp_impl(self, mode: RoundingMode) -> Self {
                Self(match resolve::<$N, SCALE>(&self.0) {
                    Algorithm::Series => $series(self.0, mode, SCALE),
                    #[cfg(feature = "_wide-support")]
                    Algorithm::Tang => $series(self.0, mode, SCALE),
                })
            }
            #[inline]
            fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                Self(match resolve::<$N, SCALE>(&self.0) {
                    Algorithm::Series => $series(self.0, mode, SCALE),
                    #[cfg(feature = "_wide-support")]
                    Algorithm::Tang => $series(self.0, mode, SCALE),
                })
            }
            #[inline]
            fn exp2_impl(self, mode: RoundingMode) -> Self {
                self.exp2_strict_with(mode)
            }
            #[inline]
            fn exp2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                self.exp2_strict_with(mode)
            }
        }
    };
}

/// Emit `impl ExpPolicy for D<Int<$N>, SCALE>` for a wide tier that
/// carries a Tang band. `$series` realises `Algorithm::Series`; the
/// `$tang` block (a `match SCALE` over the band) realises
/// `Algorithm::Tang`. `select` only yields `Tang` for the band scales, so
/// the `$tang` arm is exhaustive over the reachable scales; the
/// `unreachable!()` covers the const-eliminated rest.
#[cfg(feature = "_wide-support")]
macro_rules! exp_policy_wide_tang {
    ($T:ident, $N:literal, $series:path, $tang:expr) => {
        impl<const SCALE: u32> ExpPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn exp_impl(self, mode: RoundingMode) -> Self {
                let raw = self.0;
                Self(match resolve::<$N, SCALE>(&raw) {
                    Algorithm::Series => $series(raw, mode, SCALE),
                    Algorithm::Tang => ($tang)(raw, mode),
                })
            }
            #[inline]
            fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                let raw = self.0;
                Self(match resolve::<$N, SCALE>(&raw) {
                    Algorithm::Series => $series(raw, mode, SCALE),
                    Algorithm::Tang => ($tang)(raw, mode),
                })
            }
            #[inline]
            fn exp2_impl(self, mode: RoundingMode) -> Self {
                self.exp2_strict_with(mode)
            }
            #[inline]
            fn exp2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                self.exp2_strict_with(mode)
            }
        }
    };
}

// D57 — Tang bands at SCALE 18..=22 (M=128) and 45..=56 (M=512).
#[cfg(any(feature = "d57", feature = "wide"))]
exp_policy_wide_tang!(D57, 3, exp::wide_kernel::exp_strict_d57, |raw: Int<3>,
                                                                 mode|
 -> Int<3> {
    match SCALE {
        18..=22 => exp::lookup_d57_s18_22_tang::exp_strict::<SCALE>(raw, mode),
        45..=56 => exp::lookup_d57_s45_56::exp_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

#[cfg(any(feature = "d76", feature = "wide"))]
exp_policy_wide_series!(D76, 4, exp::wide_kernel::exp_strict_d76);

// D115 — Tang band at SCALE 50..=60.
#[cfg(any(feature = "d115", feature = "wide"))]
exp_policy_wide_tang!(D115, 6, exp::wide_kernel::exp_strict_d115, |raw: Int<6>,
                                                                   mode|
 -> Int<6> {
    match SCALE {
        50..=60 => exp::lookup_d115_s57_tang::exp_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D153 — Tang band at SCALE 70..=82.
#[cfg(any(feature = "d153", feature = "wide"))]
exp_policy_wide_tang!(D153, 8, exp::wide_kernel::exp_strict_d153, |raw: Int<8>,
                                                                   mode|
 -> Int<8> {
    match SCALE {
        70..=82 => exp::lookup_d153_s70_82_tang::exp_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

#[cfg(any(feature = "d230", feature = "wide"))]
exp_policy_wide_series!(D230, 12, exp::wide_kernel::exp_strict_d230);

// D307 — Tang exp probed at SCALE 150 and showed a ~5% regression vs the
// canonical `wide_kernel::exp_strict_d307`; D307's Int<16> work integer
// is at the Tang-exp crossover. Surface `exp` keeps Series; the
// `tang_exp_fixed` machinery in the lookup module stays for the trig
// hyperbolics, not wired here.
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
exp_policy_wide_series!(D307, 16, exp::wide_kernel::exp_strict_d307);

// D462 — Tang exp probed at SCALE 225..=235 and LOST (~75% regression):
// at Int<48> the Tang post-reduction Taylor needs ~95 wide mults vs the
// Smith r/2^n path's ~28 wide squarings. Series stays the default; the
// lookup kernel is retained behind `cfg(test)` as the lab probe.
#[cfg(any(feature = "d462", feature = "x-wide"))]
exp_policy_wide_series!(D462, 24, exp::wide_kernel::exp_strict_d462);

// D616 — Tang lookup exp at SCALE 300..=315 was break-even at best
// (~250 µs Tang vs ~230 µs wide_kernel); the table multiply on a 1024-bit
// work integer matches the Smith squaring tail it elides. Series default;
// the lookup module stays for `tang_exp_fixed` only.
#[cfg(any(feature = "d616", feature = "x-wide"))]
exp_policy_wide_series!(D616, 32, exp::wide_kernel::exp_strict_d616);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
exp_policy_wide_series!(D924, 48, exp::wide_kernel::exp_strict_d924);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
exp_policy_wide_series!(D1232, 64, exp::wide_kernel::exp_strict_d1232);
