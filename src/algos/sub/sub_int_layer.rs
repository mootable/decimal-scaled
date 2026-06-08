//! `sub_int_layer` — decimal subtraction via the `Int<N>` layer.

use crate::int::types::Int;

/// Decimal subtraction via the `Int<N>` layer. Panics on overflow in BOTH
/// debug and release — a fixed-width decimal has no ±∞/NaN and silently
/// returning a wrapped value is a wrong number with no signal, so the
/// default operator fails loudly (the explicit `wrapping_sub` /
/// `checked_sub` / `saturating_sub` / `overflowing_sub` variants carry the
/// modular / `None` / clamp / flag policies). No rescaling needed —
/// same-SCALE operands share the scale factor.
#[inline]
pub(crate) fn sub_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    a.checked_sub(b).expect("attempt to subtract with overflow")
}
