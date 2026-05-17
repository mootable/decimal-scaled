//! Per-tier library_comparison bench for D923 (3072-bit, xx-wide).
//! Run: cargo bench --features "xx-wide" --bench lib_cmp_d923

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D923, "3072", 461, 923);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
