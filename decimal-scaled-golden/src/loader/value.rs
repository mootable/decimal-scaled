// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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

    /// Render the value as a `[-]int[.frac]` decimal string with the fraction
    /// TRUNCATED to `scale` digits — e.g. for handing to a subject's `FromStr`.
    ///
    /// The golden values carry up to `gen_precision` (~1233) fraction digits; a
    /// fixed-point `FromStr` rejects more fraction digits than its scale (it is
    /// not lossy on parse), so a representability check must first reduce the
    /// fraction to the cell's scale. Truncating (not rounding) is sufficient for
    /// a magnitude/range check — the integer part decides representability.
    pub fn to_decimal_string_at_scale(&self, scale: u32) -> String {
        let sign = if self.negative { "-" } else { "" };
        let int = if self.int_digits.is_empty() { "0" } else { &self.int_digits };
        let frac: String = self.frac_digits.chars().take(scale as usize).collect();
        if frac.is_empty() {
            format!("{sign}{int}")
        } else {
            format!("{sign}{int}.{frac}")
        }
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
        let bump = should_bump(
            self.negative, residual, mode,
            last_kept_is_odd(&kept), last_kept_is_0_or_5(&kept),
        );
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

/// True if the last digit of the kept magnitude string is 0 or 5 (the
/// `ZeroFiveUp` pivot — GDA `round-05up`).
fn last_kept_is_0_or_5(kept: &str) -> bool {
    matches!(kept.bytes().last(), Some(b'0') | Some(b'5'))
}

/// Whether to add one unit to the (toward-zero) kept magnitude.
///
/// Every arm names every mode — no catch-all. A `_` arm here silently gives a
/// newly added mode some other mode's answer (`ZeroFiveUp` would have inherited
/// the nearest modes' `Above => true`), and the grader would then score a wrong
/// expected value rather than fail to build.
fn should_bump(
    negative: bool,
    residual: Residual,
    mode: RoundingMode,
    last_kept_odd: bool,
    last_kept_0_or_5: bool,
) -> bool {
    use RoundingMode::*;
    use Residual::*;
    match residual {
        // Nothing was discarded: the kept digits ARE the value, so no mode
        // bumps — including the two GDA modes, both of which are conditioned on
        // a non-zero discarded part.
        Zero => false,
        // Something WAS discarded, and it is under the half point.
        Below => match mode {
            HalfToEven | HalfAwayFromZero | HalfTowardZero | Trunc => false,
            Ceiling => !negative,
            Floor => negative,
            AwayFromZero => true,
            ZeroFiveUp => last_kept_0_or_5,
        },
        Above => match mode {
            Trunc => false,
            Floor => negative,
            Ceiling => !negative,
            // Nearest modes round away from the kept value.
            HalfToEven | HalfAwayFromZero | HalfTowardZero => true,
            AwayFromZero => true,
            // NOT a nearest mode: a discarded part just under one whole unit
            // still truncates unless the last kept digit is the pivot.
            ZeroFiveUp => last_kept_0_or_5,
        },
        Tie => match mode {
            Trunc | HalfTowardZero => false,
            HalfAwayFromZero => true,
            HalfToEven => last_kept_odd,
            Floor => negative,
            Ceiling => !negative,
            AwayFromZero => true,
            ZeroFiveUp => last_kept_0_or_5,
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
    fn away_from_zero_bumps_on_any_discard() {
        // Below / Above / Tie all discard something, so all three bump — and the
        // bump grows the MAGNITUDE, so the sign rides along unchanged.
        let m = RoundingMode::AwayFromZero;
        assert_eq!(GoldenValue::parse("1.21").unwrap().round_to(1, m, false), "13");
        assert_eq!(GoldenValue::parse("1.29").unwrap().round_to(1, m, false), "13");
        assert_eq!(GoldenValue::parse("1.25").unwrap().round_to(1, m, false), "13");
        assert_eq!(GoldenValue::parse("-1.21").unwrap().round_to(1, m, false), "-13");
        assert_eq!(GoldenValue::parse("-1.29").unwrap().round_to(1, m, false), "-13");
        assert_eq!(GoldenValue::parse("-1.25").unwrap().round_to(1, m, false), "-13");
        // The bump carries.
        assert_eq!(GoldenValue::parse("1.91").unwrap().round_to(1, m, false), "20");
        // A residual hidden BELOW the stored digits still counts as discarded.
        assert_eq!(GoldenValue::parse("1.2").unwrap().round_to(1, m, true), "13");
        assert_eq!(GoldenValue::parse("-1.2").unwrap().round_to(1, m, true), "-13");
    }
    #[test]
    fn away_from_zero_leaves_an_exact_value_alone() {
        // Nothing discarded => no bump, either sign.
        let m = RoundingMode::AwayFromZero;
        assert_eq!(GoldenValue::parse("1.20").unwrap().round_to(1, m, false), "12");
        assert_eq!(GoldenValue::parse("-1.20").unwrap().round_to(1, m, false), "-12");
        assert_eq!(GoldenValue::parse("12.00").unwrap().round_to(1, m, false), "120");
    }
    #[test]
    fn zero_five_up_pivots_on_the_last_kept_digit() {
        // Last kept 0 or 5 => away from zero; any other digit => truncate.
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(GoldenValue::parse("1.09").unwrap().round_to(1, m, false), "11");
        assert_eq!(GoldenValue::parse("1.59").unwrap().round_to(1, m, false), "16");
        assert_eq!(GoldenValue::parse("1.49").unwrap().round_to(1, m, false), "14");
        assert_eq!(GoldenValue::parse("1.69").unwrap().round_to(1, m, false), "16");
        // Sign-symmetric: the pivot decides the magnitude, not the direction.
        assert_eq!(GoldenValue::parse("-1.09").unwrap().round_to(1, m, false), "-11");
        assert_eq!(GoldenValue::parse("-1.59").unwrap().round_to(1, m, false), "-16");
        assert_eq!(GoldenValue::parse("-1.49").unwrap().round_to(1, m, false), "-14");
        assert_eq!(GoldenValue::parse("-1.69").unwrap().round_to(1, m, false), "-16");
        // A zero kept magnitude pivots too (last kept digit is '0').
        assert_eq!(GoldenValue::parse("0.09").unwrap().round_to(1, m, false), "1");
        assert_eq!(GoldenValue::parse("-0.09").unwrap().round_to(1, m, false), "-1");
    }
    #[test]
    fn zero_five_up_ignores_the_size_of_the_discarded_part() {
        // The property that separates ZeroFiveUp from every other mode: the
        // discarded part only has to be NON-ZERO. A discard just short of a whole
        // unit does not bump a non-pivot digit, and the tiniest discard does bump
        // a pivot digit — the opposite ordering to every nearest/directed mode.
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(GoldenValue::parse("1.49").unwrap().round_to(1, m, false), "14");
        assert_eq!(GoldenValue::parse("1.51").unwrap().round_to(1, m, false), "16");
        // Same two inputs under the modes that DO weigh the discarded part.
        assert_eq!(GoldenValue::parse("1.49").unwrap().round_to(1, RoundingMode::HalfToEven, false), "15");
        assert_eq!(GoldenValue::parse("1.49").unwrap().round_to(1, RoundingMode::Ceiling, false), "15");
        assert_eq!(GoldenValue::parse("1.51").unwrap().round_to(1, RoundingMode::HalfToEven, false), "15");
        // Exact ties are no different: still the pivot's decision alone.
        assert_eq!(GoldenValue::parse("1.55").unwrap().round_to(1, m, false), "16");
        assert_eq!(GoldenValue::parse("1.65").unwrap().round_to(1, m, false), "16");
    }
    #[test]
    fn zero_five_up_needs_something_discarded() {
        // A pivot digit alone is not enough — an exact value never bumps.
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(GoldenValue::parse("1.00").unwrap().round_to(1, m, false), "10");
        assert_eq!(GoldenValue::parse("1.50").unwrap().round_to(1, m, false), "15");
        assert_eq!(GoldenValue::parse("-1.50").unwrap().round_to(1, m, false), "-15");
        // ...but a residual below the stored digits IS a discard.
        assert_eq!(GoldenValue::parse("1.5").unwrap().round_to(1, m, true), "16");
        assert_eq!(GoldenValue::parse("-1.5").unwrap().round_to(1, m, true), "-16");
        assert_eq!(GoldenValue::parse("1.4").unwrap().round_to(1, m, true), "14");
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
