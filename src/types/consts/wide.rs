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
//! tightens the 0.5 ULP contract on `DecimalConstants` for every
//! wide-tier scale.

// These imports are only reachable when at least one wide-tier
// feature is enabled — every item in this module is per-tier
// `#[cfg]`-gated below. Narrow-only builds compile the module
// empty.
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide", feature = "x-wide"))]
use crate::wide_int::{Int256, Int512, Int1024};
#[cfg(any(feature = "d76", feature = "d153", feature = "d307", feature = "wide", feature = "x-wide"))]
use crate::types::consts::DecimalConstants;

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
    use crate::types::widths::D76;
    D76::<D76_SCALE_REF>::from_bits(PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn tau_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::types::widths::D76;
    D76::<D76_SCALE_REF>::from_bits(TAU_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn half_pi_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::types::widths::D76;
    D76::<D76_SCALE_REF>::from_bits(HALF_PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::types::widths::D76;
    D76::<D76_SCALE_REF>::from_bits(QUARTER_PI_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn e_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::types::widths::D76;
    D76::<D76_SCALE_REF>::from_bits(E_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d76", feature = "wide"))]
pub(crate) fn golden_at_target_d76<const TARGET: u32>() -> Int256 {
    use crate::types::widths::D76;
    D76::<D76_SCALE_REF>::from_bits(GOLDEN_RAW_D256)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── D153 ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "d153", feature = "wide"))]
const D153_SCALE_REF: u32 = 152;

#[cfg(any(feature = "d153", feature = "wide"))]
const PI_RAW_D512: Int512 = match Int512::from_str_radix(PI_D153_S152, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: PI_D153_S152 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const TAU_RAW_D512: Int512 = match Int512::from_str_radix(TAU_D153_S152, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: TAU_D153_S152 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const HALF_PI_RAW_D512: Int512 = match Int512::from_str_radix(HALF_PI_D153_S152, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: HALF_PI_D153_S152 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const QUARTER_PI_RAW_D512: Int512 = match Int512::from_str_radix(QUARTER_PI_D153_S152, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: QUARTER_PI_D153_S152 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const E_RAW_D512: Int512 = match Int512::from_str_radix(E_D153_S152, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: E_D153_S152 not parseable"),
};
#[cfg(any(feature = "d153", feature = "wide"))]
const GOLDEN_RAW_D512: Int512 = match Int512::from_str_radix(GOLDEN_D153_S152, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: GOLDEN_D153_S152 not parseable"),
};

#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn pi_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::types::widths::D153;
    D153::<D153_SCALE_REF>::from_bits(PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn tau_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::types::widths::D153;
    D153::<D153_SCALE_REF>::from_bits(TAU_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn half_pi_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::types::widths::D153;
    D153::<D153_SCALE_REF>::from_bits(HALF_PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::types::widths::D153;
    D153::<D153_SCALE_REF>::from_bits(QUARTER_PI_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn e_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::types::widths::D153;
    D153::<D153_SCALE_REF>::from_bits(E_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) fn golden_at_target_d153<const TARGET: u32>() -> Int512 {
    use crate::types::widths::D153;
    D153::<D153_SCALE_REF>::from_bits(GOLDEN_RAW_D512)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── D307 ────────────────────────────────────────────────────────────

#[cfg(any(feature = "d307", feature = "wide"))]
const D307_SCALE_REF: u32 = 306;

#[cfg(any(feature = "d307", feature = "wide"))]
const PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(PI_D307_S306, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: PI_D307_S306 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const TAU_RAW_D1024: Int1024 = match Int1024::from_str_radix(TAU_D307_S306, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: TAU_D307_S306 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const HALF_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(HALF_PI_D307_S306, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: HALF_PI_D307_S306 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const QUARTER_PI_RAW_D1024: Int1024 = match Int1024::from_str_radix(QUARTER_PI_D307_S306, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: QUARTER_PI_D307_S306 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const E_RAW_D1024: Int1024 = match Int1024::from_str_radix(E_D307_S306, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: E_D307_S306 not parseable"),
};
#[cfg(any(feature = "d307", feature = "wide"))]
const GOLDEN_RAW_D1024: Int1024 = match Int1024::from_str_radix(GOLDEN_D307_S306, 10) {
    Ok(v) => v,
    Err(()) => panic!("consts_wide: GOLDEN_D307_S306 not parseable"),
};

#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn pi_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::types::widths::D307;
    D307::<D307_SCALE_REF>::from_bits(PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn tau_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::types::widths::D307;
    D307::<D307_SCALE_REF>::from_bits(TAU_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn half_pi_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::types::widths::D307;
    D307::<D307_SCALE_REF>::from_bits(HALF_PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn quarter_pi_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::types::widths::D307;
    D307::<D307_SCALE_REF>::from_bits(QUARTER_PI_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn e_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::types::widths::D307;
    D307::<D307_SCALE_REF>::from_bits(E_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}
#[cfg(any(feature = "d307", feature = "wide"))]
pub(crate) fn golden_at_target_d307<const TARGET: u32>() -> Int1024 {
    use crate::types::widths::D307;
    D307::<D307_SCALE_REF>::from_bits(GOLDEN_RAW_D1024)
        .rescale::<TARGET>()
        .to_bits()
}

// ─── DecimalConstants impls ──────────────────────────────────────────────
//
// These shadow the impls that `decl_decimal_consts!(wide …)` would
// emit. To avoid duplicate trait impls, the wide-arm macro invocations
// in `types/widths.rs` were removed (search for `decl_decimal_consts!(wide`).

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> DecimalConstants for crate::types::widths::D76<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d76::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d76::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d76::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d76::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d76::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d76::<SCALE>()) }
    #[inline] fn pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D76::<D76_SCALE_REF>::from_bits(PI_RAW_D256).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn tau_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D76::<D76_SCALE_REF>::from_bits(TAU_RAW_D256).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn half_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D76::<D76_SCALE_REF>::from_bits(HALF_PI_RAW_D256).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn quarter_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D76::<D76_SCALE_REF>::from_bits(QUARTER_PI_RAW_D256).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn golden_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D76::<D76_SCALE_REF>::from_bits(GOLDEN_RAW_D256).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn e_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D76::<D76_SCALE_REF>::from_bits(E_RAW_D256).rescale_with::<SCALE>(mode).to_bits())
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> DecimalConstants for crate::types::widths::D153<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d153::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d153::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d153::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d153::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d153::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d153::<SCALE>()) }
    #[inline] fn pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D153::<D153_SCALE_REF>::from_bits(PI_RAW_D512).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn tau_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D153::<D153_SCALE_REF>::from_bits(TAU_RAW_D512).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn half_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D153::<D153_SCALE_REF>::from_bits(HALF_PI_RAW_D512).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn quarter_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D153::<D153_SCALE_REF>::from_bits(QUARTER_PI_RAW_D512).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn golden_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D153::<D153_SCALE_REF>::from_bits(GOLDEN_RAW_D512).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn e_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D153::<D153_SCALE_REF>::from_bits(E_RAW_D512).rescale_with::<SCALE>(mode).to_bits())
    }
}

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> DecimalConstants for crate::types::widths::D307<SCALE> {
    #[inline] fn pi() -> Self { Self(pi_at_target_d307::<SCALE>()) }
    #[inline] fn tau() -> Self { Self(tau_at_target_d307::<SCALE>()) }
    #[inline] fn half_pi() -> Self { Self(half_pi_at_target_d307::<SCALE>()) }
    #[inline] fn quarter_pi() -> Self { Self(quarter_pi_at_target_d307::<SCALE>()) }
    #[inline] fn golden() -> Self { Self(golden_at_target_d307::<SCALE>()) }
    #[inline] fn e() -> Self { Self(e_at_target_d307::<SCALE>()) }
    #[inline] fn pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D307::<D307_SCALE_REF>::from_bits(PI_RAW_D1024).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn tau_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D307::<D307_SCALE_REF>::from_bits(TAU_RAW_D1024).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn half_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D307::<D307_SCALE_REF>::from_bits(HALF_PI_RAW_D1024).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn quarter_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D307::<D307_SCALE_REF>::from_bits(QUARTER_PI_RAW_D1024).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn golden_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D307::<D307_SCALE_REF>::from_bits(GOLDEN_RAW_D1024).rescale_with::<SCALE>(mode).to_bits())
    }
    #[inline] fn e_with(mode: crate::support::rounding::RoundingMode) -> Self {
        Self(crate::types::widths::D307::<D307_SCALE_REF>::from_bits(E_RAW_D1024).rescale_with::<SCALE>(mode).to_bits())
    }
}

// ─── New half-width and wider tiers ──────────────────────────────────
//
// Generated per the build.rs `for &scale in &[57, 115, 230, 462, 616,
// 924, 1232]` loop. Each tier mirrors the D76 / D153 / D307 pattern:
// (1) a raw `Int*` const parsed from the build-time decimal string,
// (2) a `<const_name>_at_target_d<scale>::<TARGET>()` accessor that
// rescales down to the caller's SCALE, and (3) a `DecimalConstants` impl
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
            use crate::types::widths::$D;
            $D::<{ $scale }>::from_bits($PI_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $tau_fn<const TARGET: u32>() -> $Storage {
            use crate::types::widths::$D;
            $D::<{ $scale }>::from_bits($TAU_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $half_pi_fn<const TARGET: u32>() -> $Storage {
            use crate::types::widths::$D;
            $D::<{ $scale }>::from_bits($HALF_PI_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $quarter_pi_fn<const TARGET: u32>() -> $Storage {
            use crate::types::widths::$D;
            $D::<{ $scale }>::from_bits($QUARTER_PI_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $e_fn<const TARGET: u32>() -> $Storage {
            use crate::types::widths::$D;
            $D::<{ $scale }>::from_bits($E_RAW).rescale::<TARGET>().to_bits()
        }
        #[cfg(any(feature = $feature, feature = $umbrella))]
        pub(crate) fn $golden_fn<const TARGET: u32>() -> $Storage {
            use crate::types::widths::$D;
            $D::<{ $scale }>::from_bits($GOLDEN_RAW).rescale::<TARGET>().to_bits()
        }

        #[cfg(any(feature = $feature, feature = $umbrella))]
        impl<const SCALE: u32> DecimalConstants for crate::types::widths::$D<SCALE> {
            #[inline] fn pi() -> Self { Self($pi_fn::<SCALE>()) }
            #[inline] fn tau() -> Self { Self($tau_fn::<SCALE>()) }
            #[inline] fn half_pi() -> Self { Self($half_pi_fn::<SCALE>()) }
            #[inline] fn quarter_pi() -> Self { Self($quarter_pi_fn::<SCALE>()) }
            #[inline] fn golden() -> Self { Self($golden_fn::<SCALE>()) }
            #[inline] fn e() -> Self { Self($e_fn::<SCALE>()) }
            #[inline] fn pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
                Self(crate::types::widths::$D::<{ $scale }>::from_bits($PI_RAW).rescale_with::<SCALE>(mode).to_bits())
            }
            #[inline] fn tau_with(mode: crate::support::rounding::RoundingMode) -> Self {
                Self(crate::types::widths::$D::<{ $scale }>::from_bits($TAU_RAW).rescale_with::<SCALE>(mode).to_bits())
            }
            #[inline] fn half_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
                Self(crate::types::widths::$D::<{ $scale }>::from_bits($HALF_PI_RAW).rescale_with::<SCALE>(mode).to_bits())
            }
            #[inline] fn quarter_pi_with(mode: crate::support::rounding::RoundingMode) -> Self {
                Self(crate::types::widths::$D::<{ $scale }>::from_bits($QUARTER_PI_RAW).rescale_with::<SCALE>(mode).to_bits())
            }
            #[inline] fn golden_with(mode: crate::support::rounding::RoundingMode) -> Self {
                Self(crate::types::widths::$D::<{ $scale }>::from_bits($GOLDEN_RAW).rescale_with::<SCALE>(mode).to_bits())
            }
            #[inline] fn e_with(mode: crate::support::rounding::RoundingMode) -> Self {
                Self(crate::types::widths::$D::<{ $scale }>::from_bits($E_RAW).rescale_with::<SCALE>(mode).to_bits())
            }
        }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
use crate::wide_int::Int192;
#[cfg(any(feature = "d115", feature = "wide"))]
use crate::wide_int::Int384;
#[cfg(any(feature = "d230", feature = "wide"))]
use crate::wide_int::Int768;
#[cfg(any(feature = "d462", feature = "x-wide"))]
use crate::wide_int::Int1536;
#[cfg(any(feature = "d616", feature = "x-wide"))]
use crate::wide_int::Int2048;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
use crate::wide_int::Int3072;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
use crate::wide_int::Int4096;

// SCALE_REF per tier = highest k where τ × 10^k still fits the
// storage's signed range. Computed in build.rs and matched here.
// D57/D115/D462/D616/D924/D1232 cannot use their nominal tier max
// because τ ≈ 6.28 pushes just past the i_max boundary; D230 and
// D1232 borderline cases work at 230 / 1231 respectively.
decl_wide_consts_tier!(
    D57, Int192, 56, D57_SCALE_REF,
    PI_D57_S56, TAU_D57_S56, HALF_PI_D57_S56, QUARTER_PI_D57_S56, E_D57_S56, GOLDEN_D57_S56,
    PI_RAW_D192, TAU_RAW_D192, HALF_PI_RAW_D192, QUARTER_PI_RAW_D192, E_RAW_D192, GOLDEN_RAW_D192,
    pi_at_target_d57, tau_at_target_d57, half_pi_at_target_d57,
    quarter_pi_at_target_d57, e_at_target_d57, golden_at_target_d57,
    "d57", "wide",
);

decl_wide_consts_tier!(
    D115, Int384, 114, D115_SCALE_REF,
    PI_D115_S114, TAU_D115_S114, HALF_PI_D115_S114, QUARTER_PI_D115_S114, E_D115_S114, GOLDEN_D115_S114,
    PI_RAW_D384, TAU_RAW_D384, HALF_PI_RAW_D384, QUARTER_PI_RAW_D384, E_RAW_D384, GOLDEN_RAW_D384,
    pi_at_target_d115, tau_at_target_d115, half_pi_at_target_d115,
    quarter_pi_at_target_d115, e_at_target_d115, golden_at_target_d115,
    "d115", "wide",
);

decl_wide_consts_tier!(
    D230, Int768, 229, D230_SCALE_REF,
    PI_D230_S229, TAU_D230_S229, HALF_PI_D230_S229, QUARTER_PI_D230_S229, E_D230_S229, GOLDEN_D230_S229,
    PI_RAW_D768, TAU_RAW_D768, HALF_PI_RAW_D768, QUARTER_PI_RAW_D768, E_RAW_D768, GOLDEN_RAW_D768,
    pi_at_target_d230, tau_at_target_d230, half_pi_at_target_d230,
    quarter_pi_at_target_d230, e_at_target_d230, golden_at_target_d230,
    "d230", "wide",
);

decl_wide_consts_tier!(
    D462, Int1536, 461, D462_SCALE_REF,
    PI_D462_S461, TAU_D462_S461, HALF_PI_D462_S461, QUARTER_PI_D462_S461, E_D462_S461, GOLDEN_D462_S461,
    PI_RAW_D1536, TAU_RAW_D1536, HALF_PI_RAW_D1536, QUARTER_PI_RAW_D1536, E_RAW_D1536, GOLDEN_RAW_D1536,
    pi_at_target_d462, tau_at_target_d462, half_pi_at_target_d462,
    quarter_pi_at_target_d462, e_at_target_d462, golden_at_target_d462,
    "d462", "x-wide",
);

decl_wide_consts_tier!(
    D616, Int2048, 615, D616_SCALE_REF,
    PI_D616_S615, TAU_D616_S615, HALF_PI_D616_S615, QUARTER_PI_D616_S615, E_D616_S615, GOLDEN_D616_S615,
    PI_RAW_D2048, TAU_RAW_D2048, HALF_PI_RAW_D2048, QUARTER_PI_RAW_D2048, E_RAW_D2048, GOLDEN_RAW_D2048,
    pi_at_target_d616, tau_at_target_d616, half_pi_at_target_d616,
    quarter_pi_at_target_d616, e_at_target_d616, golden_at_target_d616,
    "d616", "x-wide",
);

decl_wide_consts_tier!(
    D924, Int3072, 923, D924_SCALE_REF,
    PI_D924_S923, TAU_D924_S923, HALF_PI_D924_S923, QUARTER_PI_D924_S923, E_D924_S923, GOLDEN_D924_S923,
    PI_RAW_D3072, TAU_RAW_D3072, HALF_PI_RAW_D3072, QUARTER_PI_RAW_D3072, E_RAW_D3072, GOLDEN_RAW_D3072,
    pi_at_target_d924, tau_at_target_d924, half_pi_at_target_d924,
    quarter_pi_at_target_d924, e_at_target_d924, golden_at_target_d924,
    "d924", "xx-wide",
);

decl_wide_consts_tier!(
    D1232, Int4096, 1231, D1232_SCALE_REF,
    PI_D1232_S1231, TAU_D1232_S1231, HALF_PI_D1232_S1231, QUARTER_PI_D1232_S1231, E_D1232_S1231, GOLDEN_D1232_S1231,
    PI_RAW_D4096, TAU_RAW_D4096, HALF_PI_RAW_D4096, QUARTER_PI_RAW_D4096, E_RAW_D4096, GOLDEN_RAW_D4096,
    pi_at_target_d1232, tau_at_target_d1232, half_pi_at_target_d1232,
    quarter_pi_at_target_d1232, e_at_target_d1232, golden_at_target_d1232,
    "d1232", "xx-wide",
);
