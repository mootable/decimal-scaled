// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Angle-conversion `MulPiRatio` kernels -- to_degrees / to_radians.
//!
//! The routed realisations of `to_degrees`/`to_radians` for the wide
//! tiers: multiply by the exact `180/pi` (resp. `pi/180`) ratio in the
//! guard-digit work integer, then round correctly to storage. These are
//! the `MulPiRatio` algorithm named by
//! `crate::policy::to_degrees` / `crate::policy::to_radians`; the policy `MulPiRatio`
//! arm routes DOWN to them directly (no inversion through the inherent
//! `*_strict_with` shell).
//!
//! Identities, dispatched DOWN to the `Int<N>` work int via the tier core:
//! - to_degrees(x) = x * 180 / pi
//! - to_radians(x) = x * pi / 180
//!
//! These never call a decimal `*_strict_with` on their own value.

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::support::rounding::RoundingMode;

/// `MulPiRatio` to_degrees for a wide tier -- `x * 180 / pi` in the
/// guard-digit work integer, correctly rounded to storage.
#[inline]
#[must_use]
pub(crate) fn to_degrees_mul_pi_ratio<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let working_scale = SCALE + C::GUARD;
    let working_value = C::to_work(raw);
    // `x * 180/π`: multiply by the exact oracle `deg_per_rad` factor
    // instead of dividing by `π` (the divide was the main cost).
    let degrees = C::mul(working_value, C::deg_per_rad::<SCALE>(working_scale), working_scale);
    C::round_to_storage_with(degrees, working_scale, SCALE, mode)
}

/// `MulPiRatio` to_radians for a wide tier -- `x * pi / 180` in the
/// guard-digit work integer, correctly rounded to storage.
///
/// **NOT equivalent to the inherent `to_radians_strict_with` shell, and
/// not a drop-in for it (measured 2026-09-03).** The shell forms
/// `mul(x, pi)` and then divides by 180; this kernel multiplies by the
/// `rad_per_deg` table constant. Since `pi/180 < 1`, that constant's
/// absolute quantum at the working scale is `10^-GUARD` on a value
/// smaller than 1, so the operand's magnitude amplifies it: the kernel
/// gives up about `log10(180)` digits of relative precision that the
/// shell keeps by scaling through `pi` first. At `D57<0>` the kernel
/// returns `to_radians(10^32)` as `...768500` -- quantized to ~`10^2`,
/// wrong by 11 units -- where the shell returns the correctly rounded
/// `...768489`. Routing the wide tiers here would be a precision
/// regression, so the wide shells are deliberately left unrouted
/// pending a policy decision on carrying both forms as named
/// algorithms.
#[inline]
#[must_use]
pub(crate) fn to_radians_mul_pi_ratio<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let working_scale = SCALE + C::GUARD;
    let working_value = C::to_work(raw);
    // `x * π/180`: multiply by the exact oracle `rad_per_deg` factor.
    let radians = C::mul(working_value, C::rad_per_deg::<SCALE>(working_scale), working_scale);
    C::round_to_storage_with(radians, working_scale, SCALE, mode)
}

// ── Unit tests: the MulPiRatio kernel is bit-exact against the routed
// `*_strict_with` shell at every input, scale, tier and mode (skill 7).
#[cfg(test)]
mod tests {
    use super::*;

    const MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::D;
        use crate::int::types::Int;
        use crate::types::widths::wide_trig_d57::Core;

        const S: u32 = 19;
        fn raw9(units: i128) -> Int<3> {
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }
        const INPUTS9: [i128; 5] = [
            0,
            1_000_000_000,
            45_000_000_000,
            -1_000_000_000,
            -45_000_000_000,
        ];

        #[test]
        fn to_degrees_to_radians_mul_pi_ratio_match_routed() {
            for &units in &INPUTS9 {
                let raw = raw9(units);
                for &mode in &MODES {
                    assert_eq!(
                        to_degrees_mul_pi_ratio::<Core, S>(raw, mode),
                        D::<Int<3>, S>(raw).to_degrees_strict_with(mode).0,
                        "D57 to_degrees MulPiRatio != routed at units={units} mode={mode:?}"
                    );
                    assert_eq!(
                        to_radians_mul_pi_ratio::<Core, S>(raw, mode),
                        D::<Int<3>, S>(raw).to_radians_strict_with(mode).0,
                        "D57 to_radians MulPiRatio != routed at units={units} mode={mode:?}"
                    );
                }
            }
        }
    }

    /// The inherent `to_radians_strict_with` shell computes exactly the
    /// kept `Schoolbook` kernel, across tiers, scales, a decade magnitude
    /// ladder climbing the whole representable range, the notable angles,
    /// and all eight rounding modes.
    ///
    /// This is the evidence behind leaving `to_radians` unrouted. The
    /// shell's inline body is `mul(x, pi) / lit(180)` — the
    /// `angle_schoolbook::to_radians_schoolbook` expression verbatim — so
    /// `Schoolbook`, not the `MulPiRatio` arm `select` currently returns,
    /// is the routing target that preserves today's values. Routing the
    /// wide cells as `select` stands would be a precision regression:
    /// `MulPiRatio` is wrong by 11 units at `D57<0>::to_radians(10^32)`
    /// (see [`to_radians_mul_pi_ratio`]).
    ///
    /// `to_degrees` has no counterpart assertion here on purpose. Its
    /// schoolbook is a genuinely different algorithm (`div(x * 180, pi)`
    /// rather than a multiply by `180/pi`) and is about `log10(180/pi)`
    /// digits weaker, landing 1 ULP low at the directed-mode boundaries
    /// — e.g. `D57<0>::to_degrees(10^28)` under `Ceiling`, where the true
    /// value is `...548141.0517`, the routed kernel returns `...142`, and
    /// schoolbook returns `...141`. So it cannot serve as an equivalence
    /// reference, despite `angle_schoolbook`'s module doc claiming it
    /// matches bit-exactly.
    // Every tier this sweeps is a wide one, so the whole probe is
    // `_wide-support`-only; without it the body would expand to nothing.
    #[cfg(all(feature = "std", feature = "_wide-support"))]
    #[test]
    fn to_radians_shell_computes_the_schoolbook_kernel_across_the_surface() {
        let mut fails: std::vec::Vec<std::string::String> = std::vec::Vec::new();
        let mut checks: u32 = 0;
        let mut rad_fails: u32 = 0;

        macro_rules! probe_tier {
            ($label:literal, $N:literal, $S:literal, $Core:ty) => {{
                type T = crate::D<crate::int::types::Int<$N>, $S>;
                let one = <T>::ONE;

                // Decade ladder from one ULP up to overflow, plus the
                // notable angles on both sides of the conversion.
                let mut xs: std::vec::Vec<T> = std::vec::Vec::new();
                xs.push(<T>::ZERO);
                // Climb the WHOLE representable range, not a fixed number
                // of steps: the divergence hunted here is magnitude-driven,
                // so the ladder must actually reach the top of the tier.
                if let Ok(ten) = <T>::try_from(10i64) {
                    let mut mag = <T>::MIN_POSITIVE;
                    for _ in 0..1024 {
                        xs.push(mag);
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| mag * ten)) {
                            Ok(next) if next.to_bits() != mag.to_bits() => mag = next,
                            _ => break,
                        }
                    }
                }
                // The top of the range, approached by successive halving:
                // brackets where a magnitude-driven divergence sets in.
                {
                    let mut hi = <T>::MAX;
                    for _ in 0..24 {
                        xs.push(hi);
                        let next = hi >> 1;
                        if next.to_bits() == hi.to_bits() || next == <T>::ZERO {
                            break;
                        }
                        hi = next;
                    }
                }
                for units in [1i64, 30, 45, 57, 60, 90, 180, 270, 360] {
                    if let Ok(v) = <T>::try_from(units) {
                        xs.push(v);
                        // Just off the round angle, to land the working
                        // scale away from a trailing run of zeros.
                        if let Ok(off) =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                v + <T>::MIN_POSITIVE
                            }))
                        {
                            xs.push(off);
                        }
                    }
                }
                xs.push(one);
                xs.push(<T>::MAX);

                let base = xs.len();
                for i in 0..base {
                    let v = xs[i];
                    if v != <T>::ZERO {
                        if let Ok(neg) =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -v))
                        {
                            xs.push(neg);
                        }
                    }
                }

                for x in xs {
                    let raw = x.to_bits();
                    for &mode in &MODES {
                        let kr = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            crate::algos::trig::angle_schoolbook::to_radians_schoolbook::<$Core, $S>(
                                raw, mode,
                            )
                        }))
                        .ok();
                        let sr = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            <T>::from_bits(raw).to_radians_strict_with(mode).to_bits()
                        }))
                        .ok();
                        checks += 1;
                        if kr != sr {
                            rad_fails += 1;
                            if fails.len() < 20 {
                                fails.push(std::format!(
                                    "{}<{}> to_radians({x}) mode={mode:?} schoolbook={:?} shell={:?}",
                                    $label,
                                    $S,
                                    kr.map(|v| v.to_string()),
                                    sr.map(|v| v.to_string())
                                ));
                            }
                        }
                    }
                }
            }};
        }

        let prior_hook = std::panic::take_hook();
        std::panic::set_hook(std::boxed::Box::new(|_| {}));

        #[cfg(any(feature = "d57", feature = "wide"))]
        {
            probe_tier!("D57", 3, 0, crate::types::widths::wide_trig_d57::Core);
            probe_tier!("D57", 3, 19, crate::types::widths::wide_trig_d57::Core);
            probe_tier!("D57", 3, 30, crate::types::widths::wide_trig_d57::Core);
            probe_tier!("D57", 3, 56, crate::types::widths::wide_trig_d57::Core);
        }
        #[cfg(any(feature = "d76", feature = "wide"))]
        {
            probe_tier!("D76", 4, 40, crate::types::widths::wide_trig_d76::Core);
            probe_tier!("D76", 4, 75, crate::types::widths::wide_trig_d76::Core);
        }
        #[cfg(any(feature = "d115", feature = "wide"))]
        {
            probe_tier!("D115", 6, 86, crate::types::widths::wide_trig_d115::Core);
            probe_tier!("D115", 6, 114, crate::types::widths::wide_trig_d115::Core);
        }
        #[cfg(any(feature = "d153", feature = "wide"))]
        {
            probe_tier!("D153", 8, 100, crate::types::widths::wide_trig_d153::Core);
        }
        #[cfg(any(feature = "d230", feature = "wide"))]
        {
            probe_tier!("D230", 12, 200, crate::types::widths::wide_trig_d230::Core);
        }
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        {
            probe_tier!("D307", 16, 150, crate::types::widths::wide_trig_d307::Core);
        }

        std::panic::set_hook(prior_hook);

        assert!(checks > 0, "no tier was probed - the test graded nothing");
        std::println!(
            "to_radians schoolbook vs inherent shell: {checks} probes, {rad_fails} differ"
        );
        assert!(
            fails.is_empty(),
            "{rad_fails} of {checks} probes differ between the to_radians schoolbook \
             kernel and the inherent shell:\n{}",
            fails.join("\n")
        );
    }
}
