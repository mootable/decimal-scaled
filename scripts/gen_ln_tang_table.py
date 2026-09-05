"""Generate the baked binary Tang `ln(1 + i/M)` lookup table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_const_table.py` / `gen_golden_precision.py`). It is NOT run at build
time: it emits TWO committed Rust source files, and that output is what
the crate compiles. `build.rs` is untouched.

    src/algos/support/ln_tang_table.rs         full B = 7168-bit slots
    src/algos/support/ln_tang_table_narrow.rs  the top 8 limbs of each

Both come from ONE pass over the oracle: the narrow table is literally the
high-limb prefix of the wide one (see `NARROW_B_LIMBS` below), so they
cannot drift apart, and a narrow-only build carries ~8 KB instead of the
wide table's ~115 KB. Registering only one of them in
`.github/workflows/generator-drift.yml` would leave the other unguarded —
both paths are listed there.

## What it stores

The wide `ln` kernel (`algos::ln::ln_tang`) uses Tang's table-driven
reduction with `M = LN_TANG_M = 128`: it indexes `L_i = ln(1 + i/M)` for
`i in 0..=M`. The previous accessor recomputed `L_i` per call by running
a full `ln_fixed` Series at the working scale — the dominant cost of the
wide-`ln` regression. This table bakes `L_i` ONCE (oracle: flint/Arb, with
rigorous interval bounds) so the per-call cost collapses to one multiply +
one shift.

## Storage — binary fixed-point, MS-limb-first

Every slot `L_i = ln(1 + i/M)` lies in `[0, ln 2] ⊂ [0, 1)`, so it is a
pure fraction. We store the correctly-rounded binary fixed-point

    slot_i = round(L_i · 2^B)          (an unsigned B-bit integer)

as a fixed-length little-endian `[u64; LIMBS]` array, but emitted
**most-significant limb first** within the entry so that a narrower tier
can read a contiguous HIGH-limb PREFIX (a free slice) and the widest tier
reads the whole entry. `B` is sized for the WIDEST enabled tier's maximum
working scale (D1232: working scale capped at `W::BITS/8 = 2048` decimal
digits) plus generous guard bits, so the slice→convert reconstruction is
correctly rounded at every tier.

    i = 0   -> L_0 = ln(1)   = 0      (stored as all-zero)
    i = M   -> L_M = ln(2)            (the largest slot, < 1)

## How the accessor consumes it

At call time, for working scale `w`, the accessor reconstructs

    round(L_i · 10^w) ≈ round(slot_i · 10^w / 2^B)
                      = (slot_i · 10^w + 2^(B-1)) >> B

in the tier's work integer `W`. One multiply + one add + one shift —
far less than the Series it replaces. The binary store is exact to
`2^-(B+1)`; scaled by `10^w` (w ≤ 2048) that absolute error is
`≤ 10^w · 2^-(B+1)`, and with `B` chosen so `2^B >> 2·10^w` the converted
slot is correctly rounded at scale `w` (the guard bits sit far below the
working ULP, and the artanh reconstruction re-rounds with GUARD digits +
Ziv on top).
"""

from __future__ import annotations

import datetime
from flint import arb, fmpq

from tang_flint_oracle import ORACLE_PREC_BITS, set_precision, slot_limbs

# ── Tang table size (do NOT change here; the M-sweep is a separate task) ─
M = 128

# ── Binary precision of the baked entry ──────────────────────────────────
#
# The widest enabled tier is D1232: work integer W = Int<256> = 16384
# bits. The wide-`ln` directed/nearest narrowing caps the working scale
# `w` at `W::BITS / 8` DECIMAL digits (see
# `round_to_storage_directed_impl`'s `cap_digits`), i.e. 2048 digits for
# D1232 — the deepest L_i precision any call can demand.
#
#   need_bits = w_max · log2(10) = 2048 · 3.32193 ≈ 6803 bits
#
# We add generous guard bits and round B up to a u64-limb multiple so the
# converted slot is correctly rounded at every working scale, and so the
# conversion arithmetic `slot · 10^w` (≈ B + 6803 bits) fits inside W
# (16384 bits) with comfortable headroom.
W_MAX_DECIMAL = 2048          # D1232 working-scale cap = Int<256>::BITS / 8
B_LIMBS = 112                 # 112 · 64 = 7168 bits
B = B_LIMBS * 64              # = 7168; guard ≈ 7168 − 6803 = 365 bits ≈ 110 dec digits

# ── The NARROW high-prefix table ─────────────────────────────────────────
#
# The narrow tiers (D18 / D38) reach `ln` through the same Tang kernel, but
# the wide table above is `129 · 112 · 8 B ≈ 115 KB` and is
# `_wide-support`-gated — a narrow-only (`no_std`) build must not carry it.
# Because slots are stored MS-limb first, the narrow table is simply the
# TRUNCATION of each wide slot to its top `NARROW_B_LIMBS` limbs, emitted
# from the SAME oracle values in the same run. That makes "the narrow table
# is the wide table's prefix" structural rather than a claim to re-verify,
# and the two reconstruct bit-identically at every working scale the narrow
# table covers.
#
# Sizing — the deepest `w` a narrow call can demand:
#
#   narrow work rung        Int<12>   (see `policy::work_rung`; Int<24> is
#                                      unusable narrow — `resize_to` stages
#                                      `ceil(N/2)` u128 limbs against
#                                      `MAX_U128_LIMB = 4·MAX_WORK_N = 8`)
#   escalation cap          BITS/8 = 12·64/8 = 96 decimal digits
#   accessor need_bits      96 · 3322/1000 + 64 = 382 bits
#   limbs required          ceil(382 / 64) = 6
#
# `NARROW_B_LIMBS = 8` therefore carries 2 limbs (128 bits ≈ 38 decimal
# digits) of headroom over the worst narrow cell, at ≈ 8 KB.
NARROW_B_LIMBS = 8            # 8 · 64 = 512 bits
NARROW_B = NARROW_B_LIMBS * 64

# ── Oracle precision ──────────────────────────────────────────────────────
# Set by the shared flint/Arb oracle (`scripts/tang_flint_oracle.py`) well
# above the B bits retained, so every emitted digit is pinned by a rigorous
# interval bound rather than assumed correct at some working precision.
set_precision()


def slot_limbs_msb_first(i: int):
    """Return the B-bit `round(ln(1+i/M) · 2^B)` as a fixed-length
    `[u64; B_LIMBS]` little-endian magnitude, emitted MOST-SIGNIFICANT
    limb first (so a narrow tier reads a high-limb prefix)."""
    if i == 0:
        # ln(1) = 0 exactly.
        return [0] * B_LIMBS
    # `1 + i/M` is an exact rational, so the only rounding is Arb's own —
    # and that is carried in the ball's radius. `L_i` lies in (0, ln2].
    # Passed as a thunk so the oracle can recompute it if it must escalate.
    return slot_limbs(lambda: arb(fmpq(M + i, M)).log(),
                      B, B_LIMBS, f"ln slot {i}")


def emit_slot_array(w, name, limbs_per_slot, slots):
    """Emit one `[[u64; limbs_per_slot]; M + 1]` static, 4 limbs per line.

    `slots` holds the FULL-width MS-first limbs; a narrower table is the
    leading `limbs_per_slot` of each, i.e. the high-limb prefix.
    """
    w(f"pub(crate) static {name}: [[u64; {limbs_per_slot}]; {M + 1}] = [")
    for i, limbs in enumerate(slots):
        w(f"    // i = {i}: ln(1 + {i}/{M})")
        w("    [")
        for j in range(0, limbs_per_slot, 4):
            chunk = limbs[j:j + 4]
            chunk_str = ", ".join(f"0x{l:016x}" for l in chunk)
            w(f"        {chunk_str},")
        w("    ],")
    w("];")
    w("")


def emit_wide(slots):
    out = []
    w = out.append

    w("// SPDX-FileCopyrightText: 2026 John Moxley")
    w("// SPDX-License-Identifier: MIT OR Apache-2.0")
    w("")
    w("//! Baked binary Tang `ln(1 + i/M)` lookup table (`M = 128`).")
    w("//!")
    w("//! GENERATED by `scripts/gen_ln_tang_table.py` (flint/Arb oracle —")
    w("//! every digit below is pinned by a rigorous interval bound). Do")
    w("//! NOT edit by hand; re-run the script and commit its output. This")
    w("//! file is NOT produced at build time — `build.rs` is untouched.")
    w("//!")
    w("//! Any change belongs in `scripts/gen_ln_tang_table.py`, never in")
    w("//! this file — including a new `match` arm when a `RoundingMode`")
    w("//! variant is added. Re-run the generator and commit both; a")
    w("//! hand-edit here is silently reverted the next time anyone")
    w("//! regenerates.")
    w("//!")
    w("//! Each of the `M + 1 = 129` slots holds the natural log")
    w("//! `L_i = ln(1 + i/128)` (`i ∈ [0, 128]`, all in `[0, ln 2] ⊂ [0,")
    w("//! 1)`) as a correctly-rounded BINARY fixed-point value")
    w(f"//! `round(L_i · 2^{B})` — a `B = {B}`-bit unsigned magnitude stored as a")
    w(f"//! fixed-length `[u64; {B_LIMBS}]` little-endian array, but laid out")
    w("//! **most-significant limb first** within the entry. A narrower")
    w("//! tier reads a contiguous HIGH-limb PREFIX (a free slice); the")
    w("//! widest tier (D1232) reads the whole entry. The `i = 0` slot is")
    w("//! `ln(1) = 0` (all-zero); the `i = 128` slot is `ln 2`.")
    w("//!")
    w(f"//! `B = {B}` is sized for the widest enabled tier's max working scale")
    w(f"//! (D1232: `W = Int<256>`, working scale capped at `W::BITS/8 =")
    w(f"//! {W_MAX_DECIMAL}` decimal digits ≈ {W_MAX_DECIMAL * 332193 // 100000} bits) PLUS guard bits, so")
    w("//! the slice→convert reconstruction `round(slot · 10^w / 2^B)` is")
    w("//! correctly rounded at every tier and the conversion product")
    w("//! `slot · 10^w` fits inside `W`.")
    w("")
    w("/// Tang table size — `ln(1 + i/M)`, `i ∈ [0, M]`. Matches")
    w("/// `LN_TANG_M` in `macros::wide_transcendental`. The M-sweep is a")
    w("/// separate task; do NOT change this here.")
    w(f"pub(crate) const LN_TANG_M: u32 = {M};")
    w("")
    w("/// Binary fixed-point exponent: each slot is `round(ln(1+i/M) ·")
    w(f"/// 2^B)`. `B = {B}` bits = `{B_LIMBS}` u64 limbs.")
    w(f"pub(crate) const LN_TANG_B: u32 = {B};")
    w("")
    w("/// Number of u64 limbs per stored slot (`B / 64`).")
    w(f"pub(crate) const LN_TANG_LIMBS: usize = {B_LIMBS};")
    w("")
    w("/// The `M + 1` baked slots `round(ln(1+i/M) · 2^B)`, each a")
    w(f"/// `[u64; {B_LIMBS}]` little-endian magnitude emitted MOST-SIGNIFICANT")
    w("/// limb FIRST (so a narrow tier reads a high-limb prefix). Index by")
    w("/// `i ∈ [0, 128]`.")
    emit_slot_array(w, "LN_TANG_SLOTS", B_LIMBS, slots)

    # ── The tier accessor — a thin wrapper over the ONE shared body ──────
    w("use crate::int::types::traits::BigInt;")
    w("")
    w("/// `ln(1 + idx/M)` reconstructed at working scale `w` (`idx ∈ [0,")
    w("/// M]`) in the tier's work integer `W` (a value `x` held as `x ·")
    w("/// 10^w`). Replaces the per-call `ln_fixed` Series recompute.")
    w("///")
    w("/// The reconstruction itself lives ONCE, in")
    w("/// [`crate::algos::support::ln_tang_slot::ln_table_entry_from_slot`],")
    w("/// shared with the narrow high-prefix table")
    w("/// ([`crate::algos::support::ln_tang_table_narrow`]) — this wrapper")
    w("/// only names which slot to read. `idx = 0` short-circuits to `0`")
    w("/// (`ln 1`); the MS-limb-first layout is what makes the shared body's")
    w("/// slice a contiguous high-limb PREFIX.")
    w("#[inline]")
    w("pub(crate) fn ln_table_entry_baked<W: BigInt>(w: u32, idx: usize, pow10_w: W) -> W {")
    w("    if idx == 0 {")
    w("        return W::ZERO;")
    w("    }")
    w("    crate::algos::support::ln_tang_slot::ln_table_entry_from_slot::<W>(")
    w("        w,")
    w("        &LN_TANG_SLOTS[idx],")
    w("        pow10_w,")
    w("    )")
    w("}")
    w("")

    return "\n".join(out) + "\n"


def emit_narrow(slots):
    """The narrow high-prefix table — the top `NARROW_B_LIMBS` limbs of the
    SAME slots, so it is the wide table's prefix by construction."""
    out = []
    w = out.append

    w("// SPDX-FileCopyrightText: 2026 John Moxley")
    w("// SPDX-License-Identifier: MIT OR Apache-2.0")
    w("")
    w("//! Baked binary Tang `ln(1 + i/M)` lookup table — the NARROW high")
    w("//! prefix (`M = 128`).")
    w("//!")
    w("//! GENERATED by `scripts/gen_ln_tang_table.py` (flint/Arb oracle —")
    w("//! every digit below is pinned by a rigorous interval bound). Do")
    w("//! NOT edit by hand; re-run the script and commit its output. This")
    w("//! file is NOT produced at build time — `build.rs` is untouched.")
    w("//!")
    w("//! Any change belongs in `scripts/gen_ln_tang_table.py`, never in")
    w("//! this file. Re-run the generator and commit both; a hand-edit here")
    w("//! is silently reverted the next time anyone regenerates.")
    w("//!")
    w(f"//! Each of the `M + 1 = {M + 1}` slots is the TRUNCATION of the matching")
    w(f"//! [`crate::algos::support::ln_tang_table`] slot to its top")
    w(f"//! `{NARROW_B_LIMBS}` limbs. Both are emitted from the same oracle values in the")
    w("//! same generator run, so this table **is** the wide table's high-limb")
    w("//! prefix by construction — not a second data set to keep in step.")
    w("//! `ln_tang_slot`'s tests assert that identity wherever both compile.")
    w("//!")
    w("//! ## Why it exists")
    w("//!")
    w(f"//! Binary size, and nothing else. The wide table is `{M + 1} · {B_LIMBS} · 8 B`")
    w(f"//! ≈ 115 KB and is `_wide-support`-gated, so a narrow (`D18`/`D38`-only,")
    w(f"//! `no_std`) build must not carry it; this one is ≈ {(M + 1) * NARROW_B_LIMBS * 8 // 1024} KB.")
    w("//!")
    w(f"//! `B = {NARROW_B}` bits is sized against the narrow work rung `Int<12>`,")
    w("//! whose Ziv-escalation cap is `BITS/8 = 96` decimal digits — the")
    w("//! deepest working scale a narrow call can demand. The accessor needs")
    w("//! `ceil((96 · 3322/1000 + 64) / 64) = 6` limbs there, so the stored")
    w(f"//! `{NARROW_B_LIMBS}` carry two limbs (≈ 38 decimal digits) of headroom.")
    w("")
    w("/// Number of u64 limbs per stored narrow slot.")
    w(f"pub(crate) const LN_TANG_NARROW_LIMBS: usize = {NARROW_B_LIMBS};")
    w("")
    w("/// Binary fixed-point exponent of a narrow slot — the wide entry's")
    w(f"/// `round(ln(1+i/M) · 2^{B})` truncated to `2^{NARROW_B}`.")
    w(f"pub(crate) const LN_TANG_NARROW_B: u32 = {NARROW_B};")
    w("")
    w(f"/// The `M + 1` narrow slots, each the top `{NARROW_B_LIMBS}` limbs of the wide")
    w("/// entry, MOST-SIGNIFICANT limb first. Index by `i ∈ [0, 128]`. The")
    w("/// `i = 0` slot is `ln(1) = 0`; the `i = 128` slot is `ln 2`.")
    emit_slot_array(w, "LN_TANG_SLOTS_NARROW", NARROW_B_LIMBS,
                    [s[:NARROW_B_LIMBS] for s in slots])
    w("use crate::int::types::traits::BigInt;")
    w("")
    w("/// `ln(1 + idx/M)` at working scale `w`, read from the narrow prefix")
    w("/// table into the work integer `W`.")
    w("///")
    w("/// The reconstruction lives ONCE, in")
    w("/// [`crate::algos::support::ln_tang_slot::ln_table_entry_from_slot`];")
    w("/// this wrapper only names which slot to read. `idx = 0` is `ln 1 = 0`.")
    w("///")
    w("/// # Panics")
    w("///")
    w("/// If `w` demands more than [`LN_TANG_NARROW_LIMBS`] limbs of slot —")
    w("/// the validity wall bounding this table against the narrow rung's")
    w("/// reach. A silently truncated slot would yield wrong digits.")
    w("#[inline]")
    w("pub(crate) fn ln_table_entry_narrow<W: BigInt>(w: u32, idx: usize, pow10_w: W) -> W {")
    w("    if idx == 0 {")
    w("        return W::ZERO;")
    w("    }")
    w("    crate::algos::support::ln_tang_slot::ln_table_entry_from_slot::<W>(")
    w("        w,")
    w("        &LN_TANG_SLOTS_NARROW[idx],")
    w("        pow10_w,")
    w("    )")
    w("}")
    w("")

    return "\n".join(out) + "\n"


def main():
    # Every slot is computed ONCE at full width; the narrow table is the
    # high-limb prefix of those same values, so the two cannot drift apart.
    slots = [slot_limbs_msb_first(i) for i in range(M + 1)]

    for src, path in (
        (emit_wide(slots), "src/algos/support/ln_tang_table.rs"),
        (emit_narrow(slots), "src/algos/support/ln_tang_table_narrow.rs"),
    ):
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            f.write(src)
        print(f"wrote {path} ({len(src)} chars)")
    print(f"B={B} ({B_LIMBS} limbs), narrow B={NARROW_B} "
          f"({NARROW_B_LIMBS} limbs), arb prec={ORACLE_PREC_BITS} bits, "
          f"M={M}, {datetime.date.today()}")


if __name__ == "__main__":
    main()
