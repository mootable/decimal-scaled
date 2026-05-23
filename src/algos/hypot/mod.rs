//! Hypotenuse algorithm family.
//!
//! Unlike the raw-storage root kernels in [`crate::algos::sqrt`] /
//! [`crate::algos::cbrt`], `hypot` is a **decimal-level** algorithm: it
//! composes the tier's own decimal operator surface and the `sqrt`
//! surface rather than working on the raw storage integer directly
//! (§1a — "use the tier's methods wherever possible"). The single
//! variant serves every `(N, SCALE)` cell; the per-cell selection lives
//! in [`crate::policy::hypot`].
//!
//! - [`hypot_scale_trick`] — `max(|a|,|b|)·sqrt(1 + (min/max)²)`, the
//!   overflow-avoiding scale-ratio trick. Generic over the decimal tier.
//!
//! [`hypot_scale_trick`]: crate::algos::hypot::hypot_scale_trick::hypot_scale_trick

pub(crate) mod hypot_scale_trick;
