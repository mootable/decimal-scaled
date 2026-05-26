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
//! every `D{N}` type is its nominal precision in decimal digits; the highest
//! `SCALE` the type accepts is `MAX_SCALE = N - 1`, leaving at least one
//! integer digit of headroom at every legal scale.
//!
//! | Type | Storage | `MAX_SCALE` | Feature gate |
//! |------|---------|-------------|--------------|
//! | [`D18<SCALE>`]   | `i64`     |   17 | always on |
//! | [`D38<SCALE>`]   | `i128`    |   37 | always on |
//! | [`D57<SCALE>`]   | 192-bit   |   56 | `d57` or `wide` |
//! | [`D76<SCALE>`]   | 256-bit   |   75 | `d76` or `wide` |
//! | [`D115<SCALE>`]  | 384-bit   |  114 | `d115` or `wide` |
//! | [`D153<SCALE>`]  | 512-bit   |  152 | `d153` or `wide` |
//! | [`D230<SCALE>`]  | 768-bit   |  229 | `d230` or `wide` |
//! | [`D307<SCALE>`]  | 1024-bit  |  306 | `d307` or `wide` |
//! | [`D462<SCALE>`]  | 1536-bit  |  461 | `d462` or `x-wide` |
//! | [`D616<SCALE>`]  | 2048-bit  |  615 | `d616` or `x-wide` |
//! | [`D924<SCALE>`]  | 3072-bit  |  923 | `d924` or `xx-wide` |
//! | [`D1232<SCALE>`] | 4096-bit  | 1231 | `d1232` or `xx-wide` |
//!
//! Umbrellas: `wide` enables D57 / D76 / D115 / D153 / D230 / D307;
//! `x-wide` adds D462 + D616; `xx-wide` adds D924 + D1232. Every
//! adjacent pair has lossless `.widen()` / fallible `.narrow()`
//! helpers plus `From` / `TryFrom` impls.
//!
//! Concrete scale aliases such as `D38s12 = D38<12>` are emitted for every
//! supported `SCALE`. `SCALE = MAX_SCALE + 1` (i.e. `SCALE = N` for `D{N}`) is
//! rejected at compile time: the v0.4.0 scale cap fixes `MAX_SCALE = N - 1` so
//! every legal scale retains at least one integer digit of headroom.
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
#![cfg_attr(
    any(feature = "cross-scale-ops", feature = "exact-scratch-nightly"),
    feature(generic_const_exprs)
)]
#![cfg_attr(
    any(feature = "cross-scale-ops", feature = "exact-scratch-nightly"),
    allow(incomplete_features)
)]
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

mod algo_x_support;
mod algos;
mod identity;
mod support;
mod types;
#[cfg(feature = "bench-alt")]
#[doc(hidden)]
pub mod __bench_internals {
    #[inline(never)]
    pub fn mul_slice(a: &[u64], b: &[u64], out: &mut [u64]) {
        crate::int::algos::mul::mul_schoolbook::mul_schoolbook(a, b, out)
    }
    /// Truncated-low fixed-width schoolbook multiply (base-2^64),
    /// `out = (a * b) mod 2^(64*N)`. The kernel the wide-tier exp/powf
    /// Taylor work-multiply (`Int<N>::wrapping_mul`) routes through.
    /// Exposed for the `mul_low_u128_ab` pilot microbench.
    #[inline(never)]
    pub fn mul_low_u64<const N: usize>(a: &[u64; N], b: &[u64; N], out: &mut [u64; N]) {
        crate::int::algos::mul::mul_schoolbook::mul_low_limb::<N, u64>(a, b, out)
    }
    /// u128-limb-packed truncated-low schoolbook multiply (even `N` only):
    /// bit-identical low `N` limbs to [`mul_low_u64`], computed in `N/2`
    /// base-2^128 limbs — the `L = u128` monomorphisation of the one
    /// `mul_low_limb` kernel. Exposed for the `mul_low_u128_ab` `LimbSize`
    /// microbench (u64 vs u128 of the same generic kernel).
    #[inline(never)]
    pub fn mul_low_u128<const N: usize>(a: &[u64; N], b: &[u64; N], out: &mut [u64; N]) {
        crate::int::algos::mul::mul_schoolbook::mul_low_limb::<N, u128>(a, b, out)
    }
    /// Per-`N` monomorphic wrappers over the ONE generic full-product
    /// schoolbook kernel `mul_full_limb::<N, L>` (the production
    /// `int::policy::mul` Schoolbook arm). Generated by the `mul_full_wrappers!`
    /// macro below so each width is a concrete monomorphisation — `Int<N>:
    /// ComputeInt` is satisfied at the concrete `N`, so the private bound never
    /// leaks through a generic `pub` signature (a generic export fails to
    /// compile). Thin wrappers (no per-tier algorithm copy): the `u64` arm is
    /// `L = u64` (base-2^64), the `u128` arm is `L = u128` (packed, even `N`
    /// only). Exposed for the `mul_full_ab` full-product LimbSize +
    /// Karatsuba-crossover microbench.
    macro_rules! mul_full_wrappers {
        // u64-only (odd N — u128 packing requires even N)
        ($u64name:ident, $n:literal) => {
            #[inline(never)]
            pub fn $u64name(a: &[u64; $n], b: &[u64; $n], out: &mut [u64]) {
                crate::int::algos::mul::mul_schoolbook::mul_full_limb::<$n, u64>(a, b, out)
            }
        };
        // both u64 + u128 (even N)
        ($u64name:ident, $u128name:ident, $n:literal) => {
            #[inline(never)]
            pub fn $u64name(a: &[u64; $n], b: &[u64; $n], out: &mut [u64]) {
                crate::int::algos::mul::mul_schoolbook::mul_full_limb::<$n, u64>(a, b, out)
            }
            #[inline(never)]
            pub fn $u128name(a: &[u64; $n], b: &[u64; $n], out: &mut [u64]) {
                crate::int::algos::mul::mul_schoolbook::mul_full_limb::<$n, u128>(a, b, out)
            }
        };
    }
    mul_full_wrappers!(mul_full_u64_2, mul_full_u128_2, 2);
    mul_full_wrappers!(mul_full_u64_3, 3);
    mul_full_wrappers!(mul_full_u64_4, mul_full_u128_4, 4);
    mul_full_wrappers!(mul_full_u64_6, mul_full_u128_6, 6);
    mul_full_wrappers!(mul_full_u64_8, mul_full_u128_8, 8);
    mul_full_wrappers!(mul_full_u64_12, mul_full_u128_12, 12);
    mul_full_wrappers!(mul_full_u64_16, mul_full_u128_16, 16);
    mul_full_wrappers!(mul_full_u64_24, mul_full_u128_24, 24);
    mul_full_wrappers!(mul_full_u64_32, mul_full_u128_32, 32);
    mul_full_wrappers!(mul_full_u64_48, mul_full_u128_48, 48);
    mul_full_wrappers!(mul_full_u64_64, mul_full_u128_64, 64);
    /// `out = (x²) mod 2^(64*N)` via the u64-limb truncated-low symmetric
    /// square — the kernel the wide-tier exp/powf Smith squaring loop
    /// (`BigInt::wrapping_sqr_low_u128`) routes through at `L = u64`. Exposed
    /// for the `sqr_low_u128_ab` pilot microbench.
    #[inline(never)]
    pub fn sqr_low_u64<const N: usize>(x: &[u64; N], out: &mut [u64; N]) {
        crate::int::algos::sqr::sqr_low_limb::sqr_low_limb::<N, u64>(x, out)
    }
    /// u128-limb-packed truncated-low symmetric square (even `N` only):
    /// bit-identical low `N` limbs to [`sqr_low_u64`], computed in `N/2`
    /// base-2^128 limbs — the `L = u128` monomorphisation of the one
    /// `sqr_low_limb` kernel. Exposed for the `sqr_low_u128_ab` `LimbSize`
    /// microbench (u64 vs u128 of the same generic kernel).
    #[inline(never)]
    pub fn sqr_low_u128<const N: usize>(x: &[u64; N], out: &mut [u64; N]) {
        crate::int::algos::sqr::sqr_low_limb::sqr_low_limb::<N, u128>(x, out)
    }
    #[inline(never)]
    pub fn mul_fixed<const L: usize, const D: usize>(
        a: &[u64; L],
        b: &[u64; L],
        out: &mut [u64; D],
    ) {
        crate::int::algos::mul::mul_schoolbook::mul_schoolbook_fixed::<L, D>(a, b, out)
    }
    /// Non-allocating Karatsuba multiply forced to recurse at the given
    /// `threshold` (rather than the parked production
    /// `KARATSUBA_THRESHOLD_U64`), so the crossover sweep can measure the
    /// kernel at sub-threshold widths. `threshold >= 4` (the recursion's
    /// termination floor). `out` is zeroed by the callee.
    #[inline(never)]
    pub fn mul_karatsuba_forced(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
        crate::int::algos::mul::mul_karatsuba::mul_karatsuba_forced(a, b, out, threshold)
    }
    /// Division engine candidates exposed for the `div_kernel_ab`
    /// microbench (the dispatch-seam A/B that recovers the WIDE integer
    /// division regression). Both take little-endian u64 magnitude limb
    /// slices; `quot` / `rem` are written by the engine.
    #[inline(never)]
    pub fn div_knuth_slice(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
        crate::int::algos::div::div_knuth::div_knuth(num, den, quot, rem)
    }
    #[inline(never)]
    pub fn div_dispatch_slice(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
        crate::int::policy::div_rem::dispatch(num, den, quot, rem)
    }
    /// Base-2¹²⁸ (u128-limb) Knuth candidate for `div_kernel_ab` — the
    /// divide side of the `LimbSize` axis, PARKED (not wired into any
    /// policy). Bit-identical to [`div_knuth_slice`]; A/B measures whether
    /// the aligned u128 carry-chain beats base-2⁶⁴ despite the 4-mult q̂·v
    /// product.
    #[inline(never)]
    pub fn div_knuth_u128_limb_slice(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
        crate::int::algos::div::div_knuth_u128_limb::div_knuth_u128_limb(num, den, quot, rem)
    }
    /// Burnikel-Ziegler chunking engine FORCED on (production engagement
    /// guard bypassed) so the Knuth-vs-BZ crossover can be timed at
    /// sub-threshold widths in `div_kernel_ab`.
    #[inline(never)]
    pub fn div_bz_forced_slice(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
        crate::int::algos::div::div_burnikel_ziegler_with_knuth::bz_chunk_core_forced(
            num, den, quot, rem,
        )
    }
    /// `div_rem` single-/double-limb hardware fast-path candidate for
    /// `div_kernel_ab` — the `Algorithm::Rem` arm. Bit-identical to the other
    /// engines; A/B measures the small-divisor regime where the const
    /// `u128 / u64` path is the production routing.
    #[inline(never)]
    pub fn div_rem_fast_slice(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
        crate::int::algos::div::div_rem::div_rem(num, den, quot, rem)
    }
    /// `div_rem_schoolbook` binary shift-subtract reference baseline for
    /// `div_kernel_ab` — the `Algorithm::Schoolbook` arm. Bit-identical to the
    /// other engines; A/B confirms it is the slowest (the never-routed
    /// baseline) at every shape.
    #[inline(never)]
    pub fn div_schoolbook_slice(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
        crate::int::algos::div::div_rem_schoolbook::div_rem_schoolbook(num, den, quot, rem)
    }
    /// Integer hypot kernel candidates for the `hypot_ab` microbench:
    /// the production Pythagoras path (radicand `a²+b²` + Newton slice
    /// `isqrt` + round) vs the native-u128 narrow fast path. Both
    /// bit-identical; A/B confirms which is faster per width.
    #[inline(never)]
    #[allow(private_bounds)]
    pub fn hypot_pythagoras<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> Option<crate::int::types::Int<N>>
    where
        crate::int::types::Int<N>: crate::int::types::compute_int::ComputeInt,
    {
        crate::int::algos::hypot::hypot_pythagoras::hypot_pythagoras::<N>(a, b, mode)
    }
    #[inline(never)]
    #[allow(private_bounds)]
    pub fn hypot_u128_fast<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> Option<crate::int::types::Int<N>>
    where
        crate::int::types::Int<N>: crate::int::types::compute_int::ComputeInt,
    {
        crate::int::algos::hypot::hypot_u128_fast::hypot_u128_fast::<N>(a, b, mode)
    }

    /// Remainder algorithm candidates exposed for the `rem_kernel_ab`
    /// microbench (the dispatch-seam A/B that decides the `rem` policy
    /// `select` arm per width).
    #[inline(never)]
    pub fn rem_native<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
    ) -> crate::int::types::Int<N> {
        crate::int::algos::rem::rem_native::rem_native::<N>(a, b)
    }
    #[inline(never)]
    pub fn rem_via_div_rem<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
    ) -> crate::int::types::Int<N> {
        crate::int::algos::rem::rem_via_div_rem::rem_via_div_rem::<N>(a, b)
    }
    #[inline(never)]
    pub fn rem_schoolbook<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
    ) -> crate::int::types::Int<N> {
        crate::int::algos::rem::rem_schoolbook::rem_schoolbook::<N>(a, b)
    }

    /// Integer `sqr` kernel candidates exposed for the `int_unary_kernel_ab`
    /// microbench (the dispatch-seam A/B that confirms the `sqr` policy
    /// routes to `HalfProduct` at every width vs the `Schoolbook` reference).
    #[inline(never)]
    pub fn sqr_half_product<const N: usize>(
        x: crate::int::types::Uint<N>,
    ) -> crate::int::types::Uint<N> {
        crate::int::algos::sqr::sqr_half_product::sqr_half_product::<N>(x)
    }
    #[inline(never)]
    pub fn sqr_schoolbook<const N: usize>(
        x: crate::int::types::Uint<N>,
    ) -> crate::int::types::Uint<N> {
        crate::int::algos::sqr::sqr_schoolbook::sqr_schoolbook::<N>(x)
    }

    /// Integer `pow` kernel candidates exposed for the `int_unary_kernel_ab`
    /// microbench (confirms `SquareAndMultiply` beats the `Schoolbook`
    /// repeated-multiply reference at the small fixed exponents `pow` uses).
    #[inline(never)]
    pub fn pow_square_and_multiply<const N: usize>(
        base: crate::int::types::Uint<N>,
        exp: u32,
    ) -> crate::int::types::Uint<N> {
        crate::int::algos::pow::pow_square_and_multiply::pow_square_and_multiply::<N>(base, exp)
    }
    #[inline(never)]
    pub fn pow_schoolbook<const N: usize>(
        base: crate::int::types::Uint<N>,
        exp: u32,
    ) -> crate::int::types::Uint<N> {
        crate::int::algos::pow::pow_schoolbook::pow_schoolbook::<N>(base, exp)
    }

    /// Integer `isqrt` kernel candidates (Newton f64-seeded vs the bitwise
    /// `Schoolbook` reference) over little-endian magnitude limb slices,
    /// exposed for the `int_unary_kernel_ab` microbench.
    #[inline(never)]
    pub fn isqrt_newton_slice(n: &[u64], out: &mut [u64]) {
        crate::int::algos::isqrt::isqrt_newton::isqrt_newton(n, out)
    }
    #[inline(never)]
    pub fn isqrt_schoolbook_slice(n: &[u64], out: &mut [u64]) {
        crate::int::algos::isqrt::isqrt_schoolbook::isqrt_schoolbook(n, out)
    }

    /// Integer `icbrt` kernel candidates (Newton f64-seeded vs the bitwise
    /// `Schoolbook` reference) over little-endian magnitude limb slices,
    /// exposed for the `int_unary_kernel_ab` microbench.
    #[inline(never)]
    pub fn icbrt_newton_slice(n: &[u64], out: &mut [u64]) {
        crate::int::algos::icbrt::icbrt_newton::icbrt_newton(n, out)
    }
    #[inline(never)]
    pub fn icbrt_schoolbook_slice(n: &[u64], out: &mut [u64]) {
        crate::int::algos::icbrt::icbrt_schoolbook::icbrt_schoolbook(n, out)
    }

    /// Decimal divide kernels exposed for the `mul_div_native_ab`
    /// microbench (the dispatch-seam A/B that decided narrow `N == 2`
    /// should route to the hardware-`i128` `div_native` arm; mul stays on
    /// the generic `mul_widen_divide` arm at every band).
    #[inline(never)]
    pub fn dec_div_native<const N: usize, const SCALE: u32>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
        mult: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<N> {
        crate::algos::div::div_native::div_native::<N, SCALE>(a, b, mult, mode)
    }
    #[inline(never)]
    pub fn dec_div_widen_scale_n1(
        a: crate::int::types::Int<1>,
        b: crate::int::types::Int<1>,
        mult: crate::int::types::Int<1>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<1> {
        crate::algos::div::div_widen_scale::div_widen_scale::<1>(a, b, mult, mode)
    }
    #[inline(never)]
    pub fn dec_div_widen_scale_n2(
        a: crate::int::types::Int<2>,
        b: crate::int::types::Int<2>,
        mult: crate::int::types::Int<2>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<2> {
        crate::algos::div::div_widen_scale::div_widen_scale::<2>(a, b, mult, mode)
    }
    /// Decimal multiply kernels exposed for the `mul_div_native_ab`
    /// microbench (the narrow native-vs-widen mul A/B at the dispatch seam).
    #[inline(never)]
    pub fn dec_mul_native<const N: usize, const SCALE: u32>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
        mult: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<N> {
        crate::algos::mul::mul_native::mul_native::<N, SCALE>(a, b, mult, mode)
    }
    #[inline(never)]
    pub fn dec_mul_widen_divide_n1<const SCALE: u32>(
        a: crate::int::types::Int<1>,
        b: crate::int::types::Int<1>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<1> {
        crate::algos::mul::mul_widen_divide::mul_widen_divide::<1, SCALE>(a, b, mode)
    }
    #[inline(never)]
    pub fn dec_mul_widen_divide_n2<const SCALE: u32>(
        a: crate::int::types::Int<2>,
        b: crate::int::types::Int<2>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<2> {
        crate::algos::mul::mul_widen_divide::mul_widen_divide::<2, SCALE>(a, b, mode)
    }
    /// Decimal remainder kernels exposed for the `rem_kernel_ab` microbench
    /// (the narrow native-vs-int-layer decimal rem A/B at the dispatch seam).
    #[inline(never)]
    #[allow(private_bounds)]
    pub fn dec_rem_int_layer<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
    ) -> crate::int::types::Int<N>
    where
        crate::int::types::Int<N>: crate::int::types::compute_int::ComputeInt,
    {
        crate::algos::rem::rem_int_layer::rem_int_layer::<N>(a, b)
    }
    /// The OLD wide decimal-remainder path: `Int::wrapping_rem` (the const
    /// single-algorithm `div_rem`, whose multi-limb fallback is an
    /// `O(bit_len)` binary shift-subtract). Exposed so `rem_kernel_ab` can
    /// A/B the recovered operator/Knuth `rem_int_layer` against the
    /// regressed path it replaced, on the live-bench `k * 10^SCALE` shape.
    #[inline(never)]
    pub fn int_wrapping_rem_slice<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
    ) -> crate::int::types::Int<N> {
        a.wrapping_rem(b)
    }
    #[inline(never)]
    pub fn dec_rem_native<const N: usize>(
        a: crate::int::types::Int<N>,
        b: crate::int::types::Int<N>,
    ) -> crate::int::types::Int<N> {
        crate::algos::rem::rem_native::rem_native::<N>(a, b)
    }
    /// Root kernels exposed for the `root_kernel_ab` microbench (the
    /// dispatch-seam A/B recovering the D57<20> cbrt regression vs prod
    /// 0.4.4). `sqrt_mg` / `sqrt_newton_slice` are kept as the D38 sqrt
    /// reference seam (the kernel was confirmed unchanged vs 0.4.4 and
    /// stays on the 256-bit arm; the hot u128 path is ~25 ns). `cbrt_*`
    /// are the D57<20> candidates: the f64-seeded `Int<6>` native arm vs
    /// the `Int<6>` + int-`icbrt` table-seed arm vs the generic slice.
    #[inline(never)]
    pub fn sqrt_mg<const SCALE: u32>(
        raw: crate::int::types::Int<2>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<2> {
        crate::algos::sqrt::sqrt_mg_divide::sqrt_mg_divide(raw, SCALE, mode)
    }
    #[inline(never)]
    pub fn sqrt_newton_slice<const SCALE: u32>(
        raw: crate::int::types::Int<2>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<2> {
        crate::algos::sqrt::sqrt_newton::sqrt_newton::<2>(raw, SCALE, mode)
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn cbrt_native_d57s20(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::cbrt::cbrt_native::cbrt_native_d57s20(raw, mode)
    }
    /// Candidate D57<20> cube root (0.4.4 full-radicand f64 seed + Newton
    /// pre-step) for the `root_kernel_ab` A/B against `cbrt_native_d57s20`.
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn cbrt_native_fast_d57s20(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<3, 6>(
            raw,
            const { crate::int::types::Int::<6>::TEN.pow(40) },
            mode,
        )
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn cbrt_table_seed_d57s20(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::cbrt::cbrt_newton_with_table_seed::cbrt_newton_with_table_seed(raw, mode)
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn cbrt_newton_slice<const SCALE: u32>(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::cbrt::cbrt_newton::cbrt_newton::<3>(raw, SCALE, mode)
    }
    /// Generic native root candidates over storage `N`, work width `W`,
    /// and `SCALE` -- exposed so `root_kernel_ab` can A/B the f64-seeded
    /// tight-`Int<W>` Newton arm against the generic slice at the wide
    /// tiers (D76 .. D307).
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn sqrt_native_w<const N: usize, const W: usize, const SCALE: u32>(
        raw: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<N> {
        crate::algos::sqrt::sqrt_native::sqrt_native::<N, W>(
            raw,
            const { crate::int::types::Int::<W>::TEN.pow(SCALE) },
            mode,
        )
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    #[allow(private_bounds)]
    pub fn sqrt_newton_slice_n<const N: usize, const SCALE: u32>(
        raw: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<N>
    where
        crate::int::types::Int<N>: crate::int::types::compute_int::ComputeInt,
    {
        crate::algos::sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode)
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn cbrt_native_w<const N: usize, const W: usize, const SCALE: u32>(
        raw: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<N> {
        crate::algos::cbrt::cbrt_native::cbrt_native::<N, W>(
            raw,
            const { crate::int::types::Int::<W>::TEN.pow(2 * SCALE) },
            mode,
        )
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    #[allow(private_bounds)]
    pub fn cbrt_newton_slice_n<const N: usize, const SCALE: u32>(
        raw: crate::int::types::Int<N>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<N>
    where
        crate::int::types::Int<N>: crate::int::types::compute_int::ComputeInt,
    {
        crate::algos::cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode)
    }

    // ── transcendental Series-vs-Tang dispatch-seam A/B exports ──────────
    //
    // The `policy::{exp,ln,trig}` Series-vs-Tang choice is made by hand-tuned
    // discrete `(N, SCALE)` bands but was never gated by an N-way `compare_all`
    // at the dispatch seam (the bands were tuned via `benches/lookup/*` kernel-
    // ISOLATION benches). These exports expose the Series kernel and the exact
    // production Tang kernel (same `Core`/`M`/`GUARD`/flag params the policy
    // `tang_routed` passes) at the band cells so `*_series_tang_ab` can A/B them
    // bit-identity-asserted at the seam, AND probe just-out-of-band scales.
    // One concrete fn per `(fn, tier)` cell because `Core` is a concrete per-
    // tier type; SCALE stays a const generic so the same fn covers in-band and
    // just-out-of-band probes.

    // exp — Series (generic-over-Core `exp_series`).
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn exp_series_d57<const SCALE: u32>(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw, mode)
    }
    #[cfg(any(feature = "d115", feature = "wide"))]
    #[inline(never)]
    pub fn exp_series_d115<const SCALE: u32>(
        raw: crate::int::types::Int<6>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<6> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw, mode)
    }
    #[cfg(any(feature = "d153", feature = "wide"))]
    #[inline(never)]
    pub fn exp_series_d153<const SCALE: u32>(
        raw: crate::int::types::Int<8>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<8> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw, mode)
    }
    // exp — Tang (production params per `policy::exp::tang_routed`).
    // D57 has TWO bands: 18..=22 (M=128,G=8) and 45..=56 (M=512,G=30); both
    // run flags <false,false,false>. Pass the band's M/G as const generics.
    #[cfg(all(feature = "_wide-support", any(feature = "d57", feature = "wide")))]
    #[inline(never)]
    pub fn exp_tang_d57<const SCALE: u32, const M: u32, const G: u32>(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, M, G, false, false, false>(raw, mode)
    }
    // D115 band 50..=60: M=128, G=8, flags <true,true,false>.
    #[cfg(all(feature = "_wide-support", any(feature = "d115", feature = "wide")))]
    #[inline(never)]
    pub fn exp_tang_d115<const SCALE: u32>(
        raw: crate::int::types::Int<6>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<6> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 128, 8, true, true, false>(raw, mode)
    }
    // D153 band 70..=82: M=128, G=10, flags <true,false,true>.
    #[cfg(all(feature = "_wide-support", any(feature = "d153", feature = "wide")))]
    #[inline(never)]
    pub fn exp_tang_d153<const SCALE: u32>(
        raw: crate::int::types::Int<8>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<8> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 128, 10, true, false, true>(raw, mode)
    }
    // D115 / D153 parameterised Tang (M/G const generics, directed +
    // external-extra flags <true,true,false> — the production wide shape) so
    // the wide A/B can map M/G across the full scale spread at these tiers,
    // not just the single fixed band config above.
    #[cfg(all(feature = "_wide-support", any(feature = "d115", feature = "wide")))]
    #[inline(never)]
    pub fn exp_tang_d115_p<const SCALE: u32, const M: u32, const G: u32>(
        raw: crate::int::types::Int<6>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<6> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, M, G, true, true, false>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d153", feature = "wide")))]
    #[inline(never)]
    pub fn exp_tang_d153_p<const SCALE: u32, const M: u32, const G: u32>(
        raw: crate::int::types::Int<8>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<8> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, M, G, true, true, false>(raw, mode)
    }

    // exp — D76 (Int<4>) and D230 (Int<12>): the two wide-buildable tiers
    // that had NO A/B export (D76 has no Tang rectangle at all). Series +
    // parameterised Tang (M/G const generics, directed + external-extra
    // flags <true,true,false> — the production wide-tier shape, valid at the
    // max-scale extreme) so the wide A/B (`exp_wide_series_tang_ab`) can map
    // them across the full scale spread and assert Tang == Series (validity).
    #[cfg(any(feature = "d76", feature = "wide"))]
    #[inline(never)]
    pub fn exp_series_d76<const SCALE: u32>(raw: crate::int::types::Int<4>, mode: crate::RoundingMode) -> crate::int::types::Int<4> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d76", feature = "wide")))]
    #[inline(never)]
    pub fn exp_tang_d76<const SCALE: u32, const M: u32, const G: u32>(raw: crate::int::types::Int<4>, mode: crate::RoundingMode) -> crate::int::types::Int<4> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d76::Core, SCALE, M, G, true, true, false>(raw, mode)
    }
    #[cfg(any(feature = "d230", feature = "wide"))]
    #[inline(never)]
    pub fn exp_series_d230<const SCALE: u32>(raw: crate::int::types::Int<12>, mode: crate::RoundingMode) -> crate::int::types::Int<12> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d230", feature = "wide")))]
    #[inline(never)]
    pub fn exp_tang_d230<const SCALE: u32, const M: u32, const G: u32>(raw: crate::int::types::Int<12>, mode: crate::RoundingMode) -> crate::int::types::Int<12> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d230::Core, SCALE, M, G, true, true, false>(raw, mode)
    }

    // exp — wide tiers (N = 16/24/32/48/64). Series + parameterised Tang
    // (M/G as const generics, single-shot flags <false,false,false>) so the
    // wide-tier A/B (`exp_wide_series_tang_ab`) can sweep the table size and
    // guard and assert Tang == Series (the validity wall) before timing.
    #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
    #[inline(never)]
    pub fn exp_series_d307<const SCALE: u32>(raw: crate::int::types::Int<16>, mode: crate::RoundingMode) -> crate::int::types::Int<16> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d307", feature = "wide", feature = "x-wide")))]
    #[inline(never)]
    pub fn exp_tang_d307<const SCALE: u32, const M: u32, const G: u32>(raw: crate::int::types::Int<16>, mode: crate::RoundingMode) -> crate::int::types::Int<16> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, M, G, true, true, false>(raw, mode)
    }
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    #[inline(never)]
    pub fn exp_series_d462<const SCALE: u32>(raw: crate::int::types::Int<24>, mode: crate::RoundingMode) -> crate::int::types::Int<24> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d462", feature = "x-wide")))]
    #[inline(never)]
    pub fn exp_tang_d462<const SCALE: u32, const M: u32, const G: u32>(raw: crate::int::types::Int<24>, mode: crate::RoundingMode) -> crate::int::types::Int<24> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d462::Core, SCALE, M, G, true, true, false>(raw, mode)
    }
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    #[inline(never)]
    pub fn exp_series_d616<const SCALE: u32>(raw: crate::int::types::Int<32>, mode: crate::RoundingMode) -> crate::int::types::Int<32> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d616", feature = "x-wide")))]
    #[inline(never)]
    pub fn exp_tang_d616<const SCALE: u32, const M: u32, const G: u32>(raw: crate::int::types::Int<32>, mode: crate::RoundingMode) -> crate::int::types::Int<32> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d616::Core, SCALE, M, G, true, true, false>(raw, mode)
    }
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    #[inline(never)]
    pub fn exp_series_d924<const SCALE: u32>(raw: crate::int::types::Int<48>, mode: crate::RoundingMode) -> crate::int::types::Int<48> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d924", feature = "xx-wide")))]
    #[inline(never)]
    pub fn exp_tang_d924<const SCALE: u32, const M: u32, const G: u32>(raw: crate::int::types::Int<48>, mode: crate::RoundingMode) -> crate::int::types::Int<48> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d924::Core, SCALE, M, G, true, true, false>(raw, mode)
    }
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    #[inline(never)]
    pub fn exp_series_d1232<const SCALE: u32>(raw: crate::int::types::Int<64>, mode: crate::RoundingMode) -> crate::int::types::Int<64> {
        crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d1232", feature = "xx-wide")))]
    #[inline(never)]
    pub fn exp_tang_d1232<const SCALE: u32, const M: u32, const G: u32>(raw: crate::int::types::Int<64>, mode: crate::RoundingMode) -> crate::int::types::Int<64> {
        crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d1232::Core, SCALE, M, G, true, true, false>(raw, mode)
    }

    // ln — Series (generic-over-Core `ln_series`).
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn ln_series_d57<const SCALE: u32>(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw, mode)
    }
    #[cfg(any(feature = "d115", feature = "wide"))]
    #[inline(never)]
    pub fn ln_series_d115<const SCALE: u32>(
        raw: crate::int::types::Int<6>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<6> {
        crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw, mode)
    }
    #[cfg(any(feature = "d153", feature = "wide"))]
    #[inline(never)]
    pub fn ln_series_d153<const SCALE: u32>(
        raw: crate::int::types::Int<8>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<8> {
        crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw, mode)
    }
    // ln — Tang (production params per `policy::ln::tang_routed`).
    // D57 18..=22: <GUARD=8, ITERS=100, false>.
    #[cfg(all(feature = "_wide-support", any(feature = "d57", feature = "wide")))]
    #[inline(never)]
    pub fn ln_tang_d57<const SCALE: u32>(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 8, 100, false>(raw, mode)
    }
    // D115 50..=60: <GUARD=8, ITERS=200, true>.
    #[cfg(all(feature = "_wide-support", any(feature = "d115", feature = "wide")))]
    #[inline(never)]
    pub fn ln_tang_d115<const SCALE: u32>(
        raw: crate::int::types::Int<6>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<6> {
        crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 8, 200, true>(raw, mode)
    }
    // D153 70..=82: <GUARD=10, ITERS=200, true>.
    #[cfg(all(feature = "_wide-support", any(feature = "d153", feature = "wide")))]
    #[inline(never)]
    pub fn ln_tang_d153<const SCALE: u32>(
        raw: crate::int::types::Int<8>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<8> {
        crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, 200, true>(raw, mode)
    }

    // trig forward (sin) — the only narrow-wide tier with a real Tang wiring
    // is D57 (band 44..=56, M=512); tiers 4/6/12/32/48/64 route the Tang select
    // arm back to `sin_series` (and `select` never returns Tang there), and
    // D153/D307/D462 have their own bespoke impls. D57 is the clean `wide`-
    // buildable band edge.
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[inline(never)]
    pub fn sin_series_d57<const SCALE: u32>(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::support::wide_trig_core::sin_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw, mode)
    }
    #[cfg(all(feature = "_wide-support", any(feature = "d57", feature = "wide")))]
    #[inline(never)]
    pub fn sin_tang_d57<const SCALE: u32>(
        raw: crate::int::types::Int<3>,
        mode: crate::RoundingMode,
    ) -> crate::int::types::Int<3> {
        crate::algos::trig::sincos_tang::sin_tang_with_taylor::<crate::types::widths::wide_trig_d57::Core, SCALE, 512>(raw, mode)
    }

    /// `to_radians` candidates for the `to_radians_ab` microbench (the
    /// wide-tier angle-conversion regression vs 0.4.4). `direct` calls the
    /// `MulPiRatio` kernel straight on the tier `Core` (no policy / resize
    /// indirection); `public` goes through the full public method path
    /// (`D::to_radians_strict_with` -> `to_radians_dispatch` ->
    /// `to_radians::dispatch` -> `mul_pi_ratio_routed`). The gap between the
    /// two is the dispatch/resize overhead the rewrite added.
    macro_rules! to_radians_bench {
        ($direct:ident, $public:ident, $n:literal, $core:ident, $feat:literal) => {
            #[cfg(any(feature = $feat, feature = "wide"))]
            #[inline(never)]
            pub fn $direct<const SCALE: u32>(
                raw: crate::int::types::Int<$n>,
                mode: crate::RoundingMode,
            ) -> crate::int::types::Int<$n> {
                crate::algos::trig::angle_mul_pi_ratio::to_radians_mul_pi_ratio::<
                    crate::types::widths::$core::Core,
                    SCALE,
                >(raw, mode)
            }
            #[cfg(any(feature = $feat, feature = "wide"))]
            #[inline(never)]
            pub fn $public<const SCALE: u32>(
                raw: crate::int::types::Int<$n>,
                mode: crate::RoundingMode,
            ) -> crate::int::types::Int<$n> {
                crate::D::<crate::int::types::Int<$n>, SCALE>(raw)
                    .to_radians_strict_with(mode)
                    .0
            }
        };
    }
    to_radians_bench!(to_radians_direct_d57, to_radians_public_d57, 3, wide_trig_d57, "d57");
    to_radians_bench!(to_radians_direct_d76, to_radians_public_d76, 4, wide_trig_d76, "d76");
    to_radians_bench!(to_radians_direct_d115, to_radians_public_d115, 6, wide_trig_d115, "d115");
    to_radians_bench!(to_radians_direct_d153, to_radians_public_d153, 8, wide_trig_d153, "d153");

    /// Build an `Int<N>` from a little-endian magnitude limb array (sign
    /// false). Lets the bench construct wide operands without exposing the
    /// internal constructors.
    #[inline(never)]
    pub fn int_from_mag_limbs<const N: usize>(mag: &[u64; N]) -> crate::int::types::Int<N> {
        crate::int::types::Int::<N>::from_mag_limbs(mag, false)
    }

    #[inline(never)]
    pub fn mul_u64_into<const L: usize, const LP1: usize>(
        a: &[u64; L],
        n: u64,
        out: &mut [u64; LP1],
    ) {
        crate::int::algos::mul::mul_schoolbook::mul_schoolbook_into::<L, LP1>(a, n, out)
    }

    // Newton-reciprocal divide research kernel — wrapped via concrete
    // shims so the bench harness gets head-to-head comparisons against
    // [`crate::algos::support::mg_divide::div_wide_pow10_chain`] without
    // exposing trait machinery.
    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
    pub mod newton_vs_mg {
        use crate::algos::support::newton_reciprocal::NewtonReciprocal as NR;
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
                    type W = $width;

                    /// Storage type for this tier — opaque to the bench.
                    #[derive(Clone, Copy)]
                    pub struct Storage(W);

                    /// Build a representative non-zero numerator from a top-limb position.
                    #[inline(never)]
                    pub fn build_numerator(top_limb_idx: usize) -> Storage {
                        use crate::int::types::traits::BigInt;
                        let mut mag = [0u128; 64];
                        mag[top_limb_idx] = 1u128 << 32;
                        mag[1] = 0xdeadbeef_cafef00d_u128;
                        Storage(W::from_mag_sign_u128(&mag, false))
                    }

                    #[inline(never)]
                    pub fn mg_chain(n: Storage, scale: u32) -> Storage {
                        Storage(crate::algos::support::mg_divide::div_wide_pow10_chain::<W>(
                            n.0,
                            scale,
                            RoundingMode::HalfToEven,
                        ))
                    }

                    #[inline(never)]
                    pub fn mg_single(n: Storage, scale: u32) -> Storage {
                        Storage(crate::algos::support::mg_divide::div_wide_pow10::<W>(
                            n.0,
                            scale,
                            RoundingMode::HalfToEven,
                        ))
                    }

                    #[inline(never)]
                    pub fn newton(n: Storage, scale: u32, table: &NewtonReciprocal) -> Storage {
                        Storage(
                            crate::algos::support::newton_reciprocal::div_wide_pow10_newton_with::<W>(
                                n.0,
                                scale,
                                RoundingMode::HalfToEven,
                                &table.0,
                            ),
                        )
                    }
                }
            };
        }
        shim!(d307, crate::int::types::Int<16>, "x-wide");
        shim!(d616, crate::int::types::Int<32>, "x-wide");
        shim!(d924, crate::int::types::Int<48>, "xx-wide");
        shim!(d1232, crate::int::types::Int<64>, "xx-wide");
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
// The integer layer is unconditional. D38's strict transcendentals use
// `Int512` as their guard-digit work integer (replacing the previous
// `algos::support::fixed::Fixed` 256-bit sign-magnitude type), so the wide-
// integer family must be available in every feature configuration —
// not just `feature = "wide"` builds. Compile-time impact is modest:
// ~2k LOC of self-contained limb arithmetic plus the const-generic
// `Int<N>` / `Uint<N>` monomorphisations.
// 0.5.0 const-generic integer layer. The integer side of the crate
// now mirrors the decimal layer's bucket split, all under `int/`:
// `int::types` (the `Int<N>`/`Uint<N>` types + named `IntXXXX`
// aliases), `int::policy` (algorithm-selection dispatch), and
// `int::algos` (width-matched algorithms, including the raw slice limb
// primitives in `int::algos::support::limbs`).
mod int;
mod policy;

#[cfg(feature = "serde")]
pub use crate::support::serde_helpers;

pub use crate::support::error::{ConvertError, ParseError};
pub use crate::support::rounding::RoundingMode;
pub use crate::types::consts::DecimalConstants;
pub use crate::types::traits::Decimal;
pub use crate::types::traits::DecimalArithmetic;
pub use crate::types::traits::DecimalConvert;
pub use crate::types::traits::DecimalTranscendental;
pub use crate::types::traits::WidthLE;
pub use crate::types::unified::D;

#[cfg(feature = "dyn")]
pub use crate::types::traits::dyn_decimal::{DecimalWidth, DynDecimal, RawStorage};

// D38 — the 128-bit foundation, plus every scale alias D38s0..=D38s37
// (v0.4.0 cap: MAX_SCALE = name - 1).
pub use crate::types::widths::{
    D38, D38s0, D38s1, D38s2, D38s3, D38s4, D38s5, D38s6, D38s7, D38s8, D38s9, D38s10, D38s11,
    D38s12, D38s13, D38s14, D38s15, D38s16, D38s17, D38s18, D38s19, D38s20, D38s21, D38s22, D38s23,
    D38s24, D38s25, D38s26, D38s27, D38s28, D38s29, D38s30, D38s31, D38s32, D38s33, D38s34, D38s35,
    D38s36, D38s37,
};


// D18 — 64-bit storage, scale 0..=18.
pub use crate::types::widths::{
    D18, D18s0, D18s1, D18s2, D18s3, D18s4, D18s5, D18s6, D18s7, D18s8, D18s9, D18s10, D18s11,
    D18s12, D18s13, D18s14, D18s15, D18s16, D18s17,
};

// The generic hand-rolled integer `Int<N>` (and unsigned `Uint<N>`) — the
// storage backend for D38 (`Int<2>`) and every wide tier. Exported so callers
// can name a decimal's storage type, e.g. for `from_bits` / `to_bits`. The
// fixed-width aliases (`Int256`, …) are the same types at specific `N`.
pub use crate::int::types::{Int, Uint};

// D76 — 256-bit storage, behind the `d76` / `wide` features.
#[cfg(any(feature = "d76", feature = "wide"))]
pub use crate::types::widths::{
    D76, D76s0, D76s1, D76s2, D76s3, D76s4, D76s6, D76s9, D76s12, D76s15, D76s18, D76s20, D76s24,
    D76s28, D76s32, D76s35, D76s38, D76s42, D76s48, D76s50, D76s56, D76s64, D76s70, D76s75,
};


// D153 — 512-bit storage, behind the `d153` / `wide` features.
#[cfg(any(feature = "d153", feature = "wide"))]
pub use crate::types::widths::{
    D153, D153s0, D153s1, D153s2, D153s4, D153s6, D153s9, D153s12, D153s15, D153s18, D153s20,
    D153s24, D153s28, D153s32, D153s35, D153s38, D153s50, D153s57, D153s75, D153s76, D153s100,
    D153s115, D153s140, D153s150, D153s152,
};

// D307 — 1024-bit storage, behind the `d307` / `wide` features.
#[cfg(any(feature = "d307", feature = "wide"))]
pub use crate::types::widths::{
    D307, D307s0, D307s1, D307s2, D307s4, D307s6, D307s9, D307s12, D307s15, D307s18, D307s20,
    D307s24, D307s28, D307s32, D307s35, D307s38, D307s50, D307s75, D307s100, D307s115, D307s150,
    D307s153, D307s200, D307s230, D307s275, D307s300, D307s306,
};

// ─── New half-width and wider tiers ───────────────────────────────────

// D57 — 192-bit storage; half-width between D38 and D76.
#[cfg(any(feature = "d57", feature = "wide"))]
pub use crate::types::widths::{
    D57, D57s0, D57s1, D57s2, D57s4, D57s6, D57s9, D57s12, D57s18, D57s20, D57s24, D57s28, D57s32,
    D57s38, D57s42, D57s48, D57s52, D57s56,
};

// D115 — 384-bit; half-width between D76 and D153.
#[cfg(any(feature = "d115", feature = "wide"))]
pub use crate::types::widths::{
    D115, D115s0, D115s1, D115s4, D115s8, D115s16, D115s24, D115s32, D115s38, D115s50, D115s57,
    D115s64, D115s76, D115s90, D115s100, D115s110, D115s114,
};

// D230 — 768-bit; half-width between D153 and D307.
#[cfg(any(feature = "d230", feature = "wide"))]
pub use crate::types::widths::{
    D230, D230s0, D230s1, D230s6, D230s18, D230s38, D230s57, D230s75, D230s100, D230s115, D230s140,
    D230s153, D230s175, D230s200, D230s215, D230s225, D230s229,
};

// D462 — 1536-bit; half-width between D307 and D616.
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub use crate::types::widths::{
    D462, D462s0, D462s1, D462s18, D462s38, D462s75, D462s115, D462s153, D462s200, D462s230,
    D462s275, D462s307, D462s350, D462s400, D462s440, D462s460, D462s461,
};

// D616 — 2048-bit; new top wide tier. Int2048 / Uint2048 are
// already exported above for x-wide / d307 widening; no re-export
// here.
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub use crate::types::widths::{
    D616, D616s0, D616s1, D616s38, D616s75, D616s115, D616s153, D616s200, D616s230, D616s275,
    D616s308, D616s380, D616s462, D616s500, D616s555, D616s600, D616s615,
};

// D924 — 3072-bit; half-width between D616 and D1232.
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub use crate::types::widths::{
    D924, D924s0, D924s1, D924s75, D924s153, D924s230, D924s307, D924s400, D924s461, D924s462,
    D924s500, D924s616, D924s700, D924s800, D924s860, D924s900, D924s920, D924s923,
};

// D1232 — 4096-bit; widest tier shipped.
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub use crate::types::widths::{
    D1232, D1232s0, D1232s1, D1232s75, D1232s153, D1232s230, D1232s307, D1232s461, D1232s616,
    D1232s700, D1232s800, D1232s900, D1232s924, D1232s1000, D1232s1100, D1232s1180, D1232s1220,
    D1232s1230, D1232s1231,
};

// ─── Construction macros (re-exports + per-scale wrappers) ────────────

/// The narrow-tier proc-macros are always available with the
/// `macros` feature; the wide-tier proc-macros are additionally
/// feature-gated to match their target type's availability.
#[cfg(feature = "macros")]
pub use decimal_scaled_macros::{d18, d38};

#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
pub use decimal_scaled_macros::d76;

#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))]
pub use decimal_scaled_macros::d153;

#[cfg(all(
    feature = "macros",
    any(feature = "d307", feature = "wide", feature = "x-wide")
))]
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

/// `d18s0!(value)` — equivalent to `d18!(value, scale 0)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d18s0  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 0  $(, $($rest)*)?) }; }
/// `d18s2!(value)` — equivalent to `d18!(value, scale 2)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d18s2  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 2  $(, $($rest)*)?) }; }
/// `d18s4!(value)` — equivalent to `d18!(value, scale 4)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d18s4  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 4  $(, $($rest)*)?) }; }
/// `d18s6!(value)` — equivalent to `d18!(value, scale 6)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d18s6  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 6  $(, $($rest)*)?) }; }
/// `d18s9!(value)` — equivalent to `d18!(value, scale 9)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d18s9  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 9  $(, $($rest)*)?) }; }
/// `d18s12!(value)` — equivalent to `d18!(value, scale 12)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d18s12 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d18!($v, scale 12 $(, $($rest)*)?) }; }

// D38 curated scales.
/// `d38s0!(value)` — equivalent to `d38!(value, scale 0)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s0  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 0  $(, $($rest)*)?) }; }
/// `d38s2!(value)` — equivalent to `d38!(value, scale 2)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s2  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 2  $(, $($rest)*)?) }; }
/// `d38s4!(value)` — equivalent to `d38!(value, scale 4)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s4  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 4  $(, $($rest)*)?) }; }
/// `d38s6!(value)` — equivalent to `d38!(value, scale 6)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s6  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 6  $(, $($rest)*)?) }; }
/// `d38s8!(value)` — equivalent to `d38!(value, scale 8)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s8  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 8  $(, $($rest)*)?) }; }
/// `d38s9!(value)` — equivalent to `d38!(value, scale 9)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s9  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 9  $(, $($rest)*)?) }; }
/// `d38s12!(value)` — equivalent to `d38!(value, scale 12)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s12 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 12 $(, $($rest)*)?) }; }
/// `d38s15!(value)` — equivalent to `d38!(value, scale 15)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s15 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 15 $(, $($rest)*)?) }; }
/// `d38s18!(value)` — equivalent to `d38!(value, scale 18)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s18 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 18 $(, $($rest)*)?) }; }
/// `d38s24!(value)` — equivalent to `d38!(value, scale 24)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s24 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 24 $(, $($rest)*)?) }; }
/// `d38s35!(value)` — equivalent to `d38!(value, scale 35)`.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! d38s35 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d38!($v, scale 35 $(, $($rest)*)?) }; }

// D76 curated scales.
/// `d76s0!(value)` — equivalent to `d76!(value, scale 0)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
#[macro_export]
macro_rules! d76s0  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 0  $(, $($rest)*)?) }; }
/// `d76s2!(value)` — equivalent to `d76!(value, scale 2)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
#[macro_export]
macro_rules! d76s2  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 2  $(, $($rest)*)?) }; }
/// `d76s6!(value)` — equivalent to `d76!(value, scale 6)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
#[macro_export]
macro_rules! d76s6  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 6  $(, $($rest)*)?) }; }
/// `d76s12!(value)` — equivalent to `d76!(value, scale 12)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
#[macro_export]
macro_rules! d76s12 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 12 $(, $($rest)*)?) }; }
/// `d76s18!(value)` — equivalent to `d76!(value, scale 18)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
#[macro_export]
macro_rules! d76s18 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 18 $(, $($rest)*)?) }; }
/// `d76s35!(value)` — equivalent to `d76!(value, scale 35)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
#[macro_export]
macro_rules! d76s35 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 35 $(, $($rest)*)?) }; }
/// `d76s50!(value)` — equivalent to `d76!(value, scale 50)`.
#[cfg(all(feature = "macros", any(feature = "d76", feature = "wide")))]
#[macro_export]
macro_rules! d76s50 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d76!($v, scale 50 $(, $($rest)*)?) }; }

// D153 curated scales.
/// `d153s0!(value)` — equivalent to `d153!(value, scale 0)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))]
#[macro_export]
macro_rules! d153s0   { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 0   $(, $($rest)*)?) }; }
/// `d153s35!(value)` — equivalent to `d153!(value, scale 35)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))]
#[macro_export]
macro_rules! d153s35  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 35  $(, $($rest)*)?) }; }
/// `d153s75!(value)` — equivalent to `d153!(value, scale 75)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))]
#[macro_export]
macro_rules! d153s75  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 75  $(, $($rest)*)?) }; }
/// `d153s150!(value)` — equivalent to `d153!(value, scale 150)`.
#[cfg(all(feature = "macros", any(feature = "d153", feature = "wide")))]
#[macro_export]
macro_rules! d153s150 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d153!($v, scale 150 $(, $($rest)*)?) }; }

// D307 curated scales.
/// `d307s0!(value)` — equivalent to `d307!(value, scale 0)`.
#[cfg(all(
    feature = "macros",
    any(feature = "d307", feature = "wide", feature = "x-wide")
))]
#[macro_export]
macro_rules! d307s0   { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 0   $(, $($rest)*)?) }; }
/// `d307s35!(value)` — equivalent to `d307!(value, scale 35)`.
#[cfg(all(
    feature = "macros",
    any(feature = "d307", feature = "wide", feature = "x-wide")
))]
#[macro_export]
macro_rules! d307s35  { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 35  $(, $($rest)*)?) }; }
/// `d307s150!(value)` — equivalent to `d307!(value, scale 150)`.
#[cfg(all(
    feature = "macros",
    any(feature = "d307", feature = "wide", feature = "x-wide")
))]
#[macro_export]
macro_rules! d307s150 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 150 $(, $($rest)*)?) }; }
/// `d307s300!(value)` — equivalent to `d307!(value, scale 300)`.
#[cfg(all(
    feature = "macros",
    any(feature = "d307", feature = "wide", feature = "x-wide")
))]
#[macro_export]
macro_rules! d307s300 { ($v:tt $(, $($rest:tt)*)?) => { $crate::d307!($v, scale 300 $(, $($rest)*)?) }; }
