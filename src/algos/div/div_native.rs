//! `div_native` -- decimal division via the hardware `i128` path, for narrow
//! storage widths (`N <= 2`, i.e. D18 / D38).
//!
//! When the storage fits a single `i128` (`N == 1` is `i64`-backed but
//! widens losslessly; `N == 2` *is* `i128`), the whole scale-then-divide can
//! be done in hardware integers instead of forming a `2N`-limb scaled
//! numerator in a scratch buffer and routing it through the slice divide.
//!
//! Two specialised arms, selected on `N` at compile time (the unused arm is
//! dead-code-eliminated per monomorphisation):
//!
//! * **`N == 1` (D18):** the scaled numerator `a * 10^SCALE` fits `i128` (an
//!   `i64` magnitude times `10^18 < 2^60`) and the divisor `b` is an `i64`
//!   magnitude that fits `u64`. The rescale is therefore an `i128 / u64`
//!   schoolbook divide -- two hardware `divq` via
//!   [`crate::macros::arithmetic::i128_divrem_by_u64_with_mode`] -- not the
//!   LLVM `__divti3` soft-call an `i128 / i128` (the `apply_rounding`
//!   double-divmod) would lower to. `b`'s sign is folded into the numerator so
//!   the directed-rounding tie-break sees the true quotient sign. This mirrors
//!   0.4.4 native D18 div.
//! * **`N == 2` (D38):** the divisor can be a full `i128` and the scaled
//!   numerator can exceed `i128`, so the rescale delegates to the shared
//!   hardware kernel
//!   [`crate::algos::support::mg_divide::div_pow10_div_with`] (an `i128` fast
//!   path with a `256`-bit fallback; returns `None` on a zero divisor or on
//!   `i128` overflow of the quotient).
//!
//! # Overflow / divide-by-zero contract
//!
//! A zero divisor panics up front (matching `i128 /` and the `WidenScale`
//! kernel). `div_pow10_div_with` returns `None` for an out-of-range quotient
//! (and for the zero divisor it never sees, guarded here): debug-panic /
//! release-wrap per the standard integer contract. The release wrap is
//! re-derived as `(a * 10^SCALE) / b` with wrapping `Int<N>` arithmetic,
//! matching the naive form and the `WidenScale` wrap in range.
//!
//! # Layering
//!
//! All arithmetic is `i128` / `Int<N>` integer work dispatched DOWN to the
//! int layer and the shared `mg_divide` leaf; this fn never calls a decimal
//! method on its own value. Valid only for `N <= 2` (where `as_i128` is
//! lossless) -- [`crate::policy::div`] routes only `N == 1 | 2` here.

use crate::algos::support::mg_divide::div_pow10_div_with;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Hardware-`i128` decimal divide kernel for narrow storage (`N <= 2`).
///
/// Computes `(a * 10^SCALE) / b` rounded under `mode`. Panics on a zero
/// divisor. On `i128` overflow of the quotient, debug-panics / release-wraps
/// per the standard integer contract (the wrap re-derives via `Int<N>`
/// wrapping ops). `mult` is the pre-computed `10^SCALE` in `Int<N>`, used
/// only on the release-wrap path (the hardware kernel forms its own).
#[inline]
#[must_use]
pub(crate) fn div_native<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    if b == Int::<N>::ZERO {
        panic!("attempt to divide by zero");
    }

    if N == 1 {
        // D18: numerator a * 10^SCALE fits i128 (i64 magnitude * 10^18 < 2^124),
        // and the divisor b is an i64 magnitude that fits u64. The rescale
        // divide is therefore an i128 / u64 schoolbook divide -- two hardware
        // divq instructions via i128_divrem_by_u64_with_mode -- not the
        // __divti3 soft-call an i128 / i128 (apply_rounding) would lower to.
        let bi = b.as_i128();
        // Fold b's sign into the numerator so the signed numerator passed to
        // `i128_divrem_by_u64` carries the TRUE result sign (`sign(a) ^
        // sign(b)`). The helper decides the directed-rounding tie-break from
        // the numerator sign (`result_positive = !n_neg`), so the divisor it
        // sees must be the positive magnitude AND the numerator must already
        // bear the quotient sign -- otherwise Floor / Ceiling round the wrong
        // way for a negative divisor.
        let b_neg = bi < 0;
        let n = a.as_i128() * 10i128.pow(SCALE);
        let n = if b_neg { -n } else { n };
        let b_mag: u64 = bi.unsigned_abs() as u64;
        let result = crate::macros::arithmetic::i128_divrem_by_u64_with_mode(n, b_mag, mode);
        if cfg!(debug_assertions) {
            assert!(
                result >= i64::MIN as i128 && result <= i64::MAX as i128,
                "attempt to divide with overflow"
            );
        }
        return Int::<N>::from_i128(result);
    }

    // N == 2 (D38): the shared i128 / 256-bit kernel.
    let ai = a.as_i128();
    let bi = b.as_i128();
    match div_pow10_div_with::<SCALE>(ai, bi, mode) {
        Some(q) => Int::<N>::from_i128(q),
        None => {
            if cfg!(debug_assertions) {
                panic!("attempt to divide with overflow");
            }
            // Release wrap: (a * 10^SCALE) / b in wrapping Int<N> arithmetic.
            a.wrapping_mul(mult).wrapping_div(b)
        }
    }
}
