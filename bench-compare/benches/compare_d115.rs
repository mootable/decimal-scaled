//! Per-width branch-vs-prod compare bench for D115.
//! Run: cargo bench --bench compare_d115
//!
//! Branch (`decimal_scaled::D115<25>`) vs prod (`prod::D115<25>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D115", 25, D115, D115);
