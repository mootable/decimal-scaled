//! Floating-point power policy — narrow tier only (same scope
//! rationale as [`crate::policy::ln`] / [`crate::policy::exp`]).

use crate::algos::pow;
use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

pub(crate) trait PowPolicy: Sized {
    /// `self^exp` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self;

    /// `self^exp` with caller-chosen working digits.
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self;
}

impl<const SCALE: u32> PowPolicy for D9<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_strict_d9(self, exp, mode)
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_with_d9(self, exp, working_digits, mode)
    }
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

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> PowPolicy for D38<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        if SCALE >= 23 {
            Self(pow::borrow_d57::powf_strict::<SCALE>(self.0, exp.0, mode))
        } else {
            Self(pow::fixed_d38::powf_strict::<SCALE>(self.0, exp.0, mode))
        }
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        if SCALE >= 23 {
            Self(pow::borrow_d57::powf_with::<SCALE>(
                self.0,
                exp.0,
                working_digits,
                mode,
            ))
        } else {
            Self(pow::fixed_d38::powf_with::<SCALE>(
                self.0,
                exp.0,
                working_digits,
                mode,
            ))
        }
    }
}

#[cfg(not(any(feature = "d57", feature = "wide")))]
impl<const SCALE: u32> PowPolicy for D38<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        Self(pow::fixed_d38::powf_strict::<SCALE>(self.0, exp.0, mode))
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(pow::fixed_d38::powf_with::<SCALE>(
            self.0,
            exp.0,
            working_digits,
            mode,
        ))
    }
}
