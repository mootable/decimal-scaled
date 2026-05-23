// SPDX-License-Identifier: MIT OR Apache-2.0
//
//! Retained algorithm-lab probes — NOT wired to dispatch.
//!
//! These are Tang-style table-driven exp kernels that were benchmarked
//! against the live `exp_series` kernel and lost
//! or broke even. They are kept for future re-probing once conditions
//! change (e.g. once Karatsuba multiplication lands or the work-width `W`
//! lift removes the per-tier cost). The live dispatch kernels are in the
//! parent `exp` module; these lost on perf and stay only as reference
//! probes.
#![allow(dead_code)]

pub(crate) mod tang_d462_s225_235;
pub(crate) mod tang_d1232_s610_620;
