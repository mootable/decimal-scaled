//! 0.5 ULP gate against the mpmath oracle.
//!
//! Each golden table under `tests/golden/<func>_d<N>_s<S>.txt` is
//! produced by `scripts/gen_golden_precision.py` at mpmath dps=500,
//! with one `<input_raw>\t<expected_raw>` per line.
//!
//! For every (tier, function) we parse the file, call the kernel,
//! and assert `|kernel_result - expected| <= 1` storage LSB. That
//! is the 0.5 ULP contract — half a ULP of true-real rounding plus
//! at most a single-LSB transcription room, exactly the same
//! shape `precision_strict_05_ulp.rs` uses for D38<12>.
//!
//! On failure, the assertion message prints the per-case tuple so
//! a kernel regression is locatable to one (function, width, scale,
//! input).
//!
//! Test split per width (e.g. `d38`, `d76`, …) so the local
//! iteration loop is fast: `cargo test --test ulp_strict_golden
//! d76` runs only the D76<35> band.
//!
//! Compile-gated to the crate-default rounding mode (`HalfToEven`);
//! the goldens were generated under that assumption.

#![cfg(all(
    not(feature = "fast"),
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]

// ─── Common helpers ────────────────────────────────────────────────────

/// Trim a line: drop comments (`#`-prefixed) and blank lines.
fn parse_line(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let mut parts = line.split('\t');
    let lhs = parts.next()?;
    let rhs = parts.next()?;
    Some((lhs, rhs))
}

/// Maximum allowed LSB delta between kernel and oracle. The crate's
/// 0.5 ULP contract permits up to one storage LSB of rounding plus
/// at-most-one LSB of mpmath truth transcription, which matches the
/// existing `tests/precision_strict_05_ulp.rs` budget.
const MAX_LSB_DELTA: i128 = 1;

// ─── D38<19> band ──────────────────────────────────────────────────────

/// D38<19> uses `i128` storage; parsing is direct and the delta fits
/// `i128` arithmetic comfortably.
mod d38 {
    use super::{MAX_LSB_DELTA, parse_line};
    use decimal_scaled::D38;

    type D = D38<19>;

    /// Cast raw input through `D::from_bits`, call the kernel, return
    /// the resulting raw bits.
    fn call(func: &str, raw: i128) -> i128 {
        let d = D::from_bits(raw);
        match func {
            "ln"   => d.ln_strict().to_bits(),
            "exp"  => d.exp_strict().to_bits(),
            "sin"  => d.sin_strict().to_bits(),
            "cos"  => d.cos_strict().to_bits(),
            "tan"  => d.tan_strict().to_bits(),
            "atan" => d.atan_strict().to_bits(),
            "sqrt" => d.sqrt_strict().to_bits(),
            "cbrt" => d.cbrt_strict().to_bits(),
            other  => panic!("unknown function: {other}"),
        }
    }

    fn check(func: &str, table: &str) {
        let mut failures = 0usize;
        for line in table.lines() {
            let Some((lhs, rhs)) = parse_line(line) else { continue };
            let raw_in: i128 = lhs.parse().expect("input parse");
            let expected: i128 = rhs.parse().expect("expected parse");
            let actual = call(func, raw_in);
            let delta = (actual - expected).abs();
            if delta > MAX_LSB_DELTA {
                if failures < 16 {
                    eprintln!(
                        "FAIL: {func} D38<19> input={raw_in} expected={expected} \
                         actual={actual} delta={delta} LSB",
                    );
                }
                failures += 1;
            }
        }
        assert!(
            failures == 0,
            "{func}: {failures} cases exceeded {MAX_LSB_DELTA} LSB",
        );
    }

    #[test] fn ln()   { check("ln",   include_str!("golden/ln_d38_s19.txt")); }
    #[test] fn exp()  { check("exp",  include_str!("golden/exp_d38_s19.txt")); }
    #[test] fn sin()  { check("sin",  include_str!("golden/sin_d38_s19.txt")); }
    #[test] fn cos()  { check("cos",  include_str!("golden/cos_d38_s19.txt")); }
    #[test] fn tan()  { check("tan",  include_str!("golden/tan_d38_s19.txt")); }
    #[test] fn atan() { check("atan", include_str!("golden/atan_d38_s19.txt")); }
    #[test] fn sqrt() { check("sqrt", include_str!("golden/sqrt_d38_s19.txt")); }
    #[test] fn cbrt() { check("cbrt", include_str!("golden/cbrt_d38_s19.txt")); }
}

// ─── Wide-tier shared driver macro ─────────────────────────────────────
//
// Each wide tier needs the same shape: parse a signed wide-int from
// the string, lift to the typed decimal, call the kernel, then
// compute `|actual - expected|` and compare against `MAX_LSB_DELTA`
// (one LSB of storage). The macro takes the typed decimal alias and
// the underlying signed wide-int type; the rest is identical
// across tiers.

macro_rules! decl_wide_band {
    (
        mod $modname:ident,
        type $D:ty,
        storage $Int:ty,
        feature_gate $($cfg:meta)*,
        files {
            ln   = $ln:literal,
            exp  = $exp:literal,
            sin  = $sin:literal,
            cos  = $cos:literal,
            tan  = $tan:literal,
            atan = $atan:literal,
            sqrt = $sqrt:literal,
            cbrt = $cbrt:literal,
        }
    ) => {
        #[$($cfg)*]
        mod $modname {
            use super::{MAX_LSB_DELTA, parse_line};
            type D = $D;
            type Int = $Int;

            fn parse_int(s: &str) -> Int {
                <Int>::from_str_radix(s, 10).expect("parse wide int")
            }

            fn abs_int(x: Int) -> Int {
                let zero = <Int>::from_i128(0);
                if x < zero { zero - x } else { x }
            }

            fn call(func: &str, raw: Int) -> Int {
                let d = <D>::from_bits(raw);
                match func {
                    "ln"   => d.ln_strict().to_bits(),
                    "exp"  => d.exp_strict().to_bits(),
                    "sin"  => d.sin_strict().to_bits(),
                    "cos"  => d.cos_strict().to_bits(),
                    "tan"  => d.tan_strict().to_bits(),
                    "atan" => d.atan_strict().to_bits(),
                    "sqrt" => d.sqrt_strict().to_bits(),
                    "cbrt" => d.cbrt_strict().to_bits(),
                    other  => panic!("unknown function: {other}"),
                }
            }

            fn check(func: &str, table: &str) {
                let cap = <Int>::from_i128(MAX_LSB_DELTA);
                let mut failures = 0usize;
                for line in table.lines() {
                    let Some((lhs, rhs)) = parse_line(line) else { continue };
                    let raw_in = parse_int(lhs);
                    let expected = parse_int(rhs);
                    let actual = call(func, raw_in);
                    let delta = abs_int(actual - expected);
                    if delta > cap {
                        if failures < 16 {
                            eprintln!(
                                "FAIL: {func} {} input={raw_in} expected={expected} \
                                 actual={actual} delta={delta} LSB",
                                stringify!($modname),
                            );
                        }
                        failures += 1;
                    }
                }
                assert!(
                    failures == 0,
                    "{}: {func}: {failures} cases exceeded {MAX_LSB_DELTA} LSB",
                    stringify!($modname),
                );
            }

            #[test] fn ln()   { check("ln",   include_str!($ln)); }
            #[test] fn exp()  { check("exp",  include_str!($exp)); }
            #[test] fn sin()  { check("sin",  include_str!($sin)); }
            #[test] fn cos()  { check("cos",  include_str!($cos)); }
            #[test] fn tan()  { check("tan",  include_str!($tan)); }
            #[test] fn atan() { check("atan", include_str!($atan)); }
            #[test] fn sqrt() { check("sqrt", include_str!($sqrt)); }
            #[test] fn cbrt() { check("cbrt", include_str!($cbrt)); }
        }
    };
}

// ─── D76<35> ───────────────────────────────────────────────────────────

decl_wide_band! {
    mod d76,
    type decimal_scaled::D76<35>,
    storage decimal_scaled::Int256,
    feature_gate cfg(any(feature = "d76", feature = "wide")),
    files {
        ln   = "golden/ln_d76_s35.txt",
        exp  = "golden/exp_d76_s35.txt",
        sin  = "golden/sin_d76_s35.txt",
        cos  = "golden/cos_d76_s35.txt",
        tan  = "golden/tan_d76_s35.txt",
        atan = "golden/atan_d76_s35.txt",
        sqrt = "golden/sqrt_d76_s35.txt",
        cbrt = "golden/cbrt_d76_s35.txt",
    }
}

// ─── D153<76> ──────────────────────────────────────────────────────────

decl_wide_band! {
    mod d153,
    type decimal_scaled::D153<76>,
    storage decimal_scaled::Int512,
    feature_gate cfg(any(feature = "d153", feature = "wide")),
    files {
        ln   = "golden/ln_d153_s76.txt",
        exp  = "golden/exp_d153_s76.txt",
        sin  = "golden/sin_d153_s76.txt",
        cos  = "golden/cos_d153_s76.txt",
        tan  = "golden/tan_d153_s76.txt",
        atan = "golden/atan_d153_s76.txt",
        sqrt = "golden/sqrt_d153_s76.txt",
        cbrt = "golden/cbrt_d153_s76.txt",
    }
}

// ─── D307<150> ─────────────────────────────────────────────────────────

decl_wide_band! {
    mod d307,
    type decimal_scaled::D307<150>,
    storage decimal_scaled::Int1024,
    feature_gate cfg(any(feature = "d307", feature = "x-wide")),
    files {
        ln   = "golden/ln_d307_s150.txt",
        exp  = "golden/exp_d307_s150.txt",
        sin  = "golden/sin_d307_s150.txt",
        cos  = "golden/cos_d307_s150.txt",
        tan  = "golden/tan_d307_s150.txt",
        atan = "golden/atan_d307_s150.txt",
        sqrt = "golden/sqrt_d307_s150.txt",
        cbrt = "golden/cbrt_d307_s150.txt",
    }
}

// ─── D616<308> ─────────────────────────────────────────────────────────

decl_wide_band! {
    mod d616,
    type decimal_scaled::D616<308>,
    storage decimal_scaled::Int2048,
    feature_gate cfg(any(feature = "d616", feature = "x-wide")),
    files {
        ln   = "golden/ln_d616_s308.txt",
        exp  = "golden/exp_d616_s308.txt",
        sin  = "golden/sin_d616_s308.txt",
        cos  = "golden/cos_d616_s308.txt",
        tan  = "golden/tan_d616_s308.txt",
        atan = "golden/atan_d616_s308.txt",
        sqrt = "golden/sqrt_d616_s308.txt",
        cbrt = "golden/cbrt_d616_s308.txt",
    }
}

// ─── D1232<615> ────────────────────────────────────────────────────────

decl_wide_band! {
    mod d1232,
    type decimal_scaled::D1232<615>,
    storage decimal_scaled::Int4096,
    feature_gate cfg(any(feature = "d1232", feature = "xx-wide")),
    files {
        ln   = "golden/ln_d1232_s615.txt",
        exp  = "golden/exp_d1232_s615.txt",
        sin  = "golden/sin_d1232_s615.txt",
        cos  = "golden/cos_d1232_s615.txt",
        tan  = "golden/tan_d1232_s615.txt",
        atan = "golden/atan_d1232_s615.txt",
        sqrt = "golden/sqrt_d1232_s615.txt",
        cbrt = "golden/cbrt_d1232_s615.txt",
    }
}

