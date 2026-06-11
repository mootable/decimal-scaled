// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A/B: decimal `+` at the bbc-regressed D462 (`Int<24>`) cell — live vs the
//! pinned 0.4.4 release (`ds-044`) — with D616 (`Int<32>`, bbc-flat) as the
//! control pair.
//!
//! Context: bench-branch-compare run 27317386158 shows `add D462` at 2× vs
//! 0.4.4 across every sampled scale while every other width is ≤ +6%, yet the
//! ripple-carry kernel source is byte-identical between the two versions and
//! the local width curve shows a >16-limb codegen cliff (per-limb cost ~0.85 ns
//! through `Int<16>`, ~2.1 ns at `Int<24>`). This bench answers, on one
//! machine and one compiler: does 0.4.4's compiled D462 add really escape the
//! cliff the live build pays?
//!
//! Run (the bench self-pins to the machine's highest core):
//! `cargo bench -p decimal-scale-test --features x-wide,history-044 --bench add_history_ab`

use criterion::{criterion_group, Criterion};
use std::hint::black_box;
use std::time::Duration;

fn micro() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .measurement_time(Duration::from_millis(400))
        .warm_up_time(Duration::from_millis(150))
}

/// Mid-magnitude scale-115 operand strings. Value shape is irrelevant to a
/// fixed-N ripple add (the loop runs all N limbs regardless); the sum stays
/// far inside both tiers' range so the checked add never panics.
fn operands() -> (String, String) {
    let a = format!("{}.{}", "123456789".repeat(10), "7".repeat(115));
    let b = format!("{}.{}", "987654321".repeat(9), "3".repeat(115));
    (a, b)
}

macro_rules! bench_pair {
    ($c:expr, $label:literal, $Live:ty, $V044:ty) => {{
        let (a_s, b_s) = operands();
        {
            let a: $Live = a_s.parse().unwrap();
            let b: $Live = b_s.parse().unwrap();
            $c.bench_function(concat!("add_hist/", $label, "/live"), |bn| {
                bn.iter(|| black_box(a) + black_box(b))
            });
        }
        {
            let a: $V044 = a_s.parse().unwrap();
            let b: $V044 = b_s.parse().unwrap();
            $c.bench_function(concat!("add_hist/", $label, "/v044"), |bn| {
                bn.iter(|| black_box(a) + black_box(b))
            });
        }
    }};
}

fn bench_add_history(c: &mut Criterion) {
    // The regressed cell: D462<115> (Int<24>), bbc +159%.
    bench_pair!(c, "D462s115", decimal_scaled::D462<115>, ds_044::D462<115>);
    // The control: D616<115> (Int<32>), bbc ≤ +2%.
    bench_pair!(c, "D616s115", decimal_scaled::D616<115>, ds_044::D616<115>);
}

/// Live-vs-0.4.4 rows for the bbc trig/ln scale-0 cluster (sin D115<0> +33%,
/// ln D115<0> +18%, ln D462<231> +36% in run 27322009266) — the same
/// real-vs-GHA-artifact split the add rows settled for D462. Operands mirror
/// the bbc bench exactly: trig argument 0, ln argument 2, at scale 0; the ln
/// D462<231> cell uses the fractional "2.0" form.
macro_rules! bench_unary_pair {
    ($c:expr, $label:literal, $op:ident, $arg:literal, $Live:ty, $V044:ty) => {{
        {
            let x: $Live = $arg.parse().unwrap();
            $c.bench_function(concat!("hist/", $label, "/live"), |bn| {
                bn.iter(|| black_box(x).$op())
            });
        }
        {
            let x: $V044 = $arg.parse().unwrap();
            $c.bench_function(concat!("hist/", $label, "/v044"), |bn| {
                bn.iter(|| black_box(x).$op())
            });
        }
    }};
}

/// Live-vs-0.4.4 binary-op rows: mul D230<229> (+12% bbc, replicated ×4 —
/// the last real arithmetic focus row). bbc operands: 2 × 3 at scale 229
/// (fractional forms "2.0"/"3.5" at s>0 per op_str).
fn bench_mul_history(c: &mut Criterion) {
    {
        let a: decimal_scaled::D230<229> = "2.0".parse().unwrap();
        let b: decimal_scaled::D230<229> = "3.5".parse().unwrap();
        c.bench_function("hist/mul_D230s229/live", |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
    }
    {
        let a: ds_044::D230<229> = "2.0".parse().unwrap();
        let b: ds_044::D230<229> = "3.5".parse().unwrap();
        c.bench_function("hist/mul_D230s229/v044", |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
    }
}

fn bench_trig_ln_history(c: &mut Criterion) {
    bench_unary_pair!(c, "sin_D115s0_x0", sin, "0", decimal_scaled::D115<0>, ds_044::D115<0>);
    bench_unary_pair!(c, "cos_D115s0_x0", cos, "0", decimal_scaled::D115<0>, ds_044::D115<0>);
    bench_unary_pair!(c, "sin_D307s0_x0", sin, "0", decimal_scaled::D307<0>, ds_044::D307<0>);
    bench_unary_pair!(c, "ln_D115s0_x2", ln, "2", decimal_scaled::D115<0>, ds_044::D115<0>);
    bench_unary_pair!(c, "ln_D462s231_x2", ln, "2.0", decimal_scaled::D462<231>, ds_044::D462<231>);
}

criterion_group! {
    name = benches;
    config = micro();
    targets = bench_add_history, bench_trig_ln_history, bench_mul_history
}

/// Self-pins to the highest-index logical core (the quietest under Windows,
/// which schedules system work on the low cores) so the measurement does not
/// migrate mid-run — same pattern as the root crate's `int_ops_micro`.
fn main() {
    if let Some(c) = core_affinity::get_core_ids().and_then(|v| v.into_iter().last()) {
        core_affinity::set_for_current(c);
    }
    benches();
    Criterion::default().configure_from_args().final_summary();
}
