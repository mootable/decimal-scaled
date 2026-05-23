//! Side-by-side arithmetic comparison of two `decimal-scaled` versions.
//!
//! `branch` = the branch under test (the `decimal_scaled` path dep);
//! `prod` = the latest published release (the baseline). Both run the
//! same operators across every shared width, constructed via `FromStr` so
//! the harness depends only on the public surface common to both versions.
//! One Criterion run produces both columns; pair `arith/<W>/branch/<op>`
//! against `arith/<W>/prod/<op>`.
//!
//! `D9` is intentionally absent — it was removed in 0.5.0, so it has no
//! counterpart on the `branch` side.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

/// Bench the six by-value operators for one `Copy` decimal type under a
/// `new/` or `old/` sub-tag inside the given group.
macro_rules! ops {
    ($g:expr, $side:literal, $ty:ty) => {{
        let a: $ty = "1234.5".parse().unwrap();
        let b: $ty = "67.89".parse().unwrap();
        $g.bench_function(concat!($side, "/add"), |bn| bn.iter(|| black_box(a) + black_box(b)));
        $g.bench_function(concat!($side, "/sub"), |bn| bn.iter(|| black_box(a) - black_box(b)));
        $g.bench_function(concat!($side, "/mul"), |bn| bn.iter(|| black_box(a) * black_box(b)));
        $g.bench_function(concat!($side, "/div"), |bn| bn.iter(|| black_box(a) / black_box(b)));
        $g.bench_function(concat!($side, "/rem"), |bn| bn.iter(|| black_box(a) % black_box(b)));
        $g.bench_function(concat!($side, "/neg"), |bn| bn.iter(|| -black_box(a)));
    }};
}

/// Register one width: a Criterion group `arith/<W>` holding the `new/*`
/// and `old/*` rows for the two crate versions at the same SCALE.
macro_rules! width {
    ($c:expr, $w:literal, $scale:literal, $newmod:ident, $oldmod:ident) => {{
        let mut g = $c.benchmark_group(concat!("arith/", $w));
        ops!(g, "branch", ::decimal_scaled::$newmod<$scale>);
        ops!(g, "prod", ::prod::$oldmod<$scale>);
        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    // Scales kept <= 30 — proven to parse cleanly on every published
    // version (the bench-history harness uses D307<30>); width, not scale,
    // drives fixed-point arithmetic cost, so this still exercises each
    // storage tier's full limb width.
    width!(c, "D18", 9, D18, D18); // base storage (no feature)
    width!(c, "D38", 10, D38, D38); // base storage
    width!(c, "D57", 20, D57, D57); // wide
    width!(c, "D76", 20, D76, D76); // wide
    width!(c, "D115", 25, D115, D115); // wide
    width!(c, "D153", 25, D153, D153); // wide
    width!(c, "D230", 30, D230, D230); // wide
    width!(c, "D307", 30, D307, D307); // x-wide
    width!(c, "D462", 30, D462, D462); // x-wide
    width!(c, "D616", 30, D616, D616); // x-wide
    width!(c, "D924", 30, D924, D924); // xx-wide
    width!(c, "D1232", 30, D1232, D1232); // xx-wide
}

criterion_group!(benches, bench);
criterion_main!(benches);
