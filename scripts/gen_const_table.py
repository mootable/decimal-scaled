"""Generate the per-scale, oracle-sourced, width-deduplicated wide
transcendental constant table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_golden_precision.py`). It is NOT run at build time: it emits a
committed Rust source file `src/consts/table.rs`, and that
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
# The wide-transcendental const-fold path requests a constant at the HOT
# working scale `w = SCALE + GUARD` (GUARD = 30) for any SCALE in
# `0 ..= max_scale`. But the directed-rounding / Ziv-escalation + Tang-
# reconstruction paths request `w` UP TO the cap `W::BITS / 8` decimal digits
# (W = the tier's work integer), and that runtime `w` is served by a pure
# STATIC LOOKUP (`*_by_working_scale`) — never a recompute. So each tier needs
# working scales `0 ..= work_limbs * 8` (= W::BITS/8). The work-int limb count
# is therefore LOAD-BEARING: it sets each band's Ziv cap (below).
# (The value is width-independent — the accessor zero-extends.)
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
#   base    0 ..= W_BASE  : reachable by any `_wide-support` tier. Sized to the
#                           Ziv cap (W::BITS/8) of the widest wide-group tier,
#                           D307 (work Int<64> -> 64*8 = 512). [D307 in `wide`
#                           per lib.rs; it pulls `_wide-support`, so BASE covers
#                           its full Ziv band.]
#   xwide   .. ..= W_XW    : x-wide tiers; widest is D616 (work Int<128> ->
#                           128*8 = 1024).
#   xxwide  .. ..= W_XXW   : xx-wide tiers; widest is D1232 (work Int<256> ->
#                           256*8 = 2048).
# Each band's max = the max Ziv cap of its own feature group, so a build never
# Ziv-escalates into an absent (more-gated) band.
W_BASE = 512    # D307 Ziv cap (W::BITS/8, work Int<64>) — widest wide-group tier
W_XW = 1024     # D616 Ziv cap (work Int<128>)
W_XXW = 2048    # D1232 Ziv cap (work Int<256>)

# Per-constant band maxes. Not every constant needs the full Ziv band — only
# the ones the transcendental CORES request at deep/Ziv working scales do.
# Verified against the cores' `*_cf` accessors: pi / ln2 / ln10 / deg_per_rad /
# rad_per_deg are the ONLY constants read at a working scale; tau / half_pi /
# quarter_pi / e / golden are DecimalConstants-only.
#   ZIV (pi, ln2, ln10): cores Ziv-escalate + Tang-reconstruct up to W::BITS/8.
#   HOT (deg_per_rad, rad_per_deg): cores read the HOT scale SCALE+GUARD only
#       (the angle kernels do not Ziv-escalate) -> max_scale + GUARD.
#   DEC (tau, half_pi, quarter_pi, e, golden): DecimalConstants only, read at
#       the type's const SCALE <= max_scale -> max_scale.
# (NARROW stays 0..=W_NARROW for every constant — D18/D38 DecimalConstants.)
ZIV_MAXES = (W_BASE, W_XW, W_XXW)   # 512 / 1024 / 2048  (D307 / D616 / D1232 Ziv cap)
HOT_MAXES = (336, 645, 1261)        # (D307 / D616 / D1232) max_scale + GUARD
DEC_MAXES = (306, 615, 1231)        # (D307 / D616 / D1232) max_scale
# ln2 is special: exp's range reduction requests ln2 at the EXTENDED working
# scale `w_ext = w + extra`, where `extra = ceil(|k|*log10(2)) + margin` is the
# range-reduction lift that absorbs the `2^k` amplification (k = round(v/ln2)).
# For a large-argument exp whose result still fits the tier's work integer W
# (so it stays on the per-tier table path, NOT the wider series path), the lift
# is ~1.25 * result-digits and the Ziv guard adds up to `W::BITS/8 - int_digits`,
# so the request reaches `w_ext <= 2040 + 0.25*max_scale` — just past the plain
# `W::BITS/8` Ziv cap. Sized to ~1.5x the Ziv cap so the per-tier table path
# never escalates past its band. (The widest large-result cases route to the
# series-ln2 wide path, which does NOT read this table.)
# Sized to the wide/large-result path's peak bound: exp_generic::exp_fixed (the
# series-free, table-sourced ln2 path) runs on Wexp and its squaring peak caps
# w_ext at `(Wexp::BITS - 512)/6.644` (exp_internal_peak_bits) — ~1156/2389/4855
# for D307/D616/D1232 (Wexp = Int<128>/Int<256>/Int<512>). Round up with margin so
# the constant-sourced wide exp/hyperbolic path never escalates past its band.
LN2_MAXES = (1280, 2560, 5120)      # wide-path peak bound (D307 / D616 / D1232)
CONST_CLASS = {
    "pi": ZIV_MAXES, "ln2": LN2_MAXES, "ln10": ZIV_MAXES,
    "deg_per_rad": HOT_MAXES, "rad_per_deg": HOT_MAXES,
    "tau": DEC_MAXES, "half_pi": DEC_MAXES, "quarter_pi": DEC_MAXES,
    "e": DEC_MAXES, "golden": DEC_MAXES,
}

# The ALWAYS-PRESENT narrow band. The public `DecimalConstants` trait
# (D18 = Int<1>, scale 0..=17; D38 = Int<2>, scale 0..=38) sources its
# constants from this table in EVERY build — including default (no
# `_wide-support`) and `--no-default-features` (no_std). It is therefore
# emitted WITHOUT any feature gate. It covers 0..=38: scale 38 is past
# D38's representable range for pi/tau/e (they overflow i128 there), but
# the entry must still exist so the narrow path can READ it and apply
# its own storage-range guard (panic with "out of storage range").
W_NARROW = 38   # max D38 scale (entry present so the narrow path can range-check it)
# ln2's NARROW band is wider than the other constants': the narrow tiers' exp
# (`exp_series_2limb`, D18/D38) run the generic `exp_generic::exp_fixed`, whose
# range reduction now reads `ln2` FROM THIS TABLE (not a series) at the EXTENDED
# working scale `w_ext = w + extra` — up to ~165 digits even for a 38-digit D38
# result (the `2^k` lift). The default / no_std build has NO wider band, so the
# always-present NARROW band must cover that itself. 512 leaves ample margin.
LN2_NARROW = 512

# Gate strings. The base band is needed by every wide-support build.
BASE_CFG = 'feature = "_wide-support"'
# The x-wide band (513..=1024) is reached by D462/D616 (and any build that
# turns those on directly or via x-wide).
XW_CFG = 'any(feature = "d462", feature = "d616", feature = "x-wide")'
# The xx-wide band (1025..=2048) is reached by D924/D1232 (or xx-wide).
XXW_CFG = 'any(feature = "d924", feature = "d1232", feature = "xx-wide")'

# ── Oracle precision ──────────────────────────────────────────────────
# Comfortably above W_XXW (2048) with wide margin so floor + round-bit
# are exact at the top scale.
mp.dps = 6000

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
    w("//! zero). The narrow band (`*_NARROW`, scales 0..=%d) is ALWAYS" % W_NARROW)
    w("//! present — the public `DecimalConstants` trait on D18/D38 reads it")
    w("//! in every build (default / no_std included). The three wider bands")
    w("//! are feature-gated to match the tiers that can request them")
    w("//! (mirrors `src/types/consts/wide.rs`).")
    w("")
    w("/// A single table entry: `(scale, floor-limbs little-endian, round-up bit)`.")
    w("pub(crate) type Entry = (u32, &'static [u64], u8);")
    w("")

    # The NARROW band (0..=W_NARROW) is ALWAYS present (no `cfg`): the public
    # `DecimalConstants` trait on D18/D38 reads it in every build. The three
    # wider bands are feature-gated, and each constant's band maxes are sized to
    # its CLASS (ZIV / HOT / DEC — see CONST_CLASS) so only the constants that
    # Ziv-escalate carry the full deep band.
    for name, getter in CONSTS:
        value = getter()
        upper = name.upper()
        base_max, xw_max, xxw_max = CONST_CLASS[name]
        narrow_max = LN2_NARROW if name == "ln2" else W_NARROW
        bands = [
            ("NARROW", 0, narrow_max, None),
            ("BASE", 0, base_max, BASE_CFG),
            ("XW", base_max + 1, xw_max, XW_CFG),
            ("XXW", xw_max + 1, xxw_max, XXW_CFG),
        ]
        for band, lo, hi, cfg in bands:
            if cfg is not None:
                w(f"#[cfg({cfg})]")
            w(f"static {upper}_{band}: &[Entry] = &[")
            out.extend(emit_entries(value, lo, hi))
            w("];")
            w("")

    # ── Per-constant `const fn` lookups, band-gated by `#[cfg]` on the
    # statements so a disabled band's static is never referenced. ───────
    for name, _ in CONSTS:
        upper = name.upper()
        base_max, xw_max, _ = CONST_CLASS[name]
        w("/// `floor(%s * 10^scale)` limbs (little-endian, narrowest-fit)" % name)
        w("/// plus the round-up bit, for the const working `scale`.")
        w("///")
        w("/// `const fn` so a caller keyed on the const-generic SCALE folds")
        w("/// to the single matching entry per monomorphisation — no runtime")
        w("/// search on the hot path.")
        w(f"pub(crate) const fn {name}_entry(scale: u32) -> (&'static [u64], u8) {{")
        w("    // NARROW band (0..=%d) is always present — the public" % W_NARROW)
        w("    // `DecimalConstants` trait on D18/D38 reads it in every build,")
        w("    // including default / no_std (no `_wide-support`).")
        w(f"    if (scale as usize) < {upper}_NARROW.len() {{")
        w(f"        let e = {upper}_NARROW[scale as usize];")
        w("        return (e.1, e.2);")
        w("    }")
        w(f"    #[cfg({BASE_CFG})]")
        w("    {")
        w(f"        if (scale as usize) < {upper}_BASE.len() {{")
        w(f"            let e = {upper}_BASE[scale as usize];")
        w("            return (e.1, e.2);")
        w("        }")
        w("    }")
        w(f"    #[cfg({XW_CFG})]")
        w("    {")
        w(f"        let base_lo = {base_max} + 1;")
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
        w(f"        let xw_lo = {xw_max} + 1;")
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
    w("/// Like [`round_entry`], but returns `None` when the value does")
    w("/// not fit the SIGNED positive range of the work/storage integer")
    w("/// `W` (i.e. it would exceed `Int::<W::LIMBS>::MAX`). Used by the")
    w("/// PUBLIC constant accessors, where a constant requested at a")
    w("/// scale too large for the type's storage must surface an overflow")
    w("/// (the caller panics with an \"out of storage range\" message),")
    w("/// not silently wrap. The constants are positive and the limbs are")
    w("/// narrowest-fit, so the fit test is purely structural:")
    w("///")
    w("///   * more limbs than `W` holds            -> overflow;")
    w("///   * exactly `W::LIMBS` limbs and the top limb has its high bit")
    w("///     set (>= 2^63) -> the magnitude reaches into `W`'s sign bit")
    w("///     -> overflow (the `+1` round-up bump cannot clear an already-")
    w("///     set top bit, so no false negative);")
    w("///   * otherwise it fits, and the rounded fold is exact.")
    w("///")
    w("/// The INTERNAL kernel path (`*_by_scale` / `*_by_working_scale`) does NOT use")
    w("/// this — it folds into a wide WORK integer where the value always")
    w("/// fits and must never panic.")
    w("#[inline]")
    w("fn round_entry_checked<W: BigInt>(")
    w("    limbs: &[u64],")
    w("    round_up: u8,")
    w("    mode: RoundingMode,")
    w(") -> Option<W> {")
    w("    let n = W::LIMBS;")
    w("    if limbs.len() > n {")
    w("        return None;")
    w("    }")
    w("    if limbs.len() == n && (limbs[n - 1] & 0x8000_0000_0000_0000) != 0 {")
    w("        return None;")
    w("    }")
    w("    Some(round_entry::<W>(limbs, round_up, mode))")
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
    #   *_by_working_scale — a plain `fn` keyed on the RUNTIME working scale `w`.
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
        w(f"pub(crate) fn {name}_by_working_scale<W: BigInt>(w: u32, mode: RoundingMode) -> W {{")
        w(f"    let (limbs, round_up) = {name}_entry(w);")
        w("    round_entry::<W>(limbs, round_up, mode)")
        w("}")
        w("")
        # A storage-RANGE-CHECKED accessor for the constants whose
        # magnitude can exceed a type's storage at the type's top scale
        # (deg_per_rad ~ 57.3). The public `DecimalConstants` impls use
        # this so an out-of-range request PANICS (via the caller) rather
        # than silently folding a wrapped value, matching every other
        # constant. (rad_per_deg ~ 0.0175 never overflows but gets the
        # symmetric accessor for consistency.)
        # Every PUBLIC DecimalConstants constant gets *_by_scale_checked: the
        # single unified impl range-checks ALL of them (pi/tau/e overflow storage
        # past the tier's top scale too, not just deg_per_rad). ln2/ln10 are
        # internal-only (no public constant), so they skip the checked accessor.
        if name not in ("ln2", "ln10"):
            w(f"/// `{name}` at the CONST working `scale` as in [`{name}_by_scale`],")
            w("/// but returns `None` when the value does not fit the SIGNED")
            w("/// storage range of `W` (see [`round_entry_checked`]). Used by the")
            w("/// PUBLIC `DecimalConstants` impls so an over-range request panics")
            w("/// rather than silently wrapping; NOT for the internal kernel path.")
            w("#[inline]")
            w(f"pub(crate) fn {name}_by_scale_checked<W: BigInt>(")
            w("    scale: u32,")
            w("    mode: RoundingMode,")
            w(") -> Option<W> {")
            w(f"    let (limbs, round_up) = {name}_entry(scale);")
            w("    round_entry_checked::<W>(limbs, round_up, mode)")
            w("}")
            w("")

    # ── Strong-fold const-fn API: bake a constant into `Int<N>` at a concrete
    # `N` in a const-block — a GUARANTEED compile-time fold (not optimizer-
    # dependent like the generic `*_by_scale`). The DecimalConstants impls and
    # the trig `PI_RAW` const use these.
    w("/// Zero-extends a narrow little-endian limb slice into `Int<N>` as a")
    w("/// `const fn` — the strong-fold primitive. Builds `[0u64; N]` (plain")
    w("/// const-generic `N`, no `generic_const_exprs`) + `Int::from_limbs`, so a")
    w("/// caller can bake the value in a const-block at a concrete `N`.")
    w("const fn limbs_to_int_n<const N: usize>(limbs: &[u64]) -> crate::int::types::Int<N> {")
    w("    let mut arr = [0u64; N];")
    w("    let mut i = 0;")
    w("    while i < limbs.len() {")
    w("        arr[i] = limbs[i];")
    w("        i += 1;")
    w("    }")
    w("    crate::int::types::Int::<N>::from_limbs(arr)")
    w("}")
    w("")
    for name, _ in CONSTS:
        w(f"/// `{name}` at the CONST `scale` as a compile-time-baked `Int<N>`")
        w("/// (strong fold). Evaluate it in a const-block at a concrete-`N` leaf:")
        w(f"/// the `{name}_entry` lookup folds, `limbs_to_int_n` bakes the zero-")
        w("/// extend, and the `+1` bump folds when `mode` is const. Runtime / Ziv")
        w(f"/// path: [`{name}_by_working_scale`].")
        w(f"pub(crate) const fn {name}_const_n<const N: usize>(")
        w("    scale: u32,")
        w("    mode: RoundingMode,")
        w(") -> crate::int::types::Int<N> {")
        w(f"    let (limbs, round_up) = {name}_entry(scale);")
        w("    let floor = limbs_to_int_n::<N>(limbs);")
        w("    let bump = match mode {")
        w("        RoundingMode::Trunc | RoundingMode::Floor => false,")
        w("        RoundingMode::Ceiling => true,")
        w("        RoundingMode::HalfToEven")
        w("        | RoundingMode::HalfAwayFromZero")
        w("        | RoundingMode::HalfTowardZero => round_up != 0,")
        w("    };")
        w("    if bump {")
        w("        floor.wrapping_add(crate::int::types::Int::<N>::ONE)")
        w("    } else {")
        w("        floor")
        w("    }")
        w("}")
        w("")

    # `PI_RAW_D76_S75`: pi rounded half-to-even to 75 frac digits as an `Int<4>`,
    # emitted UNGATED. The narrow trig series kernel (D38, always compiled)
    # consumes it, but scale 75 lives in the `_wide-support`-gated BASE band — so
    # this single value is ungated here (replaces `from_str_radix(PI_D76_S75)`).
    _q75, _rb75 = floor_and_roundbit(mp.pi, 75)
    _pi75_limbs = ", ".join(f"0x{l:016x}" for l in limbs_le(_q75 + _rb75))
    w("/// `pi` rounded half-to-even to 75 fractional digits as `Int<4>`")
    w("/// (`round(pi * 10^75)`), UNGATED — the narrow trig kernel needs it in")
    w("/// every build (scale 75 sits in the `_wide-support`-gated BASE band, so it")
    w("/// cannot read the per-scale table there). Replaces the old build-time")
    w("/// `from_str_radix(PI_D76_S75)`.")
    w(f"pub(crate) const PI_RAW_D76_S75: crate::int::types::Int<4> = limbs_to_int_n::<4>(&[{_pi75_limbs}]);")
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
    w("    /// Uses an `Int<16>` work integer, which only exists in a")
    w('    /// `_wide-support` build, so the test is gated to that build (a')
    w("    /// narrow-only build has no work integer this wide to exercise).")
    w('    #[cfg(feature = "_wide-support")]')
    w("    #[test]")
    w("    fn by_scale_eq_by_working_scale() {")
    w("        for s in [0u32, 1, 17, 18, 19, 30, 38, 86] {")
    w("            for m in [HalfToEven, Trunc, Ceiling, Floor, HalfAwayFromZero, HalfTowardZero] {")
    w("                assert_eq!(pi_by_scale::<Int<16>>(s, m), pi_by_working_scale::<Int<16>>(s, m));")
    w("                assert_eq!(ln2_by_scale::<Int<16>>(s, m), ln2_by_working_scale::<Int<16>>(s, m));")
    w("            }")
    w("        }")
    w("    }")
    w("")
    w("    /// Width-independence: the same scale gives the same value")
    w("    /// (zero-extended) in different work-int widths. Exercises")
    w("    /// `Int<16>` / `Int<32>` work integers (and `resize_to` between")
    w('    /// them), which only exist in a `_wide-support` build — so the')
    w("    /// test is gated there. The always-present narrow band is")
    w("    /// covered by `modes_derive_from_floor_and_roundbit` above.")
    w('    #[cfg(feature = "_wide-support")]')
    w("    #[test]")
    w("    fn value_is_width_independent() {")
    w("        for s in [0u32, 5, 17, 18, 30, 38, 50] {")
    w("            let a = pi_by_scale::<Int<16>>(s, HalfToEven);")
    w("            let b = pi_by_scale::<Int<32>>(s, HalfToEven);")
    w("            assert_eq!(a, b.resize_to::<Int<16>>());")
    w("        }")
    w("    }")
    w("}")
    w("")

    # ── POW10 lookup: exact 10^exp, narrowest-fit little-endian limbs,
    # width-generic (zero-extended into W). The wide tiers' `pow10` (the
    # no-const-table tiers D924/D1232, and the width-generic `exp_generic`)
    # read this as a static lookup instead of recomputing `10^exp` by repeated
    # squaring. EXACT (10^exp is an integer) -> no round-up bit. Bands mirror
    # ln2's (the deepest exp working-scale path); `pow10_in` falls back to a
    # runtime `TEN.pow` beyond the generated range.
    POW10_NARROW = 512
    POW10_BASE_MAX, POW10_XW_MAX, POW10_XXW_MAX = (1280, 2560, 5120)
    pow10_bands = [
        ("NARROW", 0, POW10_NARROW, None),
        ("BASE", 0, POW10_BASE_MAX, BASE_CFG),
        ("XW", POW10_BASE_MAX + 1, POW10_XW_MAX, XW_CFG),
        ("XXW", POW10_XW_MAX + 1, POW10_XXW_MAX, XXW_CFG),
    ]
    for band, lo, hi, cfg in pow10_bands:
        if cfg is not None:
            w(f"#[cfg({cfg})]")
        w(f"static POW10_{band}: &[&[u64]] = &[")
        for e in range(lo, hi + 1):
            limbs = limbs_le(10 ** e)
            limb_str = ", ".join(f"0x{l:016x}" for l in limbs)
            w(f"    &[{limb_str}],")
        w("];")
        w("")
    w("/// Limbs (little-endian) of `10^exp` if `exp` is within a generated POW10")
    w("/// band, else `None`. Bands are feature-gated; the always-present NARROW")
    w("/// band covers the default / no_std build. `const fn` so a const `exp`")
    w("/// folds to the matching entry.")
    w("#[inline]")
    w("const fn pow10_entry(exp: u32) -> Option<&'static [u64]> {")
    w("    if (exp as usize) < POW10_NARROW.len() {")
    w("        return Some(POW10_NARROW[exp as usize]);")
    w("    }")
    w(f"    #[cfg({BASE_CFG})]")
    w("    {")
    w("        if (exp as usize) < POW10_BASE.len() {")
    w("            return Some(POW10_BASE[exp as usize]);")
    w("        }")
    w("    }")
    w(f"    #[cfg({XW_CFG})]")
    w("    {")
    w(f"        let base_lo = {POW10_BASE_MAX} + 1;")
    w("        if exp >= base_lo {")
    w("            let idx = (exp - base_lo) as usize;")
    w("            if idx < POW10_XW.len() {")
    w("                return Some(POW10_XW[idx]);")
    w("            }")
    w("        }")
    w("    }")
    w(f"    #[cfg({XXW_CFG})]")
    w("    {")
    w(f"        let xw_lo = {POW10_XW_MAX} + 1;")
    w("        if exp >= xw_lo {")
    w("            let idx = (exp - xw_lo) as usize;")
    w("            if idx < POW10_XXW.len() {")
    w("                return Some(POW10_XXW[idx]);")
    w("            }")
    w("        }")
    w("    }")
    w("    None")
    w("}")
    w("")
    w("/// `10^exp` in the work integer `W`. A static lookup of the generated")
    w("/// POW10 table (zero-extended into `W`) when `exp` is in range; otherwise")
    w("/// a runtime `W::TEN.pow(exp)` recompute (the rare deep tail beyond the")
    w("/// table). Replaces the per-tier / exp_generic `TEN.pow` recompute on the")
    w("/// wide path.")
    w("#[inline]")
    w("pub(crate) fn pow10_in<W: BigInt>(exp: u32) -> W {")
    w("    match pow10_entry(exp) {")
    w("        Some(limbs) => limbs_to_w::<W>(limbs),")
    w("        None => W::TEN.pow(exp),")
    w("    }")
    w("}")
    w("")

    src = "\n".join(out) + "\n"
    path = "src/consts/table.rs"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    print(f"wrote {path} ({len(src)} bytes), mp.dps={mp.dps}, "
          f"{datetime.date.today()}")


if __name__ == "__main__":
    main()
