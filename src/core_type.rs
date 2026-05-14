//! Core type definition: [`D128`] and the concrete scale aliases
//! [`D128s0`] through [`D128s38`].
//!
//! `D128<const SCALE: u32>` is a `#[repr(transparent)]` newtype around
//! `i128`. The stored integer equals `actual_value * 10^SCALE`.
//!
//! The `#[repr(transparent)]` annotation is load-bearing: it guarantees
//! the same ABI as a bare `i128`, so `from_bits` / `to_bits` round-trips
//! are exact and the type is safe to embed in C-ABI plugin payloads.

/// Scaled fixed-point decimal with 128-bit storage.
///
/// `SCALE` is the base-10 exponent. A logical value `v` is stored as
/// `v * 10^SCALE` in the underlying `i128`. For example, with `SCALE = 12`
/// the number `1.5` is stored as `i128(1_500_000_000_000)`.
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
/// `Hash`, `Eq`, and `Ord` are derived from `i128`. Two `D128<S>` values
/// are equal if and only if their underlying `i128` fields are bit-equal.
/// This works because the scale is fixed at compile time -- each logical
/// value has exactly one representation.
///
/// # Const-generic scale
///
/// The const generic allows scale variants (`D128<9>`, `D128<6>`, etc.)
/// as trivial type aliases without duplicating any method implementations.
/// Mixed-scale arithmetic is deliberately not provided; callers convert
/// explicitly.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D128<const SCALE: u32>(pub i128);

// Manual `Debug` is implemented in `display.rs` and renders via `Display`
// so the canonical decimal string is shown rather than the raw i128.

/// `Default` returns `ZERO`, matching `i128::default() == 0`.
///
/// This lets `#[derive(Default)]` work correctly on structs that contain
/// `D128<S>` fields.
impl<const SCALE: u32> Default for D128<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

// Scale aliases: D128s0 through D128s38.
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

/// Scale alias: `D128<0>`. 1 LSB = 1 (thin `i128` wrapper, no rescale).
/// Range ~+/-1.7e38.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s0 = D128<0>;

/// Scale alias: `D128<1>`. 1 LSB = 10^-1 (1 decimal digit).
/// Range ~+/-1.7e37.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s1 = D128<1>;

/// Scale alias: `D128<2>`. 1 LSB = 10^-2 (cents). Range ~+/-1.7e36.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s2 = D128<2>;

/// Scale alias: `D128<3>`. 1 LSB = 10^-3 (thousandths; 1 mm at m units).
/// Range ~+/-1.7e35.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s3 = D128<3>;

/// Scale alias: `D128<4>`. 1 LSB = 10^-4 (basis points). Range ~+/-1.7e34.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s4 = D128<4>;

/// Scale alias: `D128<5>`. 1 LSB = 10^-5. Range ~+/-1.7e33.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s5 = D128<5>;

/// Scale alias: `D128<6>`. 1 LSB = 10^-6 (1 um at mm units; ppm).
/// Range ~+/-1.7e32.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s6 = D128<6>;

/// Scale alias: `D128<7>`. 1 LSB = 10^-7. Range ~+/-1.7e31.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s7 = D128<7>;

/// Scale alias: `D128<8>`. 1 LSB = 10^-8 (satoshi-grade). Range ~+/-1.7e30.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s8 = D128<8>;

/// Scale alias: `D128<9>`. 1 LSB = 10^-9 (1 nm at mm units; ppb).
/// Range ~+/-1.7e29.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s9 = D128<9>;

/// Scale alias: `D128<10>`. 1 LSB = 10^-10. Range ~+/-1.7e28.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s10 = D128<10>;

/// Scale alias: `D128<11>`. 1 LSB = 10^-11. Range ~+/-1.7e27.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s11 = D128<11>;

/// Scale alias: `D128<12>`. 1 LSB = 10^-12 (1 pm at mm units).
/// Range ~+/-1.7e14 model units.
///
/// This is the primary concrete alias for general use. At `SCALE = 12`:
/// - 1 LSB is `10^-12` model units.
/// - The representable integer range is approximately +/-1.7e14 model units.
/// - Squared-component operations (e.g. dot products) overflow beyond
///   roughly 13,000 km at mm units.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s12 = D128<12>;

/// Scale alias: `D128<13>`. 1 LSB = 10^-13. Range ~+/-1.7e25.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s13 = D128<13>;

/// Scale alias: `D128<14>`. 1 LSB = 10^-14. Range ~+/-1.7e24.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s14 = D128<14>;

/// Scale alias: `D128<15>`. 1 LSB = 10^-15 (femto). Range ~+/-1.7e23.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s15 = D128<15>;

/// Scale alias: `D128<16>`. 1 LSB = 10^-16. Range ~+/-1.7e22.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s16 = D128<16>;

/// Scale alias: `D128<17>`. 1 LSB = 10^-17. Range ~+/-1.7e21.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s17 = D128<17>;

/// Scale alias: `D128<18>`. 1 LSB = 10^-18 (atto; high-precision scientific).
/// Range ~+/-1.7e20.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s18 = D128<18>;

/// Scale alias: `D128<19>`. 1 LSB = 10^-19. Range ~+/-1.7e19.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s19 = D128<19>;

/// Scale alias: `D128<20>`. 1 LSB = 10^-20. Range ~+/-1.7e18.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s20 = D128<20>;

/// Scale alias: `D128<21>`. 1 LSB = 10^-21 (zepto). Range ~+/-1.7e17.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s21 = D128<21>;

/// Scale alias: `D128<22>`. 1 LSB = 10^-22. Range ~+/-1.7e16.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s22 = D128<22>;

/// Scale alias: `D128<23>`. 1 LSB = 10^-23. Range ~+/-1.7e15.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s23 = D128<23>;

/// Scale alias: `D128<24>`. 1 LSB = 10^-24 (yocto). Range ~+/-1.7e14.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s24 = D128<24>;

/// Scale alias: `D128<25>`. 1 LSB = 10^-25. Range ~+/-1.7e13.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s25 = D128<25>;

/// Scale alias: `D128<26>`. 1 LSB = 10^-26. Range ~+/-1.7e12.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s26 = D128<26>;

/// Scale alias: `D128<27>`. 1 LSB = 10^-27. Range ~+/-1.7e11.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s27 = D128<27>;

/// Scale alias: `D128<28>`. 1 LSB = 10^-28. Range ~+/-1.7e10.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s28 = D128<28>;

/// Scale alias: `D128<29>`. 1 LSB = 10^-29. Range ~+/-1.7e9.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s29 = D128<29>;

/// Scale alias: `D128<30>`. 1 LSB = 10^-30. Range ~+/-1.7e8.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s30 = D128<30>;

/// Scale alias: `D128<31>`. 1 LSB = 10^-31. Range ~+/-1.7e7.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s31 = D128<31>;

/// Scale alias: `D128<32>`. 1 LSB = 10^-32. Range ~+/-1.7e6.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s32 = D128<32>;

/// Scale alias: `D128<33>`. 1 LSB = 10^-33. Range ~+/-1.7e5.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s33 = D128<33>;

/// Scale alias: `D128<34>`. 1 LSB = 10^-34. Range ~+/-1.7e4.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s34 = D128<34>;

/// Scale alias: `D128<35>`. 1 LSB = 10^-35. Range ~+/-1.7e3.
///
/// Matches `SCALE_REF` in `consts.rs`: the math constants `pi`, `tau`,
/// `e`, and `golden` are stored at this reference scale internally, so
/// at `SCALE = 35` they round-trip without precision loss.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s35 = D128<35>;

/// Scale alias: `D128<36>`. 1 LSB = 10^-36. Range ~+/-170.
///
/// The math constants (`pi`, `tau`, `e`, `golden`) are stored at a
/// 35-digit reference. Above `SCALE = 35` they are scaled up from that
/// reference, so trailing digits are zero-extended rather than
/// meaningfully precise.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s36 = D128<36>;

/// Scale alias: `D128<37>`. 1 LSB = 10^-37. Range ~+/-17.
///
/// Math constants lose precision above `SCALE = 35`; see `D128s36`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s37 = D128<37>;

/// Scale alias: `D128<38>`. 1 LSB = 10^-38. Range ~+/-1.7
/// (sub-unit dimensionless ratios).
///
/// This is the maximum supported scale. `10^38 < i128::MAX < 10^39`, so
/// `SCALE = 39` is not supported (`multiplier()` would overflow). Math
/// constants lose precision above `SCALE = 35`; see `D128s36`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
pub type D128s38 = D128<38>;

// The `ParseError` enum lives in `src/error.rs` and is re-exported
// from the crate root. It is not width-specific.
pub use crate::error::ParseError;

// Inherent basics + Decimal trait impl: emitted by the macro generator
// (one invocation per width). See src/decimal_macro.rs for the macro
// definition and the surface it produces.
crate::macros::basics::decl_decimal_basics!(D128, i128, 38);
crate::macros::display::decl_decimal_display!(D128);
crate::macros::float_bridge::decl_decimal_float_bridge!(D128, i128);
crate::macros::conversions::decl_from_primitive!(D128, i128, i8);
crate::macros::conversions::decl_from_primitive!(D128, i128, i16);
crate::macros::conversions::decl_from_primitive!(D128, i128, i32);
crate::macros::conversions::decl_from_primitive!(D128, i128, i64);
crate::macros::conversions::decl_from_primitive!(D128, i128, u8);
crate::macros::conversions::decl_from_primitive!(D128, i128, u16);
crate::macros::conversions::decl_from_primitive!(D128, i128, u32);
crate::macros::conversions::decl_from_primitive!(D128, i128, u64);
crate::macros::conversions::decl_try_from_i128!(D128, i128);
crate::macros::conversions::decl_try_from_u128!(D128, i128);
crate::macros::conversions::decl_try_from_i128!(D64, i64);
crate::macros::conversions::decl_try_from_u128!(D64, i64);
crate::macros::conversions::decl_try_from_i128!(D32, i32);
crate::macros::conversions::decl_try_from_u128!(D32, i32);
crate::macros::conversions::decl_try_from_f64!(D128, i128);
crate::macros::conversions::decl_try_from_f32!(D128, i128);
crate::macros::conversions::decl_try_from_f64!(D64, i64);
crate::macros::conversions::decl_try_from_f32!(D64, i64);
crate::macros::conversions::decl_try_from_f64!(D32, i32);
crate::macros::conversions::decl_try_from_f32!(D32, i32);
crate::macros::conversions::decl_decimal_int_conversion_methods!(D128, i128, i64);
crate::macros::conversions::decl_decimal_int_conversion_methods!(D64, i64, i64);
crate::macros::conversions::decl_decimal_int_conversion_methods!(D32, i32, i32);

// ---------------------------------------------------------------------
// D32 — 32-bit storage, scale 0..=9. Embedded / register-sized ledger
// type. SCALE = 9 fits ~21.5 with 9 decimal digits of precision; SCALE
// = 0 covers ±2.1 × 10⁹ unscaled. Only the basics emitted in this
// sub-phase; arithmetic / display / num_traits land incrementally.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 32-bit storage. See [`D128`] for the
/// shape documentation; D32 has the same surface scaled to `i32` and
/// `MAX_SCALE = 9`.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D32<const SCALE: u32>(pub i32);

impl<const SCALE: u32> Default for D32<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

crate::macros::basics::decl_decimal_basics!(D32, i32, 9);
crate::macros::arithmetic::decl_decimal_arithmetic!(D32, i32, i64);
crate::macros::conversions::decl_from_primitive!(D32, i32, i8);
crate::macros::conversions::decl_from_primitive!(D32, i32, i16);
crate::macros::conversions::decl_from_primitive!(D32, i32, u8);
crate::macros::conversions::decl_from_primitive!(D32, i32, u16);
crate::macros::display::decl_decimal_display!(D32);
crate::macros::overflow::decl_decimal_overflow_variants!(D32, i32, i64);
crate::macros::num_traits::decl_decimal_num_traits_basics!(D32);
crate::macros::sign::decl_decimal_sign_methods!(D32, i32);
crate::macros::consts::decl_decimal_consts!(D32, i32);
crate::macros::from_str::decl_decimal_from_str!(D32, i32);
crate::macros::float_bridge::decl_decimal_float_bridge!(D32, i32);
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D32);
crate::macros::strict_transcendentals::decl_strict_transcendentals_via_d128!(D32);
crate::macros::rounding_methods::decl_decimal_rounding_methods!(D32);
crate::macros::helpers::decl_decimal_helpers!(D32);

/// Scale alias: `D32<0>`. 1 LSB = 1 (thin `i32` wrapper). Range ±2.1 × 10⁹.
pub type D32s0 = D32<0>;
/// Scale alias: `D32<1>`. 1 LSB = 10^-1. Range ±2.1 × 10⁸.
pub type D32s1 = D32<1>;
/// Scale alias: `D32<2>`. 1 LSB = 10^-2 (cents). Range ±2.1 × 10⁷.
pub type D32s2 = D32<2>;
/// Scale alias: `D32<3>`. 1 LSB = 10^-3 (mills). Range ±2.1 × 10⁶.
pub type D32s3 = D32<3>;
/// Scale alias: `D32<4>`. 1 LSB = 10^-4 (basis points). Range ±2.1 × 10⁵.
pub type D32s4 = D32<4>;
/// Scale alias: `D32<5>`. 1 LSB = 10^-5. Range ±2.1 × 10⁴.
pub type D32s5 = D32<5>;
/// Scale alias: `D32<6>`. 1 LSB = 10^-6 (ppm). Range ±2.1 × 10³.
pub type D32s6 = D32<6>;
/// Scale alias: `D32<7>`. 1 LSB = 10^-7. Range ±214.
pub type D32s7 = D32<7>;
/// Scale alias: `D32<8>`. 1 LSB = 10^-8 (satoshi). Range ±21.4.
pub type D32s8 = D32<8>;
/// Scale alias: `D32<9>`. 1 LSB = 10^-9 (nano). Range ±2.1.
pub type D32s9 = D32<9>;

// ---------------------------------------------------------------------
// D64 — 64-bit storage, scale 0..=18. Interchange size; fits a GPR on
// 64-bit hosts and maps cleanly to ANSI / SQL `DECIMAL(18, S)` columns.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 64-bit storage. See [`D128`] for the
/// shape documentation; D64 has the same surface scaled to `i64` and
/// `MAX_SCALE = 18`.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D64<const SCALE: u32>(pub i64);

impl<const SCALE: u32> Default for D64<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

crate::macros::basics::decl_decimal_basics!(D64, i64, 18);
crate::macros::arithmetic::decl_decimal_arithmetic!(D64, i64, i128);
crate::macros::conversions::decl_from_primitive!(D64, i64, i8);
crate::macros::conversions::decl_from_primitive!(D64, i64, i16);
crate::macros::conversions::decl_from_primitive!(D64, i64, i32);
crate::macros::conversions::decl_from_primitive!(D64, i64, u8);
crate::macros::conversions::decl_from_primitive!(D64, i64, u16);
crate::macros::conversions::decl_from_primitive!(D64, i64, u32);
crate::macros::display::decl_decimal_display!(D64);
crate::macros::overflow::decl_decimal_overflow_variants!(D64, i64, i128);
crate::macros::num_traits::decl_decimal_num_traits_basics!(D64);
crate::macros::sign::decl_decimal_sign_methods!(D64, i64);
crate::macros::consts::decl_decimal_consts!(D64, i64);
crate::macros::from_str::decl_decimal_from_str!(D64, i64);
crate::macros::float_bridge::decl_decimal_float_bridge!(D64, i64);
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D64);
crate::macros::strict_transcendentals::decl_strict_transcendentals_via_d128!(D64);
crate::macros::rounding_methods::decl_decimal_rounding_methods!(D64);
crate::macros::helpers::decl_decimal_helpers!(D64);

// Cross-width widening (lossless). D32 -> D64, D32 -> D128, D64 -> D128.
crate::macros::conversions::decl_cross_width_widening!(D64, i64, D32, i32);
crate::macros::conversions::decl_cross_width_widening!(D128, i128, D32, i32);
crate::macros::conversions::decl_cross_width_widening!(D128, i128, D64, i64);

// Cross-width narrowing (fallible). D128 -> D64, D128 -> D32, D64 -> D32.
crate::macros::conversions::decl_cross_width_narrowing!(D64, i64, D128, i128);
crate::macros::conversions::decl_cross_width_narrowing!(D32, i32, D128, i128);
crate::macros::conversions::decl_cross_width_narrowing!(D32, i32, D64, i64);

/// Scale alias: `D64<0>`. 1 LSB = 1. Range ±9.2 × 10¹⁸.
pub type D64s0 = D64<0>;
/// Scale alias: `D64<1>`. 1 LSB = 10^-1. Range ±9.2 × 10¹⁷.
pub type D64s1 = D64<1>;
/// Scale alias: `D64<2>`. 1 LSB = 10^-2 (cents). Range ±9.2 × 10¹⁶.
pub type D64s2 = D64<2>;
/// Scale alias: `D64<3>`. 1 LSB = 10^-3 (mills). Range ±9.2 × 10¹⁵.
pub type D64s3 = D64<3>;
/// Scale alias: `D64<4>`. 1 LSB = 10^-4 (basis points). Range ±9.2 × 10¹⁴.
pub type D64s4 = D64<4>;
/// Scale alias: `D64<5>`. 1 LSB = 10^-5. Range ±9.2 × 10¹³.
pub type D64s5 = D64<5>;
/// Scale alias: `D64<6>`. 1 LSB = 10^-6 (ppm). Range ±9.2 × 10¹².
pub type D64s6 = D64<6>;
/// Scale alias: `D64<7>`. 1 LSB = 10^-7. Range ±9.2 × 10¹¹.
pub type D64s7 = D64<7>;
/// Scale alias: `D64<8>`. 1 LSB = 10^-8 (satoshi). Range ±9.2 × 10¹⁰.
pub type D64s8 = D64<8>;
/// Scale alias: `D64<9>`. 1 LSB = 10^-9 (nano). Range ±9.2 × 10⁹.
pub type D64s9 = D64<9>;
/// Scale alias: `D64<10>`. 1 LSB = 10^-10. Range ±9.2 × 10⁸.
pub type D64s10 = D64<10>;
/// Scale alias: `D64<11>`. 1 LSB = 10^-11. Range ±9.2 × 10⁷.
pub type D64s11 = D64<11>;
/// Scale alias: `D64<12>`. 1 LSB = 10^-12 (pico). Range ±9.2 × 10⁶.
pub type D64s12 = D64<12>;
/// Scale alias: `D64<13>`. 1 LSB = 10^-13. Range ±9.2 × 10⁵.
pub type D64s13 = D64<13>;
/// Scale alias: `D64<14>`. 1 LSB = 10^-14. Range ±9.2 × 10⁴.
pub type D64s14 = D64<14>;
/// Scale alias: `D64<15>`. 1 LSB = 10^-15 (femto). Range ±9200.
pub type D64s15 = D64<15>;
/// Scale alias: `D64<16>`. 1 LSB = 10^-16. Range ±920.
pub type D64s16 = D64<16>;
/// Scale alias: `D64<17>`. 1 LSB = 10^-17. Range ±92.
pub type D64s17 = D64<17>;
/// Scale alias: `D64<18>`. 1 LSB = 10^-18 (atto). Range ±9.2.
pub type D64s18 = D64<18>;

// ---------------------------------------------------------------------
// D256 — 256-bit storage (bnum `I256`), scale 0..=76. First of the
// wide tier; gated behind the `d256` / `wide` Cargo features. Covers
// the full IEEE-754 decimal128 range and gives 35-digit fractional
// precision with integer-part headroom (see research doc §1).
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 256-bit storage. See [`D128`] for the
/// shape documentation; D256 has the same surface scaled to a 256-bit
/// signed integer and `MAX_SCALE = 76`.
///
/// Gated behind the `d256` (or umbrella `wide`) Cargo feature. The
/// storage backend is `bnum`'s `I256`; this is the interim backend per
/// `research/multi_width_decimals.md` §3.
#[cfg(any(feature = "d256", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D256<const SCALE: u32>(pub crate::wide::I256);

#[cfg(any(feature = "d256", feature = "wide"))]
impl<const SCALE: u32> Default for D256<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::basics::decl_decimal_basics!(wide D256, crate::wide::I256, 76);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::arithmetic::decl_decimal_arithmetic!(wide D256, crate::wide::I256, crate::wide::I512);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::display::decl_decimal_display!(wide D256, crate::wide::U256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::overflow::decl_decimal_overflow_variants!(wide D256, crate::wide::I256, crate::wide::I512);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::num_traits::decl_decimal_num_traits_basics!(D256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::sign::decl_decimal_sign_methods!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::consts::decl_decimal_consts!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::from_str::decl_decimal_from_str!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::helpers::decl_decimal_helpers!(wide D256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, i8);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, i16);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, i32);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, i64);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, u8);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, u16);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, u32);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D256, crate::wide::I256, u64);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_try_from_i128!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_try_from_u128!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_try_from_f64!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_try_from_f32!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_decimal_int_conversion_methods!(wide D256, crate::wide::I256, i128);
// Cross-width widening into D256 (lossless): D32 / D64 / D128 -> D256.
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D256, crate::wide::I256, D32, i32);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D256, crate::wide::I256, D64, i64);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D256, crate::wide::I256, D128, i128);
// Cross-width narrowing from D256 (fallible): D256 -> D128 / D64 / D32.
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D128, i128, D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D64, i64, D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D32, i32, D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::float_bridge::decl_decimal_float_bridge!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::rescale::decl_decimal_rescale!(wide D256, crate::wide::I256);
#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::rounding_methods::decl_decimal_rounding_methods!(wide D256);

/// Scale alias: `D256<0>`. 1 LSB = 1 (256-bit integer ledger).
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s0 = D256<0>;
/// Scale alias: `D256<2>`. 1 LSB = 10^-2 (cents).
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s2 = D256<2>;
/// Scale alias: `D256<6>`. 1 LSB = 10^-6 (ppm).
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s6 = D256<6>;
/// Scale alias: `D256<12>`. 1 LSB = 10^-12 (pico; financial standard).
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s12 = D256<12>;
/// Scale alias: `D256<18>`. 1 LSB = 10^-18 (atto).
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s18 = D256<18>;
/// Scale alias: `D256<35>`. 1 LSB = 10^-35 (matches `SCALE_REF`).
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s35 = D256<35>;
/// Scale alias: `D256<50>`. 1 LSB = 10^-50 (deep scientific precision).
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s50 = D256<50>;
/// Scale alias: `D256<76>`. 1 LSB = 10^-76. Maximum supported scale.
#[cfg(any(feature = "d256", feature = "wide"))]
pub type D256s76 = D256<76>;

// ---------------------------------------------------------------------
// D512 — 512-bit storage (bnum `I512`), scale 0..=153. Wide-scientific
// tier; gated behind the `d512` / `wide` Cargo features.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 512-bit storage. See [`D128`] for the
/// shape documentation; D512 has the same surface scaled to a 512-bit
/// signed integer and `MAX_SCALE = 153`.
///
/// Gated behind the `d512` (or umbrella `wide`) Cargo feature. The
/// storage backend is `bnum`'s `I512`.
#[cfg(any(feature = "d512", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D512<const SCALE: u32>(pub crate::wide::I512);

#[cfg(any(feature = "d512", feature = "wide"))]
impl<const SCALE: u32> Default for D512<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::basics::decl_decimal_basics!(wide D512, crate::wide::I512, 153);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::arithmetic::decl_decimal_arithmetic!(wide D512, crate::wide::I512, crate::wide::I1024);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::display::decl_decimal_display!(wide D512, crate::wide::U512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::overflow::decl_decimal_overflow_variants!(wide D512, crate::wide::I512, crate::wide::I1024);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::num_traits::decl_decimal_num_traits_basics!(D512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::sign::decl_decimal_sign_methods!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::consts::decl_decimal_consts!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::from_str::decl_decimal_from_str!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::helpers::decl_decimal_helpers!(wide D512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, i8);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, i16);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, i32);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, i64);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, u8);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, u16);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, u32);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D512, crate::wide::I512, u64);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_try_from_i128!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_try_from_u128!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_try_from_f64!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_try_from_f32!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_decimal_int_conversion_methods!(wide D512, crate::wide::I512, i128);
// Cross-width widening into D512 (lossless): D128 / D256 -> D512.
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_cross_width_widening!(wide D512, crate::wide::I512, D128, i128);
#[cfg(all(any(feature = "d512", feature = "wide"), any(feature = "d256", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D512, crate::wide::I512, D256, crate::wide::I256);
// Cross-width narrowing from D512 (fallible): D512 -> D256 / D128.
#[cfg(all(any(feature = "d512", feature = "wide"), any(feature = "d256", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D256, crate::wide::I256, D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D128, i128, D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::float_bridge::decl_decimal_float_bridge!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::rescale::decl_decimal_rescale!(wide D512, crate::wide::I512);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::rounding_methods::decl_decimal_rounding_methods!(wide D512);

/// Scale alias: `D512<0>`. 1 LSB = 1 (512-bit integer ledger).
#[cfg(any(feature = "d512", feature = "wide"))]
pub type D512s0 = D512<0>;
/// Scale alias: `D512<35>`. 1 LSB = 10^-35 (matches `SCALE_REF`).
#[cfg(any(feature = "d512", feature = "wide"))]
pub type D512s35 = D512<35>;
/// Scale alias: `D512<75>`. 1 LSB = 10^-75 (wide-scientific midpoint).
#[cfg(any(feature = "d512", feature = "wide"))]
pub type D512s75 = D512<75>;
/// Scale alias: `D512<150>`. 1 LSB = 10^-150.
#[cfg(any(feature = "d512", feature = "wide"))]
pub type D512s150 = D512<150>;
/// Scale alias: `D512<153>`. 1 LSB = 10^-153. Maximum supported scale.
#[cfg(any(feature = "d512", feature = "wide"))]
pub type D512s153 = D512<153>;

// ---------------------------------------------------------------------
// D1024 — 1024-bit storage (bnum `I1024`), scale 0..=307. Deep
// arbitrary-precision tier; gated behind the `d1024` / `wide` features.
// ---------------------------------------------------------------------

/// Scaled fixed-point decimal with 1024-bit storage. See [`D128`] for
/// the shape documentation; D1024 has the same surface scaled to a
/// 1024-bit signed integer and `MAX_SCALE = 307`.
///
/// Gated behind the `d1024` (or umbrella `wide`) Cargo feature. The
/// storage backend is `bnum`'s `I1024`.
#[cfg(any(feature = "d1024", feature = "wide"))]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct D1024<const SCALE: u32>(pub crate::wide::I1024);

#[cfg(any(feature = "d1024", feature = "wide"))]
impl<const SCALE: u32> Default for D1024<SCALE> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::basics::decl_decimal_basics!(wide D1024, crate::wide::I1024, 307);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::arithmetic::decl_decimal_arithmetic!(wide D1024, crate::wide::I1024, crate::wide::I2048);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::display::decl_decimal_display!(wide D1024, crate::wide::U1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::overflow::decl_decimal_overflow_variants!(wide D1024, crate::wide::I1024, crate::wide::I2048);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::num_traits::decl_decimal_num_traits_basics!(D1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::sign::decl_decimal_sign_methods!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::consts::decl_decimal_consts!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::from_str::decl_decimal_from_str!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::storage_formatters::decl_decimal_storage_formatters!(D1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::helpers::decl_decimal_helpers!(wide D1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, i8);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, i16);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, i32);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, i64);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, u8);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, u16);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, u32);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_from_primitive!(wide D1024, crate::wide::I1024, u64);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_try_from_i128!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_try_from_u128!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_try_from_f64!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_try_from_f32!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::conversions::decl_decimal_int_conversion_methods!(wide D1024, crate::wide::I1024, i128);
// Cross-width widening into D1024 (lossless): D256 / D512 -> D1024.
#[cfg(all(any(feature = "d1024", feature = "wide"), any(feature = "d256", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D1024, crate::wide::I1024, D256, crate::wide::I256);
#[cfg(all(any(feature = "d1024", feature = "wide"), any(feature = "d512", feature = "wide")))]
crate::macros::conversions::decl_cross_width_widening!(wide D1024, crate::wide::I1024, D512, crate::wide::I512);
// Cross-width narrowing from D1024 (fallible): D1024 -> D512 / D256.
#[cfg(all(any(feature = "d1024", feature = "wide"), any(feature = "d512", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D512, crate::wide::I512, D1024, crate::wide::I1024);
#[cfg(all(any(feature = "d1024", feature = "wide"), any(feature = "d256", feature = "wide")))]
crate::macros::conversions::decl_cross_width_narrowing!(wide D256, crate::wide::I256, D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::float_bridge::decl_decimal_float_bridge!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::rescale::decl_decimal_rescale!(wide D1024, crate::wide::I1024);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::rounding_methods::decl_decimal_rounding_methods!(wide D1024);

/// Scale alias: `D1024<0>`. 1 LSB = 1 (1024-bit integer ledger).
#[cfg(any(feature = "d1024", feature = "wide"))]
pub type D1024s0 = D1024<0>;
/// Scale alias: `D1024<35>`. 1 LSB = 10^-35 (matches `SCALE_REF`).
#[cfg(any(feature = "d1024", feature = "wide"))]
pub type D1024s35 = D1024<35>;
/// Scale alias: `D1024<150>`. 1 LSB = 10^-150.
#[cfg(any(feature = "d1024", feature = "wide"))]
pub type D1024s150 = D1024<150>;
/// Scale alias: `D1024<300>`. 1 LSB = 10^-300.
#[cfg(any(feature = "d1024", feature = "wide"))]
pub type D1024s300 = D1024<300>;
/// Scale alias: `D1024<307>`. 1 LSB = 10^-307. Maximum supported scale.
#[cfg(any(feature = "d1024", feature = "wide"))]
pub type D1024s307 = D1024<307>;

#[cfg(test)]
mod tests {
    use super::*;

    /// `from_bits` / `to_bits` round-trip is exact.
    #[test]
    fn from_bits_to_bits_round_trip() {
        let raw: i128 = 1_500_000_000_000;
        let v: D128s12 = D128s12::from_bits(raw);
        assert_eq!(v.to_bits(), raw);
    }

    /// `ZERO` has raw bit value 0.
    #[test]
    fn zero_is_zero_bits() {
        assert_eq!(D128s12::ZERO.to_bits(), 0);
    }

    /// Two instances with identical raw bits compare equal.
    #[test]
    fn equal_by_underlying_bits() {
        assert_eq!(
            D128s12::from_bits(42_000_000_000_000),
            D128s12::from_bits(42_000_000_000_000)
        );
        assert_ne!(D128s12::from_bits(42), D128s12::from_bits(43));
    }

    /// Ord is derived from i128: smaller bits compare less.
    #[test]
    fn ord_by_underlying_bits() {
        assert!(D128s12::from_bits(1) < D128s12::from_bits(2));
        assert!(D128s12::from_bits(-1) < D128s12::from_bits(0));
    }

    /// `multiplier()` returns 10^SCALE. At SCALE = 12 that is 10^12.
    #[test]
    fn multiplier_is_ten_to_scale() {
        assert_eq!(D128s12::multiplier(), 1_000_000_000_000_i128);
    }

    /// `SCALE` associated const returns the const-generic scale.
    #[test]
    fn scale_const_matches_type_parameter() {
        assert_eq!(D128s12::SCALE, 12);
        const N: u32 = D128s12::SCALE;
        assert_eq!(N, 12);
    }

    /// `scale()` method returns the const-generic scale and is
    /// independent of the instance's value.
    #[test]
    fn scale_method_matches_type_parameter() {
        assert_eq!(D128s12::ZERO.scale(), 12);
        assert_eq!(D128s12::ONE.scale(), 12);
        assert_eq!(D128s12::from_bits(i128::MAX).scale(), 12);
        assert_eq!(D128s12::from_bits(-7).scale(), 12);
    }

    /// Both forms agree at non-default scales.
    #[test]
    fn scale_at_other_scales() {
        type D6 = super::D128<6>;
        type D0 = super::D128<0>;
        type D38 = super::D128<38>;
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
        assert_eq!(D128s12::ONE.to_bits(), 1_000_000_000_000_i128);
    }

    /// `MAX` is `i128::MAX`.
    #[test]
    fn max_is_i128_max() {
        assert_eq!(D128s12::MAX.to_bits(), i128::MAX);
    }

    /// `MIN` is `i128::MIN`.
    #[test]
    fn min_is_i128_min() {
        assert_eq!(D128s12::MIN.to_bits(), i128::MIN);
    }

    /// `ONE` is not equal to `ZERO`.
    #[test]
    fn one_is_not_zero() {
        assert_ne!(D128s12::ONE, D128s12::ZERO);
        assert!(D128s12::ONE.is_positive());
    }

    /// `multiplier()` works correctly at non-default scales.
    #[test]
    fn multiplier_at_other_scales() {
        type D6 = super::D128<6>;
        assert_eq!(D6::multiplier(), 1_000_000_i128);
        assert_eq!(D6::ONE.to_bits(), 1_000_000_i128);

        type D0 = super::D128<0>;
        assert_eq!(D0::multiplier(), 1_i128);
        assert_eq!(D0::ONE.to_bits(), 1_i128);
    }

    // ----- D32 / D64 sanity tests -----

    #[test]
    fn d32_basics() {
        assert_eq!(super::D32s2::ZERO.to_bits(), 0_i32);
        assert_eq!(super::D32s2::ONE.to_bits(), 100_i32);
        assert_eq!(super::D32s2::MAX.to_bits(), i32::MAX);
        assert_eq!(super::D32s2::MIN.to_bits(), i32::MIN);
        assert_eq!(super::D32s2::multiplier(), 100_i32);
        assert_eq!(super::D32s2::SCALE, 2);
    }

    #[test]
    fn d64_basics() {
        assert_eq!(super::D64s9::ZERO.to_bits(), 0_i64);
        assert_eq!(super::D64s9::ONE.to_bits(), 1_000_000_000_i64);
        assert_eq!(super::D64s9::multiplier(), 1_000_000_000_i64);
        assert_eq!(super::D64s9::SCALE, 9);
    }

    #[test]
    fn d32_arithmetic() {
        let a = super::D32s2::from_bits(150); // 1.50
        let b = super::D32s2::from_bits(250); // 2.50
        assert_eq!((a + b).to_bits(), 400);
        assert_eq!((b - a).to_bits(), 100);
        assert_eq!((-a).to_bits(), -150);

        let x = super::D32s2::from_bits(200); // 2.00
        let y = super::D32s2::from_bits(300); // 3.00
        assert_eq!((x * y).to_bits(), 600); // 6.00
        assert_eq!((y / x).to_bits(), 150); // 1.50
        assert_eq!((y % x).to_bits(), 100); // 1.00
    }

    #[test]
    fn d64_arithmetic() {
        let a = super::D64s9::from_bits(1_500_000_000); // 1.5
        let b = super::D64s9::from_bits(2_500_000_000); // 2.5
        assert_eq!((a + b).to_bits(), 4_000_000_000);
        assert_eq!((b - a).to_bits(), 1_000_000_000);
        assert_eq!((-a).to_bits(), -1_500_000_000);

        let x = super::D64s9::from_bits(2_000_000_000); // 2.0
        let y = super::D64s9::from_bits(3_000_000_000); // 3.0
        assert_eq!((x * y).to_bits(), 6_000_000_000);
        assert_eq!((y / x).to_bits(), 1_500_000_000);
        assert_eq!((y % x).to_bits(), 1_000_000_000);
    }

    #[test]
    fn d32_display() {
        let v: super::D32s2 = super::D32s2::from_bits(150); // 1.50
        let s = alloc::format!("{}", v);
        assert_eq!(s, "1.50");
        let neg: super::D32s2 = super::D32s2::from_bits(-2050); // -20.50
        assert_eq!(alloc::format!("{}", neg), "-20.50");
        let zero: super::D32s2 = super::D32s2::ZERO;
        assert_eq!(alloc::format!("{}", zero), "0.00");
        let int_only: super::D32s0 = super::D32s0::from_bits(42);
        assert_eq!(alloc::format!("{}", int_only), "42");
    }

    #[test]
    fn d64_display() {
        let v: super::D64s9 = super::D64s9::from_bits(1_500_000_000); // 1.500000000
        assert_eq!(alloc::format!("{}", v), "1.500000000");
        let neg: super::D64s9 = super::D64s9::from_bits(-1_500_000_000);
        assert_eq!(alloc::format!("{}", neg), "-1.500000000");
    }

    #[test]
    fn d32_debug() {
        let v: super::D32s2 = super::D32s2::from_bits(150);
        let s = alloc::format!("{:?}", v);
        assert_eq!(s, "D32<2>(1.50)");
    }

    #[test]
    fn cross_width_widening_d32_to_d64() {
        let small: super::D32s2 = super::D32s2::from_bits(150);
        let wider: super::D64s2 = small.into();
        assert_eq!(wider.to_bits(), 150_i64);
    }

    #[test]
    fn cross_width_widening_d32_to_d128() {
        let small: super::D32s2 = super::D32s2::from_bits(-150);
        let wider: super::D128s2 = small.into();
        assert_eq!(wider.to_bits(), -150_i128);
    }

    #[test]
    fn cross_width_widening_d64_to_d128() {
        let mid: super::D64s9 = super::D64s9::from_bits(i64::MAX);
        let wider: super::D128s9 = mid.into();
        assert_eq!(wider.to_bits(), i64::MAX as i128);
    }

    #[test]
    fn cross_width_narrowing_d128_to_d64_in_range() {
        let wide: super::D128s9 = super::D128s9::from_bits(1_500_000_000);
        let narrow: super::D64s9 = wide.try_into().unwrap();
        assert_eq!(narrow.to_bits(), 1_500_000_000);
    }

    #[test]
    fn cross_width_narrowing_d128_to_d64_out_of_range() {
        let wide: super::D128s9 = super::D128s9::from_bits(i128::MAX);
        let narrow: Result<super::D64s9, _> = wide.try_into();
        assert!(narrow.is_err());
    }

    #[test]
    fn cross_width_narrowing_d64_to_d32_in_range() {
        let mid: super::D64s2 = super::D64s2::from_bits(150);
        let narrow: super::D32s2 = mid.try_into().unwrap();
        assert_eq!(narrow.to_bits(), 150);
    }

    #[test]
    fn cross_width_narrowing_d64_to_d32_out_of_range() {
        let mid: super::D64s2 = super::D64s2::from_bits(i64::MAX);
        let narrow: Result<super::D32s2, _> = mid.try_into();
        assert!(narrow.is_err());
    }

    #[test]
    fn d32_consts() {
        use crate::consts::DecimalConsts;
        type D32s4 = super::D32<4>;
        // pi at scale 4 = 3.1416 -> bits = 31416.
        assert_eq!(D32s4::pi().to_bits(), 31416);
        // e at scale 4 = 2.7183 -> bits = 27183.
        assert_eq!(D32s4::e().to_bits(), 27183);
    }

    #[test]
    fn d32_from_str() {
        use core::str::FromStr;
        let v = super::D32s2::from_str("1.50").unwrap();
        assert_eq!(v.to_bits(), 150);
        let neg = super::D32s2::from_str("-20.50").unwrap();
        assert_eq!(neg.to_bits(), -2050);
        // Out of range for D32s2 (i32::MAX is ~2.1e9).
        assert!(super::D32s2::from_str("1000000000000.00").is_err());
    }

    #[test]
    fn d64_from_str() {
        use core::str::FromStr;
        let v = super::D64s9::from_str("1.500000000").unwrap();
        assert_eq!(v.to_bits(), 1_500_000_000);
        let neg = super::D64s9::from_str("-1.500000000").unwrap();
        assert_eq!(neg.to_bits(), -1_500_000_000);
    }

    #[test]
    fn d64_consts() {
        use crate::consts::DecimalConsts;
        type D64s12 = super::D64<12>;
        // pi at scale 12 = 3.141592653590 (matches D128s12).
        assert_eq!(D64s12::pi().to_bits(), 3_141_592_653_590);
        // tau at scale 12 = 6.283185307180.
        assert_eq!(D64s12::tau().to_bits(), 6_283_185_307_180);
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_basics() {
        use crate::decimal_trait::Decimal;
        use crate::wide::I256;
        assert_eq!(super::D256s2::ZERO.to_bits(), I256::from_str_radix("0", 10).unwrap());
        assert_eq!(super::D256s2::ONE.to_bits(), I256::from_str_radix("100", 10).unwrap());
        assert_eq!(super::D256s2::MAX.to_bits(), I256::MAX);
        assert_eq!(super::D256s2::MIN.to_bits(), I256::MIN);
        assert_eq!(super::D256s2::multiplier(), I256::from_str_radix("100", 10).unwrap());
        assert_eq!(super::D256s2::SCALE, 2);
        assert_eq!(super::D256s2::ZERO.scale(), 2);
        // SCALE = 76 multiplier is 10^76, well within 256-bit range.
        let m76 = super::D256s76::multiplier();
        assert_eq!(
            m76,
            I256::from_str_radix("10000000000000000000000000000000000000000000000000000000000000000000000000000", 10).unwrap()
        );
        assert_eq!(<super::D256s12 as Decimal>::MAX_SCALE, 76);
        // round-trip
        let raw = I256::from_str_radix("123456789012345678901234567890", 10).unwrap();
        assert_eq!(super::D256s12::from_bits(raw).to_bits(), raw);
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_arithmetic() {
        type D = super::D256<12>;
        let one = D::ONE;
        let two = D::from_bits(D::multiplier() + D::multiplier());
        let three = D::from_bits(D::multiplier() * crate::wide::I256::from_str_radix("3", 10).unwrap());
        // add / sub / neg
        assert_eq!((one + two), three);
        assert_eq!((three - one), two);
        assert_eq!((-one).to_bits(), -D::multiplier());
        // mul: 2 * 3 == 6
        let six = D::from_bits(D::multiplier() * crate::wide::I256::from_str_radix("6", 10).unwrap());
        assert_eq!((two * three), six);
        // div: 6 / 2 == 3
        assert_eq!((six / two), three);
        // rem: 6 % 2 == 0  (storage-level remainder)
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
        let half = D::from_bits(D::multiplier() / crate::wide::I256::from_str_radix("2", 10).unwrap());
        let one_and_half = one + half;
        let product = one_and_half * one_and_half;
        let expected = D::from_bits(
            D::multiplier() * crate::wide::I256::from_str_radix("2", 10).unwrap()
                + D::multiplier() / crate::wide::I256::from_str_radix("4", 10).unwrap(),
        );
        assert_eq!(product, expected);
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_display() {
        type D = super::D256<12>;
        let one = D::ONE;
        assert_eq!(alloc::format!("{}", one), "1.000000000000");
        assert_eq!(alloc::format!("{}", -one), "-1.000000000000");
        assert_eq!(alloc::format!("{}", D::ZERO), "0.000000000000");
        let half = D::from_bits(D::multiplier() / crate::wide::I256::from_str_radix("2", 10).unwrap());
        assert_eq!(alloc::format!("{}", half), "0.500000000000");
        assert_eq!(alloc::format!("{:?}", one), "D256<12>(1.000000000000)");
        // scale 0 prints no fractional part
        let int_only: super::D256<0> = super::D256::<0>::ONE;
        assert_eq!(alloc::format!("{}", int_only), "1");
        // very large magnitude near the 76-digit ceiling
        let big = super::D256s76::MAX;
        let s = alloc::format!("{}", big);
        assert!(s.starts_with("5.78960446"));
        assert_eq!(s.len(), "5.".len() + 76);
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_sign_and_helpers() {
        type D = super::D256<6>;
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
        let half = D::from_bits(D::multiplier() / crate::wide::I256::from_str_radix("2", 10).unwrap());
        assert_eq!(two.recip(), half);
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_overflow_variants() {
        type D = super::D256<2>;
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
        assert_eq!(two.checked_mul(three), Some(D::from_bits(D::multiplier() * crate::wide::I256::from_str_radix("6", 10).unwrap())));
        assert_eq!(D::ONE.checked_div(D::ZERO), None);
        assert_eq!((three).checked_div(D::ONE), Some(three));
        // wrapping_add of one storage LSB at MAX wraps around to MIN.
        let one_lsb = D::from_bits(crate::wide::I256::from_str_radix("1", 10).unwrap());
        assert_eq!(D::MAX.wrapping_add(one_lsb), D::MIN);
        // overflowing
        assert_eq!(D::ONE.overflowing_add(D::ONE), (two, false));
        assert_eq!(D::MAX.overflowing_add(D::ONE).1, true);
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_consts_and_from_str() {
        use crate::consts::DecimalConsts;
        use core::str::FromStr;
        // pi at scale 12 matches the D128 reference.
        assert_eq!(
            super::D256::<12>::pi().to_bits(),
            crate::wide::I256::from_str_radix("3141592653590", 10).unwrap()
        );
        assert_eq!(
            super::D256::<4>::e().to_bits(),
            crate::wide::I256::from_str_radix("27183", 10).unwrap()
        );
        // FromStr within i128 range
        let v = super::D256::<2>::from_str("1.50").unwrap();
        assert_eq!(v.to_bits(), crate::wide::I256::from_str_radix("150", 10).unwrap());
        let neg = super::D256::<2>::from_str("-20.50").unwrap();
        assert_eq!(neg.to_bits(), crate::wide::I256::from_str_radix("-2050", 10).unwrap());
        // num_traits Zero / One
        use num_traits::{One, Zero};
        assert!(super::D256::<6>::zero().is_zero());
        assert!(super::D256::<6>::one().is_one());
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_conversions() {
        use crate::wide::I256;
        type D = super::D256<6>;
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
        assert_eq!(D::from_int(9i128), 9i32.into());
        assert_eq!(D::from_i32(-4), (-4i32).into());
        // to_int_lossy: 2.5 with HalfToEven -> 2
        use crate::rounding::RoundingMode;
        let two_and_half = D::from_bits(I256::from_str_radix("2500000", 10).unwrap());
        assert_eq!(two_and_half.to_int_lossy_with(RoundingMode::HalfToEven), 2);
        assert_eq!(two_and_half.to_int_lossy_with(RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(two_and_half.to_int_lossy_with(RoundingMode::Ceiling), 3);
        assert_eq!(two_and_half.to_int_lossy_with(RoundingMode::Floor), 2);
        let neg_two_and_half = -two_and_half;
        assert_eq!(neg_two_and_half.to_int_lossy_with(RoundingMode::Floor), -3);
        assert_eq!(neg_two_and_half.to_int_lossy_with(RoundingMode::Trunc), -2);
        // cross-width widening D128 -> D256 (lossless)
        let d128: super::D128s6 = super::D128s6::from_bits(-150);
        let widened: super::D256<6> = d128.into();
        assert_eq!(widened.to_bits(), I256::from_str_radix("-150", 10).unwrap());
        // cross-width narrowing D256 -> D128 in range
        let in_range: super::D256<6> = super::D256::<6>::from_bits(I256::from_str_radix("999", 10).unwrap());
        let narrowed: super::D128s6 = in_range.try_into().unwrap();
        assert_eq!(narrowed.to_bits(), 999i128);
        // cross-width narrowing D256 -> D128 out of range
        let out_of_range = super::D256s76::MAX;
        let narrow_fail: Result<super::D128<76>, _> = out_of_range.try_into();
        assert!(narrow_fail.is_err());
    }

    #[cfg(any(feature = "d256", feature = "wide"))]
    #[test]
    fn d256_rescale_rounding_floats() {
        use crate::rounding::RoundingMode;
        use crate::wide::I256;
        type D6 = super::D256<6>;
        // rescale up (lossless): scale 6 -> scale 9
        let v = D6::from_bits(I256::from_str_radix("1500000", 10).unwrap()); // 1.5
        let up: super::D256<9> = v.rescale::<9>();
        assert_eq!(up.to_bits(), I256::from_str_radix("1500000000", 10).unwrap());
        // rescale down (lossy, HalfToEven): scale 6 -> scale 2
        let down: super::D256<2> = v.rescale::<2>();
        assert_eq!(down.to_bits(), I256::from_str_radix("150", 10).unwrap());
        // rescale down with explicit mode: 2.5 (scale 0 representation) ...
        let two_p_five = super::D256::<1>::from_bits(I256::from_str_radix("25", 10).unwrap());
        let r0: super::D256<0> = two_p_five.rescale_with::<0>(RoundingMode::HalfToEven);
        assert_eq!(r0.to_bits(), I256::from_str_radix("2", 10).unwrap());
        let r0b: super::D256<0> = two_p_five.rescale_with::<0>(RoundingMode::HalfAwayFromZero);
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
        let from_f = D6::from_f64_lossy(2.5);
        assert_eq!(from_f.to_bits(), I256::from_str_radix("2500000", 10).unwrap());
        assert_eq!(D6::from_f64_lossy(f64::NAN), D6::ZERO);
        assert_eq!(D6::from_f64_lossy(f64::INFINITY), D6::MAX);
        let round_trip = D6::ONE.to_f64_lossy();
        assert!((round_trip - 1.0).abs() < 1e-9);
    }

    #[cfg(any(feature = "d512", feature = "wide"))]
    #[test]
    fn d512_smoke() {
        use crate::decimal_trait::Decimal;
        use crate::wide::I512;
        type D = super::D512<35>;
        assert_eq!(<D as Decimal>::MAX_SCALE, 153);
        assert_eq!(D::ZERO.to_bits(), I512::from_str_radix("0", 10).unwrap());
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::from_int(6i128));
        assert_eq!((three * two) / two, three);
        assert_eq!(alloc::format!("{}", one).len(), "1.".len() + 35);
        assert_eq!(D::from_int(5i128).to_int_lossy(), 5);
        // rescale across the wide range
        let up: super::D512<150> = one.rescale::<150>();
        assert_eq!(up, super::D512::<150>::ONE);
        // 153-digit ceiling multiplier fits in I512
        let _ = super::D512s153::multiplier();
    }

    #[cfg(any(feature = "d1024", feature = "wide"))]
    #[test]
    fn d1024_smoke() {
        use crate::decimal_trait::Decimal;
        use crate::wide::I1024;
        type D = super::D1024<35>;
        assert_eq!(<D as Decimal>::MAX_SCALE, 307);
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::from_int(6i128));
        assert_eq!((three * two) / two, three);
        assert_eq!(D::ZERO.to_bits(), I1024::from_str_radix("0", 10).unwrap());
        assert_eq!(alloc::format!("{}", one).len(), "1.".len() + 35);
        // cross-width: D256 -> D1024 widening, D1024 -> D256 narrowing
        #[cfg(any(feature = "d256", feature = "wide"))]
        {
            let small: super::D256<35> = super::D256::<35>::ONE;
            let widened: super::D1024<35> = small.into();
            assert_eq!(widened, D::ONE);
            let narrowed: super::D256<35> = widened.try_into().unwrap();
            assert_eq!(narrowed, super::D256::<35>::ONE);
        }
        // 307-digit ceiling multiplier fits in I1024
        let _ = super::D1024s307::multiplier();
    }

    #[test]
    fn d32_op_assign() {
        let mut v = super::D32s2::from_bits(100);
        v += super::D32s2::from_bits(50);
        assert_eq!(v.to_bits(), 150);
        v -= super::D32s2::from_bits(25);
        assert_eq!(v.to_bits(), 125);
        v *= super::D32s2::from_bits(200); // *2.00
        assert_eq!(v.to_bits(), 250);
        v /= super::D32s2::from_bits(200); // /2.00
        assert_eq!(v.to_bits(), 125);
        v %= super::D32s2::from_bits(100);
        assert_eq!(v.to_bits(), 25);
    }
}
