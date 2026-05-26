//! Per-tier library_comparison bench for D616 (2048-bit, x-wide).
//! Run: cargo bench --features "x-wide xx-wide" --bench lib_cmp_d616

#[macro_use]
mod lib_cmp_common;

// Scale set dedup{0, 30, S/2, S-1} with S=616: {0, 30, 308, 615}.
new_tier_body!(D616, "2048", [0, 30, 308, 615], 30);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
