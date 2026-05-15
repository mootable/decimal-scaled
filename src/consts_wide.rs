//! Per-width raw constants for the wide-tier decimal types.
//!
//! The raw constants are computed at build time by `build.rs` using a
//! hand-rolled multi-precision arithmetic core (no external deps).
//! Each width gets its own `SCALE_REF` matched to its storage's max
//! precision: D256 at 75 frac digits, D512 at 153, D1024 at 307. The
//! corresponding raw values land in `OUT_DIR/wide_consts.rs` as
//! decimal-digit strings, parsed at compile time by
//! `Int*::from_str_radix` (a `const fn`).
//!
//! This closes the SCALE > 37 panic in `D256<76>::pi()` etc. and
//! tightens the 0.5 ULP contract on `DecimalConsts` for every
//! wide-tier scale.

use crate::wide_int::{Int256, Int512, Int1024};
use crate::consts::DecimalConsts;

include!(concat!(env!("OUT_DIR"), "/wide_consts.rs"));

// ─── D256 ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "d256", feature = "wide"))]
const D256_SCALE_REF: u32 = 75;

#[cfg(any(feature = "d256", feature = "wide"))]
const PI_RAW_D256: Int256 = match Int256::from_str_radix(PI_D256_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: PI_D256_S75 not parseable"),
};
#[cfg(any(feature = "d256", feature = "wide"))]
const TAU_RAW_D256: Int256 = match Int256::from_str_radix(TAU_D256_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: TAU_D256_S75 not parseable"),
};
#[cfg(any(feature = "d256", feature = "wide"))]
const HALF_PI_RAW_D256: Int256 = match Int256::from_str_radix(HALF_PI_D256_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: HALF_PI_D256_S75 not parseable"),
};
#[cfg(any(feature = "d256", feature = "wide"))]
const QUARTER_PI_RAW_D256: Int256 = match Int256::from_str_radix(QUARTER_PI_D256_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: QUARTER_PI_D256_S75 not parseable"),
};
#[cfg(any(feature = "d256", feature = "wide"))]
const E_RAW_D256: Int256 = match Int256::from_str_radix(E_D256_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: E_D256_S75 not parseable"),
};
#[cfg(any(feature = "d256", feature = "wide"))]
const GOLDEN_RAW_D256: Int256 = match Int256::from_str_radix(GOLDEN_D256_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: GOLDEN_D256_S75 not parseable"),
};

#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) fn pi_at_target_d256<const TARGET: u32>() -> Int256 {
    use crate::core_type::D256;
    D256::<D256_SCALE_REF>::from_bits(PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) fn tau_at_target_d256<const TARGET: u32>() -> Int256 {
    use crate::core_type::D256;
    D256::<D256_SCALE_REF>::from_bits(TAU_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) fn half_pi_at_target_d256<const TARGET: u32>() -> Int256 {
    use crate::core_type::D256;
    D256::<D256_SCALE_REF>::from_bits(HALF_PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d256<const TARGET: u32>() -> Int256 {
    use crate::core_type::D256;
    D256::<D256_SCALE_REF>::from_bits(QUARTER_PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) fn e_at_target_d256<const TARGET: u32>() -> Int256 {
    use crate::core_type::D256;
    D256::<D256_SCALE_REF>::from_bits(E_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) fn golden_at_target_d256<const TARGET: u32>() -> Int256 {
    use crate::core_type::D256;
    D256::<D256_SCALE_REF>::from_bits(GOLDEN_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── D512 ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "d512", feature = "wide"))]
const D512_SCALE_REF: u32 = 153;

#[cfg(any(feature = "d512", feature = "wide"))]
const PI_RAW_D512: Int512 = match Int512::from_str_radix(PI_D512_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: PI_D512_S153 not parseable"),
};
#[cfg(any(feature = "d512", feature = "wide"))]
const TAU_RAW_D512: Int512 = match Int512::from_str_radix(TAU_D512_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: TAU_D512_S153 not parseable"),
};
#[cfg(any(feature = "d512", feature = "wide"))]
const HALF_PI_RAW_D512: Int512 = match Int512::from_str_radix(HALF_PI_D512_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: HALF_PI_D512_S153 not parseable"),
};
#[cfg(any(feature = "d512", feature = "wide"))]
const QUARTER_PI_RAW_D512: Int512 = match Int512::from_str_radix(QUARTER_PI_D512_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: QUARTER_PI_D512_S153 not parseable"),
};
#[cfg(any(feature = "d512", feature = "wide"))]
const E_RAW_D512: Int512 = match Int512::from_str_radix(E_D512_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: E_D512_S153 not parseable"),
};
#[cfg(any(feature = "d512", feature = "wide"))]
const GOLDEN_RAW_D512: Int512 = match Int512::from_str_radix(GOLDEN_D512_S153, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: GOLDEN_D512_S153 not parseable"),
};

#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) fn pi_at_target_d512<const TARGET: u32>() -> Int512 {
    use crate::core_type::D512;
    D512::<D512_SCALE_REF>::from_bits(PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) fn tau_at_target_d512<const TARGET: u32>() -> Int512 {
    use crate::core_type::D512;
    D512::<D512_SCALE_REF>::from_bits(TAU_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) fn half_pi_at_target_d512<const TARGET: u32>() -> Int512 {
    use crate::core_type::D512;
    D512::<D512_SCALE_REF>::from_bits(HALF_PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d512<const TARGET: u32>() -> Int512 {
    use crate::core_type::D512;
    D512::<D512_SCALE_REF>::from_bits(QUARTER_PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) fn e_at_target_d512<const TARGET: u32>() -> Int512 {
    use crate::core_type::D512;
    D512::<D512_SCALE_REF>::from_bits(E_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) fn golden_at_target_d512<const TARGET: u32>() -> Int512 {
    use crate::core_type::D512;
    D512::<D512_SCALE_REF>::from_bits(GOLDEN_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── D1024 ────────────────────────────────────────────────────────────

#[cfg(any(feature = "d1024", feature = "wide"))]
const D1024_SCALE_REF: u32 = 307;

#[cfg(any(feature = "d1024", feature = "wide"))]
const PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(PI_D1024_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: PI_D1024_S307 not parseable"),
};
#[cfg(any(feature = "d1024", feature = "wide"))]
const TAU_RAW_D1024: Int1024 = match Int1024::from_str_radix(TAU_D1024_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: TAU_D1024_S307 not parseable"),
};
#[cfg(any(feature = "d1024", feature = "wide"))]
const HALF_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(HALF_PI_D1024_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: HALF_PI_D1024_S307 not parseable"),
};
#[cfg(any(feature = "d1024", feature = "wide"))]
const QUARTER_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(QUARTER_PI_D1024_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: QUARTER_PI_D1024_S307 not parseable"),
};
#[cfg(any(feature = "d1024", feature = "wide"))]
const E_RAW_D1024: Int1024 = match Int1024::from_str_radix(E_D1024_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: E_D1024_S307 not parseable"),
};
#[cfg(any(feature = "d1024", feature = "wide"))]
const GOLDEN_RAW_D1024: Int1024 = match Int1024::from_str_radix(GOLDEN_D1024_S307, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts_wide: GOLDEN_D1024_S307 not parseable"),
};

#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) fn pi_at_target_d1024<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D1024;
    D1024::<D1024_SCALE_REF>::from_bits(PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) fn tau_at_target_d1024<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D1024;
    D1024::<D1024_SCALE_REF>::from_bits(TAU_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) fn half_pi_at_target_d1024<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D1024;
    D1024::<D1024_SCALE_REF>::from_bits(HALF_PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d1024<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D1024;
    D1024::<D1024_SCALE_REF>::from_bits(QUARTER_PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) fn e_at_target_d1024<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D1024;
    D1024::<D1024_SCALE_REF>::from_bits(E_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) fn golden_at_target_d1024<const TARGET: u32>() -> Int1024 {
    use crate::core_type::D1024;
    D1024::<D1024_SCALE_REF>::from_bits(GOLDEN_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── DecimalConsts impls ──────────────────────────────────────────────
//
// These shadow the impls that `decl_decimal_consts!(wide …)` would
// emit. To avoid duplicate trait impls, the wide-arm macro invocations
// in `core_type.rs` were removed (search for `decl_decimal_consts!(wide`).

#[cfg(any(feature = "d256", feature = "wide"))]
impl<const SCALE: u32> DecimalConsts for crate::core_type::D256<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d256::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d256::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d256::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d256::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d256::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d256::<SCALE>()) }
}

#[cfg(any(feature = "d512", feature = "wide"))]
impl<const SCALE: u32> DecimalConsts for crate::core_type::D512<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d512::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d512::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d512::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d512::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d512::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d512::<SCALE>()) }
}

#[cfg(any(feature = "d1024", feature = "wide"))]
impl<const SCALE: u32> DecimalConsts for crate::core_type::D1024<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d1024::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d1024::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d1024::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d1024::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d1024::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d1024::<SCALE>()) }
}
