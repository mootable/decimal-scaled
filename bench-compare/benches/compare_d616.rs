//! Per-width branch-vs-prod compare bench for D616, fanned out over SCALE.
//! Run: cargo bench --bench compare_d616           (all scales for this width)
//!      cargo bench --bench compare_d616 -- s30    (just scale 30)
//!
//! Branch (`decimal_scaled::D616<S>`) vs prod (`prod::D616<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = {0, S/4, S/2, 3S/4, S-1} with S=616: [0, 154, 308, 462, 615].

#[macro_use]
mod compare_common;

width_bench!("D616", D616, D616, [0, 154, 308, 462, 615]);
