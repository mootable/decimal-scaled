// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier exp algorithm map — the N-way width × scale × algorithm sweep
//! for the `policy::exp::select` Series-vs-Tang-vs-Schoolbook decision.
//!
//! For each wide tier (D57..D1232), at the owner-standard 5-point scale grid
//! `{0, S/4, S/2, 3S/4, S-1}`, it asserts each non-Series candidate (the three
//! Tang table/guard configs AND the `Schoolbook` reference kernel) is
//! BIT-IDENTICAL to Series across the operand spread × all six rounding modes
//! (the **validity wall** — a candidate that disagrees anywhere is reported
//! INVALID and dropped), then ranks the surviving candidates against Series.
//!
//! Wire a wide `select`/`tang_routed` arm to a non-Series algorithm ONLY for a
//! cell where that algorithm is BOTH bit-identical to Series AND faster.
//!
//! Every variant of the exp `Algorithm` enum is benched, no exceptions —
//! `Series` (the reference / always-valid baseline), `Tang` (3 configs), and
//! `Schoolbook` (owner directive 2026-05-29). Schoolbook is the naive Maclaurin
//! kernel registered as the unrouted reference arm; it is included and measured
//! at every cell so its result is reported (it shares the same Fixed-intermediate
//! Smith-reduction core as Series, so it is expected to bench as a near-tie —
//! reported FROM THE DATA, not pre-dropped).
//!
//! A separate value-gate sweep exercises the `wide_tang_gate` `|x| < ~100`
//! boundary at low scale: small `|x|` (0.5, 10), near the gate (~90, ~100,
//! ~110), and large `|x|` (500) — where Tang's `k·ln2` lift may exceed the work
//! width and Tang becomes INELIGIBLE (the harness drops it).
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench exp_wide_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    exp_schoolbook_d115, exp_schoolbook_d1232, exp_schoolbook_d153, exp_schoolbook_d230,
    exp_schoolbook_d307, exp_schoolbook_d462, exp_schoolbook_d57, exp_schoolbook_d616,
    exp_schoolbook_d76, exp_schoolbook_d924, exp_series_d115, exp_series_d1232, exp_series_d153,
    exp_series_d230, exp_series_d307, exp_series_d462, exp_series_d57, exp_series_d616,
    exp_series_d76, exp_series_d924, exp_tang_d115_p, exp_tang_d1232, exp_tang_d153_p,
    exp_tang_d230, exp_tang_d307, exp_tang_d462, exp_tang_d57, exp_tang_d616, exp_tang_d76,
    exp_tang_d924, int_from_mag_limbs,
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

/// `acc = acc * m + add`, in place across the little-endian limb array. Pure
/// big-int limb arithmetic so any decimal scale (even scale >= 39, where
/// `10^scale` overflows u128) is representable in the wide `Int<N>`.
fn mul_add_small<const N: usize>(acc: &mut [u64; N], m: u64, add: u64) {
    let mut carry = add as u128;
    for limb in acc.iter_mut() {
        let prod = (*limb as u128) * (m as u128) + carry;
        *limb = prod as u64;
        carry = prod >> 64;
    }
}

/// Build raw = (x_num/x_den) * 10^scale directly in `Int<N>` limbs, never via
/// `10u128.pow` (which overflows for scale >= 39). Computes `x_num * 10^scale`
/// limb-wise then divides exactly by the small denominator (2 or 1 here).
fn build_raw<const N: usize>(x_num: u64, x_den: u64, scale: u32) -> Int<N> {
    let mut mag = [0u64; N];
    mag[0] = x_num;
    for _ in 0..scale {
        mul_add_small::<N>(&mut mag, 10, 0);
    }
    if x_den != 1 {
        let mut rem = 0u128;
        for limb in mag.iter_mut().rev() {
            let cur = (rem << 64) | (*limb as u128);
            *limb = (cur / x_den as u128) as u64;
            rem = cur % x_den as u128;
        }
    }
    int_from_mag_limbs::<N>(&mag)
}

#[derive(Clone)]
struct One<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

/// exp arguments at decimal `SCALE`: x in {0.5, 1.5, 3.0} -> raw = x*10^SCALE.
/// 0.5 is the bbc operand; the others stay inside the shared reduction window.
fn exp_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![
        One { label: "x0.5", raw: build_raw::<N>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<N>(3, 2, scale) },
        One { label: "x3.0", raw: build_raw::<N>(3, 1, scale) },
    ]
}

type ExpFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

/// One cell: validate each non-Series candidate against Series (bit-identical
/// across the spread × all modes), drop the invalid ones, then rank the
/// survivors and Series in a single `compare_all` run over the supplied inputs.
fn cell_inputs<const N: usize>(
    c: &mut Criterion,
    group: &str,
    inputs: &[One<N>],
    series: ExpFn<N>,
    candidates: &[(&'static str, ExpFn<N>)],
) {
    let mut runs: Vec<(&'static str, Box<dyn Fn(One<N>) -> Int<N>>)> =
        vec![("series", Box::new(move |o: One<N>| series(o.raw, MODE)))];
    for &(label, cand) in candidates {
        let mut valid = true;
        'outer: for o in inputs {
            for m in ALL_MODES {
                if cand(o.raw, m) != series(o.raw, m) {
                    println!(
                        "VALIDITY [{group}]: {label} != series ({}, mode {m:?}) -> INVALID, skipping",
                        o.label
                    );
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            runs.push((label, Box::new(move |o: One<N>| cand(o.raw, MODE))));
        }
    }
    // compare_all needs >= 2 survivors; if every non-Series candidate is
    // INVALID the cell is Series-only (report it and skip the timed run).
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all non-Series candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), inputs.to_vec(), runs);
}

/// Scale-grid cell over the standard {0.5,1.5,3.0} spread.
fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: ExpFn<N>,
    candidates: &[(&'static str, ExpFn<N>)],
) {
    cell_inputs::<N>(c, group, &exp_inputs::<N>(scale), series, candidates);
}

/// One (tier, scale) cell: Series vs the three Tang configs (M=128/G=30 = the
/// production wide config; M=512/G=30 = wider table — the production wide-tier
/// choice for D76..D462; M=512/G=60 = wider table + wider guard) AND Schoolbook
/// (the reference Maclaurin kernel). SCALE is a literal const generic so each
/// cell is its own monomorphisation, as the policy sees it.
macro_rules! cell5 {
    ($c:expr, $n:literal, $name:literal, $scale:literal, $series:ident, $tang:ident, $school:ident) => {
        cell::<$n>(
            $c,
            concat!("exp_", $name, "_s", stringify!($scale)),
            $scale,
            $series::<$scale>,
            &[
                ("tang_m128_g30", $tang::<$scale, 128, 30>),
                ("tang_m512_g30", $tang::<$scale, 512, 30>),
                ("tang_m512_g60", $tang::<$scale, 512, 60>),
                ("schoolbook", $school::<$scale>),
            ],
        );
    };
}

/// Sweep a tier across the FIVE coarse scale points
/// `{0, S/4, S/2, 3S/4, S-1}` (the owner-standard sampling) as literals.
macro_rules! tier {
    ($c:expr, $n:literal, $name:literal, $series:ident, $tang:ident, $school:ident,
     $s0:literal, $s1:literal, $s2:literal, $s3:literal, $s4:literal) => {{
        cell5!($c, $n, $name, $s0, $series, $tang, $school);
        cell5!($c, $n, $name, $s1, $series, $tang, $school);
        cell5!($c, $n, $name, $s2, $series, $tang, $school);
        cell5!($c, $n, $name, $s3, $series, $tang, $school);
        cell5!($c, $n, $name, $s4, $series, $tang, $school);
    }};
}

fn benches(c: &mut Criterion) {
    // Per tier: scales {0, S/4, S/2, 3S/4, S-1} (the owner-standard 5-point
    // coarse sampling). The TOP point is the tier's MAX-SCALE EXTREME, capped
    // at `capacity - 2` so the x3.0 input (`e^3 ≈ 20`, two integer digits)
    // still fits storage — a larger top scale leaves < 2 integer digits and
    // the strict-transcendental range check panics during cell setup.
    // D57 (Int<3>, cap 57) — full 5-point grid (was only in the narrow
    // exp_series_tang_ab band probe).
    tier!(c, 3, "d57", exp_series_d57, exp_tang_d57, exp_schoolbook_d57, 0, 14, 28, 42, 55);
    // D76 (Int<4>, cap 76) — the bbc `exp_D76_s75`/`powf_D76_s75` regression.
    tier!(c, 4, "d76", exp_series_d76, exp_tang_d76, exp_schoolbook_d76, 0, 19, 38, 57, 74);
    // D115 (Int<6>, cap 115).
    tier!(c, 6, "d115", exp_series_d115, exp_tang_d115_p, exp_schoolbook_d115, 0, 28, 57, 86, 113);
    // D153 (Int<8>, cap 153).
    tier!(c, 8, "d153", exp_series_d153, exp_tang_d153_p, exp_schoolbook_d153, 0, 38, 76, 114, 151);
    // D230 (Int<12>, cap 230).
    tier!(c, 12, "d230", exp_series_d230, exp_tang_d230, exp_schoolbook_d230, 0, 57, 115, 172, 228);
    // D307 (Int<16>, cap 307).
    tier!(c, 16, "d307", exp_series_d307, exp_tang_d307, exp_schoolbook_d307, 0, 76, 153, 230, 305);
    // D462 (Int<24>, cap 462).
    tier!(c, 24, "d462", exp_series_d462, exp_tang_d462, exp_schoolbook_d462, 0, 115, 231, 346, 460);
    // D616 (Int<32>, cap 616).
    tier!(c, 32, "d616", exp_series_d616, exp_tang_d616, exp_schoolbook_d616, 0, 154, 308, 462, 614);
    // D924 (Int<48>, cap 924).
    tier!(c, 48, "d924", exp_series_d924, exp_tang_d924, exp_schoolbook_d924, 0, 231, 462, 693, 922);
    // D1232 (Int<64>, cap 1232).
    tier!(c, 64, "d1232", exp_series_d1232, exp_tang_d1232, exp_schoolbook_d1232, 0, 308, 616, 924, 1230);

    // ── Value-gate sweep — exercise the `wide_tang_gate` `|x| < ~100` boundary
    // at LOW scale (room for large integer parts). At each input the harness
    // re-validates every candidate vs Series, so an input where Tang is no
    // longer bit-identical (its `k·ln2` lift overflows the guard) drops Tang
    // automatically — i.e. the eligible set shrinks exactly where the policy
    // gate must hand off to Series. Done at a representative narrow-wide tier
    // (D76, Tang's strongest win-region) and a mid tier (D230). Scale 4 keeps
    // a small fractional spread while leaving integer-digit room for x≈500.
    value_gate::<4>(c, "d76", 4, exp_series_d76::<4>, exp_tang_d76::<4, 512, 30>, exp_schoolbook_d76::<4>);
    value_gate::<12>(c, "d230", 4, exp_series_d230::<4>, exp_tang_d230::<4, 512, 30>, exp_schoolbook_d230::<4>);
}

/// Value-gate cells: small / near-boundary / large `|x|` each as its OWN cell
/// (so the validity wall is applied per input-class — large `|x|` that breaks
/// Tang's `k·ln2` lift drops Tang there, while small `|x|` keeps it).
fn value_gate<const N: usize>(
    c: &mut Criterion,
    name: &str,
    scale: u32,
    series: ExpFn<N>,
    tang: ExpFn<N>,
    school: ExpFn<N>,
) {
    // (label, x_num, x_den) at the given scale. x = x_num/x_den. Capped near
    // |x|=110 because exp(x) itself overflows the tier's storage well before
    // x=500 (e^500 ≈ 10^217 >> D76's ~72 integer digits), which would panic in
    // the strict range check — so the meaningful gate boundary to map is the
    // |x| ≈ 90..110 band where Tang's k·ln2 lift approaches/exceeds the guard.
    let pts: [(&str, u64, u64); 7] = [
        ("x0.5", 1, 2),
        ("x10", 10, 1),
        ("x90", 90, 1),
        ("x99", 99, 1),
        ("x100", 100, 1),
        ("x101", 101, 1),
        ("x110", 110, 1),
    ];
    for (lbl, num, den) in pts {
        let inputs = vec![One { label: lbl, raw: build_raw::<N>(num, den, scale) }];
        let group = format!("exp_gate_{name}_{lbl}");
        cell_inputs::<N>(
            c,
            &group,
            &inputs,
            series,
            &[("tang_m512_g30", tang), ("schoolbook", school)],
        );
    }
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
