//! Per-width branch-vs-prod compare bench for D462, fanned out over SCALE.
//! Run: cargo bench --bench compare_d462           (all scales for this width)
//!      cargo bench --bench compare_d462 -- s30    (just scale 30)
//!
//! Branch (`decimal_scaled::D462<S>`) vs prod (`prod::D462<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = {0, S/4, S/2, 3S/4, S-1} with S=462: [0, 115, 231, 346, 461].

#[macro_use]
mod compare_common;

width_bench!("D462", D462, D462, [0, 115, 231, 346, 461]);
