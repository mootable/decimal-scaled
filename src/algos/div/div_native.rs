// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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
//! * **`N == 1` (D18):** the scaled numerator `dividend * 10^SCALE` fits
//!   `i128` (an `i64` magnitude times `10^18 < 2^60`) and the `divisor` is an
//!   `i64` magnitude that fits `u64`. The rescale is therefore an `i128 / u64`
//!   schoolbook divide -- two hardware `divq` via
//!   [`crate::macros::arithmetic::i128_divrem_by_u64_with_mode`] -- not the
//!   LLVM `__divti3` soft-call an `i128 / i128` (the `apply_rounding`
//!   double-divmod) would lower to. The divisor's sign is folded into the
//!   numerator so the directed-rounding tie-break sees the true quotient sign.
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
//! (and for the zero divisor it never sees, guarded here): the default
//! operator panics on that overflow in BOTH debug and release — a fixed-width
//! decimal has no ±∞/NaN, so silently returning a wrapped value is a wrong
//! number with no signal. The explicit `wrapping_div` / `checked_div` /
//! `saturating_div` / `overflowing_div` variants (in `crate::macros::overflow`)
//! carry the modular / `None` / clamp / flag policies via their own `Int<N>`
//! paths, not this kernel.
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
/// Computes `(dividend * 10^SCALE) / divisor` rounded under `mode`. Panics on a zero
/// divisor and on `i128` overflow of the quotient in BOTH debug and release
/// per the decimal default-operator contract.
#[inline]
#[must_use]
pub(crate) fn div_native<const N: usize, const SCALE: u32>(
    dividend: Int<N>,
    divisor: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    if divisor == Int::<N>::ZERO {
        panic!("attempt to divide by zero");
    }

    if N == 1 {
        // D18: numerator dividend * 10^SCALE fits i128 (i64 magnitude * 10^18
        // < 2^124), and the divisor is an i64 magnitude that fits u64. The
        // rescale divide is therefore an i128 / u64 schoolbook divide -- two
        // hardware divq instructions via i128_divrem_by_u64_with_mode -- not
        // the __divti3 soft-call an i128 / i128 (apply_rounding) would lower to.
        let divisor_raw = divisor.as_i128();
        // Fold the divisor's sign into the numerator so the signed numerator
        // passed to `i128_divrem_by_u64` carries the TRUE result sign
        // (`sign(dividend) ^ sign(divisor)`). The helper decides the
        // directed-rounding tie-break from
        // the numerator sign (`result_positive = !n_neg`), so the divisor it
        // sees must be the positive magnitude AND the numerator must already
        // bear the quotient sign -- otherwise Floor / Ceiling round the wrong
        // way for a negative divisor.
        let divisor_is_negative = divisor_raw < 0;
        let numerator = dividend.as_i128() * crate::consts::pow10::dispatch_i128(SCALE);
        let numerator = if divisor_is_negative { -numerator } else { numerator };
        let divisor_mag: u64 = divisor_raw.unsigned_abs() as u64;
        let quotient =
            crate::macros::arithmetic::i128_divrem_by_u64_with_mode(numerator, divisor_mag, mode);
        assert!(
            quotient >= i64::MIN as i128 && quotient <= i64::MAX as i128,
            "attempt to divide with overflow"
        );
        return Int::<N>::from_i128(quotient);
    }

    // N == 2 (D38): the shared i128 / 256-bit kernel.
    let dividend_raw = dividend.as_i128();
    let divisor_raw = divisor.as_i128();
    match div_pow10_div_with::<SCALE>(dividend_raw, divisor_raw, mode) {
        Some(quotient) => Int::<N>::from_i128(quotient),
        None => panic!("attempt to divide with overflow"),
    }
}
