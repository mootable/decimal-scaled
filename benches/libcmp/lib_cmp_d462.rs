//! Per-tier library_comparison bench for D462 (1536-bit, x-wide).
//! Run: cargo bench --features "x-wide xx-wide" --bench lib_cmp_d462

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D462, "1536", 230, 461);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
