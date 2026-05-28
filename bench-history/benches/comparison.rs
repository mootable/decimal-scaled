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
//!   cargo bench --bench comparison -- _s19/
//! and the trailing `/` anchors the scale (`_s19/` matches `mul_D38_s19/t`
//! but NOT a hypothetical `_s190/`).
//!
//! Scale set — the cross-version CAP applies (NOT the clean per-tier
//! {0, S/4, S/2, 3S/4, S-1} the rest of the perf tools moved to). The
//! published v0.2.x / v0.3.x lines cap at SCALE <= 38: their value storage
//! is u128-mantissa-bounded (constructing `2` at SCALE s means `2*10^s`,
//! and 10^39 overflows the u128 intermediate / FromStr ceiling), so any
//! scale > 38 breaks the old-version cells. Because every version cell must
//! run the IDENTICAL harness (matching criterion group names for the
//! cross-version trend), the scale set is the intersection that parses on
//! ALL pinned versions — i.e. the largest standard 5-point set that fits the
//! <= 38 window, which is the D38 tier's {0, 9, 19, 28, 37}. We apply that
//! ONE set to all three benched widths (D38, D76, D307); we do NOT use D76's
//! {0,19,38,57,75} or D307's {0,76,153,230,306} because their high scales
//! overflow the old lines. This drops the old fixed scale 30 (per the perf-
//! tool standardisation) while staying within the cross-version window.
//! v0.4.0+ handle arbitrary SCALE, but the cap is set by the OLDEST cell.

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::{D38, D76, D307};
use std::hint::black_box;

// Cross-version-safe scale set (see module note — FromStr u128 ceiling on
// the old lines caps the shared window at SCALE <= 38). The D38 tier's
// 5-point set {0, S/4, S/2, 3S/4, S-1} = {0, 9, 19, 28, 37} is the largest
// standard 5-point set that fits the cap; applied to all benched widths so
// every version cell runs the identical harness.
const SCALES: &[usize] = &[0, 9, 19, 28, 37];

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
    width_at_scale!(c, "D38", D38, 9);
    width_at_scale!(c, "D38", D38, 19);
    width_at_scale!(c, "D38", D38, 28);
    width_at_scale!(c, "D38", D38, 37);
    width_at_scale!(c, "D76", D76, 0);
    width_at_scale!(c, "D76", D76, 9);
    width_at_scale!(c, "D76", D76, 19);
    width_at_scale!(c, "D76", D76, 28);
    width_at_scale!(c, "D76", D76, 37);
    width_at_scale!(c, "D307", D307, 0);
    width_at_scale!(c, "D307", D307, 9);
    width_at_scale!(c, "D307", D307, 19);
    width_at_scale!(c, "D307", D307, 28);
    width_at_scale!(c, "D307", D307, 37);
}

criterion_group!(benches, bench);
criterion_main!(benches);
