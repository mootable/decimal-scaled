//! Forward-trig schoolbook reference kernels -- sin / cos / tan / atan.
//!
//! These are the **naive textbook** realisations of the forward family,
//! registered as an unrouted `Schoolbook` arm of
//! [`crate::policy::trig::forward::Algorithm`]. They exist as a
//! correctness reference (and an A/B microbench partner) for the tuned
//! `*_series` / Tang kernels -- `select` never routes to them.
//!
//! Each function is the plain textbook definition, computed in the
//! guard-digit work type and dispatched DOWN to the `Int<N>` layer for
//! its integer work (the leaf `*_fixed` Maclaurin kernels do their
//! arithmetic in `W: BigInt` / the 256-bit `Fixed` work int):
//!
//! - **sin / cos** -- a Maclaurin series after `mod 2pi` argument
//!   reduction into a small quadrant: the leaf `sin_fixed` / `cos_fixed`
//!   (wide) and `sin_fixed` / `sin_cos_fixed` (narrow) ARE that
//!   range-reduced Taylor evaluation.
//! - **tan** = `sin / cos` -- the joint `sin_cos_fixed` kernel then one
//!   work-int divide (`C::div` wide / `Fixed::div` narrow). Panics at the
//!   poles where the cosine is zero (odd multiples of pi/2).
//! - **atan** -- the arctan Maclaurin series with argument reduction
//!   supplied by the leaf `atan_fixed`. Result in `(-pi/2, pi/2)`.
//!
//! Correct rounding: wide kernels go through
//! [`WideTrigCore::round_to_storage_directed`] (the same Ziv-escalating
//! narrowing the `*_series` kernels use); narrow kernels round with
//! `Fixed::round_to_i128_with` at the strict guard. No small-x linear
//! shortcut and no Tang table -- the schoolbook is the unembellished
//! textbook path.

use crate::algos::ln::ln_series_2limb::STRICT_GUARD;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::{atan_fixed, sin_cos_fixed, sin_fixed, to_fixed};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// -- Wide tier -- generic over the tier core `C: WideTrigCore` --------

/// Schoolbook `sin` for a wide tier -- `sin(x)` via the range-reduced
/// Maclaurin leaf [`WideTrigCore::sin_fixed`], rounded correctly with
/// Ziv escalation.
#[inline]
#[must_use]
pub(crate) fn sin_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::sin_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// Schoolbook `cos` for a wide tier -- `cos(x)` via the range-reduced
/// Maclaurin leaf [`WideTrigCore::cos_fixed`].
#[inline]
#[must_use]
pub(crate) fn cos_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::cos_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// Schoolbook `tan` for a wide tier -- the textbook quotient
/// `tan(x) = sin(x) / cos(x)` from the joint
/// [`WideTrigCore::sin_cos_fixed`] leaf, divided in the work int via
/// [`WideTrigCore::div`]. Panics at the poles (cosine zero).
#[inline]
#[must_use]
pub(crate) fn tan_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let (sin_w, cos_w) = C::sin_cos_fixed(C::to_work_w(raw, guard), w);
        if cos_w == C::zero() {
            panic!("schoolbook tan: cosine is zero (argument is an odd multiple of pi/2)");
        }
        C::div(sin_w, cos_w, w)
    })
}

/// Schoolbook `atan` for a wide tier -- the arctan Maclaurin series with
/// argument reduction supplied by the leaf [`WideTrigCore::atan_fixed`].
/// Result in `(-pi/2, pi/2)`.
#[inline]
#[must_use]
pub(crate) fn atan_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        C::atan_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

// -- Narrow tier -- `Int<2>` storage, math in the 256-bit `Fixed` -----

/// Narrow schoolbook `sin` core -- `sin(x)` via the range-reduced
/// Maclaurin `Fixed` leaf [`sin_fixed`], rounded at the strict guard.
#[inline]
#[must_use]
fn sin_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    sin_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("sin", SCALE))
}

/// Narrow schoolbook `cos` core -- `cos(x)` recovered from the joint
/// [`sin_cos_fixed`] `Fixed` leaf (shared mod-2pi reduction).
#[inline]
#[must_use]
fn cos_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let w = SCALE + STRICT_GUARD;
    let (_s, c) = sin_cos_fixed(to_fixed(raw), w);
    c.round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("cos", SCALE))
}

/// Narrow schoolbook `tan` core -- the textbook quotient
/// `tan(x) = sin(x) / cos(x)` from the joint [`sin_cos_fixed`] `Fixed`
/// leaf. Panics at the poles (cosine zero).
#[inline]
#[must_use]
fn tan_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let (s, c) = sin_cos_fixed(to_fixed(raw), w);
    if c.is_zero() {
        panic!("schoolbook tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    s.div(c, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("tan", SCALE))
}

/// Narrow schoolbook `atan` core -- the arctan Maclaurin series with
/// argument reduction supplied by the [`atan_fixed`] `Fixed` leaf.
#[inline]
#[must_use]
fn atan_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    atan_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atan", SCALE))
}

// -- `Int<2>` entry points (bridge `Int<2> -> i128`) ------------------

/// Narrow schoolbook `sin` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn sin_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(sin_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook `cos` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn cos_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(cos_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook `tan` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn tan_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(tan_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook `atan` for `Int<2>` storage.
#[inline]
#[must_use]
pub(crate) fn atan_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(atan_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}
