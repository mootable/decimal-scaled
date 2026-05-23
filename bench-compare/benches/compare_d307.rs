//! Per-width branch-vs-prod compare bench for D307.
//! Run: cargo bench --bench compare_d307
//!
//! Branch (`decimal_scaled::D307<30>`) vs prod (`prod::D307<30>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D307", 30, D307, D307);
