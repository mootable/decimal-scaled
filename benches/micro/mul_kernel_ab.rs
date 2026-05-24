// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Worked example for the `ab_microbench` N-way comparison API.
//!
//! Decision being modelled: the multiply policy
//! (`src/int/policy/mul.rs`) must choose, per storage width, between the
//! schoolbook kernel and the non-allocating Karatsuba kernel. The settled
//! crossover lives in `KARATSUBA_THRESHOLD_U64`; this bench is the fast
//! pre-check a future agent runs to sanity-check (or re-derive) which of the
//! registered algorithm arms should win at a given width BEFORE paying for
//! the full crossover sweep in `int_ops_micro`/`mul_div_candidates`.
//!
//! Two registered algorithm arms (both exported via
//! `decimal_scaled::__bench_internals`, hence `bench-alt`):
//!
//! - `school` -> `mul_slice` (schoolbook `O(n^2)`).
//! - `kara`   -> `mul_karatsuba_forced`, forced to recurse exactly once at
//!   each width (threshold == L) so we measure a single real Karatsuba level,
//!   the exact shape the dispatcher produces when the threshold equals L.
//!
//! A third algorithm arm (e.g. `("toom3", toom3_run)`) would slot into the
//! `compare_all` vec alongside the two existing entries - no other changes
//! needed. The harness scales to N arms.
//!
//! Two concrete TYPES: the two storage widths L = 16 limbs (Int1024) and
//! L = 32 limbs (Int2048), supplied to the recipe via the `ab_sweep!` macro.
//!
//! Small value set: three seeded equal-length operand pairs spanning low /
//! mid / high limb magnitudes. The harness `black_box`-es both operands and
//! the product, so no kernel is const-folded away.
//!
//! Run with:
//! `cargo bench --features "wide bench-alt" --bench mul_kernel_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{mul_karatsuba_forced, mul_slice};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

/// A seeded, equal-length operand pair plus a label for its `BenchmarkId`.
#[derive(Clone)]
struct Operands {
    label: &'static str,
    a: Vec<u64>,
    b: Vec<u64>,
}

/// Deterministic mid-magnitude limb fill (same recipe as `int_ops_micro`).
fn synthetic(seed: u64, n: usize) -> Vec<u64> {
    (0..n)
        .map(|i| {
            seed.wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(i as u64 * 0x1357_9BDF)
                ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03)
        })
        .collect()
}

/// Build the small value set for a width: low / mid / high seed pairs, each
/// `l` limbs wide.
fn operand_set(l: usize) -> Vec<Operands> {
    vec![
        Operands { label: "low", a: synthetic(3, l), b: synthetic(5, l) },
        Operands { label: "mid", a: synthetic(7, l), b: synthetic(13, l) },
        Operands { label: "high", a: synthetic(1009, l), b: synthetic(7919, l) },
    ]
}

/// Schoolbook candidate: allocate + zero a `2l` output, multiply, return it.
fn school_run(ops: Operands) -> Vec<u64> {
    let mut out = vec![0u64; ops.a.len() * 2];
    mul_slice(&ops.a, &ops.b, &mut out);
    out
}

/// Karatsuba candidate: one forced recursion level (threshold == L). The
/// kernel zeroes `out` itself, but we hand it a correctly sized buffer.
fn kara_run(ops: Operands) -> Vec<u64> {
    let l = ops.a.len();
    let mut out = vec![0u64; l * 2];
    mul_karatsuba_forced(&ops.a, &ops.b, &mut out, l);
    out
}

/// N-way comparison for a single width using all registered algorithm arms.
///
/// The width is the TYPE under test; `width_label` names the criterion group
/// after the equivalent `Int<L>`. To add a third algorithm arm (e.g. Toom-3),
/// append `("toom3", toom3_run)` to the `compare_all` vec.
fn compare_width(c: &mut Criterion, l: usize, width_label: &str) {
    // Correctness cross-check: all arms must agree before we time them.
    for ops in operand_set(l) {
        assert_eq!(
            school_run(ops.clone()),
            kara_run(ops.clone()),
            "school vs kara disagree at width {l} ({})",
            ops.label
        );
    }

    compare_all(
        c,
        &format!("mul_kernel/{width_label}"),
        |ops: &Operands| ops.label.to_string(),
        operand_set(l),
        vec![
            ("school", school_run as fn(Operands) -> Vec<u64>),
            ("kara",   kara_run),
            // ("toom3", toom3_run),  // slot a third arm here
        ],
    );
}

/// Sweep all registered algorithm arms at two concrete widths via the ergonomic
/// macro. The `$ty` marker
/// documents the storage type each width corresponds to; the closure pins
/// the concrete limb count.
fn bench_mul_kernel(c: &mut Criterion) {
    use decimal_scaled::Int;
    decimal_scaled_ab_sweep!(c =>
        Int<8>  => |c: &mut Criterion| compare_width(c, 8,  "Int512"),
        Int<12> => |c: &mut Criterion| compare_width(c, 12, "Int768"),
        Int<16> => |c: &mut Criterion| compare_width(c, 16, "Int1024"),
        Int<24> => |c: &mut Criterion| compare_width(c, 24, "Int1536"),
        Int<32> => |c: &mut Criterion| compare_width(c, 32, "Int2048"),
        Int<48> => |c: &mut Criterion| compare_width(c, 48, "Int3072"),
        Int<64> => |c: &mut Criterion| compare_width(c, 64, "Int4096"),
    );
}

// `ab_sweep!` is `#[macro_export]`ed from the crate root of this bench
// binary; reference it by its export path so the `#[path]`-included module
// and the macro agree.
use crate::ab_sweep as decimal_scaled_ab_sweep;

/// Custom entry point (mirrors `int_ops_micro.rs`): build the focused
/// criterion config, run the A/B sweep, then emit the criterion final
/// summary so `--save-baseline` / HTML reports still work.
fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_mul_kernel(&mut c);
    c.final_summary();
}
