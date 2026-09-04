//! Inverse-hyperbolic compositions over `ln` / `log1p`, evaluated on the
//! wide `Wagm` composition work integer at the tier's fixed working
//! scale (`SCALE + GUARD`).
//!
//! The `ln`-side sibling of [`super::hyper_exp_identity`]: where the
//! forward hyperbolics compose over `exp`, `acosh` and `atanh` compose
//! over `ln`. Both kernels here are single-shot at the fixed working
//! scale — they do NOT escalate a work rung — which is what makes them
//! the fast arm of `policy::trig`'s inverse-hyperbolic matcher, with
//! `hyper_schoolbook::{acosh,atanh}_schoolbook` the Ziv-escalating
//! alternative.
//!
//! Generic over the tier core `C`, so one kernel serves every wide
//! width; there is no scale precondition, hence no band suffix on the
//! kernel names.

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::wide_trig_core::{round_to_storage_with_g, WideTrigCore};
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Series `ln(v)` at the composition work width — the generic form of
/// the per-tier `ln_fixed_series_agm` shell.
///
/// Deliberately the Series engine, NOT [`WideTrigCore::ln_fixed_routed_agm`]:
/// `asinh` pins Series because at MAX scale (input `±1`) the `sqrt` step
/// ahead of the `ln` loses sub-working-scale precision that the Tang
/// path's internal residue signal cannot detect caller-side. Keeping the
/// pin is what makes [`asinh_series_composition`] a relocation rather
/// than an engine swap.
///
/// `ln 2` is read with the runtime-keyed `ln2_by_working_scale`, matching
/// the shell this was lifted from exactly; the const-folded
/// `ln2_by_scale` sibling would be the faster read on the common
/// `working_scale == SCALE + GUARD` path, but switching to it is a value
/// question to settle separately, not part of the lift.
#[inline]
fn ln_series<C: WideTrigCore>(working_value: C::Wagm, working_scale: u32) -> C::Wagm
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    eg::ln_fixed::<C::Wagm>(
        working_value,
        working_scale,
        crate::consts::ln2_by_working_scale::<C::Wagm>(
            working_scale,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        ),
    )
}

/// Inverse hyperbolic sine, as `sign · ln(|x| + √(x² + 1))`, defined on
/// all reals. For `|x| ≥ 1` the radicand is factored (via the reciprocal)
/// to keep `x²` inside the working width.
///
/// **Not on the `policy::trig::inverse_hyper` matcher, unlike its two
/// siblings below.** This is the composition the inherent `asinh_strict`
/// shell has always run, lifted here verbatim so the computation lives in
/// a named kernel instead of inside the type-shell macro. It is NOT
/// interchangeable with the policy's `asinh` path
/// (`policy::trig::extra_rung::asinh_strict`, a rung-selected schoolbook):
/// that path's `ln` is the routed Tang/Series choice, while this one is
/// pinned to Series by [`ln_series`]. Registering this as an `Algorithm`
/// variant, so the matcher can choose between the two per cell, is the
/// open follow-up — it changes which engine runs for `asinh_strict` and
/// so is a routing decision, not a relocation.
///
/// `guard` is the number of guard digits below `SCALE` to compute at.
/// It is a plain runtime argument, not a const: the `_strict` entry
/// passes the tier's `C::GUARD` while the `_approx` entry passes the
/// caller's chosen width, and taking it here is what lets ONE kernel
/// serve both shells instead of each carrying its own copy.
#[inline]
pub(crate) fn asinh_series_composition<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    guard: u32,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let storage_zero = C::storage_zero();
    if raw == storage_zero {
        return storage_zero;
    }
    let working_scale = SCALE + guard;
    // Two-core: composition runs on the wide `Wagm` work int.
    let zero = <C::Wagm as BigInt>::ZERO;
    let one_at_working_scale = eg::pow10::<C::Wagm>(working_scale);
    let working_value = C::to_work_scaled_agm(raw, guard);
    let abs_working_value = if working_value < zero {
        zero - working_value
    } else {
        working_value
    };
    let inner = if abs_working_value >= one_at_working_scale {
        // |x| >= 1: ln|x| + ln(1 + sqrt(1 + 1/x^2)), so `x^2` never
        // forms and the working width is not exceeded by the radicand.
        let reciprocal =
            eg::div::<C::Wagm>(one_at_working_scale, abs_working_value, working_scale);
        let root = eg::sqrt_fixed::<C::Wagm>(
            one_at_working_scale + eg::mul::<C::Wagm>(reciprocal, reciprocal, working_scale),
            working_scale,
        );
        ln_series::<C>(abs_working_value, working_scale)
            + ln_series::<C>(one_at_working_scale + root, working_scale)
    } else {
        let root = eg::sqrt_fixed::<C::Wagm>(
            eg::mul::<C::Wagm>(abs_working_value, abs_working_value, working_scale)
                + one_at_working_scale,
            working_scale,
        );
        ln_series::<C>(abs_working_value + root, working_scale)
    };
    let signed = if working_value < zero { zero - inner } else { inner };
    round_to_storage_with_g::<C::Storage, C::Wagm>(
        signed,
        working_scale,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
    )
}

/// Inverse hyperbolic cosine, as `ln(x + √(x² − 1))`, defined for
/// `x ≥ 1`. For `x ≥ 2` the radicand is factored to keep `x²` inside
/// the working width; near 1 it takes the `log1p` gap form.
///
/// Panics if `x < 1`.
///
/// `guard` is the number of guard digits below `SCALE` to compute at —
/// a plain runtime argument, as on its two siblings here. The routed
/// `_strict` path passes the tier's `C::GUARD`; the `_approx` shell
/// passes the caller's chosen width. Taking it matters for more than
/// tidiness: the near-1 correction below is only sound because the
/// radicand is split, and the size of the band it protects depends on
/// `guard`, so a shell computing at its own guard MUST come through
/// here rather than carry its own copy.
#[inline]
pub(crate) fn acosh_ln_composition<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    guard: u32,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let working_scale = SCALE + guard;
    // Two-core: composition runs on the wide `Wagm` work int.
    let one_at_working_scale = eg::pow10::<C::Wagm>(working_scale);
    let working_value = C::to_work_scaled_agm(raw, guard);
    if working_value < one_at_working_scale {
        panic!("acosh: argument must be >= 1");
    }
    let two_at_working_scale = one_at_working_scale + one_at_working_scale;
    let inner = if working_value >= two_at_working_scale {
        let reciprocal = eg::div::<C::Wagm>(one_at_working_scale, working_value, working_scale);
        let root = eg::sqrt_fixed::<C::Wagm>(
            one_at_working_scale - eg::mul::<C::Wagm>(reciprocal, reciprocal, working_scale),
            working_scale,
        );
        C::ln_fixed_routed_agm::<SCALE>(working_value, working_scale)
            + C::ln_fixed_routed_agm::<SCALE>(one_at_working_scale + root, working_scale)
    } else {
        // Near 1: acosh(1+t) = log1p(t + sqrt(t*(t+2))).
        // The gap above 1 is exact, so `v^2 - 1 = (v-1)*(v+1) = t*(t+2)`
        // is formed without the catastrophic cancellation of
        // `mul(v,v) - 1` as `v -> 1`, and `log1p` avoids re-forming
        // `1 + arg` when the gap (hence `arg`) is tiny.
        let gap = working_value - one_at_working_scale;
        // The radicand is taken as sqrt(t)*sqrt(t+2), NOT as
        // sqrt(mul(t, t+2)). Forming the product FIRST rounds it to the
        // working scale, and `t*(t+2) = 2t + t^2` loses the `t^2` term
        // outright once `t^2 < 10^-(SCALE+guard)` — while that term is
        // still significant against the result's own last place,
        // because `sqrt` amplifies a relative radicand error of `t/2`
        // into a relative result error of `t/4` on a result of size
        // `sqrt(2t)`. The resulting error is `0.354 * 10^(SCALE-1.5k)`
        // ULP for `t = 10^-k`, non-zero exactly on
        //
        //     SCALE + guard < 2k    and    3k < 2*SCALE
        //
        // with BOTH ends strict, and stated against the `guard` actually
        // passed — not the tier's `GUARD`, since a caller-chosen guard
        // smaller than it opens the band LOWER. The upper end is
        // `3k < 2*SCALE`, NOT `k <= 2*SCALE/3`: where 3 divides SCALE
        // the endpoint `k = 2*SCALE/3` is correct rather than defective
        // (measured 114 -> 75 not 76, and 150 -> 99 not 100). The lower
        // end is exact too — the error is 0 at `2k == SCALE + guard`.
        //
        // The band is non-empty when an integer fits strictly between
        // those two bounds. `SCALE > 3*guard` is only the CONTINUOUS
        // relaxation of that: integer rounding at both ends pushes the
        // real onset later and makes it non-monotone there. Solving the
        // integer form at the tier's own `guard = 30` puts the first
        // non-empty SCALE at 95 — 96 is empty again, 97 upward is not —
        // rather than the 91 the relaxation implies. That onset is
        // DERIVED from the two bounds, not measured; the measured cells
        // are the two anchors above.
        //
        // Splitting the radicand keeps each factor at full
        // working-scale relative precision, so no small term is ever
        // rounded into a large one: the residual is then bounded by the
        // sqrt's own half-ULP at the working scale, i.e. ~10^-GUARD of
        // a storage ULP, for every `k`.
        let root = eg::mul::<C::Wagm>(
            eg::sqrt_fixed::<C::Wagm>(gap, working_scale),
            eg::sqrt_fixed::<C::Wagm>(gap + two_at_working_scale, working_scale),
            working_scale,
        );
        eg::log1p_fixed::<C::Wagm>(gap + root, working_scale)
    };
    round_to_storage_with_g::<C::Storage, C::Wagm>(
        inner,
        working_scale,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
    )
}

/// Inverse hyperbolic tangent, as `ln((1+x)/(1−x)) / 2`, defined for
/// `|x| < 1`.
///
/// Panics if `|x| >= 1`.
///
/// `guard` is the number of guard digits below `SCALE` to compute at —
/// a plain runtime argument, as on [`asinh_series_composition`]. The
/// routed `_strict` path passes the tier's `C::GUARD`; the `_approx`
/// shell passes the caller's chosen width, which is what lets this one
/// kernel serve both instead of the shell carrying a second copy of the
/// composition.
#[inline]
pub(crate) fn atanh_ln_composition<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    guard: u32,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let working_scale = SCALE + guard;
    // Two-core: the composition runs on the wide `Wagm` work int (its
    // ln + the gap-form subtraction), narrowing back to storage at the
    // end — so a narrowed primitive `W` does not clip the composition's
    // precision.
    let zero = <C::Wagm as BigInt>::ZERO;
    let one_at_working_scale = eg::pow10::<C::Wagm>(working_scale);
    let working_value = C::to_work_scaled_agm(raw, guard);
    let abs_working_value = if working_value < zero {
        zero - working_value
    } else {
        working_value
    };
    if abs_working_value >= one_at_working_scale {
        panic!("atanh: argument out of domain (-1, 1)");
    }
    // Gap form: atanh(x) = (1/2)*[ln(1+x) - ln(1-x)].
    // `one_at_working_scale - working_value` is the exact working-scale
    // gap (`working_value` is the storage input lifted by appending
    // guard zeros), so neither `ln_fixed` argument suffers the `(1-x)`
    // catastrophic cancellation the ratio form does near +-1. No
    // product is rounded into either argument, which is why this shell
    // carries no analogue of the `acosh` radicand correction above.
    let atanh_value = (C::ln_fixed_routed_agm::<SCALE>(
        one_at_working_scale + working_value,
        working_scale,
    ) - C::ln_fixed_routed_agm::<SCALE>(
        one_at_working_scale - working_value,
        working_scale,
    )) >> 1;
    round_to_storage_with_g::<C::Storage, C::Wagm>(
        atanh_value,
        working_scale,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
    )
}

#[cfg(all(test, feature = "std"))]
mod tests {
    //! The routed `LnComposition` arm is graded against the kept
    //! `Schoolbook` alternative — the Ziv-escalating reference the
    //! golden suite drives. Agreement across the key is what licenses
    //! `select` returning the composition everywhere.

    /// Cross-algorithm wall: the routed composition must equal the
    /// schoolbook reference bit-for-bit at every probed cell.
    ///
    /// Inputs are derived from the composition's own seams, not
    /// sampled: `acosh` branches at `x >= 2` and walls its domain at
    /// `x >= 1`, so the approach to 1 is swept decade by decade — that
    /// is the band where the un-factored radicand used to lose the
    /// `t^2` term. `atanh`'s domain is the open `(-1, 1)`, so both
    /// walls are approached from inside, at both signs.
    #[test]
    fn composition_matches_schoolbook_reference() {
        let mut fails: std::vec::Vec<std::string::String> = std::vec::Vec::new();
        let mut checks: u32 = 0;

        macro_rules! probe_tier {
            ($fails:expr, $checks:expr, $label:literal, $N:literal, $S:literal, $Core:ty) => {{
                type T = crate::D<crate::int::types::Int<$N>, $S>;
                let m = crate::support::rounding::DEFAULT_ROUNDING_MODE;
                let zero = <T>::ZERO;
                let ulp = <T>::MIN_POSITIVE;
                let one = <T>::ONE;
                let two = one + one;

                // Decade sweep of the gap above 1 (acosh) and of the
                // distance from 0 and from the +/-1 walls (atanh).
                let mut mags: std::vec::Vec<T> = std::vec::Vec::new();
                if let Ok(ten) = <T>::try_from(10i64) {
                    let mut mag = ulp;
                    for _ in 0..($S + 4) {
                        mags.push(mag);
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| mag * ten)) {
                            Ok(next) if next.to_bits() != mag.to_bits() => mag = next,
                            _ => break,
                        }
                    }
                }

                // ---- acosh: x = 1 + gap, plus the x >= 2 branch ----
                let mut xs: std::vec::Vec<T> = std::vec::Vec::new();
                xs.push(one);
                for g in &mags {
                    if let Ok(x) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| one + *g))
                    {
                        xs.push(x);
                    }
                }
                xs.push(two);
                if let Ok(v) = <T>::try_from(3i64) {
                    xs.push(v);
                }
                xs.push(<T>::MAX);
                for x in xs {
                    if x < one {
                        continue;
                    }
                    let raw = x.to_bits();
                    let a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        super::acosh_ln_composition::<$Core, $S>(
                            raw,
                            <$Core as super::WideTrigCore>::GUARD,
                            m,
                        )
                    }))
                    .ok();
                    let b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        crate::policy::trig::extra_rung::acosh_strict::<$Core, $S>(raw, m)
                    }))
                    .ok();
                    $checks += 1;
                    if a != b {
                        $fails.push(std::format!(
                            "{}<{}> acosh({x}) composition={:?} schoolbook={:?}",
                            $label,
                            $S,
                            a.map(|v| <T>::from_bits(v).to_string()),
                            b.map(|v| <T>::from_bits(v).to_string())
                        ));
                    }
                }

                // ---- atanh: both walls from inside, both signs ----
                let mut xs: std::vec::Vec<T> = std::vec::Vec::new();
                xs.push(zero);
                for g in &mags {
                    xs.push(*g);
                    if let Ok(w) =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| one - *g))
                    {
                        xs.push(w);
                    }
                }
                let base = xs.len();
                for i in 0..base {
                    let v = xs[i];
                    if v != zero {
                        xs.push(-v);
                    }
                }
                for x in xs {
                    if x >= one || x <= -one {
                        continue;
                    }
                    let raw = x.to_bits();
                    let a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        super::atanh_ln_composition::<$Core, $S>(
                            raw,
                            <$Core as super::WideTrigCore>::GUARD,
                            m,
                        )
                    }))
                    .ok();
                    let b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        crate::policy::trig::extra_rung::atanh_strict::<$Core, $S>(raw, m)
                    }))
                    .ok();
                    $checks += 1;
                    if a != b {
                        $fails.push(std::format!(
                            "{}<{}> atanh({x}) composition={:?} schoolbook={:?}",
                            $label,
                            $S,
                            a.map(|v| <T>::from_bits(v).to_string()),
                            b.map(|v| <T>::from_bits(v).to_string())
                        ));
                    }
                }
            }};
        }

        let prior_hook = std::panic::take_hook();
        std::panic::set_hook(std::boxed::Box::new(|_| {}));

        #[cfg(any(feature = "d57", feature = "wide"))]
        {
            probe_tier!(fails, checks, "D57", 3, 20, crate::types::widths::wide_trig_d57::Core);
            probe_tier!(fails, checks, "D57", 3, 56, crate::types::widths::wide_trig_d57::Core);
        }
        #[cfg(any(feature = "d76", feature = "wide"))]
        {
            probe_tier!(fails, checks, "D76", 4, 75, crate::types::widths::wide_trig_d76::Core);
        }
        #[cfg(any(feature = "d115", feature = "wide"))]
        {
            probe_tier!(fails, checks, "D115", 6, 86, crate::types::widths::wide_trig_d115::Core);
            probe_tier!(fails, checks, "D115", 6, 114, crate::types::widths::wide_trig_d115::Core);
        }
        #[cfg(any(feature = "d307", feature = "x-wide"))]
        {
            probe_tier!(fails, checks, "D307", 16, 150, crate::types::widths::wide_trig_d307::Core);
        }

        std::panic::set_hook(prior_hook);

        assert!(checks > 0, "no tier was probed - the test graded nothing");
        std::println!("composition vs schoolbook: {checks} probes, {} differ", fails.len());
        assert!(
            fails.is_empty(),
            "{} of {checks} probes differ between the routed composition and the \
             schoolbook reference:\n{}",
            fails.len(),
            fails.join("\n")
        );
    }
}
