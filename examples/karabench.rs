//! Direct comparison: `mul_slice` (schoolbook) vs `mul_karatsuba_forced`
//! (non-allocating Karatsuba) at n = 8, 16, 32 u64 limbs.
//!
//! Run with:
//!     cargo run --release --example karabench --features bench-alt

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::__bench_internals as bench;

const ITERS: u32 = 1_000_000;

/// Karatsuba base-case floor (the recursion's termination width); forces the
/// kernel to actually recurse at these sub-production widths.
const THRESHOLD: usize = 4;

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
    let a = [0x9d3c_a7f1_2b6e_84d5u64; N];
    let b = [0xc7e2_5a91_38f0_b46cu64; N];
    let mut out_school = vec![0u64; 2 * N];
    let mut out_kara = vec![0u64; 2 * N];

    println!("== n = {} limbs ({}) ==", N, label);
    time(&format!("  mul_slice            (schoolbook) n={}", N), || {
        for v in out_school.iter_mut() {
            *v = 0;
        }
        bench::mul_slice(black_box(&a), black_box(&b), black_box(&mut out_school));
    });
    time(&format!("  mul_karatsuba_forced (kara)       n={}", N), || {
        bench::mul_karatsuba_forced(
            black_box(&a),
            black_box(&b),
            black_box(&mut out_kara),
            THRESHOLD,
        );
    });
    assert_eq!(out_school, out_kara, "mul kernels disagree at n={}", N);
    println!();
}

fn main() {
    run::<8>("Int512");
    run::<16>("Int1024");
    run::<32>("Int2048");
}
