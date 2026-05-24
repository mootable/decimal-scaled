//! `sub_int_layer` — decimal subtraction via the `Int<N>` layer.

use crate::int::types::Int;

/// Decimal subtraction via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: panics (with "overflow") in debug builds,
/// wraps (two's-complement) in release. No rescaling needed — same-SCALE
/// operands share the scale factor.
#[inline]
pub(crate) fn sub_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    if cfg!(debug_assertions) {
        a.checked_sub(b).expect("attempt to subtract with overflow")
    } else {
        a.wrapping_sub(b)
    }
}
