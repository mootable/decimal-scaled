// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Truncated-low squaring policy — the limb-width (`u64` / `u128`) matcher.
//!
//! [`BigInt::wrapping_sqr_low_u128`] computes `(x²) mod 2^(64·N)` — the low `N`
//! limbs of the square, the high half never formed — via the ONE generic
//! kernel [`sqr_low_limb`]`<N, L: Limb>`. As with the multiply sibling
//! [`crate::int::policy::mul_low`], there is a single algorithm (the
//! truncated-low symmetric square); what this policy owns is the **second
//! matcher axis** (`docs/ARCHITECTURE.md` → "Limb width — the matcher's second
//! axis"): the [`LimbSize`] the kernel runs in.
//!
//! `u128` limbs halve the limb count (≈¼ the partial products at the cost of a
//! wider 128×128 inner step) and the square keeps its symmetry halving in
//! either width, so `u128` wins on the **wide even** work widths the wide-tier
//! exp/powf Smith squaring runs on. Which cells win is a per-`N` property
//! settled by microbench (`benches/micro/sqr_low_u128_ab.rs`) and recorded in
//! [`limb_size`] as policy DATA — NOT a blanket rule and NOT a kernel literal.
//! `u128` is gated to **even `N`** by [`LimbSize::for_packing`] (packing pairs
//! two `u64` per `u128`; an odd `N` would drop the top limb), so every entry
//! stays even-`N`-correct.
//!
//! [`BigInt::wrapping_sqr_low_u128`]: crate::int::types::traits::BigInt::wrapping_sqr_low_u128
//! [`sqr_low_limb`]: crate::int::algos::sqr::sqr_low_limb::sqr_low_limb
//! [`LimbSize`]: crate::int::types::compute_int::LimbSize

use crate::int::algos::sqr::sqr_low_limb::sqr_low_limb;
use crate::int::types::compute_int::LimbSize;

// ── 1. the algorithm — singleton: truncated-low symmetric square ───────

/// The truncated-low squaring algorithm. A singleton: there is one algorithm
/// (the truncated-low symmetric square, [`sqr_low_limb`] — the variant is the
/// CamelCase of the kernel fn minus the `sqr_` prefix).
///
/// The [`LimbSize`] axis is the algorithm's OWN second-stage choice
/// ([`Algorithm::limb_size`]), selected *after* the algorithm and *by* it —
/// the u64/u128 crossover is algorithm-dependent, so it is co-located with the
/// algorithm, not the verdict.
///
/// [`sqr_low_limb`]: crate::int::algos::sqr::sqr_low_limb::sqr_low_limb
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    LowLimb,
}

impl Algorithm {
    /// The benched limb width THIS algorithm runs in at `N` u64 limbs — the
    /// matcher's **second axis**, selected after (and owned by) the algorithm
    /// because the u64/u128 crossover is algorithm-dependent. **Per-cell policy
    /// DATA**, not a blanket: `U128` only where a microbench shows it wins AND
    /// it is valid (even `N` — enforced by [`LimbSize::for_packing`], which
    /// drives an odd `N` to `U64`).
    ///
    /// **`LowLimb`** (benched via `sqr_low_u128_ab`, `u128` vs `u64`
    /// truncated-low symmetric square): `u128` wins or ties at every even width
    /// measured. The sweep spans the FULL even set — the narrow storage tiers
    /// {2,4,6,8,12,16,24} (the crossover-bisection region) AND the wide exp/powf
    /// Smith-squaring WORK integers {32,48,64,96,128,192,256} (the decisive,
    /// live band). Representative verdicts, two independent core-22-pinned runs:
    ///
    /// | N (square site)             | `u128` vs `u64`      |
    /// |-----------------------------|----------------------|
    /// | 256 (D1232 work)            | 1.21 .. 1.26× faster |
    /// | 192 (D924 work)             | 1.29 .. 1.39× faster |
    /// | 128 (D616 work)             | 1.23 .. 1.30× faster |
    /// | 96                          | 1.32 .. 1.37× faster |
    /// | 64 (D307 work, D1232 store) | 1.16 .. 1.30× faster |
    /// | 48                          | 1.17 .. 1.22× faster |
    /// | 32                          | 1.06 .. 1.15× faster |
    /// | 24                          | 1.13 .. 1.18× faster |
    /// | 16                          | 1.10 .. 1.26× faster |
    /// | 12                          | ≈tie (1.02× / 1.06× either way) |
    /// | 8                           | ≈tie (u64 by ~1.01..1.05×) |
    /// | 6                           | 1.11× faster         |
    /// | 4                           | tie .. 1.13× faster  |
    /// | 2                           | tie .. 1.17× faster  |
    ///
    /// Every LIVE call site (the wide-transcendental Smith squaring, `N` ≥ 32)
    /// wins `u128` decisively. The only cells that lean `u64` are the narrow
    /// {8,12} near-ties — and those straddle zero across runs (12 flips winner
    /// run-to-run; 8 is u64 by a single-percent margin) and, crucially, are NOT
    /// live `sqr_low` sites (no wide-transcendental instantiates them). Carving
    /// them to `U64` would add a special-case arm for a sub-percent gain that
    /// never executes in production — so `for_packing`-all-even holds (mirrors
    /// the `mul_low` mapper's same narrow-tie-but-not-live finding). This stays
    /// the tuning seam: if a future bench shows `u128` losing at some LIVE even
    /// cell, carve that `N` out to `U64` here; the kernel and dispatch stay
    /// untouched.
    #[inline]
    const fn limb_size<const N: usize>(self) -> LimbSize {
        match self {
            Algorithm::LowLimb => LimbSize::for_packing(N),
        }
    }
}

// ── 2. the verdict — the algorithm (limb width is the algorithm's own) ─

/// A settled algorithm. The canonical verdict shape: one algorithm at every
/// `N`, so it is always `ByAlgorithm`. The limb width is NOT carried here — it
/// is the chosen algorithm's own [`Algorithm::limb_size`], derived in
/// [`dispatch`].
#[derive(Clone, Copy)]
enum Select {
    ByAlgorithm(Algorithm),
}

// ── 3. the matcher ────────────────────────────────────────────────────

/// Pick the algorithm for the truncated-low square. One algorithm at every
/// width, so this is width-independent; the chosen algorithm's own
/// [`Algorithm::limb_size`] carries the only `N`-dependent decision.
const fn select() -> Select {
    Select::ByAlgorithm(Algorithm::LowLimb)
}

// ── 4. the dispatcher: resolve the algorithm, then its limb width ─────

/// Truncated-low square `out = (x²) mod 2^(64·N)` — the single site
/// [`BigInt::wrapping_sqr_low_u128`] flows through. Two-stage verdict: the
/// algorithm is resolved first, then asked for its own benched limb width
/// ([`Algorithm::limb_size`]). Both are const here, so the `const { … }` block
/// folds them and this compiles to one direct `sqr_low_limb::<N, _>` call per
/// monomorphisation with the unchosen arm dead-arm eliminated. `out` is written
/// in full; bit-identical to [`BigInt::wrapping_mul`]`(x, x)` mod `2^(64·N)` at
/// either width.
///
/// [`BigInt::wrapping_sqr_low_u128`]: crate::int::types::traits::BigInt::wrapping_sqr_low_u128
/// [`BigInt::wrapping_mul`]: crate::int::types::traits::BigInt::wrapping_mul
#[inline]
pub(crate) fn dispatch<const N: usize>(x: &[u64; N], out: &mut [u64; N]) {
    let (algo, limb) = const {
        let Select::ByAlgorithm(algo) = select();
        (algo, algo.limb_size::<N>())
    };
    match (algo, limb) {
        (Algorithm::LowLimb, LimbSize::U64) => sqr_low_limb::<N, u64>(x, out),
        (Algorithm::LowLimb, LimbSize::U128) => sqr_low_limb::<N, u128>(x, out),
    }
}
