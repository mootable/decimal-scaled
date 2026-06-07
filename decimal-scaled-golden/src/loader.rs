use crate::rounding::RoundingMode;
use crate::value::GoldenValue;

/// The correctly-rounded expected value at `(width, scale, mode)`, or `None` if
/// the golden value does not fit the tier. `gen_precision` = max_decimal_width + 2.
pub fn expected(
    out: &GoldenValue,
    width: u32,
    scale: u32,
    mode: RoundingMode,
    gen_precision: usize,
) -> Option<String> {
    if !out.fits(width, scale) {
        return None;
    }
    let truncated = out.truncated_at(gen_precision);
    Some(out.round_to(scale, mode, truncated))
}

#[cfg(test)]
mod tests {
    use super::*;

    const GEN_PRECISION: usize = 1233; // max_decimal_width(1231) + 2

    #[test]
    fn expected_rounds_and_fits() {
        let out = GoldenValue::parse("1.4142135").unwrap();
        // fits D38<19>; round to scale 4 nearest
        assert_eq!(
            expected(&out, 38, 4, RoundingMode::HalfToEven, GEN_PRECISION),
            Some("14142".to_string()) // 1.4142
        );
    }
    #[test]
    fn skips_when_not_fitting() {
        let out = GoldenValue::parse("123.4").unwrap(); // 3 int digits
        assert_eq!(
            expected(&out, 38, 37, RoundingMode::HalfToEven, GEN_PRECISION),
            None
        );
    }
}
