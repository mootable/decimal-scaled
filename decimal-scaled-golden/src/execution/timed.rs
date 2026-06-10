// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `Timed` — run repeatedly for the average time; parse is hoisted out, the timed
//! loop is compute only.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

use crate::collector::{ExecutionCollector, ExecutionResult};
use crate::function::Function;
use crate::rounding::RoundingMode;
use crate::subject::{Computed, DecimalSubject, Overflow};

use super::strategy::{compute_erased, parse_vals, ExecutionStrategy};

/// Run the op `number_of_executions` times and report the average time; the
/// stringified result comes from the first run.
pub struct Timed {
    pub number_of_executions: u32,
}

impl ExecutionStrategy for Timed {
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
        let n = self.number_of_executions.max(1);
        let timed = catch_unwind(AssertUnwindSafe(|| {
            let start = Instant::now();
            for _ in 0..n {
                std::hint::black_box(op(&vals));
            }
            start.elapsed().as_nanos() as u64 / n as u64
        }));
        if let Ok(avg) = timed {
            sink.record_timing(avg);
        }
    }
}
