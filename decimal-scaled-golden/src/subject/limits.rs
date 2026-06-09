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
}
