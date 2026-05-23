//! Per-width branch-vs-prod compare bench for D924.
//! Run: cargo bench --bench compare_d924
//!
//! Branch (`decimal_scaled::D924<30>`) vs prod (`prod::D924<30>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D924", 30, D924, D924);
