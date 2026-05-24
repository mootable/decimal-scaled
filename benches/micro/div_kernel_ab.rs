// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide integer-division engine timing bench (`src/int/algos/div`).
//!
//! Exercises the production Knuth-D engine ([`div_knuth`]) and the full
//! divisor-shape dispatcher ([`div_rem::dispatch`]) at the WIDE limb counts
//! that regressed versus the 0.4.4 prod tag (D307=16, D616=32, D924=48,
//! D1232=64 u64 limbs). It is the fast pre-check that confirmed the
//! merged-carry D4 multiply-subtract win at D924/D1232 before the GHA
//! sweep. Two operand shapes per width:
//!  - `balanced`  — dividend and divisor both `N` limbs (the wide `rem` /
//!    `div_rem` shape; `m ~ 0`).
//!  - `wide_num`  — dividend `2N` limbs over an `N`-limb divisor (the `div`
//!    quotient shape; a full O(m·n) Knuth quotient loop — the engine hot
//!    path that `div` and the dispatched `cbrt` flow through).
//!
//! Inputs and outputs are `black_box`-guarded by the harness.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench div_kernel_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{div_dispatch_slice, div_knuth_slice};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

#[derive(Clone)]
struct Shape {
    label: &'static str,
    num: Vec<u64>,
    den: Vec<u64>,
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

fn shapes_bal(n: usize) -> Vec<Shape> {
    vec![Shape { label: "balanced", num: fill(7919, n), den: fill(104729, n) }]
}
fn shapes_wide(n: usize) -> Vec<Shape> {
    vec![Shape { label: "wide_num", num: fill(1009, 2 * n), den: fill(13, n) }]
}

fn run_knuth(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    div_knuth_slice(&s.num, &s.den, &mut q, &mut r);
    r
}
fn run_dispatch(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    div_dispatch_slice(&s.num, &s.den, &mut q, &mut r);
    r
}

fn compare_width(c: &mut Criterion, n: usize, label: &str, shapes: fn(usize) -> Vec<Shape>) {
    // Correctness gate: the engine and the dispatcher agree before timing.
    for s in shapes(n) {
        let mut q0 = vec![0u64; s.num.len()];
        let mut r0 = vec![0u64; s.num.len()];
        div_knuth_slice(&s.num, &s.den, &mut q0, &mut r0);
        let mut q1 = vec![0u64; s.num.len()];
        let mut r1 = vec![0u64; s.num.len()];
        div_dispatch_slice(&s.num, &s.den, &mut q1, &mut r1);
        assert_eq!(q0, q1, "knuth vs dispatch quot mismatch {label} {}", s.label);
        assert_eq!(r0, r1, "knuth vs dispatch rem mismatch {label} {}", s.label);
    }
    compare_all(
        c,
        &format!("div_kernel/{label}"),
        |s: &Shape| s.label.to_string(),
        shapes(n),
        vec![
            ("knuth", run_knuth as fn(Shape) -> Vec<u64>),
            ("dispatch", run_dispatch),
        ],
    );
}

fn bench(c: &mut Criterion) {
    for &(n, lbl) in &[
        (16usize, "D307_16limb"),
        (32, "D616_32limb"),
        (48, "D924_48limb"),
        (64, "D1232_64limb"),
    ] {
        compare_width(c, n, &format!("bal_{lbl}"), shapes_bal);
        compare_width(c, n, &format!("wide_{lbl}"), shapes_wide);
    }
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
