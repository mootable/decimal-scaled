// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! N-way map bench for the integer multiply policy (step-2).
//!
//! Races every fixed-array candidate the int-mul policy could route, per
//! width, on the SAME operands: schoolbook (u64 / u128 limbs), Karatsuba
//! (u64 / u128), Toom-Cook-3 (u64 / u128). Widths span the wide storage tiers
//! (D462..D1232 = 24..64 limbs) AND the transcendental work widths
//! (96/128/192/256); the policy treats the 256 winner as the open-ended arm
//! for all N >= 256 (owner directive 2026-05-30). Bit-identicality to the
//! slice schoolbook oracle is asserted before timing.
//!
//! NOTE: the Karatsuba / Toom-3 arms recurse at threshold = N (a single split
//! then schoolbook on the halves) -- a LOWER bound on the recursive benefit.
//! If a recursive arm is competitive at the wide widths, the threshold is the
//! tunable axis to sweep next.
//!
//! Run pinned on cores 16-19:
//!   powershell.exe -NoProfile -File scripts/pin_run.ps1 \
//!     -Mask 0xF0000 -Bench mul_toom3_ab \
//!     -Features "wide x-wide xx-wide bench-alt"

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    mul_slice,
    mul_full_u64_24, mul_full_u128_24, mul_kara_u64_24, mul_kara_u128_24, mul_toom3_u64_24, mul_toom3_u128_24,
    mul_full_u64_32, mul_full_u128_32, mul_kara_u64_32, mul_kara_u128_32, mul_toom3_u64_32, mul_toom3_u128_32,
    mul_full_u64_48, mul_full_u128_48, mul_kara_u64_48, mul_kara_u128_48, mul_toom3_u64_48, mul_toom3_u128_48,
    mul_full_u64_64, mul_full_u128_64, mul_kara_u64_64, mul_kara_u128_64, mul_toom3_u64_64, mul_toom3_u128_64,
    mul_full_u64_96, mul_full_u128_96, mul_kara_u64_96, mul_kara_u128_96, mul_toom3_u64_96, mul_toom3_u128_96,
    mul_full_u64_128, mul_full_u128_128, mul_kara_u64_128, mul_kara_u128_128, mul_toom3_u64_128, mul_toom3_u128_128,
    mul_full_u64_192, mul_full_u128_192, mul_kara_u64_192, mul_kara_u128_192, mul_toom3_u64_192, mul_toom3_u128_192,
    mul_full_u64_256, mul_full_u128_256, mul_kara_u64_256, mul_kara_u128_256, mul_toom3_u64_256, mul_toom3_u128_256,
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

/// Generate one width's N-way race over the six fixed-array candidates.
macro_rules! width_bench {
    ($fn:ident, $group:expr, $N:literal, $D:literal,
     $u64:ident, $u128:ident, $ku64:ident, $ku128:ident, $tu64:ident, $tu128:ident) => {
        fn $fn(c: &mut Criterion) {
            const N: usize = $N;
            // validity wall: every candidate bit-identical to the oracle
            for ops in operands::<N>() {
                let oc = oracle::<N>(&ops);
                let mut r = vec![0u64; $D];
                $u64(&ops.a, &ops.b, &mut r);   assert_eq!(r, oc, "u64 != oracle {} ({})", $N, ops.label);
                let mut r = vec![0u64; $D]; $u128(&ops.a, &ops.b, &mut r);  assert_eq!(r, oc, "u128 != oracle {} ({})", $N, ops.label);
                let mut r = vec![0u64; $D]; $ku64(&ops.a, &ops.b, &mut r);  assert_eq!(r, oc, "kara_u64 != oracle {} ({})", $N, ops.label);
                let mut r = vec![0u64; $D]; $ku128(&ops.a, &ops.b, &mut r); assert_eq!(r, oc, "kara_u128 != oracle {} ({})", $N, ops.label);
                let mut r = vec![0u64; $D]; $tu64(&ops.a, &ops.b, &mut r);  assert_eq!(r, oc, "toom3_u64 != oracle {} ({})", $N, ops.label);
                let mut r = vec![0u64; $D]; $tu128(&ops.a, &ops.b, &mut r); assert_eq!(r, oc, "toom3_u128 != oracle {} ({})", $N, ops.label);
            }
            fn run_u64(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $u64(&ops.a, &ops.b, &mut o); o }
            fn run_u128(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $u128(&ops.a, &ops.b, &mut o); o }
            fn run_ku64(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $ku64(&ops.a, &ops.b, &mut o); o }
            fn run_ku128(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $ku128(&ops.a, &ops.b, &mut o); o }
            fn run_tu64(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $tu64(&ops.a, &ops.b, &mut o); o }
            fn run_tu128(ops: Operands<$N>) -> Vec<u64> { let mut o = vec![0u64; $D]; $tu128(&ops.a, &ops.b, &mut o); o }
            compare_all(c, $group, |ops: &Operands<N>| ops.label.to_string(), operands::<N>(), vec![
                ("u64",        run_u64   as fn(Operands<N>) -> Vec<u64>),
                ("u128",       run_u128  as fn(Operands<N>) -> Vec<u64>),
                ("kara_u64",   run_ku64  as fn(Operands<N>) -> Vec<u64>),
                ("kara_u128",  run_ku128 as fn(Operands<N>) -> Vec<u64>),
                ("toom3_u64",  run_tu64  as fn(Operands<N>) -> Vec<u64>),
                ("toom3_u128", run_tu128 as fn(Operands<N>) -> Vec<u64>),
            ]);
        }
    };
}

width_bench!(bw_24,  "mul/Int1536_n24",  24,  48,  mul_full_u64_24,  mul_full_u128_24,  mul_kara_u64_24,  mul_kara_u128_24,  mul_toom3_u64_24,  mul_toom3_u128_24);
width_bench!(bw_32,  "mul/Int2048_n32",  32,  64,  mul_full_u64_32,  mul_full_u128_32,  mul_kara_u64_32,  mul_kara_u128_32,  mul_toom3_u64_32,  mul_toom3_u128_32);
width_bench!(bw_48,  "mul/Int3072_n48",  48,  96,  mul_full_u64_48,  mul_full_u128_48,  mul_kara_u64_48,  mul_kara_u128_48,  mul_toom3_u64_48,  mul_toom3_u128_48);
width_bench!(bw_64,  "mul/Int4096_n64",  64,  128, mul_full_u64_64,  mul_full_u128_64,  mul_kara_u64_64,  mul_kara_u128_64,  mul_toom3_u64_64,  mul_toom3_u128_64);
width_bench!(bw_96,  "mul/Int6144_n96",  96,  192, mul_full_u64_96,  mul_full_u128_96,  mul_kara_u64_96,  mul_kara_u128_96,  mul_toom3_u64_96,  mul_toom3_u128_96);
width_bench!(bw_128, "mul/Int8192_n128", 128, 256, mul_full_u64_128, mul_full_u128_128, mul_kara_u64_128, mul_kara_u128_128, mul_toom3_u64_128, mul_toom3_u128_128);
width_bench!(bw_192, "mul/Int12288_n192",192, 384, mul_full_u64_192, mul_full_u128_192, mul_kara_u64_192, mul_kara_u128_192, mul_toom3_u64_192, mul_toom3_u128_192);
width_bench!(bw_256, "mul/Int16384_n256",256, 512, mul_full_u64_256, mul_full_u128_256, mul_kara_u64_256, mul_kara_u128_256, mul_toom3_u64_256, mul_toom3_u128_256);

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bw_24(&mut c);
    bw_32(&mut c);
    bw_48(&mut c);
    bw_64(&mut c);
    bw_96(&mut c);
    bw_128(&mut c);
    bw_192(&mut c);
    bw_256(&mut c);
    c.final_summary();
}
