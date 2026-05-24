//! Per-tier library_comparison bench for D115 (384-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d115

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D115, "384", 57, 114);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
