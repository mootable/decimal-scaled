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

/// Near-min analytic pin for `exp`. When `|v| < 10^(-SCALE/2)` the deviation
/// `e^v − (1 + v) = v²/2 + …` is provably below half a storage ULP, and `e^v > 1 + v`
/// strictly (exp is convex), so the correctly-rounded result is exactly `1 + v` for
/// every mode except `Ceiling`, which the positive deviation — however deep it
/// sits — rounds up by one ULP.
/// This short-circuits the widening: at these tiny inputs its `s >> n` range reduction
/// loses bits (the working guard carries fewer factors of 2 than `n`), and the
/// resulting sub-ULP deficit borrows into the result digit at the `…999000` /
/// `1.000…` grid line — a deciding digit past the work integer's reach, so the
/// escalation can't recover it. Returns `None` (defer to the normal path) otherwise.
/// The cheap bit-length pre-filter exits before the `pow10` for every non-tiny input,
/// so the hot path is unaffected.
#[inline]
fn exp_near_min_pin<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> Option<C::Storage> {
    let half = SCALE / 2;
    let zero = C::storage_zero();
    if half == 0 || raw == zero {
        return None;
    }
    let absr = if raw < zero { zero - raw } else { raw };
    // `10^half` has ≈ half·log2(10) bits; skip the pow10 unless |raw| is plausibly
    // below it (true only for genuinely tiny inputs — every normal call exits here).
    // A value of bit-length `bl` is at least `2^(bl−1)`, so the exit is certain
    // only once `2^(bl−1) >= 10^half` ⟺ `(bl−1)·log10(2) >= half` — comparing
    // `bl` itself (not `bl−1`) silently dropped the top quarter of the band.
    let bl = <C::Storage as BigInt>::BITS - absr.leading_zeros();
    if ((bl - 1) as u64) * 100_000 >= (half as u64) * 332_193 {
        return None;
    }
    // Exact: |raw| < 10^half ⟺ v²/2 < ½ ULP and the deviation sits past the scale.
    if absr >= crate::consts::pow10::dispatch::<C::Storage>(half) {
        return None;
    }
    let g = C::storage_one(SCALE) + raw; // (1 + v), exact since |v| < 1
    Some(match mode {
        RoundingMode::Ceiling => g + <C::Storage as BigInt>::from_i128(1),
        _ => g,
    })
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
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    if let Some(r) = exp_near_min_pin::<C, SCALE>(raw, mode) {
        return r;
    }
    // `exp(x)` for `x != 0` is transcendental (Lindemann–Weierstrass), so its
    // true value is never exactly on a storage grid line — a zero working
    // residual is a sub-resolution artifact, not a true zero. The `never_exact`
    // rule makes Ceiling round up (Floor stays) on inputs whose deciding
    // residual sits below the work-int resolution (`exp(-10^-S)` just under
    // `1.0`). `raw == 0` (the one exact case) is pinned above.
    //
    // Two-width near-min widening: near `x ≈ 0` the half-ULP tie of
    // `exp(±10^-k)` is decided by the `x³/6` term at digit ≈ `1.5·SCALE`, beyond
    // the tier work integer's escalation reach at mid/high scales; retry at
    // `C::Wexp` when the deciding digit is unreachable in `C::W`. Deep ties past
    // the precision horizon stay exact ties.
    round_to_storage_widening_g::<C::Storage, C::W, C::Wexp>(
        C::GUARD,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard| C::exp_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard),
        |guard| {
            crate::algos::exp::exp_generic::exp_fixed::<C::Wexp>(
                to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard),
                SCALE + guard,
            )
        },
    )
}

/// `ln_strict` for a wide tier — generic over the tier `C`. Panics if
/// `raw <= 0`. Replaces the per-tier `ln_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn ln_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    ln_series_g::<C, C::W, SCALE>(raw, mode)
}

/// Rung-generic `ln_series` — the Brent-reduction wide-ln kernel run at an
/// arbitrary work rung `Wk` (decoupled from `C::W`), so the policy can
/// run / bench Series at its minimal valid work width
/// (mirrors [`crate::algos::ln::ln_tang::ln_tang_g`]; [`ln_series`] is the
/// `Wk = C::W` alias, bit-identical). Calls the already-work-int-generic
/// `exp_generic::ln_fixed::<Wk>` directly (bypassing the fixed-`C::W` trait
/// method `C::ln_fixed`), with `ln 2` const-folded at the base working scale on
/// the hot path — value-identical to the per-tier `ln2_cf::<SCALE>`.
#[inline]
#[must_use]
pub(crate) fn ln_series_g<C: WideTrigCore, Wk: BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if raw <= C::storage_zero() {
        panic!("wide-tier ln: argument must be positive");
    }
    round_to_storage_directed_g::<C::Storage, Wk>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let ln2 = if w == SCALE + C::GUARD {
                crate::consts::ln2_by_scale::<Wk>(w, crate::support::rounding::DEFAULT_ROUNDING_MODE)
            } else {
                crate::consts::ln2_by_working_scale::<Wk>(
                    w,
                    crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            };
            crate::algos::exp::exp_generic::ln_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard),
                w,
                ln2,
            )
        },
    )
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
// trait method (free-fn hoist, no trait-surface growth).
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

/// Absolute floor (`10^4` work-integer units) separating a genuine deciding-
/// digit SIGNAL from the kernel's own working-scale rounding NOISE. The strict
/// kernels compute to a few-ULP working accuracy (≈10²–10³ work units of
/// error); a residual-to-boundary distance above this is a real deciding digit,
/// below it is noise. Once a deciding term is representable its signal grows
/// 10× per extra working digit, so it clears the floor within a handful of
/// digits. Using an ABSOLUTE floor (not the old `divisor/1000`, which scales
/// with the working scale and so never fired for a near-min input) is what lets
/// the loop tell "resolved" from "spinning on kernel noise" — and so return the
/// clean exact-tie base narrowing instead of a noise-driven deep misround.
const ZIV_RESOLVE_FLOOR_POW10: u32 = 4;

/// How deep the Ziv escalations may PROBE, in working-scale digits. A
/// near-tie whose deciding term lies beyond this is unverifiable (the widest
/// shipped tier, D1232 / `Int<64>` storage, carries ~1232 significant digits,
/// and the golden data is generated to match), so the escalation stops and
/// falls to its unresolved endgame — the near-min resolver then applies the
/// never-exact sub-resolution rule (a strictly positive tail under the
/// computed digits) instead of trusting a kernel-noise-driven deep narrowing.
const ZIV_PRECISION_HORIZON: u32 = 1264;

/// Exact decimal digit count of a non-negative work value (`v > 0`).
/// Bit-length estimate (`digits <= floor(bl·log10 2) + 1`, at most one high),
/// refined by a single `pow10` compare. Cold-path helper for the positional
/// cross-depth confirmation below.
fn dec_digits_g<S: BigInt>(v: S) -> u32 {
    let bl = <S as BigInt>::BITS - v.leading_zeros();
    let mut d = ((bl as u64 * 30_103) / 100_000) as u32 + 1;
    if d > 1 && v < crate::consts::pow10::dispatch::<S>(d - 1) {
        d -= 1;
    }
    d
}

/// Single-width near-min escalation for `cosh` / `exp`, returning
/// `(value, resolved)`. A near-tie's deciding term (`cosh`'s `x⁴/24`,
/// `exp`'s `x³/6`) is trusted only once it clears the absolute kernel-noise
/// floor AND a probe at a DIFFERENT depth reproduces its position and side —
/// the kernels' working-scale error can far exceed the floor (measured
/// ~10^12 units at some depths), but noise always sits in the bottom digits
/// of whatever `w` produced it, while a genuine deciding term keeps a
/// depth-independent fractional position. `resolved == false` means the cap
/// (the work integer's capacity, never past [`ZIV_PRECISION_HORIZON`]) was
/// reached without a confirmed deciding term: the value is then the BASE
/// probe SNAPPED to its nearest grid line (absorbing the kernel's sub-floor
/// noise) with the `never_exact` sub-resolution rule applied on top — the
/// true value carries a strictly positive tail below the computed digits, so
/// Ceiling (positive result) / Floor (negative) nudge one ULP off the grid
/// line and an unresolved half-ULP boundary rounds as ABOVE half for the
/// nearest modes. A widening caller may still retry at a wider integer.
#[allow(clippy::too_many_arguments)]
fn near_min_resolve_g<St: BigInt + Copy, S: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    never_exact: bool,
    st_max: St,
    st_min: St,
    mut recompute: impl FnMut(u32) -> S,
) -> (St, bool)
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::support::rounding::{is_nearest_mode, should_bump, RoundingMode};
    let lit = |n: i128| <S as BigInt>::from_i128(n);
    let pow10 = |n: u32| crate::consts::pow10::dispatch::<S>(n);
    let bit_length = |v: S| -> u32 {
        let m = if v < <S as BigInt>::ZERO { -v } else { v };
        <S as BigInt>::BITS - m.leading_zeros()
    };
    let floor = pow10(ZIV_RESOLVE_FLOOR_POW10);
    // The kernels' error is value-RELATIVE (~an ULP of the value at the
    // working scale), so a result carrying `int_digits` integer digits has a
    // noise floor of ~10^int_digits WORKING UNITS regardless of `w`: a
    // fractional digit at depth `p` only clears that noise once
    // `w > p + int_digits`. The probe horizon therefore extends by
    // `int_digits` (the width cap already subtracts them, protecting the
    // kernel's internal headroom).
    let max_guard_for = |int_digits: u32| -> u32 {
        let cap = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        cap.saturating_sub(target)
            .min((ZIV_PRECISION_HORIZON + int_digits).saturating_sub(target))
            .max(base_guard)
    };
    let int_digits_of = |v: St| -> u32 {
        let n = BigInt::resize_to::<S>(v);
        let m = if n < lit(0) { -n } else { n };
        ((bit_length(m) as u64 * 30103 / 100_000) as u32 + 1).saturating_sub(target)
    };
    let range_check = |signed: S| -> St {
        let max_w = BigInt::resize_to::<S>(st_max);
        let min_w = BigInt::resize_to::<S>(st_min);
        if signed > max_w || signed < min_w {
            panic!("wide-tier strict transcendental: result out of range");
        }
        BigInt::resize_to::<St>(signed)
    };
    let finish = |neg: bool, q: S, bump: bool| -> St {
        let q_mag = if bump { q + lit(1) } else { q };
        range_check(if neg { -q_mag } else { q_mag })
    };
    // Leading fractional-digit position of the deciding residual `dist`
    // (working units at scale `target + g`) — the cross-depth confirmation
    // key: a genuine deciding term keeps this position across depths.
    let pos_of = |dist: S, g: u32| -> u32 { target + g - dec_digits_g::<S>(dist) + 1 };
    // One working-scale probe: `(neg, q, rem, divisor)` of the recomputed
    // value at guard `g`, magnitude split at the storage grid.
    let mut probe = |g: u32| -> (bool, S, S, S) {
        let v = recompute(g);
        let neg = v < lit(0);
        let mag = if neg { -v } else { v };
        let divisor = pow10(g);
        let (q, rem) = mag.div_rem(divisor);
        (neg, q, rem, divisor)
    };

    if is_nearest_mode(mode) {
        // Nearest rounding of a probe, by the true `rem` vs `half` ordering.
        let round_half = |neg: bool, q: S, rem: S, divisor: S| -> St {
            let half = divisor / lit(2);
            let cmp_r = if rem < half {
                ::core::cmp::Ordering::Less
            } else if rem == half {
                ::core::cmp::Ordering::Equal
            } else {
                ::core::cmp::Ordering::Greater
            };
            finish(neg, q, should_bump(mode, cmp_r, q.bit(0), !neg))
        };
        let (neg0, q0, rem0, div0) = probe(base_guard);
        let half0 = div0 / lit(2);
        let dist0 = if rem0 < half0 { half0 - rem0 } else { rem0 - half0 };
        if dist0 > pow10(base_guard) / lit(1000) {
            return (round_half(neg0, q0, rem0, div0), true); // not near a half-ULP tie
        }
        let lo = round_half(neg0, q0, rem0, div0);
        let max_guard = max_guard_for(int_digits_of(lo));
        // Cross-depth confirmation: the kernels' working-scale error can reach
        // well past the absolute noise floor (measured ~10^12 units at some
        // depths), but noise always sits in the BOTTOM digits of whatever `w`
        // produced it, while a genuine deciding term keeps a depth-independent
        // fractional position. A probe's signal `(position, side)` is therefore
        // trusted only once a probe at a DIFFERENT depth reproduces it.
        let mut pending: Option<(u32, bool)> = if dist0 > floor {
            Some((pos_of(dist0, base_guard), rem0 > half0))
        } else {
            None
        };
        let mut guard = base_guard;
        loop {
            if guard >= max_guard {
                // Cap reached without a confirmed deciding term. An unconfirmed
                // signal from the deepest probe gets ONE shifted confirm probe
                // (real positions reproduce; noise tracks the bottom of `w`).
                if let Some((pp, ps)) = pending {
                    let back = ZIV_RESOLVE_FLOOR_POW10 + 3;
                    if max_guard > base_guard + back {
                        let g_c = max_guard - back;
                        let (neg, q, rem, div) = probe(g_c);
                        let half = div / lit(2);
                        let dist = if rem < half { half - rem } else { rem - half };
                        if dist > floor {
                            let p = pos_of(dist, g_c);
                            if (rem > half) == ps && p.abs_diff(pp) <= 1 {
                                return (round_half(neg, q, rem, div), true);
                            }
                        }
                    }
                }
                // Still unresolved: the deciding term sits below every probed
                // depth. With the never-exact rule the half boundary carries a
                // strictly positive sub-resolution tail — the residual is
                // ABOVE half, so every nearest mode rounds away from the kept
                // value; otherwise keep the clean base narrowing. Either way
                // the widening caller may retry at a wider integer.
                if never_exact {
                    return (finish(neg0, q0, true), false);
                }
                return (lo, false);
            }
            let step = (target + base_guard).max(base_guard);
            let next_guard = guard.saturating_add(step).min(max_guard);
            let (neg, q, rem, div) = probe(next_guard);
            let half = div / lit(2);
            let hi_dist = if rem < half { half - rem } else { rem - half };
            if hi_dist > floor {
                let p = pos_of(hi_dist, next_guard);
                let above = rem > half;
                if let Some((pp, ps)) = pending {
                    if above == ps && p.abs_diff(pp) <= 1 {
                        // Confirmed deciding digit — trustworthy.
                        return (round_half(neg, q, rem, div), true);
                    }
                }
                pending = Some((p, above));
            }
            guard = next_guard;
        }
    }

    // directed
    // Directed semantics: any nonzero residual (or the `never_exact`
    // sub-resolution rule) nudges Ceiling/Floor by the result's sign.
    let dir_round = |neg: bool, q: S, rem: S| -> St {
        let result_positive = !neg;
        let residual_present = rem != lit(0) || never_exact;
        let bump = residual_present
            && match mode {
                RoundingMode::Trunc => false,
                RoundingMode::Floor => !result_positive,
                RoundingMode::Ceiling => result_positive,
                _ => unreachable!(),
            };
        finish(neg, q, bump)
    };
    let (neg0, q0, rem0, div0) = probe(base_guard);
    let dist0 = if rem0 < div0 - rem0 { rem0 } else { div0 - rem0 };
    if dist0 > pow10(base_guard) / lit(1000) {
        return (dir_round(neg0, q0, rem0), true); // clear of a grid line
    }
    let base = dir_round(neg0, q0, rem0);
    let max_guard = max_guard_for(int_digits_of(base));
    // Cross-depth confirmation — see the nearest branch: a probe's signal
    // `(position, side)` is trusted only once a probe at a different depth
    // reproduces it (noise tracks the bottom of `w`; real digits do not move).
    let mut pending: Option<(u32, bool)> = if dist0 > floor {
        Some((pos_of(dist0, base_guard), rem0 < div0 - rem0))
    } else {
        None
    };
    let mut guard = base_guard;
    loop {
        if guard >= max_guard {
            // Cap reached without a confirmed deciding term. An unconfirmed
            // signal from the deepest probe gets ONE shifted confirm probe.
            if let Some((pp, ps)) = pending {
                let back = ZIV_RESOLVE_FLOOR_POW10 + 3;
                if max_guard > base_guard + back {
                    let g_c = max_guard - back;
                    let (neg, q, rem, div) = probe(g_c);
                    let dist = if rem < div - rem { rem } else { div - rem };
                    if dist > floor {
                        let p = pos_of(dist, g_c);
                        if (rem < div - rem) == ps && p.abs_diff(pp) <= 1 {
                            return (dir_round(neg, q, rem), true);
                        }
                    }
                }
            }
            // Still unresolved: the deciding digit is beyond reach. SNAP the
            // base probe to its nearest grid line — the sub-floor remainder is
            // kernel noise around it (the undershoot that would otherwise
            // leave the narrowing one ULP short) — then apply the
            // `never_exact` sub-resolution rule on the grid value: the true
            // value carries a strictly positive tail below the computed
            // digits, so Ceiling (positive result) / Floor (negative) nudge
            // one ULP off the line; Trunc keeps it. A non-`never_exact`
            // caller keeps the bare grid line.
            let q_grid = if rem0 > div0 / lit(2) { q0 + lit(1) } else { q0 };
            let tail_bump = never_exact
                && match mode {
                    RoundingMode::Trunc => false,
                    RoundingMode::Floor => neg0,
                    RoundingMode::Ceiling => !neg0,
                    _ => unreachable!(),
                };
            return (finish(neg0, q_grid, tail_bump), false);
        }
        let step = (target + base_guard).max(base_guard);
        let next_guard = guard.saturating_add(step).min(max_guard);
        let (neg, q, rem, div) = probe(next_guard);
        let hi_dist = if rem < div - rem { rem } else { div - rem };
        if hi_dist > floor {
            let p = pos_of(hi_dist, next_guard);
            let above = rem < div - rem;
            if let Some((pp, ps)) = pending {
                if above == ps && p.abs_diff(pp) <= 1 {
                    // Confirmed deciding digit — trustworthy.
                    return (dir_round(neg, q, rem), true);
                }
            }
            pending = Some((p, above));
        }
        guard = next_guard;
    }
}

/// Two-width near-min narrowing for `cosh` / `exp`: resolve the near-tie at the
/// tier work integer `S1`; if its deciding digit was unreachable there (and a
/// wider integer would reach further, i.e. `S1` is below the precision
/// horizon), retry at the next-wider `S2`. A tie unresolved at both widths
/// falls to the never-exact endgame (the sub-resolution positive tail).
/// `never_exact` mirrors the `exp` sub-resolution rule.
#[inline]
#[allow(clippy::too_many_arguments)]
pub(crate) fn round_to_storage_widening_g<St: BigInt + Copy, S1: BigInt, S2: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    never_exact: bool,
    st_max: St,
    st_min: St,
    recompute1: impl FnMut(u32) -> S1,
    recompute2: impl FnMut(u32) -> S2,
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (v1, resolved1) =
        near_min_resolve_g::<St, S1>(base_guard, target, mode, never_exact, st_max, st_min, recompute1);
    // `S1` only proves a residual is past the probe horizon when its width
    // actually reaches it — and a result carrying integer digits raises both
    // the horizon AND the noise floor by that count (see the resolver), so
    // the reach test must include them.
    let int_digits = {
        let m = if v1 < <St as BigInt>::ZERO { -v1 } else { v1 };
        let bl = <St as BigInt>::BITS - m.leading_zeros();
        ((bl as u64 * 30103 / 100_000) as u32 + 1).saturating_sub(target)
    };
    if resolved1 || (<S1>::BITS / 8) >= ZIV_PRECISION_HORIZON + int_digits {
        return v1;
    }
    near_min_resolve_g::<St, S2>(base_guard, target, mode, never_exact, st_max, st_min, recompute2).0
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

    let floor = pow10(ZIV_RESOLVE_FLOOR_POW10);
    if is_nearest_mode(mode) {
        // Round to nearest at a working scale `target + guard`, reporting the
        // sub-storage residual's distance to the half-ULP boundary
        // (`dist_half`). A round-to-nearest decision is trustworthy only once
        // `dist_half` exceeds the ABSOLUTE kernel-noise floor — a genuine
        // deciding digit (`exp(1e-14)`'s `x³/6`, `cosh(1e-28)`'s `x⁴/24`, both
        // just past an exact half). While `dist_half` sits inside the floor the
        // residual is the kernel's own working-scale rounding noise, not a real
        // deciding digit, so the narrowing is a Table-Maker's-Dilemma tie.
        let mut nearest_narrow = |guard: u32| -> (St, S) {
            let w = target + guard;
            let v = recompute(guard);
            let narrowed = round_to_storage_with(v, w, target, mode);
            let mag = if v < lit(0) { -v } else { v };
            let divisor = pow10(guard);
            let rem = mag.div_rem(divisor).1;
            let half = divisor / lit(2);
            let dist_half = if rem < half { half - rem } else { rem - half };
            (narrowed, dist_half)
        };
        let (lo, dist0) = nearest_narrow(base_guard);
        // Ordinary input — residual clear of the half boundary by more than the
        // (generous) `divisor/1000` near-tie band — keep the single base
        // narrowing (bit-identical to the prior single-shot path). The escalate
        // trigger stays the wide band; the absolute `floor` below is only the
        // STOP test (signal vs noise), not the escalate trigger.
        if !force_confirm && dist0 > pow10(base_guard) / lit(1000) {
            return lo;
        }
        let int_digits = {
            let n = BigInt::resize_to::<S>(lo);
            let m = if n < lit(0) { -n } else { n };
            let bl = bit_length(m);
            let storage_digits = (bl as u64 * 30103 / 100_000) as u32 + 1;
            storage_digits.saturating_sub(target)
        };
        let cap_digits = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        let max_guard = cap_digits
            .saturating_sub(target)
            .min(ZIV_PRECISION_HORIZON.saturating_sub(target))
            .max(base_guard);
        let mut guard = base_guard;
        let mut best = lo;
        loop {
            if guard >= max_guard {
                // Cap reached without clearing the noise floor. `force_confirm`
                // (acosh/atanh) trusts its last stable narrowing; otherwise the
                // deciding digit is below the work integer's / the crate's reach
                // — return the CLEAN base narrowing (the exact-tie answer the
                // finite-precision oracle agrees with), NOT the deepest
                // narrowing (which is dominated by kernel noise at this depth).
                return if force_confirm { best } else { lo };
            }
            let step = (target + base_guard).max(base_guard);
            let next_guard = guard.saturating_add(step).min(max_guard);
            let (hi, hi_dist) = nearest_narrow(next_guard);
            if force_confirm {
                if hi == best {
                    return best;
                }
            } else if hi_dist > floor {
                // Deciding digit is now a clear signal above the noise floor —
                // this narrowing is trustworthy.
                return hi;
            }
            guard = next_guard;
            best = hi;
        }
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
