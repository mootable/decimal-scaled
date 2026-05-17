//! Per-tier library_comparison bench for D615 (2048-bit, x-wide).
//! Run: cargo bench --features "x-wide xx-wide" --bench lib_cmp_d615

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D615, "2048", 308, 615);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
