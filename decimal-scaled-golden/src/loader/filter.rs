// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `FilterLoader` — a `CaseLoader` decorator that subsets another loader's cases.

use std::borrow::Cow;

use crate::function::Function;
use crate::subject::Limits;

use super::loader::{CaseLoader, GoldenCase};

/// Wraps another [`CaseLoader`] and yields only the cases its predicate keeps — so a
/// gate can run a representative SUBSET of an existing golden set (a width/scale band,
/// the edge inputs, every Nth row, …) without duplicating data. The wrapped loader's
/// [`oracle_limits`](CaseLoader::oracle_limits) pass through unchanged.
///
/// The predicate is `Fn(Function, &GoldenCase) -> bool`, so it can vary the kept set
/// per function. Compose it over `FileLoader` for a fast CI gate while the full
/// surface stays the heavy `--ignored` gate, both reading one source.
pub struct FilterLoader<L, F> {
    inner: L,
    keep: F,
}

impl<L, F> FilterLoader<L, F> {
    pub fn new(inner: L, keep: F) -> FilterLoader<L, F> {
        FilterLoader { inner, keep }
    }
}

impl<L, F> CaseLoader for FilterLoader<L, F>
where
    L: CaseLoader,
    F: Fn(Function, &GoldenCase) -> bool,
{
    fn load(&self, func: Function) -> Cow<'_, [GoldenCase]> {
        let kept: Vec<GoldenCase> = self
            .inner
            .load(func)
            .iter()
            .filter(|c| (self.keep)(func, c))
            .cloned()
            .collect();
        Cow::Owned(kept)
    }

    fn oracle_limits(&self) -> Limits {
        self.inner.oracle_limits()
    }
}
