// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `Limits` — the representability envelope.

/// The subject's representability envelope at a value: what it can hold and how
/// deep it rounds. `min`/`max` are `Option` because some libraries are genuinely
/// unbounded (`None` = no bound on that side). `max_precision` is always concrete.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Limits {
    /// Most-negative representable magnitude; `None` = unbounded below.
    pub min_value: Option<String>,
    /// Most-positive representable magnitude; `None` = unbounded above.
    pub max_value: Option<String>,
    /// Native fractional depth (places after the point).
    pub max_precision: u32,
    /// Maximum coefficient figures the subject can hold *exactly* — the count of
    /// significant figures spanning its stored mantissa (e.g. a fixed-significant
    /// decimal like fastnum D512 ≈ 154, decimal-rs 38). An input whose figure span
    /// exceeds this is not exactly representable (its literal cannot be ingested
    /// without rounding), so the runner skips it — the same exact-input rule the
    /// fractional `max_precision` enforces, extended to total figures. `None` =
    /// unbounded (arbitrary-precision subjects, and fixed-scale subjects whose
    /// magnitude envelope already bounds what reaches them).
    pub max_significant_digits: Option<u32>,
}
