//! M2 gate for `limbs_mul_u64_fixed<const L, const D>` — per-tier
//! specialised schoolbook mul where loop bounds are compile-time
//! constants so LLVM can unroll. Times against the slice variant
//! `limbs_mul_u64` at every wide-tier operand size.

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

fn synthetic<const N: usize>(seed: u64) -> [u64; N] {
    let mut a = [0u64; N];
    for i in 0..N {
        a[i] = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15u64).wrapping_add(i as u64 * 0x1357_9BDFu64);
    }
    a
}

macro_rules! pair {
    ($g:ident, $label:literal, $L:expr, $D:expr) => {{
        let a: [u64; $L] = synthetic(7);
        let b: [u64; $L] = synthetic(13);
        let mut out_slice = vec![0u64; $D];
        let mut out_fixed = [0u64; $D];

        $g.bench_function(concat!($label, "/slice"), |bn| {
            bn.iter(|| {
                for slot in out_slice.iter_mut() { *slot = 0; }
                decimal_scaled::__bench_internals::mul_slice(
                    black_box(&a),
                    black_box(&b),
                    black_box(&mut out_slice),
                );
            });
        });
        $g.bench_function(concat!($label, "/fixed"), |bn| {
            bn.iter(|| {
                for slot in out_fixed.iter_mut() { *slot = 0; }
                decimal_scaled::__bench_internals::mul_fixed::<$L, $D>(
                    black_box(&a),
                    black_box(&b),
                    black_box(&mut out_fixed),
                );
            });
        });
    }};
}

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("m2_mul_fixed");
    g.sample_size(50);
    g.measurement_time(std::time::Duration::from_secs(3));

    pair!(g, "L4",   4,  8);   // D76 storage
    pair!(g, "L8",   8,  16);  // D153 storage
    pair!(g, "L16",  16, 32);  // D307 storage
    pair!(g, "L24",  24, 48);  // D461 storage
    pair!(g, "L32",  32, 64);  // D615 storage
    pair!(g, "L48",  48, 96);  // D923 storage
    pair!(g, "L64",  64, 128); // D1231 storage

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
