//! Floating-point power policy — the per-`(N, SCALE)` algorithm matcher.
//!
//! `D<Int<N>, SCALE>::powf_strict_with(exp, mode)` delegates to
//! [`PowPolicy::powf_impl`], which resolves the canonical
//! [`Algorithm`]/[`select`] verdict (the `sqrt` exemplar shape):
//!
//! 1. an [`Algorithm`] enum — the real power algorithms, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block,
//!    then an **exhaustive** `match algo`.
//!
//! # The one power algorithm — `ExpWithLn`
//!
//! `powf` is the hybrid `b^y = exp(y · ln b)`: a composition of the
//! `exp` and `ln` algorithms (the established identity, not a separate
//! transcendental method). Its fn is `powf_exp_with_ln` and the enum
//! variant is `ExpWithLn`. Across all `(N, SCALE)` it is the sole
//! algorithm; the narrow tiers realise it on the 256-bit `Fixed`
//! intermediate (`pow::fixed_d38`, with D18 widening in via the
//! `widen_to_work` strategy), the wide tiers via the inherent
//! `powf_strict_with` shell that composes the wide-tier `exp`/`ln`
//! cores (not yet policy-routed — see `phase4/migration_explog.md`,
//! "the bulk of pow's Phase-4 lift").
//!
//! # Deferred: the `IntSquareMultiply` value matcher
//!
//! When the exponent is a small integer (`|n| ≤ 64`) the kernels run
//! binary square-and-multiply instead of `exp∘ln`. Today that integer
//! short-circuit is a value-dependent *step inside* each kernel
//! (`pow::fixed_d38::exp_as_small_int` etc.). Promoting it to a distinct
//! `Algorithm::IntSquareMultiply` (fn `pow_int_square_multiply`) selected
//! by a `Select::ByValue` arm (`powf_exp_small_int`, keyed on the
//! exponent operand) is the family's one recommended value matcher
//! (`phase4/migration_explog.md` §powf). It is deferred here with the
//! integer fast path left in the kernel body: `ExpWithLn` alone is a
//! complete, total `select`, and lifting the squaring path out of the
//! kernels is a behaviour-affecting change with no perf change (same code
//! path) — to be done with the wide-tier `powf` policy lift. The
//! `ByValue` verdict shape is wired below so the arm drops in cleanly.

use crate::algos::pow;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

pub(crate) trait PowPolicy: Sized {
    /// `self^exp` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self;

    /// `self^exp` with caller-chosen working digits.
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ── 1. the real power algorithms — NAMED, no `Default` ─────────────────

/// The power algorithms this policy chooses between. The variant is the
/// CamelCase of the kernel's name minus the `powf_` prefix
/// (`powf_exp_with_ln` → `ExpWithLn`) — strict 1:1.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// `powf_exp_with_ln` — the hybrid `b^y = exp(y · ln b)`. The single
    /// algorithm today; realised by `pow::fixed_d38` (narrow) and the
    /// inherent wide-tier shell.
    ExpWithLn,
    // Deferred: `IntSquareMultiply` (fn `pow_int_square_multiply`),
    // selected by the `powf_exp_small_int` `ByValue` matcher. See module
    // docs — the integer fast path currently lives inside the kernels.
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The `ByValue` shape is
/// wired so the deferred small-integer-exponent matcher
/// (`powf_exp_small_int` → `IntSquareMultiply`) drops in cleanly; `powf`
/// returns only `ByAlgorithm(ExpWithLn)` today.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the power algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; every cell is `ExpWithLn` (narrow tiers
/// reach it via the `widen_to_work` strategy).
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE);
    Select::ByAlgorithm(Algorithm::ExpWithLn)
}

/// Resolve the `(N, SCALE)` verdict to a concrete `Algorithm`.
#[inline]
fn resolve<const N: usize, const SCALE: u32>(base: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(base),
    }
}

// ── Narrow tier (D18) — widen-to-D38 work width, then ExpWithLn ───────
//
// D18 widens base and exponent into the D38 `Fixed` work width (the
// `widen_to_work` strategy) and runs `powf_exp_with_ln` there.
impl<const SCALE: u32> PowPolicy for D18<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_strict_d18(self, exp, mode)
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_with_d18(self, exp, working_digits, mode)
    }
}

// ── D38 — ExpWithLn on the in-tree `Fixed`-256 kernel ──────────────────
//
// `powf` composes `exp(y·ln x)` on the 256-bit `Fixed` guard
// intermediate. The borrow_d57 round trip was retired once the 0.4.2
// MG-routed `Fixed` primitives made the bespoke path win across the whole
// SCALE range (the empirical SCALE-23 crossover that motivated the split
// is gone). The integer-exponent fast path lives inside
// `pow::fixed_d38::powf_*` (the deferred `IntSquareMultiply` step).
impl<const SCALE: u32> PowPolicy for D38<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        Self(match resolve::<2, SCALE>(&self.0) {
            Algorithm::ExpWithLn => pow::fixed_d38::powf_strict::<SCALE>(self.0, exp.0, mode),
        })
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(match resolve::<2, SCALE>(&self.0) {
            Algorithm::ExpWithLn => {
                pow::fixed_d38::powf_with::<SCALE>(self.0, exp.0, working_digits, mode)
            }
        })
    }
}

// Wide-tier `powf` is not policy-routed today — it lives in the inherent
// `powf_strict_with` shell emitted by `decl_wide_transcendental!`, which
// composes the wide-tier `exp`/`ln` cores. Migrating it into a wide-tier
// `PowPolicy` impl (mirroring the `exp`/`ln` wide tiers above) is the
// bulk of pow's Phase-4 lift, recorded in `phase4/migration_explog.md`.
