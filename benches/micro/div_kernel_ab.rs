// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide integer-division engine timing bench (`src/int/algos/div`).
//!
//! A/B the two production division engines directly, **Knuth-D**
//! ([`div_knuth`]) versus the **Burnikel-Ziegler chunking** engine
//! ([`div_burnikel_ziegler_with_knuth`]), to locate the empirical limb
//! crossover that sets [`crate::int::policy::div_rem::BZ_THRESHOLD`].
//!
//! The BZ engine carries an internal engagement guard (`n <
//! BZ_THRESHOLD` short-circuits to Knuth), so it is exercised here via
//! `div_bz_forced_slice`, which bypasses the guard and always runs the
//! chunking core. This lets the sweep measure BZ at sub-threshold widths
//! and find where it starts beating Knuth.
//!
//! Candidate divisor widths (effective u64 limbs → tier):
//!  3=D57, 4=D76, 6=D115, 8=D153, 12=D230, 16=D307, 24=D462, 32=D616,
//!  48=D924, 64=D1232.
//!
//! Two operand shapes per width:
//!  - `balanced`  — dividend and divisor both `N` limbs (the wide `rem` /
//!    `div_rem` shape; `m ~ 0`). BZ's `top >= 2*n` engagement gate is NOT
//!    met here, so this shape is informational (Knuth always wins/ties).
//!  - `wide_num`  — dividend `2N` limbs over an `N`-limb divisor (the
//!    `div` quotient shape; the full O(m*n) Knuth quotient loop and the
//!    shape BZ's gate actually targets — this is the crossover-deciding
//!    shape).
//!
//! Inputs and outputs are `black_box`-guarded by the harness.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench div_kernel_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    div_bz_forced_slice, div_dispatch_slice, div_knuth_slice, div_knuth_u128_limb_slice,
};

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
fn run_bz(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    div_bz_forced_slice(&s.num, &s.den, &mut q, &mut r);
    r
}
fn run_u128(s: Shape) -> Vec<u64> {
    let mut q = vec![0u64; s.num.len()];
    let mut r = vec![0u64; s.num.len()];
    div_knuth_u128_limb_slice(&s.num, &s.den, &mut q, &mut r);
    r
}

fn compare_width(c: &mut Criterion, n: usize, label: &str, shapes: fn(usize) -> Vec<Shape>) {
    // Correctness gate: forced BZ and Knuth must agree before timing — they
    // are two engines for the same exact-integer result (bit-identical).
    for s in shapes(n) {
        let mut q0 = vec![0u64; s.num.len()];
        let mut r0 = vec![0u64; s.num.len()];
        div_knuth_slice(&s.num, &s.den, &mut q0, &mut r0);
        let mut q1 = vec![0u64; s.num.len()];
        let mut r1 = vec![0u64; s.num.len()];
        div_bz_forced_slice(&s.num, &s.den, &mut q1, &mut r1);
        assert_eq!(q0, q1, "knuth vs bz quot mismatch {label} {}", s.label);
        assert_eq!(r0, r1, "knuth vs bz rem mismatch {label} {}", s.label);
        // And the production dispatcher still agrees (engine-choice neutral).
        let mut q2 = vec![0u64; s.num.len()];
        let mut r2 = vec![0u64; s.num.len()];
        div_dispatch_slice(&s.num, &s.den, &mut q2, &mut r2);
        assert_eq!(q0, q2, "knuth vs dispatch quot mismatch {label} {}", s.label);
        assert_eq!(r0, r2, "knuth vs dispatch rem mismatch {label} {}", s.label);
        // And the u128-limb candidate (base 2^128); bit-identical, falling
        // back to base-2^64 Knuth on odd / sub-4-limb divisors.
        let mut q3 = vec![0u64; s.num.len()];
        let mut r3 = vec![0u64; s.num.len()];
        div_knuth_u128_limb_slice(&s.num, &s.den, &mut q3, &mut r3);
        assert_eq!(q0, q3, "knuth vs u128 quot mismatch {label} {}", s.label);
        assert_eq!(r0, r3, "knuth vs u128 rem mismatch {label} {}", s.label);
    }
    compare_all(
        c,
        &format!("div_kernel/{label}"),
        |s: &Shape| s.label.to_string(),
        shapes(n),
        vec![
            ("knuth", run_knuth as fn(Shape) -> Vec<u64>),
            ("bz", run_bz),
            ("u128", run_u128),
        ],
    );
}

fn bench(c: &mut Criterion) {
    for &(n, lbl) in &[
        (3usize, "D57_3limb"),
        (4, "D76_4limb"),
        (6, "D115_6limb"),
        (8, "D153_8limb"),
        (12, "D230_12limb"),
        (16, "D307_16limb"),
        (24, "D462_24limb"),
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
