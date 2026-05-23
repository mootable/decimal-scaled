//! Per-width branch-vs-prod compare bench for D230.
//! Run: cargo bench --bench compare_d230
//!
//! Branch (`decimal_scaled::D230<30>`) vs prod (`prod::D230<30>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D230", 30, D230, D230);
