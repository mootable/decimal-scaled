//! Per-width branch-vs-prod compare bench for D924, fanned out over SCALE.
//! Run: cargo bench --bench compare_d924           (all scales for this width)
//!      cargo bench --bench compare_d924 -- s30    (just scale 30)
//!
//! Branch (`decimal_scaled::D924<S>`) vs prod (`prod::D924<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = dedup{0, S/2, S-1, 30 if 30 <= S-1} with S=924: {0, 30, 462, 923}.

#[macro_use]
mod compare_common;

width_bench!("D924", D924, D924, [0, 30, 462, 923]);
