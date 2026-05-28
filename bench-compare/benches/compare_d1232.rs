//! Per-width branch-vs-prod compare bench for D1232, fanned out over SCALE.
//! Run: cargo bench --bench compare_d1232          (all scales for this width)
//!      cargo bench --bench compare_d1232 -- s30   (just scale 30)
//!
//! Branch (`decimal_scaled::D1232<S>`) vs prod (`prod::D1232<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = {0, S/4, S/2, 3S/4, S-1} with S=1232: [0, 308, 616, 924, 1231].

#[macro_use]
mod compare_common;

width_bench!("D1232", D1232, D1232, [0, 308, 616, 924, 1231]);
