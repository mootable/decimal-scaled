//! D38 exponential / base-2 exponential via widen → D57 wide_kernel
//! → narrow back.
//!
//! See `algos::ln::borrow_d57` for the broader rationale.
//!
//! Correctness note for `exp` / `exp2`: unlike ln/sin/cos/atan,
//! `exp(D38::<S>::MAX)` may overflow `D38::<S>` even when D57 holds
//! it comfortably. The bespoke `fixed_d38::exp_strict` panics on
//! overflow via `round_to_i128_with` returning `None`; these wrappers
//! preserve that panic semantic via the narrowing `TryFrom` failing
//! and the wrapping `expect(...)`.

// `crate::algos::ln::borrow_d57` is referenced in the module docs as a
// plain code span rather than an intra-doc link because both modules
// are pub(crate); intra-doc links to private items break the
// `RUSTDOCFLAGS=-D warnings` doc build (the docs.yml workflow uses
// this flag to catch broken intra-doc links).

use crate::types::widths::{D38, D57};
use crate::support::rounding::RoundingMode;

/// D38 exponential via widen → D57 wide_kernel → narrow back. Strict
/// working scale.
///
/// For `SCALE ∈ 18..=22` this routes through the narrow-GUARD
/// `lookup_d57_s18_22` kernel — the D38<19> midpoint scale picks up
/// the same ~22% reclaim the D57<20> direct path measures.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_tang::exp_strict::<SCALE>(widened.0, mode)
    } else {
        super::wide_kernel::exp_strict_d57(widened.0, mode, SCALE)
    };
    let wide = D57::<SCALE>::from_bits(raw_wide);
    let narrowed: D38<SCALE> = wide.try_into().unwrap_or_else(|_| panic!(
        "exp_strict: result out of range — produced {wide}, D38<{SCALE}> represents only |x| < 1.7e{}",
        38_i32 - SCALE as i32,
    ));
    narrowed.0
}

/// D38 base-2 exponential via widen → D57 inherent `exp2_strict_with`
/// → narrow back. D57's `exp2_strict_with` shares the
/// `wide_trig_d57::exp_fixed` core that the borrow `exp_strict`
/// uses, so this picks up the same speedup.
#[inline]
#[must_use]
pub(crate) fn exp2_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result = widened.exp2_strict_with(mode);
    let narrowed: D38<SCALE> = result.try_into().unwrap_or_else(|_| panic!(
        "exp2_strict: result out of range — produced {result}, D38<{SCALE}> represents only |x| < 1.7e{}",
        38_i32 - SCALE as i32,
    ));
    narrowed.0
}
