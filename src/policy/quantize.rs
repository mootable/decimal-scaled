// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Quantize policy — the per-`(N, SCALE, TARGET_SCALE)` algorithm matcher
//! for changing a decimal's quantum at a fixed storage width.
//!
//! The `quantize` / `quantize_with` / `with_scale` methods emitted on every
//! width delegate to [`QuantizePolicy::quantize_impl`], which forwards to
//! the one shared [`dispatch`] function. `dispatch` follows the canonical
//! policy shape (see `docs/ARCHITECTURE.md` → "Policy file structure"),
//! mirroring [`crate::policy::neg`]:
//!
//! 1. an [`Algorithm`] enum — the real quantize algorithm, no `Default`
//!    variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (quantize has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE, TARGET_SCALE)`, total
//!    over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE, TARGET_SCALE>() }`
//!    block, then an **exhaustive** `match algo` — no `_`, no panic.
//!
//! # The key is three consts, and all three are the operation's own
//!
//! `quantize` moves ONE axis of the decimal key: it holds the storage width
//! `N` fixed and maps `SCALE → TARGET_SCALE`. So its key is the unary
//! decimal key plus the target scale — the second scale the operation is
//! *defined by*, not a width derived from another width. Nothing here is a
//! work width.
//!
//! # Why one algorithm, and why a policy anyway
//!
//! There is one way to change a decimal's quantum at a fixed width: scale
//! the stored integer by `10^|TARGET_SCALE − SCALE|`. What is genuinely
//! open is the DIVIDE inside the scale-down direction — and that choice is
//! already owned by the `÷10^scale` matcher
//! ([`crate::algos::support::rescale`]), which the kernel routes through.
//! `quantize` therefore has one algorithm today, and per
//! `docs/ARCHITECTURE.md` ("A single-algorithm op is therefore still worth
//! a policy") it still gets this seam: a pure `ByAlgorithm` matcher that
//! folds to one direct kernel call, and a ready place to add or swap a
//! kernel per `(N, SCALE, TARGET_SCALE)` cell without restructuring the
//! call sites.

use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real quantize algorithm — NAMED, no `Default` ──────────────

/// The quantize algorithms this policy chooses between. The single variant
/// is the CamelCase of the kernel fn's name minus the `quantize_` function
/// prefix (`quantize_pow10` → `Pow10`) — strict 1:1 with the kernel fn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`quantize_pow10`](crate::algos::quantize::quantize_pow10::quantize_pow10)
    /// — scales the stored integer by `10^|TARGET_SCALE − SCALE|`: an exact
    /// multiply by the baked `10^shift` going up, a rounded `÷10^shift`
    /// through the rescale matcher going down.
    Pow10,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the value decides". The quantize picker always
/// returns `ByAlgorithm`: the scaling direction and shift are fixed by the
/// two const scales, so no operand value can change the choice. `ByValue`
/// is part of the canonical shape for uniformity; `select` never returns
/// it.
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on the three consts, total over the key ──

/// Pick the quantize algorithm for storage limb count `N` moving from
/// decimal `SCALE` to `TARGET_SCALE`. Total over the key; `Pow10` wins at
/// every cell.
const fn select<const N: usize, const SCALE: u32, const TARGET_SCALE: u32>() -> Select<N> {
    let _ = (SCALE, TARGET_SCALE);
    Select::ByAlgorithm(Algorithm::Pow10)
}

// ── 4. the dispatcher: fold the verdict, then dispatch ────────────────

/// Decimal quantize dispatcher for storage `Int<N>`, from `SCALE` to
/// `TARGET_SCALE`.
///
/// Resolves the compile-time algorithm verdict via
/// `const { select::<N, SCALE, TARGET_SCALE>() }` (folds per
/// monomorphisation; dead arms are eliminated in release) then dispatches
/// exhaustively over [`Algorithm`].
///
/// Returns `None` **only** on a scale-up that overflows `Int<N>`. The
/// panic for that case belongs to the caller, whose message names the
/// concrete tier.
///
/// Not `const fn`: the wide integer's `Div` / `Rem` operators are not
/// `const`.
#[inline]
pub(crate) fn dispatch<const N: usize, const SCALE: u32, const TARGET_SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>>
where
    Limbs<N>: ComputeLimbs,
{
    let algo = match const { select::<N, SCALE, TARGET_SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(_) => Algorithm::Pow10,
    };
    match algo {
        Algorithm::Pow10 => {
            crate::algos::quantize::quantize_pow10::quantize_pow10::<N, SCALE, TARGET_SCALE>(
                raw, mode,
            )
        }
    }
}

// ── per-type `QuantizePolicy` trait ───────────────────────────────────

/// Per-type policy: which kernel a `D<Int<N>, SCALE>` uses to change its
/// quantum.
///
/// The trait exists so the width-generic method surface can reach
/// [`dispatch`] without naming `N` at the call site: the emitted methods
/// hold a `$Storage` type, the dispatcher wants the limb count, and this
/// blanket impl carries `N` across. The associated [`Self::Storage`] is
/// the raw integer the caller re-wraps at the target scale.
pub(crate) trait QuantizePolicy: Sized {
    /// The raw storage integer behind this decimal.
    type Storage;

    /// Quantize to `TARGET_SCALE` under `mode`, returning the raw storage
    /// integer. `None` is a scale-up overflow; the caller panics with its
    /// own tier-named message.
    fn quantize_impl<const TARGET_SCALE: u32>(self, mode: RoundingMode)
        -> Option<Self::Storage>;
}

impl<const N: usize, const SCALE: u32> QuantizePolicy for crate::D<Int<N>, SCALE>
where
    Limbs<N>: ComputeLimbs,
{
    type Storage = Int<N>;

    #[inline]
    fn quantize_impl<const TARGET_SCALE: u32>(
        self,
        mode: RoundingMode,
    ) -> Option<Int<N>> {
        dispatch::<N, SCALE, TARGET_SCALE>(self.0, mode)
    }
}
