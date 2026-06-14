// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer sum-of-squares algorithm family.
//!
//! `sum_sq(a, b) = a^2 + b^2` for `Int<N>`, generic over the storage limb
//! count `N`. The sqrt-free magnitude primitive: a distance comparison
//! compares `a^2 + b^2` rather than `sqrt(a^2 + b^2)` (the root is
//! monotonic), so this is the cheap inner core the hypotenuse family shares.
//!
//! - [`sum_sq_schoolbook`](sum_sq_schoolbook::sum_sq_schoolbook) -- the
//!   method form: form `a^2 + b^2` in a limb `Buf2` scratch, fit-check to
//!   `Int<N>`; [`None`] signals true overflow (the sum exceeds the signed
//!   range of `Int<N>`).
//! - [`sum_sq_radicand`](sum_sq_schoolbook::sum_sq_radicand) -- the shared
//!   full-width radicand former, used directly by
//!   [`crate::int::algos::hypot::hypot_pythagoras`] (which roots it rather
//!   than fit-checking it, so it keeps every representable hypot).
//!
//! The per-`N` policy lives in [`crate::int::policy::sum_sq`].

pub(crate) mod sum_sq_schoolbook;
/// Comba full-width squaring variant — selected by [`crate::int::policy::sum_sq`]
/// at the wide tiers (`N >= 3`); bit-identical to the schoolbook radicand.
pub(crate) mod sum_sq_comba;
