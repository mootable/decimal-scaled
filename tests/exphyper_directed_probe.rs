//! Near-zero directed-rounding probe for `exp` / `exp2` / `cosh` / `sinh` /
//! `tanh`. Replays the tiny `±1e-k` golden inputs on the gate's
//! `(width, scale)` cell grid under all six rounding modes, deriving the
//! expected storage rendering by rounding the full-precision golden expected
//! string at the cell scale. Scoped to the near-zero band (`3k >= SCALE`)
//! where the directed-rounding pins decide the result.

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

/// Round the full-precision decimal string `exp` to `scale` fraction digits
/// under `mode`, rendered the way `to_string()` renders storage (sign +
/// integer part + '.' + exactly `scale` digits; no negative zero).
fn round_dec_str(exp: &str, scale: usize, mode: RoundingMode) -> String {
    let neg = exp.starts_with('-');
    let mag = exp.trim_start_matches('-');
    let (int_part, frac_part) = match mag.split_once('.') {
        Some((i, f)) => (i, f),
        None => (mag, ""),
    };
    let mut kept: Vec<u8> = frac_part.bytes().take(scale).collect();
    while kept.len() < scale {
        kept.push(b'0');
    }
    let rest = if frac_part.len() > scale { &frac_part[scale..] } else { "" };
    let rest_nonzero = rest.bytes().any(|b| b != b'0');
    let bump = rest_nonzero
        && match mode {
            RoundingMode::Trunc => false,
            RoundingMode::Floor => neg,
            RoundingMode::Ceiling => !neg,
            _ => {
                let first = rest.as_bytes()[0];
                if first > b'5' {
                    true
                } else if first < b'5' {
                    false
                } else if rest[1..].bytes().any(|b| b != b'0') {
                    true
                } else {
                    match mode {
                        RoundingMode::HalfAwayFromZero => true,
                        RoundingMode::HalfTowardZero => false,
                        _ => {
                            let last = kept
                                .last()
                                .copied()
                                .unwrap_or_else(|| *int_part.as_bytes().last().unwrap());
                            (last - b'0') % 2 == 1
                        }
                    }
                }
            }
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

/// Load the unary tiny `±1e-k` cases from one golden file.
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
        let mag = inp.trim_start_matches('-');
        let Some((i, f)) = mag.split_once('.') else {
            continue;
        };
        if i != "0" {
            continue;
        }
        let ftrim = f.trim_end_matches('0');
        if ftrim.is_empty() || !ftrim.ends_with('1') {
            continue;
        }
        if !ftrim[..ftrim.len() - 1].bytes().all(|b| b == b'0') {
            continue;
        }
        out.push((inp.to_string(), exp.to_string()));
    }
    out
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
    ($bad:expr, $cases:expr, $func:literal, $ty:ty, $tn:literal, $s:expr, $method:ident) => {{
        for (inp, exp) in $cases.iter() {
            let k = sig_frac_digits(inp);
            if k > $s || 4 * k < $s {
                continue;
            }
            let v = <$ty>::from_str(inp).unwrap();
            for mode in MODES {
                let got = v.$method(mode).to_string();
                let want = round_dec_str(exp, $s, mode);
                if got != want {
                    $bad.push(format!(
                        "{} {}<{}> {:?} in=1e-{}{} {}",
                        $func,
                        $tn,
                        $s,
                        mode,
                        k,
                        if inp.starts_with('-') { "(neg)" } else { "(pos)" },
                        digest(&got, &want)
                    ));
                }
            }
        }
    }};
}

macro_rules! all_cells {
    ($bad:expr, $cases:expr, $func:literal, $method:ident) => {{
        cell!($bad, $cases, $func, D18<4>, "D18", 4, $method);
        cell!($bad, $cases, $func, D18<9>, "D18", 9, $method);
        cell!($bad, $cases, $func, D18<13>, "D18", 13, $method);
        cell!($bad, $cases, $func, D18<17>, "D18", 17, $method);
        cell!($bad, $cases, $func, D38<9>, "D38", 9, $method);
        cell!($bad, $cases, $func, D38<19>, "D38", 19, $method);
        cell!($bad, $cases, $func, D38<28>, "D38", 28, $method);
        cell!($bad, $cases, $func, D38<37>, "D38", 37, $method);
        cell!($bad, $cases, $func, D57<14>, "D57", 14, $method);
        cell!($bad, $cases, $func, D57<28>, "D57", 28, $method);
        cell!($bad, $cases, $func, D57<42>, "D57", 42, $method);
        cell!($bad, $cases, $func, D57<56>, "D57", 56, $method);
        cell!($bad, $cases, $func, D76<19>, "D76", 19, $method);
        cell!($bad, $cases, $func, D76<38>, "D76", 38, $method);
        cell!($bad, $cases, $func, D76<57>, "D76", 57, $method);
        cell!($bad, $cases, $func, D76<75>, "D76", 75, $method);
        cell!($bad, $cases, $func, D115<28>, "D115", 28, $method);
        cell!($bad, $cases, $func, D115<57>, "D115", 57, $method);
        cell!($bad, $cases, $func, D115<86>, "D115", 86, $method);
        cell!($bad, $cases, $func, D115<114>, "D115", 114, $method);
        cell!($bad, $cases, $func, D153<38>, "D153", 38, $method);
        cell!($bad, $cases, $func, D153<76>, "D153", 76, $method);
        cell!($bad, $cases, $func, D153<114>, "D153", 114, $method);
        cell!($bad, $cases, $func, D153<152>, "D153", 152, $method);
        cell!($bad, $cases, $func, D230<57>, "D230", 57, $method);
        cell!($bad, $cases, $func, D230<115>, "D230", 115, $method);
        cell!($bad, $cases, $func, D230<172>, "D230", 172, $method);
        cell!($bad, $cases, $func, D230<229>, "D230", 229, $method);
        cell!($bad, $cases, $func, D307<76>, "D307", 76, $method);
        cell!($bad, $cases, $func, D307<153>, "D307", 153, $method);
        cell!($bad, $cases, $func, D307<230>, "D307", 230, $method);
        cell!($bad, $cases, $func, D307<306>, "D307", 306, $method);
        cell!($bad, $cases, $func, D462<115>, "D462", 115, $method);
        cell!($bad, $cases, $func, D462<231>, "D462", 231, $method);
        cell!($bad, $cases, $func, D462<346>, "D462", 346, $method);
        cell!($bad, $cases, $func, D462<461>, "D462", 461, $method);
        cell!($bad, $cases, $func, D616<154>, "D616", 154, $method);
        cell!($bad, $cases, $func, D616<308>, "D616", 308, $method);
        cell!($bad, $cases, $func, D616<462>, "D616", 462, $method);
        cell!($bad, $cases, $func, D616<615>, "D616", 615, $method);
        cell!($bad, $cases, $func, D924<231>, "D924", 231, $method);
        cell!($bad, $cases, $func, D924<462>, "D924", 462, $method);
        cell!($bad, $cases, $func, D924<693>, "D924", 693, $method);
        cell!($bad, $cases, $func, D924<923>, "D924", 923, $method);
        cell!($bad, $cases, $func, D1232<308>, "D1232", 308, $method);
        cell!($bad, $cases, $func, D1232<616>, "D1232", 616, $method);
        cell!($bad, $cases, $func, D1232<924>, "D1232", 924, $method);
        cell!($bad, $cases, $func, D1232<1231>, "D1232", 1231, $method);
    }};
}

fn finish(bad: Vec<String>) {
    if !bad.is_empty() {
        for b in &bad {
            println!("BAD {b}");
        }
        panic!("{} mismatching near-zero cells", bad.len());
    }
}

#[test]
fn probe_exp() {
    let cases = load("exp");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "exp", exp_strict_with);
    finish(bad);
}

#[test]
fn probe_exp2() {
    let cases = load("exp2");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "exp2", exp2_strict_with);
    finish(bad);
}

#[test]
fn probe_cosh() {
    let cases = load("cosh");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "cosh", cosh_strict_with);
    finish(bad);
}

#[test]
fn probe_sinh() {
    let cases = load("sinh");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "sinh", sinh_strict_with);
    finish(bad);
}

#[test]
fn probe_tanh() {
    let cases = load("tanh");
    let mut bad = Vec::new();
    all_cells!(bad, cases, "tanh", tanh_strict_with);
    finish(bad);
}
