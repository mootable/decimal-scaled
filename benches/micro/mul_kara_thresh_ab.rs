// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Karatsuba-u128 recursion-threshold sweep (step-2 tuning).
//!
//! The int-mul map showed Karatsuba-u128 overtaking schoolbook-u128 at N>=128
//! at threshold = N (a single split). This bench sweeps the recursion base
//! width (threshold, in u64-limb units) at the crossover region + the top
//! widths, racing each against the schoolbook-u128 baseline, to find the
//! optimal depth AND whether a tuned threshold moves the crossover below 128.
//! Bit-identicality to the slice schoolbook oracle is asserted before timing.
//!
//! Run pinned on cores 16-19:
//!   powershell.exe -NoProfile -File scripts/pin_run.ps1 \
//!     -Mask 0xF0000 -Bench mul_kara_thresh_ab \
//!     -Features "wide x-wide xx-wide bench-alt"

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    mul_slice,
    mul_full_u128_64, mul_full_u128_96, mul_full_u128_128, mul_full_u128_256,
    mul_kara_u128_64_t, mul_kara_u128_96_t, mul_kara_u128_128_t, mul_kara_u128_256_t,
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

fn oracle<const N: usize>(ops: &Operands<N>) -> Vec<u64> {
    let mut out = vec![0u64; 2 * N];
    mul_slice(&ops.a, &ops.b, &mut out);
    out
}

macro_rules! thresh_bench {
    ($fn:ident, $group:expr, $N:literal, $D:literal, $base:ident, $kt:ident) => {
        fn $fn(c: &mut Criterion) {
            const N: usize = $N;
            for ops in operands::<N>() {
                let oc = oracle::<N>(&ops);
                let mut r = vec![0u64; $D]; $base(&ops.a, &ops.b, &mut r);
                assert_eq!(r, oc, "school_u128 != oracle {} ({})", $N, ops.label);
                for t in [16usize, 24, 32, 48] {
                    let mut r = vec![0u64; $D]; $kt(&ops.a, &ops.b, &mut r, t);
                    assert_eq!(r, oc, "kara_u128 t={} != oracle {} ({})", t, $N, ops.label);
                }
            }
            fn base_run(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $base(&ops.a, &ops.b, &mut o); o }
            fn k16(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $kt(&ops.a, &ops.b, &mut o, 16); o }
            fn k24(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $kt(&ops.a, &ops.b, &mut o, 24); o }
            fn k32(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $kt(&ops.a, &ops.b, &mut o, 32); o }
            fn k48(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $kt(&ops.a, &ops.b, &mut o, 48); o }
            compare_all(c, $group, |ops: &Operands<N>| ops.label.to_string(), operands::<N>(), vec![
                ("school_u128", base_run as fn(Operands<N>) -> Vec<u64>),
                ("kara_t16",    k16      as fn(Operands<N>) -> Vec<u64>),
                ("kara_t24",    k24      as fn(Operands<N>) -> Vec<u64>),
                ("kara_t32",    k32      as fn(Operands<N>) -> Vec<u64>),
                ("kara_t48",    k48      as fn(Operands<N>) -> Vec<u64>),
            ]);
        }
    };
}

thresh_bench!(tb_64,  "karathresh/n64",  64,  128, mul_full_u128_64,  mul_kara_u128_64_t);
thresh_bench!(tb_96,  "karathresh/n96",  96,  192, mul_full_u128_96,  mul_kara_u128_96_t);
thresh_bench!(tb_128, "karathresh/n128", 128, 256, mul_full_u128_128, mul_kara_u128_128_t);
thresh_bench!(tb_256, "karathresh/n256", 256, 512, mul_full_u128_256, mul_kara_u128_256_t);

fn main() {
    let mut c = micro_criterion().configure_from_args();
    tb_64(&mut c);
    tb_96(&mut c);
    tb_128(&mut c);
    tb_256(&mut c);
    c.final_summary();
}
