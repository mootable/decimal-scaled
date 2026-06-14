//! Focused samply probe for the INT division ENGINES (`src/int/algos/div`),
//! one `engine × shape × width` per run — for the per-algorithm EFFICIENCY
//! analysis (pipeline step 1), NOT the routing/which-wins A/B (that is the
//! policy-mapper's job). Each run tight-loops ONE engine on ONE operand shape so
//! samply attributes time cleanly to that engine's own frames.
//!
//! Engines (via `__bench_internals`, the same surface `div_kernel_ab` benches):
//!   knuth  u128  bz  remfast  school
//! Shapes:
//!   bal    — balanced `N / N`            (the `rem` / balanced-divide shape)
//!   wide   — wide-numerator `2N / N`     (the decimal `/` quotient shape)
//!   tiny   — `2 / 3` in N-limb storage   (the bbc small-quotient / setup path)
//!   small  — full-N dividend / 1-limb den (the single-limb-divisor regime)
//! Widths: d18 d38 d57 d76 d115 d153 d230 d307 d462 d616 d924 d1232.
//!
//! Run: int_div_probe <width>_<shape>_<engine>
//!   int_div_probe d57_tiny_remfast     # the bbc-regression engine path
//!   int_div_probe d924_wide_u128       # the wide u128 engine
//! Build: cargo build --release --example int_div_probe \
//!          --features "wide x-wide xx-wide bench-alt"

use std::hint::black_box;
use std::time::{Duration, Instant};

use decimal_scaled::__bench_internals::{
    div_bz_forced_slice, div_knuth_slice, div_knuth_u128_limb_slice, div_rem_fast_slice,
    div_schoolbook_slice,
};

/// Deterministic dense full-width limbs (top limb's high bit set → max-magnitude
/// divisor / a real division). Mirrors `div_kernel_ab::fill`.
fn fill(seed: u64, used: usize) -> Vec<u64> {
    let mut v = vec![0u64; used];
    for i in 0..used {
        v[i] = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add((i as u64).wrapping_mul(0x1357_9BDF))
            ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    }
    if used > 0 {
        v[used - 1] |= 0x8000_0000_0000_0000;
    }
    v
}

fn shape(kind: &str, n: usize) -> (Vec<u64>, Vec<u64>) {
    match kind {
        "bal" => (fill(7919, n), fill(104729, n)),
        "wide" => (fill(1009, 2 * n), fill(13, n)),
        "tiny" => {
            let mut num = vec![0u64; n];
            let mut den = vec![0u64; n];
            num[0] = 2;
            den[0] = 3;
            (num, den)
        }
        "small" => (fill(2027, n), vec![0x9E37_79B9_7F4A_7C17]),
        other => panic!("unknown shape {other:?}"),
    }
}

type Engine = fn(&[u64], &[u64], &mut [u64], &mut [u64]);

fn engine(name: &str) -> Engine {
    match name {
        "knuth" => div_knuth_slice,
        "u128" => div_knuth_u128_limb_slice,
        "bz" => div_bz_forced_slice,
        "remfast" => div_rem_fast_slice,
        "school" => div_schoolbook_slice,
        other => panic!("unknown engine {other:?}"),
    }
}

fn width(tag: &str) -> usize {
    match tag {
        "d18" => 1,
        "d38" => 2,
        "d57" => 3,
        "d76" => 4,
        "d115" => 6,
        "d153" => 8,
        "d230" => 12,
        "d307" => 16,
        "d462" => 24,
        "d616" => 32,
        "d924" => 48,
        "d1232" => 64,
        other => panic!("unknown width {other:?}"),
    }
}

fn main() {
    let sel = std::env::args().nth(1).unwrap_or_default();
    let parts: Vec<&str> = sel.split('_').collect();
    if parts.len() != 3 {
        eprintln!("usage: int_div_probe <width>_<shape>_<engine>  (got {sel:?})");
        std::process::exit(2);
    }
    let n = width(parts[0]);
    let (num, den) = shape(parts[1], n);
    let eng = engine(parts[2]);

    let mut q = vec![0u64; num.len()];
    let mut r = vec![0u64; num.len()];
    // Time-budget the loop (~4 s of samples) rather than a fixed iter count, so
    // the ~3-orders-of-magnitude cost spread (narrow tiny ~ns → wide dense ~ms)
    // all profile in bounded time. Check the clock once per 256-call batch.
    let budget = Duration::from_secs(4);
    let t0 = Instant::now();
    let mut count = 0u64;
    while t0.elapsed() < budget {
        for _ in 0..256 {
            eng(black_box(&num), black_box(&den), &mut q, &mut r);
            black_box(&q);
            black_box(&r);
        }
        count += 256;
    }
    let el = t0.elapsed();
    black_box(&q);
    black_box(&r);
    eprintln!(
        "{sel}: n={n} calls={count} loop={:.3}s ({:.1} ns/call)",
        el.as_secs_f64(),
        el.as_nanos() as f64 / count as f64,
    );
}
