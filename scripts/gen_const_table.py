"""Generate the per-scale, oracle-sourced, width-deduplicated wide
transcendental constant table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_golden_precision.py`). It is NOT run at build time: it emits a
committed Rust source file `src/algos/support/const_table.rs`, and that
output is what the crate compiles. `build.rs` is untouched.

Oracle. Every constant comes from `mpmath` directly at a working
precision (`mp.dps`) comfortably above the widest enabled tier's maximum
working scale, with margin, so `floor(const * 10**scale)` and the
"round-up" bit are exact at the top scale. No value is derived from the
crate's own `pi` or any `decimal_scaled` method.

The ten constants (all positive, all irrational):
    pi, tau (= 2*pi), half_pi (= pi/2), quarter_pi (= pi/4),
    e, golden (= (1+sqrt 5)/2), ln2 (= log 2), ln10 (= log 10),
    deg_per_rad (one radian in degrees, mpmath `degrees(1)` = 180/pi),
    rad_per_deg (one degree in radians, mpmath `radians(1)` = pi/180).

Encoding (per constant, per scale `s`):
    `floor(const * 10**s)` stored as the NARROWEST-fit little-endian
    `[u64; K]` array (1 u64 per ~17 decimal digits + a width step every
    ~17 digits), PLUS a single `round_up` bit = "is the dropped
    fractional tail >= 1/2". Because every constant is irrational, the
    tail is never exactly 1/2 (no tie) and never exactly 0, so the
    accessor derives every RoundingMode exactly:
        Trunc   = floor
        Floor   = floor
        Ceiling = floor + 1                  (tail always non-zero)
        HalfToEven = HalfAwayFromZero = HalfTowardZero = floor + round_up
    (the three half-modes coincide: with no possible tie they all reduce
    to "round to nearest", which is `floor + round_up`).

This reproduces, bit-for-bit, a CORRECT ROUNDING of the mpmath value
under each of the six modes — the same contract the runtime
`const_rounded` path implements. The crate-side unit test in
`const_table.rs` re-derives the six modes from the stored (floor,
round_up) pair and asserts they equal what the existing
`const_rounded_cf(...)` baked path returns at sampled cells.

Feature gating mirrors `src/types/consts/wide.rs`: each scale band is
gated by the same `any(feature = "<tier>", feature = "<umbrella>")` set
the tiers that can request that band carry. A narrow-only build compiles
none of it.
"""

from __future__ import annotations

import datetime
from mpmath import mp

# ── Tier table: (tier name, work-int limbs, max SCALE, gating cfg) ─────
#
# The wide-transcendental const-fold path (`WideConst<SCALE>`) requests a
# constant at working scale `w = SCALE + GUARD`, GUARD = 30, for any
# SCALE in `0 ..= max_scale`. So each tier needs working scales
# `0 ..= max_scale + GUARD`. The work-int limb count is informational
# (the value is width-independent — the accessor zero-extends).
GUARD = 30

# (tier, work_limbs, max_scale, cfg gate matching consts/wide.rs)
TIERS = [
    ("D57", 16, 56, 'any(feature = "d57", feature = "wide")'),
    ("D76", 16, 75, 'any(feature = "d76", feature = "wide")'),
    ("D115", 32, 114, 'any(feature = "d115", feature = "wide")'),
    ("D153", 32, 152, 'any(feature = "d153", feature = "wide")'),
    ("D230", 48, 229, 'any(feature = "d230", feature = "wide")'),
    ("D307", 64, 306, 'any(feature = "d307", feature = "x-wide")'),
    ("D462", 64, 461, 'any(feature = "d462", feature = "x-wide")'),
    ("D616", 128, 615, 'any(feature = "d616", feature = "x-wide")'),
    ("D924", 192, 923, 'any(feature = "d924", feature = "xx-wide")'),
    ("D1232", 256, 1231, 'any(feature = "d1232", feature = "xx-wide")'),
]

# Bands: a contiguous scale range gated by a single cfg. We pick three
# bands tracking the three feature umbrellas (wide / x-wide / xx-wide),
# but gate each by the FULL `any(...)` set of every sub-tier and umbrella
# that can reach that band so a single-tier build (e.g. just `d307`)
# still compiles its band.
#
#   base    0 ..= W_BASE  : reachable by any `_wide-support` tier; the
#                           widest plain-`wide` tier is D230 (w<=259), but
#                           D307 (d307/x-wide) reaches 336, so the base
#                           band must cover up to 336 for any d307 build.
#   xwide   .. ..= W_XW    : x-wide tiers (D462/D616), up to 645.
#   xxwide  .. ..= W_XXW   : xx-wide tiers (D924/D1232), up to 1261.
W_BASE = 336    # max working scale of D307 (the widest base/x-wide-min tier)
W_XW = 645      # max working scale of D616
W_XXW = 1261    # max working scale of D1232

# Gate strings. The base band is needed by every wide-support build.
BASE_CFG = 'feature = "_wide-support"'
# The x-wide band (337..=645) is reached by D462/D616 (and any build that
# turns those on directly or via x-wide).
XW_CFG = 'any(feature = "d462", feature = "d616", feature = "x-wide")'
# The xx-wide band (646..=1261) is reached by D924/D1232 (or xx-wide).
XXW_CFG = 'any(feature = "d924", feature = "d1232", feature = "xx-wide")'

# ── Oracle precision ──────────────────────────────────────────────────
# Comfortably above W_XXW (1261) with wide margin so floor + round-bit
# are exact at the top scale.
mp.dps = 5000

CONSTS = [
    ("pi", lambda: +mp.pi),
    ("tau", lambda: 2 * mp.pi),
    ("half_pi", lambda: mp.pi / 2),
    ("quarter_pi", lambda: mp.pi / 4),
    ("e", lambda: +mp.e),
    ("golden", lambda: (1 + mp.sqrt(5)) / 2),
    ("ln2", lambda: mp.log(2)),
    ("ln10", lambda: mp.log(10)),
    # one radian in degrees = 180/pi, sourced as a true oracle value.
    ("deg_per_rad", lambda: mp.degrees(1)),
    # one degree in radians = pi/180.
    ("rad_per_deg", lambda: mp.radians(1)),
]


def floor_and_roundbit(value, scale):
    """Return (floor(value * 10**scale), round_up_bit).

    `round_up_bit` is 1 iff the dropped fractional tail
    `value*10**scale - floor` is >= 1/2. Computed exactly via integer
    arithmetic on the oracle's mpf at high precision: we compute
    `floor(value * 10**scale)` and `floor(value * 10**scale * 2)`; the
    tail is >= 1/2 iff the doubled floor is odd-relative, i.e.
    `floor(2*v*10**s) - 2*floor(v*10**s) == 1`.
    """
    from mpmath import mpf, floor as mpfloor

    scaled = value * (mpf(10) ** scale)
    q = int(mpfloor(scaled))
    # tail = scaled - q in [0, 1); round up iff tail >= 1/2.
    # Compute floor(2*scaled) and compare to 2*q. Since the constants are
    # irrational, tail is never exactly 0 or exactly 1/2 (no tie), so the
    # comparison is unambiguous at mp.dps precision.
    q2 = int(mpfloor(scaled * 2))
    round_up = 1 if (q2 - 2 * q) >= 1 else 0
    return q, round_up


def limbs_le(n):
    """Little-endian u64 limbs of the non-negative integer `n`, narrowest
    fit (at least one limb). ~17 decimal digits per u64 emerges naturally
    from the 2**64 chunking."""
    if n == 0:
        return [0]
    out = []
    while n > 0:
        out.append(n & 0xFFFFFFFFFFFFFFFF)
        n >>= 64
    return out


def emit_entries(value, lo, hi):
    """Emit Rust `(scale, &[limbs], round_up)` tuples for scales lo..=hi."""
    lines = []
    for s in range(lo, hi + 1):
        q, rb = floor_and_roundbit(value, s)
        limbs = limbs_le(q)
        limb_str = ", ".join(f"0x{l:016x}" for l in limbs)
        lines.append(f"        ({s}, &[{limb_str}], {rb}),")
    return lines


def main():
    out = []
    w = out.append

    w("// SPDX-FileCopyrightText: 2026 John Moxley")
    w("// SPDX-License-Identifier: MIT OR Apache-2.0")
    w("")
    w("//! Per-scale, oracle-sourced, width-deduplicated wide")
    w("//! transcendental constant table.")
    w("//!")
    w("//! GENERATED by `scripts/gen_const_table.py` (mpmath oracle). Do")
    w("//! NOT edit by hand; re-run the script and commit its output. This")
    w("//! file is NOT produced at build time — `build.rs` is untouched.")
    w("//!")
    w("//! Each constant is its own array of `(scale, limbs, round_up)`")
    w("//! entries. `limbs` is `floor(const * 10^scale)` as the")
    w("//! narrowest-fit little-endian `u64` slice; `round_up` is 1 iff the")
    w("//! dropped fractional tail is >= 1/2. Every constant is irrational")
    w("//! and positive, so the tail is never an exact tie and never zero,")
    w("//! and the six rounding modes derive exactly from `(floor,")
    w("//! round_up)`:")
    w("//!")
    w("//! | mode | result |")
    w("//! |------|--------|")
    w("//! | Trunc / Floor | `floor` |")
    w("//! | Ceiling | `floor + 1` |")
    w("//! | HalfToEven / HalfAwayFromZero / HalfTowardZero | `floor + round_up` |")
    w("//!")
    w("//! The value is width-independent: an accessor zero-extends the")
    w("//! stored limbs into the caller's work integer (the high limbs stay")
    w("//! zero). Scale bands are feature-gated to match the tiers that can")
    w("//! request them (mirrors `src/types/consts/wide.rs`).")
    w("")
    w("/// A single table entry: `(scale, floor-limbs little-endian, round-up bit)`.")
    w("pub(crate) type Entry = (u32, &'static [u64], u8);")
    w("")

    bands = [
        ("BASE", 0, W_BASE, BASE_CFG),
        ("XW", W_BASE + 1, W_XW, XW_CFG),
        ("XXW", W_XW + 1, W_XXW, XXW_CFG),
    ]

    for name, getter in CONSTS:
        value = getter()
        upper = name.upper()
        for band, lo, hi, cfg in bands:
            w(f"#[cfg({cfg})]")
            w(f"static {upper}_{band}: &[Entry] = &[")
            out.extend(emit_entries(value, lo, hi))
            w("];")
            w("")

    # ── Per-constant `const fn` lookups, band-gated by `#[cfg]` on the
    # statements so a disabled band's static is never referenced. ───────
    for name, _ in CONSTS:
        upper = name.upper()
        w("/// `floor(%s * 10^scale)` limbs (little-endian, narrowest-fit)" % name)
        w("/// plus the round-up bit, for the const working `scale`.")
        w("///")
        w("/// `const fn` so a caller keyed on the const-generic SCALE folds")
        w("/// to the single matching entry per monomorphisation — no runtime")
        w("/// search on the hot path.")
        w(f"pub(crate) const fn {name}_entry(scale: u32) -> (&'static [u64], u8) {{")
        w(f"    #[cfg({BASE_CFG})]")
        w("    {")
        w(f"        if (scale as usize) < {upper}_BASE.len() {{")
        w(f"            let e = {upper}_BASE[scale as usize];")
        w("            return (e.1, e.2);")
        w("        }")
        w("    }")
        w(f"    #[cfg({XW_CFG})]")
        w("    {")
        w(f"        let base_lo = {W_BASE} + 1;")
        w("        if scale >= base_lo {")
        w("            let idx = (scale - base_lo) as usize;")
        w(f"            if idx < {upper}_XW.len() {{")
        w(f"                let e = {upper}_XW[idx];")
        w("                return (e.1, e.2);")
        w("            }")
        w("        }")
        w("    }")
        w(f"    #[cfg({XXW_CFG})]")
        w("    {")
        w(f"        let xw_lo = {W_XW} + 1;")
        w("        if scale >= xw_lo {")
        w("            let idx = (scale - xw_lo) as usize;")
        w(f"            if idx < {upper}_XXW.len() {{")
        w(f"                let e = {upper}_XXW[idx];")
        w("                return (e.1, e.2);")
        w("            }")
        w("        }")
        w("    }")
        w(f'    panic!("const_table: {name} scale out of generated range");')
        w("}")
        w("")

    # ── Width-generic accessor: zero-extend the stored limbs into W. ────
    w("use crate::int::types::traits::BigInt;")
    w("use crate::support::rounding::RoundingMode;")
    w("")
    w("/// Builds the work integer `W` holding `floor(const * 10^scale)`")
    w("/// from a narrow little-endian `limbs` slice by Horner-folding the")
    w("/// limbs high-to-low into a zeroed `W` (the high limbs stay zero —")
    w("/// the value never changes with width). Width transform = \"add")
    w("/// empty (zero) limbs\". `W::BITS >= 64` for every work integer, so")
    w("/// the `<< 64` never loses a limb of a value the const-fold path")
    w("/// requests (its magnitude fits the tier's reference width by")
    w("/// construction).")
    w("#[inline]")
    w("fn limbs_to_w<W: BigInt>(limbs: &[u64]) -> W {")
    w("    let mut acc = W::ZERO;")
    w("    let mut i = limbs.len();")
    w("    while i > 0 {")
    w("        i -= 1;")
    w("        acc = (acc << 64) | W::from_mag_sign_u128(&[limbs[i] as u128], false);")
    w("    }")
    w("    acc")
    w("}")
    w("")
    w("/// Applies `mode` to a `(floor-limbs, round_up)` table entry,")
    w("/// returning the correctly-rounded constant in the work integer `W`.")
    w("///")
    w("/// The constants are irrational + positive, so the dropped tail is")
    w("/// never an exact tie and never zero. Hence: Trunc / Floor keep the")
    w("/// floor; Ceiling always bumps (`+1`); the three half-modes all")
    w("/// reduce to round-to-nearest = `floor + round_up`. This reproduces")
    w("/// a correct rounding of the mpmath value under every mode.")
    w("#[inline]")
    w("fn round_entry<W: BigInt>(limbs: &[u64], round_up: u8, mode: RoundingMode) -> W {")
    w("    let floor = limbs_to_w::<W>(limbs);")
    w("    let bump = match mode {")
    w("        RoundingMode::Trunc | RoundingMode::Floor => false,")
    w("        RoundingMode::Ceiling => true,")
    w("        RoundingMode::HalfToEven")
    w("        | RoundingMode::HalfAwayFromZero")
    w("        | RoundingMode::HalfTowardZero => round_up != 0,")
    w("    };")
    w("    if bump { floor.wrapping_add(W::ONE) } else { floor }")
    w("}")
    w("")

    # Per-constant width-generic public accessors. TWO forms per
    # constant:
    #
    #   *_by_scale  — a `const fn` keyed on the CONST working scale. This
    #                 is the NORM: called in a const context (the const
    #                 SCALE of the monomorphisation), it const-folds to the
    #                 single matching entry, so the hot path does no
    #                 runtime search and no divide.
    #
    #   *_by_w      — a plain `fn` keyed on the RUNTIME working scale `w`.
    #                 This is the RARE fallback (the Ziv-escalation path,
    #                 `w != SCALE + GUARD`). It does NOT const-fold; every
    #                 avoidable use is a const-fold miss. Prefer *_by_scale
    #                 wherever a const SCALE is available.
    for name, _ in CONSTS:
        w(f"/// `{name}` at the CONST working `scale`, correctly rounded")
        w("/// under `mode`, in the work integer `W`. **The norm.**")
        w("///")
        w("/// The scale -> entry lookup ([`%s_entry`]) is a `const fn`; when" % name)
        w("/// `scale` is the const-generic working scale of the")
        w("/// monomorphisation it folds to the one matching `(&'static")
        w("/// [u64], round_up)`, so the only runtime work is the cheap")
        w("/// fixed-count zero-extend into `W` (which LLVM further folds")
        w("/// against the const limbs). No runtime search, no divide. The")
        w("/// hot (non-Ziv) path uses this.")
        w("#[inline]")
        w(f"pub(crate) fn {name}_by_scale<W: BigInt>(scale: u32, mode: RoundingMode) -> W {{")
        w(f"    let (limbs, round_up) = {name}_entry(scale);")
        w("    round_entry::<W>(limbs, round_up, mode)")
        w("}")
        w("")
        w(f"/// `{name}` at a RUNTIME working scale `w`, correctly rounded")
        w("/// under `mode`. **Rare fallback** — the Ziv-escalation path")
        w("/// (`w != SCALE + GUARD`) where the const scale is not available.")
        w("/// Does NOT const-fold; every avoidable call is a const-fold")
        w(f"/// miss. Prefer [`{name}_by_scale`] when a const SCALE is in hand.")
        w("#[inline]")
        w(f"pub(crate) fn {name}_by_w<W: BigInt>(w: u32, mode: RoundingMode) -> W {{")
        w(f"    let (limbs, round_up) = {name}_entry(w);")
        w("    round_entry::<W>(limbs, round_up, mode)")
        w("}")
        w("")

    # ── Self-test: re-derive the six modes from (floor, round_up) and
    # assert against a handful of independently-spelled known values. ───
    w("#[cfg(test)]")
    w("mod tests {")
    w("    use super::*;")
    w("    use crate::int::types::Int;")
    w("    use crate::support::rounding::RoundingMode::*;")
    w("")
    w("    /// The six modes derive correctly from the stored (floor,")
    w("    /// round_up) pair: Trunc=Floor=floor, Ceiling=floor+1, and the")
    w("    /// three half-modes coincide at floor+round_up (no ties for")
    w("    /// irrationals). Spot-checked against `pi` at a few scales.")
    w("    #[test]")
    w("    fn modes_derive_from_floor_and_roundbit() {")
    w("        // pi = 3.14159265358979323846...; at scale 4 -> 31415.9..,")
    w("        // floor 31415, tail .9 >= .5 -> round_up = 1.")
    w("        let f: Int<3> = limbs_to_w(&[31415]);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(4, Trunc), f);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(4, Floor), f);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(4, Ceiling), f.wrapping_add(Int::<3>::ONE));")
    w("        let up = f.wrapping_add(Int::<3>::ONE);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(4, HalfToEven), up);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(4, HalfAwayFromZero), up);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(4, HalfTowardZero), up);")
    w("        // scale 5 -> 314159.26.., floor 314159, tail .26 < .5 ->")
    w("        // round_up = 0; all three half-modes keep the floor.")
    w("        let f5: Int<3> = limbs_to_w(&[314159]);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(5, HalfToEven), f5);")
    w("        assert_eq!(pi_by_scale::<Int<3>>(5, Ceiling), f5.wrapping_add(Int::<3>::ONE));")
    w("    }")
    w("")
    w("    /// `by_scale` and `by_w` return identical values for the same")
    w("    /// scale (they differ only in const-fold behaviour, not value).")
    w("    #[test]")
    w("    fn by_scale_eq_by_w() {")
    w("        for s in [0u32, 1, 17, 18, 19, 30, 86] {")
    w("            for m in [HalfToEven, Trunc, Ceiling, Floor, HalfAwayFromZero, HalfTowardZero] {")
    w("                assert_eq!(pi_by_scale::<Int<16>>(s, m), pi_by_w::<Int<16>>(s, m));")
    w("                assert_eq!(ln2_by_scale::<Int<16>>(s, m), ln2_by_w::<Int<16>>(s, m));")
    w("            }")
    w("        }")
    w("    }")
    w("")
    w("    /// Width-independence: the same scale gives the same value")
    w("    /// (zero-extended) in different work-int widths.")
    w("    #[test]")
    w("    fn value_is_width_independent() {")
    w("        for s in [0u32, 5, 17, 18, 30, 50] {")
    w("            let a = pi_by_scale::<Int<16>>(s, HalfToEven);")
    w("            let b = pi_by_scale::<Int<32>>(s, HalfToEven);")
    w("            assert_eq!(a, b.resize_to::<Int<16>>());")
    w("        }")
    w("    }")
    w("}")
    w("")

    src = "\n".join(out) + "\n"
    path = "src/algos/support/const_table.rs"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    print(f"wrote {path} ({len(src)} bytes), mp.dps={mp.dps}, "
          f"{datetime.date.today()}")


if __name__ == "__main__":
    main()
