// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log1p` algorithm family — `log1p(t) = ln(1 + t)`, domain `t > -1`.
//!
//! Two generic kernels, one per argument regime; there is no per-tier
//! variant and no narrow/wide split. `crate::policy::log1p` chooses
//! between them by value and supplies each width's work integer and
//! guard.
//!
//! Variants:
//!
//! - [`log1p_artanh`] — the Goldberg/Higham reformulation
//!   `log1p(t) = 2·artanh(u)`, `u = t / (2 + t)`, reaching the result
//!   without ever forming `1 + t`. Wraps the width-generic working-scale
//!   kernel [`crate::algos::exp::exp_generic::log1p_fixed`] already
//!   shipped for `acosh` / `atanh`. The series has NO range reduction,
//!   so it is the small-`|t|` kernel: its ratio is `u²`, which stops
//!   converging usefully as `|u| → 1` (`t → -1` or `t → ∞`).
//! - [`log1p_with_ln`] — the composition for everything else: form
//!   `1 + t` at the working scale (EXACT in fixed point) and run the
//!   width-generic `ln` kernel, which carries its own sqrt range
//!   reduction.
//!
//! # Why two kernels, and why `log1p` is not more accurate than `ln`
//!
//! In binary floating point `log1p` exists because `1 + t` loses every
//! significant digit of a tiny `t`. In this crate's FIXED-POINT
//! representation `1 + t` is exactly representable at the working scale,
//! so that cancellation cannot occur and `log1p` carries **no accuracy
//! advantage** over `ln(1 + t)`. The function is provided for API parity
//! and standards conformance (C `log1p`, IEEE 754-2019 `logp1`).
//!
//! The split is therefore about **cost and convergence**, not accuracy:
//! for a tiny `t` the artanh series terminates in a couple of terms
//! while `ln` would burn its whole sqrt-reduction chain, and near the
//! domain edge the artanh series does not converge at all while `ln`
//! is uniformly good.
//!
//! The same split is already in the tree: the wide `acosh_strict` uses
//! `log1p_fixed` near 1 and the `ln` kernel away from it.

pub(crate) mod log1p_artanh;
pub(crate) mod log1p_with_ln;

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Directed-rounding post-adjust for the sub-resolution band near `t = 0`
/// — the `log1p` face of
/// [`wide_trig_core::adjust_log_near_zero`], which carries the full
/// analysis. `log1p`'s linear term IS its own argument, so the gap `δ`
/// is `raw` and no subtraction is needed; `one` is supplied only because
/// the parabola term is `Q = raw²/(2·10^SCALE)`.
///
/// `log1p` shares BOTH of that adjust's grid points, not just the first.
/// The tangent case (`result == raw`) is the original one: `log1p(t) < t`
/// strictly, so a downward-directed result that landed on `t` is above
/// the true value. The parabola case is the same `δ² ≡ 0 (mod 2·10^SCALE)`
/// family `ln` hits near `x = 1` — read at `t` rather than at `x − 1`, it
/// is the identical condition, the quadratic is an exact whole number of
/// ULPs, the value steps to a DIFFERENT grid point, and the tangent test
/// no-ops there. So `Ceiling` for `t > 0` was wrong in exactly the way
/// `ln`'s was, and is corrected by the same bracket.
///
/// [`wide_trig_core::adjust_log_near_zero`]: crate::algos::support::wide_trig_core::adjust_log_near_zero
#[inline]
pub(crate) fn adjust_near_zero<St: BigInt, S: BigInt, const SCALE: u32>(
    result: St,
    raw: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if crate::support::rounding::is_nearest_mode(mode) {
        return result;
    }
    let one = crate::consts::pow10::dispatch::<St>(SCALE);
    crate::algos::support::wide_trig_core::adjust_log_near_zero::<St, S>(result, raw, one, mode)
}

/// Panics unless `t > -1`, i.e. unless the raw storage value exceeds
/// `-10^SCALE`. The family-level precondition, shared by both kernels.
///
/// Mirrors the `ln` kernels' `argument must be positive` guard:
/// `log1p`'s domain violation is precisely `1 + t <= 0`, which is `ln`'s
/// own out-of-domain condition on the value it is asked for.
///
/// `10^scale` is representable in every tier at every admissible scale
/// (`MAX_SCALE` is one below the tier's digit capacity), so building the
/// bound cannot itself overflow.
#[inline]
pub(crate) fn guard_domain<St: BigInt>(raw: St, scale: u32) {
    if raw <= -crate::consts::pow10::dispatch::<St>(scale) {
        panic!("log1p: argument must be greater than -1");
    }
}

/// The two `log1p` tests that cannot live in `decimal-scale-test`: each needs a
/// crate-private item with no public equivalent, so moving them would have meant
/// widening visibility or weakening the assertion. Everything else from the old
/// `tests.rs` is now `decimal-scale-test/tests/api/log1p.rs`.
#[cfg(test)]
mod crate_internal_tests {
    use super::log1p_artanh::log1p_artanh_g;
    use super::log1p_with_ln::log1p_with_ln_g;
    use crate::algos::support::narrow_ziv::WZiv;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;
    use crate::D;

    const MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// `1.0` at scale 20.
    const UNIT: i128 = 10_i128.pow(20);

    fn d38s20(raw: i128) -> D<Int<2>, 20> {
        D::<Int<2>, 20>(Int::<2>::from_i128(raw))
    }

    /// The two kernels must agree wherever both are valid — the region wall
    /// moves cost, never the value. Checked on the overlap band (both are
    /// correct for `|t| ≤ 1/2`) by driving each kernel directly at the
    /// narrow work integer.
    ///
    /// STAYS CRATE-INTERNAL: `log1p_artanh_g`, `log1p_with_ln_g` and `WZiv` are
    /// all `pub(crate)`. Driving the two kernels against each other is the whole
    /// point, and the public surface exposes only whichever one the matcher
    /// picks — so through it this test could not tell the two apart.
    #[test]
    fn both_kernels_agree_inside_the_overlap_band() {
        const GUARD: u32 = 30;
        const TS: [i128; 9] = [
            0,
            1,
            -1,
            UNIT / 1_000,
            -UNIT / 1_000,
            UNIT / 3,
            -UNIT / 3,
            UNIT / 2,
            -UNIT / 2,
        ];
        for &t in &TS {
            for &mode in &MODES {
                let v = Int::<2>::from_i128(t);
                assert_eq!(
                    log1p_artanh_g::<Int<2>, WZiv, 20>(
                        v,
                        GUARD,
                        Int::<2>::MAX,
                        Int::<2>::MIN,
                        mode
                    ),
                    log1p_with_ln_g::<Int<2>, WZiv, 20>(
                        v,
                        GUARD,
                        Int::<2>::MAX,
                        Int::<2>::MIN,
                        mode
                    ),
                    "artanh != with_ln at t_raw={t} mode={mode:?}"
                );
            }
        }
    }

    /// Every `*_with(mode)` has a default-mode sibling that agrees with it.
    ///
    /// STAYS CRATE-INTERNAL: `support::rounding::DEFAULT_ROUNDING_MODE` is not
    /// exported. The default is feature-selected, so naming a mode literally
    /// here would assert against a guess rather than against the constant the
    /// no-mode entry points actually use.
    #[test]
    fn log1p_default_mode_siblings_agree() {
        let t = UNIT / 2;
        assert_eq!(
            d38s20(t).log1p_approx(45).to_bits().as_i128(),
            d38s20(t)
                .log1p_approx_with(45, crate::support::rounding::DEFAULT_ROUNDING_MODE)
                .to_bits()
                .as_i128(),
            "log1p_approx != log1p_approx_with(default mode)"
        );
        assert_eq!(
            d38s20(t).log1p_strict().to_bits().as_i128(),
            d38s20(t)
                .log1p_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
                .to_bits()
                .as_i128(),
            "log1p_strict != log1p_strict_with(default mode)"
        );
    }
}
