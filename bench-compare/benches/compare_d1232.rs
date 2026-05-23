//! Per-width branch-vs-prod compare bench for D1232.
//! Run: cargo bench --bench compare_d1232
//!
//! Branch (`decimal_scaled::D1232<30>`) vs prod (`prod::D1232<30>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D1232", 30, D1232, D1232);
