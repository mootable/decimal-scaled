// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The ONE slot→working-scale reconstruction shared by both baked Tang
//! `ln` tables, plus the marker types that name which table a kernel
//! reads.
//!
//! ## Why there are two tables and one body
//!
//! The Tang `ln` slots are stored MOST-SIGNIFICANT-limb first precisely so
//! that a narrower tier reads a contiguous HIGH-limb PREFIX of the same
//! value (see `ln_tang_table`'s header). The wide table keeps the full
//! `B = 7168`-bit entry that `D1232`'s working scale demands; the narrow
//! table (`ln_tang_table_narrow`) keeps the top `8` limbs — the exact
//! TRUNCATION of the same oracle value, emitted by the same generator run.
//!
//! Two tables exist for one reason only: **binary size**. The wide table is
//! `129 · 112 · 8 B ≈ 115 KB` and is `_wide-support`-gated, so a narrow
//! (`D18`/`D38`-only, `no_std`) build must not carry it; the narrow prefix
//! is `129 · 8 · 8 B ≈ 8 KB`. They are not two data sets — the narrow one
//! is a prefix of the wide one, which is a property this module's tests
//! assert directly wherever both are compiled.
//!
//! Because a prefix read of the narrow table and a prefix read of the wide
//! table return the SAME limbs for any `p <= 8`, the reconstruction body
//! below is table-independent: it takes the slot as a `&[u64]` slice and
//! asserts the requested precision fits it. One body, two data sources —
//! not one algorithm per width (Constitution rule 2).
//!
//! ## Which table a kernel reads
//!
//! [`LnTangTable`] is a compile-time selector, not a runtime one: the
//! kernel takes it as a type parameter, so `T::entry::<W>` resolves to a
//! direct call and the unchosen table is never referenced. It is a trait
//! rather than a passed-in closure because the kernel needs the accessor at
//! TWO different work widths (the rung width and the fall-up width), and a
//! closure is monomorphic in its return type while a generic trait method
//! serves both from one parameter.

use crate::int::types::traits::BigInt;

/// Which baked Tang `ln` table a kernel reads — a zero-sized compile-time
/// selector (see the module header). Implemented by [`NarrowSlots`] and,
/// where the wide table is compiled, [`WideSlots`].
pub(crate) trait LnTangTable {
    /// `ln(1 + idx/M)` reconstructed at working scale `w` in the work
    /// integer `W`, held as a value scaled by `10^w`. `pow10_w` is `10^w`
    /// in `W`, supplied by the caller (a lookup, never a recompute).
    fn entry<W: BigInt>(w: u32, idx: usize, pow10_w: W) -> W;
}

/// Reconstruct `round(L_idx · 10^w)` in `W` from a stored slot.
///
/// The slot is `round(L_idx · 2^B)` laid out MS-limb first, so the high
/// `p` limbs are the binary fixed-point of `L_idx` at exponent
/// `bp = 64·p`:
///
/// ```text
/// slot_hi = floor(slot / 2^(B − 64·p))
/// round(L_idx · 10^w) = (slot_hi · 10^w + 2^(bp−1)) >> bp
/// ```
///
/// One zero-extend, one multiply, one add, one shift — the whole point of
/// baking the table, replacing a per-call `ln_fixed` Series recompute.
///
/// `p` is chosen so `bp` carries the working scale's value bits
/// (`w · log2(10)`, via the rational `3322/1000` over-estimate) plus a
/// one-limb guard, and so the product `slot_hi · 10^w` still fits `W`.
///
/// The caller short-circuits `idx == 0` (`ln 1 = 0`); this body does not
/// need to, because slot 0 is all-zero and would reconstruct to zero
/// anyway — the short-circuit is a cost saving, not a correctness one.
///
/// # Panics
///
/// If the working scale demands more limbs of precision than `slot`
/// carries. That is a validity wall, not a safety net: a silently
/// truncated slot yields wrong digits, so the assert must stay. It is what
/// bounds the narrow table's `8` limbs against the narrow rung's reach.
#[inline]
pub(crate) fn ln_table_entry_from_slot<W: BigInt>(w: u32, slot: &[u64], pow10_w: W) -> W {
    // Binary precision needed: `w · log2(10)` value bits + a 64-bit
    // (one-limb) guard so the converted slot rounds correctly.
    let need_bits = (w as u64) * 3322 / 1000 + 64;
    let p_full = need_bits.div_ceil(64) as usize;
    assert!(
        p_full <= slot.len(),
        "ln_tang: working scale {} needs {} slot limbs, table holds {}",
        w,
        p_full,
        slot.len()
    );
    let p = p_full.max(1);
    // Zero-extend the top `p` limbs (MS-first) into W:
    //   slot_hi = sum_{k=0..p-1} slot[k] · 2^(64·(p−1−k)).
    let mut slot_hi = W::ZERO;
    for s in slot.iter().take(p) {
        slot_hi = (slot_hi << 64) | W::from_mag_sign_u128(&[*s as u128], false);
    }
    let bp = (64 * p) as u32;
    let scaled = slot_hi * pow10_w;
    // Round-half-up: add 2^(bp−1), then shift right by bp.
    let bias = W::ONE << (bp - 1);
    (scaled + bias) >> bp
}

/// The full-precision (`112`-limb) table — every wide tier reads this.
#[cfg(feature = "_wide-support")]
pub(crate) struct WideSlots;

#[cfg(feature = "_wide-support")]
impl LnTangTable for WideSlots {
    #[inline]
    fn entry<W: BigInt>(w: u32, idx: usize, pow10_w: W) -> W {
        super::ln_tang_table::ln_table_entry_baked::<W>(w, idx, pow10_w)
    }
}

/// The `8`-limb high-prefix table — the narrow tiers (`D18`/`D38`) read
/// this, in every build. See the module header for why it exists.
pub(crate) struct NarrowSlots;

impl LnTangTable for NarrowSlots {
    #[inline]
    fn entry<W: BigInt>(w: u32, idx: usize, pow10_w: W) -> W {
        super::ln_tang_table_narrow::ln_table_entry_narrow::<W>(w, idx, pow10_w)
    }
}

#[cfg(test)]
mod tests {
    //! The narrow table's defining property: it IS the wide table's
    //! high-limb prefix. Only assertable where both are compiled.

    #[test]
    #[cfg(feature = "_wide-support")]
    fn narrow_table_is_the_wide_table_prefix() {
        use crate::algos::support::ln_tang_table::{LN_TANG_M, LN_TANG_SLOTS};
        use crate::algos::support::ln_tang_table_narrow::{
            LN_TANG_NARROW_LIMBS, LN_TANG_SLOTS_NARROW,
        };
        assert_eq!(LN_TANG_SLOTS_NARROW.len(), LN_TANG_SLOTS.len());
        assert_eq!(LN_TANG_SLOTS_NARROW.len(), LN_TANG_M as usize + 1);
        for (i, (narrow, wide)) in LN_TANG_SLOTS_NARROW
            .iter()
            .zip(LN_TANG_SLOTS.iter())
            .enumerate()
        {
            assert_eq!(
                &narrow[..],
                &wide[..LN_TANG_NARROW_LIMBS],
                "narrow slot {i} is not the wide slot's high prefix"
            );
        }
    }

    /// Both tables must reconstruct the SAME value at any working scale
    /// the narrow table covers — the property that lets one kernel body
    /// serve both.
    #[test]
    #[cfg(feature = "_wide-support")]
    fn both_tables_reconstruct_identically_within_narrow_reach() {
        use crate::algos::exp::exp_generic as eg;
        use crate::algos::support::ln_tang_slot::{LnTangTable, NarrowSlots, WideSlots};
        use crate::int::types::Int;
        // The narrow rung is `Int<12>`, whose escalation cap is
        // `BITS/8 = 96` decimal digits — the deepest `w` a narrow call
        // can demand.
        for w in [0u32, 1, 17, 37, 47, 68, 96] {
            let pow10 = eg::pow10::<Int<12>>(w);
            for idx in [0usize, 1, 17, 64, 127, 128] {
                assert_eq!(
                    NarrowSlots::entry::<Int<12>>(w, idx, pow10),
                    WideSlots::entry::<Int<12>>(w, idx, pow10),
                    "tables disagree at w={w} idx={idx}"
                );
            }
        }
    }
}
