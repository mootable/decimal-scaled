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
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;

// ── 1. the real multiply algorithms — NAMED, no `Default` ─────────────

/// The multiply algorithms the length matcher chooses between. Variants
/// are the CamelCase of each kernel fn's name minus the `mul_` function
/// prefix (`mul_schoolbook` → `Schoolbook`, `mul_karatsuba` →
/// `Karatsuba`) — strict 1:1 with the kernels in
/// [`crate::int::algos::support::limbs`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`mul_schoolbook`] — base-2⁶⁴ schoolbook outer product.
    Schoolbook,
    /// [`mul_karatsuba`] — non-allocating recursive Karatsuba.
    Karatsuba,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the (runtime) length decides". The mul picker
/// always returns `ByShape`: the choice is fully determined by the
/// operands' lengths, known only at run time. `ByAlgorithm` is part of
/// the canonical shape for uniformity across functions.
#[derive(Clone, Copy)]
enum Select {
    #[allow(dead_code)]
    ByAlgorithm(Algorithm),
    /// Classifier over `(a_len, b_len)` → the chosen algorithm.
    ByShape(fn(usize, usize) -> Algorithm),
}

// ── policy data: the benched crossover threshold ──────────────────────

/// Karatsuba threshold: the (equal) operand limb-count at or above which
/// [`dispatch`] routes to the non-allocating Karatsuba kernel instead of
/// schoolbook. File-private policy data — `dispatch` threads it into the
/// kernel as an argument; nothing outside this policy imports it.
///
/// **48**, the benched crossover (`benches/micro/mul_kernel_ab.rs`,
/// schoolbook vs one-level Karatsuba): schoolbook wins through 32 limbs
/// (2.1× @8 … 1.32× @32), Karatsuba wins at 48 (1.20×) and breaks even at
/// 64 (1.01×). So D924 (48) / D1232 (64) storage products and wider
/// cross-scale multiplies take Karatsuba; D616 (32) and narrower stay
/// schoolbook. (The in-tree Karatsuba is weak — high crossover, small gain;
/// a Toom-3 / tighter split would lower it. Future work.)
///
/// Must be `>= 4`: the recursion's z1 sum product runs on `⌈n/2⌉ + 1`
/// limbs, which only strictly shrinks below `n` once `n >= 4`.
const KARATSUBA_THRESHOLD: usize = 48;

// ── 3. the matcher: keyed on the runtime operand lengths ──────────────

/// Pick the multiply algorithm for the operands' lengths. Total over the
/// lengths; reproduces the exact byte-for-byte routing of the historic
/// inline picker: equal-length operands at or above
/// [`KARATSUBA_THRESHOLD`] take Karatsuba, everything else (unequal, or
/// below the threshold) takes schoolbook.
const fn select() -> Select {
    Select::ByShape(|a_len: usize, b_len: usize| {
        if a_len == b_len && a_len >= KARATSUBA_THRESHOLD {
            Algorithm::Karatsuba
        } else {
            Algorithm::Schoolbook
        }
    })
}

// ── 4. the dispatcher: classify the lengths, then dispatch ────────────

/// Equal-length-aware u64 multiplier dispatcher — the single site every
/// `widen_mul` wide multiply flows through. Picks the non-allocating
/// Karatsuba kernel at or above the threshold; otherwise schoolbook.
///
/// `out` must be sized `>= a.len() + b.len()`. The Karatsuba arm zeroes
/// `out` itself; the schoolbook arm requires the caller to have zeroed it
/// (matching the historic contract).
pub(crate) fn dispatch(a: &[u64], b: &[u64], out: &mut [u64]) {
    let algo = match const { select() } {
        Select::ByAlgorithm(a) => a,
        Select::ByShape(f) => f(a.len(), b.len()),
    };
    match algo {
        Algorithm::Karatsuba => {
            for o in out.iter_mut() {
                *o = 0;
            }
            mul_karatsuba(a, b, out, KARATSUBA_THRESHOLD);
        }
        Algorithm::Schoolbook => mul_schoolbook(a, b, out),
    }
}
