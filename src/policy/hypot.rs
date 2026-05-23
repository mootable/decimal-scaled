//! Hypot policy — the per-`(N, SCALE)` algorithm matcher for
//! `sqrt(self² + other²)` (hypotenuse without intermediate overflow).
//!
//! See `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! `D<Int<N>, SCALE>::hypot_strict_with(other, mode)` delegates to
//! [`HypotPolicy::hypot_impl`], which follows the canonical policy shape:
//!
//! 1. an [`Algorithm`] enum — the real hypot algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (`hypot` has no value split so `ByValue` is never returned);
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
//! The single algorithm (`hypot_scale_trick`) computes
//! `max(|a|,|b|) · sqrt(1 + (min/max)²)` — the scale-ratio trick that
//! keeps the intermediate in `[1, 2]`, preventing overflow in the radicand.
//! It delegates to `SqrtPolicy::sqrt_impl` (the decimal sqrt surface) and
//! the tier's own arithmetic operators (`/`, `*`, `+`) — both are already
//! policy-dispatched for the tier's `(N, SCALE)`. See §1a (RULES.md):
//! "Decimal algorithms use `Int<N>` / BigInt METHODS wherever possible" and
//! "Stay in your tier — no cross-tier shortcuts."
//!
//! `hypot(0, 0) = 0` (bit-exact); `hypot(0, x) = |x|` (the inner sqrt of
//! `1 + 0` is exactly 1).

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

// ── 1. the real hypot algorithm — NAMED, no `Default` ────────────────

/// The hypot algorithms this policy chooses between. The single variant is
/// the CamelCase of the kernel name minus the `hypot_` prefix — strict 1:1
/// with the kernel computation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `hypot_scale_trick` — `max(|a|, |b|) · sqrt(1 + (min/max)²)`.
    /// The ratio `min/max ∈ [0,1]` keeps `ratio² + 1 ∈ [1, 2]`, so the
    /// inner sqrt never overflows. The outer multiply overflows only when
    /// the true hypotenuse exceeds the type's range. Delegates to the
    /// `SqrtPolicy` surface (`sqrt_strict_with`) and the tier's arithmetic
    /// operators — no raw integer arithmetic, no cross-tier reach.
    ScaleTrick,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The hypot picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the hypot algorithm for storage limb count `N` and decimal `SCALE`.
/// Total over the key; `ScaleTrick` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE); // keys accepted for uniformity; one algorithm
    Select::ByAlgorithm(Algorithm::ScaleTrick)
}

// ── per-type `HypotPolicy` trait ──────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for
/// `hypot_strict_with`.
pub(crate) trait HypotPolicy: Sized {
    /// `sqrt(self² + other²)` without intermediate overflow, under the
    /// supplied rounding mode (applied to the inner sqrt step).
    fn hypot_impl(self, other: Self, mode: RoundingMode) -> Self;
}

// ── D18 ── widen-to-D38, call D38's `hypot_impl`, narrow back ─────────
//
// D18 has no native sqrt kernel above the D38 `Fixed` work width.
// Widening to D38 and calling D38's `HypotPolicy::hypot_impl` is the
// narrower-tier strategy (the same approach as D18's `LnPolicy`, etc.).
impl<const SCALE: u32> HypotPolicy for D18<SCALE> {
    #[inline]
    fn hypot_impl(self, other: Self, mode: RoundingMode) -> Self {
        let algo = match const { select::<1, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::ScaleTrick,
        };
        match algo {
            Algorithm::ScaleTrick => {
                let wide: D38<SCALE> = self.into();
                let wide_other: D38<SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide.hypot_strict_with(wide_other, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            "D18::hypot",
                            SCALE,
                        )
                    })
            }
        }
    }
}

// ── D38 ── scale-trick via the decimal operator surface ───────────────
//
// The `ScaleTrick` kernel is implemented directly here (not via
// `hypot_strict_with`, which would be circular since `hypot_strict_with`
// delegates here). The computation uses only the tier's decimal operators
// (`abs`, `>=`, `/`, `*`) and `sqrt_strict_with` — the canonical §1a
// "use Int<N> / BigInt methods" approach at the decimal layer.
impl<const SCALE: u32> HypotPolicy for D38<SCALE> {
    #[inline]
    fn hypot_impl(self, other: Self, mode: RoundingMode) -> Self {
        let algo = match const { select::<2, SCALE>() } {
            Select::ByAlgorithm(a) => a,
            Select::ByValue(_) => Algorithm::ScaleTrick,
        };
        match algo {
            Algorithm::ScaleTrick => hypot_scale_trick_d38(self, other, mode),
        }
    }
}

/// The scale-trick hypot computation for D38: `max·sqrt(1+(min/max)²)`.
/// Uses the decimal operator surface (abs, >=, /, *, sqrt_strict_with) and
/// the SqrtPolicy surface — no raw integer arithmetic, no cross-tier reach.
#[inline]
fn hypot_scale_trick_d38<const SCALE: u32>(
    a: D38<SCALE>,
    b: D38<SCALE>,
    mode: RoundingMode,
) -> D38<SCALE> {
    let a = a.abs();
    let b = b.abs();
    let (large, small) = if a >= b { (a, b) } else { (b, a) };
    if large == D38::<SCALE>::ZERO {
        D38::<SCALE>::ZERO
    } else {
        let ratio = small / large;
        let one_plus_sq = D38::<SCALE>::ONE + ratio * ratio;
        large * one_plus_sq.sqrt_strict_with(mode)
    }
}

// ── Wide tiers — scale-trick via the inherent `hypot_strict_with` shell ─
//
// Every wide tier emits `hypot_strict_with` via `decl_wide_roots!` in
// `src/macros/wide_roots.rs`. That method IS the `ScaleTrick` realisation
// for the tier — it uses the same ratio-sqrt composition as D38 but on the
// wider operators. The policy delegates to it without duplicating the body.

/// Emit `impl HypotPolicy for D<Int<$N>, SCALE>` for a wide tier.
#[allow(unused_macros)]
macro_rules! hypot_policy_wide {
    ($T:ident, $N:literal) => {
        impl<const SCALE: u32> HypotPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn hypot_impl(self, other: Self, mode: RoundingMode) -> Self {
                let algo = match const { select::<$N, SCALE>() } {
                    Select::ByAlgorithm(a) => a,
                    Select::ByValue(_) => Algorithm::ScaleTrick,
                };
                match algo {
                    Algorithm::ScaleTrick => self.hypot_strict_with(other, mode),
                }
            }
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
hypot_policy_wide!(D57, 3);
#[cfg(any(feature = "d76", feature = "wide"))]
hypot_policy_wide!(D76, 4);
#[cfg(any(feature = "d115", feature = "wide"))]
hypot_policy_wide!(D115, 6);
#[cfg(any(feature = "d153", feature = "wide"))]
hypot_policy_wide!(D153, 8);
#[cfg(any(feature = "d230", feature = "wide"))]
hypot_policy_wide!(D230, 12);
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
hypot_policy_wide!(D307, 16);
#[cfg(any(feature = "d462", feature = "x-wide"))]
hypot_policy_wide!(D462, 24);
#[cfg(any(feature = "d616", feature = "x-wide"))]
hypot_policy_wide!(D616, 32);
#[cfg(any(feature = "d924", feature = "xx-wide"))]
hypot_policy_wide!(D924, 48);
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
hypot_policy_wide!(D1232, 64);
