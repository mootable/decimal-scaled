// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Truncated-low multiply policy — the limb-width (`u64` / `u128`) matcher.
//!
//! [`BigInt::wrapping_mul_low_u128`] computes `(a · b) mod 2^(64·N)` — the
//! low `N` limbs of the product, the high half never formed — via the ONE
//! generic kernel [`mul_low_limb`]`<N, L: Limb>`. There is a single
//! algorithm (truncated-low schoolbook); what this policy owns is the
//! **second matcher axis** (`docs/ARCHITECTURE.md` → "Limb width — the
//! matcher's second axis"): the [`LimbSize`] the kernel runs in.
//!
//! `u128` limbs halve the limb count (≈¼ the partial products at the cost
//! of a wider 128×128 inner step), so they win on the **wide even** work
//! widths but lose to plain `u64` at narrow even widths (the pack/unpack
//! and wider-multiply overhead is not amortised). Which cells win is a
//! per-`N` property settled by microbench (`benches/micro/mul_low_u128_ab.rs`)
//! and recorded in [`limb_size`] as policy DATA — NOT a blanket rule and
//! NOT a kernel literal. `u128` is gated to **even `N`** by
//! [`LimbSize::for_packing`] (packing pairs two `u64` per `u128`; an odd
//! `N` would drop the top limb), so every entry stays even-`N`-correct.
//!
//! [`BigInt::wrapping_mul_low_u128`]: crate::int::types::traits::BigInt::wrapping_mul_low_u128
//! [`mul_low_limb`]: crate::int::algos::mul::mul_schoolbook::mul_low_limb
//! [`LimbSize`]: crate::int::types::compute_int::LimbSize

use crate::int::algos::mul::mul_schoolbook::mul_low_limb;
use crate::int::types::compute_int::LimbSize;

// ── 1. the algorithm — singleton: truncated-low schoolbook ────────────

/// The truncated-low multiply algorithm. A singleton: there is one
/// algorithm (the truncated-low schoolbook, [`mul_low_limb`] — the variant
/// is the CamelCase of the kernel fn minus the `mul_` prefix). The matcher's
/// real work is the [`LimbSize`] axis it pairs with this; the variant keeps
/// the canonical `(Algorithm, LimbSize)` verdict shape uniform with the
/// other policies (and leaves room for a future second low-mul algorithm).
///
/// [`mul_low_limb`]: crate::int::algos::mul::mul_schoolbook::mul_low_limb
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    LowLimb,
}

// ── 2. the verdict — (Algorithm, LimbSize), const in BOTH axes ────────

/// A settled `(algorithm, limb width)`. Both axes are const properties of
/// `N` (limb width is value-independent — never a `ByValue` decision), so
/// the whole verdict const-folds per monomorphisation and [`dispatch`]
/// collapses to a single direct typed call.
#[derive(Clone, Copy)]
enum Select {
    ByAlgorithm(Algorithm, LimbSize),
}

// ── policy DATA: the benched per-N limb-width table ───────────────────

/// The benched limb width for the truncated-low product over `N` u64
/// limbs. **Per-cell policy data**, not a blanket: `u128` is returned only
/// where a microbench shows it wins, and only where it is valid (even `N`
/// — enforced by [`LimbSize::for_packing`], which drives an odd `N` to
/// `U64`).
///
/// **Benched** (`benches/micro/mul_low_u128_ab.rs`, `u128` vs `u64`
/// truncated-low schoolbook). `u128` wins at every even width measured,
/// decisively at the three live work-integer widths every
/// [`BigInt::wrapping_mul_low_u128`] call lands on (the wide-tier exp/powf
/// Taylor multiply runs there):
///
/// | N (work integer) | `u128` vs `u64` |
/// |------------------|-----------------|
/// | 128 (D616 work)  | 1.20× faster    |
/// | 192 (D924 work)  | 1.35× faster    |
/// | 256 (D1232 work) | 1.29× faster    |
/// | 48 / 64 (storage, not call sites) | 1.16× faster |
/// | 32 (storage, not a call site)     | 1.04× (tie)  |
///
/// So `U128` for every even `N` is the measured optimum here — no even
/// cell regresses. This is the tuning seam: if a future bench shows `u128`
/// losing at some even cell, carve that `N` out to `U64` HERE (edit only
/// this fn) — the kernel and dispatch stay untouched.
///
/// [`BigInt::wrapping_mul_low_u128`]: crate::int::types::traits::BigInt::wrapping_mul_low_u128
#[inline]
const fn limb_size<const N: usize>() -> LimbSize {
    LimbSize::for_packing(N)
}

// ── 3. the matcher ────────────────────────────────────────────────────

/// Pick `(algorithm, limb width)` for an `N`-limb truncated-low product.
/// One algorithm; the limb width is the benched [`limb_size`] verdict.
const fn select<const N: usize>() -> Select {
    Select::ByAlgorithm(Algorithm::LowLimb, limb_size::<N>())
}

// ── 4. the dispatcher: fold the verdict, then exhaustive match ────────

/// Truncated-low product `out = (a · b) mod 2^(64·N)` — the single site
/// [`BigInt::wrapping_mul_low_u128`] flows through. The verdict const-folds
/// (`const { select::<N>() }`), so this compiles to one direct
/// `mul_low_limb::<N, _>` call per monomorphisation with the unchosen arm
/// dead-arm eliminated. `out` is written in full (the kernel zeroes its own
/// accumulator); bit-identical to [`BigInt::wrapping_mul`] mod `2^(64·N)`
/// at either limb width.
///
/// [`BigInt::wrapping_mul_low_u128`]: crate::int::types::traits::BigInt::wrapping_mul_low_u128
/// [`BigInt::wrapping_mul`]: crate::int::types::traits::BigInt::wrapping_mul
#[inline]
pub(crate) fn dispatch<const N: usize>(a: &[u64; N], b: &[u64; N], out: &mut [u64; N]) {
    let (algo, limb) = match const { select::<N>() } {
        Select::ByAlgorithm(a, l) => (a, l),
    };
    match (algo, limb) {
        (Algorithm::LowLimb, LimbSize::U64) => mul_low_limb::<N, u64>(a, b, out),
        (Algorithm::LowLimb, LimbSize::U128) => mul_low_limb::<N, u128>(a, b, out),
    }
}
