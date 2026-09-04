// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Floating-point power policy — the per-(N, SCALE) algorithm matcher.
//!
//! `D<Int<N>, SCALE>::powf_with(exp, mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`.
//!
//! # The power algorithms
//!
//! `powf` is the hybrid `b^y = exp(y * ln b)`: a composition of the `exp`
//! and `ln` algorithms. Two registered compositions and one reference:
//!
//! - `ExpWithLn` — the bare composition. Routed on the narrow tiers,
//!   realised on the 256-bit `Fixed` intermediate via
//!   `pow::powf_series_2limb` (D18 widened to Int<2>), whose
//!   integer-exponent square-and-multiply pin (`powi_raw`) lives inside
//!   the kernel; its wide realisation is the C-generic
//!   `pow::pow_schoolbook` over the tier's `WideTrigCore` core.
//! - `PinnedExpWithLn` — the composition with the exact-power pins, the
//!   algebraic `x^0.5` pin and the result-sized working lift in front of
//!   it (`pow::powf_pinned_exp_with_ln`). Routed on every wide tier: the
//!   wide `powf_with` shell calls [`dispatch`] like every other
//!   shell, and `select` names this algorithm for `N >= 3`.
//! - `Schoolbook` — the unrouted naive `exp(y*ln x)` reference.
//!
//! One door, total over the key: `dispatch` serves the narrow tiers
//! (which have no `WideTrigCore` core) and the wide tiers alike.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    ExpWithLn,
    /// `exp(y·ln x)` with the exact-power pins in front of it — the
    /// integer-power and terminating-reciprocal pins, the algebraic
    /// `x^0.5 ≡ √x` pin, and the analytically-gated result-sized working
    /// lift. Realised by `algos::pow::powf_pinned_exp_with_ln`; this is
    /// what the wide `powf_with` surface actually runs.
    ///
    /// A DISTINCT algorithm from [`Algorithm::ExpWithLn`], not a tuned
    /// spelling of it: each pin exists because the bare composition
    /// rounds wrong somewhere (a directed-mode reciprocal, a
    /// perfect-square base, a deep-overflow argument that wraps the
    /// lifted `ln`). `select` names it for every wide width (`N >= 3`);
    /// on the narrow tiers its realisation is the same
    /// `powf_series_2limb` kernel `ExpWithLn` runs, so the two differ
    /// only where the pins are a separate composition.
    PinnedExpWithLn,
    #[allow(dead_code)]
    Schoolbook,
}

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    match N {
        // Narrow tiers: the `Fixed`-256 kernel, which carries its own
        // integer-exponent pin. Both compositions realise as this same
        // kernel here; `ExpWithLn` is the name of what runs.
        1 | 2 => Select::ByAlgorithm(Algorithm::ExpWithLn),
        // Wide tiers: the pinned composition — what the wide
        // `powf_with` shell has always run. Naming it here is what
        // puts that production path under the matcher.
        _ => Select::ByAlgorithm(Algorithm::PinnedExpWithLn),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(choose) => choose(&base),
    };
    match algo {
        Algorithm::ExpWithLn => exp_with_ln_routed::<N, SCALE>(base, exponent, mode),
        Algorithm::PinnedExpWithLn => pinned_exp_with_ln_routed::<N, SCALE>(base, exponent, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(base, exponent, mode),
    }
}

/// The `PinnedExpWithLn` realisation per width — the wide production
/// `powf` path, reached only through [`dispatch`].
///
/// The `match N` is the bridge from the `(N, SCALE)` matcher key to the
/// tier's `WideTrigCore` core: on stable Rust nothing else names a
/// width's core from its limb count, and the narrow tiers (which have no
/// core) must be served by the same door for `select` to stay total over
/// the key. Every wide arm names the SAME generic kernel at its own core;
/// the unchosen arms are pruned per monomorphisation.
///
/// The `x^0.5` pin is handed the cell's own `sqrt::dispatch`, which is
/// the engine the shell reached through `self.sqrt_with(mode)`.
///
/// The narrow arms are the SAME `powf_series_2limb` kernel `ExpWithLn`
/// uses: that kernel already carries its own integer-exponent pin
/// (`powi_raw`), so the two algorithms differ only on the wide tiers,
/// which is where the pins are a separate composition. Because those
/// arms are mentioned from every wide instantiation of this fn, that
/// kernel must be monomorphisable at every wide `SCALE` (see its
/// `powi_raw_checked`).
#[inline]
fn pinned_exp_with_ln_routed<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    // Only the wide arms below use it; a narrow-only build has none.
    #[cfg(feature = "_wide-support")]
    macro_rules! pinned {
        ($k:literal, $core:ident) => {
            crate::algos::pow::powf_pinned_exp_with_ln::powf_pinned_exp_with_ln::<
                crate::types::widths::$core::Core,
                SCALE,
            >(
                base.resize_to::<Int<$k>>(),
                exponent.resize_to::<Int<$k>>(),
                mode,
                crate::policy::sqrt::dispatch::<$k, SCALE>,
            )
            .resize_to::<Int<N>>()
        };
    }
    match N {
        1 | 2 => crate::algos::pow::powf_series_2limb::powf::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf", SCALE)
        }),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => pinned!(3, wide_trig_d57),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => pinned!(4, wide_trig_d76),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => pinned!(6, wide_trig_d115),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => pinned!(8, wide_trig_d153),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => pinned!(12, wide_trig_d230),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => pinned!(16, wide_trig_d307),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => pinned!(24, wide_trig_d462),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => pinned!(32, wide_trig_d616),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => pinned!(48, wide_trig_d924),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => pinned!(64, wide_trig_d1232),
        _ => crate::algos::pow::powf_series_2limb::powf::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf", SCALE)
        }),
    }
}

/// The `checked` primitive for `checked_powf[_with]`.
///
/// Narrow tiers (`N == 1 | 2`) run the seamed `powf_series_2limb`
/// kernel: its out-of-range `None` (and the `Int<2> -> Int<1>` narrow
/// fit) propagate exactly. The wide arms hop to the tier's inherent
/// `powf_with` shell — the SAME path the default wide `powf`
/// surface takes (the shell delegates to [`dispatch`], and bit-identity
/// with the default form is the contract) — so a wide out-of-range
/// result still panics inside that shell.
#[inline]
#[must_use]
pub(crate) fn checked_dispatch<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::pow::powf_series_2limb::powf::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::D::<Int<3>, SCALE>(base.resize_to::<Int<3>>()).powf_with(crate::D::<Int<3>, SCALE>(exponent.resize_to::<Int<3>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::D::<Int<4>, SCALE>(base.resize_to::<Int<4>>()).powf_with(crate::D::<Int<4>, SCALE>(exponent.resize_to::<Int<4>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::D::<Int<6>, SCALE>(base.resize_to::<Int<6>>()).powf_with(crate::D::<Int<6>, SCALE>(exponent.resize_to::<Int<6>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::D::<Int<8>, SCALE>(base.resize_to::<Int<8>>()).powf_with(crate::D::<Int<8>, SCALE>(exponent.resize_to::<Int<8>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::D::<Int<12>, SCALE>(base.resize_to::<Int<12>>()).powf_with(crate::D::<Int<12>, SCALE>(exponent.resize_to::<Int<12>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::D::<Int<16>, SCALE>(base.resize_to::<Int<16>>()).powf_with(crate::D::<Int<16>, SCALE>(exponent.resize_to::<Int<16>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::D::<Int<24>, SCALE>(base.resize_to::<Int<24>>()).powf_with(crate::D::<Int<24>, SCALE>(exponent.resize_to::<Int<24>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::D::<Int<32>, SCALE>(base.resize_to::<Int<32>>()).powf_with(crate::D::<Int<32>, SCALE>(exponent.resize_to::<Int<32>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::D::<Int<48>, SCALE>(base.resize_to::<Int<48>>()).powf_with(crate::D::<Int<48>, SCALE>(exponent.resize_to::<Int<48>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::D::<Int<64>, SCALE>(base.resize_to::<Int<64>>()).powf_with(crate::D::<Int<64>, SCALE>(exponent.resize_to::<Int<64>>()), mode).0.resize_to::<Int<N>>()),
        _ => crate::algos::pow::powf_series_2limb::powf::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>),
    }
}

#[inline]
fn exp_with_ln_routed<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 | 2 => crate::algos::pow::powf_series_2limb::powf::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf", SCALE)
        }),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d57::Core, SCALE,
        >(base.resize_to::<Int<3>>(), exponent.resize_to::<Int<3>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d76::Core, SCALE,
        >(base.resize_to::<Int<4>>(), exponent.resize_to::<Int<4>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d115::Core, SCALE,
        >(base.resize_to::<Int<6>>(), exponent.resize_to::<Int<6>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d153::Core, SCALE,
        >(base.resize_to::<Int<8>>(), exponent.resize_to::<Int<8>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d230::Core, SCALE,
        >(base.resize_to::<Int<12>>(), exponent.resize_to::<Int<12>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d307::Core, SCALE,
        >(base.resize_to::<Int<16>>(), exponent.resize_to::<Int<16>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d462::Core, SCALE,
        >(base.resize_to::<Int<24>>(), exponent.resize_to::<Int<24>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d616::Core, SCALE,
        >(base.resize_to::<Int<32>>(), exponent.resize_to::<Int<32>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d924::Core, SCALE,
        >(base.resize_to::<Int<48>>(), exponent.resize_to::<Int<48>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d1232::Core, SCALE,
        >(base.resize_to::<Int<64>>(), exponent.resize_to::<Int<64>>(), mode)
        .resize_to::<Int<N>>(),
        _ => crate::algos::pow::powf_series_2limb::powf::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf", SCALE)
        }),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(
            crate::algos::pow::pow_schoolbook::pow_schoolbook_strict::<SCALE>(
                base.resize_to::<Int<2>>(),
                exponent.resize_to::<Int<2>>(),
                mode,
            ),
            "powf",
            SCALE,
        ),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d57::Core, SCALE,
        >(base.resize_to::<Int<3>>(), exponent.resize_to::<Int<3>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d76::Core, SCALE,
        >(base.resize_to::<Int<4>>(), exponent.resize_to::<Int<4>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d115::Core, SCALE,
        >(base.resize_to::<Int<6>>(), exponent.resize_to::<Int<6>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d153::Core, SCALE,
        >(base.resize_to::<Int<8>>(), exponent.resize_to::<Int<8>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d230::Core, SCALE,
        >(base.resize_to::<Int<12>>(), exponent.resize_to::<Int<12>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d307::Core, SCALE,
        >(base.resize_to::<Int<16>>(), exponent.resize_to::<Int<16>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d462::Core, SCALE,
        >(base.resize_to::<Int<24>>(), exponent.resize_to::<Int<24>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d616::Core, SCALE,
        >(base.resize_to::<Int<32>>(), exponent.resize_to::<Int<32>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d924::Core, SCALE,
        >(base.resize_to::<Int<48>>(), exponent.resize_to::<Int<48>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d1232::Core, SCALE,
        >(base.resize_to::<Int<64>>(), exponent.resize_to::<Int<64>>(), mode)
        .resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(
            crate::algos::pow::pow_schoolbook::pow_schoolbook_strict::<SCALE>(
                base.resize_to::<Int<2>>(),
                exponent.resize_to::<Int<2>>(),
                mode,
            ),
            "powf",
            SCALE,
        ),
    }
}

