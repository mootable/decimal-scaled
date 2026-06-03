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
    /// The wide composition / AGM work integer (the two-core split): the
    /// compositions + the AGM run on `Wagm` so a narrowed primitive
    /// [`Self::W`] cannot clip their precision. Aliases [`Self::W`] until
    /// the primitive `$Work` is narrowed.
    type Wagm: BigInt + Copy + PartialEq;
    /// The tier's storage integer (`x · 10^SCALE`).
    type Storage: BigInt + Copy + PartialEq;

    /// Guard digits added below the storage scale on the strict path.
    const GUARD: u32;

    // ── value zero / small constants in `Storage` and `W` ──────────────

    /// The storage `0`.
    fn storage_zero() -> Self::Storage;
    /// The storage representation of `1` at scale `SCALE` (`10^SCALE`).
    fn storage_one(scale: u32) -> Self::Storage;
    /// The storage `MAX` / `MIN` (the tier's representable bounds). Supplied to
    /// the work-rung narrowing range check (`round_to_storage_*_g`): `MAX`/`MIN`
    /// are inherent consts on `Int<N>`, NOT on `BigInt`, so a tier-generic
    /// kernel sources them through the `Core` here rather than `Self::Storage::MAX`.
    fn storage_max() -> Self::Storage;
    /// See [`Self::storage_max`].
    fn storage_min() -> Self::Storage;
    /// The work-integer `0`.
    fn zero() -> Self::W;

    // ── working-scale lift / narrow ────────────────────────────────────

    /// Builds a working-scale `W` from raw storage, scaling by
    /// `10^working_digits` (raw is `value · 10^SCALE`).
    fn to_work_scaled(raw: Self::Storage, working_digits: u32) -> Self::W;
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
    /// `Wagm` storage-bridge: lift raw storage to the wide composition work
    /// integer (the `Wagm` sibling of [`Self::to_work_scaled`]).
    fn to_work_scaled_agm(raw: Self::Storage, working_digits: u32) -> Self::Wagm;
    /// `Wagm` storage-bridge: narrow a `Wagm` composition value at scale `w`
    /// to scale `target` under `mode` (the `Wagm` sibling of
    /// [`Self::round_to_storage_with`]).
    fn round_to_storage_with_agm(
        v: Self::Wagm,
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

    /// `e^v` for a working-scale value `v` at scale `w`. `SCALE`
    /// const-folds the internal `ln 2` — see [`Self::ln_fixed`].
    fn exp_fixed<const SCALE: u32>(v_w: Self::W, w: u32) -> Self::W;
    /// Natural log of a positive working-scale value at scale `w`.
    ///
    /// `SCALE` is the decimal layer's own storage scale: on the common
    /// path `w == SCALE + GUARD`, so the kernel reads its `ln 2`
    /// constant from the compile-time baked `WideConst<SCALE>` rather
    /// than re-deriving it at runtime; any other `w` (Ziv escalation)
    /// falls to the runtime const. Bit-identical either way.
    fn ln_fixed<const SCALE: u32>(v_w: Self::W, w: u32) -> Self::W;
    /// Sine of a working-scale value at scale `w`. `SCALE` const-folds
    /// the internal `π` — see [`Self::ln_fixed`].
    fn sin_fixed<const SCALE: u32>(v_w: Self::W, w: u32) -> Self::W;
    /// Cosine of a working-scale value at scale `w`. `SCALE` const-folds
    /// the internal `π` — see [`Self::ln_fixed`].
    fn cos_fixed<const SCALE: u32>(v_w: Self::W, w: u32) -> Self::W;
    /// Joint sine + cosine of a working-scale value at scale `w`. `SCALE`
    /// const-folds the internal `π` — see [`Self::ln_fixed`].
    fn sin_cos_fixed<const SCALE: u32>(v_w: Self::W, w: u32) -> (Self::W, Self::W);
    /// Arctangent of a working-scale value at scale `w`. `SCALE`
    /// const-folds the internal `π/2` — see [`Self::ln_fixed`].
    fn atan_fixed<const SCALE: u32>(v_w: Self::W, w: u32) -> Self::W;

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
    /// the sign. `SCALE` const-folds the internal `ln 2` (via
    /// `exp_fixed`) — see [`Self::ln_fixed`].
    fn sinh_pos_wide<const SCALE: u32>(av_w: Self::W, w: u32) -> Self::W;
    /// `cosh(|x|)` at working scale `w` via the `(e^x + e^-x)/2`
    /// identity. `SCALE` const-folds the internal `ln 2` — see
    /// [`Self::sinh_pos_wide`].
    fn cosh_pos_wide<const SCALE: u32>(av_w: Self::W, w: u32) -> Self::W;
    /// `tanh(|x|)` at working scale `w` via the
    /// `(e^x - e^-x)/(e^x + e^-x)` identity; caller reapplies the sign.
    /// `SCALE` const-folds the internal `ln 2` — see
    /// [`Self::sinh_pos_wide`].
    fn tanh_pos_wide<const SCALE: u32>(av_w: Self::W, w: u32) -> Self::W;

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
    /// `ln 2` at working scale `w`, const-folded at the layer's own
    /// `SCALE` (the baked `WideConst<SCALE>` on the common
    /// `w == SCALE + GUARD` path) — see [`Self::ln_fixed`].
    fn ln2<const SCALE: u32>(w: u32) -> Self::W;
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
    /// `ln 2`). Recomputed on the stack per call; `SCALE` const-folds
    /// the internal `ln 2` — see [`Self::ln_fixed`].
    fn ln_table_entry<const SCALE: u32>(w: u32, idx: usize) -> Self::W;

    /// The Tang exp table slot `exp(j · ln2 / M)` at working scale `w`
    /// for table size `M`. Recomputed on the stack per call; `SCALE`
    /// const-folds the internal `ln 2` — see [`Self::ln_fixed`].
    fn exp_table_entry<const SCALE: u32>(w: u32, idx: usize, m: u32) -> Self::W;

    // ── π constants + the sincos Tang table (the sincos Tang kernel) ───

    /// `π` at working scale `w`, const-folded at the layer's own `SCALE`
    /// (the baked `WideConst<SCALE>` on the common `w == SCALE + GUARD`
    /// path) — see [`Self::ln_fixed`].
    fn pi<const SCALE: u32>(w: u32) -> Self::W;
    /// `π/2` at working scale `w`, const-folded at the layer's own
    /// `SCALE` — see [`Self::pi`].
    fn half_pi<const SCALE: u32>(w: u32) -> Self::W;

    /// `180/π` (degrees per radian) at working scale `w`, const-folded at
    /// the layer's own `SCALE` — see [`Self::pi`]. The exact angle-scale
    /// factor the `to_degrees` `MulPiRatio` kernel multiplies by (`x *
    /// 180/π`), replacing the runtime divide-by-`π`.
    fn deg_per_rad<const SCALE: u32>(w: u32) -> Self::W;
    /// `π/180` (radians per degree) at working scale `w`, const-folded at
    /// the layer's own `SCALE` — see [`Self::pi`]. The exact angle-scale
    /// factor the `to_radians` `MulPiRatio` kernel multiplies by (`x *
    /// π/180`).
    fn rad_per_deg<const SCALE: u32>(w: u32) -> Self::W;

    /// The sincos Tang table slot `(sin(c_j), cos(c_j))` at working
    /// scale `w` for table size `m`, where `c_j = j · π / (4·m)` and
    /// `j ∈ [0, m]` (the `j = m` slot is `(sin π/4, cos π/4)`, needed
    /// because rounding can lift a residual to the table boundary).
    /// Recomputed on the stack per call; `SCALE` const-folds the
    /// internal `π` — see [`Self::ln_fixed`].
    fn sincos_table_entry<const SCALE: u32>(w: u32, idx: usize, m: u32) -> (Self::W, Self::W);
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
        C::exp_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
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
        C::ln_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
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
    let r = C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::sin_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
    });
    adjust_bounded_extremum::<C, SCALE>(r, raw, mode)
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
    let r = C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::cos_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
    });
    adjust_bounded_extremum::<C, SCALE>(r, raw, mode)
}

/// Directed-rounding post-adjust for `sin`/`cos` near an extremum the
/// working-scale kernel cannot resolve.
///
/// `sin`/`cos` lie STRICTLY inside `(−1, 1)` for every representable
/// non-special argument: `cos = +1` only at the already-pinned `raw == 0`,
/// and `cos = −1` / `sin = ±1` occur only at arguments (`π`, `π/2 + kπ`) that
/// are never exactly representable. But for an argument within the input
/// granularity of an extremum the deviation `δ²/2` from `±1` can sit far below
/// any REACHABLE working scale — e.g. ~`10⁻³⁴⁷` at D462 s346, against a
/// ~462-digit work-integer ceiling — so the kernel rounds to exactly
/// `±10^SCALE` and a DIRECTED mode then lands on the wrong side of the grid
/// line (the value is interior, but the kernel saw it AS the extremum).
///
/// Because the true value is strictly interior, the directed side is known a
/// priori with no extra precision:
/// - just below `+1` (`result == +one`): Floor / Trunc step down one LSB to
///   `one − 1`; Ceiling keeps `one`.
/// - just above `−1` (`result == −one`): Ceiling / Trunc step toward zero to
///   `−one + 1`; Floor keeps `−one`.
///
/// Nearest modes are unaffected (rounding to `±1` IS correct to nearest there).
/// A no-op unless the directed result is exactly `±10^SCALE` and `raw != 0`, so
/// reachable cells (already resolved off the grid line) and the exact
/// `cos(0) = 1` point pass through untouched. The rule is continuous over the
/// whole near-extremum region, not fitted to one benched cell.
#[inline]
pub(crate) fn adjust_bounded_extremum<C: WideTrigCore, const SCALE: u32>(
    result: C::Storage,
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if crate::support::rounding::is_nearest_mode(mode) || raw == C::storage_zero() {
        return result;
    }
    let one = C::storage_one(SCALE);
    let neg_one = C::storage_zero() - one;
    if result == one {
        match mode {
            RoundingMode::Floor | RoundingMode::Trunc => one - <C::Storage as BigInt>::ONE,
            _ => result,
        }
    } else if result == neg_one {
        match mode {
            RoundingMode::Ceiling | RoundingMode::Trunc => neg_one + <C::Storage as BigInt>::ONE,
            _ => result,
        }
    } else {
        result
    }
}

/// Directed-rounding post-adjust for `ln` very near `x = 1`.
///
/// Concavity gives `ln(x) < x − 1` STRICTLY for every `x ≠ 1`, and `ln(x)`
/// is transcendental for algebraic `x ≠ 1`, so it never lands exactly on a
/// storage grid line. For `x` within the input granularity of 1 the deficit
/// `(x − 1) − ln(x) ≈ (x − 1)²/2` can sit far below any REACHABLE working
/// scale (e.g. `x = 1 + 10⁻ˢ` leaves the deficit ~`10⁻²ˢ`), so the kernel
/// rounds to exactly the linear term `δ = raw − 10^SCALE` and a downward
/// mode then keeps `δ` though the true value is strictly below it. Because
/// `ln(x) < x − 1`, a CORRECT downward result can never equal `δ`, so
/// `result == δ` is unambiguously the sub-resolution overshoot — step down
/// one LSB. `ln(1) = 0` is exact (`raw == one`) and excluded; nearest modes
/// (frac ≈ 1⁻, rounds to `δ`) and `Ceiling` (`δ` IS the correct ceiling)
/// are already right. `Floor` steps down for both signs; `Trunc` (toward
/// zero) steps down only for `x > 1` (positive value). A no-op unless the
/// result is exactly `δ`, so reachable cells pass untouched. Mirrors
/// [`adjust_bounded_extremum`] / [`adjust_cosh_near_min`].
#[inline]
pub(crate) fn adjust_ln_near_one<C: WideTrigCore, const SCALE: u32>(
    result: C::Storage,
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if crate::support::rounding::is_nearest_mode(mode) {
        return result;
    }
    let one = C::storage_one(SCALE);
    if raw == one {
        return result; // ln(1) = 0 is exact
    }
    let delta = raw - one; // (x − 1) at storage scale (signed)
    if result != delta {
        return result; // only the sub-resolution linear-term overshoot
    }
    match mode {
        RoundingMode::Floor => result - <C::Storage as BigInt>::ONE,
        RoundingMode::Trunc if raw > one => result - <C::Storage as BigInt>::ONE,
        _ => result,
    }
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
    let (sin0, cos0) = C::sin_cos_fixed::<SCALE>(C::to_work(raw), w0);
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
    let (sin_w, cos_w) = C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, C::GUARD + extra), w);
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
        C::atan_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
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
    let v_w = C::to_work_scaled(raw, GUARD);
    let r = C::atan_fixed::<SCALE>(v_w, w);
    C::round_to_storage_with(r, w, SCALE, mode)
}

// ─── Work-int-generic narrowing / lift free fns (the SCALE-derived work-rung
//     surface) ─────────────────────────────────────────────────────────────
//
// Hoisted out of the per-tier `decl_wide_transcendental!` macro so a
// tier-generic kernel (e.g. `ln_tang_g<C, Wk>`) can lift/narrow at an
// arbitrary work rung `Wk` WITHOUT a per-tier module path and WITHOUT a new
// trait method (owner directive: free-fn hoist, no trait-surface growth).
// `St` (storage) appears only as the input/output type + the range-check
// bounds; `St` has no trait-level `MAX`/`MIN`, so the caller supplies them
// (`st_max`/`st_min`). The per-tier macro forwards pass `<$Storage>::MAX/MIN`
// (bit-identical to the prior inline bodies); a tier-generic caller passes
// `C::storage_max()/storage_min()`. The `÷10^shift` divides are already
// width-generic (`div_wide_pow10::<S>` / `dispatch_wide_pow10::<S>`).

/// Work-int-generic lift-up: widen storage `St` into the work integer `S` and
/// scale by `10^working_digits`. Storage-generic sibling of the per-tier
/// `to_work_scaled`; sources `10^d` from the width-generic `exp_generic::pow10`.
#[inline]
pub(crate) fn to_work_scaled_g<St: BigInt, S: BigInt>(raw: St, working_digits: u32) -> S
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    BigInt::resize_to::<S>(raw) * crate::algos::exp::exp_generic::pow10::<S>(working_digits)
}

/// Work-int-generic narrowing of a working-scale value `v` (at scale `w`) down
/// to storage scale `target`, rounded under `mode`, into storage `St`.
/// `st_max`/`st_min` are `St::MAX`/`MIN`, caller-supplied.
#[inline]
pub(crate) fn round_to_storage_with_g<St: BigInt + Copy, S: BigInt>(
    v: S,
    w: u32,
    target: u32,
    mode: RoundingMode,
    st_max: St,
    st_min: St,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let shift = w - target;
    let rounded = if shift == 0 {
        v
    } else if shift <= 38 {
        crate::algos::support::mg_divide::div_wide_pow10::<S>(v, shift, mode)
    } else {
        crate::algos::support::rescale::dispatch_wide_pow10::<S>(v, shift, mode)
    };
    let max_w = BigInt::resize_to::<S>(st_max);
    let min_w = BigInt::resize_to::<S>(st_min);
    if rounded > max_w || rounded < min_w {
        panic!("wide-tier strict transcendental: result out of range");
    }
    BigInt::resize_to::<St>(rounded)
}

/// Work-int-generic directed-rounding narrowing with Ziv escalation. `St` =
/// storage output, `S` = work integer (a rung `Wk` or the tier `W`).
#[inline]
pub(crate) fn round_to_storage_directed_g<St: BigInt + Copy, S: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    st_max: St,
    st_min: St,
    recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(base_guard, target, mode, false, false, st_max, st_min, recompute)
}

/// `never_exact` directed narrowing (an irrational-valued kernel, e.g. `exp`):
/// a zero working residual is a sub-resolution positive residual.
#[inline]
pub(crate) fn round_to_storage_directed_never_exact_g<St: BigInt + Copy, S: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    st_max: St,
    st_min: St,
    recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(base_guard, target, mode, false, true, st_max, st_min, recompute)
}

/// Near-special-point directed narrowing (`acosh` at 1, `atanh` at ±1):
/// force a confirm recompute even in nearest modes.
#[inline]
pub(crate) fn round_to_storage_directed_near_special_g<St: BigInt + Copy, S: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    st_max: St,
    st_min: St,
    recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(base_guard, target, mode, true, false, st_max, st_min, recompute)
}

fn round_to_storage_directed_impl_g<St: BigInt + Copy, S: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    force_confirm: bool,
    never_exact: bool,
    st_max: St,
    st_min: St,
    mut recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::support::rounding::{is_nearest_mode, RoundingMode};

    let lit = |n: i128| <S as BigInt>::from_i128(n);
    let pow10 = |n: u32| crate::consts::pow10::dispatch::<S>(n);
    let bit_length = |v: S| -> u32 {
        let m = if v < <S as BigInt>::ZERO { -v } else { v };
        <S as BigInt>::BITS - m.leading_zeros()
    };
    let round_to_storage_with = |v: S, w: u32, t: u32, m: RoundingMode| -> St {
        round_to_storage_with_g::<St, S>(v, w, t, m, st_max, st_min)
    };

    let base_w = target + base_guard;
    if is_nearest_mode(mode) {
        if !force_confirm {
            return round_to_storage_with(recompute(base_guard), base_w, target, mode);
        }
        let mut nearest_narrow = |guard: u32| -> St {
            let w = target + guard;
            round_to_storage_with(recompute(guard), w, target, mode)
        };
        let lo = nearest_narrow(base_guard);
        let int_digits = {
            let n = BigInt::resize_to::<S>(lo);
            let m = if n < lit(0) { -n } else { n };
            let bl = bit_length(m);
            let storage_digits = (bl as u64 * 30103 / 100_000) as u32 + 1;
            storage_digits.saturating_sub(target)
        };
        let cap_digits = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        let max_guard = cap_digits.saturating_sub(target).max(base_guard);
        let mut guard = base_guard;
        let mut best = lo;
        loop {
            if guard >= max_guard {
                break;
            }
            let step = (target + base_guard).max(base_guard);
            let next_guard = guard.saturating_add(step).min(max_guard);
            let hi = nearest_narrow(next_guard);
            if hi == best {
                break;
            }
            guard = next_guard;
            best = hi;
        }
        return best;
    }

    let mut directed_narrow = |guard: u32| -> (S, S, S) {
        let w = target + guard;
        let v = recompute(guard);
        let shift = w - target;
        let neg = v < lit(0);
        let mag = if neg { -v } else { v };
        let divisor = pow10(shift);
        let (q, rem) = mag.div_rem(divisor);
        let result_positive = !neg;
        let residual_present = rem != lit(0) || never_exact;
        let bump = residual_present
            && match mode {
                RoundingMode::Trunc => false,
                RoundingMode::Floor => !result_positive,
                RoundingMode::Ceiling => result_positive,
                _ => unreachable!(),
            };
        let q_mag = if bump { q + lit(1) } else { q };
        let signed = if neg { -q_mag } else { q_mag };
        let dist = if rem < divisor - rem {
            rem
        } else {
            divisor - rem
        };
        (signed, dist, divisor)
    };

    let (mut lo, dist0, divisor0) = directed_narrow(base_guard);

    let band0 = divisor0 / lit(1000);
    let near_grid = force_confirm || dist0 <= band0;

    let signed = if !near_grid {
        lo
    } else {
        let int_digits = {
            let m = if lo < lit(0) { -lo } else { lo };
            let bl = bit_length(m);
            let storage_digits = (bl as u64 * 30103 / 100_000) as u32 + 1;
            storage_digits.saturating_sub(target)
        };
        let cap_digits = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        let max_guard = cap_digits.saturating_sub(target).max(base_guard);

        let mut guard = base_guard;
        loop {
            if guard >= max_guard {
                break lo;
            }
            let step = (target + base_guard).max(base_guard);
            let next_guard = guard.saturating_add(step).min(max_guard);
            let (hi, hi_dist, hi_div) = directed_narrow(next_guard);
            let hi_band = hi_div / lit(1000);
            let resolved = hi_dist == lit(0) || hi_dist > hi_band;
            if hi == lo && resolved {
                break hi;
            }
            guard = next_guard;
            lo = hi;
        }
    };

    let max_w = BigInt::resize_to::<S>(st_max);
    let min_w = BigInt::resize_to::<S>(st_min);
    if signed > max_w || signed < min_w {
        panic!("wide-tier strict transcendental: result out of range");
    }
    BigInt::resize_to::<St>(signed)
}
