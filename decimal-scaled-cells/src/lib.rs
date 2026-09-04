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
//! `cells!`-generated fan-out compiles here, exactly once, into a leaf rlib.
//! The subjects above it (the erased `DsSubject` in decimal-scale-test, the
//! historical adapters in `history`) call these concrete, non-generic entry
//! points and stay light.
//!
//! The fan-out is FOUR leaves per cell, one per stage of the harness's subject
//! contract, so each stage can be called on its own:
//!
//! - [`dispatch_parse`] — literal → [`CellValue`] (a PARSED handle, not text);
//! - [`dispatch_compute_fn`] — `(width, scale)` → the cell's compute leaf,
//!   resolved ONCE and thereafter called on pre-parsed values only;
//! - [`dispatch_format`] — [`CellValue`] → its canonical decimal string;
//! - [`dispatch_limits`] — the cell's storage envelope.
//!
//! **Why the split is four and not one.** `decimal-scaled-golden`'s `Timed`
//! strategy hoists `string_to_value` out of the timed span and calls only the
//! `execute` closure inside it, so a subject whose closure also parses and
//! formats measures parse + op + format while a typed peer measures the op
//! alone. A single `&[String] -> Computed<String>` leaf forces exactly that.
//! [`CellValue`] keeps the subject ERASED (one subject type for all 127 cells,
//! so the runner/collector/validator pipeline still monomorphises once) while
//! putting parse and format where the trait says they belong.
//!
//! [`CellValue`] carries the parsed value as its tier's RAW STORAGE, held in
//! that tier's scale-0 alias: `D<W><S>` and `D<W><0>` are the same storage
//! integer behind a different const, so `to_bits`/`from_bits` move between them
//! as a newtype unwrap/wrap — no conversion, no allocation, `Copy`, stack-only.
//! One enum variant per WIDTH; the scale rides with the subject, exactly as the
//! `(width, scale)` dispatch always has.
//!
//! Pure codegen placement: no algorithm bodies, no per-tier logic — each shim is
//! the same one-line delegate into the existing generic surface it always was
//! (the per-cell listing in the `cells!` macro is instantiation enumeration, not
//! algorithm duplication).

use decimal_scaled::{DecimalArithmetic, DecimalTranscendental, RoundingMode as DsMode};
use decimal_scaled_golden::{Computed, Function, Limits};

// Historical-release subjects for the version-history gates; each pinned
// release compiles only behind its `history-*` feature.
#[cfg(any(feature = "history-044", feature = "history-033"))]
pub mod history;

/// Every function the golden set covers (a missing file just contributes no cases).
pub const FUNCS: &[Function] = &[
    Function::Sqrt, Function::Cbrt, Function::Exp, Function::Ln, Function::Log2, Function::Log10,
    Function::Exp2, Function::Expm1, Function::Log1p, Function::Sin, Function::Cos, Function::Tan,
    Function::Atan, Function::Asin, Function::Acos, Function::Sinh, Function::Cosh, Function::Tanh,
    Function::Asinh, Function::Acosh, Function::Atanh, Function::Log, Function::Atan2,
    Function::Powf, Function::Hypot, Function::Add, Function::Sub, Function::Mul, Function::Div,
    Function::Rem,
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
        Function::Sqrt => x.sqrt_with(m),
        Function::Cbrt => x.cbrt_with(m),
        Function::Exp => x.exp_with(m),
        Function::Ln => x.ln_with(m),
        Function::Log2 => x.log2_with(m),
        Function::Log10 => x.log10_with(m),
        Function::Exp2 => x.exp2_with(m),
        Function::Expm1 => x.expm1_with(m),
        Function::Log1p => x.log1p_with(m),
        Function::Sin => x.sin_with(m),
        Function::Cos => x.cos_with(m),
        Function::Tan => x.tan_with(m),
        Function::Atan => x.atan_with(m),
        Function::Asin => x.asin_with(m),
        Function::Acos => x.acos_with(m),
        Function::Sinh => x.sinh_with(m),
        Function::Cosh => x.cosh_with(m),
        Function::Tanh => x.tanh_with(m),
        Function::Asinh => x.asinh_with(m),
        Function::Acosh => x.acosh_with(m),
        Function::Atanh => x.atanh_with(m),
        Function::Log => x.log_with(bin(), m),
        Function::Atan2 => x.atan2_with(bin(), m),
        Function::Powf => x.powf_with(bin(), m),
        Function::Hypot => x.hypot_with(bin(), m),
        Function::Add => x + bin(),
        Function::Sub => x - bin(),
        Function::Mul => x.ds_mul_with(bin(), m),
        Function::Div => x.ds_div_with(bin(), m),
        Function::Rem => x % bin(),
    }
}

/// Parse one input literal at one concrete decimal type `D`. Parse of a
/// harness-vetted (representable) input cannot fail — the runner's input filter
/// (`limits`) runs before any execution — so a failure here is a golden-data fault
/// and panics with the offending literal, which the harness catches and reports.
fn parse_typed<D>(s: &str) -> D
where
    D: core::str::FromStr,
{
    s.parse::<D>()
        .unwrap_or_else(|_| panic!("could not parse representable input {s:?}"))
}

/// The compute leaf of one `(width, scale)` cell: the op ONLY, over pre-parsed
/// [`CellValue`]s. [`dispatch_compute_fn`] resolves the cell once, so the
/// `(width, scale)` match is paid at setup and never inside a timed span — what
/// remains is the same `match func` a typed peer adapter's closure runs.
///
/// The strict op panics on an out-of-range result; the harness catches that as
/// `Computed::Panic` and judges it against the cell's range.
pub type CellCompute = fn(Function, &[CellValue], DsMode) -> Computed<CellValue>;

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

/// Enumerate the `(width, scale)` cells and fan the leaf operations out to the
/// concrete decimal type for each. Two cell lists fall out of one fan-out:
/// [`GOLDEN_CELLS`] — the band-edge correctness/history grid (the golden gate and the
/// version-history pins walk this) — and [`CELLS`], its union with the extra
/// lib-compare-only scales (`; compare ...` per tier). [`CELLS`] is what
/// [`dispatch_compute_fn`] covers and what `DsSubject` can run; the lib-compare bench
/// FILTERS it by [`COMPARE_SCALES`], so the comparison's scale choices never enlarge
/// the golden grid — the benches stay decoupled, sharing only this compile-once
/// monomorphisation home. The four dispatch fns are the concrete shim entry points
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
        /// lib-compare-only scales (`; compare ...`). What [`dispatch_compute_fn`]
        /// covers and `DsSubject` can run; whether a cell is RUNNABLE in this build is
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

        /// One PARSED decimal-scaled cell value, erased over the width tiers.
        ///
        /// The payload is the tier's raw storage integer, carried in that tier's
        /// scale-0 alias: `D<W><S>` and `D<W><0>` are the same `#[repr(transparent)]`
        /// storage behind a different const, so `to_bits`/`from_bits` move between
        /// them as a newtype unwrap/wrap. `Copy`, stack-only, no allocation, and the
        /// scale rides with the subject exactly as the `(width, scale)` dispatch
        /// always has — so the whole harness pipeline still sees ONE `Value` type
        /// for all of [`CELLS`], while parse and format sit outside the op.
        #[derive(Clone, Copy, Debug)]
        pub enum CellValue {
            $(
                $(#[$cfg])*
                /// A parsed value of this width tier, as its raw storage.
                $D(decimal_scaled::$D<0>),
            )+
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
                    use crate::CellValue;

                    /// This tier's scale-erased carrier: the same storage integer,
                    /// named at scale 0.
                    type Carrier = decimal_scaled::$D<0>;

                    /// Cell type → carrier. `to_bits`/`from_bits` are the documented
                    /// newtype unwrap/wrap, so this is free at every width.
                    #[inline]
                    fn carry<const S: u32>(v: decimal_scaled::$D<S>) -> Carrier {
                        <Carrier>::from_bits(v.to_bits())
                    }

                    /// Carrier → cell type at scale `S`; the inverse of [`carry`].
                    #[inline]
                    fn at<const S: u32>(c: Carrier) -> decimal_scaled::$D<S> {
                        <decimal_scaled::$D<S>>::from_bits(c.to_bits())
                    }

                    /// This tier's carrier out of an erased value. A value of another
                    /// tier cannot occur — the subject parses and computes at one
                    /// cell — so it is a harness fault, not a subject outcome.
                    #[inline]
                    fn carrier(v: &CellValue) -> Carrier {
                        match v {
                            CellValue::$D(c) => *c,
                            #[allow(unreachable_patterns)]
                            _ => panic!("decimal-scaled cell value is not a width-{} value", $w),
                        }
                    }

                    /// The compute leaf of ONE cell: unwrap the carriers, run the op,
                    /// re-wrap. No parse, no format — this is the whole timed body.
                    fn cell<const S: u32>(
                        func: Function, inputs: &[CellValue], m: DsMode,
                    ) -> Computed<CellValue> {
                        let x = at::<S>(carrier(&inputs[0]));
                        let d2 = inputs.get(1).map(|v| at::<S>(carrier(v)));
                        Computed::Value(CellValue::$D(carry::<S>(crate::compute(func, x, d2, m))))
                    }

                    pub fn compute_fn(scale: u32) -> crate::CellCompute {
                        match scale {
                            $( $s => cell::<{ $s }> as crate::CellCompute, )+
                            $( $( $cs => cell::<{ $cs }> as crate::CellCompute, )+ )?
                            _ => panic!("no decimal-scaled cell for (width={}, scale={scale})", $w),
                        }
                    }

                    pub fn parse(scale: u32, s: &str) -> CellValue {
                        CellValue::$D(match scale {
                            $( $s => carry::<{ $s }>(crate::parse_typed::<decimal_scaled::$D<$s>>(s)), )+
                            $( $( $cs => carry::<{ $cs }>(crate::parse_typed::<decimal_scaled::$D<$cs>>(s)), )+ )?
                            _ => panic!("no decimal-scaled cell for (width={}, scale={scale})", $w),
                        })
                    }

                    pub fn format(scale: u32, v: &CellValue) -> String {
                        let c = carrier(v);
                        match scale {
                            $( $s => at::<{ $s }>(c).to_string(), )+
                            $( $( $cs => at::<{ $cs }>(c).to_string(), )+ )?
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

        /// Parse one input literal at the concrete decimal type of one band-edge
        /// `(width, scale)` cell — the non-generic shim entry the erased subjects'
        /// `string_to_value` calls. Panics on a cell this build does not compile
        /// (the caller filters on [`tier_compiled`]).
        pub fn dispatch_parse(width: u32, scale: u32, s: &str) -> CellValue {
            match width {
                $( $(#[$cfg])* $w => tier_dispatch::$D::parse(scale, s), )+
                _ => panic!("no decimal-scaled cell for (width={width}, scale={scale})"),
            }
        }

        /// Resolve one band-edge `(width, scale)` cell to its compute leaf — the op
        /// ONLY, over pre-parsed [`CellValue`]s. Called once per subject execution
        /// (never inside a timed span), so the cell match is not charged to the
        /// operation. Panics on a cell this build does not compile — as
        /// [`dispatch_limits`], which the runner calls first for every input,
        /// already does.
        pub fn dispatch_compute_fn(width: u32, scale: u32) -> CellCompute {
            match width {
                $( $(#[$cfg])* $w => tier_dispatch::$D::compute_fn(scale), )+
                _ => panic!("no decimal-scaled cell for (width={width}, scale={scale})"),
            }
        }

        /// Format a [`CellValue`] back to canonical decimal text at its cell — the
        /// erased subjects' `value_to_string`, and the other half of the pair that
        /// keeps conversion out of the op.
        pub fn dispatch_format(width: u32, scale: u32, v: &CellValue) -> String {
            match width {
                $( $(#[$cfg])* $w => tier_dispatch::$D::format(scale, v), )+
                _ => panic!("no decimal-scaled cell for (width={width}, scale={scale})"),
            }
        }

        /// The storage envelope of one band-edge `(width, scale)` cell — the
        /// non-generic shim sibling of [`dispatch_compute_fn`].
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
