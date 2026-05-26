//! Per-tier library_comparison bench for D462 (1536-bit, x-wide).
//! Run: cargo bench --features "x-wide xx-wide" --bench lib_cmp_d462

#[macro_use]
mod lib_cmp_common;

// Scale set dedup{0, 30, S/2, S-1} with S=462: {0, 30, 231, 461}.
new_tier_body!(D462, "1536", [0, 30, 231, 461], 30);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
