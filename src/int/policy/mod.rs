//! Per-width / limb-count algorithm-selection dispatch.
//!
//! The integer layer's counterpart to the decimal `policy/` bucket:
//! the place where operand width / limb count selects which kernel
//! runs. Two such decisions exist today:
//!
//! - **Multiply** — schoolbook below `KARATSUBA_MIN` limbs, Karatsuba
//!   at or above it (`limbs::limbs_mul_fast` / `limbs_mul_fast_u64`).
//! - **Divide** — single-limb hardware fast paths, then Knuth
//!   Algorithm D, then Burnikel–Ziegler for very wide divisors
//!   (`limbs::limbs_divmod_dispatch` / `limbs_divmod_dispatch_u64`).
//!
//! Both dispatchers are currently written inline in the divide/multiply
//! kernels in [`crate::int::limbs`], threaded through the same `const`
//! evaluation and scratch-buffer machinery as the kernels they choose
//! between. Lifting them out cleanly means separating the threshold
//! constants and the shape-classification (`strip leading zeros →
//! effective limb count`) from the buffer setup they share with the
//! kernels — a non-trivial split that risks the correctness invariant.
//!
//! This module is therefore the named home for that dispatch; the
//! thresholds and selection logic remain re-exported from where they
//! are defined until the extraction can be done without churning the
//! hot paths.
//!
//! TODO(0.5.0): finish policy extraction — move `KARATSUBA_MIN`, the
//! `BZ_THRESHOLD` divide selection, and the shape-classification helper
//! out of `limbs` into this module, leaving the kernels to take an
//! already-chosen algorithm.

// Re-export the dispatch entry points under the policy bucket so
// callers can reach them by intent rather than by their current
// physical home in `limbs`.
#[allow(unused_imports)]
pub(crate) use crate::int::limbs::{limbs_divmod_dispatch, limbs_divmod_dispatch_u64};
