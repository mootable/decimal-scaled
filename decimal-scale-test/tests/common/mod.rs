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

    /// Caching loader over the committed golden set. `GOLDEN_DIR` overrides the
    /// compile-time path — a staged CI exe runs on a different runner than the one
    /// that built it, so the baked `CARGO_MANIFEST_DIR` is only valid while GHA
    /// checkout paths stay deterministic; the env override removes that coupling.
    pub fn golden() -> CachingLoader {
        match std::env::var("GOLDEN_DIR") {
            Ok(dir) if !dir.trim().is_empty() => CachingLoader::new(&dir),
            _ => CachingLoader::new(golden_dir()),
        }
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

/// Compose the 1-in-`sample` subset with an optional `(k, n)` row stripe
/// (`GOLDEN_STRIPE=k/n`): the stripe keeps rows where `line % n == k`. Stripes are a
/// PARTITION — disjoint, union = every row — so a fleet of `n` striped jobs covers the
/// full surface with no overlap (and `sample` can still thin each stripe for local
/// iteration).
pub fn row_filter(
    sample: usize,
    stripe: Option<(usize, usize)>,
) -> impl Fn(Function, &GoldenCase) -> bool {
    let sampler = sampled(sample);
    move |f, c| sampler(f, c) && stripe.is_none_or(|(k, n)| c.line % n == k)
}
