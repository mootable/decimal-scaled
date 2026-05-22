//! Trigonometric policy.
//!
//! Narrow tier (D18 / D38) covers forward + inverse + atan2 on
//! the `Fixed` 256-bit intermediate. Wide tier (D57 .. D1232) covers
//! the forward kernels (sin / cos / tan / atan) via per-tier kernels
//! in [`crate::algos::trig::wide_kernel`]; the inverse family
//! (asin / acos / atan2) for the wide tiers continues to delegate to
//! the macro-emitted inherent methods on the type, which compose
//! atan_fixed + sqrt_fixed + half_pi internally (no separate algos
//! kernel needed today).
//!
//! The wide-tier macro does not ship runtime-`working_digits` variants
//! of `sin_fixed` / `cos_fixed` / `tan_fixed` / `atan_fixed`, so the
//! `*_with_impl` methods for wide tiers ignore the caller-supplied
//! digits and delegate to the strict path (matching the precedent set
//! in [`crate::policy::ln`]).

use crate::algos::trig;
use crate::policy::triplet::{policy_triplet, wtag};
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

pub(crate) trait TrigPolicy: Sized {
    fn sin_impl(self, mode: RoundingMode) -> Self;
    fn sin_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn cos_impl(self, mode: RoundingMode) -> Self;
    fn cos_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn tan_impl(self, mode: RoundingMode) -> Self;
    fn tan_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atan_impl(self, mode: RoundingMode) -> Self;
    fn atan_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn asin_impl(self, mode: RoundingMode) -> Self;
    fn asin_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn acos_impl(self, mode: RoundingMode) -> Self;
    fn acos_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self;
    fn atan2_with_impl(self, other: Self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Hyperbolic family ──────────────────────────────────────────
    fn sinh_impl(self, mode: RoundingMode) -> Self;
    fn sinh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn cosh_impl(self, mode: RoundingMode) -> Self;
    fn cosh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn tanh_impl(self, mode: RoundingMode) -> Self;
    fn tanh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn asinh_impl(self, mode: RoundingMode) -> Self;
    fn asinh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn acosh_impl(self, mode: RoundingMode) -> Self;
    fn acosh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atanh_impl(self, mode: RoundingMode) -> Self;
    fn atanh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Angle conversions ─────────────────────────────────────────
    fn to_degrees_impl(self, mode: RoundingMode) -> Self;
    fn to_degrees_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn to_radians_impl(self, mode: RoundingMode) -> Self;
    fn to_radians_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

/// Emits the narrow-tier `TrigPolicy` impl that widens to D38, calls
/// the D38 method, then narrows back. The forward family
/// (sin/cos/tan/atan/asin/acos/atan2) uses dedicated `widen_to_d38`
/// kernels; the hyperbolics and angle conversions widen via the same
/// `TryInto` shape the macro-emitted shells already use.
macro_rules! impl_narrow_trig {
    ($T:ident,
     $sin_s:path, $sin_w:path,
     $cos_s:path, $cos_w:path,
     $tan_s:path, $tan_w:path,
     $atan_s:path, $atan_w:path,
     $asin_s:path, $asin_w:path,
     $acos_s:path, $acos_w:path,
     $atan2_s:path, $atan2_w:path
    ) => {
        impl<const SCALE: u32> TrigPolicy for $T<SCALE> {
            #[inline]
            fn sin_impl(self, mode: RoundingMode) -> Self {
                $sin_s(self, mode)
            }
            #[inline]
            fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $sin_w(self, wd, mode)
            }
            #[inline]
            fn cos_impl(self, mode: RoundingMode) -> Self {
                $cos_s(self, mode)
            }
            #[inline]
            fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $cos_w(self, wd, mode)
            }
            #[inline]
            fn tan_impl(self, mode: RoundingMode) -> Self {
                $tan_s(self, mode)
            }
            #[inline]
            fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $tan_w(self, wd, mode)
            }
            #[inline]
            fn atan_impl(self, mode: RoundingMode) -> Self {
                $atan_s(self, mode)
            }
            #[inline]
            fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $atan_w(self, wd, mode)
            }
            #[inline]
            fn asin_impl(self, mode: RoundingMode) -> Self {
                $asin_s(self, mode)
            }
            #[inline]
            fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $asin_w(self, wd, mode)
            }
            #[inline]
            fn acos_impl(self, mode: RoundingMode) -> Self {
                $acos_s(self, mode)
            }
            #[inline]
            fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                $acos_w(self, wd, mode)
            }
            #[inline]
            fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
                $atan2_s(self, other, mode)
            }
            #[inline]
            fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self {
                $atan2_w(self, other, wd, mode)
            }

            // Hyperbolics and angle conversions widen → D38 → narrow.
            #[inline]
            fn sinh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::sinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn sinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::sinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn cosh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::cosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn cosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::cosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn tanh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::tanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn tanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::tanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn asinh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::asinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn asinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::asinh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn acosh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::acosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn acosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::acosh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn atanh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::atanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn atanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_approx_with(wd, mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::atanh"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn to_degrees_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict_with(mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_degrees"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn to_degrees_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_approx_with(wd, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_degrees"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn to_radians_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_strict_with(mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_radians"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            fn to_radians_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_approx_with(wd, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::to_radians"),
                            SCALE,
                        )
                    })
            }
        }
    };
}


impl_narrow_trig!(
    D18,
    trig::widen_to_d38::sin_strict_d18,
    trig::widen_to_d38::sin_with_d18,
    trig::widen_to_d38::cos_strict_d18,
    trig::widen_to_d38::cos_with_d18,
    trig::widen_to_d38::tan_strict_d18,
    trig::widen_to_d38::tan_with_d18,
    trig::widen_to_d38::atan_strict_d18,
    trig::widen_to_d38::atan_with_d18,
    trig::widen_to_d38::asin_strict_d18,
    trig::widen_to_d38::asin_with_d18,
    trig::widen_to_d38::acos_strict_d18,
    trig::widen_to_d38::acos_with_d18,
    trig::widen_to_d38::atan2_strict_d18,
    trig::widen_to_d38::atan2_with_d18
);

// D38 — see `crate::policy::ln` for the borrow-D57 rationale.
//
// When D57 is available, sin / cos / tan / atan / asin / acos / atan2
// all route through `borrow_d57`. The `_with` variants collapse to
// strict because the D57 wide_kernel has no runtime-`working_digits`
// path. `fixed_d38::*` is retained as an alternate kernel.

/// D38 hyperbolic + angle-conversion methods share one `Fixed` core
/// regardless of whether the forward trig path borrows D57 or runs
/// `fixed_d38`. Emit them once so both cfg branches stay short.
macro_rules! d38_hyperbolic_and_angle {
    () => {
        #[inline]
        fn sinh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::sinh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn sinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::sinh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn cosh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::cosh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn cosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::cosh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn tanh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::tanh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn tanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::tanh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn asinh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::asinh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn asinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::asinh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn acosh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::acosh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn acosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::acosh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn atanh_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::atanh_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn atanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::atanh_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn to_degrees_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_degrees_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn to_degrees_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_degrees_with(self.0, SCALE, wd, mode))
        }
        #[inline]
        fn to_radians_impl(self, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_radians_strict::<SCALE>(self.0, mode))
        }
        #[inline]
        fn to_radians_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
            Self(trig::fixed_d38::to_radians_with(self.0, SCALE, wd, mode))
        }
    };
}

// D38 — route forward trig (sin / cos / tan) and the hyperbolic
// family through `fixed_d38`; keep the inverse trig family
// (atan / asin / acos / atan2) on the borrow_d57 path. With the
// 0.4.2 MG-routed `Fixed` primitives the bespoke kernel wins ~2x
// on sin / cos / tan, but the wide_kernel's atan algorithm is
// qualitatively faster than fixed_d38's adaptive-halvings path
// (bench-trial at SCALE 19: borrow_d57 atan ~28 µs, fixed_d38 atan
// ~66 µs). asin / acos / atan2 compose `atan` internally, so they
// inherit that gap.
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for D38<SCALE> {
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::sin_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::sin_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::cos_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::cos_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::tan_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::tan_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::atan_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::atan_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::asin_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::asin_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::acos_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::acos_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::atan2_strict::<SCALE>(
            self.0, other.0, mode,
        ))
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::borrow_d57::atan2_strict::<SCALE>(
            self.0, other.0, mode,
        ))
    }

    d38_hyperbolic_and_angle!();
}

#[cfg(not(any(feature = "d57", feature = "wide")))]
impl<const SCALE: u32> TrigPolicy for D38<SCALE> {
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::sin_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::sin_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::cos_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::cos_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::tan_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::tan_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::atan_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::atan_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::asin_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::asin_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::acos_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::acos_with::<SCALE>(self.0, wd, mode))
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::atan2_strict::<SCALE>(
            self.0, other.0, mode,
        ))
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self {
        Self(trig::fixed_d38::atan2_with::<SCALE>(
            self.0, other.0, wd, mode,
        ))
    }

    d38_hyperbolic_and_angle!();
}

// ── Wide tiers — base/std/no_std triplet keyed on `match (W, SCALE)` ─
//
// The forward family (sin / cos / tan / atan) routes through the
// `policy_triplet!` free fns per width: each has a const-folded
// `match (W, SCALE)` base arm table that selects either a per-band
// lookup kernel or the generic `wide_kernel`. `std` is identical to
// `base` for every trig method — every trig kernel is pure-integer; the
// only std machinery is the wide-kernel constant cache (a later hoist
// concern, not visible to the policy).
//
// The inverse family (asin / acos / atan2), the hyperbolics, and the
// angle conversions keep their existing bodies: most delegate to the
// macro-emitted inherent `*_strict_with` shells (no raw-storage free-fn
// equivalent), and the few per-band lookup arms (e.g. D57 inverse,
// D57 / D115 / D153 / D307 hyperbolics) stay hand-written because their
// fall-through is an inherent method, not a raw-storage kernel.

/// Default delegating tail shared by the wide-tier `TrigPolicy` impls:
/// the inverse family (asin / acos / atan2), hyperbolics, and angle
/// conversions all delegate to the macro-emitted inherent
/// `*_strict_with` methods. Width-specific per-band overrides (D57
/// inverse / hyper, D115 / D153 / D307 hyper) replace this with a
/// hand-written tail.
macro_rules! wide_trig_delegating_tail {
    () => {
        #[inline]
        fn asin_impl(self, mode: RoundingMode) -> Self {
            self.asin_strict_with(mode)
        }
        #[inline]
        fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.asin_strict_with(mode)
        }
        #[inline]
        fn acos_impl(self, mode: RoundingMode) -> Self {
            self.acos_strict_with(mode)
        }
        #[inline]
        fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.acos_strict_with(mode)
        }
        #[inline]
        fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
            self.atan2_strict_with(other, mode)
        }
        #[inline]
        fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
            self.atan2_strict_with(other, mode)
        }

        #[inline]
        fn sinh_impl(self, mode: RoundingMode) -> Self {
            self.sinh_strict_with(mode)
        }
        #[inline]
        fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.sinh_strict_with(mode)
        }
        #[inline]
        fn cosh_impl(self, mode: RoundingMode) -> Self {
            self.cosh_strict_with(mode)
        }
        #[inline]
        fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.cosh_strict_with(mode)
        }
        #[inline]
        fn tanh_impl(self, mode: RoundingMode) -> Self {
            self.tanh_strict_with(mode)
        }
        #[inline]
        fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.tanh_strict_with(mode)
        }
        #[inline]
        fn asinh_impl(self, mode: RoundingMode) -> Self {
            self.asinh_strict_with(mode)
        }
        #[inline]
        fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.asinh_strict_with(mode)
        }
        #[inline]
        fn acosh_impl(self, mode: RoundingMode) -> Self {
            self.acosh_strict_with(mode)
        }
        #[inline]
        fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.acosh_strict_with(mode)
        }
        #[inline]
        fn atanh_impl(self, mode: RoundingMode) -> Self {
            self.atanh_strict_with(mode)
        }
        #[inline]
        fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.atanh_strict_with(mode)
        }

        #[inline]
        fn to_degrees_impl(self, mode: RoundingMode) -> Self {
            self.to_degrees_strict_with(mode)
        }
        #[inline]
        fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.to_degrees_strict_with(mode)
        }
        #[inline]
        fn to_radians_impl(self, mode: RoundingMode) -> Self {
            self.to_radians_strict_with(mode)
        }
        #[inline]
        fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            self.to_radians_strict_with(mode)
        }
    };
}

/// Emits the forward-family trait methods (sin / cos / tan / atan, each
/// strict + `_with`) for one wide tier, routing through the supplied
/// triplet free fns (the `_with` forms drop `working_digits`).
macro_rules! wide_trig_forward_methods {
    (
        $T:ident,
        $sin_std:ident,  $sin_no:ident,
        $cos_std:ident,  $cos_no:ident,
        $tan_std:ident,  $tan_no:ident,
        $atan_std:ident, $atan_no:ident
    ) => {
        #[inline]
        fn sin_impl(self, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($sin_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($sin_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
        #[inline]
        fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($sin_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($sin_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
        #[inline]
        fn cos_impl(self, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($cos_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($cos_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
        #[inline]
        fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($cos_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($cos_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
        #[inline]
        fn tan_impl(self, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($tan_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($tan_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
        #[inline]
        fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($tan_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($tan_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
        #[inline]
        fn atan_impl(self, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($atan_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($atan_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
        #[inline]
        fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
            #[cfg(feature = "std")]
            {
                Self($atan_std::<{ wtag::$T }, SCALE>(self.0, mode))
            }
            #[cfg(not(feature = "std"))]
            {
                Self($atan_no::<{ wtag::$T }, SCALE>(self.0, mode))
            }
        }
    };
}

// ── D57 — forward-family triplets (sin / cos / tan / atan) ──────────
//
// Bespoke band arms divert SCALE 18..=22 (sincos/atan) and 44..=56
// (sin/cos/atan; tan has no 44..=56 band) before the generic kernel.
#[cfg(any(feature = "d57", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<3>,
    base_fn = sin_d57_base, std_fn = sin_d57_std, no_std_fn = sin_d57_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D57, 18..=22) => trig::lookup_d57_s18_22_sincos::sin_strict::<SCALE>(raw, mode),
        (wtag::D57, 44..=56) => trig::lookup_d57_s44_56_sincos::sin_strict::<SCALE>(raw, mode),
        (wtag::D57, _)       => trig::wide_kernel::sin_strict_d57(raw, mode, SCALE)
    },
    std = {},
}
#[cfg(any(feature = "d57", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<3>,
    base_fn = cos_d57_base, std_fn = cos_d57_std, no_std_fn = cos_d57_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D57, 18..=22) => trig::lookup_d57_s18_22_sincos::cos_strict::<SCALE>(raw, mode),
        (wtag::D57, 44..=56) => trig::lookup_d57_s44_56_sincos::cos_strict::<SCALE>(raw, mode),
        (wtag::D57, _)       => trig::wide_kernel::cos_strict_d57(raw, mode, SCALE)
    },
    std = {},
}
#[cfg(any(feature = "d57", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<3>,
    base_fn = tan_d57_base, std_fn = tan_d57_std, no_std_fn = tan_d57_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D57, 18..=22) => trig::lookup_d57_s18_22_sincos::tan_strict::<SCALE>(raw, mode),
        (wtag::D57, _)       => trig::wide_kernel::tan_strict_d57(raw, mode, SCALE)
    },
    std = {},
}
#[cfg(any(feature = "d57", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<3>,
    base_fn = atan_d57_base, std_fn = atan_d57_std, no_std_fn = atan_d57_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D57, 18..=22) => trig::lookup_d57_s18_22_atan::atan_strict::<SCALE>(raw, mode),
        (wtag::D57, 44..=56) => trig::lookup_d57_s44_56_atan::atan_strict::<SCALE>(raw, mode),
        (wtag::D57, _)       => trig::wide_kernel::atan_strict_d57(raw, mode, SCALE)
    },
    std = {},
}

// D57 — inverse and hyperbolic families keep their per-band lookup
// arms (SCALE 18..=22) and inherent fall-through; only the forward
// family routes through the triplet.
#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D57<SCALE> {
    wide_trig_forward_methods!(
        D57,
        sin_d57_std,
        sin_d57_no_std,
        cos_d57_std,
        cos_d57_no_std,
        tan_d57_std,
        tan_d57_no_std,
        atan_d57_std,
        atan_d57_no_std
    );

    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_inverse::asin_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.asin_strict_with(mode),
        }
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_inverse::asin_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.asin_strict_with(mode),
        }
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_inverse::acos_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.acos_strict_with(mode),
        }
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_inverse::acos_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.acos_strict_with(mode),
        }
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_inverse::atan2_strict::<SCALE>(
                self.0, other.0, mode,
            )),
            _ => self.atan2_strict_with(other, mode),
        }
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_inverse::atan2_strict::<SCALE>(
                self.0, other.0, mode,
            )),
            _ => self.atan2_strict_with(other, mode),
        }
    }

    // Hyperbolics + angle conversions — delegate to inherent shells.
    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            18..=22 => Self(trig::lookup_d57_s18_22_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn asinh_impl(self, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn acosh_impl(self, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn atanh_impl(self, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn to_degrees_impl(self, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_radians_impl(self, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
    #[inline]
    fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
}

// ── D76 — width default (no bands) ─────────────────────────────────
#[cfg(any(feature = "d76", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<4>,
    base_fn = sin_d76_base, std_fn = sin_d76_std, no_std_fn = sin_d76_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D76, _) => trig::wide_kernel::sin_strict_d76(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d76", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<4>,
    base_fn = cos_d76_base, std_fn = cos_d76_std, no_std_fn = cos_d76_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D76, _) => trig::wide_kernel::cos_strict_d76(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d76", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<4>,
    base_fn = tan_d76_base, std_fn = tan_d76_std, no_std_fn = tan_d76_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D76, _) => trig::wide_kernel::tan_strict_d76(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d76", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<4>,
    base_fn = atan_d76_base, std_fn = atan_d76_std, no_std_fn = atan_d76_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D76, _) => trig::wide_kernel::atan_strict_d76(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D76<SCALE> {
    wide_trig_forward_methods!(
        D76,
        sin_d76_std,
        sin_d76_no_std,
        cos_d76_std,
        cos_d76_no_std,
        tan_d76_std,
        tan_d76_no_std,
        atan_d76_std,
        atan_d76_no_std
    );
    wide_trig_delegating_tail!();
}

// ── D115 — forward via wide_kernel; sinh/cosh/tanh divert SCALE
// 50..=60 through the Tang-style hyper lookup (hand-written tail). ──
#[cfg(any(feature = "d115", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<6>,
    base_fn = sin_d115_base, std_fn = sin_d115_std, no_std_fn = sin_d115_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D115, _) => trig::wide_kernel::sin_strict_d115(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d115", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<6>,
    base_fn = cos_d115_base, std_fn = cos_d115_std, no_std_fn = cos_d115_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D115, _) => trig::wide_kernel::cos_strict_d115(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d115", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<6>,
    base_fn = tan_d115_base, std_fn = tan_d115_std, no_std_fn = tan_d115_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D115, _) => trig::wide_kernel::tan_strict_d115(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d115", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<6>,
    base_fn = atan_d115_base, std_fn = atan_d115_std, no_std_fn = atan_d115_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D115, _) => trig::wide_kernel::atan_strict_d115(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D115<SCALE> {
    wide_trig_forward_methods!(
        D115,
        sin_d115_std,
        sin_d115_no_std,
        cos_d115_std,
        cos_d115_no_std,
        tan_d115_std,
        tan_d115_no_std,
        atan_d115_std,
        atan_d115_no_std
    );

    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }

    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            50..=60 => Self(trig::lookup_d115_s57_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            50..=60 => Self(trig::lookup_d115_s57_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            50..=60 => Self(trig::lookup_d115_s57_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            50..=60 => Self(trig::lookup_d115_s57_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            50..=60 => Self(trig::lookup_d115_s57_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            50..=60 => Self(trig::lookup_d115_s57_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn asinh_impl(self, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn acosh_impl(self, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn atanh_impl(self, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn to_degrees_impl(self, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_radians_impl(self, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
    #[inline]
    fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
}

// ── D153 — forward sin/cos/tan/atan divert SCALE 70..=82; sinh/cosh/
// tanh divert the same band (hand-written tail). ───────────────────
#[cfg(any(feature = "d153", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<8>,
    base_fn = sin_d153_base, std_fn = sin_d153_std, no_std_fn = sin_d153_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D153, 70..=82) => trig::lookup_d153_s70_82_sincos::sin_strict::<SCALE>(raw, mode),
        (wtag::D153, _)       => trig::wide_kernel::sin_strict_d153(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d153", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<8>,
    base_fn = cos_d153_base, std_fn = cos_d153_std, no_std_fn = cos_d153_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D153, 70..=82) => trig::lookup_d153_s70_82_sincos::cos_strict::<SCALE>(raw, mode),
        (wtag::D153, _)       => trig::wide_kernel::cos_strict_d153(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d153", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<8>,
    base_fn = tan_d153_base, std_fn = tan_d153_std, no_std_fn = tan_d153_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D153, 70..=82) => trig::lookup_d153_s70_82_sincos::tan_strict::<SCALE>(raw, mode),
        (wtag::D153, _)       => trig::wide_kernel::tan_strict_d153(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d153", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<8>,
    base_fn = atan_d153_base, std_fn = atan_d153_std, no_std_fn = atan_d153_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D153, 70..=82) => trig::lookup_d153_s70_82_atan::atan_strict::<SCALE>(raw, mode),
        (wtag::D153, _)       => trig::wide_kernel::atan_strict_d153(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D153<SCALE> {
    wide_trig_forward_methods!(
        D153,
        sin_d153_std,
        sin_d153_no_std,
        cos_d153_std,
        cos_d153_no_std,
        tan_d153_std,
        tan_d153_no_std,
        atan_d153_std,
        atan_d153_no_std
    );

    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }

    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            70..=82 => Self(trig::lookup_d153_s70_82_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            70..=82 => Self(trig::lookup_d153_s70_82_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            70..=82 => Self(trig::lookup_d153_s70_82_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            70..=82 => Self(trig::lookup_d153_s70_82_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            70..=82 => Self(trig::lookup_d153_s70_82_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            70..=82 => Self(trig::lookup_d153_s70_82_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn asinh_impl(self, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn acosh_impl(self, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn atanh_impl(self, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn to_degrees_impl(self, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_radians_impl(self, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
    #[inline]
    fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
}

// ── D230 — width default (no bands) ────────────────────────────────
#[cfg(any(feature = "d230", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<12>,
    base_fn = sin_d230_base, std_fn = sin_d230_std, no_std_fn = sin_d230_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D230, _) => trig::wide_kernel::sin_strict_d230(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d230", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<12>,
    base_fn = cos_d230_base, std_fn = cos_d230_std, no_std_fn = cos_d230_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D230, _) => trig::wide_kernel::cos_strict_d230(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d230", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<12>,
    base_fn = tan_d230_base, std_fn = tan_d230_std, no_std_fn = tan_d230_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D230, _) => trig::wide_kernel::tan_strict_d230(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d230", feature = "wide"))]
policy_triplet! {
    storage = crate::int::types::Int<12>,
    base_fn = atan_d230_base, std_fn = atan_d230_std, no_std_fn = atan_d230_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D230, _) => trig::wide_kernel::atan_strict_d230(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D230<SCALE> {
    wide_trig_forward_methods!(
        D230,
        sin_d230_std,
        sin_d230_no_std,
        cos_d230_std,
        cos_d230_no_std,
        tan_d230_std,
        tan_d230_no_std,
        atan_d230_std,
        atan_d230_no_std
    );
    wide_trig_delegating_tail!();
}

// ── D307 — forward sin/cos/tan/atan divert SCALE 140..=160; sinh/
// cosh/tanh divert the same band (hand-written tail). ──────────────
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<16>,
    base_fn = sin_d307_base, std_fn = sin_d307_std, no_std_fn = sin_d307_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D307, 140..=160) => trig::lookup_d307_s140_160_sincos::sin_strict::<SCALE>(raw, mode),
        (wtag::D307, _)         => trig::wide_kernel::sin_strict_d307(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<16>,
    base_fn = cos_d307_base, std_fn = cos_d307_std, no_std_fn = cos_d307_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D307, 140..=160) => trig::lookup_d307_s140_160_sincos::cos_strict::<SCALE>(raw, mode),
        (wtag::D307, _)         => trig::wide_kernel::cos_strict_d307(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<16>,
    base_fn = tan_d307_base, std_fn = tan_d307_std, no_std_fn = tan_d307_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D307, 140..=160) => trig::lookup_d307_s140_160_sincos::tan_strict::<SCALE>(raw, mode),
        (wtag::D307, _)         => trig::wide_kernel::tan_strict_d307(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<16>,
    base_fn = atan_d307_base, std_fn = atan_d307_std, no_std_fn = atan_d307_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D307, 140..=160) => trig::lookup_d307_s140_160_atan::atan_strict::<SCALE>(raw, mode),
        (wtag::D307, _)         => trig::wide_kernel::atan_strict_d307(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D307<SCALE> {
    wide_trig_forward_methods!(
        D307,
        sin_d307_std,
        sin_d307_no_std,
        cos_d307_std,
        cos_d307_no_std,
        tan_d307_std,
        tan_d307_no_std,
        atan_d307_std,
        atan_d307_no_std
    );

    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }

    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            140..=160 => Self(trig::lookup_d307_s140_160_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            140..=160 => Self(trig::lookup_d307_s140_160_hyper::sinh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.sinh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            140..=160 => Self(trig::lookup_d307_s140_160_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            140..=160 => Self(trig::lookup_d307_s140_160_hyper::cosh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.cosh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self {
        match SCALE {
            140..=160 => Self(trig::lookup_d307_s140_160_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        match SCALE {
            140..=160 => Self(trig::lookup_d307_s140_160_hyper::tanh_strict::<SCALE>(
                self.0, mode,
            )),
            _ => self.tanh_strict_with(mode),
        }
    }
    #[inline]
    fn asinh_impl(self, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asinh_strict_with(mode)
    }
    #[inline]
    fn acosh_impl(self, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acosh_strict_with(mode)
    }
    #[inline]
    fn atanh_impl(self, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.atanh_strict_with(mode)
    }
    #[inline]
    fn to_degrees_impl(self, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_degrees_strict_with(mode)
    }
    #[inline]
    fn to_radians_impl(self, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
    #[inline]
    fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.to_radians_strict_with(mode)
    }
}

// ── D462 — forward sin/cos/tan/atan divert SCALE 225..=235; the
// hyperbolics keep the inherent shells (Tang slot lost here). ──────
#[cfg(any(feature = "d462", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<24>,
    base_fn = sin_d462_base, std_fn = sin_d462_std, no_std_fn = sin_d462_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D462, 225..=235) => trig::lookup_d462_s225_235_sincos::sin_strict::<SCALE>(raw, mode),
        (wtag::D462, _)         => trig::wide_kernel::sin_strict_d462(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d462", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<24>,
    base_fn = cos_d462_base, std_fn = cos_d462_std, no_std_fn = cos_d462_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D462, 225..=235) => trig::lookup_d462_s225_235_sincos::cos_strict::<SCALE>(raw, mode),
        (wtag::D462, _)         => trig::wide_kernel::cos_strict_d462(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d462", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<24>,
    base_fn = tan_d462_base, std_fn = tan_d462_std, no_std_fn = tan_d462_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D462, 225..=235) => trig::lookup_d462_s225_235_sincos::tan_strict::<SCALE>(raw, mode),
        (wtag::D462, _)         => trig::wide_kernel::tan_strict_d462(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d462", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<24>,
    base_fn = atan_d462_base, std_fn = atan_d462_std, no_std_fn = atan_d462_no_std,
    recv = raw, mode = mode, params = {},
    base = {
        (wtag::D462, 225..=235) => trig::lookup_d462_s225_235_atan::atan_strict::<SCALE>(raw, mode),
        (wtag::D462, _)         => trig::wide_kernel::atan_strict_d462(raw, mode, SCALE)
    }, std = {},
}
#[cfg(any(feature = "d462", feature = "x-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D462<SCALE> {
    wide_trig_forward_methods!(
        D462,
        sin_d462_std,
        sin_d462_no_std,
        cos_d462_std,
        cos_d462_no_std,
        tan_d462_std,
        tan_d462_no_std,
        atan_d462_std,
        atan_d462_no_std
    );
    wide_trig_delegating_tail!();
}

// ── D616 — width default (no bands) ────────────────────────────────
#[cfg(any(feature = "d616", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<32>,
    base_fn = sin_d616_base, std_fn = sin_d616_std, no_std_fn = sin_d616_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D616, _) => trig::wide_kernel::sin_strict_d616(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d616", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<32>,
    base_fn = cos_d616_base, std_fn = cos_d616_std, no_std_fn = cos_d616_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D616, _) => trig::wide_kernel::cos_strict_d616(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d616", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<32>,
    base_fn = tan_d616_base, std_fn = tan_d616_std, no_std_fn = tan_d616_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D616, _) => trig::wide_kernel::tan_strict_d616(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d616", feature = "x-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<32>,
    base_fn = atan_d616_base, std_fn = atan_d616_std, no_std_fn = atan_d616_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D616, _) => trig::wide_kernel::atan_strict_d616(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d616", feature = "x-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D616<SCALE> {
    wide_trig_forward_methods!(
        D616,
        sin_d616_std,
        sin_d616_no_std,
        cos_d616_std,
        cos_d616_no_std,
        tan_d616_std,
        tan_d616_no_std,
        atan_d616_std,
        atan_d616_no_std
    );
    wide_trig_delegating_tail!();
}

// ── D924 — width default (no bands) ────────────────────────────────
#[cfg(any(feature = "d924", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<48>,
    base_fn = sin_d924_base, std_fn = sin_d924_std, no_std_fn = sin_d924_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D924, _) => trig::wide_kernel::sin_strict_d924(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d924", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<48>,
    base_fn = cos_d924_base, std_fn = cos_d924_std, no_std_fn = cos_d924_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D924, _) => trig::wide_kernel::cos_strict_d924(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d924", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<48>,
    base_fn = tan_d924_base, std_fn = tan_d924_std, no_std_fn = tan_d924_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D924, _) => trig::wide_kernel::tan_strict_d924(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d924", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<48>,
    base_fn = atan_d924_base, std_fn = atan_d924_std, no_std_fn = atan_d924_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D924, _) => trig::wide_kernel::atan_strict_d924(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D924<SCALE> {
    wide_trig_forward_methods!(
        D924,
        sin_d924_std,
        sin_d924_no_std,
        cos_d924_std,
        cos_d924_no_std,
        tan_d924_std,
        tan_d924_no_std,
        atan_d924_std,
        atan_d924_no_std
    );
    wide_trig_delegating_tail!();
}

// ── D1232 — width default (no bands) ───────────────────────────────
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<64>,
    base_fn = sin_d1232_base, std_fn = sin_d1232_std, no_std_fn = sin_d1232_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D1232, _) => trig::wide_kernel::sin_strict_d1232(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<64>,
    base_fn = cos_d1232_base, std_fn = cos_d1232_std, no_std_fn = cos_d1232_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D1232, _) => trig::wide_kernel::cos_strict_d1232(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<64>,
    base_fn = tan_d1232_base, std_fn = tan_d1232_std, no_std_fn = tan_d1232_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D1232, _) => trig::wide_kernel::tan_strict_d1232(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
policy_triplet! {
    storage = crate::int::types::Int<64>,
    base_fn = atan_d1232_base, std_fn = atan_d1232_std, no_std_fn = atan_d1232_no_std,
    recv = raw, mode = mode, params = {},
    base = { (wtag::D1232, _) => trig::wide_kernel::atan_strict_d1232(raw, mode, SCALE) }, std = {},
}
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D1232<SCALE> {
    wide_trig_forward_methods!(
        D1232,
        sin_d1232_std,
        sin_d1232_no_std,
        cos_d1232_std,
        cos_d1232_no_std,
        tan_d1232_std,
        tan_d1232_no_std,
        atan_d1232_std,
        atan_d1232_no_std
    );
    wide_trig_delegating_tail!();
}
