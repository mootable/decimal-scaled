//! Floating-point power policy — narrow tier only (same scope
//! rationale as [`crate::policy::ln`] / [`crate::policy::exp`]).

use crate::algos::pow;
use crate::policy::triplet::{policy_triplet, wtag};
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

pub(crate) trait PowPolicy: Sized {
    /// `self^exp` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self;

    /// `self^exp` with caller-chosen working digits.
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self;
}

impl<const SCALE: u32> PowPolicy for D18<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_strict_d18(self, exp, mode)
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_with_d18(self, exp, working_digits, mode)
    }
}

// ── D38 — width override (scale-gated) ────────────────────────────
//
// When D57 is available, D38's `powf` at HIGH scales (SCALE >= 23)
// routes through `borrow_d57` — widen base and exponent to D57, call
// D57's inherent `powf_{strict,approx}_with`, narrow back. D57's powf
// composes `exp(y · ln x)` on the same wide-tier `ln_fixed`/`exp_fixed`
// cores the matching ln/exp borrow wrappers use, so D38 picks up those
// speedups in composed form.
//
// At LOW scales (SCALE < 23), `fixed_d38` wins by ~1.5× because the
// widen+narrow overhead dominates when the bespoke Fixed kernel is
// already cheap.
//
// Empirical crossover (2.0^3.0, 5 trials × 5000 iters, 2026-05-18):
//   SCALE 18–22:  fixed wins ~1.5×.
//   SCALE 23, 24: BORROW wins ~4.2–4.4× (large, `fixed_d38` hits a
//                 slow taylor convergence path at working scale 53/54).
//   SCALE 25, 26: borrow wins narrowly (~3–7%).
//   SCALE 27:     near-parity (borrow ~1.3% slower, noise).
//   SCALE 28, 29: BORROW wins ~5× (same slow-path spike at w=58/59).
//   SCALE 30+:    borrow at-parity or slightly winning.
//
// Threshold pick (23): captures the SCALE=23/24 ~4× wins. Mild trade-off
// on `powf_with(working_digits ≈ 10)` at scales 25, 26, 27, 30 where
// fixed wins by 10–31%; accepted because the strict-path 4× wins are
// the dominant signal across the matrix.
//
// The hand-tuned `fixed_d38` kernel is retained both as the
// low-scale path AND as the D57-disabled fallback.

// D38 — uniformly route through `fixed_d38::powf_*` after the 0.4.2
// MG-routing of `Fixed::mul` / `div_small` / `divmod_u256_by_pow10` /
// `rescale_down`. The previous SCALE-23 split favoured `borrow_d57`
// at high scales because `Fixed::mul` was paying a 256-iteration bit
// loop for the divide-by-pow10(w). With the chained MG kernel the
// bespoke path now wins across the whole SCALE range; the empirical
// crossover that motivated the split is no longer present.
// D38 routes `powf` through the `policy_triplet!` free fns keyed on a
// const-folded `match (W, SCALE)`. There is no scale cascade and no std
// override today — `std` is identical to `base` — but the triplet shape
// keeps D38 uniform with the other families and ready for a future
// override cell. The strict and `_with` forms call different kernels
// (`powf_strict` vs `powf_with`), so each gets its own triplet; the
// `_with` triplet carries `working_digits` as an extra param.
policy_triplet! {
    storage = crate::int::types::Int<2>,
    base_fn = powf_d38_base, std_fn = powf_d38_std, no_std_fn = powf_d38_no_std,
    recv = raw, mode = mode,
    params = { exp_raw: crate::int::types::Int<2> },
    base = { (wtag::D38, _) => pow::fixed_d38::powf_strict::<SCALE>(raw, exp_raw, mode) },
    std = {},
}
policy_triplet! {
    storage = crate::int::types::Int<2>,
    base_fn = powf_with_d38_base, std_fn = powf_with_d38_std, no_std_fn = powf_with_d38_no_std,
    recv = raw, mode = mode,
    params = { exp_raw: crate::int::types::Int<2>, working_digits: u32 },
    base = { (wtag::D38, _) => pow::fixed_d38::powf_with::<SCALE>(raw, exp_raw, working_digits, mode) },
    std = {},
}

impl<const SCALE: u32> PowPolicy for D38<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(powf_d38_std::<{ wtag::D38 }, SCALE>(self.0, exp.0, mode))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(powf_d38_no_std::<{ wtag::D38 }, SCALE>(self.0, exp.0, mode))
        }
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        #[cfg(feature = "std")]
        {
            Self(powf_with_d38_std::<{ wtag::D38 }, SCALE>(
                self.0,
                exp.0,
                working_digits,
                mode,
            ))
        }
        #[cfg(not(feature = "std"))]
        {
            Self(powf_with_d38_no_std::<{ wtag::D38 }, SCALE>(
                self.0,
                exp.0,
                working_digits,
                mode,
            ))
        }
    }
}
