//! Core type definitions for every decimal width and their scale aliases.
//!
//! Each width is a `#[repr(transparent)]` newtype around an integer
//! storage of the matching size. The stored integer equals
//! `actual_value * 10^SCALE`. Widths:
//!
//! | Type | Storage | `MAX_SCALE` |
//! |------|---------|-------------|
//! | [`D9<SCALE>`]  | `i32`             | 9   |
//! | [`D18<SCALE>`] | `i64`             | 18  |
//! | [`D38<SCALE>`] | `i128`            | 38  |
//! | [`D76<SCALE>`] | `crate::wide_int::I256` | 76 |
//! | [`D153<SCALE>`] | `crate::wide_int::I512` | 153 |
//! | [`D307<SCALE>`] | `crate::wide_int::I1024` | 307 |
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
pub type D38<const SCALE: u32> = crate::D<i128, SCALE>;

// Manual `Debug` is implemented in `display.rs` (via the
// `decl_decimal_display!` macro) and renders via `Display` so the
// canonical decimal string is shown rather than the raw i128.

/// `Default` returns `ZERO`, matching `i128::default() == 0`.
///
/// This lets `#[derive(Default)]` work correctly on structs that contain
/// `D38<S>` fields.
///
/// Implemented on the underlying `crate::D<i128, SCALE>` because
/// `D38<SCALE>` is now an alias of that type. `ZERO` is emitted by
/// the basics macro further down in this file.
impl<const SCALE: u32> Default for crate::D<i128, SCALE> {
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
/// Math constants lose precision above `SCALE = 35`; see `D38s36`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s37 = D38<37>;

/// Scale alias: `D38<38>`. 1 LSB = 10^-38. Range ~+/-1.7
/// (sub-unit dimensionless ratios).
///
/// This is the maximum supported scale. `10^38 < i128::MAX < 10^39`, so
/// `SCALE = 39` is not supported (`multiplier()` would overflow). Math
/// constants lose precision above `SCALE = 35`; see `D38s36`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D38s38 = D38<38>;

// The `ParseError` enum lives in `src/error.rs` and is re-exported
// from the crate root. It is not width-specific.
pub use crate::error::ParseError;

// Inherent basics + Decimal trait impl: emitted by the macro generator
// (one invocation per width). See src/decimal_macro.rs for the macro
// definition and the surface it produces.
crate::macros::basics::decl_decimal_basics!(D38, i128, 38);
crate::macros::display::decl_decimal_display!(D38);
// FromStr and the raw-storage hex / octal / binary formatters: the
// shared macros. D38's hand-coded versions were equivalent (`FromStr`
// delegated to the same `parse_decimal` path; the formatters delegate
// straight to the `i128` formatter).
crate::macros::from_str::decl_decimal_from_str!(D38, i128);
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D38);
// Bitwise operators (BitAnd/Or/Xor/Not, Shl/Shr) and bit-manipulation
// methods (unsigned_shr, rotate_*, *_zeros, count_*, *_power_of_two) on
// the raw storage. Previously hand-coded for D38 only; now a shared
// macro so every width has the surface.
crate::macros::bitwise::decl_decimal_bitwise!(D38, i128);
// Euclidean / floor / ceil division, abs_diff, midpoint, and the
// is_zero / is_normal / is_nan / is_infinite / is_finite predicates.
crate::macros::int_methods::decl_decimal_int_methods!(D38, i128);
// FromPrimitive / ToPrimitive / NumCast via the shared macro.
crate::macros::num_traits::decl_decimal_num_traits_conversions!(D38, i128);
crate::macros::float_bridge::decl_decimal_float_bridge!(D38, i128);
crate::macros::conversions::decl_from_primitive!(D38, i128, i8);
crate::macros::conversions::decl_from_primitive!(D38, i128, i16);
crate::macros::conversions::decl_from_primitive!(D38, i128, i32);
crate::macros::conversions::decl_from_primitive!(D38, i128, i64);
crate::macros::conversions::decl_from_primitive!(D38, i128, u8);
crate::macros::conversions::decl_from_primitive!(D38, i128, u16);
crate::macros::conversions::decl_from_primitive!(D38, i128, u32);
crate::macros::conversions::decl_from_primitive!(D38, i128, u64);
crate::macros::conversions::decl_try_from_i128!(D38, i128);
crate::macros::conversions::decl_try_from_u128!(D38, i128);
crate::macros::conversions::decl_try_from_i128!(D18, i64);
crate::macros::conversions::decl_try_from_u128!(D18, i64);
crate::macros::conversions::decl_try_from_i128!(D9, i32);
crate::macros::conversions::decl_try_from_u128!(D9, i32);
crate::macros::conversions::decl_try_from_f64!(D38, i128);
crate::macros::conversions::decl_try_from_f32!(D38, i128);
crate::macros::conversions::decl_try_from_f64!(D18, i64);
crate::macros::conversions::decl_try_from_f32!(D18, i64);
crate::macros::conversions::decl_try_from_f64!(D9, i32);
crate::macros::conversions::decl_try_from_f32!(D9, i32);
crate::macros::conversions::decl_decimal_int_conversion_methods!(D38, i128, i64);
// abs / signum / is_positive / is_negative, min / max / clamp / recip /
// copysign, and floor / ceil / round / trunc / fract are emitted by the
// shared macros — D38's hand-coded versions were byte-identical to the
// macro output (see `src/macros/{sign,helpers,rounding_methods}.rs`).
crate::macros::sign::decl_decimal_sign_methods!(D38, i128);
crate::macros::helpers::decl_decimal_helpers!(D38);
crate::macros::rounding_methods::decl_decimal_rounding_methods!(D38);
// Overflow-variant families for add / sub / neg / rem: the macro's
// shared `@common` arm. D38's hand-coded versions were byte-identical.
// The mul / div variants stay hand-coded in `src/overflow_variants.rs`
// because they route through the type-specific `mg_divide` path.
crate::macros::overflow::decl_decimal_overflow_variants!(@common D38, i128);
// Add / Sub / Neg / Rem operator impls (and their `*Assign` forms): the
// arithmetic macro's shared `@common` arm. Mul / Div stay hand-coded in
// `src/arithmetic.rs` (the `mg_divide` 256-bit-widening path).
crate::macros::arithmetic::decl_decimal_arithmetic!(@common D38, i128);
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
// `log_exp_strict.rs` / `trig_strict.rs` / `powers_strict.rs` using
// the hand-tuned 256-bit `d_w128_kernels::Fixed` work integer. They
// are the **chosen winners** per the per-type-kernel policy:
//
// - `decl_wide_transcendental!(D38, i128, Int512, …)` would deliver
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

crate::macros::conversions::decl_decimal_int_conversion_methods!(D18, i64, i64);
crate::macros::conversions::decl_decimal_int_conversion_methods!(D9, i32, i32);

// ─── D38 narrow ───────────────────────────────────────────────────────
// D38::widen is wide-tier-only and is emitted further down in the
// wide block. D38::narrow is always available.

impl<const SCALE: u32> D38<SCALE> {
    /// Demote to the previous storage tier ([`D18`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::OutOfRange)` if the value
    /// doesn't fit `i64`'s range at the given scale.
    ///
    /// ```
    /// use decimal_scaled::D38s9;
    /// let a = D38s9::from_int(1_000_000);
    /// let b = a.narrow().unwrap();
    /// assert_eq!(b.to_bits() as i128, a.to_bits());
    /// ```
    #[inline]
    pub fn narrow(self) -> Result<D18<SCALE>, crate::error::ConvertError> {
        self.try_into()
    }
}

// ---------------------------------------------------------------------
// D9 — 32-bit storage, scale 0..=9. Embedded / register-sized ledger
// type. SCALE = 9 fits ~21.5 with 9 decimal digits of precision; SCALE
// = 0 covers ±2.1 × 10⁹ unscaled. Only the basics emitted in this
// sub-phase; arithmetic / display / num_traits land incrementally.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 32-bit storage. See [`D38`] for the
/// shape documentation; D9 has the same surface scaled to `i32` and
/// `MAX_SCALE = 9`.
///
/// Now a type alias of the unified [`crate::D`] generic decimal type:
/// `D9<S>` is `D<i32, S>`. Both spellings are interchangeable. The
/// `#[repr(transparent)]` layout over `i32` is preserved through the
/// alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
pub type D9<const SCALE: u32> = crate::D<i32, SCALE>;

/// `Default` returns `ZERO`, matching `i32::default() == 0`.
///
/// Implemented on the underlying `crate::D<i32, SCALE>` because
/// `D9<SCALE>` is now an alias of that type. `ZERO` is emitted by
/// the basics macro further down in this file.
impl<const SCALE: u32> Default for crate::D<i32, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

crate::macros::basics::decl_decimal_basics!(D9, i32, 9);
crate::macros::arithmetic::decl_decimal_arithmetic!(D9, i32, i64);
crate::macros::conversions::decl_from_primitive!(D9, i32, i8);
crate::macros::conversions::decl_from_primitive!(D9, i32, i16);
crate::macros::conversions::decl_from_primitive!(D9, i32, u8);
crate::macros::conversions::decl_from_primitive!(D9, i32, u16);
crate::macros::display::decl_decimal_display!(D9);
crate::macros::overflow::decl_decimal_overflow_variants!(D9, i32, i64);
crate::macros::num_traits::decl_decimal_num_traits_basics!(D9);
crate::macros::sign::decl_decimal_sign_methods!(D9, i32);
crate::macros::consts::decl_decimal_consts!(D9, i32);
crate::macros::from_str::decl_decimal_from_str!(D9, i32);
crate::macros::float_bridge::decl_decimal_float_bridge!(D9, i32);
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D9);
crate::macros::strict_transcendentals::decl_strict_transcendentals_via_d38!(D9);
crate::macros::transcendental_trait::decl_decimal_transcendental_impl!(D9);
crate::macros::fast_transcendentals::decl_fast_transcendentals_via_f64!(D9);
crate::macros::pow::decl_decimal_pow!(D9);
crate::macros::rounding_methods::decl_decimal_rounding_methods!(D9);
crate::macros::helpers::decl_decimal_helpers!(D9);
crate::macros::bitwise::decl_decimal_bitwise!(D9, i32);
crate::macros::int_methods::decl_decimal_int_methods!(D9, i32);
crate::macros::num_traits::decl_decimal_num_traits_conversions!(D9, i32);

/// Scale alias: `D9<0>`. 1 LSB = 1 (thin `i32` wrapper). Range ±2.1 × 10⁹.
pub type D9s0 = D9<0>;
/// Scale alias: `D9<1>`. 1 LSB = 10^-1. Range ±2.1 × 10⁸.
pub type D9s1 = D9<1>;
/// Scale alias: `D9<2>`. 1 LSB = 10^-2 (cents). Range ±2.1 × 10⁷.
pub type D9s2 = D9<2>;
/// Scale alias: `D9<3>`. 1 LSB = 10^-3 (mills). Range ±2.1 × 10⁶.
pub type D9s3 = D9<3>;
/// Scale alias: `D9<4>`. 1 LSB = 10^-4 (basis points). Range ±2.1 × 10⁵.
pub type D9s4 = D9<4>;
/// Scale alias: `D9<5>`. 1 LSB = 10^-5. Range ±2.1 × 10⁴.
pub type D9s5 = D9<5>;
/// Scale alias: `D9<6>`. 1 LSB = 10^-6 (ppm). Range ±2.1 × 10³.
pub type D9s6 = D9<6>;
/// Scale alias: `D9<7>`. 1 LSB = 10^-7. Range ±214.
pub type D9s7 = D9<7>;
/// Scale alias: `D9<8>`. 1 LSB = 10^-8 (satoshi). Range ±21.4.
pub type D9s8 = D9<8>;
/// Scale alias: `D9<9>`. 1 LSB = 10^-9 (nano). Range ±2.1.
pub type D9s9 = D9<9>;

// ---------------------------------------------------------------------
// D18 — 64-bit storage, scale 0..=18. Interchange size; fits a GPR on
// 64-bit hosts and maps cleanly to ANSI / SQL `DECIMAL(18, S)` columns.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 64-bit storage. See [`D38`] for the
/// shape documentation; D18 has the same surface scaled to `i64` and
/// `MAX_SCALE = 18`.
///
/// Now a type alias of the unified [`crate::D`] generic decimal type:
/// `D18<S>` is `D<i64, S>`. Both spellings are interchangeable. The
/// `#[repr(transparent)]` layout over `i64` is preserved through the
/// alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
pub type D18<const SCALE: u32> = crate::D<i64, SCALE>;

/// `Default` returns `ZERO`, matching `i64::default() == 0`.
///
/// Implemented on the underlying `crate::D<i64, SCALE>` because
/// `D18<SCALE>` is now an alias of that type. `ZERO` is emitted by
/// the basics macro further down in this file.
impl<const SCALE: u32> Default for crate::D<i64, SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

crate::macros::basics::decl_decimal_basics!(D18, i64, 18);
crate::macros::arithmetic::decl_decimal_arithmetic!(D18, i64, i128);
crate::macros::conversions::decl_from_primitive!(D18, i64, i8);
crate::macros::conversions::decl_from_primitive!(D18, i64, i16);
crate::macros::conversions::decl_from_primitive!(D18, i64, i32);
crate::macros::conversions::decl_from_primitive!(D18, i64, u8);
crate::macros::conversions::decl_from_primitive!(D18, i64, u16);
crate::macros::conversions::decl_from_primitive!(D18, i64, u32);
crate::macros::display::decl_decimal_display!(D18);
crate::macros::overflow::decl_decimal_overflow_variants!(D18, i64, i128);
crate::macros::num_traits::decl_decimal_num_traits_basics!(D18);
crate::macros::sign::decl_decimal_sign_methods!(D18, i64);
crate::macros::consts::decl_decimal_consts!(D18, i64);
crate::macros::from_str::decl_decimal_from_str!(D18, i64);
crate::macros::float_bridge::decl_decimal_float_bridge!(D18, i64);
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D18);
crate::macros::strict_transcendentals::decl_strict_transcendentals_via_d38!(D18);
crate::macros::transcendental_trait::decl_decimal_transcendental_impl!(D18);
crate::macros::fast_transcendentals::decl_fast_transcendentals_via_f64!(D18);
crate::macros::pow::decl_decimal_pow!(D18);
crate::macros::rounding_methods::decl_decimal_rounding_methods!(D18);
crate::macros::helpers::decl_decimal_helpers!(D18);
crate::macros::bitwise::decl_decimal_bitwise!(D18, i64);
crate::macros::int_methods::decl_decimal_int_methods!(D18, i64);
crate::macros::num_traits::decl_decimal_num_traits_conversions!(D18, i64);

// Cross-width widening (lossless). D9 -> D18, D9 -> D38, D18 -> D38.
crate::macros::conversions::decl_cross_width_widening!(D18, i64, D9, i32);
crate::macros::conversions::decl_cross_width_widening!(D38, i128, D9, i32);
crate::macros::conversions::decl_cross_width_widening!(D38, i128, D18, i64);

// Cross-width narrowing (fallible). D38 -> D18, D38 -> D9, D18 -> D9.
crate::macros::conversions::decl_cross_width_narrowing!(D18, i64, D38, i128);
crate::macros::conversions::decl_cross_width_narrowing!(D9, i32, D38, i128);
crate::macros::conversions::decl_cross_width_narrowing!(D9, i32, D18, i64);

// ─── `widen` / `narrow` — hop one storage tier at a time ──────────────
//
// `widen` always succeeds (the next-larger storage strictly covers
// every value the smaller one can hold). `narrow` returns
// `Result<NarrowerType<SCALE>, ConvertError>` because the value may
// not fit. Both keep the scale unchanged; combine with `rescale` if
// you need to change scale and width together.

impl<const SCALE: u32> D9<SCALE> {
    /// Promote to the next storage tier ([`D18`]) at the same `SCALE`.
    ///
    /// Lossless — every `D9<SCALE>` value fits `D18<SCALE>` by
    /// construction (`i32::MAX` < `i64::MAX`). Chains via further
    /// `widen` calls if you need to climb to D38 / D76 / etc.
    ///
    /// ```
    /// use decimal_scaled::D9s6;
    /// let a = D9s6::from_int(123);
    /// let b = a.widen();              // D18<6>
    /// assert_eq!(b.to_bits(), a.to_bits() as i64);
    /// ```
    #[inline]
    #[must_use]
    pub fn widen(self) -> D18<SCALE> {
        self.into()
    }
}

impl<const SCALE: u32> D18<SCALE> {
    /// Promote to the next storage tier ([`D38`]) at the same `SCALE`.
    /// Lossless.
    ///
    /// ```
    /// use decimal_scaled::D18s9;
    /// let a = D18s9::from_int(7);
    /// let b = a.widen();              // D38<9>
    /// assert_eq!(b.to_bits(), a.to_bits() as i128);
    /// ```
    #[inline]
    #[must_use]
    pub fn widen(self) -> D38<SCALE> {
        self.into()
    }

    /// Demote to the previous storage tier ([`D9`]) at the same
    /// `SCALE`.
    ///
    /// # Errors
    ///
    /// Returns `Err(ConvertError::OutOfRange)` if the value doesn't
    /// fit `i32`'s range at the given scale.
    ///
    /// ```
    /// use decimal_scaled::D18s5;
    /// let a = D18s5::from_int(7);
    /// let b = a.narrow().unwrap();    // D9<5>
    /// assert_eq!(b.to_bits() as i64, a.to_bits());
    /// ```
    #[inline]
    pub fn narrow(self) -> Result<D9<SCALE>, crate::error::ConvertError> {
        self.try_into()
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
pub type D18s17 = D18<17>;
/// Scale alias: `D18<18>`. 1 LSB = 10^-18 (atto). Range ±9.2.
pub type D18s18 = D18<18>;

// ---------------------------------------------------------------------
// D76 — 256-bit storage (`Int256`), scale 0..=76. First of the
// wide tier; gated behind the `d76` / `wide` Cargo features. Covers
// the full IEEE-754 decimal128 range and gives 35-digit fractional
// precision with integer-part headroom (see research doc §1).
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 256-bit storage. See [`D38`] for the
/// shape documentation; D76 has the same surface scaled to a 256-bit
/// signed integer and `MAX_SCALE = 76`.
///
/// Gated behind the `d76` (or umbrella `wide`) Cargo feature. The
/// storage backend is `Int256`.
#[cfg(any(feature = "d76", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D76<const SCALE: u32>(pub crate::wide_int::I256);

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> Default for D76<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D76,
    crate::wide_int::I256,
    crate::wide_int::U256,
    crate::wide_int::I512,
    crate::wide_int::Int512,
    crate::wide_int::Int1024,
    crate::wide_int::Int1024,
    wide_trig_d76,
    76
);
// Cross-width widening into D76 (lossless): D9 / D18 / D38 -> D76.
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D76, crate::wide_int::I256, D9, i32);
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D76, crate::wide_int::I256, D18, i64);
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D76, crate::wide_int::I256, D38, i128);
// Cross-width narrowing from D76 (fallible): D76 -> D38 / D18 / D9.
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D38, i128, D76, crate::wide_int::I256);
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D18, i64, D76, crate::wide_int::I256);
#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D9, i32, D76, crate::wide_int::I256);

// ─── D38::widen / D76 hop methods ─────────────────────────────────────

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> D38<SCALE> {
    /// Promote to the next storage tier ([`D56`]) at the same `SCALE`.
    /// Lossless. Available with the `d56` (or umbrella `wide`) Cargo
    /// feature enabled.
    ///
    /// ```
    /// # #[cfg(feature = "wide")] {
    /// use decimal_scaled::D38s12;
    /// let a = D38s12::from_int(1_000_000);
    /// let _wider = a.widen();  // D56<12>
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn widen(self) -> D56<SCALE> {
        self.into()
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> D76<SCALE> {
    /// Demote to the previous storage tier ([`D56`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::Overflow)` if the value
    /// doesn't fit `D56`'s range at the given scale.
    #[inline]
    pub fn narrow(self) -> Result<D56<SCALE>, crate::error::ConvertError> {
        self.try_into()
    }
}

/// Scale alias: `D76<0>`. 1 LSB = 1 (256-bit integer ledger).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s0  = D76<0>;
/// Scale alias: `D76<1>`. 1 LSB = 10^-1 (tenths).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s1  = D76<1>;
/// Scale alias: `D76<2>`. 1 LSB = 10^-2 (cents).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s2  = D76<2>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s3  = D76<3>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s4  = D76<4>;
/// Scale alias: `D76<6>`. 1 LSB = 10^-6 (ppm).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s6  = D76<6>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s9  = D76<9>;
/// Scale alias: `D76<12>`. 1 LSB = 10^-12 (pico; financial standard).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s12 = D76<12>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s15 = D76<15>;
/// Scale alias: `D76<18>`. 1 LSB = 10^-18 (atto).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s18 = D76<18>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s20 = D76<20>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s24 = D76<24>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s28 = D76<28>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s32 = D76<32>;
/// Scale alias: `D76<35>`. 1 LSB = 10^-35 (matches `SCALE_REF`).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s35 = D76<35>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s38 = D76<38>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s42 = D76<42>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s48 = D76<48>;
/// Scale alias: `D76<50>`. 1 LSB = 10^-50 (deep scientific precision).
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s50 = D76<50>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s56 = D76<56>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s64 = D76<64>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s70 = D76<70>;
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s75 = D76<75>;
/// Scale alias: `D76<76>`. 1 LSB = 10^-76. Maximum supported scale.
#[cfg(any(feature = "d76", feature = "wide"))] pub type D76s76 = D76<76>;

// ---------------------------------------------------------------------
// D153 — 512-bit storage (`Int512`), scale 0..=153. Wide-scientific
// tier; gated behind the `d153` / `wide` Cargo features.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 512-bit storage. See [`D38`] for the
/// shape documentation; D153 has the same surface scaled to a 512-bit
/// signed integer and `MAX_SCALE = 153`.
///
/// Gated behind the `d153` (or umbrella `wide`) Cargo feature. The
/// storage backend is `Int512`.
#[cfg(any(feature = "d153", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D153<const SCALE: u32>(pub crate::wide_int::I512);

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> Default for D153<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D153,
    crate::wide_int::I512,
    crate::wide_int::U512,
    crate::wide_int::I1024,
    crate::wide_int::Int1024,
    crate::wide_int::Int2048,
    crate::wide_int::Int2048,
    wide_trig_d153,
    153
);
// Cross-width widening into D153 (lossless): D38 / D76 -> D153.
#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D153, crate::wide_int::I512, D38, i128);
#[cfg(all(any(feature = "d153", feature = "wide"), any(feature = "d76", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D153, crate::wide_int::I512, D76, crate::wide_int::I256);
// Cross-width narrowing from D153 (fallible): D153 -> D76 / D38.
#[cfg(all(any(feature = "d153", feature = "wide"), any(feature = "d76", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D76, crate::wide_int::I256, D153, crate::wide_int::I512);
#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D38, i128, D153, crate::wide_int::I512);

// ─── D76::widen / D153 hop methods ────────────────────────────────────

#[cfg(all(any(feature = "d76", feature = "wide"), any(feature = "d114", feature = "wide")))]
impl<const SCALE: u32> D76<SCALE> {
    /// Promote to the next storage tier ([`D114`]) at the same
    /// `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> D114<SCALE> {
        self.into()
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> D153<SCALE> {
    /// Demote to the previous storage tier ([`D114`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::Overflow)` if the value
    /// doesn't fit the narrower storage's range at the given scale.
    #[cfg(any(feature = "d114", feature = "wide"))]
    #[inline]
    pub fn narrow(self) -> Result<D114<SCALE>, crate::error::ConvertError> {
        self.try_into()
    }
}

/// Scale alias: `D153<0>`. 1 LSB = 1 (512-bit integer ledger).
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s0   = D153<0>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s1   = D153<1>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s2   = D153<2>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s4   = D153<4>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s6   = D153<6>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s9   = D153<9>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s12  = D153<12>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s15  = D153<15>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s18  = D153<18>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s20  = D153<20>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s24  = D153<24>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s28  = D153<28>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s32  = D153<32>;
/// Scale alias: `D153<35>`. 1 LSB = 10^-35 (matches D38 `SCALE_REF`).
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s35  = D153<35>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s38  = D153<38>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s50  = D153<50>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s57  = D153<57>;
/// Scale alias: `D153<75>`. 1 LSB = 10^-75 (wide-scientific midpoint).
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s75  = D153<75>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s76  = D153<76>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s100 = D153<100>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s115 = D153<115>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s140 = D153<140>;
/// Scale alias: `D153<150>`. 1 LSB = 10^-150.
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s150 = D153<150>;
#[cfg(any(feature = "d153", feature = "wide"))] pub type D153s152 = D153<152>;
/// Scale alias: `D153<153>`. 1 LSB = 10^-153. Maximum supported scale.
#[cfg(any(feature = "d153", feature = "wide"))]
pub type D153s153 = D153<153>;

// ---------------------------------------------------------------------
// D307 — 1024-bit storage (`Int1024`), scale 0..=307. Deep
// arbitrary-precision tier; gated behind the `d307` / `wide` features.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 1024-bit storage. See [`D38`] for
/// the shape documentation; D307 has the same surface scaled to a
/// 1024-bit signed integer and `MAX_SCALE = 307`.
///
/// Gated behind the `d307` (or umbrella `wide`) Cargo feature. The
/// storage backend is `Int1024`.
#[cfg(any(feature = "d307", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D307<const SCALE: u32>(pub crate::wide_int::I1024);

#[cfg(any(feature = "d307", feature = "wide"))]
impl<const SCALE: u32> Default for D307<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d307", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D307,
    crate::wide_int::I1024,
    crate::wide_int::U1024,
    crate::wide_int::I2048,
    crate::wide_int::Int2048,
    crate::wide_int::Int4096,
    crate::wide_int::Int4096,
    wide_trig_d307,
    307
);
// Cross-width widening into D307 (lossless): D76 / D153 -> D307.
#[cfg(all(any(feature = "d307", feature = "wide"), any(feature = "d76", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D307, crate::wide_int::I1024, D76, crate::wide_int::I256);
#[cfg(all(any(feature = "d307", feature = "wide"), any(feature = "d153", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D307, crate::wide_int::I1024, D153, crate::wide_int::I512);
// Cross-width narrowing from D307 (fallible): D307 -> D153 / D76.
#[cfg(all(any(feature = "d307", feature = "wide"), any(feature = "d153", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D153, crate::wide_int::I512, D307, crate::wide_int::I1024);
#[cfg(all(any(feature = "d307", feature = "wide"), any(feature = "d76", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D76, crate::wide_int::I256, D307, crate::wide_int::I1024);

// ─── D153::widen / D307 hop methods ───────────────────────────────────

#[cfg(all(any(feature = "d153", feature = "wide"), any(feature = "d230", feature = "wide")))]
impl<const SCALE: u32> D153<SCALE> {
    /// Promote to the next storage tier ([`D230`]) at the same
    /// `SCALE`. Lossless.
    #[inline]
    #[must_use]
    pub fn widen(self) -> D230<SCALE> {
        self.into()
    }
}

#[cfg(any(feature = "d307", feature = "wide"))]
impl<const SCALE: u32> D307<SCALE> {
    /// Demote to the previous storage tier ([`D230`]) at the same
    /// `SCALE`. Returns `Err(ConvertError::Overflow)` if the value
    /// doesn't fit the narrower storage's range at the given scale.
    #[cfg(any(feature = "d230", feature = "wide"))]
    #[inline]
    pub fn narrow(self) -> Result<D230<SCALE>, crate::error::ConvertError> {
        self.try_into()
    }

    /// Promote to the next storage tier ([`D461`]) at the same
    /// `SCALE`. Lossless. Requires `d461` / `x-wide`.
    #[cfg(any(feature = "d461", feature = "x-wide"))]
    #[inline]
    #[must_use]
    pub fn widen(self) -> D461<SCALE> {
        self.into()
    }
}

/// Scale alias: `D307<0>`. 1 LSB = 1 (1024-bit integer ledger).
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s0   = D307<0>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s1   = D307<1>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s2   = D307<2>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s4   = D307<4>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s6   = D307<6>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s9   = D307<9>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s12  = D307<12>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s15  = D307<15>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s18  = D307<18>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s20  = D307<20>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s24  = D307<24>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s28  = D307<28>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s32  = D307<32>;
/// Scale alias: `D307<35>`. 1 LSB = 10^-35 (matches D38 `SCALE_REF`).
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s35  = D307<35>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s38  = D307<38>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s50  = D307<50>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s75  = D307<75>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s100 = D307<100>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s115 = D307<115>;
/// Scale alias: `D307<150>`. 1 LSB = 10^-150.
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s150 = D307<150>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s153 = D307<153>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s200 = D307<200>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s230 = D307<230>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s275 = D307<275>;
/// Scale alias: `D307<300>`. 1 LSB = 10^-300.
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s300 = D307<300>;
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s306 = D307<306>;
/// Scale alias: `D307<307>`. 1 LSB = 10^-307. Maximum supported scale.
#[cfg(any(feature = "d307", feature = "wide"))] pub type D307s307 = D307<307>;

// ─── Half-width and wider tiers (D56 / D114 / D230 / D461 / D615 / D923 / D1231) ───
//
// These fill the (2^n + 2^(n+1))/2 gaps between the existing
// power-of-two storage tiers, plus extend the top end past D307.
// Each tier has the same surface as D76 / D153 / D307: full
// `decl_decimal_full!` emission (every arithmetic / transcendental
// method), plus scale aliases at 0 / mid / max.
//
// Cross-width widening / narrowing methods are emitted to the
// immediate-neighbour tiers only — `D56 ↔ D38`, `D56 ↔ D76`, etc.
// Multi-tier hops go via the chain (e.g. D56 → D76 → D153) at the
// cost of one intermediate.

// ── D56 (192-bit / 3 u64 limbs) ────────────────────────────────────────

/// Scaled fixed-point decimal with 192-bit storage. Half-width tier
/// between D38 and D76 — useful when the D38 i128 ceiling is in
/// reach but D76's 256-bit storage is wasteful. Now a type alias of
/// the unified [`crate::D`] generic decimal type: `D56<S>` is
/// `D<crate::wide_int::Int192, S>`. Both spellings are interchangeable.
///
/// The `#[repr(transparent)]` layout over `Int192` is preserved
/// through the alias because the underlying [`crate::D`] is itself
/// `#[repr(transparent)]` over its storage parameter.
///
/// Gated behind the `d56` (or umbrella `wide`) Cargo feature.
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56<const SCALE: u32> = crate::D<crate::wide_int::Int192, SCALE>;

/// `Default` returns `ZERO`, matching the all-zero limb pattern of
/// `Int192`.
///
/// Implemented on the underlying `crate::D<crate::wide_int::Int192, SCALE>`
/// because `D56<SCALE>` is now an alias of that type. `ZERO` is emitted
/// by the basics macro further down in this file.
#[cfg(any(feature = "d56", feature = "wide"))]
impl<const SCALE: u32> Default for crate::D<crate::wide_int::Int192, SCALE> {
    #[inline]
    fn default() -> Self { Self::ZERO }
}

#[cfg(any(feature = "d56", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D56,
    crate::wide_int::I192,
    crate::wide_int::U192,
    crate::wide_int::I384,
    crate::wide_int::Int384,
    crate::wide_int::Int512,
    crate::wide_int::Int1024,
    wide_trig_d56,
    57
);
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s0 = D56<0>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s1 = D56<1>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s2 = D56<2>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s4 = D56<4>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s6 = D56<6>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s9 = D56<9>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s12 = D56<12>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s18 = D56<18>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s20 = D56<20>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s24 = D56<24>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s28 = D56<28>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s32 = D56<32>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s38 = D56<38>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s42 = D56<42>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s48 = D56<48>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s52 = D56<52>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s56 = D56<56>;
#[cfg(any(feature = "d56", feature = "wide"))]
pub type D56s57 = D56<57>;

// ── D114 (384-bit / 6 u64 limbs) ───────────────────────────────────────

/// Scaled fixed-point decimal with 384-bit storage. Half-width tier
/// between D76 and D153.
///
/// Gated behind the `d114` (or umbrella `wide`) Cargo feature.
#[cfg(any(feature = "d114", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D114<const SCALE: u32>(pub crate::wide_int::I384);

#[cfg(any(feature = "d114", feature = "wide"))]
impl<const SCALE: u32> Default for D114<SCALE> {
    #[inline]
    fn default() -> Self { Self::ZERO }
}

#[cfg(any(feature = "d114", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D114,
    crate::wide_int::I384,
    crate::wide_int::U384,
    crate::wide_int::I768,
    crate::wide_int::Int768,
    crate::wide_int::Int1024,
    crate::wide_int::Int1024,
    wide_trig_d114,
    115
);
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s0 = D114<0>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s1 = D114<1>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s4 = D114<4>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s8 = D114<8>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s16 = D114<16>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s24 = D114<24>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s32 = D114<32>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s38 = D114<38>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s50 = D114<50>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s57 = D114<57>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s64 = D114<64>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s76 = D114<76>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s90 = D114<90>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s100 = D114<100>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s110 = D114<110>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s114 = D114<114>;
#[cfg(any(feature = "d114", feature = "wide"))]
pub type D114s115 = D114<115>;

// ── D230 (768-bit / 12 u64 limbs) ──────────────────────────────────────

/// Scaled fixed-point decimal with 768-bit storage. Half-width tier
/// between D153 and D307.
///
/// Gated behind the `d230` (or umbrella `wide`) Cargo feature.
#[cfg(any(feature = "d230", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D230<const SCALE: u32>(pub crate::wide_int::I768);

#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> Default for D230<SCALE> {
    #[inline]
    fn default() -> Self { Self::ZERO }
}

#[cfg(any(feature = "d230", feature = "wide"))]
crate::macros::full::decl_decimal_full!(
    wide D230,
    crate::wide_int::I768,
    crate::wide_int::U768,
    crate::wide_int::I1536,
    crate::wide_int::Int1536,
    crate::wide_int::Int3072,
    crate::wide_int::Int3072,
    wide_trig_d230,
    230
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
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s229 = D230<229>;
#[cfg(any(feature = "d230", feature = "wide"))]
pub type D230s230 = D230<230>;

// ── D461 (1536-bit / 24 u64 limbs) ─────────────────────────────────────

/// Scaled fixed-point decimal with 1536-bit storage. Half-width tier
/// between D307 and D615.
///
/// Gated behind the `d461` (or umbrella `x-wide`) Cargo feature.
#[cfg(any(feature = "d461", feature = "x-wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D461<const SCALE: u32>(pub crate::wide_int::I1536);

#[cfg(any(feature = "d461", feature = "x-wide"))]
impl<const SCALE: u32> Default for D461<SCALE> {
    #[inline]
    fn default() -> Self { Self::ZERO }
}

#[cfg(any(feature = "d461", feature = "x-wide"))]
crate::macros::full::decl_decimal_full!(
    wide D461,
    crate::wide_int::I1536,
    crate::wide_int::U1536,
    crate::wide_int::I3072,
    crate::wide_int::Int3072,
    crate::wide_int::Int4096,
    crate::wide_int::Int4096,
    wide_trig_d461,
    462
);
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s0 = D461<0>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s1 = D461<1>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s18 = D461<18>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s38 = D461<38>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s75 = D461<75>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s115 = D461<115>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s153 = D461<153>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s200 = D461<200>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s230 = D461<230>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s275 = D461<275>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s307 = D461<307>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s350 = D461<350>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s400 = D461<400>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s440 = D461<440>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s460 = D461<460>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s461 = D461<461>;
#[cfg(any(feature = "d461", feature = "x-wide"))]
pub type D461s462 = D461<462>;

// ── D615 (2048-bit / 32 u64 limbs) ─────────────────────────────────────

/// Scaled fixed-point decimal with 2048-bit storage. New top tier
/// beyond D307; supports correctly-rounded transcendentals at scale
/// up to 616 decimal digits.
///
/// Gated behind the `d615` (or umbrella `x-wide`) Cargo feature.
#[cfg(any(feature = "d615", feature = "x-wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D615<const SCALE: u32>(pub crate::wide_int::I2048);

#[cfg(any(feature = "d615", feature = "x-wide"))]
impl<const SCALE: u32> Default for D615<SCALE> {
    #[inline]
    fn default() -> Self { Self::ZERO }
}

#[cfg(any(feature = "d615", feature = "x-wide"))]
crate::macros::full::decl_decimal_full!(
    wide D615,
    crate::wide_int::I2048,
    crate::wide_int::U2048,
    crate::wide_int::I4096,
    crate::wide_int::Int4096,
    crate::wide_int::Int8192,
    crate::wide_int::Int8192,
    wide_trig_d615,
    616
);
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s0 = D615<0>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s1 = D615<1>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s38 = D615<38>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s75 = D615<75>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s115 = D615<115>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s153 = D615<153>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s200 = D615<200>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s230 = D615<230>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s275 = D615<275>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s308 = D615<308>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s380 = D615<380>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s462 = D615<462>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s500 = D615<500>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s555 = D615<555>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s600 = D615<600>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s615 = D615<615>;
#[cfg(any(feature = "d615", feature = "x-wide"))]
pub type D615s616 = D615<616>;

// ── D923 (3072-bit / 48 u64 limbs) ─────────────────────────────────────

/// Scaled fixed-point decimal with 3072-bit storage. Half-width tier
/// between D615 and D1231; supports SCALE up to 924 digits.
///
/// Gated behind the `d923` (or umbrella `xx-wide`) Cargo feature.
#[cfg(any(feature = "d923", feature = "xx-wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D923<const SCALE: u32>(pub crate::wide_int::I3072);

#[cfg(any(feature = "d923", feature = "xx-wide"))]
impl<const SCALE: u32> Default for D923<SCALE> {
    #[inline]
    fn default() -> Self { Self::ZERO }
}

#[cfg(any(feature = "d923", feature = "xx-wide"))]
crate::macros::full::decl_decimal_full!(
    wide D923,
    crate::wide_int::I3072,
    crate::wide_int::U3072,
    crate::wide_int::I6144,
    crate::wide_int::Int6144,
    crate::wide_int::Int12288,
    crate::wide_int::Int12288,
    wide_trig_d923,
    924
);
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s0 = D923<0>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s1 = D923<1>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s75 = D923<75>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s153 = D923<153>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s230 = D923<230>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s307 = D923<307>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s400 = D923<400>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s461 = D923<461>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s462 = D923<462>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s500 = D923<500>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s616 = D923<616>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s700 = D923<700>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s800 = D923<800>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s860 = D923<860>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s900 = D923<900>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s920 = D923<920>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s923 = D923<923>;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
pub type D923s924 = D923<924>;

// ── D1231 (4096-bit / 64 u64 limbs) ────────────────────────────────────

/// Scaled fixed-point decimal with 4096-bit storage. Widest tier
/// shipped; supports SCALE up to 1232 digits.
///
/// Gated behind the `d1231` (or umbrella `xx-wide`) Cargo feature.
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D1231<const SCALE: u32>(pub crate::wide_int::I4096);

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
impl<const SCALE: u32> Default for D1231<SCALE> {
    #[inline]
    fn default() -> Self { Self::ZERO }
}

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
crate::macros::full::decl_decimal_full!(
    wide D1231,
    crate::wide_int::I4096,
    crate::wide_int::U4096,
    crate::wide_int::I8192,
    crate::wide_int::Int8192,
    crate::wide_int::Int16384,
    crate::wide_int::Int16384,
    wide_trig_d1231,
    1232
);
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s0 = D1231<0>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1 = D1231<1>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s75 = D1231<75>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s153 = D1231<153>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s230 = D1231<230>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s307 = D1231<307>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s461 = D1231<461>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s616 = D1231<616>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s700 = D1231<700>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s800 = D1231<800>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s900 = D1231<900>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s924 = D1231<924>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1000 = D1231<1000>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1100 = D1231<1100>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1180 = D1231<1180>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1220 = D1231<1220>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1230 = D1231<1230>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1231 = D1231<1231>;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
pub type D1231s1232 = D1231<1232>;

// ─── Cross-tier next-neighbour widen/narrow chain ─────────────────────
//
// The historical .widen() / .narrow() methods on D38/D76/D153/D307
// follow the power-of-two storage sequence (D38→D76→D153→D307). The
// 0.2.6 tier ladder fills in half-widths between each pair plus
// extends to D1231; the complete ladder is:
//
//   D9 → D18 → D38 → D56 → D76 → D114 → D153 → D230 → D307 →
//   D461 → D615 → D923 → D1231
//
// The next-neighbour .widen() / .narrow() methods on the new tiers go
// to the immediate adjacent rung (D56.widen() → D76, D76.widen()
// already returns D153 which is the existing power-of-two next-up,
// etc.). The cross-tier From / TryFrom impls below cover the
// neighbour pairs that weren't already declared by the legacy
// D38/D76/D153/D307 blocks.
//
// Coverage strategy: declare every NEW adjacent pair both ways. The
// existing legacy declarations (D9↔D18, D9/D18/D38↔D76, D38/D76↔D153,
// D76/D153↔D307) stay where they are; this block adds the conversions
// that hop through the new tiers (D38↔D56, D56↔D76, D76↔D114, etc.).

// D38 ↔ D56
#[cfg(any(feature = "d56", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D56, crate::wide_int::I192, D38, i128);
#[cfg(any(feature = "d56", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D38, i128, D56, crate::wide_int::I192);

// D56 ↔ D76
#[cfg(all(any(feature = "d56", feature = "wide"), any(feature = "d76", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D76, crate::wide_int::I256, D56, crate::wide_int::I192);
#[cfg(all(any(feature = "d56", feature = "wide"), any(feature = "d76", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D56, crate::wide_int::I192, D76, crate::wide_int::I256);

// D76 ↔ D114
#[cfg(all(any(feature = "d76", feature = "wide"), any(feature = "d114", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D114, crate::wide_int::I384, D76, crate::wide_int::I256);
#[cfg(all(any(feature = "d76", feature = "wide"), any(feature = "d114", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D76, crate::wide_int::I256, D114, crate::wide_int::I384);

// D114 ↔ D153
#[cfg(all(any(feature = "d114", feature = "wide"), any(feature = "d153", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D153, crate::wide_int::I512, D114, crate::wide_int::I384);
#[cfg(all(any(feature = "d114", feature = "wide"), any(feature = "d153", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D114, crate::wide_int::I384, D153, crate::wide_int::I512);

// D153 ↔ D230
#[cfg(all(any(feature = "d153", feature = "wide"), any(feature = "d230", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D230, crate::wide_int::I768, D153, crate::wide_int::I512);
#[cfg(all(any(feature = "d153", feature = "wide"), any(feature = "d230", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D153, crate::wide_int::I512, D230, crate::wide_int::I768);

// D230 ↔ D307
#[cfg(all(any(feature = "d230", feature = "wide"), any(feature = "d307", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D307, crate::wide_int::I1024, D230, crate::wide_int::I768);
#[cfg(all(any(feature = "d230", feature = "wide"), any(feature = "d307", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D230, crate::wide_int::I768, D307, crate::wide_int::I1024);

// D307 ↔ D461
#[cfg(all(any(feature = "d307", feature = "wide"), any(feature = "d461", feature = "x-wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D461, crate::wide_int::I1536, D307, crate::wide_int::I1024);
#[cfg(all(any(feature = "d307", feature = "wide"), any(feature = "d461", feature = "x-wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D307, crate::wide_int::I1024, D461, crate::wide_int::I1536);

// D461 ↔ D615
#[cfg(all(any(feature = "d461", feature = "x-wide"), any(feature = "d615", feature = "x-wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D615, crate::wide_int::I2048, D461, crate::wide_int::I1536);
#[cfg(all(any(feature = "d461", feature = "x-wide"), any(feature = "d615", feature = "x-wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D461, crate::wide_int::I1536, D615, crate::wide_int::I2048);

// D615 ↔ D923
#[cfg(all(any(feature = "d615", feature = "x-wide"), any(feature = "d923", feature = "xx-wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D923, crate::wide_int::I3072, D615, crate::wide_int::I2048);
#[cfg(all(any(feature = "d615", feature = "x-wide"), any(feature = "d923", feature = "xx-wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D615, crate::wide_int::I2048, D923, crate::wide_int::I3072);

// D923 ↔ D1231
#[cfg(all(any(feature = "d923", feature = "xx-wide"), any(feature = "d1231", feature = "xx-wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D1231, crate::wide_int::I4096, D923, crate::wide_int::I3072);
#[cfg(all(any(feature = "d923", feature = "xx-wide"), any(feature = "d1231", feature = "xx-wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D923, crate::wide_int::I3072, D1231, crate::wide_int::I4096);

// .widen() / .narrow() methods on the new tiers — each points at the
// IMMEDIATE neighbour in the comprehensive ladder above. The legacy
// .widen() / .narrow() on D38/D76/D153/D307 are unchanged (still go
// to the power-of-two next-up) for source compatibility; users who
// want to traverse through the half-widths should use the methods
// declared here, or the From / TryFrom impls directly.

#[cfg(any(feature = "d56", feature = "wide"))]
impl<const SCALE: u32> D56<SCALE> {
    /// Demote to the immediate previous tier ([`D38`]) at the same `SCALE`.
    /// Returns `Err(ConvertError::Overflow)` if the value exceeds `i128` range.
    #[inline]
    pub fn narrow(self) -> Result<D38<SCALE>, crate::error::ConvertError> { self.try_into() }
    /// Promote to the next storage tier ([`D76`]) at the same `SCALE`. Lossless.
    #[inline] #[must_use]
    pub fn widen(self) -> D76<SCALE> { self.into() }
}

#[cfg(any(feature = "d114", feature = "wide"))]
impl<const SCALE: u32> D114<SCALE> {
    /// Demote to the immediate previous tier ([`D76`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<D76<SCALE>, crate::error::ConvertError> { self.try_into() }
    /// Promote to the next storage tier ([`D153`]) at the same `SCALE`. Lossless.
    #[inline] #[must_use]
    pub fn widen(self) -> D153<SCALE> { self.into() }
}

#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> D230<SCALE> {
    /// Demote to the immediate previous tier ([`D153`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<D153<SCALE>, crate::error::ConvertError> { self.try_into() }
    /// Promote to the next storage tier ([`D307`]) at the same `SCALE`. Lossless.
    #[inline] #[must_use]
    pub fn widen(self) -> D307<SCALE> { self.into() }
}

#[cfg(any(feature = "d461", feature = "x-wide"))]
impl<const SCALE: u32> D461<SCALE> {
    /// Demote to the immediate previous tier ([`D307`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<D307<SCALE>, crate::error::ConvertError> { self.try_into() }
    /// Promote to the next storage tier ([`D615`]) at the same `SCALE`. Lossless.
    #[inline] #[must_use]
    pub fn widen(self) -> D615<SCALE> { self.into() }
}

#[cfg(any(feature = "d615", feature = "x-wide"))]
impl<const SCALE: u32> D615<SCALE> {
    /// Demote to the immediate previous tier ([`D461`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<D461<SCALE>, crate::error::ConvertError> { self.try_into() }
}

// `widen` lives in a second impl gated on D923's feature — D615 can
// be enabled without xx-wide (docs.rs builds this case), in which
// case D923 doesn't exist as a type and the unconditional `widen`
// method above breaks the doc build.
#[cfg(all(
    any(feature = "d615", feature = "x-wide"),
    any(feature = "d923", feature = "xx-wide"),
))]
impl<const SCALE: u32> D615<SCALE> {
    /// Promote to the next storage tier ([`D923`]) at the same `SCALE`. Lossless.
    #[inline] #[must_use]
    pub fn widen(self) -> D923<SCALE> { self.into() }
}

#[cfg(any(feature = "d923", feature = "xx-wide"))]
impl<const SCALE: u32> D923<SCALE> {
    /// Demote to the immediate previous tier ([`D615`]) at the same `SCALE`.
    #[inline]
    pub fn narrow(self) -> Result<D615<SCALE>, crate::error::ConvertError> { self.try_into() }
    /// Promote to the next storage tier ([`D1231`]) at the same `SCALE`. Lossless.
    #[inline] #[must_use]
    pub fn widen(self) -> D1231<SCALE> { self.into() }
}

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
impl<const SCALE: u32> D1231<SCALE> {
    /// Demote to the immediate previous tier ([`D923`]) at the same `SCALE`.
    /// D1231 is the widest shipped tier, so there is no `.widen()` method.
    #[inline]
    pub fn narrow(self) -> Result<D923<SCALE>, crate::error::ConvertError> { self.try_into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `from_bits` / `to_bits` round-trip is exact.
    #[test]
    fn from_bits_to_bits_round_trip() {
        let raw: i128 = 1_500_000_000_000;
        let v: D38s12 = D38s12::from_bits(raw);
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
            D38s12::from_bits(42_000_000_000_000),
            D38s12::from_bits(42_000_000_000_000)
        );
        assert_ne!(D38s12::from_bits(42), D38s12::from_bits(43));
    }

    /// Ord is derived from i128: smaller bits compare less.
    #[test]
    fn ord_by_underlying_bits() {
        assert!(D38s12::from_bits(1) < D38s12::from_bits(2));
        assert!(D38s12::from_bits(-1) < D38s12::from_bits(0));
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
        assert_eq!(D38s12::from_bits(i128::MAX).scale(), 12);
        assert_eq!(D38s12::from_bits(-7).scale(), 12);
    }

    /// Both forms agree at non-default scales.
    #[test]
    fn scale_at_other_scales() {
        type D6 = super::D38<6>;
        type D0 = super::D38<0>;
        type D38 = super::D38<38>;
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
        type D6 = super::D38<6>;
        assert_eq!(D6::multiplier(), 1_000_000_i128);
        assert_eq!(D6::ONE.to_bits(), 1_000_000_i128);

        type D0 = super::D38<0>;
        assert_eq!(D0::multiplier(), 1_i128);
        assert_eq!(D0::ONE.to_bits(), 1_i128);
    }

    // ----- D9 / D18 sanity tests -----

    #[test]
    fn d9_basics() {
        assert_eq!(super::D9s2::ZERO.to_bits(), 0_i32);
        assert_eq!(super::D9s2::ONE.to_bits(), 100_i32);
        assert_eq!(super::D9s2::MAX.to_bits(), i32::MAX);
        assert_eq!(super::D9s2::MIN.to_bits(), i32::MIN);
        assert_eq!(super::D9s2::multiplier(), 100_i32);
        assert_eq!(super::D9s2::SCALE, 2);
    }

    #[test]
    fn d18_basics() {
        assert_eq!(super::D18s9::ZERO.to_bits(), 0_i64);
        assert_eq!(super::D18s9::ONE.to_bits(), 1_000_000_000_i64);
        assert_eq!(super::D18s9::multiplier(), 1_000_000_000_i64);
        assert_eq!(super::D18s9::SCALE, 9);
    }

    #[test]
    fn d9_arithmetic() {
        let a = super::D9s2::from_bits(150); // 1.50
        let b = super::D9s2::from_bits(250); // 2.50
        assert_eq!((a + b).to_bits(), 400);
        assert_eq!((b - a).to_bits(), 100);
        assert_eq!((-a).to_bits(), -150);

        let x = super::D9s2::from_bits(200); // 2.00
        let y = super::D9s2::from_bits(300); // 3.00
        assert_eq!((x * y).to_bits(), 600); // 6.00
        assert_eq!((y / x).to_bits(), 150); // 1.50
        assert_eq!((y % x).to_bits(), 100); // 1.00
    }

    #[test]
    fn d18_arithmetic() {
        let a = super::D18s9::from_bits(1_500_000_000); // 1.5
        let b = super::D18s9::from_bits(2_500_000_000); // 2.5
        assert_eq!((a + b).to_bits(), 4_000_000_000);
        assert_eq!((b - a).to_bits(), 1_000_000_000);
        assert_eq!((-a).to_bits(), -1_500_000_000);

        let x = super::D18s9::from_bits(2_000_000_000); // 2.0
        let y = super::D18s9::from_bits(3_000_000_000); // 3.0
        assert_eq!((x * y).to_bits(), 6_000_000_000);
        assert_eq!((y / x).to_bits(), 1_500_000_000);
        assert_eq!((y % x).to_bits(), 1_000_000_000);
    }

    #[test]
    fn d9_display() {
        let v: super::D9s2 = super::D9s2::from_bits(150); // 1.50
        let s = alloc::format!("{}", v);
        assert_eq!(s, "1.50");
        let neg: super::D9s2 = super::D9s2::from_bits(-2050); // -20.50
        assert_eq!(alloc::format!("{}", neg), "-20.50");
        let zero: super::D9s2 = super::D9s2::ZERO;
        assert_eq!(alloc::format!("{}", zero), "0.00");
        let int_only: super::D9s0 = super::D9s0::from_bits(42);
        assert_eq!(alloc::format!("{}", int_only), "42");
    }

    #[test]
    fn d18_display() {
        let v: super::D18s9 = super::D18s9::from_bits(1_500_000_000); // 1.500000000
        assert_eq!(alloc::format!("{}", v), "1.500000000");
        let neg: super::D18s9 = super::D18s9::from_bits(-1_500_000_000);
        assert_eq!(alloc::format!("{}", neg), "-1.500000000");
    }

    #[test]
    fn d9_debug() {
        let v: super::D9s2 = super::D9s2::from_bits(150);
        let s = alloc::format!("{:?}", v);
        assert_eq!(s, "D9<2>(1.50)");
    }

    #[test]
    fn cross_width_widening_d9_to_d18() {
        let small: super::D9s2 = super::D9s2::from_bits(150);
        let wider: super::D18s2 = small.into();
        assert_eq!(wider.to_bits(), 150_i64);
    }

    #[test]
    fn cross_width_widening_d9_to_d38() {
        let small: super::D9s2 = super::D9s2::from_bits(-150);
        let wider: super::D38s2 = small.into();
        assert_eq!(wider.to_bits(), -150_i128);
    }

    #[test]
    fn cross_width_widening_d18_to_d38() {
        let mid: super::D18s9 = super::D18s9::from_bits(i64::MAX);
        let wider: super::D38s9 = mid.into();
        assert_eq!(wider.to_bits(), i64::MAX as i128);
    }

    #[test]
    fn cross_width_narrowing_d38_to_d18_in_range() {
        let wide: super::D38s9 = super::D38s9::from_bits(1_500_000_000);
        let narrow: super::D18s9 = wide.try_into().unwrap();
        assert_eq!(narrow.to_bits(), 1_500_000_000);
    }

    #[test]
    fn cross_width_narrowing_d38_to_d18_out_of_range() {
        let wide: super::D38s9 = super::D38s9::from_bits(i128::MAX);
        let narrow: Result<super::D18s9, _> = wide.try_into();
        assert!(narrow.is_err());
    }

    #[test]
    fn cross_width_narrowing_d18_to_d9_in_range() {
        let mid: super::D18s2 = super::D18s2::from_bits(150);
        let narrow: super::D9s2 = mid.try_into().unwrap();
        assert_eq!(narrow.to_bits(), 150);
    }

    #[test]
    fn cross_width_narrowing_d18_to_d9_out_of_range() {
        let mid: super::D18s2 = super::D18s2::from_bits(i64::MAX);
        let narrow: Result<super::D9s2, _> = mid.try_into();
        assert!(narrow.is_err());
    }

    #[test]
    fn d9_consts() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        use crate::consts::DecimalConstants;
        type D9s4 = super::D9<4>;
        // pi at scale 4 = 3.1416 -> bits = 31416.
        assert_eq!(D9s4::pi().to_bits(), 31416);
        // e at scale 4 = 2.7183 -> bits = 27183.
        assert_eq!(D9s4::e().to_bits(), 27183);
    }

    #[test]
    fn d9_from_str() {
        use core::str::FromStr;
        let v = super::D9s2::from_str("1.50").unwrap();
        assert_eq!(v.to_bits(), 150);
        let neg = super::D9s2::from_str("-20.50").unwrap();
        assert_eq!(neg.to_bits(), -2050);
        // Out of range for D9s2 (i32::MAX is ~2.1e9).
        assert!(super::D9s2::from_str("1000000000000.00").is_err());
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
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        use crate::consts::DecimalConstants;
        type D18s12 = super::D18<12>;
        // pi at scale 12 = 3.141592653590 (matches D38s12).
        assert_eq!(D18s12::pi().to_bits(), 3_141_592_653_590);
        // tau at scale 12 = 6.283185307180.
        assert_eq!(D18s12::tau().to_bits(), 6_283_185_307_180);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_basics() {
        use crate::decimal_trait::Decimal;
        use crate::arithmetic_trait::DecimalArithmetic;
        use crate::wide_int::I256;
        assert_eq!(super::D76s2::ZERO.to_bits(), I256::from_str_radix("0", 10).unwrap());
        assert_eq!(super::D76s2::ONE.to_bits(), I256::from_str_radix("100", 10).unwrap());
        assert_eq!(super::D76s2::MAX.to_bits(), I256::MAX);
        assert_eq!(super::D76s2::MIN.to_bits(), I256::MIN);
        assert_eq!(super::D76s2::multiplier(), I256::from_str_radix("100", 10).unwrap());
        assert_eq!(super::D76s2::SCALE, 2);
        assert_eq!(super::D76s2::ZERO.scale(), 2);
        // SCALE = 76 multiplier is 10^76, well within 256-bit range.
        let m76 = super::D76s76::multiplier();
        assert_eq!(
            m76,
            I256::from_str_radix("10000000000000000000000000000000000000000000000000000000000000000000000000000", 10).unwrap()
        );
        assert_eq!(<super::D76s12 as DecimalArithmetic>::MAX_SCALE, 76);
        // round-trip
        let raw = I256::from_str_radix("123456789012345678901234567890", 10).unwrap();
        assert_eq!(super::D76s12::from_bits(raw).to_bits(), raw);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_arithmetic() {
        type D = super::D76<12>;
        let one = D::ONE;
        let two = D::from_bits(D::multiplier() + D::multiplier());
        let three = D::from_bits(D::multiplier() * crate::wide_int::I256::from_str_radix("3", 10).unwrap());
        // add / sub / neg
        assert_eq!((one + two), three);
        assert_eq!((three - one), two);
        assert_eq!((-one).to_bits(), -D::multiplier());
        // mul: 2 * 3 == 6
        let six = D::from_bits(D::multiplier() * crate::wide_int::I256::from_str_radix("6", 10).unwrap());
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
        let half = D::from_bits(D::multiplier() / crate::wide_int::I256::from_str_radix("2", 10).unwrap());
        let one_and_half = one + half;
        let product = one_and_half * one_and_half;
        let expected = D::from_bits(
            D::multiplier() * crate::wide_int::I256::from_str_radix("2", 10).unwrap()
                + D::multiplier() / crate::wide_int::I256::from_str_radix("4", 10).unwrap(),
        );
        assert_eq!(product, expected);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_display() {
        type D = super::D76<12>;
        let one = D::ONE;
        assert_eq!(alloc::format!("{}", one), "1.000000000000");
        assert_eq!(alloc::format!("{}", -one), "-1.000000000000");
        assert_eq!(alloc::format!("{}", D::ZERO), "0.000000000000");
        let half = D::from_bits(D::multiplier() / crate::wide_int::I256::from_str_radix("2", 10).unwrap());
        assert_eq!(alloc::format!("{}", half), "0.500000000000");
        assert_eq!(alloc::format!("{:?}", one), "D76<12>(1.000000000000)");
        // scale 0 prints no fractional part
        let int_only: super::D76<0> = super::D76::<0>::ONE;
        assert_eq!(alloc::format!("{}", int_only), "1");
        // very large magnitude near the 76-digit ceiling
        let big = super::D76s76::MAX;
        let s = alloc::format!("{}", big);
        assert!(s.starts_with("5.78960446"));
        assert_eq!(s.len(), "5.".len() + 76);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_sign_and_helpers() {
        type D = super::D76<6>;
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
        let half = D::from_bits(D::multiplier() / crate::wide_int::I256::from_str_radix("2", 10).unwrap());
        assert_eq!(two.recip(), half);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_overflow_variants() {
        type D = super::D76<2>;
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
        assert_eq!(two.checked_mul(three), Some(D::from_bits(D::multiplier() * crate::wide_int::I256::from_str_radix("6", 10).unwrap())));
        assert_eq!(D::ONE.checked_div(D::ZERO), None);
        assert_eq!((three).checked_div(D::ONE), Some(three));
        // wrapping_add of one storage LSB at MAX wraps around to MIN.
        let one_lsb = D::from_bits(crate::wide_int::I256::from_str_radix("1", 10).unwrap());
        assert_eq!(D::MAX.wrapping_add(one_lsb), D::MIN);
        // overflowing
        assert_eq!(D::ONE.overflowing_add(D::ONE), (two, false));
        assert_eq!(D::MAX.overflowing_add(D::ONE).1, true);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_consts_and_from_str() {
        use crate::consts::DecimalConstants;
        use core::str::FromStr;
        // pi at scale 12 matches the D38 reference.
        assert_eq!(
            super::D76::<12>::pi().to_bits(),
            crate::wide_int::I256::from_str_radix("3141592653590", 10).unwrap()
        );
        assert_eq!(
            super::D76::<4>::e().to_bits(),
            crate::wide_int::I256::from_str_radix("27183", 10).unwrap()
        );
        // FromStr within i128 range
        let v = super::D76::<2>::from_str("1.50").unwrap();
        assert_eq!(v.to_bits(), crate::wide_int::I256::from_str_radix("150", 10).unwrap());
        let neg = super::D76::<2>::from_str("-20.50").unwrap();
        assert_eq!(neg.to_bits(), crate::wide_int::I256::from_str_radix("-2050", 10).unwrap());
        // num_traits Zero / One
        use ::num_traits::{One, Zero};
        assert!(super::D76::<6>::zero().is_zero());
        assert!(super::D76::<6>::one().is_one());
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_conversions() {
        use crate::wide_int::I256;
        type D = super::D76<6>;
        // From<primitive int>
        let from_i32: D = 5i32.into();
        assert_eq!(from_i32.to_bits(), I256::from_str_radix("5000000", 10).unwrap());
        let from_u64: D = 7u64.into();
        assert_eq!(from_u64.to_bits(), I256::from_str_radix("7000000", 10).unwrap());
        let from_neg: D = (-3i16).into();
        assert_eq!(from_neg.to_bits(), I256::from_str_radix("-3000000", 10).unwrap());
        // TryFrom<i128> / TryFrom<u128>
        let from_i128 = D::try_from(123i128).unwrap();
        assert_eq!(from_i128.to_bits(), I256::from_str_radix("123000000", 10).unwrap());
        let from_u128 = D::try_from(u128::MAX).unwrap();
        assert_eq!(
            from_u128.to_bits(),
            I256::from_str_radix("340282366920938463463374607431768211455", 10).unwrap()
                * I256::from_str_radix("1000000", 10).unwrap()
        );
        // TryFrom<f64>
        let from_f64 = D::try_from(2.5f64).unwrap();
        assert_eq!(from_f64.to_bits(), I256::from_str_radix("2500000", 10).unwrap());
        assert!(D::try_from(f64::NAN).is_err());
        // from_int / from_i32
        assert_eq!(D::from_int(9i128), D::from(9i32));
        assert_eq!(D::from_i32(-4), D::from(-4i32));
        // to_int: 2.5 with HalfToEven -> 2
        use crate::rounding::RoundingMode;
        let two_and_half = D::from_bits(I256::from_str_radix("2500000", 10).unwrap());
        assert_eq!(two_and_half.to_int_with(RoundingMode::HalfToEven), 2);
        assert_eq!(two_and_half.to_int_with(RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(two_and_half.to_int_with(RoundingMode::Ceiling), 3);
        assert_eq!(two_and_half.to_int_with(RoundingMode::Floor), 2);
        let neg_two_and_half = -two_and_half;
        assert_eq!(neg_two_and_half.to_int_with(RoundingMode::Floor), -3);
        assert_eq!(neg_two_and_half.to_int_with(RoundingMode::Trunc), -2);
        // cross-width widening D38 -> D76 (lossless)
        let d38: super::D38s6 = super::D38s6::from_bits(-150);
        let widened: super::D76<6> = d38.into();
        assert_eq!(widened.to_bits(), I256::from_str_radix("-150", 10).unwrap());
        // cross-width narrowing D76 -> D38 in range
        let in_range: super::D76<6> = super::D76::<6>::from_bits(I256::from_str_radix("999", 10).unwrap());
        let narrowed: super::D38s6 = in_range.try_into().unwrap();
        assert_eq!(narrowed.to_bits(), 999i128);
        // cross-width narrowing D76 -> D38 out of range
        let out_of_range = super::D76s76::MAX;
        let narrow_fail: Result<super::D38<76>, _> = out_of_range.try_into();
        assert!(narrow_fail.is_err());
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn d76_rescale_rounding_floats() {
        use crate::rounding::RoundingMode;
        use crate::wide_int::I256;
        type D6 = super::D76<6>;
        // rescale up (lossless): scale 6 -> scale 9
        let v = D6::from_bits(I256::from_str_radix("1500000", 10).unwrap()); // 1.5
        let up: super::D76<9> = v.rescale::<9>();
        assert_eq!(up.to_bits(), I256::from_str_radix("1500000000", 10).unwrap());
        // rescale down (lossy, HalfToEven): scale 6 -> scale 2
        let down: super::D76<2> = v.rescale::<2>();
        assert_eq!(down.to_bits(), I256::from_str_radix("150", 10).unwrap());
        // rescale down with explicit mode: 2.5 (scale 0 representation) ...
        let two_p_five = super::D76::<1>::from_bits(I256::from_str_radix("25", 10).unwrap());
        let r0: super::D76<0> = two_p_five.rescale_with::<0>(RoundingMode::HalfToEven);
        assert_eq!(r0.to_bits(), I256::from_str_radix("2", 10).unwrap());
        let r0b: super::D76<0> = two_p_five.rescale_with::<0>(RoundingMode::HalfAwayFromZero);
        assert_eq!(r0b.to_bits(), I256::from_str_radix("3", 10).unwrap());
        // floor / ceil / round / trunc / fract on 1.5 at scale 6
        assert_eq!(v.floor(), D6::ONE);
        assert_eq!(v.ceil(), D6::ONE + D6::ONE);
        assert_eq!(v.round(), D6::ONE + D6::ONE); // half away from zero
        assert_eq!(v.trunc(), D6::ONE);
        assert_eq!(v.fract(), D6::from_bits(I256::from_str_radix("500000", 10).unwrap()));
        // negative: -1.5
        let neg = -v;
        assert_eq!(neg.floor(), -(D6::ONE + D6::ONE));
        assert_eq!(neg.ceil(), -D6::ONE);
        assert_eq!(neg.round(), -(D6::ONE + D6::ONE));
        // float bridge
        let from_f = D6::from_f64(2.5);
        assert_eq!(from_f.to_bits(), I256::from_str_radix("2500000", 10).unwrap());
        assert_eq!(D6::from_f64(f64::NAN), D6::ZERO);
        assert_eq!(D6::from_f64(f64::INFINITY), D6::MAX);
        let round_trip = D6::ONE.to_f64();
        assert!((round_trip - 1.0).abs() < 1e-9);
    }

    #[cfg(any(feature = "d153", feature = "wide"))]
    #[test]
    fn d153_smoke() {
        use crate::decimal_trait::Decimal;
        use crate::arithmetic_trait::DecimalArithmetic;
        use crate::wide_int::I512;
        type D = super::D153<35>;
        assert_eq!(<D as DecimalArithmetic>::MAX_SCALE, 153);
        assert_eq!(D::ZERO.to_bits(), I512::from_str_radix("0", 10).unwrap());
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::from_int(6i128));
        assert_eq!((three * two) / two, three);
        assert_eq!(alloc::format!("{}", one).len(), "1.".len() + 35);
        assert_eq!(D::from_int(5i128).to_int(), 5);
        // rescale across the wide range
        let up: super::D153<150> = one.rescale::<150>();
        assert_eq!(up, super::D153::<150>::ONE);
        // 153-digit ceiling multiplier fits in I512
        let _ = super::D153s153::multiplier();
    }

    #[cfg(any(feature = "d307", feature = "wide"))]
    #[test]
    fn d307_smoke() {
        use crate::decimal_trait::Decimal;
        use crate::arithmetic_trait::DecimalArithmetic;
        use crate::wide_int::I1024;
        type D = super::D307<35>;
        assert_eq!(<D as DecimalArithmetic>::MAX_SCALE, 307);
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::from_int(6i128));
        assert_eq!((three * two) / two, three);
        assert_eq!(D::ZERO.to_bits(), I1024::from_str_radix("0", 10).unwrap());
        assert_eq!(alloc::format!("{}", one).len(), "1.".len() + 35);
        // cross-width: D76 -> D307 widening, D307 -> D76 narrowing
        #[cfg(any(feature = "d76", feature = "wide"))]
        {
            let small: super::D76<35> = super::D76::<35>::ONE;
            let widened: super::D307<35> = small.into();
            assert_eq!(widened, D::ONE);
            let narrowed: super::D76<35> = widened.try_into().unwrap();
            assert_eq!(narrowed, super::D76::<35>::ONE);
        }
        // 307-digit ceiling multiplier fits in I1024
        let _ = super::D307s307::multiplier();
    }

    #[test]
    fn d9_op_assign() {
        let mut v = super::D9s2::from_bits(100);
        v += super::D9s2::from_bits(50);
        assert_eq!(v.to_bits(), 150);
        v -= super::D9s2::from_bits(25);
        assert_eq!(v.to_bits(), 125);
        v *= super::D9s2::from_bits(200); // *2.00
        assert_eq!(v.to_bits(), 250);
        v /= super::D9s2::from_bits(200); // /2.00
        assert_eq!(v.to_bits(), 125);
        v %= super::D9s2::from_bits(100);
        assert_eq!(v.to_bits(), 25);
    }
}
