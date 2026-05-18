//! Generic wide-integer square-root kernel — `isqrt(raw · 10^SCALE)`
//! with a single round step.
//!
//! For a `Dxx<SCALE>` value with raw storage `r`, the logical value is
//! `r / 10^SCALE`, so the square-root raw storage is
//! `round(sqrt(r · 10^SCALE))`. The radicand is formed exactly in a
//! wider integer (so the multiply by `10^SCALE` cannot overflow), the
//! exact integer square root is taken with `isqrt`, and a single
//! round-to-nearest step lands the result on the type's last
//! representable place. The result is within 0.5 ULP under any of the
//! six [`crate::rounding::RoundingMode`] values.
//!
//! # Per-tier free functions
//!
//! The kernel is parameterised over `(Storage, SqrtWide)`. The wide
//! integer family doesn't share a common trait that exposes `isqrt` /
//! `pow` / `resize` / arithmetic uniformly, so the kernel is emitted
//! per tier via a small macro and made available as free functions:
//!
//! - [`sqrt_d56`] — `Storage = Int192`, `SqrtWide = Int384`.
//! - [`sqrt_d76`] — `Storage = Int256`, `SqrtWide = Int512`.
//! - [`sqrt_d114`] — `Storage = Int384`, `SqrtWide = Int768`.
//! - [`sqrt_d153`] — `Storage = Int512`, `SqrtWide = Int1024`.
//! - [`sqrt_d230`] — `Storage = Int768`, `SqrtWide = Int1536`.
//! - [`sqrt_d307`] — `Storage = Int1024`, `SqrtWide = Int2048`.
//! - [`sqrt_d461`] — `Storage = Int1536`, `SqrtWide = Int3072`.
//! - [`sqrt_d615`] — `Storage = Int2048`, `SqrtWide = Int4096`.
//! - [`sqrt_d923`] — `Storage = Int3072`, `SqrtWide = Int6144`.
//! - [`sqrt_d1231`] — `Storage = Int4096`, `SqrtWide = Int8192`.
//!
//! Each function returns `Storage::ZERO` for `raw <= 0` (the
//! saturate-not-panic policy matching the typed `sqrt_strict` surface).

use crate::rounding::RoundingMode;

/// Emits a `sqrt_<tier>(raw, scale, mode) -> Storage` free function.
///
/// The body is the same `isqrt` + round-mode-aware bump path the
/// original `decl_wide_roots!` macro inlined into every `Dxx<S>` —
/// extracted verbatim so behaviour is byte-identical.
macro_rules! decl_sqrt_kernel {
    ($name:ident, $Storage:ty, $SqrtWide:ty) => {
        /// Square-root kernel for one wide tier. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, scale: u32, mode: RoundingMode) -> $Storage {
            let zero_s = <$Storage>::from_str_radix("0", 10)
                .expect("sqrt kernel: invalid base-10 literal");
            if raw <= zero_s {
                return zero_s;
            }
            let zero = <$SqrtWide>::from_str_radix("0", 10)
                .expect("sqrt kernel: invalid base-10 literal");
            let one = <$SqrtWide>::from_str_radix("1", 10)
                .expect("sqrt kernel: invalid base-10 literal");
            let ten = <$SqrtWide>::from_str_radix("10", 10)
                .expect("sqrt kernel: invalid base-10 literal");
            let n: $SqrtWide = raw.resize::<$SqrtWide>() * ten.pow(scale);
            let q = n.isqrt();
            let diff = n - q * q;
            let halfway_round_up = diff > q;
            let diff_nonzero = diff != zero;
            let bump = match mode {
                RoundingMode::HalfToEven
                | RoundingMode::HalfAwayFromZero
                | RoundingMode::HalfTowardZero => halfway_round_up,
                RoundingMode::Trunc | RoundingMode::Floor => false,
                RoundingMode::Ceiling => diff_nonzero,
            };
            let q = if bump { q + one } else { q };
            q.resize::<$Storage>()
        }
    };
}

#[cfg(any(feature = "d56", feature = "wide"))]
decl_sqrt_kernel!(sqrt_d56, crate::wide_int::Int192, crate::wide_int::Int384);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_sqrt_kernel!(sqrt_d76, crate::wide_int::Int256, crate::wide_int::Int512);

#[cfg(any(feature = "d114", feature = "wide"))]
decl_sqrt_kernel!(sqrt_d114, crate::wide_int::Int384, crate::wide_int::Int768);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_sqrt_kernel!(sqrt_d153, crate::wide_int::Int512, crate::wide_int::Int1024);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_sqrt_kernel!(sqrt_d230, crate::wide_int::Int768, crate::wide_int::Int1536);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_sqrt_kernel!(sqrt_d307, crate::wide_int::Int1024, crate::wide_int::Int2048);

#[cfg(any(feature = "d461", feature = "x-wide"))]
decl_sqrt_kernel!(sqrt_d461, crate::wide_int::Int1536, crate::wide_int::Int3072);

#[cfg(any(feature = "d615", feature = "x-wide"))]
decl_sqrt_kernel!(sqrt_d615, crate::wide_int::Int2048, crate::wide_int::Int4096);

#[cfg(any(feature = "d923", feature = "xx-wide"))]
decl_sqrt_kernel!(sqrt_d923, crate::wide_int::Int3072, crate::wide_int::Int6144);

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
decl_sqrt_kernel!(sqrt_d1231, crate::wide_int::Int4096, crate::wide_int::Int8192);
