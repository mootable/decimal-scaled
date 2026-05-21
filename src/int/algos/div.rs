//! Width-matched limb division primitives.
//!
//! Re-exported from [`crate::int::limbs`]; see [`super`] for the
//! re-export rationale. The divisor-dispatch / Knuth-D / Burnikel-
//! Ziegler stack stays where it is — the integer layer routes through
//! [`limbs_divmod_u64`] (single-limb fast paths) and the dispatcher
//! for the multi-limb case.

#[allow(unused_imports)]
pub(crate) use crate::int::limbs::{
    limbs_divmod_dispatch_u64, limbs_divmod_u64, limbs_isqrt_u64,
};
