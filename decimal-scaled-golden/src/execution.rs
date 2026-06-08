//! Execution strategies — HOW a cell is run, independent of WHAT is validated.
//! A peer of the `Validator` (Tester), not a validator. The strategy owns ALL
//! timing policy (warmup, the clock); the subject owns parsing + the compute
//! closure. Timing measures ONLY `execute` because the subject hoists parse out
//! of the thunk (`Subject::compute_thunk`) and the thunk never formats.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

use crate::function::Function;
use crate::parser::GoldenCase;
use crate::rounding::RoundingMode;
use crate::subject::{Overflow, Subject};

/// How to run a cell's compute. Returns the measured batch duration, or `None`
/// when the strategy doesn't time (or nothing converts).
pub trait ExecutionStrategy {
    fn measure(
        &self,
        subject: &dyn Subject,
        cases: &[GoldenCase],
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
    ) -> Option<u64>;
}

/// No timing. Correctness-only runs use this; the validators still run (the
/// runner evaluates each case separately for validation).
pub struct RunOnce;

impl ExecutionStrategy for RunOnce {
    fn measure(
        &self,
        _subject: &dyn Subject,
        _cases: &[GoldenCase],
        _function: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> Option<u64> {
        None
    }
}

/// Warm up, then time ONE pass over the subject's compute-only thunk. Parsing is
/// hoisted into `compute_thunk` (untimed) and the thunk never formats, so this
/// measures execute only. Whole-batch nanoseconds. Run serially — never
/// alongside other timed work.
pub struct Timed {
    pub warmup: u32,
}

impl ExecutionStrategy for Timed {
    fn measure(
        &self,
        subject: &dyn Subject,
        cases: &[GoldenCase],
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
    ) -> Option<u64> {
        let mut thunk = subject.compute_thunk(cases, function, mode, overflow)?;
        catch_unwind(AssertUnwindSafe(|| {
            for _ in 0..self.warmup {
                thunk();
            }
            let start = Instant::now();
            thunk();
            start.elapsed().as_nanos() as u64
        }))
        .ok()
    }
}
