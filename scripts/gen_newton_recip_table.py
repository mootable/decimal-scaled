#!/usr/bin/env python3
"""Generate `src/consts/newton_recip.rs` — the baked Newton-reciprocal table
for the `÷10^scale` rescale (the §9.20 baked-reciprocal lever).

For scale `s`, the Newton reciprocal is

    r(s) = floor( 2^(64*k) / 10^s ),   k = even(width_limbs + s//19 + 3)

(exactly what `newton_reciprocal::precompute` computes via `div_rem_mag_slice`
— integer floor division, so this Python `//` is bit-identical).

KEY (owner insight, verified): for a FIXED scale the reciprocal is the SAME
number `1/10^s`, just truncated to more limbs at wider widths. So
`r_w(s) == r_132(s) >> 64*(132 - w)` exactly (the high `k_w+1` limbs of
`r_132(s)`) — for an EVEN `w`. Because `k` rounds UP to even, an odd width's
prefix is one limb off and is refused by the emitted accessor's guard.
Therefore we bake ONE reciprocal per scale at the widest working width
(132 u64 limbs = Int<132>, the split D1232 Tang working width) and every
narrower tier reads a prefix slice — no per-tier storage. ~6.5 MB for
s0..1850, all tiers shared.

The maths above is derived in the symbols `r(s)`, `r_w(s)`, `k` and `w`;
those symbol forms are kept deliberately. Everything else names the
quantity it holds.

Run: python scripts/gen_newton_recip_table.py   (rerun = byte-identical)
"""
import os

# Widest baked working width, in u64 limbs (the split D1232 Tang work).
MAX_WIDTH_LIMBS = 132
# Highest baked scale (the D924 AGM working-scale cap).
MAX_SCALE = 1850


def shift_limbs(scale: int) -> int:
    """`k` — the numerator's power of 2^64, rounded UP to even."""
    pow_len = max(scale // 19 + 3, 1)
    shift_limbs_raw = MAX_WIDTH_LIMBS + pow_len
    # round UP to even (matches precompute)
    return shift_limbs_raw + (shift_limbs_raw % 2)


def recip(scale: int) -> int:
    """`r(scale) = floor(2^(64*k) / 10^scale)` at the widest baked width."""
    return (1 << (64 * shift_limbs(scale))) // (10 ** scale)


def limbs_le(value: int, limb_count: int):
    return [(value >> (64 * i)) & 0xFFFFFFFFFFFFFFFF for i in range(limb_count)]


def main():
    out = []
    a = out.append
    a("// SPDX-FileCopyrightText: 2026 John Moxley")
    a("// SPDX-License-Identifier: MIT OR Apache-2.0")
    a("")
    a("//! Baked Newton-reciprocal table for the `/10^scale` rescale.")
    a("//!")
    a("//! `r(s) = floor(2^(64*k) / 10^s)`, `k = even(width_limbs + s/19 + 3)`,")
    a("//! stored little-endian u64 at the widest working width (132 u64 limbs")
    a("//! = Int<132>, the split D1232 Tang working width). A narrower `w`-limb")
    a("//! tier reads the HIGH `k_w + 1` limbs: `r_w(s) = r_132(s) >> 64*(132 -")
    a("//! w)` (an exact prefix — the reciprocal of `10^s` is one number,")
    a("//! truncated to fewer limbs at narrower widths), so all tiers SHARE one")
    a("//! per-scale reciprocal. The prefix identity holds for an EVEN `w` only;")
    a("//! see the guard in [`newton_recip_le`].")
    a("//!")
    a("//! EXACT BY CONSTRUCTION — no oracle is involved, and none is needed.")
    a("//! Each value is `(1 << (64*k)) // (10**s)`: an exact big-integer floor")
    a("//! division, computed with Python's arbitrary-precision integers. There")
    a("//! is no transcendental, no floating point and no rounding anywhere in")
    a("//! the derivation, so there are no digits to bound and nothing an")
    a("//! interval oracle could tighten. It is also bit-identical to what")
    a("//! `newton_reciprocal::precompute` computes at runtime via")
    a("//! `div_rem_mag_slice`, which is the property the table exists to bake.")
    a("//!")
    a("//! Compile-time read-only data (architectural-review Class K — NOT a")
    a("//! runtime cache); size-local consumption via the width slice. GENERATED")
    a("//! by `scripts/gen_newton_recip_table.py` — do not edit by hand.")
    a("")
    a("/// Widest baked working width, in u64 limbs (Int<132> = the split D1232")
    a("/// Tang working width).")
    a('#[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a(f"pub(crate) const NEWTON_RECIP_MAX_WIDTH_LIMBS: usize = {MAX_WIDTH_LIMBS};")
    a("/// Highest baked scale (inclusive).")
    a('#[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a(f"pub(crate) const NEWTON_RECIP_MAX_SCALE: u32 = {MAX_SCALE};")
    a("")
    a('#[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a("#[rustfmt::skip]")
    a(f"static NEWTON_RECIP: [&[u64]; {MAX_SCALE + 1}] = [")
    for scale in range(MAX_SCALE + 1):
        limbs = limbs_le(recip(scale), shift_limbs(scale) + 1)
        a("    &[" + ", ".join(f"0x{x:016x}" for x in limbs) + "],")
    a("];")
    a("")
    a("/// `floor(2^(64*k) / 10^scale)` little-endian for a `width_limbs`-limb")
    a("/// working integer (`k = even(width_limbs + scale/19 + 3)`), or `None`")
    a("/// when the request is not served: outside the baked range (`scale >")
    a("/// NEWTON_RECIP_MAX_SCALE` / `width_limbs >")
    a("/// NEWTON_RECIP_MAX_WIDTH_LIMBS`), or an ODD `width_limbs` (see the")
    a("/// guard below). The caller then falls back to the runtime reciprocal /")
    a("/// MgChain.")
    a("#[inline]")
    a("pub(crate) fn newton_recip_le(scale: u32, width_limbs: usize) -> Option<&'static [u64]> {")
    a("    // ODD WIDTHS ARE NOT SERVED: the baked row is a valid prefix only at")
    a("    // an EVEN width. The row holds the reciprocal at the baked width with")
    a("    // `k = even(baked_width + pow_len)` limbs, and a narrower reader takes")
    a("    // the high-limb prefix `>> 64*(baked_width - width_limbs)`. Since `k`")
    a("    // rounds UP to even and the baked width is itself even, that shift")
    a("    // lands on the reader's own `k = even(width_limbs + pow_len)` only")
    a("    // when `width_limbs` is even too. At an odd width the slice is one")
    a("    // limb off and carries the reciprocal of a DIFFERENT power of two, so")
    a("    // the exact-prefix identity does not hold. Return `None` and let the")
    a("    // caller use its runtime divide, which is exact at any width — a")
    a("    // graceful fallback, not a panic.")
    a("    //")
    a("    // Nothing reaches this today (the Newton band floor is 24 limbs and")
    a("    // the crate instantiates no odd `Int<N>` that wide); it is a")
    a("    // deliberate wall so a future odd width degrades instead of reading a")
    a("    // wrong reciprocal prefix.")
    a("    if !width_limbs.is_multiple_of(2) {")
    a("        return None;")
    a("    }")
    a("    // The table is gated behind the wide features (size-local: the narrow /")
    a("    // base build never reaches the wide rescale). Absent => `None` => the")
    a("    // caller falls back to the runtime reciprocal / MgChain.")
    a('    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a("    {")
    a("        if scale <= NEWTON_RECIP_MAX_SCALE")
    a("            && width_limbs <= NEWTON_RECIP_MAX_WIDTH_LIMBS")
    a("        {")
    a("            // High `k_w + 1` limbs of the baked width-132 reciprocal (drop")
    a("            // the low `132 - width_limbs`):")
    a("            // `r_w = r_132 >> 64*(132 - width_limbs)`.")
    a("            return Some(")
    a("                &NEWTON_RECIP[scale as usize]")
    a("                    [NEWTON_RECIP_MAX_WIDTH_LIMBS - width_limbs..],")
    a("            );")
    a("        }")
    a("    }")
    a("    let _ = (scale, width_limbs);")
    a("    None")
    a("}")
    a("")

    path = os.path.join("src", "consts", "newton_recip.rs")
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(out))
    size = os.path.getsize(path)
    print(f"wrote {path}: {size/1024/1024:.2f} MB, {MAX_SCALE + 1} scales, "
          f"width {MAX_WIDTH_LIMBS} u64 limbs")


if __name__ == "__main__":
    main()
