//! Per-tier library_comparison bench for D57 (192-bit storage).
//! Run with:
//!   cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d57
//! See benches/lib_cmp_common.rs for the shared macros + helpers.

#[macro_use]
mod lib_cmp_common;

// Scale set dedup{0, 30, S/2, S-1} with S=57: {0, 28, 30, 56}.
new_tier_body!(D57, "192", [0, 28, 30, 56], 30);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
