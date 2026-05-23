//! Per-width branch-vs-prod compare bench for D76.
//! Run: cargo bench --bench compare_d76
//!
//! Branch (`decimal_scaled::D76<20>`) vs prod (`prod::D76<20>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.

#[macro_use]
mod compare_common;

width_bench!("D76", 20, D76, D76);
