use crate::function::Function;
use crate::rounding::RoundingMode;

/// A decimal implementation under test. Implemented by a thin adapter in each
/// library (decimal-scaled + competitors), never in this crate.
pub trait DecimalSubject {
    /// Display name (e.g. "decimal-scaled", "rust_decimal").
    fn name(&self) -> &str;

    /// The widest decimal-digit capacity this subject supports for `func`
    /// (0 => function not exposed).
    fn max_width(&self, func: Function) -> u32;

    /// The maximum scale (decimal places) supported at `width` for `func`.
    fn max_scale(&self, func: Function, width: u32) -> u32;

    /// Rounding modes this subject can apply (subset of `RoundingMode::ALL`).
    fn rounding_modes(&self) -> &[RoundingMode];

    /// Evaluate `func` at `(width, scale)` on the string inputs under `mode`,
    /// returning the result as a `digits.digits` string. `None` => not applicable
    /// (input out of domain/range for this subject, or mode unsupported).
    fn eval(
        &self,
        func: Function,
        width: u32,
        scale: u32,
        inputs: &[&str],
        mode: RoundingMode,
    ) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mock;
    impl DecimalSubject for Mock {
        fn name(&self) -> &str { "mock" }
        fn max_width(&self, _f: Function) -> u32 { 38 }
        fn max_scale(&self, _f: Function, _w: u32) -> u32 { 37 }
        fn rounding_modes(&self) -> &[RoundingMode] { &RoundingMode::ALL }
        fn eval(&self, _f: Function, _w: u32, _s: u32, inputs: &[&str], _m: RoundingMode)
            -> Option<String> { inputs.first().map(|s| s.to_string()) }
    }

    #[test]
    fn trait_object_works() {
        let s: &dyn DecimalSubject = &Mock;
        assert_eq!(s.name(), "mock");
        assert_eq!(
            s.eval(Function::Sqrt, 38, 19, &["4"], RoundingMode::HalfToEven),
            Some("4".into())
        );
    }
}
