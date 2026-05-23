//! Per-width branch-vs-prod compare bench for D462.
//! Run: cargo bench --bench compare_d462
//!
//! Branch (`decimal_scaled::D462<30>`) vs prod (`prod::D462<30>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D462", 30, D462, D462);
