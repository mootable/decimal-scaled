//! Micro-bench: wide-tier `sqrt_strict` per tier and midpoint scale.
//!
//! Run with:
//!     cargo run --release --example sqrtbench --features wide,x-wide

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::{D38, D76, D153, D307};

const ITERS: u32 = 200_000;

fn time<F: FnMut()>(label: &str, mut f: F) {
    for _ in 0..2_000 {
        f();
    }
    let start = Instant::now();
    for _ in 0..ITERS {
        f();
    }
    let elapsed = start.elapsed();
    let per = elapsed.as_nanos() as f64 / ITERS as f64;
    println!("  {:<40} {:>10.1} ns/op", label, per);
}

fn main() {
    println!("== D38<19>::sqrt(2) — baseline ==");
    {
        let a = D38::<19>::try_from(2_i64).unwrap();
        time("D38<19>::sqrt_strict(2)", || {
            black_box(black_box(a).sqrt_strict());
        });
    }

    println!();
    println!("== D76<35>::sqrt(2) ==");
    {
        let a = D76::<35>::try_from(2_i64).unwrap();
        time("D76<35>::sqrt_strict(2)", || {
            black_box(black_box(a).sqrt_strict());
        });
    }

    println!();
    println!("== D76<76>::sqrt(2) ==");
    {
        let a = D76::<76>::try_from(2_i64).unwrap();
        time("D76<76>::sqrt_strict(2)", || {
            black_box(black_box(a).sqrt_strict());
        });
    }

    println!();
    println!("== D153<75>::sqrt(2) ==");
    {
        let a = D153::<75>::try_from(2_i64).unwrap();
        time("D153<75>::sqrt_strict(2)", || {
            black_box(black_box(a).sqrt_strict());
        });
    }

    println!();
    println!("== D153<153>::sqrt(2) ==");
    {
        let a = D153::<153>::try_from(2_i64).unwrap();
        time("D153<153>::sqrt_strict(2)", || {
            black_box(black_box(a).sqrt_strict());
        });
    }

    println!();
    println!("== D307<150>::sqrt(2) ==");
    {
        let a = D307::<150>::try_from(2_i64).unwrap();
        time("D307<150>::sqrt_strict(2)", || {
            black_box(black_box(a).sqrt_strict());
        });
    }

    println!();
    println!("== D307<307>::sqrt(2) ==");
    {
        let a = D307::<307>::try_from(2_i64).unwrap();
        time("D307<307>::sqrt_strict(2)", || {
            black_box(black_box(a).sqrt_strict());
        });
    }
}
