//! Reusable width-matched integer algorithms.
//!
//! The integer layer's algorithm bucket: the width-matched routines
//! that `Int<N>` / `Uint<N>` (and future width-generic kernels) compose
//! on. Most surface the live u64-base limb routines defined in
//! [`crate::int::limbs`] under one roof; the named `IntXXXX` types keep
//! calling them at their definition site, so re-exporting (rather than
//! moving) leaves those call sites untouched. Routines the named types
//! do not need — currently the truncated low-`N` product
//! [`mul::limbs_mul_low_u64_fixed`] and the half-product squaring
//! [`mul::limbs_sqr_low_u64_fixed`] — are defined directly in the
//! relevant submodule.

pub(crate) mod add_sub;
pub(crate) mod cmp;
pub(crate) mod div;
pub(crate) mod mul;
pub(crate) mod shift;
