//! Multiply policy — the schoolbook-vs-Karatsuba algorithm matcher.
//!
//! Like division, the integer multiply choice keys on the **runtime
//! length** of the operands, not the const limb count `N`, so it is a
//! [`Select::ByValue`] case in the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"): the const layer
//! settles on "the value decides", the value-matcher classifies the
//! operand lengths and returns an [`Algorithm`] tag, and the dispatcher
//! does an **exhaustive** `match algo` to a pure kernel in
//! [`crate::int::algos::support::limbs`].
//!
//! The kernels ([`mul_schoolbook`] / [`mul_karatsuba`]) stay pure; this
//! file owns the *choice* — the benched crossover threshold
//! ([`KARATSUBA_THRESHOLD`]) is policy DATA here, not a magic number in a
//! kernel.

use crate::int::algos::mul::mul_karatsuba::mul_karatsuba;
use crate::int::algos::mul::mul_schoolbook::mul_full_limb;
use crate::int::types::Int;
use crate::int::types::compute_int::{ComputeInt, LimbSize};

// ── 1. the real multiply algorithms — NAMED, no `Default` ─────────────

/// The multiply algorithms the length matcher chooses between. Variants
/// are the CamelCase of each kernel fn's name minus the `mul_` function
/// prefix (`mul_full_limb` → `Schoolbook`, `mul_karatsuba` → `Karatsuba`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`mul_full_limb`] — fixed-width schoolbook outer product, run in the
    /// [`Algorithm::limb_size`]-chosen limb width (`u64` / packed `u128`).
    Schoolbook,
    /// [`mul_karatsuba`] — non-allocating recursive Karatsuba.
    Karatsuba,
}

impl Algorithm {
    /// The benched limb width [`Algorithm::Schoolbook`] runs in at `N` u64
    /// limbs — the matcher's **second axis** (`docs/ARCHITECTURE.md` → "Limb
    /// width — the matcher's second axis"), selected after (and owned by) the
    /// algorithm because the u64/u128 crossover is algorithm-dependent. `u128`
    /// packs the operands into `N/2` limbs (≈¼ the partial products at a wider
    /// 128×128→256 inner step), valid only for EVEN `N` — [`LimbSize::for_packing`]
    /// drives odd `N` to `U64`. Per the sibling [`crate::int::policy::mul_low`]
    /// pilot (`mul_low_u128_ab`) `u128` wins at every even width measured; this
    /// is the per-cell tuning seam — carve any even `N` that the bbc shows
    /// regressing back to `U64` here, kernel and dispatch untouched. Karatsuba
    /// has no limb-width axis (it recurses to slice schoolbook).
    #[inline]
    const fn limb_size<const N: usize>(self) -> LimbSize {
        match self {
            Algorithm::Schoolbook => LimbSize::for_packing(N),
            Algorithm::Karatsuba => LimbSize::U64,
        }
    }
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the (runtime) length decides". `ByShape`
/// classifies the operand lengths (known at run time) → the algorithm;
/// `ByAlgorithm` is part of the canonical shape for uniformity.
#[derive(Clone, Copy)]
enum Select {
    #[allow(dead_code)]
    ByAlgorithm(Algorithm),
    /// Classifier over `(a_len, b_len)` → the chosen algorithm.
    ByShape(fn(usize, usize) -> Algorithm),
}

// ── policy data: the benched crossover threshold ──────────────────────

/// Karatsuba threshold: the (equal) operand limb-count at or above which
/// [`dispatch`] routes to the non-allocating Karatsuba kernel instead of the
/// fixed-width schoolbook. File-private policy data.
///
/// **64**, the benched crossover (`mul_kernel_ab`, schoolbook vs one-level
/// Karatsuba): schoolbook wins through 48 limbs and Karatsuba first wins at
/// 64. So only the N=64 product (D1232 storage) and wider take Karatsuba;
/// D924 (48) and narrower stay schoolbook. (The in-tree Karatsuba is weak — a
/// Toom-3 / tighter split would lower it; kept for that re-tune.)
///
/// Must be `>= 4`: the recursion's z1 sum product runs on `⌈n/2⌉ + 1`
/// limbs, which only strictly shrinks below `n` once `n >= 4`.
const KARATSUBA_THRESHOLD: usize = 64;

// ── 3. the matcher: keyed on the runtime operand lengths ──────────────

/// Pick the multiply algorithm for the operands' lengths. Equal-length
/// operands at or above [`KARATSUBA_THRESHOLD`] take Karatsuba; everything
/// else (unequal, or below the threshold) takes the fixed-width schoolbook.
const fn select() -> Select {
    Select::ByShape(|a_len: usize, b_len: usize| {
        if a_len == b_len && a_len >= KARATSUBA_THRESHOLD {
            Algorithm::Karatsuba
        } else {
            Algorithm::Schoolbook
        }
    })
}

// ── 4. the dispatcher: classify lengths, resolve limb width, dispatch ─

/// Equal-length `Int<N>×Int<N>` full-product dispatcher — the single site
/// every `widen_mul` wide multiply flows through. Resolves the algorithm
/// (Karatsuba at/above the threshold, else schoolbook), then for schoolbook
/// asks the chosen algorithm for its benched limb width
/// ([`Algorithm::limb_size`]) and runs the ONE generic [`mul_full_limb`]
/// kernel at `u64` / `u128`. Both stages are const here, so the `const { … }`
/// block folds them to one direct call per monomorphisation with the unchosen
/// arms dead-arm eliminated.
///
/// `out` must be sized `>= 2·N`. Every arm writes `out` in full (the kernels
/// zero their own accumulators); the result is bit-identical at either limb
/// width and against the historic slice schoolbook.
#[inline]
pub(crate) fn dispatch<const N: usize>(a: &[u64; N], b: &[u64; N], out: &mut [u64])
where
    Int<N>: ComputeInt,
{
    // Lengths are both N (equal), so the run-time classifier folds to a
    // const verdict per monomorphisation; resolve its limb width too.
    let (algo, limb) = {
        let algo = match const { select() } {
            Select::ByAlgorithm(a) => a,
            Select::ByShape(f) => f(N, N),
        };
        (algo, algo.limb_size::<N>())
    };
    match (algo, limb) {
        (Algorithm::Schoolbook, LimbSize::U64) => mul_full_limb::<N, u64>(a, b, out),
        (Algorithm::Schoolbook, LimbSize::U128) => mul_full_limb::<N, u128>(a, b, out),
        (Algorithm::Karatsuba, _) => {
            for o in out.iter_mut() {
                *o = 0;
            }
            mul_karatsuba(a, b, out, KARATSUBA_THRESHOLD);
        }
    }
}
