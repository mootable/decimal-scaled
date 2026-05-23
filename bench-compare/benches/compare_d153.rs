//! Per-width branch-vs-prod compare bench for D153.
//! Run: cargo bench --bench compare_d153
//!
//! Branch (`decimal_scaled::D153<25>`) vs prod (`prod::D153<25>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D153", 25, D153, D153);
