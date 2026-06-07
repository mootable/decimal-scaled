use crate::bigdec::abs_diff;
use crate::outcome::Outcome;
use crate::rounding::RoundingMode;
use crate::value::GoldenValue;

/// Normalize a subject's `digits.digits` output to a signed scaled-integer at
/// `scale` (the form `round_to`/`expected` produce). `None` if unparseable.
fn to_scaled_int(got: &str, scale: u32) -> Option<String> {
    let gv = GoldenValue::parse(got)?;
    Some(gv.round_to(scale, RoundingMode::Trunc, false))
}

/// Score one subject output string against the golden value at `(width, scale, claimed)`.
/// Returns `Pass` / `MisRounded` / `WrongMode` / `Skipped` (never `Error`/`Panic` —
/// those are produced upstream by the tester).
pub fn validate_one(
    got: &str,
    golden: &GoldenValue,
    width: u32,
    scale: u32,
    claimed: RoundingMode,
    gen_precision: usize,
) -> Outcome {
    if !golden.fits(width, scale) {
        return Outcome::Skipped;
    }
    let truncated = golden.truncated_at(gen_precision);
    let got_scaled = match to_scaled_int(got, scale) {
        Some(s) => s,
        None => return Outcome::MisRounded { delta: "nan".to_string() },
    };
    let exp_claimed = golden.round_to(scale, claimed, truncated);
    if got_scaled == exp_claimed {
        return Outcome::Pass;
    }
    for m in RoundingMode::ALL {
        if m == claimed { continue; }
        if got_scaled == golden.round_to(scale, m, truncated) {
            return Outcome::WrongMode { used: m };
        }
    }
    Outcome::MisRounded { delta: abs_diff(&got_scaled, &exp_claimed) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::GoldenValue;
    const GP: usize = 1233;

    #[test]
    fn pass_when_correct() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        assert_eq!(validate_one("1.4142", &g, 38, 4, RoundingMode::HalfToEven, GP), Outcome::Pass);
    }
    #[test]
    fn detects_wrong_mode() {
        // 1.4142135 at scale 6 is an exact tie (terminating 5). HalfToEven rounds
        // UP to 1.414214; the truncated 1.414213 matches the modes that round a
        // positive tie DOWN -- the validator reports the FIRST in ALL order =
        // HalfTowardZero.
        let g = GoldenValue::parse("1.4142135").unwrap();
        assert_eq!(
            validate_one("1.414213", &g, 38, 6, RoundingMode::HalfToEven, GP),
            Outcome::WrongMode { used: RoundingMode::HalfTowardZero }
        );
    }
    #[test]
    fn mis_rounded_reports_delta() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        assert_eq!(
            validate_one("1.4140", &g, 38, 4, RoundingMode::HalfToEven, GP),
            Outcome::MisRounded { delta: "2".to_string() }
        );
    }
    #[test]
    fn skipped_when_not_fitting() {
        let g = GoldenValue::parse("123.4").unwrap();
        assert_eq!(validate_one("123.4", &g, 38, 37, RoundingMode::HalfToEven, GP), Outcome::Skipped);
    }
}
