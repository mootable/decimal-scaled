//! Per-tier library_comparison bench for D230 (768-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d230

#[macro_use]
mod lib_cmp_common;

new_tier_body!(D230, "768", 115, 230);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
