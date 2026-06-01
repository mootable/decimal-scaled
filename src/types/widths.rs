//! Core type definitions for every decimal width and their scale aliases.
//!
//! Each width is a `#[repr(transparent)]` newtype around an integer
//! storage of the matching size. The stored integer equals
//! `actual_value * 10^SCALE`. Widths:
//!
//! `MAX_SCALE = name - 1` for every width (the v0.4.0 cap guaranteeing
//! at least one integer digit of headroom at every legal `SCALE`):
//!
//! | Type | Storage | `MAX_SCALE` |
//! |------|---------|-------------|
//! | [`D18<SCALE>`] | `i64`             | 17  |
//! | [`D38<SCALE>`] | `i128`            | 37  |
//! | [`D76<SCALE>`] | `crate::int::types::Int<4>` | 75 |
//! | [`D153<SCALE>`] | `crate::int::types::Int<8>` | 152 |
//! | [`D307<SCALE>`] | `crate::int::types::Int<16>` | 306 |
//!
//! The `#[repr(transparent)]` annotation is load-bearing: it guarantees
//! the same ABI as the underlying integer, so `from_bits` / `to_bits`
//! round-trips are exact and the types are safe to embed in C-ABI
//! plugin payloads when the underlying integer matches a primitive.

/// Scaled fixed-point decimal with 128-bit storage. Now a type alias
/// of the unified [`crate::D`] generic decimal type: `D38<S>` is
/// `D<i128, S>`. Both spellings are interchangeable.
///
/// `SCALE` is the base-10 exponent. A logical value `v` is stored as
/// `v * 10^SCALE` in the underlying `i128`. For example, with `SCALE = 12`
/// the number `1.5` is stored as `i128(1_500_000_000_000)`.
///
/// The `#[repr(transparent)]` layout over `i128` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// # Precision
///
/// N/A: type definition, no arithmetic performed.
///
/// # Determinism
///
/// All arithmetic is integer arithmetic on `i128`. The same inputs produce
/// the same bit-pattern on every platform.
///
/// # Equality and ordering
///
/// `Hash`, `Eq`, and `Ord` are derived from `i128`. Two `D38<S>` values
/// are equal if and only if their underlying `i128` fields are bit-equal.
/// This works because the scale is fixed at compile time -- each logical
/// value has exactly one representation.
///
/// # Const-generic scale
///
/// The const generic allows scale variants (`D38<9>`, `D38<6>`, etc.)
/// as trivial type aliases without duplicating any method implementations.
/// Mixed-scale arithmetic is deliberately not provided; callers convert
/// explicitly.
pub type D38<const SCALE: u32> = crate::D<crate::int::types::Int<2>, SCALE>;

// Manual `Debug` is implemented in `display.rs` (via the
// `decl_decimal_display!` macro) and renders via `Display` so the
// canonical decimal string is shown rather than the raw i128.

/// `Default` returns `ZERO`, matching `i128::default() == 0`.
///
/// This lets `#[derive(Default)]` work correctly on structs that contain
/// `D38<S>` fields.
///
/// Implemented on the underlying `crate::D<decimal_scaled::Int<2>, SCALE>` because
/// `D38<SCALE>` is now an alias of that type. `ZERO` is emitted by
/// the basics macro further down in this file.
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<2>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

// Scale aliases: D38s0 through D38s38.
//
// Each alias names a specific SCALE value. The const-generic impl block
// makes every method generic, so adding aliases is purely additive.
//
// Representable integer range is approximately `i128::MAX / 10^SCALE`.
// `i128::MAX` is approximately 1.7e38.
//
// SCALE = 0 is supported (mg_divide special-cases it to plain i128
// arithmetic). SCALE = 38 is the upper bound: 10^38 < i128::MAX, but
// 10^39 overflows. The math constants (pi, tau, e, golden) have a
// 35-digit reference in consts.rs (SCALE_REF = 35); at SCALE > 35
// they are zero-extended and gain no real precision.

/// Scale alias: `D38<0>`. 1 LSB = 1 (thin `i128` wrapper, no rescale).
/// Range ~+/-1.7e38.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s0 = D38<0>;

/// Scale alias: `D38<1>`. 1 LSB = 10^-1 (1 decimal digit).
/// Range ~+/-1.7e37.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s1 = D38<1>;

/// Scale alias: `D38<2>`. 1 LSB = 10^-2 (cents). Range ~+/-1.7e36.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s2 = D38<2>;

/// Scale alias: `D38<3>`. 1 LSB = 10^-3 (thousandths; 1 mm at m units).
/// Range ~+/-1.7e35.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s3 = D38<3>;

/// Scale alias: `D38<4>`. 1 LSB = 10^-4 (basis points). Range ~+/-1.7e34.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s4 = D38<4>;

/// Scale alias: `D38<5>`. 1 LSB = 10^-5. Range ~+/-1.7e33.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s5 = D38<5>;

/// Scale alias: `D38<6>`. 1 LSB = 10^-6 (1 um at mm units; ppm).
/// Range ~+/-1.7e32.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s6 = D38<6>;

/// Scale alias: `D38<7>`. 1 LSB = 10^-7. Range ~+/-1.7e31.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s7 = D38<7>;

/// Scale alias: `D38<8>`. 1 LSB = 10^-8 (satoshi-grade). Range ~+/-1.7e30.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s8 = D38<8>;

/// Scale alias: `D38<9>`. 1 LSB = 10^-9 (1 nm at mm units; ppb).
/// Range ~+/-1.7e29.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s9 = D38<9>;

/// Scale alias: `D38<10>`. 1 LSB = 10^-10. Range ~+/-1.7e28.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s10 = D38<10>;

/// Scale alias: `D38<11>`. 1 LSB = 10^-11. Range ~+/-1.7e27.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s11 = D38<11>;

/// Scale alias: `D38<12>`. 1 LSB = 10^-12 (1 pm at mm units).
/// Range ~+/-1.7e14 model units.
///
/// This is the primary concrete alias for general use. At `SCALE = 12`:
/// - 1 LSB is `10^-12` model units.
/// - The representable integer range is approximately +/-1.7e14 model units.
/// - Squared-component operations (e.g. dot products) overflow beyond
/// roughly 13,000 km at mm units.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s12 = D38<12>;

/// Scale alias: `D38<13>`. 1 LSB = 10^-13. Range ~+/-1.7e25.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s13 = D38<13>;

/// Scale alias: `D38<14>`. 1 LSB = 10^-14. Range ~+/-1.7e24.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s14 = D38<14>;

/// Scale alias: `D38<15>`. 1 LSB = 10^-15 (femto). Range ~+/-1.7e23.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s15 = D38<15>;

/// Scale alias: `D38<16>`. 1 LSB = 10^-16. Range ~+/-1.7e22.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s16 = D38<16>;

/// Scale alias: `D38<17>`. 1 LSB = 10^-17. Range ~+/-1.7e21.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s17 = D38<17>;

/// Scale alias: `D38<18>`. 1 LSB = 10^-18 (atto; high-precision scientific).
/// Range ~+/-1.7e20.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s18 = D38<18>;

/// Scale alias: `D38<19>`. 1 LSB = 10^-19. Range ~+/-1.7e19.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s19 = D38<19>;

/// Scale alias: `D38<20>`. 1 LSB = 10^-20. Range ~+/-1.7e18.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s20 = D38<20>;

/// Scale alias: `D38<21>`. 1 LSB = 10^-21 (zepto). Range ~+/-1.7e17.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s21 = D38<21>;

/// Scale alias: `D38<22>`. 1 LSB = 10^-22. Range ~+/-1.7e16.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s22 = D38<22>;

/// Scale alias: `D38<23>`. 1 LSB = 10^-23. Range ~+/-1.7e15.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s23 = D38<23>;

/// Scale alias: `D38<24>`. 1 LSB = 10^-24 (yocto). Range ~+/-1.7e14.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s24 = D38<24>;

/// Scale alias: `D38<25>`. 1 LSB = 10^-25. Range ~+/-1.7e13.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s25 = D38<25>;

/// Scale alias: `D38<26>`. 1 LSB = 10^-26. Range ~+/-1.7e12.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s26 = D38<26>;

/// Scale alias: `D38<27>`. 1 LSB = 10^-27. Range ~+/-1.7e11.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s27 = D38<27>;

/// Scale alias: `D38<28>`. 1 LSB = 10^-28. Range ~+/-1.7e10.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s28 = D38<28>;

/// Scale alias: `D38<29>`. 1 LSB = 10^-29. Range ~+/-1.7e9.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s29 = D38<29>;

/// Scale alias: `D38<30>`. 1 LSB = 10^-30. Range ~+/-1.7e8.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s30 = D38<30>;

/// Scale alias: `D38<31>`. 1 LSB = 10^-31. Range ~+/-1.7e7.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s31 = D38<31>;

/// Scale alias: `D38<32>`. 1 LSB = 10^-32. Range ~+/-1.7e6.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s32 = D38<32>;

/// Scale alias: `D38<33>`. 1 LSB = 10^-33. Range ~+/-1.7e5.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s33 = D38<33>;

/// Scale alias: `D38<34>`. 1 LSB = 10^-34. Range ~+/-1.7e4.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s34 = D38<34>;

/// Scale alias: `D38<35>`. 1 LSB = 10^-35. Range ~+/-1.7e3.
///
/// Matches `SCALE_REF` in `consts.rs`: the math constants `pi`, `tau`,
/// `e`, and `golden` are stored at this reference scale internally, so
/// at `SCALE = 35` they round-trip without precision loss.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s35 = D38<35>;

/// Scale alias: `D38<36>`. 1 LSB = 10^-36. Range ~+/-170.
///
/// The math constants (`pi`, `tau`, `e`, `golden`) are stored at a
/// 35-digit reference. Above `SCALE = 35` they are scaled up from that
/// reference, so trailing digits are zero-extended rather than
/// meaningfully precise.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s36 = D38<36>;

/// Scale alias: `D38<37>`. 1 LSB = 10^-37. Range ~+/-17.
///
/// This is the maximum supported scale: `MAX_SCALE = 37` guarantees at
/// least one integer digit (`|x| >= 1`) for every representable value.
/// `10^38 < i128::MAX < 10^39`, so the storage could in principle hold a
/// scale-38 representation, but doing so would leave `|x| < 1.7` with no
/// integer-digit headroom -- the v0.4.0 cap rules this out by design.
/// Math constants lose precision above `SCALE = 35`; see `D38s36`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s37 = D38<37>;

// The `ParseError` enum lives in `src/error.rs` and is re-exported
// from the crate root. It is not width-specific.
pub use crate::support::error::ParseError;

// Inherent basics + Decimal trait impl: emitted by the macro generator
// (one invocation per width). See src/decimal_macro.rs for the macro
// definition and the surface it produces.
crate::macros::basics::decl_decimal_basics!(wide D38, crate::int::types::Int<2>, 37);
crate::macros::display::decl_decimal_display!(wide D38, crate::int::types::Uint<2>);
// FromStr and the raw-storage hex / octal / binary formatters: the
// shared macros. D38's hand-coded versions were equivalent (`FromStr`
// delegated to the same `parse_decimal` path; the formatters delegate
// straight to the `i128` formatter).
crate::macros::from_str::decl_decimal_from_str!(wide D38, crate::int::types::Int<2>);
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D38);
// Bitwise operators (BitAnd/Or/Xor/Not, Shl/Shr) and bit-manipulation
// methods (unsigned_shr, rotate_*, *_zeros, count_*, *_power_of_two) on
// the raw storage. Previously hand-coded for D38 only; now a shared
// macro so every width has the surface.
crate::macros::bitwise::decl_decimal_bitwise!(wide D38, crate::int::types::Int<2>);
// Euclidean / floor / ceil division, abs_diff, midpoint, and the
// is_zero / is_normal / is_nan / is_infinite / is_finite predicates.
crate::macros::int_methods::decl_decimal_int_methods!(wide D38, crate::int::types::Int<2>);
// FromPrimitive / ToPrimitive / NumCast via the shared macro.
crate::macros::num_traits::decl_decimal_num_traits_conversions!(wide D38, crate::int::types::Int<2>);
crate::macros::float_bridge::decl_decimal_float_bridge!(wide D38, crate::int::types::Int<2>);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, i8);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, i16);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, i32);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, i64);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, u8);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, u16);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, u32);
crate::macros::conversions::decl_from_primitive!(wide D38, crate::int::types::Int<2>, u64);
crate::macros::conversions::decl_try_from_i128!(wide D38, crate::int::types::Int<2>);
crate::macros::conversions::decl_try_from_u128!(wide D38, crate::int::types::Int<2>);
crate::macros::conversions::decl_try_from_i128!(wide D18, crate::int::types::Int<1>);
crate::macros::conversions::decl_try_from_u128!(wide D18, crate::int::types::Int<1>);
// D18 (i64 storage): `i64` / `u64` cannot be infallible `From` because
// `value * 10^SCALE` overflows the 64-bit storage for SCALE >= 1 (and a
// `u64` above `i64::MAX` overflows even at SCALE 0), so the standard
// surface for these source types on the 64-bit tier is `TryFrom`.
// Wider tiers (D38+) get `From<i64>` / `From<u64>` from
// `decl_from_primitive!` because their storage holds the scaled value.
crate::macros::conversions::decl_try_from_i64!(wide D18, crate::int::types::Int<1>);
crate::macros::conversions::decl_try_from_u64!(wide D18, crate::int::types::Int<1>);
crate::macros::conversions::decl_try_from_f64!(wide D38, crate::int::types::Int<2>);
crate::macros::conversions::decl_try_from_f32!(wide D38, crate::int::types::Int<2>);
crate::macros::conversions::decl_try_from_f64!(wide D18, crate::int::types::Int<1>);
crate::macros::conversions::decl_try_from_f32!(wide D18, crate::int::types::Int<1>);
crate::macros::conversions::decl_decimal_int_conversion_methods!(wide D38, crate::int::types::Int<2>, i64);
// abs / signum / is_positive / is_negative, min / max / clamp / recip /
// copysign, and floor / ceil / round / trunc / fract are emitted by the
// shared macros — D38's hand-coded versions were byte-identical to the
// macro output (see `src/macros/{sign,helpers,rounding_methods}.rs`).
crate::macros::sign::decl_decimal_sign_methods!(wide D38, crate::int::types::Int<2>);
crate::macros::helpers::decl_decimal_helpers!(wide D38);
crate::macros::rounding_methods::decl_decimal_rounding_methods!(wide D38);
// Overflow-variant families for add / sub / neg / rem: the macro's
// shared `@common` arm. D38's hand-coded versions were byte-identical.
// The mul / div variants stay hand-coded in `src/overflow_variants.rs`
// because they route through the type-specific `mg_divide` path.
crate::macros::overflow::decl_decimal_overflow_variants!(wide D38, crate::int::types::Int<2>, crate::int::types::Int<4>);
// Add / Sub / Neg / Rem operator impls (and their `*Assign` forms): the
// arithmetic macro's shared `@common` arm. Mul / Div stay hand-coded in
// `src/arithmetic.rs` (the `mg_divide` 256-bit-widening path).
crate::macros::arithmetic::decl_decimal_arithmetic!(wide D38, crate::int::types::Int<2>, crate::int::types::Int<4>);
// num-traits: Zero / One / Num / Bounded / Signed / Checked{Add,Sub,Mul,
// Div,Rem,Neg} via the shared macro — D38's hand-coded impls were
// equivalent. FromPrimitive / ToPrimitive / NumCast stay hand-coded in
// `src/num_traits_impls.rs` (not part of the macro surface).
crate::macros::num_traits::decl_decimal_num_traits_basics!(D38);
crate::macros::transcendental_trait::decl_decimal_transcendental_impl!(D38);

// D38 strict transcendentals: hand-tuned per-type kernels.
//
// The canonical public `*_strict` surface (`ln_strict`, `exp_strict`,
// `sin_strict`, `powf_strict`, …) is emitted by the per-type files
// `types/log_exp.rs` / `types/trig.rs` / `types/powers.rs` using
// the hand-tuned 256-bit `algos::support::fixed::Fixed` work integer. They
// are the **chosen winners** per the per-type-kernel policy:
//
// - `decl_wide_transcendental!(D38, crate::int::types::Int<2>, Int<8>, …)` would deliver
//   the same surface using the generic limb arithmetic. Bench
//   analysis (ln 29 µs hand-tuned vs ≈ 100+ µs macro path) puts the
//   macro firmly past the 1.5× crossover, so the hand-tuned kernel
//   wins.
//
// The alternative macro implementation is **not compiled** in normal
// builds — invoking the macro here unconditionally would emit
// duplicate-name methods that conflict with the canonical override.
// Under a future `bench-alt` feature the macro can be re-invoked
// with a renamed-suffix shape (`*_strict_macro_alt`) so a benchmark
// can compare both paths in one binary; until that knob exists the
// macro path stays dormant for D38.
//
// Same naming convention applies to per-type lossy overrides as
// they land: `*_lossy_override` opt-in companion, canonical name
// reserved for the chosen-winner implementation.

crate::macros::conversions::decl_decimal_int_conversion_methods!(wide D18, crate::int::types::Int<1>, i64);

// ─── D38 narrow ───────────────────────────────────────────────────────
// D38::widen is wide-tier-only and is emitted further down in the
// wide block. D38::narrow is always available.

impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    /// Demote to the previous storage tier ([`D18`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::OutOfRange)` if the value
    /// doesn't fit `i64`'s range at the given scale.
    ///
    /// ```
    /// use decimal_scaled::D38s9;
    /// let a = D38s9::from(1_000_000);
    /// let b = a.narrow().unwrap();
    /// assert_eq!(i128::from(b.to_bits()), i128::from(a.to_bits()));
    /// ```
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<1>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

// ---------------------------------------------------------------------
// D18 — 64-bit storage, scale 0..=18. Interchange size; fits a GPR on
// 64-bit hosts and maps cleanly to ANSI / SQL `DECIMAL(18, S)` columns.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 64-bit storage. See [`D38`] for the
/// shape documentation; D18 has the same surface scaled to `i64` and
/// `MAX_SCALE = 17` (the v0.4.0 cap: `MAX_SCALE = name - 1`).
///
/// Now a type alias of the unified [`crate::D`] generic decimal type:
/// `D18<S>` is `D<i64, S>`. Both spellings are interchangeable. The
/// `#[repr(transparent)]` layout over `i64` is preserved through the
/// alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
pub type D18<const SCALE: u32> = crate::D<crate::int::types::Int<1>, SCALE>;

/// `Default` returns `ZERO`.
///
/// Implemented on the underlying `crate::D<Int<1>, SCALE>` because
/// `D18<SCALE>` is now an alias of that type. `ZERO` is emitted by
/// the basics macro further down in this file.
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<1>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

crate::macros::basics::decl_decimal_basics!(wide D18, crate::int::types::Int<1>, 17);
crate::macros::arithmetic::decl_decimal_arithmetic!(wide D18, crate::int::types::Int<1>, crate::int::types::Int<2>);
crate::macros::conversions::decl_from_primitive!(wide D18, crate::int::types::Int<1>, i8);
crate::macros::conversions::decl_from_primitive!(wide D18, crate::int::types::Int<1>, i16);
crate::macros::conversions::decl_from_primitive!(wide D18, crate::int::types::Int<1>, i32);
crate::macros::conversions::decl_from_primitive!(wide D18, crate::int::types::Int<1>, u8);
crate::macros::conversions::decl_from_primitive!(wide D18, crate::int::types::Int<1>, u16);
crate::macros::conversions::decl_from_primitive!(wide D18, crate::int::types::Int<1>, u32);
crate::macros::display::decl_decimal_display!(wide D18, crate::int::types::Uint<1>);
crate::macros::overflow::decl_decimal_overflow_variants!(wide D18, crate::int::types::Int<1>, crate::int::types::Int<2>);
crate::macros::num_traits::decl_decimal_num_traits_basics!(D18);
crate::macros::sign::decl_decimal_sign_methods!(wide D18, crate::int::types::Int<1>);
// D18 (Int<1>) `DecimalConstants` comes from the single generic impl in
// `src/types/consts/d38.rs` (sourced from the unified `consts` table) — no
// per-tier macro invocation.
crate::macros::from_str::decl_decimal_from_str!(wide D18, crate::int::types::Int<1>);
crate::macros::float_bridge::decl_decimal_float_bridge!(wide D18, crate::int::types::Int<1>);
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D18);
crate::macros::strict_transcendentals::decl_strict_transcendentals_via_d38!(D18);
crate::macros::transcendental_trait::decl_decimal_transcendental_impl!(D18);
crate::macros::fast_transcendentals::decl_fast_transcendentals_via_f64!(D18);
crate::macros::pow::decl_decimal_pow!(D18);
crate::macros::rounding_methods::decl_decimal_rounding_methods!(wide D18);
crate::macros::helpers::decl_decimal_helpers!(wide D18);
crate::macros::bitwise::decl_decimal_bitwise!(wide D18, crate::int::types::Int<1>);
crate::macros::int_methods::decl_decimal_int_methods!(wide D18, crate::int::types::Int<1>);
crate::macros::num_traits::decl_decimal_num_traits_conversions!(wide D18, crate::int::types::Int<1>);

// Cross-width widening (lossless). D18 -> D38.
crate::macros::conversions::decl_cross_width_widening!(wide D38, crate::int::types::Int<2>, D18, crate::int::types::Int<1>);

// Cross-width narrowing (fallible). D38 -> D18.
crate::macros::conversions::decl_cross_width_narrowing!(wide D18, crate::int::types::Int<1>, D38, crate::int::types::Int<2>);

// ─── `widen` — hop one storage tier up ────────────────────────────────
//
// `widen` always succeeds (the next-larger storage strictly covers every
// value the smaller one can hold). It keeps the scale unchanged; combine
// with `rescale` if you need to change scale and width together. D18 is
// the narrowest tier, so it has no `narrow`.

impl<const SCALE: u32> crate::D<crate::int::types::Int<1>, SCALE> {
    /// Promote to the next storage tier ([`D38`]) at the same `SCALE`.
    /// Lossless.
    ///
    /// ```
    /// use decimal_scaled::D18s9;
    /// let a = D18s9::from(7);
    /// let b = a.widen();              // D38<9>
    /// assert_eq!(i128::from(b.to_bits()), i128::from(a.to_bits()));
    /// ```
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<2>, SCALE> {
        self.into()
    }
}

/// Scale alias: `D18<0>`. 1 LSB = 1. Range ±9.2 × 10¹⁸.
pub type D18s0 = D18<0>;
/// Scale alias: `D18<1>`. 1 LSB = 10^-1. Range ±9.2 × 10¹⁷.
pub type D18s1 = D18<1>;
/// Scale alias: `D18<2>`. 1 LSB = 10^-2 (cents). Range ±9.2 × 10¹⁶.
pub type D18s2 = D18<2>;
/// Scale alias: `D18<3>`. 1 LSB = 10^-3 (mills). Range ±9.2 × 10¹⁵.
pub type D18s3 = D18<3>;
/// Scale alias: `D18<4>`. 1 LSB = 10^-4 (basis points). Range ±9.2 × 10¹⁴.
pub type D18s4 = D18<4>;
/// Scale alias: `D18<5>`. 1 LSB = 10^-5. Range ±9.2 × 10¹³.
pub type D18s5 = D18<5>;
/// Scale alias: `D18<6>`. 1 LSB = 10^-6 (ppm). Range ±9.2 × 10¹².
pub type D18s6 = D18<6>;
/// Scale alias: `D18<7>`. 1 LSB = 10^-7. Range ±9.2 × 10¹¹.
pub type D18s7 = D18<7>;
/// Scale alias: `D18<8>`. 1 LSB = 10^-8 (satoshi). Range ±9.2 × 10¹⁰.
pub type D18s8 = D18<8>;
/// Scale alias: `D18<9>`. 1 LSB = 10^-9 (nano). Range ±9.2 × 10⁹.
pub type D18s9 = D18<9>;
/// Scale alias: `D18<10>`. 1 LSB = 10^-10. Range ±9.2 × 10⁸.
pub type D18s10 = D18<10>;
/// Scale alias: `D18<11>`. 1 LSB = 10^-11. Range ±9.2 × 10⁷.
pub type D18s11 = D18<11>;
/// Scale alias: `D18<12>`. 1 LSB = 10^-12 (pico). Range ±9.2 × 10⁶.
pub type D18s12 = D18<12>;
/// Scale alias: `D18<13>`. 1 LSB = 10^-13. Range ±9.2 × 10⁵.
pub type D18s13 = D18<13>;
/// Scale alias: `D18<14>`. 1 LSB = 10^-14. Range ±9.2 × 10⁴.
pub type D18s14 = D18<14>;
/// Scale alias: `D18<15>`. 1 LSB = 10^-15 (femto). Range ±9200.
pub type D18s15 = D18<15>;
/// Scale alias: `D18<16>`. 1 LSB = 10^-16. Range ±920.
pub type D18s16 = D18<16>;
/// Scale alias: `D18<17>`. 1 LSB = 10^-17. Range ±92.
///
/// Maximum supported scale (v0.4.0 cap: `MAX_SCALE = name - 1`
/// guarantees at least one integer digit at every legal SCALE).
pub type D18s17 = D18<17>;

// ---------------------------------------------------------------------
// D76 — 256-bit storage (`Int<4>`), scale 0..=76. First of the
// wide tier; gated behind the `d76` / `wide` Cargo features. Covers
// the full IEEE-754 decimal128 range and gives 35-digit fractional
// precision with integer-part headroom (see research doc §1).
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 256-bit storage. See [`D38`] for the
/// shape documentation; D76 has the same surface scaled to a 256-bit
/// signed integer and `MAX_SCALE = 75`. Now a type alias of the unified
/// [`crate::D`] generic decimal type: `D76<S>` is
/// `D<crate::int::types::Int<4>, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<4>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d76` (or umbrella `wide`) Cargo feature. The
/// storage backend is `Int<4>`.
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76<const SCALE: u32> = crate::D<crate::int::types::Int<4>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<4>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<4>, SCALE>`
/// because `D76<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<4>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D76,
    crate::int::types::Int<4>,
    crate::int::types::Uint<4>,
    crate::int::types::Int<8>,
    crate::int::types::Int<8>,
    crate::int::types::Int<16>,
    crate::int::types::Int<16>,
    crate::int::types::Int<32>,
    crate::int::types::Int<16>,
    wide_trig_d76,
    75,
    4,
    400,
    512
);
// Cross-width widening into D76 (lossless): D18 / D38 -> D76.
#[cfg(any(feature = "d76", feature = "wide"))]
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D76, crate::int::types::Int<4>, D18, crate::int::types::Int<1>);
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D76, crate::int::types::Int<4>, D38, crate::int::types::Int<2>);
// Cross-width narrowing from D76 (fallible): D76 -> D38 / D18.
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D38, crate::int::types::Int<2>, D76, crate::int::types::Int<4>);
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D18, crate::int::types::Int<1>, D76, crate::int::types::Int<4>);
#[cfg(any(feature = "d76", feature = "wide"))]

// ─── D38::widen / D76 hop methods ─────────────────────────────────────

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    /// Promote to the next storage tier ([`D57`]) at the same `SCALE`.
    /// Lossless. Available with the `d57` (or umbrella `wide`) Cargo
    /// feature enabled.
    ///
    /// ```
    /// # #[cfg(feature = "wide")] {
    /// use decimal_scaled::D38s12;
    /// let a = D38s12::from(1_000_000);
    /// let _wider = a.widen();  // D57<12>
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<3>, SCALE> {
        self.into()
    }
}

#[cfg(all(
    any(feature = "d76", feature = "wide"),
    any(feature = "d57", feature = "wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<4>, SCALE> {
    /// Demote to the previous storage tier ([`D57`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::Overflow)` if the value
    /// doesn't fit `D57`'s range at the given scale.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<3>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

/// Scale alias: `D76<0>`. 1 LSB = 1 (256-bit integer ledger).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s0 = D76<0>;
/// Scale alias: `D76<1>`. 1 LSB = 10^-1 (tenths).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s1 = D76<1>;
/// Scale alias: `D76<2>`. 1 LSB = 10^-2 (cents).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s2 = D76<2>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s3 = D76<3>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s4 = D76<4>;
/// Scale alias: `D76<6>`. 1 LSB = 10^-6 (ppm).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s6 = D76<6>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s9 = D76<9>;
/// Scale alias: `D76<12>`. 1 LSB = 10^-12 (pico; financial standard).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s12 = D76<12>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s15 = D76<15>;
/// Scale alias: `D76<18>`. 1 LSB = 10^-18 (atto).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s18 = D76<18>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s20 = D76<20>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s24 = D76<24>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s28 = D76<28>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s32 = D76<32>;
/// Scale alias: `D76<35>`. 1 LSB = 10^-35 (matches `SCALE_REF`).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s35 = D76<35>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s38 = D76<38>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s42 = D76<42>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s48 = D76<48>;
/// Scale alias: `D76<50>`. 1 LSB = 10^-50 (deep scientific precision).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s50 = D76<50>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s56 = D76<56>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s64 = D76<64>;
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s70 = D76<70>;
/// Scale alias: `D76<75>`. 1 LSB = 10^-75. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d76", feature = "wide"))]
pub type D76s75 = D76<75>;

// ---------------------------------------------------------------------
// D153 — 512-bit storage (`Int<8>`), scale 0..=153. Wide-scientific
// tier; gated behind the `d153` / `wide` Cargo features.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 512-bit storage. See [`D38`] for the
/// shape documentation; D153 has the same surface scaled to a 512-bit
/// signed integer and `MAX_SCALE = 152`. Now a type alias of the unified
/// [`crate::D`] generic decimal type: `D153<S>` is
/// `D<crate::int::types::Int<8>, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<8>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d153` (or umbrella `wide`) Cargo feature. The
/// storage backend is `Int<8>`.
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153<const SCALE: u32> = crate::D<crate::int::types::Int<8>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<8>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<8>, SCALE>`
/// because `D153<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<8>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D153,
    crate::int::types::Int<8>,
    crate::int::types::Uint<8>,
    crate::int::types::Int<16>,
    crate::int::types::Int<16>,
    crate::int::types::Int<32>,
    crate::int::types::Int<32>,
    crate::int::types::Int<64>,
    crate::int::types::Int<32>,
    wide_trig_d153,
    152,
    8,
    200,
    512
);
// Cross-width widening into D153 (lossless): D38 / D76 -> D153.
#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D153, crate::int::types::Int<8>, D38, crate::int::types::Int<2>);
#[cfg(all(
    any(feature = "d153", feature = "wide"),
    any(feature = "d76", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D153, crate::int::types::Int<8>, D76, crate::int::types::Int<4>);
// Cross-width narrowing from D153 (fallible): D153 -> D76 / D38.
#[cfg(all(
    any(feature = "d153", feature = "wide"),
    any(feature = "d76", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D76, crate::int::types::Int<4>, D153, crate::int::types::Int<8>);
#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D38, crate::int::types::Int<2>, D153, crate::int::types::Int<8>);

// ─── D76::widen / D153 hop methods ────────────────────────────────────

#[cfg(all(
    any(feature = "d76", feature = "wide"),
    any(feature = "d115", feature = "wide")
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<4>, SCALE> {
    /// Promote to the next storage tier ([`D115`]) at the same
    /// `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<6>, SCALE> {
        self.into()
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<8>, SCALE> {
    /// Demote to the previous storage tier ([`D115`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::Overflow)` if the value
    /// doesn't fit the narrower storage's range at the given scale.
    #[cfg(any(feature = "d115", feature = "wide"))]
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<6>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

/// Scale alias: `D153<0>`. 1 LSB = 1 (512-bit integer ledger).
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s0 = D153<0>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s1 = D153<1>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s2 = D153<2>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s4 = D153<4>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s6 = D153<6>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s9 = D153<9>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s12 = D153<12>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s15 = D153<15>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s18 = D153<18>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s20 = D153<20>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s24 = D153<24>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s28 = D153<28>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s32 = D153<32>;
/// Scale alias: `D153<35>`. 1 LSB = 10^-35 (matches D38 `SCALE_REF`).
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s35 = D153<35>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s38 = D153<38>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s50 = D153<50>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s57 = D153<57>;
/// Scale alias: `D153<75>`. 1 LSB = 10^-75 (wide-scientific midpoint).
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s75 = D153<75>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s76 = D153<76>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s100 = D153<100>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s115 = D153<115>;
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s140 = D153<140>;
/// Scale alias: `D153<150>`. 1 LSB = 10^-150.
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s150 = D153<150>;
/// Scale alias: `D153<152>`. 1 LSB = 10^-152. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s152 = D153<152>;

// ---------------------------------------------------------------------
// D307 — 1024-bit storage (`Int<16>`), scale 0..=307. Deep
// arbitrary-precision tier; gated behind the `d307` / `wide` features.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 1024-bit storage. See [`D38`] for
/// the shape documentation; D307 has the same surface scaled to a
/// 1024-bit signed integer and `MAX_SCALE = 306`. Now a type alias of
/// the unified [`crate::D`] generic decimal type: `D307<S>` is
/// `D<crate::int::types::Int<16>, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<16>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d307` (or umbrella `wide`) Cargo feature. The
/// storage backend is `Int<16>`.
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307<const SCALE: u32> = crate::D<crate::int::types::Int<16>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<16>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<16>, SCALE>`
/// because `D307<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d307", feature = "wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<16>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d307", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D307,
    crate::int::types::Int<16>,
    crate::int::types::Uint<16>,
    crate::int::types::Int<32>,
    crate::int::types::Int<32>,
    crate::int::types::Int<64>,
    crate::int::types::Int<64>,
    crate::int::types::Int<128>,
    crate::int::types::Int<64>,
    wide_trig_d307,
    306,
    16,
    400,
    512
);
// Cross-width widening into D307 (lossless): D76 / D153 -> D307.
#[cfg(all(
    any(feature = "d307", feature = "wide"),
    any(feature = "d76", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D307, crate::int::types::Int<16>, D76, crate::int::types::Int<4>);
#[cfg(all(
    any(feature = "d307", feature = "wide"),
    any(feature = "d153", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D307, crate::int::types::Int<16>, D153, crate::int::types::Int<8>);
// Cross-width narrowing from D307 (fallible): D307 -> D153 / D76.
#[cfg(all(
    any(feature = "d307", feature = "wide"),
    any(feature = "d153", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D153, crate::int::types::Int<8>, D307, crate::int::types::Int<16>);
#[cfg(all(
    any(feature = "d307", feature = "wide"),
    any(feature = "d76", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D76, crate::int::types::Int<4>, D307, crate::int::types::Int<16>);

// ─── D153::widen / D307 hop methods ───────────────────────────────────

#[cfg(all(
    any(feature = "d153", feature = "wide"),
    any(feature = "d230", feature = "wide")
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<8>, SCALE> {
    /// Promote to the next storage tier ([`D230`]) at the same
    /// `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<12>, SCALE> {
        self.into()
    }
}

#[cfg(any(feature = "d307", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<16>, SCALE> {
    /// Demote to the previous storage tier ([`D230`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::Overflow)` if the value
    /// doesn't fit the narrower storage's range at the given scale.
    #[cfg(any(feature = "d230", feature = "wide"))]
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<12>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }

    /// Promote to the next storage tier ([`D462`]) at the same
    /// `SCALE`. Lossless. Requires `d462` / `x-wide`.
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<24>, SCALE> {
        self.into()
    }
}

/// Scale alias: `D307<0>`. 1 LSB = 1 (1024-bit integer ledger).
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s0 = D307<0>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s1 = D307<1>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s2 = D307<2>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s4 = D307<4>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s6 = D307<6>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s9 = D307<9>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s12 = D307<12>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s15 = D307<15>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s18 = D307<18>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s20 = D307<20>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s24 = D307<24>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s28 = D307<28>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s32 = D307<32>;
/// Scale alias: `D307<35>`. 1 LSB = 10^-35 (matches D38 `SCALE_REF`).
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s35 = D307<35>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s38 = D307<38>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s50 = D307<50>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s75 = D307<75>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s100 = D307<100>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s115 = D307<115>;
/// Scale alias: `D307<150>`. 1 LSB = 10^-150.
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s150 = D307<150>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s153 = D307<153>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s200 = D307<200>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s230 = D307<230>;
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s275 = D307<275>;
/// Scale alias: `D307<300>`. 1 LSB = 10^-300.
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s300 = D307<300>;
/// Scale alias: `D307<306>`. 1 LSB = 10^-306. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d307", feature = "wide"))]
pub type D307s306 = D307<306>;

// ─── Half-width and wider tiers (D57 / D115 / D230 / D462 / D616 / D924 / D1232) ───
//
// These fill the (2^n + 2^(n+1))/2 gaps between the existing
// power-of-two storage tiers, plus extend the top end past D307.
// Each tier has the same surface as D76 / D153 / D307: full
// `decl_decimal_full!` emission (every arithmetic / transcendental
// method), plus scale aliases at 0 / mid / max.
//
// Cross-width widening / narrowing methods are emitted to the
// immediate-neighbour tiers only — `D57 ↔ D38`, `D57 ↔ D76`, etc.
// Multi-tier hops go via the chain (e.g. D57 → D76 → D153) at the
// cost of one intermediate.

// ── D57 (192-bit / 3 u64 limbs) ────────────────────────────────────────

/// Scaled fixed-point decimal with 192-bit storage. Half-width tier
/// between D38 and D76 — useful when the D38 i128 ceiling is in
/// reach but D76's 256-bit storage is wasteful. Now a type alias of
/// the unified [`crate::D`] generic decimal type: `D57<S>` is
/// `D<crate::int::types::Int<3>, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<3>` is preserved
/// through the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d57` (or umbrella `wide`) Cargo feature.
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57<const SCALE: u32> = crate::D<crate::int::types::Int<3>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<3>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<3>, SCALE>`
/// because `D57<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<3>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d57", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D57,
    crate::int::types::Int<3>,
    crate::int::types::Uint<3>,
    crate::int::types::Int<6>,
    crate::int::types::Int<6>,
    crate::int::types::Int<8>,
    crate::int::types::Int<16>,
    crate::int::types::Int<32>,
    crate::int::types::Int<16>,
    wide_trig_d57,
    56,
    3,
    100,
    128
);
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s0 = D57<0>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s1 = D57<1>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s2 = D57<2>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s4 = D57<4>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s6 = D57<6>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s9 = D57<9>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s12 = D57<12>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s18 = D57<18>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s20 = D57<20>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s24 = D57<24>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s28 = D57<28>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s32 = D57<32>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s38 = D57<38>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s42 = D57<42>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s48 = D57<48>;
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s52 = D57<52>;
/// Scale alias: `D57<56>`. 1 LSB = 10^-56. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d57", feature = "wide"))]
pub type D57s56 = D57<56>;

// ── D115 (384-bit / 6 u64 limbs) ───────────────────────────────────────

/// Scaled fixed-point decimal with 384-bit storage. Half-width tier
/// between D76 and D153. Now a type alias of the unified [`crate::D`]
/// generic decimal type: `D115<S>` is `D<crate::int::types::Int<6>, S>`.
/// Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<6>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d115` (or umbrella `wide`) Cargo feature.
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115<const SCALE: u32> = crate::D<crate::int::types::Int<6>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<6>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<6>, SCALE>`
/// because `D115<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<6>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d115", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D115,
    crate::int::types::Int<6>,
    crate::int::types::Uint<6>,
    crate::int::types::Int<12>,
    crate::int::types::Int<12>,
    crate::int::types::Int<16>,
    crate::int::types::Int<32>,
    crate::int::types::Int<64>,
    crate::int::types::Int<32>,
    wide_trig_d115,
    114,
    6,
    200,
    512
);
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s0 = D115<0>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s1 = D115<1>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s4 = D115<4>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s8 = D115<8>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s16 = D115<16>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s24 = D115<24>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s32 = D115<32>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s38 = D115<38>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s50 = D115<50>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s57 = D115<57>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s64 = D115<64>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s76 = D115<76>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s90 = D115<90>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s100 = D115<100>;
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s110 = D115<110>;
/// Scale alias: `D115<114>`. 1 LSB = 10^-114. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d115", feature = "wide"))]
pub type D115s114 = D115<114>;

// ── D230 (768-bit / 12 u64 limbs) ──────────────────────────────────────

/// Scaled fixed-point decimal with 768-bit storage. Half-width tier
/// between D153 and D307. Now a type alias of the unified [`crate::D`]
/// generic decimal type: `D230<S>` is `D<crate::int::types::Int<12>, S>`.
/// Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<12>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d230` (or umbrella `wide`) Cargo feature.
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230<const SCALE: u32> = crate::D<crate::int::types::Int<12>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<12>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<12>, SCALE>`
/// because `D230<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<12>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d230", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D230,
    crate::int::types::Int<12>,
    crate::int::types::Uint<12>,
    crate::int::types::Int<24>,
    crate::int::types::Int<24>,
    crate::int::types::Int<48>,
    crate::int::types::Int<48>,
    crate::int::types::Int<96>,
    crate::int::types::Int<48>,
    wide_trig_d230,
    229,
    12,
    400,
    512
);
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s0 = D230<0>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s1 = D230<1>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s6 = D230<6>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s18 = D230<18>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s38 = D230<38>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s57 = D230<57>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s75 = D230<75>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s100 = D230<100>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s115 = D230<115>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s140 = D230<140>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s153 = D230<153>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s175 = D230<175>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s200 = D230<200>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s215 = D230<215>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s225 = D230<225>;
/// Scale alias: `D230<229>`. 1 LSB = 10^-229. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s229 = D230<229>;

// ── D462 (1536-bit / 24 u64 limbs) ─────────────────────────────────────

/// Scaled fixed-point decimal with 1536-bit storage. Half-width tier
/// between D307 and D616. Now a type alias of the unified [`crate::D`]
/// generic decimal type: `D462<S>` is `D<crate::int::types::Int<24>, S>`.
/// Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<24>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d462` (or umbrella `x-wide`) Cargo feature.
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462<const SCALE: u32> = crate::D<crate::int::types::Int<24>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<24>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<24>, SCALE>`
/// because `D462<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d462", feature = "x-wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<24>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d462", feature = "x-wide"))]
crate::macros::full::decl_decimal_full!(
    wide D462,
    crate::int::types::Int<24>,
    crate::int::types::Uint<24>,
    crate::int::types::Int<48>,
    crate::int::types::Int<48>,
    crate::int::types::Int<64>,
    crate::int::types::Int<64>,
    crate::int::types::Int<128>,
    crate::int::types::Int<64>,
    wide_trig_d462,
    461,
    24,
    400,
    512
);
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s0 = D462<0>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s1 = D462<1>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s18 = D462<18>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s38 = D462<38>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s75 = D462<75>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s115 = D462<115>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s153 = D462<153>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s200 = D462<200>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s230 = D462<230>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s275 = D462<275>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s307 = D462<307>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s350 = D462<350>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s400 = D462<400>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s440 = D462<440>;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s460 = D462<460>;
/// Scale alias: `D462<461>`. 1 LSB = 10^-461. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub type D462s461 = D462<461>;

// ── D616 (2048-bit / 32 u64 limbs) ─────────────────────────────────────

/// Scaled fixed-point decimal with 2048-bit storage. New top tier
/// beyond D307; supports correctly-rounded transcendentals at scale
/// up to 616 decimal digits. Now a type alias of the unified
/// [`crate::D`] generic decimal type: `D616<S>` is
/// `D<crate::int::types::Int<32>, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<32>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d616` (or umbrella `x-wide`) Cargo feature.
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616<const SCALE: u32> = crate::D<crate::int::types::Int<32>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<32>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<32>, SCALE>`
/// because `D616<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d616", feature = "x-wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<32>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d616", feature = "x-wide"))]
crate::macros::full::decl_decimal_full!(
    wide D616,
    crate::int::types::Int<32>,
    crate::int::types::Uint<32>,
    crate::int::types::Int<64>,
    crate::int::types::Int<64>,
    crate::int::types::Int<128>,
    crate::int::types::Int<128>,
    crate::int::types::Int<256>,
    crate::int::types::Int<128>,
    wide_trig_d616,
    615,
    32,
    400,
    512
);
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s0 = D616<0>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s1 = D616<1>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s38 = D616<38>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s75 = D616<75>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s115 = D616<115>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s153 = D616<153>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s200 = D616<200>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s230 = D616<230>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s275 = D616<275>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s308 = D616<308>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s380 = D616<380>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s462 = D616<462>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s500 = D616<500>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s555 = D616<555>;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s600 = D616<600>;
/// Scale alias: `D616<615>`. 1 LSB = 10^-615. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub type D616s615 = D616<615>;

// ── D924 (3072-bit / 48 u64 limbs) ─────────────────────────────────────

/// Scaled fixed-point decimal with 3072-bit storage. Half-width tier
/// between D616 and D1232; supports SCALE up to 924 digits. Now a type
/// alias of the unified [`crate::D`] generic decimal type: `D924<S>`
/// is `D<crate::int::types::Int<48>, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<48>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d924` (or umbrella `xx-wide`) Cargo feature.
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924<const SCALE: u32> = crate::D<crate::int::types::Int<48>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<48>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<48>, SCALE>`
/// because `D924<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<48>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d924", feature = "xx-wide"))]
// `no_const_table`: 953-entry `Int<192>` POW10_TABLE build exceeds
// the stable-rust const-eval step budget, so `10^w` is recomputed on
// the stack each call instead of read from a compile-time table.
crate::macros::full::decl_decimal_full!(
    wide D924,
    crate::int::types::Int<48>,
    crate::int::types::Uint<48>,
    crate::int::types::Int<96>,
    crate::int::types::Int<96>,
    crate::int::types::Int<192>,
    crate::int::types::Int<192>,
    crate::int::types::Int<256>,
    crate::int::types::Int<192>,
    wide_trig_d924,
    923,
    48,
    400,
    512,
    no_const_table
);
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s0 = D924<0>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s1 = D924<1>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s75 = D924<75>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s153 = D924<153>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s230 = D924<230>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s307 = D924<307>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s400 = D924<400>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s461 = D924<461>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s462 = D924<462>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s500 = D924<500>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s616 = D924<616>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s700 = D924<700>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s800 = D924<800>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s860 = D924<860>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s900 = D924<900>;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s920 = D924<920>;
/// Scale alias: `D924<923>`. 1 LSB = 10^-923. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub type D924s923 = D924<923>;

// ── D1232 (4096-bit / 64 u64 limbs) ────────────────────────────────────

/// Scaled fixed-point decimal with 4096-bit storage. Widest tier
/// shipped; supports SCALE up to 1232 digits. Now a type alias of the
/// unified [`crate::D`] generic decimal type: `D1232<S>` is
/// `D<crate::int::types::Int<64>, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int<64>` is preserved through
/// the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d1232` (or umbrella `xx-wide`) Cargo feature.
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232<const SCALE: u32> = crate::D<crate::int::types::Int<64>, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int<64>`.
///
/// Implemented on the underlying `crate::D<crate::int::types::Int<64>, SCALE>`
/// because `D1232<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl<const SCALE: u32> Default for crate::D<crate::int::types::Int<64>, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
// `no_const_table`: 1262-entry `Int<256>` POW10_TABLE build exceeds
// the stable-rust const-eval step budget, so `10^w` is recomputed on
// the stack each call instead of read from a compile-time table.
crate::macros::full::decl_decimal_full!(
    wide D1232,
    crate::int::types::Int<64>,
    crate::int::types::Uint<64>,
    crate::int::types::Int<128>,
    crate::int::types::Int<128>,
    crate::int::types::Int<256>,
    crate::int::types::Int<256>,
    crate::int::types::Int<512>,
    crate::int::types::Int<256>,
    wide_trig_d1232,
    1231,
    64,
    400,
    512,
    no_const_table
);
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s0 = D1232<0>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s1 = D1232<1>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s75 = D1232<75>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s153 = D1232<153>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s230 = D1232<230>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s307 = D1232<307>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s461 = D1232<461>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s616 = D1232<616>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s700 = D1232<700>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s800 = D1232<800>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s900 = D1232<900>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s924 = D1232<924>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s1000 = D1232<1000>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s1100 = D1232<1100>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s1180 = D1232<1180>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s1220 = D1232<1220>;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s1230 = D1232<1230>;
/// Scale alias: `D1232<1231>`. 1 LSB = 10^-1231. Maximum supported scale
/// (v0.4.0 cap: `MAX_SCALE = name - 1`).
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub type D1232s1231 = D1232<1231>;

// ─── Cross-tier next-neighbour widen/narrow chain ─────────────────────
//
// The historical .widen() / .narrow() methods on D38/D76/D153/D307
// follow the power-of-two storage sequence (D38→D76→D153→D307). The
// 0.2.6 tier ladder fills in half-widths between each pair plus
// extends to D1232; the complete ladder is:
//
//   D18 → D38 → D57 → D76 → D115 → D153 → D230 → D307 →
//   D462 → D616 → D924 → D1232
//
// The next-neighbour .widen() / .narrow() methods on the new tiers go
// to the immediate adjacent rung (D57.widen() → D76, D76.widen()
// already returns D153 which is the existing power-of-two next-up,
// etc.). The cross-tier From / TryFrom impls below cover the
// neighbour pairs that weren't already declared by the legacy
// D38/D76/D153/D307 blocks.
//
// Coverage strategy: declare every NEW adjacent pair both ways. The
// existing legacy declarations (D18/D18/D38↔D76, D38/D76↔D153,
// D76/D153↔D307) stay where they are; this block adds the conversions
// that hop through the new tiers (D38↔D57, D57↔D76, D76↔D115, etc.).

// D38 ↔ D57
#[cfg(any(feature = "d57", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D57, crate::int::types::Int<3>, D38, crate::int::types::Int<2>);
#[cfg(any(feature = "d57", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D38, crate::int::types::Int<2>, D57, crate::int::types::Int<3>);

// D57 ↔ D76
#[cfg(all(
    any(feature = "d57", feature = "wide"),
    any(feature = "d76", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D76, crate::int::types::Int<4>, D57, crate::int::types::Int<3>);
#[cfg(all(
    any(feature = "d57", feature = "wide"),
    any(feature = "d76", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D57, crate::int::types::Int<3>, D76, crate::int::types::Int<4>);

// D76 ↔ D115
#[cfg(all(
    any(feature = "d76", feature = "wide"),
    any(feature = "d115", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D115, crate::int::types::Int<6>, D76, crate::int::types::Int<4>);
#[cfg(all(
    any(feature = "d76", feature = "wide"),
    any(feature = "d115", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D76, crate::int::types::Int<4>, D115, crate::int::types::Int<6>);

// D115 ↔ D153
#[cfg(all(
    any(feature = "d115", feature = "wide"),
    any(feature = "d153", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D153, crate::int::types::Int<8>, D115, crate::int::types::Int<6>);
#[cfg(all(
    any(feature = "d115", feature = "wide"),
    any(feature = "d153", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D115, crate::int::types::Int<6>, D153, crate::int::types::Int<8>);

// D153 ↔ D230
#[cfg(all(
    any(feature = "d153", feature = "wide"),
    any(feature = "d230", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D230, crate::int::types::Int<12>, D153, crate::int::types::Int<8>);
#[cfg(all(
    any(feature = "d153", feature = "wide"),
    any(feature = "d230", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D153, crate::int::types::Int<8>, D230, crate::int::types::Int<12>);

// D230 ↔ D307
#[cfg(all(
    any(feature = "d230", feature = "wide"),
    any(feature = "d307", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D307, crate::int::types::Int<16>, D230, crate::int::types::Int<12>);
#[cfg(all(
    any(feature = "d230", feature = "wide"),
    any(feature = "d307", feature = "wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D230, crate::int::types::Int<12>, D307, crate::int::types::Int<16>);

// D307 ↔ D462
#[cfg(all(
    any(feature = "d307", feature = "wide"),
    any(feature = "d462", feature = "x-wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D462, crate::int::types::Int<24>, D307, crate::int::types::Int<16>);
#[cfg(all(
    any(feature = "d307", feature = "wide"),
    any(feature = "d462", feature = "x-wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D307, crate::int::types::Int<16>, D462, crate::int::types::Int<24>);

// D462 ↔ D616
#[cfg(all(
    any(feature = "d462", feature = "x-wide"),
    any(feature = "d616", feature = "x-wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D616, crate::int::types::Int<32>, D462, crate::int::types::Int<24>);
#[cfg(all(
    any(feature = "d462", feature = "x-wide"),
    any(feature = "d616", feature = "x-wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D462, crate::int::types::Int<24>, D616, crate::int::types::Int<32>);

// D616 ↔ D924
#[cfg(all(
    any(feature = "d616", feature = "x-wide"),
    any(feature = "d924", feature = "xx-wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D924, crate::int::types::Int<48>, D616, crate::int::types::Int<32>);
#[cfg(all(
    any(feature = "d616", feature = "x-wide"),
    any(feature = "d924", feature = "xx-wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D616, crate::int::types::Int<32>, D924, crate::int::types::Int<48>);

// D924 ↔ D1232
#[cfg(all(
    any(feature = "d924", feature = "xx-wide"),
    any(feature = "d1232", feature = "xx-wide")
))]
crate::macros::conversions::decl_cross_width_widening!(wide D1232, crate::int::types::Int<64>, D924, crate::int::types::Int<48>);
#[cfg(all(
    any(feature = "d924", feature = "xx-wide"),
    any(feature = "d1232", feature = "xx-wide")
))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D924, crate::int::types::Int<48>, D1232, crate::int::types::Int<64>);

// .widen() / .narrow() methods on the new tiers — each points at the
// IMMEDIATE neighbour in the comprehensive ladder above. The legacy
// .widen() / .narrow() on D38/D76/D153/D307 are unchanged (still go
// to the power-of-two next-up) for source compatibility; users who
// want to traverse through the half-widths should use the methods
// declared here, or the From / TryFrom impls directly.

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<3>, SCALE> {
    /// Demote to the immediate previous tier ([`D38`]) at the same `SCALE`.
    /// Returns `Err(ConvertError::Overflow)` if the value exceeds `i128` range.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<2>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

// `widen` lives in a neighbour-gated impl: D57 can be enabled without
// D76 (e.g. `--features d57`), in which case D76 doesn't exist as a
// type and an unconditional `widen` would not compile.
#[cfg(all(
    any(feature = "d57", feature = "wide"),
    any(feature = "d76", feature = "wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<3>, SCALE> {
    /// Promote to the next storage tier ([`D76`]) at the same `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<4>, SCALE> {
        self.into()
    }
}

// Each gap tier's `narrow` / `widen` lives in a neighbour-gated impl:
// a single-tier build (e.g. `--features d115`) enables neither the
// lower nor the upper neighbour, so referencing those types from an
// unconditional method would not compile. (Same pattern as the
// D616 → D924 split below.)
#[cfg(all(
    any(feature = "d115", feature = "wide"),
    any(feature = "d76", feature = "wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<6>, SCALE> {
    /// Demote to the immediate previous tier ([`D76`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<4>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

#[cfg(all(
    any(feature = "d115", feature = "wide"),
    any(feature = "d153", feature = "wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<6>, SCALE> {
    /// Promote to the next storage tier ([`D153`]) at the same `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<8>, SCALE> {
        self.into()
    }
}

#[cfg(all(
    any(feature = "d230", feature = "wide"),
    any(feature = "d153", feature = "wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<12>, SCALE> {
    /// Demote to the immediate previous tier ([`D153`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<8>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

#[cfg(all(
    any(feature = "d230", feature = "wide"),
    any(feature = "d307", feature = "wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<12>, SCALE> {
    /// Promote to the next storage tier ([`D307`]) at the same `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<16>, SCALE> {
        self.into()
    }
}

#[cfg(all(
    any(feature = "d462", feature = "x-wide"),
    any(feature = "d307", feature = "wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<24>, SCALE> {
    /// Demote to the immediate previous tier ([`D307`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<16>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

#[cfg(all(
    any(feature = "d462", feature = "x-wide"),
    any(feature = "d616", feature = "x-wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<24>, SCALE> {
    /// Promote to the next storage tier ([`D616`]) at the same `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<32>, SCALE> {
        self.into()
    }
}

#[cfg(all(
    any(feature = "d616", feature = "x-wide"),
    any(feature = "d462", feature = "x-wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<32>, SCALE> {
    /// Demote to the immediate previous tier ([`D462`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<24>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

// `widen` lives in a second impl gated on D924's feature — D616 can
// be enabled without xx-wide (docs.rs builds this case), in which
// case D924 doesn't exist as a type and the unconditional `widen`
// method above breaks the doc build.
#[cfg(all(
    any(feature = "d616", feature = "x-wide"),
    any(feature = "d924", feature = "xx-wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<32>, SCALE> {
    /// Promote to the next storage tier ([`D924`]) at the same `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<48>, SCALE> {
        self.into()
    }
}

#[cfg(all(
    any(feature = "d924", feature = "xx-wide"),
    any(feature = "d616", feature = "x-wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<48>, SCALE> {
    /// Demote to the immediate previous tier ([`D616`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<32>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

#[cfg(all(
    any(feature = "d924", feature = "xx-wide"),
    any(feature = "d1232", feature = "xx-wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<48>, SCALE> {
    /// Promote to the next storage tier ([`D1232`]) at the same `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> crate::D<crate::int::types::Int<64>, SCALE> {
        self.into()
    }
}

#[cfg(all(
    any(feature = "d1232", feature = "xx-wide"),
    any(feature = "d924", feature = "xx-wide"),
))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<64>, SCALE> {
    /// Demote to the immediate previous tier ([`D924`]) at the same `SCALE`.
    /// D1232 is the widest shipped tier, so there is no `.widen()` method.
    #[inline]
    pub fn narrow(self) -> Result<crate::D<crate::int::types::Int<48>, SCALE>, crate::support::error::ConvertError> {
        self.try_into()
    }
}

// ─── Const-generic width sugar: `widen_n` / `narrow_n` ─────────────────
//
// Direct decimal-level mirror of the int-layer const base
// (`Int::resize_n` / `Int::try_narrow`, story 1.2.1): one const-generic
// pair on `D<Int<N>, SCALE>` that hops to ANY target width `M` in a
// single call, at the SAME scale (a pure width conversion — exact, no
// `RoundingMode`). Both delegate straight to the int const base, so they
// are usable in `const` context.
//
// Named with the `_n` suffix — NOT plain `widen` / `narrow` — for the
// same reason `Int::resize_n` carries it: the per-width tiers above
// already define inherent `widen(self)` / `narrow(self)` (no turbofish,
// single-tier hops kept for source compatibility), and a second inherent
// method of the same name on the aliased `D<Int<N>, SCALE>` would be a
// duplicate definition (E0592). The `_n` const-generic methods compose
// freely with those: `widen()` is `widen_n::<NEIGHBOUR>()`.
impl<const N: usize, const SCALE: u32> crate::D<crate::int::types::Int<N>, SCALE> {
    /// Widen to a wider storage `Int<M>` (`M >= N`) at the same `SCALE`.
    /// Sign-extends; always lossless. `const`.
    ///
    /// Mirror of [`crate::int::types::Int::widen`] lifted to the decimal
    /// wrapper: the logical value is unchanged, only the storage width
    /// grows. Use [`Self::narrow_n`] for the fallible reverse hop and the
    /// existing per-tier [`D38::widen`]-style methods for single-tier
    /// neighbour hops.
    #[inline]
    #[must_use]
    pub const fn widen_n<const M: usize>(self) -> crate::D<crate::int::types::Int<M>, SCALE> {
        debug_assert!(M >= N, "widen_n requires M >= N");
        crate::D(self.0.resize_n::<M>())
    }

    /// Narrow to a narrower storage `Int<M>` (`1 <= M <= N`) at the same
    /// `SCALE`. Returns `None` when the value does not fit `Int<M>` as
    /// two's complement. `const`.
    ///
    /// Mirror of [`crate::int::types::Int::narrow`] lifted to the decimal
    /// wrapper. The narrowest decimal storage is `Int<1>` (D18), so a
    /// `narrow_n::<0>()` is meaningless and is rejected by the int base's
    /// `1 <= M` debug-assert.
    #[inline]
    pub const fn narrow_n<const M: usize>(
        self,
    ) -> Option<crate::D<crate::int::types::Int<M>, SCALE>> {
        match self.0.try_narrow::<M>() {
            Some(raw) => Some(crate::D(raw)),
            None => None,
        }
    }
}

// ─── Cross-scale-op constructors + comparators ─────────────────────────
//
// One invocation per width emits `mul_of`, `add_of`, `sub_of`, `div_of`,
// `rem_of`, `max_of`, `min_of`, `clamp_of`, `cmp_of`, `eq_of`, `ne_of`,
// `lt_of`, `le_of`, `gt_of`, `ge_of` (plus the `_with(mode)` siblings
// for the constructors that involve a possibly-lossy rescale of inputs).
// Operands of any width ≤ Self's storage are accepted via the
// `WidthLE` bound; operands of any SCALE are accepted via the
// const-generic `S1` / `S2` parameters. See
// `crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops` for
// the body.

crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D18, crate::int::types::Int<1>);
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D38, crate::int::types::Int<2>);

#[cfg(any(feature = "d57", feature = "wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D57, crate::int::types::Int<3>);

#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D76, crate::int::types::Int<4>);

#[cfg(any(feature = "d115", feature = "wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D115, crate::int::types::Int<6>);

#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D153, crate::int::types::Int<8>);

#[cfg(any(feature = "d230", feature = "wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D230, crate::int::types::Int<12>);

#[cfg(any(feature = "d307", feature = "wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D307, crate::int::types::Int<16>);

#[cfg(any(feature = "d462", feature = "x-wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D462, crate::int::types::Int<24>);

#[cfg(any(feature = "d616", feature = "x-wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D616, crate::int::types::Int<32>);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D924, crate::int::types::Int<48>);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
crate::macros::cross_scale_ops::decl_decimal_cross_scale_ops!(D1232, crate::int::types::Int<64>);

#[cfg(test)]
mod tests {
    use super::*;

    /// `from_bits` / `to_bits` round-trip is exact.
    #[test]
    fn from_bits_to_bits_round_trip() {
        let raw: i128 = 1_500_000_000_000;
        let v: D38s12 = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(raw));
        assert_eq!(v.to_bits(), raw);
    }

    /// `ZERO` has raw bit value 0.
    #[test]
    fn zero_is_zero_bits() {
        assert_eq!(D38s12::ZERO.to_bits(), 0);
    }

    /// Two instances with identical raw bits compare equal.
    #[test]
    fn equal_by_underlying_bits() {
        assert_eq!(
            D38s12::from_bits(crate::int::types::Int::<2>::from_i128(42_000_000_000_000)),
            D38s12::from_bits(crate::int::types::Int::<2>::from_i128(42_000_000_000_000))
        );
        assert_ne!(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(42)), D38s12::from_bits(crate::int::types::Int::<2>::from_i128(43)));
    }

    /// Ord is derived from i128: smaller bits compare less.
    #[test]
    fn ord_by_underlying_bits() {
        assert!(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1)) < D38s12::from_bits(crate::int::types::Int::<2>::from_i128(2)));
        assert!(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1)) < D38s12::from_bits(crate::int::types::Int::<2>::from_i128(0)));
    }

    /// `multiplier()` returns 10^SCALE. At SCALE = 12 that is 10^12.
    #[test]
    fn multiplier_is_ten_to_scale() {
        assert_eq!(D38s12::multiplier(), 1_000_000_000_000_i128);
    }

    /// `SCALE` associated const returns the const-generic scale.
    #[test]
    fn scale_const_matches_type_parameter() {
        assert_eq!(D38s12::SCALE, 12);
        const N: u32 = D38s12::SCALE;
        assert_eq!(N, 12);
    }

    /// `scale()` method returns the const-generic scale and is
    /// independent of the instance's value.
    #[test]
    fn scale_method_matches_type_parameter() {
        assert_eq!(D38s12::ZERO.scale(), 12);
        assert_eq!(D38s12::ONE.scale(), 12);
        assert_eq!(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(i128::MAX)).scale(), 12);
        assert_eq!(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-7)).scale(), 12);
    }

    /// Both forms agree at non-default scales.
    #[test]
    fn scale_at_other_scales() {
        type D6 = crate::D<crate::int::types::Int<2>, 6>;
        type D0 = crate::D<crate::int::types::Int<2>, 0>;
        type D38 = crate::D<crate::int::types::Int<2>, 38>;
        assert_eq!(D6::SCALE, 6);
        assert_eq!(D0::SCALE, 0);
        assert_eq!(D38::SCALE, 38);
        assert_eq!(D6::ZERO.scale(), 6);
        assert_eq!(D0::ZERO.scale(), 0);
        assert_eq!(D38::ZERO.scale(), 38);
    }

    /// `ONE` has bit pattern 10^SCALE so that the logical value is 1.
    #[test]
    fn one_has_scaled_bit_pattern() {
        assert_eq!(D38s12::ONE.to_bits(), 1_000_000_000_000_i128);
    }

    /// `MAX` is `i128::MAX`.
    #[test]
    fn max_is_i128_max() {
        assert_eq!(D38s12::MAX.to_bits(), i128::MAX);
    }

    /// `MIN` is `i128::MIN`.
    #[test]
    fn min_is_i128_min() {
        assert_eq!(D38s12::MIN.to_bits(), i128::MIN);
    }

    /// `ONE` is not equal to `ZERO`.
    #[test]
    fn one_is_not_zero() {
        assert_ne!(D38s12::ONE, D38s12::ZERO);
        assert!(D38s12::ONE.is_positive());
    }

    /// `multiplier()` works correctly at non-default scales.
    #[test]
    fn multiplier_at_other_scales() {
        type D6 = crate::D<crate::int::types::Int<2>, 6>;
        assert_eq!(D6::multiplier(), 1_000_000_i128);
        assert_eq!(D6::ONE.to_bits(), 1_000_000_i128);

        type D0 = crate::D<crate::int::types::Int<2>, 0>;
        assert_eq!(D0::multiplier(), 1_i128);
        assert_eq!(D0::ONE.to_bits(), 1_i128);
    }

    // ----- D18 sanity tests -----

    #[test]
    fn d18_basics() {
        assert_eq!(super::D18s9::ZERO.to_bits(), 0_i64);
        assert_eq!(super::D18s9::ONE.to_bits(), 1_000_000_000_i64);
        assert_eq!(super::D18s9::multiplier(), 1_000_000_000_i64);
        assert_eq!(super::D18s9::SCALE, 9);
    }

    #[test]
    fn d18_arithmetic() {
        let a = super::D18s9::from_bits(crate::int::types::Int::<1>::from_i64(1_500_000_000)); // 1.5
        let b = super::D18s9::from_bits(crate::int::types::Int::<1>::from_i64(2_500_000_000)); // 2.5
        assert_eq!((a + b).to_bits(), 4_000_000_000);
        assert_eq!((b - a).to_bits(), 1_000_000_000);
        assert_eq!((-a).to_bits(), -1_500_000_000);

        let x = super::D18s9::from_bits(crate::int::types::Int::<1>::from_i64(2_000_000_000)); // 2.0
        let y = super::D18s9::from_bits(crate::int::types::Int::<1>::from_i64(3_000_000_000)); // 3.0
        assert_eq!((x * y).to_bits(), 6_000_000_000);
        assert_eq!((y / x).to_bits(), 1_500_000_000);
        assert_eq!((y % x).to_bits(), 1_000_000_000);
    }

    #[test]
    fn d18_display() {
        let v: super::D18s9 = super::D18s9::from_bits(crate::int::types::Int::<1>::from_i64(1_500_000_000)); // 1.500000000
        assert_eq!(alloc::format!("{}", v), "1.500000000");
        let neg: super::D18s9 = super::D18s9::from_bits(crate::int::types::Int::<1>::from_i64(-1_500_000_000));
        assert_eq!(alloc::format!("{}", neg), "-1.500000000");
    }

    #[test]
    fn cross_width_widening_d18_to_d38() {
        let mid: super::D18s9 = super::D18s9::from_bits(crate::int::types::Int::<1>::from_i64(i64::MAX));
        let wider: super::D38s9 = mid.into();
        assert_eq!(wider.to_bits(), i64::MAX as i128);
    }

    #[test]
    fn cross_width_narrowing_d38_to_d18_in_range() {
        let wide: super::D38s9 = super::D38s9::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000));
        let narrow: super::D18s9 = wide.try_into().unwrap();
        assert_eq!(narrow.to_bits(), 1_500_000_000);
    }

    #[test]
    fn cross_width_narrowing_d38_to_d18_out_of_range() {
        let wide: super::D38s9 = super::D38s9::from_bits(crate::int::types::Int::<2>::from_i128(i128::MAX));
        let narrow: Result<super::D18s9, _> = wide.try_into();
        assert!(narrow.is_err());
    }

    #[test]
    fn d18_from_str() {
        use core::str::FromStr;
        let v = super::D18s9::from_str("1.500000000").unwrap();
        assert_eq!(v.to_bits(), 1_500_000_000);
        let neg = super::D18s9::from_str("-1.500000000").unwrap();
        assert_eq!(neg.to_bits(), -1_500_000_000);
    }

    #[test]
    fn d18_consts() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        use crate::types::consts::DecimalConstants;
        type D18s12 = crate::D<crate::int::types::Int<1>, 12>;
        // pi at scale 12 = 3.141592653590 (matches D38s12).
        assert_eq!(D18s12::pi().to_bits(), 3_141_592_653_590);
        // tau at scale 12 = 6.283185307180.
        assert_eq!(D18s12::tau().to_bits(), 6_283_185_307_180);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_basics() {
        use crate::types::traits::arithmetic::DecimalArithmetic;
        use crate::int::types::Int;
        assert_eq!(
            super::D76s2::ZERO.to_bits(),
            Int::<4>::from_str_radix("0", 10).unwrap()
        );
        assert_eq!(
            super::D76s2::ONE.to_bits(),
            Int::<4>::from_str_radix("100", 10).unwrap()
        );
        assert_eq!(super::D76s2::MAX.to_bits(), Int::<4>::MAX);
        assert_eq!(super::D76s2::MIN.to_bits(), Int::<4>::MIN);
        assert_eq!(
            super::D76s2::multiplier(),
            Int::<4>::from_str_radix("100", 10).unwrap()
        );
        assert_eq!(super::D76s2::SCALE, 2);
        assert_eq!(super::D76s2::ZERO.scale(), 2);
        // SCALE = 75 (new MAX_SCALE) multiplier is 10^75, well within 256-bit range.
        let m75 = super::D76s75::multiplier();
        assert_eq!(
            m75,
            Int::<4>::from_str_radix(
                "1000000000000000000000000000000000000000000000000000000000000000000000000000",
                10
            )
            .unwrap()
        );
        assert_eq!(<super::D76s12 as DecimalArithmetic>::MAX_SCALE, 75);
        // round-trip
        let raw = Int::<4>::from_str_radix("123456789012345678901234567890", 10).unwrap();
        assert_eq!(super::D76s12::from_bits(raw).to_bits(), raw);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_arithmetic() {
        type D = crate::D<crate::int::types::Int<4>, 12>;
        let one = D::ONE;
        let two = D::from_bits(D::multiplier() + D::multiplier());
        let three =
            D::from_bits(D::multiplier() * crate::int::types::Int::<4>::from_str_radix("3", 10).unwrap());
        // add / sub / neg
        assert_eq!((one + two), three);
        assert_eq!((three - one), two);
        assert_eq!((-one).to_bits(), -D::multiplier());
        // mul: 2 * 3 == 6
        let six =
            D::from_bits(D::multiplier() * crate::int::types::Int::<4>::from_str_radix("6", 10).unwrap());
        assert_eq!((two * three), six);
        // div: 6 / 2 == 3
        assert_eq!((six / two), three);
        // rem: 6 % 2 == 0 (storage-level remainder)
        assert_eq!((six % two), D::ZERO);
        // assign forms
        let mut v = one;
        v += two;
        assert_eq!(v, three);
        v *= two;
        assert_eq!(v, six);
        v /= two;
        assert_eq!(v, three);
        v -= one;
        assert_eq!(v, two);
        v %= two;
        assert_eq!(v, D::ZERO);
        // fractional: 1.5 * 1.5 == 2.25 at scale 12
        let half =
            D::from_bits(D::multiplier() / crate::int::types::Int::<4>::from_str_radix("2", 10).unwrap());
        let one_and_half = one + half;
        let product = one_and_half * one_and_half;
        let expected = D::from_bits(
            D::multiplier() * crate::int::types::Int::<4>::from_str_radix("2", 10).unwrap()
                + D::multiplier() / crate::int::types::Int::<4>::from_str_radix("4", 10).unwrap(),
        );
        assert_eq!(product, expected);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_display() {
        type D = crate::D<crate::int::types::Int<4>, 12>;
        let one = D::ONE;
        assert_eq!(alloc::format!("{}", one), "1.000000000000");
        assert_eq!(alloc::format!("{}", -one), "-1.000000000000");
        assert_eq!(alloc::format!("{}", D::ZERO), "0.000000000000");
        let half =
            D::from_bits(D::multiplier() / crate::int::types::Int::<4>::from_str_radix("2", 10).unwrap());
        assert_eq!(alloc::format!("{}", half), "0.500000000000");
        assert_eq!(alloc::format!("{:?}", one), "D76<12>(1.000000000000)");
        // scale 0 prints no fractional part
        let int_only: crate::D<crate::int::types::Int<4>, 0> = crate::D::<crate::int::types::Int<4>, 0>::ONE;
        assert_eq!(alloc::format!("{}", int_only), "1");
        // very large magnitude near the 75-digit ceiling (new MAX_SCALE)
        let big = super::D76s75::MAX;
        let s = alloc::format!("{}", big);
        assert!(s.starts_with("57.8960446"));
        assert_eq!(s.len(), "57.".len() + 75);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_sign_and_helpers() {
        type D = crate::D<crate::int::types::Int<4>, 6>;
        let neg = -D::ONE;
        assert!(neg.is_negative());
        assert!(D::ONE.is_positive());
        assert!(!D::ZERO.is_positive());
        assert_eq!(neg.abs(), D::ONE);
        assert_eq!(D::ONE.signum(), D::ONE);
        assert_eq!(neg.signum(), neg);
        assert_eq!(D::ZERO.signum(), D::ZERO);
        // min / max / clamp
        let two = D::ONE + D::ONE;
        assert_eq!(D::ONE.min(two), D::ONE);
        assert_eq!(D::ONE.max(two), two);
        assert_eq!(two.clamp(D::ZERO, D::ONE), D::ONE);
        // copysign
        assert_eq!(D::ONE.copysign(neg), neg);
        assert_eq!(neg.copysign(D::ONE), D::ONE);
        // recip: 1/2 at scale 6
        let half =
            D::from_bits(D::multiplier() / crate::int::types::Int::<4>::from_str_radix("2", 10).unwrap());
        assert_eq!(two.recip(), half);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_overflow_variants() {
        type D = crate::D<crate::int::types::Int<4>, 2>;
        // checked_add overflow at MAX
        assert_eq!(D::MAX.checked_add(D::ONE), None);
        assert_eq!(D::ONE.checked_add(D::ONE), Some(D::ONE + D::ONE));
        // saturating
        assert_eq!(D::MAX.saturating_add(D::ONE), D::MAX);
        assert_eq!(D::MIN.saturating_sub(D::ONE), D::MIN);
        // checked_neg of MIN overflows
        assert_eq!(D::MIN.checked_neg(), None);
        assert_eq!(D::ONE.checked_neg(), Some(-D::ONE));
        // checked_mul / checked_div
        let two = D::ONE + D::ONE;
        let three = two + D::ONE;
        assert_eq!(
            two.checked_mul(three),
            Some(D::from_bits(
                D::multiplier() * crate::int::types::Int::<4>::from_str_radix("6", 10).unwrap()
            ))
        );
        assert_eq!(D::ONE.checked_div(D::ZERO), None);
        assert_eq!((three).checked_div(D::ONE), Some(three));
        // wrapping_add of one storage LSB at MAX wraps around to MIN.
        let one_lsb = D::from_bits(crate::int::types::Int::<4>::from_str_radix("1", 10).unwrap());
        assert_eq!(D::MAX.wrapping_add(one_lsb), D::MIN);
        // overflowing
        assert_eq!(D::ONE.overflowing_add(D::ONE), (two, false));
        assert_eq!(D::MAX.overflowing_add(D::ONE).1, true);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_consts_and_from_str() {
        use crate::types::consts::DecimalConstants;
        use core::str::FromStr;
        // pi at scale 12 matches the D38 reference.
        assert_eq!(
            crate::D::<crate::int::types::Int<4>, 12>::pi().to_bits(),
            crate::int::types::Int::<4>::from_str_radix("3141592653590", 10).unwrap()
        );
        assert_eq!(
            crate::D::<crate::int::types::Int<4>, 4>::e().to_bits(),
            crate::int::types::Int::<4>::from_str_radix("27183", 10).unwrap()
        );
        // FromStr within i128 range
        let v = crate::D::<crate::int::types::Int<4>, 2>::from_str("1.50").unwrap();
        assert_eq!(
            v.to_bits(),
            crate::int::types::Int::<4>::from_str_radix("150", 10).unwrap()
        );
        let neg = crate::D::<crate::int::types::Int<4>, 2>::from_str("-20.50").unwrap();
        assert_eq!(
            neg.to_bits(),
            crate::int::types::Int::<4>::from_str_radix("-2050", 10).unwrap()
        );
        // num_traits Zero / One
        use ::num_traits::{One, Zero};
        assert!(crate::D::<crate::int::types::Int<4>, 6>::zero().is_zero());
        assert!(crate::D::<crate::int::types::Int<4>, 6>::one().is_one());
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_conversions() {
        use crate::int::types::Int;
        type D = crate::D<crate::int::types::Int<4>, 6>;
        // From<primitive int>
        let from_i32: D = 5i32.into();
        assert_eq!(
            from_i32.to_bits(),
            Int::<4>::from_str_radix("5000000", 10).unwrap()
        );
        let from_u64: D = 7u64.into();
        assert_eq!(
            from_u64.to_bits(),
            Int::<4>::from_str_radix("7000000", 10).unwrap()
        );
        let from_neg: D = (-3i16).into();
        assert_eq!(
            from_neg.to_bits(),
            Int::<4>::from_str_radix("-3000000", 10).unwrap()
        );
        // TryFrom<i128> / TryFrom<u128>
        let from_i128 = D::try_from(123i128).unwrap();
        assert_eq!(
            from_i128.to_bits(),
            Int::<4>::from_str_radix("123000000", 10).unwrap()
        );
        let from_u128 = D::try_from(u128::MAX).unwrap();
        assert_eq!(
            from_u128.to_bits(),
            Int::<4>::from_str_radix("340282366920938463463374607431768211455", 10).unwrap()
                * Int::<4>::from_str_radix("1000000", 10).unwrap()
        );
        // TryFrom<f64>
        let from_f64 = D::try_from(2.5f64).unwrap();
        assert_eq!(
            from_f64.to_bits(),
            Int::<4>::from_str_radix("2500000", 10).unwrap()
        );
        assert!(D::try_from(f64::NAN).is_err());
        // from_int / from_i32
        assert_eq!(D::from_int(9i128), D::from(9i32));
        assert_eq!(D::from_i32(-4), D::from(-4i32));
        // to_int: 2.5 with HalfToEven -> 2
        use crate::support::rounding::RoundingMode;
        let two_and_half = D::from_bits(Int::<4>::from_str_radix("2500000", 10).unwrap());
        assert_eq!(two_and_half.to_int_with(RoundingMode::HalfToEven), 2);
        assert_eq!(two_and_half.to_int_with(RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(two_and_half.to_int_with(RoundingMode::Ceiling), 3);
        assert_eq!(two_and_half.to_int_with(RoundingMode::Floor), 2);
        let neg_two_and_half = -two_and_half;
        assert_eq!(neg_two_and_half.to_int_with(RoundingMode::Floor), -3);
        assert_eq!(neg_two_and_half.to_int_with(RoundingMode::Trunc), -2);
        // cross-width widening D38 -> D76 (lossless)
        let d38: super::D38s6 = super::D38s6::from_bits(crate::int::types::Int::<2>::from_i128(-150));
        let widened: crate::D<crate::int::types::Int<4>, 6> = d38.into();
        assert_eq!(widened.to_bits(), Int::<4>::from_str_radix("-150", 10).unwrap());
        // cross-width narrowing D76 -> D38 in range
        let in_range: crate::D<crate::int::types::Int<4>, 6> =
            crate::D::<crate::int::types::Int<4>, 6>::from_bits(Int::<4>::from_str_radix("999", 10).unwrap());
        let narrowed: super::D38s6 = in_range.try_into().unwrap();
        assert_eq!(narrowed.to_bits(), 999i128);
        // cross-width narrowing D76 -> D38 out of range
        let out_of_range = super::D76s75::MAX;
        let narrow_fail: Result<crate::D<crate::int::types::Int<2>, 75>, _> = out_of_range.try_into();
        assert!(narrow_fail.is_err());
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_rescale_rounding_floats() {
        use crate::support::rounding::RoundingMode;
        use crate::int::types::Int;
        type D6 = crate::D<crate::int::types::Int<4>, 6>;
        // rescale up (lossless): scale 6 -> scale 9
        let v = D6::from_bits(Int::<4>::from_str_radix("1500000", 10).unwrap()); // 1.5
        let up: crate::D<crate::int::types::Int<4>, 9> = v.rescale::<9>();
        assert_eq!(
            up.to_bits(),
            Int::<4>::from_str_radix("1500000000", 10).unwrap()
        );
        // rescale down (lossy, HalfToEven): scale 6 -> scale 2
        let down: crate::D<crate::int::types::Int<4>, 2> = v.rescale::<2>();
        assert_eq!(down.to_bits(), Int::<4>::from_str_radix("150", 10).unwrap());
        // rescale down with explicit mode: 2.5 (scale 0 representation) ...
        let two_p_five = crate::D::<crate::int::types::Int<4>, 1>::from_bits(Int::<4>::from_str_radix("25", 10).unwrap());
        let r0: crate::D<crate::int::types::Int<4>, 0> = two_p_five.rescale_with::<0>(RoundingMode::HalfToEven);
        assert_eq!(r0.to_bits(), Int::<4>::from_str_radix("2", 10).unwrap());
        let r0b: crate::D<crate::int::types::Int<4>, 0> = two_p_five.rescale_with::<0>(RoundingMode::HalfAwayFromZero);
        assert_eq!(r0b.to_bits(), Int::<4>::from_str_radix("3", 10).unwrap());
        // floor / ceil / round / trunc / fract on 1.5 at scale 6
        assert_eq!(v.floor(), D6::ONE);
        assert_eq!(v.ceil(), D6::ONE + D6::ONE);
        assert_eq!(v.round(), D6::ONE + D6::ONE); // half away from zero
        assert_eq!(v.trunc(), D6::ONE);
        assert_eq!(
            v.fract(),
            D6::from_bits(Int::<4>::from_str_radix("500000", 10).unwrap())
        );
        // negative: -1.5
        let neg = -v;
        assert_eq!(neg.floor(), -(D6::ONE + D6::ONE));
        assert_eq!(neg.ceil(), -D6::ONE);
        assert_eq!(neg.round(), -(D6::ONE + D6::ONE));
        // float bridge
        let from_f = D6::from_f64(2.5);
        assert_eq!(
            from_f.to_bits(),
            Int::<4>::from_str_radix("2500000", 10).unwrap()
        );
        assert_eq!(D6::from_f64(f64::NAN), D6::ZERO);
        assert_eq!(D6::from_f64(f64::INFINITY), D6::MAX);
        let round_trip = D6::ONE.to_f64();
        assert!((round_trip - 1.0).abs() < 1e-9);
    }

    #[cfg(any(feature = "d153", feature = "wide"))]
    #[test]
    fn d153_smoke() {
        use crate::types::traits::arithmetic::DecimalArithmetic;
        use crate::int::types::Int;
        type D = crate::D<crate::int::types::Int<8>, 35>;
        assert_eq!(<D as DecimalArithmetic>::MAX_SCALE, 152);
        assert_eq!(D::ZERO.to_bits(), Int::<8>::from_str_radix("0", 10).unwrap());
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::from_int(6i128));
        assert_eq!((three * two) / two, three);
        assert_eq!(alloc::format!("{}", one).len(), "1.".len() + 35);
        assert_eq!(D::from_int(5i128).to_int(), 5);
        // rescale across the wide range
        let up: crate::D<crate::int::types::Int<8>, 150> = one.rescale::<150>();
        assert_eq!(up, crate::D::<crate::int::types::Int<8>, 150>::ONE);
        // 152-digit ceiling multiplier fits in Int<8> (new MAX_SCALE)
        let _ = super::D153s152::multiplier();
    }

    #[cfg(any(feature = "d307", feature = "wide"))]
    #[test]
    fn d307_smoke() {
        use crate::types::traits::arithmetic::DecimalArithmetic;
        use crate::int::types::Int;
        type D = crate::D<crate::int::types::Int<16>, 35>;
        assert_eq!(<D as DecimalArithmetic>::MAX_SCALE, 306);
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::from_int(6i128));
        assert_eq!((three * two) / two, three);
        assert_eq!(D::ZERO.to_bits(), Int::<16>::from_str_radix("0", 10).unwrap());
        assert_eq!(alloc::format!("{}", one).len(), "1.".len() + 35);
        // cross-width: D76 -> D307 widening, D307 -> D76 narrowing
        #[cfg(any(feature = "d76", feature = "wide"))]
        {
            let small: crate::D<crate::int::types::Int<4>, 35> = crate::D::<crate::int::types::Int<4>, 35>::ONE;
            let widened: crate::D<crate::int::types::Int<16>, 35> = small.into();
            assert_eq!(widened, D::ONE);
            let narrowed: crate::D<crate::int::types::Int<4>, 35> = widened.try_into().unwrap();
            assert_eq!(narrowed, crate::D::<crate::int::types::Int<4>, 35>::ONE);
        }
        // 306-digit ceiling multiplier fits in Int<16> (new MAX_SCALE)
        let _ = super::D307s306::multiplier();
    }
}
