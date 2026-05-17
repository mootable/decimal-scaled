//! Direct comparison: `limbs_mul` (schoolbook) vs `limbs_mul_fast`
//! (Karatsuba above threshold) at n = 4, 8, 16, 32 limbs of u128.
//!
//! Run with:
//!     cargo run --release --example karabench --features wide,x-wide

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::__bench_internals as bench;

const ITERS: u32 = 1_000_000;

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
    println!("  {:<48} {:>10.1} ns/op", label, per);
}

fn run<const N: usize>(label: &str) {
    let a = [u128::from_le_bytes([0x9d; 16]); N];
    let b = [u128::from_le_bytes([0xc7; 16]); N];
    let mut out_school = vec![0u128; 2 * N];
    let mut out_fast = vec![0u128; 2 * N];

    println!("== n = {} limbs ({}) ==", N, label);
    time(&format!("  limbs_mul       (schoolbook) n={}", N), || {
        for v in out_school.iter_mut() {
            *v = 0;
        }
        bench::limbs_mul(black_box(&a), black_box(&b), black_box(&mut out_school));
    });
    time(&format!("  limbs_mul_fast  (dispatcher) n={}", N), || {
        for v in out_fast.iter_mut() {
            *v = 0;
        }
        bench::limbs_mul_fast(black_box(&a), black_box(&b), black_box(&mut out_fast));
    });
    assert_eq!(out_school, out_fast, "mul kernels disagree at n={}", N);
    println!();
}

fn main() {
    run::<8>("Int1024");
    run::<16>("Int2048");
    run::<32>("Int4096");
}
