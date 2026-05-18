//! D38 cube-root kernel — `mg_divide::cbrt_raw_with_signed`.
//!
//! Captures the width-level specialisation that has lived on D38: a
//! hand-tuned 384-bit cube-root path tailored to `i128` storage,
//! strictly faster than the generic wide kernel (which would widen
//! `i128 → Int512` and use the generic Newton path).
//!
//! Sign handling lives here; the underlying mg_divide function takes
//! an unsigned magnitude plus a sign flag.

use crate::rounding::RoundingMode;

/// D38 cube-root kernel. Sign of the input is preserved.
#[inline]
#[must_use]
pub(crate) fn cbrt(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let negative = raw < 0;
    let q = crate::mg_divide::cbrt_raw_with_signed(raw.unsigned_abs(), scale, negative, mode);
    let result = q as i128;
    if negative { -result } else { result }
}
