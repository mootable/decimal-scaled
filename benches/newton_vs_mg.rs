//! Head-to-head micro-bench: Newton-Raphson reciprocal divide vs.
//! Möller-Granlund chain divide for `n / 10^SCALE` at the wide tiers.
//!
//! Setup cost (per `(SCALE, width)`): MG chain has no setup beyond
//! looking up the per-stage 38-entry magic table; Newton has a one-shot
//! reciprocal computation. The table is computed once outside the
//! measured loop so the per-call number is apples-to-apples with MG.
//!
//! Filter via the Criterion positional argument; the per-cell group
//! prefix is `D{tier}_s{scale}/{algo}`.

#![cfg(all(feature = "bench-alt", feature = "x-wide", feature = "xx-wide"))]

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::__bench_internals::newton_vs_mg::{NewtonReciprocal, d307, d616, d924, d1232};

macro_rules! bench_cell {
    ($c:ident, $tier:ident, $label:literal, $scale:expr, $width_limbs:expr, $top_limb:expr) => {{
        let n = $tier::build_numerator($top_limb);
        let table = NewtonReciprocal::precompute($scale, $width_limbs);
        let mut g = $c.benchmark_group($label);
        g.sample_size(20);
        g.measurement_time(std::time::Duration::from_secs(5));

        // MG chain handles scale > 38; for scale ≤ 38 use the single-pass MG.
        if $scale > 38 {
            g.bench_function("mg_chain", |b| {
                b.iter(|| black_box($tier::mg_chain(black_box(n), black_box($scale))))
            });
        } else {
            g.bench_function("mg_single", |b| {
                b.iter(|| black_box($tier::mg_single(black_box(n), black_box($scale))))
            });
        }

        g.bench_function("newton", |b| {
            b.iter(|| black_box($tier::newton(black_box(n), black_box($scale), black_box(&table))))
        });

        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    // D307 SCALE 150 (design) and 307 (max) — 1024-bit storage.
    bench_cell!(c, d307, "D307_s38",  38,  8, 6);
    bench_cell!(c, d307, "D307_s150", 150, 8, 6);
    bench_cell!(c, d307, "D307_s307", 307, 8, 6);

    // D616 SCALE 308 (design) and 616 (max) — 2048-bit storage.
    bench_cell!(c, d616, "D616_s38",  38,  16, 14);
    bench_cell!(c, d616, "D616_s100", 100, 16, 14);
    bench_cell!(c, d616, "D616_s200", 200, 16, 14);
    bench_cell!(c, d616, "D616_s308", 308, 16, 14);
    bench_cell!(c, d616, "D616_s460", 460, 16, 14);
    bench_cell!(c, d616, "D616_s616", 616, 16, 14);

    // D924 SCALE 460 (design) and 924 (max) — 3072-bit storage.
    bench_cell!(c, d924, "D924_s38",  38,  24, 22);
    bench_cell!(c, d924, "D924_s200", 200, 24, 22);
    bench_cell!(c, d924, "D924_s460", 460, 24, 22);
    bench_cell!(c, d924, "D924_s924", 924, 24, 22);

    // D1232 SCALE 615 (design) and 1231 (max) — 4096-bit storage.
    bench_cell!(c, d1232, "D1232_s38",   38,   32, 30);
    bench_cell!(c, d1232, "D1232_s200",  200,  32, 30);
    bench_cell!(c, d1232, "D1232_s400",  400,  32, 30);
    bench_cell!(c, d1232, "D1232_s615",  615,  32, 30);
    bench_cell!(c, d1232, "D1232_s900",  900,  32, 30);
    bench_cell!(c, d1232, "D1232_s1231", 1231, 32, 30);
}

criterion_group!(benches, bench);
criterion_main!(benches);
