//! Per-width branch-vs-prod compare bench for D115, fanned out over SCALE.
//! Run: cargo bench --bench compare_d115           (all scales for this width)
//!      cargo bench --bench compare_d115 -- s30    (just scale 30)
//!
//! Branch (`decimal_scaled::D115<S>`) vs prod (`prod::D115<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = dedup{0, S/2, S-1, 30 if 30 <= S-1} with S=115: {0, 30, 57, 114}.

#[macro_use]
mod compare_common;

width_bench!("D115", D115, D115, [0, 30, 57, 114]);
