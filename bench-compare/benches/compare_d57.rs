//! Per-width branch-vs-prod compare bench for D57.
//! Run: cargo bench --bench compare_d57
//!
//! Branch (`decimal_scaled::D57<20>`) vs prod (`prod::D57<20>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D57", 20, D57, D57);
