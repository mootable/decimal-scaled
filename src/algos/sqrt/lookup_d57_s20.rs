//! Bespoke square-root kernel slot for `D57<20>`.
//!
//! Currently a thin pass-through to [`super::generic_wide::sqrt_d57`] —
//! byte-identical output, identical cost. This file exists so the
//! [`crate::policy::sqrt`] override for `D57<20>` has a stable callee
//! whose body can be replaced with a tuned implementation (lookup
//! table, specialised initial guess, scale-20 domain reduction, …)
//! without touching the policy file or the typed method shell.
//!
//! When the bespoke kernel lands here, **replace this file's
//! implementation only** — the call site in `crate::policy::sqrt`
//! stays unchanged.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::rounding::RoundingMode;
use crate::wide_int::Int192;

/// `D57<20>` square-root kernel. Pass-through to the generic wide
/// kernel today; replace with a tuned implementation when ready.
#[inline]
#[must_use]
pub(crate) fn sqrt(raw: Int192, mode: RoundingMode) -> Int192 {
    super::generic_wide::sqrt_d57(raw, 20, mode)
}
