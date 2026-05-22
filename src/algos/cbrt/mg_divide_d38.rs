//! D38 cube-root kernel — `mg_divide::cbrt_raw_with_signed`.
//!
//! Captures the width-level specialisation that has lived on D38: a
//! hand-tuned 384-bit cube-root path tailored to `i128` storage,
//! strictly faster than the generic wide kernel (which would widen
//! `i128 → Int512` and use the generic Newton path).
//!
//! Sign handling lives here; the underlying mg_divide function takes
//! an unsigned magnitude plus a sign flag.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// D38 cube-root kernel. Sign of the input is preserved.
///
/// `Int<2>` entry point: bridges the decimal storage type to the `i128`
/// core ([`cbrt_raw`]) at the algorithm boundary — the hand-tuned
/// 384-bit math is unchanged. `i128` does not escape this module.
#[inline]
#[must_use]
pub(crate) fn cbrt(raw: Int<2>, scale: u32, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(cbrt_raw(raw.as_i128(), scale, mode))
}

/// `i128` core of the D38 cube-root kernel.
#[inline]
#[must_use]
fn cbrt_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let negative = raw < 0;
    let q =
        crate::algos::mg_divide::cbrt_raw_with_signed(raw.unsigned_abs(), scale, negative, mode);
    let result = q as i128;
    if negative { -result } else { result }
}
