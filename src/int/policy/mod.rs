//! Per-width / limb-count algorithm-selection dispatch.
//!
//! The integer layer's counterpart to the decimal `policy/` bucket:
//! the place where operand width / limb count selects which kernel
//! runs. Unlike the decimal families, the integer types carry **no
//! `SCALE`** — `Int<N>` / `Uint<N>` and the named `Int*` / `Uint*`
//! aliases are pure fixed-width integers — so the dispatch key is the
//! limb count `N` (a compile-time const at every monomorphisation) and,
//! for division, the **runtime shape** of the operands (effective limb
//! count after stripping leading zeros). There is therefore no
//! `match (W, SCALE)` table to write for this layer: the decimal
//! `match (W, SCALE)` form has no `SCALE` axis here, and the divmod
//! selection cannot be a compile-time match because it keys on a
//! property only known at run time.
//!
//! The per-family dispatch as it actually ships today:
//!
//! - **add / sub / neg** — width-keyed only. One limb loop over the
//!   const `N` limbs ([`limbs::limbs_add_assign_u64_fixed`] /
//!   [`limbs::limbs_sub_assign_u64_fixed`] and `wrapping_neg`); no
//!   algorithm choice, the loop unrolls per monomorphisation.
//! - **mul** — schoolbook at every width. The fixed-width types use the
//!   truncated low-`N` schoolbook product
//!   ([`limbs::limbs_mul_low_u64_fixed`] / `limbs_sqr_low_u64_fixed`);
//!   the slice primitive [`limbs::limbs_mul_fast`] additionally crosses
//!   over to Karatsuba at `KARATSUBA_MIN` limbs, but the named integer
//!   widths in this crate stay in the schoolbook range.
//! - **÷ 10^SCALE** (decimal scale-narrowing) — this `(W, SCALE)`-keyed
//!   path is part of the **decimal** storage boundary, not the integer
//!   layer: D9 / D18 narrow tiers divide on hardware
//!   (`i128_divrem_by_u64_with_mode`), D38 takes the 2-word MG path, and
//!   the wide tiers route `mg_divide::div_wide_pow10_with`, which selects
//!   MG-single (`SCALE ≤ 19`, divisor fits one word) vs MG-lifted
//!   (wider) internally. It lives in `crate::algos::mg_divide` /
//!   `crate::macros::arithmetic`; the integer layer exposes only the raw
//!   divmod the wide path builds on.
//! - **divmod** — divisor-shape keyed at run time
//!   ([`limbs_divmod_dispatch`]): single-limb divisor (incl. every
//!   `10^scale`, `scale ≤ 19`) takes the hardware fast path; a divisor of
//!   `n ≥ 8` limbs whose numerator top ≥ `2·n` takes Burnikel–Ziegler;
//!   everything else takes Knuth Algorithm D. The const-evaluable
//!   `wrapping_div` / `wrapping_rem` stay on the `const fn`
//!   `limbs_divmod` so they can run at compile time.
//! - **isqrt / icbrt** — D38 has bespoke 256/384-bit kernels in
//!   `crate::algos::sqrt` / `crate::algos::cbrt`; the generic fixed-width
//!   types fall through to the shared limb isqrt / Brent–Zimmermann
//!   `root_int` ([`limbs::limbs_isqrt_u64`] and `Uint::root_int`).
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
