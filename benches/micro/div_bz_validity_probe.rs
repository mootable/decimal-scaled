// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Recursive Burnikel-Ziegler validity probe.
//!
//! A recursive BZ divide of a 2n-limb dividend by an n-limb divisor costs
//! at least ONE n-by-n back-multiply (Q*B) per top-level step, plus two
//! half-width recursive divides. Its asymptotic win over Knuth exists only
//! when that multiply is sub-quadratic. This crate fixes the Karatsuba
//! threshold at 256 limbs, so every division-reachable width (divisor <=
//! 64 limbs, dividend <= 128) uses schoolbook multiply.
//!
//! This probe times a single n-by-n schoolbook multiply against a full
//! div_knuth(2n, n). If the multiply alone already costs a meaningful
//! fraction of (or more than) the whole Knuth divide, recursive BZ -- which
//! performs that multiply PLUS recursion -- cannot beat Knuth at these
//! widths. It establishes the no-win conclusion empirically.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide" --bench div_bz_validity_probe`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{div_knuth_slice, mul_slice};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

#[derive(Clone)]
struct Shape {
    label: &'static str,
    n: usize,
}

fn fill(seed: u64, used: usize) -> Vec<u64> {
    let mut v = vec![0u64; used];
    for i in 0..used {
        v[i] = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add((i as u64).wrapping_mul(0x1357_9BDF))
            ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    }
    if used > 0 {
        v[used - 1] |= 0x8000_0000_0000_0000;
    }
    v
}

fn run_mul_nn(s: Shape) -> Vec<u64> {
    let a = fill(7919, s.n);
    let b = fill(104729, s.n);
    let mut out = vec![0u64; 2 * s.n + 1];
    mul_slice(&a, &b, &mut out);
    out
}

fn run_knuth_2n(s: Shape) -> Vec<u64> {
    let num = fill(1009, 2 * s.n);
    let den = fill(13, s.n);
    let mut q = vec![0u64; 2 * s.n];
    let mut r = vec![0u64; 2 * s.n];
    div_knuth_slice(&num, &den, &mut q, &mut r);
    r
}

fn bench(c: &mut Criterion) {
    for &(n, lbl) in &[
        (8usize, "D153_8limb"),
        (16, "D307_16limb"),
        (32, "D616_32limb"),
        (64, "D1232_64limb"),
        (128, "xscale_128limb"),
    ] {
        compare_all(
            c,
            &format!("div_bz_probe/{lbl}"),
            |s: &Shape| s.label.to_string(),
            vec![Shape { label: lbl, n }],
            vec![
                ("mul_nxn", run_mul_nn as fn(Shape) -> Vec<u64>),
                ("knuth_2nxn", run_knuth_2n),
            ],
        );
    }
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
