//! Const-generic base-10 fixed-point decimal types for deterministic arithmetic.
//!
//! # Overview
//!
//! `decimal-scaled` provides `D38<const SCALE: u32>`, a fixed-point decimal
//! type backed by `i128`. The stored integer encodes `actual_value * 10^SCALE`,
//! so decimal literals like `1.1` round-trip exactly without any binary
//! approximation. All core arithmetic is integer-only and produces identical
//! bit-patterns on every platform.
//!
//! # Primary types
//!
//! - [`D38<SCALE>`] is the const-generic foundation. Every method is
//! implemented once and is available at any scale.
//! - [`D38s12`] is the concrete alias `D38<12>`. At `SCALE = 12`, one LSB
//! equals `10^-12` model units and the representable range is roughly
//! +/-1.7e14 model units.
//! - Scale aliases [`D38s0`] through [`D38s38`] cover every supported scale.
//! `SCALE = 39` is not supported because `10^39` overflows `i128`.
//!
//! # Equality and hashing
//!
//! Because each logical value has exactly one representation at a fixed scale,
//! `Hash`, `Eq`, `PartialEq`, `PartialOrd`, and `Ord` are all derived from
//! the underlying `i128`. Two `D38<S>` values compare equal if and only if
//! their raw bit patterns are identical. This gives predictable behaviour when
//! `D38` values are used as `HashMap` keys, unlike variable-scale decimal
//! types where `1.10` and `1.1` may hash differently.
//!
//! # `num-traits` compatibility
//!
//! [`D38<SCALE>`] implements the standard `num-traits` 0.2 surface,
//! including [`num_traits::Zero`], [`num_traits::One`], [`num_traits::Num`],
//! [`num_traits::Bounded`], [`num_traits::Signed`],
//! [`num_traits::FromPrimitive`], [`num_traits::ToPrimitive`], and the
//! `Checked{Add,Sub,Mul,Div,Rem,Neg}` family. These impls are unconditional
//! (not behind a feature flag) because generic numeric code in the wider
//! ecosystem consumes this surface by default.
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

mod arithmetic;
#[cfg(feature = "bench-alt")]
mod bench_alt;
mod consts;
mod consts_wide;
mod core_type;
mod decimal_trait;
mod display;
mod equalities;
mod error;
mod macros;
mod fixed_compat;
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


pub use consts::DecimalConsts;
pub use decimal_trait::Decimal;
pub use error::{ConvertError, ParseError};
pub use rounding::RoundingMode;

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
    D76, D76s0, D76s2, D76s6, D76s12, D76s18, D76s35, D76s50, D76s76,
};

// The hand-rolled wide-integer types — the storage backend for the
// wide decimal tiers, also useful on their own.
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide"))]
pub use wide_int::{
    Int256, Int512, Int1024, Int2048, Int4096, Uint256, Uint512, Uint1024, Uint2048, Uint4096,
};

// D153 — 512-bit storage, behind the `d153` / `wide` features.
#[cfg(any(feature = "d153", feature = "wide"))]
pub use core_type::{D153, D153s0, D153s35, D153s75, D153s150, D153s153};

// D307 — 1024-bit storage, behind the `d307` / `wide` features.
#[cfg(any(feature = "d307", feature = "wide"))]
pub use core_type::{D307, D307s0, D307s35, D307s150, D307s300, D307s307};

#[cfg(feature = "macros")]
pub use decimal_scaled_macros::d38;
