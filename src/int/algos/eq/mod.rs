// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer equality algorithm family.
//!
//! - [`eq_limbwise`](eq_limbwise::eq_limbwise) — the width-agnostic
//!   limb-by-limb two's-complement equality test selected by
//!   [`crate::int::policy::eq`].
//! - [`eq_xor_fold`](eq_xor_fold::eq_xor_fold) — CANDIDATE (unwired): an
//!   OR-fold of per-limb XORs, a branchless equality cheaper than reusing the
//!   comparison kernel. Kept per `docs/ARCHITECTURE.md` → "Keeping the
//!   alternatives"; reached only by its own bit-identity test until a
//!   policy-mapper pass benches and (if it wins) wires it.

pub(crate) mod eq_limbwise;
pub(crate) mod eq_xor_fold;
