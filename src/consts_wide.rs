//! Per-width raw constants for the wide-tier decimal types.
//!
//! The raw constants are computed at build time by `build.rs` using a
//! hand-rolled multi-precision arithmetic core (no external deps).
//! Each width gets its own `SCALE_REF` matched to its storage's max
//! precision: D76 at 75 frac digits, D153 at 153, D307 at 307. The
//! corresponding raw values land in `OUT_DIR/wide_consts.rs` as
//! decimal-digit strings, parsed at compile time by
//! `Int*::from_str_radix` (a `const fn`).
//!
//! This closes the SCALE > 37 panic in `D76<76>::pi()` etc. and
//! tightens the 0.5 ULP contract on `DecimalConsts` for every
//! wide-tier scale.

use crate::wide_int::{Int256, Int512, Int1024};
use crate::consts::DecimalConsts;

include!(concat!(env!("OUT_DIR"), "/wide_consts.rs"));

// ─── D76 ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "d76", feature = "wide"))]
const D76_SCALE_REF: u32 = 75;

#[cfg(any(feature = "d76", feature = "wide"))]
const PI_RAW_D256: Int256 = match Int256::from_str_radix(PI_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: PI_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const TAU_RAW_D256: Int256 = match Int256::from_str_radix(TAU_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: TAU_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const HALF_PI_RAW_D256: Int256 = match Int256::from_str_radix(HALF_PI_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: HALF_PI_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const QUARTER_PI_RAW_D256: Int256 = match Int256::from_str_radix(QUARTER_PI_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: QUARTER_PI_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const E_RAW_D256: Int256 = match Int256::from_str_radix(E_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: E_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const GOLDEN_RAW_D256: Int256 = match Int256::from_str_radix(GOLDEN_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: GOLDEN_D76_S75 not parseable"),
};

#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn pi_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::core_type::D76;
    D76::<D76_SCALE_REF>::from_bits(PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn tau_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::core_type::D76;
    D76::<D76_SCALE_REF>::from_bits(TAU_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn half_pi_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::core_type::D76;
    D76::<D76_SCALE_REF>::from_bits(HALF_PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::core_type::D76;
    D76::<D76_SCALE_REF>::from_bits(QUARTER_PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn e_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::core_type::D76;
    D76::<D76_SCALE_REF>::from_bits(E_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn golden_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::core_type::D76;
    D76::<D76_SCALE_REF>::from_bits(GOLDEN_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── D153 ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "d153", feature = "wide"))]
const D153_SCALE_REF: u32 = 153;

#[cfg(any(feature = "d153", feature = "wide"))]
const PI_RAW_D512: Int512 = match Int512::from_str_radix(PI_D153_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: PI_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const TAU_RAW_D512: Int512 = match Int512::from_str_radix(TAU_D153_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: TAU_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const HALF_PI_RAW_D512: Int512 = match Int512::from_str_radix(HALF_PI_D153_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: HALF_PI_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const QUARTER_PI_RAW_D512: Int512 = match Int512::from_str_radix(QUARTER_PI_D153_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: QUARTER_PI_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const E_RAW_D512: Int512 = match Int512::from_str_radix(E_D153_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: E_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const GOLDEN_RAW_D512: Int512 = match Int512::from_str_radix(GOLDEN_D153_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: GOLDEN_D153_S153 not parseable"),
};

#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn pi_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::core_type::D153;
    D153::<D153_SCALE_REF>::from_bits(PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn tau_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::core_type::D153;
    D153::<D153_SCALE_REF>::from_bits(TAU_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn half_pi_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::core_type::D153;
    D153::<D153_SCALE_REF>::from_bits(HALF_PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::core_type::D153;
    D153::<D153_SCALE_REF>::from_bits(QUARTER_PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn e_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::core_type::D153;
    D153::<D153_SCALE_REF>::from_bits(E_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn golden_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::core_type::D153;
    D153::<D153_SCALE_REF>::from_bits(GOLDEN_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── D307 ────────────────────────────────────────────────────────────

#[cfg(any(feature = "d307", feature = "wide"))]
const D307_SCALE_REF: u32 = 307;

#[cfg(any(feature = "d307", feature = "wide"))]
const PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(PI_D307_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: PI_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const TAU_RAW_D1024: Int1024 = match Int1024::from_str_radix(TAU_D307_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: TAU_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const HALF_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(HALF_PI_D307_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: HALF_PI_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const QUARTER_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(QUARTER_PI_D307_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: QUARTER_PI_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const E_RAW_D1024: Int1024 = match Int1024::from_str_radix(E_D307_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: E_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const GOLDEN_RAW_D1024: Int1024 = match Int1024::from_str_radix(GOLDEN_D307_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: GOLDEN_D307_S307 not parseable"),
};

#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn pi_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D307;
    D307::<D307_SCALE_REF>::from_bits(PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn tau_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D307;
    D307::<D307_SCALE_REF>::from_bits(TAU_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn half_pi_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D307;
    D307::<D307_SCALE_REF>::from_bits(HALF_PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D307;
    D307::<D307_SCALE_REF>::from_bits(QUARTER_PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn e_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D307;
    D307::<D307_SCALE_REF>::from_bits(E_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn golden_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D307;
    D307::<D307_SCALE_REF>::from_bits(GOLDEN_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── DecimalConsts impls ──────────────────────────────────────────────
//
// These shadow the impls that `decl_decimal_consts!(wide …)` would
// emit. To avoid duplicate trait impls, the wide-arm macro invocations
// in `core_type.rs` were removed (search for `decl_decimal_consts!(wide`).

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> DecimalConsts for crate::core_type::D76<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d76::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d76::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d76::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d76::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d76::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d76::<SCALE>()) }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> DecimalConsts for crate::core_type::D153<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d153::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d153::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d153::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d153::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d153::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d153::<SCALE>()) }
}

#[cfg(any(feature = "d307", feature = "wide"))]
impl<const SCALE: u32> DecimalConsts for crate::core_type::D307<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d307::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d307::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d307::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d307::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d307::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d307::<SCALE>()) }
}
