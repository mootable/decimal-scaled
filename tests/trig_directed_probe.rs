//! Directed-rounding probe for `sin` / `cos` / `asinh`. Replays EVERY
//! golden input on the gate's `(width, scale)` cell grid under all six
//! rounding modes, deriving the expected storage rendering by rounding
//! the full-precision golden expected string at the cell scale. No band
//! filter: the failing clusters for these functions span near-zero,
//! near-extremum, and mid-range inputs.

#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use core::str::FromStr;
use decimal_scaled::RoundingMode;
use decimal_scaled::{D115, D1232, D153, D18, D230, D307, D38, D462, D57, D616, D76, D924};

const MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Fraction digits up to the last non-zero one — the depth at which the
/// literal is exactly representable (mirrors the golden runner's filter).
fn sig_frac_digits(s: &str) -> usize {
    match s.split_once('.') {
        None => 0,
        Some((_, f)) => f.trim_end_matches('0').len(),
    }
}

/// The golden generation precision: a stored value carrying this many
/// fraction digits is TRUNCATED (a strictly positive tail exists below its
/// stored digits); a shorter one is exact as printed (the proof-based
/// exactness marker).
const GEN_PRECISION: usize = 1233;

/// Residual classification relative to the half point — mirrors the golden
/// gate's `classify_residual`.
#[derive(Clone, Copy, PartialEq)]
enum Residual {
    Zero,
    Below,
    Tie,
    Above,
}

/// Round the golden decimal string `exp` to `scale` fraction digits under
/// `mode`, mirroring the golden gate's `GoldenValue::round_to` (including
/// the truncation marker), rendered the way `to_string()` renders storage
/// (sign + integer part + '.' + exactly `scale` digits; no negative zero).
fn round_dec_str(exp: &str, scale: usize, mode: RoundingMode) -> String {
    let neg = exp.starts_with('-');
    let mag = exp.trim_start_matches('-');
    let (int_part, frac_part) = match mag.split_once('.') {
        Some((i, f)) => (i, f),
        None => (mag, ""),
    };
    let truncated = frac_part.len() >= GEN_PRECISION;
    let mut kept: Vec<u8> = frac_part.bytes().take(scale).collect();
    while kept.len() < scale {
        kept.push(b'0');
    }
    let rest: &[u8] =
        if frac_part.len() > scale { &frac_part.as_bytes()[scale..] } else { &[] };
    let residual = match rest.iter().position(|&b| b != b'0') {
        None => {
            if truncated {
                Residual::Below
            } else {
                Residual::Zero
            }
        }
        Some(0) => match rest[0] {
            b'5' => {
                if rest[1..].iter().any(|&b| b != b'0') || truncated {
                    Residual::Above
                } else {
                    Residual::Tie
                }
            }
            d if d < b'5' => Residual::Below,
            _ => Residual::Above,
        },
        Some(_) => Residual::Below,
    };
    let last_kept_odd = {
        let last = kept
            .last()
            .copied()
            .unwrap_or_else(|| *int_part.as_bytes().last().unwrap());
        (last - b'0') % 2 == 1
    };
    let bump = match residual {
        Residual::Zero => false,
        Residual::Below => matches!(
            (mode, neg),
            (RoundingMode::Ceiling, false) | (RoundingMode::Floor, true)
        ),
        Residual::Above => match mode {
            RoundingMode::Trunc => false,
            RoundingMode::Floor => neg,
            RoundingMode::Ceiling => !neg,
            _ => true,
        },
        Residual::Tie => match mode {
            RoundingMode::Trunc | RoundingMode::HalfTowardZero => false,
            RoundingMode::HalfAwayFromZero => true,
            RoundingMode::HalfToEven => last_kept_odd,
            RoundingMode::Floor => neg,
            RoundingMode::Ceiling => !neg,
        },
    };
    let mut int_digits: Vec<u8> = int_part.bytes().collect();
    if bump {
        let mut i = kept.len();
        loop {
            if i == 0 {
                let mut j = int_digits.len();
                loop {
                    if j == 0 {
                        int_digits.insert(0, b'1');
                        break;
                    }
                    j -= 1;
                    if int_digits[j] == b'9' {
                        int_digits[j] = b'0';
                    } else {
                        int_digits[j] += 1;
                        break;
                    }
                }
                break;
            }
            i -= 1;
            if kept[i] == b'9' {
                kept[i] = b'0';
            } else {
                kept[i] += 1;
                break;
            }
        }
    }
    let int_s = String::from_utf8(int_digits).unwrap();
    let frac_s = String::from_utf8(kept).unwrap();
    let zero = int_s.bytes().all(|b| b == b'0') && frac_s.bytes().all(|b| b == b'0');
    let sign = if neg && !zero { "-" } else { "" };
    if scale == 0 {
        format!("{sign}{int_s}")
    } else {
        format!("{sign}{int_s}.{frac_s}")
    }
}

/// Load the unary cases from one golden file — EVERY data line (the trig
/// failing clusters are not confined to a magnitude band).
fn load(name: &str) -> Vec<(String, String)> {
    let path = format!(
        "{}/decimal-scaled-golden/golden/{}.golden",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let body = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut out = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(inp), Some(exp), None) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        out.push((inp.to_string(), exp.to_string()));
    }
    out
}

/// The input literal with insignificant trailing fraction zeros removed (the
/// gate's representability rule counts SIGNIFICANT fraction digits).
fn trim_input(inp: &str) -> String {
    if !inp.contains('.') {
        return inp.to_string();
    }
    inp.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Significant integer digits of the expected magnitude (leading sign/zeros
/// stripped) — the result-fits-the-width pre-check.
fn int_sig_digits(exp: &str) -> usize {
    let mag = exp.trim_start_matches('-');
    let int_part = mag.split_once('.').map_or(mag, |(i, _)| i);
    int_part.trim_start_matches('0').len()
}

/// Short digest of a long rendered value: head plus the window around the
/// first byte where the two strings differ.
fn digest(a: &str, b: &str) -> String {
    let d = a
        .bytes()
        .zip(b.bytes())
        .position(|(x, y)| x != y)
        .unwrap_or(a.len().min(b.len()));
    let lo = d.saturating_sub(4);
    let ha = &a[lo..(d + 4).min(a.len())];
    let hb = &b[lo..(d + 4).min(b.len())];
    format!("diff@{d} got[..]={ha} want[..]={hb}")
}

macro_rules! cell {
    ($bad:expr, $cases:expr, $func:literal, $ty:ty, $tn:literal, $s:expr, $wd:expr, $method:ident) => {{
        for (inp, exp) in $cases.iter() {
            // Representable at the cell: significant fraction digits fit the
            // scale (`from_str` rejects deeper literals; it also enforces the
            // width's integer range on the input).
            if sig_frac_digits(inp) > $s {
                continue;
            }
            let lit = trim_input(inp);
            let Ok(v) = <$ty>::from_str(&lit) else {
                continue;
            };
            // Result fits the width (an out-of-range result is the overflow
            // contract's domain — it panics — not the rounding probe's).
            if int_sig_digits(exp) > ($wd as usize) - ($s as usize) {
                continue;
            }
            for mode in MODES {
                let got = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                    v.$method(mode).to_string()
                }));
                let want = round_dec_str(exp, $s, mode);
                match got {
                    Ok(got) => {
                        if got != want {
                            $bad.push(format!(
                                "{} {}<{}> {:?} in={} {}",
                                $func,
                                $tn,
                                $s,
                                mode,
                                &inp[..inp.len().min(24)],
                                digest(&got, &want)
                            ));
                        }
                    }
                    Err(_) => {
                        $bad.push(format!(
                            "{} {}<{}> {:?} in={} PANIC (want {})",
                            $func,
                            $tn,
                            $s,
                            mode,
                            &inp[..inp.len().min(24)],
                            &want[..want.len().min(24)]
                        ));
                    }
                }
            }
        }
    }};
}

// The authoritative golden gate's full `(width, scale)` cell grid
// (`decimal-scale-test/src/lib.rs` `cells!`), so the probe cannot under-cover
// a gate cell again.
macro_rules! all_cells {
    ($bad:expr, $cases:expr, $func:literal, $method:ident) => {{
        cell!($bad, $cases, $func, D18<0>, "D18", 0, 18, $method);
        cell!($bad, $cases, $func, D18<3>, "D18", 3, 18, $method);
        cell!($bad, $cases, $func, D18<4>, "D18", 4, 18, $method);
        cell!($bad, $cases, $func, D18<9>, "D18", 9, 18, $method);
        cell!($bad, $cases, $func, D18<13>, "D18", 13, 18, $method);
        cell!($bad, $cases, $func, D18<17>, "D18", 17, 18, $method);
        cell!($bad, $cases, $func, D38<0>, "D38", 0, 38, $method);
        cell!($bad, $cases, $func, D38<2>, "D38", 2, 38, $method);
        cell!($bad, $cases, $func, D38<6>, "D38", 6, 38, $method);
        cell!($bad, $cases, $func, D38<9>, "D38", 9, 38, $method);
        cell!($bad, $cases, $func, D38<10>, "D38", 10, 38, $method);
        cell!($bad, $cases, $func, D38<12>, "D38", 12, 38, $method);
        cell!($bad, $cases, $func, D38<17>, "D38", 17, 38, $method);
        cell!($bad, $cases, $func, D38<18>, "D38", 18, 38, $method);
        cell!($bad, $cases, $func, D38<19>, "D38", 19, 38, $method);
        cell!($bad, $cases, $func, D38<28>, "D38", 28, 38, $method);
        cell!($bad, $cases, $func, D38<37>, "D38", 37, 38, $method);
        cell!($bad, $cases, $func, D57<0>, "D57", 0, 57, $method);
        cell!($bad, $cases, $func, D57<14>, "D57", 14, 57, $method);
        cell!($bad, $cases, $func, D57<20>, "D57", 20, 57, $method);
        cell!($bad, $cases, $func, D57<28>, "D57", 28, 57, $method);
        cell!($bad, $cases, $func, D57<30>, "D57", 30, 57, $method);
        cell!($bad, $cases, $func, D57<42>, "D57", 42, 57, $method);
        cell!($bad, $cases, $func, D57<56>, "D57", 56, 57, $method);
        cell!($bad, $cases, $func, D76<0>, "D76", 0, 76, $method);
        cell!($bad, $cases, $func, D76<18>, "D76", 18, 76, $method);
        cell!($bad, $cases, $func, D76<19>, "D76", 19, 76, $method);
        cell!($bad, $cases, $func, D76<38>, "D76", 38, 76, $method);
        cell!($bad, $cases, $func, D76<40>, "D76", 40, 76, $method);
        cell!($bad, $cases, $func, D76<57>, "D76", 57, 76, $method);
        cell!($bad, $cases, $func, D76<75>, "D76", 75, 76, $method);
        cell!($bad, $cases, $func, D115<0>, "D115", 0, 115, $method);
        cell!($bad, $cases, $func, D115<28>, "D115", 28, 115, $method);
        cell!($bad, $cases, $func, D115<50>, "D115", 50, 115, $method);
        cell!($bad, $cases, $func, D115<57>, "D115", 57, 115, $method);
        cell!($bad, $cases, $func, D115<86>, "D115", 86, 115, $method);
        cell!($bad, $cases, $func, D115<114>, "D115", 114, 115, $method);
        cell!($bad, $cases, $func, D153<0>, "D153", 0, 153, $method);
        cell!($bad, $cases, $func, D153<38>, "D153", 38, 153, $method);
        cell!($bad, $cases, $func, D153<76>, "D153", 76, 153, $method);
        cell!($bad, $cases, $func, D153<114>, "D153", 114, 153, $method);
        cell!($bad, $cases, $func, D153<152>, "D153", 152, 153, $method);
        cell!($bad, $cases, $func, D230<0>, "D230", 0, 230, $method);
        cell!($bad, $cases, $func, D230<57>, "D230", 57, 230, $method);
        cell!($bad, $cases, $func, D230<115>, "D230", 115, 230, $method);
        cell!($bad, $cases, $func, D230<172>, "D230", 172, 230, $method);
        cell!($bad, $cases, $func, D230<229>, "D230", 229, 230, $method);
        cell!($bad, $cases, $func, D307<0>, "D307", 0, 307, $method);
        cell!($bad, $cases, $func, D307<30>, "D307", 30, 307, $method);
        cell!($bad, $cases, $func, D307<50>, "D307", 50, 307, $method);
        cell!($bad, $cases, $func, D307<70>, "D307", 70, 307, $method);
        cell!($bad, $cases, $func, D307<76>, "D307", 76, 307, $method);
        cell!($bad, $cases, $func, D307<120>, "D307", 120, 307, $method);
        cell!($bad, $cases, $func, D307<153>, "D307", 153, 307, $method);
        cell!($bad, $cases, $func, D307<230>, "D307", 230, 307, $method);
        cell!($bad, $cases, $func, D307<290>, "D307", 290, 307, $method);
        cell!($bad, $cases, $func, D307<306>, "D307", 306, 307, $method);
        cell!($bad, $cases, $func, D462<0>, "D462", 0, 462, $method);
        cell!($bad, $cases, $func, D462<30>, "D462", 30, 462, $method);
        cell!($bad, $cases, $func, D462<100>, "D462", 100, 462, $method);
        cell!($bad, $cases, $func, D462<115>, "D462", 115, 462, $method);
        cell!($bad, $cases, $func, D462<180>, "D462", 180, 462, $method);
        cell!($bad, $cases, $func, D462<231>, "D462", 231, 462, $method);
        cell!($bad, $cases, $func, D462<346>, "D462", 346, 462, $method);
        cell!($bad, $cases, $func, D462<461>, "D462", 461, 462, $method);
        cell!($bad, $cases, $func, D616<0>, "D616", 0, 616, $method);
        cell!($bad, $cases, $func, D616<30>, "D616", 30, 616, $method);
        cell!($bad, $cases, $func, D616<130>, "D616", 130, 616, $method);
        cell!($bad, $cases, $func, D616<154>, "D616", 154, 616, $method);
        cell!($bad, $cases, $func, D616<240>, "D616", 240, 616, $method);
        cell!($bad, $cases, $func, D616<308>, "D616", 308, 616, $method);
        cell!($bad, $cases, $func, D616<462>, "D616", 462, 616, $method);
        cell!($bad, $cases, $func, D616<590>, "D616", 590, 616, $method);
        cell!($bad, $cases, $func, D616<615>, "D616", 615, 616, $method);
        cell!($bad, $cases, $func, D924<0>, "D924", 0, 924, $method);
        cell!($bad, $cases, $func, D924<30>, "D924", 30, 924, $method);
        cell!($bad, $cases, $func, D924<180>, "D924", 180, 924, $method);
        cell!($bad, $cases, $func, D924<231>, "D924", 231, 924, $method);
        cell!($bad, $cases, $func, D924<350>, "D924", 350, 924, $method);
        cell!($bad, $cases, $func, D924<462>, "D924", 462, 924, $method);
        cell!($bad, $cases, $func, D924<693>, "D924", 693, 924, $method);
        cell!($bad, $cases, $func, D924<900>, "D924", 900, 924, $method);
        cell!($bad, $cases, $func, D924<923>, "D924", 923, 924, $method);
        cell!($bad, $cases, $func, D1232<0>, "D1232", 0, 1232, $method);
        cell!($bad, $cases, $func, D1232<30>, "D1232", 30, 1232, $method);
        cell!($bad, $cases, $func, D1232<250>, "D1232", 250, 1232, $method);
        cell!($bad, $cases, $func, D1232<308>, "D1232", 308, 1232, $method);
        cell!($bad, $cases, $func, D1232<470>, "D1232", 470, 1232, $method);
        cell!($bad, $cases, $func, D1232<616>, "D1232", 616, 1232, $method);
        cell!($bad, $cases, $func, D1232<924>, "D1232", 924, 1232, $method);
        cell!($bad, $cases, $func, D1232<1200>, "D1232", 1200, 1232, $method);
        cell!($bad, $cases, $func, D1232<1231>, "D1232", 1231, 1232, $method);
    }};
}

fn finish(bad: Vec<String>) {
    if !bad.is_empty() {
        for b in &bad {
            println!("BAD {b}");
        }
        panic!("{} mismatching cells", bad.len());
    }
}

#[test]
fn probe_sin() {
    let cases = load("sin");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "sin", sin_strict_with);
    finish(bad);
}

#[test]
fn probe_cos() {
    let cases = load("cos");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "cos", cos_strict_with);
    finish(bad);
}

#[test]
fn probe_asinh() {
    let cases = load("asinh");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "asinh", asinh_strict_with);
    finish(bad);
}
