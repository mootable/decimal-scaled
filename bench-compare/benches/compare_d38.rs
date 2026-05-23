//! Per-width branch-vs-prod compare bench for D38.
//! Run: cargo bench --bench compare_d38
//!
//! Branch (`decimal_scaled::D38<10>`) vs prod (`prod::D38<10>`) across the
//! shared public function surface. See `compare_common.rs` for the macros
//! + coverage notes.
//!
//! D38 is the one tier where `hypot`'s default public method pairs cleanly
//! branch-vs-prod under `strict`, so this width additionally benches it.

#[macro_use]
mod compare_common;

use criterion::{Criterion, criterion_group, criterion_main};

fn bench(c: &mut Criterion) {
    funcs!(c, "D38", "branch", ::decimal_scaled::D38<10>);
    funcs!(c, "D38", "prod", ::prod::D38<10>);

    // `hypot`'s default public method is D38-only under `strict` in both
    // versions; pair branch-vs-prod at that one tier.
    hypot_d38!(c, "D38", "branch", ::decimal_scaled::D38<10>);
    hypot_d38!(c, "D38", "prod", ::prod::D38<10>);
}

criterion_group!(benches, bench);
criterion_main!(benches);
