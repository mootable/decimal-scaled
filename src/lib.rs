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
//! | [`D57<SCALE>`]   | 192-bit   |   57 | `d57` or `wide` |
//! | [`D76<SCALE>`]   | 256-bit   |   76 | `d76` or `wide` |
//! | [`D115<SCALE>`]  | 384-bit   |  115 | `d115` or `wide` |
//! | [`D153<SCALE>`]  | 512-bit   |  153 | `d153` or `wide` |
//! | [`D230<SCALE>`]  | 768-bit   |  230 | `d230` or `wide` |
//! | [`D307<SCALE>`]  | 1024-bit  |  307 | `d307` or `wide` |
//! | [`D462<SCALE>`]  | 1536-bit  |  462 | `d462` or `x-wide` |
//! | [`D616<SCALE>`]  | 2048-bit  |  616 | `d616` or `x-wide` |
//! | [`D924<SCALE>`]  | 3072-bit  |  924 | `d924` or `xx-wide` |
//! | [`D1232<SCALE>`] | 4096-bit  | 1232 | `d1232` or `xx-wide` |
//!
//! Umbrellas: `wide` enables D57 / D76 / D115 / D153 / D230 / D307;
//! `x-wide` adds D462 + D616; `xx-wide` adds D924 + D1232. Every
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
#![cfg_attr(feature = "cross-scale-ops", feature(generic_const_exprs))]
#![cfg_attr(feature = "cross-scale-ops", allow(incomplete_features))]
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
mod identity;
mod support;
mod types;
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
    #[inline(never)]
    pub fn mul_u64_into<const L: usize, const LP1: usize>(
        a: &[u64; L],
        n: u64,
        out: &mut [u64; LP1],
    ) {
        crate::wide_int::limbs_mul_u64_into::<L, LP1>(a, n, out)
    }

    // Newton-reciprocal divide research kernel — wrapped via concrete
    // shims so the bench harness gets head-to-head comparisons against
    // [`crate::algos::mg_divide::div_wide_pow10_chain_with`] without
    // exposing trait machinery.
    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
    pub mod newton_vs_mg {
        use crate::algos::newton_reciprocal::NewtonReciprocal as NR;
        pub struct NewtonReciprocal(pub(crate) NR);
        impl NewtonReciprocal {
            #[inline(never)]
            pub fn precompute(scale: u32, width_limbs: usize) -> Self {
                Self(NR::precompute(scale, width_limbs))
            }
        }

        macro_rules! shim {
            ($pub_name:ident, $width:ty, $feat:literal) => {
                #[cfg(any(feature = $feat))]
                pub mod $pub_name {
                    use super::NewtonReciprocal;
                    use crate::RoundingMode;
                    use $width as W;

                    /// Storage type for this tier — opaque to the bench.
                    #[derive(Clone, Copy)]
                    pub struct Storage(W);

                    /// Build a representative non-zero numerator from a top-limb position.
                    #[inline(never)]
                    pub fn build_numerator(top_limb_idx: usize) -> Storage {
                        use crate::wide_int::WideInt;
                        let mut mag = [0u128; 64];
                        mag[top_limb_idx] = 1u128 << 32;
                        mag[1] = 0xdeadbeef_cafef00d_u128;
                        Storage(W::from_mag_sign_u128(&mag, false))
                    }

                    #[inline(never)]
                    pub fn mg_chain(n: Storage, scale: u32) -> Storage {
                        Storage(crate::algos::mg_divide::div_wide_pow10_chain_with::<
                            W,
                            { <W as crate::wide_int::WideInt>::U128_LIMBS },
                        >(
                            n.0,
                            scale,
                            RoundingMode::HalfToEven,
                        ))
                    }

                    #[inline(never)]
                    pub fn mg_single(n: Storage, scale: u32) -> Storage {
                        Storage(crate::algos::mg_divide::div_wide_pow10_with::<
                            W,
                            { <W as crate::wide_int::WideInt>::U128_LIMBS },
                        >(
                            n.0,
                            scale,
                            RoundingMode::HalfToEven,
                        ))
                    }

                    #[inline(never)]
                    pub fn newton(
                        n: Storage,
                        scale: u32,
                        table: &NewtonReciprocal,
                    ) -> Storage {
                        Storage(crate::algos::newton_reciprocal::div_wide_pow10_newton_with::<W>(
                            n.0,
                            scale,
                            RoundingMode::HalfToEven,
                            &table.0,
                        ))
                    }
                }
            };
        }
        shim!(d307, crate::wide_int::I1024, "x-wide");
        shim!(d616, crate::wide_int::I2048, "x-wide");
        shim!(d924, crate::wide_int::I3072, "xx-wide");
        shim!(d1232, crate::wide_int::I4096, "xx-wide");
    }
}
mod macros;

#[cfg(feature = "cross-scale-ops")]
pub mod cross_scale;

/// Nightly-gated auto-inferred cross-scale operations. See
/// [`mod@cross_scale`] for details.
#[cfg(feature = "cross-scale-ops")]
pub use crate::cross_scale as cross;

// `bitwise` and `num_traits_impls` used to live here as test-only
// modules; their tests now run as Cargo integration tests under
// `tests/`. The macro-generated impls themselves are emitted by
// `decl_decimal_bitwise!` / `decl_decimal_num_traits_basics!` from
// `types/widths.rs`, alongside every other surface.
//
// `wide_int` is unconditional. D38's strict transcendentals use
// `Int512` as their guard-digit work integer (replacing the previous
// `algos::fixed_d38::Fixed` 256-bit sign-magnitude type), so the wide-
// integer family must be available in every feature configuration —
// not just `feature = "wide"` builds. Compile-time impact is modest:
// ~2k LOC of self-contained limb arithmetic plus the per-width
// `decl_wide_int!` instantiations.
mod wide_int;
// 0.5.0 const-generic integer layer (`Int<N>`/`Uint<N>`), introduced
// alongside `wide_int`'s named `IntXXXX` types during the unification.
// See research plan; named types become `pub type` aliases over these.
mod int;
mod policy;

#[cfg(feature = "serde")]
pub use crate::support::serde_helpers;


pub use crate::types::consts::DecimalConstants;
pub use crate::types::traits::DecimalArithmetic;
pub use crate::types::traits::DecimalConvert;
pub use crate::types::unified::D;
pub use crate::types::traits::Decimal;
pub use crate::support::error::{ConvertError, ParseError};
pub use crate::support::rounding::RoundingMode;
pub use crate::types::traits::DecimalTranscendental;
pub use crate::types::traits::WidthLE;

#[cfg(feature = "dyn")]
pub use crate::types::traits::dyn_decimal::{DecimalWidth, DynDecimal, RawStorage};

// D38 — the 128-bit foundation, plus every scale alias D38s0..=D38s37
// (v0.4.0 cap: MAX_SCALE = name - 1).
pub use crate::types::widths::{
    D38, D38s0, D38s1, D38s2, D38s3, D38s4, D38s5, D38s6, D38s7, D38s8, D38s9, D38s10,
    D38s11, D38s12, D38s13, D38s14, D38s15, D38s16, D38s17, D38s18, D38s19, D38s20,
    D38s21, D38s22, D38s23, D38s24, D38s25, D38s26, D38s27, D38s28, D38s29, D38s30,
    D38s31, D38s32, D38s33, D38s34, D38s35, D38s36, D38s37,
};

// D9 — 32-bit storage, scale 0..=9.
pub use crate::types::widths::{
    D9, D9s0, D9s1, D9s2, D9s3, D9s4, D9s5, D9s6, D9s7, D9s8,
};

// D18 — 64-bit storage, scale 0..=18.
pub use crate::types::widths::{
    D18, D18s0, D18s1, D18s2, D18s3, D18s4, D18s5, D18s6, D18s7, D18s8, D18s9, D18s10, D18s11,
    D18s12, D18s13, D18s14, D18s15, D18s16, D18s17,
};

// D76 — 256-bit storage, behind the `d76` / `wide` features.
#[cfg(any(feature = "d76", feature = "wide"))]
pub use crate::types::widths::{
    D76,
    D76s0, D76s1, D76s2, D76s3, D76s4, D76s6, D76s9, D76s12, D76s15,
    D76s18, D76s20, D76s24, D76s28, D76s32, D76s35, D76s38, D76s42,
    D76s48, D76s50, D76s56, D76s64, D76s70, D76s75,
};

// The hand-rolled wide-integer types — the storage backend for the
// wide decimal tiers, also useful on their own.
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide"))]
pub use wide_int::{
    Int256, Int512, Int1024, Int2048, Int4096, Uint256, Uint512, Uint1024, Uint2048, Uint4096,
};

// D153 — 512-bit storage, behind the `d153` / `wide` features.
#[cfg(any(feature = "d153", feature = "wide"))]
pub use crate::types::widths::{
    D153,
    D153s0, D153s1, D153s2, D153s4, D153s6, D153s9, D153s12, D153s15,
    D153s18, D153s20, D153s24, D153s28, D153s32, D153s35, D153s38,
    D153s50, D153s57, D153s75, D153s76, D153s100, D153s115, D153s140,
    D153s150, D153s152,
};

// D307 — 1024-bit storage, behind the `d307` / `wide` features.
#[cfg(any(feature = "d307", feature = "wide"))]
pub use crate::types::widths::{
    D307,
    D307s0, D307s1, D307s2, D307s4, D307s6, D307s9, D307s12, D307s15,
    D307s18, D307s20, D307s24, D307s28, D307s32, D307s35, D307s38,
    D307s50, D307s75, D307s100, D307s115, D307s150, D307s153,
    D307s200, D307s230, D307s275, D307s300, D307s306,
};

// ─── New half-width and wider tiers ───────────────────────────────────

// D57 — 192-bit storage; half-width between D38 and D76.
#[cfg(any(feature = "d57", feature = "wide"))]
pub use crate::types::widths::{
    D57,
    D57s0, D57s1, D57s2, D57s4, D57s6, D57s9, D57s12, D57s18, D57s20, D57s24,
    D57s28, D57s32, D57s38, D57s42, D57s48, D57s52, D57s56,
};
#[cfg(any(feature = "d57", feature = "wide"))]
pub use wide_int::{Int192, Uint192};

// D115 — 384-bit; half-width between D76 and D153.
#[cfg(any(feature = "d115", feature = "wide"))]
pub use crate::types::widths::{
    D115,
    D115s0, D115s1, D115s4, D115s8, D115s16, D115s24, D115s32, D115s38, D115s50,
    D115s57, D115s64, D115s76, D115s90, D115s100, D115s110, D115s114,
};
#[cfg(any(feature = "d115", feature = "wide"))]
pub use wide_int::{Int384, Uint384};

// D230 — 768-bit; half-width between D153 and D307.
#[cfg(any(feature = "d230", feature = "wide"))]
pub use crate::types::widths::{
    D230,
    D230s0, D230s1, D230s6, D230s18, D230s38, D230s57, D230s75, D230s100, D230s115,
    D230s140, D230s153, D230s175, D230s200, D230s215, D230s225, D230s229,
};
#[cfg(any(feature = "d230", feature = "wide"))]
pub use wide_int::{Int768, Uint768};

// D462 — 1536-bit; half-width between D307 and D616.
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub use crate::types::widths::{
    D462,
    D462s0, D462s1, D462s18, D462s38, D462s75, D462s115, D462s153, D462s200, D462s230,
    D462s275, D462s307, D462s350, D462s400, D462s440, D462s460, D462s461,
};
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub use wide_int::{Int1536, Uint1536};

// D616 — 2048-bit; new top wide tier. Int2048 / Uint2048 are
// already exported above for x-wide / d307 widening; no re-export
// here.
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub use crate::types::widths::{
    D616,
    D616s0, D616s1, D616s38, D616s75, D616s115, D616s153, D616s200, D616s230, D616s275,
    D616s308, D616s380, D616s462, D616s500, D616s555, D616s600, D616s615,
};

// D924 — 3072-bit; half-width between D616 and D1232.
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub use crate::types::widths::{
    D924,
    D924s0, D924s1, D924s75, D924s153, D924s230, D924s307, D924s400, D924s461, D924s462,
    D924s500, D924s616, D924s700, D924s800, D924s860, D924s900, D924s920, D924s923,
};
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub use wide_int::{Int3072, Int6144, Int12288, Uint3072, Uint6144, Uint12288};

// D1232 — 4096-bit; widest tier shipped.
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub use crate::types::widths::{
    D1232,
    D1232s0, D1232s1, D1232s75, D1232s153, D1232s230, D1232s307, D1232s461, D1232s616,
    D1232s700, D1232s800, D1232s900, D1232s924, D1232s1000, D1232s1100,
    D1232s1180, D1232s1220, D1232s1230, D1232s1231,
};
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
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

#[cfg(all(feature = "macros", any(feature = "d57", feature = "wide")))]
pub use decimal_scaled_macros::d57;

#[cfg(all(feature = "macros", any(feature = "d115", feature = "wide")))]
pub use decimal_scaled_macros::d115;

#[cfg(all(feature = "macros", any(feature = "d230", feature = "wide")))]
pub use decimal_scaled_macros::d230;

#[cfg(all(feature = "macros", any(feature = "d462", feature = "x-wide")))]
pub use decimal_scaled_macros::d462;

#[cfg(all(feature = "macros", any(feature = "d616", feature = "x-wide")))]
pub use decimal_scaled_macros::d616;

#[cfg(all(feature = "macros", any(feature = "d924", feature = "xx-wide")))]
pub use decimal_scaled_macros::d924;

#[cfg(all(feature = "macros", any(feature = "d1232", feature = "xx-wide")))]
pub use decimal_scaled_macros::d1232;

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
