//! Per-tier library_comparison bench for D57 (192-bit storage).
//! Run with:
//!   cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d57
//! See benches/lib_cmp_common.rs for the shared macros + helpers.

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D57, "192", 28, 56);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
