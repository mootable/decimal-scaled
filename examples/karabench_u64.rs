//! Cross-tier u64 multiply bench: non-allocating Karatsuba vs schoolbook
//! at the wide-tier storage widths (D462 = 24, D616 = 32, D924 = 48,
//! D1232 = 64 u64 limbs). One kernel lifting every wide tier.
//!
//! Run with:
//!     cargo run --release --features wide,x-wide,xx-wide,bench-alt \
//!         --example karabench_u64

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::__bench_internals as bench;

const ITERS: u32 = 1_000_000;

fn time<F: FnMut()>(mut f: F) -> f64 {
    for _ in 0..2_000 {
        f();
    }
    let start = Instant::now();
    for _ in 0..ITERS {
        f();
    }
    start.elapsed().as_nanos() as f64 / ITERS as f64
}

fn run<const N: usize>(label: &str, th: usize) {
    let a = [0x9d3c_a7f1_2b6e_84d5u64; N];
    let b = [0xc7e2_5a91_38f0_b46cu64; N];
    let mut out_school = vec![0u64; 2 * N];
    let mut out_kara = vec![0u64; 2 * N];

    let school = time(|| {
        for v in out_school.iter_mut() {
            *v = 0;
        }
        bench::mul_slice(black_box(&a), black_box(&b), black_box(&mut out_school));
    });
    // Force Karatsuba to recurse with base case `th` so the kernel
    // actually engages (multiple levels when th << N).
    let kara = time(|| {
        bench::mul_karatsuba_forced(black_box(&a), black_box(&b), black_box(&mut out_kara), th);
    });
    assert_eq!(out_school, out_kara, "kernels disagree at n={N}");
    let delta = (kara - school) / school * 100.0;
    println!(
        "{label:>7}  L={N:>3}  th={th:>3}  school {school:>8.1}  kara {kara:>8.1}  delta {delta:>+6.1}%",
    );
}

fn main() {
    println!("u64 wide-tier multiply: schoolbook vs non-alloc Karatsuba");
    println!("(threshold forced to 16-limb base case so Karatsuba engages)");
    // Two passes to expose run-to-run noise on the unpinned machine.
    for pass in 0..2 {
        println!("-- pass {pass} --");
        run::<24>("D462", 16);
        run::<32>("D616", 16);
        run::<48>("D924", 16);
        run::<64>("D1232", 16);
    }
}
