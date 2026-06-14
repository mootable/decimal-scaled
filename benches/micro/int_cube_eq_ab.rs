// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the integer `cube` and `eq` policies
//! (`src/int/policy/cube.rs`, `src/int/policy/eq.rs`).
//!
//! Each policy currently routes a single live kernel; this bench measures the
//! UNWIRED candidate against the shipped kernel across the storage-width axis
//! `N`, so the policy-mapper pass can wire the candidate at the cells where it
//! wins (or leave it unwired with a clean "no win" verdict):
//!
//! - `cube`: `schoolbook` -> `cube_schoolbook` (the shipped sqr-then-multiply
//!   `x²·x`) vs `comba` -> `cube_fused_comba` (the candidate single fused
//!   product-scanning pass).
//! - `eq`: `limbwise` -> `eq_limbwise` (the shipped equality, reusing the
//!   comparison kernel `cmp_fixed`) vs `xor_fold` -> `eq_xor_fold` (the
//!   candidate branchless OR-fold of per-limb XORs).
//!
//! Each candidate is asserted bit-identical to the shipped kernel across the
//! operand spread BEFORE timing (the validity wall); the harness
//! `black_box`-guards inputs/outputs.
//!
//! Run: `cargo bench --features "wide x-wide xx-wide bench-alt" --bench int_cube_eq_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    cube_fused_comba, cube_schoolbook, eq_limbwise, eq_xor_fold,
};
use decimal_scaled::{Int, Uint};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use crate::ab_sweep as decimal_scaled_ab_sweep;
use ab_microbench::{compare_all, micro_criterion};

/// Deterministic SplitMix64 limb fill.
fn mix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// ── cube: schoolbook vs comba ─────────────────────────────────────────────

#[derive(Clone)]
struct CubeIn<const N: usize> {
    label: &'static str,
    x: Uint<N>,
}

fn cube_inputs<const N: usize>() -> Vec<CubeIn<N>> {
    let mk = |seed: u64| -> Uint<N> {
        let mut s = seed;
        let mut mag = [0u64; N];
        for limb in mag.iter_mut() {
            *limb = mix(&mut s);
        }
        Uint::<N>::from_limbs(mag)
    };
    vec![
        CubeIn { label: "low", x: mk(7) },
        CubeIn { label: "mid", x: mk(1009) },
        CubeIn { label: "high", x: mk(7919) },
    ]
}

fn bench_cube<const N: usize>(c: &mut Criterion, label: &str) {
    for i in cube_inputs::<N>() {
        assert_eq!(
            cube_fused_comba::<N>(i.x).as_limbs(),
            cube_schoolbook::<N>(i.x).as_limbs(),
            "cube comba vs schoolbook {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("cube/{label}"),
        |i: &CubeIn<N>| i.label.to_string(),
        cube_inputs::<N>(),
        vec![
            ("schoolbook", (|i: CubeIn<N>| cube_schoolbook::<N>(i.x)) as fn(CubeIn<N>) -> Uint<N>),
            ("comba", |i: CubeIn<N>| cube_fused_comba::<N>(i.x)),
        ],
    );
}

// ── eq: limbwise vs xor_fold ──────────────────────────────────────────────

#[derive(Clone)]
struct EqIn<const N: usize> {
    label: &'static str,
    a: Int<N>,
    b: Int<N>,
}

fn eq_inputs<const N: usize>() -> Vec<EqIn<N>> {
    let mut s = 0xC0FF_EE00_1234_5678_u64 ^ (N as u64);
    let mut mk = || -> [u64; N] {
        let mut mag = [0u64; N];
        for limb in mag.iter_mut() {
            *limb = mix(&mut s);
        }
        mag
    };
    let base = mk();
    // equal pair (the worst case — must read every limb)
    let eq_a = Int::<N>::from_limbs(base);
    let eq_b = Int::<N>::from_limbs(base);
    // differ in the bottom limb (early-out for an MSB-first cmp, full read for the fold)
    let mut lo = base;
    lo[0] ^= 1;
    let lo_b = Int::<N>::from_limbs(lo);
    // differ in the top limb (early-out for the fold's natural order is the same)
    let mut hi = base;
    hi[N - 1] ^= 1 << 63;
    let hi_b = Int::<N>::from_limbs(hi);
    vec![
        EqIn { label: "equal", a: eq_a, b: eq_b },
        EqIn { label: "diff_lo", a: eq_a, b: lo_b },
        EqIn { label: "diff_hi", a: eq_a, b: hi_b },
    ]
}

fn bench_eq<const N: usize>(c: &mut Criterion, label: &str) {
    for i in eq_inputs::<N>() {
        assert_eq!(
            eq_xor_fold::<N>(i.a, i.b),
            eq_limbwise::<N>(i.a, i.b),
            "eq xor_fold vs limbwise {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("eq/{label}"),
        |i: &EqIn<N>| i.label.to_string(),
        eq_inputs::<N>(),
        vec![
            ("limbwise", (|i: EqIn<N>| eq_limbwise::<N>(i.a, i.b)) as fn(EqIn<N>) -> bool),
            ("xor_fold", |i: EqIn<N>| eq_xor_fold::<N>(i.a, i.b)),
        ],
    );
}

fn bench(c: &mut Criterion) {
    decimal_scaled_ab_sweep!(c =>
        Uint<1>  => |c: &mut Criterion| bench_cube::<1>(c, "n1_D18"),
        Uint<2>  => |c: &mut Criterion| bench_cube::<2>(c, "n2_D38"),
        Uint<3>  => |c: &mut Criterion| bench_cube::<3>(c, "n3_D57"),
        Uint<4>  => |c: &mut Criterion| bench_cube::<4>(c, "n4_D76"),
        Uint<6>  => |c: &mut Criterion| bench_cube::<6>(c, "n6_D115"),
        Uint<8>  => |c: &mut Criterion| bench_cube::<8>(c, "n8_D153"),
        Uint<16> => |c: &mut Criterion| bench_cube::<16>(c, "n16_D307"),
        Uint<32> => |c: &mut Criterion| bench_cube::<32>(c, "n32_D616"),
        Uint<64> => |c: &mut Criterion| bench_cube::<64>(c, "n64_D1232"),
        Int<1>   => |c: &mut Criterion| bench_eq::<1>(c, "n1_D18"),
        Int<2>   => |c: &mut Criterion| bench_eq::<2>(c, "n2_D38"),
        Int<3>   => |c: &mut Criterion| bench_eq::<3>(c, "n3_D57"),
        Int<4>   => |c: &mut Criterion| bench_eq::<4>(c, "n4_D76"),
        Int<6>   => |c: &mut Criterion| bench_eq::<6>(c, "n6_D115"),
        Int<8>   => |c: &mut Criterion| bench_eq::<8>(c, "n8_D153"),
        Int<16>  => |c: &mut Criterion| bench_eq::<16>(c, "n16_D307"),
        Int<32>  => |c: &mut Criterion| bench_eq::<32>(c, "n32_D616"),
        Int<64>  => |c: &mut Criterion| bench_eq::<64>(c, "n64_D1232")
    );
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
