//! Natural-logarithm policy — the per-`(N, SCALE)` algorithm matcher
//! (plus the derived `log` / `log2` / `log10`).
//!
//! `D<Int<N>, SCALE>::ln_strict_with(mode)` delegates to
//! [`LnPolicy::ln_impl`], which resolves the canonical
//! [`Algorithm`]/[`select`] verdict (the `sqrt` exemplar shape — see
//! `docs/ARCHITECTURE.md` → "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real natural-log algorithms, no
//!    `Default` variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (`ln` has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block,
//!    then an **exhaustive** `match algo` — no `_`, no panic.
//!
//! The const verdict folds per monomorphisation, so every concrete
//! `D<Int<N>, SCALE>` compiles to a direct call to one kernel.
//!
//! # Why the kernels are threaded per-tier
//!
//! Same reasoning as [`crate::policy::exp`]: the two surviving algorithms
//! — `ln_series` (Brent argument-reduced artanh series) and `ln_tang`
//! (Tang table-driven reduction) — have SCALE-/width-specific kernel
//! realisations today (narrow `Fixed`-256 in `ln::fixed_d38`, wide
//! per-tier `wide_trig_<tier>` cores in `ln::wide_kernel`, Tang bands in
//! the `ln::lookup_*` kernels). Collapsing the kernel *bodies* to one
//! generic-over-`W` `ln_series` / `ln_tang` is the **4.1 genericisation
//! prerequisite** (`phase4/migration_explog.md`), not a matcher concern.
//! The matcher expresses the algorithm *choice* per cell canonically and
//! each per-tier impl binds the concrete kernel.
//!
//! `log` / `log2` / `log10` are **derived** from `ln` — a ratio
//! (`log_base = ln/ln(base)`) or constant divide (`log2 = ln/ln2`,
//! `log10 = ln/ln10`). They reuse the `ln` `Algorithm` enum and the same
//! `select`; they carry no own variants.

use crate::algos::ln;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

/// Per-width policy for natural log and the log family. See module docs.
pub(crate) trait LnPolicy: Sized {
    // ── Natural log ────────────────────────────────────────────────

    /// Strict natural log under the supplied rounding mode. Working
    /// scale is `SCALE + STRICT_GUARD` (const-folded).
    fn ln_impl(self, mode: RoundingMode) -> Self;

    /// Natural log with caller-chosen `working_digits` above the storage
    /// scale, under the supplied rounding mode.
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Log with chosen base ───────────────────────────────────────

    /// `log_base(self)` under the supplied rounding mode (strict guard).
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

// ── 1. the real natural-log algorithms — NAMED, no `Default` ──────────

/// The natural-log algorithms this policy chooses between. Variants are
/// the CamelCase of each kernel's name minus the `ln_` prefix
/// (`ln_series` → `Series`, `ln_tang` → `Tang`) — strict 1:1.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `ln_series` — Brent argument-reduced `2·artanh((m−1)/(m+1))`
    /// fixed-point series. The generic default; realised by
    /// `ln::fixed_d38` (narrow) and `ln::wide_kernel` (wide).
    Series,
    /// `ln_tang` — Tang table-driven range reduction. Selected on the
    /// benched SCALE bands; realised by the `ln::lookup_*` kernels.
    /// Gated with the wide tiers (every `Tang` cell is at N ≥ 3).
    #[cfg(feature = "_wide-support")]
    Tang,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". `ByValue` is part of the
/// canonical shape; `ln` never returns it (the `raw <= 0` panic is a
/// guard, not an algorithm choice).
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the natural-log algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; the `_` arm is the generic `Series`
/// default. Reproduces the legacy `(W, SCALE)` triplet routing: each wide
/// tier's mid- (and, on the upper widths, deep-) storage band diverts to
/// `Tang`; every other cell runs `Series`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // D57 (`Int<3>`).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 18..=22) => Select::ByAlgorithm(Algorithm::Tang),
        // D115 (`Int<6>`).
        #[cfg(any(feature = "d115", feature = "wide"))]
        (6, 50..=60) => Select::ByAlgorithm(Algorithm::Tang),
        // D153 (`Int<8>`).
        #[cfg(any(feature = "d153", feature = "wide"))]
        (8, 70..=82) => Select::ByAlgorithm(Algorithm::Tang),
        // D230 (`Int<12>`).
        #[cfg(any(feature = "d230", feature = "wide"))]
        (12, 110..=120) => Select::ByAlgorithm(Algorithm::Tang),
        // D307 (`Int<16>`) — two bands (mid + deep).
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        (16, 140..=160) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        (16, 285..=295) => Select::ByAlgorithm(Algorithm::Tang),
        // D462 (`Int<24>`).
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        (24, 225..=235) => Select::ByAlgorithm(Algorithm::Tang),
        // D616 (`Int<32>`) — two bands.
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        (32, 300..=315) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        (32, 585..=595) => Select::ByAlgorithm(Algorithm::Tang),
        // D924 (`Int<48>`) — two bands.
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        (48, 455..=465) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        (48, 895..=905) => Select::ByAlgorithm(Algorithm::Tang),
        // D1232 (`Int<64>`) — two bands.
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        (64, 610..=620) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        (64, 1195..=1205) => Select::ByAlgorithm(Algorithm::Tang),
        // Everything else — generic Series.
        _ => Select::ByAlgorithm(Algorithm::Series),
    }
}

/// Resolve the `(N, SCALE)` verdict to a concrete `Algorithm`.
#[inline]
fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(raw),
    }
}

// ── Narrow tier (D18) — widen-to-D38 work width, then Series ──────────
//
// D18 widens into the D38 `Fixed` work width for every log-family method
// (the `widen_to_work` strategy). `log` delegates to `LogPolicy::log_impl`
// (the `log` seam); `log2`/`log10` widen, call D38's method, narrow back.
impl<const SCALE: u32> LnPolicy for D18<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        ln::widen_to_d38::ln_strict_d18(self, mode)
    }
    #[inline]
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        ln::widen_to_d38::ln_with_d18(self, working_digits, mode)
    }
    #[inline]
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
        use crate::policy::log::LogPolicy;
        LogPolicy::log_impl(self, base, mode)
    }
    #[inline]
    fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
        use crate::policy::log::LogPolicy;
        LogPolicy::log_with_impl(self, base, working_digits, mode)
    }
    #[inline]
    fn log2_impl(self, mode: RoundingMode) -> Self {
        let wide: D38<SCALE> = self.into();
        ::core::convert::TryInto::try_into(wide.log2_strict_with(mode)).unwrap_or_else(|_| {
            crate::support::diagnostics::overflow_panic_with_scale("D18::log2", SCALE)
        })
    }
    #[inline]
    fn log2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        let wide: D38<SCALE> = self.into();
        ::core::convert::TryInto::try_into(wide.log2_approx_with(working_digits, mode))
            .unwrap_or_else(|_| {
                crate::support::diagnostics::overflow_panic_with_scale("D18::log2", SCALE)
            })
    }
    #[inline]
    fn log10_impl(self, mode: RoundingMode) -> Self {
        let wide: D38<SCALE> = self.into();
        ::core::convert::TryInto::try_into(wide.log10_strict_with(mode)).unwrap_or_else(|_| {
            crate::support::diagnostics::overflow_panic_with_scale("D18::log10", SCALE)
        })
    }
    #[inline]
    fn log10_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        let wide: D38<SCALE> = self.into();
        ::core::convert::TryInto::try_into(wide.log10_approx_with(working_digits, mode))
            .unwrap_or_else(|_| {
                crate::support::diagnostics::overflow_panic_with_scale("D18::log10", SCALE)
            })
    }
}

// ── D38 — Series on the in-tree `Fixed`-256 kernel ─────────────────────
//
// The borrow_d57 round trip was retired once the 0.4.2 MG-routed `Fixed`
// primitives made the D38-native kernel beat the widen-and-back path.
// N==2 always selects Series. `log` delegates to `LogPolicy::log_impl`.
impl<const SCALE: u32> LnPolicy for D38<SCALE> {
    #[inline]
    fn ln_impl(self, mode: RoundingMode) -> Self {
        Self(match resolve::<2, SCALE>(&self.0) {
            // N==2 only selects Series; the gated Tang arm is
            // dead-arm-eliminated and forwards to Series for exhaustiveness.
            Algorithm::Series => ln::fixed_d38::ln_strict::<SCALE>(self.0, mode),
            #[cfg(feature = "_wide-support")]
            Algorithm::Tang => ln::fixed_d38::ln_strict::<SCALE>(self.0, mode),
        })
    }
    #[inline]
    fn ln_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(ln::fixed_d38::ln_with(self.0, SCALE, working_digits, mode))
    }
    #[inline]
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
        use crate::policy::log::LogPolicy;
        LogPolicy::log_impl(self, base, mode)
    }
    #[inline]
    fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
        use crate::policy::log::LogPolicy;
        LogPolicy::log_with_impl(self, base, working_digits, mode)
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

// ── Wide tiers — Series default, Tang on the benched bands ─────────────
//
// `ln_impl` / `ln_with_impl` resolve the canonical verdict and match it
// against the tier's concrete kernels. The `log` family delegates to the
// inherent `*_strict_with` shells (which compose `ln_fixed` / `ln2` /
// `ln10`; no raw-storage free-fn equivalent). `std` == `non_std`.

/// Emit `impl LnPolicy for D<Int<$N>, SCALE>` for a wide tier with **no**
/// Tang band: every cell is `Series`.
#[allow(unused_macros)]
macro_rules! ln_policy_wide_series {
    ($T:ident, $N:literal, $series:path) => {
        impl<const SCALE: u32> LnPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn ln_impl(self, mode: RoundingMode) -> Self {
                Self(match resolve::<$N, SCALE>(&self.0) {
                    Algorithm::Series => $series(self.0, mode, SCALE),
                    #[cfg(feature = "_wide-support")]
                    Algorithm::Tang => $series(self.0, mode, SCALE),
                })
            }
            #[inline]
            fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                Self(match resolve::<$N, SCALE>(&self.0) {
                    Algorithm::Series => $series(self.0, mode, SCALE),
                    #[cfg(feature = "_wide-support")]
                    Algorithm::Tang => $series(self.0, mode, SCALE),
                })
            }
            ln_policy_log_family!();
        }
    };
}

/// Emit `impl LnPolicy for D<Int<$N>, SCALE>` for a wide tier carrying a
/// Tang band. `$series` realises `Series`; the `$tang` block (a
/// `match SCALE` over the band(s)) realises `Tang`.
#[cfg(feature = "_wide-support")]
#[allow(unused_macros)]
macro_rules! ln_policy_wide_tang {
    ($T:ident, $N:literal, $series:path, $tang:expr) => {
        impl<const SCALE: u32> LnPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn ln_impl(self, mode: RoundingMode) -> Self {
                let raw = self.0;
                Self(match resolve::<$N, SCALE>(&raw) {
                    Algorithm::Series => $series(raw, mode, SCALE),
                    Algorithm::Tang => ($tang)(raw, mode),
                })
            }
            #[inline]
            fn ln_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                let raw = self.0;
                Self(match resolve::<$N, SCALE>(&raw) {
                    Algorithm::Series => $series(raw, mode, SCALE),
                    Algorithm::Tang => ($tang)(raw, mode),
                })
            }
            ln_policy_log_family!();
        }
    };
}

/// The shared `log` / `log2` / `log10` method bodies for the wide tiers.
/// `log` delegates to `LogPolicy::log_impl` (the `log` seam);
/// `log2` / `log10` delegate to the inherent `*_strict_with` shells
/// (the derived functions compose `ln` with a constant; no raw free-fn today).
#[allow(unused_macros)]
macro_rules! ln_policy_log_family {
    () => {
        #[inline]
        fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
            use crate::policy::log::LogPolicy;
            LogPolicy::log_impl(self, base, mode)
        }
        #[inline]
        fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
            use crate::policy::log::LogPolicy;
            LogPolicy::log_with_impl(self, base, working_digits, mode)
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
    };
}
// D57 — Tang band at SCALE 18..=22.
#[cfg(any(feature = "d57", feature = "wide"))]
ln_policy_wide_tang!(D57, 3, ln::wide_kernel::ln_strict_d57, |raw: Int<3>, mode| -> Int<3> {
    match SCALE {
        18..=22 => ln::lookup_d57_s18_22_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

#[cfg(any(feature = "d76", feature = "wide"))]
ln_policy_wide_series!(D76, 4, ln::wide_kernel::ln_strict_d76);

// D115 — Tang band at SCALE 50..=60.
#[cfg(any(feature = "d115", feature = "wide"))]
ln_policy_wide_tang!(D115, 6, ln::wide_kernel::ln_strict_d115, |raw: Int<6>, mode| -> Int<6> {
    match SCALE {
        50..=60 => ln::lookup_d115_s57_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D153 — Tang band at SCALE 70..=82.
#[cfg(any(feature = "d153", feature = "wide"))]
ln_policy_wide_tang!(D153, 8, ln::wide_kernel::ln_strict_d153, |raw: Int<8>, mode| -> Int<8> {
    match SCALE {
        70..=82 => ln::lookup_d153_s70_82_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D230 — Tang band at SCALE 110..=120.
#[cfg(any(feature = "d230", feature = "wide"))]
ln_policy_wide_tang!(D230, 12, ln::wide_kernel::ln_strict_d230, |raw: Int<12>, mode| -> Int<12> {
    match SCALE {
        110..=120 => ln::lookup_d230_s110_120_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D307 — Tang bands at SCALE 140..=160 (mid) and 285..=295 (deep).
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
ln_policy_wide_tang!(D307, 16, ln::wide_kernel::ln_strict_d307, |raw: Int<16>, mode| -> Int<16> {
    match SCALE {
        140..=160 => ln::lookup_d307_s140_160_tang::ln_strict::<SCALE>(raw, mode),
        285..=295 => ln::lookup_d307_s285_295_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D462 — Tang band at SCALE 225..=235.
#[cfg(any(feature = "d462", feature = "x-wide"))]
ln_policy_wide_tang!(D462, 24, ln::wide_kernel::ln_strict_d462, |raw: Int<24>, mode| -> Int<24> {
    match SCALE {
        225..=235 => ln::lookup_d462_s225_235_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D616 — Tang bands at SCALE 300..=315 (mid) and 585..=595 (deep).
#[cfg(any(feature = "d616", feature = "x-wide"))]
ln_policy_wide_tang!(D616, 32, ln::wide_kernel::ln_strict_d616, |raw: Int<32>, mode| -> Int<32> {
    match SCALE {
        300..=315 => ln::lookup_d616_s300_315_tang::ln_strict::<SCALE>(raw, mode),
        585..=595 => ln::lookup_d616_s585_595_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D924 — Tang bands at SCALE 455..=465 (mid) and 895..=905 (deep).
#[cfg(any(feature = "d924", feature = "xx-wide"))]
ln_policy_wide_tang!(D924, 48, ln::wide_kernel::ln_strict_d924, |raw: Int<48>, mode| -> Int<48> {
    match SCALE {
        455..=465 => ln::lookup_d924_s455_465_tang::ln_strict::<SCALE>(raw, mode),
        895..=905 => ln::lookup_d924_s895_905_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});

// D1232 — Tang bands at SCALE 610..=620 (mid) and 1195..=1205 (deep).
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
ln_policy_wide_tang!(D1232, 64, ln::wide_kernel::ln_strict_d1232, |raw: Int<64>, mode| -> Int<64> {
    match SCALE {
        610..=620 => ln::lookup_d1232_s610_620_tang::ln_strict::<SCALE>(raw, mode),
        1195..=1205 => ln::lookup_d1232_s1195_1205_tang::ln_strict::<SCALE>(raw, mode),
        _ => unreachable!(),
    }
});
