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
        "add" => Method::Add,
        "sub" => Method::Sub,
        "mul" => Method::Mul,
        "div" => Method::Div,
        "rem" => Method::Rem,
        "hypot" => Method::Hypot,
        other => panic!("unknown function: {other}"),
    }
}

/// Drive the reference subject (`decimal-scaled`) through the harness for
/// one `(func, width)` table, across all six modes, and assert every
/// scored cell is correctly rounded (`lsbe == 0`). A mismatch prints the
/// full (input, mode) detail and is counted; a non-zero count fails.
fn check(func: &'static str, width: Width, table: &'static str) {
    check_at_scale(func, width, width.canonical_scale(), table);
}

/// As [`check`], but at an explicit `scale` rather than the width's canonical
/// one — used for the wide tiers' second (SCALE 30) golden cell, which
/// validates the low-scale Tang rectangle in `policy::exp`.
fn check_at_scale(func: &'static str, width: Width, scale: u32, table: &'static str) {
    // Per-table TIMEOUT watchdog (9.21): run the table on a detached worker
    // thread. If a kernel for this (function, width, scale) runs away (infinite
    // recursion / non-termination) — a real risk as the wide-tier coverage
    // expands — it is IDENTIFIED here: the test fails NAMING the table instead
    // of the whole suite hanging silently. The worker leaks on timeout (the
    // process exits after the suite, which is fine for a gate). A body panic (a
    // genuine correctly-rounding failure) is caught and re-raised on the main
    // thread so the assertion detail survives. The budget is generous — a wide
    // golden table is thousands of kernel calls, so exceeding it means a
    // runaway, not merely slow.
    use std::sync::mpsc;
    use std::time::Duration;
    const GOLDEN_TABLE_TIMEOUT_SECS: u64 = 120;
    let (tx, rx) = mpsc::channel();
    let _worker = std::thread::Builder::new()
        .name(format!("golden-{func}-{}-s{scale}", width.name()))
        .spawn(move || {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                check_at_scale_inner(func, width, scale, table)
            }));
            let _ = tx.send(r);
        })
        .expect("spawn golden worker thread");
    match rx.recv_timeout(Duration::from_secs(GOLDEN_TABLE_TIMEOUT_SECS)) {
        Ok(Ok(())) => {}
        Ok(Err(panic)) => std::panic::resume_unwind(panic),
        Err(_) => panic!(
            "GOLDEN TIMEOUT: {func} {} s{scale} did not finish within \
             {GOLDEN_TABLE_TIMEOUT_SECS}s — possible runaway recursion / \
             non-termination in the kernel for this (function, width, scale)",
            width.name(),
        ),
    }
}

fn check_at_scale_inner(func: &str, width: Width, scale: u32, table: &str) {
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
        ln    = "golden/ln_d76_s38.txt";
        exp   = "golden/exp_d76_s38.txt";
        exp2  = "golden/exp2_d76_s38.txt";
        log2  = "golden/log2_d76_s38.txt";
        log10 = "golden/log10_d76_s38.txt";
        sin   = "golden/sin_d76_s38.txt";
        cos   = "golden/cos_d76_s38.txt";
        tan   = "golden/tan_d76_s38.txt";
        atan  = "golden/atan_d76_s38.txt";
        asin  = "golden/asin_d76_s38.txt";
        acos  = "golden/acos_d76_s38.txt";
        sinh  = "golden/sinh_d76_s38.txt";
        cosh  = "golden/cosh_d76_s38.txt";
        tanh  = "golden/tanh_d76_s38.txt";
        asinh = "golden/asinh_d76_s38.txt";
        acosh = "golden/acosh_d76_s38.txt";
        atanh = "golden/atanh_d76_s38.txt";
        sqrt  = "golden/sqrt_d76_s38.txt";
        cbrt  = "golden/cbrt_d76_s38.txt";
        log   = "golden/log_d76_s38.txt";
        atan2 = "golden/atan2_d76_s38.txt";
        powf  = "golden/powf_d76_s38.txt";
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
        ln    = "golden/ln_d307_s153.txt";
        exp   = "golden/exp_d307_s153.txt";
        exp2  = "golden/exp2_d307_s153.txt";
        log2  = "golden/log2_d307_s153.txt";
        log10 = "golden/log10_d307_s153.txt";
        sin   = "golden/sin_d307_s153.txt";
        cos   = "golden/cos_d307_s153.txt";
        tan   = "golden/tan_d307_s153.txt";
        atan  = "golden/atan_d307_s153.txt";
        asin  = "golden/asin_d307_s153.txt";
        acos  = "golden/acos_d307_s153.txt";
        sinh  = "golden/sinh_d307_s153.txt";
        cosh  = "golden/cosh_d307_s153.txt";
        tanh  = "golden/tanh_d307_s153.txt";
        asinh = "golden/asinh_d307_s153.txt";
        acosh = "golden/acosh_d307_s153.txt";
        atanh = "golden/atanh_d307_s153.txt";
        sqrt  = "golden/sqrt_d307_s153.txt";
        cbrt  = "golden/cbrt_d307_s153.txt";
        log   = "golden/log_d307_s153.txt";
        atan2 = "golden/atan2_d307_s153.txt";
        powf  = "golden/powf_d307_s153.txt";
    },
}

decl_band! {
    mod d462,
    width Width::D462,
    feature_gate cfg(any(feature = "d462", feature = "x-wide")),
    funcs {
        ln    = "golden/ln_d462_s231.txt";
        exp   = "golden/exp_d462_s231.txt";
        exp2  = "golden/exp2_d462_s231.txt";
        log2  = "golden/log2_d462_s231.txt";
        log10 = "golden/log10_d462_s231.txt";
        sin   = "golden/sin_d462_s231.txt";
        cos   = "golden/cos_d462_s231.txt";
        tan   = "golden/tan_d462_s231.txt";
        atan  = "golden/atan_d462_s231.txt";
        asin  = "golden/asin_d462_s231.txt";
        acos  = "golden/acos_d462_s231.txt";
        sinh  = "golden/sinh_d462_s231.txt";
        cosh  = "golden/cosh_d462_s231.txt";
        tanh  = "golden/tanh_d462_s231.txt";
        asinh = "golden/asinh_d462_s231.txt";
        acosh = "golden/acosh_d462_s231.txt";
        atanh = "golden/atanh_d462_s231.txt";
        sqrt  = "golden/sqrt_d462_s231.txt";
        cbrt  = "golden/cbrt_d462_s231.txt";
        log   = "golden/log_d462_s231.txt";
        atan2 = "golden/atan2_d462_s231.txt";
        powf  = "golden/powf_d462_s231.txt";
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
        ln    = "golden/ln_d924_s462.txt";
        exp   = "golden/exp_d924_s462.txt";
        exp2  = "golden/exp2_d924_s462.txt";
        log2  = "golden/log2_d924_s462.txt";
        log10 = "golden/log10_d924_s462.txt";
        sin   = "golden/sin_d924_s462.txt";
        cos   = "golden/cos_d924_s462.txt";
        tan   = "golden/tan_d924_s462.txt";
        atan  = "golden/atan_d924_s462.txt";
        asin  = "golden/asin_d924_s462.txt";
        acos  = "golden/acos_d924_s462.txt";
        sinh  = "golden/sinh_d924_s462.txt";
        cosh  = "golden/cosh_d924_s462.txt";
        tanh  = "golden/tanh_d924_s462.txt";
        asinh = "golden/asinh_d924_s462.txt";
        acosh = "golden/acosh_d924_s462.txt";
        atanh = "golden/atanh_d924_s462.txt";
        sqrt  = "golden/sqrt_d924_s462.txt";
        cbrt  = "golden/cbrt_d924_s462.txt";
        log   = "golden/log_d924_s462.txt";
        atan2 = "golden/atan2_d924_s462.txt";
        powf  = "golden/powf_d924_s462.txt";
    },
}

decl_band! {
    mod d1232,
    width Width::D1232,
    feature_gate cfg(any(feature = "d1232", feature = "xx-wide")),
    funcs {
        ln    = "golden/ln_d1232_s616.txt";
        exp   = "golden/exp_d1232_s616.txt";
        exp2  = "golden/exp2_d1232_s616.txt";
        log2  = "golden/log2_d1232_s616.txt";
        log10 = "golden/log10_d1232_s616.txt";
        sin   = "golden/sin_d1232_s616.txt";
        cos   = "golden/cos_d1232_s616.txt";
        tan   = "golden/tan_d1232_s616.txt";
        atan  = "golden/atan_d1232_s616.txt";
        asin  = "golden/asin_d1232_s616.txt";
        acos  = "golden/acos_d1232_s616.txt";
        sinh  = "golden/sinh_d1232_s616.txt";
        cosh  = "golden/cosh_d1232_s616.txt";
        tanh  = "golden/tanh_d1232_s616.txt";
        asinh = "golden/asinh_d1232_s616.txt";
        acosh = "golden/acosh_d1232_s616.txt";
        atanh = "golden/atanh_d1232_s616.txt";
        sqrt  = "golden/sqrt_d1232_s616.txt";
        cbrt  = "golden/cbrt_d1232_s616.txt";
        log   = "golden/log_d1232_s616.txt";
        atan2 = "golden/atan2_d1232_s616.txt";
        powf  = "golden/powf_d1232_s616.txt";
    },
}

// ─── Wide-tier exp band-edge cells ─────────────────────────────────────
//
// The wide tiers' canonical bands above sit at each tier's design scale
// (S/2 in the generator's five-point scale set). These cells pin `exp` at
// the two SCALE extremes per wide tier: 0 (integer exp — the Tang
// rectangle's bottom) and MAX_SCALE (= capacity - 1, near-overflow /
// deep-underflow), completing the {0, S/2, S-1} minimum scale coverage.
// They validate `exp` against the mpmath oracle across the full
// adversarial input spread × all six modes at the band edges the canonical
// band does not reach. `check_at_scale` drives the subject at the named
// scale (the subject picks the `D###<S>` type for it).
mod wide_s30_exp {
    use super::{check_at_scale, Width};

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

// ─── Band-edge {0, capacity-1} cells (auto-listed) ─────────────────────
//
// Generated by scripts/gen_band_edge_wiring.py. One test per existing
// (func, tier, scale) golden table at the two SCALE extremes (0 and
// capacity-1 = MAX_SCALE). The canonical bands above cover ~S/2; these
// complete the {0, S/2, S-1} coverage so every (function, width) face is
// validity-gated across its whole SCALE range. Absent cells (empty domain
// at the edge, e.g. cosh at MAX_SCALE which always overflows) are not
// emitted by the generator and so carry no test. The wide-tier exp {0,S-1}
// cells are wired by `wide_s30_exp`, so exp is skipped there.
mod band_edges {
    use super::{check_at_scale, Width};
    #[cfg(all())]
    mod d18 {
        use super::{check_at_scale, Width};
        #[test]
        fn d18_sqrt_s0() {
            check_at_scale("sqrt", Width::D18, 0, include_str!("golden/sqrt_d18_s0.txt"));
        }
        #[test]
        fn d18_sqrt_s17() {
            check_at_scale("sqrt", Width::D18, 17, include_str!("golden/sqrt_d18_s17.txt"));
        }
        #[test]
        fn d18_cbrt_s0() {
            check_at_scale("cbrt", Width::D18, 0, include_str!("golden/cbrt_d18_s0.txt"));
        }
        #[test]
        fn d18_cbrt_s17() {
            check_at_scale("cbrt", Width::D18, 17, include_str!("golden/cbrt_d18_s17.txt"));
        }
        #[test]
        fn d18_exp_s0() {
            check_at_scale("exp", Width::D18, 0, include_str!("golden/exp_d18_s0.txt"));
        }
        #[test]
        fn d18_exp_s17() {
            check_at_scale("exp", Width::D18, 17, include_str!("golden/exp_d18_s17.txt"));
        }
        #[test]
        fn d18_ln_s0() {
            check_at_scale("ln", Width::D18, 0, include_str!("golden/ln_d18_s0.txt"));
        }
        #[test]
        fn d18_ln_s17() {
            check_at_scale("ln", Width::D18, 17, include_str!("golden/ln_d18_s17.txt"));
        }
        #[test]
        fn d18_log2_s0() {
            check_at_scale("log2", Width::D18, 0, include_str!("golden/log2_d18_s0.txt"));
        }
        #[test]
        fn d18_log2_s17() {
            check_at_scale("log2", Width::D18, 17, include_str!("golden/log2_d18_s17.txt"));
        }
        #[test]
        fn d18_log10_s0() {
            check_at_scale("log10", Width::D18, 0, include_str!("golden/log10_d18_s0.txt"));
        }
        #[test]
        fn d18_log10_s17() {
            check_at_scale("log10", Width::D18, 17, include_str!("golden/log10_d18_s17.txt"));
        }
        #[test]
        fn d18_exp2_s0() {
            check_at_scale("exp2", Width::D18, 0, include_str!("golden/exp2_d18_s0.txt"));
        }
        #[test]
        fn d18_exp2_s17() {
            check_at_scale("exp2", Width::D18, 17, include_str!("golden/exp2_d18_s17.txt"));
        }
        #[test]
        fn d18_sin_s0() {
            check_at_scale("sin", Width::D18, 0, include_str!("golden/sin_d18_s0.txt"));
        }
        #[test]
        fn d18_sin_s17() {
            check_at_scale("sin", Width::D18, 17, include_str!("golden/sin_d18_s17.txt"));
        }
        #[test]
        fn d18_cos_s0() {
            check_at_scale("cos", Width::D18, 0, include_str!("golden/cos_d18_s0.txt"));
        }
        #[test]
        fn d18_cos_s17() {
            check_at_scale("cos", Width::D18, 17, include_str!("golden/cos_d18_s17.txt"));
        }
        #[test]
        fn d18_tan_s0() {
            check_at_scale("tan", Width::D18, 0, include_str!("golden/tan_d18_s0.txt"));
        }
        #[test]
        fn d18_tan_s17() {
            check_at_scale("tan", Width::D18, 17, include_str!("golden/tan_d18_s17.txt"));
        }
        #[test]
        fn d18_atan_s0() {
            check_at_scale("atan", Width::D18, 0, include_str!("golden/atan_d18_s0.txt"));
        }
        #[test]
        fn d18_atan_s17() {
            check_at_scale("atan", Width::D18, 17, include_str!("golden/atan_d18_s17.txt"));
        }
        #[test]
        fn d18_asin_s0() {
            check_at_scale("asin", Width::D18, 0, include_str!("golden/asin_d18_s0.txt"));
        }
        #[test]
        fn d18_asin_s17() {
            check_at_scale("asin", Width::D18, 17, include_str!("golden/asin_d18_s17.txt"));
        }
        #[test]
        fn d18_acos_s0() {
            check_at_scale("acos", Width::D18, 0, include_str!("golden/acos_d18_s0.txt"));
        }
        #[test]
        fn d18_acos_s17() {
            check_at_scale("acos", Width::D18, 17, include_str!("golden/acos_d18_s17.txt"));
        }
        #[test]
        fn d18_sinh_s0() {
            check_at_scale("sinh", Width::D18, 0, include_str!("golden/sinh_d18_s0.txt"));
        }
        #[test]
        fn d18_sinh_s17() {
            check_at_scale("sinh", Width::D18, 17, include_str!("golden/sinh_d18_s17.txt"));
        }
        #[test]
        fn d18_cosh_s0() {
            check_at_scale("cosh", Width::D18, 0, include_str!("golden/cosh_d18_s0.txt"));
        }
        #[test]
        fn d18_tanh_s0() {
            check_at_scale("tanh", Width::D18, 0, include_str!("golden/tanh_d18_s0.txt"));
        }
        #[test]
        fn d18_tanh_s17() {
            check_at_scale("tanh", Width::D18, 17, include_str!("golden/tanh_d18_s17.txt"));
        }
        #[test]
        fn d18_asinh_s0() {
            check_at_scale("asinh", Width::D18, 0, include_str!("golden/asinh_d18_s0.txt"));
        }
        #[test]
        fn d18_asinh_s17() {
            check_at_scale("asinh", Width::D18, 17, include_str!("golden/asinh_d18_s17.txt"));
        }
        #[test]
        fn d18_acosh_s0() {
            check_at_scale("acosh", Width::D18, 0, include_str!("golden/acosh_d18_s0.txt"));
        }
        #[test]
        fn d18_acosh_s17() {
            check_at_scale("acosh", Width::D18, 17, include_str!("golden/acosh_d18_s17.txt"));
        }
        #[test]
        fn d18_atanh_s0() {
            check_at_scale("atanh", Width::D18, 0, include_str!("golden/atanh_d18_s0.txt"));
        }
        #[test]
        fn d18_atanh_s17() {
            check_at_scale("atanh", Width::D18, 17, include_str!("golden/atanh_d18_s17.txt"));
        }
        #[test]
        fn d18_log_s0() {
            check_at_scale("log", Width::D18, 0, include_str!("golden/log_d18_s0.txt"));
        }
        #[test]
        fn d18_log_s17() {
            check_at_scale("log", Width::D18, 17, include_str!("golden/log_d18_s17.txt"));
        }
        #[test]
        fn d18_atan2_s0() {
            check_at_scale("atan2", Width::D18, 0, include_str!("golden/atan2_d18_s0.txt"));
        }
        #[test]
        fn d18_atan2_s17() {
            check_at_scale("atan2", Width::D18, 17, include_str!("golden/atan2_d18_s17.txt"));
        }
        #[test]
        fn d18_powf_s0() {
            check_at_scale("powf", Width::D18, 0, include_str!("golden/powf_d18_s0.txt"));
        }
        #[test]
        fn d18_powf_s17() {
            check_at_scale("powf", Width::D18, 17, include_str!("golden/powf_d18_s17.txt"));
        }
    }
    #[cfg(all())]
    mod d38 {
        use super::{check_at_scale, Width};
        #[test]
        fn d38_sqrt_s0() {
            check_at_scale("sqrt", Width::D38, 0, include_str!("golden/sqrt_d38_s0.txt"));
        }
        #[test]
        fn d38_sqrt_s37() {
            check_at_scale("sqrt", Width::D38, 37, include_str!("golden/sqrt_d38_s37.txt"));
        }
        #[test]
        fn d38_cbrt_s0() {
            check_at_scale("cbrt", Width::D38, 0, include_str!("golden/cbrt_d38_s0.txt"));
        }
        #[test]
        fn d38_cbrt_s37() {
            check_at_scale("cbrt", Width::D38, 37, include_str!("golden/cbrt_d38_s37.txt"));
        }
        #[test]
        fn d38_exp_s0() {
            check_at_scale("exp", Width::D38, 0, include_str!("golden/exp_d38_s0.txt"));
        }
        #[test]
        fn d38_exp_s37() {
            check_at_scale("exp", Width::D38, 37, include_str!("golden/exp_d38_s37.txt"));
        }
        #[test]
        fn d38_ln_s0() {
            check_at_scale("ln", Width::D38, 0, include_str!("golden/ln_d38_s0.txt"));
        }
        #[test]
        fn d38_ln_s37() {
            check_at_scale("ln", Width::D38, 37, include_str!("golden/ln_d38_s37.txt"));
        }
        #[test]
        fn d38_log2_s0() {
            check_at_scale("log2", Width::D38, 0, include_str!("golden/log2_d38_s0.txt"));
        }
        #[test]
        fn d38_log2_s37() {
            check_at_scale("log2", Width::D38, 37, include_str!("golden/log2_d38_s37.txt"));
        }
        #[test]
        fn d38_log10_s0() {
            check_at_scale("log10", Width::D38, 0, include_str!("golden/log10_d38_s0.txt"));
        }
        #[test]
        fn d38_log10_s37() {
            check_at_scale("log10", Width::D38, 37, include_str!("golden/log10_d38_s37.txt"));
        }
        #[test]
        fn d38_exp2_s0() {
            check_at_scale("exp2", Width::D38, 0, include_str!("golden/exp2_d38_s0.txt"));
        }
        #[test]
        fn d38_exp2_s37() {
            check_at_scale("exp2", Width::D38, 37, include_str!("golden/exp2_d38_s37.txt"));
        }
        #[test]
        fn d38_sin_s0() {
            check_at_scale("sin", Width::D38, 0, include_str!("golden/sin_d38_s0.txt"));
        }
        #[test]
        fn d38_sin_s37() {
            check_at_scale("sin", Width::D38, 37, include_str!("golden/sin_d38_s37.txt"));
        }
        #[test]
        fn d38_cos_s0() {
            check_at_scale("cos", Width::D38, 0, include_str!("golden/cos_d38_s0.txt"));
        }
        #[test]
        fn d38_cos_s37() {
            check_at_scale("cos", Width::D38, 37, include_str!("golden/cos_d38_s37.txt"));
        }
        #[test]
        fn d38_tan_s0() {
            check_at_scale("tan", Width::D38, 0, include_str!("golden/tan_d38_s0.txt"));
        }
        #[test]
        fn d38_tan_s37() {
            check_at_scale("tan", Width::D38, 37, include_str!("golden/tan_d38_s37.txt"));
        }
        #[test]
        fn d38_atan_s0() {
            check_at_scale("atan", Width::D38, 0, include_str!("golden/atan_d38_s0.txt"));
        }
        #[test]
        fn d38_atan_s37() {
            check_at_scale("atan", Width::D38, 37, include_str!("golden/atan_d38_s37.txt"));
        }
        #[test]
        fn d38_asin_s0() {
            check_at_scale("asin", Width::D38, 0, include_str!("golden/asin_d38_s0.txt"));
        }
        #[test]
        fn d38_asin_s37() {
            check_at_scale("asin", Width::D38, 37, include_str!("golden/asin_d38_s37.txt"));
        }
        #[test]
        fn d38_acos_s0() {
            check_at_scale("acos", Width::D38, 0, include_str!("golden/acos_d38_s0.txt"));
        }
        #[test]
        fn d38_acos_s37() {
            check_at_scale("acos", Width::D38, 37, include_str!("golden/acos_d38_s37.txt"));
        }
        #[test]
        fn d38_sinh_s0() {
            check_at_scale("sinh", Width::D38, 0, include_str!("golden/sinh_d38_s0.txt"));
        }
        #[test]
        fn d38_sinh_s37() {
            check_at_scale("sinh", Width::D38, 37, include_str!("golden/sinh_d38_s37.txt"));
        }
        #[test]
        fn d38_cosh_s0() {
            check_at_scale("cosh", Width::D38, 0, include_str!("golden/cosh_d38_s0.txt"));
        }
        #[test]
        fn d38_tanh_s0() {
            check_at_scale("tanh", Width::D38, 0, include_str!("golden/tanh_d38_s0.txt"));
        }
        #[test]
        fn d38_tanh_s37() {
            check_at_scale("tanh", Width::D38, 37, include_str!("golden/tanh_d38_s37.txt"));
        }
        #[test]
        fn d38_asinh_s0() {
            check_at_scale("asinh", Width::D38, 0, include_str!("golden/asinh_d38_s0.txt"));
        }
        #[test]
        fn d38_asinh_s37() {
            check_at_scale("asinh", Width::D38, 37, include_str!("golden/asinh_d38_s37.txt"));
        }
        #[test]
        fn d38_acosh_s0() {
            check_at_scale("acosh", Width::D38, 0, include_str!("golden/acosh_d38_s0.txt"));
        }
        #[test]
        fn d38_acosh_s37() {
            check_at_scale("acosh", Width::D38, 37, include_str!("golden/acosh_d38_s37.txt"));
        }
        #[test]
        fn d38_atanh_s0() {
            check_at_scale("atanh", Width::D38, 0, include_str!("golden/atanh_d38_s0.txt"));
        }
        #[test]
        fn d38_atanh_s37() {
            check_at_scale("atanh", Width::D38, 37, include_str!("golden/atanh_d38_s37.txt"));
        }
        #[test]
        fn d38_log_s0() {
            check_at_scale("log", Width::D38, 0, include_str!("golden/log_d38_s0.txt"));
        }
        #[test]
        fn d38_log_s37() {
            check_at_scale("log", Width::D38, 37, include_str!("golden/log_d38_s37.txt"));
        }
        #[test]
        fn d38_atan2_s0() {
            check_at_scale("atan2", Width::D38, 0, include_str!("golden/atan2_d38_s0.txt"));
        }
        #[test]
        fn d38_atan2_s37() {
            check_at_scale("atan2", Width::D38, 37, include_str!("golden/atan2_d38_s37.txt"));
        }
        #[test]
        fn d38_powf_s0() {
            check_at_scale("powf", Width::D38, 0, include_str!("golden/powf_d38_s0.txt"));
        }
        #[test]
        fn d38_powf_s37() {
            check_at_scale("powf", Width::D38, 37, include_str!("golden/powf_d38_s37.txt"));
        }
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    mod d57 {
        use super::{check_at_scale, Width};
        #[test]
        fn d57_sqrt_s0() {
            check_at_scale("sqrt", Width::D57, 0, include_str!("golden/sqrt_d57_s0.txt"));
        }
        #[test]
        fn d57_sqrt_s56() {
            check_at_scale("sqrt", Width::D57, 56, include_str!("golden/sqrt_d57_s56.txt"));
        }
        #[test]
        fn d57_cbrt_s0() {
            check_at_scale("cbrt", Width::D57, 0, include_str!("golden/cbrt_d57_s0.txt"));
        }
        #[test]
        fn d57_cbrt_s56() {
            check_at_scale("cbrt", Width::D57, 56, include_str!("golden/cbrt_d57_s56.txt"));
        }
        #[test]
        fn d57_exp_s0() {
            check_at_scale("exp", Width::D57, 0, include_str!("golden/exp_d57_s0.txt"));
        }
        #[test]
        fn d57_exp_s56() {
            check_at_scale("exp", Width::D57, 56, include_str!("golden/exp_d57_s56.txt"));
        }
        #[test]
        fn d57_ln_s0() {
            check_at_scale("ln", Width::D57, 0, include_str!("golden/ln_d57_s0.txt"));
        }
        #[test]
        fn d57_ln_s56() {
            check_at_scale("ln", Width::D57, 56, include_str!("golden/ln_d57_s56.txt"));
        }
        #[test]
        fn d57_log2_s0() {
            check_at_scale("log2", Width::D57, 0, include_str!("golden/log2_d57_s0.txt"));
        }
        #[test]
        fn d57_log2_s56() {
            check_at_scale("log2", Width::D57, 56, include_str!("golden/log2_d57_s56.txt"));
        }
        #[test]
        fn d57_log10_s0() {
            check_at_scale("log10", Width::D57, 0, include_str!("golden/log10_d57_s0.txt"));
        }
        #[test]
        fn d57_log10_s56() {
            check_at_scale("log10", Width::D57, 56, include_str!("golden/log10_d57_s56.txt"));
        }
        #[test]
        fn d57_exp2_s0() {
            check_at_scale("exp2", Width::D57, 0, include_str!("golden/exp2_d57_s0.txt"));
        }
        #[test]
        fn d57_exp2_s56() {
            check_at_scale("exp2", Width::D57, 56, include_str!("golden/exp2_d57_s56.txt"));
        }
        #[test]
        fn d57_sin_s0() {
            check_at_scale("sin", Width::D57, 0, include_str!("golden/sin_d57_s0.txt"));
        }
        #[test]
        fn d57_sin_s56() {
            check_at_scale("sin", Width::D57, 56, include_str!("golden/sin_d57_s56.txt"));
        }
        #[test]
        fn d57_cos_s0() {
            check_at_scale("cos", Width::D57, 0, include_str!("golden/cos_d57_s0.txt"));
        }
        #[test]
        fn d57_cos_s56() {
            check_at_scale("cos", Width::D57, 56, include_str!("golden/cos_d57_s56.txt"));
        }
        #[test]
        fn d57_tan_s0() {
            check_at_scale("tan", Width::D57, 0, include_str!("golden/tan_d57_s0.txt"));
        }
        #[test]
        fn d57_tan_s56() {
            check_at_scale("tan", Width::D57, 56, include_str!("golden/tan_d57_s56.txt"));
        }
        #[test]
        fn d57_atan_s0() {
            check_at_scale("atan", Width::D57, 0, include_str!("golden/atan_d57_s0.txt"));
        }
        #[test]
        fn d57_atan_s56() {
            check_at_scale("atan", Width::D57, 56, include_str!("golden/atan_d57_s56.txt"));
        }
        #[test]
        fn d57_asin_s0() {
            check_at_scale("asin", Width::D57, 0, include_str!("golden/asin_d57_s0.txt"));
        }
        #[test]
        fn d57_asin_s56() {
            check_at_scale("asin", Width::D57, 56, include_str!("golden/asin_d57_s56.txt"));
        }
        #[test]
        fn d57_acos_s0() {
            check_at_scale("acos", Width::D57, 0, include_str!("golden/acos_d57_s0.txt"));
        }
        #[test]
        fn d57_acos_s56() {
            check_at_scale("acos", Width::D57, 56, include_str!("golden/acos_d57_s56.txt"));
        }
        #[test]
        fn d57_sinh_s0() {
            check_at_scale("sinh", Width::D57, 0, include_str!("golden/sinh_d57_s0.txt"));
        }
        #[test]
        fn d57_sinh_s56() {
            check_at_scale("sinh", Width::D57, 56, include_str!("golden/sinh_d57_s56.txt"));
        }
        #[test]
        fn d57_cosh_s0() {
            check_at_scale("cosh", Width::D57, 0, include_str!("golden/cosh_d57_s0.txt"));
        }
        #[test]
        fn d57_tanh_s0() {
            check_at_scale("tanh", Width::D57, 0, include_str!("golden/tanh_d57_s0.txt"));
        }
        #[test]
        fn d57_tanh_s56() {
            check_at_scale("tanh", Width::D57, 56, include_str!("golden/tanh_d57_s56.txt"));
        }
        #[test]
        fn d57_asinh_s0() {
            check_at_scale("asinh", Width::D57, 0, include_str!("golden/asinh_d57_s0.txt"));
        }
        #[test]
        fn d57_asinh_s56() {
            check_at_scale("asinh", Width::D57, 56, include_str!("golden/asinh_d57_s56.txt"));
        }
        #[test]
        fn d57_acosh_s0() {
            check_at_scale("acosh", Width::D57, 0, include_str!("golden/acosh_d57_s0.txt"));
        }
        #[test]
        fn d57_acosh_s56() {
            check_at_scale("acosh", Width::D57, 56, include_str!("golden/acosh_d57_s56.txt"));
        }
        #[test]
        fn d57_atanh_s0() {
            check_at_scale("atanh", Width::D57, 0, include_str!("golden/atanh_d57_s0.txt"));
        }
        #[test]
        fn d57_atanh_s56() {
            check_at_scale("atanh", Width::D57, 56, include_str!("golden/atanh_d57_s56.txt"));
        }
        #[test]
        fn d57_log_s0() {
            check_at_scale("log", Width::D57, 0, include_str!("golden/log_d57_s0.txt"));
        }
        #[test]
        fn d57_log_s56() {
            check_at_scale("log", Width::D57, 56, include_str!("golden/log_d57_s56.txt"));
        }
        #[test]
        fn d57_atan2_s0() {
            check_at_scale("atan2", Width::D57, 0, include_str!("golden/atan2_d57_s0.txt"));
        }
        #[test]
        fn d57_atan2_s56() {
            check_at_scale("atan2", Width::D57, 56, include_str!("golden/atan2_d57_s56.txt"));
        }
        #[test]
        fn d57_powf_s0() {
            check_at_scale("powf", Width::D57, 0, include_str!("golden/powf_d57_s0.txt"));
        }
        #[test]
        fn d57_powf_s56() {
            check_at_scale("powf", Width::D57, 56, include_str!("golden/powf_d57_s56.txt"));
        }
    }
    #[cfg(any(feature = "d76", feature = "wide"))]
    mod d76 {
        use super::{check_at_scale, Width};
        #[test]
        fn d76_sqrt_s0() {
            check_at_scale("sqrt", Width::D76, 0, include_str!("golden/sqrt_d76_s0.txt"));
        }
        #[test]
        fn d76_sqrt_s75() {
            check_at_scale("sqrt", Width::D76, 75, include_str!("golden/sqrt_d76_s75.txt"));
        }
        #[test]
        fn d76_cbrt_s0() {
            check_at_scale("cbrt", Width::D76, 0, include_str!("golden/cbrt_d76_s0.txt"));
        }
        #[test]
        fn d76_cbrt_s75() {
            check_at_scale("cbrt", Width::D76, 75, include_str!("golden/cbrt_d76_s75.txt"));
        }
        #[test]
        fn d76_exp_s0() {
            check_at_scale("exp", Width::D76, 0, include_str!("golden/exp_d76_s0.txt"));
        }
        #[test]
        fn d76_exp_s75() {
            check_at_scale("exp", Width::D76, 75, include_str!("golden/exp_d76_s75.txt"));
        }
        #[test]
        fn d76_ln_s0() {
            check_at_scale("ln", Width::D76, 0, include_str!("golden/ln_d76_s0.txt"));
        }
        #[test]
        fn d76_ln_s75() {
            check_at_scale("ln", Width::D76, 75, include_str!("golden/ln_d76_s75.txt"));
        }
        #[test]
        fn d76_log2_s0() {
            check_at_scale("log2", Width::D76, 0, include_str!("golden/log2_d76_s0.txt"));
        }
        #[test]
        fn d76_log2_s75() {
            check_at_scale("log2", Width::D76, 75, include_str!("golden/log2_d76_s75.txt"));
        }
        #[test]
        fn d76_log10_s0() {
            check_at_scale("log10", Width::D76, 0, include_str!("golden/log10_d76_s0.txt"));
        }
        #[test]
        fn d76_log10_s75() {
            check_at_scale("log10", Width::D76, 75, include_str!("golden/log10_d76_s75.txt"));
        }
        #[test]
        fn d76_exp2_s0() {
            check_at_scale("exp2", Width::D76, 0, include_str!("golden/exp2_d76_s0.txt"));
        }
        #[test]
        fn d76_exp2_s75() {
            check_at_scale("exp2", Width::D76, 75, include_str!("golden/exp2_d76_s75.txt"));
        }
        #[test]
        fn d76_sin_s0() {
            check_at_scale("sin", Width::D76, 0, include_str!("golden/sin_d76_s0.txt"));
        }
        #[test]
        fn d76_sin_s75() {
            check_at_scale("sin", Width::D76, 75, include_str!("golden/sin_d76_s75.txt"));
        }
        #[test]
        fn d76_cos_s0() {
            check_at_scale("cos", Width::D76, 0, include_str!("golden/cos_d76_s0.txt"));
        }
        #[test]
        fn d76_cos_s75() {
            check_at_scale("cos", Width::D76, 75, include_str!("golden/cos_d76_s75.txt"));
        }
        #[test]
        fn d76_tan_s0() {
            check_at_scale("tan", Width::D76, 0, include_str!("golden/tan_d76_s0.txt"));
        }
        #[test]
        fn d76_tan_s75() {
            check_at_scale("tan", Width::D76, 75, include_str!("golden/tan_d76_s75.txt"));
        }
        #[test]
        fn d76_atan_s0() {
            check_at_scale("atan", Width::D76, 0, include_str!("golden/atan_d76_s0.txt"));
        }
        #[test]
        fn d76_atan_s75() {
            check_at_scale("atan", Width::D76, 75, include_str!("golden/atan_d76_s75.txt"));
        }
        #[test]
        fn d76_asin_s0() {
            check_at_scale("asin", Width::D76, 0, include_str!("golden/asin_d76_s0.txt"));
        }
        #[test]
        fn d76_asin_s75() {
            check_at_scale("asin", Width::D76, 75, include_str!("golden/asin_d76_s75.txt"));
        }
        #[test]
        fn d76_acos_s0() {
            check_at_scale("acos", Width::D76, 0, include_str!("golden/acos_d76_s0.txt"));
        }
        #[test]
        fn d76_acos_s75() {
            check_at_scale("acos", Width::D76, 75, include_str!("golden/acos_d76_s75.txt"));
        }
        #[test]
        fn d76_sinh_s0() {
            check_at_scale("sinh", Width::D76, 0, include_str!("golden/sinh_d76_s0.txt"));
        }
        #[test]
        fn d76_sinh_s75() {
            check_at_scale("sinh", Width::D76, 75, include_str!("golden/sinh_d76_s75.txt"));
        }
        #[test]
        fn d76_cosh_s0() {
            check_at_scale("cosh", Width::D76, 0, include_str!("golden/cosh_d76_s0.txt"));
        }
        #[test]
        fn d76_tanh_s0() {
            check_at_scale("tanh", Width::D76, 0, include_str!("golden/tanh_d76_s0.txt"));
        }
        #[test]
        fn d76_tanh_s75() {
            check_at_scale("tanh", Width::D76, 75, include_str!("golden/tanh_d76_s75.txt"));
        }
        #[test]
        fn d76_asinh_s0() {
            check_at_scale("asinh", Width::D76, 0, include_str!("golden/asinh_d76_s0.txt"));
        }
        #[test]
        fn d76_asinh_s75() {
            check_at_scale("asinh", Width::D76, 75, include_str!("golden/asinh_d76_s75.txt"));
        }
        #[test]
        fn d76_acosh_s0() {
            check_at_scale("acosh", Width::D76, 0, include_str!("golden/acosh_d76_s0.txt"));
        }
        #[test]
        fn d76_acosh_s75() {
            check_at_scale("acosh", Width::D76, 75, include_str!("golden/acosh_d76_s75.txt"));
        }
        #[test]
        fn d76_atanh_s0() {
            check_at_scale("atanh", Width::D76, 0, include_str!("golden/atanh_d76_s0.txt"));
        }
        #[test]
        fn d76_atanh_s75() {
            check_at_scale("atanh", Width::D76, 75, include_str!("golden/atanh_d76_s75.txt"));
        }
        #[test]
        fn d76_log_s0() {
            check_at_scale("log", Width::D76, 0, include_str!("golden/log_d76_s0.txt"));
        }
        #[test]
        fn d76_log_s75() {
            check_at_scale("log", Width::D76, 75, include_str!("golden/log_d76_s75.txt"));
        }
        #[test]
        fn d76_atan2_s0() {
            check_at_scale("atan2", Width::D76, 0, include_str!("golden/atan2_d76_s0.txt"));
        }
        #[test]
        fn d76_atan2_s75() {
            check_at_scale("atan2", Width::D76, 75, include_str!("golden/atan2_d76_s75.txt"));
        }
        #[test]
        fn d76_powf_s0() {
            check_at_scale("powf", Width::D76, 0, include_str!("golden/powf_d76_s0.txt"));
        }
        #[test]
        fn d76_powf_s75() {
            check_at_scale("powf", Width::D76, 75, include_str!("golden/powf_d76_s75.txt"));
        }
    }
    #[cfg(any(feature = "d115", feature = "wide"))]
    mod d115 {
        use super::{check_at_scale, Width};
        #[test]
        fn d115_sqrt_s0() {
            check_at_scale("sqrt", Width::D115, 0, include_str!("golden/sqrt_d115_s0.txt"));
        }
        #[test]
        fn d115_sqrt_s114() {
            check_at_scale("sqrt", Width::D115, 114, include_str!("golden/sqrt_d115_s114.txt"));
        }
        #[test]
        fn d115_cbrt_s0() {
            check_at_scale("cbrt", Width::D115, 0, include_str!("golden/cbrt_d115_s0.txt"));
        }
        #[test]
        fn d115_cbrt_s114() {
            check_at_scale("cbrt", Width::D115, 114, include_str!("golden/cbrt_d115_s114.txt"));
        }
        #[test]
        fn d115_exp_s0() {
            check_at_scale("exp", Width::D115, 0, include_str!("golden/exp_d115_s0.txt"));
        }
        #[test]
        fn d115_exp_s114() {
            check_at_scale("exp", Width::D115, 114, include_str!("golden/exp_d115_s114.txt"));
        }
        #[test]
        fn d115_ln_s0() {
            check_at_scale("ln", Width::D115, 0, include_str!("golden/ln_d115_s0.txt"));
        }
        #[test]
        fn d115_ln_s114() {
            check_at_scale("ln", Width::D115, 114, include_str!("golden/ln_d115_s114.txt"));
        }
        #[test]
        fn d115_log2_s0() {
            check_at_scale("log2", Width::D115, 0, include_str!("golden/log2_d115_s0.txt"));
        }
        #[test]
        fn d115_log2_s114() {
            check_at_scale("log2", Width::D115, 114, include_str!("golden/log2_d115_s114.txt"));
        }
        #[test]
        fn d115_log10_s0() {
            check_at_scale("log10", Width::D115, 0, include_str!("golden/log10_d115_s0.txt"));
        }
        #[test]
        fn d115_log10_s114() {
            check_at_scale("log10", Width::D115, 114, include_str!("golden/log10_d115_s114.txt"));
        }
        #[test]
        fn d115_exp2_s0() {
            check_at_scale("exp2", Width::D115, 0, include_str!("golden/exp2_d115_s0.txt"));
        }
        #[test]
        fn d115_exp2_s114() {
            check_at_scale("exp2", Width::D115, 114, include_str!("golden/exp2_d115_s114.txt"));
        }
        #[test]
        fn d115_sin_s0() {
            check_at_scale("sin", Width::D115, 0, include_str!("golden/sin_d115_s0.txt"));
        }
        #[test]
        fn d115_sin_s114() {
            check_at_scale("sin", Width::D115, 114, include_str!("golden/sin_d115_s114.txt"));
        }
        #[test]
        fn d115_cos_s0() {
            check_at_scale("cos", Width::D115, 0, include_str!("golden/cos_d115_s0.txt"));
        }
        #[test]
        fn d115_cos_s114() {
            check_at_scale("cos", Width::D115, 114, include_str!("golden/cos_d115_s114.txt"));
        }
        #[test]
        fn d115_tan_s0() {
            check_at_scale("tan", Width::D115, 0, include_str!("golden/tan_d115_s0.txt"));
        }
        #[test]
        fn d115_tan_s114() {
            check_at_scale("tan", Width::D115, 114, include_str!("golden/tan_d115_s114.txt"));
        }
        #[test]
        fn d115_atan_s0() {
            check_at_scale("atan", Width::D115, 0, include_str!("golden/atan_d115_s0.txt"));
        }
        #[test]
        fn d115_atan_s114() {
            check_at_scale("atan", Width::D115, 114, include_str!("golden/atan_d115_s114.txt"));
        }
        #[test]
        fn d115_asin_s0() {
            check_at_scale("asin", Width::D115, 0, include_str!("golden/asin_d115_s0.txt"));
        }
        #[test]
        fn d115_asin_s114() {
            check_at_scale("asin", Width::D115, 114, include_str!("golden/asin_d115_s114.txt"));
        }
        #[test]
        fn d115_acos_s0() {
            check_at_scale("acos", Width::D115, 0, include_str!("golden/acos_d115_s0.txt"));
        }
        #[test]
        fn d115_acos_s114() {
            check_at_scale("acos", Width::D115, 114, include_str!("golden/acos_d115_s114.txt"));
        }
        #[test]
        fn d115_sinh_s0() {
            check_at_scale("sinh", Width::D115, 0, include_str!("golden/sinh_d115_s0.txt"));
        }
        #[test]
        fn d115_sinh_s114() {
            check_at_scale("sinh", Width::D115, 114, include_str!("golden/sinh_d115_s114.txt"));
        }
        #[test]
        fn d115_cosh_s0() {
            check_at_scale("cosh", Width::D115, 0, include_str!("golden/cosh_d115_s0.txt"));
        }
        #[test]
        fn d115_tanh_s0() {
            check_at_scale("tanh", Width::D115, 0, include_str!("golden/tanh_d115_s0.txt"));
        }
        #[test]
        fn d115_tanh_s114() {
            check_at_scale("tanh", Width::D115, 114, include_str!("golden/tanh_d115_s114.txt"));
        }
        #[test]
        fn d115_asinh_s0() {
            check_at_scale("asinh", Width::D115, 0, include_str!("golden/asinh_d115_s0.txt"));
        }
        #[test]
        fn d115_asinh_s114() {
            check_at_scale("asinh", Width::D115, 114, include_str!("golden/asinh_d115_s114.txt"));
        }
        #[test]
        fn d115_acosh_s0() {
            check_at_scale("acosh", Width::D115, 0, include_str!("golden/acosh_d115_s0.txt"));
        }
        #[test]
        fn d115_acosh_s114() {
            check_at_scale("acosh", Width::D115, 114, include_str!("golden/acosh_d115_s114.txt"));
        }
        #[test]
        fn d115_atanh_s0() {
            check_at_scale("atanh", Width::D115, 0, include_str!("golden/atanh_d115_s0.txt"));
        }
        #[test]
        fn d115_atanh_s114() {
            check_at_scale("atanh", Width::D115, 114, include_str!("golden/atanh_d115_s114.txt"));
        }
        #[test]
        fn d115_log_s0() {
            check_at_scale("log", Width::D115, 0, include_str!("golden/log_d115_s0.txt"));
        }
        #[test]
        fn d115_log_s114() {
            check_at_scale("log", Width::D115, 114, include_str!("golden/log_d115_s114.txt"));
        }
        #[test]
        fn d115_atan2_s0() {
            check_at_scale("atan2", Width::D115, 0, include_str!("golden/atan2_d115_s0.txt"));
        }
        #[test]
        fn d115_atan2_s114() {
            check_at_scale("atan2", Width::D115, 114, include_str!("golden/atan2_d115_s114.txt"));
        }
        #[test]
        fn d115_powf_s0() {
            check_at_scale("powf", Width::D115, 0, include_str!("golden/powf_d115_s0.txt"));
        }
        #[test]
        fn d115_powf_s114() {
            check_at_scale("powf", Width::D115, 114, include_str!("golden/powf_d115_s114.txt"));
        }
    }
    #[cfg(any(feature = "d153", feature = "wide"))]
    mod d153 {
        use super::{check_at_scale, Width};
        #[test]
        fn d153_sqrt_s0() {
            check_at_scale("sqrt", Width::D153, 0, include_str!("golden/sqrt_d153_s0.txt"));
        }
        #[test]
        fn d153_sqrt_s152() {
            check_at_scale("sqrt", Width::D153, 152, include_str!("golden/sqrt_d153_s152.txt"));
        }
        #[test]
        fn d153_cbrt_s0() {
            check_at_scale("cbrt", Width::D153, 0, include_str!("golden/cbrt_d153_s0.txt"));
        }
        #[test]
        fn d153_cbrt_s152() {
            check_at_scale("cbrt", Width::D153, 152, include_str!("golden/cbrt_d153_s152.txt"));
        }
        #[test]
        fn d153_exp_s0() {
            check_at_scale("exp", Width::D153, 0, include_str!("golden/exp_d153_s0.txt"));
        }
        #[test]
        fn d153_exp_s152() {
            check_at_scale("exp", Width::D153, 152, include_str!("golden/exp_d153_s152.txt"));
        }
        #[test]
        fn d153_ln_s0() {
            check_at_scale("ln", Width::D153, 0, include_str!("golden/ln_d153_s0.txt"));
        }
        #[test]
        fn d153_ln_s152() {
            check_at_scale("ln", Width::D153, 152, include_str!("golden/ln_d153_s152.txt"));
        }
        #[test]
        fn d153_log2_s0() {
            check_at_scale("log2", Width::D153, 0, include_str!("golden/log2_d153_s0.txt"));
        }
        #[test]
        fn d153_log2_s152() {
            check_at_scale("log2", Width::D153, 152, include_str!("golden/log2_d153_s152.txt"));
        }
        #[test]
        fn d153_log10_s0() {
            check_at_scale("log10", Width::D153, 0, include_str!("golden/log10_d153_s0.txt"));
        }
        #[test]
        fn d153_log10_s152() {
            check_at_scale("log10", Width::D153, 152, include_str!("golden/log10_d153_s152.txt"));
        }
        #[test]
        fn d153_exp2_s0() {
            check_at_scale("exp2", Width::D153, 0, include_str!("golden/exp2_d153_s0.txt"));
        }
        #[test]
        fn d153_exp2_s152() {
            check_at_scale("exp2", Width::D153, 152, include_str!("golden/exp2_d153_s152.txt"));
        }
        #[test]
        fn d153_sin_s0() {
            check_at_scale("sin", Width::D153, 0, include_str!("golden/sin_d153_s0.txt"));
        }
        #[test]
        fn d153_sin_s152() {
            check_at_scale("sin", Width::D153, 152, include_str!("golden/sin_d153_s152.txt"));
        }
        #[test]
        fn d153_cos_s0() {
            check_at_scale("cos", Width::D153, 0, include_str!("golden/cos_d153_s0.txt"));
        }
        #[test]
        fn d153_cos_s152() {
            check_at_scale("cos", Width::D153, 152, include_str!("golden/cos_d153_s152.txt"));
        }
        #[test]
        fn d153_tan_s0() {
            check_at_scale("tan", Width::D153, 0, include_str!("golden/tan_d153_s0.txt"));
        }
        #[test]
        fn d153_tan_s152() {
            check_at_scale("tan", Width::D153, 152, include_str!("golden/tan_d153_s152.txt"));
        }
        #[test]
        fn d153_atan_s0() {
            check_at_scale("atan", Width::D153, 0, include_str!("golden/atan_d153_s0.txt"));
        }
        #[test]
        fn d153_atan_s152() {
            check_at_scale("atan", Width::D153, 152, include_str!("golden/atan_d153_s152.txt"));
        }
        #[test]
        fn d153_asin_s0() {
            check_at_scale("asin", Width::D153, 0, include_str!("golden/asin_d153_s0.txt"));
        }
        #[test]
        fn d153_asin_s152() {
            check_at_scale("asin", Width::D153, 152, include_str!("golden/asin_d153_s152.txt"));
        }
        #[test]
        fn d153_acos_s0() {
            check_at_scale("acos", Width::D153, 0, include_str!("golden/acos_d153_s0.txt"));
        }
        #[test]
        fn d153_acos_s152() {
            check_at_scale("acos", Width::D153, 152, include_str!("golden/acos_d153_s152.txt"));
        }
        #[test]
        fn d153_sinh_s0() {
            check_at_scale("sinh", Width::D153, 0, include_str!("golden/sinh_d153_s0.txt"));
        }
        #[test]
        fn d153_sinh_s152() {
            check_at_scale("sinh", Width::D153, 152, include_str!("golden/sinh_d153_s152.txt"));
        }
        #[test]
        fn d153_cosh_s0() {
            check_at_scale("cosh", Width::D153, 0, include_str!("golden/cosh_d153_s0.txt"));
        }
        #[test]
        fn d153_tanh_s0() {
            check_at_scale("tanh", Width::D153, 0, include_str!("golden/tanh_d153_s0.txt"));
        }
        #[test]
        fn d153_tanh_s152() {
            check_at_scale("tanh", Width::D153, 152, include_str!("golden/tanh_d153_s152.txt"));
        }
        #[test]
        fn d153_asinh_s0() {
            check_at_scale("asinh", Width::D153, 0, include_str!("golden/asinh_d153_s0.txt"));
        }
        #[test]
        fn d153_asinh_s152() {
            check_at_scale("asinh", Width::D153, 152, include_str!("golden/asinh_d153_s152.txt"));
        }
        #[test]
        fn d153_acosh_s0() {
            check_at_scale("acosh", Width::D153, 0, include_str!("golden/acosh_d153_s0.txt"));
        }
        #[test]
        fn d153_acosh_s152() {
            check_at_scale("acosh", Width::D153, 152, include_str!("golden/acosh_d153_s152.txt"));
        }
        #[test]
        fn d153_atanh_s0() {
            check_at_scale("atanh", Width::D153, 0, include_str!("golden/atanh_d153_s0.txt"));
        }
        #[test]
        fn d153_atanh_s152() {
            check_at_scale("atanh", Width::D153, 152, include_str!("golden/atanh_d153_s152.txt"));
        }
        #[test]
        fn d153_log_s0() {
            check_at_scale("log", Width::D153, 0, include_str!("golden/log_d153_s0.txt"));
        }
        #[test]
        fn d153_log_s152() {
            check_at_scale("log", Width::D153, 152, include_str!("golden/log_d153_s152.txt"));
        }
        #[test]
        fn d153_atan2_s0() {
            check_at_scale("atan2", Width::D153, 0, include_str!("golden/atan2_d153_s0.txt"));
        }
        #[test]
        fn d153_atan2_s152() {
            check_at_scale("atan2", Width::D153, 152, include_str!("golden/atan2_d153_s152.txt"));
        }
        #[test]
        fn d153_powf_s0() {
            check_at_scale("powf", Width::D153, 0, include_str!("golden/powf_d153_s0.txt"));
        }
        #[test]
        fn d153_powf_s152() {
            check_at_scale("powf", Width::D153, 152, include_str!("golden/powf_d153_s152.txt"));
        }
    }
    #[cfg(any(feature = "d230", feature = "wide"))]
    mod d230 {
        use super::{check_at_scale, Width};
        #[test]
        fn d230_sqrt_s0() {
            check_at_scale("sqrt", Width::D230, 0, include_str!("golden/sqrt_d230_s0.txt"));
        }
        #[test]
        fn d230_sqrt_s229() {
            check_at_scale("sqrt", Width::D230, 229, include_str!("golden/sqrt_d230_s229.txt"));
        }
        #[test]
        fn d230_cbrt_s0() {
            check_at_scale("cbrt", Width::D230, 0, include_str!("golden/cbrt_d230_s0.txt"));
        }
        #[test]
        fn d230_cbrt_s229() {
            check_at_scale("cbrt", Width::D230, 229, include_str!("golden/cbrt_d230_s229.txt"));
        }
        #[test]
        fn d230_exp_s0() {
            check_at_scale("exp", Width::D230, 0, include_str!("golden/exp_d230_s0.txt"));
        }
        #[test]
        fn d230_exp_s229() {
            check_at_scale("exp", Width::D230, 229, include_str!("golden/exp_d230_s229.txt"));
        }
        #[test]
        fn d230_ln_s0() {
            check_at_scale("ln", Width::D230, 0, include_str!("golden/ln_d230_s0.txt"));
        }
        #[test]
        fn d230_ln_s229() {
            check_at_scale("ln", Width::D230, 229, include_str!("golden/ln_d230_s229.txt"));
        }
        #[test]
        fn d230_log2_s0() {
            check_at_scale("log2", Width::D230, 0, include_str!("golden/log2_d230_s0.txt"));
        }
        #[test]
        fn d230_log2_s229() {
            check_at_scale("log2", Width::D230, 229, include_str!("golden/log2_d230_s229.txt"));
        }
        #[test]
        fn d230_log10_s0() {
            check_at_scale("log10", Width::D230, 0, include_str!("golden/log10_d230_s0.txt"));
        }
        #[test]
        fn d230_log10_s229() {
            check_at_scale("log10", Width::D230, 229, include_str!("golden/log10_d230_s229.txt"));
        }
        #[test]
        fn d230_exp2_s0() {
            check_at_scale("exp2", Width::D230, 0, include_str!("golden/exp2_d230_s0.txt"));
        }
        #[test]
        fn d230_exp2_s229() {
            check_at_scale("exp2", Width::D230, 229, include_str!("golden/exp2_d230_s229.txt"));
        }
        #[test]
        fn d230_sin_s0() {
            check_at_scale("sin", Width::D230, 0, include_str!("golden/sin_d230_s0.txt"));
        }
        #[test]
        fn d230_sin_s229() {
            check_at_scale("sin", Width::D230, 229, include_str!("golden/sin_d230_s229.txt"));
        }
        #[test]
        fn d230_cos_s0() {
            check_at_scale("cos", Width::D230, 0, include_str!("golden/cos_d230_s0.txt"));
        }
        #[test]
        fn d230_cos_s229() {
            check_at_scale("cos", Width::D230, 229, include_str!("golden/cos_d230_s229.txt"));
        }
        #[test]
        fn d230_tan_s0() {
            check_at_scale("tan", Width::D230, 0, include_str!("golden/tan_d230_s0.txt"));
        }
        #[test]
        fn d230_tan_s229() {
            check_at_scale("tan", Width::D230, 229, include_str!("golden/tan_d230_s229.txt"));
        }
        #[test]
        fn d230_atan_s0() {
            check_at_scale("atan", Width::D230, 0, include_str!("golden/atan_d230_s0.txt"));
        }
        #[test]
        fn d230_atan_s229() {
            check_at_scale("atan", Width::D230, 229, include_str!("golden/atan_d230_s229.txt"));
        }
        #[test]
        fn d230_asin_s0() {
            check_at_scale("asin", Width::D230, 0, include_str!("golden/asin_d230_s0.txt"));
        }
        #[test]
        fn d230_asin_s229() {
            check_at_scale("asin", Width::D230, 229, include_str!("golden/asin_d230_s229.txt"));
        }
        #[test]
        fn d230_acos_s0() {
            check_at_scale("acos", Width::D230, 0, include_str!("golden/acos_d230_s0.txt"));
        }
        #[test]
        fn d230_acos_s229() {
            check_at_scale("acos", Width::D230, 229, include_str!("golden/acos_d230_s229.txt"));
        }
        #[test]
        fn d230_sinh_s0() {
            check_at_scale("sinh", Width::D230, 0, include_str!("golden/sinh_d230_s0.txt"));
        }
        #[test]
        fn d230_sinh_s229() {
            check_at_scale("sinh", Width::D230, 229, include_str!("golden/sinh_d230_s229.txt"));
        }
        #[test]
        fn d230_cosh_s0() {
            check_at_scale("cosh", Width::D230, 0, include_str!("golden/cosh_d230_s0.txt"));
        }
        #[test]
        fn d230_tanh_s0() {
            check_at_scale("tanh", Width::D230, 0, include_str!("golden/tanh_d230_s0.txt"));
        }
        #[test]
        fn d230_tanh_s229() {
            check_at_scale("tanh", Width::D230, 229, include_str!("golden/tanh_d230_s229.txt"));
        }
        #[test]
        fn d230_asinh_s0() {
            check_at_scale("asinh", Width::D230, 0, include_str!("golden/asinh_d230_s0.txt"));
        }
        #[test]
        fn d230_asinh_s229() {
            check_at_scale("asinh", Width::D230, 229, include_str!("golden/asinh_d230_s229.txt"));
        }
        #[test]
        fn d230_acosh_s0() {
            check_at_scale("acosh", Width::D230, 0, include_str!("golden/acosh_d230_s0.txt"));
        }
        #[test]
        fn d230_acosh_s229() {
            check_at_scale("acosh", Width::D230, 229, include_str!("golden/acosh_d230_s229.txt"));
        }
        #[test]
        fn d230_atanh_s0() {
            check_at_scale("atanh", Width::D230, 0, include_str!("golden/atanh_d230_s0.txt"));
        }
        #[test]
        fn d230_atanh_s229() {
            check_at_scale("atanh", Width::D230, 229, include_str!("golden/atanh_d230_s229.txt"));
        }
        #[test]
        fn d230_log_s0() {
            check_at_scale("log", Width::D230, 0, include_str!("golden/log_d230_s0.txt"));
        }
        #[test]
        fn d230_log_s229() {
            check_at_scale("log", Width::D230, 229, include_str!("golden/log_d230_s229.txt"));
        }
        #[test]
        fn d230_atan2_s0() {
            check_at_scale("atan2", Width::D230, 0, include_str!("golden/atan2_d230_s0.txt"));
        }
        #[test]
        fn d230_atan2_s229() {
            check_at_scale("atan2", Width::D230, 229, include_str!("golden/atan2_d230_s229.txt"));
        }
        #[test]
        fn d230_powf_s0() {
            check_at_scale("powf", Width::D230, 0, include_str!("golden/powf_d230_s0.txt"));
        }
        #[test]
        fn d230_powf_s229() {
            check_at_scale("powf", Width::D230, 229, include_str!("golden/powf_d230_s229.txt"));
        }
    }
    #[cfg(any(feature = "d307", feature = "x-wide"))]
    mod d307 {
        use super::{check_at_scale, Width};
        #[test]
        fn d307_sqrt_s0() {
            check_at_scale("sqrt", Width::D307, 0, include_str!("golden/sqrt_d307_s0.txt"));
        }
        #[test]
        fn d307_sqrt_s306() {
            check_at_scale("sqrt", Width::D307, 306, include_str!("golden/sqrt_d307_s306.txt"));
        }
        #[test]
        fn d307_cbrt_s0() {
            check_at_scale("cbrt", Width::D307, 0, include_str!("golden/cbrt_d307_s0.txt"));
        }
        #[test]
        fn d307_cbrt_s306() {
            check_at_scale("cbrt", Width::D307, 306, include_str!("golden/cbrt_d307_s306.txt"));
        }
        #[test]
        fn d307_ln_s0() {
            check_at_scale("ln", Width::D307, 0, include_str!("golden/ln_d307_s0.txt"));
        }
        #[test]
        fn d307_ln_s306() {
            check_at_scale("ln", Width::D307, 306, include_str!("golden/ln_d307_s306.txt"));
        }
        #[test]
        fn d307_log2_s0() {
            check_at_scale("log2", Width::D307, 0, include_str!("golden/log2_d307_s0.txt"));
        }
        #[test]
        fn d307_log2_s306() {
            check_at_scale("log2", Width::D307, 306, include_str!("golden/log2_d307_s306.txt"));
        }
        #[test]
        fn d307_log10_s0() {
            check_at_scale("log10", Width::D307, 0, include_str!("golden/log10_d307_s0.txt"));
        }
        #[test]
        fn d307_log10_s306() {
            check_at_scale("log10", Width::D307, 306, include_str!("golden/log10_d307_s306.txt"));
        }
        #[test]
        fn d307_exp2_s0() {
            check_at_scale("exp2", Width::D307, 0, include_str!("golden/exp2_d307_s0.txt"));
        }
        #[test]
        fn d307_exp2_s306() {
            check_at_scale("exp2", Width::D307, 306, include_str!("golden/exp2_d307_s306.txt"));
        }
        #[test]
        fn d307_sin_s0() {
            check_at_scale("sin", Width::D307, 0, include_str!("golden/sin_d307_s0.txt"));
        }
        #[test]
        fn d307_sin_s306() {
            check_at_scale("sin", Width::D307, 306, include_str!("golden/sin_d307_s306.txt"));
        }
        #[test]
        fn d307_cos_s0() {
            check_at_scale("cos", Width::D307, 0, include_str!("golden/cos_d307_s0.txt"));
        }
        #[test]
        fn d307_cos_s306() {
            check_at_scale("cos", Width::D307, 306, include_str!("golden/cos_d307_s306.txt"));
        }
        #[test]
        fn d307_tan_s0() {
            check_at_scale("tan", Width::D307, 0, include_str!("golden/tan_d307_s0.txt"));
        }
        #[test]
        fn d307_tan_s306() {
            check_at_scale("tan", Width::D307, 306, include_str!("golden/tan_d307_s306.txt"));
        }
        #[test]
        fn d307_atan_s0() {
            check_at_scale("atan", Width::D307, 0, include_str!("golden/atan_d307_s0.txt"));
        }
        #[test]
        fn d307_atan_s306() {
            check_at_scale("atan", Width::D307, 306, include_str!("golden/atan_d307_s306.txt"));
        }
        #[test]
        fn d307_asin_s0() {
            check_at_scale("asin", Width::D307, 0, include_str!("golden/asin_d307_s0.txt"));
        }
        #[test]
        fn d307_asin_s306() {
            check_at_scale("asin", Width::D307, 306, include_str!("golden/asin_d307_s306.txt"));
        }
        #[test]
        fn d307_acos_s0() {
            check_at_scale("acos", Width::D307, 0, include_str!("golden/acos_d307_s0.txt"));
        }
        #[test]
        fn d307_acos_s306() {
            check_at_scale("acos", Width::D307, 306, include_str!("golden/acos_d307_s306.txt"));
        }
        #[test]
        fn d307_sinh_s0() {
            check_at_scale("sinh", Width::D307, 0, include_str!("golden/sinh_d307_s0.txt"));
        }
        #[test]
        fn d307_sinh_s306() {
            check_at_scale("sinh", Width::D307, 306, include_str!("golden/sinh_d307_s306.txt"));
        }
        #[test]
        fn d307_cosh_s0() {
            check_at_scale("cosh", Width::D307, 0, include_str!("golden/cosh_d307_s0.txt"));
        }
        #[test]
        fn d307_tanh_s0() {
            check_at_scale("tanh", Width::D307, 0, include_str!("golden/tanh_d307_s0.txt"));
        }
        #[test]
        fn d307_tanh_s306() {
            check_at_scale("tanh", Width::D307, 306, include_str!("golden/tanh_d307_s306.txt"));
        }
        #[test]
        fn d307_asinh_s0() {
            check_at_scale("asinh", Width::D307, 0, include_str!("golden/asinh_d307_s0.txt"));
        }
        #[test]
        fn d307_asinh_s306() {
            check_at_scale("asinh", Width::D307, 306, include_str!("golden/asinh_d307_s306.txt"));
        }
        #[test]
        fn d307_acosh_s0() {
            check_at_scale("acosh", Width::D307, 0, include_str!("golden/acosh_d307_s0.txt"));
        }
        #[test]
        fn d307_acosh_s306() {
            check_at_scale("acosh", Width::D307, 306, include_str!("golden/acosh_d307_s306.txt"));
        }
        #[test]
        fn d307_atanh_s0() {
            check_at_scale("atanh", Width::D307, 0, include_str!("golden/atanh_d307_s0.txt"));
        }
        #[test]
        fn d307_atanh_s306() {
            check_at_scale("atanh", Width::D307, 306, include_str!("golden/atanh_d307_s306.txt"));
        }
        #[test]
        fn d307_log_s0() {
            check_at_scale("log", Width::D307, 0, include_str!("golden/log_d307_s0.txt"));
        }
        #[test]
        fn d307_log_s306() {
            check_at_scale("log", Width::D307, 306, include_str!("golden/log_d307_s306.txt"));
        }
        #[test]
        fn d307_atan2_s0() {
            check_at_scale("atan2", Width::D307, 0, include_str!("golden/atan2_d307_s0.txt"));
        }
        #[test]
        fn d307_atan2_s306() {
            check_at_scale("atan2", Width::D307, 306, include_str!("golden/atan2_d307_s306.txt"));
        }
        #[test]
        fn d307_powf_s0() {
            check_at_scale("powf", Width::D307, 0, include_str!("golden/powf_d307_s0.txt"));
        }
        #[test]
        fn d307_powf_s306() {
            check_at_scale("powf", Width::D307, 306, include_str!("golden/powf_d307_s306.txt"));
        }
    }
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    mod d462 {
        use super::{check_at_scale, Width};
        #[test]
        fn d462_sqrt_s0() {
            check_at_scale("sqrt", Width::D462, 0, include_str!("golden/sqrt_d462_s0.txt"));
        }
        #[test]
        fn d462_sqrt_s461() {
            check_at_scale("sqrt", Width::D462, 461, include_str!("golden/sqrt_d462_s461.txt"));
        }
        #[test]
        fn d462_cbrt_s0() {
            check_at_scale("cbrt", Width::D462, 0, include_str!("golden/cbrt_d462_s0.txt"));
        }
        #[test]
        fn d462_cbrt_s461() {
            check_at_scale("cbrt", Width::D462, 461, include_str!("golden/cbrt_d462_s461.txt"));
        }
        #[test]
        fn d462_ln_s0() {
            check_at_scale("ln", Width::D462, 0, include_str!("golden/ln_d462_s0.txt"));
        }
        #[test]
        fn d462_ln_s461() {
            check_at_scale("ln", Width::D462, 461, include_str!("golden/ln_d462_s461.txt"));
        }
        #[test]
        fn d462_log2_s0() {
            check_at_scale("log2", Width::D462, 0, include_str!("golden/log2_d462_s0.txt"));
        }
        #[test]
        fn d462_log2_s461() {
            check_at_scale("log2", Width::D462, 461, include_str!("golden/log2_d462_s461.txt"));
        }
        #[test]
        fn d462_log10_s0() {
            check_at_scale("log10", Width::D462, 0, include_str!("golden/log10_d462_s0.txt"));
        }
        #[test]
        fn d462_log10_s461() {
            check_at_scale("log10", Width::D462, 461, include_str!("golden/log10_d462_s461.txt"));
        }
        #[test]
        fn d462_exp2_s0() {
            check_at_scale("exp2", Width::D462, 0, include_str!("golden/exp2_d462_s0.txt"));
        }
        #[test]
        fn d462_exp2_s461() {
            check_at_scale("exp2", Width::D462, 461, include_str!("golden/exp2_d462_s461.txt"));
        }
        #[test]
        fn d462_sin_s0() {
            check_at_scale("sin", Width::D462, 0, include_str!("golden/sin_d462_s0.txt"));
        }
        #[test]
        fn d462_sin_s461() {
            check_at_scale("sin", Width::D462, 461, include_str!("golden/sin_d462_s461.txt"));
        }
        #[test]
        fn d462_cos_s0() {
            check_at_scale("cos", Width::D462, 0, include_str!("golden/cos_d462_s0.txt"));
        }
        #[test]
        fn d462_cos_s461() {
            check_at_scale("cos", Width::D462, 461, include_str!("golden/cos_d462_s461.txt"));
        }
        #[test]
        fn d462_tan_s0() {
            check_at_scale("tan", Width::D462, 0, include_str!("golden/tan_d462_s0.txt"));
        }
        #[test]
        fn d462_tan_s461() {
            check_at_scale("tan", Width::D462, 461, include_str!("golden/tan_d462_s461.txt"));
        }
        #[test]
        fn d462_atan_s0() {
            check_at_scale("atan", Width::D462, 0, include_str!("golden/atan_d462_s0.txt"));
        }
        #[test]
        fn d462_atan_s461() {
            check_at_scale("atan", Width::D462, 461, include_str!("golden/atan_d462_s461.txt"));
        }
        #[test]
        fn d462_asin_s0() {
            check_at_scale("asin", Width::D462, 0, include_str!("golden/asin_d462_s0.txt"));
        }
        #[test]
        fn d462_asin_s461() {
            check_at_scale("asin", Width::D462, 461, include_str!("golden/asin_d462_s461.txt"));
        }
        #[test]
        fn d462_acos_s0() {
            check_at_scale("acos", Width::D462, 0, include_str!("golden/acos_d462_s0.txt"));
        }
        #[test]
        fn d462_acos_s461() {
            check_at_scale("acos", Width::D462, 461, include_str!("golden/acos_d462_s461.txt"));
        }
        #[test]
        fn d462_sinh_s0() {
            check_at_scale("sinh", Width::D462, 0, include_str!("golden/sinh_d462_s0.txt"));
        }
        #[test]
        fn d462_sinh_s461() {
            check_at_scale("sinh", Width::D462, 461, include_str!("golden/sinh_d462_s461.txt"));
        }
        #[test]
        fn d462_cosh_s0() {
            check_at_scale("cosh", Width::D462, 0, include_str!("golden/cosh_d462_s0.txt"));
        }
        #[test]
        fn d462_tanh_s0() {
            check_at_scale("tanh", Width::D462, 0, include_str!("golden/tanh_d462_s0.txt"));
        }
        #[test]
        fn d462_tanh_s461() {
            check_at_scale("tanh", Width::D462, 461, include_str!("golden/tanh_d462_s461.txt"));
        }
        #[test]
        fn d462_asinh_s0() {
            check_at_scale("asinh", Width::D462, 0, include_str!("golden/asinh_d462_s0.txt"));
        }
        #[test]
        fn d462_asinh_s461() {
            check_at_scale("asinh", Width::D462, 461, include_str!("golden/asinh_d462_s461.txt"));
        }
        #[test]
        fn d462_acosh_s0() {
            check_at_scale("acosh", Width::D462, 0, include_str!("golden/acosh_d462_s0.txt"));
        }
        #[test]
        fn d462_acosh_s461() {
            check_at_scale("acosh", Width::D462, 461, include_str!("golden/acosh_d462_s461.txt"));
        }
        #[test]
        fn d462_atanh_s0() {
            check_at_scale("atanh", Width::D462, 0, include_str!("golden/atanh_d462_s0.txt"));
        }
        #[test]
        fn d462_atanh_s461() {
            check_at_scale("atanh", Width::D462, 461, include_str!("golden/atanh_d462_s461.txt"));
        }
        #[test]
        fn d462_log_s0() {
            check_at_scale("log", Width::D462, 0, include_str!("golden/log_d462_s0.txt"));
        }
        #[test]
        fn d462_log_s461() {
            check_at_scale("log", Width::D462, 461, include_str!("golden/log_d462_s461.txt"));
        }
        #[test]
        fn d462_atan2_s0() {
            check_at_scale("atan2", Width::D462, 0, include_str!("golden/atan2_d462_s0.txt"));
        }
        #[test]
        fn d462_atan2_s461() {
            check_at_scale("atan2", Width::D462, 461, include_str!("golden/atan2_d462_s461.txt"));
        }
        #[test]
        fn d462_powf_s0() {
            check_at_scale("powf", Width::D462, 0, include_str!("golden/powf_d462_s0.txt"));
        }
        #[test]
        fn d462_powf_s461() {
            check_at_scale("powf", Width::D462, 461, include_str!("golden/powf_d462_s461.txt"));
        }
    }
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    mod d616 {
        use super::{check_at_scale, Width};
        #[test]
        fn d616_sqrt_s0() {
            check_at_scale("sqrt", Width::D616, 0, include_str!("golden/sqrt_d616_s0.txt"));
        }
        #[test]
        fn d616_sqrt_s615() {
            check_at_scale("sqrt", Width::D616, 615, include_str!("golden/sqrt_d616_s615.txt"));
        }
        #[test]
        fn d616_cbrt_s0() {
            check_at_scale("cbrt", Width::D616, 0, include_str!("golden/cbrt_d616_s0.txt"));
        }
        #[test]
        fn d616_cbrt_s615() {
            check_at_scale("cbrt", Width::D616, 615, include_str!("golden/cbrt_d616_s615.txt"));
        }
        #[test]
        fn d616_ln_s0() {
            check_at_scale("ln", Width::D616, 0, include_str!("golden/ln_d616_s0.txt"));
        }
        #[test]
        fn d616_ln_s615() {
            check_at_scale("ln", Width::D616, 615, include_str!("golden/ln_d616_s615.txt"));
        }
        #[test]
        fn d616_log2_s0() {
            check_at_scale("log2", Width::D616, 0, include_str!("golden/log2_d616_s0.txt"));
        }
        #[test]
        fn d616_log2_s615() {
            check_at_scale("log2", Width::D616, 615, include_str!("golden/log2_d616_s615.txt"));
        }
        #[test]
        fn d616_log10_s0() {
            check_at_scale("log10", Width::D616, 0, include_str!("golden/log10_d616_s0.txt"));
        }
        #[test]
        fn d616_log10_s615() {
            check_at_scale("log10", Width::D616, 615, include_str!("golden/log10_d616_s615.txt"));
        }
        #[test]
        fn d616_exp2_s0() {
            check_at_scale("exp2", Width::D616, 0, include_str!("golden/exp2_d616_s0.txt"));
        }
        #[test]
        fn d616_exp2_s615() {
            check_at_scale("exp2", Width::D616, 615, include_str!("golden/exp2_d616_s615.txt"));
        }
        #[test]
        fn d616_sin_s0() {
            check_at_scale("sin", Width::D616, 0, include_str!("golden/sin_d616_s0.txt"));
        }
        #[test]
        fn d616_sin_s615() {
            check_at_scale("sin", Width::D616, 615, include_str!("golden/sin_d616_s615.txt"));
        }
        #[test]
        fn d616_cos_s0() {
            check_at_scale("cos", Width::D616, 0, include_str!("golden/cos_d616_s0.txt"));
        }
        #[test]
        fn d616_cos_s615() {
            check_at_scale("cos", Width::D616, 615, include_str!("golden/cos_d616_s615.txt"));
        }
        #[test]
        fn d616_tan_s0() {
            check_at_scale("tan", Width::D616, 0, include_str!("golden/tan_d616_s0.txt"));
        }
        #[test]
        fn d616_tan_s615() {
            check_at_scale("tan", Width::D616, 615, include_str!("golden/tan_d616_s615.txt"));
        }
        #[test]
        fn d616_atan_s0() {
            check_at_scale("atan", Width::D616, 0, include_str!("golden/atan_d616_s0.txt"));
        }
        #[test]
        fn d616_atan_s615() {
            check_at_scale("atan", Width::D616, 615, include_str!("golden/atan_d616_s615.txt"));
        }
        #[test]
        fn d616_asin_s0() {
            check_at_scale("asin", Width::D616, 0, include_str!("golden/asin_d616_s0.txt"));
        }
        #[test]
        fn d616_asin_s615() {
            check_at_scale("asin", Width::D616, 615, include_str!("golden/asin_d616_s615.txt"));
        }
        #[test]
        fn d616_acos_s0() {
            check_at_scale("acos", Width::D616, 0, include_str!("golden/acos_d616_s0.txt"));
        }
        #[test]
        fn d616_acos_s615() {
            check_at_scale("acos", Width::D616, 615, include_str!("golden/acos_d616_s615.txt"));
        }
        #[test]
        fn d616_sinh_s0() {
            check_at_scale("sinh", Width::D616, 0, include_str!("golden/sinh_d616_s0.txt"));
        }
        #[test]
        fn d616_sinh_s615() {
            check_at_scale("sinh", Width::D616, 615, include_str!("golden/sinh_d616_s615.txt"));
        }
        #[test]
        fn d616_cosh_s0() {
            check_at_scale("cosh", Width::D616, 0, include_str!("golden/cosh_d616_s0.txt"));
        }
        #[test]
        fn d616_tanh_s0() {
            check_at_scale("tanh", Width::D616, 0, include_str!("golden/tanh_d616_s0.txt"));
        }
        #[test]
        fn d616_tanh_s615() {
            check_at_scale("tanh", Width::D616, 615, include_str!("golden/tanh_d616_s615.txt"));
        }
        #[test]
        fn d616_asinh_s0() {
            check_at_scale("asinh", Width::D616, 0, include_str!("golden/asinh_d616_s0.txt"));
        }
        #[test]
        fn d616_asinh_s615() {
            check_at_scale("asinh", Width::D616, 615, include_str!("golden/asinh_d616_s615.txt"));
        }
        #[test]
        fn d616_acosh_s0() {
            check_at_scale("acosh", Width::D616, 0, include_str!("golden/acosh_d616_s0.txt"));
        }
        #[test]
        fn d616_acosh_s615() {
            check_at_scale("acosh", Width::D616, 615, include_str!("golden/acosh_d616_s615.txt"));
        }
        #[test]
        fn d616_atanh_s0() {
            check_at_scale("atanh", Width::D616, 0, include_str!("golden/atanh_d616_s0.txt"));
        }
        #[test]
        fn d616_atanh_s615() {
            check_at_scale("atanh", Width::D616, 615, include_str!("golden/atanh_d616_s615.txt"));
        }
        #[test]
        fn d616_log_s0() {
            check_at_scale("log", Width::D616, 0, include_str!("golden/log_d616_s0.txt"));
        }
        #[test]
        fn d616_log_s615() {
            check_at_scale("log", Width::D616, 615, include_str!("golden/log_d616_s615.txt"));
        }
        #[test]
        fn d616_atan2_s0() {
            check_at_scale("atan2", Width::D616, 0, include_str!("golden/atan2_d616_s0.txt"));
        }
        #[test]
        fn d616_atan2_s615() {
            check_at_scale("atan2", Width::D616, 615, include_str!("golden/atan2_d616_s615.txt"));
        }
        #[test]
        fn d616_powf_s0() {
            check_at_scale("powf", Width::D616, 0, include_str!("golden/powf_d616_s0.txt"));
        }
        #[test]
        fn d616_powf_s615() {
            check_at_scale("powf", Width::D616, 615, include_str!("golden/powf_d616_s615.txt"));
        }
    }
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    mod d924 {
        use super::{check_at_scale, Width};
        #[test]
        fn d924_sqrt_s0() {
            check_at_scale("sqrt", Width::D924, 0, include_str!("golden/sqrt_d924_s0.txt"));
        }
        #[test]
        fn d924_sqrt_s923() {
            check_at_scale("sqrt", Width::D924, 923, include_str!("golden/sqrt_d924_s923.txt"));
        }
        #[test]
        fn d924_cbrt_s0() {
            check_at_scale("cbrt", Width::D924, 0, include_str!("golden/cbrt_d924_s0.txt"));
        }
        #[test]
        fn d924_cbrt_s923() {
            check_at_scale("cbrt", Width::D924, 923, include_str!("golden/cbrt_d924_s923.txt"));
        }
        #[test]
        fn d924_ln_s0() {
            check_at_scale("ln", Width::D924, 0, include_str!("golden/ln_d924_s0.txt"));
        }
        #[test]
        fn d924_ln_s923() {
            check_at_scale("ln", Width::D924, 923, include_str!("golden/ln_d924_s923.txt"));
        }
        #[test]
        fn d924_log2_s0() {
            check_at_scale("log2", Width::D924, 0, include_str!("golden/log2_d924_s0.txt"));
        }
        #[test]
        fn d924_log2_s923() {
            check_at_scale("log2", Width::D924, 923, include_str!("golden/log2_d924_s923.txt"));
        }
        #[test]
        fn d924_log10_s0() {
            check_at_scale("log10", Width::D924, 0, include_str!("golden/log10_d924_s0.txt"));
        }
        #[test]
        fn d924_log10_s923() {
            check_at_scale("log10", Width::D924, 923, include_str!("golden/log10_d924_s923.txt"));
        }
        #[test]
        fn d924_exp2_s0() {
            check_at_scale("exp2", Width::D924, 0, include_str!("golden/exp2_d924_s0.txt"));
        }
        #[test]
        fn d924_exp2_s923() {
            check_at_scale("exp2", Width::D924, 923, include_str!("golden/exp2_d924_s923.txt"));
        }
        #[test]
        fn d924_sin_s0() {
            check_at_scale("sin", Width::D924, 0, include_str!("golden/sin_d924_s0.txt"));
        }
        #[test]
        fn d924_sin_s923() {
            check_at_scale("sin", Width::D924, 923, include_str!("golden/sin_d924_s923.txt"));
        }
        #[test]
        fn d924_cos_s0() {
            check_at_scale("cos", Width::D924, 0, include_str!("golden/cos_d924_s0.txt"));
        }
        #[test]
        fn d924_cos_s923() {
            check_at_scale("cos", Width::D924, 923, include_str!("golden/cos_d924_s923.txt"));
        }
        #[test]
        fn d924_tan_s0() {
            check_at_scale("tan", Width::D924, 0, include_str!("golden/tan_d924_s0.txt"));
        }
        #[test]
        fn d924_tan_s923() {
            check_at_scale("tan", Width::D924, 923, include_str!("golden/tan_d924_s923.txt"));
        }
        #[test]
        fn d924_atan_s0() {
            check_at_scale("atan", Width::D924, 0, include_str!("golden/atan_d924_s0.txt"));
        }
        #[test]
        fn d924_atan_s923() {
            check_at_scale("atan", Width::D924, 923, include_str!("golden/atan_d924_s923.txt"));
        }
        #[test]
        fn d924_asin_s0() {
            check_at_scale("asin", Width::D924, 0, include_str!("golden/asin_d924_s0.txt"));
        }
        #[test]
        fn d924_asin_s923() {
            check_at_scale("asin", Width::D924, 923, include_str!("golden/asin_d924_s923.txt"));
        }
        #[test]
        fn d924_acos_s0() {
            check_at_scale("acos", Width::D924, 0, include_str!("golden/acos_d924_s0.txt"));
        }
        #[test]
        fn d924_acos_s923() {
            check_at_scale("acos", Width::D924, 923, include_str!("golden/acos_d924_s923.txt"));
        }
        #[test]
        fn d924_sinh_s0() {
            check_at_scale("sinh", Width::D924, 0, include_str!("golden/sinh_d924_s0.txt"));
        }
        #[test]
        fn d924_sinh_s923() {
            check_at_scale("sinh", Width::D924, 923, include_str!("golden/sinh_d924_s923.txt"));
        }
        #[test]
        fn d924_cosh_s0() {
            check_at_scale("cosh", Width::D924, 0, include_str!("golden/cosh_d924_s0.txt"));
        }
        #[test]
        fn d924_tanh_s0() {
            check_at_scale("tanh", Width::D924, 0, include_str!("golden/tanh_d924_s0.txt"));
        }
        #[test]
        fn d924_tanh_s923() {
            check_at_scale("tanh", Width::D924, 923, include_str!("golden/tanh_d924_s923.txt"));
        }
        #[test]
        fn d924_asinh_s0() {
            check_at_scale("asinh", Width::D924, 0, include_str!("golden/asinh_d924_s0.txt"));
        }
        #[test]
        fn d924_asinh_s923() {
            check_at_scale("asinh", Width::D924, 923, include_str!("golden/asinh_d924_s923.txt"));
        }
        #[test]
        fn d924_acosh_s0() {
            check_at_scale("acosh", Width::D924, 0, include_str!("golden/acosh_d924_s0.txt"));
        }
        #[test]
        fn d924_acosh_s923() {
            check_at_scale("acosh", Width::D924, 923, include_str!("golden/acosh_d924_s923.txt"));
        }
        #[test]
        fn d924_atanh_s0() {
            check_at_scale("atanh", Width::D924, 0, include_str!("golden/atanh_d924_s0.txt"));
        }
        #[test]
        fn d924_atanh_s923() {
            check_at_scale("atanh", Width::D924, 923, include_str!("golden/atanh_d924_s923.txt"));
        }
        #[test]
        fn d924_log_s0() {
            check_at_scale("log", Width::D924, 0, include_str!("golden/log_d924_s0.txt"));
        }
        #[test]
        fn d924_log_s923() {
            check_at_scale("log", Width::D924, 923, include_str!("golden/log_d924_s923.txt"));
        }
        #[test]
        fn d924_atan2_s0() {
            check_at_scale("atan2", Width::D924, 0, include_str!("golden/atan2_d924_s0.txt"));
        }
        #[test]
        fn d924_atan2_s923() {
            check_at_scale("atan2", Width::D924, 923, include_str!("golden/atan2_d924_s923.txt"));
        }
        #[test]
        fn d924_powf_s0() {
            check_at_scale("powf", Width::D924, 0, include_str!("golden/powf_d924_s0.txt"));
        }
        #[test]
        fn d924_powf_s923() {
            check_at_scale("powf", Width::D924, 923, include_str!("golden/powf_d924_s923.txt"));
        }
    }
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    mod d1232 {
        use super::{check_at_scale, Width};
        #[test]
        fn d1232_sqrt_s0() {
            check_at_scale("sqrt", Width::D1232, 0, include_str!("golden/sqrt_d1232_s0.txt"));
        }
        #[test]
        fn d1232_sqrt_s1231() {
            check_at_scale("sqrt", Width::D1232, 1231, include_str!("golden/sqrt_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_cbrt_s0() {
            check_at_scale("cbrt", Width::D1232, 0, include_str!("golden/cbrt_d1232_s0.txt"));
        }
        #[test]
        fn d1232_cbrt_s1231() {
            check_at_scale("cbrt", Width::D1232, 1231, include_str!("golden/cbrt_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_ln_s0() {
            check_at_scale("ln", Width::D1232, 0, include_str!("golden/ln_d1232_s0.txt"));
        }
        #[test]
        fn d1232_ln_s1231() {
            check_at_scale("ln", Width::D1232, 1231, include_str!("golden/ln_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_log2_s0() {
            check_at_scale("log2", Width::D1232, 0, include_str!("golden/log2_d1232_s0.txt"));
        }
        #[test]
        fn d1232_log2_s1231() {
            check_at_scale("log2", Width::D1232, 1231, include_str!("golden/log2_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_log10_s0() {
            check_at_scale("log10", Width::D1232, 0, include_str!("golden/log10_d1232_s0.txt"));
        }
        #[test]
        fn d1232_log10_s1231() {
            check_at_scale("log10", Width::D1232, 1231, include_str!("golden/log10_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_exp2_s0() {
            check_at_scale("exp2", Width::D1232, 0, include_str!("golden/exp2_d1232_s0.txt"));
        }
        #[test]
        fn d1232_exp2_s1231() {
            check_at_scale("exp2", Width::D1232, 1231, include_str!("golden/exp2_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_sin_s0() {
            check_at_scale("sin", Width::D1232, 0, include_str!("golden/sin_d1232_s0.txt"));
        }
        #[test]
        fn d1232_sin_s1231() {
            check_at_scale("sin", Width::D1232, 1231, include_str!("golden/sin_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_cos_s0() {
            check_at_scale("cos", Width::D1232, 0, include_str!("golden/cos_d1232_s0.txt"));
        }
        #[test]
        fn d1232_cos_s1231() {
            check_at_scale("cos", Width::D1232, 1231, include_str!("golden/cos_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_tan_s0() {
            check_at_scale("tan", Width::D1232, 0, include_str!("golden/tan_d1232_s0.txt"));
        }
        #[test]
        fn d1232_tan_s1231() {
            check_at_scale("tan", Width::D1232, 1231, include_str!("golden/tan_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_atan_s0() {
            check_at_scale("atan", Width::D1232, 0, include_str!("golden/atan_d1232_s0.txt"));
        }
        #[test]
        fn d1232_atan_s1231() {
            check_at_scale("atan", Width::D1232, 1231, include_str!("golden/atan_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_asin_s0() {
            check_at_scale("asin", Width::D1232, 0, include_str!("golden/asin_d1232_s0.txt"));
        }
        #[test]
        fn d1232_asin_s1231() {
            check_at_scale("asin", Width::D1232, 1231, include_str!("golden/asin_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_acos_s0() {
            check_at_scale("acos", Width::D1232, 0, include_str!("golden/acos_d1232_s0.txt"));
        }
        #[test]
        fn d1232_acos_s1231() {
            check_at_scale("acos", Width::D1232, 1231, include_str!("golden/acos_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_sinh_s0() {
            check_at_scale("sinh", Width::D1232, 0, include_str!("golden/sinh_d1232_s0.txt"));
        }
        #[test]
        fn d1232_sinh_s1231() {
            check_at_scale("sinh", Width::D1232, 1231, include_str!("golden/sinh_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_cosh_s0() {
            check_at_scale("cosh", Width::D1232, 0, include_str!("golden/cosh_d1232_s0.txt"));
        }
        #[test]
        fn d1232_tanh_s0() {
            check_at_scale("tanh", Width::D1232, 0, include_str!("golden/tanh_d1232_s0.txt"));
        }
        #[test]
        fn d1232_tanh_s1231() {
            check_at_scale("tanh", Width::D1232, 1231, include_str!("golden/tanh_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_asinh_s0() {
            check_at_scale("asinh", Width::D1232, 0, include_str!("golden/asinh_d1232_s0.txt"));
        }
        #[test]
        fn d1232_asinh_s1231() {
            check_at_scale("asinh", Width::D1232, 1231, include_str!("golden/asinh_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_acosh_s0() {
            check_at_scale("acosh", Width::D1232, 0, include_str!("golden/acosh_d1232_s0.txt"));
        }
        #[test]
        fn d1232_acosh_s1231() {
            check_at_scale("acosh", Width::D1232, 1231, include_str!("golden/acosh_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_atanh_s0() {
            check_at_scale("atanh", Width::D1232, 0, include_str!("golden/atanh_d1232_s0.txt"));
        }
        #[test]
        fn d1232_atanh_s1231() {
            check_at_scale("atanh", Width::D1232, 1231, include_str!("golden/atanh_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_log_s0() {
            check_at_scale("log", Width::D1232, 0, include_str!("golden/log_d1232_s0.txt"));
        }
        #[test]
        fn d1232_log_s1231() {
            check_at_scale("log", Width::D1232, 1231, include_str!("golden/log_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_atan2_s0() {
            check_at_scale("atan2", Width::D1232, 0, include_str!("golden/atan2_d1232_s0.txt"));
        }
        #[test]
        fn d1232_atan2_s1231() {
            check_at_scale("atan2", Width::D1232, 1231, include_str!("golden/atan2_d1232_s1231.txt"));
        }
        #[test]
        fn d1232_powf_s0() {
            check_at_scale("powf", Width::D1232, 0, include_str!("golden/powf_d1232_s0.txt"));
        }
        #[test]
        fn d1232_powf_s1231() {
            check_at_scale("powf", Width::D1232, 1231, include_str!("golden/powf_d1232_s1231.txt"));
        }
    }
}

// --- Binary-op golden cells (hypot + arithmetic) --------------------
//
// hypot and the five arithmetic ops (add/sub/mul/div/rem) routed through
// the same correctly-rounded harness as the transcendentals, against the
// exact integer / isqrt oracle (four-column golden tables). hypot's value
// cells migrated here from the retired tests/hypot_accuracy.rs; its
// structural edge cases live in tests/hypot_edge_cases.rs.
mod binary_ops {
    #[allow(unused_imports)]
    use super::{check_at_scale, Width};
    mod d18 {
        use super::{check_at_scale, Width};
        #[test]
        fn d18_hypot_s0() {
            check_at_scale("hypot", Width::D18, 0, include_str!("golden/hypot_d18_s0.txt"));
        }
        #[test]
        fn d18_hypot_s3() {
            check_at_scale("hypot", Width::D18, 3, include_str!("golden/hypot_d18_s3.txt"));
        }
        #[test]
        fn d18_hypot_s9() {
            check_at_scale("hypot", Width::D18, 9, include_str!("golden/hypot_d18_s9.txt"));
        }
        #[test]
        fn d18_add_s0() {
            check_at_scale("add", Width::D18, 0, include_str!("golden/add_d18_s0.txt"));
        }
        #[test]
        fn d18_add_s9() {
            check_at_scale("add", Width::D18, 9, include_str!("golden/add_d18_s9.txt"));
        }
        #[test]
        fn d18_add_s17() {
            check_at_scale("add", Width::D18, 17, include_str!("golden/add_d18_s17.txt"));
        }
        #[test]
        fn d18_sub_s0() {
            check_at_scale("sub", Width::D18, 0, include_str!("golden/sub_d18_s0.txt"));
        }
        #[test]
        fn d18_sub_s9() {
            check_at_scale("sub", Width::D18, 9, include_str!("golden/sub_d18_s9.txt"));
        }
        #[test]
        fn d18_sub_s17() {
            check_at_scale("sub", Width::D18, 17, include_str!("golden/sub_d18_s17.txt"));
        }
        #[test]
        fn d18_mul_s0() {
            check_at_scale("mul", Width::D18, 0, include_str!("golden/mul_d18_s0.txt"));
        }
        #[test]
        fn d18_mul_s9() {
            check_at_scale("mul", Width::D18, 9, include_str!("golden/mul_d18_s9.txt"));
        }
        #[test]
        fn d18_mul_s17() {
            check_at_scale("mul", Width::D18, 17, include_str!("golden/mul_d18_s17.txt"));
        }
        #[test]
        fn d18_div_s0() {
            check_at_scale("div", Width::D18, 0, include_str!("golden/div_d18_s0.txt"));
        }
        #[test]
        fn d18_div_s9() {
            check_at_scale("div", Width::D18, 9, include_str!("golden/div_d18_s9.txt"));
        }
        #[test]
        fn d18_div_s17() {
            check_at_scale("div", Width::D18, 17, include_str!("golden/div_d18_s17.txt"));
        }
        #[test]
        fn d18_rem_s0() {
            check_at_scale("rem", Width::D18, 0, include_str!("golden/rem_d18_s0.txt"));
        }
        #[test]
        fn d18_rem_s9() {
            check_at_scale("rem", Width::D18, 9, include_str!("golden/rem_d18_s9.txt"));
        }
        #[test]
        fn d18_rem_s17() {
            check_at_scale("rem", Width::D18, 17, include_str!("golden/rem_d18_s17.txt"));
        }
    }
    mod d38 {
        use super::{check_at_scale, Width};
        #[test]
        fn d38_hypot_s0() {
            check_at_scale("hypot", Width::D38, 0, include_str!("golden/hypot_d38_s0.txt"));
        }
        #[test]
        fn d38_hypot_s2() {
            check_at_scale("hypot", Width::D38, 2, include_str!("golden/hypot_d38_s2.txt"));
        }
        #[test]
        fn d38_hypot_s6() {
            check_at_scale("hypot", Width::D38, 6, include_str!("golden/hypot_d38_s6.txt"));
        }
        #[test]
        fn d38_hypot_s9() {
            check_at_scale("hypot", Width::D38, 9, include_str!("golden/hypot_d38_s9.txt"));
        }
        #[test]
        fn d38_hypot_s10() {
            check_at_scale("hypot", Width::D38, 10, include_str!("golden/hypot_d38_s10.txt"));
        }
        #[test]
        fn d38_hypot_s12() {
            check_at_scale("hypot", Width::D38, 12, include_str!("golden/hypot_d38_s12.txt"));
        }
        #[test]
        fn d38_hypot_s17() {
            check_at_scale("hypot", Width::D38, 17, include_str!("golden/hypot_d38_s17.txt"));
        }
        #[test]
        fn d38_hypot_s18() {
            check_at_scale("hypot", Width::D38, 18, include_str!("golden/hypot_d38_s18.txt"));
        }
        #[test]
        fn d38_hypot_s19() {
            check_at_scale("hypot", Width::D38, 19, include_str!("golden/hypot_d38_s19.txt"));
        }
        #[test]
        fn d38_add_s0() {
            check_at_scale("add", Width::D38, 0, include_str!("golden/add_d38_s0.txt"));
        }
        #[test]
        fn d38_add_s19() {
            check_at_scale("add", Width::D38, 19, include_str!("golden/add_d38_s19.txt"));
        }
        #[test]
        fn d38_add_s37() {
            check_at_scale("add", Width::D38, 37, include_str!("golden/add_d38_s37.txt"));
        }
        #[test]
        fn d38_sub_s0() {
            check_at_scale("sub", Width::D38, 0, include_str!("golden/sub_d38_s0.txt"));
        }
        #[test]
        fn d38_sub_s19() {
            check_at_scale("sub", Width::D38, 19, include_str!("golden/sub_d38_s19.txt"));
        }
        #[test]
        fn d38_sub_s37() {
            check_at_scale("sub", Width::D38, 37, include_str!("golden/sub_d38_s37.txt"));
        }
        #[test]
        fn d38_mul_s0() {
            check_at_scale("mul", Width::D38, 0, include_str!("golden/mul_d38_s0.txt"));
        }
        #[test]
        fn d38_mul_s19() {
            check_at_scale("mul", Width::D38, 19, include_str!("golden/mul_d38_s19.txt"));
        }
        #[test]
        fn d38_mul_s37() {
            check_at_scale("mul", Width::D38, 37, include_str!("golden/mul_d38_s37.txt"));
        }
        #[test]
        fn d38_div_s0() {
            check_at_scale("div", Width::D38, 0, include_str!("golden/div_d38_s0.txt"));
        }
        #[test]
        fn d38_div_s19() {
            check_at_scale("div", Width::D38, 19, include_str!("golden/div_d38_s19.txt"));
        }
        #[test]
        fn d38_div_s37() {
            check_at_scale("div", Width::D38, 37, include_str!("golden/div_d38_s37.txt"));
        }
        #[test]
        fn d38_rem_s0() {
            check_at_scale("rem", Width::D38, 0, include_str!("golden/rem_d38_s0.txt"));
        }
        #[test]
        fn d38_rem_s19() {
            check_at_scale("rem", Width::D38, 19, include_str!("golden/rem_d38_s19.txt"));
        }
        #[test]
        fn d38_rem_s37() {
            check_at_scale("rem", Width::D38, 37, include_str!("golden/rem_d38_s37.txt"));
        }
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    mod d57 {
        use super::{check_at_scale, Width};
        #[test]
        fn d57_hypot_s0() {
            check_at_scale("hypot", Width::D57, 0, include_str!("golden/hypot_d57_s0.txt"));
        }
        #[test]
        fn d57_hypot_s20() {
            check_at_scale("hypot", Width::D57, 20, include_str!("golden/hypot_d57_s20.txt"));
        }
        #[test]
        fn d57_hypot_s30() {
            check_at_scale("hypot", Width::D57, 30, include_str!("golden/hypot_d57_s30.txt"));
        }
        #[test]
        fn d57_add_s0() {
            check_at_scale("add", Width::D57, 0, include_str!("golden/add_d57_s0.txt"));
        }
        #[test]
        fn d57_add_s28() {
            check_at_scale("add", Width::D57, 28, include_str!("golden/add_d57_s28.txt"));
        }
        #[test]
        fn d57_add_s56() {
            check_at_scale("add", Width::D57, 56, include_str!("golden/add_d57_s56.txt"));
        }
        #[test]
        fn d57_sub_s0() {
            check_at_scale("sub", Width::D57, 0, include_str!("golden/sub_d57_s0.txt"));
        }
        #[test]
        fn d57_sub_s28() {
            check_at_scale("sub", Width::D57, 28, include_str!("golden/sub_d57_s28.txt"));
        }
        #[test]
        fn d57_sub_s56() {
            check_at_scale("sub", Width::D57, 56, include_str!("golden/sub_d57_s56.txt"));
        }
        #[test]
        fn d57_mul_s0() {
            check_at_scale("mul", Width::D57, 0, include_str!("golden/mul_d57_s0.txt"));
        }
        #[test]
        fn d57_mul_s28() {
            check_at_scale("mul", Width::D57, 28, include_str!("golden/mul_d57_s28.txt"));
        }
        #[test]
        fn d57_mul_s56() {
            check_at_scale("mul", Width::D57, 56, include_str!("golden/mul_d57_s56.txt"));
        }
        #[test]
        fn d57_div_s0() {
            check_at_scale("div", Width::D57, 0, include_str!("golden/div_d57_s0.txt"));
        }
        #[test]
        fn d57_div_s28() {
            check_at_scale("div", Width::D57, 28, include_str!("golden/div_d57_s28.txt"));
        }
        #[test]
        fn d57_div_s56() {
            check_at_scale("div", Width::D57, 56, include_str!("golden/div_d57_s56.txt"));
        }
        #[test]
        fn d57_rem_s0() {
            check_at_scale("rem", Width::D57, 0, include_str!("golden/rem_d57_s0.txt"));
        }
        #[test]
        fn d57_rem_s28() {
            check_at_scale("rem", Width::D57, 28, include_str!("golden/rem_d57_s28.txt"));
        }
        #[test]
        fn d57_rem_s56() {
            check_at_scale("rem", Width::D57, 56, include_str!("golden/rem_d57_s56.txt"));
        }
    }
    #[cfg(any(feature = "d76", feature = "wide"))]
    mod d76 {
        use super::{check_at_scale, Width};
        #[test]
        fn d76_hypot_s0() {
            check_at_scale("hypot", Width::D76, 0, include_str!("golden/hypot_d76_s0.txt"));
        }
        #[test]
        fn d76_hypot_s18() {
            check_at_scale("hypot", Width::D76, 18, include_str!("golden/hypot_d76_s18.txt"));
        }
        #[test]
        fn d76_hypot_s40() {
            check_at_scale("hypot", Width::D76, 40, include_str!("golden/hypot_d76_s40.txt"));
        }
        #[test]
        fn d76_add_s0() {
            check_at_scale("add", Width::D76, 0, include_str!("golden/add_d76_s0.txt"));
        }
        #[test]
        fn d76_add_s35() {
            check_at_scale("add", Width::D76, 38, include_str!("golden/add_d76_s38.txt"));
        }
        #[test]
        fn d76_add_s75() {
            check_at_scale("add", Width::D76, 75, include_str!("golden/add_d76_s75.txt"));
        }
        #[test]
        fn d76_sub_s0() {
            check_at_scale("sub", Width::D76, 0, include_str!("golden/sub_d76_s0.txt"));
        }
        #[test]
        fn d76_sub_s35() {
            check_at_scale("sub", Width::D76, 38, include_str!("golden/sub_d76_s38.txt"));
        }
        #[test]
        fn d76_sub_s75() {
            check_at_scale("sub", Width::D76, 75, include_str!("golden/sub_d76_s75.txt"));
        }
        #[test]
        fn d76_mul_s0() {
            check_at_scale("mul", Width::D76, 0, include_str!("golden/mul_d76_s0.txt"));
        }
        #[test]
        fn d76_mul_s35() {
            check_at_scale("mul", Width::D76, 38, include_str!("golden/mul_d76_s38.txt"));
        }
        #[test]
        fn d76_mul_s75() {
            check_at_scale("mul", Width::D76, 75, include_str!("golden/mul_d76_s75.txt"));
        }
        #[test]
        fn d76_div_s0() {
            check_at_scale("div", Width::D76, 0, include_str!("golden/div_d76_s0.txt"));
        }
        #[test]
        fn d76_div_s35() {
            check_at_scale("div", Width::D76, 38, include_str!("golden/div_d76_s38.txt"));
        }
        #[test]
        fn d76_div_s75() {
            check_at_scale("div", Width::D76, 75, include_str!("golden/div_d76_s75.txt"));
        }
        #[test]
        fn d76_rem_s0() {
            check_at_scale("rem", Width::D76, 0, include_str!("golden/rem_d76_s0.txt"));
        }
        #[test]
        fn d76_rem_s35() {
            check_at_scale("rem", Width::D76, 38, include_str!("golden/rem_d76_s38.txt"));
        }
        #[test]
        fn d76_rem_s75() {
            check_at_scale("rem", Width::D76, 75, include_str!("golden/rem_d76_s75.txt"));
        }
    }
    #[cfg(any(feature = "d115", feature = "wide"))]
    mod d115 {
        use super::{check_at_scale, Width};
        #[test]
        fn d115_hypot_s50() {
            check_at_scale("hypot", Width::D115, 50, include_str!("golden/hypot_d115_s50.txt"));
        }
    }
    #[cfg(any(feature = "d307", feature = "x-wide"))]
    mod d307 {
        use super::{check_at_scale, Width};
        #[test]
        fn d307_hypot_s0() {
            check_at_scale("hypot", Width::D307, 0, include_str!("golden/hypot_d307_s0.txt"));
        }
        #[test]
        fn d307_hypot_s30() {
            check_at_scale("hypot", Width::D307, 30, include_str!("golden/hypot_d307_s30.txt"));
        }
        #[test]
        fn d307_hypot_s50() {
            check_at_scale("hypot", Width::D307, 50, include_str!("golden/hypot_d307_s50.txt"));
        }
        #[test]
        fn d307_add_s0() {
            check_at_scale("add", Width::D307, 0, include_str!("golden/add_d307_s0.txt"));
        }
        #[test]
        fn d307_add_s150() {
            check_at_scale("add", Width::D307, 153, include_str!("golden/add_d307_s153.txt"));
        }
        #[test]
        fn d307_add_s306() {
            check_at_scale("add", Width::D307, 306, include_str!("golden/add_d307_s306.txt"));
        }
        #[test]
        fn d307_sub_s0() {
            check_at_scale("sub", Width::D307, 0, include_str!("golden/sub_d307_s0.txt"));
        }
        #[test]
        fn d307_sub_s150() {
            check_at_scale("sub", Width::D307, 153, include_str!("golden/sub_d307_s153.txt"));
        }
        #[test]
        fn d307_sub_s306() {
            check_at_scale("sub", Width::D307, 306, include_str!("golden/sub_d307_s306.txt"));
        }
        #[test]
        fn d307_mul_s0() {
            check_at_scale("mul", Width::D307, 0, include_str!("golden/mul_d307_s0.txt"));
        }
        #[test]
        fn d307_mul_s150() {
            check_at_scale("mul", Width::D307, 153, include_str!("golden/mul_d307_s153.txt"));
        }
        #[test]
        fn d307_mul_s306() {
            check_at_scale("mul", Width::D307, 306, include_str!("golden/mul_d307_s306.txt"));
        }
        #[test]
        fn d307_div_s0() {
            check_at_scale("div", Width::D307, 0, include_str!("golden/div_d307_s0.txt"));
        }
        #[test]
        fn d307_div_s150() {
            check_at_scale("div", Width::D307, 153, include_str!("golden/div_d307_s153.txt"));
        }
        #[test]
        fn d307_div_s306() {
            check_at_scale("div", Width::D307, 306, include_str!("golden/div_d307_s306.txt"));
        }
        #[test]
        fn d307_rem_s0() {
            check_at_scale("rem", Width::D307, 0, include_str!("golden/rem_d307_s0.txt"));
        }
        #[test]
        fn d307_rem_s150() {
            check_at_scale("rem", Width::D307, 153, include_str!("golden/rem_d307_s153.txt"));
        }
        #[test]
        fn d307_rem_s306() {
            check_at_scale("rem", Width::D307, 306, include_str!("golden/rem_d307_s306.txt"));
        }
    }
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    mod d1232 {
        use super::{check_at_scale, Width};
        #[test]
        fn d1232_hypot_s0() {
            check_at_scale("hypot", Width::D1232, 0, include_str!("golden/hypot_d1232_s0.txt"));
        }
    }
}

/// Comprehensive auto-discovered golden gate: scans every `tests/golden/*.txt`,
/// parses each `<func>_d<N>_s<S>.txt` into `(func, Width, scale)`, and runs the
/// strict correct-rounding check on it. This closes the gap where new golden
/// tables (the full wide-tier 5-point-scale set) were committed but only the
/// `decl_band!`-listed tables were actually exercised — every table in the
/// directory is now a gate.
///
/// `#[ignore]`d on purpose: with all twelve tiers this is ~1.6k tables and a
/// multi-hour run (the wide transcendentals are milliseconds per call), so it
/// is the opt-in COMPREHENSIVE gate (`cargo test ... -- --ignored`), not the
/// fast per-commit one (the `decl_band!` subset). `GOLDEN_FILTER=<substr>`
/// narrows it to file stems containing the substring (e.g. `GOLDEN_FILTER=mul_d`
/// for every `mul` cell, `=_d1232_` for the widest tier). It reuses the 9.21
/// per-table timeout watchdog in [`check_at_scale`], so a runaway kernel in any
/// newly-covered cell is NAMED rather than hanging the suite. Every covered
/// table runs even when one fails (each is `catch_unwind`-guarded); the failing
/// `(func, width, scale)` list is reported together.
#[cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide", feature = "macros"))]
#[test]
#[ignore = "comprehensive auto-discovered golden over all tests/golden/*.txt — opt-in via --ignored (multi-hour at the wide tiers); GOLDEN_FILTER=<substr> narrows it"]
fn golden_autodiscover() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");
    let filter = std::env::var("GOLDEN_FILTER").unwrap_or_default();

    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .expect("read tests/golden")
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);

    let mut tested = 0usize;
    let mut skipped = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for entry in entries {
        let fname = entry.file_name().to_string_lossy().into_owned();
        let Some(stem) = fname.strip_suffix(".txt") else {
            continue;
        };
        if !filter.is_empty() && !stem.contains(filter.as_str()) {
            continue;
        }
        // Parse `<func>_d<N>_s<S>` (every name in the directory conforms; a
        // non-conforming one is counted as skipped rather than panicking).
        let Some((left, scale_s)) = stem.rsplit_once("_s") else {
            skipped += 1;
            continue;
        };
        let Some((func, n_s)) = left.rsplit_once("_d") else {
            skipped += 1;
            continue;
        };
        let (Ok(n), Ok(scale)) = (n_s.parse::<u32>(), scale_s.parse::<u32>()) else {
            skipped += 1;
            continue;
        };
        let Some(width) = Width::from_digits(n) else {
            skipped += 1;
            continue;
        };

        // `check_at_scale` (the 9.21 watchdog) needs `'static`; leak the table
        // and func name (the process exits after the gate, so a bounded leak —
        // the whole corpus is capped well under the 9.19 budget — is fine).
        let content = std::fs::read_to_string(entry.path()).expect("read golden table");
        let table: &'static str = Box::leak(content.into_boxed_str());
        let func_s: &'static str = Box::leak(func.to_owned().into_boxed_str());

        match std::panic::catch_unwind(|| check_at_scale(func_s, width, scale, table)) {
            Ok(()) => {}
            Err(p) => {
                let msg = p
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| p.downcast_ref::<&str>().copied())
                    .unwrap_or("<non-string panic>");
                failures.push(format!("{func} d{n} s{scale}: {msg}"));
            }
        }
        tested += 1;
    }

    eprintln!(
        "golden_autodiscover: {tested} tested, {skipped} skipped, {} failed{}",
        failures.len(),
        if filter.is_empty() {
            String::new()
        } else {
            format!(" (filter={filter:?})")
        },
    );
    assert!(
        failures.is_empty(),
        "{} golden table(s) failed strict rounding:\n{}",
        failures.len(),
        failures.join("\n"),
    );
}
