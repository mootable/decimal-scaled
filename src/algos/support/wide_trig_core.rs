// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic surface over the per-tier wide guard-digit cores.
//!
//! Each wide decimal tier (D57 .. D1232) carries a guard-digit
//! transcendental core, emitted by `decl_wide_transcendental!` into a
//! `wide_trig_<tier>` module against a tier-specific work integer `W`
//! (e.g. `Int<16>` for D307) and tier-specific constant tables. The
//! per-family wide-tier kernels (`algos::{exp,ln,trig}::wide_kernel`)
//! historically shipped one thin `*_strict_<tier>` wrapper per tier per
//! function — 60 near-identical 3-to-20-line bodies differing only by
//! the work integer `W`, the storage integer, and the `core` module.
//!
//! [`WideTrigCore`] is the trait that lets those 60 wrappers collapse to
//! six generic `*_series` functions ([`exp_series`], [`ln_series`],
//! [`sin_series`], [`cos_series`], [`tan_series`], [`atan_series`]).
//! `W` and the storage integer cannot be computed from a single const
//! parameter on stable rust (`W = Int<2N>` needs `generic_const_exprs`),
//! so each tier binds them as **associated types** on a per-tier `Core`
//! marker; `select`/routing is unchanged, only the call target moves
//! from the per-tier wrapper to `*_series::<Core, SCALE>`. This is the
//! same work-width-threading pattern the `sqrt` pilot uses, expressed as
//! a trait because the core surface (not just one work width) is shared.
//!
//! # Layering
//!
//! The `*_series` functions are **algorithm functions**: they call only
//! the trait surface (which forwards *down* into the per-tier core's
//! kernels) and the width-free `near_pole_tan` helper. They never call a
//! method on their own decimal type — the type's `*_strict` methods
//! delegate *down* through the policy dispatch to here.
//!
//! # Per-tier impls
//!
//! The trait is implemented once per tier by `decl_wide_transcendental!`
//! (the `impl WideTrigCore for Core` block it emits): each method
//! forwards to the sibling free function the macro already emits in the
//! per-tier core module (`$core::exp_fixed`, `$core::round_to_storage_*`,
//! …). Collapsing those per-tier bodies themselves to one `BigInt`-
//! generic implementation (the `exp_generic` precedent in
//! `crate::macros::wide_transcendental`) is a separate, later step — the
//! trait surface here is what makes that collapse a local change rather
//! than a routing change.

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Tier-generic surface over a wide guard-digit transcendental core.
///
/// Implemented once per wide tier by `decl_wide_transcendental!` on a
/// per-tier zero-sized `Core` marker, binding the tier's work integer
/// [`Self::W`] and storage integer [`Self::Storage`] as associated
/// types. The six `*_series` free functions in this module are generic
/// over `C: WideTrigCore` and drive the whole wide-tier strict
/// transcendental surface through it.
pub(crate) trait WideTrigCore {
    /// The tier's guard-digit work integer: a value `x` is held as the
    /// `W` integer `x · 10^w` at a working scale `w`.
    type W: BigInt + Copy + PartialEq;
    /// The next-wider work integer used by the large-result `exp` path
    /// (`exp_fixed_wide`). Aliases [`Self::W`] on the widest tier. Not
    /// consumed by the `*_series` functions directly; threaded so a
    /// later `BigInt`-generic core lift has the binding available.
    type Wexp: BigInt + Copy + PartialEq;
    /// The tier's storage integer (`x · 10^SCALE`).
    type Storage: BigInt + Copy + PartialEq;

    /// Guard digits added below the storage scale on the strict path.
    const GUARD: u32;

    // ── value zero / small constants in `Storage` and `W` ──────────────

    /// The storage `0`.
    fn storage_zero() -> Self::Storage;
    /// The storage representation of `1` at scale `SCALE` (`10^SCALE`).
    fn storage_one(scale: u32) -> Self::Storage;
    /// The work-integer `0`.
    fn zero() -> Self::W;

    // ── working-scale lift / narrow ────────────────────────────────────

    /// Builds a working-scale `W` from raw storage, scaling by
    /// `10^working_digits` (raw is `value · 10^SCALE`).
    fn to_work_w(raw: Self::Storage, working_digits: u32) -> Self::W;
    /// Builds a working-scale `W` from raw storage at the const `GUARD`.
    fn to_work(raw: Self::Storage) -> Self::W;
    /// Rounds a working-scale `W` value at scale `w` to scale `target`
    /// under `mode` and narrows to storage.
    fn round_to_storage_with(
        v: Self::W,
        w: u32,
        target: u32,
        mode: RoundingMode,
    ) -> Self::Storage;
    /// Directed-rounding narrowing with Ziv escalation. `recompute(g)`
    /// returns the kernel value computed with `g` guard digits.
    fn round_to_storage_directed(
        base_guard: u32,
        target: u32,
        mode: RoundingMode,
        recompute: &mut dyn FnMut(u32) -> Self::W,
    ) -> Self::Storage;
    /// Directed-rounding narrowing for a kernel whose true result is **never
    /// exactly representable** at the storage scale — a non-zero-argument
    /// transcendental (`exp`), irrational by Lindemann–Weierstrass and so
    /// always strictly between two storage grid lines. Identical to
    /// [`Self::round_to_storage_directed`] except a working residual of exactly
    /// zero is treated as a genuine sub-resolution positive residual (Ceiling
    /// rounds up, Floor / Trunc keep the floor, nearest is unaffected) — the
    /// only correctly-rounded answer when the deciding residual sits below the
    /// work integer's resolution (e.g. `exp(-10^-S)` just under `1.0`). The
    /// caller MUST pin its algebraic-exact inputs (`exp 0`) before this.
    fn round_to_storage_directed_never_exact(
        base_guard: u32,
        target: u32,
        mode: RoundingMode,
        recompute: &mut dyn FnMut(u32) -> Self::W,
    ) -> Self::Storage;

    // ── the per-tier guard-digit kernels ──────────────────────────────

    /// `e^v` for a working-scale value `v` at scale `w`.
    fn exp_fixed(v_w: Self::W, w: u32) -> Self::W;
    /// Natural log of a positive working-scale value at scale `w`.
    fn ln_fixed(v_w: Self::W, w: u32) -> Self::W;
    /// Sine of a working-scale value at scale `w`.
    fn sin_fixed(v_w: Self::W, w: u32) -> Self::W;
    /// Cosine of a working-scale value at scale `w`.
    fn cos_fixed(v_w: Self::W, w: u32) -> Self::W;
    /// Joint sine + cosine of a working-scale value at scale `w`.
    fn sin_cos_fixed(v_w: Self::W, w: u32) -> (Self::W, Self::W);
    /// Arctangent of a working-scale value at scale `w`.
    fn atan_fixed(v_w: Self::W, w: u32) -> Self::W;

    // ── working-scale helpers the tan kernel needs ────────────────────

    /// `(a · 10^w) / b`, rounded half-to-even.
    fn div(a: Self::W, b: Self::W, w: u32) -> Self::W;
    /// `(a · b) / 10^w`, rounded half-to-even — the plain work-int
    /// multiply. Needed by the inverse / inverse-hyperbolic schoolbooks
    /// (`x^2`, `inv^2`, `t*(t+2)`).
    fn mul(a: Self::W, b: Self::W, w: u32) -> Self::W;
    /// Integer square root of a non-negative working-scale value at
    /// scale `w` (`sqrt(v / 10^w) * 10^w`). The leaf asin/acos/asinh/
    /// acosh schoolbooks need it (`asin = atan(x / sqrt(1 - x^2))`).
    /// Dispatches down to the work-int root.
    fn sqrt_fixed(v: Self::W, w: u32) -> Self::W;
    /// `ln(1 + t)` at working scale `w`, accurate for small `t` — the
    /// near-1 branch of the acosh schoolbook (avoids the `v^2 - 1`
    /// cancellation as `v -> 1`).
    fn log1p_fixed(t: Self::W, w: u32) -> Self::W;
    /// Bit length of `|v|` (0 for zero).
    fn bit_length(v: Self::W) -> u32;

    // hyperbolic exp-identity kernels (sinh/cosh/tanh schoolbooks)

    /// The `ceil(|x| * log10(e))` integer-digit lift for the large-arg
    /// `e^x` reassembly, used to set the base guard so a big `sinh`/
    /// `cosh` stays sub-storage-ULP. `mag_at_scale` is `x * 10^scale`.
    fn exp_result_int_digits(mag_at_scale: Self::W, scale: u32) -> u32;
    /// `sinh(|x|)` at working scale `w` via the `(e^x - e^-x)/2`
    /// identity (composed in the wider [`Self::Wexp`]); caller reapplies
    /// the sign.
    fn sinh_pos_wide(av_w: Self::W, w: u32) -> Self::W;
    /// `cosh(|x|)` at working scale `w` via the `(e^x + e^-x)/2`
    /// identity.
    fn cosh_pos_wide(av_w: Self::W, w: u32) -> Self::W;
    /// `tanh(|x|)` at working scale `w` via the
    /// `(e^x - e^-x)/(e^x + e^-x)` identity; caller reapplies the sign.
    fn tanh_pos_wide(av_w: Self::W, w: u32) -> Self::W;

    /// Directed-rounding narrowing with Ziv escalation, forcing a
    /// confirm recompute even in nearest modes — the acosh / atanh
    /// near-special path (the residual can sit on a rounding boundary).
    fn round_to_storage_directed_near_special(
        base_guard: u32,
        target: u32,
        mode: RoundingMode,
        recompute: &mut dyn FnMut(u32) -> Self::W,
    ) -> Self::Storage;

    // ── working-scale helpers the Tang lookup kernels need ─────────────

    /// The work-integer `1` at working scale `w` (`10^w`), cached.
    fn one(w: u32) -> Self::W;
    /// The work-integer literal `n` (small unsigned).
    fn lit(n: u128) -> Self::W;
    /// `ln 2` at working scale `w`, cached.
    fn ln2(w: u32) -> Self::W;
    /// `(a · b) / 10^w`, rounded half-to-even, with a precomputed
    /// `10^w` divisor (loop-friendly).
    fn mul_cached(a: Self::W, b: Self::W, pow10_w: Self::W) -> Self::W;
    /// `(a · 10^w) / b`, rounded half-to-even, with a precomputed
    /// `10^w` numerator factor (loop-friendly).
    fn div_cached(a: Self::W, b: Self::W, pow10_w: Self::W) -> Self::W;
    /// Rounds a working-scale value to the nearest integer (ties away
    /// from zero); the range-reduction quotient for the Tang exp kernel.
    fn round_to_nearest_int(v: Self::W, w: u32) -> i128;
    /// `10^n` in the work integer (the un-cached power; used to widen by
    /// `extra` digits in the Tang exp reassembly).
    fn pow10(n: u32) -> Self::W;
    /// `Self::W::BITS` — the work integer's bit width.
    fn w_bits() -> u32;

    /// The `ln(1 + i/M)` Tang table slot at working scale `w` (table
    /// size `M = 128`; the `i = 0` slot is `0`, the `i = M` slot is
    /// `ln 2`). Memoised per thread per `w` by the tier's `Core`.
    fn ln_table_entry(w: u32, idx: usize) -> Self::W;

    /// The Tang exp table slot `exp(j · ln2 / M)` at working scale `w`
    /// for table size `M`. Memoised per thread per `(w, M)` by the
    /// tier's `Core`.
    fn exp_table_entry(w: u32, idx: usize, m: u32) -> Self::W;

    // ── π constants + the sincos Tang table (the sincos Tang kernel) ───

    /// `π` at working scale `w`, cached.
    fn pi(w: u32) -> Self::W;
    /// `π/2` at working scale `w`, cached.
    fn half_pi(w: u32) -> Self::W;

    /// The sincos Tang table slot `(sin(c_j), cos(c_j))` at working
    /// scale `w` for table size `m`, where `c_j = j · π / (4·m)` and
    /// `j ∈ [0, m]` (the `j = m` slot is `(sin π/4, cos π/4)`, needed
    /// because rounding can lift a residual to the table boundary).
    /// Memoised per thread per `(w, m)` by the tier's `Core`.
    fn sincos_table_entry(w: u32, idx: usize, m: u32) -> (Self::W, Self::W);
}

/// `exp_strict` for a wide tier — generic over the tier `C`.
///
/// `raw == 0` short-circuits to the type's `ONE` raw (`10^SCALE`) rather
/// than running the Taylor series. Replaces the per-tier
/// `exp_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn exp_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    // `exp(x)` for `x != 0` is transcendental (Lindemann–Weierstrass), so its
    // true value is never exactly on a storage grid line — a zero working
    // residual is a sub-resolution artifact, not a true zero. Use the
    // never-exact narrowing so Ceiling rounds up (and Floor stays) on inputs
    // whose deciding residual sits below the work-int resolution (`exp(-10^-S)`
    // just under `1.0`). `raw == 0` (the one exact case) is pinned above.
    C::round_to_storage_directed_never_exact(C::GUARD, SCALE, mode, &mut |guard| {
        C::exp_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// `ln_strict` for a wide tier — generic over the tier `C`. Panics if
/// `raw <= 0`. Replaces the per-tier `ln_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn ln_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw <= C::storage_zero() {
        panic!("wide-tier ln: argument must be positive");
    }
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::ln_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// `sin_strict` for a wide tier — generic over the tier `C`. Replaces
/// the per-tier `sin_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn sin_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::sin_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// `cos_strict` for a wide tier — generic over the tier `C`. Standalone
/// `cos_fixed` path (cofunction identity, one `sin_fixed`, no sqrt).
/// Replaces the per-tier `cos_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn cos_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::cos_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// `tan_strict` for a wide tier — generic over the tier `C`. Panics at
/// odd multiples of π/2 where the cosine is zero. Ports the near-pole
/// recompute (`near_pole_tan::tan_extra_digits`, width-free). Replaces
/// the per-tier `tan_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn tan_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w0 = SCALE + C::GUARD;
    let (sin0, cos0) = C::sin_cos_fixed(C::to_work(raw), w0);
    if cos0 == C::zero() {
        panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let probe = C::div(sin0, cos0, w0);
    let extra = crate::algos::trig::near_pole_tan::tan_extra_digits(C::bit_length(probe), w0)
        .saturating_sub(C::GUARD);
    if extra == 0 {
        return C::round_to_storage_with(probe, w0, SCALE, mode);
    }
    let w = w0 + extra;
    let (sin_w, cos_w) = C::sin_cos_fixed(C::to_work_w(raw, C::GUARD + extra), w);
    let r = C::div(sin_w, cos_w, w);
    C::round_to_storage_with(r, w, SCALE, mode)
}

/// `atan_strict` for a wide tier — generic over the tier `C`. Result in
/// `(−π/2, π/2)`. Replaces the per-tier `atan_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn atan_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::atan_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// Narrow-`GUARD` single-shot `atan_strict` for a wide tier — generic
/// over the tier `C`, the decimal `SCALE`, and the band's narrow guard
/// `GUARD`. Routes the canonical [`WideTrigCore::atan_fixed`] kernel
/// through `w = SCALE + GUARD` and narrows once with
/// [`WideTrigCore::round_to_storage_with`] (no Ziv escalation — the band
/// guards leave the working error many orders of magnitude below half a
/// storage ULP). Replaces the per-tier atan narrow wrappers.
#[inline]
#[must_use]
pub(crate) fn atan_narrow<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w = SCALE + GUARD;
    let v_w = C::to_work_w(raw, GUARD);
    let r = C::atan_fixed(v_w, w);
    C::round_to_storage_with(r, w, SCALE, mode)
}
