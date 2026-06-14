// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Concrete band-edge `(width, scale)` cell shims over decimal-scaled.
//!
//! Every generic decimal-scaled kernel monomorphises in the crate that
//! *instantiates* it, and stable rustc shares no generic instantiations across
//! crates — so whichever crate carries the band-edge cell fan-out pays the full
//! heavy-monomorphisation bill, and pays it AGAIN for every additional target
//! that re-instantiates the same cells (a lib's own unit-test harness, a second
//! test binary). This crate is the compile-once home for that bill: the
//! `cells!`-generated fan-out — parse → compute → format ([`dispatch_compute`])
//! and the storage envelope ([`dispatch_limits`]), per cell — compiles here,
//! exactly once, into a leaf rlib. The subjects above it (the erased
//! `DsSubject` in decimal-scale-test, the historical adapters in `history`)
//! call these concrete, non-generic entry points and stay light.
//!
//! Pure codegen placement: no new types, no algorithm bodies, no per-tier
//! logic — each shim is the same one-line delegate into the existing generic
//! surface it always was (the per-cell listing in the `cells!` macro is
//! instantiation enumeration, not algorithm duplication).

use decimal_scaled::{DecimalArithmetic, DecimalTranscendental, RoundingMode as DsMode};
use decimal_scaled_golden::{Computed, Function, Limits};

// Historical-release subjects for the version-history gates; each pinned
// release compiles only behind its `history-*` feature.
#[cfg(any(feature = "history-044", feature = "history-033"))]
pub mod history;

/// Every function the golden set covers (a missing file just contributes no cases).
pub const FUNCS: &[Function] = &[
    Function::Sqrt, Function::Cbrt, Function::Exp, Function::Ln, Function::Log2, Function::Log10,
    Function::Exp2, Function::Sin, Function::Cos, Function::Tan, Function::Atan, Function::Asin,
    Function::Acos, Function::Sinh, Function::Cosh, Function::Tanh, Function::Asinh, Function::Acosh,
    Function::Atanh, Function::Log, Function::Atan2, Function::Powf, Function::Hypot, Function::Add,
    Function::Sub, Function::Mul, Function::Div, Function::Rem,
];

/// Inherent rounded mul/div aren't on a width-generic trait, so bridge them
/// locally — one delegating impl per width, scale-generic.
pub trait DsOps: Sized {
    fn ds_mul_with(self, o: Self, m: DsMode) -> Self;
    fn ds_div_with(self, o: Self, m: DsMode) -> Self;
}
macro_rules! impl_ds_ops {
    ($( $(#[$cfg:meta])* $D:ident ),+ $(,)?) => { $(
        $(#[$cfg])*
        impl<const S: u32> DsOps for decimal_scaled::$D<S> {
            fn ds_mul_with(self, o: Self, m: DsMode) -> Self { self.mul_with(o, m) }
            fn ds_div_with(self, o: Self, m: DsMode) -> Self { self.div_with(o, m) }
        }
    )+ };
}
impl_ds_ops!(
    D18, D38,
    #[cfg(feature = "d57")] D57,
    #[cfg(feature = "d76")] D76,
    #[cfg(feature = "d115")] D115,
    #[cfg(feature = "d153")] D153,
    #[cfg(feature = "d230")] D230,
    #[cfg(feature = "d307")] D307,
    #[cfg(feature = "d462")] D462,
    #[cfg(feature = "d616")] D616,
    #[cfg(feature = "d924")] D924,
    #[cfg(feature = "d1232")] D1232,
);

/// The op only (after parse, before format). `d2` is the second operand for binary
/// functions; a missing one is a golden-data fault and panics (the harness records it).
pub fn compute<D>(func: Function, x: D, d2: Option<D>, m: DsMode) -> D
where
    D: DecimalArithmetic + DecimalTranscendental + DsOps + Copy,
{
    let bin = || d2.expect("binary function needs two operands");
    match func {
        Function::Sqrt => x.sqrt_strict_with(m),
        Function::Cbrt => x.cbrt_strict_with(m),
        Function::Exp => x.exp_strict_with(m),
        Function::Ln => x.ln_strict_with(m),
        Function::Log2 => x.log2_strict_with(m),
        Function::Log10 => x.log10_strict_with(m),
        Function::Exp2 => x.exp2_strict_with(m),
        Function::Sin => x.sin_strict_with(m),
        Function::Cos => x.cos_strict_with(m),
        Function::Tan => x.tan_strict_with(m),
        Function::Atan => x.atan_strict_with(m),
        Function::Asin => x.asin_strict_with(m),
        Function::Acos => x.acos_strict_with(m),
        Function::Sinh => x.sinh_strict_with(m),
        Function::Cosh => x.cosh_strict_with(m),
        Function::Tanh => x.tanh_strict_with(m),
        Function::Asinh => x.asinh_strict_with(m),
        Function::Acosh => x.acosh_strict_with(m),
        Function::Atanh => x.atanh_strict_with(m),
        Function::Log => x.log_strict_with(bin(), m),
        Function::Atan2 => x.atan2_strict_with(bin(), m),
        Function::Powf => x.powf_strict_with(bin(), m),
        Function::Hypot => x.hypot_strict_with(bin(), m),
        Function::Add => x + bin(),
        Function::Sub => x - bin(),
        Function::Mul => x.ds_mul_with(bin(), m),
        Function::Div => x.ds_div_with(bin(), m),
        Function::Rem => x % bin(),
    }
}

/// Parse → compute → format at one concrete decimal type `D`. The strict op panics on
/// an out-of-range result; the harness catches that as `Computed::Panic` and judges it
/// against the cell's range. Parse of a harness-vetted (representable) input cannot
/// fail; a failure is a golden-data fault and panics with the offending literal.
fn compute_typed<D>(func: Function, inputs: &[String], m: DsMode) -> Computed<String>
where
    D: DecimalArithmetic
        + DecimalTranscendental
        + DsOps
        + core::str::FromStr
        + core::fmt::Display
        + Copy,
{
    let parse =
        |s: &str| s.parse::<D>().unwrap_or_else(|_| panic!("could not parse representable input {s:?}"));
    let x = parse(&inputs[0]);
    let d2 = inputs.get(1).map(|s| parse(s));
    Computed::Value(compute(func, x, d2, m).to_string())
}

/// The exact storage envelope of one concrete decimal type, in decimal — decimal-scaled's
/// own MIN/MAX constants and its fixed fractional depth. No bit-width math leaks into the
/// harness, and the magnitude envelope + fractional depth bound exactly what it can hold,
/// so no separate significant-figure cap is needed.
fn limits_typed<D>(scale: u32) -> Limits
where
    D: DecimalArithmetic + core::fmt::Display,
{
    Limits {
        min_value: Some(<D as DecimalArithmetic>::MIN.to_string()),
        max_value: Some(<D as DecimalArithmetic>::MAX.to_string()),
        max_precision: scale,
        max_significant_digits: None,
    }
}

/// Enumerate the `(width, scale)` cells and fan the two leaf operations out to the
/// concrete decimal type for each. Two cell lists fall out of one fan-out:
/// [`GOLDEN_CELLS`] — the band-edge correctness/history grid (the golden gate and the
/// version-history pins walk this) — and [`CELLS`], its union with the extra
/// lib-compare-only scales (`; compare ...` per tier). [`CELLS`] is what
/// [`dispatch_compute`] covers and what `DsSubject` can run; the lib-compare bench
/// FILTERS it by [`COMPARE_SCALES`], so the comparison's scale choices never enlarge
/// the golden grid — the benches stay decoupled, sharing only this compile-once
/// monomorphisation home. The two dispatch fns are the concrete shim entry points
/// every subject routes through.
macro_rules! cells {
    ($(
        $(#[$cfg:meta])* $D:ident => $w:literal {
            $($s:literal),+ $(,)?
            $(; compare $($cs:literal),+ $(,)?)?
        }
    );+ $(;)?) => {
        /// The band-edge `(width, scale)` correctness grid — the golden gate and the
        /// version-history pins walk exactly this (NOT the lib-compare-only scales).
        pub const GOLDEN_CELLS: &[(u32, u32)] = &[ $( $( ($w, $s), )+ )+ ];

        /// Every COMPILED `(width, scale)` cell: the golden grid PLUS the
        /// lib-compare-only scales (`; compare ...`). What [`dispatch_compute`] covers
        /// and `DsSubject` can run; whether a cell is RUNNABLE in this build is
        /// [`tier_compiled`]. Golden/history filter to [`GOLDEN_CELLS`]; the lib-compare
        /// bench filters to the [`COMPARE_SCALES`] subset, so its scale choices never
        /// enlarge the golden grid (the benches share only this compile-once home).
        pub const CELLS: &[(u32, u32)] =
            &[ $( $( ($w, $s), )+ $( $( ($w, $cs), )+ )? )+ ];

        /// Is this width's decimal tier compiled into the current build?
        pub const fn tier_compiled(width: u32) -> bool {
            match width {
                $( $(#[$cfg])* $w => true, )+
                _ => false,
            }
        }

        /// Per-tier dispatch leaves: one `cfg`-gated child module per tier (the
        /// scale match inside needs no gating — the whole module vanishes with
        /// its feature), and a width match with one arm per tier.
        mod tier_dispatch {
            $(
                $(#[$cfg])*
                #[allow(non_snake_case)]
                pub mod $D {
                    use decimal_scaled_golden::{Computed, Function, Limits};
                    use decimal_scaled::RoundingMode as DsMode;
                    pub fn compute(
                        scale: u32, func: Function, inputs: &[String], m: DsMode,
                    ) -> Computed<String> {
                        match scale {
                            $( $s => crate::compute_typed::<decimal_scaled::$D<$s>>(func, inputs, m), )+
                            $( $( $cs => crate::compute_typed::<decimal_scaled::$D<$cs>>(func, inputs, m), )+ )?
                            _ => panic!("no decimal-scaled cell for (width={}, scale={scale})", $w),
                        }
                    }
                    pub fn limits(scale: u32) -> Limits {
                        match scale {
                            $( $s => crate::limits_typed::<decimal_scaled::$D<$s>>($s), )+
                            $( $( $cs => crate::limits_typed::<decimal_scaled::$D<$cs>>($cs), )+ )?
                            _ => panic!("no decimal-scaled cell for (width={}, scale={scale})", $w),
                        }
                    }
                }
            )+
        }

        /// Parse → compute → format at the concrete decimal type of one band-edge
        /// `(width, scale)` cell — the non-generic shim entry the erased subjects
        /// call. Panics on a cell this build does not compile (the caller filters
        /// on [`tier_compiled`]).
        pub fn dispatch_compute(
            width: u32, scale: u32, func: Function, inputs: &[String], m: DsMode,
        ) -> Computed<String> {
            match width {
                $( $(#[$cfg])* $w => tier_dispatch::$D::compute(scale, func, inputs, m), )+
                _ => panic!("no decimal-scaled cell for (width={width}, scale={scale})"),
            }
        }

        /// The storage envelope of one band-edge `(width, scale)` cell — the
        /// non-generic shim sibling of [`dispatch_compute`].
        pub fn dispatch_limits(width: u32, scale: u32) -> Limits {
            match width {
                $( $(#[$cfg])* $w => tier_dispatch::$D::limits(scale), )+
                _ => panic!("no decimal-scaled cell for (width={width}, scale={scale})"),
            }
        }
    };
}

// The `; compare <scales>` tail on a tier adds the lib-compare-only cells the bench
// needs at that width — exactly the COMPARE_SCALES that are not already a golden cell
// of the tier (and that the tier can hold). Golden/history never walk these; the
// lib-compare bench selects them via COMPARE_SCALES. D18/D38 list none because every
// COMPARE_SCALE they can hold (17; 17/28/37) is already a golden cell.
cells! {
    // D18 — Int<1>, 64-bit storage (always compiled)
    D18 => 18 { 0, 3, 4, 9, 13, 17 };
    // D38 — Int<2>, 128-bit (always compiled)
    D38 => 38 { 0, 2, 6, 9, 10, 12, 17, 18, 19, 28, 37 };
    // D57 — Int<3>, 192-bit
    #[cfg(feature = "d57")]
    D57 => 57 { 0, 14, 20, 28, 30, 42, 56 ; compare 17, 37 };
    // D76 — Int<4>, 256-bit
    #[cfg(feature = "d76")]
    D76 => 76 { 0, 18, 19, 38, 40, 57, 75 ; compare 17, 28, 37 };
    // D115 — Int<6>, 384-bit
    #[cfg(feature = "d115")]
    D115 => 115 { 0, 28, 50, 57, 86, 114 ; compare 17, 37 };
    // D153 — Int<8>, 512-bit
    #[cfg(feature = "d153")]
    D153 => 153 { 0, 38, 76, 114, 152 ; compare 17, 28, 37 };
    // D230 — Int<12>, 768-bit
    #[cfg(feature = "d230")]
    D230 => 230 { 0, 30, 57, 115, 172, 229 ; compare 17, 28, 37, 152 };
    // D307 — Int<16>, 1024-bit (s290: the ln lookup band s285-295)
    #[cfg(feature = "d307")]
    D307 => 307 { 0, 30, 50, 70, 76, 120, 153, 230, 290, 306 ; compare 17, 28, 37, 152 };
    // D462 — Int<24>, 1536-bit
    #[cfg(feature = "d462")]
    D462 => 462 { 0, 30, 100, 115, 180, 231, 346, 461 ; compare 17, 28, 37, 152 };
    // D616 — Int<32>, 2048-bit (s590: the ln lookup band s585-595)
    #[cfg(feature = "d616")]
    D616 => 616 { 0, 30, 130, 154, 240, 308, 462, 590, 615 ; compare 17, 28, 37, 152 };
    // D924 — Int<48>, 3072-bit (s900: the ln lookup band s895-905)
    #[cfg(feature = "d924")]
    D924 => 924 { 0, 30, 180, 231, 350, 462, 693, 900, 923 ; compare 17, 28, 37, 152 };
    // D1232 — Int<64>, 4096-bit (s1200: the ln lookup band s1195-1205)
    #[cfg(feature = "d1232")]
    D1232 => 1232 { 0, 30, 250, 308, 470, 616, 924, 1200, 1231 ; compare 17, 28, 37, 152 };
}

/// The fixed decimal-scaled SCALES the lib-compare bench times each width at — one per
/// peer-precision level: **17** (D18 ceiling / narrow anchor), **28** (rust_decimal),
/// **37** (D38 ceiling = decimal-rs & g_math's 38 significant digits), **152** (D153
/// ceiling ≈ fastnum's 154). The bench selects, per width, those of these the tier can
/// hold; golden/history never see them. Each is present at every holding tier — either
/// already a golden cell or added via that tier's `; compare` tail above.
pub const COMPARE_SCALES: &[u32] = &[17, 28, 37, 152];
