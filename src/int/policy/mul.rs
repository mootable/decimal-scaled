//! Multiply policy — the schoolbook-vs-Karatsuba algorithm matcher.
//!
//! Like division, the integer multiply choice keys on the **runtime
//! length** of the operands, not the const limb count `N`, so it is a
//! [`Select::ByValue`] case in the canonical policy shape (see
//! `docs/ARCHITECTURE.md` → "Policy file structure"): the const layer
//! settles on "the value decides", the value-matcher classifies the
//! operand lengths and returns an [`Algorithm`] tag, and the dispatcher
//! does an **exhaustive** `match algo` to a pure kernel in
//! [`crate::int::algos::limbs`].
//!
//! The kernels ([`mul_schoolbook`] / [`mul_karatsuba`]) stay pure; this
//! file owns the *choice* — the benched crossover threshold
//! ([`KARATSUBA_THRESHOLD`]) is policy DATA here, not a magic number in a
//! kernel.

use crate::int::algos::limbs::{mul_karatsuba, mul_schoolbook};

// ── 1. the real multiply algorithms — NAMED, no `Default` ─────────────

/// The multiply algorithms the length matcher chooses between. Variants
/// are the CamelCase of each kernel fn's name minus the `mul_` function
/// prefix (`mul_schoolbook` → `Schoolbook`, `mul_karatsuba` →
/// `Karatsuba`) — strict 1:1 with the kernels in
/// [`crate::int::algos::limbs`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`mul_schoolbook`] — base-2⁶⁴ schoolbook outer product.
    Schoolbook,
    /// [`mul_karatsuba`] — non-allocating recursive Karatsuba.
    Karatsuba,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled algorithm, or "the (runtime) length decides". The mul picker
/// always returns `ByLength`: the choice is fully determined by the
/// operands' lengths, known only at run time. `ByAlgorithm` is part of
/// the canonical shape for uniformity across functions.
#[derive(Clone, Copy)]
enum Select {
    #[allow(dead_code)]
    ByAlgorithm(Algorithm),
    /// Classifier over `(a_len, b_len)` → the chosen algorithm.
    ByLength(fn(usize, usize) -> Algorithm),
}

// ── policy data: the benched crossover threshold ──────────────────────

/// Karatsuba threshold for the u64-base multiplier: the (equal) operand
/// limb-count at or above which [`mul_fast`] routes through the
/// non-allocating Karatsuba kernel instead of schoolbook.
///
/// [`mul_fast`] is the single site every equal-length wide multiply flows
/// through (via the `Int<N>` widening product), so one threshold governs
/// the crossover for every tier from one place. Set at **256 u64 limbs** —
/// above the widest equal-length multiply the crate emits (D1232 storage
/// = 64 limbs; the widest transcendental work-int is 192–256 limbs). At
/// this setting every shipped tier base-cases to the LLVM-unrolled
/// schoolbook [`mul_schoolbook`], so the kernel is reachable and correct
/// without changing the product behaviour of any shipped width.
///
/// NEEDS-BENCH: the 256 value is the spec/architecture default, not a
/// tuned crossover. It must be re-swept on the pinned GHA bench
/// (`benches/int_ops_micro.rs`, `mul_crossover`, plus the per-tier wide
/// `mul` cells) before being lowered to engage any shipped tier.
///
/// Must be `>= 4`: the recursion's z1 sum product runs on `⌈n/2⌉ + 1`
/// limbs, which only strictly shrinks below `n` once `n >= 4`, so a
/// threshold below 4 would fail to terminate. **Policy data** — the
/// kernels never see this number except as the threshold argument the
/// dispatcher threads in.
pub(crate) const KARATSUBA_THRESHOLD: usize = 256;

/// The production Karatsuba threshold as a function, so callers (and the
/// kernel's own scratch-sizing tests) reach the policy datum by intent.
#[inline]
pub(crate) const fn karatsuba_threshold() -> usize {
    KARATSUBA_THRESHOLD
}

// ── 3. the matcher: keyed on the runtime operand lengths ──────────────

/// Pick the multiply algorithm for the operands' lengths. Total over the
/// lengths; reproduces the exact byte-for-byte routing of the historic
/// inline picker: equal-length operands at or above
/// [`KARATSUBA_THRESHOLD`] take Karatsuba, everything else (unequal, or
/// below the threshold) takes schoolbook.
const fn select() -> Select {
    Select::ByLength(|a_len: usize, b_len: usize| {
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
pub(crate) fn mul_fast(a: &[u64], b: &[u64], out: &mut [u64]) {
    let algo = match const { select() } {
        Select::ByAlgorithm(a) => a,
        Select::ByLength(f) => f(a.len(), b.len()),
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
