//! Per-tier library_comparison bench for D230 (768-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d230

#[macro_use]
mod lib_cmp_common;

// Scale set dedup{0, 30, S/2, S-1} with S=230: {0, 30, 115, 229}.
new_tier_body!(D230, "768", [0, 30, 115, 229], 30);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
