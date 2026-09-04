"""Generate the baked binary Tang `ln(1 + i/M)` lookup table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_const_table.py` / `gen_golden_precision.py`). It is NOT run at build
time: it emits a committed Rust source file
`src/algos/support/ln_tang_table.rs`, and that output is what the crate
compiles. `build.rs` is untouched.

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

# ── Narrow-tier prefix width ─────────────────────────────────────────────
#
# A build without `_wide-support` has only D18/D38, and 112 limbs (115,584
# bytes) would dwarf what those tiers can consume. They need just the
# HIGH-limb PREFIX their working scale demands, and the emitted array is
# MS-limb-first precisely so a prefix IS the value at a lower exponent.
#
# Sized by the SAME formula the reader applies at run time — kept in one
# place below as `ln_table_limbs_for`, so this and the Rust side cannot
# drift:
#
#   need_bits = w·3322/1000 + 64   ->   limbs = ceil(need_bits / 64)
#
# The widest `w` any NARROW caller presents is SCALE 38 + STRICT_GUARD 30
# = 68: the hyperbolics (`acosh`/`asinh`/`atanh`) call `ln_fixed` at that
# working scale. The ln family itself runs at SCALE + LN_GUARD = 48, which
# needs only 4 limbs; 5 covers the hyperbolics too, so routing one of them
# through Tang later cannot trip the width bound.
#
#   w = 68 -> 225 + 64 = 289 bits -> 5 limbs (320 bits, 31 bits of slack)
#
# Measured headroom: 5 limbs actually cover every w <= 77, nine scales past
# the widest a narrow caller presents. w = 78 is the first that would need a
# sixth limb.
#
# The narrow slot is emitted as `limbs[:NARROW_B_LIMBS]` — literally the
# first limbs of the SAME computed value, not a second computation at a
# lower precision. So the narrow array is a prefix of the wide one by
# construction, and a narrow build and a wide build return bit-identical
# results for every `(w, idx)` they both accept.
NARROW_W_MAX_DECIMAL = 68
NARROW_B_LIMBS = ((NARROW_W_MAX_DECIMAL * 3322 // 1000 + 64) + 63) // 64   # = 5
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


def main():
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
    w(f"//! fixed-length `[u64; LN_TANG_LIMBS]` little-endian array, but laid out")
    w("//! **most-significant limb first** within the entry. A narrower")
    w("//! tier reads a contiguous HIGH-limb PREFIX (a free slice); the")
    w("//! widest tier (D1232) reads the whole entry. The `i = 0` slot is")
    w("//!")
    w("//! That prefix property is also how ONE table serves two build")
    w(f"//! configurations: `_wide-support` compiles the full {B_LIMBS}-limb entry,")
    w(f"//! a narrow-only build compiles the leading {NARROW_B_LIMBS} limbs of the SAME")
    w("//! values ({} bytes against {}). Emitted from one computation in one".format(
        (M + 1) * NARROW_B_LIMBS * 8, (M + 1) * B_LIMBS * 8))
    w("//! pass, so the two widths cannot drift, and results are")
    w("//! bit-identical wherever both builds accept the working scale.")
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
    w("/// Number of u64 limbs per stored slot, and so the binary exponent")
    w("/// `B = 64 · LN_TANG_LIMBS` each slot is scaled by.")
    w("///")
    w("/// Two widths, ONE table. A `_wide-support` build carries the full")
    w(f"/// `{B_LIMBS}`-limb entry the widest tier (D1232) consumes; a narrow-only")
    w(f"/// build carries the `{NARROW_B_LIMBS}`-limb HIGH-limb PREFIX of that SAME value —")
    w(f"/// {(M + 1) * NARROW_B_LIMBS * 8} bytes against {(M + 1) * B_LIMBS * 8}. Because the narrow array is a")
    w("/// prefix rather than a second computation, and the reader always")
    w("/// takes the top `p` limbs, both builds return BIT-IDENTICAL results")
    w("/// for every `(w, idx)` they both accept: the feature flag changes how")
    w("/// much of the constant is compiled in, never its value.")
    w("#[cfg(feature = \"_wide-support\")]")
    w(f"pub(crate) const LN_TANG_LIMBS: usize = {B_LIMBS};")
    w("#[cfg(not(feature = \"_wide-support\"))]")
    w(f"pub(crate) const LN_TANG_LIMBS: usize = {NARROW_B_LIMBS};")
    w("")
    w("/// Binary fixed-point exponent: each slot is `round(ln(1+i/M) · 2^B)`,")
    w("/// truncated to the compiled width. Derived so it cannot disagree with")
    w("/// the array it describes.")
    w("pub(crate) const LN_TANG_B: u32 = (LN_TANG_LIMBS as u32) * 64;")
    w("")
    w("/// Limbs of `L_idx` a working scale `w` needs: `w·log2(10)` value bits")
    w("/// plus a 64-bit guard, rounded up to whole limbs. `3322/1000` is a")
    w("/// slight OVER-estimate of `log2(10)`, so the answer never under-sizes.")
    w("///")
    w("/// THE single source of this formula. [`ln_table_entry_baked`] applies")
    w("/// it at run time and [`ln_table_fits`] applies it at compile time, so")
    w("/// the two cannot drift apart.")
    w("#[inline]")
    w("#[must_use]")
    w("pub(crate) const fn ln_table_limbs_for(w: u32) -> usize {")
    w("    let need_bits = (w as u64) * 3322 / 1000 + 64;")
    w("    // `div_ceil` by hand: usable in `const` on every supported toolchain.")
    w("    ((need_bits + 63) / 64) as usize")
    w("}")
    w("")
    w("/// Whether the table COMPILED INTO THIS BUILD covers working scale `w`.")
    w("///")
    w("/// `const fn` on purpose. The runtime `assert!` in")
    w("/// [`ln_table_entry_baked`] is the last line of defence, not the first:")
    w("/// a caller whose working scale is const-foldable (`SCALE + GUARD`)")
    w("/// should assert THIS in a `const { }` block, turning \"this build's")
    w("/// table is too small for this scale\" into a compile error instead of")
    w("/// a production panic on the narrow tier.")
    w("#[inline]")
    w("#[must_use]")
    w("pub(crate) const fn ln_table_fits(w: u32) -> bool {")
    w("    ln_table_limbs_for(w) <= LN_TANG_LIMBS")
    w("}")
    w("")
    w("/// The `M + 1` baked slots `round(ln(1+i/M) · 2^B)`, each a")
    w(f"/// `[u64; {B_LIMBS}]` little-endian magnitude emitted MOST-SIGNIFICANT")
    w("/// limb FIRST (so a narrow tier reads a high-limb prefix). Index by")
    w("/// `i ∈ [0, 128]`.")
    # Compute every slot ONCE, then emit the full array and its prefix from
    # the same values — the property that makes the two widths incapable of
    # disagreeing.
    all_limbs = [slot_limbs_msb_first(i) for i in range(M + 1)]

    def emit_slots(width: int) -> None:
        w(f"pub(crate) static LN_TANG_SLOTS: [[u64; {width}]; {M + 1}] = [")
        for i in range(M + 1):
            # one slot per line group; chunk the limbs across lines for
            # readability (4 limbs per line).
            w(f"    // i = {i}: ln(1 + {i}/{M})")
            w("    [")
            for j in range(0, width, 4):
                # Clamp to `width`: `all_limbs[i]` always holds the full
                # B_LIMBS value, so the last chunk of a narrower emission
                # must not run past the declared array length.
                chunk = all_limbs[i][j:min(j + 4, width)]
                chunk_str = ", ".join(f"0x{l:016x}" for l in chunk)
                w(f"        {chunk_str},")
            w("    ],")
        w("];")
        w("")

    w("#[cfg(feature = \"_wide-support\")]")
    emit_slots(B_LIMBS)
    w("/// The narrow-build table: the leading `LN_TANG_LIMBS` limbs of each")
    w("/// entry above, sliced from the SAME computed value rather than")
    w("/// recomputed at a lower precision.")
    w("#[cfg(not(feature = \"_wide-support\"))]")
    emit_slots(NARROW_B_LIMBS)

    # ── Width-generic accessor ────────────────────────────────────────────
    w("use crate::int::types::traits::BigInt;")
    w("")
    w("/// `ln(1 + idx/M)` reconstructed at working scale `w` (`idx ∈ [0,")
    w("/// M]`) in the tier's work integer `W` (a value `x` held as `x ·")
    w("/// 10^w`). Replaces the per-call `ln_fixed` Series recompute.")
    w("///")
    w("/// The slot is stored as `slot = round(L_idx · 2^B)` (B = LN_TANG_B,")
    w("/// MS-limb first). We SLICE the high-order `p` limbs needed for this")
    w("/// working scale — `slot_hi = floor(slot / 2^(B − 64·p))`, the binary")
    w("/// fixed-point of `L_idx` at exponent `bp = 64·p` — then reconstruct")
    w("///")
    w("/// ```text")
    w("/// round(L_idx · 10^w) = round(slot_hi · 10^w / 2^bp)")
    w("///                     = (slot_hi · 10^w + 2^(bp−1)) >> bp")
    w("/// ```")
    w("///")
    w("/// entirely in `W`: one zero-extend, one multiply, one add, one")
    w("/// shift. `p` is chosen so `bp` carries the working scale's bits plus")
    w("/// generous guard, and so the product `slot_hi · 10^w` (≈ bp + w·")
    w("/// log2(10) bits ≈ 6.6·w bits, with w ≤ W::BITS/8) fits `W`.")
    w("///")
    w("/// `idx = 0` short-circuits to `0` (ln 1). The MS-limb-first layout")
    w("/// makes the slice a contiguous high-limb PREFIX: a narrow tier reads")
    w("/// fewer limbs, the widest tier reads the whole entry.")
    w("#[inline]")
    w("pub(crate) fn ln_table_entry_baked<W: BigInt>(w: u32, idx: usize, pow10_w: W) -> W {")
    w("    if idx == 0 {")
    w("        return W::ZERO;")
    w("    }")
    w("    let slot = &LN_TANG_SLOTS[idx];")
    w("    // Binary precision needed: w·log2(10) value bits + guard. Use the")
    w("    // rational 3322/1000 ≈ log2(10) (a slight over-estimate) and a")
    w("    // 64-bit (one-limb) guard so the converted slot rounds correctly")
    w("    // yet the conversion product `slot_hi · 10^w` (≈ bp + w·log2(10)")
    w("    // ≈ 2·w·log2(10) + 64 ≈ 0.83·W::BITS + 64 at the `w = W::BITS/8`")
    w("    // cap) stays inside `W` even on the narrowest work integer")
    w("    // (Int<16> = 1024 bits: 0.83·1024 + 64 ≈ 914 < 1024). Round the")
    w("    // limb count up; assert it fits the stored width.")
    w("    let p_full = ln_table_limbs_for(w);")
    w("    // LAST line of defence, not the first: a const-foldable working")
    w("    // scale should have been checked by `const { ln_table_fits(w) }` at")
    w("    // the call site, so reaching this assert means a caller with a")
    w("    // genuinely runtime `w` overran the width compiled into this build.")
    w("    assert!(")
    w("        p_full <= LN_TANG_LIMBS,")
    w("        \"ln_tang: working scale {w} out of generated range ({LN_TANG_LIMBS} limbs)\"")
    w("    );")
    w("    let p = p_full.max(1);")
    w("    // Zero-extend the top `p` limbs (MS-first) into W:")
    w("    //   slot_hi = sum_{k=0..p-1} slot[k] · 2^(64·(p−1−k)).")
    w("    let mut slot_hi = W::ZERO;")
    w("    for s in slot.iter().take(p) {")
    w("        slot_hi = (slot_hi << 64)")
    w("            | W::from_mag_sign_u128(&[*s as u128], false);")
    w("    }")
    w("    let bp = (64 * p) as u32;")
    w("    // `10^w` in `W` — supplied by the caller from the kernel's baked")
    w("    // `pow10_table` static (a lookup, not a per-call recompute).")
    w("    let scaled = slot_hi * pow10_w;")
    w("    // Round-half-up: add 2^(bp−1), then shift right by bp.")
    w("    let bias = W::ONE << (bp - 1);")
    w("    (scaled + bias) >> bp")
    w("}")
    w("")

    src = "\n".join(out) + "\n"
    path = "src/algos/support/ln_tang_table.rs"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    print(f"wrote {path} ({len(src)} chars), B={B} ({B_LIMBS} limbs), "
          f"arb prec={ORACLE_PREC_BITS} bits, M={M}, {datetime.date.today()}")


if __name__ == "__main__":
    main()
