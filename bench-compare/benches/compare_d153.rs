//! Per-width branch-vs-prod compare bench for D153, fanned out over SCALE.
//! Run: cargo bench --bench compare_d153           (all scales for this width)
//!      cargo bench --bench compare_d153 -- s30    (just scale 30)
//!
//! Branch (`decimal_scaled::D153<S>`) vs prod (`prod::D153<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = {0, S/4, S/2, 3S/4, S-1} with S=153: [0, 38, 76, 114, 152].

#[macro_use]
mod compare_common;

width_bench!("D153", D153, D153, [0, 38, 76, 114, 152]);
