//! `neg_int_layer` — decimal negation via the `Int<N>` layer.

use crate::int::types::Int;

/// Decimal negation via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: panics (with "overflow") in debug builds
/// (`-MIN` is unrepresentable in two's-complement), wraps in release
/// (`-MIN == MIN`). No rescaling needed — the scale is unchanged.
#[inline]
pub(crate) fn neg_int_layer<const N: usize>(a: Int<N>) -> Int<N> {
    if cfg!(debug_assertions) {
        a.checked_neg().expect("attempt to negate with overflow")
    } else {
        a.wrapping_neg()
    }
}
