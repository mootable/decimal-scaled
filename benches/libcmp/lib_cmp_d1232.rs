//! Per-tier library_comparison bench for D1232 (4096-bit, xx-wide).
//! Run: cargo bench --features "xx-wide" --bench lib_cmp_d1232

#[macro_use]
mod lib_cmp_common;

// Scale set {0, S/4, S/2, 3S/4, S-1} with S=1232: {0, 308, 616, 924, 1231}.
new_tier_body!(D1232, "4096", [0, 308, 616, 924, 1231], 616);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
