//! `neg_int_layer` — decimal negation via the `Int<N>` layer.

use crate::int::types::Int;

/// Decimal negation via the `Int<N>` layer. Panics on overflow in BOTH
/// debug and release (`-MIN` is unrepresentable in two's-complement) — a
/// fixed-width decimal has no ±∞/NaN and silently returning the wrapped
/// `-MIN == MIN` is a wrong number with no signal, so the default operator
/// fails loudly (the explicit `wrapping_neg` / `checked_neg` /
/// `saturating_neg` / `overflowing_neg` variants carry the modular / `None`
/// / clamp / flag policies). No rescaling needed — the scale is unchanged.
#[inline]
pub(crate) fn neg_int_layer<const N: usize>(a: Int<N>) -> Int<N> {
    a.checked_neg().expect("attempt to negate with overflow")
}
