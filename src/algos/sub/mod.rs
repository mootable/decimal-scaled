// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Decimal subtraction algorithm family.
//!
//! One algorithm: [`sub_int_layer`] — same-`SCALE` subtraction needs no
//! rescaling. The per-`(N, SCALE)` choice lives in [`crate::policy::sub`],
//! which delegates *down* to this kernel.
//!
//! [`sub_int_layer`]: crate::algos::sub::sub_int_layer::sub_int_layer

pub(crate) mod sub_int_layer;
