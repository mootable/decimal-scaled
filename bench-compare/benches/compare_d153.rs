//! Per-width branch-vs-prod compare bench for D153, fanned out over SCALE.
//! Run: cargo bench --bench compare_d153           (all scales for this width)
//!      cargo bench --bench compare_d153 -- s30    (just scale 30)
//!
//! Branch (`decimal_scaled::D153<S>`) vs prod (`prod::D153<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = dedup{0, S/2, S-1, 30 if 30 <= S-1} with S=153: {0, 30, 76, 152}.

#[macro_use]
mod compare_common;

width_bench!("D153", D153, D153, [0, 30, 76, 152]);
