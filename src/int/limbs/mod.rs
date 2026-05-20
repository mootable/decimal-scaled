//! Reusable width-matched limb algorithms.
//!
//! The integer layer's "integer algos": the per-limb primitives that
//! `Int<N>` / `Uint<N>` (and future width-generic kernels) compose on.
//! Most are the live u64-base routines defined in [`crate::wide_int`],
//! surfaced here under one roof; the named `IntXXXX` types keep calling
//! them at their original home, so re-exporting (rather than moving)
//! leaves those call sites untouched. New routines that the legacy
//! types do not need — currently the truncated low-`N` product
//! [`mul::limbs_mul_low_u64_fixed`] — are defined directly in the
//! relevant submodule.

pub(crate) mod add_sub;
pub(crate) mod cmp;
pub(crate) mod div;
pub(crate) mod mul;
pub(crate) mod shift;
