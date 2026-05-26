//! Like-for-like cross-version benchmark, fanned out over (width × scale).
//!
//! Same Criterion harness, same input distributions, same operations —
//! the only thing that changes across CI cells is the `decimal-scaled`
//! dependency version in `../Cargo.toml`. The version axis is the cell
//! axis (one compiled exe per version); WITHIN a version the harness now
//! fans out over a per-width scale set so a scale-dependent shift shows up
//! in the cross-version trend, not just one canonical scale per width.
//!
//! Group naming mirrors `bench-compare`: each benched id is
//! `<op>_<W>_s<scale>/t` (the lone `t` function id is the version-neutral
//! row — the version is the artifact the cell uploads). A single scale is
//! therefore selectable with a criterion name-filter, e.g.
//!   cargo bench --bench comparison -- _s30/
//! and the trailing `/` anchors the scale (`_s30/` matches `mul_D38_s30/t`
//! but NOT a hypothetical `_s300/`).
//!
//! Scale set: the published v0.2.x / v0.3.x lines have a u128 ceiling in
//! FromStr (10^39 overflows the intermediate), so a string like "1234.5"
//! only parses up to SCALE <= 38. For cross-version like-for-like we stay
//! within that window: {0, 10, 30} per width (30 kept as the fixed
//! reference point used by prior runs). v0.4.0+ handle arbitrary SCALE but
//! we cap so every version cell runs the identical harness.

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::{D38, D76, D307};
use std::hint::black_box;

// Cross-version-safe scale set (see module note — FromStr u128 ceiling on
// the old lines caps the shared window). 30 is the fixed reference scale.
const SCALES: &[usize] = &[0, 10, 30];

/// Register one width's full op set at one SCALE. The SCALE is a const
/// generic, so the body monomorphises per scale; the `$w`/`$scale` literal
/// pair names the group `<op>_<W>_s<scale>` and the lone `t` function id is
/// the version-neutral row.
macro_rules! width_at_scale {
    ($c:expr, $w:literal, $ty:ident, $scale:literal) => {{
        // Operands kept to a single integer digit so they parse at every
        // scale in the set (mirrors bench-compare's S-1 narrow-room rule).
        let a = $ty::<$scale>::try_from(2).unwrap();
        let b = $ty::<$scale>::try_from(1).unwrap();

        bench_one!($c, "add", $w, $scale, |bn| {
            bn.iter(|| black_box(a) + black_box(b))
        });
        bench_one!($c, "mul", $w, $scale, |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
        bench_one!($c, "div", $w, $scale, |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
        bench_one!($c, "sqrt", $w, $scale, |bn| bn.iter(|| black_box(a).sqrt_strict()));
        bench_one!($c, "ln", $w, $scale, |bn| bn.iter(|| black_box(a).ln_strict()));
        bench_one!($c, "sin", $w, $scale, |bn| bn.iter(|| black_box(a).sin_strict()));
    }};
}

/// One Criterion group `<op>_<W>_s<scale>` with a single `t` row.
macro_rules! bench_one {
    ($c:expr, $fn:literal, $w:literal, $scale:literal, $body:expr) => {{
        let mut g = $c.benchmark_group(concat!($fn, "_", $w, "_s", $scale));
        g.bench_function("t", $body);
        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    // The macro needs literal scales for the group name + const generic, so
    // unroll the (width × scale) grid explicitly. SCALES documents the set.
    let _ = SCALES;
    width_at_scale!(c, "D38", D38, 0);
    width_at_scale!(c, "D38", D38, 10);
    width_at_scale!(c, "D38", D38, 30);
    width_at_scale!(c, "D76", D76, 0);
    width_at_scale!(c, "D76", D76, 10);
    width_at_scale!(c, "D76", D76, 30);
    width_at_scale!(c, "D307", D307, 0);
    width_at_scale!(c, "D307", D307, 10);
    width_at_scale!(c, "D307", D307, 30);
}

criterion_group!(benches, bench);
criterion_main!(benches);
