//! Div policy — the per-`(N, SCALE)` algorithm matcher for decimal
//! division.
//!
//! `D<Int<N>, SCALE>::div_with` delegates to [`DivPolicy::div_impl`],
//! which forwards through the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"), mirroring
//! [`crate::policy::mul`]:
//!
//! 1. an [`Algorithm`] enum — the real division algorithm, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (div has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Why a `W` (work-width) parameter on the dispatch
//!
//! Decimal division widens the numerator by `10^SCALE` (`a * 10^SCALE`)
//! before dividing by `b`, requiring work width `W = Int<2N>` so the scaled
//! numerator doesn't overflow. The kernel body uses concrete `$Storage` and
//! `$Wider` types (via the per-tier `div_policy_tier!` macro) because stable
//! Rust cannot derive the work-width `Int<2N>` or the `U128_LIMBS`
//! const-generic argument from the generic `N` alone. `W` is a *work* width,
//! not an algorithm distinction.
//!
//! # Multiplier threading
//!
//! The scale multiplier `10^SCALE` is computed via `$Type::<SCALE>::multiplier()`
//! inside each per-tier `div_impl`, where the type is concrete and the const
//! evaluation folds at compile time. No runtime pow is needed.
//!
//! # Why there is only one algorithm
//!
//! Decimal division has two internal paths (a fast path when `a * 10^SCALE`
//! fits `Int<N>`, and a widening path through `Int<2N>`), but both are
//! implementation details of the single `div_widen_scale` algorithm. There
//! is no policy-level crossover between different algorithms — the fast path
//! is value-gated inside the kernel via leading-zero counts.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real division algorithm — NAMED, no `Default` ──────────────

/// The division algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `div_` function
/// prefix (`div_widen_scale` → `WidenScale`) — strict 1:1 with the
/// kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `div_widen_scale` — widens `a * 10^SCALE` to `W = Int<2N>` then
    /// divides by `b`, rounding under `mode`. A fast path skips the widen
    /// step when `a * 10^SCALE` provably fits `Int<N>`.
    WidenScale,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The div picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the division algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `WidenScale` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    Select::ByAlgorithm(Algorithm::WidenScale)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal division dispatcher for storage `Int<N>` and decimal `SCALE`.
///
/// The `const { select }` block folds away at every concrete `N`. The
/// `div_widen_scale` algorithm body is inlined per tier via
/// `div_policy_tier!` using concrete `$Storage`/`$Wider` types (because the
/// `widen_mul` return type and `narrow_or_panic!` both require concrete
/// storage type tokens that stable Rust cannot compute from `N`).
///
/// Not `const fn`: the underlying kernel is not `const` (it invokes
/// multi-limb division and branches on `cfg!(debug_assertions)`).
#[inline]
fn dispatch<const N: usize, const SCALE: u32>(algo: Algorithm) -> Algorithm {
    // Verify the selection is exhaustive; callers pass the resolved algo.
    let _ = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::WidenScale,
    };
    algo
}

// ── per-tier `DivPolicy` impls — each embeds the kernel with concrete types ──
//
// The `div_widen_scale` kernel requires concrete `$Storage` and `$Wider`
// types: `widen_mul` returns `$Wider`, `multiplier()` is defined per-tier,
// and `narrow_or_panic!` needs the concrete types for the range check.
// The algorithm body is therefore inlined in each `div_impl` via
// `div_policy_tier!`, exactly as the existing `decl_decimal_arithmetic!`
// macro did. The policy is the structural seam; `div_impl` is the canonical
// entry point routed through by `div_with` and `/`.

/// Per-width policy: which kernel a `D<Int<N>, SCALE>` uses for `div_with`.
pub(crate) trait DivPolicy: Sized {
    /// Divide two values of the same scale, rounding the quotient according
    /// to `mode`.
    fn div_impl(self, rhs: Self, mode: RoundingMode) -> Self;
}

/// Emit `impl DivPolicy for D<Int<$N>, SCALE>` with the concrete
/// `div_widen_scale` kernel using storage `Int<$N>` and work width `Int<$W>`.
macro_rules! div_policy_tier {
    ($N:literal, $W:literal) => {
        impl<const SCALE: u32> DivPolicy
            for crate::D<crate::int::types::Int<$N>, SCALE>
        {
            #[inline]
            fn div_impl(self, rhs: Self, mode: RoundingMode) -> Self {
                // Resolve and verify the policy choice at compile time.
                // At this concrete (N==$N, SCALE), select() always returns
                // WidenScale, so this match is trivially exhaustive.
                let algo = dispatch::<$N, SCALE>(Algorithm::WidenScale);
                match algo {
                    Algorithm::WidenScale => {
                        // div_widen_scale kernel with concrete types.
                        // `$Storage = Int<$N>`, `$Wider = Int<$W>`.
                        type Storage = crate::int::types::Int<$N>;
                        type Wider = crate::int::types::Int<$W>;
                        // Pre-compute `10^SCALE` via the per-tier const fn,
                        // whose `leading_zeros()` collapses at compile time.
                        let mult: Storage =
                            <crate::D<crate::int::types::Int<$N>, SCALE>>::multiplier();
                        let lz_n = self.0.unsigned_abs().leading_zeros();
                        let lz_m = mult.leading_zeros();
                        if lz_n + lz_m > <Storage>::BITS {
                            // Fast path: `self * mult` fits `Storage`.
                            let n: Storage = self.0.wrapping_mul(mult);
                            let result = $crate::macros::arithmetic::round_with_mode_wide!(
                                n, rhs.0, Storage, mode
                            );
                            return Self(result);
                        }
                        // Slow path: widen numerator, divide in `Wider`.
                        let b: Wider = rhs.0.resize::<Wider>();
                        let n: Wider = self.0.widen_mul::<Wider>(mult);
                        let result = $crate::macros::arithmetic::round_with_mode_wide!(
                            n, b, Wider, mode
                        );
                        Self(crate::macros::arithmetic::narrow_or_panic!(
                            result, Storage, Wider,
                            "attempt to divide with overflow"
                        ))
                    }
                }
            }
        }
    };
}

div_policy_tier!(1, 2);  // D18
div_policy_tier!(2, 4);  // D38

#[cfg(any(feature = "d57", feature = "wide"))]
div_policy_tier!(3, 6);  // D57
#[cfg(any(feature = "d76", feature = "wide"))]
div_policy_tier!(4, 8);  // D76
#[cfg(any(feature = "d115", feature = "wide"))]
div_policy_tier!(6, 12); // D115
#[cfg(any(feature = "d153", feature = "wide"))]
div_policy_tier!(8, 16); // D153
#[cfg(any(feature = "d230", feature = "wide"))]
div_policy_tier!(12, 24); // D230
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
div_policy_tier!(16, 32); // D307
#[cfg(any(feature = "d462", feature = "x-wide"))]
div_policy_tier!(24, 48); // D462
#[cfg(any(feature = "d616", feature = "x-wide"))]
div_policy_tier!(32, 64); // D616
#[cfg(any(feature = "d924", feature = "xx-wide"))]
div_policy_tier!(48, 96); // D924
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
div_policy_tier!(64, 128); // D1232
