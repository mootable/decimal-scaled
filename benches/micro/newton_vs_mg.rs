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

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::__bench_internals::newton_vs_mg::{
    NewtonReciprocal, b6144, d1232, d307, d462, d616, d924,
};
use std::hint::black_box;

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
            b.iter(|| {
                black_box($tier::newton(
                    black_box(n),
                    black_box($scale),
                    black_box(&table),
                ))
            })
        });

        // u128-packed Newton -- full kernel on u128 limbs against the
        // cached u128-packed `r`/`pow_scale`. Integrated-call gate per
        // the owner's mandate (the prior standalone-mul A/B was a TRAP).
        g.bench_function("newton_u128", |b| {
            b.iter(|| {
                black_box($tier::newton_u128(
                    black_box(n),
                    black_box($scale),
                    black_box(&table),
                ))
            })
        });

        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    // D307 SCALE 150 (design) and 307 (max) — 1024-bit storage.
    bench_cell!(c, d307, "D307_s38", 38, 16, 6);
    bench_cell!(c, d307, "D307_s150", 150, 16, 6);
    bench_cell!(c, d307, "D307_s307", 307, 16, 6);

    // 1536-bit (Int<24>) — D462 storage AND D230's Work integer. The bbc
    // cells `exp_D230_s{172,229}` rescale at this width: D230 working scale
    // `w = SCALE + GUARD(30)` lands at 202 and 259 respectively. Coarse
    // 5-point sweep across the D462 scale range `{0, 115, 230, 346, 461}`
    // plus the two D230 work-scale anchors (202, 259) so the bisection has
    // direct evidence at the bbc cells. (`Int<24>` carries `MAX_R_U64`
    // headroom — table fits.)
    bench_cell!(c, d462, "B1536_s38", 38, 24, 10);
    bench_cell!(c, d462, "B1536_s115", 115, 24, 10);
    // Bisection between s115 (MG wins) and s202 (Newton wins) to localise
    // the crossover (continuous win-region per Constitution rule 6 + the
    // Class I no-point-snap rule).
    bench_cell!(c, d462, "B1536_s140", 140, 24, 10);
    bench_cell!(c, d462, "B1536_s160", 160, 24, 10);
    bench_cell!(c, d462, "B1536_s180", 180, 24, 10);
    bench_cell!(c, d462, "B1536_s190", 190, 24, 10);
    bench_cell!(c, d462, "B1536_s195", 195, 24, 10);
    bench_cell!(c, d462, "B1536_s200", 200, 24, 10);
    bench_cell!(c, d462, "B1536_s202", 202, 24, 10);
    bench_cell!(c, d462, "B1536_s230", 230, 24, 10);
    bench_cell!(c, d462, "B1536_s259", 259, 24, 10);
    bench_cell!(c, d462, "B1536_s346", 346, 24, 10);
    bench_cell!(c, d462, "B1536_s461", 461, 24, 10);

    // D616 SCALE 308 (design) and 616 (max) — 2048-bit storage.
    bench_cell!(c, d616, "D616_s38", 38, 32, 14);
    bench_cell!(c, d616, "D616_s100", 100, 32, 14);
    bench_cell!(c, d616, "D616_s200", 200, 32, 14);
    bench_cell!(c, d616, "D616_s308", 308, 32, 14);
    bench_cell!(c, d616, "D616_s460", 460, 32, 14);
    bench_cell!(c, d616, "D616_s616", 616, 32, 14);

    // D924 SCALE 460 (design) and 924 (max) — 3072-bit storage.
    bench_cell!(c, d924, "D924_s38", 38, 48, 22);
    bench_cell!(c, d924, "D924_s200", 200, 48, 22);
    bench_cell!(c, d924, "D924_s460", 460, 48, 22);
    bench_cell!(c, d924, "D924_s924", 924, 48, 22);

    // D1232 SCALE 615 (design) and 1231 (max) — 4096-bit storage.
    bench_cell!(c, d1232, "D1232_s38", 38, 64, 30);
    bench_cell!(c, d1232, "D1232_s200", 200, 64, 30);
    bench_cell!(c, d1232, "D1232_s400", 400, 64, 30);
    bench_cell!(c, d1232, "D1232_s615", 615, 64, 30);
    bench_cell!(c, d1232, "D1232_s900", 900, 64, 30);
    bench_cell!(c, d1232, "D1232_s1231", 1231, 64, 30);

    // ── Wider width (audit 2026-05-28) ─────────────────────────────
    //
    // 6144-bit (Int<96>): D230 Wexp / D924 Work. Per the all-widths
    // rule (axis = width × scale-band), a representative 5+ sweep
    // covers the D924 transcendental w_prime range (AGM tops out at
    // `2·SCALE_max + 4 = 1850` for D924 strict_agm — see the buffer-
    // sizing block + `newton_wins` doc in `newton_reciprocal.rs`).
    //
    // 8192 / 12288 / 16384 / 32768 are NOT benched here: the Newton
    // precompute's `2^k / 10^scale` numerator at the AGM-widened
    // scales exceeds the routed `div_knuth` build-max scratch
    // (`MAX_SINGLE_LIMBS = 258`). The sibling-agent atanh-diagnosis
    // integrated bench also reported MG winning 5–58× at
    // Int<192>/Int<256> low-scale, so even with extended scratch the
    // structural picture suggests MG is the right engine there.

    bench_cell!(c, b6144, "B6144_s38", 38, 96, 44);
    bench_cell!(c, b6144, "B6144_s115", 115, 96, 44);
    bench_cell!(c, b6144, "B6144_s200", 200, 96, 44);
    bench_cell!(c, b6144, "B6144_s400", 400, 96, 44);
    bench_cell!(c, b6144, "B6144_s600", 600, 96, 44);
    bench_cell!(c, b6144, "B6144_s800", 800, 96, 44);
    bench_cell!(c, b6144, "B6144_s953", 953, 96, 44);
    // AGM-band cells — D924 strict_agm reaches `w_prime ≈ 1850` at
    // SCALE 923; the 6144 + AGM cell is the hot path for D924
    // ln/exp/asinh/atanh/acosh _strict_agm.
    bench_cell!(c, b6144, "B6144_s1234", 1234, 96, 44);
    bench_cell!(c, b6144, "B6144_s1500", 1500, 96, 44);
    bench_cell!(c, b6144, "B6144_s1850", 1850, 96, 44);
}

criterion_group!(benches, bench);
criterion_main!(benches);
