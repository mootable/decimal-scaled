// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the INT-tier unary policies that have a registered
//! reference arm: `sqr`, `pow`, `isqrt`, `icbrt`.
//!
//! For each policy this benches the ROUTED algorithm against its unrouted
//! `Schoolbook` reference (the registered-but-unselected baseline) so the
//! `select` arm can be confirmed to route to the benched winner:
//!
//! - `sqr`   -> `half_product` vs `schoolbook` (truncated `x*x`).
//! - `pow`   -> `square_and_multiply` vs `schoolbook` (repeated multiply).
//! - `isqrt` -> `newton` (f64-seeded) vs `schoolbook` (bitwise).
//! - `icbrt` -> `newton` (f64-seeded) vs `schoolbook` (bitwise).
//!
//! Each candidate is asserted bit-identical across the operand spread before
//! timing (correctness gate); the harness `black_box`-guards inputs/outputs.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench int_unary_kernel_ab`

use criterion::Criterion;
use decimal_scaled::Uint;
use decimal_scaled::__bench_internals::{
    icbrt_newton_slice, icbrt_schoolbook_slice, isqrt_newton_slice, isqrt_schoolbook_slice,
    pow_schoolbook, pow_square_and_multiply, sqr_half_product, sqr_schoolbook,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use crate::ab_sweep as decimal_scaled_ab_sweep;
use ab_microbench::{compare_all, micro_criterion};

/// Deterministic limb fill for a `Uint<N>`.
fn synth<const N: usize>(seed: u64) -> Uint<N> {
    let mut mag = [0u64; N];
    for i in 0..N {
        mag[i] = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add(i as u64 * 0x1357_9BDF)
            ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    }
    Uint::<N>::from_limbs(mag)
}

// sqr: half_product vs schoolbook
#[derive(Clone)]
struct SqIn<const N: usize> {
    label: &'static str,
    x: Uint<N>,
}

fn sqr_inputs<const N: usize>() -> Vec<SqIn<N>> {
    vec![
        SqIn { label: "low", x: synth::<N>(7) },
        SqIn { label: "mid", x: synth::<N>(1009) },
        SqIn { label: "high", x: synth::<N>(7919) },
    ]
}

fn bench_sqr<const N: usize>(c: &mut Criterion, label: &str) {
    for i in sqr_inputs::<N>() {
        assert_eq!(
            sqr_half_product::<N>(i.x),
            sqr_schoolbook::<N>(i.x),
            "sqr half_product vs schoolbook {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("sqr/{label}"),
        |i: &SqIn<N>| i.label.to_string(),
        sqr_inputs::<N>(),
        vec![
            ("half_product", (|i: SqIn<N>| sqr_half_product::<N>(i.x)) as fn(SqIn<N>) -> Uint<N>),
            ("schoolbook", |i: SqIn<N>| sqr_schoolbook::<N>(i.x)),
        ],
    );
}

// pow: square_and_multiply vs schoolbook (small fixed exponents, root use)
#[derive(Clone)]
struct PowIn<const N: usize> {
    label: &'static str,
    base: Uint<N>,
    exp: u32,
}

fn pow_inputs<const N: usize>() -> Vec<PowIn<N>> {
    vec![
        PowIn { label: "e2", base: synth::<N>(7), exp: 2 },
        PowIn { label: "e3", base: synth::<N>(11), exp: 3 },
        PowIn { label: "e10", base: synth::<N>(13), exp: 10 },
    ]
}

fn bench_pow<const N: usize>(c: &mut Criterion, label: &str) {
    for i in pow_inputs::<N>() {
        assert_eq!(
            pow_square_and_multiply::<N>(i.base, i.exp),
            pow_schoolbook::<N>(i.base, i.exp),
            "pow sqm vs schoolbook {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("pow/{label}"),
        |i: &PowIn<N>| i.label.to_string(),
        pow_inputs::<N>(),
        vec![
            (
                "square_and_multiply",
                (|i: PowIn<N>| pow_square_and_multiply::<N>(i.base, i.exp))
                    as fn(PowIn<N>) -> Uint<N>,
            ),
            ("schoolbook", |i: PowIn<N>| pow_schoolbook::<N>(i.base, i.exp)),
        ],
    );
}

// isqrt / icbrt: newton vs schoolbook (slice kernels)
#[derive(Clone)]
struct RootIn {
    label: &'static str,
    n: Vec<u64>,
}

fn root_inputs(limbs: usize) -> Vec<RootIn> {
    let mk = |seed: u64| -> Vec<u64> {
        let mut v = vec![0u64; limbs];
        for i in 0..limbs {
            v[i] = seed
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(i as u64 * 0x1357_9BDF)
                ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
        }
        if v[limbs - 1] == 0 {
            v[limbs - 1] = 0x8000_0000_0000_0001;
        }
        v
    };
    vec![
        RootIn { label: "a", n: mk(7) },
        RootIn { label: "b", n: mk(1009) },
        RootIn { label: "c", n: mk(7919) },
    ]
}

fn isqrt_n(i: RootIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    isqrt_newton_slice(&i.n, &mut out);
    out
}
fn isqrt_s(i: RootIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    isqrt_schoolbook_slice(&i.n, &mut out);
    out
}
fn icbrt_n(i: RootIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    icbrt_newton_slice(&i.n, &mut out);
    out
}
fn icbrt_s(i: RootIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    icbrt_schoolbook_slice(&i.n, &mut out);
    out
}

fn bench_isqrt(c: &mut Criterion, limbs: usize, label: &str) {
    for i in root_inputs(limbs) {
        assert_eq!(isqrt_n(i.clone()), isqrt_s(i.clone()), "isqrt newton vs schoolbook {label} {}", i.label);
    }
    compare_all(
        c,
        &format!("isqrt/{label}"),
        |i: &RootIn| i.label.to_string(),
        root_inputs(limbs),
        vec![
            ("newton", Box::new(isqrt_n) as Box<dyn Fn(RootIn) -> Vec<u64>>),
            ("schoolbook", Box::new(isqrt_s)),
        ],
    );
}

fn bench_icbrt(c: &mut Criterion, limbs: usize, label: &str) {
    for i in root_inputs(limbs) {
        assert_eq!(icbrt_n(i.clone()), icbrt_s(i.clone()), "icbrt newton vs schoolbook {label} {}", i.label);
    }
    compare_all(
        c,
        &format!("icbrt/{label}"),
        |i: &RootIn| i.label.to_string(),
        root_inputs(limbs),
        vec![
            ("newton", Box::new(icbrt_n) as Box<dyn Fn(RootIn) -> Vec<u64>>),
            ("schoolbook", Box::new(icbrt_s)),
        ],
    );
}

fn bench(c: &mut Criterion) {
    decimal_scaled_ab_sweep!(c =>
        Uint<1> => |c: &mut Criterion| bench_sqr::<1>(c, "Int64_D18"),
        Uint<2> => |c: &mut Criterion| bench_sqr::<2>(c, "Int128_D38"),
        Uint<4> => |c: &mut Criterion| bench_sqr::<4>(c, "Int256_D76"),
        Uint<8> => |c: &mut Criterion| bench_sqr::<8>(c, "Int512_D153"),
        Uint<1> => |c: &mut Criterion| bench_pow::<1>(c, "Int64_D18"),
        Uint<2> => |c: &mut Criterion| bench_pow::<2>(c, "Int128_D38"),
        Uint<8> => |c: &mut Criterion| bench_pow::<8>(c, "Int512_D153")
    );
    bench_isqrt(c, 1, "n1_D18");
    bench_isqrt(c, 2, "n2_D38");
    bench_isqrt(c, 8, "n8_D153");
    bench_isqrt(c, 16, "n16_D307");
    bench_icbrt(c, 1, "n1_D18");
    bench_icbrt(c, 2, "n2_D38");
    bench_icbrt(c, 8, "n8_D153");
    bench_icbrt(c, 16, "n16_D307");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
