//! To-degrees policy — the per-`(N, SCALE)` algorithm matcher for
//! radians-to-degrees angle conversion.
//!
//! See `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! `D<Int<N>, SCALE>::to_degrees_strict_with(mode)` delegates to
//! [`ToDegreesPolicy::to_degrees_impl`], which follows the canonical policy
//! shape:
//!
//! 1. an [`Algorithm`] enum — the real to-degrees algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (`to_degrees` has no value split so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Algorithm
//!
//! The single algorithm (`to_degrees_mul_pi_ratio`) multiplies by
//! `180 / π` (exact rational approximation in the guard-digit intermediate)
//! via the `trig::trig_series_2limb::to_degrees_strict` kernel (narrow tier) or the
//! inherent `to_degrees_strict_with` shell (wide tiers). There is no
//! crossover threshold — one computation everywhere.
//!
//! # Relationship to `trig.rs`
//!
//! `to_degrees` is one of the angle-conversion methods bundled under
//! `TrigPolicy`. This dedicated file is the structural seam for the
//! `to_degrees` operation specifically; `TrigPolicy::to_degrees_impl`
//! delegates here, and this policy delegates to the existing trig kernel
//! surface (§1a of RULES.md). Do NOT touch the trig kernels in
//! `src/algos/trig/`.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real to-degrees algorithm — NAMED, no `Default` ───────────

/// The to-degrees algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel name minus the `to_degrees_` prefix —
/// strict 1:1.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `to_degrees_mul_pi_ratio` — multiply by `180/π` via the guard-digit
    /// `Fixed` intermediate. Realised by `trig::trig_series_2limb::to_degrees_strict`
    /// (narrow tier) and the inherent `to_degrees_strict_with` shell (wide
    /// tiers). One computation, all tiers and scales.
    MulPiRatio,
    /// `to_degrees_schoolbook` -- naive textbook reference (multiply
    /// by the pi ratio in the work int). UNROUTED: `select` never
    /// returns it; correctness reference + microbench partner.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The to-degrees picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the to-degrees algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `MulPiRatio` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE); // keys accepted for uniformity; one algorithm
    Select::ByAlgorithm(Algorithm::MulPiRatio)
}

// ── per-type `ToDegreesPolicy` trait ─────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for
/// `to_degrees_strict_with`.
pub(crate) trait ToDegreesPolicy: Sized {
    /// Convert radians to degrees under the supplied rounding mode (strict
    /// guard width).
    fn to_degrees_impl(self, mode: RoundingMode) -> Self;

    /// Convert radians to degrees with caller-chosen guard digits and
    /// rounding mode.
    fn to_degrees_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ── D18 ── widen-to-D38, call D38's to_degrees shell, narrow back ─────
impl<const SCALE: u32> ToDegreesPolicy for crate::D<crate::int::types::Int<1>, SCALE> {
    #[inline]
    fn to_degrees_impl(self, mode: RoundingMode) -> Self {
        let algo = match const { select::<1, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::MulPiRatio,
        };
        match algo {
            Algorithm::MulPiRatio => {
                let wide: crate::D<crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict_with(mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            "D18::to_degrees",
                            SCALE,
                        )
                    })
            }
            Algorithm::Schoolbook => {
                let wide: crate::D<crate::int::types::Int<2>, SCALE> = self.into();
                let r = crate::algos::trig::angle_schoolbook::to_degrees_schoolbook_narrow::<SCALE>(wide.0, mode);
                ::core::convert::TryInto::try_into(crate::D::<crate::int::types::Int<2>, SCALE>(r))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            "D18::to_degrees",
                            SCALE,
                        )
                    })
            }
        }
    }

    #[inline]
    fn to_degrees_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        let algo = match const { select::<1, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::MulPiRatio,
        };
        match algo {
            Algorithm::MulPiRatio => {
                let wide: crate::D<crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            "D18::to_degrees",
                            SCALE,
                        )
                    })
            }
            Algorithm::Schoolbook => {
                let wide: crate::D<crate::int::types::Int<2>, SCALE> = self.into();
                let r = crate::algos::trig::angle_schoolbook::to_degrees_schoolbook_narrow::<SCALE>(wide.0, mode);
                ::core::convert::TryInto::try_into(crate::D::<crate::int::types::Int<2>, SCALE>(r))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale("D18::to_degrees", SCALE)
                    })
            }
        }
    }
}

// ── D38 ── native `trig_series_2limb` kernel ─────────────────────────────────
impl<const SCALE: u32> ToDegreesPolicy for crate::D<crate::int::types::Int<2>, SCALE> {
    #[inline]
    fn to_degrees_impl(self, mode: RoundingMode) -> Self {
        let algo = match const { select::<2, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::MulPiRatio,
        };
        match algo {
            Algorithm::MulPiRatio => {
                Self(crate::algos::trig::trig_series_2limb::to_degrees_strict::<SCALE>(self.0, mode))
            }
            Algorithm::Schoolbook => {
                Self(crate::algos::trig::angle_schoolbook::to_degrees_schoolbook_narrow::<SCALE>(self.0, mode))
            }
        }
    }

    #[inline]
    fn to_degrees_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        let algo = match const { select::<2, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::MulPiRatio,
        };
        match algo {
            Algorithm::MulPiRatio => Self(crate::algos::trig::trig_series_2limb::to_degrees_with(
                self.0,
                SCALE,
                working_digits,
                mode,
            )),
            Algorithm::Schoolbook => Self(crate::algos::trig::angle_schoolbook::to_degrees_schoolbook_narrow::<SCALE>(self.0, mode)),
        }
    }
}

// ── Wide tiers — delegate to the inherent `to_degrees_strict_with` shell ─

/// Emit `impl ToDegreesPolicy for D<Int<$N>, SCALE>` for a wide tier.
#[allow(unused_macros)]
macro_rules! to_degrees_policy_wide {
    ($T:ident, $N:literal, $Core:ty) => {
        impl<const SCALE: u32> ToDegreesPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn to_degrees_impl(self, mode: RoundingMode) -> Self {
                let algo = match const { select::<$N, SCALE>() } {
                    Select::ByAlgorithm(a) => a,
                    Select::ByValue(_) => Algorithm::MulPiRatio,
                };
                match algo {
                    Algorithm::MulPiRatio => Self::from_bits(crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<$Core, SCALE>(self.to_bits(), mode)),
                    Algorithm::Schoolbook => Self::from_bits(crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<$Core, SCALE>(self.to_bits(), mode)),
                }
            }

            #[inline]
            fn to_degrees_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                let algo = match const { select::<$N, SCALE>() } {
                    Select::ByAlgorithm(a) => a,
                    Select::ByValue(_) => Algorithm::MulPiRatio,
                };
                match algo {
                    Algorithm::MulPiRatio => Self::from_bits(crate::algos::trig::angle_mul_pi_ratio::to_degrees_mul_pi_ratio::<$Core, SCALE>(self.to_bits(), mode)),
                    Algorithm::Schoolbook => Self::from_bits(crate::algos::trig::angle_schoolbook::to_degrees_schoolbook::<$Core, SCALE>(self.to_bits(), mode)),
                }
            }
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
to_degrees_policy_wide!(D57, 3, crate::types::widths::wide_trig_d57::Core);
#[cfg(any(feature = "d76", feature = "wide"))]
to_degrees_policy_wide!(D76, 4, crate::types::widths::wide_trig_d76::Core);
#[cfg(any(feature = "d115", feature = "wide"))]
to_degrees_policy_wide!(D115, 6, crate::types::widths::wide_trig_d115::Core);
#[cfg(any(feature = "d153", feature = "wide"))]
to_degrees_policy_wide!(D153, 8, crate::types::widths::wide_trig_d153::Core);
#[cfg(any(feature = "d230", feature = "wide"))]
to_degrees_policy_wide!(D230, 12, crate::types::widths::wide_trig_d230::Core);
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
to_degrees_policy_wide!(D307, 16, crate::types::widths::wide_trig_d307::Core);
#[cfg(any(feature = "d462", feature = "x-wide"))]
to_degrees_policy_wide!(D462, 24, crate::types::widths::wide_trig_d462::Core);
#[cfg(any(feature = "d616", feature = "x-wide"))]
to_degrees_policy_wide!(D616, 32, crate::types::widths::wide_trig_d616::Core);
#[cfg(any(feature = "d924", feature = "xx-wide"))]
to_degrees_policy_wide!(D924, 48, crate::types::widths::wide_trig_d924::Core);
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
to_degrees_policy_wide!(D1232, 64, crate::types::widths::wide_trig_d1232::Core);
