//! Width-matched limb addition and subtraction primitives.
//!
//! These are the live u64-base routines, surfaced here as the integer
//! layer's reusable building blocks. The definitions live in
//! [`crate::int::limbs`] (the named `IntXXXX` types and their 326 call
//! sites still reference them there); this module re-exports them so
//! the `Int<N>`/`Uint<N>` layer and any future width-generic algorithm
//! can reach them under one roof without a wholesale move.

// Surfaced as the reusable limb layer; not every primitive is consumed
// inside this crate yet (the named `IntXXXX` types call them at their
// definition site), so the re-export is intentionally broad.
#[allow(unused_imports)]
pub(crate) use crate::int::limbs::{
    limbs_add_assign_u64, limbs_add_assign_u64_fixed, limbs_sub_assign_u64,
    limbs_sub_assign_u64_fixed,
};
