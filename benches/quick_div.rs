//! Focused microbench for wide-tier div, used while iterating on
//! limbs_divmod_dispatch_u64 / BZ threshold / MG magic-table tuning.
//! ~30 s end-to-end so it fits the "<60s before kicking a full sweep"
//! discipline.

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::{D307, D615, D923, D1231};

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("quick_div");
    g.sample_size(30);
    g.measurement_time(std::time::Duration::from_secs(3));

    {
        let a = D307::<150>::from_int(2);
        let b = D307::<150>::from_int(1);
        g.bench_function("D307<150>/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("D307<150>/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    }
    {
        let a = D615::<308>::from_int(2);
        let b = D615::<308>::from_int(1);
        g.bench_function("D615<308>/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("D615<308>/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    }
    {
        let a = D923::<461>::from_int(2);
        let b = D923::<461>::from_int(1);
        g.bench_function("D923<461>/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("D923<461>/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    }
    {
        let a = D1231::<616>::from_int(2);
        let b = D1231::<616>::from_int(1);
        g.bench_function("D1231<616>/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("D1231<616>/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    }

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
