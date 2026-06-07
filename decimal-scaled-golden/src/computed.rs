/// The result of asking a library to compute or convert a value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Computed<T> {
    /// The library produced a result.
    Value(T),
    /// Not applicable: out of domain, or not representable at this width/scale.
    Skip,
    /// The library failed on an input it should have handled; reason is for the report.
    Error(String),
}
