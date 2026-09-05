// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `to_int` policy — the per-`(N, SCALE)` algorithm matcher for reading a
//! decimal out as a rounded `i64`.
//!
//! The `to_int` / `to_int_with` methods emitted on every width delegate to
//! [`ToIntPolicy::to_int_impl`], which forwards to the one shared
//! [`dispatch`]. `dispatch` follows the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"), mirroring
//! [`crate::policy::quantize`]:
//!
//! 1. an [`Algorithm`] enum — the real algorithms, no `Default` variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides";
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block, then
//!    an **exhaustive** `match algo` — no `_`, no panic.
//!
//! # The key is the unary decimal key, and nothing else
//!
//! `to_int` maps a `D<Int<N>, SCALE>` to an `i64`: its key is `(N, SCALE)`
//! — the decimal-unary consts the caller already holds in its type. No
//! work width, no derived const.
//!
//! # Why there are two algorithms
//!
//! Both compute the same rounded whole number; they differ in how the
//! truncated quotient and its remainder are produced, and their costs
//! differ in SHAPE rather than by a constant factor:
//!
//! * [`Algorithm::QuotientRemainder`] pays two full-width typed divides
//!   through the width-erased blanket `Int<N>` `/` / `%` door. At
//!   `Int<1>` that is a single hardware divide; it grows with the
//!   storage width whatever the operand holds.
//! * [`Algorithm::Rescale`] pays one pass through the `÷10^scale`
//!   matcher, sized off the operand's significant limb length, plus a
//!   u128 magnitude set-up that has nothing to amortise against at the
//!   narrow tiers.
//!
//! That crossover is an empirical question, which is exactly what this
//! seam exists to answer. `select` names `QuotientRemainder` at every
//! cell today — the behaviour that shipped in 0.5.x — and moves a cell
//! only on the benchmark (`benches/micro/to_int_surface.rs`), never on
//! the expectation above.

use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real algorithms — NAMED, no `Default` ──────────────────────

/// The `to_int` algorithms this policy chooses between. Each variant is
/// the CamelCase of its kernel fn's name minus the `to_int_` function
/// prefix — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`to_int_quotient_remainder`](crate::algos::to_int::to_int_quotient_remainder::to_int_quotient_remainder)
    /// — two typed full-width divides on `Int<N>`, then the shared
    /// rounding decision.
    QuotientRemainder,
    /// [`to_int_rescale`](crate::algos::to_int::to_int_rescale::to_int_rescale)
    /// — one pass through the `÷10^scale` matcher, which produces the
    /// quotient and applies `mode` together.
    Rescale,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". Both `to_int` kernels are
/// correct for every operand, so the choice is a pure `(N, SCALE)` cost
/// question and `select` always returns `ByAlgorithm`. `ByValue` is part
/// of the canonical shape for uniformity.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the `to_int` algorithm for storage limb count `N` at decimal
/// `SCALE`. Total over the key.
///
/// Every cell names `QuotientRemainder`, the algorithm that shipped in
/// 0.5.x, so lifting the computation out of the macro moved no cell. The
/// [`Algorithm::Rescale`] arm is wired and reachable but not yet routed:
/// moving a cell to it is a decision for `to_int_surface`, and a
/// contiguous width/scale region rather than one benched cell.
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = (N, SCALE);
    Select::ByAlgorithm(Algorithm::QuotientRemainder)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal `to_int` dispatcher for storage `Int<N>` at `SCALE`.
///
/// Resolves the compile-time verdict via `const { select::<N, SCALE>() }`
/// (folds per monomorphisation; dead arms are eliminated in release) then
/// dispatches exhaustively over [`Algorithm`].
///
/// `divisor` is `10^SCALE`, supplied by the caller as its own
/// `multiplier()` const-fn — see the kernel's note on why it is an
/// operand rather than derived here.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    divisor: Int<N>,
    mode: RoundingMode,
) -> i64
where
    Limbs<N>: ComputeLimbs,
{
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(f) => f(&raw),
    };
    match algo {
        Algorithm::QuotientRemainder => {
            crate::algos::to_int::to_int_quotient_remainder::to_int_quotient_remainder::<N>(
                raw, divisor, mode,
            )
        }
        Algorithm::Rescale => {
            crate::algos::to_int::to_int_rescale::to_int_rescale::<N, SCALE>(raw, mode)
        }
    }
}

// ── per-type `ToIntPolicy` trait ──────────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses to read itself
/// out as a rounded `i64`.
///
/// The trait exists so the width-generic method surface can reach
/// [`dispatch`] without naming `N` at the call site: the emitted methods
/// hold a `$Storage` type, the dispatcher wants the limb count, and this
/// blanket impl carries `N` across — the same bridge
/// [`QuantizePolicy`](crate::policy::quantize::QuantizePolicy) provides.
pub(crate) trait ToIntPolicy: Sized {
    /// The raw storage integer behind this decimal.
    type Storage;

    /// Round to a whole number under `mode` and narrow to `i64`,
    /// saturating out of range. `divisor` is this type's `10^SCALE`.
    fn to_int_impl(self, divisor: Self::Storage, mode: RoundingMode) -> i64;
}

impl<const N: usize, const SCALE: u32> ToIntPolicy for crate::D<Int<N>, SCALE>
where
    Limbs<N>: ComputeLimbs,
{
    type Storage = Int<N>;

    #[inline]
    fn to_int_impl(self, divisor: Int<N>, mode: RoundingMode) -> i64 {
        dispatch::<N, SCALE>(self.0, divisor, mode)
    }
}
