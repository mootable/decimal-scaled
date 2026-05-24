//! Hypot policy — the per-`(N, SCALE)` algorithm matcher for
//! `sqrt(self² + other²)` (hypotenuse without intermediate overflow).
//!
//! See `docs/ARCHITECTURE.md` → "Policy file structure".
//!
//! `D<Int<N>, SCALE>::hypot_strict_with(other, mode)` delegates to
//! [`HypotPolicy::hypot_impl`], which forwards to the one shared
//! [`dispatch`] generic function. `dispatch` follows the canonical policy
//! shape (mirroring [`crate::policy::sqrt`]):
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
//! The single algorithm ([`crate::algos::hypot::hypot_isqrt`]) forms the
//! radicand `a² + b²` in a wider work integer `W` and takes the root via
//! the integer-layer [`crate::int::types::traits::BigInt::isqrt`] — the
//! same int `isqrt` dispatch [`crate::policy::sqrt`] uses. The root goes
//! **down** to the integer layer; the kernel never calls the decimal
//! `sqrt` surface on the tier's own value.
//!
//! # Why a `W` (work-width) parameter on the dispatch
//!
//! `hypot_isqrt` forms `a² + b²` in a next-up work width `W = Int<2N>`.
//! Computing `Int<2N>` from `N` generically needs `generic_const_exprs`
//! (nightly, forbidden on stable), so the concrete `W` is supplied by each
//! storage tier's `hypot_impl` and threaded through the dispatch — exactly
//! as [`crate::policy::sqrt`] threads its Newton work width.
//!
//! `hypot(0, 0) = 0` (bit-exact); `hypot(0, x) = |x|` (`isqrt(x²) = |x|`).

use crate::algos::hypot;
use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses for
/// `hypot_strict_with`.
pub(crate) trait HypotPolicy: Sized {
    /// `sqrt(self² + other²)` without intermediate overflow, under the
    /// supplied rounding mode (applied to the root step).
    fn hypot_impl(self, other: Self, mode: RoundingMode) -> Self;
}

// ── 1. the real hypot algorithm — NAMED, no `Default` ────────────────

/// The hypot algorithms this policy chooses between. The single variant is
/// the CamelCase of the kernel name minus the `hypot_` prefix — strict 1:1
/// with the kernel computation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`hypot::hypot_isqrt::hypot_isqrt`] — `round(sqrt(a² + b²))` over a
    /// work width `W` covering `a² + b²`, taking the floor root through
    /// the integer-layer `isqrt`. The generic default for every tier.
    Isqrt,
    /// Schoolbook reference tag -- delegates to the same
    /// [`hypot::hypot_isqrt::hypot_isqrt`] kernel. `hypot_isqrt` IS the
    /// schoolbook form (`a² + b²` in `W`, then `W::isqrt`); this variant
    /// exists as an explicit benchmarkable seam.
    #[allow(dead_code)]
    Schoolbook,
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
/// Total over the key; `Isqrt` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE); // keys accepted for uniformity; one algorithm
    Select::ByAlgorithm(Algorithm::Isqrt)
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Shared hypot dispatch for storage `Int<N>` and `hypot_isqrt` work width
/// `W`. `W` is the next-up work width covering `a² + b²` (`Int<2N>`),
/// supplied by the caller because `Int<2N>` is not computable from `N` on
/// stable. Negative inputs are handled by squaring (sign drops out).
#[inline]
#[must_use]
fn dispatch<const N: usize, const SCALE: u32, W>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    W: BigInt,
{
    // Both operands carry the same `10^SCALE`, so it divides out of the
    // root; `SCALE` is used only to label the out-of-range panic.
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&a),
    };
    match algo {
        Algorithm::Isqrt => hypot::hypot_isqrt::hypot_isqrt::<Int<N>, W>(a, b, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale("hypot", SCALE)
            }),
        Algorithm::Schoolbook => hypot::hypot_isqrt::hypot_isqrt::<Int<N>, W>(a, b, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale("hypot", SCALE)
            }),
    }
}

// ── per-tier `HypotPolicy` impls — each binds its concrete work width ──
//
// Every impl forwards to the one `dispatch`; the only per-tier datum is
// the work width `W = Int<2N>`. The dispatch's `const { select }` block
// folds away the unreachable arms for each tier. This mirrors
// `crate::policy::sqrt` exactly.

/// Emit `impl HypotPolicy for D<Int<$N>, SCALE>` forwarding to
/// [`dispatch`] with the tier's `hypot_isqrt` work width `Int<$W>`.
macro_rules! hypot_policy_tier {
    ($N:literal, $W:literal) => {
        impl<const SCALE: u32> HypotPolicy
            for crate::D<crate::int::types::Int<$N>, SCALE>
        {
            #[inline]
            fn hypot_impl(self, other: Self, mode: RoundingMode) -> Self {
                Self(dispatch::<$N, SCALE, Int<$W>>(self.0, other.0, mode))
            }
        }
    };
}

// D18 / D38 — work width `Int<2N>` holds `a² + b²` exactly.
hypot_policy_tier!(1, 2); // D18
hypot_policy_tier!(2, 4); // D38

// Wide tiers: `W = Int<2N>` is the radicand work width.
#[cfg(any(feature = "d57", feature = "wide"))]
hypot_policy_tier!(3, 6); // D57
#[cfg(any(feature = "d76", feature = "wide"))]
hypot_policy_tier!(4, 8); // D76
#[cfg(any(feature = "d115", feature = "wide"))]
hypot_policy_tier!(6, 12); // D115
#[cfg(any(feature = "d153", feature = "wide"))]
hypot_policy_tier!(8, 16); // D153
#[cfg(any(feature = "d230", feature = "wide"))]
hypot_policy_tier!(12, 24); // D230
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
hypot_policy_tier!(16, 32); // D307
#[cfg(any(feature = "d462", feature = "x-wide"))]
hypot_policy_tier!(24, 48); // D462
#[cfg(any(feature = "d616", feature = "x-wide"))]
hypot_policy_tier!(32, 64); // D616
#[cfg(any(feature = "d924", feature = "xx-wide"))]
hypot_policy_tier!(48, 96); // D924
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
hypot_policy_tier!(64, 128); // D1232
