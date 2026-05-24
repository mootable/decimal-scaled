//! Decimal division algorithm family.
//!
//! One algorithm: [`div_widen_scale`] — widens `a * 10^SCALE` to the
//! next-up work width `W` then divides by `b`, rounding under `mode`, with
//! a value-gated fast path that skips the widen step when the scaled
//! numerator provably fits the storage width. The per-`(N, SCALE)` choice
//! lives in [`crate::policy::div`], which delegates *down* to this kernel.
//!
//! [`div_widen_scale`]: crate::algos::div::div_widen_scale::div_widen_scale

pub(crate) mod div_widen_scale;
