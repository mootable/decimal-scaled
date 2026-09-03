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

/// Inverse hyperbolic cosine, as `ln(x + √(x² − 1))`, defined for
/// `x ≥ 1`. For `x ≥ 2` the radicand is factored to keep `x²` inside
/// the working width; near 1 it takes the `log1p` gap form.
///
/// Panics if `x < 1`.
#[inline]
pub(crate) fn acosh_ln_composition<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let working_scale = SCALE + C::GUARD;
    // Two-core: composition runs on the wide `Wagm` work int.
    let one_at_working_scale = eg::pow10::<C::Wagm>(working_scale);
    let working_value = C::to_work_scaled_agm(raw, C::GUARD);
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
        // outright once `t^2 < 10^-(SCALE+GUARD)` — while that term is
        // still significant against the result's own last place,
        // because `sqrt` amplifies a relative radicand error of `t/2`
        // into a relative result error of `t/4` on a result of size
        // `sqrt(2t)`. The resulting error is `0.354 * 10^(SCALE-1.5k)`
        // ULP for `t = 10^-k`, non-zero exactly in the window
        // `(SCALE+GUARD)/2 < k <= 2*SCALE/3`, which is non-empty only
        // for `SCALE > 90` (measured at D115<114> and D307<150>).
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
#[inline]
pub(crate) fn atanh_ln_composition<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let working_scale = SCALE + C::GUARD;
    // Two-core: the composition runs on the wide `Wagm` work int (its
    // ln + the gap-form subtraction), narrowing back to storage at the
    // end — so a narrowed primitive `W` does not clip the composition's
    // precision.
    let zero = <C::Wagm as BigInt>::ZERO;
    let one_at_working_scale = eg::pow10::<C::Wagm>(working_scale);
    let working_value = C::to_work_scaled_agm(raw, C::GUARD);
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
                        super::acosh_ln_composition::<$Core, $S>(raw, m)
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
                        super::atanh_ln_composition::<$Core, $S>(raw, m)
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
