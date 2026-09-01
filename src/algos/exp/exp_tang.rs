// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic Tang-style table-driven `exp_strict` kernel.
//!
//! Tang 1989, "Table-driven implementation of the exponential function
//! in IEEE floating-point arithmetic" (ACM TOMS 16(4)):
//!
//! ```text
//! e^v = 2^k · e^s,            s = v − k·ln 2,           |s| ≤ ln 2 / 2
//!     = 2^k · e^(c_j) · e^δ,  c_j = j · ln 2 / M,       j ∈ [0, M)
//!                              δ  = s − c_j,            |δ| ≤ ln 2 / (2M)
//! ```
//!
//! A two-stage range reduction collapses the post-stage-1 Taylor into a
//! table multiply (`exp(c_j)` read from the baked `M`-entry
//! `exp_tang_table` consts, the indexed slot converted to the working
//! scale per lookup) plus a short Taylor on the tiny remainder `δ`. The
//! result is reassembled as `2^(k+k_adj) · table[j] · e^δ`.
//!
//! ## Layering
//!
//! This is an **algorithm function** (`docs/ARCHITECTURE.md` →
//! "Layering direction"): it computes only through the
//! [`WideTrigCore`] trait surface and `BigInt` arithmetic on the work
//! integer; it never calls a method on a decimal type. `policy::exp`
//! calls [`exp_tang`] *down*; the type's `exp_strict` method delegates
//! *down* through the policy. The trig hyperbolic kernels reuse
//! [`tang_exp_fixed`] directly for their shared `(e^v, e^-v)` pair.
//!
//! Collapses the four per-tier D57 (18..=22 / 45..=56), D115
//! and D153 Tang exp kernels
//! kernels into one generic over `C: WideTrigCore`, the table size `M`,
//! and the per-band reduction/narrowing flags.

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::exp_tang_table::exp_table_entry_baked;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Tang-style `e^v` on an already-lifted `working_value` (`= x ·
/// 10^w`), returned at the same `working_scale`. Generic over the tier `C`
/// and the table size `M`.
///
/// `INTERNAL_EXTRA` selects the large-`k` mitigation. For `k > 0` the final
/// `2^k` reassembly (a LEFT shift) amplifies the reduction residual by
/// `2^k ≈ 10^(k·log10 2)` decimal digits, so a fixed narrow guard cannot
/// cover an unbounded `k`. When `true` the whole reduction runs at an
/// `extended_working_scale = working_scale + extra_digits`
/// (`extra_digits = ceil(k·log10 2) + 12`, sized for `k > 0` only) and the
/// result is narrowed back to `working_scale` round-to-nearest; when `false`
/// the body runs at the caller-supplied `working_scale` (the caller absorbs
/// the `extra_digits` lift in its own guard, or the band's `k` is small
/// enough not to need it). For `k ≤ 0` the reassembly is an error-shrinking
/// RIGHT shift, so no lift is taken (`extra_digits = 0`) — see the body
/// comment. This is the shared surface the trig hyperbolic kernels reuse.
#[must_use]
pub(crate) fn tang_exp_fixed<
    C: WideTrigCore,
    const M: u32,
    const INTERNAL_EXTRA: bool,
    const SCALE: u32,
>(
    working_value: C::W,
    working_scale: u32,
) -> C::W
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    // Thin `WideTrigCore`-bound wrapper over the width-generic
    // [`tang_exp_fixed_g`]: binds the work integer to `C::W` and supplies
    // `ln 2` from `C::ln2::<SCALE>` (the crate's feature-flagged default
    // rounding mode + the per-scale const-fold). One Tang `exp` kernel — the
    // wide compositions call `tang_exp_fixed_g` directly at their `Wagm` work
    // width.
    tang_exp_fixed_g::<C::W, M, INTERNAL_EXTRA>(
        working_value, working_scale, |at_scale| C::ln2::<SCALE>(at_scale))
}

/// Width-generic core of [`tang_exp_fixed`] — the Tang `exp` body over any
/// [`BigInt`] work integer `S`, reusing the unified `exp_generic` fixed-point
/// arithmetic leaves (the sibling of [`crate::algos::ln::ln_tang::tang_ln_fixed_g`]).
///
/// `ln 2` is supplied by an accessor `ln2(working_scale)` so the caller owns
/// the rounding mode (the crate's feature-flagged default — never a hardcoded
/// one); the Tang `exp` table is the already-width-generic
/// [`exp_table_entry_baked`] (binary, scale-independent). `tang_exp_fixed::<C>`
/// is the thin tier-bound wrapper; the wide compositions (`powf`/`exp2`/the
/// hyperbolics) call this directly at their `Wagm` work width.
#[must_use]
pub(crate) fn tang_exp_fixed_g<S: BigInt, const M: u32, const INTERNAL_EXTRA: bool>(
    working_value: S,
    working_scale: u32,
    ln2: impl Fn(u32) -> S,
) -> S
where
    S::Scratch: ComputeLimbs,
{
    // Stage 0 (INTERNAL_EXTRA only): size an `extended_working_scale
    // = working_scale + extra_digits` from `|k|` so the `2^k` reassembly does
    // not amplify the reduction residual past the storage LSB. Matches the
    // dynamic-margin reduction the generic `exp_fixed` uses (Muller,
    // *Elementary Functions* 3rd ed., §11.1). `k` is scale-invariant, so
    // reuse the value computed at `working_scale` below.
    let k = {
        let one_at_working_scale = eg::one::<S>(working_scale);
        eg::round_to_nearest_int::<S>(
            eg::div_cached::<S>(working_value, ln2(working_scale), one_at_working_scale),
            working_scale)
    };

    let (extended_working_scale, extended_working_value, extra_digits) = if INTERNAL_EXTRA {
        // Size the extended scale from `k` only for `k > 0`: the `2^k`
        // reassembly amplifies the residual only on the LEFT shift. For `k < 0`
        // (underflow) the reassembly is an error-shrinking RIGHT shift, so the
        // base scale already suffices — and inflating the extended scale there
        // would drive the table-entry product `slot_hi · 10^ext` (≈
        // `2·ext·log2(10)` bits) past the work integer `S`, silently wrapping
        // `exp(c_j)`. (Same asymmetry the `EXTERNAL_EXTRA` wrapper applies.)
        let extra_digits: u32 = if k <= 0 {
            0
        } else {
            let digits = (k as u128 * 30103).div_ceil(100_000) as u32;
            digits + 12
        };
        let extended_working_value = if extra_digits == 0 {
            working_value
        } else {
            working_value * eg::pow10::<S>(extra_digits)
        };
        (working_scale + extra_digits, extended_working_value, extra_digits)
    } else {
        (working_scale, working_value, 0)
    };

    // Overflow gate (up front, before any extended-scale work). The body runs
    // at `extended_working_scale` — `one_at_extended_scale = 10^ext` and the
    // `2^k` reassembly `exp_reduced_arg << k` — so a result too large to
    // represent needs `ext` digits (`≈ ext·log2(10)` bits) PLUS the `k`-bit
    // shift to exceed `S`. Without this gate the `10^ext` literal alone
    // silently WRAPS once it passes `S`'s
    // width (e.g. exp2(1005) = e^696.7: `ext ≈ 372` ⇒ ~1236 bits > Wagm's
    // 1024 ⇒ garbage), and the result came back as 0 instead of panicking. A
    // fixed-width decimal has no infinity: PANIC, uniform across debug AND
    // release (the strict-transcendental overflow contract). In-range results
    // fit `S` (wider than storage) with room, so this never fires for a
    // representable cell. digits→bits: `log2(10) ≈ 3322/1000`.
    {
        let peak_bits =
            (extended_working_scale as u64) * 3322 / 1000 + if k >= 0 { k as u64 } else { 0 };
        if peak_bits >= <S as BigInt>::BITS as u64 {
            panic!("tang_exp_fixed: result out of range");
        }
    }

    let one_at_extended_scale = eg::one::<S>(extended_working_scale);
    let pow10_at_extended_scale = one_at_extended_scale;
    let ln2_at_extended_scale = ln2(extended_working_scale);

    // Stage 1: v = k·ln 2 + s, |s| ≤ ln 2 / 2.
    let k_ln2 = if k >= 0 {
        ln2_at_extended_scale * eg::lit::<S>(k)
    } else {
        -(ln2_at_extended_scale * eg::lit::<S>(-k))
    };
    let reduced_arg = extended_working_value - k_ln2;

    // Stage 2: s = j_signed · (ln 2 / M) + δ, |δ| ≤ ln 2 / (2M).
    let table_index_signed = eg::round_to_nearest_int::<S>(
        eg::div_cached::<S>(
            reduced_arg * eg::lit::<S>(M as i128),
            ln2_at_extended_scale,
            pow10_at_extended_scale),
        extended_working_scale);
    let table_point = if table_index_signed >= 0 {
        (ln2_at_extended_scale * eg::lit::<S>(table_index_signed)) / eg::lit::<S>(M as i128)
    } else {
        -((ln2_at_extended_scale * eg::lit::<S>(-table_index_signed))
            / eg::lit::<S>(M as i128))
    };
    let delta = reduced_arg - table_point;
    let (table_index, k_adj) = if table_index_signed >= 0 {
        (table_index_signed as u32, 0i128)
    } else {
        ((table_index_signed + M as i128) as u32, -1i128)
    };
    debug_assert!(table_index < M, "tang_exp_fixed: table index out of range");

    // Taylor on δ.
    let mut sum = one_at_extended_scale + delta;
    let mut term = delta;
    let mut term_index: u128 = 2;
    loop {
        term = eg::mul::<S>(term, delta, extended_working_scale)
            / eg::lit::<S>(term_index as i128);
        if term == eg::zero::<S>() {
            break;
        }
        sum = sum + term;
        term_index += 1;
        if term_index > 200 {
            break;
        }
    }

    let exp_table_value = exp_table_entry_baked::<S>(
        extended_working_scale, table_index as usize, M, pow10_at_extended_scale);
    let exp_reduced_arg = eg::mul::<S>(exp_table_value, sum, extended_working_scale);

    let k_total = k + k_adj;
    let exp_at_extended_scale = if k_total >= 0 {
        let shift = k_total as u32;
        // The `2^k` reassembly `exp_reduced_arg << shift` wraps past `S`'s width once the
        // result is too large to represent — a genuinely out-of-range exp. A
        // fixed-width decimal has no infinity, so PANIC, uniform across debug
        // AND release (the strict-transcendental overflow contract). This was a
        // `debug_assert!`, so a RELEASE build silently WRAPPED to garbage — e.g.
        // `exp2(1005)` (= e^696.7, far beyond every tier) returned 0 instead of
        // panicking, while `exp2(200)` (overflow that still fits `S`, panicking
        // later at the storage narrow) was correct: a tier/scale-INVARIANT
        // violation the full-surface golden surfaced.
        if eg::bit_length::<S>(exp_reduced_arg) + shift >= <S as BigInt>::BITS {
            panic!("tang_exp_fixed: result out of range");
        }
        exp_reduced_arg << shift
    } else {
        let right_shift_bits = (-k_total) as u32;
        if right_shift_bits as u128 >= eg::bit_length::<S>(exp_reduced_arg) as u128 {
            // Deep underflow: `e^v` (`v < 0` here, since `k_total < 0`) is
            // strictly positive but below the working resolution. Return the
            // smallest positive working value (`1 = 10^-ext`), NOT a bare
            // zero, so the caller's directed narrowing keeps the sign —
            // Ceiling rounds UP to one storage ULP while Floor / Trunc /
            // nearest still give 0. A bare zero loses positivity and rounds
            // Ceiling to 0 (the `powf("2","-200")` mid-scale defect). This
            // matches `exp_generic::try_exp_fixed`'s deep-underflow return.
            eg::lit::<S>(1)
        } else {
            exp_reduced_arg >> right_shift_bits
        }
    };

    // `e^v > 0` for every finite `v`: a zero result on the `k_total < 0`
    // (underflow) branch is genuine sub-resolution underflow, NOT a true
    // zero — return the smallest positive working value so the directed
    // narrowing keeps the sign (Ceiling → 1 ULP). Mirrors the `exp_generic`
    // catch-all; restricted to `k_total < 0`, the only regime where
    // underflow to 0 is physical.
    let exp_at_extended_scale = if k_total < 0 && exp_at_extended_scale == eg::zero::<S>() {
        eg::lit::<S>(1)
    } else {
        exp_at_extended_scale
    };

    if !INTERNAL_EXTRA || extra_digits == 0 {
        exp_at_extended_scale
    } else {
        // Narrow the extended-scale result back to `working_scale`
        // round-to-nearest (ties up via the `+ half` bias). `extra_digits` is
        // bounded so `10^extra_digits` stays well inside the working width.
        let extra_pow10 = eg::pow10::<S>(extra_digits);
        let half = extra_pow10 / eg::lit::<S>(2);
        if exp_at_extended_scale >= eg::zero::<S>() {
            (exp_at_extended_scale + half) / extra_pow10
        } else {
            -((-exp_at_extended_scale + half) / extra_pow10)
        }
    }
}

/// Tier-generic Tang-style `e^x` strict kernel.
///
/// - `C` — the per-tier [`WideTrigCore`] marker (`wide_trig_d*::Core`).
/// - `SCALE` — the decimal storage scale.
/// - `M` — the Tang table size (`128` or `512`).
/// - `GUARD` — the narrow guard for this band (`8`, `10`, or the tier's
///   canonical `30`).
/// - `DIRECTED` — route the final narrowing through the directed-rounding
///   Ziv escalation (`true`), else narrow once with
///   `round_to_storage_with` (`false`).
/// - `EXTERNAL_EXTRA` — compute the large-`|k|` working-scale lift
///   `extra_digits` in this wrapper and fold it into the directed base guard
///   (the D115 shape; requires `DIRECTED`).
/// - `INTERNAL_EXTRA` — let [`tang_exp_fixed`] do the `extra_digits` lift +
///   narrow-back internally (the D153 shape).
#[inline]
#[must_use]
pub(crate) fn exp_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const M: u32,
    const GUARD: u32,
    const DIRECTED: bool,
    const EXTERNAL_EXTRA: bool,
    const INTERNAL_EXTRA: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }

    if !DIRECTED && crate::support::rounding::is_nearest_mode(mode) {
        // Single-shot narrowing (D57 18..=22 and 45..=56) — NEAREST modes
        // only. Reduction runs at the const-folded `w = SCALE + GUARD`; the
        // band guard keeps the working error well under half a storage ULP,
        // so a single narrow is correctly rounded to nearest. Directed modes
        // (which must decide which SIDE of a grid line the true value lies,
        // and can sit a sub-resolution residual below the work-int's
        // resolution — `exp(-10^-S)` just under `1.0` at MAX scale) fall
        // through to the never-exact Ziv path below.
        let working_scale = SCALE + GUARD;
        let working_value = C::to_work_scaled(raw, GUARD);
        let exp_working_value =
            tang_exp_fixed::<C, M, INTERNAL_EXTRA, SCALE>(working_value, working_scale);
        return C::round_to_storage_with(exp_working_value, working_scale, SCALE, mode);
    }

    let base_guard_digits = if EXTERNAL_EXTRA {
        // The final reassembly is `exp_s << k` for `k ≥ 0` and `exp_s >> |k|`
        // for `k < 0`. Only the LEFT shift (`k ≥ 0`) amplifies the
        // working-scale rounding error by `2^k` (≈ `|k|·log10 2` digits), so
        // only there must the base guard widen by `extra_digits` to keep the
        // post-shift residual inside the guard. For `k < 0` (the underflow
        // direction — `e^(large negative)`) the reassembly is a RIGHT shift
        // that shrinks the value and its absolute error, so the base `GUARD`
        // already covers it with vast margin; inflating the guard there is not
        // only needless but HARMFUL — it drives the working scale
        // `SCALE + base_guard_digits` high enough that the Tang table-entry
        // product (`exp_table_entry_baked`'s `slot_hi · 10^w`, ≈ `2·w·log2(10)`
        // bits) overflows the work integer `S`, silently wrapping the `exp(c_j)`
        // factor (the deep-underflow misround at the wide tiers' max scale).
        // So size `extra_digits` from `k` only when `k > 0`.
        let working_scale = SCALE + GUARD;
        let one_at_working_scale = C::one(working_scale);
        let working_value_probe = C::to_work_scaled(raw, GUARD);
        let k = C::round_to_nearest_int(
            C::div_cached(
                working_value_probe,
                C::ln2::<SCALE>(working_scale),
                one_at_working_scale),
            working_scale);
        let extra_digits: u32 = if k <= 0 {
            0
        } else {
            let abs_k = k as u128;
            let digits = (abs_k * 30103).div_ceil(100_000);
            let capped = digits.min((C::w_bits() / 4) as u128) as u32;
            capped + 12 + (capped >> 2)
        };
        GUARD + extra_digits
    } else {
        GUARD
    };

    // Directed modes decide which side of a storage grid line the true
    // result falls; near a grid line the working-scale approximation can
    // land on the wrong side, so route through the shared Ziv escalation.
    // Nearest modes narrow once. `exp(x)` for `x != 0` is transcendental
    // (never exactly on a grid line — `raw == 0` is pinned above), so use the
    // never-exact narrowing: a zero working residual is a sub-resolution
    // artifact, and Ceiling must still round up (Floor / Trunc keep the floor)
    // on inputs whose deciding residual is below the work-int resolution
    // (`exp(-10^-S)` just under `1.0`).
    C::round_to_storage_directed_never_exact(base_guard_digits, SCALE, mode, &mut |guard_digits| {
        tang_exp_fixed::<C, M, INTERNAL_EXTRA, SCALE>(
            C::to_work_scaled(raw, guard_digits), SCALE + guard_digits)
    })
}

#[cfg(all(test, feature = "wide"))]
mod tests {
    //! Deep-underflow correctness for the Tang `exp` path.
    //!
    //! At D76<75> the Tang
    //! `wide_tang_gate` admits large negative arguments (`e^(−34..−58)`, all
    //! representable). An `EXTERNAL_EXTRA` guard must NOT inflate the working
    //! scale `w` by `≈ |k|·log10 2` digits for these — the `k < 0`
    //! reassembly is an error-shrinking RIGHT shift that needs no such guard.
    //! Such inflation pushes the table-entry product `slot_hi · 10^w` past the tier work
    //! integer `Int<16>` (1024 bits), silently wrapping the `exp(c_j)` factor
    //! (~25 % error). The wider D307<75> tier runs the Series path and is the
    //! oracle: `exp` rounded to scale 75 is the same value at every storage
    //! width, so the two decimal renderings must be identical.

    use crate::types::widths::{D307, D76};
    use crate::RoundingMode;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    #[test]
    fn tang_deep_underflow_matches_wide_series_oracle_d76_s75() {
        // Negative args spanning the underflow regime the Tang gate routes —
        // from below the overflow boundary (~−33) up to the storage edge
        // (max |x| ≈ 57.9 at D76<75>).
        let args = ["-20.0", "-33.5", "-34.37", "-40.0", "-45.123", "-50.25", "-55.0", "-57.5"];
        for arg_text in args {
            let d76_value: D76<75> = arg_text.parse().unwrap();
            let d307_value: D307<75> = arg_text.parse().unwrap();
            for mode in MODES {
                let got = d76_value.exp_strict_with(mode).to_string();
                let want = d307_value.exp_strict_with(mode).to_string();
                assert_eq!(
                    got, want,
                    "exp({arg_text}) D76<75> vs D307<75> oracle, mode {mode:?}"
                );
            }
        }
    }
}

#[cfg(all(test, any(feature = "d57", feature = "d76", feature = "wide")))]
mod powf_deep_underflow_regression {
    //! Guard (powf.golden:8048): `powf("2","-200") = 2^-200
    //! ≈ 6.223e-61` at a mid storage scale is a sub-resolution positive — it
    //! MUST round to 0 under the nearest / Floor / Trunc modes and to one
    //! storage ULP under Ceiling at every scale `< 61`. The `exp(y·ln x)`
    //! composition's `k_lift` sizer must account for the exp argument's SIGN:
    //! sizing a deeply
    //! negative argument (`-200·ln 2 ≈ -138.6`, whose result `e^-138.6 < 1`
    //! needs zero lift) a ~90-digit lift would inflate the working
    //! scale until the non-widening `mul_agm(y, ln_x, w)` low product
    //! overflows the `Wagm` work integer and WRAPS the exp argument to
    //! ≈ -0.21, returning `e^-0.21 ≈ 0.808` — a magnitude-class error at
    //! D57<28/30/42> and D76 mid-scales. Two coordinated guards: (a) the powf
    //! shell takes no lift for a negative argument (mirrors
    //! `exp2_result_int_digits`); (b) the Tang deep-underflow branch returns
    //! the smallest positive working value (mirrors `exp_generic`), not bare
    //! zero, so Ceiling rounds the sub-resolution positive up to 1 ULP.

    use crate::RoundingMode;

    /// The five modes that round a sub-resolution positive DOWN to 0.
    const DOWN_MODES: [RoundingMode; 5] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
    ];

    /// `"0.00…01"` — one storage ULP at `scale` (the smallest positive).
    fn one_ulp_str(scale: usize) -> String {
        let mut text = String::from("0.");
        for _ in 0..scale - 1 {
            text.push('0');
        }
        text.push('1');
        text
    }

    #[cfg(any(feature = "d57", feature = "wide"))]
    #[test]
    fn powf_2_neg200_d57_mid_scales_six_modes() {
        use crate::types::widths::D57;
        // s28 is the exact golden cell; s42 a second mid-scale, both < 61.
        macro_rules! check_d57 {
            ($s:literal) => {{
                let base: D57<$s> = "2".parse().unwrap();
                let exp: D57<$s> = "-200".parse().unwrap();
                let zero: D57<$s> = "0".parse().unwrap();
                let one_ulp: D57<$s> = one_ulp_str($s).parse().unwrap();
                for mode in DOWN_MODES {
                    assert_eq!(
                        base.powf_strict_with(exp, mode), zero,
                        "D57<{}> powf(2,-200) {mode:?} must round the sub-resolution positive to 0", $s
                    );
                }
                assert_eq!(
                    base.powf_strict_with(exp, RoundingMode::Ceiling), one_ulp,
                    "D57<{}> powf(2,-200) Ceiling must round the sub-resolution positive up to 1 ULP", $s
                );
            }};
        }
        check_d57!(28);
        check_d57!(42);
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn powf_2_neg200_d76_mid_scales_six_modes() {
        use crate::types::widths::D76;
        macro_rules! check_d76 {
            ($s:literal) => {{
                let base: D76<$s> = "2".parse().unwrap();
                let exp: D76<$s> = "-200".parse().unwrap();
                let zero: D76<$s> = "0".parse().unwrap();
                let one_ulp: D76<$s> = one_ulp_str($s).parse().unwrap();
                for mode in DOWN_MODES {
                    assert_eq!(
                        base.powf_strict_with(exp, mode), zero,
                        "D76<{}> powf(2,-200) {mode:?} must round the sub-resolution positive to 0", $s
                    );
                }
                assert_eq!(
                    base.powf_strict_with(exp, RoundingMode::Ceiling), one_ulp,
                    "D76<{}> powf(2,-200) Ceiling must round the sub-resolution positive up to 1 ULP", $s
                );
            }};
        }
        check_d76!(40);
        check_d76!(50);
    }
}
