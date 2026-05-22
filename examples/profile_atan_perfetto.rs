//! Section-by-section timing of `D38::<19>::atan_strict` via
//! tracing-chrome. Writes `trace/atan_perfetto.json` for Perfetto.
//!
//! Build + run:
//!   cargo run --release --features wide,x-wide,xx-wide,macros,perf-trace --example profile_atan_perfetto

use core::hint::black_box;
use decimal_scaled::D38;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

type Work = D38<19>;

fn main() {
    let iters: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);

    std::fs::create_dir_all("trace").expect("create trace/");
    let (chrome_layer, _guard) = ChromeLayerBuilder::new()
        .file("trace/atan_perfetto.json")
        .include_args(false)
        .build();
    tracing_subscriber::registry().with(chrome_layer).init();

    let mut acc = Work::from_int(0);
    for i in 0..iters {
        // Argument varies in (0, ~2) to exercise both the reciprocal-
        // reduction branch (x > 1) and the direct-Taylor branch
        // (x < 1).
        let x = Work::from_int(2) - Work::from_int(i as i64) / Work::from_int(1_000);
        acc = acc + x.atan_strict();
    }
    println!("{} iters; acc = {}", iters, black_box(acc));
    println!("trace written to trace/atan_perfetto.json");
}
