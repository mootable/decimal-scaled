// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Same-build A/B for the wide-transcendental `÷10^w` round-divide
//! recovery: the SLOW generic-Knuth `round_div` by `10^w` (the
//! pre-recovery transcendental-band reduce) vs the FAST production
//! `round_div_pow10` MG / Newton power-of-10 kernel.
//!
//! The fix is a GENERIC kernel-class change — it affects EVERY wide
//! work-integer width at EVERY working scale `w`. So this sweeps the
//! full surface: the work widths the wide tiers actually run at
//! (`Int<8>`=D57, `Int<16>`=D76/D115, `Int<32>`=D153, `Int<48>`=D230,
//! `Int<64>`=D307/D462, `Int<128>`=D616) crossed with a fine `w`
//! grid spanning the single-chunk MG regime (`w<=38`) and the
//! chain / Newton regime (`w>38`) up to the widest tiers' working
//! scale. Each cell ASSERTS the two paths are bit-identical (the
//! validity wall) before timing, then reports the FAST/SLOW ratio.
//! Goal: FAST wins-or-ties everywhere, regresses nothing; any
//! (width, w) sub-region where FAST is slower is a crossover to gate.
//!
//! Run: `cargo bench --features "wide x-wide macros bench-alt" --bench div_recover_ab`

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::Int;
use decimal_scaled::__bench_internals::{
    int_from_mag_limbs, round_div_pow10_fast, round_div_pow10_slow,
};

/// A near-full-width positive numerator for `Int<N>` (top limb kept
/// modest so it stays a plain positive magnitude, like a working-scale
/// product). Deterministic.
fn numerator<const N: usize>() -> Int<N> {
    let mut mag = [0u64; N];
    for (i, slot) in mag.iter_mut().enumerate() {
        *slot = 0x9E37_79B9_7F4A_7C15u64
            .wrapping_mul(i as u64 + 1)
            .wrapping_add(0x1357_9BDF_2468_ACE0);
    }
    mag[N - 1] &= 0x0000_FFFF_FFFF_FFFF;
    int_from_mag_limbs::<N>(&mag)
}

/// Coarse wall-clock timing, us/op.
fn time_us<T>(iters: u32, mut f: impl FnMut() -> T) -> f64 {
    for _ in 0..(iters / 4).max(1) {
        black_box(f());
    }
    let t0 = Instant::now();
    for _ in 0..iters {
        black_box(f());
    }
    t0.elapsed().as_secs_f64() * 1e6 / f64::from(iters)
}

/// One concrete-width `w` sweep: per `w`, assert bit-identity then time
/// both paths. `$n` is the concrete work-width limb count (so the fast
/// shim's `ComputeInt` bound is satisfied at this monomorphisation).
macro_rules! sweep {
    ($label:literal, $n:literal, $w_max:expr, $iters:expr) => {{
        let n = numerator::<$n>();
        let mut ws: Vec<u32> = vec![1, 4, 8, 16, 24, 32, 38, 39, 48, 64, 80, 100];
        let w_max: u32 = $w_max;
        for q in [w_max / 4, w_max / 2, (3 * w_max) / 4, w_max.saturating_sub(1)] {
            if q > 100 {
                ws.push(q);
            }
        }
        ws.retain(|&w| w <= w_max);
        ws.sort_unstable();
        ws.dedup();
        for w in ws {
            let slow = round_div_pow10_slow::<Int<$n>>(n, w);
            let fast = round_div_pow10_fast::<Int<$n>>(n, w);
            assert_eq!(slow, fast, concat!("MISMATCH ", $label, " w={}"), w);

            let iters: u32 = $iters;
            let t_slow =
                time_us(iters, || round_div_pow10_slow::<Int<$n>>(black_box(n), black_box(w)));
            let t_fast =
                time_us(iters, || round_div_pow10_fast::<Int<$n>>(black_box(n), black_box(w)));
            let ratio = t_slow / t_fast;
            let verdict = if ratio >= 1.10 {
                "FAST wins"
            } else if ratio <= 0.91 {
                "FAST REGRESSES <<<"
            } else {
                "~tie"
            };
            println!(
                "{:<12} w={:<4} slow={:>9.4} fast={:>9.4} us  ratio={:>6.2}x  {}",
                $label, w, t_slow, t_fast, ratio, verdict
            );
        }
    }};
}

fn main() {
    // GUARD is 30/60 below storage scale; working scale w = SCALE+GUARD+extra.
    // w_max per work width ~ widest-tier-at-that-width max scale + ~60:
    //   Int<8>  -> D57  (max 56)   Int<16> -> D76/D115 (max 115)
    //   Int<32> -> D153 (max 152)  Int<48> -> D230 (max 229)
    //   Int<64> -> D307/D462 (461) Int<128>-> D616 (615)
    println!("== wide-transcendental div_pow10 A/B: SLOW(round_div) vs FAST(round_div_pow10) ==");
    sweep!("Int8/D57", 8, 120, 6000);
    sweep!("Int16/D76", 16, 180, 5000);
    sweep!("Int32/D153", 32, 215, 3000);
    sweep!("Int48/D230", 48, 290, 1500);
    sweep!("Int64/D307", 64, 520, 1000);
    sweep!("Int128/D616", 128, 675, 400);
}
