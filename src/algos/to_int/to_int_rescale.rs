// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `to_int_rescale` — round `raw / 10^SCALE` to a whole number through
//! the crate's `÷10^scale` matcher, then narrow to `i64`.
//!
//! The alternative to
//! [`to_int_quotient_remainder`](crate::algos::to_int::to_int_quotient_remainder),
//! and the same move
//! [`quantize_pow10`](crate::algos::quantize::quantize_pow10) makes: the
//! divide is not this kernel's to choose, so it routes through
//! [`dispatch_wide_pow10`](crate::algos::support::rescale::dispatch_wide_pow10)
//! and lets that matcher pick between the MG single-chunk, MG chain and
//! baked-Barrett kernels on the operand's real significant length.
//!
//! # Why this is a different algorithm, not a tidier spelling
//!
//! The rounding rule needs the truncated quotient AND the remainder. Two
//! typed operators (`raw / d` then `raw % d`) are two independent
//! full-width divisions for one division's worth of information, and
//! both go through the width-erased blanket `Int<N>` `/` / `%` door with
//! its build-max scratch. The rescale kernels produce the quotient and
//! apply `mode` in a single walk over the magnitude limbs, sized off the
//! operand's significant length rather than the storage width.
//!
//! That is a different cost curve, not a smaller constant, which is why
//! both kernels are kept and [`crate::policy::to_int`] chooses per cell:
//! the typed divides are a single hardware divide at `Int<1>`, where the
//! u128 magnitude set-up here has nothing to amortise against.
//!
//! # Rounding
//!
//! Every arm of the rescale matcher takes its verdict from
//! [`should_bump`](crate::support::rounding::should_bump), assembling
//! `remainder_cmp` as `|r|` against `|m| − |r|` (MG) or against `|m|/2`
//! (Barrett) — identical verdicts for the even divisor `10^SCALE`, and
//! identical to the comparison
//! [`to_int_quotient_remainder`](crate::algos::to_int::to_int_quotient_remainder)
//! forms. So the two kernels agree mode-for-mode by construction; this
//! one simply never writes the decision down.

use crate::algos::to_int::narrow_saturating::narrow_saturating_i64;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Rounds `raw / 10^SCALE` to a whole number under `mode` and narrows it
/// to `i64`, saturating out of range.
///
/// `SCALE == 0` is the identity — the value is already whole, so there is
/// nothing to round. It is taken here rather than left to the rescale
/// door, for two reasons. It const-folds: `SCALE` is a const generic, so
/// a scale-0 monomorphisation compiles to the narrow alone and every
/// other one drops the branch. And it makes the kernel's contract
/// self-evident instead of resting on the door's own `scale == 0`
/// early-out, which `quantize_pow10` never reaches (it returns on
/// `TARGET_SCALE == SCALE` first) and so does not exercise.
#[inline]
pub(crate) fn to_int_rescale<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> i64
where
    Limbs<N>: ComputeLimbs,
{
    if SCALE == 0 {
        return narrow_saturating_i64(raw);
    }
    let rounded = crate::algos::support::rescale::dispatch_wide_pow10(raw, SCALE, mode);
    narrow_saturating_i64(rounded)
}
