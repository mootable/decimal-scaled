//! Per-tier library_comparison bench for D230 (768-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d230

#[macro_use]
mod lib_cmp_common;

// Scale set {0, S/4, S/2, 3S/4, S-1} with S=230: {0, 57, 115, 172, 229}.
new_tier_body!(D230, "768", [0, 57, 115, 172, 229], 115);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
