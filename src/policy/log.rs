//! Log-base policy — the per-`(N, SCALE)` algorithm matcher for the
//! arbitrary-base decimal logarithm `log(self, base)`.
//!
//! See `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! `D<Int<N>, SCALE>::log_strict_with(base, mode)` delegates to
//! [`LogPolicy::log_impl`], which forwards to the canonical policy shape:
//!
//! 1. an [`Algorithm`] enum — the real log-base algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (`log` has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Why one algorithm
//!
//! `log(self, base) = ln(self) / ln(base)`. Every tier and scale uses the
//! same ratio-of-natural-logs computation: the narrow tier runs through the
//! `ln::fixed_d38::log_strict` kernel; the wide tiers run through the
//! per-tier `$core::log_strict_with_kernel` / `log_approx_with_kernel` free
//! functions (emitted by `decl_wide_transcendental!`). There is no
//! crossover threshold that selects a different algorithm at the policy
//! level — `LnDivide` is the one algorithm serving all cells. `ByValue` is
//! present for canonical-shape uniformity; `select` never returns it.
//!
//! # Relationship to `ln.rs`
//!
//! `log` is derived from `ln` (§1a: a decimal algorithm calling another
//! decimal algorithm's surface). The `log` policy is the structural seam
//! for the `log` operation; it delegates to the `LnPolicy` surface (`ln`
//! kernel + divide) rather than re-implementing `ln`. Do NOT touch
//! `src/algos/{exp,ln,trig,pow}` kernels — this policy calls the existing
//! `ln` SURFACE.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real log-base algorithm — NAMED, no `Default` ─────────────

/// The log-base algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel's conceptual name — strict 1:1 with the
/// computation (`log_ln_divide` → `LnDivide`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `log_ln_divide` — `log(self, base) = ln(self) / ln(base)`. The
    /// narrow tier routes through `ln::fixed_d38::log_strict`; the wide
    /// tiers route through the per-tier `$core::log_strict_with_kernel` /
    /// `log_approx_with_kernel` free functions. This is the only formula
    /// used everywhere.
    LnDivide,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The log picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the log-base algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `LnDivide` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE); // keys accepted for uniformity; one algorithm
    Select::ByAlgorithm(Algorithm::LnDivide)
}

// ── per-type `LogPolicy` trait ────────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for `log`.
pub(crate) trait LogPolicy: Sized {
    /// `log(self, base)` under the supplied rounding mode (strict guard).
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self;

    /// `log(self, base)` with caller-chosen guard digits and rounding mode.
    fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ── D18 ── widen-to-D38, call D38's log, narrow back ─────────────────
//
// D18 has no native log kernel. It widens to the D38 `Fixed` work width,
// delegates to D38's `LnPolicy::log_impl`, then narrows the result back.
impl<const SCALE: u32> LogPolicy for crate::D<crate::int::types::Int<1>, SCALE> {
    #[inline]
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
        let algo = match const { select::<1, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::LnDivide,
        };
        match algo {
            Algorithm::LnDivide => {
                let wide: crate::D<crate::int::types::Int<2>, SCALE> = self.into();
                let wbase: crate::D<crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide.log_strict_with(wbase, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            "D18::log",
                            SCALE,
                        )
                    })
            }
        }
    }

    #[inline]
    fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
        let algo = match const { select::<1, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::LnDivide,
        };
        match algo {
            Algorithm::LnDivide => {
                let wide: crate::D<crate::int::types::Int<2>, SCALE> = self.into();
                let wbase: crate::D<crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide.log_approx_with(wbase, working_digits, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            "D18::log",
                            SCALE,
                        )
                    })
            }
        }
    }
}

// ── D38 ── native `Fixed`-256 kernel via `ln::fixed_d38::log_strict` ─
impl<const SCALE: u32> LogPolicy for crate::D<crate::int::types::Int<2>, SCALE> {
    #[inline]
    fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
        let algo = match const { select::<2, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::LnDivide,
        };
        match algo {
            Algorithm::LnDivide => {
                Self(crate::algos::ln::fixed_d38::log_strict::<SCALE>(self.0, base.0, mode))
            }
        }
    }

    #[inline]
    fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
        let algo = match const { select::<2, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::LnDivide,
        };
        match algo {
            Algorithm::LnDivide => Self(crate::algos::ln::fixed_d38::log_with(
                self.0,
                base.0,
                SCALE,
                working_digits,
                mode,
            )),
        }
    }
}

// ── Wide tiers — call the `LnDivide` algorithm kernel directly ────────
//
// The `LnDivide` realisation for each wide tier is a pair of free
// kernels emitted into the tier's per-tier `$core` module by
// `decl_wide_transcendental!` (`log_strict_with_kernel` /
// `log_approx_with_kernel`). They hold the real computation — exact-power
// pin + directed-rounding Ziv escalation. The policy calls them *down*;
// the inherent `log_strict_with` / `log_approx_with` methods delegate
// *down* to this policy. No inversion, no loop: the impl lives in the
// algorithm, the method is a thin delegator.

/// Emit `impl LogPolicy for D<Int<$N>, SCALE>` for a wide tier.
/// `$core_mod` is the leaf ident of the tier's per-tier
/// transcendental-core module (e.g. `wide_trig_d57`); it lives at
/// `crate::types::widths::$core_mod` and carries the `LnDivide` kernels.
#[allow(unused_macros)]
macro_rules! log_policy_wide {
    ($T:ident, $N:literal, $core_mod:ident) => {
        impl<const SCALE: u32> LogPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn log_impl(self, base: Self, mode: RoundingMode) -> Self {
                let algo = match const { select::<$N, SCALE>() } {
                    Select::ByAlgorithm(a) => a,
                    Select::ByValue(_) => Algorithm::LnDivide,
                };
                match algo {
                    Algorithm::LnDivide => Self::from_bits(
                        crate::types::widths::$core_mod::log_strict_with_kernel::<SCALE>(
                            self.to_bits(),
                            base.to_bits(),
                            mode,
                        ),
                    ),
                }
            }

            #[inline]
            fn log_with_impl(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self {
                let algo = match const { select::<$N, SCALE>() } {
                    Select::ByAlgorithm(a) => a,
                    Select::ByValue(_) => Algorithm::LnDivide,
                };
                match algo {
                    Algorithm::LnDivide => {
                        // `working_digits == GUARD` reproduces the strict
                        // path exactly (the prior `_approx_with` short-circuit).
                        if working_digits == crate::types::widths::$core_mod::GUARD {
                            Self::from_bits(
                                crate::types::widths::$core_mod::log_strict_with_kernel::<SCALE>(
                                    self.to_bits(),
                                    base.to_bits(),
                                    mode,
                                ),
                            )
                        } else {
                            Self::from_bits(
                                crate::types::widths::$core_mod::log_approx_with_kernel::<SCALE>(
                                    self.to_bits(),
                                    base.to_bits(),
                                    working_digits,
                                    mode,
                                ),
                            )
                        }
                    }
                }
            }
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
log_policy_wide!(D57, 3, wide_trig_d57);
#[cfg(any(feature = "d76", feature = "wide"))]
log_policy_wide!(D76, 4, wide_trig_d76);
#[cfg(any(feature = "d115", feature = "wide"))]
log_policy_wide!(D115, 6, wide_trig_d115);
#[cfg(any(feature = "d153", feature = "wide"))]
log_policy_wide!(D153, 8, wide_trig_d153);
#[cfg(any(feature = "d230", feature = "wide"))]
log_policy_wide!(D230, 12, wide_trig_d230);
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
log_policy_wide!(D307, 16, wide_trig_d307);
#[cfg(any(feature = "d462", feature = "x-wide"))]
log_policy_wide!(D462, 24, wide_trig_d462);
#[cfg(any(feature = "d616", feature = "x-wide"))]
log_policy_wide!(D616, 32, wide_trig_d616);
#[cfg(any(feature = "d924", feature = "xx-wide"))]
log_policy_wide!(D924, 48, wide_trig_d924);
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
log_policy_wide!(D1232, 64, wide_trig_d1232);
