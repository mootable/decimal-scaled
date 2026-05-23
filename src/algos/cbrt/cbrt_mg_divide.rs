//! `cbrt_mg_divide` — D38 cube-root kernel
//! (`mg_divide::cbrt_raw_with_signed`).
//!
//! Captures the **width-bespoke specialisation** that has lived on D38
//! since before the algorithm library existed: a hand-tuned cube root on
//! a 384-bit intermediate tailored to the `Int<2>` (`i128`/`u128`)
//! storage. Strictly faster than the generic Newton kernel widening
//! `Int<2> → Int<8>` and running the generic `icbrt`.
//!
//! Genuinely width-bespoke (Q4.1b): the body is `i128`/`u128`-specific
//! 384-bit arithmetic, so it cannot be made generic over the storage
//! width without losing the intrinsic-backed path. It serves `N == 2`
//! only (the D18 `N == 1` tier widens to `Int<2>` in the policy layer
//! and reuses it).
//!
//! Signature mirrors [`crate::algos::cbrt::cbrt_newton`]: takes the raw
//! storage integer, the scale, and the rounding mode; returns the raw
//! storage integer of the cube root. Sign of the input is preserved; the
//! underlying mg_divide function takes an unsigned magnitude plus a sign
//! flag.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// D38 cube-root kernel. Sign of the input is preserved.
///
/// `Int<2>` entry point: bridges the decimal storage type to the `i128`
/// core ([`cbrt_mg_divide_raw`]) at the algorithm boundary — the
/// hand-tuned 384-bit math is unchanged. `i128` does not escape this
/// module.
#[inline]
#[must_use]
pub(crate) fn cbrt_mg_divide(raw: Int<2>, scale: u32, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(cbrt_mg_divide_raw(raw.as_i128(), scale, mode))
}

/// `i128` core of the D38 cube-root kernel.
#[inline]
#[must_use]
fn cbrt_mg_divide_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let negative = raw < 0;
    let q =
        crate::algos::support::mg_divide::cbrt_raw_with_signed(raw.unsigned_abs(), scale, negative, mode);
    let result = q as i128;
    if negative {
        -result
    } else {
        result
    }
}
