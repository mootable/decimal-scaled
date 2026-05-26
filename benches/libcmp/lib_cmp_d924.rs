//! Per-tier library_comparison bench for D924 (3072-bit, xx-wide).
//! Run: cargo bench --features "xx-wide" --bench lib_cmp_d924

#[macro_use]
mod lib_cmp_common;

// Scale set dedup{0, 30, S/2, S-1} with S=924: {0, 30, 462, 923}.
new_tier_body!(D924, "3072", [0, 30, 462, 923], 30);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
