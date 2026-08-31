// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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
    /// Report-only radix annotation, populated from the subject's `storage_radix()`.
    /// **Superseded for verdicts** by the `storage_radix()` / `rounding_radix()` trait
    /// methods (the single source of truth); kept so the report can name the grid.
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

/// The grid a subject stores and rounds on. **Verdict-determining**: a subject's
/// `storage_radix()` (the trait method — the single source of truth) drives which
/// `radix:value` golden entry it is graded against, and the reach it is graded to.
/// `tag()` / `from_tag()` map a variant to the golden grammar's base tag.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Radix {
    #[default]
    Decimal,
    Binary,
}

impl Radix {
    /// The base tag this radix carries in a golden output field: `"10"` for decimal,
    /// `"2"` for binary. The value-chooser (`loader::select_radix_output`) matches a
    /// `radix:value` entry's prefix against this. Base only — no precision, no kind.
    pub fn tag(&self) -> &'static str {
        match self {
            Radix::Decimal => "10",
            Radix::Binary => "2",
        }
    }

    /// The radix a golden base tag denotes, or `None` for an unrecognised tag — the
    /// inverse of `tag`, used to match a parsed `radix:` prefix to a subject's radix.
    pub fn from_tag(tag: &str) -> Option<Radix> {
        match tag {
            "10" => Some(Radix::Decimal),
            "2" => Some(Radix::Binary),
            _ => None,
        }
    }
}
