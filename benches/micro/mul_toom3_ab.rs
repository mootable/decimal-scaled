// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! N-way microbench for the Toom-Cook 3-way integer multiply (step-1).
//!
//! Compares toom3 against slice-schoolbook, fixed-u64, u128-packed, and
//! Karatsuba at N = 24, 32, 48, 64. Bit-identicality asserted before timing.
//!
//! Run pinned on cores 16-19:
//!
//!   powershell.exe -NoProfile -File scripts/pin_run.ps1 \
//!     -Mask 0xF0000 -Bench mul_toom3_ab \
//!     -Features "wide x-wide xx-wide bench-alt"

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    mul_karatsuba_forced, mul_slice, mul_toom3_slice,
    mul_full_u64_24, mul_full_u64_32, mul_full_u64_48, mul_full_u64_64,
    mul_full_u128_24, mul_full_u128_32, mul_full_u128_48, mul_full_u128_64,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

#[derive(Clone)]
struct Operands<const N: usize> {
    label: &'static str,
    a: [u64; N],
    b: [u64; N],
}

fn fill<const N: usize>(seed: u64) -> [u64; N] {
    let mut out = [0u64; N];
    let mut state = seed;
    for x in out.iter_mut() {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        *x = z ^ (z >> 31);
    }
    out
}

fn operands<const N: usize>() -> Vec<Operands<N>> {
    vec![
        Operands { label: "low",  a: fill::<N>(3),    b: fill::<N>(5) },
        Operands { label: "mid",  a: fill::<N>(7),    b: fill::<N>(13) },
        Operands { label: "high", a: fill::<N>(1009), b: fill::<N>(7919) },
    ]
}

fn school_run<const N: usize>(ops: Operands<N>) -> Vec<u64> {
    let mut out = vec![0u64; 2 * N];
    mul_slice(&ops.a, &ops.b, &mut out);
    out
}

fn toom3_run<const N: usize>(ops: Operands<N>) -> Vec<u64> {
    let mut out = vec![0u64; 2 * N];
    mul_toom3_slice(&ops.a, &ops.b, &mut out);
    out
}

fn kara_run<const N: usize>(ops: Operands<N>) -> Vec<u64> {
    let mut out = vec![0u64; 2 * N];
    mul_karatsuba_forced(&ops.a, &ops.b, &mut out, N);
    out
}

fn bench_width_24(c: &mut Criterion) {
    const N: usize = 24;
    for ops in operands::<N>() {
        let oracle = school_run::<N>(ops.clone());
        let toom3  = toom3_run::<N>(ops.clone());
        let kara   = kara_run::<N>(ops.clone());
        assert_eq!(toom3, oracle, "toom3 != school n=24 ({})", ops.label);
        assert_eq!(kara,  oracle, "kara != school n=24 ({})", ops.label);
        let mut u64r  = vec![0u64; 48]; mul_full_u64_24(&ops.a, &ops.b, &mut u64r);
        let mut u128r = vec![0u64; 48]; mul_full_u128_24(&ops.a, &ops.b, &mut u128r);
        assert_eq!(u64r,  oracle, "u64 != school n=24 ({})", ops.label);
        assert_eq!(u128r, oracle, "u128 != school n=24 ({})", ops.label);
    }
    fn u64_fn(ops: Operands<24>) -> Vec<u64> { let mut o = vec![0u64;48]; mul_full_u64_24(&ops.a,&ops.b,&mut o); o }
    fn u128_fn(ops: Operands<24>) -> Vec<u64> { let mut o = vec![0u64;48]; mul_full_u128_24(&ops.a,&ops.b,&mut o); o }
    compare_all(c, "mul_toom3/Int1536", |ops: &Operands<N>| ops.label.to_string(), operands::<N>(), vec![
        ("school", school_run::<N> as fn(Operands<N>) -> Vec<u64>),
        ("u64",    u64_fn          as fn(Operands<N>) -> Vec<u64>),
        ("u128",   u128_fn         as fn(Operands<N>) -> Vec<u64>),
        ("kara",   kara_run::<N>   as fn(Operands<N>) -> Vec<u64>),
        ("toom3",  toom3_run::<N>  as fn(Operands<N>) -> Vec<u64>),
    ]);
}

fn bench_width_32(c: &mut Criterion) {
    const N: usize = 32;
    for ops in operands::<N>() {
        let oracle = school_run::<N>(ops.clone());
        let toom3  = toom3_run::<N>(ops.clone());
        let kara   = kara_run::<N>(ops.clone());
        assert_eq!(toom3, oracle, "toom3 != school n=32 ({})", ops.label);
        assert_eq!(kara,  oracle, "kara != school n=32 ({})", ops.label);
        let mut u64r  = vec![0u64; 64]; mul_full_u64_32(&ops.a, &ops.b, &mut u64r);
        let mut u128r = vec![0u64; 64]; mul_full_u128_32(&ops.a, &ops.b, &mut u128r);
        assert_eq!(u64r,  oracle, "u64 != school n=32 ({})", ops.label);
        assert_eq!(u128r, oracle, "u128 != school n=32 ({})", ops.label);
    }
    fn u64_fn(ops: Operands<32>) -> Vec<u64> { let mut o = vec![0u64;64]; mul_full_u64_32(&ops.a,&ops.b,&mut o); o }
    fn u128_fn(ops: Operands<32>) -> Vec<u64> { let mut o = vec![0u64;64]; mul_full_u128_32(&ops.a,&ops.b,&mut o); o }
    compare_all(c, "mul_toom3/Int2048", |ops: &Operands<N>| ops.label.to_string(), operands::<N>(), vec![
        ("school", school_run::<N> as fn(Operands<N>) -> Vec<u64>),
        ("u64",    u64_fn          as fn(Operands<N>) -> Vec<u64>),
        ("u128",   u128_fn         as fn(Operands<N>) -> Vec<u64>),
        ("kara",   kara_run::<N>   as fn(Operands<N>) -> Vec<u64>),
        ("toom3",  toom3_run::<N>  as fn(Operands<N>) -> Vec<u64>),
    ]);
}

fn bench_width_48(c: &mut Criterion) {
    const N: usize = 48;
    for ops in operands::<N>() {
        let oracle = school_run::<N>(ops.clone());
        let toom3  = toom3_run::<N>(ops.clone());
        let kara   = kara_run::<N>(ops.clone());
        assert_eq!(toom3, oracle, "toom3 != school n=48 ({})", ops.label);
        assert_eq!(kara,  oracle, "kara != school n=48 ({})", ops.label);
        let mut u64r  = vec![0u64; 96]; mul_full_u64_48(&ops.a, &ops.b, &mut u64r);
        let mut u128r = vec![0u64; 96]; mul_full_u128_48(&ops.a, &ops.b, &mut u128r);
        assert_eq!(u64r,  oracle, "u64 != school n=48 ({})", ops.label);
        assert_eq!(u128r, oracle, "u128 != school n=48 ({})", ops.label);
    }
    fn u64_fn(ops: Operands<48>) -> Vec<u64> { let mut o = vec![0u64;96]; mul_full_u64_48(&ops.a,&ops.b,&mut o); o }
    fn u128_fn(ops: Operands<48>) -> Vec<u64> { let mut o = vec![0u64;96]; mul_full_u128_48(&ops.a,&ops.b,&mut o); o }
    compare_all(c, "mul_toom3/Int3072", |ops: &Operands<N>| ops.label.to_string(), operands::<N>(), vec![
        ("school", school_run::<N> as fn(Operands<N>) -> Vec<u64>),
        ("u64",    u64_fn          as fn(Operands<N>) -> Vec<u64>),
        ("u128",   u128_fn         as fn(Operands<N>) -> Vec<u64>),
        ("kara",   kara_run::<N>   as fn(Operands<N>) -> Vec<u64>),
        ("toom3",  toom3_run::<N>  as fn(Operands<N>) -> Vec<u64>),
    ]);
}

fn bench_width_64(c: &mut Criterion) {
    const N: usize = 64;
    for ops in operands::<N>() {
        let oracle = school_run::<N>(ops.clone());
        let toom3  = toom3_run::<N>(ops.clone());
        let kara   = kara_run::<N>(ops.clone());
        assert_eq!(toom3, oracle, "toom3 != school n=64 ({})", ops.label);
        assert_eq!(kara,  oracle, "kara != school n=64 ({})", ops.label);
        let mut u64r  = vec![0u64; 128]; mul_full_u64_64(&ops.a, &ops.b, &mut u64r);
        let mut u128r = vec![0u64; 128]; mul_full_u128_64(&ops.a, &ops.b, &mut u128r);
        assert_eq!(u64r,  oracle, "u64 != school n=64 ({})", ops.label);
        assert_eq!(u128r, oracle, "u128 != school n=64 ({})", ops.label);
    }
    fn u64_fn(ops: Operands<64>) -> Vec<u64> { let mut o = vec![0u64;128]; mul_full_u64_64(&ops.a,&ops.b,&mut o); o }
    fn u128_fn(ops: Operands<64>) -> Vec<u64> { let mut o = vec![0u64;128]; mul_full_u128_64(&ops.a,&ops.b,&mut o); o }
    compare_all(c, "mul_toom3/Int4096", |ops: &Operands<N>| ops.label.to_string(), operands::<N>(), vec![
        ("school", school_run::<N> as fn(Operands<N>) -> Vec<u64>),
        ("u64",    u64_fn          as fn(Operands<N>) -> Vec<u64>),
        ("u128",   u128_fn         as fn(Operands<N>) -> Vec<u64>),
        ("kara",   kara_run::<N>   as fn(Operands<N>) -> Vec<u64>),
        ("toom3",  toom3_run::<N>  as fn(Operands<N>) -> Vec<u64>),
    ]);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_width_24(&mut c);
    bench_width_32(&mut c);
    bench_width_48(&mut c);
    bench_width_64(&mut c);
    c.final_summary();
}
