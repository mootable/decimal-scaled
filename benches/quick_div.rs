//! Focused microbench for wide-tier div, used while iterating on
//! limbs_divmod_dispatch_u64 / BZ threshold / MG magic-table tuning.
//! ~30 s end-to-end so it fits the "<60s before kicking a full sweep"
//! discipline.

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::{D307, D616, D924, D1232};
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("quick_div");
    g.sample_size(30);
    g.measurement_time(std::time::Duration::from_secs(3));

    {
        let a = D307::<150>::from_int(2);
        let b = D307::<150>::from_int(1);
        g.bench_function("D307<150>/div", |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
        g.bench_function("D307<150>/mul", |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
    }
    {
        let a = D616::<308>::from_int(2);
        let b = D616::<308>::from_int(1);
        g.bench_function("D616<308>/div", |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
        g.bench_function("D616<308>/mul", |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
    }
    {
        let a = D924::<461>::from_int(2);
        let b = D924::<461>::from_int(1);
        g.bench_function("D924<461>/div", |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
        g.bench_function("D924<461>/mul", |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
    }
    {
        let a = D1232::<616>::from_int(2);
        let b = D1232::<616>::from_int(1);
        g.bench_function("D1232<616>/div", |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
        g.bench_function("D1232<616>/mul", |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
    }

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
