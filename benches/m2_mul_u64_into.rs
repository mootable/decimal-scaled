//! Micro-bench for `limbs_mul_u64_into<L, L+1>` — the single-u64
//! multiplier specialisation — against the equivalent
//! `limbs_mul_u64_fixed<L, 2·L>(a, [n, 0, ..., 0])` Knuth-shaped
//! path. The new primitive does `L` widening muls + `L` adds; the
//! `fixed` variant runs the `L²` outer-product loop with most
//! inner iterations short-circuited on `b[j] == 0`.

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
    ($g:ident, $label:literal, $L:expr, $LP1:expr, $D:expr) => {{
        let a: [u64; $L] = synthetic(7);
        let n: u64 = 0xA5A5_A5A5_5A5A_5A5Au64;
        let mut b: [u64; $L] = [0u64; $L];
        b[0] = n;
        let mut out_into = [0u64; $LP1];
        let mut out_fixed = [0u64; $D];

        $g.bench_function(concat!($label, "/into"), |bn| {
            bn.iter(|| {
                for slot in out_into.iter_mut() { *slot = 0; }
                decimal_scaled::__bench_internals::mul_u64_into::<$L, $LP1>(
                    black_box(&a),
                    black_box(n),
                    black_box(&mut out_into),
                );
            });
        });
        $g.bench_function(concat!($label, "/fixed_b_single"), |bn| {
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
    let mut g = c.benchmark_group("m2_mul_u64_into");
    g.sample_size(50);
    g.measurement_time(std::time::Duration::from_secs(3));

    pair!(g, "L2",   2,  3,  4);   // D38 storage (two-u64 split of i128)
    pair!(g, "L3",   3,  4,  6);   // D57 storage (192-bit)
    pair!(g, "L4",   4,  5,  8);   // D76 storage (256-bit)
    pair!(g, "L6",   6,  7,  12);  // D115 storage (384-bit)
    pair!(g, "L8",   8,  9,  16);  // D153 storage (512-bit)
    pair!(g, "L12",  12, 13, 24);  // D230 storage (768-bit)
    pair!(g, "L16",  16, 17, 32);  // D307 storage (1024-bit)
    pair!(g, "L24",  24, 25, 48);  // D462 storage (1536-bit)
    pair!(g, "L32",  32, 33, 64);  // D616 storage (2048-bit)

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
