// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer cubing algorithm family.
//!
//! - [`cube_schoolbook`] — sqr-then-multiply (`x²·x`) via the const
//!   [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] and
//!   [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] kernels. The per-`N`
//!   choice lives in [`crate::int::policy::cube`].
//!
//! - [`cube_fused_comba`] — CANDIDATE (unwired): a single fused
//!   product-scanning pass for `x³`, the cube analogue of the symmetric comba
//!   square. Kept per `docs/ARCHITECTURE.md` → "Keeping the alternatives";
//!   reached only by its own bit-identity test until a policy-mapper pass
//!   benches and (if it wins) wires it.
//!
//! [`cube_schoolbook`]: crate::int::algos::cube::cube_schoolbook::cube_schoolbook
//! [`cube_fused_comba`]: crate::int::algos::cube::cube_fused_comba::cube_fused_comba

pub(crate) mod cube_fused_comba;
pub(crate) mod cube_schoolbook;
