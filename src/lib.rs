//! Const-generic base-10 fixed-point decimal types for deterministic arithmetic.
//!
//! # Overview
//!
//! `decimal-scaled` provides a family of fixed-point decimal types whose stored
//! integer encodes `actual_value * 10^SCALE`. Decimal literals like `1.1`
//! round-trip exactly without any binary approximation, and all core arithmetic
//! is integer-only — identical bit-patterns on every platform.
//!
//! # Primary types
//!
//! Each width has a `D<digits><const SCALE: u32>` const-generic shape with the
//! same method surface; pick the narrowest that fits your range. The number on
//! every `D{N}` type is `MAX_SCALE` — the highest `SCALE` the storage can hold.
//!
//! | Type | Storage | `MAX_SCALE` | Feature gate |
//! |------|---------|-------------|--------------|
//! | [`D9<SCALE>`]    | `i32`     |    9 | always on |
//! | [`D18<SCALE>`]   | `i64`     |   18 | always on |
//! | [`D38<SCALE>`]   | `i128`    |   38 | always on |
//! | [`D56<SCALE>`]   | 192-bit   |   57 | `d56` or `wide` |
//! | [`D76<SCALE>`]   | 256-bit   |   76 | `d76` or `wide` |
//! | [`D114<SCALE>`]  | 384-bit   |  115 | `d114` or `wide` |
//! | [`D153<SCALE>`]  | 512-bit   |  153 | `d153` or `wide` |
//! | [`D230<SCALE>`]  | 768-bit   |  230 | `d230` or `wide` |
//! | [`D307<SCALE>`]  | 1024-bit  |  307 | `d307` or `wide` |
//! | [`D461<SCALE>`]  | 1536-bit  |  462 | `d461` or `x-wide` |
//! | [`D615<SCALE>`]  | 2048-bit  |  616 | `d615` or `x-wide` |
//! | [`D923<SCALE>`]  | 3072-bit  |  924 | `d923` or `xx-wide` |
//! | [`D1231<SCALE>`] | 4096-bit  | 1232 | `d1231` or `xx-wide` |
//!
//! Umbrellas: `wide` enables D56 / D76 / D114 / D153 / D230 / D307;
//! `x-wide` adds D461 + D615; `xx-wide` adds D923 + D1231. Every
//! adjacent pair has lossless `.widen()` / fallible `.narrow()`
//! helpers plus `From` / `TryFrom` impls.
//!
//! Concrete scale aliases such as `D38s12 = D38<12>` are emitted for every
//! supported `SCALE`. `SCALE = MAX_SCALE + 1` is rejected at compile time —
//! `10^(MAX_SCALE+1)` overflows the storage type.
//!
//! The width-generic [`Decimal`] trait carries the surface that is identical
//! across widths (constants, arithmetic operators, sign methods, integer
//! variants, pow / checked / wrapping / saturating / overflowing, float bridge,
//! Euclidean / floor / ceil division, etc.). Use it to write helpers that work
//! across widths; reach for the concrete type for width-specific operations
//! like `rescale::<TARGET>()` whose const-generic parameter cannot live on a
//! trait method.
//!
//! # Equality and hashing
//!
//! Because each logical value has exactly one representation at a fixed scale,
//! `Hash`, `Eq`, `PartialEq`, `PartialOrd`, and `Ord` are all derived from the
//! underlying integer storage. Two `Dxx<S>` values compare equal if and only
//! if their raw bit patterns are identical. This gives predictable behaviour
//! when decimal values are used as `HashMap` keys, unlike variable-scale
//! decimal types where `1.10` and `1.1` may hash differently.
//!
//! # `num-traits` compatibility
//!
//! Every width implements the standard `num-traits` 0.2 surface:
//! `Zero`, `One`, `Num`, `Bounded`, `Signed`, `FromPrimitive`,
//! `ToPrimitive`, and the `Checked{Add,Sub,Mul,Div,Rem,Neg}` family
//! (see [`::num_traits`]). These impls are unconditional (not behind a
//! feature flag) because generic numeric code in the wider ecosystem
//! consumes this surface by default.
//!
//! # `no_std` support
//!
//! The crate compiles with `no_std + alloc` when default features are
//! disabled. `alloc` is required for `Display::to_string` and
//! `FromStr::from_str`. Targets without `alloc` are not supported.
//!
//! # Feature flags
//!
//! - `std` (default): enables the fast implementations of transcendental
//! functions (trigonometry, logarithms, exponentials, square root, cube
//! root, float power) that delegate to platform `f64` intrinsics.
//! - `alloc`: pulled in automatically; required for string formatting and
//! parsing.
//! - `serde`: enables `serde_helpers` for serialisation and deserialisation.
//! - `strict`: enables integer-only implementations of all transcendental
//! functions. When `strict` is active each function that would otherwise
//! route through `f64` is instead implemented using integer-only
//! algorithms. Explicit float-conversion methods (`to_f64`,
//! `from_f64`, etc.) remain available regardless; they are type
//! conversions, not mathematical operations. `strict` does not require
//! `std`; the integer transcendental implementations compile under
//! `no_std + alloc`.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "experimental-floats", feature(f16, f128))]
// ── Clippy allow-list ─────────────────────────────────────────────────
//
// These are pedantic lints whose patterns this crate uses
// intentionally and pervasively. Each is justified inline; allowing
// them at the crate level is preferable to spraying per-site
// `#[allow]` attributes or rewriting against the crate's domain.
#![allow(
    // Decimal width names overlap with type prefixes; the lint adds no
    // signal here.
    clippy::module_name_repetitions,
    // We use unindented Markdown continuation in module docs.
    clippy::doc_lazy_continuation,
    // We routinely place a blank line between a method's `#[cfg]`
    // attribute and its doc/body for readability.
    clippy::empty_line_after_outer_attr,
    // Big-integer arithmetic regularly casts between signed/unsigned
    // and between widths. The wraps / truncations / sign flips are
    // intentional — `unsigned_abs` paths, two's-complement tricks,
    // narrowing the final result back to storage after a widened mul.
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    // We prefer `as` casts over `T::from(x)` in arithmetic-heavy
    // inner loops for readability and to match the surrounding
    // big-integer idiom.
    clippy::cast_lossless,
    // Float bridges (`to_f64`, `to_f32`) are explicitly lossy by
    // contract. The lint is a tautology here.
    clippy::cast_precision_loss,
    // Literals like `1_000_000_000_000` carry the scale visually and
    // are kept unseparated when they encode `10^SCALE`.
    clippy::unreadable_literal,
    // `if cond { panic!(…) }` is the crate's canonical bounds-check
    // shape; `assert!(…)` would lose the dynamic message.
    clippy::manual_assert,
    // `Result<_, ()>` is the only honest error type for `const fn`
    // digit-validity checks where no allocator is available.
    clippy::result_unit_err,
    // `if …; if …` chains read more cleanly than `if … && …` in the
    // const-fn limb-arithmetic helpers.
    clippy::collapsible_if,
    // Big-int / fixed-point inner loops use `i`, `j`, `k`, `n`, `m`
    // as conventional names. Renaming to `outer_index` etc. hurts
    // readability without payoff.
    clippy::similar_names,
    clippy::many_single_char_names,
    // Strict-transcendental kernels exceed 100 lines because they
    // unroll a series-evaluation loop; splitting them just to please
    // the line-count lint would scatter the algorithm.
    clippy::too_many_lines,
    // `#[inline(always)]` is set deliberately on small hot-path
    // helpers (`apply_rounding`, `panic_or_wrap_*`). The lint
    // assumes the inliner knows better; here we override on purpose.
    clippy::inline_always,
    // Strict-vs-fast comparisons in `tests/` deliberately compare
    // raw `f64` results bit-for-bit. The lint can't tell test code
    // from production.
    clippy::float_cmp,
    // Some narrow helpers `let result = …; result + 1` are flagged
    // as let-else candidates; the explicit form is clearer in the
    // big-int helpers.
    clippy::manual_let_else,
    // `format!("{x}") + "y"` is fine when both pieces stay tiny.
    clippy::format_push_string,
    // `if-else-if` chains over disjoint conditions sometimes read
    // more clearly than `match` (especially with `<` / `>=` arms).
    clippy::comparison_chain,
    // Macro-emitted methods that return `Self` are wrapped with
    // `#[must_use]` where it would catch bugs; the lint's
    // recommendation on tiny constructors is noise.
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    // `# Errors` / `# Panics` sections: every public function's
    // behaviour on error / panic is described in its main doc
    // paragraph (and matches the pattern of the std-library
    // primitive it shadows). The lint's per-section requirement
    // adds boilerplate without information.
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    // Doc-comment backticks are added where they matter (type and
    // function names); the lint flags every identifier-looking
    // word, including math symbols and abbreviations.
    clippy::doc_markdown,
)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Re-export `tracing` under the crate so the perf-trace cfg-gated
// `info_span!` calls in macro-emitted modules can reach it via
// `$crate::tracing::…`. Internal-only — gated by the same feature.
#[cfg(feature = "perf-trace")]
#[doc(hidden)]
pub use ::tracing;

mod algos;
mod arithmetic;
#[cfg(feature = "bench-alt")]
mod bench_alt;
#[cfg(feature = "bench-alt")]
#[doc(hidden)]
pub mod __bench_internals {
    #[inline(never)]
    pub fn limbs_mul(a: &[u128], b: &[u128], out: &mut [u128]) {
        crate::wide_int::limbs_mul(a, b, out)
    }
    #[inline(never)]
    pub fn limbs_mul_fast(a: &[u128], b: &[u128], out: &mut [u128]) {
        crate::wide_int::limbs_mul_fast(a, b, out)
    }
    #[inline(never)]
    pub fn mul_slice(a: &[u64], b: &[u64], out: &mut [u64]) {
        crate::wide_int::limbs_mul_u64(a, b, out)
    }
    #[inline(never)]
    pub fn mul_fixed<const L: usize, const D: usize>(
        a: &[u64; L],
        b: &[u64; L],
        out: &mut [u64; D],
    ) {
        crate::wide_int::limbs_mul_u64_fixed::<L, D>(a, b, out)
    }
}
mod consts;
mod consts_wide;
mod core_type;
mod decimal_trait;
mod display;
mod equalities;
mod error;
mod macros;
mod num_traits;
mod log_exp_strict;
mod log_exp_fast;

// `bitwise` and `num_traits_impls` used to live here as test-only
// modules; their tests now run as Cargo integration tests under
// `tests/`. The macro-generated impls themselves are emitted by
// `decl_decimal_bitwise!` / `decl_decimal_num_traits_basics!` from
// `core_type.rs`, alongside every other surface.
mod rescale;
mod rounding;
mod mg_divide;
mod d_w128_kernels;
// `wide_int` is now unconditional. D38's strict transcendentals use
// `Int512` as their guard-digit work integer (replacing the previous
// `d_w128_kernels::Fixed` 256-bit sign-magnitude type), so the wide-
// integer family must be available in every feature configuration —
// not just `feature = "wide"` builds. Compile-time impact is modest:
// ~2k LOC of self-contained limb arithmetic plus the per-width
// `decl_wide_int!` instantiations.
mod wide_int;
mod overflow_variants;
mod policy;
mod powers_strict;
mod powers_fast;

#[cfg(feature = "serde")]
pub mod serde_helpers;
// `trig` is compiled when it has any surface to emit: the integer-only
// `*_strict` methods (present unless `fast`) or the f64-bridge
// methods (present with `std`).
#[cfg(any(not(feature = "fast"), feature = "std"))]
mod trig_strict;
mod trig_fast;
mod transcendental_trait;
mod arithmetic_trait;
mod convert_trait;
#[cfg(feature = "dyn")]
pub mod dyn_decimal;


pub use consts::DecimalConstants;
#[allow(deprecated)]
pub use consts::DecimalConsts;
pub use arithmetic_trait::DecimalArithmetic;
pub use convert_trait::DecimalConvert;
pub use decimal_trait::Decimal;
pub use error::{ConvertError, ParseError};
pub use rounding::RoundingMode;
pub use transcendental_trait::DecimalTranscendental;

#[cfg(feature = "dyn")]
pub use dyn_decimal::{DecimalWidth, DynDecimal, RawStorage};

// D38 — the 128-bit foundation, plus every scale alias D38s0..=D38s38.
pub use core_type::{
    D38, D38s0, D38s1, D38s2, D38s3, D38s4, D38s5, D38s6, D38s7, D38s8, D38s9, D38s10,
    D38s11, D38s12, D38s13, D38s14, D38s15, D38s16, D38s17, D38s18, D38s19, D38s20,
    D38s21, D38s22, D38s23, D38s24, D38s25, D38s26, D38s27, D38s28, D38s29, D38s30,
    D38s31, D38s32, D38s33, D38s34, D38s35, D38s36, D38s37, D38s38,
};

// D9 — 32-bit storage, scale 0..=9.
pub use core_type::{
    D9, D9s0, D9s1, D9s2, D9s3, D9s4, D9s5, D9s6, D9s7, D9s8, D9s9,
};

// D18 — 64-bit storage, scale 0..=18.
pub use core_type::{
    D18, D18s0, D18s1, D18s2, D18s3, D18s4, D18s5, D18s6, D18s7, D18s8, D18s9, D18s10, D18s11,
    D18s12, D18s13, D18s14, D18s15, D18s16, D18s17, D18s18,
};

// D76 — 256-bit storage, behind the `d76` / `wide` features.
#[cfg(any(feature = "d76", feature = "wide"))]
pub use core_type::{
    D76,
    D76s0, D76s1, D76s2, D76s3, D76s4, D76s6, D76s9, D76s12, D76s15,
    D76s18, D76s20, D76s24, D76s28, D76s32, D76s35, D76s38, D76s42,
    D76s48, D76s50, D76s56, D76s64, D76s70, D76s75, D76s76,
};

// The hand-rolled wide-integer types — the storage backend for the
// wide decimal tiers, also useful on their own.
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide"))]
pub use wide_int::{
    Int256, Int512, Int1024, Int2048, Int4096, Uint256, Uint512, Uint1024, Uint2048, Uint4096,
};

// D153 — 512-bit storage, behind the `d153` / `wide` features.
#[cfg(any(feature = "d153", feature = "wide"))]
pub use core_type::{
    D153,
    D153s0, D153s1, D153s2, D153s4, D153s6, D153s9, D153s12, D153s15,
    D153s18, D153s20, D153s24, D153s28, D153s32, D153s35, D153s38,
    D153s50, D153s57, D153s75, D153s76, D153s100, D153s115, D153s140,
    D153s150, D153s152, D153s153,
};

// D307 — 1024-bit storage, behind the `d307` / `wide` features.
#[cfg(any(feature = "d307", feature = "wide"))]
pub use core_type::{
    D307,
    D307s0, D307s1, D307s2, D307s4, D307s6, D307s9, D307s12, D307s15,
    D307s18, D307s20, D307s24, D307s28, D307s32, D307s35, D307s38,
    D307s50, D307s75, D307s100, D307s115, D307s150, D307s153,
    D307s200, D307s230, D307s275, D307s300, D307s306, D307s307,
};

// ─── New half-width and wider tiers ───────────────────────────────────

// D56 — 192-bit storage; half-width between D38 and D76.
#[cfg(any(feature = "d56", feature = "wide"))]
pub use core_type::{
    D56,
    D56s0, D56s1, D56s2, D56s4, D56s6, D56s9, D56s12, D56s18, D56s20, D56s24,
    D56s28, D56s32, D56s38, D56s42, D56s48, D56s52, D56s56, D56s57,
};
#[cfg(any(feature = "d56", feature = "wide"))]
pub use wide_int::{Int192, Uint192};

// D114 — 384-bit; half-width between D76 and D153.
#[cfg(any(feature = "d114", feature = "wide"))]
pub use core_type::{
    D114,
    D114s0, D114s1, D114s4, D114s8, D114s16, D114s24, D114s32, D114s38, D114s50,
    D114s57, D114s64, D114s76, D114s90, D114s100, D114s110, D114s114, D114s115,
};
#[cfg(any(feature = "d114", feature = "wide"))]
pub use wide_int::{Int384, Uint384};

// D230 — 768-bit; half-width between D153 and D307.
#[cfg(any(feature = "d230", feature = "wide"))]
pub use core_type::{
    D230,
    D230s0, D230s1, D230s6, D230s18, D230s38, D230s57, D230s75, D230s100, D230s115,
    D230s140, D230s153, D230s175, D230s200, D230s215, D230s225, D230s229, D230s230,
};
#[cfg(any(feature = "d230", feature = "wide"))]
pub use wide_int::{Int768, Uint768};

// D461 — 1536-bit; half-width between D307 and D615.
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub use core_type::{
    D461,
    D461s0, D461s1, D461s18, D461s38, D461s75, D461s115, D461s153, D461s200, D461s230,
    D461s275, D461s307, D461s350, D461s400, D461s440, D461s460, D461s461, D461s462,
};
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub use wide_int::{Int1536, Uint1536};

// D615 — 2048-bit; new top wide tier. Int2048 / Uint2048 are
// already exported above for x-wide / d307 widening; no re-export
// here.
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub use core_type::{
    D615,
    D615s0, D615s1, D615s38, D615s75, D615s115, D615s153, D615s200, D615s230, D615s275,
    D615s308, D615s380, D615s462, D615s500, D615s555, D615s600, D615s615, D615s616,
};

// D923 — 3072-bit; half-width between D615 and D1231.
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub use core_type::{
    D923,
    D923s0, D923s1, D923s75, D923s153, D923s230, D923s307, D923s400, D923s461, D923s462,
    D923s500, D923s616, D923s700, D923s800, D923s860, D923s900, D923s920, D923s923, D923s924,
};
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub use wide_int::{Int3072, Int6144, Int12288, Uint3072, Uint6144, Uint12288};

// D1231 — 4096-bit; widest tier shipped.
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub use core_type::{
    D1231,
    D1231s0, D1231s1, D1231s75, D1231s153, D1231s230, D1231s307, D1231s461, D1231s616,
    D1231s700, D1231s800, D1231s900, D1231s924, D1231s1000, D1231s1100,
    D1231s1180, D1231s1220, D1231s1230, D1231s1231, D1231s1232,
};
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub use wide_int::{Int8192, Int16384, Uint8192, Uint16384};

// ─── Construction macros (re-exports + per-scale wrappers) ────────────

/// The narrow-tier proc-macros are always available with the
/// `macros` feature; the wide-tier proc-macros are additionally
/// feature-gated to match their target type's availability.
#[cfg(feature = "macros")]
pub use decimal_scaled_macros::{d9, d18, d38};

#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
pub use decimal_scaled_macros::d76;

#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))]
pub use decimal_scaled_macros::d153;

#[cfg(all(feature = "macros", any(feature = "d307", feature = "wide", feature = "x-wide")))]
pub use decimal_scaled_macros::d307;

#[cfg(all(feature = "macros", any(feature = "d56", feature = "wide")))]
pub use decimal_scaled_macros::d56;

#[cfg(all(feature = "macros", any(feature = "d114", feature = "wide")))]
pub use decimal_scaled_macros::d114;

#[cfg(all(feature = "macros", any(feature = "d230", feature = "wide")))]
pub use decimal_scaled_macros::d230;

#[cfg(all(feature = "macros", any(feature = "d461", feature = "x-wide")))]
pub use decimal_scaled_macros::d461;

#[cfg(all(feature = "macros", any(feature = "d615", feature = "x-wide")))]
pub use decimal_scaled_macros::d615;

#[cfg(all(feature = "macros", any(feature = "d923", feature = "xx-wide")))]
pub use decimal_scaled_macros::d923;

#[cfg(all(feature = "macros", any(feature = "d1231", feature = "xx-wide")))]
pub use decimal_scaled_macros::d1231;

// Per-scale wrappers — curated subset of pre-baked
// `<dN>s<SCALE>!` macros that forward to the corresponding
// proc-macro with `scale N` added. Long-tail scales remain
// reachable via the explicit `, scale N` qualifier.
//
// Each alias is a tiny `macro_rules!`. We don't generate them
// through a nested macro because `macro_rules!` doesn't support
// directly emitting another `macro_rules!` without `$$` escapes
// that aren't available in stable Rust; explicit per-line
// declarations keep things debuggable and only cost ~40 lines.

// D9 curated scales.
/// `d9s0!(value)` — equivalent to `d9!(value, scale 0)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d9s0  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d9!($v, scale 0  $(, $($rest)*)?) }; }
/// `d9s2!(value)` — equivalent to `d9!(value, scale 2)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d9s2  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d9!($v, scale 2  $(, $($rest)*)?) }; }
/// `d9s4!(value)` — equivalent to `d9!(value, scale 4)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d9s4  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d9!($v, scale 4  $(, $($rest)*)?) }; }
/// `d9s6!(value)` — equivalent to `d9!(value, scale 6)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d9s6  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d9!($v, scale 6  $(, $($rest)*)?) }; }
/// `d9s9!(value)` — equivalent to `d9!(value, scale 9)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d9s9  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d9!($v, scale 9  $(, $($rest)*)?) }; }

// D18 curated scales.
/// `d18s0!(value)` — equivalent to `d18!(value, scale 0)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d18s0  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 0  $(, $($rest)*)?) }; }
/// `d18s2!(value)` — equivalent to `d18!(value, scale 2)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d18s2  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 2  $(, $($rest)*)?) }; }
/// `d18s4!(value)` — equivalent to `d18!(value, scale 4)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d18s4  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 4  $(, $($rest)*)?) }; }
/// `d18s6!(value)` — equivalent to `d18!(value, scale 6)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d18s6  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 6  $(, $($rest)*)?) }; }
/// `d18s9!(value)` — equivalent to `d18!(value, scale 9)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d18s9  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 9  $(, $($rest)*)?) }; }
/// `d18s12!(value)` — equivalent to `d18!(value, scale 12)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d18s12 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 12 $(, $($rest)*)?) }; }
/// `d18s18!(value)` — equivalent to `d18!(value, scale 18)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d18s18 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 18 $(, $($rest)*)?) }; }

// D38 curated scales.
/// `d38s0!(value)` — equivalent to `d38!(value, scale 0)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s0  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 0  $(, $($rest)*)?) }; }
/// `d38s2!(value)` — equivalent to `d38!(value, scale 2)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s2  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 2  $(, $($rest)*)?) }; }
/// `d38s4!(value)` — equivalent to `d38!(value, scale 4)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s4  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 4  $(, $($rest)*)?) }; }
/// `d38s6!(value)` — equivalent to `d38!(value, scale 6)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s6  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 6  $(, $($rest)*)?) }; }
/// `d38s8!(value)` — equivalent to `d38!(value, scale 8)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s8  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 8  $(, $($rest)*)?) }; }
/// `d38s9!(value)` — equivalent to `d38!(value, scale 9)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s9  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 9  $(, $($rest)*)?) }; }
/// `d38s12!(value)` — equivalent to `d38!(value, scale 12)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s12 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 12 $(, $($rest)*)?) }; }
/// `d38s15!(value)` — equivalent to `d38!(value, scale 15)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s15 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 15 $(, $($rest)*)?) }; }
/// `d38s18!(value)` — equivalent to `d38!(value, scale 18)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s18 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 18 $(, $($rest)*)?) }; }
/// `d38s24!(value)` — equivalent to `d38!(value, scale 24)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s24 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 24 $(, $($rest)*)?) }; }
/// `d38s35!(value)` — equivalent to `d38!(value, scale 35)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s35 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 35 $(, $($rest)*)?) }; }
/// `d38s38!(value)` — equivalent to `d38!(value, scale 38)`.
#[cfg(feature = "macros")] #[macro_export]
macro_rules! d38s38 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 38 $(, $($rest)*)?) }; }

// D76 curated scales.
/// `d76s0!(value)` — equivalent to `d76!(value, scale 0)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s0  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 0  $(, $($rest)*)?) }; }
/// `d76s2!(value)` — equivalent to `d76!(value, scale 2)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s2  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 2  $(, $($rest)*)?) }; }
/// `d76s6!(value)` — equivalent to `d76!(value, scale 6)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s6  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 6  $(, $($rest)*)?) }; }
/// `d76s12!(value)` — equivalent to `d76!(value, scale 12)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s12 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 12 $(, $($rest)*)?) }; }
/// `d76s18!(value)` — equivalent to `d76!(value, scale 18)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s18 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 18 $(, $($rest)*)?) }; }
/// `d76s35!(value)` — equivalent to `d76!(value, scale 35)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s35 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 35 $(, $($rest)*)?) }; }
/// `d76s50!(value)` — equivalent to `d76!(value, scale 50)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s50 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 50 $(, $($rest)*)?) }; }
/// `d76s76!(value)` — equivalent to `d76!(value, scale 76)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))] #[macro_export]
macro_rules! d76s76 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 76 $(, $($rest)*)?) }; }

// D153 curated scales.
/// `d153s0!(value)` — equivalent to `d153!(value, scale 0)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))] #[macro_export]
macro_rules! d153s0   { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 0   $(, $($rest)*)?) }; }
/// `d153s35!(value)` — equivalent to `d153!(value, scale 35)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))] #[macro_export]
macro_rules! d153s35  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 35  $(, $($rest)*)?) }; }
/// `d153s75!(value)` — equivalent to `d153!(value, scale 75)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))] #[macro_export]
macro_rules! d153s75  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 75  $(, $($rest)*)?) }; }
/// `d153s150!(value)` — equivalent to `d153!(value, scale 150)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))] #[macro_export]
macro_rules! d153s150 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 150 $(, $($rest)*)?) }; }
/// `d153s153!(value)` — equivalent to `d153!(value, scale 153)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))] #[macro_export]
macro_rules! d153s153 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 153 $(, $($rest)*)?) }; }

// D307 curated scales.
/// `d307s0!(value)` — equivalent to `d307!(value, scale 0)`.
#[cfg(all(feature = "macros", any(feature = "d307", feature = "wide", feature = "x-wide")))] #[macro_export]
macro_rules! d307s0   { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 0   $(, $($rest)*)?) }; }
/// `d307s35!(value)` — equivalent to `d307!(value, scale 35)`.
#[cfg(all(feature = "macros", any(feature = "d307", feature = "wide", feature = "x-wide")))] #[macro_export]
macro_rules! d307s35  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 35  $(, $($rest)*)?) }; }
/// `d307s150!(value)` — equivalent to `d307!(value, scale 150)`.
#[cfg(all(feature = "macros", any(feature = "d307", feature = "wide", feature = "x-wide")))] #[macro_export]
macro_rules! d307s150 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 150 $(, $($rest)*)?) }; }
/// `d307s300!(value)` — equivalent to `d307!(value, scale 300)`.
#[cfg(all(feature = "macros", any(feature = "d307", feature = "wide", feature = "x-wide")))] #[macro_export]
macro_rules! d307s300 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 300 $(, $($rest)*)?) }; }
/// `d307s307!(value)` — equivalent to `d307!(value, scale 307)`.
#[cfg(all(feature = "macros", any(feature = "d307", feature = "wide", feature = "x-wide")))] #[macro_export]
macro_rules! d307s307 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 307 $(, $($rest)*)?) }; }
