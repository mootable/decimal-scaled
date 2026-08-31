// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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
//! **One classifier, two doors** (`docs/ARCHITECTURE.md` → "Const entry +
//! slice entry"). The length classifier ([`select`]'s `ByShape`) backs TWO
//! entry points over the SAME decision:
//! - [`dispatch`] — the const-`N` door for `Int<N>×Int<N>` callers (the
//!   wide-transcendental work-muls): the lengths are both `N`, so the
//!   classifier folds to a const verdict per monomorphisation and additionally
//!   takes the [`LimbSize`] (`u64`/`u128`) axis. The hot path.
//! - [`dispatch_slice`] — the runtime-length door for genuine slice callers
//!   (the decimal slice roots and the rescale product path) that hold bare
//!   `&[u64]` of runtime length and no `N`. It runs the IDENTICAL classifier
//!   on `a.len()`/`b.len()` and routes to the `u64` slice kernels — no const
//!   `N` means no `u128` packing, but the product is bit-identical.
//!
//! The kernels ([`mul_schoolbook`] / [`mul_karatsuba`]) stay pure; this
//! file owns the *choice* — the benched crossover ([`KARATSUBA_ENGAGE`]) and
//! recursion depth ([`KARATSUBA_RECURSE`]) are policy DATA here, not magic
//! numbers in a kernel.

use crate::int::algos::mul::mul_karatsuba::{mul_karatsuba, mul_karatsuba_limb};
use crate::int::algos::mul::mul_schoolbook::{mul_full_limb, mul_schoolbook};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs, LimbSize};

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
    /// drives odd `N` to `U64`. The full-product A/B (`mul_full_ab`,
    /// `mul_full_limb::<N,u64>` vs `::<N,u128>`, N = 2..64) confirms `u128`
    /// wins or ties at EVERY even width — a statistical tie below N = 16
    /// (within ±5%, both directions across runs) and a decisive 1.13–1.34x win
    /// at N >= 16 (matching the sibling [`crate::int::policy::mul_low`] pilot).
    /// So every even `N` packs to `u128`; this is the per-cell tuning seam —
    /// carve any even `N` that benchmarks show regressing back to `U64` here,
    /// kernel and dispatch untouched.
    ///
    /// Karatsuba shares the SAME limb-width axis: the policy-map (`mul_toom3_ab`
    /// + the `mul_kara_thresh_ab` sweep) showed the Limb-generic Karatsuba
    /// (`mul_karatsuba_limb`) in `u128` limbs beats schoolbook-u128 by ~1.34x
    /// (N=128) .. 1.39x (N=256) at recursion threshold 48, and the matcher only
    /// engages Karatsuba at EVEN `N >= KARATSUBA_ENGAGE`, so it always packs to
    /// `u128` (`for_packing` returns `U128` for even `N`).
    #[inline]
    const fn limb_size<const N: usize>(self) -> LimbSize {
        match self {
            Algorithm::Schoolbook => LimbSize::for_packing(N),
            Algorithm::Karatsuba => LimbSize::for_packing(N),
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

/// Karatsuba **engage** point: the (equal) operand limb-count at or above which
/// [`dispatch`] routes EVEN-width products to the Limb-generic Karatsuba kernel
/// (`mul_karatsuba_limb::<N, u128>`) instead of the u128 fixed-width schoolbook.
/// File-private policy data.
///
/// **`128`** — the policy-map (`mul_toom3_ab`, every fixed-array candidate ×
/// u64/u128 raced 24..256, pinned) plus the `mul_kara_thresh_ab` recursion-depth
/// sweep localize the crossover to `(96, 128]`: schoolbook-u128 wins `N <= 96`,
/// the u128-packed recursive Karatsuba wins `N >= 128` by **1.34x at N=128 and
/// 1.39x at N=256** (vs the old one-level Karatsuba which lost everywhere — the
/// reason this was `usize::MAX`). Only EVEN `N` reaches Karatsuba (so it always
/// packs to `u128`); odd / `< 128` widths stay schoolbook. The exact crossover
/// in `(96, 128]` is academic — no shipped storage tier (<=64) or work width
/// (96/128/192/256) lies strictly between 96 and 128.
const KARATSUBA_ENGAGE: usize = 128;

/// Karatsuba **recursion** base: the limb-count below which the kernel stops
/// splitting and runs schoolbook. **`48`** is the swept optimum (`kara_t48`
/// beat `t16/t24/t32` at N=128 and decisively at N=256). Distinct from
/// [`KARATSUBA_ENGAGE`] (when to USE Karatsuba) — this is how DEEP it recurses.
/// The kernel requires `>= 4` (the z1 sum product on `⌈n/2⌉ + 1` limbs only
/// strictly shrinks below `n` once `n >= 4`); 48 satisfies that.
const KARATSUBA_RECURSE: usize = 48;

// ── 3. the matcher: keyed on the runtime operand lengths ──────────────

/// Pick the multiply algorithm for the operands' lengths. Equal-length EVEN
/// operands at or above [`KARATSUBA_ENGAGE`] take Karatsuba; everything else
/// (unequal, odd, or below the engage point) takes the fixed-width schoolbook.
const fn select() -> Select {
    Select::ByShape(|a_len: usize, b_len: usize| {
        // Equal-length products at/above the engage point take the u128-packed
        // recursive Karatsuba — but ONLY at EVEN widths, since that path packs
        // two u64 limbs per u128 (odd widths can't pack, and the swept win is a
        // u128 result). Everything else (unequal, odd, or below ENGAGE) takes
        // the fixed-width schoolbook. For Int<N>×Int<N> both lengths are N, so
        // this folds to a const verdict on N per monomorphisation.
        let take_karatsuba = a_len == b_len && a_len >= KARATSUBA_ENGAGE && a_len.is_multiple_of(2);
        if take_karatsuba {
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
    Limbs<N>: ComputeLimbs,
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
        // The engaged Karatsuba arm: u128-packed Limb-generic kernel (the
        // swept winner at even N >= ENGAGE), recursing to the schoolbook base
        // at KARATSUBA_RECURSE. Writes `out` in full (the unpack overwrites it).
        (Algorithm::Karatsuba, LimbSize::U128) => {
            mul_karatsuba_limb::<N, u128>(a, b, out, KARATSUBA_RECURSE)
        }
        // Unreached (the matcher only engages Karatsuba at even N, which always
        // packs to u128) but kept exhaustive: the u64 slice Karatsuba, zeroing
        // its accumulator first.
        (Algorithm::Karatsuba, LimbSize::U64) => {
            for o in out.iter_mut() {
                *o = 0;
            }
            mul_karatsuba(a, b, out, KARATSUBA_RECURSE);
        }
    }
}

/// Runtime-length **slice door** over the SAME [`select`] length classifier as
/// the const [`dispatch`] (`docs/ARCHITECTURE.md` → "Const entry + slice
/// entry — one length/shape classifier, two doors"). For the genuine slice
/// callers — the decimal slice roots (`sqrt_newton`, `cbrt_newton`) and the
/// rescale product path (`div_widen_scale`) — whose operands are bare `&[u64]`
/// of runtime length with no `N` in their types, so they cannot take the const
/// door. They route here instead of reaching past the matcher to a hardcoded
/// kernel (the Class-G bypass this door removes).
///
/// The classifier is run on the runtime `a.len()`/`b.len()`: equal-length EVEN
/// operands at or above [`KARATSUBA_ENGAGE`] take the recursive Karatsuba
/// ([`mul_karatsuba`], recursing to schoolbook at [`KARATSUBA_RECURSE`]);
/// everything else (unequal, odd, or below the engage point) takes the slice
/// schoolbook ([`mul_schoolbook`]). The product is **bit-identical** to a plain
/// `mul_schoolbook` call on the same operands at every shape.
///
/// # Limb width
///
/// The slice door runs the **`u64`** kernels only. The [`LimbSize`] (`u128`-
/// packing) axis the const door takes needs a compile-time `N` to size the
/// packed `[L; N]` / `ComputeLimbs` buffers, which a runtime-length slice does
/// not have — so it stays `u64`. That axis is a const-door-only optimisation;
/// the result is the same integer either way.
///
/// # Caller contract
///
/// `out` must be **zeroed by the caller** and sized `>= a.len() + b.len()`
/// (exactly the existing [`mul_schoolbook`] contract — every converted caller
/// already satisfies it). No scratch parameter: the Karatsuba slice entry
/// self-sizes its own (sanctioned width-erased build-max) scratch internally.
/// The Karatsuba arm additionally needs `a.len() == b.len()`, which the
/// classifier guarantees before it is reached (it only engages Karatsuba for
/// equal even lengths `>= KARATSUBA_ENGAGE`).
#[inline]
pub(crate) fn dispatch_slice(a: &[u64], b: &[u64], out: &mut [u64]) {
    // The SAME classifier as the const door, evaluated on the operands' runtime
    // lengths instead of folded on `N`.
    let algo = match const { select() } {
        Select::ByAlgorithm(alg) => alg,
        Select::ByShape(classify) => classify(a.len(), b.len()),
    };
    match algo {
        Algorithm::Schoolbook => mul_schoolbook(a, b, out),
        Algorithm::Karatsuba => mul_karatsuba(a, b, out, KARATSUBA_RECURSE),
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch_slice, mul_schoolbook};

    /// [`dispatch_slice`] is **bit-identical** to a plain [`mul_schoolbook`]
    /// over the same operands at every shape it classifies — the routing-blind
    /// correctness guarantee both doors share (every multiply algorithm is the
    /// same product). The equal-length EVEN widths at/above the current engage
    /// point (128) drive the door's Karatsuba arm, so a mis-wired Karatsuba arm
    /// (wrong threshold, un-zeroed `out`, wrong kernel) would surface as a
    /// mismatch HERE; the unequal / odd / sub-engage shapes drive the schoolbook
    /// arm. This asserts only that the *product* is correct, never *which* arm
    /// ran — routing stays the matcher's tunable, which policy tests don't pin.
    #[test]
    fn dispatch_slice_bit_identical_to_schoolbook() {
        // SplitMix64 — Vigna 2014, public-domain reference algorithm.
        let mut state: u64 = 0xD15C_0DED_51CE_D006;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };

        // Operand-length pairs spanning every arm of the classifier. The engage
        // point is 128 (equal + even + `>= 128` ⇒ Karatsuba); these reach it.
        const SHAPES: &[(usize, usize)] = &[
            // equal even < engage, equal odd, small → schoolbook
            (1, 1),
            (2, 2),
            (4, 4),
            (16, 16),
            (64, 64),
            (17, 17),
            (33, 33),
            (129, 129),
            // equal EVEN >= engage → Karatsuba arm
            (128, 128),
            (130, 130),
            (192, 192),
            (256, 256),
            // unequal (incl. the single-limb ×10 / scaled-numerator shapes) → schoolbook
            (1, 288),
            (288, 1),
            (64, 1),
            (100, 128),
            (127, 128),
        ];

        let edge_fill = |buf: &mut [u64], kind: usize, next: &mut dyn FnMut() -> u64| match kind {
            0 => buf.iter_mut().for_each(|x| *x = 0),
            1 => buf.iter_mut().for_each(|x| *x = u64::MAX),
            2 => {
                buf.iter_mut().for_each(|x| *x = 0);
                if let Some(last) = buf.last_mut() {
                    *last = u64::MAX;
                }
            }
            3 => {
                buf.iter_mut().for_each(|x| *x = 0);
                buf[0] = u64::MAX;
            }
            _ => buf.iter_mut().for_each(|x| *x = next()),
        };

        for &(la, lb) in SHAPES {
            let mut pairs: Vec<(Vec<u64>, Vec<u64>)> = Vec::new();
            for ka in 0..5 {
                for kb in 0..5 {
                    let mut a = vec![0u64; la];
                    let mut b = vec![0u64; lb];
                    edge_fill(&mut a, ka, &mut next);
                    edge_fill(&mut b, kb, &mut next);
                    pairs.push((a, b));
                }
            }
            let randoms = if la.max(lb) <= 16 { 40 } else { 8 };
            for _ in 0..randoms {
                let mut a = vec![0u64; la];
                let mut b = vec![0u64; lb];
                for x in a.iter_mut() {
                    *x = next();
                }
                for x in b.iter_mut() {
                    *x = next();
                }
                pairs.push((a, b));
            }

            for (a, b) in &pairs {
                // `out` zeroed + sized `a.len() + b.len()` — the door's contract.
                let mut oracle = vec![0u64; la + lb];
                mul_schoolbook(a, b, &mut oracle);
                let mut got = vec![0u64; la + lb];
                dispatch_slice(a, b, &mut got);
                assert_eq!(
                    got, oracle,
                    "dispatch_slice != mul_schoolbook at shape ({la}, {lb})\na={a:?}\nb={b:?}"
                );
            }
        }
    }
}
