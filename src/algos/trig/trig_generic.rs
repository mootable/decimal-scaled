// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Width-generic forward-trig fixed-point kernels (`sin` / `cos` /
//! `sin_cos` on a working-scale integer).
//!
//! The per-tier `decl_wide_transcendental!` cores bound these bodies to
//! one tier work integer `W`; this module lifts them to free functions
//! generic over any [`BigInt`] work integer `S` — the same hoist
//! `exp_generic` performs for `exp_fixed` / `ln_fixed` (Constitution
//! rule 2: one generic algorithm, no per-tier copies). The per-tier
//! macro `sin_fixed` / `cos_fixed` / `sin_cos_fixed` are now thin
//! forwarders threading the tier's const-folded `π` (`pi_cf::<SCALE>`),
//! and the SCALE-derived work-rung shells
//! (`wide_trig_core::{sin,cos,tan}_series_g`) run the same bodies at a
//! narrower rung integer `Wk`.
//!
//! `π` is supplied by the caller as a working-scale value rather than
//! computed here, so the caller owns the const-fold seam
//! (`pi_by_scale` on the hot `w == SCALE + GUARD` path,
//! `pi_by_working_scale` on the Ziv escalation path) — exactly the
//! `ln2` parameter shape of [`exp_generic::ln_fixed`].
//!
//! All arithmetic leaves (`mul`, `div`, `scale_by_k`,
//! `round_to_nearest_int`, `sqrt_fixed`) are the shared
//! [`exp_generic`](crate::algos::exp::exp_generic) fixed-point
//! primitives — the identical integer operations the per-tier cores
//! forward to, so a value computed here is bit-identical to the
//! per-tier core's at any width that holds it without overflow.

use crate::algos::exp::exp_generic as eg;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;

/// Taylor series for `sin` on a reduced `r ∈ [0, π/4]`, at scale `w`.
///
/// `sin(r) = r − r³/3! + r⁵/5! − …`
fn sin_taylor<S: BigInt>(r: S, w: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    let r2 = eg::mul::<S>(r, r, w);
    let mut sum = r;
    let mut term = r;
    let mut k: u128 = 1;
    loop {
        term = eg::mul::<S>(term, r2, w) / eg::lit::<S>(((2 * k) * (2 * k + 1)) as i128);
        if term == eg::zero::<S>() {
            break;
        }
        if k % 2 == 1 {
            sum = sum - term;
        } else {
            sum = sum + term;
        }
        k += 1;
        if k > eg::SERIES_CAP {
            break;
        }
    }
    sum
}

/// Taylor series for `cos` on a reduced `r ∈ [0, π/4]`, at scale `w`.
///
/// `cos(r) = 1 − r²/2! + r⁴/4! − r⁶/6! + …`
///
/// Converges faster than [`sin_taylor`] at the same `r` because the
/// leading `1` dominates the small even-power corrections — used as the
/// "upper-half" branch of [`sin_fixed`] when the reduced argument
/// exceeds π/4.
fn cos_taylor<S: BigInt>(r: S, w: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    let r2 = eg::mul::<S>(r, r, w);
    let one_w = eg::one::<S>(w);
    let mut sum = one_w;
    let mut term = one_w;
    let mut k: u128 = 1;
    loop {
        term = eg::mul::<S>(term, r2, w) / eg::lit::<S>(((2 * k - 1) * (2 * k)) as i128);
        if term == eg::zero::<S>() {
            break;
        }
        if k % 2 == 1 {
            sum = sum - term;
        } else {
            sum = sum + term;
        }
        k += 1;
        if k > eg::SERIES_CAP {
            break;
        }
    }
    sum
}

/// Sine of a working-scale value `v_w` (`= x · 10^w`) at scale `w`,
/// with `π` supplied at the same scale (`pi_w = π · 10^w`).
///
/// Reduces to `|r| ≤ π/2` via mod-τ; then folds to `r ∈ [0, π/2]` via
/// `sin(π − x) = sin(x)`; then routes to `sin_taylor` if `r ≤ π/4` or
/// `cos_taylor(π/2 − r)` otherwise. The `[0, π/4]` window halves the
/// convergence argument and roughly halves the Taylor term count, and
/// cos converges faster than sin at the same argument because of the
/// constant-1 leading term.
///
/// ## Argument-magnitude validity (the reduction error)
///
/// `τ = 2π·10^w` is correctly rounded (error ≤ 1 working unit), so the
/// reduced residue `r = v_w − q·τ` carries an absolute error of up to
/// `q ≈ |x|/2π` working units — the mod-τ cancellation eats one guard
/// digit per integer digit of `|x|`. A caller choosing the work width /
/// guard must budget `digits(|x|)` on top of the precision it needs
/// (the work-rung selector's `D_BUDGET` axis; see
/// `policy::work_rung::trig_rung`). `q` must also fit `i128`
/// ([`eg::round_to_nearest_int`] truncates past it) — a bound inherited
/// from the per-tier cores, not introduced here.
pub(crate) fn sin_fixed<S: BigInt>(v_w: S, w: u32, pi_w: S) -> S
where
    S::Scratch: ComputeLimbs,
{
    let tau = pi_w + pi_w;
    let hp = pi_w >> 1;
    let qp = hp >> 1; // π/4
    let q = eg::round_to_nearest_int::<S>(eg::div::<S>(v_w, tau, w), w);
    let r = v_w - eg::scale_by_k::<S>(tau, q);
    let neg = r < eg::zero::<S>();
    let abs_r = if neg { -r } else { r };
    let reduced = if abs_r >= hp { pi_w - abs_r } else { abs_r };
    let s = if reduced > qp {
        // sin(reduced) = cos(π/2 − reduced); the cos argument lies in
        // [0, π/4].
        cos_taylor::<S>(hp - reduced, w)
    } else {
        sin_taylor::<S>(reduced, w)
    };
    if neg { -s } else { s }
}

/// Cosine of a working-scale value via the cofunction identity
/// `cos(x) = sin(π/2 − x)` — one [`sin_fixed`] evaluation, no sqrt.
pub(crate) fn cos_fixed<S: BigInt>(v_w: S, w: u32, pi_w: S) -> S
where
    S::Scratch: ComputeLimbs,
{
    sin_fixed::<S>((pi_w >> 1) - v_w, w, pi_w)
}

/// Joint sine + cosine of a working-scale value at scale `w`.
///
/// One Taylor series + one wide sqrt + one wide mul, vs two independent
/// Taylor evaluations:
///
/// - Reduce mod τ and fold to `|r| ∈ [0, π/2]`, tracking both signs
///   (sin from the mod-τ residue, cos from whether the unfolded `|r|`
///   exceeded `π/2`).
/// - Evaluate `|sin(reduced)|` via the same `sin_taylor` / `cos_taylor`
///   branch as [`sin_fixed`].
/// - Recover `|cos(reduced)|` from the Pythagorean identity
///   `√(1 − sin²)`.
/// - Apply the cached signs.
pub(crate) fn sin_cos_fixed<S: BigInt>(v_w: S, w: u32, pi_w: S) -> (S, S)
where
    S::Scratch: ComputeLimbs,
{
    let tau = pi_w + pi_w;
    let hp = pi_w >> 1;
    let qp = hp >> 1;
    let q = eg::round_to_nearest_int::<S>(eg::div::<S>(v_w, tau, w), w);
    let r = v_w - eg::scale_by_k::<S>(tau, q);
    let sin_neg = r < eg::zero::<S>();
    let abs_r = if sin_neg { -r } else { r };
    let cos_neg = abs_r > hp; // |r| > π/2 → cos negative.
    let reduced = if cos_neg { pi_w - abs_r } else { abs_r };
    let s_abs = if reduced > qp {
        cos_taylor::<S>(hp - reduced, w)
    } else {
        sin_taylor::<S>(reduced, w)
    };
    // cos² + sin² = 1 ⇒ |cos| = √(1 − sin²).
    let one_w = eg::one::<S>(w);
    let s2 = eg::mul::<S>(s_abs, s_abs, w);
    let cos_abs = eg::sqrt_fixed::<S>(one_w - s2, w);
    let sin_result = if sin_neg { -s_abs } else { s_abs };
    let cos_result = if cos_neg { -cos_abs } else { cos_abs };
    (sin_result, cos_result)
}
