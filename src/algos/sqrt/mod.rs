//! Square-root algorithm family.
//!
//! Each variant in this module is a kernel — a free function taking the
//! raw storage integer plus the runtime scale and rounding mode, and
//! returning the raw storage integer of the square root.
//!
//! Variants:
//!
//! - [`generic_wide`] — `isqrt` over a wide work integer that strictly
//!   covers `raw · 10^SCALE`. Width-level default for the wide tiers
//!   (D56 / D76 / D114 / D153 / D230 / D307 / D461 / D615 / D923 /
//!   D1231). The result is exact to the last representable place
//!   (within 0.5 ULP) under any of the six [`RoundingMode`]s.
//! - [`mg_divide_d38`] — hand-tuned `mul2` + 256-bit `isqrt_256`
//!   tailored to D38's `i128` storage. **Width specialisation for
//!   D38** — captures the kernel that has shipped with D38 since
//!   before the algorithm library existed.
//! - [`widen_to_d38`] — widen → `mg_divide_d38::sqrt` → narrow.
//!   **Width specialisation for D9 and D18**; captures the existing
//!   delegation pattern from `decl_strict_transcendentals_via_d38!`.
//! - [`lookup_d56_s20`] — stub kernel slot for `D56<20>` tuning;
//!   currently delegates to `generic_wide` byte-for-byte.
//!
//! [`RoundingMode`]: crate::rounding::RoundingMode

pub(crate) mod generic_wide;
pub(crate) mod lookup_d56_s20;
pub(crate) mod mg_divide_d38;
pub(crate) mod widen_to_d38;
