// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the wide-tier `to_radians` angle conversion.
//!
//! Models the `bench-branch-compare` to_radians regression vs 0.4.4
//! (D57 +64%, D76 +41%, D115 +56%, D153 +43%). The `MulPiRatio` kernel
//! (`x * pi / 180` in the guard-digit work integer) is identical to
//! 0.4.4's algorithm, so the candidates here isolate the routing /
//! resize overhead the rewrite added between the public method and the
//! kernel:
//! - `public` -> `D::to_radians_strict_with` (the full path through
//!   `to_radians_dispatch` -> `to_radians::dispatch` ->
//!   `mul_pi_ratio_routed`, with its `resize_to` round-trips).
//! - `direct`  -> the `MulPiRatio` kernel called straight on the tier
//!   core (no policy / resize indirection).
//!
//! Both are bit-identical (asserted before timing). The gap is the
//! per-call dispatch overhead.
//!
//! Run: `cargo bench --features wide --bench to_radians_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    to_radians_direct_d115, to_radians_direct_d153, to_radians_direct_d57, to_radians_direct_d76,
    to_radians_public_d115, to_radians_public_d153, to_radians_public_d57, to_radians_public_d76,
};
use decimal_scaled::{Int, RoundingMode};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;

#[derive(Clone)]
struct RIn<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

/// Degree inputs `x * 10^SCALE` for a few representative angles.
fn inputs<const N: usize>(scale: u32) -> Vec<RIn<N>> {
    // Build `deg * 10^scale` directly in Int<N> (no u128 overflow at
    // high scale).
    let scaled = |deg: i128| -> Int<N> {
        let mut v = Int::<N>::try_from(deg).unwrap();
        let ten = Int::<N>::try_from(10i128).unwrap();
        for _ in 0..scale {
            v = v * ten;
        }
        v
    };
    vec![
        RIn { label: "30deg", raw: scaled(30) },
        RIn { label: "45deg", raw: scaled(45) },
        RIn { label: "180deg", raw: scaled(180) },
    ]
}

macro_rules! rad_cell {
    ($c:expr, $n:literal, $scale:literal, $label:literal, $direct:ident, $public:ident) => {{
        for o in inputs::<$n>($scale) {
            assert_eq!(
                $direct::<$scale>(o.raw, MODE),
                $public::<$scale>(o.raw, MODE),
                "to_radians arms disagree at {} ({})",
                $label,
                o.label
            );
        }
        compare_all(
            $c,
            concat!("to_radians/", $label),
            |o: &RIn<$n>| o.label.to_string(),
            inputs::<$n>($scale),
            vec![
                (
                    "public",
                    (|o: RIn<$n>| $public::<$scale>(o.raw, MODE)) as fn(RIn<$n>) -> Int<$n>,
                ),
                ("direct", |o: RIn<$n>| $direct::<$scale>(o.raw, MODE)),
            ],
        );
    }};
}

fn bench(c: &mut Criterion) {
    rad_cell!(c, 3, 19, "D57_s19", to_radians_direct_d57, to_radians_public_d57);
    rad_cell!(c, 4, 24, "D76_s24", to_radians_direct_d76, to_radians_public_d76);
    rad_cell!(c, 6, 38, "D115_s38", to_radians_direct_d115, to_radians_public_d115);
    rad_cell!(c, 8, 50, "D153_s50", to_radians_direct_d153, to_radians_public_d153);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
