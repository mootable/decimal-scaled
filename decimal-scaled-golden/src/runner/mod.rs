// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The runner: drives a subject over the golden cases. `GoldenRunner` is the trait;
//! `SequentialRunner` / `ParallelRunner` differ only in scheduling. The shared per-cell
//! work (`run_cell`, the input filter) lives here.

mod parallel;
mod runner;
mod sequential;

pub use parallel::ParallelRunner;
pub use runner::GoldenRunner;
pub use sequential::SequentialRunner;

use crate::string_decimal::within;
use crate::collector::{ExecutionCollector, ExecutionResult};
use crate::execution::ExecutionStrategy;
use crate::function::Function;
use crate::loader::{select_radix_output, GoldenCase, GoldenValue, HexFloat};
use crate::outcome::Outcome;
use crate::subject::{Capabilities, Computed, DecimalSubject, FnSupport, Limits, Radix};
use crate::validators::{ValidationContext, Validator};

/// Run + classify + validate one golden case: build the cell, skip unrepresentable
/// inputs, execute via the strategy, then (if there are validators and the golden
/// parses) build a `ValidationContext` and collect each validator's verdict.
#[allow(clippy::too_many_arguments)]
fn run_cell<S: DecimalSubject, E: ExecutionStrategy>(
    subject: &S,
    strategy: &E,
    validators: &[Box<dyn Validator + Sync>],
    function: Function,
    caps: &Capabilities,
    support: FnSupport,
    case: &GoldenCase,
    oracle: &Limits,
) -> ExecutionCollector {
    let mut cell = ExecutionCollector::new(case.inputs.clone(), case.output_raw.clone(), case.line);

    // Input filter: every input must be exactly representable by the subject.
    if !case.inputs.iter().all(|s| input_representable(subject, s)) {
        cell.mark_skipped();
        return cell;
    }

    strategy.execute(subject, &case.inputs, function, support.mode, support.overflow, &mut cell);

    if !validators.is_empty() {
        // Select-then-parse: the subject's STORAGE radix drives which `radix:value`
        // golden entry it is graded against (spec §1.2). A no-`:` field is today's
        // single value, returned verbatim — so the untagged corpus is unaffected.
        let chosen = select_radix_output(&case.output_raw, subject.storage_radix());
        // BINARY grading path (spec §2/§3): a binary subject that declares a mantissa
        // width grades against the `2:` hex-float — the deep golden rounded to the
        // subject's OWN width, in binary. It engages ONLY when `chosen` parses as a
        // hex-float, i.e. a `2:` entry was actually selected. On the untagged corpus
        // `chosen` is a decimal value, `HexFloat::parse` returns `None`, and grading
        // falls through to the decimal path BYTE-IDENTICALLY (backward compatible).
        if subject.storage_radix() == Radix::Binary {
            if let Some(bits) = subject.mantissa_bits() {
                if let Some(hexfloat) = HexFloat::parse(chosen) {
                    let verdict = cell.result().and_then(|r| grade_binary(bits, &hexfloat, r));
                    if let Some(outcome) = verdict {
                        cell.add_validation(outcome);
                    }
                    return cell;
                }
            }
        }
        if let Some(golden) = GoldenValue::parse(chosen) {
            // Classify the CHOSEN value, not the raw (possibly tagged) field: a
            // value-aware subject's `limits` reads digits off the value string, so a
            // tagged field like `10:1.5,2:1.6` would feed it garbage (collapsing its
            // depth). Byte-identical on the untagged corpus (chosen == output_raw).
            let limits = subject.limits(chosen);
            cell.oracle_limited = limits.max_precision > oracle.max_precision;
            // Collect verdicts while the context borrows the cell's result, then
            // release the borrow before pushing them back into the cell.
            let outcomes: Vec<_> = {
                let ctx = ValidationContext {
                    function,
                    result: cell.result().expect("the cell ran"),
                    golden_value: &golden,
                    limits: &limits,
                    oracle_limits: oracle,
                    capabilities: caps,
                };
                validators.iter().filter_map(|v| v.validate(&ctx)).collect()
            };
            for o in outcomes {
                cell.add_validation(o);
            }
        }
    }
    cell
}

/// True if `input` is *exactly* representable by the subject: its significant
/// fraction digits fit the subject's depth at that value, and its magnitude fits
/// the envelope. This is the INPUT skip.
fn input_representable<S: DecimalSubject>(subject: &S, input: &str) -> bool {
    let lim = subject.limits(input);
    significant_frac_digits(input) <= lim.max_precision as usize
        && lim
            .max_significant_digits
            .is_none_or(|cap| significant_digit_span(input) <= cap as usize)
        && within(input, lim.min_value.as_deref(), lim.max_value.as_deref())
}

/// Count of fraction digits up to the last non-zero one (trailing zeros ignored):
/// the depth at which the value is exactly representable. `1.00` → 0, `1.50` → 1.
fn significant_frac_digits(s: &str) -> usize {
    s.split_once('.').map(|(_, f)| f.trim_end_matches('0').len()).unwrap_or(0)
}

/// Count of significant figures spanning the value's stored mantissa: integer and
/// fraction digits concatenated (the point is positional), leading zeros dropped.
/// `"1000"` → 4, `"0.00123"` → 3, `"12.30"` → 4, `"0"` → 0. This is the figure
/// count a fixed-significant subject's coefficient must hold to ingest the literal
/// exactly — the total-figure analogue of `significant_frac_digits`.
fn significant_digit_span(s: &str) -> usize {
    s.bytes()
        .filter(u8::is_ascii_digit)
        .skip_while(|b| *b == b'0')
        .count()
}

/// Grade a binary subject's output against the deep golden hex-float (spec §2/§3):
/// round the deep value to the subject's OWN mantissa width, in binary
/// (round-half-to-even — the IEEE binary grid's rounding, which is exactly what
/// hardware floats produce), then compare to the subject's output.
///
/// f64 (53) and f32 (24) are graded NATIVELY: the rounded deep value IS an `f64`/`f32`,
/// and the subject's shortest-round-tripping decimal output recovers its exact float
/// via `str::parse` (the std parser is correctly rounded), so a bit-exact comparison
/// decides the verdict. A non-finite / panicked result is the overflow validator's
/// domain, not rounding's, so it abstains here (`None`).
///
/// Wider mantissas (g_math's 128-bit Q128.128) are NOT graded here: comparing the
/// 128-bit golden to g_math's DECIMAL output needs a correctly-rounded
/// decimal→128-bit-binary parse of that output — intricate wide-mantissa numerical
/// code deferred per the branch's stop-clause (see `RADIX_FINDINGS.md`). Such a cell
/// abstains (`None`) rather than ship an unverified verdict.
fn grade_binary(bits: u32, hexfloat: &HexFloat, result: &ExecutionResult) -> Option<Outcome> {
    let got = match result {
        ExecutionResult::Computed(Computed::Value(s)) => s,
        _ => return None,
    };
    match bits {
        53 => {
            let golden = hexfloat.round_to_bits(53).to_f64();
            let subject: f64 = got.parse().ok()?;
            Some(if f64_bits_eq(golden, subject) {
                Outcome::Pass
            } else {
                Outcome::MisRounded { delta: f64_ulp_delta(golden, subject) }
            })
        }
        24 => {
            let golden = hexfloat.round_to_bits(24).to_f32();
            let subject: f32 = got.parse().ok()?;
            Some(if f32_bits_eq(golden, subject) {
                Outcome::Pass
            } else {
                Outcome::MisRounded { delta: f32_ulp_delta(golden, subject) }
            })
        }
        _ => None,
    }
}

/// Bit-exact f64 equality, treating `-0.0` and `+0.0` as equal (NaN never reaches
/// here — the runner maps non-finite results to `NonReal` before grading).
fn f64_bits_eq(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a == 0.0 && b == 0.0)
}

fn f32_bits_eq(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits() || (a == 0.0 && b == 0.0)
}

/// The ULP distance between two same-sign finite f64s (their ordered bit patterns'
/// difference) — a small integer for a near-miss, reported as the mis-round delta.
fn f64_ulp_delta(a: f64, b: f64) -> String {
    (a.to_bits() as i64).wrapping_sub(b.to_bits() as i64).unsigned_abs().to_string()
}

fn f32_ulp_delta(a: f32, b: f32) -> String {
    (a.to_bits() as i32).wrapping_sub(b.to_bits() as i32).unsigned_abs().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::{CellStatus, ExecutionResult};
    use crate::execution::RunOnce;
    use crate::outcome::Outcome;
    use crate::rounding::RoundingMode;
    use crate::subject::{Computed, Overflow, Radix};
    use crate::validators::RoundingValidator;
    use crate::CaseLoader;
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    struct Sqrt64;
    impl DecimalSubject for Sqrt64 {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sqrt,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
            Capabilities { name: "sqrt64".into(), radix: Radix::Decimal, config: BTreeMap::new(), functions }
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse::<f64>().expect("parse f64")
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v:.4}")
        }
        fn limits(&self, _value: &str) -> Limits {
            Limits { min_value: None, max_value: None, max_precision: 4, max_significant_digits: None }
        }
        fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> Computed<f64> {
            |inputs| Computed::Value(inputs[0].sqrt())
        }
    }

    struct FixedLoader;
    impl CaseLoader for FixedLoader {
        fn load(&self, _f: Function) -> Cow<'_, [GoldenCase]> {
            Cow::Owned(vec![GoldenCase { inputs: vec!["2".into()], output_raw: "1.4142135".into(), line: 0 }])
        }
        fn oracle_limits(&self) -> Limits {
            Limits { min_value: None, max_value: None, max_precision: 1231, max_significant_digits: None }
        }
    }

    #[test]
    fn series_runs_a_cell_and_passes() {
        let runner = SequentialRunner {
            strategy: RunOnce,
            loader: Box::new(FixedLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let sc = runner.run(&Sqrt64, &[Function::Sqrt]);
        let fc = &sc.functions[0];
        assert!(fc.supported());
        assert_eq!(fc.cells.len(), 1);
        assert_eq!(fc.cells[0].value(), Some("1.4142"));
        assert_eq!(fc.cells[0].validations, vec![Outcome::Pass]);
    }

    #[test]
    fn too_precise_input_skipped() {
        struct PreciseLoader;
        impl CaseLoader for PreciseLoader {
            fn load(&self, _f: Function) -> Cow<'_, [GoldenCase]> {
                Cow::Owned(vec![GoldenCase { inputs: vec!["1.234567".into()], output_raw: "1.1111".into(), line: 0 }])
            }
            fn oracle_limits(&self) -> Limits {
                Limits { min_value: None, max_value: None, max_precision: 1231, max_significant_digits: None }
            }
        }
        let runner = SequentialRunner {
            strategy: RunOnce,
            loader: Box::new(PreciseLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let sc = runner.run(&Sqrt64, &[Function::Sqrt]);
        assert_eq!(sc.functions[0].cells[0].status, CellStatus::Done(ExecutionResult::Skipped));
    }

    #[test]
    fn parallel_matches_series() {
        struct ManyLoader;
        impl CaseLoader for ManyLoader {
            fn load(&self, _f: Function) -> Cow<'_, [GoldenCase]> {
                Cow::Owned(
                    (1..=20)
                        .map(|n| GoldenCase {
                            inputs: vec![n.to_string()],
                            output_raw: format!("{:.7}", (n as f64).sqrt()),
                            line: n,
                        })
                        .collect(),
                )
            }
            fn oracle_limits(&self) -> Limits {
                Limits { min_value: None, max_value: None, max_precision: 1231, max_significant_digits: None }
            }
        }
        let par = ParallelRunner {
            threads: 4,
            strategy: RunOnce,
            loader: Box::new(ManyLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let ser = SequentialRunner {
            strategy: RunOnce,
            loader: Box::new(ManyLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let pc = par.run(&Sqrt64, &[Function::Sqrt]);
        let sc = ser.run(&Sqrt64, &[Function::Sqrt]);
        let (pcells, scells) = (&pc.functions[0].cells, &sc.functions[0].cells);
        assert_eq!(pcells.len(), scells.len());
        for (p, s) in pcells.iter().zip(scells) {
            assert_eq!(p.value(), s.value());
            assert_eq!(p.validations, s.validations);
        }
    }

    #[test]
    fn figure_span_counts_stored_coefficient_digits() {
        // Trailing zeros count (they are stored figures the coefficient must hold);
        // leading zeros and the point do not.
        assert_eq!(significant_digit_span("1000"), 4);
        assert_eq!(significant_digit_span("0.00123"), 3);
        assert_eq!(significant_digit_span("12.30"), 4);
        assert_eq!(significant_digit_span("-0.5"), 1);
        assert_eq!(significant_digit_span("0"), 0);
        // A 1232-digit wide-tier integer exceeds any fixed-significant cap.
        assert_eq!(significant_digit_span(&format!("1{}", "0".repeat(1231))), 1232);
    }

    #[test]
    fn too_many_figures_input_skipped() {
        // A subject with a 4-figure coefficient cap skips a 5-figure input even when
        // its fractional depth and magnitude would otherwise admit it.
        struct Capped4;
        impl DecimalSubject for Capped4 {
            type Value = f64;
            fn capabilities(&self) -> Capabilities {
                let mut functions = BTreeMap::new();
                functions.insert(
                    Function::Sqrt,
                    FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
                );
                Capabilities { name: "capped4".into(), radix: Radix::Decimal, config: BTreeMap::new(), functions }
            }
            fn string_to_value(&self, s: &str) -> f64 {
                s.parse::<f64>().expect("parse f64")
            }
            fn value_to_string(&self, v: &f64) -> String {
                format!("{v:.4}")
            }
            fn limits(&self, _value: &str) -> Limits {
                Limits { min_value: None, max_value: None, max_precision: 4, max_significant_digits: Some(4) }
            }
            fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> Computed<f64> {
                |inputs| Computed::Value(inputs[0].sqrt())
            }
        }
        assert!(input_representable(&Capped4, "1234"));     // 4 figures — fits
        assert!(!input_representable(&Capped4, "12345"));   // 5 figures — skipped
        assert!(!input_representable(&Capped4, "10000"));   // trailing zeros count
    }

    /// A value-aware Binary subject whose grade depth is the fraction-digit count of
    /// the value it is handed, and which emits `1.61` — one place off the golden `1.6`
    /// at the SECOND decimal. The cell passes only if `limits` is fed the CHOSEN value
    /// `"1.6"` (depth 1, so the 2nd-decimal miss is below the graded digit); feeding it
    /// the raw tagged field `"10:1.5,2:1.6"` would deepen the grade and surface a false
    /// MisRounded. Guards the select-then-classify wiring (limits sees the chosen value).
    struct BinaryDepthFromValue;
    impl DecimalSubject for BinaryDepthFromValue {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sqrt,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
            Capabilities { name: "bin".into(), radix: Radix::Binary, config: BTreeMap::new(), functions }
        }
        fn storage_radix(&self) -> Radix {
            Radix::Binary
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse().unwrap_or(0.0)
        }
        fn value_to_string(&self, _v: &f64) -> String {
            "1.61".to_string()
        }
        fn limits(&self, value: &str) -> Limits {
            let depth = value
                .split_once('.')
                .map(|(_, f)| f.trim_end_matches('0').len())
                .unwrap_or(0) as u32;
            Limits { min_value: None, max_value: None, max_precision: depth, max_significant_digits: None }
        }
        fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> Computed<f64> {
            |_| Computed::Value(1.61)
        }
    }

    #[test]
    fn limits_classify_the_chosen_radix_value_not_the_raw_field() {
        struct TaggedLoader;
        impl CaseLoader for TaggedLoader {
            fn load(&self, _f: Function) -> Cow<'_, [GoldenCase]> {
                Cow::Owned(vec![GoldenCase {
                    inputs: vec!["4".into()],
                    output_raw: "10:1.5,2:1.6".into(),
                    line: 0,
                }])
            }
            fn oracle_limits(&self) -> Limits {
                Limits { min_value: None, max_value: None, max_precision: 1231, max_significant_digits: None }
            }
        }
        let runner = SequentialRunner {
            strategy: RunOnce,
            loader: Box::new(TaggedLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let sc = runner.run(&BinaryDepthFromValue, &[Function::Sqrt]);
        // Binary subject selects "1.6"; `limits` sees "1.6" (depth 1), so the
        // 2nd-decimal miss in the emitted 1.61 is below the graded digit -> Pass.
        // Classifying the raw "10:1.5,2:1.6" instead would grade deeper and fail.
        assert_eq!(sc.functions[0].cells[0].validations, vec![Outcome::Pass]);
    }

    // ---- binary grading path (spec §2/§3) -----------------------------------------

    #[test]
    fn grade_binary_f64_pass_and_miss() {
        // 0.1's deep significand rounded to 53 bits IS f64 0.1; the f64 subject's
        // shortest decimal "0.1" parses back to it -> Pass.
        let hf = HexFloat::parse("0xccccccccccccdp-55").unwrap();
        let pass = ExecutionResult::Computed(Computed::Value("0.1".into()));
        assert_eq!(grade_binary(53, &hf, &pass), Some(Outcome::Pass));
        // The next f64 above 0.1 (one ULP off) -> MisRounded with a non-empty delta.
        let off = ExecutionResult::Computed(Computed::Value(format!(
            "{}",
            f64::from_bits(0.1f64.to_bits() + 1)
        )));
        match grade_binary(53, &hf, &off) {
            Some(Outcome::MisRounded { delta }) => assert_eq!(delta, "1"),
            other => panic!("expected MisRounded, got {other:?}"),
        }
    }

    #[test]
    fn grade_binary_f32_pass() {
        let hf = HexFloat::parse("0xccccccccccccdp-55").unwrap();
        let pass = ExecutionResult::Computed(Computed::Value("0.1".into()));
        assert_eq!(grade_binary(24, &hf, &pass), Some(Outcome::Pass));
    }

    #[test]
    fn grade_binary_abstains_for_wide_width_and_nonvalue() {
        let hf = HexFloat::parse("0x1p0").unwrap();
        // 128-bit width is the deferred wide path (stop-clause) -> no verdict.
        let v = ExecutionResult::Computed(Computed::Value("1".into()));
        assert_eq!(grade_binary(128, &hf, &v), None);
        // A panicked / non-finite result is the overflow validator's domain.
        let p = ExecutionResult::Computed(Computed::Panic("overflow".into()));
        assert_eq!(grade_binary(53, &hf, &p), None);
    }

    /// A conformant f64 library for the demonstration: it emits the CORRECTLY-ROUNDED
    /// f64 of `sin(0.1665)` — `0.1657317731848025`, which is exactly
    /// `round_to_53(the cell's 2: golden)` AND what the platform's `f64::sin` yields
    /// for this cell. `radix`/`mantissa_bits` switch it between the BINARY grading path
    /// (`Binary` + `Some(53)`) and the DECIMAL path (`Decimal` + `None`) so the SAME
    /// output can be graded both ways.
    struct DemoF64 {
        radix: Radix,
        mantissa_bits: Option<u32>,
    }
    impl DecimalSubject for DemoF64 {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sin,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Infinity },
            );
            Capabilities { name: "demo_f64".into(), radix: self.radix, config: BTreeMap::new(), functions }
        }
        fn storage_radix(&self) -> Radix {
            self.radix
        }
        fn mantissa_bits(&self) -> Option<u32> {
            self.mantissa_bits
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse().unwrap_or(0.0)
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v}")
        }
        fn limits(&self, value: &str) -> Limits {
            // Mimic f64: ~16 significant digits, fractional depth 16 - int_digits (<=15).
            let intd = value
                .trim_start_matches(['-', '+'])
                .split('.')
                .next()
                .unwrap_or("")
                .trim_start_matches('0')
                .len() as u32;
            Limits {
                min_value: None,
                max_value: None,
                max_precision: 16u32.saturating_sub(intd).min(15),
                max_significant_digits: Some(16),
            }
        }
        fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> Computed<f64> {
            // The correctly-rounded f64 of the true sin(0.1665) (verified bit-equal to
            // round_to_53 of the cell's `2:` golden, and to f64::sin here).
            |_inputs| Computed::Value(0.1657317731848025_f64)
        }
    }

    /// THE demonstration (spec §2): a correctly-rounded f64 PASSES against the deep
    /// `2:` golden but FAILS against the `10:` decimal golden — the radix-divergence
    /// the deep-dual-radix model re-grades. The cell is real generated data from
    /// `demo/radix_divergence/sin.golden`.
    #[test]
    fn binary_correct_f64_passes_2_but_fails_10() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/demo/radix_divergence");
        let cell_0_1665 = |radix: Radix, mantissa_bits: Option<u32>| {
            let runner = SequentialRunner {
                strategy: RunOnce,
                loader: Box::new(crate::FileLoader::new(dir)),
                // gen_precision 130 matches the demo file header.
                validators: vec![Box::new(RoundingValidator { gen_precision: 130 })],
            };
            let sc = runner.run(&DemoF64 { radix, mantissa_bits }, &[Function::Sin]);
            sc.functions[0]
                .cells
                .iter()
                .find(|c| c.inputs == ["0.1665"])
                .expect("the 0.1665 cell")
                .validations
                .clone()
        };
        // BINARY (storage Binary, 53-bit width): the `2:` hex-float rounded to 53 bits
        // is exactly the emitted f64 -> Pass.
        assert_eq!(cell_0_1665(Radix::Binary, Some(53)), vec![Outcome::Pass]);
        // DECIMAL (the SAME output, graded against the `10:` value at f64's 15-frac
        // depth): the deep golden rounds half-to-even to ...803, but the f64's shortest
        // decimal rounds (an exact half tie -> even) to ...802 — a directed-rounding
        // match (Trunc/Floor), so the verdict is WrongMode, NOT Pass. That false
        // decimal miss is exactly the cell binary grading corrects.
        let decimal = cell_0_1665(Radix::Decimal, None);
        assert!(
            matches!(decimal.as_slice(), [Outcome::WrongMode { .. }]),
            "decimal grading must NOT pass the correctly-rounded f64; got {decimal:?}"
        );
    }
}
