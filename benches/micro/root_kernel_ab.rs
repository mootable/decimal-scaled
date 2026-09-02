// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B microbench for the root families: native tight-`Int<W>`
//! f64-seeded Newton vs the generic build-max slice path, at the
//! bench-branch-compare cells (sqrt/cbrt @ D57/D76/D115/D153/D230/D307).
//!
//! Each candidate is asserted bit-identical across the operand spread and
//! all eight rounding modes before timing (correctness gate), then the
//! harness `black_box`-guards inputs/outputs so nothing const-folds.

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    cbrt_native_d57s20, cbrt_native_fast_a_w, cbrt_native_fast_b_w, cbrt_native_fast_3limb_s20,
    cbrt_mg_n, cbrt_native_w, cbrt_newton_slice, cbrt_newton_slice_n, cbrt_table_seed_d57s20,
    int_from_mag_limbs, sqrt_mg, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice, sqrt_newton_slice_n,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;
const ALL_MODES: [RoundingMode; 8] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
    RoundingMode::AwayFromZero,
    RoundingMode::ZeroFiveUp,
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
    let fst = |o: OneD57| cbrt_native_fast_3limb_s20(o.raw, MODE);
    for o in cbrt_inputs() {
        for m in ALL_MODES {
            let r_nat = cbrt_native_d57s20(o.raw, m);
            let r_tbl = cbrt_table_seed_d57s20(o.raw, m);
            let r_slc = cbrt_newton_slice::<S>(o.raw, m);
            let r_fst = cbrt_native_fast_3limb_s20(o.raw, m);
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


// ── NARROW-tier roots (N == 2): the arms `select` never raced ─────────
// `policy::{sqrt,cbrt}::select` routes `(1, _)` and `(2, _)` to
// `Algorithm::MgDivide` unconditionally. The pre-existing `sqrt_d38_s19`
// group above races MgDivide only against the generic `slice` — the
// tight-`Int<W>` f64-seeded `Native` arm the wide tiers use was never
// entered as a candidate here. These groups add it, at the policy's own
// full-range work width (sqrt `W = 2N = 4`, cbrt `W = 3N = 6`), on the
// bench-branch-compare cells where the inversion is worst (D38<28>) and
// on a cell below the `10^scale > u64::MAX` boundary (D38<19>) to check
// whether MgDivide should be KEPT there.
// The two INT-LAYER candidates are `native` (Newton run in a tight concrete
// `Int<W>`, whose `radicand / x` dispatches through `int::policy::div_rem`)
// and `slice` (the width-agnostic int `isqrt`/`icbrt`). `mg_divide` is the
// only candidate that BYPASSES the int layer — it reimplements division
// bit-serially over `u128` pairs in `algos::support::mg_divide`. So the
// question each group answers is not "which is fastest" but "does the bypass
// beat the better int-layer path by a margin that earns a separate kernel".
#[derive(Clone)]
struct NarrowOne<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

/// bbc-shaped operands (value 2.0 / 1.5 at the cell's scale) plus a
/// near-full-magnitude raw — the value class where MgDivide's `u128` fast
/// path can no longer fire. `big` is per-`N` because `Int<1>` tops out at
/// `i64::MAX`.
fn narrow_inputs<const N: usize>(scale: u32, big: u128) -> Vec<NarrowOne<N>> {
    let p = 10u128.pow(scale);
    vec![
        NarrowOne { label: "v2.0", raw: fromu::<N>(2 * p) },
        NarrowOne { label: "v1.5", raw: fromu::<N>(3 * p / 2) },
        NarrowOne { label: "v_big", raw: fromu::<N>(big) },
    ]
}

const BIG_N1: u128 = 9_000_000_000_000_000_000; // just under i64::MAX
const BIG_N2: u128 = 170_000_000_000_000_000_000_000_000_000_000_000_000; // ~i128::MAX

macro_rules! narrow_root_bench {
    ($fnname:ident, $mg:ident, $nat:ident, $slc:ident,
     $n:literal, $w:literal, $s:literal, $big:expr, $group:literal) => {
        fn $fnname(c: &mut Criterion) {
            let mg = |o: NarrowOne<$n>| $mg::<$n, $s>(o.raw, MODE);
            let nat = |o: NarrowOne<$n>| $nat::<$n, $w, $s>(o.raw, MODE);
            let slc = |o: NarrowOne<$n>| $slc::<$n, $s>(o.raw, MODE);
            // Validity wall: every candidate must be bit-identical to the
            // slice reference across all eight modes before it is timed.
            for o in narrow_inputs::<$n>($s, $big) {
                for m in ALL_MODES {
                    let r_ref = $slc::<$n, $s>(o.raw, m);
                    assert_eq!($mg::<$n, $s>(o.raw, m), r_ref,
                        concat!($group, " mg vs slice {} mode {:?}"), o.label, m);
                    assert_eq!($nat::<$n, $w, $s>(o.raw, m), r_ref,
                        concat!($group, " native vs slice {} mode {:?}"), o.label, m);
                }
            }
            compare_all(
                c,
                $group,
                |o: &NarrowOne<$n>| o.label.to_string(),
                narrow_inputs::<$n>($s, $big),
                vec![
                    ("mg_divide", Box::new(mg) as Box<dyn Fn(NarrowOne<$n>) -> Int<$n>>),
                    ("native", Box::new(nat)),
                    ("slice", Box::new(slc)),
                ],
            );
        }
    };
}

// ── N == 1 (D18): the cells routed on mechanism-equivalence, now measured ──
// D18's bbc scale grid is {0, 4, 9, 13, 17}. sqrt W = 2N = 2, cbrt W = 3N = 3.
narrow_root_bench!(bench_cbrt_d18_s0, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 0, BIG_N1, "cbrt_d18_s00");
narrow_root_bench!(bench_cbrt_d18_s4, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 4, BIG_N1, "cbrt_d18_s04");
narrow_root_bench!(bench_cbrt_d18_s9, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 9, BIG_N1, "cbrt_d18_s09");
narrow_root_bench!(bench_cbrt_d18_s13, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 13, BIG_N1, "cbrt_d18_s13");
narrow_root_bench!(bench_cbrt_d18_s17, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 17, BIG_N1, "cbrt_d18_s17");
// Crossover bisection for the cbrt slice→native boundary, at BOTH narrow
// widths: bracketed above by s4 (native 1.45-1.93x) and below by s0/s2
// (tie or slice). These pin the exact first scale native wins.
narrow_root_bench!(bench_cbrt_d18_s1, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 1, BIG_N1, "cbrt_d18_s01");
narrow_root_bench!(bench_cbrt_d18_s2, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 2, BIG_N1, "cbrt_d18_s02");
narrow_root_bench!(bench_cbrt_d18_s3, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 1, 3, 3, BIG_N1, "cbrt_d18_s03");
narrow_root_bench!(bench_cbrt_d38_s3, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 3, BIG_N2, "cbrt_d38_s03");
narrow_root_bench!(bench_sqrt_d18_s0, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 1, 2, 0, BIG_N1, "sqrt_d18_s00");
narrow_root_bench!(bench_sqrt_d18_s4, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 1, 2, 4, BIG_N1, "sqrt_d18_s04");
narrow_root_bench!(bench_sqrt_d18_s9, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 1, 2, 9, BIG_N1, "sqrt_d18_s09");
narrow_root_bench!(bench_sqrt_d18_s13, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 1, 2, 13, BIG_N1, "sqrt_d18_s13");
narrow_root_bench!(bench_sqrt_d18_s17, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 1, 2, 17, BIG_N1, "sqrt_d18_s17");

// Bisection cells: the slice→native crossover sits in (0, 9] for cbrt and
// the mg_divide→native crossover in (0, 9] for sqrt. These localize both so
// the policy arm can be placed at the TRUE crossover covering a continuous
// win-region, not snapped to a sampled bbc point.
narrow_root_bench!(bench_cbrt_d38_s2, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 2, BIG_N2, "cbrt_d38_s02");
narrow_root_bench!(bench_cbrt_d38_s4, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 4, BIG_N2, "cbrt_d38_s04");
narrow_root_bench!(bench_cbrt_d38_s6, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 6, BIG_N2, "cbrt_d38_s06");
narrow_root_bench!(bench_sqrt_d38_s2, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 2, 4, 2, BIG_N2, "sqrt_d38_s02");
narrow_root_bench!(bench_sqrt_d38_s4, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 2, 4, 4, BIG_N2, "sqrt_d38_s04");
narrow_root_bench!(bench_sqrt_d38_s6, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 2, 4, 6, BIG_N2, "sqrt_d38_s06");
narrow_root_bench!(bench_cbrt_d38_s9, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 9, BIG_N2, "cbrt_d38_s09");
narrow_root_bench!(bench_cbrt_d38_s14, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 14, BIG_N2, "cbrt_d38_s14");
narrow_root_bench!(bench_cbrt_d38_s19, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 19, BIG_N2, "cbrt_d38_s19");
narrow_root_bench!(bench_sqrt_d38_s0, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 2, 4, 0, BIG_N2, "sqrt_d38_s00");
narrow_root_bench!(bench_sqrt_d38_s9, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 2, 4, 9, BIG_N2, "sqrt_d38_s09");
narrow_root_bench!(bench_cbrt_d38_s28, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 28, BIG_N2, "cbrt_d38_s28");
narrow_root_bench!(bench_cbrt_d38_s0, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 0, BIG_N2, "cbrt_d38_s0");
narrow_root_bench!(bench_cbrt_d38_s37, cbrt_mg_n, cbrt_native_fast_a_w, cbrt_newton_slice_n, 2, 6, 37, BIG_N2, "cbrt_d38_s37");
narrow_root_bench!(bench_sqrt_d38_s28, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 2, 4, 28, BIG_N2, "sqrt_d38_s28");
narrow_root_bench!(bench_sqrt_d38_s19n, sqrt_mg_n, sqrt_native_w, sqrt_newton_slice_n, 2, 4, 19, BIG_N2, "sqrt_d38_s19n");

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

// ── MAX-SCALE (S-1) cells: the bench-branch-compare regressions ──────────
// These are the cells the bbc flagged (sqrt/cbrt at SCALE = name-1, where the
// radicand `mag·10^(S-1)` / `mag·10^(2(S-1))` is genuinely wide, so the tight
// `Int<W=2N/3N>` native arm should win decisively over the generic slice — the
// slice pays a linear-`scale`-length ×10 radicand-build loop + build-max isqrt
// scratch). Work widths: sqrt W=2N, cbrt W=3N (cover the full scale range).
wide_root_bench!(bench_sqrt_d115_max, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 114, "sqrt_d115_s114");
wide_root_bench!(bench_sqrt_d153_max, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 152, "sqrt_d153_s152");
wide_root_bench!(bench_sqrt_d230_max, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 229, "sqrt_d230_s229");
wide_root_bench!(bench_sqrt_d307_max, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 306, "sqrt_d307_s306");
wide_root_bench!(bench_sqrt_d462_max, sqrt_native_w, sqrt_newton_slice_n, 24, 48, 461, "sqrt_d462_s461");
wide_root_bench!(bench_sqrt_d616_max, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 615, "sqrt_d616_s615");
wide_root_bench!(bench_sqrt_d924_max, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 923, "sqrt_d924_s923");
wide_root_bench!(bench_sqrt_d1232_max, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 1231, "sqrt_d1232_s1231");
wide_root_bench!(bench_cbrt_d115_max, cbrt_native_w, cbrt_newton_slice_n, 6, 18, 114, "cbrt_d115_s114");
wide_root_bench!(bench_cbrt_d153_max, cbrt_native_w, cbrt_newton_slice_n, 8, 24, 152, "cbrt_d153_s152");
wide_root_bench!(bench_cbrt_d230_max, cbrt_native_w, cbrt_newton_slice_n, 12, 36, 229, "cbrt_d230_s229");
wide_root_bench!(bench_cbrt_d307_max, cbrt_native_w, cbrt_newton_slice_n, 16, 48, 306, "cbrt_d307_s306");
wide_root_bench!(bench_cbrt_d462_max, cbrt_native_w, cbrt_newton_slice_n, 24, 72, 461, "cbrt_d462_s461");
wide_root_bench!(bench_cbrt_d616_max, cbrt_native_w, cbrt_newton_slice_n, 32, 96, 615, "cbrt_d616_s615");
wide_root_bench!(bench_cbrt_d924_max, cbrt_native_w, cbrt_newton_slice_n, 48, 144, 923, "cbrt_d924_s923");
wide_root_bench!(bench_cbrt_d1232_max, cbrt_native_w, cbrt_newton_slice_n, 64, 192, 1231, "cbrt_d1232_s1231");

// ── crossover bisection: where does native overtake the slice by scale? ──
// At full-range W (2N sqrt / 3N cbrt) native loses at low/mid scale (full-W
// Knuth divides on a small radicand) but wins at high scale (slice's linear
// ×10 build + build-max scratch dominate). Bisect the crossover per tier.
// D307 (N=16, sqrt W=32, cbrt W=48): scales 76 / 150 / 230.
wide_root_bench!(bench_sqrt_d307_s76, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 76, "sqrt_d307_s76");
wide_root_bench!(bench_sqrt_d307_s150b, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 150, "sqrt_d307_s150b");
wide_root_bench!(bench_sqrt_d307_s230, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 230, "sqrt_d307_s230");
wide_root_bench!(bench_cbrt_d307_s76, cbrt_native_w, cbrt_newton_slice_n, 16, 48, 76, "cbrt_d307_s76");
wide_root_bench!(bench_cbrt_d307_s150b, cbrt_native_w, cbrt_newton_slice_n, 16, 48, 150, "cbrt_d307_s150b");
wide_root_bench!(bench_cbrt_d307_s230, cbrt_native_w, cbrt_newton_slice_n, 16, 48, 230, "cbrt_d307_s230");
// D115 (N=6, sqrt W=12, cbrt W=18): scales 28 / 57 / 85.
wide_root_bench!(bench_sqrt_d115_s28, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 28, "sqrt_d115_s28");
wide_root_bench!(bench_sqrt_d115_s57b, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 57, "sqrt_d115_s57b");
wide_root_bench!(bench_sqrt_d115_s85, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 85, "sqrt_d115_s85");
wide_root_bench!(bench_cbrt_d115_s28, cbrt_native_w, cbrt_newton_slice_n, 6, 18, 28, "cbrt_d115_s28");
wide_root_bench!(bench_cbrt_d115_s57b, cbrt_native_w, cbrt_newton_slice_n, 6, 18, 57, "cbrt_d115_s57b");
wide_root_bench!(bench_cbrt_d115_s85, cbrt_native_w, cbrt_newton_slice_n, 6, 18, 85, "cbrt_d115_s85");
// D153 (N=8, sqrt W=16, cbrt W=24): scales 38 / 76 / 114.
wide_root_bench!(bench_sqrt_d153_s38, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 38, "sqrt_d153_s38");
wide_root_bench!(bench_sqrt_d153_s76, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 76, "sqrt_d153_s76");
wide_root_bench!(bench_sqrt_d153_s114, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 114, "sqrt_d153_s114");
wide_root_bench!(bench_cbrt_d153_s38, cbrt_native_w, cbrt_newton_slice_n, 8, 24, 38, "cbrt_d153_s38");
wide_root_bench!(bench_cbrt_d153_s76, cbrt_native_w, cbrt_newton_slice_n, 8, 24, 76, "cbrt_d153_s76");
wide_root_bench!(bench_cbrt_d153_s114, cbrt_native_w, cbrt_newton_slice_n, 8, 24, 114, "cbrt_d153_s114");
// D230 (N=12, sqrt W=24, cbrt W=36): scales 57 / 114 / 172.
wide_root_bench!(bench_sqrt_d230_s57, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 57, "sqrt_d230_s57");
wide_root_bench!(bench_sqrt_d230_s114, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 114, "sqrt_d230_s114");
wide_root_bench!(bench_sqrt_d230_s172, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 172, "sqrt_d230_s172");
// D230 sqrt threshold-tightening bisection (current threshold s>=48 was
// based on conservative s>=4N heuristic; at s=57 slice still wins 1.09x —
// crossover is above 57).
wide_root_bench!(bench_sqrt_d230_s72, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 72, "sqrt_d230_s72");
wide_root_bench!(bench_sqrt_d230_s85, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 85, "sqrt_d230_s85");
wide_root_bench!(bench_sqrt_d230_s100, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 100, "sqrt_d230_s100");
// D307 sqrt threshold-tightening bisection (current threshold s>=64).
// At s=30 slice wins 3.74x, at s=76 native wins 1.05x. Crossover between.
wide_root_bench!(bench_sqrt_d307_s48, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 48, "sqrt_d307_s48");
wide_root_bench!(bench_sqrt_d307_s56, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 56, "sqrt_d307_s56");
wide_root_bench!(bench_sqrt_d307_s64, sqrt_native_w, sqrt_newton_slice_n, 16, 32, 64, "sqrt_d307_s64");
// D153 sqrt threshold-tightening bisection (current threshold s>=32).
// At s=25 slice wins 1.21x, at s=38 native wins 1.07x. Crossover between.
wide_root_bench!(bench_sqrt_d153_s28, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 28, "sqrt_d153_s28");
wide_root_bench!(bench_sqrt_d153_s32, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 32, "sqrt_d153_s32");
wide_root_bench!(bench_sqrt_d153_s35, sqrt_native_w, sqrt_newton_slice_n, 8, 16, 35, "sqrt_d153_s35");
// D115 sqrt threshold-tightening bisection (current threshold s>=24).
// At s=25 tied, at s=28 native wins 1.06x. Want lowest valid threshold.
wide_root_bench!(bench_sqrt_d115_s20, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 20, "sqrt_d115_s20");
wide_root_bench!(bench_sqrt_d115_s22, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 22, "sqrt_d115_s22");
wide_root_bench!(bench_sqrt_d115_s24, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 24, "sqrt_d115_s24");
// D462 sqrt crossover sweep (currently threshold s>=96 = 4N).
wide_root_bench!(bench_sqrt_d462_s24, sqrt_native_w, sqrt_newton_slice_n, 24, 48, 24, "sqrt_d462_s24");
wide_root_bench!(bench_sqrt_d462_s48, sqrt_native_w, sqrt_newton_slice_n, 24, 48, 48, "sqrt_d462_s48");
wide_root_bench!(bench_sqrt_d462_s72, sqrt_native_w, sqrt_newton_slice_n, 24, 48, 72, "sqrt_d462_s72");
wide_root_bench!(bench_sqrt_d462_s96, sqrt_native_w, sqrt_newton_slice_n, 24, 48, 96, "sqrt_d462_s96");
wide_root_bench!(bench_sqrt_d462_s120, sqrt_native_w, sqrt_newton_slice_n, 24, 48, 120, "sqrt_d462_s120");
wide_root_bench!(bench_sqrt_d462_s230, sqrt_native_w, sqrt_newton_slice_n, 24, 48, 230, "sqrt_d462_s230");
// D616 sqrt crossover sweep (currently threshold s>=128 = 4N).
wide_root_bench!(bench_sqrt_d616_s32, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 32, "sqrt_d616_s32");
wide_root_bench!(bench_sqrt_d616_s64, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 64, "sqrt_d616_s64");
wide_root_bench!(bench_sqrt_d616_s96, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 96, "sqrt_d616_s96");
wide_root_bench!(bench_sqrt_d616_s128, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 128, "sqrt_d616_s128");
wide_root_bench!(bench_sqrt_d616_s307, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 307, "sqrt_d616_s307");
// D924 sqrt crossover sweep (currently threshold s>=192 = 4N).
wide_root_bench!(bench_sqrt_d924_s96, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 96, "sqrt_d924_s96");
wide_root_bench!(bench_sqrt_d924_s144, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 144, "sqrt_d924_s144");
wide_root_bench!(bench_sqrt_d924_s192, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 192, "sqrt_d924_s192");
wide_root_bench!(bench_sqrt_d924_s462, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 462, "sqrt_d924_s462");
// D1232 sqrt crossover sweep (currently threshold s>=256 = 4N).
wide_root_bench!(bench_sqrt_d1232_s128, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 128, "sqrt_d1232_s128");
wide_root_bench!(bench_sqrt_d1232_s192, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 192, "sqrt_d1232_s192");
wide_root_bench!(bench_sqrt_d1232_s256, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 256, "sqrt_d1232_s256");
wide_root_bench!(bench_sqrt_d1232_s615, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 615, "sqrt_d1232_s615");
// D616 sqrt second-pass bisection (s=128 slice 1.20x, s=307 native 1.64x).
wide_root_bench!(bench_sqrt_d616_s160, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 160, "sqrt_d616_s160");
wide_root_bench!(bench_sqrt_d616_s192, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 192, "sqrt_d616_s192");
wide_root_bench!(bench_sqrt_d616_s224, sqrt_native_w, sqrt_newton_slice_n, 32, 64, 224, "sqrt_d616_s224");
// D924 sqrt second-pass bisection (s=192 tied 1.03x, s=462 native 2.19x).
wide_root_bench!(bench_sqrt_d924_s220, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 220, "sqrt_d924_s220");
wide_root_bench!(bench_sqrt_d924_s260, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 260, "sqrt_d924_s260");
wide_root_bench!(bench_sqrt_d924_s320, sqrt_native_w, sqrt_newton_slice_n, 48, 96, 320, "sqrt_d924_s320");
// D1232 sqrt second-pass bisection (s=256 tied 1.01x, s=615 native 2.60x).
wide_root_bench!(bench_sqrt_d1232_s300, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 300, "sqrt_d1232_s300");
wide_root_bench!(bench_sqrt_d1232_s400, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 400, "sqrt_d1232_s400");
wide_root_bench!(bench_sqrt_d1232_s500, sqrt_native_w, sqrt_newton_slice_n, 64, 128, 500, "sqrt_d1232_s500");
// D230 sqrt second-pass bisection (s=57 slice 1.07x, s=72 native 1.05x — tight).
wide_root_bench!(bench_sqrt_d230_s62, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 62, "sqrt_d230_s62");
wide_root_bench!(bench_sqrt_d230_s66, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 66, "sqrt_d230_s66");
wide_root_bench!(bench_sqrt_d230_s70, sqrt_native_w, sqrt_newton_slice_n, 12, 24, 70, "sqrt_d230_s70");
// D115 sqrt second-pass — confirm s=22 is the right gate (vs s=21).
wide_root_bench!(bench_sqrt_d115_s21, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 21, "sqrt_d115_s21");
wide_root_bench!(bench_sqrt_d115_s23, sqrt_native_w, sqrt_newton_slice_n, 6, 12, 23, "sqrt_d115_s23");
wide_root_bench!(bench_cbrt_d230_s57, cbrt_native_w, cbrt_newton_slice_n, 12, 36, 57, "cbrt_d230_s57");
wide_root_bench!(bench_cbrt_d230_s114, cbrt_native_w, cbrt_newton_slice_n, 12, 36, 114, "cbrt_d230_s114");
wide_root_bench!(bench_cbrt_d230_s172, cbrt_native_w, cbrt_newton_slice_n, 12, 36, 172, "cbrt_d230_s172");
// 3/4-band focus for cbrt (the bbc residuals): D230 around s172 + D462
// around s346. The current policy routes both these cells to Native; bbc
// says the chosen arm is 1.10-1.13x slower than 0.4.4, so we re-test the
// Native-vs-slice verdict at + immediately around those points.
wide_root_bench!(bench_cbrt_d230_s144, cbrt_native_w, cbrt_newton_slice_n, 12, 36, 144, "cbrt_d230_s144");
wide_root_bench!(bench_cbrt_d230_s200, cbrt_native_w, cbrt_newton_slice_n, 12, 36, 200, "cbrt_d230_s200");
wide_root_bench!(bench_cbrt_d462_s173, cbrt_native_w, cbrt_newton_slice_n, 24, 72, 173, "cbrt_d462_s173");
wide_root_bench!(bench_cbrt_d462_s230, cbrt_native_w, cbrt_newton_slice_n, 24, 72, 230, "cbrt_d462_s230");
wide_root_bench!(bench_cbrt_d462_s288, cbrt_native_w, cbrt_newton_slice_n, 24, 72, 288, "cbrt_d462_s288");
wide_root_bench!(bench_cbrt_d462_s346, cbrt_native_w, cbrt_newton_slice_n, 24, 72, 346, "cbrt_d462_s346");
wide_root_bench!(bench_cbrt_d462_s400, cbrt_native_w, cbrt_newton_slice_n, 24, 72, 400, "cbrt_d462_s400");
// D307 around s230 (3/4 of S=307); already-defined s230 is the 3/4 point.
// D616 around s461 (3/4 of S=616).
wide_root_bench!(bench_cbrt_d616_s307, cbrt_native_w, cbrt_newton_slice_n, 32, 96, 307, "cbrt_d616_s307");
wide_root_bench!(bench_cbrt_d616_s461, cbrt_native_w, cbrt_newton_slice_n, 32, 96, 461, "cbrt_d616_s461");

// 3-way cbrt seed A/B: routed `fast_a` (0.4.4 full-radicand f64 seed, with
// shipped-seed fallback past bit_length 1020) vs `fast_b` (width-safe top-bits
// seed with tight 2^(r/3) residue + +1 margin) vs the slice. Run at the wide
// cells where fast_a falls back to the shipped seed (D230_s172, D462_s346,
// D616_s461 -- 3/4 band) -- these are exactly the bbc residual cells where the
// shipped-seed 2.5x over-shoot dominates, so fast_b should pull ahead.
macro_rules! wide_cbrt_3way {
    ($fnname:ident, $n:literal, $w:literal, $s:literal, $group:literal) => {
        fn $fnname(c: &mut Criterion) {
            let fa = |o: WideOne<$n>| cbrt_native_fast_a_w::<$n, $w, $s>(o.raw, MODE);
            let fb = |o: WideOne<$n>| cbrt_native_fast_b_w::<$n, $w, $s>(o.raw, MODE);
            let slc = |o: WideOne<$n>| cbrt_newton_slice_n::<$n, $s>(o.raw, MODE);
            for o in wide_inputs::<$n>() {
                for m in ALL_MODES {
                    let r_fa = cbrt_native_fast_a_w::<$n, $w, $s>(o.raw, m);
                    let r_fb = cbrt_native_fast_b_w::<$n, $w, $s>(o.raw, m);
                    let r_slc = cbrt_newton_slice_n::<$n, $s>(o.raw, m);
                    assert_eq!(r_fa, r_slc, concat!($group, " fast_a vs slice {} mode {:?}"), o.label, m);
                    assert_eq!(r_fb, r_slc, concat!($group, " fast_b vs slice {} mode {:?}"), o.label, m);
                }
            }
            compare_all(
                c,
                $group,
                |o: &WideOne<$n>| o.label.to_string(),
                wide_inputs::<$n>(),
                vec![
                    ("fast_a", Box::new(fa) as Box<dyn Fn(WideOne<$n>) -> Int<$n>>),
                    ("fast_b", Box::new(fb)),
                    ("slice", Box::new(slc)),
                ],
            );
        }
    };
}

// 3/4-scale band at the bbc residual cells: D230_s172, D462_s346 (primary),
// plus immediate neighbours (D230_s144, D230_s200; D462_s230, D462_s400) and
// the analogous wider tiers (D616_s461, D307_s230 -- already exists, re-bench
// here under 3way naming for direct fast_a vs fast_b comparison).
wide_cbrt_3way!(bench_cbrt3_d230_s144, 12, 36, 144, "cbrt3_d230_s144");
wide_cbrt_3way!(bench_cbrt3_d230_s172, 12, 36, 172, "cbrt3_d230_s172");
wide_cbrt_3way!(bench_cbrt3_d230_s200, 12, 36, 200, "cbrt3_d230_s200");
wide_cbrt_3way!(bench_cbrt3_d307_s230, 16, 48, 230, "cbrt3_d307_s230");
wide_cbrt_3way!(bench_cbrt3_d462_s230, 24, 72, 230, "cbrt3_d462_s230");
wide_cbrt_3way!(bench_cbrt3_d462_s346, 24, 72, 346, "cbrt3_d462_s346");
wide_cbrt_3way!(bench_cbrt3_d462_s400, 24, 72, 400, "cbrt3_d462_s400");
wide_cbrt_3way!(bench_cbrt3_d616_s461, 32, 96, 461, "cbrt3_d616_s461");
// 3/4-band low-side: D115_s57b, D153_s76 -- where fast_a's f64 path IS valid
// (smaller cells). These document that fast_a beats fast_b inside the f64
// range (so fast_b is not a blanket replacement -- it's a wide-cell fix).
wide_cbrt_3way!(bench_cbrt3_d115_s57, 6, 18, 57, "cbrt3_d115_s57");
wide_cbrt_3way!(bench_cbrt3_d153_s76, 8, 24, 76, "cbrt3_d153_s76");

fn bench_cbrt_seed_3way(c: &mut Criterion) {
    bench_cbrt3_d230_s144(c);
    bench_cbrt3_d230_s172(c);
    bench_cbrt3_d230_s200(c);
    bench_cbrt3_d307_s230(c);
    bench_cbrt3_d462_s230(c);
    bench_cbrt3_d462_s346(c);
    bench_cbrt3_d462_s400(c);
    bench_cbrt3_d616_s461(c);
    bench_cbrt3_d115_s57(c);
    bench_cbrt3_d153_s76(c);
}

fn bench_wide_bisect(c: &mut Criterion) {
    bench_sqrt_d307_s76(c);
    bench_sqrt_d307_s150b(c);
    bench_sqrt_d307_s230(c);
    bench_cbrt_d307_s76(c);
    bench_cbrt_d307_s150b(c);
    bench_cbrt_d307_s230(c);
    bench_sqrt_d115_s28(c);
    bench_sqrt_d115_s57b(c);
    bench_sqrt_d115_s85(c);
    bench_cbrt_d115_s28(c);
    bench_cbrt_d115_s57b(c);
    bench_cbrt_d115_s85(c);
    bench_sqrt_d153_s38(c);
    bench_sqrt_d153_s76(c);
    bench_sqrt_d153_s114(c);
    bench_cbrt_d153_s38(c);
    bench_cbrt_d153_s76(c);
    bench_cbrt_d153_s114(c);
    bench_sqrt_d230_s57(c);
    bench_sqrt_d230_s114(c);
    bench_sqrt_d230_s172(c);
    bench_cbrt_d230_s57(c);
    bench_cbrt_d230_s114(c);
    bench_cbrt_d230_s172(c);
    // sqrt threshold-tightening bisection
    bench_sqrt_d115_s20(c);
    bench_sqrt_d115_s22(c);
    bench_sqrt_d115_s24(c);
    bench_sqrt_d153_s28(c);
    bench_sqrt_d153_s32(c);
    bench_sqrt_d153_s35(c);
    bench_sqrt_d230_s72(c);
    bench_sqrt_d230_s85(c);
    bench_sqrt_d230_s100(c);
    bench_sqrt_d307_s48(c);
    bench_sqrt_d307_s56(c);
    bench_sqrt_d307_s64(c);
    // sqrt wide-tier crossover sweep
    bench_sqrt_d462_s24(c);
    bench_sqrt_d462_s48(c);
    bench_sqrt_d462_s72(c);
    bench_sqrt_d462_s96(c);
    bench_sqrt_d462_s120(c);
    bench_sqrt_d462_s230(c);
    bench_sqrt_d616_s32(c);
    bench_sqrt_d616_s64(c);
    bench_sqrt_d616_s96(c);
    bench_sqrt_d616_s128(c);
    bench_sqrt_d616_s307(c);
    bench_sqrt_d924_s96(c);
    bench_sqrt_d924_s144(c);
    bench_sqrt_d924_s192(c);
    bench_sqrt_d924_s462(c);
    bench_sqrt_d1232_s128(c);
    bench_sqrt_d1232_s192(c);
    bench_sqrt_d1232_s256(c);
    bench_sqrt_d1232_s615(c);
    // second-pass bisection
    bench_sqrt_d115_s21(c);
    bench_sqrt_d115_s23(c);
    bench_sqrt_d230_s62(c);
    bench_sqrt_d230_s66(c);
    bench_sqrt_d230_s70(c);
    bench_sqrt_d616_s160(c);
    bench_sqrt_d616_s192(c);
    bench_sqrt_d616_s224(c);
    bench_sqrt_d924_s220(c);
    bench_sqrt_d924_s260(c);
    bench_sqrt_d924_s320(c);
    bench_sqrt_d1232_s300(c);
    bench_sqrt_d1232_s400(c);
    bench_sqrt_d1232_s500(c);
    bench_cbrt_d230_s144(c);
    bench_cbrt_d230_s200(c);
    bench_cbrt_d462_s173(c);
    bench_cbrt_d462_s230(c);
    bench_cbrt_d462_s288(c);
    bench_cbrt_d462_s346(c);
    bench_cbrt_d462_s400(c);
    bench_cbrt_d616_s307(c);
    bench_cbrt_d616_s461(c);
}

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

fn bench_wide_max(c: &mut Criterion) {
    bench_sqrt_d115_max(c);
    bench_sqrt_d153_max(c);
    bench_sqrt_d230_max(c);
    bench_sqrt_d307_max(c);
    bench_sqrt_d462_max(c);
    bench_sqrt_d616_max(c);
    bench_sqrt_d924_max(c);
    bench_sqrt_d1232_max(c);
    bench_cbrt_d115_max(c);
    bench_cbrt_d153_max(c);
    bench_cbrt_d230_max(c);
    bench_cbrt_d307_max(c);
    bench_cbrt_d462_max(c);
    bench_cbrt_d616_max(c);
    bench_cbrt_d924_max(c);
    bench_cbrt_d1232_max(c);
}


/// The narrow-tier cells `select` routes to `MgDivide` without ever having
/// raced the `Native` arm against it.
fn bench_narrow(c: &mut Criterion) {
    bench_cbrt_d18_s0(c);
    bench_cbrt_d18_s1(c);
    bench_cbrt_d18_s2(c);
    bench_cbrt_d18_s3(c);
    bench_cbrt_d38_s3(c);
    bench_cbrt_d18_s4(c);
    bench_cbrt_d18_s9(c);
    bench_cbrt_d18_s13(c);
    bench_cbrt_d18_s17(c);
    bench_sqrt_d18_s0(c);
    bench_sqrt_d18_s4(c);
    bench_sqrt_d18_s9(c);
    bench_sqrt_d18_s13(c);
    bench_sqrt_d18_s17(c);
    bench_cbrt_d38_s0(c);
    bench_cbrt_d38_s2(c);
    bench_cbrt_d38_s4(c);
    bench_cbrt_d38_s6(c);
    bench_sqrt_d38_s2(c);
    bench_sqrt_d38_s4(c);
    bench_sqrt_d38_s6(c);
    bench_cbrt_d38_s9(c);
    bench_cbrt_d38_s14(c);
    bench_cbrt_d38_s19(c);
    bench_cbrt_d38_s28(c);
    bench_cbrt_d38_s37(c);
    bench_sqrt_d38_s0(c);
    bench_sqrt_d38_s9(c);
    bench_sqrt_d38_s19n(c);
    bench_sqrt_d38_s28(c);
}

fn benches(c: &mut Criterion) {
    bench_narrow(c);
    bench_sqrt(c);
    bench_cbrt(c);
    bench_wide(c);
    bench_wide_max(c);
    bench_wide_bisect(c);
    bench_cbrt_seed_3way(c);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
