//! Exponential policy.
//!
//! Same cascade shape as [`crate::policy::ln`]: narrow tier on the
//! `Fixed` 256-bit intermediate, wide tier on per-tier `exp_strict`
//! kernels in [`crate::algos::exp::wide_kernel`]. The wide-tier macro
//! does not ship a runtime-`working_digits` variant of `exp_fixed`, so
//! the wide-tier `*_with_impl` methods ignore the caller-supplied
//! digits and delegate to the strict path.
//!
//! Functions covered: `exp` (natural) and `exp2` (base-2).

use crate::algos::exp;
use crate::policy::triplet::{policy_triplet, wtag};
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D9, D18, D38};

pub(crate) trait ExpPolicy: Sized {
    /// `e^self` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn exp_impl(self, mode: RoundingMode) -> Self;

    /// `e^self` with caller-chosen working digits.
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    /// `2^self` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn exp2_impl(self, mode: RoundingMode) -> Self;

    /// `2^self` with caller-chosen working digits.
    fn exp2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

// ── Narrow tier — widen-to-D38 cascade ─────────────────────────────

macro_rules! impl_exp_widen {
    ($T:ident, $exp_strict:path, $exp_with:path) => {
        impl<const SCALE: u32> ExpPolicy for $T<SCALE> {
            #[inline]
            fn exp_impl(self, mode: RoundingMode) -> Self {
                $exp_strict(self, mode)
            }
            #[inline]
            fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                $exp_with(self, working_digits, mode)
            }
            #[inline]
            fn exp2_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_strict_with(mode)).unwrap_or_else(
                    |_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::exp2"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            fn exp2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($T), "::exp2"),
                            SCALE,
                        )
                    })
            }
        }
    };
}

impl_exp_widen!(
    D9,
    exp::widen_to_d38::exp_strict_d9,
    exp::widen_to_d38::exp_with_d9
);
impl_exp_widen!(
    D18,
    exp::widen_to_d38::exp_strict_d18,
    exp::widen_to_d38::exp_with_d18
);

// ── D38 — see `crate::policy::ln` for the borrow-D57 rationale. ────

// D38 — use the in-tree `Fixed`-256 `exp_fixed` directly. The
// borrow_d57 path was retained earlier when D38's bespoke kernel was
// ~2× slower than D57's wide_kernel at matched precision. With the
// 0.4.2 MG-routed `Fixed::mul` / `div_small` / `divmod_u256_by_pow10`
// fast paths the D38-native kernel beats the borrow-and-back round
// trip across the whole SCALE range — measured ~2× faster at
// SCALE 19 (10-12 µs versus 22 µs on the GHA shared-runner pool).
impl<const SCALE: u32> ExpPolicy for D38<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp_with(
            self.0,
            SCALE,
            working_digits,
            mode,
        ))
    }
    #[inline]
    fn exp2_impl(self, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp2_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn exp2_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp2_with(
            self.0,
            SCALE,
            working_digits,
            mode,
        ))
    }
}

// ── Wide tiers — width default: per-tier wide_kernel ────────────────
//
// `exp_impl` / `exp_with_impl` share ONE base arm table per width via the
// `policy_triplet!` free fns (the `_with` form drops `working_digits`,
// matching today's verbatim-copy behaviour). `std` is identical to `base`
// for exp — the only std machinery is the wide-kernel constant cache (a
// later hoist concern, not visible to the policy). `exp2_impl` /
// `exp2_with_impl` keep delegating to the inherent `exp2_strict_with`
// shell (no scale cascade, no raw-storage free-fn equivalent).

// No-band wide widths: a single `(wtag::$T, _)` base arm.
macro_rules! impl_wide_exp {
    ($T:ident, $Storage:ty, $base_fn:ident, $std_fn:ident, $no_std_fn:ident, $kernel:path) => {
        policy_triplet! {
            storage   = $Storage,
            base_fn   = $base_fn,
            std_fn    = $std_fn,
            no_std_fn = $no_std_fn,
            recv      = raw,
            mode      = mode,
            params    = {},
            base      = { (wtag::$T, _) => $kernel(raw, mode, SCALE) },
            std       = {},
        }

        impl<const SCALE: u32> ExpPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn exp_impl(self, mode: RoundingMode) -> Self {
                #[cfg(feature = "std")]
                {
                    Self($std_fn::<{ wtag::$T }, SCALE>(self.0, mode))
                }
                #[cfg(not(feature = "std"))]
                {
                    Self($no_std_fn::<{ wtag::$T }, SCALE>(self.0, mode))
                }
            }
            #[inline]
            fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                #[cfg(feature = "std")]
                {
                    Self($std_fn::<{ wtag::$T }, SCALE>(self.0, mode))
                }
                #[cfg(not(feature = "std"))]
                {
                    Self($no_std_fn::<{ wtag::$T }, SCALE>(self.0, mode))
                }
            }
            #[inline]
            fn exp2_impl(self, mode: RoundingMode) -> Self {
                self.exp2_strict_with(mode)
            }
            #[inline]
            fn exp2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
                self.exp2_strict_with(mode)
            }
        }
    };
}

// D57 — bespoke arms so `exp` can divert SCALE in 18..=22 and 45..=56
// through their lookup tables before the generic `wide_kernel`.
#[cfg(any(feature = "d57", feature = "wide"))]
policy_triplet! {
    storage   = crate::wide_int::Int192,
    base_fn   = exp_d57_base,
    std_fn    = exp_d57_std,
    no_std_fn = exp_d57_no_std,
    recv      = raw,
    mode      = mode,
    params    = {},
    base      = {
        (wtag::D57, 18..=22) => exp::lookup_d57_s18_22_tang::exp_strict::<SCALE>(raw, mode),
        (wtag::D57, 45..=56) => exp::lookup_d57_s45_56::exp_strict::<SCALE>(raw, mode),
        (wtag::D57, _)       => exp::wide_kernel::exp_strict_d57(raw, mode, SCALE)
    },
    std       = {},
}

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::types::widths::D57<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(exp_d57_std::<{ wtag::D57 }, SCALE>(self.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(exp_d57_no_std::<{ wtag::D57 }, SCALE>(self.0, mode))
        }
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(exp_d57_std::<{ wtag::D57 }, SCALE>(self.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(exp_d57_no_std::<{ wtag::D57 }, SCALE>(self.0, mode))
        }
    }
    #[inline]
    fn exp2_impl(self, mode: RoundingMode) -> Self {
        self.exp2_strict_with(mode)
    }
    #[inline]
    fn exp2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.exp2_strict_with(mode)
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl_wide_exp!(
    D76,
    crate::wide_int::Int256,
    exp_d76_base,
    exp_d76_std,
    exp_d76_no_std,
    exp::wide_kernel::exp_strict_d76
);

// D115 — bespoke arm so `exp` can divert SCALE in 50..=60 through the
// Tang-style narrow-GUARD lookup before falling back to `wide_kernel`.
#[cfg(any(feature = "d115", feature = "wide"))]
policy_triplet! {
    storage   = crate::wide_int::Int384,
    base_fn   = exp_d115_base,
    std_fn    = exp_d115_std,
    no_std_fn = exp_d115_no_std,
    recv      = raw,
    mode      = mode,
    params    = {},
    base      = {
        (wtag::D115, 50..=60) => exp::lookup_d115_s57_tang::exp_strict::<SCALE>(raw, mode),
        (wtag::D115, _)       => exp::wide_kernel::exp_strict_d115(raw, mode, SCALE)
    },
    std       = {},
}

#[cfg(any(feature = "d115", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::types::widths::D115<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(exp_d115_std::<{ wtag::D115 }, SCALE>(self.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(exp_d115_no_std::<{ wtag::D115 }, SCALE>(self.0, mode))
        }
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(exp_d115_std::<{ wtag::D115 }, SCALE>(self.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(exp_d115_no_std::<{ wtag::D115 }, SCALE>(self.0, mode))
        }
    }
    #[inline]
    fn exp2_impl(self, mode: RoundingMode) -> Self {
        self.exp2_strict_with(mode)
    }
    #[inline]
    fn exp2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.exp2_strict_with(mode)
    }
}

// D153 — bespoke arm so `exp` can divert SCALE in 70..=82 through the
// Tang-style narrow-GUARD lookup. See
// [`crate::algos::exp::lookup_d153_s70_82_tang`].
#[cfg(any(feature = "d153", feature = "wide"))]
policy_triplet! {
    storage   = crate::wide_int::Int512,
    base_fn   = exp_d153_base,
    std_fn    = exp_d153_std,
    no_std_fn = exp_d153_no_std,
    recv      = raw,
    mode      = mode,
    params    = {},
    base      = {
        (wtag::D153, 70..=82) => exp::lookup_d153_s70_82_tang::exp_strict::<SCALE>(raw, mode),
        (wtag::D153, _)       => exp::wide_kernel::exp_strict_d153(raw, mode, SCALE)
    },
    std       = {},
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::types::widths::D153<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(exp_d153_std::<{ wtag::D153 }, SCALE>(self.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(exp_d153_no_std::<{ wtag::D153 }, SCALE>(self.0, mode))
        }
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(exp_d153_std::<{ wtag::D153 }, SCALE>(self.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(exp_d153_no_std::<{ wtag::D153 }, SCALE>(self.0, mode))
        }
    }
    #[inline]
    fn exp2_impl(self, mode: RoundingMode) -> Self {
        self.exp2_strict_with(mode)
    }
    #[inline]
    fn exp2_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        self.exp2_strict_with(mode)
    }
}

#[cfg(any(feature = "d230", feature = "wide"))]
impl_wide_exp!(
    D230,
    crate::wide_int::Int768,
    exp_d230_base,
    exp_d230_std,
    exp_d230_no_std,
    exp::wide_kernel::exp_strict_d230
);

// D307 — Tang exp surface dispatch was bench-trialed at SCALE 150 and
// showed a ~5% regression on `exp(2)` against the canonical
// `wide_kernel::exp_strict_d307`. D307's Int1024 working integer is
// approaching the Tang-exp crossover identified at D462/D616
// (Int3072+ where adaptive Smith r/2^n in `exp_fixed` matches the
// Tang table-multiply cost). Surface `exp_impl` therefore keeps the
// generic `wide_kernel`. The `tang_exp_fixed` machinery is still
// retained for the hyperbolic kernels at SCALE 140..=160, where the
// shared lift + narrow-GUARD pattern wins despite the wash on exp
// itself.
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl_wide_exp!(
    D307,
    crate::wide_int::Int1024,
    exp_d307_base,
    exp_d307_std,
    exp_d307_no_std,
    exp::wide_kernel::exp_strict_d307
);

// D462 — Tang exp probed at SCALE 225..=235 and LOST (~75% regression
// against the canonical `wide_kernel::exp_strict_d462`). At Int3072
// working width the Tang post-reduction Taylor needs ~95 wide mults
// (one per term) vs. the canonical Smith r/2^n path's ~28 wide
// squarings — the table-elimination of the reduction does not pay for
// the longer Taylor at this depth. See
// `src/algos/exp/lookup_d462_s225_235_tang.rs` for the kernel left in
// place behind `cfg(test)` so the algorithm-lab probe can re-run.
// Dispatch keeps the canonical wide_kernel emission.
#[cfg(any(feature = "d462", feature = "x-wide"))]
impl_wide_exp!(
    D462,
    crate::wide_int::Int1536,
    exp_d462_base,
    exp_d462_std,
    exp_d462_no_std,
    exp::wide_kernel::exp_strict_d462
);

// D616 — width default. The Tang-lookup exp at SCALE 300..=315 was
// bench-trialled and rejected: at D616's wide working integer the
// wide_kernel `exp_fixed` (with the adaptive Smith r/2^n already
// applied) runs in ~230 µs, while the table-multiply Tang lookup runs
// at ~250 µs — i.e. break-even at best. The Tang table multiply on a
// 1024-byte working integer is the same cost class as the Smith
// squaring tail the lookup is meant to elide, so no win materialises at
// this depth. The lookup module stays in tree (it ships the
// `tang_exp_fixed` helper the hyperbolic kernels need) but is NOT wired
// in policy.
#[cfg(any(feature = "d616", feature = "x-wide"))]
impl_wide_exp!(
    D616,
    crate::wide_int::Int2048,
    exp_d616_base,
    exp_d616_std,
    exp_d616_no_std,
    exp::wide_kernel::exp_strict_d616
);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl_wide_exp!(
    D924,
    crate::wide_int::Int3072,
    exp_d924_base,
    exp_d924_std,
    exp_d924_no_std,
    exp::wide_kernel::exp_strict_d924
);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl_wide_exp!(
    D1232,
    crate::wide_int::Int4096,
    exp_d1232_base,
    exp_d1232_std,
    exp_d1232_no_std,
    exp::wide_kernel::exp_strict_d1232
);
