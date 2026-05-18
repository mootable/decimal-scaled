//! Per-tier library_comparison bench for D924 (3072-bit, xx-wide).
//! Run: cargo bench --features "xx-wide" --bench lib_cmp_d924

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D924, "3072", 461, 923);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
