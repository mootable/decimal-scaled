// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Decimal negation algorithm family.
//!
//! One algorithm: [`neg_int_layer`] — a sign flip needs no rescaling. The
//! per-`(N, SCALE)` choice lives in [`crate::policy::neg`], which delegates
//! *down* to this kernel.
//!
//! [`neg_int_layer`]: crate::algos::neg::neg_int_layer::neg_int_layer

pub(crate) mod neg_int_layer;
