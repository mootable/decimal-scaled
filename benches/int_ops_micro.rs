//! Per-width baseline microbenchmark for the in-tree wide signed
//! integers (`Int256` … `Int4096`).
//!
//! Captures a comparable snapshot of the core integer-operation cost
//! at each storage width, so a later integer-algorithm refactor can be
//! measured against it. Each width runs the same op set on fixed,
//! seeded mid-magnitude operands; operand construction is hoisted out
//! of every timed closure.
//!
//! Square (`x * x`) and cube (`x * x * x`) patterns are timed
//! separately from generic mul: they are the dedicated specialisation
//! targets and want their own baseline line.
//!
//! Deliberately microbench-scale — short sample / measurement windows
//! so the whole file completes in well under a couple of minutes on a
//! working machine.
//!
//! Run with: `cargo bench --features wide --bench int_ops_micro`.

use criterion::{Criterion, criterion_group};
use decimal_scaled::{Int256, Int512, Int1024, Int2048, Int4096};
use std::hint::black_box;
use std::time::Duration;

// Fixed seed operands. Chosen mid-magnitude (representative, non-trivial,
// no leading-limb-only or single-word shortcuts) and reused verbatim at
// every width via `from_i128`, so cross-width numbers stay comparable.
const A: i128 = 0x0123_4567_89AB_CDEF_0123_4567_89AB_CDEF;
const B: i128 = 0x0000_0FED_CBA9_8765_4321_0FED_CBA9_8765;

// Operand for the power / square / cube paths. The `*` operator is a
// checked mul that panics on overflow, and `pow7` of a full 128-bit
// value blows past the 256-bit floor width. `pow7` is the binding
// constraint: `x^7` must stay inside 256 bits at the narrowest width,
// so `|x| < 2^36`. This value (~2^35) satisfies that while staying
// multi-word, and is reused verbatim at every width for comparability.
const C: i128 = 0x0000_0000_0000_0000_0000_0007_FEDC_BA98;

/// Times the common op set for one width.
///
/// `$Int` is the concrete wide signed type; `$w` is the group label.
macro_rules! bench_width {
    ($c:expr, $w:literal, $Int:ty) => {{
        let a = <$Int>::from_i128(A);
        let b = <$Int>::from_i128(B);
        let c = <$Int>::from_i128(C);

        let mut g = $c.benchmark_group(concat!("int_ops/", $w));

        g.bench_function("add", |bn| bn.iter(|| black_box(a) + black_box(b)));
        g.bench_function("sub", |bn| bn.iter(|| black_box(a) - black_box(b)));
        g.bench_function("mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
        g.bench_function("div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("rem", |bn| bn.iter(|| black_box(a) % black_box(b)));

        // Power: small fixed exponents. ^3 and ^7 share the cube/odd
        // binary-exponentiation path the C3 work cares about.
        g.bench_function("pow3", |bn| bn.iter(|| black_box(c).pow(black_box(3))));
        g.bench_function("pow7", |bn| bn.iter(|| black_box(c).pow(black_box(7))));

        // Square / cube patterns written as repeated `*`, the exact
        // shape the specialisation targets.
        g.bench_function("square", |bn| {
            bn.iter(|| {
                let x = black_box(c);
                x * x
            })
        });
        g.bench_function("cube", |bn| {
            bn.iter(|| {
                let x = black_box(c);
                x * x * x
            })
        });

        g.finish();
    }};
}

fn bench_int_ops(c: &mut Criterion) {
    bench_width!(c, "Int256", Int256);
    bench_width!(c, "Int512", Int512);
    bench_width!(c, "Int1024", Int1024);
    bench_width!(c, "Int2048", Int2048);
    bench_width!(c, "Int4096", Int4096);
}

/// Karatsuba-crossover sweep: schoolbook (`mul_slice`) vs the
/// non-allocating Karatsuba kernel (forced to recurse via
/// `mul_karatsuba_forced`) on equal-length operands at the candidate
/// limb counts spanning the predicted crossover band. The smallest `L`
/// where Karatsuba wins is the crossover; `KARATSUBA_THRESHOLD_U64` is
/// then set a notch above it. Requires `--features bench-alt`.
///
/// Operands are seeded mid-magnitude and built outside the timed
/// closure; both paths re-zero their own output inside it (matching
/// the contract the production dispatcher relies on).
#[cfg(feature = "bench-alt")]
fn bench_mul_crossover(c: &mut Criterion) {
    use decimal_scaled::__bench_internals::{mul_karatsuba_forced, mul_slice};

    fn synthetic(seed: u64, n: usize) -> Vec<u64> {
        (0..n)
            .map(|i| {
                seed.wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    .wrapping_add(i as u64 * 0x1357_9BDF)
                    ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03)
            })
            .collect()
    }

    let mut g = c.benchmark_group("mul_crossover");
    g.sample_size(50);
    g.measurement_time(Duration::from_millis(800));

    // For each width L, drive a SINGLE Karatsuba level: set the forced
    // threshold to L itself, so n = L recurses exactly once into three
    // ~L/2-limb schoolbook leaves — the exact shape the production
    // dispatcher produces when `KARATSUBA_THRESHOLD_U64 == L`. This
    // measures the real one-level crossover, not pathological deep
    // recursion into tiny leaves.
    for &l in &[8usize, 12, 16, 24, 32, 48, 64] {
        let a = synthetic(7, l);
        let b = synthetic(13, l);
        let mut out = vec![0u64; 2 * l];

        g.bench_function(format!("L{l}/school"), |bn| {
            bn.iter(|| {
                for s in out.iter_mut() {
                    *s = 0;
                }
                mul_slice(black_box(&a), black_box(&b), black_box(&mut out));
            });
        });
        g.bench_function(format!("L{l}/kara"), |bn| {
            bn.iter(|| {
                mul_karatsuba_forced(black_box(&a), black_box(&b), black_box(&mut out), l);
            });
        });
    }
    g.finish();
}

/// Short windows keep the whole file at microbench scale.
fn micro() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .measurement_time(Duration::from_millis(400))
        .warm_up_time(Duration::from_millis(150))
}

#[cfg(feature = "bench-alt")]
criterion_group! {
    name = benches;
    config = micro();
    targets = bench_int_ops, bench_mul_crossover
}

#[cfg(not(feature = "bench-alt"))]
criterion_group! {
    name = benches;
    config = micro();
    targets = bench_int_ops
}

/// Custom entry point: pin the current thread to a single fixed core
/// before handing off to criterion, then run the standard harness.
///
/// Criterion's timing loop runs on this thread, so pinning it keeps the
/// measurement on one core for the whole run and removes the
/// cross-core-migration jitter that otherwise widens the variance of
/// these sub-microsecond integer ops.
fn main() {
    if let Some(c) = core_affinity::get_core_ids().and_then(|v| v.into_iter().next()) {
        core_affinity::set_for_current(c);
    }
    benches();
    Criterion::default().configure_from_args().final_summary();
}
