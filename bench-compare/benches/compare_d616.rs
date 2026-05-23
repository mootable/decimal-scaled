//! Per-width branch-vs-prod compare bench for D616.
//! Run: cargo bench --bench compare_d616
//!
//! Branch (`decimal_scaled::D616<30>`) vs prod (`prod::D616<30>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D616", 30, D616, D616);
