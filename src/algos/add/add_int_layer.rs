// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `add_int_layer` — decimal addition via the `Int<N>` layer.

use crate::int::types::Int;

/// Decimal addition via the `Int<N>` layer. Panics on overflow in BOTH
/// debug and release — a fixed-width decimal has no ±∞/NaN and silently
/// returning a wrapped value is a wrong number with no signal, so the
/// default operator fails loudly (the explicit `wrapping_add` /
/// `checked_add` / `saturating_add` / `overflowing_add` variants carry the
/// modular / `None` / clamp / flag policies). No rescaling needed —
/// same-SCALE operands share the scale factor.
#[inline]
pub(crate) fn add_int_layer<const N: usize>(lhs: Int<N>, rhs: Int<N>) -> Int<N> {
    lhs.checked_add(rhs).expect("attempt to add with overflow")
}
