use crate::rounding::RoundingMode;

/// A parsed singular golden value: sign, integer digits, and fraction digits,
/// stored as digit strings (no numeric type — width-independent).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoldenValue {
    pub negative: bool,
    pub int_digits: String,
    pub frac_digits: String,
}

impl GoldenValue {
    /// Parse one `digits.digits` field (optionally signed). None on a malformed field.
    pub fn parse(s: &str) -> Option<GoldenValue> {
        let (negative, body) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s.strip_prefix('+').unwrap_or(s)),
        };
        let (int_digits, frac_digits) = match body.split_once('.') {
            Some((i, f)) => (i, f),
            None => (body, ""),
        };
        if int_digits.is_empty() && frac_digits.is_empty() { return None; }
        if !int_digits.bytes().all(|b| b.is_ascii_digit()) { return None; }
        if !frac_digits.bytes().all(|b| b.is_ascii_digit()) { return None; }
        Some(GoldenValue {
            negative,
            int_digits: int_digits.to_string(),
            frac_digits: frac_digits.to_string(),
        })
    }

    /// True if the stored fraction reached the generated precision `gen_precision`
    /// (= max_decimal_width + 2), i.e. a residual exists below the stored digits.
    /// False => the value terminated (exact).
    pub fn truncated_at(&self, gen_precision: usize) -> bool {
        self.frac_digits.len() >= gen_precision
    }

    /// Significant integer digits (leading zeros stripped; "0" or "" => 0).
    pub fn integer_digit_count(&self) -> usize {
        self.int_digits.trim_start_matches('0').len()
    }

    /// Fits a tier of `width_digits` total decimal digits at `scale` places.
    pub fn fits(&self, width_digits: u32, scale: u32) -> bool {
        let avail = width_digits.saturating_sub(scale) as usize;
        self.integer_digit_count() <= avail
    }

    /// The correctly-rounded value at `scale` under `mode`, as a signed
    /// scaled-integer string (value * 10^scale). `truncated` = the stored value
    /// has a hidden residual below its stored digits.
    pub fn round_to(&self, scale: u32, mode: RoundingMode, truncated: bool) -> String {
        let scale = scale as usize;
        let frac = self.frac_digits.as_bytes();
        // kept = integer digits + the first `scale` fraction digits, right-padded
        // with '0' when the stored fraction is shorter (the value terminated).
        let mut kept = self.int_digits.clone();
        for i in 0..scale {
            kept.push(*frac.get(i).unwrap_or(&b'0') as char);
        }
        let rest: &[u8] = if frac.len() > scale { &frac[scale..] } else { &[] };
        let residual = classify_residual(rest, truncated);
        let bump = should_bump(self.negative, residual, mode, last_kept_is_odd(&kept));
        let mag = if bump { string_increment(&kept) } else { kept };
        let mag = mag.trim_start_matches('0');
        let mag = if mag.is_empty() { "0" } else { mag };
        if self.negative && mag != "0" { format!("-{mag}") } else { mag.to_string() }
    }
}

/// The part of the value strictly below the kept `scale` fraction digits,
/// relative to the half point.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Residual { Zero, Below, Tie, Above }

/// Classify the residual. `rest` = stored fraction bytes at index `scale..` (may
/// be empty). `truncated` = a tiny non-zero residual exists below the stored digits.
fn classify_residual(rest: &[u8], truncated: bool) -> Residual {
    match rest.iter().position(|&b| b != b'0') {
        None => if truncated { Residual::Below } else { Residual::Zero },
        Some(0) => match rest[0] {
            b'5' => {
                let more = rest[1..].iter().any(|&b| b != b'0') || truncated;
                if more { Residual::Above } else { Residual::Tie }
            }
            d if d < b'5' => Residual::Below,
            _ => Residual::Above,
        },
        Some(_) => Residual::Below,
    }
}

/// True if the last digit of the kept magnitude string is odd (HalfToEven pivot).
fn last_kept_is_odd(kept: &str) -> bool {
    kept.bytes().last().map_or(false, |b| (b - b'0') % 2 == 1)
}

/// Whether to add one unit to the (toward-zero) kept magnitude.
fn should_bump(negative: bool, residual: Residual, mode: RoundingMode, last_kept_odd: bool) -> bool {
    use RoundingMode::*;
    use Residual::*;
    match residual {
        Zero => false,
        Below => matches!((mode, negative), (Ceiling, false) | (Floor, true)),
        Above => match mode {
            Trunc => false,
            Floor => negative,
            Ceiling => !negative,
            _ => true, // nearest modes round away from the kept value
        },
        Tie => match mode {
            Trunc | HalfTowardZero => false,
            HalfAwayFromZero => true,
            HalfToEven => last_kept_odd,
            Floor => negative,
            Ceiling => !negative,
        },
    }
}

/// Add 1 to a non-negative integer digit string, with carry.
fn string_increment(s: &str) -> String {
    let mut out = s.as_bytes().to_vec();
    let mut i = out.len();
    loop {
        if i == 0 { out.insert(0, b'1'); break; }
        i -= 1;
        if out[i] == b'9' { out[i] = b'0'; } else { out[i] += 1; break; }
    }
    String::from_utf8(out).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sign_int_frac() {
        let v = GoldenValue::parse("-12.3400").unwrap();
        assert_eq!(v.negative, true);
        assert_eq!(v.int_digits, "12");
        assert_eq!(v.frac_digits, "3400");
        assert_eq!(v.truncated_at(6), false);
    }
    #[test]
    fn parses_integer_only() {
        let v = GoldenValue::parse("7").unwrap();
        assert_eq!(v.int_digits, "7");
        assert_eq!(v.frac_digits, "");
    }
    #[test]
    fn fit_by_integer_digits() {
        let v = GoldenValue::parse("123.45").unwrap();
        assert!(v.fits(38, 19));
        assert!(!v.fits(38, 37));
        let z = GoldenValue::parse("0.5").unwrap();
        assert!(z.fits(18, 17));
    }
    #[test]
    fn round_below_half_all_modes() {
        let v = GoldenValue::parse("1.24").unwrap();
        assert_eq!(v.round_to(1, RoundingMode::HalfToEven, false), "12");
        assert_eq!(v.round_to(1, RoundingMode::Ceiling, false), "13");
        assert_eq!(v.round_to(1, RoundingMode::Floor, false), "12");
        assert_eq!(v.round_to(1, RoundingMode::Trunc, false), "12");
    }
    #[test]
    fn round_exact_tie_half_to_even() {
        let v = GoldenValue::parse("1.25").unwrap();
        assert_eq!(v.round_to(1, RoundingMode::HalfToEven, false), "12");
        assert_eq!(v.round_to(1, RoundingMode::HalfAwayFromZero, false), "13");
        let w = GoldenValue::parse("1.35").unwrap();
        assert_eq!(w.round_to(1, RoundingMode::HalfToEven, false), "14");
    }
    #[test]
    fn round_above_half_when_truncated_five() {
        let v = GoldenValue::parse("1.25").unwrap();
        assert_eq!(v.round_to(1, RoundingMode::HalfToEven, true), "13");
    }
    #[test]
    fn ceiling_exact_no_bump() {
        let v = GoldenValue::parse("12.00").unwrap();
        assert_eq!(v.round_to(1, RoundingMode::Ceiling, false), "120");
    }
    #[test]
    fn classify_residual_cases() {
        assert_eq!(classify_residual(b"5", false), Residual::Tie);
        assert_eq!(classify_residual(b"5", true), Residual::Above);
        assert_eq!(classify_residual(b"04", false), Residual::Below);
        assert_eq!(classify_residual(b"0", false), Residual::Zero);
        assert_eq!(classify_residual(b"", true), Residual::Below);
        assert_eq!(classify_residual(b"6", false), Residual::Above);
        assert_eq!(classify_residual(b"50", false), Residual::Tie);
        assert_eq!(classify_residual(b"51", false), Residual::Above);
    }
}
