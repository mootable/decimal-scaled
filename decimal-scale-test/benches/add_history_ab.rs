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

criterion_group! {
    name = benches;
    config = micro();
    targets = bench_add_history
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
