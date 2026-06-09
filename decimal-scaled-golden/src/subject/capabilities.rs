//! `Capabilities` / `FnSupport` / `Radix` — what a subject can do, plus report
//! metadata.

use std::collections::BTreeMap;

use crate::function::Function;
use crate::rounding::RoundingMode;

use super::overflow::Overflow;

/// What a subject can do, pinned to one cell. `width`/`scale` are NOT typed here —
/// the runner needs neither (representability comes from `limits`); they live in
/// `config` as report metadata only.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Capabilities {
    /// Library name (for the report).
    pub name: String,
    /// The grid this subject rounds on — verdict-neutral, for the report only.
    pub radix: Radix,
    /// Report-only metadata (width, scale, storage_bits, tier, …). The runner
    /// never consults it; reporting renders it.
    pub config: BTreeMap<String, String>,
    /// Supported functions and their per-function support. Absence == unsupported.
    pub functions: BTreeMap<Function, FnSupport>,
}

impl Capabilities {
    /// The per-function support for `func`, or `None` if unsupported.
    pub fn function(&self, func: Function) -> Option<&FnSupport> {
        self.functions.get(&func)
    }
}

/// Per-function support: the rounding mode the subject is tested under, and how it
/// behaves when that function's result overflows the cell.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FnSupport {
    pub mode: RoundingMode,
    pub overflow: Overflow,
}

/// The grid a subject rounds on. Verdict-neutral: the grader is always decimal; a
/// `Binary` subject is still judged on decimal terms (the annotation only lets the
/// report explain edge-of-resolution discrepancies).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Radix {
    #[default]
    Decimal,
    Binary,
}
