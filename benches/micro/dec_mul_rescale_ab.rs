// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Whole-op decimal `mul` bench at the bbc regression cluster cells.
//!
//! Each cell is a real `D<Int<N>, SCALE>::mul` via the production
//! `mul_widen_divide` kernel — the wide product followed by the
//! `÷10^SCALE` rescale (the proven lever). For `SCALE > 38` the rescale
//! goes through `dispatch_pow10_mag_u128` → `div_pow10_chain_mag_u128`,
//! which is the path under scrutiny for the regression.
//!
//! Cells (storage limbs N → tier, SCALE):
//!   N=3  D57  s56     N=4  D76  s75
//!   N=6  D115 s57     N=8  D153 s76
//!   N=12 D230 s229
//!
//! Run:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench dec_mul_rescale_ab`

use criterion::{black_box, Criterion};
use decimal_scaled::__bench_internals::{dec_mul_widen_divide, int_from_mag_limbs};
use decimal_scaled::RoundingMode;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::micro_criterion;

use decimal_scaled::Int;

const MODE: RoundingMode = RoundingMode::HalfToEven;

/// Build a representative near-full-width operand for `Int<N>`: fill the
/// low limbs with a pseudo-random pattern but keep the top limb modest so
/// the product does not overflow `Int<N>` after the rescale.
fn operand<const N: usize>(seed: u64) -> Int<N> {
    let mut mag = [0u64; N];
    for (i, slot) in mag.iter_mut().enumerate() {
        *slot = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add((i as u64).wrapping_mul(0x1357_9BDF));
    }
    // keep the top limb small so a·b/10^scale stays representable
    mag[N - 1] &= 0xFFFF;
    int_from_mag_limbs::<N>(&mag)
}

macro_rules! cell {
    ($c:expr, $tier:literal, $n:literal, $scale:literal) => {{
        let a = operand::<$n>(1009);
        let b = operand::<$n>(2027);
        let mut group = $c.benchmark_group("dec_mul_rescale");
        group.bench_function(format!("{}_n{}_s{}", $tier, $n, $scale), |bn| {
            bn.iter(|| {
                black_box(dec_mul_widen_divide::<$n, $scale>(
                    black_box(a),
                    black_box(b),
                    MODE,
                ))
            })
        });
        group.finish();
    }};
}

fn bench(c: &mut Criterion) {
    cell!(c, "D57", 3, 56);
    cell!(c, "D76", 4, 75);
    cell!(c, "D115", 6, 57);
    cell!(c, "D153", 8, 76);
    cell!(c, "D230", 12, 229);
    cell!(c, "D462", 24, 461);
    cell!(c, "D616", 32, 615);
    cell!(c, "D924", 48, 462);
    cell!(c, "D924", 48, 923);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
