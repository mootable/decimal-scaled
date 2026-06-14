"""Generate the baked binary Tang `atan(c_j)` lookup table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_sincos_tang_table.py` / `gen_ln_tang_table.py`). It is NOT run at
build time: it emits a committed Rust source file
`src/algos/support/atan_tang_table.rs`, and that output is what the crate
compiles. `build.rs` is untouched.

## What it stores

The Tang `atan` kernel (`algos::trig::atan_tang_3limb_s44_56`) uses a
Tang-style argument-addition reduction with `M = ATAN_TANG_M = 512`:
after picking the nearest table entry `j = round(x·M)` it needs
`atan(c_j)` for `c_j = j/M` (`j ∈ [0, M]`) at the working scale. The
previous accessor recomputed that value per call by running the generic
`atan_fixed` halving-chain + Taylor at the working scale — the dominant
cost of the kernel (~74% of total at D57<56>). This table bakes the
value ONCE (oracle, mpmath) so the per-call cost collapses to one
multiply + one shift.

## Storage — binary fixed-point, MS-limb-first

For `j ∈ [1, M]` the value `c_j = j/M ∈ (0, 1]`, so

    atan(c_j) ∈ (0, atan(1) = π/4 ≈ 0.7854]   ⊂ [0, 1)

— a pure fraction strictly < 1. We store each as the correctly-rounded
binary fixed-point

    slot = round(value · 2^B)          (an unsigned B-bit integer)

as a fixed-length little-endian `[u64; LIMBS]` array, emitted
**most-significant limb first** within the entry so a narrower tier can
read a contiguous HIGH-limb PREFIX (a free slice) and the widest tier
reads the whole entry.

The `j = 0` slot is the special case `atan(0) = 0`. The accessor
SHORT-CIRCUITS `idx = 0` to `W::ZERO` (exactly as the `ln`/`sincos`
pilots short-circuit `idx = 0`). The stored `j = 0` slot is therefore
all-zero and never read.

`B` is sized for the WIDEST enabled tier's maximum working scale
(D1232: working scale capped at `W::BITS/8 = 2048` decimal digits) plus
generous guard bits — identical to the `ln`/`sincos` pilots
(`*_TANG_B = 7168`), so the slice→convert reconstruction is correctly
rounded at every tier. (The atan Tang kernel itself currently fires only
at D57<44..=56>, where `W = Int<16>`; sizing B for the widest tier keeps
the asset reusable should atan-Tang widen later, at no narrow-tier cost —
the accessor reads only the high-limb PREFIX each tier needs.)

## How the accessor consumes it

At call time, for working scale `w`, the accessor reconstructs the value

    round(value · 10^w) ≈ round(slot · 10^w / 2^B)
                        = (slot · 10^w + 2^(B-1)) >> B

in the tier's work integer `W`. One multiply + one add + one shift —
far less than the halving-chain Series it replaces.
"""

from __future__ import annotations

import datetime
from mpmath import mp, mpf, atan, floor as mpfloor

# ── Tang table size (do NOT change here; matches the const `M` in the
# atan Tang kernel `atan_tang_3limb_s44_56`, currently 512). ──────────────
M = 512

# ── Binary precision of the baked entry ──────────────────────────────────
#
# Sized IDENTICALLY to the `ln`/`sincos` pilots (`*_TANG_B = 7168`): the
# widest enabled tier is D1232 (work integer W = Int<256> = 16384 bits);
# the wide directed/nearest narrowing caps the working scale `w` at
# `W::BITS / 8 = 2048` DECIMAL digits — the deepest precision any call can
# demand.
#
#   need_bits = w_max · log2(10) = 2048 · 3.32193 ≈ 6803 bits
#
# We add generous guard bits and round B up to a u64-limb multiple so the
# converted slot is correctly rounded at every working scale, and so the
# conversion product `slot_hi · 10^w` (≈ B + 6803 bits) fits inside W
# (16384 bits) with comfortable headroom.
W_MAX_DECIMAL = 2048          # D1232 working-scale cap = Int<256>::BITS / 8
B_LIMBS = 112                 # 112 · 64 = 7168 bits
B = B_LIMBS * 64              # = 7168; guard ≈ 7168 − 6803 = 365 bits ≈ 110 dec digits

# ── Oracle precision ──────────────────────────────────────────────────────
# B bits ≈ B/3.32193 ≈ 2158 decimal digits of value; add wide margin so
# round(value · 2^B) is exact.
mp.dps = 2600


def round_to_b_limbs(val):
    """Return the B-bit `round(val · 2^B)` of a value in [0, 1) as a
    fixed-length `[u64; B_LIMBS]` little-endian magnitude, emitted
    MOST-SIGNIFICANT limb first (so a narrow tier reads a high-limb
    prefix)."""
    scaled = val * (mpf(2) ** B)
    # round-half-up via floor(scaled + 1/2). The values are all < 2^B
    # (since they are < 1), so n fits in B bits.
    n = int(mpfloor(scaled + mpf("0.5")))
    assert 0 <= n < (1 << B), f"slot value {val} out of B-bit range (n={n})"
    le = []
    x = n
    for _ in range(B_LIMBS):
        le.append(x & 0xFFFFFFFFFFFFFFFF)
        x >>= 64
    assert x == 0, "slot exceeds B_LIMBS limbs"
    return list(reversed(le))


def slot_msb_first(j: int):
    """Return the `[u64; B_LIMBS]` MS-limb-first magnitude for
    `atan(c_j)`, `c_j = j/M`. `j = 0` is stored all-zero (the accessor
    short-circuits `atan 0 = 0`)."""
    if j == 0:
        return [0] * B_LIMBS
    c_j = mpf(j) / mpf(M)                       # in (0, 1]
    a = atan(c_j)                               # in (0, π/4]  ⊂ [0, 1)
    assert 0 < a < 1, f"atan(c_{j}) = {a} not in (0, 1)"
    return round_to_b_limbs(a)


def emit_slot_array(w, limbs):
    w("        [")
    for k in range(0, B_LIMBS, 4):
        chunk = limbs[k:k + 4]
        chunk_str = ", ".join(f"0x{l:016x}" for l in chunk)
        w(f"            {chunk_str},")
    w("        ],")


def main():
    out = []
    w = out.append

    w("// SPDX-FileCopyrightText: 2026 John Moxley")
    w("// SPDX-License-Identifier: MIT OR Apache-2.0")
    w("")
    w(f"//! Baked binary Tang `atan(c_j)` lookup table (`M = {M}`).")
    w("//!")
    w("//! GENERATED by `scripts/gen_atan_tang_table.py` (mpmath oracle).")
    w("//! Do NOT edit by hand; re-run the script and commit its output.")
    w("//! This file is NOT produced at build time — `build.rs` is")
    w("//! untouched.")
    w("//!")
    w(f"//! Each of the `M + 1 = {M + 1}` slots holds `atan(c_j)` for")
    w(f"//! `c_j = j/M` (`j ∈ [0, {M}]`). For `j ≥ 1`, `c_j ∈ (0, 1]`, so")
    w("//! `atan(c_j) ∈ (0, π/4] ≈ (0, 0.7854]` — a pure fraction in")
    w("//! `[0, 1)`. Each value is stored as a correctly-rounded BINARY")
    w(f"//! fixed-point `round(· 2^{B})` — a `B = {B}`-bit unsigned magnitude")
    w(f"//! stored as a fixed-length `[u64; {B_LIMBS}]` little-endian array, but")
    w("//! laid out **most-significant limb first** within the entry. A")
    w("//! narrower tier reads a contiguous HIGH-limb PREFIX (a free")
    w("//! slice); the widest tier (D1232) reads the whole entry.")
    w("//!")
    w("//! The `j = 0` slot `atan(0) = 0` is all-zero; the accessor")
    w("//! SHORT-CIRCUITS `idx = 0` to `W::ZERO` (the stored slot is never")
    w("//! read).")
    w("//!")
    w(f"//! `B = {B}` is sized IDENTICALLY to the `ln`/`sincos` pilots")
    w("//! (`*_TANG_B`), for the widest enabled tier's max working scale")
    w(f"//! (D1232: `W = Int<256>`, working scale capped at `W::BITS/8 = {W_MAX_DECIMAL}`")
    w(f"//! decimal digits ≈ {W_MAX_DECIMAL * 332193 // 100000} bits) PLUS guard bits, so the")
    w("//! slice→convert reconstruction `round(slot · 10^w / 2^B)` is")
    w("//! correctly rounded at every tier and the conversion product")
    w("//! `slot · 10^w` fits `W`.")
    w("")
    w("/// Tang table size — `atan(c_j)` of `c_j = j/M`, `j ∈ [0, M]`.")
    w("/// Matches the const `M` in the atan Tang kernel")
    w("/// (`algos::trig::atan_tang_3limb_s44_56`).")
    w(f"pub(crate) const ATAN_TANG_M: u32 = {M};")
    w("")
    w("/// Binary fixed-point exponent: each value is `round(value ·")
    w(f"/// 2^B)`. `B = {B}` bits = `{B_LIMBS}` u64 limbs.")
    w(f"pub(crate) const ATAN_TANG_B: u32 = {B};")
    w("")
    w("/// Number of u64 limbs per stored value (`B / 64`).")
    w(f"pub(crate) const ATAN_TANG_LIMBS: usize = {B_LIMBS};")
    w("")
    w("/// The `M + 1` baked `atan(c_j)` slots `round(atan(c_j) · 2^B)`,")
    w(f"/// each a `[u64; {B_LIMBS}]` little-endian magnitude emitted")
    w("/// MOST-SIGNIFICANT limb FIRST. Index by `j ∈ [0, M]`; `j = 0` is")
    w("/// all-zero (the accessor short-circuits `atan 0 = 0`).")
    w(f"pub(crate) static ATAN_TANG_SLOTS: [[u64; {B_LIMBS}]; {M + 1}] = [")
    slots = [slot_msb_first(j) for j in range(M + 1)]
    for j in range(M + 1):
        w(f"    // j = {j}: atan({j}/{M})")
        emit_slot_array(w, slots[j])
    w("];")
    w("")

    # ── Width-generic accessor ────────────────────────────────────────────
    w("use crate::int::types::traits::BigInt;")
    w("")
    w("/// Reconstruct one baked binary fixed-point slot at working scale")
    w("/// `w` into the tier's work integer `W`. `slot` is the MS-limb-first")
    w("/// `round(value · 2^B)` (`B = ATAN_TANG_B`); we SLICE the high-order")
    w("/// `p` limbs needed for this scale and reconstruct")
    w("///")
    w("/// ```text")
    w("/// round(value · 10^w) = round(slot_hi · 10^w / 2^bp)")
    w("///                     = (slot_hi · 10^w + 2^(bp−1)) >> bp")
    w("/// ```")
    w("///")
    w("/// where `bp = 64·p`. The MS-limb-first layout makes the slice a")
    w("/// contiguous high-limb PREFIX. `pow10_w` is `10^w` in `W`, supplied")
    w("/// by the caller from the kernel's baked `pow10_table` (a lookup, not")
    w("/// a per-call recompute).")
    w("#[inline]")
    w("fn reconstruct<W: BigInt>(slot: &[u64; ATAN_TANG_LIMBS], w: u32, pow10_w: W) -> W {")
    w("    // Binary precision needed: w·log2(10) value bits + guard. Use the")
    w("    // rational 3322/1000 ≈ log2(10) (a slight over-estimate) and a")
    w("    // 64-bit (one-limb) guard so the converted slot rounds correctly")
    w("    // yet the conversion product `slot_hi · 10^w` stays inside `W`")
    w("    // even on the narrowest work integer (same budget as the `ln`")
    w("    // pilot). Round the limb count up; assert it fits the stored width.")
    w("    let need_bits = (w as u64) * 3322 / 1000 + 64;")
    w("    let p_full = need_bits.div_ceil(64) as usize;")
    w("    assert!(")
    w("        p_full <= ATAN_TANG_LIMBS,")
    w("        \"atan_tang: working scale {w} out of generated range ({ATAN_TANG_LIMBS} limbs)\"")
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
    w("    let scaled = slot_hi * pow10_w;")
    w("    // Round-half-up: add 2^(bp−1), then shift right by bp.")
    w("    let bias = W::ONE << (bp - 1);")
    w("    (scaled + bias) >> bp")
    w("}")
    w("")
    w("/// `atan(c_j)` with `c_j = idx/m` reconstructed at working scale `w`")
    w("/// (`idx ∈ [0, m]`, `m = ATAN_TANG_M`) in the tier's work integer")
    w("/// `W` (a value `x` held as `x · 10^w`). Replaces the per-call")
    w("/// `atan_fixed` halving-chain Series recompute.")
    w("///")
    w("/// `idx = 0` short-circuits to `W::ZERO` = `atan 0`. For `idx ≥ 1`")
    w("/// `atan(c_j)` is a pure fraction `< 1` read from the baked table.")
    w("/// `m` MUST equal `ATAN_TANG_M` (debug-asserted).")
    w("///")
    w("/// `pow10_w` is `10^w` in `W`, supplied by the caller from the")
    w("/// kernel's baked `pow10_table` (a lookup, not a `W::TEN.pow(w)`")
    w("/// per-call recompute).")
    w("#[inline]")
    w("pub(crate) fn atan_table_entry_baked<W: BigInt>(")
    w("    w: u32,")
    w("    idx: usize,")
    w("    m: u32,")
    w("    pow10_w: W,")
    w(") -> W {")
    w("    debug_assert!(")
    w("        m == ATAN_TANG_M,")
    w("        \"atan_table_entry_baked: m must equal ATAN_TANG_M\"")
    w("    );")
    w("    let _ = m;")
    w("    if idx == 0 {")
    w("        // atan(0) = 0 at scale w.")
    w("        return W::ZERO;")
    w("    }")
    w("    reconstruct::<W>(&ATAN_TANG_SLOTS[idx], w, pow10_w)")
    w("}")
    w("")

    src = "\n".join(out) + "\n"
    path = "src/algos/support/atan_tang_table.rs"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    print(f"wrote {path} ({len(src)} bytes), B={B} ({B_LIMBS} limbs), "
          f"mp.dps={mp.dps}, M={M}, {datetime.date.today()}")


if __name__ == "__main__":
    main()
