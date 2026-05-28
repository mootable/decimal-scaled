//! Per-tier library_comparison bench for D115 (384-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d115

#[macro_use]
mod lib_cmp_common;

// Scale set {0, S/4, S/2, 3S/4, S-1} with S=115: {0, 28, 57, 86, 114}.
new_tier_body!(D115, "384", [0, 28, 57, 86, 114], 57);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
