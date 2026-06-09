//! `Computed` — the test-level outcome of one execution.

/// The test-level outcome of one execution: what the subject produced, or one of
/// the two failures the runner injects on the subject's behalf (`Timeout`/`Panic`).
/// `T` is `Value` from the closure and `String` once erased.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Computed<T> {
    /// A finite real decimal value.
    Value(T),
    /// A result that isn't a finite real (the IEEE specials + imaginary).
    NonReal(NonReal),
    /// The library returned no value (e.g. a checked op returning `None`).
    Absent,
    /// The library returned an explicit error value; the reason flows to the report.
    Error(String),
    /// The subject exceeded its time budget (ms) — a test failure; runner-injected.
    Timeout(u64),
    /// The subject crashed (caught) — a test failure, with the panic message.
    Panic(String),
}

/// Everything outside the reals, in one place — the harness works only in real
/// numbers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NonReal {
    PositiveInfinity,
    NegativeInfinity,
    NaN,
    Imaginary,
}
