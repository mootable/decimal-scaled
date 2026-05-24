//! Decimal addition algorithm family.
//!
//! One algorithm: [`add_int_layer`] — same-`SCALE` addition needs no
//! rescaling, so the storage-level sum is the answer. The per-`(N, SCALE)`
//! choice lives in [`crate::policy::add`], which delegates *down* to this
//! kernel.
//!
//! [`add_int_layer`]: crate::algos::add::add_int_layer::add_int_layer

pub(crate) mod add_int_layer;
