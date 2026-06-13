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

    /// Tang/Series-ROUTED working-scale natural log on the wide
    /// composition integer [`Self::Wagm`] — the per-tier
    /// `ln_fixed_routed_agm` (Tang where `policy::ln::is_tang` routes
    /// it, Series otherwise; the per-tier Tang CAP is a macro literal,
    /// which is why this is a trait binding rather than a free generic).
    /// Consumed by the acosh / atanh canonical kernels.
    fn ln_fixed_routed_agm<const SCALE: u32>(v_w: Self::Wagm, w: u32) -> Self::Wagm;

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

/// Rung-generic `exp_strict` — the Series exp kernel run at an
/// arbitrary work rung `Wk` (decoupled from `C::W`; mirrors
/// [`sin_series_g`]). Identical pins ([`exp_near_min_pin`], the zero
/// pin) and the identical two-width near-min widening as
/// [`exp_series`], with the PRIMARY width swapped from `C::W` to the
/// rung: each Ziv probe regime-splits on the exact
/// [`crate::algos::exp::exp_generic::exp_peak_fits`] model (the same
/// gate the per-tier `exp_fixed` routes `exp_fixed_wide` with) — the
/// fast path runs `exp_fixed::<Wk>`, a probe whose internal
/// squaring/`2^k` peak outgrows the rung lifts to the tier's `C::Wexp`
/// and narrows the (always-rung-representable) probe VALUE back. The
/// widening RETRY stays at `C::Wexp`, unchanged: a near-tie unresolved
/// at the rung's smaller cap retries there, reaching at least the
/// tier's resolution depth. The policy gate bounds `|x|` (the
/// result-magnitude axis, `work_rung::EXP_ARG_BUDGET`) so the everyday
/// region stays on the rung's fast path.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn exp_series_g<C: WideTrigCore, Wk: BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    if let Some(r) = exp_near_min_pin::<C, SCALE>(raw, mode) {
        return r;
    }
    round_to_storage_widening_g::<C::Storage, Wk, C::Wexp>(
        C::GUARD,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let v = to_work_scaled_g::<C::Storage, Wk>(raw, guard);
            if eg::exp_peak_fits::<Wk>(v, w) {
                eg::exp_fixed::<Wk>(v, w)
            } else {
                eg::resize_or_panic::<C::Wexp, Wk>(eg::exp_fixed::<C::Wexp>(
                    to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard),
                    w,
                ))
            }
        },
        |guard| {
            eg::exp_fixed::<C::Wexp>(
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

/// Analytic directed rounding for the tiny-argument linear band of the odd
/// forward/inverse trig functions whose Maclaurin series is
/// `f(x) = x + c·x³ + …` with `|c| ≤ 1/3` (`sin`, `tan`, `atan`, `asin`).
///
/// For `|x| < 10^(−⌈SCALE/3⌉)` — i.e. `|raw| ≤ 10^(SCALE − ⌈SCALE/3⌉)` in
/// storage units — the cubic correction `|c·x³|` stays below half a storage
/// ULP, so `f(x)` rounds to exactly `x` (= `raw`) under the nearest modes
/// while sitting a STRICTLY-signed sub-ULP cubic tail off the grid line: the
/// directed result is then determined analytically by the tail's sign,
/// EXPANDING (`|f| > |x|`: `tan` `+x³/3`, `asin` `+x³/6`) or COMPRESSING
/// (`|f| < |x|`: `sin` `−x³/6`, `atan` `−x³/3`).
///
/// This is the wide-tier sibling of the narrow tier's
/// `small_x_linear_threshold` + [`tiny_odd_expanding_directed`] /
/// [`tiny_odd_compressing_directed`] pair
/// (`crate::support::rounding`). At a wide tier the cubic deciding digit
/// sits at fractional depth `≈ 3·SCALE`, far beyond the work integer's Ziv
/// escalation reach, so the directed walker cannot resolve it and falls to a
/// mode-blind grid value (the comprehensive-gate wrong-mode find: `tan`/`sin`
/// of `1e-117`/`3e-60`-class arguments at D153/D307/D616). The analytic
/// decision is the only correct source there. Returns `Some(result)` for a
/// directed mode with `raw` inside the band; `None` otherwise (nearest modes,
/// `SCALE == 0`, `raw == 0`, or `|raw|` outside the band) — the caller then
/// runs its normal kernel, unchanged.
#[inline]
pub(crate) fn tiny_x_linear_directed<St: BigInt, const SCALE: u32>(
    raw: St,
    mode: RoundingMode,
    expanding: bool,
) -> Option<St> {
    // Only the directed modes are at risk; the nearest modes round to `raw`,
    // which the kernel already produces in-band. `SCALE == 0` has no sub-unit
    // linear band (only `x = 0` is "tiny", and that is the kernel's exact pin).
    if crate::support::rounding::is_nearest_mode(mode) || SCALE == 0 {
        return None;
    }
    let zero = <St as BigInt>::ZERO;
    if raw == zero {
        return None; // f(0) is the kernel's exact-zero pin
    }
    let absr = if raw < zero { zero - raw } else { raw };
    // The small-x linear band exponent, conservative by one digit (matches the
    // narrow `small_x_linear_threshold`): `|raw| ≤ 10^(SCALE − ⌈SCALE/3⌉)`.
    let thresh_exp = SCALE - SCALE.div_ceil(3);
    // One table-read + one compare exits for every normal-magnitude argument.
    if absr > crate::consts::pow10::dispatch::<St>(thresh_exp) {
        return None;
    }
    // `one` is ONE STORAGE ULP (the integer `1`), the step the directed
    // decision adds/drops — NOT `10^SCALE` (the value 1.0).
    let one = <St as BigInt>::ONE;
    Some(if expanding {
        crate::support::rounding::tiny_odd_expanding_directed(raw, zero, one, mode)
    } else {
        crate::support::rounding::tiny_odd_compressing_directed(raw, zero, one, mode)
    })
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
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if raw == C::storage_zero() {
        return C::storage_zero(); // tan(0) = 0, the sole exact point
    }
    let w0 = SCALE + C::GUARD;
    let (sin0, cos0) = C::sin_cos_fixed::<SCALE>(C::to_work(raw), w0);
    if cos0 == C::zero() {
        panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let probe = C::div(sin0, cos0, w0);
    let extra = crate::algos::trig::near_pole_tan::tan_extra_digits(C::bit_length(probe), w0)
        .saturating_sub(C::GUARD);
    if extra == 0 {
        // Near-tie escape: a fixed-w single shot cannot see a deciding
        // digit below w (`tan(x) = x + x^3/3 + ...` lands an exact
        // rational partial on a boundary with the deciding tail deeper -
        // the asin(3e-60) family). Clear-of-band residuals keep the
        // single-shot cost; the band escalates through the walker.
        if let Some(st) = round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
            probe, w0, SCALE, mode, C::storage_max(), C::storage_min(),
        ) {
            return st;
        }
        return tan_walker::<C, SCALE>(raw, C::GUARD, mode);
    }
    let w = w0 + extra;
    let (sin_w, cos_w) = C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, C::GUARD + extra), w);
    let r = C::div(sin_w, cos_w, w);
    if let Some(st) = round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
        r, w, SCALE, mode, C::storage_max(), C::storage_min(),
    ) {
        return st;
    }
    tan_walker::<C, SCALE>(raw, C::GUARD + extra, mode)
}

/// The tier-width Ziv walker for `tan` near a rounding boundary: the
/// ratio recomputed per probe at `w = SCALE + guard`, escalating from
/// the (near-pole-lifted) `base_guard`. Reached only from the near-tie
/// band of the single-shot terminals above / in the rung kernel.
fn tan_walker<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    base_guard: u32,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let (s, c) = C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, guard), w);
        if c == C::zero() {
            panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
        }
        C::div(s, c, w)
    })
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
    // Ziv-escalated narrowing from the band guard (NOT a single shot):
    // `atan(x) = x - x^3/3 + ...` lands an exact rational partial on a
    // rounding boundary with the deciding tail below the band's fixed
    // working scale (the asin(3e-60) family). The walker's base probe is
    // the same single evaluation; clear-of-band inputs exit there.
    C::round_to_storage_directed(GUARD, SCALE, mode, &mut |guard| {
        C::atan_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
    })
}

/// Rung-generic `sin_strict` — the forward-trig Series kernel run at an
/// arbitrary work rung `Wk` (decoupled from `C::W`), so the policy can
/// run it at the minimal valid work width for low-scale cells (mirrors
/// [`ln_series_g`]; the tier-width [`sin_series`] keeps the trait-bound
/// realisation, value-identical — the integer ops are width-agnostic, so
/// the only divergence surface is the Ziv cap `Wk::BITS/8`, budgeted by
/// `policy::work_rung::trig_rung`). `GUARD` is the base guard: the tier
/// `GUARD` (30) on the Series cells, the band guard (8/10) on the
/// narrow-GUARD band cells — one kernel serves both shapes (the explicit
/// `raw == 0` pin is value-identical to the unpinned tier path: the
/// kernel computes the exact grid value either way).
///
/// `π` at the rung comes from the same per-scale constant table as the
/// per-tier `pi_cf` (`pi_by_scale` keyed on the const `SCALE + GUARD` on
/// the hot path — value-identical to `pi_by_working_scale` at the same
/// scale, only the const-fold seam differs).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn sin_series_g<C: WideTrigCore, Wk: BigInt, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    // Two-width fall-up: an unresolved-at-rung-cap near-tie reruns the
    // walker at the tier work width `C::W` (the recompute closure is the
    // tier kernel's, verbatim), so the conclusion is never weaker than
    // the tier path's — see `round_to_storage_directed_widening_g`.
    let r = round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
        GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            crate::algos::trig::trig_generic::sin_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard),
                w,
                pi_at_rung::<Wk>(w, SCALE + GUARD),
            )
        },
        |guard| C::sin_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard),
    );
    adjust_bounded_extremum::<C, SCALE>(r, raw, mode)
}

/// Rung-generic `cos_strict` — see [`sin_series_g`]. Standalone
/// `cos_fixed` path (cofunction identity, one `sin_fixed`, no sqrt).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn cos_series_g<C: WideTrigCore, Wk: BigInt, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    // Two-width fall-up — see [`sin_series_g`].
    let r = round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
        GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            crate::algos::trig::trig_generic::cos_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard),
                w,
                pi_at_rung::<Wk>(w, SCALE + GUARD),
            )
        },
        |guard| C::cos_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard),
    );
    adjust_bounded_extremum::<C, SCALE>(r, raw, mode)
}

/// Rung-generic `tan_strict` — see [`sin_series_g`]. One kernel covers
/// the two existing tan shapes, preserved bit-for-bit per call site:
///
/// - `NEAR_POLE = true, SUB_GUARD = true` — the tier-`GUARD` Series
///   shape ([`tan_series`]): the base probe sizes a per-call lift
///   (`near_pole_tan::tan_extra_digits`) MINUS the guard already paid.
/// - `NEAR_POLE = true, SUB_GUARD = false` — the narrow-band shape
///   (`sincos_narrow::tan_narrow_with_taylor` with its probe): the full
///   per-call lift on top of the band guard.
/// - `NEAR_POLE = false` — the band shape without the probe (the band
///   guard already covers the worst case): one divide + one narrowing.
///
/// The rare near-pole recompute (`extra > 0`) runs at the TIER work
/// width `C::W` — capacity for the unbounded per-call lift is exactly
/// what the tier `$Work` is sized for, and the probe value (hence
/// `extra`) is bit-identical at the rung, so the recompute reproduces
/// the tier path exactly.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn tan_series_g<
    C: WideTrigCore,
    Wk: BigInt,
    const SCALE: u32,
    const GUARD: u32,
    const NEAR_POLE: bool,
    const SUB_GUARD: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;

    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    let w0 = SCALE + GUARD;
    let (sin0, cos0) = crate::algos::trig::trig_generic::sin_cos_fixed::<Wk>(
        to_work_scaled_g::<C::Storage, Wk>(raw, GUARD),
        w0,
        pi_at_rung::<Wk>(w0, w0),
    );
    if cos0 == eg::zero::<Wk>() {
        panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let probe = eg::div::<Wk>(sin0, cos0, w0);
    if !NEAR_POLE {
        // Near-tie escape — see [`tan_series`]: clear-of-band residuals
        // keep the single-shot cost; the band escalates (rung first,
        // tier fall-up).
        if let Some(st) = round_to_storage_clear_of_tie_g::<C::Storage, Wk>(
            probe, w0, SCALE, mode, C::storage_max(), C::storage_min(),
        ) {
            return st;
        }
        return tan_walker_rung_g::<C, Wk, SCALE>(raw, GUARD, mode);
    }
    let extra_raw =
        crate::algos::trig::near_pole_tan::tan_extra_digits(eg::bit_length::<Wk>(probe), w0);
    let extra = if SUB_GUARD { extra_raw.saturating_sub(GUARD) } else { extra_raw };
    if extra == 0 {
        if let Some(st) = round_to_storage_clear_of_tie_g::<C::Storage, Wk>(
            probe, w0, SCALE, mode, C::storage_max(), C::storage_min(),
        ) {
            return st;
        }
        return tan_walker_rung_g::<C, Wk, SCALE>(raw, GUARD, mode);
    }
    // Near-pole recompute at the tier work width (the `w` here is off the
    // hot `SCALE + GUARD` path, so π comes from the runtime-keyed table —
    // exactly the per-tier `pi_cf` fallback the tier path takes).
    let w = w0 + extra;
    let (sin_w, cos_w) = crate::algos::trig::trig_generic::sin_cos_fixed::<C::W>(
        to_work_scaled_g::<C::Storage, C::W>(raw, GUARD + extra),
        w,
        crate::consts::pi_by_working_scale::<C::W>(
            w,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        ),
    );
    let r = eg::div::<C::W>(sin_w, cos_w, w);
    if let Some(st) = round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
        r, w, SCALE, mode, C::storage_max(), C::storage_min(),
    ) {
        return st;
    }
    tan_walker::<C, SCALE>(raw, GUARD + extra, mode)
}

/// Two-width near-tie walker for the rung `tan` shapes: the ratio
/// recomputed per probe at the rung `Wk` (π from the same per-scale
/// table), an unresolved-at-rung-cap walk falling up to the tier-width
/// [`tan_walker`] closure. Reached only from the near-tie band.
#[cfg(feature = "_wide-support")]
fn tan_walker_rung_g<C: WideTrigCore, Wk: BigInt, const SCALE: u32>(
    raw: C::Storage,
    base_guard: u32,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    let base_w = SCALE + base_guard;
    round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
        base_guard,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let (s, c) = crate::algos::trig::trig_generic::sin_cos_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard),
                w,
                pi_at_rung::<Wk>(w, base_w),
            );
            if c == eg::zero::<Wk>() {
                panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
            }
            eg::div::<Wk>(s, c, w)
        },
        |guard| {
            let w = SCALE + guard;
            let (s, c) = C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, guard), w);
            if c == C::zero() {
                panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
            }
            C::div(s, c, w)
        },
    )
}

/// Rung-generic `atan_strict` — the inverse-tangent kernel run at an
/// arbitrary work rung `Wk` (decoupled from `C::W`; mirrors
/// [`sin_series_g`]). One kernel covers the two existing tier shapes,
/// preserved value-for-value per call site:
///
/// - `DIRECTED = true` — the tier-`GUARD` Ziv shape ([`atan_series`]):
///   directed narrowing with escalation at the rung.
/// - `DIRECTED = false` — the narrow-band single-shot shape
///   ([`atan_narrow`], band `GUARD` 10/12): one kernel evaluation at
///   `w = SCALE + GUARD`, one narrowing, no escalation (the band guard
///   leaves the working error far below half a storage ULP).
///
/// `π` at the rung comes from the same per-scale constant table as the
/// per-tier `pi_cf` (`pi_by_scale` keyed on the const `SCALE + GUARD` on
/// the hot path) — and only its `π/2` half is consumed, by the `|x| > 1`
/// reciprocal-fold complement. Unlike sin/cos there is NO precision loss
/// proportional to `digits(|x|)` (no mod-τ cancellation); the `|x|` axis
/// is purely the lift's representation capacity, gated by the policy
/// (`forward_rung::atan_strict`).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn atan_series_g<
    C: WideTrigCore,
    Wk: BigInt,
    const SCALE: u32,
    const GUARD: u32,
    const DIRECTED: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if DIRECTED {
        // Two-width fall-up — see [`sin_series_g`].
        round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
            GUARD,
            SCALE,
            mode,
            C::storage_max(),
            C::storage_min(),
            |guard| {
                let w = SCALE + guard;
                crate::algos::trig::trig_generic::atan_fixed::<Wk>(
                    to_work_scaled_g::<C::Storage, Wk>(raw, guard),
                    w,
                    pi_at_rung::<Wk>(w, SCALE + GUARD),
                )
            },
            |guard| C::atan_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard),
        )
    } else {
        // Band shape: same Ziv-escalated two-width walker, from the band
        // guard (the single-shot it replaces could not see a deciding
        // digit below the band's fixed working scale — see
        // [`atan_narrow`]). `DIRECTED` still selects the policy-side
        // out-of-budget fallback kernel; the narrowing machinery is one.
        let base_w = SCALE + GUARD;
        round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
            GUARD,
            SCALE,
            mode,
            C::storage_max(),
            C::storage_min(),
            |guard| {
                let w = SCALE + guard;
                crate::algos::trig::trig_generic::atan_fixed::<Wk>(
                    to_work_scaled_g::<C::Storage, Wk>(raw, guard),
                    w,
                    pi_at_rung::<Wk>(w, base_w),
                )
            },
            |guard| C::atan_fixed::<SCALE>(C::to_work_scaled(raw, guard), SCALE + guard),
        )
    }
}

/// `π` at working scale `w` in the rung integer `Wk`: the per-scale
/// constant table keyed on the CONST base working scale on the hot path
/// (`w == base_w`, const-folds per monomorphisation — the rung sibling
/// of the per-tier `pi_cf`), the runtime-keyed lookup on the Ziv
/// escalation path. Value-identical either way (same table entry).
#[cfg(feature = "_wide-support")]
#[inline]
pub(crate) fn pi_at_rung<Wk: BigInt>(w: u32, base_w: u32) -> Wk {
    if w == base_w {
        crate::consts::pi_by_scale::<Wk>(base_w, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    } else {
        crate::consts::pi_by_working_scale::<Wk>(
            w,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }
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
    // `10^d` first: the truncated-low schoolbook (`mul_low_fixed`) skips the
    // zero limbs of its FIRST operand, and `10^d` is the sparse one (a guard
    // of <= 38 digits spans 1-2 limbs while the lifted value spans the whole
    // storage width), so the lift costs one inner row per `10^d` limb instead
    // of one per value limb. The wrapping low product is commutative —
    // bit-identical to the previous `resize * pow10` order.
    crate::algos::exp::exp_generic::pow10::<S>(working_digits) * BigInt::resize_to::<S>(raw)
}

/// Narrow a working-scale `signed` value (in the work int `S`) to storage
/// `St`, panicking when it exceeds the storage range. When `S` is NARROWER
/// than `St` (the work-rung case — a rung below the storage width admitted
/// by the trig magnitude gate) every `S`-representable value fits the wider
/// storage, so the bounds check is vacuously true and skipped — `st_max` /
/// `st_min` cannot even be represented in `S` (a down-resize would truncate
/// their magnitude into garbage bounds). The `LIMBS` compare const-folds per
/// monomorphisation.
#[inline]
fn narrow_range_checked_g<St: BigInt + Copy, S: BigInt>(signed: S, st_max: St, st_min: St) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if <S as BigInt>::LIMBS >= <St as BigInt>::LIMBS {
        let max_w = BigInt::resize_to::<S>(st_max);
        let min_w = BigInt::resize_to::<S>(st_min);
        if signed > max_w || signed < min_w {
            panic!("wide-tier strict transcendental: result out of range");
        }
    }
    // Down-resize through `S`'s EXACT scratch (`unpack_mag` +
    // `from_mag_sign_u64`), not the width-erased `resize_to` blanket: the
    // narrow default build sizes that blanket's `MAX_U128_LIMB` buffer to
    // its 2-limb storage, below the `Int<24>` work integer the narrow
    // near-tie walkers run in. The up-resizes above are from the SMALLER
    // `St` (its own width bounds the blanket buffer), so they stay.
    let neg = signed < <S as BigInt>::ZERO;
    let mag = if neg { -signed } else { signed };
    let mut buf =
        <S::Scratch as crate::int::types::compute_limbs::ComputeLimbs>::single_u64();
    crate::algos::exp::exp_generic::unpack_mag(mag, buf.as_mut());
    St::from_mag_sign_u64(buf.as_ref(), neg)
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
    narrow_range_checked_g::<St, S>(rounded, st_max, st_min)
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
/// `recompute` is a `&mut dyn FnMut` trait object, NOT an `impl FnMut`
/// generic: the walker body is large and every distinct closure type would
/// mint a full copy of it per call site (the dominant IR-volume entry at
/// the wide gate builds), while a trait object keeps ONE instantiation per
/// `(St, S)` pair. The dyn indirection is perf-acceptable — each
/// `recompute` call evaluates a whole transcendental kernel at the probed
/// guard, and the walker itself sits on the rare Ziv escalation path (the
/// hot path narrows single-shot and never enters it) — and it matches the
/// `WideTrigCore` trait surface, which already passes `&mut dyn FnMut`.
#[allow(clippy::too_many_arguments)]
fn near_min_resolve_g<St: BigInt + Copy, S: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    never_exact: bool,
    st_max: St,
    st_min: St,
    recompute: &mut dyn FnMut(u32) -> S,
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
    let range_check = |signed: S| -> St { narrow_range_checked_g::<St, S>(signed, st_max, st_min) };
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
        // Exact per-width Knuth scratch (the narrow build's blanket is sized
        // to its 2-limb storage; the walkers probe in `Int<24>`).
        let (q, rem) = crate::algos::exp::exp_generic::div_rem_exact(mag, divisor);
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
    mut recompute1: impl FnMut(u32) -> S1,
    mut recompute2: impl FnMut(u32) -> S2,
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (v1, resolved1) = near_min_resolve_g::<St, S1>(
        base_guard, target, mode, never_exact, st_max, st_min, &mut recompute1,
    );
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
    near_min_resolve_g::<St, S2>(
        base_guard, target, mode, never_exact, st_max, st_min, &mut recompute2,
    )
    .0
}

/// Single-shot narrowing with a NEAR-TIE escape hatch. Rounds a
/// working-scale value `v` (at scale `w`) to storage exactly as
/// [`round_to_storage_with_g`] would — PROVIDED the sub-storage residual
/// is clear of the mode's deciding boundary (the half-ULP line for the
/// nearest modes, the grid line for the directed ones) by more than the
/// near-tie band (`divisor/1000`, the shared Ziv escalate trigger).
/// Returns `None` when the residual sits inside the band: the value's
/// TRUE deciding digit may then lie below `w`'s resolution (the
/// `asin(3·10⁻⁶⁰)` family — an exact rational partial sum landing
/// exactly ON a boundary with a transcendental tail below the fixed
/// working scale), and the caller must escalate through the full Ziv
/// walker instead of concluding from this single shot. One `div_rem` —
/// the clear path costs what the plain narrowing cost.
#[inline]
pub(crate) fn round_to_storage_clear_of_tie_g<St: BigInt + Copy, S: BigInt>(
    v: S,
    w: u32,
    target: u32,
    mode: RoundingMode,
    st_max: St,
    st_min: St,
) -> Option<St>
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::support::rounding::{is_nearest_mode, should_bump};
    let lit = |n: i128| <S as BigInt>::from_i128(n);
    let shift = w - target;
    if shift == 0 {
        // Already at storage scale: the value IS the answer (no residual).
        return Some(narrow_range_checked_g::<St, S>(v, st_max, st_min));
    }
    let neg = v < lit(0);
    let mag = if neg { -v } else { v };
    let divisor = crate::consts::pow10::dispatch::<S>(shift);
    let (q, rem) = mag.div_rem(divisor);
    let band = if shift >= 3 {
        crate::consts::pow10::dispatch::<S>(shift - 3)
    } else {
        lit(0)
    };
    let bump = if is_nearest_mode(mode) {
        // Distance to the half-ULP boundary (divisor is even for shift >= 1).
        let half = divisor >> 1;
        let dist = if rem < half { half - rem } else { rem - half };
        if dist <= band {
            return None;
        }
        rem != lit(0)
            && should_bump(mode, rem.cmp(&(divisor - rem)), q.bit(0), !neg)
    } else {
        // Distance to the grid line.
        let dist = if rem < divisor - rem { rem } else { divisor - rem };
        if dist <= band {
            return None;
        }
        rem != lit(0)
            && match mode {
                RoundingMode::Trunc => false,
                RoundingMode::Floor => neg,
                RoundingMode::Ceiling => !neg,
                _ => unreachable!(),
            }
    };
    let q_mag = if bump { q + lit(1) } else { q };
    let signed = if neg { -q_mag } else { q_mag };
    Some(narrow_range_checked_g::<St, S>(signed, st_max, st_min))
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
    mut recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(
        base_guard, target, mode, false, false, st_max, st_min, &mut recompute,
    )
    .0
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
    mut recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(
        base_guard, target, mode, false, true, st_max, st_min, &mut recompute,
    )
    .0
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
    mut recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(
        base_guard, target, mode, true, false, st_max, st_min, &mut recompute,
    )
    .0
}

/// Two-width directed narrowing for the SCALE-derived work rungs: resolve
/// at the rung `S1`; if the walker reaches `S1`'s escalation cap
/// UNRESOLVED (the deciding digit lies beyond the rung's reach but
/// possibly within the tier's), rerun the WHOLE walker at the wider tier
/// work integer `S2`, so the conclusion is never weaker than the tier
/// path's. The directed/nearest twin of the exp near-min
/// [`round_to_storage_widening_g`] retry, and the rung families' fix for
/// the at-cap base-narrowing endgame: an unresolved-at-rung tie used to
/// conclude from the rung's probes, which under a DIRECTED mode can land
/// one ULP on the wrong side of a sub-rung-resolution residual the tier
/// width resolves (the `sin_d307_s153` Trunc defect — `sin(x) = x − x³/6`
/// with the cube term between the two caps). A resolved-at-rung value is
/// a CONFIRMED deciding digit the tier walker would find identically (the
/// rung cap never exceeds the tier cap), and an unresolved cell reruns
/// the tier walker verbatim — so the result is bit-identical to the tier
/// path in every case, with the (overwhelmingly common) resolved fast
/// path unchanged.
#[inline]
#[allow(clippy::too_many_arguments)]
pub(crate) fn round_to_storage_directed_widening_g<St: BigInt + Copy, S1: BigInt, S2: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    st_max: St,
    st_min: St,
    mut recompute1: impl FnMut(u32) -> S1,
    mut recompute2: impl FnMut(u32) -> S2,
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (v, resolved) = round_to_storage_directed_impl_g::<St, S1>(
        base_guard, target, mode, false, false, st_max, st_min, &mut recompute1,
    );
    if resolved || <S1 as BigInt>::BITS >= <S2 as BigInt>::BITS {
        return v;
    }
    round_to_storage_directed_impl_g::<St, S2>(
        base_guard, target, mode, false, false, st_max, st_min, &mut recompute2,
    )
    .0
}

/// Near-special two-width narrowing — the `force_confirm` sibling of
/// [`round_to_storage_directed_widening_g`] (`acosh` at 1, `atanh` at
/// ±1): an at-cap unconfirmed walk at the rung `S1` reruns at the tier
/// `S2`, never concluding shallower than the tier path.
#[inline]
#[allow(clippy::too_many_arguments)]
pub(crate) fn round_to_storage_directed_near_special_widening_g<
    St: BigInt + Copy,
    S1: BigInt,
    S2: BigInt,
>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    st_max: St,
    st_min: St,
    mut recompute1: impl FnMut(u32) -> S1,
    mut recompute2: impl FnMut(u32) -> S2,
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (v, resolved) = round_to_storage_directed_impl_g::<St, S1>(
        base_guard, target, mode, true, false, st_max, st_min, &mut recompute1,
    );
    if resolved || <S1 as BigInt>::BITS >= <S2 as BigInt>::BITS {
        return v;
    }
    round_to_storage_directed_impl_g::<St, S2>(
        base_guard, target, mode, true, false, st_max, st_min, &mut recompute2,
    )
    .0
}

/// `recompute` is a `&mut dyn FnMut` trait object, NOT an `impl FnMut`
/// generic — see [`near_min_resolve_g`]: one walker instantiation per
/// `(St, S)` instead of one per call-site closure type. Perf-acceptable
/// because the walker is the RARE Ziv escalation machinery (the hot path
/// narrows single-shot; each `recompute` call is a whole kernel
/// evaluation, dwarfing the indirect call), and it matches the
/// `WideTrigCore` trait surface, which already passes `&mut dyn FnMut`.
#[allow(clippy::too_many_arguments)]
fn round_to_storage_directed_impl_g<St: BigInt + Copy, S: BigInt>(
    base_guard: u32,
    target: u32,
    mode: RoundingMode,
    force_confirm: bool,
    never_exact: bool,
    st_max: St,
    st_min: St,
    recompute: &mut dyn FnMut(u32) -> S,
) -> (St, bool)
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
    let floor = pow10(ZIV_RESOLVE_FLOOR_POW10);
    // The near-tie band at guard `g` is `divisor/1000 = 10^(g-3)` — a table
    // lookup, not a work-integer divide (the divisor is always `10^g`).
    let band_of = |g: u32| -> S { if g >= 3 { pow10(g - 3) } else { lit(0) } };
    if is_nearest_mode(mode) {
        // Round to nearest at a working scale `target + guard`, reporting the
        // sub-storage residual's distance to the half-ULP boundary
        // (`dist_half`). A round-to-nearest decision is trustworthy only once
        // `dist_half` exceeds the ABSOLUTE kernel-noise floor — a genuine
        // deciding digit (`exp(1e-14)`'s `x³/6`, `cosh(1e-28)`'s `x⁴/24`, both
        // just past an exact half). While `dist_half` sits inside the floor the
        // residual is the kernel's own working-scale rounding noise, not a real
        // deciding digit, so the narrowing is a Table-Maker's-Dilemma tie.
        // ONE `div_rem` per narrowing: the quotient + remainder of the same
        // division yield BOTH the correctly-rounded value (the standard
        // `r` vs `m − r` comparison through `should_bump`, exactly
        // [`round_mag_with_mode`]'s rule — bit-identical to routing through
        // `round_to_storage_with`) AND the `dist_half` tie distance. The
        // previous shape divided twice (once inside `round_to_storage_with`,
        // once for the remainder) plus a divide-by-two for `half` — measured
        // at >50% of a wide `sin(0)` call (the bbc trig-s0 cluster).
        let mut nearest_narrow = |guard: u32| -> (St, S, S) {
            let v = recompute(guard);
            let neg = v < lit(0);
            let mag = if neg { -v } else { v };
            let divisor = pow10(guard);
            // Exact per-width Knuth scratch (the narrow build's blanket is sized
            // to its 2-limb storage; the walkers probe in `Int<24>`).
            let (q, rem) = crate::algos::exp::exp_generic::div_rem_exact(mag, divisor);
            let q_mag = if rem != lit(0) {
                let comp = divisor - rem;
                let bump = crate::support::rounding::should_bump(
                    mode,
                    rem.cmp(&comp),
                    q.bit(0),
                    !neg,
                );
                if bump { q + lit(1) } else { q }
            } else {
                q
            };
            let signed = if neg { -q_mag } else { q_mag };
            let narrowed = narrow_range_checked_g::<St, S>(signed, st_max, st_min);
            // `divisor = 10^guard` is even for every guard >= 1 (and the
            // guard-0 degenerate `1/2 == 1 >> 1 == 0`), so the half-ULP
            // boundary is an exact one-bit shift — not a divide.
            let half = divisor >> 1;
            let dist_half = if rem < half { half - rem } else { rem - half };
            (narrowed, dist_half, divisor)
        };
        let (lo, dist0, _divisor0) = nearest_narrow(base_guard);
        // Ordinary input — residual clear of the half boundary by more than the
        // (generous) `divisor/1000` near-tie band — keep the single base
        // narrowing (bit-identical to the prior single-shot path). The escalate
        // trigger stays the wide band; the absolute `floor` below is only the
        // STOP test (signal vs noise), not the escalate trigger.
        if !force_confirm && dist0 > band_of(base_guard) {
            return (lo, true);
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
                return (if force_confirm { best } else { lo }, false);
            }
            let step = (target + base_guard).max(base_guard);
            let unclamped = guard.saturating_add(step);
            let next_guard = unclamped.min(max_guard);
            // A probe whose depth was CLAMPED by this width's escalation cap
            // diverges from the canonical (tier-width) probe sequence — any
            // conclusion drawn from it is reported UNRESOLVED so a two-width
            // caller falls up to the tier walker instead of trusting a
            // cap-limited reading (e.g. a zero remainder that is only the
            // deciding term underflowing at the clamped working scale).
            let tainted = unclamped > max_guard;
            let (hi, hi_dist, _) = nearest_narrow(next_guard);
            if force_confirm {
                if hi == best {
                    return (best, !tainted);
                }
            } else if hi_dist > floor {
                // Deciding digit is now a clear signal above the noise floor —
                // this narrowing is trustworthy (at an unclamped, canonical
                // probe depth).
                return (hi, !tainted);
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
        // Exact per-width Knuth scratch (the narrow build's blanket is sized
        // to its 2-limb storage; the walkers probe in `Int<24>`).
        let (q, rem) = crate::algos::exp::exp_generic::div_rem_exact(mag, divisor);
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

    let (mut lo, dist0, _divisor0) = directed_narrow(base_guard);

    let band0 = band_of(base_guard);
    let near_grid = force_confirm || dist0 <= band0;

    let (signed, decided) = if !near_grid {
        (lo, true)
    } else {
        // The clean base narrowing — the unresolved endgame's answer (see
        // the cap break below). Captured BEFORE the loop starts rolling
        // `lo` through the deeper probes.
        let base = lo;
        let int_digits = {
            let m = if lo < lit(0) { -lo } else { lo };
            let bl = bit_length(m);
            let storage_digits = (bl as u64 * 30103 / 100_000) as u32 + 1;
            storage_digits.saturating_sub(target)
        };
        let cap_digits = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        let max_guard = cap_digits.saturating_sub(target).max(base_guard);

        let mut guard = base_guard;
        // Whether the LAST probe's grid-line distance cleared the absolute
        // noise floor — a genuine, representable deciding digit (a real
        // residual only GROWS with depth, so a final floor-clearing probe is
        // signal even without a second confirming probe; the asin(1e-38)
        // D38<38> Ceiling deviation `x³/6` at ULP-depth 77 first becomes
        // visible exactly at the rung's cap-clamped probe).
        let mut last_resolved = false;
        loop {
            if guard >= max_guard {
                // Cap reached. `force_confirm` (acosh/atanh) trusts its
                // last stable narrowing. A walk whose FINAL probe resolved
                // (cleared the noise floor) but had no deeper probe left to
                // confirm against trusts that probe — discarding it for the
                // base would invert a deciding digit first visible at the
                // cap. Otherwise — no probe ever cleared the floor (the
                // Table-Maker's-Dilemma residue) — return the CLEAN BASE
                // narrowing, mirroring the nearest branch's endgame: never
                // an unresolved deepest probe, which at this depth is
                // dominated by kernel noise, and at the cap-CLAMPED working
                // scale can even be a wrapped kernel value (the deep-
                // underflow `exp` probe's internal squaring peak tops the
                // work integer's sign bit, handing the walker a NEGATIVE
                // "e^x" that inverts the directed bump — the
                // exp(-62.175…) D38 s17–19 Ceiling/Floor inversion).
                break (if force_confirm || last_resolved { lo } else { base }, false);
            }
            let step = (target + base_guard).max(base_guard);
            let unclamped = guard.saturating_add(step);
            let next_guard = unclamped.min(max_guard);
            // See the nearest branch: a cap-clamped probe departs from the
            // canonical probe sequence, so its conclusion is reported
            // UNRESOLVED for the two-width fall-up.
            let tainted = unclamped > max_guard;
            let (hi, hi_dist, _hi_div) = directed_narrow(next_guard);
            // A deciding digit is a genuine SIGNAL once its distance to the
            // grid line clears the ABSOLUTE kernel-noise floor — the same
            // rule the nearest branch applies to its half-boundary
            // distance. The old relative `divisor/1000` band scales with
            // the working scale, so a SUB-RESOLUTION residual (e.g. a
            // deep-underflow `exp`, value ≪ 1 storage ULP) could never
            // clear it and every such walk ran to the cap — where the
            // deepest probe, not the clean base, used to be trusted.
            // Resolution still demands two consecutive probes agree on the
            // narrowing (`hi == lo`), a stricter consistency requirement
            // than the nearest branch's single floor-clearing probe.
            //
            // `hi_dist == lit(0)`: a probe landing EXACTLY on a grid line
            // (working-scale remainder == 0) at a depth ABOVE the true
            // deviation also counts as resolved when paired with `hi == lo`.
            // `directed_narrow` already handles the zero remainder correctly
            // (`residual_present = rem != lit(0) || never_exact` keeps the
            // directed bump active when `never_exact` is set), so `hi` IS
            // the right directed answer; `hi == lo` confirms a second,
            // independent depth reached the same conclusion. This is sound
            // for every CURRENT ladder: the step formula (`target +
            // base_guard`, ≥68 digits in the wide tiers) spans a 10^68×
            // depth gap, making coincidental paired exact-zero remainders
            // impossible unless the residual is genuinely sub-resolution to
            // the ZIV_PRECISION_HORIZON. A future ladder with a stride SHORT
            // enough to straddle a genuine residual would be pathological —
            // canonical form: stride k digits, true deviation at depth D+k,
            // so depth-D and depth-(D+k) both produce exact-zero remainders
            // while depth-(D+2k) would show the genuine non-zero residual;
            // the hi==lo+dist==0 pair would then fire early on an answer
            // that a proper confirming probe would overturn.
            let resolved = hi_dist == lit(0) || hi_dist > floor;
            if hi == lo && resolved {
                break (hi, !tainted);
            }
            guard = next_guard;
            lo = hi;
            last_resolved = resolved;
        }
    };

    (narrow_range_checked_g::<St, S>(signed, st_max, st_min), decided)
}

// Directed-walker contract tests: a strictly positive SUB-RESOLUTION
// residual (value ≪ 1 storage ULP — the deep-underflow `exp` shape) must
// round Ceiling → 1 ULP and Floor/Trunc/nearest → 0 under the
// `never_exact` walker, even when the deepest (cap-clamped) probe is
// POISONED — the stand-in for a work-integer-wrapped kernel value (the
// generic exp kernel's internal squaring peak can top the work integer's
// sign bit at the cap-clamped working scale, handing the walker a
// NEGATIVE "e^x"). Before the fix the directed branch could never resolve
// a sub-resolution residual (the relative `divisor/1000` stop band scales
// with the working scale) and at the cap trusted the DEEPEST probe — so
// the poisoned probe inverted Ceiling to 0 and Floor to -1.
#[cfg(test)]
mod directed_walker_contract {
    use super::*;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    type S = Int<24>;
    type St = Int<2>;
    const BASE_GUARD: u32 = 30;
    const TARGET: u32 = 17;

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Ceiling,
        RoundingMode::Floor,
        RoundingMode::Trunc,
    ];

    fn run(mode: RoundingMode, recompute: impl FnMut(u32) -> S) -> i128 {
        round_to_storage_directed_never_exact_g::<St, S>(
            BASE_GUARD,
            TARGET,
            mode,
            St::MAX,
            St::MIN,
            recompute,
        )
        .as_i128()
    }

    /// Negated probe value — the wrapped-kernel stand-in. Any probe at
    /// guard >= 150 is past every canonical (unclamped) depth for this
    /// `(TARGET, BASE_GUARD)`, i.e. only the cap-clamped probe lands
    /// there; poisoning the whole tail keeps the test robust if the
    /// step arithmetic ever changes.
    const POISON_FROM: u32 = 150;

    // A genuine sub-resolution positive (≈9.95e-28 storage ULPs): its
    // grid-line distance clears the ABSOLUTE kernel-noise floor at the
    // first escalation probe, so the walk resolves at a canonical depth
    // and the poisoned tail is never consulted.
    #[test]
    fn sub_resolution_positive_resolves_correctly_despite_poisoned_tail() {
        for mode in ALL_MODES {
            let got = run(mode, |g| {
                let v = <S as BigInt>::from_i128(995)
                    * crate::consts::pow10::dispatch::<S>(g - BASE_GUARD);
                if g >= POISON_FROM { -v } else { v }
            });
            let want = if mode == RoundingMode::Ceiling { 1 } else { 0 };
            assert_eq!(got, want, "sub-resolution positive, mode={mode:?}");
        }
    }

    // A residual at kernel-noise scale (5 work units, below the resolve
    // floor at EVERY depth — the Table-Maker's-Dilemma stand-in): the walk
    // runs to the cap unresolved, and the endgame must return the CLEAN
    // BASE narrowing (never-exact: Ceiling → 1, others → 0), never the
    // deepest probe's — which here is poisoned negative and would invert
    // Ceiling to 0 and Floor to -1.
    #[test]
    fn unresolved_cap_returns_clean_base_not_deepest_probe() {
        for mode in ALL_MODES {
            let got = run(mode, |g| {
                let v = <S as BigInt>::from_i128(5);
                if g >= POISON_FROM { -v } else { v }
            });
            let want = if mode == RoundingMode::Ceiling { 1 } else { 0 };
            assert_eq!(got, want, "noise-scale residual at cap, mode={mode:?}");
        }
    }

    // A deciding digit first visible ONLY at the cap-clamped final probe —
    // the asin(1e-38) D38<38> shape (CI fallout of the cap-endgame fix):
    // value = 1 ULP + 1.667e-77 ULPs, walked at the D57 borrow path's
    // Int<16> rung (max_guard = 128 − 8 − 38 = 82, first probe 30+68 = 98
    // clamped to 82). The base probe lands EXACTLY on grid (the deviation
    // is below w = 68's resolution); the single cap-clamped probe shows the
    // genuine residual (1.667e5 work units, above the noise floor) — the
    // endgame must TRUST that resolved final probe (Ceiling → 2), not
    // discard it for the on-grid base (Ceiling → 1, the regression).
    #[test]
    fn deciding_digit_first_visible_at_cap_probe_is_trusted() {
        type Rung = Int<16>;
        for (mode, want) in [
            (RoundingMode::Ceiling, 2_i128),
            (RoundingMode::Floor, 1),
            (RoundingMode::Trunc, 1),
            (RoundingMode::HalfToEven, 1),
        ] {
            let got = round_to_storage_directed_g::<St, Rung>(
                BASE_GUARD,
                TARGET_ASIN,
                mode,
                St::MAX,
                St::MIN,
                |g| {
                    // 10^g + ⌊1.667·10^(g−77)⌋ — the deviation appears at
                    // ULP-depth 77 (asin's x³/6 for x = 1e-38).
                    let one = crate::consts::pow10::dispatch::<Rung>(g);
                    if g >= 80 {
                        one + <Rung as BigInt>::from_i128(1667)
                            * crate::consts::pow10::dispatch::<Rung>(g - 80)
                    } else {
                        one
                    }
                },
            )
            .as_i128();
            assert_eq!(got, want, "late-visible deciding digit, mode={mode:?}");
        }
    }

    /// The asin-shape test's storage scale (D38<38>).
    const TARGET_ASIN: u32 = 38;
}

/// Wide-tier tiny-x DIRECTED rounding pins (the comprehensive-gate
/// wrong-mode find): for a sub-resolution-cubic argument the deciding
/// cubic digit sits at fractional depth `≈ 3·SCALE`, past the work
/// integer's Ziv reach, so the directed result is the analytic
/// [`tiny_x_linear_directed`] decision — EXPANDING (`tan`/`asin`,
/// `|f| > |x|`) pushes the magnitude OUT by one ULP, COMPRESSING
/// (`sin`/`atan`, `|f| < |x|`) pulls it IN — NOT the mode-blind grid value
/// the pre-fix narrowing returned. The three nearest modes round to `x`.
/// Covers both the Tang band (D153<76>) and the Series band (D153<152>),
/// each side of zero, through the public strict path.
#[cfg(all(test, any(feature = "d153", feature = "wide")))]
mod tiny_x_directed_pins {
    use crate::int::types::{traits::BigInt, Int};
    use crate::support::rounding::RoundingMode::{
        Ceiling, Floor, HalfAwayFromZero, HalfToEven, HalfTowardZero, Trunc,
    };

    const ULP: Int<8> = <Int<8> as BigInt>::ONE;

    /// Pin all six modes for one wide-tier `D153<SCALE>` function at the
    /// tiny argument `±3·10^(−KNEG)` (raw `±3·10^(SCALE−KNEG)`, built
    /// directly — the parser takes no scientific notation). `expanding`
    /// selects the cubic-tail direction: EXPANDING (`|f| > |x|`,
    /// `tan`/`asin`) pushes the magnitude OUT by one ULP only under the
    /// away-from-`x` directed mode (Ceiling for `+x`, Floor for `−x`);
    /// COMPRESSING (`|f| < |x|`, `sin`/`atan`) pulls it IN by one ULP under
    /// the toward-zero directed modes. The three nearest modes round to `x`.
    macro_rules! pin {
        ($scale:literal, $kneg:literal, $f:ident, $expanding:expr, $label:literal) => {{
            let r = Int::<8>::from_i128(3)
                * crate::consts::pow10::dispatch::<Int<8>>($scale - $kneg);
            let x = crate::D::<Int<8>, $scale>(r);
            let nr = -r;
            let nx = crate::D::<Int<8>, $scale>(nr);
            for m in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
                assert_eq!(x.$f(m).0, r, "{} (+x) {:?}", $label, m);
                assert_eq!(nx.$f(m).0, nr, "{} (−x) {:?}", $label, m);
            }
            if $expanding {
                assert_eq!(x.$f(Ceiling).0, r + ULP, "{} (+x) Ceiling", $label);
                assert_eq!(x.$f(Floor).0, r, "{} (+x) Floor", $label);
                assert_eq!(x.$f(Trunc).0, r, "{} (+x) Trunc", $label);
                assert_eq!(nx.$f(Floor).0, nr - ULP, "{} (−x) Floor", $label);
                assert_eq!(nx.$f(Ceiling).0, nr, "{} (−x) Ceiling", $label);
                assert_eq!(nx.$f(Trunc).0, nr, "{} (−x) Trunc", $label);
            } else {
                assert_eq!(x.$f(Floor).0, r - ULP, "{} (+x) Floor", $label);
                assert_eq!(x.$f(Trunc).0, r - ULP, "{} (+x) Trunc", $label);
                assert_eq!(x.$f(Ceiling).0, r, "{} (+x) Ceiling", $label);
                assert_eq!(nx.$f(Ceiling).0, nr + ULP, "{} (−x) Ceiling", $label);
                assert_eq!(nx.$f(Trunc).0, nr + ULP, "{} (−x) Trunc", $label);
                assert_eq!(nx.$f(Floor).0, nr, "{} (−x) Floor", $label);
            }
        }};
    }

    #[test]
    fn tan_expanding_d153_tang_and_series_bands() {
        // 3e-60 @ s76 → Tang band (70..=82); 3e-120 @ s152 → Series band —
        // both sit in the linear band so the analytic decision applies.
        pin!(76, 60, tan_strict_with, true, "tan s76");
        pin!(152, 120, tan_strict_with, true, "tan s152");
    }

    #[test]
    fn sin_compressing_d153_tang_and_series_bands() {
        pin!(76, 60, sin_strict_with, false, "sin s76");
        pin!(152, 120, sin_strict_with, false, "sin s152");
    }

    #[test]
    fn atan_compressing_d153_series_band() {
        pin!(152, 120, atan_strict_with, false, "atan s152");
    }

    #[test]
    fn asin_expanding_d153_series_band() {
        pin!(152, 120, asin_strict_with, true, "asin s152");
    }
}
