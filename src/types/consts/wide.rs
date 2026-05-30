// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Per-work-width constant references for the wide-transcendental core.
//!
//! The `DecimalConstants` impls for every decimal width now come from the
//! SINGLE generic impl in `src/types/consts/d38.rs`, sourced from the unified
//! per-scale table (`crate::consts`). The former per-tier machinery that lived
//! here — the `*_RAW_*` consts, the `*_at_target_d*` rescale helpers, the three
//! hand impls (D76/D153/D307), and the `decl_wide_consts_tier!` macro
//! (D57/D115/D230/D462/D616/D924/D1232) — has been removed.
//!
//! What remains is the legacy per-work-width reference (`*_w_ref`) consumed by
//! the wide-transcendental core's `const_rounded` fallback. That path is itself
//! System B and is slated for removal (its hot consumers now read the table);
//! this thin shim stays only until `const_rounded` is deleted.

// The build script emits the per-work-width digit-string references
// (`PI_W8_DIGITS`, …) consumed by the `*_w_ref` selectors below.
include!(concat!(env!("OUT_DIR"), "/wide_consts.rs"));

// ─── Per-work-width references for the working-scale helpers ──────────
//
// Each selector maps a work integer's u64-limb count to its `(digit string,
// stored scale, top-scale residual-vs-half hint)` triple. The digits are
// floor-truncated at the stored scale; the hint lets the helper round
// correctly at that scale. (Legacy System-B path; see the module doc.)

/// `(π digits, stored scale, top-scale residual hint)` for a work
/// integer of `limbs` u64 limbs.
pub(crate) const fn pi_w_ref(limbs: u32) -> (&'static str, u32, ::core::cmp::Ordering) {
    match limbs {
        8 => (PI_W8_DIGITS, PI_W8_SCALE, PI_W8_TOP_CMP_HALF),
        16 => (PI_W16_DIGITS, PI_W16_SCALE, PI_W16_TOP_CMP_HALF),
        32 => (PI_W32_DIGITS, PI_W32_SCALE, PI_W32_TOP_CMP_HALF),
        48 => (PI_W48_DIGITS, PI_W48_SCALE, PI_W48_TOP_CMP_HALF),
        64 => (PI_W64_DIGITS, PI_W64_SCALE, PI_W64_TOP_CMP_HALF),
        128 => (PI_W128_DIGITS, PI_W128_SCALE, PI_W128_TOP_CMP_HALF),
        192 => (PI_W192_DIGITS, PI_W192_SCALE, PI_W192_TOP_CMP_HALF),
        256 => (PI_W256_DIGITS, PI_W256_SCALE, PI_W256_TOP_CMP_HALF),
        _ => panic!("consts_wide: no pi reference for this work-integer width"),
    }
}

/// `(ln 2 digits, stored scale, top-scale residual hint)` for a work
/// integer of `limbs` u64 limbs.
pub(crate) const fn ln2_w_ref(limbs: u32) -> (&'static str, u32, ::core::cmp::Ordering) {
    match limbs {
        8 => (LN2_W8_DIGITS, LN2_W8_SCALE, LN2_W8_TOP_CMP_HALF),
        16 => (LN2_W16_DIGITS, LN2_W16_SCALE, LN2_W16_TOP_CMP_HALF),
        32 => (LN2_W32_DIGITS, LN2_W32_SCALE, LN2_W32_TOP_CMP_HALF),
        48 => (LN2_W48_DIGITS, LN2_W48_SCALE, LN2_W48_TOP_CMP_HALF),
        64 => (LN2_W64_DIGITS, LN2_W64_SCALE, LN2_W64_TOP_CMP_HALF),
        128 => (LN2_W128_DIGITS, LN2_W128_SCALE, LN2_W128_TOP_CMP_HALF),
        192 => (LN2_W192_DIGITS, LN2_W192_SCALE, LN2_W192_TOP_CMP_HALF),
        256 => (LN2_W256_DIGITS, LN2_W256_SCALE, LN2_W256_TOP_CMP_HALF),
        _ => panic!("consts_wide: no ln2 reference for this work-integer width"),
    }
}

/// `(ln 10 digits, stored scale, top-scale residual hint)` for a work
/// integer of `limbs` u64 limbs.
pub(crate) const fn ln10_w_ref(limbs: u32) -> (&'static str, u32, ::core::cmp::Ordering) {
    match limbs {
        8 => (LN10_W8_DIGITS, LN10_W8_SCALE, LN10_W8_TOP_CMP_HALF),
        16 => (LN10_W16_DIGITS, LN10_W16_SCALE, LN10_W16_TOP_CMP_HALF),
        32 => (LN10_W32_DIGITS, LN10_W32_SCALE, LN10_W32_TOP_CMP_HALF),
        48 => (LN10_W48_DIGITS, LN10_W48_SCALE, LN10_W48_TOP_CMP_HALF),
        64 => (LN10_W64_DIGITS, LN10_W64_SCALE, LN10_W64_TOP_CMP_HALF),
        128 => (LN10_W128_DIGITS, LN10_W128_SCALE, LN10_W128_TOP_CMP_HALF),
        192 => (LN10_W192_DIGITS, LN10_W192_SCALE, LN10_W192_TOP_CMP_HALF),
        256 => (LN10_W256_DIGITS, LN10_W256_SCALE, LN10_W256_TOP_CMP_HALF),
        _ => panic!("consts_wide: no ln10 reference for this work-integer width"),
    }
}
