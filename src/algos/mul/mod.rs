//! Decimal multiplication algorithm family.
//!
//! One algorithm: [`mul_widen_divide`] — widens `a * b` to the next-up work
//! width then divides the product by `10^SCALE` to return to the storage
//! scale, with a value-gated fast path that skips the widen step when the
//! product provably fits the storage width. The per-`(N, SCALE)` choice
//! lives in [`crate::policy::mul`], which delegates *down* to this kernel.
//!
//! [`mul_widen_divide`]: crate::algos::mul::mul_widen_divide::mul_widen_divide

pub(crate) mod mul_widen_divide;
