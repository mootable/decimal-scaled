"""Generate the baked binary Tang `(sin(c_j), cos(c_j))` lookup table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_ln_tang_table.py` / `gen_const_table.py`). It is NOT run at build
time: it emits a committed Rust source file
`src/algos/support/sincos_tang_table.rs`, and that output is what the
crate compiles. `build.rs` is untouched.

## What it stores

The wide `sin`/`cos`/`tan` kernel (`algos::trig::sincos_tang`) uses a
Tang-style argument reduction with `M = SINCOS_TANG_M = 512`: after
reducing the argument to a residual `c_j = j·π/(4·M)` (`j ∈ [0, M]`) it
needs the pair `(sin(c_j), cos(c_j))` at the working scale. The previous
accessor recomputed that pair per call by running a full `sin_cos_fixed`
Series at the working scale — the dominant cost of the wide-trig
regression. This table bakes the pair ONCE (oracle, mpmath) so the
per-call cost collapses to one multiply + one shift per component.

## Storage — binary fixed-point, MS-limb-first

For `j ∈ [1, M]` the angle `c_j = j·π/(4·M)` lies in
`(0, π/4]`, so

    sin(c_j) ∈ (0, sin(π/4) = √2/2 ≈ 0.7071]   ⊂ [0, 1)
    cos(c_j) ∈ [cos(π/4) = √2/2 ≈ 0.7071, 1)   ⊂ [0, 1)

— both are pure fractions strictly < 1 (cos(c_j) < 1 because c_j > 0 for
every j ≥ 1). We store each as the correctly-rounded binary fixed-point

    slot = round(value · 2^B)          (an unsigned B-bit integer)

as a fixed-length little-endian `[u64; LIMBS]` array, emitted
**most-significant limb first** within the entry so a narrower tier can
read a contiguous HIGH-limb PREFIX (a free slice) and the widest tier
reads the whole entry.

The `j = 0` slot is the special case `(sin 0, cos 0) = (0, 1)`. `cos 0 =
1.0` is the ONLY value with an integer digit and would not fit a pure
B-bit fraction. Rather than widen every slot for one entry, the accessor
SHORT-CIRCUITS `j = 0` to `(W::ZERO, pow10_w)` (i.e. `(0, 1)` at scale
`w`) — exactly as the `ln` pilot short-circuits `idx = 0` to `0`. The
stored `j = 0` slot is therefore all-zero and never read.

`B` is sized for the WIDEST enabled tier's maximum working scale (D1232:
working scale capped at `W::BITS/8 = 2048` decimal digits) plus generous
guard bits — identical to the `ln` pilot (`LN_TANG_B = 7168`), so the
slice→convert reconstruction is correctly rounded at every tier.

## How the accessor consumes it

At call time, for working scale `w`, the accessor reconstructs each
component

    round(value · 10^w) ≈ round(slot · 10^w / 2^B)
                        = (slot · 10^w + 2^(B-1)) >> B

in the tier's work integer `W`. One multiply + one add + one shift per
component — far less than the Series it replaces.
"""

from __future__ import annotations

import datetime
from mpmath import mp, mpf, sin, cos, pi, floor as mpfloor

# ── Tang table size (do NOT change here; matches the const-generic `M`
# at the `sincos_tang` call sites, currently 512). ───────────────────────
M = 512

# ── Binary precision of the baked entry ──────────────────────────────────
#
# Sized IDENTICALLY to the `ln` pilot (`LN_TANG_B = 7168`): the widest
# enabled tier is D1232 (work integer W = Int<256> = 16384 bits); the
# wide directed/nearest narrowing caps the working scale `w` at
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


def slot_pair_msb_first(j: int):
    """Return `(sin_limbs, cos_limbs)` for `c_j = j·π/(4·M)`, each a
    B-bit `[u64; B_LIMBS]` MS-limb-first magnitude. `j = 0` is stored
    all-zero (the accessor short-circuits `(0, 1)`)."""
    if j == 0:
        zero = [0] * B_LIMBS
        return zero, list(zero)
    c_j = mpf(j) * pi / (mpf(4) * mpf(M))     # in (0, π/4]
    s = sin(c_j)                               # in (0, √2/2]  ⊂ [0, 1)
    c = cos(c_j)                               # in [√2/2, 1)  ⊂ [0, 1)
    assert 0 < s < 1, f"sin(c_{j}) = {s} not in (0, 1)"
    assert 0 < c < 1, f"cos(c_{j}) = {c} not in (0, 1)"
    return round_to_b_limbs(s), round_to_b_limbs(c)


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
    w(f"//! Baked binary Tang `(sin(c_j), cos(c_j))` lookup table (`M = {M}`).")
    w("//!")
    w("//! GENERATED by `scripts/gen_sincos_tang_table.py` (mpmath oracle).")
    w("//! Do NOT edit by hand; re-run the script and commit its output.")
    w("//! This file is NOT produced at build time — `build.rs` is")
    w("//! untouched.")
    w("//!")
    w(f"//! Each of the `M + 1 = {M + 1}` slots holds the pair `(sin(c_j),")
    w(f"//! cos(c_j))` for `c_j = j·π/(4·M)` (`j ∈ [0, {M}]`). For `j ≥ 1`,")
    w("//! `c_j ∈ (0, π/4]`, so `sin(c_j) ∈ (0, √2/2]` and `cos(c_j) ∈")
    w("//! [√2/2, 1)` — both pure fractions in `[0, 1)`. Each component is")
    w(f"//! stored as a correctly-rounded BINARY fixed-point `round(· 2^{B})`")
    w(f"//! — a `B = {B}`-bit unsigned magnitude stored as a fixed-length")
    w(f"//! `[u64; {B_LIMBS}]` little-endian array, but laid out")
    w("//! **most-significant limb first** within the entry. A narrower")
    w("//! tier reads a contiguous HIGH-limb PREFIX (a free slice); the")
    w("//! widest tier (D1232) reads the whole entry.")
    w("//!")
    w("//! The `j = 0` slot `(sin 0, cos 0) = (0, 1)` is the ONLY pair with")
    w("//! an integer digit (`cos 0 = 1.0`); rather than widen every slot")
    w("//! for it, the accessor SHORT-CIRCUITS `j = 0` to `(0, pow10_w)` and")
    w("//! the stored `j = 0` slot is all-zero (never read).")
    w("//!")
    w(f"//! `B = {B}` is sized IDENTICALLY to the `ln` pilot (`LN_TANG_B`),")
    w("//! for the widest enabled tier's max working scale (D1232: `W =")
    w(f"//! Int<256>`, working scale capped at `W::BITS/8 = {W_MAX_DECIMAL}` decimal")
    w(f"//! digits ≈ {W_MAX_DECIMAL * 332193 // 100000} bits) PLUS guard bits, so the slice→convert")
    w("//! reconstruction `round(slot · 10^w / 2^B)` is correctly rounded at")
    w("//! every tier and the conversion product `slot · 10^w` fits `W`.")
    w("")
    w("/// Tang table size — `(sin, cos)` of `c_j = j·π/(4·M)`, `j ∈ [0,")
    w("/// M]`. Matches the const-generic `M` at the `sincos_tang` call")
    w("/// sites (`policy::trig`, `lib.rs`).")
    w(f"pub(crate) const SINCOS_TANG_M: u32 = {M};")
    w("")
    w("/// Binary fixed-point exponent: each component is `round(value ·")
    w(f"/// 2^B)`. `B = {B}` bits = `{B_LIMBS}` u64 limbs.")
    w(f"pub(crate) const SINCOS_TANG_B: u32 = {B};")
    w("")
    w("/// Number of u64 limbs per stored component (`B / 64`).")
    w(f"pub(crate) const SINCOS_TANG_LIMBS: usize = {B_LIMBS};")
    w("")
    w("/// The `M + 1` baked `sin(c_j)` slots `round(sin(c_j) · 2^B)`, each")
    w(f"/// a `[u64; {B_LIMBS}]` little-endian magnitude emitted")
    w("/// MOST-SIGNIFICANT limb FIRST. Index by `j ∈ [0, M]`; `j = 0` is")
    w("/// all-zero (the accessor short-circuits `sin 0 = 0`).")
    w(f"pub(crate) static SINCOS_TANG_SIN: [[u64; {B_LIMBS}]; {M + 1}] = [")
    sin_slots = []
    cos_slots = []
    for j in range(M + 1):
        s, c = slot_pair_msb_first(j)
        sin_slots.append(s)
        cos_slots.append(c)
    for j in range(M + 1):
        w(f"    // j = {j}: sin({j}·pi/(4·{M}))")
        emit_slot_array(w, sin_slots[j])
    w("];")
    w("")
    w("/// The `M + 1` baked `cos(c_j)` slots `round(cos(c_j) · 2^B)`, each")
    w(f"/// a `[u64; {B_LIMBS}]` little-endian magnitude emitted")
    w("/// MOST-SIGNIFICANT limb FIRST. Index by `j ∈ [0, M]`; `j = 0` is")
    w("/// all-zero (the accessor short-circuits `cos 0 = 1`).")
    w(f"pub(crate) static SINCOS_TANG_COS: [[u64; {B_LIMBS}]; {M + 1}] = [")
    for j in range(M + 1):
        w(f"    // j = {j}: cos({j}·pi/(4·{M}))")
        emit_slot_array(w, cos_slots[j])
    w("];")
    w("")

    # ── Width-generic accessor ────────────────────────────────────────────
    w("use crate::int::types::traits::BigInt;")
    w("")
    w("/// Reconstruct one baked binary fixed-point slot at working scale")
    w("/// `w` into the tier's work integer `W`. `slot` is the MS-limb-first")
    w("/// `round(value · 2^B)` (`B = SINCOS_TANG_B`); we SLICE the high-order")
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
    w("fn reconstruct<W: BigInt>(slot: &[u64; SINCOS_TANG_LIMBS], w: u32, pow10_w: W) -> W {")
    w("    // Binary precision needed: w·log2(10) value bits + guard. Use the")
    w("    // rational 3322/1000 ≈ log2(10) (a slight over-estimate) and a")
    w("    // 64-bit (one-limb) guard so the converted slot rounds correctly")
    w("    // yet the conversion product `slot_hi · 10^w` stays inside `W`")
    w("    // even on the narrowest work integer (same budget as the `ln`")
    w("    // pilot). Round the limb count up; assert it fits the stored width.")
    w("    let need_bits = (w as u64) * 3322 / 1000 + 64;")
    w("    let p_full = need_bits.div_ceil(64) as usize;")
    w("    assert!(")
    w("        p_full <= SINCOS_TANG_LIMBS,")
    w("        \"sincos_tang: working scale {w} out of generated range ({SINCOS_TANG_LIMBS} limbs)\"")
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
    w("/// `(sin(c_j), cos(c_j))` with `c_j = idx·π/(4·m)` reconstructed at")
    w("/// working scale `w` (`idx ∈ [0, m]`, `m = SINCOS_TANG_M`) in the")
    w("/// tier's work integer `W` (a value `x` held as `x · 10^w`). Replaces")
    w("/// the per-call `sin_cos_fixed` Series recompute.")
    w("///")
    w("/// `idx = 0` short-circuits to `(0, pow10_w)` = `(sin 0, cos 0)`")
    w("/// at scale `w`; this is the only pair with an integer digit (`cos 0")
    w("/// = 1`), so it is NOT stored as a B-bit fraction. For `idx ≥ 1` both")
    w("/// `sin(c_j)` and `cos(c_j)` are pure fractions `< 1` read from the")
    w("/// baked tables. `m` MUST equal `SINCOS_TANG_M` (debug-asserted).")
    w("///")
    w("/// `pow10_w` is `10^w` in `W`, supplied by the caller from the")
    w("/// kernel's baked `pow10_table` (a lookup, not a `W::TEN.pow(w)`")
    w("/// per-call recompute).")
    w("#[inline]")
    w("pub(crate) fn sincos_table_entry_baked<W: BigInt>(")
    w("    w: u32,")
    w("    idx: usize,")
    w("    m: u32,")
    w("    pow10_w: W,")
    w(") -> (W, W) {")
    w("    debug_assert!(")
    w("        m == SINCOS_TANG_M,")
    w("        \"sincos_table_entry_baked: m must equal SINCOS_TANG_M\"")
    w("    );")
    w("    let _ = m;")
    w("    if idx == 0 {")
    w("        // (sin 0, cos 0) = (0, 1) at scale w.")
    w("        return (W::ZERO, pow10_w);")
    w("    }")
    w("    let sin = reconstruct::<W>(&SINCOS_TANG_SIN[idx], w, pow10_w);")
    w("    let cos = reconstruct::<W>(&SINCOS_TANG_COS[idx], w, pow10_w);")
    w("    (sin, cos)")
    w("}")
    w("")

    src = "\n".join(out) + "\n"
    path = "src/algos/support/sincos_tang_table.rs"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    print(f"wrote {path} ({len(src)} bytes), B={B} ({B_LIMBS} limbs), "
          f"mp.dps={mp.dps}, M={M}, {datetime.date.today()}")


if __name__ == "__main__":
    main()
