// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B microbench for the root families: native tight-`Int<W>`
//! f64-seeded Newton vs the generic build-max slice path, at the
//! bench-branch-compare cells (sqrt/cbrt @ D57/D76/D115/D153/D230/D307).
//!
//! Each candidate is asserted bit-identical across the operand spread and
//! all six rounding modes before timing (correctness gate), then the
//! harness `black_box`-guards inputs/outputs so nothing const-folds.

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    cbrt_native_d57s20, cbrt_native_fast_d57s20, cbrt_native_w, cbrt_newton_slice,
    cbrt_newton_slice_n,
    cbrt_table_seed_d57s20, int_from_mag_limbs, sqrt_mg, sqrt_native_w, sqrt_newton_slice,
    sqrt_newton_slice_n,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;
const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Build an `Int<N>` from a non-negative magnitude (low two u64 limbs).
fn fromu<const N: usize>(v: u128) -> Int<N> {
    let mut mag = [0u64; N];
    mag[0] = v as u64;
    if N > 1 {
        mag[1] = (v >> 64) as u64;
    }
    int_from_mag_limbs::<N>(&mag)
}

// ── sqrt@D38 (Int<2>, SCALE=18) ──────────────────────────────────────
#[derive(Clone)]
struct One {
    label: &'static str,
    raw: Int<2>,
}

fn sqrt_inputs() -> Vec<One> {
    // D38<19>: value 2.0 -> raw 2e19; radicand 2e19*10^19 = 2e38 < u128::MAX,
    // so the u128::isqrt fast path fires (the live-bench hot path). Plus a
    // value that overflows u128 -> 256-bit fallback.
    vec![
        One { label: "v2.0", raw: fromu::<2>(20_000_000_000_000_000_000) },
        One { label: "v1.5", raw: fromu::<2>(15_000_000_000_000_000_000) },
        One { label: "large", raw: fromu::<2>((1u128 << 120) | 0xABCD) },
    ]
}

fn bench_sqrt(c: &mut Criterion) {
    const S: u32 = 19;
    let mg = |o: One| sqrt_mg::<S>(o.raw, MODE);
    let slc = |o: One| sqrt_newton_slice::<S>(o.raw, MODE);
    for o in sqrt_inputs() {
        for m in ALL_MODES {
            assert_eq!(sqrt_mg::<S>(o.raw, m), sqrt_newton_slice::<S>(o.raw, m), "sqrt {} mode {m:?}", o.label);
        }
    }
    compare_all(
        c,
        "sqrt_d38_s19",
        |o: &One| o.label.to_string(),
        sqrt_inputs(),
        vec![("mg_divide", Box::new(mg) as Box<dyn Fn(One) -> Int<2>>), ("slice", Box::new(slc))],
    );
}

// ── cbrt@D57<20> (Int<3>) ────────────────────────────────────────────
#[derive(Clone)]
struct OneD57 {
    label: &'static str,
    raw: Int<3>,
}

fn cbrt_inputs() -> Vec<OneD57> {
    // Raw storages at D57<20>: value 1.5 -> 1.5e20; spread of magnitudes
    // whose raw exceeds u128 (so the native arm's Int<6> width matters).
    vec![
        OneD57 { label: "v1.5", raw: fromu::<3>(150_000_000_000_000_000_000) },
        OneD57 { label: "v_mid", raw: fromu::<3>((1u128 << 90) | 0xBEEF) },
        OneD57 { label: "v_large", raw: fromu::<3>((1u128 << 126) | 0x1357) },
    ]
}

fn bench_cbrt(c: &mut Criterion) {
    const S: u32 = 20;
    let nat = |o: OneD57| cbrt_native_d57s20(o.raw, MODE);
    let tbl = |o: OneD57| cbrt_table_seed_d57s20(o.raw, MODE);
    let slc = |o: OneD57| cbrt_newton_slice::<S>(o.raw, MODE);
    let fst = |o: OneD57| cbrt_native_fast_d57s20(o.raw, MODE);
    for o in cbrt_inputs() {
        for m in ALL_MODES {
            let r_nat = cbrt_native_d57s20(o.raw, m);
            let r_tbl = cbrt_table_seed_d57s20(o.raw, m);
            let r_slc = cbrt_newton_slice::<S>(o.raw, m);
            let r_fst = cbrt_native_fast_d57s20(o.raw, m);
            assert_eq!(r_nat, r_slc, "cbrt native vs slice {} mode {m:?}", o.label);
            assert_eq!(r_tbl, r_slc, "cbrt table vs slice {} mode {m:?}", o.label);
            assert_eq!(r_fst, r_slc, "cbrt fast vs slice {} mode {m:?}", o.label);
        }
    }
    compare_all(
        c,
        "cbrt_d57_s20",
        |o: &OneD57| o.label.to_string(),
        cbrt_inputs(),
        vec![
            ("native", Box::new(nat) as Box<dyn Fn(OneD57) -> Int<3>>),
            ("table_seed", Box::new(tbl)),
            ("slice", Box::new(slc)),
            ("fast", Box::new(fst)),
        ],
    );
}


// ── wide-tier roots: native tight-Int<W> f64 Newton vs generic slice ──
// Benched at each tier's mid-scale (the lib_cmp root cell). One concrete
// fn per (fn, N, W, SCALE) cell so the WorkScratch bound is discharged at
// the concrete `N` (the trait is crate-internal -- a generic helper would
// have to name it). The macro keeps the bodies single-source.
#[derive(Clone)]
struct WideOne<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

fn wide_inputs<const N: usize>() -> Vec<WideOne<N>> {
    vec![
        WideOne { label: "v_small", raw: fromu::<N>((1u128 << 40) | 0xABCD) },
        WideOne { label: "v_mid", raw: fromu::<N>((1u128 << 90) | 0xBEEF) },
        WideOne { label: "v_large", raw: fromu::<N>((1u128 << 126) | 0x1357) },
    ]
}

macro_rules! wide_root_bench {
    ($fnname:ident, $nat:ident, $slc:ident, $n:literal, $w:literal, $s:literal, $group:literal) => {
        fn $fnname(c: &mut Criterion) {
            let nat = |o: WideOne<$n>| $nat::<$n, $w, $s>(o.raw, MODE);
            let slc = |o: WideOne<$n>| $slc::<$n, $s>(o.raw, MODE);
            for o in wide_inputs::<$n>() {
                for m in ALL_MODES {
                    assert_eq!(
                        $nat::<$n, $w, $s>(o.raw, m),
                        $slc::<$n, $s>(o.raw, m),
                        concat!($group, " {} mode {:?}"),
                        o.label, m
                    );
                }
            }
            compare_all(
                c,
                $group,
                |o: &WideOne<$n>| o.label.to_string(),
                wide_inputs::<$n>(),
                vec![
                    ("native", Box::new(nat) as Box<dyn Fn(WideOne<$n>) -> Int<$n>>),
                    ("slice", Box::new(slc)),
                ],
            );
        }
    };
}

// Native (tight-Int<W> f64-seeded Newton) vs the generic build-max slice,
// benched at the bench-branch-compare scales (D57/D76 @ 20, D115/D153 @ 25,
// D230/D307 @ 30) and at the policy's ROUTED work width — sqrt W = 2N, cbrt
// W = 3N (sized to cover the tier's full scale range), the width the live
// decimal op actually pays. D57/D76 are routed to Native (it wins at the
// full-range W there); D115+ are NOT routed by N — at the full-range W the
// per-iteration Knuth divide outweighs the slice scratch churn (these cells
// document that crossover) and they keep only their high-scale Native cells.
wide_root_bench!(bench_sqrt_d57, sqrt_native_w, sqrt_newton_slice_n, 3, 6, 20, "sqrt_d57_s20");
wide_root_bench!(bench_sqrt_d76, sqrt_native_w, sqrt_newton_slice_n, 4, 8, 20, "sqrt_d76_s20");
wide_root_bench!(bench_sqrt_d115, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 25, "sqrt_d115_s25");
wide_root_bench!(bench_sqrt_d153, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 25, "sqrt_d153_s25");
wide_root_bench!(bench_sqrt_d230, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 30, "sqrt_d230_s30");
wide_root_bench!(bench_sqrt_d307, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 30, "sqrt_d307_s30");
wide_root_bench!(bench_cbrt_d57, cbrt_native_w, cbrt_newton_slice_n, 3, 9, 20, "cbrt_d57w9_s20");
wide_root_bench!(bench_cbrt_d76, cbrt_native_w, cbrt_newton_slice_n, 4, 12, 20, "cbrt_d76_s20");
wide_root_bench!(bench_cbrt_d115, cbrt_native_w, cbrt_newton_slice_n, 6, 18, 25, "cbrt_d115_s25");
wide_root_bench!(bench_cbrt_d153, cbrt_native_w, cbrt_newton_slice_n, 8, 24, 25, "cbrt_d153_s25");
wide_root_bench!(bench_cbrt_d230, cbrt_native_w, cbrt_newton_slice_n, 12, 36, 30, "cbrt_d230_s30");
wide_root_bench!(bench_cbrt_d307, cbrt_native_w, cbrt_newton_slice_n, 16, 48, 30, "cbrt_d307_s30");

fn bench_wide(c: &mut Criterion) {
    bench_sqrt_d57(c);
    bench_sqrt_d76(c);
    bench_sqrt_d115(c);
    bench_sqrt_d153(c);
    bench_sqrt_d230(c);
    bench_sqrt_d307(c);
    bench_cbrt_d57(c);
    bench_cbrt_d76(c);
    bench_cbrt_d115(c);
    bench_cbrt_d153(c);
    bench_cbrt_d230(c);
    bench_cbrt_d307(c);
}


fn benches(c: &mut Criterion) {
    bench_sqrt(c);
    bench_cbrt(c);
    bench_wide(c);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
