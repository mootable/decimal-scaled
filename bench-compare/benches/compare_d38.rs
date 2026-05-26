//! Per-width branch-vs-prod compare bench for D38, fanned out over SCALE.
//! Run: cargo bench --bench compare_d38            (all scales for this width)
//!      cargo bench --bench compare_d38 -- s30     (just scale 30)
//!
//! Branch (`decimal_scaled::D38<S>`) vs prod (`prod::D38<S>`) across the
//! shared public function surface, at each scale in this tier's scale set.
//! See `compare_common.rs` for the macros + coverage notes.
//!
//! Scale set = dedup{0, S/2, S-1, 30 if 30 <= S-1} with S=38 (D38's capacity):
//! {0, 19, 37, 30}. Each scale becomes a criterion group suffix `_s<scale>`,
//! selectable via `-- s<scale>`.
//!
//! D38 is the one tier where `hypot`'s default public method pairs cleanly
//! branch-vs-prod under `strict`, so this width additionally benches it at
//! every scale.

#[macro_use]
mod compare_common;

use criterion::{Criterion, criterion_group, criterion_main};

/// Branch + prod surface (incl. the D38-only `hypot`) at one scale.
macro_rules! d38_scale {
    ($c:expr, $scale:literal) => {{
        funcs!($c, "D38", $scale, "branch", ::decimal_scaled::D38<$scale>);
        funcs!($c, "D38", $scale, "prod", ::prod::D38<$scale>);
        // `hypot`'s default public method is D38-only under `strict` in both
        // versions; pair branch-vs-prod at that one tier, every scale.
        hypot_d38!($c, "D38", $scale, "branch", ::decimal_scaled::D38<$scale>);
        hypot_d38!($c, "D38", $scale, "prod", ::prod::D38<$scale>);
    }};
}

fn bench(c: &mut Criterion) {
    d38_scale!(c, 0);
    d38_scale!(c, 19);
    d38_scale!(c, 30);
    d38_scale!(c, 37);
}

criterion_group!(benches, bench);
criterion_main!(benches);
