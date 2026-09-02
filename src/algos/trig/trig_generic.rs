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
//! (`pi_by_scale` on the hot `working_scale == SCALE + GUARD` path,
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

/// Taylor series for `sin` on a reduced argument `r ∈ [0, π/4]`, at
/// `working_scale`.
///
/// `sin(r) = r − r³/3! + r⁵/5! − …`
fn sin_taylor<S: BigInt>(reduced_arg: S, working_scale: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    let reduced_arg_squared = eg::mul::<S>(reduced_arg, reduced_arg, working_scale);
    let mut sum = reduced_arg;
    let mut term = reduced_arg;
    let mut term_index: u128 = 1;
    loop {
        term = eg::mul::<S>(term, reduced_arg_squared, working_scale)
            / eg::lit::<S>(((2 * term_index) * (2 * term_index + 1)) as i128);
        if term == eg::zero::<S>() {
            break;
        }
        if term_index % 2 == 1 {
            sum = sum - term;
        } else {
            sum = sum + term;
        }
        term_index += 1;
        if term_index > eg::SERIES_CAP {
            break;
        }
    }
    sum
}

/// Taylor series for `cos` on a reduced argument `r ∈ [0, π/4]`, at
/// `working_scale`.
///
/// `cos(r) = 1 − r²/2! + r⁴/4! − r⁶/6! + …`
///
/// Converges faster than [`sin_taylor`] at the same `r` because the
/// leading `1` dominates the small even-power corrections — used as the
/// "upper-half" branch of [`sin_fixed`] when the reduced argument
/// exceeds π/4.
fn cos_taylor<S: BigInt>(reduced_arg: S, working_scale: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    let reduced_arg_squared = eg::mul::<S>(reduced_arg, reduced_arg, working_scale);
    let one_w = eg::one::<S>(working_scale);
    let mut sum = one_w;
    let mut term = one_w;
    let mut term_index: u128 = 1;
    loop {
        term = eg::mul::<S>(term, reduced_arg_squared, working_scale)
            / eg::lit::<S>(((2 * term_index - 1) * (2 * term_index)) as i128);
        if term == eg::zero::<S>() {
            break;
        }
        if term_index % 2 == 1 {
            sum = sum - term;
        } else {
            sum = sum + term;
        }
        term_index += 1;
        if term_index > eg::SERIES_CAP {
            break;
        }
    }
    sum
}

/// Sine of a `working_value` (`= x · 10^working_scale`) at
/// `working_scale`, with `π` supplied at the same scale
/// (`pi_at_working_scale = π · 10^working_scale`).
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
/// `τ = 2π·10^working_scale` is correctly rounded (error ≤ 1 working
/// unit), so the reduced residue `r = working_value − tau_multiple·τ`
/// carries an absolute error of up to `tau_multiple ≈ |x|/2π` working
/// units — the mod-τ cancellation eats one guard
/// digit per integer digit of `|x|`. A caller choosing the work width /
/// guard must budget `digits(|x|)` on top of the precision it needs
/// (the work-rung selector's `D_BUDGET` axis; see
/// `policy::work_rung::trig_rung`). `tau_multiple` must also fit `i128`
/// ([`eg::round_to_nearest_int`] truncates past it) — a bound inherited
/// from the per-tier cores, not introduced here.
pub(crate) fn sin_fixed<S: BigInt>(
    working_value: S,
    working_scale: u32,
    pi_at_working_scale: S,
) -> S
where
    S::Scratch: ComputeLimbs,
{
    let tau = pi_at_working_scale + pi_at_working_scale;
    let half_pi = pi_at_working_scale >> 1;
    let quarter_pi = half_pi >> 1; // π/4
    let tau_multiple = eg::round_to_nearest_int::<S>(
        eg::div::<S>(working_value, tau, working_scale),
        working_scale,
    );
    let residue = working_value - eg::scale_by_k::<S>(tau, tau_multiple);
    let sin_neg = residue < eg::zero::<S>();
    let abs_residue = if sin_neg { -residue } else { residue };
    let reduced = if abs_residue >= half_pi {
        pi_at_working_scale - abs_residue
    } else {
        abs_residue
    };
    let sin_abs = if reduced > quarter_pi {
        // sin(reduced) = cos(π/2 − reduced); the cos argument lies in
        // [0, π/4].
        cos_taylor::<S>(half_pi - reduced, working_scale)
    } else {
        sin_taylor::<S>(reduced, working_scale)
    };
    if sin_neg { -sin_abs } else { sin_abs }
}

/// Cosine of a working-scale value via the cofunction identity
/// `cos(x) = sin(π/2 − x)` — one [`sin_fixed`] evaluation, no sqrt.
pub(crate) fn cos_fixed<S: BigInt>(
    working_value: S,
    working_scale: u32,
    pi_at_working_scale: S,
) -> S
where
    S::Scratch: ComputeLimbs,
{
    sin_fixed::<S>(
        (pi_at_working_scale >> 1) - working_value,
        working_scale,
        pi_at_working_scale,
    )
}

/// Taylor series for `atan` on a reduced argument `|x| < 1`, at
/// `working_scale`.
///
/// `atan(x) = x − x³/3 + x⁵/5 − …`
fn atan_taylor<S: BigInt>(reduced_arg: S, working_scale: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    let reduced_arg_squared = eg::mul::<S>(reduced_arg, reduced_arg, working_scale);
    let mut sum = reduced_arg;
    let mut term = reduced_arg;
    let mut term_index: u128 = 1;
    loop {
        term = eg::mul::<S>(term, reduced_arg_squared, working_scale);
        let contrib = term / eg::lit::<S>((2 * term_index + 1) as i128);
        if contrib == eg::zero::<S>() {
            break;
        }
        if term_index % 2 == 1 {
            sum = sum - contrib;
        } else {
            sum = sum + contrib;
        }
        term_index += 1;
        if term_index > eg::SERIES_CAP {
            break;
        }
    }
    sum
}

/// Arctangent of a `working_value` (`= x · 10^working_scale`) at
/// `working_scale`, with `π` supplied at the same scale
/// (`pi_at_working_scale = π · 10^working_scale`) — only the
/// `π/2` complement of the `|x| > 1` reciprocal fold consumes it.
/// Result in `(−π/2, π/2)`.
///
/// Reciprocal fold (`atan(x) = π/2 − atan(1/x)` for `x > 1`), then
/// argument halvings `atan(x) = 2·atan(x/(1+√(1+x²)))` — taken only
/// while the argument is at or above ~0.2, capped by the working-scale
/// count from the per-tier break-even analysis — then the
/// [`atan_taylor`] series on the reduced argument.
///
/// ## Argument-magnitude validity
///
/// Unlike [`sin_fixed`]'s mod-τ reduction, the fold loses NO precision
/// proportional to `digits(|x|)` — the reciprocal's relative error stays
/// one working ULP, so there is no per-integer-digit guard cost. The
/// only `|x|` axis is REPRESENTATION: the lifted `working_value` (and the
/// fold's `10^(2·working_scale)` divide numerator) must fit `S` — a
/// caller choosing a narrow work width must bound `digits(|x|)` so the
/// lift fits (the work-rung selector's gate; see `policy::work_rung`).
pub(crate) fn atan_fixed<S: BigInt>(
    working_value: S,
    working_scale: u32,
    pi_at_working_scale: S,
) -> S
where
    S::Scratch: ComputeLimbs,
{
    let one_w = eg::one::<S>(working_scale);
    let sign_neg = working_value < eg::zero::<S>();
    let mut x = if sign_neg { -working_value } else { working_value };
    let mut add_half_pi = false;
    if x > one_w {
        x = eg::div::<S>(one_w, x, working_scale);
        add_half_pi = true;
    }
    // Argument halvings: atan(x) = 2·atan(x/(1+√(1+x²))).
    //
    // ADAPTIVE on the argument, mirroring the narrow 2-limb kernel
    // (`trig::trig_series_2limb::atan_fixed`): halve only while the
    // argument is at or above ~0.2 — the band where [`atan_taylor`]
    // below still needs the reduction — and stop as soon as it is
    // inside. The scale-keyed count is retained UNCHANGED as the cap,
    // so no argument takes more halvings than it did before; a small
    // one now takes fewer.
    //
    // Each halving costs a wide sqrt + a wide divide + a wide multiply
    // at the tier's work width, so running the whole chain on an
    // argument already inside the convergence band is pure waste. The
    // narrow kernel has always been adaptive here and the wide one was
    // not, which made the wide path pay 5–6 unnecessary wide sqrts for
    // any small argument at every tier and scale. This adopts the
    // narrow kernel's existing rule rather than inventing a new one.
    //
    // Fewer halvings also REDUCES error: the result is reassembled with
    // `<< halvings`, so a shorter chain scales up less accumulated
    // error. The threshold costs one divide per call, against the 5–6
    // sqrt/divide/multiply triples it can skip.
    let halving_cap: u32 = if working_scale < 60 {
        5
    } else if working_scale < 110 {
        6
    } else {
        7
    };
    let pow10_w = one_w;
    // 0.2 at the working scale — the narrow kernel's threshold.
    let halving_threshold = one_w / eg::lit::<S>(5);
    let mut halvings: u32 = 0;
    while x >= halving_threshold && halvings < halving_cap {
        let x_squared = eg::mul::<S>(x, x, working_scale);
        let denom = one_w + eg::sqrt_fixed::<S>(one_w + x_squared, working_scale);
        x = eg::div_cached::<S>(x, denom, pow10_w);
        halvings += 1;
    }
    let mut result = atan_taylor::<S>(x, working_scale) << halvings;
    if add_half_pi {
        result = (pi_at_working_scale >> 1) - result;
    }
    if sign_neg { -result } else { result }
}

/// Joint sine + cosine of a working-scale value at `working_scale`.
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
pub(crate) fn sin_cos_fixed<S: BigInt>(
    working_value: S,
    working_scale: u32,
    pi_at_working_scale: S,
) -> (S, S)
where
    S::Scratch: ComputeLimbs,
{
    let tau = pi_at_working_scale + pi_at_working_scale;
    let half_pi = pi_at_working_scale >> 1;
    let quarter_pi = half_pi >> 1;
    let tau_multiple = eg::round_to_nearest_int::<S>(
        eg::div::<S>(working_value, tau, working_scale),
        working_scale,
    );
    let residue = working_value - eg::scale_by_k::<S>(tau, tau_multiple);
    let sin_neg = residue < eg::zero::<S>();
    let abs_residue = if sin_neg { -residue } else { residue };
    let cos_neg = abs_residue > half_pi; // |r| > π/2 → cos negative.
    let reduced = if cos_neg {
        pi_at_working_scale - abs_residue
    } else {
        abs_residue
    };
    let sin_abs = if reduced > quarter_pi {
        cos_taylor::<S>(half_pi - reduced, working_scale)
    } else {
        sin_taylor::<S>(reduced, working_scale)
    };
    // cos² + sin² = 1 ⇒ |cos| = √(1 − sin²).
    let one_w = eg::one::<S>(working_scale);
    let sin_abs_squared = eg::mul::<S>(sin_abs, sin_abs, working_scale);
    let cos_abs = eg::sqrt_fixed::<S>(one_w - sin_abs_squared, working_scale);
    let sin_result = if sin_neg { -sin_abs } else { sin_abs };
    let cos_result = if cos_neg { -cos_abs } else { cos_abs };
    (sin_result, cos_result)
}
