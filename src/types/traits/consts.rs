//! The [`DecimalConstants`] trait — mathematical constants (`pi`, `tau`,
//! `half_pi`, `quarter_pi`, `golden`, `e`, `deg_per_rad`, `rad_per_deg`,
//! `log10_2`) available on every decimal width.
//!
//! Split out of `types/consts/d38.rs` to sit with its sibling traits
//! ([`crate::DecimalArithmetic`], [`crate::DecimalConvert`],
//! [`crate::DecimalTranscendental`]); `Decimal` requires all four.
//! Callers who only need constants (not arithmetic or transcendentals)
//! can target this narrower bound:
//!
//! ```ignore
//! use decimal_scaled::DecimalConstants;
//!
//! fn circle_area<T: DecimalConstants + Copy + std::ops::Mul<Output = T>>(r: T) -> T {
//!     T::pi() * r * r
//! }
//! ```
//!
//! See [`crate::types::traits::decimal`] for the full scope rationale.

/// Well-known mathematical constants available on every decimal width
/// (`D18` / `D38` / `D76` / `D153` / `D307`).
///
/// Import this trait to call `D38s12::pi()`, `D76::<35>::e()`, etc.
///
/// All returned values are computed from a raw integer reference at
/// the tier's maximum storage precision (75 digits for D18/D38 and
/// D76; 153 for D153; 307 for D307) without passing through `f64`,
/// then rescaled down to the caller's `SCALE` with half-to-even
/// rounding. The result is **within 0.5 ULP** of the canonical
/// decimal expansion at every supported scale on every width.
///
/// The one situation where a method does not return a value is when
/// the constant's magnitude exceeds the type's storage range at the
/// caller's `SCALE` — e.g. `D38<38>::pi()` would need `3.14 × 10³⁸`,
/// which exceeds `i128::MAX ≈ 1.7×10³⁸`. The method panics with a
/// clear "constant out of storage range" message in that case.
///
/// # Crossing into f64
///
/// `to_f64()` is itself correctly rounded, but it can only round to
/// the *decimal value the type holds* — not to the underlying ideal
/// constant. `f64` carries ~15.95 decimal digits of mantissa, so any
/// constant produced at `SCALE < 15` is intrinsically coarser than
/// the `f64` grid: `D38<12>::pi().to_f64()` lands ~466 ULPs from
/// [`std::f64::consts::PI`], because the 12-digit decimal rounds
/// differently than the closest-`f64` to true π. At `SCALE ≥ 15` the
/// round-trip is bit-exact for these constants (the decimal value
/// has enough digits to disambiguate the `f64` grid).
///
/// **Practical rule for downstream code that crosses into `f64`** —
/// CAD bulge-arc tessellation, OpenGL/GLSL, hardware drivers — and
/// uses the `f64` value to count, bucket, or seed a fixed-iteration
/// loop: source mathematical constants from [`std::f64::consts`]
/// directly at the boundary rather than going through
/// `Decimal::pi().to_f64()`. Otherwise pick a `SCALE` of 15 or more
/// so the decimal value can round-trip to the canonical `f64`.
pub trait DecimalConstants: Sized {
    /// Pi (~3.14159265...). One half-turn in radians.
    ///
    /// Source: ISO 80000-2 / OEIS A000796. Rescaled per-tier (see the
    /// module-level table) to the caller's `SCALE` via the crate-default
    /// rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn pi() -> Self;

    /// Tau (~6.28318530...). One full turn in radians.
    ///
    /// Defined as `2 * pi`. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn tau() -> Self;

    /// Half-pi (~1.57079632...). One quarter-turn in radians.
    ///
    /// Defined as `pi / 2`. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn half_pi() -> Self;

    /// Quarter-pi (~0.78539816...). One eighth-turn in radians.
    ///
    /// Defined as `pi / 4`. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn quarter_pi() -> Self;

    /// The golden ratio (~1.61803398...). Dimensionless.
    ///
    /// Defined as `(1 + sqrt(5)) / 2`. Source: OEIS A001622. Rescaled
    /// per-tier (see the module-level table) to the caller's `SCALE`
    /// via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn golden() -> Self;

    /// Euler's number (~2.71828182...). Dimensionless.
    ///
    /// Source: OEIS A001113. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn e() -> Self;

    /// Degrees per radian (~57.29577951...). The factor `180/π`:
    /// multiply a radian measure by this to convert it to degrees.
    ///
    /// Source: the oracle value of `180/π`. Sourced per-scale from the
    /// constant table to the caller's `SCALE` via the crate-default
    /// rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn deg_per_rad() -> Self;

    /// Radians per degree (~0.01745329...). The factor `π/180`:
    /// multiply a degree measure by this to convert it to radians.
    ///
    /// Source: the oracle value of `π/180`. Sourced per-scale from the
    /// constant table to the caller's `SCALE` via the crate-default
    /// rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn rad_per_deg() -> Self;

    /// Base-10 logarithm of 2 (~0.30103...). The bit-to-digit factor:
    /// a value's decimal-digit count is about `bit_length * log10(2)`,
    /// so this scales between binary and decimal magnitudes.
    ///
    /// Source: the oracle value of `log(2)/log(10)`. Sourced per-scale
    /// from the constant table to the caller's `SCALE` via the
    /// crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn log10_2() -> Self;

    // ─── *_with(mode) siblings ───────────────────────────────────
    //
    // Each `<const>_with(mode)` rescales the 75-digit reference under
    // the caller-supplied `RoundingMode`. Useful when the default
    // mode (half-to-even, or whatever a `rounding-*` Cargo feature
    // selects) is the wrong direction for the use case — e.g. a CAD
    // tessellation that needs `pi_with(Floor)` so the down-stream
    // f64 conversion stays ≤ true π and segment counts can't
    // over-flow their fixed-size buffers.

    /// `pi()` under the supplied rounding mode.
    fn pi_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `tau()` under the supplied rounding mode.
    fn tau_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `half_pi()` under the supplied rounding mode.
    fn half_pi_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `quarter_pi()` under the supplied rounding mode.
    fn quarter_pi_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `golden()` under the supplied rounding mode.
    fn golden_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `e()` under the supplied rounding mode.
    fn e_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `deg_per_rad()` under the supplied rounding mode.
    fn deg_per_rad_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `rad_per_deg()` under the supplied rounding mode.
    fn rad_per_deg_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `log10_2()` under the supplied rounding mode.
    fn log10_2_with(mode: crate::support::rounding::RoundingMode) -> Self;
}


