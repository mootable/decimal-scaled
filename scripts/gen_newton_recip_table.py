#!/usr/bin/env python3
"""Generate `src/consts/newton_recip.rs` — the baked Newton-reciprocal table
for the `÷10^scale` rescale (the §9.20 baked-reciprocal lever).

For scale `s`, the Newton reciprocal is

    r(s) = floor( 2^(64*k) / 10^s ),   k = even(width_limbs + s//19 + 3)

(exactly what `newton_reciprocal::precompute` computes via `div_rem_mag_slice`
— integer floor division, so this Python `//` is bit-identical).

KEY (owner insight, verified): for a FIXED scale the reciprocal is the SAME
number `1/10^s`, just truncated to more limbs at wider widths. So
`r_w(s) == r_96(s) >> 64*(96 - w)` exactly (the high `k_w+1` limbs of `r_96(s)`).
Therefore we bake ONE reciprocal per scale at the widest working width
(96 u64 limbs = Int<96>, the AGM working width) and every narrower tier reads a
prefix slice — no per-tier storage. ~2.1 MB for s0..1850, all tiers shared.

Run: python scripts/gen_newton_recip_table.py   (rerun = byte-identical)
"""
import os

MAX_W = 132     # widest baked working width, u64 limbs (the split D1232 Tang work)
MAX_S = 1850    # highest baked scale (the D924 AGM working-scale cap)


def k96(s: int) -> int:
    pl = max(s // 19 + 3, 1)
    kk = MAX_W + pl
    return kk + (kk % 2)            # round UP to even (matches precompute)


def r96(s: int) -> int:
    return (1 << (64 * k96(s))) // (10 ** s)


def limbs_le(val: int, n: int):
    return [(val >> (64 * i)) & 0xFFFFFFFFFFFFFFFF for i in range(n)]


def main():
    out = []
    a = out.append
    a("// SPDX-FileCopyrightText: 2026 John Moxley")
    a("// SPDX-License-Identifier: MIT OR Apache-2.0")
    a("")
    a("//! Baked Newton-reciprocal table for the `/10^scale` rescale.")
    a("//!")
    a("//! `r(s) = floor(2^(64*k) / 10^s)`, `k = even(width + s/19 + 3)`, stored")
    a("//! little-endian u64 at the widest working width (96 u64 limbs = Int<96>,")
    a("//! the AGM working width). A narrower `w`-limb tier reads the HIGH")
    a("//! `k_w + 1` limbs: `r_w(s) = r_96(s) >> 64*(96 - w)` (an exact prefix —")
    a("//! the reciprocal of `10^s` is one number, truncated to fewer limbs at")
    a("//! narrower widths), so all tiers SHARE one per-scale reciprocal.")
    a("//!")
    a("//! Compile-time read-only data (architectural-review Class K — NOT a")
    a("//! runtime cache); size-local consumption via the width slice. GENERATED")
    a("//! by `scripts/gen_newton_recip_table.py` — do not edit by hand.")
    a("")
    a(f"/// Widest baked working width, in u64 limbs (Int<96> = the AGM width).")
    a('#[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a(f"pub(crate) const NEWTON_RECIP_MAX_W: usize = {MAX_W};")
    a(f"/// Highest baked scale (inclusive).")
    a('#[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a(f"pub(crate) const NEWTON_RECIP_MAX_SCALE: u32 = {MAX_S};")
    a("")
    a('#[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a("#[rustfmt::skip]")
    a(f"static NEWTON_RECIP: [&[u64]; {MAX_S + 1}] = [")
    for s in range(MAX_S + 1):
        ls = limbs_le(r96(s), k96(s) + 1)
        a("    &[" + ", ".join(f"0x{x:016x}" for x in ls) + "],")
    a("];")
    a("")
    a("/// `floor(2^(64*k) / 10^scale)` little-endian for a `width_limbs`-limb")
    a("/// working integer (`k = even(width_limbs + scale/19 + 3)`), or `None` if")
    a("/// outside the baked range (`scale > MAX_SCALE` / `width > MAX_W`), where")
    a("/// the caller falls back to the runtime reciprocal / MgChain.")
    a("#[inline]")
    a("pub(crate) fn newton_recip_le(scale: u32, width_limbs: usize) -> Option<&'static [u64]> {")
    a("    // The table is gated behind the wide features (size-local: the narrow /")
    a("    // base build never reaches the wide rescale). Absent => `None` => the")
    a("    // caller falls back to the runtime reciprocal / MgChain.")
    a('    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]')
    a("    {")
    a("        if scale <= NEWTON_RECIP_MAX_SCALE && width_limbs <= NEWTON_RECIP_MAX_W {")
    a("            // High `k_w + 1` limbs of the width-96 reciprocal (drop the low")
    a("            // `96 - width_limbs`): r_w = r_96 >> 64*(96 - width_limbs).")
    a("            return Some(&NEWTON_RECIP[scale as usize][NEWTON_RECIP_MAX_W - width_limbs..]);")
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
    print(f"wrote {path}: {size/1024/1024:.2f} MB, {MAX_S+1} scales, width {MAX_W} u64 limbs")


if __name__ == "__main__":
    main()
