//! Per-tier library_comparison bench for D1232 (4096-bit, xx-wide).
//! Run: cargo bench --features "xx-wide" --bench lib_cmp_d1232

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D1232, "4096", 616, 1231);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
