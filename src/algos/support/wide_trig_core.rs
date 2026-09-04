// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic surface over the per-tier wide guard-digit cores.
//!
//! Each wide decimal tier (D57 .. D1232) carries a guard-digit
//! transcendental core, emitted by `decl_wide_transcendental!` into a
//! `wide_trig_<tier>` module against a tier-specific work integer `W`
//! (e.g. `Int<16>` for D307) and tier-specific constant tables. The
//! per-family wide-tier kernels (`algos::{exp,ln,trig}::wide_kernel`)
//! run through six generic `*_series` functions ([`exp_series`],
//! [`ln_series`], [`sin_series`], [`cos_series`], [`tan_series`],
//! [`atan_series`]) instead of one thin `*_strict_<tier>` wrapper per tier
//! per function.
//!
//! [`WideTrigCore`] is the trait that backs those six generic functions.
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

use crate::algos::exp::exp_generic::TailSign;
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
        working_value: Self::W,
        working_scale: u32,
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
        working_value: Self::Wagm,
        working_scale: u32,
        target: u32,
        mode: RoundingMode,
    ) -> Self::Storage;
    /// Directed-rounding narrowing with Ziv escalation.
    /// `recompute(guard_digits)` returns the kernel value computed with
    /// that many guard digits.
    fn round_to_storage_directed(
        base_guard_digits: u32,
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
        base_guard_digits: u32,
        target: u32,
        mode: RoundingMode,
        recompute: &mut dyn FnMut(u32) -> Self::W,
    ) -> Self::Storage;

    // ── the per-tier guard-digit kernels ──────────────────────────────

    /// `e^v` for a `working_value` at `working_scale`. `SCALE`
    /// const-folds the internal `ln 2` — see [`Self::ln_fixed`].
    fn exp_fixed<const SCALE: u32>(working_value: Self::W, working_scale: u32) -> Self::W;
    /// Natural log of a positive working-scale value at scale `w`.
    ///
    /// `SCALE` is the decimal layer's own storage scale: on the common
    /// path `w == SCALE + GUARD`, so the kernel reads its `ln 2`
    /// constant from the compile-time baked `WideConst<SCALE>` rather
    /// than re-deriving it at runtime; any other `w` (Ziv escalation)
    /// falls to the runtime const. Bit-identical either way.
    fn ln_fixed<const SCALE: u32>(working_value: Self::W, working_scale: u32) -> Self::W;
    /// Sine of a working-scale value at `working_scale`. `SCALE` const-folds
    /// the internal `π` — see [`Self::ln_fixed`].
    fn sin_fixed<const SCALE: u32>(working_value: Self::W, working_scale: u32) -> Self::W;
    /// Cosine of a working-scale value at `working_scale`. `SCALE` const-folds
    /// the internal `π` — see [`Self::ln_fixed`].
    fn cos_fixed<const SCALE: u32>(working_value: Self::W, working_scale: u32) -> Self::W;
    /// Joint sine + cosine of a working-scale value at `working_scale`. `SCALE`
    /// const-folds the internal `π` — see [`Self::ln_fixed`].
    fn sin_cos_fixed<const SCALE: u32>(
        working_value: Self::W, working_scale: u32) -> (Self::W, Self::W);
    /// Arctangent of a working-scale value at `working_scale`. `SCALE`
    /// const-folds the internal `π/2` — see [`Self::ln_fixed`].
    fn atan_fixed<const SCALE: u32>(working_value: Self::W, working_scale: u32) -> Self::W;

    // ── working-scale helpers the tan kernel needs ────────────────────

    /// `(numerator · 10^w) / divisor`, rounded half-to-even.
    fn div(numerator: Self::W, divisor: Self::W, working_scale: u32) -> Self::W;
    /// `(lhs · rhs) / 10^w`, rounded half-to-even — the plain work-int
    /// multiply. Needed by the inverse / inverse-hyperbolic schoolbooks
    /// (`x^2`, `inv^2`, `t*(t+2)`).
    fn mul(lhs: Self::W, rhs: Self::W, working_scale: u32) -> Self::W;
    /// Integer square root of a non-negative working-scale value at
    /// scale `w` (`sqrt(v / 10^w) * 10^w`). The leaf asin/acos/asinh/
    /// acosh schoolbooks need it (`asin = atan(x / sqrt(1 - x^2))`).
    /// Dispatches down to the work-int root.
    fn sqrt_fixed(value: Self::W, working_scale: u32) -> Self::W;
    /// `ln(1 + t)` at `working_scale`, accurate for small `t` — the
    /// near-1 branch of the acosh schoolbook (avoids the `v^2 - 1`
    /// cancellation as `v -> 1`).
    fn log1p_fixed(argument: Self::W, working_scale: u32) -> Self::W;
    /// Bit length of `|v|` (0 for zero).
    fn bit_length(value: Self::W) -> u32;

    // hyperbolic exp-identity kernels (sinh/cosh/tanh schoolbooks)

    /// The `ceil(|x| * log10(e))` integer-digit lift for the large-arg
    /// `e^x` reassembly, used to set the base guard so a big `sinh`/
    /// `cosh` stays sub-storage-ULP. `mag_at_scale` is `x * 10^scale`.
    fn exp_result_int_digits(mag_at_scale: Self::W, scale: u32) -> u32;
    /// `sinh(|x|)` at working scale `w` via the `(e^x - e^-x)/2`
    /// identity (composed in the wider [`Self::Wexp`]); caller reapplies
    /// the sign. `SCALE` const-folds the internal `ln 2` (via
    /// `exp_fixed`) — see [`Self::ln_fixed`].
    fn sinh_pos_wide<const SCALE: u32>(
        abs_working_value: Self::W, working_scale: u32) -> Self::W;
    /// `cosh(|x|)` at `working_scale` via the `(e^x + e^-x)/2`
    /// identity. `SCALE` const-folds the internal `ln 2` — see
    /// [`Self::sinh_pos_wide`].
    fn cosh_pos_wide<const SCALE: u32>(
        abs_working_value: Self::W, working_scale: u32) -> Self::W;
    /// `tanh(|x|)` at `working_scale` via the
    /// `(e^x - e^-x)/(e^x + e^-x)` identity; caller reapplies the sign.
    /// `SCALE` const-folds the internal `ln 2` — see
    /// [`Self::sinh_pos_wide`].
    fn tanh_pos_wide<const SCALE: u32>(
        abs_working_value: Self::W, working_scale: u32) -> Self::W;

    /// Tang/Series-ROUTED working-scale natural log on the wide
    /// composition integer [`Self::Wagm`] — the per-tier
    /// `ln_fixed_routed_agm` (Tang where `policy::ln::is_tang` routes
    /// it, Series otherwise; the per-tier Tang CAP is a macro literal,
    /// which is why this is a trait binding rather than a free generic).
    /// Consumed by the acosh / atanh canonical kernels.
    fn ln_fixed_routed_agm<const SCALE: u32>(
        working_value: Self::Wagm, working_scale: u32) -> Self::Wagm;

    /// Series `e^x` on the wide composition integer [`Self::Wagm`] — the
    /// per-tier `exp_fixed_series_agm`, which runs the plain series while
    /// the peak fits the composition width and otherwise widens into
    /// [`Self::Wexp`]. A trait binding rather than a free generic for the
    /// same reason as [`Self::ln_fixed_routed_agm`]: the widened arm is
    /// expressed against the tier's own `Wexp`. Consumed by the joint
    /// `sinh_cosh` kernel.
    fn exp_fixed_series_agm(
        working_value: Self::Wagm, working_scale: u32) -> Self::Wagm;

    /// Tang/Series-ROUTED working-scale `e^x` on the wide composition
    /// integer [`Self::Wagm`] — the `exp` sibling of
    /// [`Self::ln_fixed_routed_agm`], and a trait binding for the same
    /// reason (the per-tier Tang `M` is a macro literal). Consumed by the
    /// `powf` canonical kernel.
    fn exp_fixed_routed_agm<const SCALE: u32>(
        working_value: Self::Wagm, working_scale: u32) -> Self::Wagm;

    /// [`Self::exp_result_int_digits`] on the wide composition integer
    /// [`Self::Wagm`]. A separate binding rather than a reuse of the
    /// `Self::W` form because the two work integers differ under the
    /// two-core split, and the digit count is capped by the tier's own
    /// [`Self::Wexp`] headroom (`exp_lift_cap`).
    fn exp_result_int_digits_agm(mag_at_scale: Self::Wagm, scale: u32) -> u32;

    /// Directed-rounding narrowing with Ziv escalation, forcing a
    /// confirm recompute even in nearest modes — the acosh / atanh
    /// near-special path (the residual can sit on a rounding boundary).
    fn round_to_storage_directed_near_special(
        base_guard_digits: u32,
        target: u32,
        mode: RoundingMode,
        recompute: &mut dyn FnMut(u32) -> Self::W,
    ) -> Self::Storage;

    // ── working-scale helpers the Tang lookup kernels need ─────────────

    /// The work-integer `1` at `working_scale` (`10^w`), cached.
    fn one(working_scale: u32) -> Self::W;
    /// The work-integer literal `value` (small unsigned).
    fn lit(value: u128) -> Self::W;
    /// `ln 2` at `working_scale`, const-folded at the layer's own
    /// `SCALE` (the baked `WideConst<SCALE>` on the common
    /// `w == SCALE + GUARD` path) — see [`Self::ln_fixed`].
    fn ln2<const SCALE: u32>(working_scale: u32) -> Self::W;
    /// `(numerator · 10^w) / divisor`, rounded half-to-even, with a
    /// precomputed `10^w` numerator factor (loop-friendly).
    fn div_cached(numerator: Self::W, divisor: Self::W, cached_pow10: Self::W) -> Self::W;
    /// Rounds a working-scale value to the nearest integer (ties away
    /// from zero); the range-reduction quotient for the Tang exp kernel.
    fn round_to_nearest_int(working_value: Self::W, working_scale: u32) -> i128;
    /// `10^exponent` in the work integer (the un-cached power; used to widen
    /// by `extra_digits` in the Tang exp reassembly).
    fn pow10(exponent: u32) -> Self::W;
    /// `Self::W::BITS` — the work integer's bit width.
    fn w_bits() -> u32;

    /// The `ln(1 + i/M)` Tang table slot at working scale `w` (table
    /// size `M = 128`; the `i = 0` slot is `0`, the `i = M` slot is
    /// `ln 2`). Recomputed on the stack per call; `SCALE` const-folds
    /// the internal `ln 2` — see [`Self::ln_fixed`].
    fn ln_table_entry<const SCALE: u32>(working_scale: u32, idx: usize) -> Self::W;

    /// The Tang exp table slot `exp(j · ln2 / M)` at `working_scale`
    /// for `table_size`. Recomputed on the stack per call; `SCALE`
    /// const-folds the internal `ln 2` — see [`Self::ln_fixed`].
    fn exp_table_entry<const SCALE: u32>(
        working_scale: u32, idx: usize, table_size: u32) -> Self::W;

    // ── π constants + the sincos Tang table (the sincos Tang kernel) ───

    /// `π` at working scale `w`, const-folded at the layer's own `SCALE`
    /// (the baked `WideConst<SCALE>` on the common `w == SCALE + GUARD`
    /// path) — see [`Self::ln_fixed`].
    fn pi<const SCALE: u32>(working_scale: u32) -> Self::W;
    /// `π/2` at `working_scale`, const-folded at the layer's own
    /// `SCALE` — see [`Self::pi`].
    fn half_pi<const SCALE: u32>(working_scale: u32) -> Self::W;

    /// `180/π` (degrees per radian) at working scale `w`, const-folded at
    /// the layer's own `SCALE` — see [`Self::pi`]. The exact angle-scale
    /// factor the `to_degrees` `MulPiRatio` kernel multiplies by (`x *
    /// 180/π`), replacing the runtime divide-by-`π`.
    fn deg_per_rad<const SCALE: u32>(working_scale: u32) -> Self::W;
    /// `π/180` (radians per degree) at working scale `w`, const-folded at
    /// the layer's own `SCALE` — see [`Self::pi`]. The exact angle-scale
    /// factor the `to_radians` `MulPiRatio` kernel multiplies by (`x *
    /// π/180`).
    fn rad_per_deg<const SCALE: u32>(working_scale: u32) -> Self::W;

    /// The sincos Tang table slot `(sin(c_j), cos(c_j))` at
    /// `working_scale` for `table_size`, where `c_j = j · π / (4·m)` and
    /// `j ∈ [0, m]` (the `j = m` slot is `(sin π/4, cos π/4)`, needed
    /// because rounding can lift a residual to the table boundary).
    /// Recomputed on the stack per call; `SCALE` const-folds the
    /// internal `π` — see [`Self::ln_fixed`].
    fn sincos_table_entry<const SCALE: u32>(
        working_scale: u32, idx: usize, table_size: u32) -> (Self::W, Self::W);
}

/// Near-min analytic pin for `exp`. When `|v| < 10^(-SCALE/2)` the deviation
/// `e^v − (1 + v) = v²/2 + …` is provably below half a storage ULP, and `e^v > 1 + v`
/// strictly (exp is convex), so the correctly-rounded result is exactly `1 + v` for
/// every mode except the three the positive deviation — however deep it sits —
/// lifts by one ULP: `Ceiling` and `AwayFromZero` always (the result is
/// positive, so up IS away), and `ZeroFiveUp` when `1 + v` ends in `0` or `5`.
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
    let abs_raw = if raw < zero { zero - raw } else { raw };
    // `10^half` has ≈ half·log2(10) bits; skip the pow10 unless |raw| is plausibly
    // below it (true only for genuinely tiny inputs — every normal call exits here).
    // A value of bit-length `bit_len` is at least `2^(bit_len−1)`, so the exit is
    // certain only once `2^(bit_len−1) >= 10^half` ⟺ `(bit_len−1)·log10(2) >= half`
    // — comparing `bit_len` itself (not `bit_len−1`) silently dropped the top
    // quarter of the band.
    let bit_len = <C::Storage as BigInt>::BITS - abs_raw.leading_zeros();
    if ((bit_len - 1) as u64) * 100_000 >= (half as u64) * 332_193 {
        return None;
    }
    // Exact: |raw| < 10^half ⟺ v²/2 < ½ ULP and the deviation sits past the scale.
    if abs_raw >= crate::consts::pow10::dispatch::<C::Storage>(half) {
        return None;
    }
    let linear_value = C::storage_one(SCALE) + raw; // (1 + v), exact since |v| < 1
    let one_ulp = <C::Storage as BigInt>::from_i128(1);
    Some(match mode {
        // The deviation is strictly positive and `linear_value > 0`, so "up"
        // and "away from zero" are the same step here.
        RoundingMode::Ceiling | RoundingMode::AwayFromZero => linear_value + one_ulp,
        // Toward-zero lands on `linear_value`, so its own last digit is the
        // pivot.
        RoundingMode::ZeroFiveUp => {
            let d = linear_value.div_rem(<C::Storage as BigInt>::TEN).1.to_i128();
            if d == 0 || d == 5 {
                linear_value + one_ulp
            } else {
                linear_value
            }
        }
        _ => linear_value,
    })
}

/// `exp` for a wide tier — generic over the tier `C`.
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
    if let Some(pinned) = exp_near_min_pin::<C, SCALE>(raw, mode) {
        return pinned;
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
    // `C::Wexp` when the deciding digit is unreachable in `C::W`. A deep tie
    // past the precision horizon is decided by the kernel's [`TailSign`]
    // where the direct series can prove one (a tagged exact half is not a
    // tie — issue #95), and stays an exact tie only when it cannot.
    round_to_storage_widening_tail_signed_g::<C::Storage, C::W, C::Wexp>(
        C::GUARD,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_value = C::to_work_scaled(raw, guard_digits);
            crate::algos::exp::exp_generic::exp_fixed_tagged_with::<C::W>(
                working_value,
                SCALE + guard_digits,
                || C::exp_fixed::<SCALE>(working_value, SCALE + guard_digits),
            )
        },
        |guard_digits| {
            crate::algos::exp::exp_generic::exp_fixed_tagged::<C::Wexp>(
                to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard_digits),
                SCALE + guard_digits,
            )
        },
    )
}

/// Rung-generic `exp` — the Series exp kernel run at an
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
    if let Some(pinned) = exp_near_min_pin::<C, SCALE>(raw, mode) {
        return pinned;
    }
    round_to_storage_widening_tail_signed_g::<C::Storage, Wk, C::Wexp>(
        C::GUARD,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            let working_value = to_work_scaled_g::<C::Storage, Wk>(raw, guard_digits);
            eg::exp_fixed_tagged_with::<Wk>(working_value, working_scale, || {
                if eg::exp_peak_fits::<Wk>(working_value, working_scale) {
                    eg::exp_fixed::<Wk>(working_value, working_scale)
                } else {
                    eg::resize_or_panic::<C::Wexp, Wk>(eg::exp_fixed::<C::Wexp>(
                        to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard_digits),
                        working_scale,
                    ))
                }
            })
        },
        |guard_digits| {
            eg::exp_fixed_tagged::<C::Wexp>(
                to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard_digits),
                SCALE + guard_digits,
            )
        },
    )
}

/// `ln` for a wide tier — generic over the tier `C`. Panics if
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
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            let ln2 = if working_scale == SCALE + C::GUARD {
                crate::consts::ln2_by_scale::<Wk>(
                    working_scale, crate::support::rounding::DEFAULT_ROUNDING_MODE)
            } else {
                crate::consts::ln2_by_working_scale::<Wk>(
                    working_scale,
                    crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            };
            crate::algos::exp::exp_generic::ln_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard_digits),
                working_scale,
                ln2,
            )
        },
    )
}

/// `sin` for a wide tier — generic over the tier `C`. Replaces
/// the per-tier `sin_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn sin_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    // Linear-band analytic directed decision (relocated DOWN from the policy
    // layer so the bare tier kernel and the rung-dispatched path agree): the
    // cubic deciding digit sits below the work integer's Ziv reach AND the
    // const-table provisioning, so only the analytic sign resolves it.
    if let Some(pinned) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, false) {
        return pinned;
    }
    let (rounded, decided) = round_to_storage_directed_decided_g::<C::Storage, C::W>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| C::sin_fixed::<SCALE>(
            C::to_work_scaled(raw, guard_digits), SCALE + guard_digits),
    );
    // Deep sub-resolution band (deciding `x^{j*}`, `j* ≥ 5`): the walker is
    // mode-blind (`decided == false`); the sign is analytic (`sin` alternates).
    // The exact alternating-series bracket first: where it closes it PROVES
    // which side of `rounded` the true value lies on, superseding the
    // `j*`-parity rule whose exactness premise fails for a multi-digit
    // significand.
    if let Some(bracketed) = adjust_alternating_bracket::<C::Storage, C::W, SCALE>(
        rounded, raw, mode, AlternatingSeries::Sin)
    {
        return bracketed;
    }
    let rounded = tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        rounded, decided, raw, mode, true, <C::W as BigInt>::BITS);
    adjust_bounded_extremum::<C, SCALE>(rounded, raw, mode)
}

/// `cos` for a wide tier — generic over the tier `C`. Standalone
/// `cos_fixed` path (cofunction identity, one `sin_fixed`, no sqrt).
/// Replaces the per-tier `cos_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn cos_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let rounded = C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard_digits| {
        C::cos_fixed::<SCALE>(C::to_work_scaled(raw, guard_digits), SCALE + guard_digits)
    });
    // [`adjust_bounded_extremum`] covers only a landing exactly ON `±10^SCALE`.
    // A landing on a DIFFERENT grid point near the extremum needs the bracket
    // to place it — the `cos(3·10⁻⁶⁴)` family, whose leading terms are exact
    // ULP multiples and whose first non-terminating term is `x⁸/8!` (the
    // factor `7` in `8!`) sitting below the walker's reach.
    if let Some(bracketed) = adjust_alternating_bracket::<C::Storage, C::W, SCALE>(
        rounded, raw, mode, AlternatingSeries::Cos)
    {
        return bracketed;
    }
    adjust_bounded_extremum::<C, SCALE>(rounded, raw, mode)
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
/// - just below `+1` (`rounded == +one`): Floor / Trunc step down one LSB to
///   `one − 1`; Ceiling and AwayFromZero keep `one`.
/// - just above `−1` (`rounded == −one`): Ceiling / Trunc step toward zero to
///   `−one + 1`; Floor and AwayFromZero keep `−one`.
/// - ZeroFiveUp follows Trunc on both sides whenever `SCALE >= 1`: the
///   toward-zero magnitude is `10^SCALE − 1`, which ends in `9`, so the
///   `0`/`5` pivot never fires.
///
/// Nearest modes are unaffected (rounding to `±1` IS correct to nearest there).
/// A no-op unless the directed result is exactly `±10^SCALE` and `raw != 0`, so
/// reachable cells (already resolved off the grid line) and the exact
/// `cos(0) = 1` point pass through untouched. The rule is continuous over the
/// whole near-extremum region, not fitted to one benched cell.
#[inline]
pub(crate) fn adjust_bounded_extremum<C: WideTrigCore, const SCALE: u32>(
    rounded: C::Storage,
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if crate::support::rounding::is_nearest_mode(mode) || raw == C::storage_zero() {
        return rounded;
    }
    let one = C::storage_one(SCALE);
    let neg_one = C::storage_zero() - one;
    // `ZeroFiveUp` pivots on the last digit of the TOWARD-ZERO result,
    // whose magnitude is `10^SCALE − 1`. For `SCALE >= 1` that ends in
    // `9`, so it never bumps and behaves as `Trunc`; at `SCALE == 0` it
    // is `0`, a pivot digit, so it bumps back onto `±1` like `_` does.
    // `AwayFromZero` always keeps the full magnitude `±1`, which is `_`.
    let zero_five_up_truncates = SCALE >= 1;
    if rounded == one {
        match mode {
            RoundingMode::Floor | RoundingMode::Trunc => one - <C::Storage as BigInt>::ONE,
            RoundingMode::ZeroFiveUp if zero_five_up_truncates => {
                one - <C::Storage as BigInt>::ONE
            }
            _ => rounded,
        }
    } else if rounded == neg_one {
        match mode {
            RoundingMode::Ceiling | RoundingMode::Trunc => neg_one + <C::Storage as BigInt>::ONE,
            RoundingMode::ZeroFiveUp if zero_five_up_truncates => {
                neg_one + <C::Storage as BigInt>::ONE
            }
            _ => rounded,
        }
    } else {
        rounded
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
    let abs_raw = if raw < zero { zero - raw } else { raw };
    // The small-x linear band exponent, conservative by one digit (matches the
    // narrow `small_x_linear_threshold`): `|raw| ≤ 10^(SCALE − ⌈SCALE/3⌉)`.
    let threshold_exponent = SCALE - SCALE.div_ceil(3);
    // One table-read + one compare exits for every normal-magnitude argument.
    if abs_raw > crate::consts::pow10::dispatch::<St>(threshold_exponent) {
        return None;
    }
    // `one` is ONE STORAGE ULP (the integer `1`), the step the directed
    // decision adds/drops — NOT `10^SCALE` (the value 1.0).
    let one = <St as BigInt>::ONE;
    // `ZeroFiveUp`'s pivot digit; `abs_raw` is already the magnitude.
    let raw_mod_10 = abs_raw.div_rem(<St as BigInt>::TEN).1.to_i128() as u8;
    Some(if expanding {
        crate::support::rounding::tiny_odd_expanding_directed(raw, zero, one, raw_mod_10, mode)
    } else {
        crate::support::rounding::tiny_odd_compressing_directed(raw, zero, one, raw_mod_10, mode)
    })
}

/// Upper bound on the deciding Taylor-term index `j*` that the deep-band
/// directed post-adjust ([`tiny_x_deep_directed_adjust`]) handles — the gate
/// there is `5 ≤ j* ≤ JMAX`. `j* ≤ JMAX` is equivalent to
/// `|x| < 10^(−SCALE/JMAX)`, a CONTINUOUS tiny-argument band (Class I) rather
/// than a per-cell fit: a smaller `|x|` raises the leading-digit position `k`,
/// which lowers `j*`.
///
/// # This bound is INERT — measured, not argued (#78)
///
/// It excludes NOTHING, at any tier and any scale. Enumerating every
/// `(SCALE, k)` cell at all ten wide tiers and counting those that pass BOTH
/// this gate and the `j*·k > reach` test gives an identical count at caps 39,
/// 41, 43, 99 and unbounded:
///
/// ```text
///   tier    reach   firing j*   cells (identical at every cap)
///   D57      120    none              0
///   D76      120    5..=5             1
///   D115     248    none              0
///   D153     248    5..=5             3
///   D230     376    5..=5             2
///   D307     504    5..=5             5
///   D462     504    5..=19         5859
///   D616     760    5..=9          4769
///   D924    1016    5..=19        22322
///   D1232   1400    5..=15        32668
/// ```
///
/// The `j*·k > reach` test does 100% of the work and the largest `j*` that ever
/// fires is 19 — 22 clear of this cap. It also performs the NON-TINY exclusion
/// that this comment previously claimed for itself: the `k = 1` corner is kept
/// out by `j*·k ≤ reach`, never by `j* > JMAX`.
///
/// # What actually carries it: `reach > MAX_SCALE`
///
/// `j*` is the smallest odd `j` with `j·k > SCALE`, so `j*·k ≈ SCALE` and
/// clearing `reach` needs `SCALE ≳ reach`. Every tier has
/// `reach = W::BITS/8 − 8` above its `MAX_SCALE`, which is why only genuinely
/// deep cells fire and `j*` stays small. Margins, tightest first:
///
/// ```text
///   D462  504 −  461 =  43      D153  248 −  152 =  96
///   D76   120 −   75 =  45      D115  248 −  114 = 134
///   D57   120 −   56 =  64      D616  760 −  615 = 145
///   D924 1016 −  923 =  93      D230  376 −  229 = 147
///                               D1232 1400 − 1231 = 169
///                               D307  504 −  306 = 198
/// ```
///
/// That pair is the one nothing enforces: the work integer's width is chosen for
/// precision and cost, `MAX_SCALE` defines the tier, and they are set in
/// different places by different concerns. This cap would only begin to bind
/// once a margin fell below about `MAX_SCALE/JMAX` (~11 at D462) — some four
/// times tighter than today — whereas `reach > MAX_SCALE` has no cushion beyond
/// that same 43 and would break if D462's work integer were narrowed one step.
///
/// Issue #78 states the dependency as `41 < 43`. No `43` is declared anywhere in
/// the tree; 43 is D462's margin above, so that mapping is an INFERENCE and is
/// recorded as one.
///
/// PROPOSED, deferred to #87: assert `W::BITS/8 − 8 > MAX_SCALE` per tier where
/// the tier macro already has both, so narrowing a work integer or raising a
/// scale breaks the BUILD rather than the arithmetic. It is the same move as
/// tying the Karatsuba width to its work integer and is better done once with
/// it than twice apart.
///
/// # Why the previous wording was wrong
///
/// It asserted a reason — "keeps the post-adjust off NON-tiny arguments" — that
/// measurement does not support. This is the second of the "unearned
/// correctness" issues to resolve that way rather than the first: the property
/// held and the stated reason was not the reason (see the truncation note in
/// `ln_tang` for the other). A doc comment explaining WHY something is safe is
/// exactly as likely to be wrong as any other unverified claim, and is trusted
/// more than most. Measure before relying on one.
const TINY_X_DEEP_JMAX: u32 = 41;

/// Analytic directed post-adjust for the DEEP sub-resolution band of the odd
/// forward/inverse trig functions — the generalisation of
/// [`tiny_x_linear_directed`] past its `j* = 3` (cubic) reach.
///
/// For a tiny `x = m·10^(−k)` whose LEADING odd Taylor terms terminate exactly
/// on the storage grid (so `f(x)` rounds to a grid value `G` under nearest),
/// the directed side is decided by the first SUB-resolution odd term `x^{j*}`
/// (`j* =` smallest odd `j` with `j·k > SCALE`). Its sign is analytically
/// certain: `sin`/`atan` alternate (`+` for `j* ≡ 1 (mod 4)`, `−` for `j* ≡ 3`),
/// `tan`/`asin` are always `+` (every coefficient positive). At a wide tier
/// `x^{j*}` sits at fractional depth `j*·k`, BEYOND both the work integer's Ziv
/// reach AND the const-table provisioning, so the directed walker returns the
/// mode-blind grid value `G` (`decided == false`) — the `±3·10^(−120)` @ D616
/// s615 find, deciding `x⁷` at depth ~841.
///
/// TWO conditions must hold for the adjust to fire — and they are independent:
/// 1. `decided == false`: the walker gave up at its escalation cap.
/// 2. `j*·k > reach`: the deciding term sits BEYOND the walker's reach
///    (`reach = work_bits/8 − 8`, the directed `BITS/8` cap with the
///    bounded-result trig `int_digits = 0`). This is the load-bearing test:
///    `decided == false` ALONE is not "mode-blind" — the walker can RESOLVE
///    the deciding term correctly yet still report `decided == false` when its
///    CONFIRM probe lands on the cap-clamped (`tainted`) rung. Only when the
///    term is genuinely past `reach` is `rounded` the mode-blind grid value `G`.
///
/// When both hold the value is on the grid (`rounded == G`, every above-scale term
/// terminated — an off-grid value's non-terminating tail is at depth `~SCALE`,
/// well WITHIN `reach`, so the walker resolves it and `j*·k ≤ reach` keeps this
/// off). The result is `G ± 1 ULP` per the deciding sign via the same
/// [`tiny_odd_expanding_directed`] / [`tiny_odd_compressing_directed`] step the
/// linear band uses. `alternating` selects the sign rule (`true` = `sin`/`atan`,
/// `false` = `tan`/`asin`); `work_bits` is the tier work integer's `BITS`
/// (`C::W` — the deepest width the widening walker reaches). A no-op for nearest
/// modes, `decided == true`, `raw == 0`, `SCALE == 0`, `|x| ≥ 1`, `j*` outside
/// `[5, JMAX]`, or `j*·k ≤ reach`.
#[inline]
pub(crate) fn tiny_x_deep_directed_adjust<St: BigInt, const SCALE: u32>(
    rounded: St,
    decided: bool,
    raw: St,
    mode: RoundingMode,
    alternating: bool,
    work_bits: u32,
) -> St {
    if decided || crate::support::rounding::is_nearest_mode(mode) || SCALE == 0 {
        return rounded;
    }
    let zero = <St as BigInt>::ZERO;
    if raw == zero {
        return rounded;
    }
    let abs_raw = if raw < zero { zero - raw } else { raw };
    // Leading-digit position `k`: `|x| = |raw|·10^(−SCALE)` and `|x| ≈ 10^(−k)`,
    // so `k = SCALE − digits(|raw|) + 1`. `|x| ≥ 1` (digits > SCALE) is not tiny.
    let digits = dec_digits_g::<St>(abs_raw);
    if digits == 0 || digits > SCALE {
        return rounded;
    }
    let k = SCALE - digits + 1;
    if k == 0 {
        return rounded;
    }
    // `j*` = smallest ODD `j` with `j·k > SCALE`. `floor(SCALE/k)+1` is the
    // smallest integer with the property (`(⌊SCALE/k⌋+1)·k = ⌊SCALE/k⌋·k + k >
    // SCALE` since `k > SCALE mod k`); round up to the next odd.
    let j_min = SCALE / k + 1;
    let j_star = if j_min % 2 == 1 { j_min } else { j_min + 1 };
    // `j* = 3` is the linear band's [`tiny_x_linear_directed`] pre-empt; only the
    // deeper terms reach here. The upper bound excludes the non-tiny corner.
    if !(5..=TINY_X_DEEP_JMAX).contains(&j_star) {
        return rounded;
    }
    // The deciding term must be BEYOND the walker's reach (else it RESOLVED
    // `rounded` correctly — see the doc). `j*·k` is its fractional depth.
    let reach = (work_bits / 8).saturating_sub(8);
    if j_star.saturating_mul(k) <= reach {
        return rounded;
    }
    let expanding = if alternating { j_star % 4 == 1 } else { true };
    let one = <St as BigInt>::ONE;
    // The step is taken from the GRID VALUE `rounded`, not from `raw`, so the
    // `ZeroFiveUp` pivot digit is `|rounded| % 10`.
    let abs_rounded = if rounded < zero { zero - rounded } else { rounded };
    let rounded_mod_10 = abs_rounded.div_rem(<St as BigInt>::TEN).1.to_i128() as u8;
    if expanding {
        crate::support::rounding::tiny_odd_expanding_directed(
            rounded,
            zero,
            one,
            rounded_mod_10,
            mode,
        )
    } else {
        crate::support::rounding::tiny_odd_compressing_directed(
            rounded,
            zero,
            one,
            rounded_mod_10,
            mode,
        )
    }
}

/// Significant limb length of `limbs` (index of the highest non-zero limb plus
/// one), clamped to at least 1 so a zero magnitude has length 1.
#[inline]
fn sig_len(limbs: &[u64]) -> usize {
    let mut len = limbs.len();
    while len > 1 && limbs[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// Exact three-way comparison of `squared_operand²` against
/// `product_lhs · product_rhs` for three NON-NEGATIVE values, evaluated at
/// DOUBLE `Wk` width. `None` means "not decidable in the available
/// scratch" — see the fail-closed note below.
///
/// The parabola test in [`adjust_log_near_zero`] compares `δ²` against
/// `2·D·10^SCALE`, and `δ²` legitimately overflows every SINGLE width in play:
/// the input family that provokes the defect has `δ ≈ 10^(SCALE/2)`, so `δ²`
/// is `≈ 10^SCALE` — already the whole storage range — and the odd-scale
/// member puts `δ²` exactly at the storage maximum. A single-width
/// `checked_mul` would therefore fold to "no answer" at precisely the cells
/// that need one, so both products are formed at double width and compared as
/// limb slices.
///
/// The buffers come from `Wk`'s own [`ComputeLimbs`] carrier
/// (`double_buffered_u64`, exact per-`N`, never a build-max literal), and the
/// multiply routes through the multiply matcher's slice door, so the
/// schoolbook/Karatsuba choice stays the matcher's rather than being hardcoded
/// here. Both operand pairs are STORAGE-magnitude values widened into `Wk`, so
/// their products occupy at most `2·St::LIMBS + 1` limbs — inside the
/// `2N + ⌈N/2⌉` buffer at every width in both the `exact-scratch` and the
/// blanket build. That capacity is CHECKED rather than assumed: a product that
/// would not fit yields `None`, which every caller treats as "make no
/// adjustment", so the shortfall can never become a truncated comparison, a
/// wrong step, or a panic.
///
/// [`ComputeLimbs`]: crate::int::types::compute_limbs::ComputeLimbs
#[inline]
fn cmp_sq_vs_prod<Wk: BigInt>(
    squared_operand: Wk, product_lhs: Wk, product_rhs: Wk) -> Option<core::cmp::Ordering>
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::int::policy::mul::dispatch_slice as mul_slice;
    use crate::int::types::compute_limbs::ComputeLimbs;

    let mut squared_limbs = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    let mut lhs_limbs = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    let mut rhs_limbs = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    eg::unpack_mag::<Wk>(squared_operand, squared_limbs.as_mut());
    eg::unpack_mag::<Wk>(product_lhs, lhs_limbs.as_mut());
    eg::unpack_mag::<Wk>(product_rhs, rhs_limbs.as_mut());
    let squared_len = sig_len(squared_limbs.as_ref());
    let lhs_len = sig_len(lhs_limbs.as_ref());
    let rhs_len = sig_len(rhs_limbs.as_ref());

    let mut lhs = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_buffered_u64();
    let mut rhs = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_buffered_u64();
    let cap = lhs.as_ref().len();
    if 2 * squared_len > cap || lhs_len + rhs_len > cap {
        return None; // fail closed — the caller makes no adjustment
    }
    mul_slice(
        &squared_limbs.as_ref()[..squared_len],
        &squared_limbs.as_ref()[..squared_len],
        &mut lhs.as_mut()[..2 * squared_len],
    );
    mul_slice(
        &lhs_limbs.as_ref()[..lhs_len],
        &rhs_limbs.as_ref()[..rhs_len],
        &mut rhs.as_mut()[..lhs_len + rhs_len],
    );
    // Both buffers are the same length and zero above their product, so the
    // full-slice compare is the product compare.
    Some(
        match crate::int::algos::support::limbs::cmp(lhs.as_ref(), rhs.as_ref()) {
            ordering if ordering < 0 => core::cmp::Ordering::Less,
            0 => core::cmp::Ordering::Equal,
            _ => core::cmp::Ordering::Greater,
        },
    )
}

/// The shared analytic post-adjust for the logarithm family inside the
/// sub-resolution band around its zero (`ln(1) = 0`, `log1p(0) = 0`).
///
/// `delta` is the LINEAR term at storage scale — `raw − 10^SCALE` for `ln`,
/// `raw` itself for `log1p` — and `one` is `10^SCALE`. Writing `u = δ/one`,
/// the true value at storage scale is
///
/// ```text
///     V  =  δ  −  Q  +  C,          Q = δ²/(2·one) > 0
/// ```
///
/// with `C` the cubic-and-beyond tail. Two elementary derivatives pin the
/// bracket EXACTLY over the whole domain (`u > −1`, `u ≠ 0`):
///
/// ```text
///     d/du [ u − ln(1+u) ]          =  u/(1+u)
///     d/du [ ln(1+u) − u + u²/2 ]   =  u²/(1+u)  >  0
/// ```
///
/// Both bracketing functions vanish at `u = 0`; the first has the sign of `u`
/// and the second is strictly increasing, so
///
/// ```text
///     δ > 0:      δ − Q   <   V   <   δ
///     δ < 0:                  V   <   δ − Q   <   δ
/// ```
///
/// # Why the defect this corrects is ONE-SIDED
///
/// `V` never lands on a storage grid line (`ln` of an algebraic `x ≠ 1` is
/// transcendental), but the Ziv walker only resolves digits down to about
/// `W::BITS/8`. When the deciding term sits deeper, the walker sees a residual
/// of exactly zero and returns the grid point it reached AS THOUGH the value
/// were exact. For `δ > 0` the true value is then strictly ABOVE that grid
/// point, so every mode that rounds down or to nearest is accidentally right
/// and only `Ceiling` is wrong; for `δ < 0` it is strictly BELOW and only
/// `Floor` is wrong. The asymmetry is just the sign of the leading uncancelled
/// term, which is why this adjust is sign-aware: applying either step to the
/// other side would corrupt an answer that is already correct.
///
/// # The two grid points, and why the second needs the parabola
///
/// * **On the tangent** (`rounded == δ`) — the quadratic `Q` itself fell below
///   the reachable working scale. `V < δ` settles it, so a downward-directed
///   result steps down one ULP: `Floor` for both signs, `Trunc` only for
///   `δ > 0` (for `δ < 0` truncation moves UP and `δ` is already correct).
/// * **On the parabola** (`rounded == δ − Q`, `Q` an exact whole number of
///   ULPs) — the `δ² ≡ 0 (mod 2·10^SCALE)` family. Its quadratic term is an
///   exact ULP multiple, so the value steps to a DIFFERENT grid point and the
///   tangent test above no-ops; the CUBIC then decides, at fractional depth
///   `≈ 3·SCALE/2`, far past the walker's reach.
///
/// The parabola test is `rounded ≤ δ − Q  ⟺  Q ≤ D  ⟺  δ² ≤ 2·D·one`, where
/// `D = δ − rounded` — an exact integer comparison ([`cmp_sq_vs_prod`]), never
/// a tolerance.
///
/// # It cannot fire on a correct result
///
/// A correct `Ceiling` has `rounded = ⌈V⌉ ≥ V`, whereas `Q ≤ D` rearranges to
/// `rounded ≤ δ − Q < V` — a contradiction. So the test is FALSE for every
/// correctly-rounded `Ceiling`, at every input, and the mirror argument holds
/// for `Floor` at `δ < 0`. The step is therefore reachable only from a
/// genuinely wrong result, which is what lets it be applied without any
/// reachability or width gate.
///
/// A no-op for the nearest modes, for the exact point `δ = 0`, and for every
/// cell whose deciding digit the walker actually reaches. Mirrors
/// [`adjust_bounded_extremum`] / [`adjust_cosh_near_min`].
#[inline]
pub(crate) fn adjust_log_near_zero<St: BigInt + Copy, Wk: BigInt>(
    rounded: St,
    delta: St,
    one: St,
    mode: RoundingMode,
) -> St
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use core::cmp::Ordering;

    if crate::support::rounding::is_nearest_mode(mode) {
        return rounded;
    }
    let zero = <St as BigInt>::ZERO;
    if delta == zero {
        return rounded; // ln(1) = 0 / log1p(0) = 0 — the one exact point
    }
    let unit = <St as BigInt>::ONE;
    let is_up = delta > zero;

    // ── ZeroFiveUp: truncate, then lift only on a 0 / 5 pivot ──────────
    //
    // `round-05up` is `round-down` unless the last digit of the TRUNCATED
    // coefficient is `0` or `5`, in which case the coefficient steps one
    // away from zero. Both halves are available here:
    //
    // * The discarded part is never zero anywhere this pass does work.
    //   `δ != 0` past the guard above, so `V = ln(1 + δ)` is the log of a
    //   rational other than `1` — transcendental by Lindemann–Weierstrass,
    //   hence never exactly on the storage grid. So the pivot alone
    //   decides; there is no "exact value, leave it alone" case to detect.
    // * The truncated value is this pass's OWN `Trunc` answer: the tangent
    //   bracket's `Trunc if is_up` arm gives `rounded - unit`, and every
    //   other path falls through to `rounded`.
    //
    // `V` carries `δ`'s sign, so the away-from-zero step is `+unit` when
    // `is_up` and `-unit` otherwise. This cannot double-bump a walker that
    // already resolved the mode: the walker only returns the lifted value
    // when the truncated one ended in `0` or `5`, so the value arriving
    // here then ends in `1` or `6` and the pivot below is false.
    if matches!(mode, RoundingMode::ZeroFiveUp) {
        let toward_zero = if rounded == delta && is_up {
            rounded - unit
        } else {
            rounded
        };
        let mag = if toward_zero < zero {
            zero - toward_zero
        } else {
            toward_zero
        };
        let pivot = matches!(mag.div_rem(<St as BigInt>::TEN).1.to_i128(), 0 | 5);
        return if pivot {
            if is_up {
                toward_zero + unit
            } else {
                toward_zero - unit
            }
        } else {
            toward_zero
        };
    }

    // ── the TANGENT bracket: `V < δ` for every `δ ≠ 0` ─────────────────
    if rounded == delta {
        return match mode {
            RoundingMode::Floor => rounded - unit,
            RoundingMode::Trunc if is_up => rounded - unit,
            // `V < δ` on both sides, so a NEGATIVE result's away-from-zero
            // step is the same one `Floor` takes; a positive result's is
            // `Ceiling`'s, which does not move here.
            RoundingMode::AwayFromZero if !is_up => rounded - unit,
            _ => rounded,
        };
    }

    // ── the PARABOLA bracket ───────────────────────────────────────────
    // `D = δ − rounded`: the gap from the linear term to the grid point the
    // walker returned, in storage ULPs.
    let gap = delta - rounded;
    let abs_delta = if is_up { delta } else { -delta };
    match mode {
        // `V > δ − Q`: a Ceiling sitting AT or BELOW the parabola is strictly
        // below the true value, so it must step up.
        // `AwayFromZero` joins each arm on the side where away-from-zero
        // IS that direction: up for `Ceiling`, down for `Floor`.
        RoundingMode::Ceiling | RoundingMode::AwayFromZero if is_up => {
            if gap <= zero {
                return rounded; // `Q > 0`, so `Q ≤ D` cannot hold
            }
            let gap_wide = gap.resize_to::<Wk>();
            match cmp_sq_vs_prod::<Wk>(
                abs_delta.resize_to::<Wk>(), gap_wide + gap_wide, one.resize_to::<Wk>())
            {
                Some(Ordering::Less | Ordering::Equal) => rounded + unit,
                _ => rounded,
            }
        }
        // `V < δ − Q`: a Floor sitting AT or ABOVE the parabola is strictly
        // above the true value, so it must step down.
        RoundingMode::Floor | RoundingMode::AwayFromZero if !is_up => {
            if gap <= zero {
                return rounded - unit; // `Q > 0 ≥ D`, so `Q ≥ D` holds
            }
            let gap_wide = gap.resize_to::<Wk>();
            match cmp_sq_vs_prod::<Wk>(
                abs_delta.resize_to::<Wk>(), gap_wide + gap_wide, one.resize_to::<Wk>())
            {
                Some(Ordering::Greater | Ordering::Equal) => rounded - unit,
                _ => rounded,
            }
        }
        _ => rounded,
    }
}

/// The four alternating power series the [`adjust_alternating_bracket`]
/// post-adjust serves, carried as the ONE thing that differs between them —
/// the term-ratio recurrence `|c_{j+2}| / |c_j| = a_j / b_j`.
///
/// | series | expansion | `a_j` | `b_j` |
/// |---|---|---|---|
/// | `Sin`   | `x − x³/3! + x⁵/5! − …` | `1`  | `(j+1)(j+2)` |
/// | `Cos`   | `1 − x²/2! + x⁴/4! − …` | `1`  | `(j+1)(j+2)` |
/// | `Atan`  | `x − x³/3 + x⁵/5 − …`   | `j`  | `j+2`        |
/// | `Asinh` | `x − x³/6 + 3x⁵/40 − …` | `j²` | `(j+1)(j+2)` |
///
/// All four alternate in sign from the leading term and have STRICTLY
/// DECREASING magnitudes for `0 < |x| < 1` — `a_j < b_j` in every row — which
/// is exactly the precondition the bracket theorem needs, so one generic
/// kernel serves all of them.
///
/// `tan` / `asin` are deliberately absent: their Taylor coefficients are all
/// positive, so consecutive partial sums approach the value from ONE side
/// instead of straddling it and the bracket does not apply. They keep the
/// existing [`tiny_x_deep_directed_adjust`] path, where the all-positive
/// coefficients make the sign unconditional.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum AlternatingSeries {
    Sin,
    Cos,
    Atan,
    Asinh,
}

impl AlternatingSeries {
    /// Index of the leading term — `0` for the even face (`cos`, whose
    /// leading term is the constant `1`), `1` for the three odd faces.
    const fn first_index(self) -> u32 {
        match self {
            AlternatingSeries::Cos => 0,
            _ => 1,
        }
    }

    /// `(a_j, b_j)` for the term at `term_index` = `j`, with
    /// `|c_{j+2}| = |c_j| · a_j / b_j`. Both fit a `u64` for every `j` the
    /// band admits (`j ≤ TINY_X_DEEP_JMAX`, so `b_j ≤ 42·43`).
    const fn ratio(self, term_index: u32) -> (u64, u64) {
        let j = term_index as u64;
        match self {
            AlternatingSeries::Sin | AlternatingSeries::Cos => (1, (j + 1) * (j + 2)),
            AlternatingSeries::Atan => (j, j + 2),
            AlternatingSeries::Asinh => (j * j, (j + 1) * (j + 2)),
        }
    }
}

/// `value[..value_len] *= multiplier` through the multiply matcher's slice
/// door, returning the new significant length. `scratch` must hold
/// `value_len + 1` limbs.
#[inline]
fn mul_small(value: &mut [u64], value_len: usize, multiplier: u64, scratch: &mut [u64]) -> usize {
    if multiplier == 1 {
        return value_len;
    }
    for s in scratch[..value_len + 1].iter_mut() {
        *s = 0;
    }
    crate::int::policy::mul::dispatch_slice(
        &value[..value_len],
        &[multiplier],
        &mut scratch[..value_len + 1],
    );
    value[..value_len + 1].copy_from_slice(&scratch[..value_len + 1]);
    sig_len(&value[..value_len + 1])
}

/// `out_limbs = 10^exponent`, returning its significant length; `None` when
/// `out_limbs` / `scratch` cannot hold it (fail closed). `10^19` is the
/// largest power of ten a `u64` holds, so the build costs `⌈exponent/19⌉`
/// small multiplies rather than `exponent` of them.
fn pow10_into(exponent: u32, out_limbs: &mut [u64], scratch: &mut [u64]) -> Option<usize> {
    for o in out_limbs.iter_mut() {
        *o = 0;
    }
    if out_limbs.is_empty() {
        return None;
    }
    out_limbs[0] = 1;
    let mut significant = 1usize;
    let mut remaining = exponent;
    while remaining > 0 {
        let step = if remaining >= 19 { 19 } else { remaining };
        if significant + 1 > out_limbs.len() || significant + 1 > scratch.len() {
            return None;
        }
        significant = mul_small(out_limbs, significant, 10u64.pow(step), scratch);
        remaining -= step;
    }
    Some(significant)
}

/// Exact-scratch divide for the bracket. Returns `false` when the scratch
/// cannot hold the shape, which every caller treats as "make no adjustment".
///
/// `int::policy::div_rem::dispatch` must NOT be used here. Its blanket engines
/// carry BUILD-MAX normalisation scratch — `div_knuth_u128_limb` declares
/// `[0u64; MAX_SINGLE_LIMBS]` = `4·MAX_WORK_N + 2` = 258 limbs against a
/// documented requirement of `dividend.len() + 2`. `MAX_WORK_N` derives from the
/// STORAGE-scaled work widths and never accounted for the AGM integer, so the
/// `asinh` face — which instantiates this kernel at `Wk = C::Wagm`
/// (`Int<192>` at D924, `Int<256>` at D1232) — presents a dividend of roughly
/// `3·Wk::LIMBS` limbs and indexes straight off the end of that buffer. That
/// is the build-max blanket leaking onto a path that can size itself exactly,
/// which rule 6 forbids and which here is not a style point but an
/// out-of-bounds panic (#86).
///
/// So the divide routes on the matcher's OWN verdict
/// ([`select_for_limbs`](crate::int::policy::div_rem::select_for_limbs)) and
/// then calls the chosen engine's `_into` door with scratch sized exactly from
/// `Wk`'s `ComputeLimbs` carrier — the same exact-scratch pattern
/// `newton_reciprocal` and `div_widen_scale` use, and the one
/// `div_rem::dispatch`'s own doc points concrete-`N` callers at.
///
/// The non-`Rem` verdicts collapse to base-2⁶⁴ Knuth, which is value-identical
/// to the u128-limb engine (that engine exists for speed, not a different
/// result). Correct for every shape this kernel presents today, but it MUST be
/// re-verified if an `Algorithm` arm joining `int::policy::div_rem` ever
/// returns a numerically different answer.
fn bracket_div<Wk: BigInt>(
    dividend: &[u64],
    divisor: &[u64],
    quotient: &mut [u64],
    remainder: &mut [u64],
) -> bool
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::int::policy::div_rem::{select_for_limbs, Algorithm};
    use crate::int::types::compute_limbs::ComputeLimbs;

    match select_for_limbs(dividend, divisor) {
        // Single-limb divisor: hardware remainder, no normalisation scratch
        // is involved at all, so there is nothing to size.
        Algorithm::Rem => {
            crate::int::algos::div::div_rem::div_rem(dividend, divisor, quotient, remainder);
            true
        }
        _ => {
            let mut norm_dividend_buf = <<Wk as BigInt>::Scratch as ComputeLimbs>::quad_u64();
            let mut norm_divisor_buf = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
            let norm_dividend = norm_dividend_buf.as_mut();
            let norm_divisor = norm_divisor_buf.as_mut();
            // Knuth needs `dividend.len() + 2` / `divisor.len()` zeroed limbs.
            // The dividend here is at most `3·Wk::LIMBS`, so `quad_u64`
            // (`4·LIMBS`) covers it — CHECKED, not assumed, because an
            // unchecked version of exactly this is what went wrong upstream.
            if norm_dividend.len() < dividend.len() + 2 || norm_divisor.len() < divisor.len() {
                return false;
            }
            for limb in norm_dividend.iter_mut() {
                *limb = 0;
            }
            for limb in norm_divisor.iter_mut() {
                *limb = 0;
            }
            crate::int::algos::div::div_knuth::div_knuth_into(
                dividend,
                divisor,
                quotient,
                remainder,
                norm_dividend,
                norm_divisor,
            );
            true
        }
    }
}

/// Analytic directed post-adjust that PROVES which side of the returned grid
/// point the true value lies on, by bracketing it between two consecutive
/// partial sums of its own alternating series.
///
/// # The defect this corrects
///
/// [`tiny_x_deep_directed_adjust`] derives the deciding term index `j*` from
/// `k = SCALE − digits + 1`, the LEADING-digit position, and then takes the
/// directed side from `j*`'s parity. Two different questions are being
/// conflated:
///
/// * whether term `j` is entirely SUB-ULP is governed by `k` — what the code
///   computes, and correct;
/// * whether term `j` is an EXACT ULP MULTIPLE is governed by the LAST
///   significant digit's position, i.e. by the digit COUNT as well — what the
///   code assumes, and correct only for a one-digit significand.
///
/// For a multi-digit significand an above-LSB term straddles the LSB and
/// carries SUB-LSB IMPRECISION — imprecision from a term the code assumed
/// contributed a whole number of ULPs, landing below the last stored digit.
/// It can sit BEYOND the walker's reach (so the walker is blind to it) yet be
/// far SHALLOWER than the `j*` term — in which case that imprecision, not
/// `c_{j*}`, decides the sign and the parity rule asserts the wrong
/// direction. Writing `raw = μ·10^t` with `μ` coprime to 10, every term is
/// the exact rational `R_j = μ^j / (j!·10^((j−1)L − t))` with `L = SCALE − t`;
/// at D462 s461 the input `3·10⁻¹⁵³ + 10⁻²⁵²` gives
/// `R_3 = 450 + 4.5·10⁻⁹⁷ + …`, so the true value is BELOW the grid point
/// while the parity rule says above. Flipping the last significant digit's
/// sign (`3·10⁻¹⁵³ − 10⁻²⁵²`) flips the sub-LSB imprecision to `−4.5·10⁻⁹⁷`
/// and the parity rule is accidentally right — which is why `k` and `j*`,
/// identical for both, cannot see the difference.
///
/// # The theorem
///
/// For an alternating series with strictly decreasing terms, consecutive
/// partial sums STRADDLE the value: `P_m < V < P_{m+1}` or the reverse, with
/// the side given by the sign of the first omitted term. So with `ρ` the
/// magnitude the walker returned, two brackets settle the rounding outright:
///
/// ```text
///     ρ ≤ P_lower  and  P_upper ≤ ρ + 1   ⟹   ρ < V < ρ + 1
///     ρ − 1 ≤ P_lower  and  P_upper ≤ ρ   ⟹   ρ − 1 < V < ρ
/// ```
///
/// Each localises `V` to ONE OPEN UNIT INTERVAL, from which every directed
/// mode's correct answer follows outright: the first is the EXPANDING step
/// (`Ceiling` moves up one ULP, `Floor`/`Trunc` stay), the second the
/// COMPRESSING step. Both are theorems about the series, carrying no width
/// gate, no scale gate and no tolerance.
///
/// # It cannot fire on a correct result, and does not consult `decided`
///
/// Because the conclusion is a proof of `V`'s position rather than an
/// inference from the walker's state, firing the first bracket establishes
/// `ρ < V`, so a `Ceiling` of `ρ` was strictly below the true value and
/// therefore wrong; the mirror argument holds for `Floor`/`Trunc` under the
/// second. The two brackets are mutually exclusive. Nothing here reads the
/// walker's `decided` flag — which is noisy in BOTH directions — so a
/// false-negative sensor can no longer mis-route the decision.
///
/// # Why the variable order is BOUNDED
///
/// "Iterate until a bracket fires" is bounded, not open-ended, for two
/// independent reasons.
///
/// The order that must actually be REACHED is `j0`, the first index whose
/// term is not a whole number of ULPs — every shallower term contributes
/// nothing below the LSB, so nothing before `j0` can decide the sign. Term
/// `j` is an exact ULP multiple only while `(j−1)·L ≤ t = SCALE − L`, so
/// exactness all the way to `j0 − 2` forces `(j0−3)·L ≤ SCALE − L`, giving
///
/// ```text
///     j0  ≤  SCALE/L + 2
/// ```
///
/// A LARGER `j0` therefore requires a SMALLER `L`, i.e. a shorter
/// significand — the two trade off, which is why the deep orders are only
/// ever reached by arguments cheap enough to carry there.
///
/// The loop is capped independently of that: the band gate admits an argument
/// only while `SCALE/k + 1 ≤ TINY_X_DEEP_JMAX`, and that quantity IS the
/// first entirely-sub-ULP index, so at most `(JMAX+3)/2` iterations run.
/// Width is bounded too — every term is smaller than the leading one, so `2N`
/// limbs hold the whole computation at every order and the accumulator never
/// grows with the order.
///
/// # Arithmetic
///
/// Everything is an ULP count in binary fixed point scaled by `2^F` with
/// `F = Wk::BITS`, so the per-term rescale is a whole-limb shift rather than
/// a division. One setup divide produces `y = ⌊x²·2^F⌋`; each step is then
/// `t ← ⌊⌊t·y / 2^F⌋ · a_j / b_j⌋`. Every operation truncates DOWNWARD, so the
/// computed term never exceeds the true one, and the resulting bound is
/// applied OUTWARD at both bracket ends so the comparison can only lose
/// coverage, never decide wrongly. That bound is NOT a small constant: the
/// dominant loss is the truncation in `y`, amplified by the term it
/// multiplies, so a step loses up to the term's own ULP magnitude (see the
/// derivation at the accumulator below) and the leading term dominates all of
/// them. It stays far below one ULP only while `Wk::BITS` exceeds roughly
/// `3.4·SCALE`, which is CHECKED at run time rather than assumed. Buffers are
/// exact per-`N` [`ComputeLimbs`] (`single_u64` / `double_u64` / `quad_u64`,
/// never a build-max), and every capacity is CHECKED: a short buffer, an
/// out-of-range quotient or a partial-sum borrow all yield `None`.
///
/// # Where it stays silent, and why that is principled
///
/// `None` means "no adjustment" and the caller keeps its existing path
/// byte-for-byte. That happens when no bracket closes to within one ULP —
/// notably when the sub-LSB imprecision is positive but SMALLER than the next
/// term, which is exactly the regime where that next term dominates and the
/// parity rule the caller falls back to is already CORRECT. The fallback is
/// therefore the right answer there, not an unfinished edge.
///
/// [`ComputeLimbs`]: crate::int::types::compute_limbs::ComputeLimbs
pub(crate) fn adjust_alternating_bracket<St: BigInt, Wk: BigInt, const SCALE: u32>(
    rounded: St,
    raw: St,
    mode: RoundingMode,
    series: AlternatingSeries,
) -> Option<St>
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::int::algos::support::limbs as lb;
    use crate::int::types::compute_limbs::ComputeLimbs;

    if crate::support::rounding::is_nearest_mode(mode) || SCALE == 0 {
        return None;
    }
    let zero = <St as BigInt>::ZERO;
    if raw == zero {
        return None;
    }
    let abs_raw = if raw < zero { zero - raw } else { raw };
    // The tiny-argument band — the SAME continuous band
    // [`tiny_x_deep_directed_adjust`] uses, read from the same constant so the
    // family keeps one band definition. `digits > SCALE` is `|x| ≥ 1`, where
    // the strictly-decreasing-terms precondition fails.
    let digits = dec_digits_g::<St>(abs_raw);
    if digits == 0 || digits > SCALE {
        return None;
    }
    let k = SCALE - digits + 1;
    if k == 0 {
        return None;
    }
    if SCALE / k + 1 > TINY_X_DEEP_JMAX {
        return None;
    }
    let rho = if rounded < zero { zero - rounded } else { rounded };

    // The work integer `Wk` carries the scratch (its `ComputeLimbs` bound is
    // the one already in scope at every caller), so the fixed point is `2^F`
    // with `F = Wk::BITS` — wider than the storage width, never narrower.
    let limb_count = <Wk as BigInt>::LIMBS;
    let double_limbs = 2 * limb_count;
    let f_bits = <Wk as BigInt>::BITS;

    let mut nb_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    let mut y_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    let mut t_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
    let mut bd_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
    let mut pr_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::quad_u64();
    let mut qt_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::quad_u64();
    let nb = nb_b.as_mut();
    let y = y_b.as_mut();
    let t = t_b.as_mut();
    let bd = bd_b.as_mut();
    let pr = pr_b.as_mut();
    let qt = qt_b.as_mut();
    // Capacity is CHECKED, never assumed.
    if nb.len() < limb_count
        || y.len() < limb_count
        || t.len() < double_limbs
        || bd.len() < double_limbs
        || pr.len() < 4 * limb_count
        || qt.len() < 4 * limb_count
    {
        return None;
    }

    // ── setup: `y = ⌊x²·2^F⌋`, the term-ratio multiplier ──────────────────
    eg::unpack_mag::<Wk>(abs_raw.resize_to::<Wk>(), nb);
    let abs_raw_len = sig_len(&nb[..limb_count]);
    for p in pr.iter_mut() {
        *p = 0;
    }
    if limb_count + 2 * abs_raw_len > pr.len() {
        return None;
    }
    // `n²·2^F`: the shift is a whole number of limbs (`F = 64·N`), so the
    // product is written straight into the offset window — no shift needed.
    crate::int::policy::mul::dispatch_slice(&nb[..abs_raw_len], &nb[..abs_raw_len],
        &mut pr[limb_count..limb_count + 2 * abs_raw_len]);
    let pow_len = pow10_into(2 * SCALE, bd, qt)?;
    let dividend_len = sig_len(&pr[..limb_count + 2 * abs_raw_len]);
    // A dividend shorter than the divisor would be a divide with a longer
    // divisor than dividend — it cannot arise for a band argument
    // (`|x| ≥ 10^−SCALE` gives `x²·2^F ≥ 1`), but the shape is checked rather
    // than assumed. `t` is still unused here, so it serves as the remainder
    // buffer.
    if qt.len() < dividend_len || t.len() < pow_len || dividend_len < pow_len {
        return None;
    }
    for q in qt.iter_mut() {
        *q = 0;
    }
    for e in t.iter_mut() {
        *e = 0;
    }
    if !bracket_div::<Wk>(
        &pr[..dividend_len], &bd[..pow_len], &mut qt[..dividend_len], &mut t[..pow_len])
    {
        return None;
    }
    // `|x| < 1` so `y < 2^F`, i.e. at most `N` limbs. A wider quotient would
    // mean a non-tiny argument slipped the band gate — fail closed rather
    // than silently truncate.
    if sig_len(&qt[..dividend_len]) > limb_count {
        return None;
    }
    y[..limb_count].copy_from_slice(&qt[..limb_count]);

    // ── the leading term, scaled by `2^F` ─────────────────────────────────
    for e in t.iter_mut() {
        *e = 0;
    }
    if series == AlternatingSeries::Cos {
        // The even face's leading term is the constant `1` = `10^SCALE` ULPs.
        // `abs_raw` is no longer needed, so its buffer carries the constant.
        let unit = crate::consts::pow10::dispatch::<Wk>(SCALE);
        eg::unpack_mag::<Wk>(unit, nb);
        lb::shl(&nb[..limb_count], f_bits, &mut t[..double_limbs]);
    } else {
        lb::shl(&nb[..abs_raw_len], f_bits, &mut t[..double_limbs]);
    }
    alternating_bracket_core::<St, Wk>(rounded, rho, mode, series, t, y, nb, bd, pr)
}

/// The RATIO face of [`adjust_alternating_bracket`] — `atan2`.
///
/// `atan2`'s series argument is `z = y/x`, which is NOT a storage value, so
/// the storage entry cannot serve it. That is why the existing
/// [`tiny_x_deep_directed_adjust`] call sites substitute the on-grid result
/// `g` for `z`, and why that substitution is CIRCULAR: `g − z` is precisely
/// the quantity whose sign is being determined. This entry takes `y_raw` and
/// `x_raw` and never forms `z`, keeping numerator and denominator paired so
/// both setup quantities remain exact integer divisions:
///
/// ```text
///     leading term   t = ⌊10^SCALE · |y| · 2^F / x⌋
///     term ratio     y_fp = ⌊|y|² · 2^F / x²⌋           ( = ⌊z²·2^F⌋ )
/// ```
///
/// Reached only with `x_raw > 0` and `|y_raw| < x_raw` — the branch where
/// `atan2` reduces to `atan(z)` with `0 < |z| < 1` and no `±π` offset — so the
/// alternating, strictly-decreasing precondition holds and the face is
/// [`AlternatingSeries::Atan`].
///
/// The band gate derives `k` from the two DIGIT COUNTS rather than from `g`,
/// which keeps the whole path clear of the circularity: with `dy` digits in
/// `|y|` and `dx` in `x`, `z` lies in `(10^(dy−dx−1), 10^(dy−dx+1))`, so
/// `k = dx − dy` is a LOWER bound on the true leading-digit position and
/// gating on it is conservative — it can only decline to attempt the proof,
/// never admit an argument outside the band.
pub(crate) fn adjust_alternating_bracket_ratio<St: BigInt, Wk: BigInt, const SCALE: u32>(
    rounded: St,
    y_raw: St,
    x_raw: St,
    mode: RoundingMode,
) -> Option<St>
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::int::types::compute_limbs::ComputeLimbs;

    if crate::support::rounding::is_nearest_mode(mode) || SCALE == 0 {
        return None;
    }
    let zero = <St as BigInt>::ZERO;
    if y_raw == zero || x_raw <= zero {
        return None;
    }
    let abs_y = if y_raw < zero { zero - y_raw } else { y_raw };
    if abs_y >= x_raw {
        return None; // |z| ≥ 1 — outside the reduced branch and the band
    }
    let dy = dec_digits_g::<St>(abs_y);
    let dx = dec_digits_g::<St>(x_raw);
    if dy == 0 || dx <= dy {
        return None;
    }
    let k = dx - dy; // conservative lower bound on the leading-digit position
    if k == 0 || SCALE / k + 1 > TINY_X_DEEP_JMAX {
        return None;
    }
    let rho = if rounded < zero { zero - rounded } else { rounded };

    // `·2^F` is applied throughout as a whole-limb offset (`F = Wk::BITS`), so
    // no bit shift is needed here — the products are written into the window.
    let limb_count = <Wk as BigInt>::LIMBS;
    let double_limbs = 2 * limb_count;

    let mut nb_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    let mut xb_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    let mut y_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::single_u64();
    let mut t_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
    let mut bd_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
    let mut pr_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::quad_u64();
    let mut qt_b = <<Wk as BigInt>::Scratch as ComputeLimbs>::quad_u64();
    let nb = nb_b.as_mut();
    let xb = xb_b.as_mut();
    let y = y_b.as_mut();
    let t = t_b.as_mut();
    let bd = bd_b.as_mut();
    let pr = pr_b.as_mut();
    let qt = qt_b.as_mut();
    if nb.len() < limb_count
        || xb.len() < limb_count
        || y.len() < limb_count
        || t.len() < double_limbs
        || bd.len() < double_limbs
        || pr.len() < 4 * limb_count
        || qt.len() < 4 * limb_count
    {
        return None;
    }

    eg::unpack_mag::<Wk>(abs_y.resize_to::<Wk>(), nb);
    eg::unpack_mag::<Wk>(x_raw.resize_to::<Wk>(), xb);
    let abs_y_len = sig_len(&nb[..limb_count]);
    let x_len = sig_len(&xb[..limb_count]);

    // ── the term ratio: `y_fp = ⌊|y|²·2^F / x²⌋` ──────────────────────────
    for p in pr.iter_mut() {
        *p = 0;
    }
    if limb_count + 2 * abs_y_len > pr.len() || 2 * x_len > bd.len() {
        return None;
    }
    // `·2^F` is a whole-limb offset, so the square is written straight into it.
    crate::int::policy::mul::dispatch_slice(&nb[..abs_y_len], &nb[..abs_y_len],
        &mut pr[limb_count..limb_count + 2 * abs_y_len]);
    for e in bd[..2 * x_len].iter_mut() {
        *e = 0;
    }
    crate::int::policy::mul::dispatch_slice(
        &xb[..x_len], &xb[..x_len], &mut bd[..2 * x_len]);
    let dividend_len = sig_len(&pr[..limb_count + 2 * abs_y_len]);
    let divisor_len = sig_len(&bd[..2 * x_len]);
    if qt.len() < dividend_len || t.len() < divisor_len || dividend_len < divisor_len {
        return None;
    }
    for q in qt.iter_mut() {
        *q = 0;
    }
    for e in t.iter_mut() {
        *e = 0;
    }
    if !bracket_div::<Wk>(&pr[..dividend_len], &bd[..divisor_len],
        &mut qt[..dividend_len], &mut t[..divisor_len])
    {
        return None;
    }
    // `|z| < 1` so `y_fp < 2^F` — at most `limb_count` limbs. Anything wider
    // means the band gate admitted a non-tiny ratio; fail closed rather than
    // truncate.
    if sig_len(&qt[..dividend_len]) > limb_count {
        return None;
    }
    y[..limb_count].copy_from_slice(&qt[..limb_count]);

    // ── the leading term: `t = ⌊10^SCALE·|y|·2^F / x⌋` ────────────────────
    let unit = crate::consts::pow10::dispatch::<Wk>(SCALE);
    eg::unpack_mag::<Wk>(unit, bd);
    let unit_len = sig_len(&bd[..limb_count]);
    for p in pr.iter_mut() {
        *p = 0;
    }
    if limb_count + unit_len + abs_y_len > pr.len() {
        return None;
    }
    crate::int::policy::mul::dispatch_slice(&bd[..unit_len], &nb[..abs_y_len],
        &mut pr[limb_count..limb_count + unit_len + abs_y_len]);
    let term_dividend_len = sig_len(&pr[..limb_count + unit_len + abs_y_len]);
    if qt.len() < term_dividend_len || bd.len() < x_len || term_dividend_len < x_len {
        return None;
    }
    for q in qt.iter_mut() {
        *q = 0;
    }
    for e in bd.iter_mut() {
        *e = 0;
    }
    if !bracket_div::<Wk>(&pr[..term_dividend_len], &xb[..x_len],
        &mut qt[..term_dividend_len], &mut bd[..x_len])
    {
        return None;
    }
    // `T_1 = 10^SCALE·|z| < 10^SCALE`, so the scaled leading term fits `2N`.
    if sig_len(&qt[..term_dividend_len]) > double_limbs {
        return None;
    }
    t[..double_limbs].copy_from_slice(&qt[..double_limbs]);

    alternating_bracket_core::<St, Wk>(
        rounded,
        rho,
        mode,
        AlternatingSeries::Atan,
        t,
        y,
        nb,
        bd,
        pr,
    )
}

/// The shared bracket loop — everything after the two setup quantities.
///
/// Split out so the storage faces ([`adjust_alternating_bracket`]) and the
/// RATIO face ([`adjust_alternating_bracket_ratio`], `atan2`) run the SAME
/// kernel and differ only in how they obtain those two quantities: `term`, the
/// leading term scaled by `2^F`, and `sq_ratio = ⌊x²·2^F⌋`, the term-ratio
/// multiplier. Everything that decides anything — the recurrence, the
/// brackets, the error bound, the fail-closed paths — lives here, once.
///
/// `term` is consumed (rewritten in place as the recurrence advances); `mag_scratch`,
/// `anchor` and `product` are caller-owned scratch, already free by the time the setup
/// has produced `term` and `sq_ratio`, and are reused here rather than re-allocated.
#[allow(clippy::too_many_arguments)]
fn alternating_bracket_core<St: BigInt, Wk: BigInt>(
    rounded: St,
    grid_magnitude: St,
    mode: RoundingMode,
    series: AlternatingSeries,
    term: &mut [u64],
    sq_ratio: &[u64],
    mag_scratch: &mut [u64],
    anchor: &mut [u64],
    product: &mut [u64],
) -> Option<St>
where
    <Wk as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::int::algos::support::limbs as lb;
    use crate::int::types::compute_limbs::ComputeLimbs;

    let work_limbs = <Wk as BigInt>::LIMBS;
    let acc_limbs = 2 * work_limbs;
    let f_bits = <Wk as BigInt>::BITS;
    let zero = <St as BigInt>::ZERO;

    let mut prev_sum_buf = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
    let mut cur_sum_buf = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
    let prev_sum = prev_sum_buf.as_mut();
    let cur_sum = cur_sum_buf.as_mut();
    if prev_sum.len() < acc_limbs
        || cur_sum.len() < acc_limbs
        || term.len() < acc_limbs
        || sq_ratio.len() < work_limbs
    {
        return None;
    }

    // The fixed comparison anchor `ρ·2^F`.
    eg::unpack_mag::<Wk>(grid_magnitude.resize_to::<Wk>(), mag_scratch);
    lb::shl(&mag_scratch[..work_limbs], f_bits, &mut anchor[..acc_limbs]);

    cur_sum[..acc_limbs].copy_from_slice(&term[..acc_limbs]);
    for e in prev_sum.iter_mut() {
        *e = 0;
    }
    let mut j = series.first_index();
    let mut subtract = true;
    let max_steps = (TINY_X_DEEP_JMAX as usize + 3) / 2;

    // ── the error bound, applied OUTWARD at both bracket ends ─────────────
    // The dominant per-step deficit is NOT the two truncations: it is the
    // truncation in `sq_ratio = ⌊x²·2^F⌋`, amplified by the term it multiplies.
    // With `sq_ratio = x²·2^F − δ`, `δ ∈ [0,1)`,
    //
    //     ⌊term·sq_ratio / 2^F⌋  =  t·x² − t·δ/2^F   and   term/2^F = T,
    //
    // so one step loses up to `T` — the term's own ULP magnitude — plus 2 for
    // the two floors. Terms decrease (`a_j < b_j` and `x² < 1`), so the
    // LEADING term `T_1` dominates every `T_j`; a term error accumulates at
    // most `m` of those and a partial sum at most `m` term errors. Hence
    // `m²·(T_1 + 2)` bounds the slack for every comparison, computed once.
    // `T_1` is the leading term's integer part, sitting in the top half of
    // `term` — the fixed point makes it a slice, not a computation.
    let mut slack_buf = <<Wk as BigInt>::Scratch as ComputeLimbs>::double_u64();
    let slack = slack_buf.as_mut();
    if slack.len() < acc_limbs {
        return None;
    }
    for e in slack.iter_mut() {
        *e = 0;
    }
    slack[..work_limbs].copy_from_slice(&term[work_limbs..acc_limbs]);
    lb::add_assign(&mut slack[..acc_limbs], &[2]);
    let steps = max_steps as u64;
    let slack_len_pre = sig_len(&slack[..acc_limbs]);
    if slack_len_pre + 1 > acc_limbs {
        return None;
    }
    let slack_len = mul_small(slack, slack_len_pre, steps * steps, product);
    // The whole argument rests on the slack staying BELOW one ULP (`2^F`, i.e.
    // `work_limbs` limbs). A tier violating that — it needs `Wk::BITS > ~3.4·SCALE` —
    // would make every comparison below meaningless, so it is CHECKED here
    // rather than assumed from the tier table.
    if slack_len > work_limbs {
        return None;
    }

    for _ in 0..max_steps {
        // ── next term: `term ← ⌊⌊term·sq_ratio / 2^F⌋ · a_j / b_j⌋` ──────
        let (ratio_num, ratio_den) = series.ratio(j);
        for p in product.iter_mut() {
            *p = 0;
        }
        if 3 * work_limbs > product.len() {
            return None;
        }
        crate::int::policy::mul::dispatch_slice(
            &term[..acc_limbs],
            &sq_ratio[..work_limbs],
            &mut product[..3 * work_limbs],
        );
        lb::shr(&product[..3 * work_limbs], f_bits, &mut term[..acc_limbs]);
        let mut term_len = sig_len(&term[..acc_limbs]);
        if ratio_num != 1 {
            if term_len + 1 > acc_limbs {
                return None;
            }
            term_len = mul_small(term, term_len, ratio_num, product);
        }
        if ratio_den != 1 {
            for p in product[..term_len].iter_mut() {
                *p = 0;
            }
            let mut remainder = [0u64; 1];
            // A single-limb divisor, so this takes the scratch-free `Rem` arm;
            // routed through the same helper so no divide in this kernel can
            // reach a build-max engine.
            if !bracket_div::<Wk>(
                &term[..term_len], &[ratio_den], &mut product[..term_len], &mut remainder)
            {
                return None;
            }
            term[..term_len].copy_from_slice(&product[..term_len]);
            for e in term[term_len..acc_limbs].iter_mut() {
                *e = 0;
            }
        }
        j += 2;

        // ── fold it in; `prev_sum`/`cur_sum` now straddle the value ─────
        prev_sum[..acc_limbs].copy_from_slice(&cur_sum[..acc_limbs]);
        let out_of_range = if subtract {
            lb::sub_assign(&mut cur_sum[..acc_limbs], &term[..acc_limbs])
        } else {
            lb::add_assign(&mut cur_sum[..acc_limbs], &term[..acc_limbs])
        };
        if out_of_range {
            return None; // borrow / carry out of range — fail closed
        }
        let (lo, hi): (&[u64], &[u64]) = if subtract {
            (&cur_sum[..acc_limbs], &prev_sum[..acc_limbs])
        } else {
            (&prev_sum[..acc_limbs], &cur_sum[..acc_limbs])
        };

        // ── the two unit-interval brackets, errors applied OUTWARD ────────
        let mut expanding: Option<bool> = None;
        // `ρ ≤ lo` and `hi ≤ ρ + 1`.
        product[..acc_limbs].copy_from_slice(&anchor[..acc_limbs]);
        lb::add_assign(&mut product[..acc_limbs], &slack[..acc_limbs]);
        if lb::cmp(lo, &product[..acc_limbs]) >= 0 {
            product[..acc_limbs].copy_from_slice(&anchor[..acc_limbs]);
            lb::add_assign(&mut product[work_limbs..acc_limbs], &[1]);
            lb::sub_assign(&mut product[..acc_limbs], &slack[..acc_limbs]);
            if lb::cmp(hi, &product[..acc_limbs]) <= 0 {
                expanding = Some(true);
            }
        }
        // `ρ − 1 ≤ lo` and `hi ≤ ρ`. Skipped at `ρ = 0`, where `ρ − 1` is not
        // a magnitude.
        if expanding.is_none() && grid_magnitude != zero {
            product[..acc_limbs].copy_from_slice(&anchor[..acc_limbs]);
            lb::sub_assign(&mut product[work_limbs..acc_limbs], &[1]);
            lb::add_assign(&mut product[..acc_limbs], &slack[..acc_limbs]);
            if lb::cmp(lo, &product[..acc_limbs]) >= 0 {
                product[..acc_limbs].copy_from_slice(&anchor[..acc_limbs]);
                lb::sub_assign(&mut product[..acc_limbs], &slack[..acc_limbs]);
                if lb::cmp(hi, &product[..acc_limbs]) <= 0 {
                    expanding = Some(false);
                }
            }
        }
        if let Some(is_expanding) = expanding {
            let one = <St as BigInt>::ONE;
            // The step is taken from `rounded`, so that is the `ZeroFiveUp`
            // pivot's magnitude.
            let abs_rounded = if rounded < zero { zero - rounded } else { rounded };
            let rounded_mod_10 = abs_rounded.div_rem(<St as BigInt>::TEN).1.to_i128() as u8;
            return Some(if is_expanding {
                crate::support::rounding::tiny_odd_expanding_directed(
                    rounded,
                    zero,
                    one,
                    rounded_mod_10,
                    mode,
                )
            } else {
                crate::support::rounding::tiny_odd_compressing_directed(
                    rounded,
                    zero,
                    one,
                    rounded_mod_10,
                    mode,
                )
            });
        }

        subtract = !subtract;
        if lb::is_zero(&term[..acc_limbs]) {
            break; // no further resolution available in the fixed point
        }
    }
    None
}

/// Directed-rounding post-adjust for `ln` very near `x = 1` — the `ln` face of
/// [`adjust_log_near_zero`], which carries the full analysis. `ln`'s linear
/// term is `δ = raw − 10^SCALE`, so the adjust reads the gap against `one`.
#[inline]
pub(crate) fn adjust_ln_near_one<C: WideTrigCore, const SCALE: u32>(
    rounded: C::Storage,
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if crate::support::rounding::is_nearest_mode(mode) {
        return rounded;
    }
    let one = C::storage_one(SCALE);
    adjust_log_near_zero::<C::Storage, C::W>(rounded, raw - one, one, mode)
}

/// `tan` for a wide tier — generic over the tier `C`. Panics at
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
    // Analytic tiny-`x` directed decision (relocated from the policy layer) —
    // `tan(x) = x + x³/3 + …` EXPANDS (every Taylor coefficient is positive).
    if let Some(pinned) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, true) {
        return pinned;
    }
    let base_working_scale = SCALE + C::GUARD;
    let (sin_base, cos_base) =
        C::sin_cos_fixed::<SCALE>(C::to_work(raw), base_working_scale);
    if cos_base == C::zero() {
        panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let probe = C::div(sin_base, cos_base, base_working_scale);
    let extra_digits = crate::algos::trig::near_pole_tan::tan_extra_digits(
        C::bit_length(probe), base_working_scale)
        .saturating_sub(C::GUARD);
    if extra_digits == 0 {
        // Near-tie escape: a fixed-w single shot cannot see a deciding
        // digit below w (`tan(x) = x + x^3/3 + ...` lands an exact
        // rational partial on a boundary with the deciding tail deeper -
        // the asin(3e-60) family). Clear-of-band residuals keep the
        // single-shot cost; the band escalates through the walker.
        if let Some(narrowed) = round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
            probe, base_working_scale, SCALE, mode, C::storage_max(), C::storage_min(),
        ) {
            return narrowed;
        }
        return tan_walker::<C, SCALE>(raw, C::GUARD, mode);
    }
    let working_scale = base_working_scale + extra_digits;
    let (sin_w, cos_w) = C::sin_cos_fixed::<SCALE>(
        C::to_work_scaled(raw, C::GUARD + extra_digits), working_scale);
    let ratio = C::div(sin_w, cos_w, working_scale);
    if let Some(narrowed) = round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
        ratio, working_scale, SCALE, mode, C::storage_max(), C::storage_min(),
    ) {
        return narrowed;
    }
    tan_walker::<C, SCALE>(raw, C::GUARD + extra_digits, mode)
}

/// The tier-width Ziv walker for `tan` near a rounding boundary: the
/// ratio recomputed per probe at `w = SCALE + guard`, escalating from
/// the (near-pole-lifted) `base_guard_digits`. Reached only from the near-tie
/// band of the single-shot terminals above / in the rung kernel.
fn tan_walker<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    base_guard_digits: u32,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (rounded, decided) = round_to_storage_directed_decided_g::<C::Storage, C::W>(
        base_guard_digits,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            let (sin_value, cos_value) =
                C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, guard_digits), working_scale);
            if cos_value == C::zero() {
                panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
            }
            C::div(sin_value, cos_value, working_scale)
        },
    );
    // Deep sub-resolution tiny-`x` band (`j* ≥ 5`): `tan` always EXPANDS.
    // A near-pole tie (`|x| ≈ π/2`, not tiny) has `j*` far above `JMAX`, so
    // the adjust is a no-op there.
    tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        rounded, decided, raw, mode, false, <C::W as BigInt>::BITS)
}

/// `atan` for a wide tier — generic over the tier `C`. Result in
/// `(−π/2, π/2)`. Replaces the per-tier `atan_strict_<tier>` wrappers.
#[inline]
#[must_use]
pub(crate) fn atan_series<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    // Analytic tiny-`x` directed decision (relocated from the policy layer) —
    // `atan` alternates like `sin` (`atan(x) = x − x³/3 + x⁵/5 − …`).
    if let Some(pinned) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, false) {
        return pinned;
    }
    let (rounded, decided) = round_to_storage_directed_decided_g::<C::Storage, C::W>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| C::atan_fixed::<SCALE>(
            C::to_work_scaled(raw, guard_digits), SCALE + guard_digits),
    );
    // Exact bracket first — see [`sin_series`]; `atan` alternates identically.
    if let Some(bracketed) = adjust_alternating_bracket::<C::Storage, C::W, SCALE>(
        rounded, raw, mode, AlternatingSeries::Atan)
    {
        return bracketed;
    }
    tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        rounded, decided, raw, mode, true, <C::W as BigInt>::BITS)
}

/// Narrow-`GUARD` single-shot `atan` for a wide tier — generic
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
    C::round_to_storage_directed(GUARD, SCALE, mode, &mut |guard_digits| {
        C::atan_fixed::<SCALE>(C::to_work_scaled(raw, guard_digits), SCALE + guard_digits)
    })
}

/// Rung-generic `sin` — the forward-trig Series kernel run at an
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
    // Analytic tiny-`x` directed decision — the SAME pre-empt the tier
    // [`sin_series`] carries (relocated from the policy layer), so this
    // rung-dispatched path and the bare tier kernel agree.
    if let Some(pinned) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, false) {
        return pinned;
    }
    // Two-width fall-up: an unresolved-at-rung-cap near-tie reruns the
    // walker at the tier work width `C::W` (the recompute closure is the
    // tier kernel's, verbatim), so the conclusion is never weaker than
    // the tier path's — see `round_to_storage_directed_widening_g`.
    let (rounded, decided) = round_to_storage_directed_widening_decided_g::<C::Storage, Wk, C::W>(
        GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            crate::algos::trig::trig_generic::sin_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard_digits),
                working_scale,
                pi_at_rung::<Wk>(working_scale, SCALE + GUARD),
            )
        },
        |guard_digits| C::sin_fixed::<SCALE>(
            C::to_work_scaled(raw, guard_digits), SCALE + guard_digits),
    );
    // The exact alternating-series bracket first: where it closes it PROVES
    // which side of `rounded` the true value lies on, superseding the
    // `j*`-parity rule whose exactness premise fails for a multi-digit
    // significand.
    if let Some(bracketed) = adjust_alternating_bracket::<C::Storage, C::W, SCALE>(
        rounded, raw, mode, AlternatingSeries::Sin)
    {
        return bracketed;
    }
    let rounded = tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        rounded, decided, raw, mode, true, <C::W as BigInt>::BITS);
    adjust_bounded_extremum::<C, SCALE>(rounded, raw, mode)
}

/// Rung-generic `cos` — see [`sin_series_g`]. Standalone
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
    let rounded = round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
        GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            crate::algos::trig::trig_generic::cos_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard_digits),
                working_scale,
                pi_at_rung::<Wk>(working_scale, SCALE + GUARD),
            )
        },
        |guard_digits| C::cos_fixed::<SCALE>(
            C::to_work_scaled(raw, guard_digits), SCALE + guard_digits),
    );
    // See [`cos_series`] — the near-extremum grid point the bounded-extremum
    // guard cannot reach.
    if let Some(bracketed) = adjust_alternating_bracket::<C::Storage, C::W, SCALE>(
        rounded, raw, mode, AlternatingSeries::Cos)
    {
        return bracketed;
    }
    adjust_bounded_extremum::<C, SCALE>(rounded, raw, mode)
}

/// Rung-generic `tan` — see [`sin_series_g`]. One kernel covers
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
    // Analytic tiny-`x` directed decision — the SAME pre-empt the tier
    // [`tan_series`] carries (relocated from the policy layer).
    if let Some(pinned) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, true) {
        return pinned;
    }
    let base_working_scale = SCALE + GUARD;
    let (sin_base, cos_base) = crate::algos::trig::trig_generic::sin_cos_fixed::<Wk>(
        to_work_scaled_g::<C::Storage, Wk>(raw, GUARD),
        base_working_scale,
        pi_at_rung::<Wk>(base_working_scale, base_working_scale),
    );
    if cos_base == eg::zero::<Wk>() {
        panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let probe = eg::div::<Wk>(sin_base, cos_base, base_working_scale);
    if !NEAR_POLE {
        // Near-tie escape — see [`tan_series`]: clear-of-band residuals
        // keep the single-shot cost; the band escalates (rung first,
        // tier fall-up).
        if let Some(narrowed) = round_to_storage_clear_of_tie_g::<C::Storage, Wk>(
            probe, base_working_scale, SCALE, mode, C::storage_max(), C::storage_min(),
        ) {
            return narrowed;
        }
        return tan_walker_rung_g::<C, Wk, SCALE>(raw, GUARD, mode);
    }
    let extra_raw = crate::algos::trig::near_pole_tan::tan_extra_digits(
        eg::bit_length::<Wk>(probe), base_working_scale);
    let extra_digits = if SUB_GUARD { extra_raw.saturating_sub(GUARD) } else { extra_raw };
    if extra_digits == 0 {
        if let Some(narrowed) = round_to_storage_clear_of_tie_g::<C::Storage, Wk>(
            probe, base_working_scale, SCALE, mode, C::storage_max(), C::storage_min(),
        ) {
            return narrowed;
        }
        return tan_walker_rung_g::<C, Wk, SCALE>(raw, GUARD, mode);
    }
    // Near-pole recompute at the tier work width (the `w` here is off the
    // hot `SCALE + GUARD` path, so π comes from the runtime-keyed table —
    // exactly the per-tier `pi_cf` fallback the tier path takes).
    let working_scale = base_working_scale + extra_digits;
    let (sin_w, cos_w) = crate::algos::trig::trig_generic::sin_cos_fixed::<C::W>(
        to_work_scaled_g::<C::Storage, C::W>(raw, GUARD + extra_digits),
        working_scale,
        crate::consts::pi_by_working_scale::<C::W>(
            working_scale,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        ),
    );
    let ratio = eg::div::<C::W>(sin_w, cos_w, working_scale);
    if let Some(narrowed) = round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
        ratio, working_scale, SCALE, mode, C::storage_max(), C::storage_min(),
    ) {
        return narrowed;
    }
    tan_walker::<C, SCALE>(raw, GUARD + extra_digits, mode)
}

/// Two-width near-tie walker for the rung `tan` shapes: the ratio
/// recomputed per probe at the rung `Wk` (π from the same per-scale
/// table), an unresolved-at-rung-cap walk falling up to the tier-width
/// [`tan_walker`] closure. Reached only from the near-tie band.
#[cfg(feature = "_wide-support")]
fn tan_walker_rung_g<C: WideTrigCore, Wk: BigInt, const SCALE: u32>(
    raw: C::Storage,
    base_guard_digits: u32,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    let base_working_scale = SCALE + base_guard_digits;
    let (rounded, decided) = round_to_storage_directed_widening_decided_g::<C::Storage, Wk, C::W>(
        base_guard_digits,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            let (sin_value, cos_value) = crate::algos::trig::trig_generic::sin_cos_fixed::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard_digits),
                working_scale,
                pi_at_rung::<Wk>(working_scale, base_working_scale),
            );
            if cos_value == eg::zero::<Wk>() {
                panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
            }
            eg::div::<Wk>(sin_value, cos_value, working_scale)
        },
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            let (sin_value, cos_value) =
                C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, guard_digits), working_scale);
            if cos_value == C::zero() {
                panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
            }
            C::div(sin_value, cos_value, working_scale)
        },
    );
    // Deep sub-resolution tiny-`x` band (`j* ≥ 5`): `tan` always EXPANDS.
    tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        rounded, decided, raw, mode, false, <C::W as BigInt>::BITS)
}

/// Rung-generic `atan` — the inverse-tangent kernel run at an
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
/// (`forward_rung::atan`).
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
    // Analytic tiny-`x` directed decision — the SAME pre-empt the tier
    // [`atan_series`] carries (relocated from the policy layer).
    if let Some(pinned) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, false) {
        return pinned;
    }
    let (rounded, decided) = if DIRECTED {
        // Two-width fall-up — see [`sin_series_g`].
        round_to_storage_directed_widening_decided_g::<C::Storage, Wk, C::W>(
            GUARD,
            SCALE,
            mode,
            C::storage_max(),
            C::storage_min(),
            |guard_digits| {
                let working_scale = SCALE + guard_digits;
                crate::algos::trig::trig_generic::atan_fixed::<Wk>(
                    to_work_scaled_g::<C::Storage, Wk>(raw, guard_digits),
                    working_scale,
                    pi_at_rung::<Wk>(working_scale, SCALE + GUARD),
                )
            },
            |guard_digits| C::atan_fixed::<SCALE>(
                C::to_work_scaled(raw, guard_digits), SCALE + guard_digits),
        )
    } else {
        // Band shape: same Ziv-escalated two-width walker, from the band
        // guard (the single-shot it replaces could not see a deciding
        // digit below the band's fixed working scale — see
        // [`atan_narrow`]). `DIRECTED` still selects the policy-side
        // out-of-budget fallback kernel; the narrowing machinery is one.
        let base_working_scale = SCALE + GUARD;
        round_to_storage_directed_widening_decided_g::<C::Storage, Wk, C::W>(
            GUARD,
            SCALE,
            mode,
            C::storage_max(),
            C::storage_min(),
            |guard_digits| {
                let working_scale = SCALE + guard_digits;
                crate::algos::trig::trig_generic::atan_fixed::<Wk>(
                    to_work_scaled_g::<C::Storage, Wk>(raw, guard_digits),
                    working_scale,
                    pi_at_rung::<Wk>(working_scale, base_working_scale),
                )
            },
            |guard_digits| C::atan_fixed::<SCALE>(
                C::to_work_scaled(raw, guard_digits), SCALE + guard_digits),
        )
    };
    // Deep sub-resolution band (`j* ≥ 5`): `atan` alternates like `sin`.
    // Exact bracket first — see [`sin_series`].
    if let Some(bracketed) = adjust_alternating_bracket::<C::Storage, C::W, SCALE>(
        rounded, raw, mode, AlternatingSeries::Atan)
    {
        return bracketed;
    }
    tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        rounded, decided, raw, mode, true, <C::W as BigInt>::BITS)
}

/// `π` at working scale `w` in the rung integer `Wk`: the per-scale
/// constant table keyed on the CONST base working scale on the hot path
/// (`w == base_w`, const-folds per monomorphisation — the rung sibling
/// of the per-tier `pi_cf`), the runtime-keyed lookup on the Ziv
/// escalation path. Value-identical either way (same table entry).
#[cfg(feature = "_wide-support")]
#[inline]
pub(crate) fn pi_at_rung<Wk: BigInt>(working_scale: u32, base_working_scale: u32) -> Wk {
    if working_scale == base_working_scale {
        crate::consts::pi_by_scale::<Wk>(
            base_working_scale, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    } else {
        crate::consts::pi_by_working_scale::<Wk>(
            working_scale,
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
// (`storage_max`/`storage_min`). The per-tier macro forwards pass `<$Storage>::MAX/MIN`
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
/// storage, so the bounds check is vacuously true and skipped — `storage_max` /
/// `storage_min` cannot even be represented in `S` (a down-resize would truncate
/// their magnitude into garbage bounds). The `LIMBS` compare const-folds per
/// monomorphisation.
#[inline]
fn narrow_range_checked_g<St: BigInt + Copy, S: BigInt>(
    signed: S, storage_max: St, storage_min: St) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    if <S as BigInt>::LIMBS >= <St as BigInt>::LIMBS {
        let max_w = BigInt::resize_to::<S>(storage_max);
        let min_w = BigInt::resize_to::<S>(storage_min);
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
    let is_negative = signed < <S as BigInt>::ZERO;
    let mag = if is_negative { -signed } else { signed };
    let mut buf =
        <S::Scratch as crate::int::types::compute_limbs::ComputeLimbs>::single_u64();
    crate::algos::exp::exp_generic::unpack_mag(mag, buf.as_mut());
    St::from_mag_sign_u64(buf.as_ref(), is_negative)
}

/// Work-int-generic narrowing of a `working_value` (at `working_scale`) down
/// to storage scale `target`, rounded under `mode`, into storage `St`.
/// `storage_max`/`storage_min` are `St::MAX`/`MIN`, caller-supplied.
#[inline]
pub(crate) fn round_to_storage_with_g<St: BigInt + Copy, S: BigInt>(
    working_value: S,
    working_scale: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let shift = working_scale - target;
    let rounded = if shift == 0 {
        working_value
    } else if shift <= 38 {
        crate::algos::support::mg_divide::div_wide_pow10::<S>(working_value, shift, mode)
    } else {
        crate::algos::support::rescale::dispatch_wide_pow10::<S>(working_value, shift, mode)
    };
    narrow_range_checked_g::<St, S>(rounded, storage_max, storage_min)
}

/// Absolute floor (`10^4` work-integer units) separating a genuine deciding-
/// digit SIGNAL from the kernel's own working-scale rounding NOISE. The strict
/// kernels compute to a few-ULP working accuracy (≈10²–10³ work units of
/// error); a residual-to-boundary distance above this is a real deciding digit,
/// below it is noise. Once a deciding term is representable its signal grows
/// 10× per extra working digit, so it clears the floor within a handful of
/// digits. Using an ABSOLUTE floor (not a relative `divisor/1000`, which scales
/// with the working scale and so never fires for a near-min input) is what lets
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
fn dec_digits_g<S: BigInt>(value: S) -> u32 {
    let bit_len = <S as BigInt>::BITS - value.leading_zeros();
    let mut digits = ((bit_len as u64 * 30_103) / 100_000) as u32 + 1;
    if digits > 1 && value < crate::consts::pow10::dispatch::<S>(digits - 1) {
        digits -= 1;
    }
    digits
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
///
/// A kernel-supplied [`TailSign`] takes precedence over all of that where it
/// can speak: a NEAREST probe whose residual is exactly half is decided by
/// the tag outright (see `tag_decides` in the body — the truth is strictly
/// off the boundary, so the tie-break must not run), returned `resolved`;
/// and the DIRECTED unresolved endgame reads the base probe's tag before
/// falling back to the `never_exact` blanket. The blanket remains the
/// untagged fallback — the Smith-chain `exp` path and the hyperbolics reach
/// here with `None` at every probe.
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
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    never_exact: bool,
    storage_max: St,
    storage_min: St,
    recompute: &mut dyn FnMut(u32) -> (S, Option<TailSign>),
) -> (St, bool)
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::support::rounding::{is_nearest_mode, should_bump, RoundingMode};
    let lit = |n: i128| <S as BigInt>::from_i128(n);
    let pow10 = |n: u32| crate::consts::pow10::dispatch::<S>(n);
    let bit_length = |value: S| -> u32 {
        let magnitude = if value < <S as BigInt>::ZERO { -value } else { value };
        <S as BigInt>::BITS - magnitude.leading_zeros()
    };
    let floor = pow10(ZIV_RESOLVE_FLOOR_POW10);
    // The kernels' error is value-RELATIVE (~an ULP of the value at the
    // working scale), so a result carrying `int_digits` integer digits has a
    // noise floor of ~10^int_digits WORKING UNITS regardless of `w`: a
    // fractional digit at depth `p` only clears that noise once
    // `w > p + int_digits`. The probe horizon therefore extends by
    // `int_digits` (the width cap already subtracts them, protecting the
    // kernel's internal headroom).
    let max_guard_digits_for = |int_digits: u32| -> u32 {
        let cap = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        cap.saturating_sub(target)
            .min((ZIV_PRECISION_HORIZON + int_digits).saturating_sub(target))
            .max(base_guard_digits)
    };
    let int_digits_of = |value: St| -> u32 {
        let widened = BigInt::resize_to::<S>(value);
        let magnitude = if widened < lit(0) { -widened } else { widened };
        ((bit_length(magnitude) as u64 * 30103 / 100_000) as u32 + 1).saturating_sub(target)
    };
    let range_check = |signed: S| -> St { narrow_range_checked_g::<St, S>(signed, storage_max, storage_min) };
    let finish = |neg: bool, q: S, bump: bool| -> St {
        let q_mag = if bump { q + lit(1) } else { q };
        range_check(if neg { -q_mag } else { q_mag })
    };
    // Leading fractional-digit position of the deciding residual `dist`
    // (working units at scale `target + g`) — the cross-depth confirmation
    // key: a genuine deciding term keeps this position across depths.
    let pos_of = |dist: S, guard_digits: u32| -> u32 {
        target + guard_digits - dec_digits_g::<S>(dist) + 1
    };
    // One working-scale probe: `(neg, q, rem, divisor)` of the recomputed
    // value at `guard_digits`, magnitude split at the storage grid.
    let mut probe = |guard_digits: u32| -> (bool, S, S, S, Option<TailSign>) {
        let (working_value, tail) = recompute(guard_digits);
        let neg = working_value < lit(0);
        let mag = if neg { -working_value } else { working_value };
        let divisor = pow10(guard_digits);
        // Exact per-width Knuth scratch (the narrow build's blanket is sized
        // to its 2-limb storage; the walkers probe in `Int<24>`).
        let (q, rem) = crate::algos::exp::exp_generic::div_rem_exact(mag, divisor);
        (neg, q, rem, divisor, tail)
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
            let q_mod_10 = q.div_rem(lit(10)).1.to_i128() as u8;
            finish(neg, q, should_bump(mode, cmp_r, q_mod_10, !neg))
        };
        // A tagged EXACT half is not a tie. A [`TailSign`] is only ever
        // produced with the kernel's accumulated error proven exactly zero
        // (see `expm1_fixed_inner` — the sole producer feeding this walker,
        // via `exp_fixed_tagged`'s direct-series branch), so the probe's
        // value is the exact partial sum and the true value sits strictly on
        // the tag's side of it — strictly off the half boundary. The
        // neighbour on that side is nearer outright, whatever the nearest
        // mode, and the tie-break must not run: this is the identical rule
        // `round_to_storage_directed_tagged_impl_g`'s `nearest_narrow`
        // already applies on the expm1/log1p path (issue #95 — all three
        // nearest modes rounding `exp(-1e-462)` up at `D1232<924>` where the
        // truth is strictly below half). The verdict is a PROOF, so it is
        // returned `resolved`: escalating could only re-derive the same
        // side, and for the `exp(±10^-k)` family at `2k = SCALE` the
        // deciding term (digit `3k`) outruns every reachable depth anyway.
        // `rem == div - rem` (never `rem + rem == div`, which could wrap) is
        // the exact-half test; `rem == 0` can never satisfy it, so the
        // degenerate guard-0 divisor needs no separate guard.
        //
        // NEAREST-ONLY BY PLACEMENT: this closure is reachable only from the
        // `is_nearest_mode` branch it is defined in, and must never be called
        // from the directed path — it carries no `mode` and would override a
        // directed round the residual has already decided. The `never_exact`
        // blanket it bypasses stays the UNTAGGED fallback; where that
        // blanket's own direction is and is not proven is recorded in
        // issue #96.
        let tag_decides = |neg: bool, q: S, rem: S, div: S, tail: Option<TailSign>| -> Option<St> {
            match tail {
                Some(t) if rem == div - rem => {
                    Some(finish(neg, q, (t == TailSign::Above) == !neg))
                }
                _ => None,
            }
        };
        let (neg0, q0, rem0, div0, tail0) = probe(base_guard_digits);
        if let Some(tagged) = tag_decides(neg0, q0, rem0, div0, tail0) {
            return (tagged, true);
        }
        let half0 = div0 / lit(2);
        let dist0 = if rem0 < half0 { half0 - rem0 } else { rem0 - half0 };
        if dist0 > pow10(base_guard_digits) / lit(1000) {
            return (round_half(neg0, q0, rem0, div0), true); // not near a half-ULP tie
        }
        let lo = round_half(neg0, q0, rem0, div0);
        let max_guard_digits = max_guard_digits_for(int_digits_of(lo));
        // Cross-depth confirmation: the kernels' working-scale error can reach
        // well past the absolute noise floor (measured ~10^12 units at some
        // depths), but noise always sits in the BOTTOM digits of whatever `w`
        // produced it, while a genuine deciding term keeps a depth-independent
        // fractional position. A probe's signal `(position, side)` is therefore
        // trusted only once a probe at a DIFFERENT depth reproduces it.
        let mut pending: Option<(u32, bool)> = if dist0 > floor {
            Some((pos_of(dist0, base_guard_digits), rem0 > half0))
        } else {
            None
        };
        let mut guard_digits = base_guard_digits;
        loop {
            if guard_digits >= max_guard_digits {
                // Cap reached without a confirmed deciding term. An unconfirmed
                // signal from the deepest probe gets ONE shifted confirm probe
                // (real positions reproduce; noise tracks the bottom of `w`).
                if let Some((pending_position, pending_side)) = pending {
                    let back = ZIV_RESOLVE_FLOOR_POW10 + 3;
                    if max_guard_digits > base_guard_digits + back {
                        let confirm_guard_digits = max_guard_digits - back;
                        let (neg, q, rem, div, tail) = probe(confirm_guard_digits);
                        if let Some(tagged) = tag_decides(neg, q, rem, div, tail) {
                            return (tagged, true);
                        }
                        let half = div / lit(2);
                        let dist = if rem < half { half - rem } else { rem - half };
                        if dist > floor {
                            let position = pos_of(dist, confirm_guard_digits);
                            if (rem > half) == pending_side
                                && position.abs_diff(pending_position) <= 1
                            {
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
            let step = (target + base_guard_digits).max(base_guard_digits);
            let next_guard_digits = guard_digits.saturating_add(step).min(max_guard_digits);
            let (neg, q, rem, div, tail) = probe(next_guard_digits);
            if let Some(tagged) = tag_decides(neg, q, rem, div, tail) {
                return (tagged, true);
            }
            let half = div / lit(2);
            let hi_dist = if rem < half { half - rem } else { rem - half };
            if hi_dist > floor {
                let position = pos_of(hi_dist, next_guard_digits);
                let above = rem > half;
                if let Some((pending_position, pending_side)) = pending {
                    if above == pending_side
                        && position.abs_diff(pending_position) <= 1
                    {
                        // Confirmed deciding digit — trustworthy.
                        return (round_half(neg, q, rem, div), true);
                    }
                }
                pending = Some((position, above));
            }
            guard_digits = next_guard_digits;
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
                // `q` is the toward-zero magnitude, so its last decimal
                // digit is the `ZeroFiveUp` pivot.
                RoundingMode::AwayFromZero => true,
                RoundingMode::ZeroFiveUp => {
                    matches!(q.div_rem(lit(10)).1.to_i128(), 0 | 5)
                }
                _ => unreachable!(),
            };
        finish(neg, q, bump)
    };
    let (neg0, q0, rem0, div0, tail0) = probe(base_guard_digits);
    let dist0 = if rem0 < div0 - rem0 { rem0 } else { div0 - rem0 };
    if dist0 > pow10(base_guard_digits) / lit(1000) {
        return (dir_round(neg0, q0, rem0), true); // clear of a grid line
    }
    let base = dir_round(neg0, q0, rem0);
    let max_guard_digits = max_guard_digits_for(int_digits_of(base));
    // Cross-depth confirmation — see the nearest branch: a probe's signal
    // `(position, side)` is trusted only once a probe at a different depth
    // reproduces it (noise tracks the bottom of `w`; real digits do not move).
    let mut pending: Option<(u32, bool)> = if dist0 > floor {
        Some((pos_of(dist0, base_guard_digits), rem0 < div0 - rem0))
    } else {
        None
    };
    let mut guard_digits = base_guard_digits;
    loop {
        if guard_digits >= max_guard_digits {
            // Cap reached without a confirmed deciding term. An unconfirmed
            // signal from the deepest probe gets ONE shifted confirm probe.
            if let Some((pending_position, pending_side)) = pending {
                let back = ZIV_RESOLVE_FLOOR_POW10 + 3;
                if max_guard_digits > base_guard_digits + back {
                    let confirm_guard_digits = max_guard_digits - back;
                    let (neg, q, rem, div, _) = probe(confirm_guard_digits);
                    let dist = if rem < div - rem { rem } else { div - rem };
                    if dist > floor {
                        let position = pos_of(dist, confirm_guard_digits);
                        if (rem < div - rem) == pending_side
                            && position.abs_diff(pending_position) <= 1
                        {
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
            // Which way the sub-resolution tail moves the MAGNITUDE. `away` =
            // further from zero; `!away` = toward it. A kernel-supplied
            // [`TailSign`] is a proof about the SIGNED value, so it converts
            // through the result's own sign — the same reading
            // `round_to_storage_directed_tagged_impl_g` makes, and the same
            // `q != 0` guard: at `q_grid == 0` a "toward zero" tail would put
            // the magnitude in `(-1, 0)`, which no magnitude occupies, so the
            // tag cannot apply and the blanket rule stands.
            //
            // `never_exact` is the untagged fallback, NOT a second opinion:
            // it is this walker's blanket "the tail is always away from zero",
            // which is what the tag exists to replace where it can be proved.
            // It is kept for the untagged path because the hyperbolics reach
            // this same endgame through `round_to_storage_widening_g` with the
            // flag set, and dropping it there would move them.
            // `proven`: the TAG decided this endgame, not the blanket. The
            // tag is a proof, and a proof needs no wider retry — the same
            // rule the nearest branch's `tag_decides` applies (issue #95):
            // the `S2` walk could only run its own ladder to this same
            // endgame and re-derive the same side from the same producer.
            // The blanket verdicts stay UNRESOLVED as before, so the
            // widening caller still retries those at the wider integer.
            //
            // The `q_grid` snap above assumes `rem0` is kernel noise. For a
            // TAGGED probe that is false: the tag is only emitted with the
            // accumulated error proven exactly zero, so `q0`/`rem0` are the
            // exact partial sum and the snap discards a real residual. The
            // tag then proves the tail's side of the PROBE's value while the
            // code reads it as the side of the GRID line — which differ
            // whenever `rem0 != 0` and `|tail| < rem0/div0`. Issue #98; the
            // fix is to skip the snap for a tagged probe rather than to gate
            // this arm. Recorded here because `proven` removes the wider
            // retry, so this endgame's answer is final where it once was not.
            let (away, proven) = match tail0 {
                Some(t) if q_grid != lit(0) => {
                    (Some((t == TailSign::Above) == !neg0), true)
                }
                _ => {
                    if never_exact {
                        (Some(true), false)
                    } else {
                        (None, false)
                    }
                }
            };
            // `finish` can only ADD to the magnitude, so a tail pointing TOWARD
            // zero is not expressible as a bump — it is expressed by lowering
            // the BASE one ULP and letting the ordinary directed rule bump back
            // up. `Some(true)` reproduces the previous `never_exact` narrowing
            // exactly (same base, same bump); `Some(false)` is the branch that
            // was previously unreachable and wrong.
            let (q_base, residual_present) = match away {
                Some(true) => (q_grid, true),
                Some(false) => (q_grid - lit(1), true),
                None => (q_grid, false),
            };
            let tail_bump = residual_present
                && match mode {
                    RoundingMode::Trunc => false,
                    RoundingMode::Floor => neg0,
                    RoundingMode::Ceiling => !neg0,
                    // The bump steps away from zero off `q_base`, so that
                    // is the toward-zero value the `ZeroFiveUp` pivot reads.
                    RoundingMode::AwayFromZero => true,
                    RoundingMode::ZeroFiveUp => {
                        matches!(q_base.div_rem(lit(10)).1.to_i128(), 0 | 5)
                    }
                    _ => unreachable!(),
                };
            return (finish(neg0, q_base, tail_bump), proven);
        }
        let step = (target + base_guard_digits).max(base_guard_digits);
        let next_guard_digits = guard_digits.saturating_add(step).min(max_guard_digits);
        let (neg, q, rem, div, _) = probe(next_guard_digits);
        let hi_dist = if rem < div - rem { rem } else { div - rem };
        if hi_dist > floor {
            let position = pos_of(hi_dist, next_guard_digits);
            let above = rem < div - rem;
            if let Some((pending_position, pending_side)) = pending {
                if above == pending_side
                    && position.abs_diff(pending_position) <= 1
                {
                    // Confirmed deciding digit — trustworthy.
                    return (dir_round(neg, q, rem), true);
                }
            }
            pending = Some((position, above));
        }
        guard_digits = next_guard_digits;
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
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    never_exact: bool,
    storage_max: St,
    storage_min: St,
    mut recompute1: impl FnMut(u32) -> S1,
    mut recompute2: impl FnMut(u32) -> S2,
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_widening_tail_signed_g::<St, S1, S2>(
        base_guard_digits,
        target,
        mode,
        never_exact,
        storage_max,
        storage_min,
        |guard_digits| (recompute1(guard_digits), None),
        |guard_digits| (recompute2(guard_digits), None),
    )
}

/// [`round_to_storage_widening_g`] for a kernel that can PROVE which side its
/// neglected tail falls on, reported per probe as a [`TailSign`].
///
/// Where the tag is `None` at every probe this is bit-identical to
/// [`round_to_storage_widening_g`] — the untagged wrapper above is exactly
/// that call. Where it is present, it replaces the `never_exact` blanket
/// ("the tail always moves the magnitude away from zero") with the side the
/// kernel actually proved, at both readings a residual cannot make: the
/// directed endgame's exactly-zero grid line, and a nearest probe's
/// exactly-half boundary (decided outright, `resolved`, so no widening
/// retry runs on a proof). See [`near_min_resolve_g`].
#[inline]
#[allow(clippy::too_many_arguments)]
pub(crate) fn round_to_storage_widening_tail_signed_g<St: BigInt + Copy, S1: BigInt, S2: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    never_exact: bool,
    storage_max: St,
    storage_min: St,
    mut recompute1: impl FnMut(u32) -> (S1, Option<TailSign>),
    mut recompute2: impl FnMut(u32) -> (S2, Option<TailSign>),
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (v1, resolved1) = near_min_resolve_g::<St, S1>(
        base_guard_digits, target, mode, never_exact, storage_max, storage_min, &mut recompute1,
    );
    // `S1` only proves a residual is past the probe horizon when its width
    // actually reaches it — and a result carrying integer digits raises both
    // the horizon AND the noise floor by that count (see the resolver), so
    // the reach test must include them.
    let int_digits = {
        let magnitude = if v1 < <St as BigInt>::ZERO { -v1 } else { v1 };
        let magnitude_bits = <St as BigInt>::BITS - magnitude.leading_zeros();
        ((magnitude_bits as u64 * 30103 / 100_000) as u32 + 1).saturating_sub(target)
    };
    if resolved1 || (<S1>::BITS / 8) >= ZIV_PRECISION_HORIZON + int_digits {
        return v1;
    }
    near_min_resolve_g::<St, S2>(
        base_guard_digits, target, mode, never_exact, storage_max, storage_min, &mut recompute2,
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
    working_value: S,
    working_scale: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
) -> Option<St>
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::support::rounding::{is_nearest_mode, should_bump};
    let lit = |n: i128| <S as BigInt>::from_i128(n);
    let shift = working_scale - target;
    if shift == 0 {
        // Already at storage scale: the value IS the answer (no residual).
        return Some(narrow_range_checked_g::<St, S>(
            working_value, storage_max, storage_min));
    }
    let neg = working_value < lit(0);
    let mag = if neg { -working_value } else { working_value };
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
            && should_bump(
                mode,
                rem.cmp(&(divisor - rem)),
                q.div_rem(lit(10)).1.to_i128() as u8,
                !neg,
            )
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
                // `q` is the toward-zero magnitude.
                RoundingMode::AwayFromZero => true,
                RoundingMode::ZeroFiveUp => {
                    matches!(q.div_rem(lit(10)).1.to_i128(), 0 | 5)
                }
                _ => unreachable!(),
            }
    };
    let q_mag = if bump { q + lit(1) } else { q };
    let signed = if neg { -q_mag } else { q_mag };
    Some(narrow_range_checked_g::<St, S>(signed, storage_max, storage_min))
}

/// Work-int-generic directed-rounding narrowing with Ziv escalation. `St` =
/// storage output, `S` = work integer (a rung `Wk` or the tier `W`).
#[inline]
pub(crate) fn round_to_storage_directed_g<St: BigInt + Copy, S: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(
        base_guard_digits, target, mode, false, false, storage_max, storage_min, &mut recompute,
    )
    .0
}

/// [`round_to_storage_directed_g`] for a kernel that can say which side its
/// neglected tail falls on — ALL SIX modes, not just the directed three.
///
/// `recompute` hands back its working-scale value together with the
/// [`TailSign`] for that probe. Where the sign is `None` this is
/// bit-identical to [`round_to_storage_directed_g`]; where it is present it
/// settles the two readings a residual cannot (an exactly-zero directed
/// residual, an exactly-half nearest one) — see
/// [`round_to_storage_directed_tagged_impl_g`].
///
/// The nearest modes are deliberately NOT excluded. They fail on the same
/// inputs for the same reason: a tail below the working resolution reads as
/// an exact tie, and the mode's tie-break then decides a tie that is not
/// there. One mechanism serves them all.
#[inline]
pub(crate) fn round_to_storage_tail_signed_g<St: BigInt + Copy, S: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute: impl FnMut(u32) -> (S, Option<TailSign>),
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_tagged_impl_g::<St, S>(
        base_guard_digits, target, mode, false, false, storage_max, storage_min, &mut recompute,
    )
    .0
}

/// As [`round_to_storage_directed_g`] but RETAINS the walker's `decided`
/// verdict (`false` once it gives up at the escalation cap — mode-blind). The
/// odd forward/inverse trig kernels read it to drive
/// [`tiny_x_deep_directed_adjust`]; every other caller keeps the `.0`-only
/// wrapper. Bit-identical narrowing — only the discarded boolean differs.
#[inline]
pub(crate) fn round_to_storage_directed_decided_g<St: BigInt + Copy, S: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute: impl FnMut(u32) -> S,
) -> (St, bool)
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(
        base_guard_digits, target, mode, false, false, storage_max, storage_min, &mut recompute,
    )
}

/// `never_exact` directed narrowing (an irrational-valued kernel, e.g. `exp`):
/// a zero working residual is a sub-resolution positive residual.
#[inline]
pub(crate) fn round_to_storage_directed_never_exact_g<St: BigInt + Copy, S: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(
        base_guard_digits, target, mode, false, true, storage_max, storage_min, &mut recompute,
    )
    .0
}

/// Near-special-point directed narrowing (`acosh` at 1, `atanh` at ±1):
/// force a confirm recompute even in nearest modes.
#[inline]
pub(crate) fn round_to_storage_directed_near_special_g<St: BigInt + Copy, S: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute: impl FnMut(u32) -> S,
) -> St
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    round_to_storage_directed_impl_g::<St, S>(
        base_guard_digits, target, mode, true, false, storage_max, storage_min, &mut recompute,
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
/// the at-cap base-narrowing endgame: an unresolved-at-rung tie that
/// concludes from the rung's probes can, under a DIRECTED mode, land
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
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute1: impl FnMut(u32) -> S1,
    mut recompute2: impl FnMut(u32) -> S2,
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (narrowed, resolved) = round_to_storage_directed_impl_g::<St, S1>(
        base_guard_digits, target, mode, false, false, storage_max, storage_min, &mut recompute1,
    );
    if resolved || <S1 as BigInt>::BITS >= <S2 as BigInt>::BITS {
        return narrowed;
    }
    round_to_storage_directed_impl_g::<St, S2>(
        base_guard_digits, target, mode, false, false, storage_max, storage_min, &mut recompute2,
    )
    .0
}

/// As [`round_to_storage_directed_widening_g`] but RETAINS the final `decided`
/// verdict (the rung's when it resolves or is the widest, else the tier
/// fall-up's). The odd forward/inverse trig rung kernels read it for
/// [`tiny_x_deep_directed_adjust`]. Bit-identical narrowing.
#[inline]
#[allow(clippy::too_many_arguments)]
pub(crate) fn round_to_storage_directed_widening_decided_g<St: BigInt + Copy, S1: BigInt, S2: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute1: impl FnMut(u32) -> S1,
    mut recompute2: impl FnMut(u32) -> S2,
) -> (St, bool)
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (narrowed, resolved) = round_to_storage_directed_impl_g::<St, S1>(
        base_guard_digits, target, mode, false, false, storage_max, storage_min, &mut recompute1,
    );
    if resolved || <S1 as BigInt>::BITS >= <S2 as BigInt>::BITS {
        return (narrowed, resolved);
    }
    round_to_storage_directed_impl_g::<St, S2>(
        base_guard_digits, target, mode, false, false, storage_max, storage_min, &mut recompute2,
    )
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
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    storage_max: St,
    storage_min: St,
    mut recompute1: impl FnMut(u32) -> S1,
    mut recompute2: impl FnMut(u32) -> S2,
) -> St
where
    S1::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    S2::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    let (narrowed, resolved) = round_to_storage_directed_impl_g::<St, S1>(
        base_guard_digits, target, mode, true, false, storage_max, storage_min, &mut recompute1,
    );
    if resolved || <S1 as BigInt>::BITS >= <S2 as BigInt>::BITS {
        return narrowed;
    }
    round_to_storage_directed_impl_g::<St, S2>(
        base_guard_digits, target, mode, true, false, storage_max, storage_min, &mut recompute2,
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
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    force_confirm: bool,
    never_exact: bool,
    storage_max: St,
    storage_min: St,
    recompute: &mut dyn FnMut(u32) -> S,
) -> (St, bool)
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    // A kernel that cannot say which side its neglected tail falls on gets
    // `None` at every probe, which is the untagged behaviour exactly.
    round_to_storage_directed_tagged_impl_g::<St, S>(
        base_guard_digits,
        target,
        mode,
        force_confirm,
        never_exact,
        storage_max,
        storage_min,
        &mut |guard_digits| (recompute(guard_digits), None),
    )
}

/// [`round_to_storage_directed_impl_g`] with a per-probe TAIL SIGN — the
/// walker's only source of truth for a residual it cannot resolve.
///
/// `recompute` returns its working-scale value together with
/// [`TailSign`], the side the terms it dropped put the true value on (see
/// that type for why nothing else can supply it). The sign belongs to the
/// probe that produced it, so there is no question of matching a tag to
/// the narrowing the walker ends up returning — each narrowing reads its
/// own.
///
/// It changes the outcome in exactly the two places the residual is blind,
/// and NOWHERE else:
///
/// * a DIRECTED round whose residual is exactly zero — the value is not on
///   the grid after all, it is one sub-ULP tail away from it, so the round
///   goes the tail's way instead of standing still;
/// * a NEAREST round whose residual is exactly half — not a tie after all,
///   so the neighbour on the tail's side wins outright and the mode's
///   tie-break never runs. A tag-decided probe ends the walk there,
///   `resolved` — the verdict is a proof, so the remaining ladder could
///   only re-derive it (the same first-probe rule [`near_min_resolve_g`]'s
///   nearest branch applies on the `exp` path, issue #95).
///
/// Every other input keeps the previous path: a non-zero directed residual
/// already dominates a sub-unit tail, and a nearest residual off the half
/// boundary is decided without one. With `None` the function is
/// bit-identical to [`round_to_storage_directed_impl_g`] at every argument.
#[allow(clippy::too_many_arguments)]
fn round_to_storage_directed_tagged_impl_g<St: BigInt + Copy, S: BigInt>(
    base_guard_digits: u32,
    target: u32,
    mode: RoundingMode,
    force_confirm: bool,
    never_exact: bool,
    storage_max: St,
    storage_min: St,
    recompute: &mut dyn FnMut(u32) -> (S, Option<TailSign>),
) -> (St, bool)
where
    S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::support::rounding::{is_nearest_mode, RoundingMode};

    let lit = |n: i128| <S as BigInt>::from_i128(n);
    let pow10 = |n: u32| crate::consts::pow10::dispatch::<S>(n);
    let bit_length = |value: S| -> u32 {
        let magnitude = if value < <S as BigInt>::ZERO { -value } else { value };
        <S as BigInt>::BITS - magnitude.leading_zeros()
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
        // The third element reports whether the TAG decided this narrowing —
        // a probe whose residual was exactly half with a kernel-proven tail
        // side. That verdict is a PROOF (see `TailSign`: the value is the
        // exact partial sum and the truth sits strictly on the tag's side of
        // the boundary), so the caller returns it `resolved` outright instead
        // of walking the ladder — the identical first-probe short-circuit
        // `near_min_resolve_g`'s nearest branch applies on the `exp` path
        // (issue #95): escalating could only re-derive the same side, and for
        // the deep-tie families the deciding term outruns every reachable
        // depth anyway, so the ladder previously ran to its cap only to hand
        // back this same tag-decided base narrowing.
        let mut nearest_narrow = |guard_digits: u32| -> (St, S, bool) {
            let (working_value, tail) = recompute(guard_digits);
            let neg = working_value < lit(0);
            let mag = if neg { -working_value } else { working_value };
            let divisor = pow10(guard_digits);
            // Exact per-width Knuth scratch (the narrow build's blanket is sized
            // to its 2-limb storage; the walkers probe in `Int<24>`).
            let (q, rem) = crate::algos::exp::exp_generic::div_rem_exact(mag, divisor);
            let mut tag_decided = false;
            let q_mag = if rem != lit(0) {
                let comp = divisor - rem;
                let ord = rem.cmp(&comp);
                match tail {
                    // An exact half is a TIE only while nothing is known about
                    // what lies below it. A tail sign says the value is off the
                    // boundary, so the neighbour on that side is nearer
                    // outright and the mode's tie-break must not run — it would
                    // decide a tie that is not there (`HalfToEven` holding a
                    // positive row down, `HalfTowardZero` a negative one).
                    Some(t) if ord == ::core::cmp::Ordering::Equal => {
                        tag_decided = true;
                        if (t == TailSign::Above) == !neg {
                            q + lit(1)
                        } else {
                            q
                        }
                    }
                    _ => {
                        let bump = crate::support::rounding::should_bump(
                            mode,
                            ord,
                            q.div_rem(lit(10)).1.to_i128() as u8,
                            !neg,
                        );
                        if bump { q + lit(1) } else { q }
                    }
                }
            } else {
                q
            };
            let signed = if neg { -q_mag } else { q_mag };
            let narrowed = narrow_range_checked_g::<St, S>(signed, storage_max, storage_min);
            // `divisor = 10^guard` is even for every guard >= 1 (and the
            // guard-0 degenerate `1/2 == 1 >> 1 == 0`), so the half-ULP
            // boundary is an exact one-bit shift — not a divide.
            let half = divisor >> 1;
            let dist_half = if rem < half { half - rem } else { rem - half };
            (narrowed, dist_half, tag_decided)
        };
        let (lo, dist0, decided0) = nearest_narrow(base_guard_digits);
        if decided0 {
            return (lo, true);
        }
        // Ordinary input — residual clear of the half boundary by more than the
        // (generous) `divisor/1000` near-tie band — keep the single base
        // narrowing (bit-identical to the prior single-shot path). The escalate
        // trigger stays the wide band; the absolute `floor` below is only the
        // STOP test (signal vs noise), not the escalate trigger.
        if !force_confirm && dist0 > band_of(base_guard_digits) {
            return (lo, true);
        }
        let int_digits = {
            let narrowed = BigInt::resize_to::<S>(lo);
            let magnitude = if narrowed < lit(0) { -narrowed } else { narrowed };
            let magnitude_bits = bit_length(magnitude);
            let storage_digits = (magnitude_bits as u64 * 30103 / 100_000) as u32 + 1;
            storage_digits.saturating_sub(target)
        };
        let cap_digits = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        let max_guard_digits = cap_digits
            .saturating_sub(target)
            .min(ZIV_PRECISION_HORIZON.saturating_sub(target))
            .max(base_guard_digits);
        let mut guard_digits = base_guard_digits;
        let mut best = lo;
        loop {
            if guard_digits >= max_guard_digits {
                // Cap reached without clearing the noise floor. `force_confirm`
                // (acosh/atanh) trusts its last stable narrowing; otherwise the
                // deciding digit is below the work integer's / the crate's reach
                // — return the CLEAN base narrowing (the exact-tie answer the
                // finite-precision oracle agrees with), NOT the deepest
                // narrowing (which is dominated by kernel noise at this depth).
                return (if force_confirm { best } else { lo }, false);
            }
            let step = (target + base_guard_digits).max(base_guard_digits);
            let unclamped = guard_digits.saturating_add(step);
            let next_guard_digits = unclamped.min(max_guard_digits);
            // A probe whose depth was CLAMPED by this width's escalation cap
            // diverges from the canonical (tier-width) probe sequence — any
            // conclusion drawn from it is reported UNRESOLVED so a two-width
            // caller falls up to the tier walker instead of trusting a
            // cap-limited reading (e.g. a zero remainder that is only the
            // deciding term underflowing at the clamped working scale).
            let tainted = unclamped > max_guard_digits;
            let (hi, hi_dist, hi_decided) = nearest_narrow(next_guard_digits);
            // A tag-decided probe is a proof at WHATEVER depth it fired —
            // clamping cannot taint it (the tag belongs to the probe that
            // produced it, not to the canonical probe sequence), and no
            // deeper or wider walk could re-derive anything but the same
            // side. (Today's tagged callers never pass `force_confirm`; the
            // proof would decide those walks identically if one ever did.)
            if hi_decided {
                return (hi, true);
            }
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
            guard_digits = next_guard_digits;
            best = hi;
        }
    }

    let mut directed_narrow = |guard_digits: u32| -> (S, S) {
        let working_scale = target + guard_digits;
        let (working_value, tail) = recompute(guard_digits);
        let shift = working_scale - target;
        let neg = working_value < lit(0);
        let mag = if neg { -working_value } else { working_value };
        let divisor = pow10(shift);
        // Exact per-width Knuth scratch (the narrow build's blanket is sized
        // to its 2-limb storage; the walkers probe in `Int<24>`).
        let (q, rem) = crate::algos::exp::exp_generic::div_rem_exact(mag, divisor);
        let result_positive = !neg;
        // Where the true value sits relative to `q`, in MAGNITUDE. A non-zero
        // residual already puts it above `q` and dominates any sub-unit tail;
        // `never_exact` is the near-min assertion of the same thing. `None` =
        // nothing distinguishes it from the grid line, the previous
        // behaviour. Only an exactly-zero residual lets the tail speak — and
        // only the tail can put the truth BELOW `q`, which no residual does.
        let away = if rem != lit(0) || never_exact {
            Some(true)
        } else {
            match tail {
                // `q == 0` would make a toward-zero step a negative magnitude.
                // The truth is then within one ULP of zero either way and the
                // narrowing already names it.
                Some(t) if q != lit(0) => Some((t == TailSign::Above) == result_positive),
                _ => None,
            }
        };
        let q_mag = match away {
            None => q,
            Some(away) => {
                let toward_zero = if away { q } else { q - lit(1) };
                let away_from_zero = if away { q + lit(1) } else { q };
                match mode {
                    RoundingMode::Trunc => toward_zero,
                    RoundingMode::Floor => {
                        if result_positive { toward_zero } else { away_from_zero }
                    }
                    RoundingMode::Ceiling => {
                        if result_positive { away_from_zero } else { toward_zero }
                    }
                    RoundingMode::AwayFromZero => away_from_zero,
                    RoundingMode::ZeroFiveUp => {
                        if matches!(toward_zero.div_rem(lit(10)).1.to_i128(), 0 | 5) {
                            away_from_zero
                        } else {
                            toward_zero
                        }
                    }
                    _ => unreachable!(),
                }
            }
        };
        let signed = if neg { -q_mag } else { q_mag };
        let dist = if rem < divisor - rem {
            rem
        } else {
            divisor - rem
        };
        (signed, dist)
    };

    let (mut lo, dist0) = directed_narrow(base_guard_digits);

    let band0 = band_of(base_guard_digits);
    let near_grid = force_confirm || dist0 <= band0;

    let (signed, decided) = if !near_grid {
        (lo, true)
    } else {
        // The clean base narrowing — the unresolved endgame's answer (see
        // the cap break below). Captured BEFORE the loop starts rolling
        // `lo` through the deeper probes.
        let base = lo;
        let int_digits = {
            let magnitude = if lo < lit(0) { -lo } else { lo };
            let magnitude_bits = bit_length(magnitude);
            let storage_digits = (magnitude_bits as u64 * 30103 / 100_000) as u32 + 1;
            storage_digits.saturating_sub(target)
        };
        // DELIBERATELY the bare `BITS/8` work-integer cap — NOT `.min(
        // ZIV_PRECISION_HORIZON)` like the nearest branch above. The two
        // boundaries serve different masters: nearest gives up at the horizon
        // because a half-ULP tie past the shipped ~1232-digit oracle is
        // unverifiable, and giving up early still yields the correct nearest
        // answer (snap to the grid value `G`). A DIRECTED sub-resolution
        // residual past the horizon is the opposite — its SIGN still decides
        // `G` vs `G ± 1`, and that decision is owned by the analytic
        // [`tiny_x_deep_directed_adjust`], which fires exactly when the
        // deciding term lies beyond `reach = work_bits/8 − 8` — the EXACT
        // complement of this `cap_digits` (bounded trig ⇒ `int_digits = 0`).
        // So the walker MUST own every deciding term up to `BITS/8`: clamping
        // here to the horizon would lower the give-up boundary below `reach`,
        // leaving cells with `j*·k ∈ (horizon, reach]` reported mode-blind
        // (`decided == false`) yet UN-adjusted by the helper (its `j*·k ≤
        // reach` returns `r` unchanged) — a directed mis-round. `BITS/8` is
        // also the const-table-safe cap (`pi`/`ln2`/`sincos` are provisioned
        // per width to ≈`BITS/8`), so probing to it never requests a const
        // past the generated table.
        let cap_digits = (<S>::BITS / 8).saturating_sub(int_digits + 8);
        let max_guard_digits = cap_digits.saturating_sub(target).max(base_guard_digits);

        let mut guard_digits = base_guard_digits;
        // Whether the LAST probe's grid-line distance cleared the absolute
        // noise floor — a genuine, representable deciding digit (a real
        // residual only GROWS with depth, so a final floor-clearing probe is
        // signal even without a second confirming probe; the asin(1e-38)
        // D38<38> Ceiling deviation `x³/6` at ULP-depth 77 first becomes
        // visible exactly at the rung's cap-clamped probe).
        let mut last_resolved = false;
        loop {
            if guard_digits >= max_guard_digits {
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
            let step = (target + base_guard_digits).max(base_guard_digits);
            let unclamped = guard_digits.saturating_add(step);
            let next_guard_digits = unclamped.min(max_guard_digits);
            // See the nearest branch: a cap-clamped probe departs from the
            // canonical probe sequence, so its conclusion is reported
            // UNRESOLVED for the two-width fall-up.
            let tainted = unclamped > max_guard_digits;
            let (hi, hi_dist) = directed_narrow(next_guard_digits);
            // A deciding digit is a genuine SIGNAL once its distance to the
            // grid line clears the ABSOLUTE kernel-noise floor — the same
            // rule the nearest branch applies to its half-boundary
            // distance. A relative `divisor/1000` band would scale with
            // the working scale, so a SUB-RESOLUTION residual (e.g. a
            // deep-underflow `exp`, value ≪ 1 storage ULP) could never
            // clear it and every such walk would run to the cap — where the
            // deepest probe, not the clean base, would be trusted.
            // Resolution still demands two consecutive probes agree on the
            // narrowing (`hi == lo`), a stricter consistency requirement
            // than the nearest branch's single floor-clearing probe.
            //
            // `hi_dist == lit(0)`: a probe landing EXACTLY on a grid line
            // (working-scale remainder == 0) at a depth ABOVE the true
            // deviation also counts as resolved when paired with `hi == lo`.
            // `directed_narrow` already handles the zero remainder correctly
            // (`away = Some(true)` when `rem != lit(0) || never_exact` keeps
            // the directed bump active when `never_exact` is set), so `hi` IS
            // the right directed answer; `hi == lo` confirms a second,
            // independent depth reached the same conclusion. This is sound
            // for every CURRENT ladder: the step formula (`target +
            // base_guard_digits`, ≥68 digits in the wide tiers) spans a 10^68×
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
            guard_digits = next_guard_digits;
            lo = hi;
            last_resolved = resolved;
        }
    };

    (narrow_range_checked_g::<St, S>(signed, storage_max, storage_min), decided)
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

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Ceiling,
        RoundingMode::Floor,
        RoundingMode::Trunc,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// The walked value is a never-exact sub-resolution POSITIVE, so every
    /// mode that moves a discarded remainder away from zero lands on 1:
    /// `Ceiling` (toward +∞), `AwayFromZero` (anything discarded), and
    /// `ZeroFiveUp` (the retained digit is `0`, one of its two bump
    /// digits). The rest narrow to 0.
    fn rounds_up_to_one(mode: RoundingMode) -> bool {
        matches!(
            mode,
            RoundingMode::Ceiling | RoundingMode::AwayFromZero | RoundingMode::ZeroFiveUp
        )
    }

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
            let got = run(mode, |guard_digits| {
                let probe_value = <S as BigInt>::from_i128(995)
                    * crate::consts::pow10::dispatch::<S>(guard_digits - BASE_GUARD);
                if guard_digits >= POISON_FROM { -probe_value } else { probe_value }
            });
            let want = i128::from(rounds_up_to_one(mode));
            assert_eq!(got, want, "sub-resolution positive, mode={mode:?}");
        }
    }

    // A residual at kernel-noise scale (5 work units, below the resolve
    // floor at EVERY depth — the Table-Maker's-Dilemma stand-in): the walk
    // runs to the cap unresolved, and the endgame must return the CLEAN
    // BASE narrowing (never-exact: the away-from-zero modes → 1, others
    // → 0), never the deepest probe's — which here is poisoned negative
    // and would invert Ceiling to 0 and Floor to -1.
    #[test]
    fn unresolved_cap_returns_clean_base_not_deepest_probe() {
        for mode in ALL_MODES {
            let got = run(mode, |guard_digits| {
                let probe_value = <S as BigInt>::from_i128(5);
                if guard_digits >= POISON_FROM { -probe_value } else { probe_value }
            });
            let want = i128::from(rounds_up_to_one(mode));
            assert_eq!(got, want, "noise-scale residual at cap, mode={mode:?}");
        }
    }

    // A deciding digit first visible ONLY at the cap-clamped final probe —
    // the asin(1e-38) D38<38> shape (CI fallout of the cap-endgame fix):
    // value = 1 ULP + 1.667e-77 ULPs, walked at the D57 borrow path's
    // Int<16> rung (max_guard_digits = 128 − 8 − 38 = 82, first probe 30+68 = 98
    // clamped to 82). The base probe lands EXACTLY on grid (the deviation
    // is below w = 68's resolution); the single cap-clamped probe shows the
    // genuine residual (1.667e5 work units, above the noise floor) — the
    // endgame must TRUST that resolved final probe (Ceiling → 2), not
    // discard it for the on-grid base (Ceiling → 1, the wrong answer).
    #[test]
    fn deciding_digit_first_visible_at_cap_probe_is_trusted() {
        type Rung = Int<16>;
        for (mode, want) in [
            (RoundingMode::Ceiling, 2_i128),
            (RoundingMode::Floor, 1),
            (RoundingMode::Trunc, 1),
            (RoundingMode::HalfToEven, 1),
            // The value is 1 ULP + a positive residual: `AwayFromZero`
            // steps off it like `Ceiling`, while `ZeroFiveUp` truncates —
            // the retained digit is `1`, not one of its `0`/`5` pivots.
            (RoundingMode::AwayFromZero, 2),
            (RoundingMode::ZeroFiveUp, 1),
        ] {
            let got = round_to_storage_directed_g::<St, Rung>(
                BASE_GUARD,
                TARGET_ASIN,
                mode,
                St::MAX,
                St::MIN,
                |guard_digits| {
                    // 10^g + ⌊1.667·10^(g−77)⌋ — the deviation appears at
                    // ULP-depth 77 (asin's x³/6 for x = 1e-38).
                    let one = crate::consts::pow10::dispatch::<Rung>(guard_digits);
                    if guard_digits >= 80 {
                        one + <Rung as BigInt>::from_i128(1667)
                            * crate::consts::pow10::dispatch::<Rung>(guard_digits - 80)
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

    /// Pin every mode for one wide-tier `D153<SCALE>` function at the
    /// tiny argument `±3·10^(−KNEG)` (raw `±3·10^(SCALE−KNEG)`, built
    /// directly — the parser takes no scientific notation). `expanding`
    /// selects the cubic-tail direction: EXPANDING (`|f| > |x|`,
    /// `tan`/`asin`) pushes the magnitude OUT by one ULP only under the
    /// away-from-`x` directed mode (Ceiling for `+x`, Floor for `−x`);
    /// COMPRESSING (`|f| < |x|`, `sin`/`atan`) pulls it IN by one ULP under
    /// the toward-zero directed modes. The three nearest modes round to `x`.
    macro_rules! pin {
        ($scale:literal, $kneg:literal, $f:ident, $expanding:expr, $label:literal) => {{
            let x_raw = Int::<8>::from_i128(3)
                * crate::consts::pow10::dispatch::<Int<8>>($scale - $kneg);
            let x = crate::D::<Int<8>, $scale>(x_raw);
            let neg_x_raw = -x_raw;
            let neg_x = crate::D::<Int<8>, $scale>(neg_x_raw);
            for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
                assert_eq!(x.$f(mode).0, x_raw, "{} (+x) {:?}", $label, mode);
                assert_eq!(neg_x.$f(mode).0, neg_x_raw, "{} (−x) {:?}", $label, mode);
            }
            if $expanding {
                assert_eq!(x.$f(Ceiling).0, x_raw + ULP, "{} (+x) Ceiling", $label);
                assert_eq!(x.$f(Floor).0, x_raw, "{} (+x) Floor", $label);
                assert_eq!(x.$f(Trunc).0, x_raw, "{} (+x) Trunc", $label);
                assert_eq!(neg_x.$f(Floor).0, neg_x_raw - ULP, "{} (−x) Floor", $label);
                assert_eq!(neg_x.$f(Ceiling).0, neg_x_raw, "{} (−x) Ceiling", $label);
                assert_eq!(neg_x.$f(Trunc).0, neg_x_raw, "{} (−x) Trunc", $label);
            } else {
                assert_eq!(x.$f(Floor).0, x_raw - ULP, "{} (+x) Floor", $label);
                assert_eq!(x.$f(Trunc).0, x_raw - ULP, "{} (+x) Trunc", $label);
                assert_eq!(x.$f(Ceiling).0, x_raw, "{} (+x) Ceiling", $label);
                assert_eq!(neg_x.$f(Ceiling).0, neg_x_raw + ULP, "{} (−x) Ceiling", $label);
                assert_eq!(neg_x.$f(Trunc).0, neg_x_raw + ULP, "{} (−x) Trunc", $label);
                assert_eq!(neg_x.$f(Floor).0, neg_x_raw, "{} (−x) Floor", $label);
            }
        }};
    }

    #[test]
    fn tan_expanding_d153_tang_and_series_bands() {
        // 3e-60 @ s76 → Tang band (70..=82); 3e-120 @ s152 → Series band —
        // both sit in the linear band so the analytic decision applies.
        pin!(76, 60, tan_with, true, "tan s76");
        pin!(152, 120, tan_with, true, "tan s152");
    }

    #[test]
    fn sin_compressing_d153_tang_and_series_bands() {
        pin!(76, 60, sin_with, false, "sin s76");
        pin!(152, 120, sin_with, false, "sin s152");
    }

    #[test]
    fn atan_compressing_d153_series_band() {
        pin!(152, 120, atan_with, false, "atan s152");
    }

    #[test]
    fn asin_expanding_d153_series_band() {
        pin!(152, 120, asin_with, true, "asin s152");
    }
}

/// Wide-tier DEEP-band tiny-x DIRECTED rounding pins — the generalisation past
/// the linear ([`tiny_x_directed_pins`]) band, where the LEADING odd Taylor
/// terms terminate exactly on the grid and a DEEPER odd term (`x⁷`, `j* = 7`)
/// decides. The comprehensive-gate find: `±3·10⁻¹²⁰` @ D616 s615 (the `x⁷`
/// digit sits at fractional depth ~841, past the `Int<96>` work integer's reach
/// AND the const tables, so the directed walker is mode-blind and only the
/// analytic [`tiny_x_deep_directed_adjust`] sign resolves it). This pins the
/// DIRECTED adjustment RELATIVE to the nearest grid value `G` (`G ≠ x` here, the
/// deep-band signature — asserted explicitly); `G`'s own accuracy is the golden
/// gate's job. EXPANDING (`tan`/`asin`, `j* ≡ 1 mod 4` for the alternating pair)
/// pushes OUT, COMPRESSING (`sin`/`atan`, `j* = 7 ≡ 3 mod 4`) pulls IN.
#[cfg(all(test, any(feature = "d616", feature = "x-wide")))]
mod tiny_x_deep_directed_pins {
    use crate::int::types::{traits::BigInt, Int};
    use crate::support::rounding::RoundingMode::{
        Ceiling, Floor, HalfAwayFromZero, HalfToEven, HalfTowardZero, Trunc,
    };

    const ULP: Int<32> = <Int<32> as BigInt>::ONE;

    /// Pin every mode for one `D616<SCALE>` function at `±3·10^(−KNEG)`
    /// (raw `±3·10^(SCALE−KNEG)`). `G` is the crate's own NEAREST result (the
    /// terminating partial sum, `≠ x`); the directed modes must be `G ± 1 ULP`
    /// per the deciding term's sign.
    macro_rules! pin_deep {
        ($scale:literal, $kneg:literal, $f:ident, $expanding:expr, $label:literal) => {{
            let x_raw = Int::<32>::from_i128(3)
                * crate::consts::pow10::dispatch::<Int<32>>($scale - $kneg);
            let x = crate::D::<Int<32>, $scale>(x_raw);
            let neg_x = crate::D::<Int<32>, $scale>(-x_raw);
            let g = x.$f(HalfToEven).0; // the on-grid nearest value
            let ng = neg_x.$f(HalfToEven).0;
            // Deep-band signature: the nearest value is the terminating partial
            // sum, strictly off the raw linear term `x`.
            assert_ne!(g, x_raw, "{}: expected the DEEP band (G != x)", $label);
            for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
                assert_eq!(x.$f(mode).0, g, "{} (+x) {:?}", $label, mode);
                assert_eq!(neg_x.$f(mode).0, ng, "{} (−x) {:?}", $label, mode);
            }
            if $expanding {
                assert_eq!(x.$f(Ceiling).0, g + ULP, "{} (+x) Ceiling", $label);
                assert_eq!(x.$f(Floor).0, g, "{} (+x) Floor", $label);
                assert_eq!(x.$f(Trunc).0, g, "{} (+x) Trunc", $label);
                assert_eq!(neg_x.$f(Floor).0, ng - ULP, "{} (−x) Floor", $label);
                assert_eq!(neg_x.$f(Ceiling).0, ng, "{} (−x) Ceiling", $label);
                assert_eq!(neg_x.$f(Trunc).0, ng, "{} (−x) Trunc", $label);
            } else {
                assert_eq!(x.$f(Floor).0, g - ULP, "{} (+x) Floor", $label);
                assert_eq!(x.$f(Trunc).0, g - ULP, "{} (+x) Trunc", $label);
                assert_eq!(x.$f(Ceiling).0, g, "{} (+x) Ceiling", $label);
                assert_eq!(neg_x.$f(Ceiling).0, ng + ULP, "{} (−x) Ceiling", $label);
                assert_eq!(neg_x.$f(Trunc).0, ng + ULP, "{} (−x) Trunc", $label);
                assert_eq!(neg_x.$f(Floor).0, ng, "{} (−x) Floor", $label);
            }
        }};
    }

    #[test]
    fn sin_atan_compressing_d616_s615_deep() {
        pin_deep!(615, 120, sin_with, false, "sin s615");
        pin_deep!(615, 120, atan_with, false, "atan s615");
    }

    #[test]
    fn tan_asin_expanding_d616_s615_deep() {
        pin_deep!(615, 120, tan_with, true, "tan s615");
        pin_deep!(615, 120, asin_with, true, "asin s615");
    }
}

/// The near-min walker's tagged-exact-half contract (issue #95): a probe
/// whose sub-storage residual is EXACTLY half and whose kernel proved the
/// tail side is decided by the tag — for every nearest mode — instead of
/// being bumped away from zero by the unresolved endgame's `never_exact`
/// blanket.
///
/// The synthetic recompute mimics `exp(-10^-k)` at `SCALE = 2k` (the
/// `exp(-1e-462)` at `D1232<924>` shape, scaled to unit widths): at every
/// guard `g` the working value is the exact partial sum
/// `10^(TARGET+g) - 10^(g+K) + 5·10^(g-1)`, whose residual below the
/// storage grid is exactly half an ULP, and the deciding term (digit `3K`,
/// which the true series subtracts) lies past both work widths' probe
/// reach — so before the tag rule the walker ran its whole ladder at BOTH
/// widths and then bumped up. The correct nearest answer is DOWN for a
/// `Below` tag and UP for an `Above` one, in all three nearest modes.
#[cfg(test)]
mod tagged_half_walker_contract {
    use super::*;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    type S1 = Int<4>;
    type S2 = Int<6>;
    type St = Int<2>;
    const BASE_GUARD: u32 = 9;
    /// Storage scale, `2·K` — the issue #95 shape (`x = -10^-K`).
    const TARGET: u32 = 20;
    const K: u32 = 10;

    const NEAREST: [RoundingMode; 3] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
    ];

    /// The exact partial sum of `exp(-10^-K)` at working scale
    /// `TARGET + g`: `1 - 10^-K + (1/2)·10^-2K`, ending exactly on the
    /// half-ULP boundary of the storage grid.
    fn half_tie<S: BigInt>(g: u32) -> S {
        crate::consts::pow10::dispatch::<S>(TARGET + g)
            - crate::consts::pow10::dispatch::<S>(g + K)
            + <S as BigInt>::from_i128(5) * crate::consts::pow10::dispatch::<S>(g - 1)
    }

    fn run(mode: RoundingMode, tail: Option<TailSign>) -> i128 {
        round_to_storage_widening_tail_signed_g::<St, S1, S2>(
            BASE_GUARD,
            TARGET,
            mode,
            true, // never_exact, as the exp callers pass
            St::MAX,
            St::MIN,
            |g| (half_tie::<S1>(g), tail),
            |g| (half_tie::<S2>(g), tail),
        )
        .as_i128()
    }

    /// The truncated storage value `1 - 10^-K` at scale `TARGET` — the
    /// correct nearest answer when the truth is strictly below half.
    const DOWN: i128 = 100_000_000_000_000_000_000 - 10_000_000_000;

    #[test]
    fn tagged_below_half_tie_rounds_down_in_every_nearest_mode() {
        for mode in NEAREST {
            assert_eq!(
                run(mode, Some(TailSign::Below)),
                DOWN,
                "a Below tag at an exact half must round down under {mode:?}"
            );
        }
    }

    #[test]
    fn tagged_above_half_tie_rounds_up_in_every_nearest_mode() {
        for mode in NEAREST {
            assert_eq!(
                run(mode, Some(TailSign::Above)),
                DOWN + 1,
                "an Above tag at an exact half must round up under {mode:?}"
            );
        }
    }

    /// The untagged path keeps the `never_exact` endgame exactly as it was:
    /// an unresolved half boundary bumps away from zero in every nearest
    /// mode. This is the Smith-chain / hyperbolic fallback the tag rule
    /// must not disturb.
    #[test]
    fn untagged_half_tie_keeps_the_never_exact_endgame_bump() {
        for mode in NEAREST {
            assert_eq!(
                run(mode, None),
                DOWN + 1,
                "the untagged never_exact blanket must still bump under {mode:?}"
            );
        }
    }

    /// The directed modes never consult the tag on this input — the
    /// residual (half an ULP from the grid) already decides them — so the
    /// tag must not move any of them.
    #[test]
    fn directed_modes_are_untouched_by_the_tag() {
        for tail in [None, Some(TailSign::Below), Some(TailSign::Above)] {
            assert_eq!(run(RoundingMode::Ceiling, tail), DOWN + 1);
            assert_eq!(run(RoundingMode::Floor, tail), DOWN);
            assert_eq!(run(RoundingMode::Trunc, tail), DOWN);
        }
    }

    /// A DIRECTED endgame decided by the TAG is a proof, so the widening
    /// retry at `S2` must not run — its ladder could only walk to this
    /// same endgame and re-derive the same side from the same producer.
    /// (The blanket `never_exact` endgame keeps its unresolved verdict
    /// and still retries, per `untagged_half_tie_keeps_the_never_exact_
    /// endgame_bump` — only a tag verdict is a proof.)
    ///
    /// The recompute lands exactly ON the storage grid at every guard
    /// (`1 - 10^-K` with the deciding deviation past both widths' reach —
    /// the issue #84 directed deep-tie shape) with a proven `Below` tail:
    /// the truth is strictly below the grid line, so Trunc/Floor step one
    /// ULP toward zero and Ceiling holds the line.
    #[test]
    fn tag_decided_directed_endgame_skips_the_wider_retry() {
        use core::cell::Cell;
        fn grid<S: BigInt>(g: u32) -> S
        where
            S::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
        {
            crate::consts::pow10::dispatch::<S>(TARGET + g)
                - crate::consts::pow10::dispatch::<S>(g + K)
        }
        for (mode, expect) in [
            (RoundingMode::Trunc, DOWN - 1),
            (RoundingMode::Floor, DOWN - 1),
            (RoundingMode::Ceiling, DOWN),
        ] {
            let s2_calls = Cell::new(0u32);
            let r = round_to_storage_widening_tail_signed_g::<St, S1, S2>(
                BASE_GUARD,
                TARGET,
                mode,
                true, // never_exact, as the exp callers pass
                St::MAX,
                St::MIN,
                |g| (grid::<S1>(g), Some(TailSign::Below)),
                |g| {
                    s2_calls.set(s2_calls.get() + 1);
                    (grid::<S2>(g), Some(TailSign::Below))
                },
            )
            .as_i128();
            assert_eq!(r, expect, "a Below tag on the grid line under {mode:?}");
            assert_eq!(
                s2_calls.get(),
                0,
                "a tag-proven directed endgame must not retry at the wider width under {mode:?}"
            );
        }
    }
}

/// The expm1/log1p sibling walker's tagged-exact-half TERMINATION contract:
/// the nearest branch of [`round_to_storage_directed_tagged_impl_g`] returns
/// a tag-decided probe immediately and `resolved`, instead of walking the
/// remaining ladder only to hand back — or let a noise-cleared deeper probe
/// overrule — the narrowing the proof already fixed. The VALUE rule
/// (`nearest_narrow`'s exact-half tag arm) predates this; these tests pin
/// the termination onto this walker, the same rule the `exp` walker ships
/// in [`near_min_resolve_g`] (issue #95).
#[cfg(test)]
mod tagged_half_sibling_walker_contract {
    use super::*;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;
    use core::cell::Cell;

    type S = Int<6>;
    type St = Int<2>;
    const BASE_GUARD: u32 = 9;
    /// Storage scale, `2·K` — the deep-tie shape (`x = -10^-K`).
    const TARGET: u32 = 20;
    const K: u32 = 10;

    const NEAREST: [RoundingMode; 3] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
    ];

    /// A working value at guard `g` whose sub-storage residual is
    /// `tenths`/10 of a storage ULP: `10^(TARGET+g) - 10^(g+K)` sits on the
    /// grid, and the last summand puts the residual at the chosen depth.
    fn probe_value(g: u32, tenths: i128) -> S {
        crate::consts::pow10::dispatch::<S>(TARGET + g)
            - crate::consts::pow10::dispatch::<S>(g + K)
            + <S as BigInt>::from_i128(tenths) * crate::consts::pow10::dispatch::<S>(g - 1)
    }

    /// The truncated storage value `1 - 10^-K` at scale `TARGET` — the
    /// correct nearest answer under a `Below` tag at an exact half.
    const DOWN: i128 = 100_000_000_000_000_000_000 - 10_000_000_000;

    /// A tag-decided exact half ends the walk at its own probe: one kernel
    /// evaluation, not the ladder.
    #[test]
    fn tagged_half_probe_terminates_the_walk_at_that_probe() {
        for mode in NEAREST {
            let calls = Cell::new(0u32);
            let r = round_to_storage_tail_signed_g::<St, S>(
                BASE_GUARD,
                TARGET,
                mode,
                St::MAX,
                St::MIN,
                |g| {
                    calls.set(calls.get() + 1);
                    (probe_value(g, 5), Some(TailSign::Below))
                },
            )
            .as_i128();
            assert_eq!(r, DOWN, "a Below tag at an exact half must round down under {mode:?}");
            assert_eq!(
                calls.get(),
                1,
                "a tag-decided exact half must end the walk at its own probe under {mode:?}"
            );
        }
    }

    /// The base probe is the exact partial sum with a proven `Below` tail; a
    /// deeper probe hands back a residual clearing the noise floor on the
    /// WRONG side (the shape kernel noise takes once a deeper evaluation
    /// loses the exactness the tag certified). The tag is a proof, so the
    /// walk must conclude from it and never consult the deeper reading.
    #[test]
    fn base_tag_outranks_a_deeper_floor_clearing_probe() {
        for mode in NEAREST {
            let r = round_to_storage_tail_signed_g::<St, S>(
                BASE_GUARD,
                TARGET,
                mode,
                St::MAX,
                St::MIN,
                |g| {
                    if g == BASE_GUARD {
                        (probe_value(g, 5), Some(TailSign::Below))
                    } else {
                        (probe_value(g, 6), None)
                    }
                },
            )
            .as_i128();
            assert_eq!(
                r, DOWN,
                "the base probe's tag proof must outrank a deeper noisy probe under {mode:?}"
            );
        }
    }

    /// An UNTAGGED exact half keeps the previous path bit-identically: the
    /// mode's own tie-break decides the base narrowing and the ladder still
    /// runs (its cap returns that clean base). Guards the short-circuit
    /// against firing without a tag.
    #[test]
    fn untagged_half_tie_keeps_the_tie_break() {
        for (mode, expect) in [
            (RoundingMode::HalfToEven, DOWN), // last kept digit 0 — even, stays
            (RoundingMode::HalfAwayFromZero, DOWN + 1),
            (RoundingMode::HalfTowardZero, DOWN),
        ] {
            let r = round_to_storage_tail_signed_g::<St, S>(
                BASE_GUARD,
                TARGET,
                mode,
                St::MAX,
                St::MIN,
                |g| (probe_value(g, 5), None),
            )
            .as_i128();
            assert_eq!(r, expect, "the untagged tie-break must stand under {mode:?}");
        }
    }
}

/// `adjust_log_near_zero` under `ZeroFiveUp` (GDA `round-05up`).
///
/// The pass repairs a directed result the Ziv walker could not resolve.
/// `round-05up` truncates unless the last decimal digit of the TRUNCATED
/// value is `0` or `5`, so the cases that separate a real implementation
/// from a no-op `_ => result` fall-through are exactly the ones where the
/// pivot's verdict disagrees with "leave `result` alone". Each test below
/// names which of its rows those are.
#[cfg(test)]
mod log_near_zero_zero_five_up {
    use super::adjust_log_near_zero;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    /// `one` is read only by the parabola arms, which `ZeroFiveUp` never
    /// reaches, so its value is immaterial to every case below.
    const ONE: i128 = 1_000_000_000_000_000_000;

    fn adj(result: i128, delta: i128, mode: RoundingMode) -> i128 {
        adjust_log_near_zero::<Int<2>, Int<24>>(
            Int::<2>::from_i128(result),
            Int::<2>::from_i128(delta),
            Int::<2>::from_i128(ONE),
            mode,
        )
        .as_i128()
    }

    /// Tangent bracket (`result == delta`), positive: the truncated value
    /// is `delta - 1`. When its last digit is not a pivot the answer must
    /// truncate to `delta - 1`; the fall-through returned `delta`. Every
    /// row here fails without the fix.
    #[test]
    fn tangent_positive_without_pivot_truncates() {
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(adj(100, 100, m), 99, "trunc 99 ends in 9: no lift");
        assert_eq!(adj(108, 108, m), 107, "trunc 107 ends in 7: no lift");
        assert_eq!(adj(1_003, 1_003, m), 1_002, "trunc 1002 ends in 2: no lift");
    }

    /// Same bracket with a pivot digit: the truncated value ends in `0` or
    /// `5`, steps one away from zero and lands back on `delta`. These rows
    /// agree with the old fall-through — they guard against over-lifting.
    #[test]
    fn tangent_positive_with_pivot_lifts() {
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(adj(101, 101, m), 101, "trunc 100 ends in 0: lift");
        assert_eq!(adj(106, 106, m), 106, "trunc 105 ends in 5: lift");
        // `delta == 1` drives the truncated value to zero, whose last
        // digit is `0` — a pivot, so it lifts back to 1.
        assert_eq!(adj(1, 1, m), 1, "trunc 0 ends in 0: lift");
    }

    /// Tangent bracket, negative: `V < delta < 0`, so the truncated value
    /// (the one nearest zero) is `delta` itself and away-from-zero is
    /// `delta - 1`. A pivot must step DOWN, which the fall-through never
    /// did — the first two rows fail without the fix.
    #[test]
    fn tangent_negative_with_pivot_steps_away_from_zero() {
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(adj(-100, -100, m), -101, "|trunc| 100 ends in 0: lift");
        assert_eq!(adj(-105, -105, m), -106, "|trunc| 105 ends in 5: lift");
        assert_eq!(adj(-103, -103, m), -103, "|trunc| 103 ends in 3: no lift");
    }

    /// Parabola bracket (`result != delta`): the truncated value is
    /// `result` itself, so a pivot digit lifts one away from zero. The
    /// fall-through returned `result` for every digit, so the pivot rows
    /// fail without the fix.
    #[test]
    fn parabola_bracket_pivots_on_the_result_digit() {
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(adj(150, 200, m), 151, "150 ends in 0: lift up");
        assert_eq!(adj(145, 200, m), 146, "145 ends in 5: lift up");
        assert_eq!(adj(147, 200, m), 147, "147 ends in 7: no lift");
        assert_eq!(adj(-150, -200, m), -151, "|-150| ends in 0: lift down");
        assert_eq!(adj(-147, -200, m), -147, "|-147| ends in 7: no lift");
    }

    /// The exact point stays exact: `delta == 0` is `ln(1)`, which no mode
    /// may move.
    #[test]
    fn zero_delta_is_untouched() {
        assert_eq!(adj(0, 0, RoundingMode::ZeroFiveUp), 0);
        assert_eq!(adj(7, 0, RoundingMode::ZeroFiveUp), 7);
    }

    /// The `ZeroFiveUp` branch must not disturb the six original modes.
    #[test]
    fn the_original_modes_are_unchanged() {
        assert_eq!(adj(100, 100, RoundingMode::Trunc), 99);
        assert_eq!(adj(100, 100, RoundingMode::Floor), 99);
        assert_eq!(adj(100, 100, RoundingMode::Ceiling), 100);
        assert_eq!(adj(-100, -100, RoundingMode::Trunc), -100);
        assert_eq!(adj(-100, -100, RoundingMode::Floor), -101);
        assert_eq!(adj(-100, -100, RoundingMode::Ceiling), -100);
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
        ] {
            assert_eq!(adj(100, 100, m), 100, "{m:?} is a no-op here");
        }
    }

    /// `AwayFromZero` takes the full step on both sides — the answer
    /// `ZeroFiveUp` defers to whenever its pivot fires.
    #[test]
    fn away_from_zero_takes_the_full_step() {
        let m = RoundingMode::AwayFromZero;
        assert_eq!(adj(100, 100, m), 100, "positive: away is Ceiling, no move");
        assert_eq!(adj(-100, -100, m), -101, "negative: away is Floor");
    }
}
