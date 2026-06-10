//! Shared infrastructure for the golden gate: a caching loader over the committed
//! golden set and the `GOLDEN_SAMPLE` row-subset predicate. The erased
//! [`DsSubject`](decimal_scale_test::DsSubject) loops its `(width, scale)` cells at
//! runtime, so no per-cell macro is needed here — the gate file drives it directly.

#![allow(dead_code)] // the proof binary uses only a subset of this module.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use decimal_scaled_golden::{CaseLoader, FileLoader, Function, GoldenCase, Limits};
use decimal_scale_test::{golden_dir, GEN_PRECISION, GUARD};

/// Loads each function's golden file ONCE and clones it per request — the per-subject
/// `run` would otherwise re-read + re-parse the multi-MB files for every cell.
pub struct CachingLoader {
    inner: FileLoader,
    cache: Mutex<HashMap<Function, Arc<Vec<GoldenCase>>>>,
}

impl CachingLoader {
    pub fn new(dir: &str) -> CachingLoader {
        CachingLoader { inner: FileLoader::new(dir), cache: Mutex::new(HashMap::new()) }
    }

    /// Caching loader over the committed golden set.
    pub fn golden() -> CachingLoader {
        CachingLoader::new(golden_dir())
    }
}

impl CaseLoader for CachingLoader {
    fn load(&self, function: Function) -> Cow<'_, [GoldenCase]> {
        let arc = self
            .cache
            .lock()
            .unwrap()
            .entry(function)
            .or_insert_with(|| Arc::new(self.inner.load(function).into_owned()))
            .clone();
        Cow::Owned((*arc).clone())
    }
    fn oracle_limits(&self) -> Limits {
        // Verifiable depth = generation precision minus the rounding-guard digits.
        Limits {
            min_value: None,
            max_value: None,
            max_precision: (GEN_PRECISION - GUARD) as u32,
            max_significant_digits: None,
        }
    }
}

/// Keep roughly 1-in-`n` rows (by golden line) plus the first few of each file — a fast
/// subset for iteration that still hits the magnitude extremes (where the rounding edge
/// cases live). `n == 1` keeps everything.
pub fn sampled(n: usize) -> impl Fn(Function, &GoldenCase) -> bool {
    move |_f, c| n <= 1 || c.line <= 4 || c.line % n == 0
}
