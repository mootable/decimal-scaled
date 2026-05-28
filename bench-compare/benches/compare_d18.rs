//! Per-width branch-vs-prod compare bench for D18, fanned out over SCALE.
//! Run: cargo bench --bench compare_d18            (all scales for this width)
//!      cargo bench --bench compare_d18 -- s9      (just scale 9)
//!
//! Branch (`decimal_scaled::D18<S>`) vs prod (`prod::D18<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = {0, S/4, S/2, 3S/4, S-1} with S=18: [0, 4, 9, 13, 17].
//! {0, 9, 17} — 30 is dropped because 30 > S-1 (17). Each scale becomes a
//! criterion group suffix `_s<scale>`, selectable via `-- s<scale>`.

#[macro_use]
mod compare_common;

width_bench!("D18", D18, D18, [0, 4, 9, 13, 17]);
