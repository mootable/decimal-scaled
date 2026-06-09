//! `Overflow` — how a subject signals an out-of-range result.

/// How a subject signals that a result overflows its cell. The non-value family
/// (`Panic`/`Error`/`Absent`/`Infinity`) is 1:1 with the non-value `Computed` arms;
/// the value family (`Saturate`/`Truncate`/`Wrap`) produces a value the runner
/// derives from the envelope.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overflow {
    /// The library crashes ⇒ `Computed::Panic`.
    Panic,
    /// The library returns an error value ⇒ `Computed::Error`.
    Error,
    /// The library returns no value (e.g. a checked `None`) ⇒ `Computed::Absent`.
    Absent,
    /// The library returns `±∞` ⇒ `Computed::NonReal::Infinity`, sign-matched.
    Infinity,
    /// Clamp to the nearest representable bound.
    Saturate,
    /// Keep the low `width` decimal digits.
    Truncate,
    /// Wrap modulo the two's-complement integer storage.
    Wrap,
}
