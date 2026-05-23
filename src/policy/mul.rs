//! Mul policy — the per-`(N, SCALE)` algorithm matcher for decimal
//! multiplication.
//!
//! `D<Int<N>, SCALE>::mul_with` delegates to [`MulPolicy::mul_impl`],
//! which forwards to the one shared [`dispatch`] generic function.
//! `dispatch` follows the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"):
//!
//! 1. an [`Algorithm`] enum — the real multiply algorithm, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (mul has no value split, so `ByValue` is never returned);
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
//! Decimal multiplication widens `a * b` into a next-up work width `W =
//! Int<2N>` before dividing by `10^SCALE` to return to the `Int<N>` scale.
//! Computing `Int<2N>` from `N` generically needs `generic_const_exprs`
//! (nightly, forbidden on stable), so the concrete `W` is supplied by each
//! storage tier's `mul_impl` and the kernel body uses `$Storage`/`$Wider`
//! concrete types (via the per-tier macro). `W` is a *work* width, not an
//! algorithm distinction — `mul_widen_divide` is one algorithm serving all
//! tiers; only the concrete type tokens differ per tier.
//!
//! # Why there is only one algorithm
//!
//! Decimal multiply has two internal paths (a fast path when the product
//! fits `Int<N>`, and a widening path through `Int<2N>`), but both are
//! purely implementation details of the single `mul_widen_divide` algorithm.
//! There is no crossover threshold that selects a *different algorithm* at
//! the policy level — the fast path is value-gated inside the kernel, not a
//! separate `Algorithm` variant.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real multiply algorithm — NAMED, no `Default` ──────────────

/// The multiply algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `mul_` function
/// prefix (`mul_widen_divide` → `WidenDivide`) — strict 1:1 with the
/// kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `mul_widen_divide` — widens `a * b` to `W = Int<2N>`, divides the
    /// product by `10^SCALE` using the MG / Newton path based on `SCALE`,
    /// then narrows back to `Int<N>`. A fast path skips the widen step when
    /// the unsigned-magnitude leading-zero count proves the product fits
    /// `Int<N>` exactly.
    WidenDivide,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The mul picker always
/// returns `ByAlgorithm`. `ByValue` is part of the canonical shape for
/// uniformity; `select` never returns it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>, &Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the multiply algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; `WidenDivide` wins at every `(N, SCALE)`.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    Select::ByAlgorithm(Algorithm::WidenDivide)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal multiply dispatcher for storage `Int<N>` and decimal `SCALE`.
///
/// The `const { select }` block folds away at every concrete `N`. The
/// `mul_widen_divide` algorithm body is inlined per tier via
/// `mul_policy_tier!` using concrete `$Storage`/`$Wider` types (because the
/// mg-divide and Newton-reciprocal kernels require concrete `U128_LIMBS`
/// const-generic arguments that stable Rust cannot compute from `N`).
///
/// Not `const fn`: the underlying kernel is not `const` (it branches on
/// `cfg!(debug_assertions)` and routes through multi-limb division).
#[inline]
fn dispatch<const N: usize, const SCALE: u32>(algo: Algorithm) -> Algorithm {
    // Verify the selection is exhaustive; callers pass the resolved algo.
    let _ = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(_) => Algorithm::WidenDivide,
    };
    algo
}

// ── per-tier `MulPolicy` impls — each embeds the kernel with concrete types ──
//
// The `mul_widen_divide` kernel requires concrete `$Storage` and `$Wider`
// types to supply the `U128_LIMBS` const-generic argument to the MG/Newton
// functions (stable Rust cannot compute this from `N`). The algorithm body
// is therefore inlined in each `mul_impl` via `mul_policy_tier!`, exactly
// as the existing `decl_decimal_arithmetic!(wide …)` macro did. The policy
// is the structural seam; `mul_impl` is the canonical entry point routed
// through by `mul_with` and `*`.
//
// The `const { select::<N, SCALE>() }` fold happens in `dispatch` but the
// single-algorithm result is known at compile time — WidenDivide is selected
// at every (N, SCALE). The `match algo` in each `mul_impl` below is still
// exhaustive over `Algorithm` (single arm, no `_`).

/// Per-width policy: which kernel a `D<Int<N>, SCALE>` uses for `mul_with`.
pub(crate) trait MulPolicy: Sized {
    /// Multiply two values of the same scale, rounding the scale-narrowing
    /// step according to `mode`.
    fn mul_impl(self, rhs: Self, mode: RoundingMode) -> Self;
}

/// Emit `impl MulPolicy for D<Int<$N>, SCALE>` with the concrete
/// `mul_widen_divide` kernel using storage `Int<$N>` and work width
/// `Int<$W>`.
macro_rules! mul_policy_tier {
    ($N:literal, $W:literal) => {
        impl<const SCALE: u32> MulPolicy
            for crate::D<crate::int::types::Int<$N>, SCALE>
        {
            #[inline]
            fn mul_impl(self, rhs: Self, mode: RoundingMode) -> Self {
                // Resolve and verify the policy choice at compile time.
                // At this concrete (N==$N, SCALE), select() always returns
                // WidenDivide, so this match is trivially exhaustive.
                let algo = dispatch::<$N, SCALE>(Algorithm::WidenDivide);
                match algo {
                    Algorithm::WidenDivide => {
                        // mul_widen_divide kernel with concrete types.
                        // `$Storage = Int<$N>`, `$Wider = Int<$W>`.
                        type Storage = crate::int::types::Int<$N>;
                        type Wider = crate::int::types::Int<$W>;
                        let lz_a = self.0.unsigned_abs().leading_zeros();
                        let lz_b = rhs.0.unsigned_abs().leading_zeros();
                        if lz_a + lz_b > <Storage>::BITS {
                            let n: Storage = self.0.wrapping_mul(rhs.0);
                            let scaled = if SCALE == 0 {
                                n
                            } else if SCALE <= 38 {
                                crate::algos::support::mg_divide::div_wide_pow10_with::<
                                    Storage,
                                    { <Storage as crate::int::types::traits::BigInt>::U128_LIMBS },
                                >(n, SCALE, mode)
                            } else {
                                crate::algos::support::newton_reciprocal::dispatch_wide_pow10_with::<
                                    Storage,
                                    { <Storage as crate::int::types::traits::BigInt>::U128_LIMBS },
                                >(n, SCALE, mode)
                            };
                            return Self(scaled);
                        }
                        let n: Wider = self.0.widen_mul::<Wider>(rhs.0);
                        let scaled = if SCALE == 0 {
                            n
                        } else if SCALE <= 38 {
                            crate::algos::support::mg_divide::div_wide_pow10_with::<
                                Wider,
                                { <Wider as crate::int::types::traits::BigInt>::U128_LIMBS },
                            >(n, SCALE, mode)
                        } else {
                            crate::algos::support::newton_reciprocal::dispatch_wide_pow10_with::<
                                Wider,
                                { <Wider as crate::int::types::traits::BigInt>::U128_LIMBS },
                            >(n, SCALE, mode)
                        };
                        Self(crate::macros::arithmetic::narrow_or_panic!(
                            scaled, Storage, Wider,
                            "attempt to multiply with overflow"
                        ))
                    }
                }
            }
        }
    };
}

mul_policy_tier!(1, 2);  // D18
mul_policy_tier!(2, 4);  // D38

#[cfg(any(feature = "d57", feature = "wide"))]
mul_policy_tier!(3, 6);  // D57
#[cfg(any(feature = "d76", feature = "wide"))]
mul_policy_tier!(4, 8);  // D76
#[cfg(any(feature = "d115", feature = "wide"))]
mul_policy_tier!(6, 12); // D115
#[cfg(any(feature = "d153", feature = "wide"))]
mul_policy_tier!(8, 16); // D153
#[cfg(any(feature = "d230", feature = "wide"))]
mul_policy_tier!(12, 24); // D230
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
mul_policy_tier!(16, 32); // D307
#[cfg(any(feature = "d462", feature = "x-wide"))]
mul_policy_tier!(24, 48); // D462
#[cfg(any(feature = "d616", feature = "x-wide"))]
mul_policy_tier!(32, 64); // D616
#[cfg(any(feature = "d924", feature = "xx-wide"))]
mul_policy_tier!(48, 96); // D924
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
mul_policy_tier!(64, 128); // D1232
