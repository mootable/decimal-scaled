// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `CriterionStrategy` — bench each cell with **criterion** (warmup, sampling,
//! outlier handling) instead of `Timed`'s naive averaging. Feature-gated (`bench`).
//!
//! It changes neither the `ExecutionStrategy` trait nor the runners. criterion needs
//! `&mut Criterion`, which lives as a `RefCell<Criterion>` ON THIS STRUCT (the trait's
//! `&self` is untouched) — and that `RefCell` makes `CriterionStrategy` `!Sync`, so
//! `ParallelRunner` (`E: Sync`) cannot compile with it while `SeriesRunner` accepts it.
//! criterion's "serial, quiet machine" requirement is thus enforced at compile time,
//! for free. The `RefCell` is load-bearing — do NOT replace it with a `Mutex`.
//!
//! criterion exposes no programmatic measurement (`bench_function` → `&mut Criterion`;
//! every `iter*` → `()`; estimates land in `target/criterion/…` + stdout/HTML), so the
//! strategy *drives* criterion and criterion's own report is the artifact; the cell's
//! `timing` stays `None`. Pair ONLY with a small curated loader (criterion is hundreds
//! of ms per benched id) and `SeriesRunner`.

use std::cell::RefCell;

use criterion::Criterion;

use crate::collector::{ExecutionCollector, ExecutionResult};
use crate::function::Function;
use crate::rounding::RoundingMode;
use crate::subject::{Computed, DecimalSubject, Overflow};

use super::strategy::{compute_erased, parse_vals, ExecutionStrategy};

/// Benches each cell's operation with criterion. `!Sync` by construction (holds a
/// `RefCell<Criterion>`), so it is `SeriesRunner`-only.
pub struct CriterionStrategy {
    crit: RefCell<Criterion>,
}

impl CriterionStrategy {
    /// A plot-free criterion at its default sampling configuration.
    #[must_use]
    pub fn new() -> CriterionStrategy {
        CriterionStrategy { crit: RefCell::new(Criterion::default().without_plots()) }
    }

    /// Wrap a caller-configured `Criterion` (sample size, measurement time, …).
    #[must_use]
    pub fn with_criterion(crit: Criterion) -> CriterionStrategy {
        CriterionStrategy { crit: RefCell::new(crit) }
    }
}

impl Default for CriterionStrategy {
    fn default() -> CriterionStrategy {
        CriterionStrategy::new()
    }
}

impl ExecutionStrategy for CriterionStrategy {
    fn execute<S: DecimalSubject>(
        &self,
        subject: &S,
        input: &[String],
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
        sink: &mut ExecutionCollector,
    ) {
        let op = subject.execute(function, mode, overflow);
        // Mirror `Timed`: parse OUT of the measured span; compute once for the
        // recorded outcome; only bench when it is a `Value` (criterion's `iter`
        // does not catch panics, so a panicking subject must never reach it).
        let vals = match parse_vals(subject, input) {
            Ok(v) => v,
            Err(msg) => {
                sink.record(ExecutionResult::Computed(Computed::Panic(msg)));
                return;
            }
        };
        let first = compute_erased(subject, &op, &vals);
        let is_value = matches!(first, Computed::Value(_));
        sink.record(ExecutionResult::Computed(first));
        if !is_value {
            return;
        }
        let id = format!("{}/{}/{}", subject.name(), function.name(), input.join(","));
        self.crit.borrow_mut().bench_function(&id, |b| {
            b.iter(|| std::hint::black_box(op(&vals)));
        });
        // criterion's own report is the artifact; `timing` stays None.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rounding::RoundingMode;
    use crate::subject::{Capabilities, FnSupport, Limits, Radix};
    use std::collections::BTreeMap;
    use std::time::Duration;

    struct SqrtF64;
    impl DecimalSubject for SqrtF64 {
        type Value = f64;
        fn name(&self) -> String {
            "sqrtf64".into()
        }
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sqrt,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
            Capabilities { name: "sqrtf64".into(), radix: Radix::Decimal, config: BTreeMap::new(), functions }
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse().expect("parse f64")
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v:.4}")
        }
        fn limits(&self, _value: &str) -> Limits {
            Limits { min_value: None, max_value: None, max_precision: 4, max_significant_digits: None }
        }
        fn execute(
            &self,
            _f: Function,
            _m: RoundingMode,
            _o: Overflow,
        ) -> impl Fn(&[f64]) -> Computed<f64> {
            |inputs| Computed::Value(inputs[0].sqrt())
        }
    }

    #[test]
    fn benches_a_cell_and_records_the_value() {
        // Tiny criterion config so the smoke test finishes quickly.
        let crit = Criterion::default()
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(10))
            .measurement_time(Duration::from_millis(50));
        let strat = CriterionStrategy::with_criterion(crit);
        let mut cell = ExecutionCollector::new(vec!["2".into()], "1.4142".into(), 0);
        strat.execute(
            &SqrtF64,
            &["2".into()],
            Function::Sqrt,
            RoundingMode::HalfToEven,
            Overflow::Panic,
            &mut cell,
        );
        assert_eq!(cell.value(), Some("1.4142"));
        // criterion's report is the artifact; timing is left None.
        assert_eq!(cell.timing, None);
    }
}
