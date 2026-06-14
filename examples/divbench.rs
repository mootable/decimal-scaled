//! Micro-bench: which kernel is slow inside `D76::<35> / D76::<35>`?
//!
//! Times the divide path layer by layer so we can see whether Knuth is
//! the bottleneck or whether the cost is sitting somewhere else
//! (`widen_mul`, `resize`, the double-divmod inside `round_with_mode_wide!`).
//!
//! Run with:
//!     cargo run --release --example divbench --features wide,x-wide

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::{D38, D76, D153, D307};

const ITERS: u32 = 1_000_000;

fn time<F: FnMut()>(label: &str, mut f: F) {
    // Warmup.
    for _ in 0..10_000 {
        f();
    }
    let start = Instant::now();
    for _ in 0..ITERS {
        f();
    }
    let elapsed = start.elapsed();
    let per = elapsed.as_nanos() as f64 / ITERS as f64;
    println!("  {:<48} {:>10.1} ns/op", label, per);
}

fn main() {
    println!("== D38<19> / D38<19> baseline ==");
    {
        let a = D38::<19>::try_from(2_i64).unwrap();
        let b = D38::<19>::try_from(1_i64).unwrap();
        time("D38::<19> a / b", || {
            black_box(black_box(a) / black_box(b));
        });
    }

    println!();
    println!("== D76<35> / D76<35> headline ==");
    {
        let a = D76::<35>::try_from(2_i64).unwrap();
        let b = D76::<35>::try_from(1_i64).unwrap();
        time("D76::<35> a / b (overall)", || {
            black_box(black_box(a) / black_box(b));
        });
    }

    println!();
    println!("== D76<76> / D76<76> wide divisor ==");
    {
        let a = D76::<76>::try_from(2_i64).unwrap();
        let b = D76::<76>::try_from(1_i64).unwrap();
        time("D76::<76> a / b (overall)", || {
            black_box(black_box(a) / black_box(b));
        });
    }

    println!();
    println!("== D153<75> mul + div ==");
    {
        let a = D153::<75>::try_from(2_i64).unwrap();
        let b = D153::<75>::try_from(1_i64).unwrap();
        time("D153::<75> a * b", || {
            black_box(black_box(a) * black_box(b));
        });
        time("D153::<75> a / b", || {
            black_box(black_box(a) / black_box(b));
        });
    }
    println!();
    println!("== D153<153> mul + div ==");
    {
        let a = D153::<153>::try_from(2_i64).unwrap();
        let b = D153::<153>::try_from(1_i64).unwrap();
        time("D153::<153> a * b", || {
            black_box(black_box(a) * black_box(b));
        });
        time("D153::<153> a / b", || {
            black_box(black_box(a) / black_box(b));
        });
    }
    println!();
    println!("== D307<150> + D307<307> mul + div ==");
    {
        let a = D307::<150>::try_from(2_i64).unwrap();
        let b = D307::<150>::try_from(1_i64).unwrap();
        time("D307::<150> a * b", || {
            black_box(black_box(a) * black_box(b));
        });
        time("D307::<150> a / b", || {
            black_box(black_box(a) / black_box(b));
        });
    }
    {
        let a = D307::<307>::try_from(2_i64).unwrap();
        let b = D307::<307>::try_from(1_i64).unwrap();
        time("D307::<307> a * b", || {
            black_box(black_box(a) * black_box(b));
        });
        time("D307::<307> a / b", || {
            black_box(black_box(a) / black_box(b));
        });
    }

    println!();
    println!("== D38<38> / D38<38> single-limb-too-wide-for-u64 ==");
    {
        let a = D38::<38>::from_bits(decimal_scaled::Int::<2>::try_from(170_000_000_000_000_000_000_000_000_000_000_000_000_i128 / 2).unwrap());
        let b = D38::<38>::from_bits(decimal_scaled::Int::<2>::try_from(100_000_000_000_000_000_000_000_000_000_000_000_i128).unwrap());
        time("D38::<38> a / b (overall)", || {
            black_box(black_box(a) / black_box(b));
        });
    }
}
