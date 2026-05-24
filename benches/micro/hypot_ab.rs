// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the integer hypot policy (`src/int/policy/hypot.rs`).
//!
//! Decision being modelled: for narrow storage (D18/D38/D57) where the
//! radicand `a² + b²` fits a `u128`, should hypot route to the generic
//! Pythagoras path (form the radicand in scratch, floor root via the Newton
//! slice `isqrt` with a per-iteration multi-precision `div_rem`, round) or to
//! the native-`u128` fast path (`hypot_u128_fast`: floor sqrt in u128 with an
//! f64 seed + exact remainder round, no multi-precision divide)?
//!
//! Two registered arms (both bit-identical across all six RoundingModes —
//! asserted before timing):
//! - `pythagoras` -> `hypot_pythagoras` (the production kernel).
//! - `u128_fast`  -> `hypot_u128_fast` (the candidate).
//!
//! Run: `cargo bench --features "wide bench-alt" --bench hypot_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{hypot_pythagoras, hypot_u128_fast};
use decimal_scaled::{Int, RoundingMode};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;

#[derive(Clone)]
struct HIn<const N: usize> {
    label: &'static str,
    a: Int<N>,
    b: Int<N>,
}

fn inputs<const N: usize>() -> Vec<HIn<N>> {
    let v = |x: i128| Int::<N>::try_from(x).unwrap();
    vec![
        HIn { label: "3_4", a: v(3), b: v(4) },
        HIn { label: "mid", a: v(123_456_789), b: v(987_654_321) },
        HIn { label: "large", a: v(1_000_000_000_000), b: v(2_345_678_901_234) },
    ]
}

macro_rules! hypot_cell {
    ($c:expr, $n:literal, $label:literal) => {{
        // Correctness cross-check: both arms must agree before timing.
        for o in inputs::<$n>() {
            assert_eq!(
                hypot_pythagoras::<$n>(o.a, o.b, MODE),
                hypot_u128_fast::<$n>(o.a, o.b, MODE),
                "hypot arms disagree at N={} ({})",
                $n,
                o.label
            );
        }
        compare_all(
            $c,
            concat!("hypot/", $label),
            |o: &HIn<$n>| o.label.to_string(),
            inputs::<$n>(),
            vec![
                (
                    "pythagoras",
                    (|o: HIn<$n>| hypot_pythagoras::<$n>(o.a, o.b, MODE))
                        as fn(HIn<$n>) -> Option<Int<$n>>,
                ),
                ("u128_fast", |o: HIn<$n>| hypot_u128_fast::<$n>(o.a, o.b, MODE)),
            ],
        );
    }};
}

fn bench_hypot(c: &mut Criterion) {
    hypot_cell!(c, 1, "D18");
    hypot_cell!(c, 2, "D38");
    hypot_cell!(c, 3, "D57");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_hypot(&mut c);
    c.final_summary();
}
