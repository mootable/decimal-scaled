"""Generate the baked binary Tang `exp(j · ln2 / M)` lookup table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_ln_tang_table.py` / `gen_const_table.py`). It is NOT run at build
time: it emits a committed Rust source file
`src/algos/support/exp_tang_table.rs`, and that output is what the crate
compiles. `build.rs` is untouched.

## What it stores

The wide `exp` kernel (`algos::exp::exp_tang`) uses Tang's table-driven
range reduction: it indexes `E_j = exp(j · ln2 / M)` for `j in 0..M`
(`WideTrigCore::exp_table_entry(w, j, M)`). The previous accessor
recomputed `E_j` per call by running a full `exp_fixed` Series at the
working scale — the dominant cost of the wide-`exp` regression. This
table bakes `E_j` ONCE (oracle, mpmath) so the per-call cost collapses to
one multiply + one shift + one add.

## One M table covers both M = 128 and M = 512

The shipped tiers use `M = 128` (D57) or `M = 512` (every other wide
tier). The `M = 128` lattice is a SUBSET of the `M = 512` lattice:

    exp(j · ln2 / 128) = exp((4j) · ln2 / 512)

so a single `M = 512` table answers both — for `M = 128` the accessor
reads table index `4 · j`. We therefore bake only the `M = 512` table.

## Storage — fractional part, binary fixed-point, MS-limb-first

Every slot `E_j = exp(j · ln2 / 512)` lies in `[1, 2)` for `j in 0..512`
(`E_0 = 1`, `E_511 < 2`). Storing the integer part would cost an extra
integer limb AND add one bit to the reconstructed product `slot · 10^w`,
risking overflow of the narrowest work integer at the directed-narrow
cap. Instead we store the FRACTIONAL part

    F_j = E_j - 1  in  [0, 1)          (a pure fraction, exactly like ln)
    slot_j = round(F_j · 2^B)          (an unsigned B-bit integer)

as a fixed-length little-endian `[u64; LIMBS]` array, emitted
**most-significant limb first** within the entry (so a narrower tier can
read a contiguous HIGH-limb PREFIX and the widest tier reads the whole
entry). The integer `1` is re-added at convert time (`+ 10^w`). This
makes the storage and accessor BIT-IDENTICAL in shape to the `ln` pilot.

    j = 0   -> E_0 = exp(0)   = 1   -> F_0 = 0          (stored all-zero)
    j = 511 -> E_511 < 2            -> F_511 < 1

`B` matches the `ln` pilot (`B_LIMBS = 112`, `B = 7168` fractional bits),
sized for the widest enabled tier's maximum working scale (D1232:
working scale capped at `W::BITS/8 = 2048` decimal digits) plus generous
guard bits, so the slice->convert reconstruction is correctly rounded at
every tier and the conversion product `slot · 10^w` fits inside `W`.

## How the accessor consumes it

At call time, for working scale `w` and (kernel) `(idx, m)`, the accessor
maps `t = idx · (512 / m)` (exact: `512 / m in {1, 4}`), then reconstructs

    round(E_t · 10^w) = 10^w + round(F_t · 10^w)
                      = 10^w + round(slot_t · 10^w / 2^B)
                      = 10^w + ((slot_t · 10^w + 2^(B-1)) >> B)

in the tier's work integer `W` — one zero-extend, one multiply, one add,
one shift, one add of `10^w`. Far less than the `exp_fixed` Series it
replaces.
"""

from __future__ import annotations

import datetime
from mpmath import mp, mpf, exp, log, floor as mpfloor

# ── Tang table size (single M = 512 table; M = 128 reads index 4j) ────────
M = 512

# ── Binary precision of the baked entry (matches the ln pilot) ────────────
#
# The widest enabled tier is D1232: work integer W = Int<256> = 16384
# bits. The wide narrowing caps the working scale `w` at `W::BITS / 8`
# DECIMAL digits, i.e. 2048 digits for D1232 — the deepest F_j precision
# any call can demand.
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

# ── Oracle precision ──────────────────────────────────────────────────────
# B bits ≈ B/3.32193 ≈ 2158 decimal digits of value; add wide margin so
# round(F_j · 2^B) is exact.
mp.dps = 2600


def slot_limbs_msb_first(j: int):
    """Return the B-bit `round((exp(j·ln2/M) − 1) · 2^B)` as a
    fixed-length `[u64; B_LIMBS]` little-endian magnitude, emitted
    MOST-SIGNIFICANT limb first (so a narrow tier reads a high-limb
    prefix)."""
    if j == 0:
        # exp(0) − 1 = 0 exactly.
        n = 0
    else:
        ln2 = log(mpf(2))
        e_j = exp(mpf(j) * ln2 / mpf(M))   # exp(j·ln2/M), in (1, 2) for j in 1..M
        frac = e_j - mpf(1)                # fractional part in (0, 1)
        scaled = frac * (mpf(2) ** B)
        # Irrational, no tie; round-half-up via floor(scaled + 1/2). All
        # F_j < 1, so n < 2^B and fits in B bits.
        n = int(mpfloor(scaled + mpf("0.5")))
        assert 0 <= n < (1 << B), f"slot {j} out of B-bit range"
    # little-endian limbs, zero-padded to exactly B_LIMBS.
    le = []
    x = n
    for _ in range(B_LIMBS):
        le.append(x & 0xFFFFFFFFFFFFFFFF)
        x >>= 64
    assert x == 0, f"slot {j} exceeds {B_LIMBS} limbs"
    # emit MS-limb first.
    return list(reversed(le))


def main():
    out = []
    w = out.append

    w("// SPDX-FileCopyrightText: 2026 John Moxley")
    w("// SPDX-License-Identifier: MIT OR Apache-2.0")
    w("")
    w(f"//! Baked binary Tang `exp(j · ln2 / M)` lookup table (`M = {M}`).")
    w("//!")
    w("//! GENERATED by `scripts/gen_exp_tang_table.py` (mpmath oracle). Do")
    w("//! NOT edit by hand; re-run the script and commit its output. This")
    w("//! file is NOT produced at build time — `build.rs` is untouched.")
    w("//!")
    w(f"//! Each of the `M = {M}` slots holds the FRACTIONAL part")
    w("//! `F_j = exp(j · ln2 / 512) − 1` (`j ∈ [0, 512)`, all in `[0, 1)`)")
    w("//! as a correctly-rounded BINARY fixed-point value")
    w(f"//! `round(F_j · 2^{B})` — a `B = {B}`-bit unsigned magnitude stored as a")
    w(f"//! fixed-length `[u64; {B_LIMBS}]` little-endian array, but laid out")
    w("//! **most-significant limb first** within the entry. A narrower")
    w("//! tier reads a contiguous HIGH-limb PREFIX (a free slice); the")
    w("//! widest tier (D1232) reads the whole entry. The integer `1` is")
    w("//! re-added at convert time, so the stored slot is a pure fraction")
    w("//! (exactly like the `ln` pilot). `F_0 = 0` (exp 0 = 1, all-zero).")
    w("//!")
    w(f"//! One `M = {M}` table answers both shipped table sizes: the `M = 128`")
    w("//! tier (D57) reads index `4 · j`, since")
    w("//! `exp(j·ln2/128) = exp((4j)·ln2/512)`.")
    w("//!")
    w(f"//! `B = {B}` is sized for the widest enabled tier's max working scale")
    w("//! (D1232: `W = Int<256>`, working scale capped at `W::BITS/8 =")
    w(f"//! {W_MAX_DECIMAL}` decimal digits ≈ {W_MAX_DECIMAL * 332193 // 100000} bits) PLUS guard bits, so")
    w("//! the slice→convert reconstruction `round(slot · 10^w / 2^B)` is")
    w("//! correctly rounded at every tier and the conversion product")
    w("//! `slot · 10^w` fits inside `W`.")
    w("")
    w("/// Tang exp table size — the BAKED lattice `exp(j · ln2 / M)`,")
    w("/// `j ∈ [0, M)`. A single `M = 512` table; the `M = 128` tier reads")
    w("/// index `4 · j`.")
    w(f"pub(crate) const EXP_TANG_M: u32 = {M};")
    w("")
    w("/// Binary fixed-point exponent: each slot is")
    w(f"/// `round((exp(j·ln2/M) − 1) · 2^B)`. `B = {B}` bits = `{B_LIMBS}` u64 limbs.")
    w(f"pub(crate) const EXP_TANG_B: u32 = {B};")
    w("")
    w("/// Number of u64 limbs per stored slot (`B / 64`).")
    w(f"pub(crate) const EXP_TANG_LIMBS: usize = {B_LIMBS};")
    w("")
    w(f"/// The `M = {M}` baked slots `round((exp(j·ln2/M) − 1) · 2^B)`, each a")
    w(f"/// `[u64; {B_LIMBS}]` little-endian magnitude emitted MOST-SIGNIFICANT")
    w("/// limb FIRST (so a narrow tier reads a high-limb prefix). Index by")
    w(f"/// `j ∈ [0, {M})`.")
    w(f"pub(crate) static EXP_TANG_SLOTS: [[u64; {B_LIMBS}]; {M}] = [")
    for j in range(M):
        limbs = slot_limbs_msb_first(j)
        w(f"    // j = {j}: exp({j}·ln2/{M}) − 1")
        w("    [")
        for k in range(0, B_LIMBS, 4):
            chunk = limbs[k:k + 4]
            chunk_str = ", ".join(f"0x{l:016x}" for l in chunk)
            w(f"        {chunk_str},")
        w("    ],")
    w("];")
    w("")

    # ── Width-generic accessor ────────────────────────────────────────────
    w("use crate::int::types::traits::BigInt;")
    w("")
    w("/// `exp(idx · ln2 / m)` reconstructed at working scale `w` in the")
    w("/// tier's work integer `W` (a value `x` held as `x · 10^w`).")
    w("/// Replaces the per-call `exp_fixed` Series recompute.")
    w("///")
    w("/// `idx ∈ [0, m)`, `m ∈ {128, 512}`. The baked table is `M = 512`;")
    w("/// the `m = 128` tier reads index `t = 4 · idx` (`512 / m`). Each")
    w("/// slot stores the FRACTIONAL part `F_t = exp(t·ln2/512) − 1` as")
    w("/// `slot = round(F_t · 2^B)` (B = EXP_TANG_B, MS-limb first); the")
    w("/// integer `1` is re-added here as `10^w`. We SLICE the high-order")
    w("/// `p` limbs needed for this working scale — `slot_hi = floor(slot /")
    w("/// 2^(B − 64·p))`, the binary fixed-point of `F_t` at exponent `bp =")
    w("/// 64·p` — then reconstruct")
    w("///")
    w("/// ```text")
    w("/// round(exp(t·ln2/512) · 10^w) = 10^w + round(F_t · 10^w)")
    w("///                             = 10^w + round(slot_hi · 10^w / 2^bp)")
    w("///                             = 10^w + ((slot_hi · 10^w + 2^(bp-1)) >> bp)")
    w("/// ```")
    w("///")
    w("/// entirely in `W`: one zero-extend, one multiply, one add, one")
    w("/// shift, one add of `10^w`. `p` is chosen so `bp` carries the")
    w("/// working scale's bits plus generous guard, and so the product")
    w("/// `slot_hi · 10^w` fits `W`. `pow10_w` is `10^w` supplied by the")
    w("/// caller (the kernel's cached `pow10_table(w)` lookup), NOT a")
    w("/// per-call `W::TEN.pow(w)` recompute.")
    w("///")
    w("/// `idx = 0` short-circuits to `10^w` (exp 0 = 1). The MS-limb-first")
    w("/// layout makes the slice a contiguous high-limb PREFIX: a narrow")
    w("/// tier reads fewer limbs, the widest tier reads the whole entry.")
    w("#[inline]")
    w("pub(crate) fn exp_table_entry_baked<W: BigInt>(w: u32, idx: usize, m: u32, pow10_w: W) -> W {")
    w("    // exp(0) = 1.")
    w("    if idx == 0 {")
    w("        return pow10_w;")
    w("    }")
    w("    // Map the kernel index onto the M = 512 lattice. m ∈ {128, 512}")
    w("    // both divide 512 exactly, so `512 / m ∈ {4, 1}` and `t` is exact.")
    w("    let t = idx * (EXP_TANG_M / m) as usize;")
    w("    let slot = &EXP_TANG_SLOTS[t];")
    w("    // Binary precision needed: w·log2(10) value bits + guard. Use the")
    w("    // rational 3322/1000 ≈ log2(10) (a slight over-estimate) and a")
    w("    // 64-bit (one-limb) guard so the converted slot rounds correctly")
    w("    // yet the conversion product `slot_hi · 10^w` (≈ bp + w·log2(10)")
    w("    // ≈ 2·w·log2(10) + 64) stays inside `W`. The stored slot is a")
    w("    // pure fraction (< 1), identical to the ln pilot, so this sizing")
    w("    // is the same. Round the limb count up; clamp to the stored")
    w("    // width.")
    w("    let need_bits = (w as u64) * 3322 / 1000 + 64;")
    w("    let mut p = need_bits.div_ceil(64) as usize;")
    w("    p = p.clamp(1, EXP_TANG_LIMBS);")
    w("    // Zero-extend the top `p` limbs (MS-first) into W:")
    w("    //   slot_hi = sum_{k=0..p-1} slot[k] · 2^(64·(p−1−k)).")
    w("    let mut slot_hi = W::ZERO;")
    w("    for s in slot.iter().take(p) {")
    w("        slot_hi = (slot_hi << 64)")
    w("            | W::from_mag_sign_u128(&[*s as u128], false);")
    w("    }")
    w("    let bp = (64 * p) as u32;")
    w("    let scaled = slot_hi * pow10_w;")
    w("    // Round-half-up the fractional part: add 2^(bp−1), then shift")
    w("    // right by bp; finally re-add the integer 1 (= 10^w).")
    w("    let bias = W::ONE << (bp - 1);")
    w("    pow10_w + ((scaled + bias) >> bp)")
    w("}")
    w("")

    src = "\n".join(out) + "\n"
    path = "src/algos/support/exp_tang_table.rs"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    print(f"wrote {path} ({len(src)} bytes), B={B} ({B_LIMBS} limbs), "
          f"mp.dps={mp.dps}, M={M}, {datetime.date.today()}")


if __name__ == "__main__":
    main()
