//! Per-width branch-vs-prod compare bench for D57, fanned out over SCALE.
//! Run: cargo bench --bench compare_d57            (all scales for this width)
//!      cargo bench --bench compare_d57 -- s30     (just scale 30)
//!
//! Branch (`decimal_scaled::D57<S>`) vs prod (`prod::D57<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = {0, S/4, S/2, 3S/4, S-1} with S=57: [0, 14, 28, 42, 56].

#[macro_use]
mod compare_common;

width_bench!("D57", D57, D57, [0, 14, 28, 42, 56]);
