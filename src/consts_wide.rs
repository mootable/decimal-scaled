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

// These imports are only reachable when at least one wide-tier
// feature is enabled — every item in this module is per-tier
// `#[cfg]`-gated below. Narrow-only builds compile the module
// empty.
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide", feature = "x-wide"))]
use crate::wide_int::{Int256, Int512, Int1024};
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide", feature = "x-wide"))]
use crate::consts::DecimalConsts;

include!(concat!(env!("OUT_DIR"), "/wide_consts.rs"));

// ─── D76 ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "d76", feature = "wide"))]
const D76_SCALE_REF: u32 = 75;

#[cfg(any(feature = "d76", feature = "wide"))]
const PI_RAW_D256: Int256 = match Int256::from_str_radix(PI_D76_S75, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: PI_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const TAU_RAW_D256: Int256 = match Int256::from_str_radix(TAU_D76_S75, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: TAU_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const HALF_PI_RAW_D256: Int256 = match Int256::from_str_radix(HALF_PI_D76_S75, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: HALF_PI_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const QUARTER_PI_RAW_D256: Int256 = match Int256::from_str_radix(QUARTER_PI_D76_S75, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: QUARTER_PI_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const E_RAW_D256: Int256 = match Int256::from_str_radix(E_D76_S75, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: E_D76_S75 not parseable"),
};
#[cfg(any(feature = "d76", feature = "wide"))]
const GOLDEN_RAW_D256: Int256 = match Int256::from_str_radix(GOLDEN_D76_S75, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: GOLDEN_D76_S75 not parseable"),
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
    Err(()) => panic!("consts_wide: PI_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const TAU_RAW_D512: Int512 = match Int512::from_str_radix(TAU_D153_S153, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: TAU_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const HALF_PI_RAW_D512: Int512 = match Int512::from_str_radix(HALF_PI_D153_S153, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: HALF_PI_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const QUARTER_PI_RAW_D512: Int512 = match Int512::from_str_radix(QUARTER_PI_D153_S153, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: QUARTER_PI_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const E_RAW_D512: Int512 = match Int512::from_str_radix(E_D153_S153, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: E_D153_S153 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const GOLDEN_RAW_D512: Int512 = match Int512::from_str_radix(GOLDEN_D153_S153, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: GOLDEN_D153_S153 not parseable"),
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
    Err(()) => panic!("consts_wide: PI_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const TAU_RAW_D1024: Int1024 = match Int1024::from_str_radix(TAU_D307_S307, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: TAU_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const HALF_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(HALF_PI_D307_S307, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: HALF_PI_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const QUARTER_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(QUARTER_PI_D307_S307, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: QUARTER_PI_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const E_RAW_D1024: Int1024 = match Int1024::from_str_radix(E_D307_S307, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: E_D307_S307 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const GOLDEN_RAW_D1024: Int1024 = match Int1024::from_str_radix(GOLDEN_D307_S307, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: GOLDEN_D307_S307 not parseable"),
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

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> DecimalConsts for crate::core_type::D307<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d307::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d307::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d307::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d307::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d307::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d307::<SCALE>()) }
}

// ─── New half-width and wider tiers ──────────────────────────────────
//
// Generated per the build.rs `for &scale in &[57, 115, 230, 462, 616,
// 924, 1232]` loop. Each tier mirrors the D76 / D153 / D307 pattern:
// (1) a raw `Int*` const parsed from the build-time decimal string,
// (2) a `<const_name>_at_target_d<scale>::<TARGET>()` accessor that
// rescales down to the caller's SCALE, and (3) a `DecimalConsts` impl
// on the decimal type.
//
// Macro to compress the repetition: each invocation produces one
// tier's full set of consts + accessor + impl.
macro_rules! decl_wide_consts_tier {
    (
        $D:ident, $Storage:ty, $scale:literal, $scale_ref:ident,
        $PI:ident, $TAU:ident, $HALF_PI:ident, $QUARTER_PI:ident,
        $E:ident, $GOLDEN:ident,
        $PI_RAW:ident, $TAU_RAW:ident, $HALF_PI_RAW:ident,
        $QUARTER_PI_RAW:ident, $E_RAW:ident, $GOLDEN_RAW:ident,
        $pi_fn:ident, $tau_fn:ident, $half_pi_fn:ident,
        $quarter_pi_fn:ident, $e_fn:ident, $golden_fn:ident,
        $feature:literal, $umbrella:literal $(,)?
    ) => {
        #[cfg(any(feature = $feature, feature = $umbrella))]
        const $scale_ref: u32 = $scale;

        #[cfg(any(feature = $feature, feature = $umbrella))]
        const $PI_RAW: $Storage = match <$Storage>::from_str_radix($PI, 10) {
            Ok(v) => v,
            Err(()) => panic!(concat!("consts_wide: ", stringify!($PI), " not parseable")),
        };
        #[cfg(any(feature = $feature, feature = $umbrella))]
        const $TAU_RAW: $Storage = match <$Storage>::from_str_radix($TAU, 10) {
            Ok(v) => v,
            Err(()) => panic!(concat!("consts_wide: ", stringify!($TAU), " not parseable")),
        };
        #[cfg(any(feature = $feature, feature = $umbrella))]
        const $HALF_PI_RAW: $Storage = match <$Storage>::from_str_radix($HALF_PI, 10) {
            Ok(v) => v,
            Err(()) => panic!(concat!("consts_wide: ", stringify!($HALF_PI), " not parseable")),
        };
        #[cfg(any(feature = $feature, feature = $umbrella))]
        const $QUARTER_PI_RAW: $Storage = match <$Storage>::from_str_radix($QUARTER_PI, 10) {
            Ok(v) => v,
            Err(()) => panic!(concat!("consts_wide: ", stringify!($QUARTER_PI), " not parseable")),
        };
        #[cfg(any(feature = $feature, feature = $umbrella))]
        const $E_RAW: $Storage = match <$Storage>::from_str_radix($E, 10) {
            Ok(v) => v,
            Err(()) => panic!(concat!("consts_wide: ", stringify!($E), " not parseable")),
        };
        #[cfg(any(feature = $feature, feature = $umbrella))]
        const $GOLDEN_RAW: $Storage = match <$Storage>::from_str_radix($GOLDEN, 10) {
            Ok(v) => v,
            Err(()) => panic!(concat!("consts_wide: ", stringify!($GOLDEN), " not parseable")),
        };

        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $pi_fn<const TARGET: u32>() -> $Storage {
            use crate::core_type::$D;
            $D::<{ $scale }>::from_bits($PI_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $tau_fn<const TARGET: u32>() -> $Storage {
            use crate::core_type::$D;
            $D::<{ $scale }>::from_bits($TAU_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $half_pi_fn<const TARGET: u32>() -> $Storage {
            use crate::core_type::$D;
            $D::<{ $scale }>::from_bits($HALF_PI_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $quarter_pi_fn<const TARGET: u32>() -> $Storage {
            use crate::core_type::$D;
            $D::<{ $scale }>::from_bits($QUARTER_PI_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $e_fn<const TARGET: u32>() -> $Storage {
            use crate::core_type::$D;
            $D::<{ $scale }>::from_bits($E_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $golden_fn<const TARGET: u32>() -> $Storage {
            use crate::core_type::$D;
            $D::<{ $scale }>::from_bits($GOLDEN_RAW).rescale::<TARGET>().to_bits()
        }

        #[cfg(any(feature = $feature, feature = $umbrella))]
        impl<const SCALE: u32> DecimalConsts for crate::core_type::$D<SCALE> {
            #[inline] fn pi() -> Self { Self($pi_fn::<SCALE>()) }
            #[inline] fn tau() -> Self { Self($tau_fn::<SCALE>()) }
            #[inline] fn half_pi() -> Self { Self($half_pi_fn::<SCALE>()) }
            #[inline] fn quarter_pi() -> Self { Self($quarter_pi_fn::<SCALE>()) }
            #[inline] fn golden() -> Self { Self($golden_fn::<SCALE>()) }
            #[inline] fn e() -> Self { Self($e_fn::<SCALE>()) }
        }
    };
}

#[cfg(any(feature = "d56", feature = "wide"))]
use crate::wide_int::Int192;
#[cfg(any(feature = "d114", feature = "wide"))]
use crate::wide_int::Int384;
#[cfg(any(feature = "d230", feature = "wide"))]
use crate::wide_int::Int768;
#[cfg(any(feature = "d461", feature = "x-wide"))]
use crate::wide_int::Int1536;
#[cfg(any(feature = "d615", feature = "x-wide"))]
use crate::wide_int::Int2048;
#[cfg(any(feature = "d923", feature = "xx-wide"))]
use crate::wide_int::Int3072;
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
use crate::wide_int::Int4096;

// SCALE_REF per tier = highest k where τ × 10^k still fits the
// storage's signed range. Computed in build.rs and matched here.
// D56/D114/D461/D615/D923/D1231 cannot use their nominal tier max
// because τ ≈ 6.28 pushes just past the i_max boundary; D230 and
// D1231 borderline cases work at 230 / 1231 respectively.
decl_wide_consts_tier!(
    D56, Int192, 56, D56_SCALE_REF,
    PI_D56_S56, TAU_D56_S56, HALF_PI_D56_S56, QUARTER_PI_D56_S56, E_D56_S56, GOLDEN_D56_S56,
    PI_RAW_D192, TAU_RAW_D192, HALF_PI_RAW_D192, QUARTER_PI_RAW_D192, E_RAW_D192, GOLDEN_RAW_D192,
    pi_at_target_d56, tau_at_target_d56, half_pi_at_target_d56,
    quarter_pi_at_target_d56, e_at_target_d56, golden_at_target_d56,
    "d56", "wide",
);

decl_wide_consts_tier!(
    D114, Int384, 114, D114_SCALE_REF,
    PI_D114_S114, TAU_D114_S114, HALF_PI_D114_S114, QUARTER_PI_D114_S114, E_D114_S114, GOLDEN_D114_S114,
    PI_RAW_D384, TAU_RAW_D384, HALF_PI_RAW_D384, QUARTER_PI_RAW_D384, E_RAW_D384, GOLDEN_RAW_D384,
    pi_at_target_d114, tau_at_target_d114, half_pi_at_target_d114,
    quarter_pi_at_target_d114, e_at_target_d114, golden_at_target_d114,
    "d114", "wide",
);

decl_wide_consts_tier!(
    D230, Int768, 230, D230_SCALE_REF,
    PI_D230_S230, TAU_D230_S230, HALF_PI_D230_S230, QUARTER_PI_D230_S230, E_D230_S230, GOLDEN_D230_S230,
    PI_RAW_D768, TAU_RAW_D768, HALF_PI_RAW_D768, QUARTER_PI_RAW_D768, E_RAW_D768, GOLDEN_RAW_D768,
    pi_at_target_d230, tau_at_target_d230, half_pi_at_target_d230,
    quarter_pi_at_target_d230, e_at_target_d230, golden_at_target_d230,
    "d230", "wide",
);

decl_wide_consts_tier!(
    D461, Int1536, 461, D461_SCALE_REF,
    PI_D461_S461, TAU_D461_S461, HALF_PI_D461_S461, QUARTER_PI_D461_S461, E_D461_S461, GOLDEN_D461_S461,
    PI_RAW_D1536, TAU_RAW_D1536, HALF_PI_RAW_D1536, QUARTER_PI_RAW_D1536, E_RAW_D1536, GOLDEN_RAW_D1536,
    pi_at_target_d461, tau_at_target_d461, half_pi_at_target_d461,
    quarter_pi_at_target_d461, e_at_target_d461, golden_at_target_d461,
    "d461", "x-wide",
);

decl_wide_consts_tier!(
    D615, Int2048, 615, D615_SCALE_REF,
    PI_D615_S615, TAU_D615_S615, HALF_PI_D615_S615, QUARTER_PI_D615_S615, E_D615_S615, GOLDEN_D615_S615,
    PI_RAW_D2048, TAU_RAW_D2048, HALF_PI_RAW_D2048, QUARTER_PI_RAW_D2048, E_RAW_D2048, GOLDEN_RAW_D2048,
    pi_at_target_d615, tau_at_target_d615, half_pi_at_target_d615,
    quarter_pi_at_target_d615, e_at_target_d615, golden_at_target_d615,
    "d615", "x-wide",
);

decl_wide_consts_tier!(
    D923, Int3072, 923, D923_SCALE_REF,
    PI_D923_S923, TAU_D923_S923, HALF_PI_D923_S923, QUARTER_PI_D923_S923, E_D923_S923, GOLDEN_D923_S923,
    PI_RAW_D3072, TAU_RAW_D3072, HALF_PI_RAW_D3072, QUARTER_PI_RAW_D3072, E_RAW_D3072, GOLDEN_RAW_D3072,
    pi_at_target_d923, tau_at_target_d923, half_pi_at_target_d923,
    quarter_pi_at_target_d923, e_at_target_d923, golden_at_target_d923,
    "d923", "xx-wide",
);

decl_wide_consts_tier!(
    D1231, Int4096, 1231, D1231_SCALE_REF,
    PI_D1231_S1231, TAU_D1231_S1231, HALF_PI_D1231_S1231, QUARTER_PI_D1231_S1231, E_D1231_S1231, GOLDEN_D1231_S1231,
    PI_RAW_D4096, TAU_RAW_D4096, HALF_PI_RAW_D4096, QUARTER_PI_RAW_D4096, E_RAW_D4096, GOLDEN_RAW_D4096,
    pi_at_target_d1231, tau_at_target_d1231, half_pi_at_target_d1231,
    quarter_pi_at_target_d1231, e_at_target_d1231, golden_at_target_d1231,
    "d1231", "xx-wide",
);
