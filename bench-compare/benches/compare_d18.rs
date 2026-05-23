//! Per-width branch-vs-prod compare bench for D18.
//! Run: cargo bench --bench compare_d18
//!
//! Branch (`decimal_scaled::D18<9>`) vs prod (`prod::D18<9>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D18", 9, D18, D18);
