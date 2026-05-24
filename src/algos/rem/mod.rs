//! Decimal remainder algorithm family.
//!
//! One algorithm: [`rem_int_layer`] — same-`SCALE` remainder needs no
//! rescaling. The per-`(N, SCALE)` choice lives in [`crate::policy::rem`],
//! which delegates *down* to this kernel.
//!
//! [`rem_int_layer`]: crate::algos::rem::rem_int_layer::rem_int_layer

pub(crate) mod rem_int_layer;
pub(crate) mod rem_native;
