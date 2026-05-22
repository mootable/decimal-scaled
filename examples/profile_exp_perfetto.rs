//! Section-by-section timing of `D307s150::exp_strict` via
//! tracing-chrome. Writes `trace/exp_perfetto.json` — load it in
//! https://ui.perfetto.dev for a flame-chart view of the
//! `range_reduce / taylor_series / postfix_squarings / reassemble`
//! sub-spans inside `exp_fixed`.
//!
//! Build + run:
//!   cargo run --release --features wide,x-wide,xx-wide,macros,perf-trace --example profile_exp_perfetto
//!
//! Default 1000 iters keeps the trace small (~MB) — there is one
//! `exp_fixed` span per call, plus four sub-spans per call. The
//! tracing-chrome subscriber flushes on `FlushGuard::drop`.

use core::hint::black_box;
use decimal_scaled::D307;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

type Work = D307<150>;

fn main() {
    let iters: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);

    std::fs::create_dir_all("trace").expect("create trace/");
    let (chrome_layer, _guard) = ChromeLayerBuilder::new()
        .file("trace/exp_perfetto.json")
        .include_args(false)
        .build();
    tracing_subscriber::registry().with(chrome_layer).init();

    let mut acc = Work::from_int(0);
    for i in 0..iters {
        let x = Work::from_int(2) + Work::from_int(i as i128) / Work::from_int(1_000_000);
        acc = acc + x.exp_strict();
    }
    println!("{} iters; acc = {}", iters, black_box(acc));
    println!("trace written to trace/exp_perfetto.json");
    println!("open https://ui.perfetto.dev and drop the file in");
}
