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
/// is the CamelCase of the kernel fn minus the `mul_` prefix).
///
/// The [`LimbSize`] axis is the algorithm's OWN second-stage choice
/// ([`Algorithm::limb_size`]), selected *after* the algorithm and *by* it —
/// the u64/u128 crossover is algorithm-dependent, so it is co-located with
/// the algorithm, not the verdict.
///
/// [`mul_low_limb`]: crate::int::algos::mul::mul_schoolbook::mul_low_limb
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    LowLimb,
}

impl Algorithm {
    /// The benched limb width THIS algorithm runs in at `N` u64 limbs — the
    /// matcher's **second axis**, selected after (and owned by) the
    /// algorithm because the u64/u128 crossover is algorithm-dependent.
    /// **Per-cell policy DATA**, not a blanket: `U128` only where a
    /// microbench shows it wins AND it is valid (even `N` — enforced by
    /// [`LimbSize::for_packing`], which drives an odd `N` to `U64`). A
    /// `const fn`, so when the algorithm is const-known the whole verdict
    /// folds; under a `ByValue` algorithm choice the arm is picked at run
    /// time and this is read per-arm (still value-independent).
    ///
    /// **`LowLimb`** (benched `benches/micro/mul_low_u128_ab.rs`, `u128` vs
    /// `u64` truncated-low schoolbook). `u128` wins at every even width
    /// measured, decisively at the three live work-integer widths every
    /// [`BigInt::wrapping_mul_low_u128`] call lands on (the wide-tier
    /// exp/powf Taylor multiply runs there):
    ///
    /// | N (work integer) | `u128` vs `u64` |
    /// |------------------|-----------------|
    /// | 128 (D616 work)  | 1.20× faster    |
    /// | 192 (D924 work)  | 1.35× faster    |
    /// | 256 (D1232 work) | 1.29× faster    |
    /// | 48 / 64 (storage, not call sites) | 1.16× faster |
    /// | 32 (storage, not a call site)     | 1.04× (tie)  |
    ///
    /// So `U128` for every even `N` is the measured optimum — no even cell
    /// regresses. This is the tuning seam: if a future bench shows `u128`
    /// losing at some even cell, carve that `N` out to `U64` in THIS arm —
    /// the kernel and dispatch stay untouched.
    ///
    /// [`BigInt::wrapping_mul_low_u128`]: crate::int::types::traits::BigInt::wrapping_mul_low_u128
    #[inline]
    const fn limb_size<const N: usize>(self) -> LimbSize {
        match self {
            Algorithm::LowLimb => LimbSize::for_packing(N),
        }
    }
}

// ── 2. the verdict — the algorithm (limb width is the algorithm's own) ─

/// A settled algorithm. The canonical verdict shape: one algorithm at every
/// `N`, so it is always `ByAlgorithm` (matching the const `add`/`sub`/`cmp`
/// policies). The limb width is NOT carried here — it is the chosen
/// algorithm's own [`Algorithm::limb_size`], derived in [`dispatch`].
#[derive(Clone, Copy)]
enum Select {
    ByAlgorithm(Algorithm),
}

// ── 3. the matcher ────────────────────────────────────────────────────

/// Pick the algorithm for the truncated-low product. One algorithm at every
/// width, so this is width-independent; the chosen algorithm's own
/// [`Algorithm::limb_size`] carries the only `N`-dependent decision.
const fn select() -> Select {
    Select::ByAlgorithm(Algorithm::LowLimb)
}

// ── 4. the dispatcher: resolve the algorithm, then its limb width ─────

/// Truncated-low product `out = (a · b) mod 2^(64·N)` — the single site
/// [`BigInt::wrapping_mul_low_u128`] flows through. Two-stage verdict: the
/// algorithm is resolved first, then asked for its own benched limb width
/// ([`Algorithm::limb_size`]). Both are const here, so the `const { … }`
/// block folds them and this compiles to one direct `mul_low_limb::<N, _>`
/// call per monomorphisation with the unchosen arm dead-arm eliminated.
/// `out` is written in full (the kernel zeroes its own accumulator);
/// bit-identical to [`BigInt::wrapping_mul`] mod `2^(64·N)` at either width.
///
/// [`BigInt::wrapping_mul_low_u128`]: crate::int::types::traits::BigInt::wrapping_mul_low_u128
/// [`BigInt::wrapping_mul`]: crate::int::types::traits::BigInt::wrapping_mul
#[inline]
pub(crate) fn dispatch<const N: usize>(a: &[u64; N], b: &[u64; N], out: &mut [u64; N]) {
    // Stage 1: resolve the algorithm. Stage 2: ask it for its limb width.
    let (algo, limb) = const {
        let algo = match select() {
            Select::ByAlgorithm(a) => a,
        };
        (algo, algo.limb_size::<N>())
    };
    match (algo, limb) {
        (Algorithm::LowLimb, LimbSize::U64) => mul_low_limb::<N, u64>(a, b, out),
        (Algorithm::LowLimb, LimbSize::U128) => mul_low_limb::<N, u128>(a, b, out),
    }
}
