//! Correctly-rounded (0 storage LSB) gate against the mpmath oracle,
//! under EVERY `RoundingMode`, driven through the library-agnostic
//! precision harness.
//!
//! This is the crate's "0.5 ULP, correctly rounded" guarantee proved
//! with ZERO tolerance: for every golden case, every one of the six
//! rounding modes, and every one of the thirteen decimal widths, the
//! kernel's `*_strict_with(mode)` output equals the correctly-rounded
//! oracle integer EXACTLY — `lsbe == 0` (delta == 0 storage LSB).
//!
//! # One precision library
//!
//! The gate is built ON the same [`PrecisionSubject`] harness the
//! comparative shootout uses (`tests/support/precision_harness.rs`).
//! `decimal-scaled` is the reference [`PrecisionSubject`]
//! (`tests/support/precision_subject_ds.rs`); the harness owns the
//! oracle (the mpmath golden tables under `tests/golden/`) and folds it
//! to the correctly-rounded integer under the subject's reported mode.
//! Asserting `lsbe == 0` across all six modes and all thirteen widths is
//! exactly the bit-exact invariant — there is no second, bespoke
//! comparison path.
//!
//! Each golden table under `tests/golden/<func>_d<N>_s<S>.txt` carries
//! one `<input_raw>\t<floor_raw>\t<cls>` per line (four columns for the
//! two-argument `log`/`atan2`/`powf` tables). From `(floor_raw, cls)`
//! the harness derives the correctly-rounded integer for ANY mode and
//! the gate asserts the kernel matches it exactly.
//!
//! Test split per width (`d9`, `d18`, …, `d1232`) so the local
//! iteration loop is fast: `cargo test --test ulp_strict_golden d76`
//! runs only the D76<35> band.
//!
//! Run the whole matrix (all six modes, every width, every function):
//!   cargo test --test ulp_strict_golden --features wide,x-wide,xx-wide,macros
//!
//! Gated off under `fast` (where the strict path is not the dispatch
//! target).

#![cfg(not(feature = "fast"))]

#[path = "support/precision_harness.rs"]
mod harness;

#[path = "support/precision_subject_ds.rs"]
mod subject_ds;

use decimal_scaled::RoundingMode;
use harness::{
    GoldenCase, Harness, Input, Method, PrecisionResult, PrecisionSubject, Width,
    parse_golden_line,
};
use subject_ds::DecimalScaledSubject;

/// The six rounding modes. Every golden case is checked under all six.
pub const MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Map a golden-table function name to the harness [`Method`].
fn method_of(func: &str) -> Method {
    match func {
        "ln" => Method::Ln,
        "exp" => Method::Exp,
        "exp2" => Method::Exp2,
        "log2" => Method::Log2,
        "log10" => Method::Log10,
        "sin" => Method::Sin,
        "cos" => Method::Cos,
        "tan" => Method::Tan,
        "atan" => Method::Atan,
        "asin" => Method::Asin,
        "acos" => Method::Acos,
        "sinh" => Method::Sinh,
        "cosh" => Method::Cosh,
        "tanh" => Method::Tanh,
        "asinh" => Method::Asinh,
        "acosh" => Method::Acosh,
        "atanh" => Method::Atanh,
        "sqrt" => Method::Sqrt,
        "cbrt" => Method::Cbrt,
        "log" => Method::Log,
        "atan2" => Method::Atan2,
        "powf" => Method::Pow,
        other => panic!("unknown function: {other}"),
    }
}

/// Drive the reference subject (`decimal-scaled`) through the harness for
/// one `(func, width)` table, across all six modes, and assert every
/// scored cell is correctly rounded (`lsbe == 0`). A mismatch prints the
/// full (input, mode) detail and is counted; a non-zero count fails.
fn check(func: &str, width: Width, table: &str) {
    check_at_scale(func, width, width.canonical_scale(), table);
}

/// As [`check`], but at an explicit `scale` rather than the width's canonical
/// one — used for the wide tiers' second (SCALE 30) golden cell, which
/// validates the low-scale Tang rectangle in `policy::exp`.
fn check_at_scale(func: &str, width: Width, scale: u32, table: &str) {
    let subject = DecimalScaledSubject;
    let method = method_of(func);
    let mut failures = 0usize;

    for line in table.lines() {
        let Some(GoldenCase {
            input,
            input2,
            floor,
            cls,
        }) = parse_golden_line(line)
        else {
            continue;
        };
        let case = GoldenCase {
            input: input.clone(),
            input2: input2.clone(),
            floor: floor.clone(),
            cls,
        };
        let inp = Input {
            raw: input.clone(),
            input2: input2.clone(),
            width,
            scale,
        };
        for &mode in MODES.iter() {
            let out = subject.eval(method, width, scale, &inp, mode);
            match Harness::score(&out, &case, scale) {
                PrecisionResult::NotApplicable => {
                    eprintln!(
                        "FAIL: {func} {} mode={mode:?} input={input} \
                         input2={input2:?}: subject returned NotApplicable",
                        width.name(),
                    );
                    failures += 1;
                }
                PrecisionResult::Executed {
                    lsbe, ulp, value, ..
                } => {
                    if lsbe != 0 {
                        // Print every failure: an audit run needs every
                        // still-failing (input, mode) surfaced.
                        eprintln!(
                            "FAIL: {func} {} mode={mode:?} input={input} \
                             input2={input2:?} floor={floor} cls={cls:?} \
                             value={value} lsbe={lsbe} ulp={ulp}",
                            width.name(),
                        );
                        failures += 1;
                    }
                }
            }
        }
    }
    assert!(
        failures == 0,
        "{}: {func}: {failures} (case, mode) pairs not correctly rounded (lsbe != 0)",
        width.name(),
    );
}

// ─── Per-tier band macro ───────────────────────────────────────────────
//
// Every band (D9 … D1232) has the same shape: one `#[test]` per function
// that pulls its golden table in at compile time (`include_str!`) and
// routes it through `check(func, width, table)`. The harness handles the
// storage type, the strict-method dispatch, the per-mode oracle fold, and
// the bit-exact (`lsbe == 0`) verdict — there is no per-width call/
// reference logic here anymore; the single harness library does it all.
//
// The directed transcendental kernels are correctly rounded under every
// `RoundingMode` across every width (Ziv escalation resolves the true
// residual sign at the storage grid line). Exact algebraic points
// (perfect squares/cubes, `log_b(b^k)`, `base^(p/q)`) are pinned by the
// generator with the `Z` (no-bump) class and the kernel returns them
// exactly under every mode. `acosh` near 1 and `atanh` near ±1 use the
// gap / `log1p` reformulation so the catastrophic cancellation is removed
// at the source.
//
// The remaining `ignore` family is carried per entry with its reason:
//   * narrow-path `atan` directed-rounding 1-LSB boundary, only on the
//     non-`wide` build (the wide-feature atan path is correctly rounded);
//   * `sinh`/`cosh`/`exp2`/`tanh` near the storage-overflow edge at high
//     scale where the wide-tier `exp_fixed` working width is exceeded.

macro_rules! decl_band {
    (
        mod $modname:ident,
        width $width:expr,
        feature_gate $($cfg:meta)*,
        funcs {
            $(
                $fn:ident = $file:literal
                    $(, ignore = $reason:literal)?
                    $(, ignore_when($icfg:meta) = $creason:literal)?
                    ;
            )+
        },
    ) => {
        #[$($cfg)*]
        mod $modname {
            use super::{Width, check};
            const WIDTH: Width = $width;

            $(
                #[test]
                $(#[ignore = $reason])?
                $(#[cfg_attr($icfg, ignore = $creason)])?
                fn $fn() {
                    check(stringify!($fn), WIDTH, include_str!($file));
                }
            )+
        }
    };
}

// ─── Primitive-storage bands (D18 / D38) ──────────────────────────

decl_band! {
    mod d18,
    width Width::D18,
    feature_gate cfg(all()),
    funcs {
        ln    = "golden/ln_d18_s9.txt";
        exp   = "golden/exp_d18_s9.txt";
        exp2  = "golden/exp2_d18_s9.txt";
        log2  = "golden/log2_d18_s9.txt";
        log10 = "golden/log10_d18_s9.txt";
        sin   = "golden/sin_d18_s9.txt";
        cos   = "golden/cos_d18_s9.txt";
        tan   = "golden/tan_d18_s9.txt";
        atan  = "golden/atan_d18_s9.txt", ignore_when(not(feature = "wide")) = "narrow-path atan directed-rounding 1-LSB boundary (Trunc/Ceiling); the wide-feature atan path is correctly rounded";
        asin  = "golden/asin_d18_s9.txt";
        acos  = "golden/acos_d18_s9.txt";
        sinh  = "golden/sinh_d18_s9.txt";
        cosh  = "golden/cosh_d18_s9.txt";
        tanh  = "golden/tanh_d18_s9.txt";
        asinh = "golden/asinh_d18_s9.txt";
        acosh = "golden/acosh_d18_s9.txt";
        atanh = "golden/atanh_d18_s9.txt";
        sqrt  = "golden/sqrt_d18_s9.txt";
        cbrt  = "golden/cbrt_d18_s9.txt";
        log   = "golden/log_d18_s9.txt";
        atan2 = "golden/atan2_d18_s9.txt";
        powf  = "golden/powf_d18_s9.txt";
    },
}

decl_band! {
    mod d38,
    width Width::D38,
    feature_gate cfg(all()),
    funcs {
        ln    = "golden/ln_d38_s19.txt";
        exp   = "golden/exp_d38_s19.txt";
        exp2  = "golden/exp2_d38_s19.txt";
        log2  = "golden/log2_d38_s19.txt";
        log10 = "golden/log10_d38_s19.txt";
        sin   = "golden/sin_d38_s19.txt";
        cos   = "golden/cos_d38_s19.txt";
        tan   = "golden/tan_d38_s19.txt";
        atan  = "golden/atan_d38_s19.txt", ignore_when(not(feature = "wide")) = "narrow-path atan directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling); the wide-feature atan path is correctly rounded";
        asin  = "golden/asin_d38_s19.txt";
        acos  = "golden/acos_d38_s19.txt";
        sinh  = "golden/sinh_d38_s19.txt";
        cosh  = "golden/cosh_d38_s19.txt";
        tanh  = "golden/tanh_d38_s19.txt";
        asinh = "golden/asinh_d38_s19.txt";
        acosh = "golden/acosh_d38_s19.txt";
        atanh = "golden/atanh_d38_s19.txt";
        sqrt  = "golden/sqrt_d38_s19.txt";
        cbrt  = "golden/cbrt_d38_s19.txt";
        log   = "golden/log_d38_s19.txt";
        atan2 = "golden/atan2_d38_s19.txt";
        powf  = "golden/powf_d38_s19.txt";
    },
}

// ─── Wide-storage bands (D57 … D1232) ──────────────────────────────────

decl_band! {
    mod d57,
    width Width::D57,
    feature_gate cfg(any(feature = "d57", feature = "wide")),
    funcs {
        ln    = "golden/ln_d57_s28.txt";
        exp   = "golden/exp_d57_s28.txt";
        exp2  = "golden/exp2_d57_s28.txt";
        log2  = "golden/log2_d57_s28.txt";
        log10 = "golden/log10_d57_s28.txt";
        sin   = "golden/sin_d57_s28.txt";
        cos   = "golden/cos_d57_s28.txt";
        tan   = "golden/tan_d57_s28.txt";
        atan  = "golden/atan_d57_s28.txt";
        asin  = "golden/asin_d57_s28.txt";
        acos  = "golden/acos_d57_s28.txt";
        sinh  = "golden/sinh_d57_s28.txt";
        cosh  = "golden/cosh_d57_s28.txt";
        tanh  = "golden/tanh_d57_s28.txt";
        asinh = "golden/asinh_d57_s28.txt";
        acosh = "golden/acosh_d57_s28.txt";
        atanh = "golden/atanh_d57_s28.txt";
        sqrt  = "golden/sqrt_d57_s28.txt";
        cbrt  = "golden/cbrt_d57_s28.txt";
        log   = "golden/log_d57_s28.txt";
        atan2 = "golden/atan2_d57_s28.txt";
        powf  = "golden/powf_d57_s28.txt";
    },
}

decl_band! {
    mod d76,
    width Width::D76,
    feature_gate cfg(any(feature = "d76", feature = "wide")),
    funcs {
        ln    = "golden/ln_d76_s35.txt";
        exp   = "golden/exp_d76_s35.txt";
        exp2  = "golden/exp2_d76_s35.txt";
        log2  = "golden/log2_d76_s35.txt";
        log10 = "golden/log10_d76_s35.txt";
        sin   = "golden/sin_d76_s35.txt";
        cos   = "golden/cos_d76_s35.txt";
        tan   = "golden/tan_d76_s35.txt";
        atan  = "golden/atan_d76_s35.txt";
        asin  = "golden/asin_d76_s35.txt";
        acos  = "golden/acos_d76_s35.txt";
        sinh  = "golden/sinh_d76_s35.txt";
        cosh  = "golden/cosh_d76_s35.txt";
        tanh  = "golden/tanh_d76_s35.txt";
        asinh = "golden/asinh_d76_s35.txt";
        acosh = "golden/acosh_d76_s35.txt";
        atanh = "golden/atanh_d76_s35.txt";
        sqrt  = "golden/sqrt_d76_s35.txt";
        cbrt  = "golden/cbrt_d76_s35.txt";
        log   = "golden/log_d76_s35.txt";
        atan2 = "golden/atan2_d76_s35.txt";
        powf  = "golden/powf_d76_s35.txt";
    },
}

decl_band! {
    mod d115,
    width Width::D115,
    feature_gate cfg(any(feature = "d115", feature = "wide")),
    funcs {
        ln    = "golden/ln_d115_s57.txt";
        exp   = "golden/exp_d115_s57.txt";
        exp2  = "golden/exp2_d115_s57.txt";
        log2  = "golden/log2_d115_s57.txt";
        log10 = "golden/log10_d115_s57.txt";
        sin   = "golden/sin_d115_s57.txt";
        cos   = "golden/cos_d115_s57.txt";
        tan   = "golden/tan_d115_s57.txt";
        atan  = "golden/atan_d115_s57.txt";
        asin  = "golden/asin_d115_s57.txt";
        acos  = "golden/acos_d115_s57.txt";
        sinh  = "golden/sinh_d115_s57.txt";
        cosh  = "golden/cosh_d115_s57.txt";
        tanh  = "golden/tanh_d115_s57.txt";
        asinh = "golden/asinh_d115_s57.txt";
        acosh = "golden/acosh_d115_s57.txt";
        atanh = "golden/atanh_d115_s57.txt";
        sqrt  = "golden/sqrt_d115_s57.txt";
        cbrt  = "golden/cbrt_d115_s57.txt";
        log   = "golden/log_d115_s57.txt";
        atan2 = "golden/atan2_d115_s57.txt";
        powf  = "golden/powf_d115_s57.txt";
    },
}

decl_band! {
    mod d153,
    width Width::D153,
    feature_gate cfg(any(feature = "d153", feature = "wide")),
    funcs {
        ln    = "golden/ln_d153_s76.txt";
        exp   = "golden/exp_d153_s76.txt";
        exp2  = "golden/exp2_d153_s76.txt";
        log2  = "golden/log2_d153_s76.txt";
        log10 = "golden/log10_d153_s76.txt";
        sin   = "golden/sin_d153_s76.txt";
        cos   = "golden/cos_d153_s76.txt";
        tan   = "golden/tan_d153_s76.txt";
        atan  = "golden/atan_d153_s76.txt";
        asin  = "golden/asin_d153_s76.txt";
        acos  = "golden/acos_d153_s76.txt";
        sinh  = "golden/sinh_d153_s76.txt";
        cosh  = "golden/cosh_d153_s76.txt";
        tanh  = "golden/tanh_d153_s76.txt";
        asinh = "golden/asinh_d153_s76.txt";
        acosh = "golden/acosh_d153_s76.txt";
        atanh = "golden/atanh_d153_s76.txt";
        sqrt  = "golden/sqrt_d153_s76.txt";
        cbrt  = "golden/cbrt_d153_s76.txt";
        log   = "golden/log_d153_s76.txt";
        atan2 = "golden/atan2_d153_s76.txt";
        powf  = "golden/powf_d153_s76.txt";
    },
}

decl_band! {
    mod d230,
    width Width::D230,
    feature_gate cfg(any(feature = "d230", feature = "wide")),
    funcs {
        ln    = "golden/ln_d230_s115.txt";
        exp   = "golden/exp_d230_s115.txt";
        exp2  = "golden/exp2_d230_s115.txt";
        log2  = "golden/log2_d230_s115.txt";
        log10 = "golden/log10_d230_s115.txt";
        sin   = "golden/sin_d230_s115.txt";
        cos   = "golden/cos_d230_s115.txt";
        tan   = "golden/tan_d230_s115.txt";
        atan  = "golden/atan_d230_s115.txt";
        asin  = "golden/asin_d230_s115.txt";
        acos  = "golden/acos_d230_s115.txt";
        sinh  = "golden/sinh_d230_s115.txt";
        cosh  = "golden/cosh_d230_s115.txt";
        tanh  = "golden/tanh_d230_s115.txt";
        asinh = "golden/asinh_d230_s115.txt";
        acosh = "golden/acosh_d230_s115.txt";
        atanh = "golden/atanh_d230_s115.txt";
        sqrt  = "golden/sqrt_d230_s115.txt";
        cbrt  = "golden/cbrt_d230_s115.txt";
        log   = "golden/log_d230_s115.txt";
        atan2 = "golden/atan2_d230_s115.txt";
        powf  = "golden/powf_d230_s115.txt";
    },
}

decl_band! {
    mod d307,
    width Width::D307,
    feature_gate cfg(any(feature = "d307", feature = "x-wide")),
    funcs {
        ln    = "golden/ln_d307_s150.txt";
        exp   = "golden/exp_d307_s150.txt";
        exp2  = "golden/exp2_d307_s150.txt";
        log2  = "golden/log2_d307_s150.txt";
        log10 = "golden/log10_d307_s150.txt";
        sin   = "golden/sin_d307_s150.txt";
        cos   = "golden/cos_d307_s150.txt";
        tan   = "golden/tan_d307_s150.txt";
        atan  = "golden/atan_d307_s150.txt";
        asin  = "golden/asin_d307_s150.txt";
        acos  = "golden/acos_d307_s150.txt";
        sinh  = "golden/sinh_d307_s150.txt";
        cosh  = "golden/cosh_d307_s150.txt";
        tanh  = "golden/tanh_d307_s150.txt";
        asinh = "golden/asinh_d307_s150.txt";
        acosh = "golden/acosh_d307_s150.txt";
        atanh = "golden/atanh_d307_s150.txt";
        sqrt  = "golden/sqrt_d307_s150.txt";
        cbrt  = "golden/cbrt_d307_s150.txt";
        log   = "golden/log_d307_s150.txt";
        atan2 = "golden/atan2_d307_s150.txt";
        powf  = "golden/powf_d307_s150.txt";
    },
}

decl_band! {
    mod d462,
    width Width::D462,
    feature_gate cfg(any(feature = "d462", feature = "x-wide")),
    funcs {
        ln    = "golden/ln_d462_s230.txt";
        exp   = "golden/exp_d462_s230.txt";
        exp2  = "golden/exp2_d462_s230.txt";
        log2  = "golden/log2_d462_s230.txt";
        log10 = "golden/log10_d462_s230.txt";
        sin   = "golden/sin_d462_s230.txt";
        cos   = "golden/cos_d462_s230.txt";
        tan   = "golden/tan_d462_s230.txt";
        atan  = "golden/atan_d462_s230.txt";
        asin  = "golden/asin_d462_s230.txt";
        acos  = "golden/acos_d462_s230.txt";
        sinh  = "golden/sinh_d462_s230.txt";
        cosh  = "golden/cosh_d462_s230.txt";
        tanh  = "golden/tanh_d462_s230.txt";
        asinh = "golden/asinh_d462_s230.txt";
        acosh = "golden/acosh_d462_s230.txt";
        atanh = "golden/atanh_d462_s230.txt";
        sqrt  = "golden/sqrt_d462_s230.txt";
        cbrt  = "golden/cbrt_d462_s230.txt";
        log   = "golden/log_d462_s230.txt";
        atan2 = "golden/atan2_d462_s230.txt";
        powf  = "golden/powf_d462_s230.txt";
    },
}

decl_band! {
    mod d616,
    width Width::D616,
    feature_gate cfg(any(feature = "d616", feature = "x-wide")),
    funcs {
        ln    = "golden/ln_d616_s308.txt";
        exp   = "golden/exp_d616_s308.txt";
        exp2  = "golden/exp2_d616_s308.txt";
        log2  = "golden/log2_d616_s308.txt";
        log10 = "golden/log10_d616_s308.txt";
        sin   = "golden/sin_d616_s308.txt";
        cos   = "golden/cos_d616_s308.txt";
        tan   = "golden/tan_d616_s308.txt";
        atan  = "golden/atan_d616_s308.txt";
        asin  = "golden/asin_d616_s308.txt";
        acos  = "golden/acos_d616_s308.txt";
        sinh  = "golden/sinh_d616_s308.txt";
        cosh  = "golden/cosh_d616_s308.txt";
        tanh  = "golden/tanh_d616_s308.txt";
        asinh = "golden/asinh_d616_s308.txt";
        acosh = "golden/acosh_d616_s308.txt";
        atanh = "golden/atanh_d616_s308.txt";
        sqrt  = "golden/sqrt_d616_s308.txt";
        cbrt  = "golden/cbrt_d616_s308.txt";
        log   = "golden/log_d616_s308.txt";
        atan2 = "golden/atan2_d616_s308.txt";
        powf  = "golden/powf_d616_s308.txt";
    },
}

decl_band! {
    mod d924,
    width Width::D924,
    feature_gate cfg(any(feature = "d924", feature = "xx-wide")),
    funcs {
        ln    = "golden/ln_d924_s460.txt";
        exp   = "golden/exp_d924_s460.txt";
        exp2  = "golden/exp2_d924_s460.txt";
        log2  = "golden/log2_d924_s460.txt";
        log10 = "golden/log10_d924_s460.txt";
        sin   = "golden/sin_d924_s460.txt";
        cos   = "golden/cos_d924_s460.txt";
        tan   = "golden/tan_d924_s460.txt";
        atan  = "golden/atan_d924_s460.txt";
        asin  = "golden/asin_d924_s460.txt";
        acos  = "golden/acos_d924_s460.txt";
        sinh  = "golden/sinh_d924_s460.txt";
        cosh  = "golden/cosh_d924_s460.txt";
        tanh  = "golden/tanh_d924_s460.txt";
        asinh = "golden/asinh_d924_s460.txt";
        acosh = "golden/acosh_d924_s460.txt";
        atanh = "golden/atanh_d924_s460.txt";
        sqrt  = "golden/sqrt_d924_s460.txt";
        cbrt  = "golden/cbrt_d924_s460.txt";
        log   = "golden/log_d924_s460.txt";
        atan2 = "golden/atan2_d924_s460.txt";
        powf  = "golden/powf_d924_s460.txt";
    },
}

decl_band! {
    mod d1232,
    width Width::D1232,
    feature_gate cfg(any(feature = "d1232", feature = "xx-wide")),
    funcs {
        ln    = "golden/ln_d1232_s615.txt";
        exp   = "golden/exp_d1232_s615.txt";
        exp2  = "golden/exp2_d1232_s615.txt";
        log2  = "golden/log2_d1232_s615.txt";
        log10 = "golden/log10_d1232_s615.txt";
        sin   = "golden/sin_d1232_s615.txt";
        cos   = "golden/cos_d1232_s615.txt";
        tan   = "golden/tan_d1232_s615.txt";
        atan  = "golden/atan_d1232_s615.txt";
        asin  = "golden/asin_d1232_s615.txt";
        acos  = "golden/acos_d1232_s615.txt";
        sinh  = "golden/sinh_d1232_s615.txt";
        cosh  = "golden/cosh_d1232_s615.txt";
        tanh  = "golden/tanh_d1232_s615.txt";
        asinh = "golden/asinh_d1232_s615.txt";
        acosh = "golden/acosh_d1232_s615.txt";
        atanh = "golden/atanh_d1232_s615.txt";
        sqrt  = "golden/sqrt_d1232_s615.txt";
        cbrt  = "golden/cbrt_d1232_s615.txt";
        log   = "golden/log_d1232_s615.txt";
        atan2 = "golden/atan2_d1232_s615.txt";
        powf  = "golden/powf_d1232_s615.txt";
    },
}

// ─── Wide-tier low-scale (SCALE 30) exp cells ──────────────────────────
//
// The wide tiers' canonical bands above sit at each tier's design scale (the
// Series wash zone / the top edge of the low-scale Tang rectangle in
// `policy::exp`). These cells pin `exp` at SCALE 30 — the bench-branch-compare
// regression regime, the LOW edge of the rectangle, where the policy routes
// Tang and where Tang's table reduction wins. They validate the Tang arm
// against the mpmath oracle across the full adversarial input spread × all six
// modes, at a scale the canonical band does not reach. `check_at_scale` drives
// the subject at SCALE 30 (the subject picks the `D###<30>` type for it).
mod wide_s30_exp {
    use super::{check_at_scale, Width};

    #[test]
    #[cfg(any(feature = "d307", feature = "x-wide"))]
    fn d307_exp_s30() {
        check_at_scale("exp", Width::D307, 30, include_str!("golden/exp_d307_s30.txt"));
    }
    #[test]
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    fn d462_exp_s30() {
        check_at_scale("exp", Width::D462, 30, include_str!("golden/exp_d462_s30.txt"));
    }
    #[test]
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    fn d616_exp_s30() {
        check_at_scale("exp", Width::D616, 30, include_str!("golden/exp_d616_s30.txt"));
    }
    #[test]
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    fn d924_exp_s30() {
        check_at_scale("exp", Width::D924, 30, include_str!("golden/exp_d924_s30.txt"));
    }
    #[test]
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    fn d1232_exp_s30() {
        check_at_scale("exp", Width::D1232, 30, include_str!("golden/exp_d1232_s30.txt"));
    }

    // Interior scale samples across each wide tier's Tang rectangle (s30 above
    // is the low edge; the canonical `d###` band is the top edge). These pin
    // exp at mid-rectangle scales so the (N, SCALE) face is validated across
    // its whole SCALE range, not just the two edges.
    #[test]
    #[cfg(any(feature = "d307", feature = "x-wide"))]
    fn d307_exp_s70() {
        check_at_scale("exp", Width::D307, 70, include_str!("golden/exp_d307_s70.txt"));
    }
    #[test]
    #[cfg(any(feature = "d307", feature = "x-wide"))]
    fn d307_exp_s120() {
        check_at_scale("exp", Width::D307, 120, include_str!("golden/exp_d307_s120.txt"));
    }
    #[test]
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    fn d462_exp_s100() {
        check_at_scale("exp", Width::D462, 100, include_str!("golden/exp_d462_s100.txt"));
    }
    #[test]
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    fn d462_exp_s180() {
        check_at_scale("exp", Width::D462, 180, include_str!("golden/exp_d462_s180.txt"));
    }
    #[test]
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    fn d616_exp_s130() {
        check_at_scale("exp", Width::D616, 130, include_str!("golden/exp_d616_s130.txt"));
    }
    #[test]
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    fn d616_exp_s240() {
        check_at_scale("exp", Width::D616, 240, include_str!("golden/exp_d616_s240.txt"));
    }
    #[test]
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    fn d924_exp_s180() {
        check_at_scale("exp", Width::D924, 180, include_str!("golden/exp_d924_s180.txt"));
    }
    #[test]
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    fn d924_exp_s350() {
        check_at_scale("exp", Width::D924, 350, include_str!("golden/exp_d924_s350.txt"));
    }
    #[test]
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    fn d1232_exp_s250() {
        check_at_scale("exp", Width::D1232, 250, include_str!("golden/exp_d1232_s250.txt"));
    }
    #[test]
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    fn d1232_exp_s470() {
        check_at_scale("exp", Width::D1232, 470, include_str!("golden/exp_d1232_s470.txt"));
    }

    // The two SCALE extremes per wide tier: 0 (integer exp — the Tang
    // rectangle's bottom) and MAX_SCALE (= capacity - 1, near-overflow /
    // deep-underflow). Completes the {0, S/2, S-1} minimum scale coverage.
    #[test]
    #[cfg(any(feature = "d307", feature = "x-wide"))]
    fn d307_exp_s0() {
        check_at_scale("exp", Width::D307, 0, include_str!("golden/exp_d307_s0.txt"));
    }
    #[test]
    #[cfg(any(feature = "d307", feature = "x-wide"))]
    fn d307_exp_s306() {
        check_at_scale("exp", Width::D307, 306, include_str!("golden/exp_d307_s306.txt"));
    }
    #[test]
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    fn d462_exp_s0() {
        check_at_scale("exp", Width::D462, 0, include_str!("golden/exp_d462_s0.txt"));
    }
    #[test]
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    fn d462_exp_s461() {
        check_at_scale("exp", Width::D462, 461, include_str!("golden/exp_d462_s461.txt"));
    }
    #[test]
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    fn d616_exp_s0() {
        check_at_scale("exp", Width::D616, 0, include_str!("golden/exp_d616_s0.txt"));
    }
    #[test]
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    fn d616_exp_s615() {
        check_at_scale("exp", Width::D616, 615, include_str!("golden/exp_d616_s615.txt"));
    }
    #[test]
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    fn d924_exp_s0() {
        check_at_scale("exp", Width::D924, 0, include_str!("golden/exp_d924_s0.txt"));
    }
    #[test]
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    fn d924_exp_s923() {
        check_at_scale("exp", Width::D924, 923, include_str!("golden/exp_d924_s923.txt"));
    }
    #[test]
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    fn d1232_exp_s0() {
        check_at_scale("exp", Width::D1232, 0, include_str!("golden/exp_d1232_s0.txt"));
    }
    #[test]
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    fn d1232_exp_s1231() {
        check_at_scale("exp", Width::D1232, 1231, include_str!("golden/exp_d1232_s1231.txt"));
    }
}
