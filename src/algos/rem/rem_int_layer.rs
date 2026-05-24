//! `rem_int_layer` — decimal remainder via the `Int<N>` layer.

use crate::int::types::Int;

/// Decimal remainder via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: division by zero always panics; `MIN % -ONE`
/// panics in debug (with "overflow") and wraps in release (matching
/// `i128::wrapping_rem`). No rescaling needed — same-SCALE operands share
/// the scale factor.
#[inline]
pub(crate) fn rem_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    if cfg!(debug_assertions) {
        // `checked_rem` returns `None` for both divide-by-zero and
        // `MIN % -ONE`. `wrapping_rem` panics for divide-by-zero and returns
        // the wrapped value for `MIN % -ONE`, so a non-panicking return means
        // we have an `MIN % -ONE` overflow.
        match a.checked_rem(b) {
            Some(v) => v,
            None => {
                let _ = a.wrapping_rem(b);
                panic!("attempt to calculate the remainder with overflow");
            }
        }
    } else {
        a.wrapping_rem(b)
    }
}
