// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Decimal `to_int` algorithm family — rounding a decimal's stored
//! integer to a whole number at `SCALE` 0 and narrowing it to `i64`.
//!
//! Two algorithms, both generic over the storage limb count `N` and the
//! const `SCALE`, differing in HOW they produce the truncated quotient
//! and its remainder:
//!
//! * [`to_int_quotient_remainder`] — two typed full-width divides on
//!   `Int<N>` (`raw / 10^SCALE` and `raw % 10^SCALE`), then the shared
//!   rounding decision. The shape that shipped in 0.5.x.
//! * [`to_int_rescale`] — one pass through the crate's `÷10^scale`
//!   matcher ([`crate::algos::support::rescale`]), which produces the
//!   quotient and applies `mode` in the same walk over the magnitude
//!   limbs.
//!
//! Both take their rounding verdict from the one shared decider,
//! [`should_bump`](crate::support::rounding::should_bump) — the
//! quotient-remainder kernel by calling it directly, the rescale kernel
//! transitively through every arm of the rescale matcher. Neither
//! carries a second copy of the eight-mode match.
//!
//! Which one runs for a given `(N, SCALE)` cell is
//! [`crate::policy::to_int`]'s decision. They are kept as alternatives
//! because their cost curves differ in shape, not by a constant: the
//! typed divides are a hardware divide at the narrow tiers and grow with
//! the full storage width, while the rescale pass carries a u128
//! magnitude set-up cost that amortises as the width grows.
//!
//! [`to_int_quotient_remainder`]:
//!     crate::algos::to_int::to_int_quotient_remainder::to_int_quotient_remainder
//! [`to_int_rescale`]: crate::algos::to_int::to_int_rescale::to_int_rescale

pub(crate) mod narrow_saturating;
pub(crate) mod to_int_quotient_remainder;
pub(crate) mod to_int_rescale;
