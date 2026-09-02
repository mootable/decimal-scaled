"""Generate the per-scale, oracle-sourced, width-deduplicated wide
transcendental constant table.

This is a ONE-OFF hand-run generator (a sibling of
`gen_golden_precision.py`). It is NOT run at build time: it emits a
committed Rust source file `src/consts/table.rs`, and that
output is what the crate compiles. `build.rs` is untouched.

Oracle. Every constant comes from `flint`/Arb through the shared
`tang_flint_oracle` module — the same determination routine the Tang
tables use. Each golden is `floor(value * 10**gp)` resolved by
`ball.floor().unique_fmpz()`, which yields a value ONLY when the
enclosing interval determines every retained digit. If it cannot, the
oracle escalates the working precision and recomputes; if it still
cannot, it raises rather than rounding past an undecided digit. So the
digits emitted below are not merely believed correct at some working
precision — the enclosure certifies them. No value is derived from the
crate's own `pi` or any `decimal_scaled` method.

The eleven constants (all positive, all irrational):
    pi, tau (= 2*pi), half_pi (= pi/2), quarter_pi (= pi/4),
    e, golden (= (1+sqrt 5)/2), ln2 (= log 2), ln10 (= log 10),
    log10_2 (= log 2 / log 10),
    deg_per_rad (one radian in degrees, = 180/pi),
    rad_per_deg (one degree in radians, = pi/180).

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

This reproduces, bit-for-bit, a CORRECT ROUNDING of the oracle value
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

from flint import arb

from tang_flint_oracle import decimal_prec_for, determine_decimal_floor

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
    "e": DEC_MAXES, "golden": DEC_MAXES, "log10_2": DEC_MAXES,
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
# pi / ln2 / ln10 get a WIDER ungated NARROW band than the other constants.
# The always-compiled narrow kernels (D18/D38 trig + ln/log/exp) read these
# three at a WORKING scale (`w = SCALE + GUARD`, up to ~68–75 digits, and ln2's
# exp range-reduction reaches `w_ext` ~165) and source them FROM THIS TABLE in
# EVERY build — including default / no_std, which has none of the gated wider
# bands. So the ungated NARROW band must itself cover the working scales those
# kernels request; 512 leaves ample margin and matches the gated BASE band's
# floor. (The other constants are read only at a const storage scale ≤ 38, so
# they keep the tight 0..=38 narrow band.)
WORKING_NARROW = 512

# Gate strings. The base band is needed by every wide-support build.
BASE_CFG = 'feature = "_wide-support"'
# The x-wide band (513..=1024) is reached by D462/D616 — and ALSO by the
# xx-wide-group tiers (D924/D1232): a band is a contiguous SCALE RANGE, and
# a wider tier's working scales pass THROUGH it on the way to its own cap
# (e.g. D1232 cosh's exp composition requests ln2 at a scale inside the
# x-wide range — a `--features d1232` single-tier build must compile this
# band or that lookup panics on the table hole instead of reaching the
# contractual overflow site). Gate every band by ALL tiers at or above it,
# never just its own feature group.
XW_CFG = (
    'any(feature = "d462", feature = "d616", feature = "d924", '
    'feature = "d1232", feature = "x-wide", feature = "xx-wide")'
)
# The xx-wide band (1025..=2048) is reached by D924/D1232 (or xx-wide) only —
# it is the TOP band, so no wider tier passes through it.
XXW_CFG = 'any(feature = "d924", feature = "d1232", feature = "xx-wide")'

# ── Oracle precision ──────────────────────────────────────────────────
# NOT a single global setting. Each golden picks its own starting working
# precision from the digit count it has to retain
# (`tang_flint_oracle.decimal_prec_for`), and the determination escalates
# from there if that proves tight. The widest golden here is ln2 at 5121
# digits.
#
# Each constant is a THUNK returning the value as an `arb` at the ambient
# working precision — not a precomputed value, because escalating the
# precision means recomputing the value, not re-reading an enclosure that
# was already fixed at the old precision.
CONSTS = [
    ("pi", lambda: arb.pi()),
    ("tau", lambda: arb.pi() * 2),
    ("half_pi", lambda: arb.pi() / 2),
    ("quarter_pi", lambda: arb.pi() / 4),
    ("e", lambda: arb(1).exp()),
    ("golden", lambda: (arb(5).sqrt() + 1) / 2),
    ("ln2", lambda: arb.const_log2()),
    ("ln10", lambda: arb(10).log()),
    # log10(2) = log(2)/log(10) ~ 0.30103 — the bit<->digit conversion
    # factor (a value's decimal-digit count ~ bit_len * log10(2)). Public
    # DecimalConstants value; sourced as a true oracle ratio.
    ("log10_2", lambda: arb.const_log2() / arb(10).log()),
    # one radian in degrees = 180/pi, sourced as a true oracle value.
    ("deg_per_rad", lambda: arb(180) / arb.pi()),
    # one degree in radians = pi/180.
    ("rad_per_deg", lambda: arb.pi() / 180),
]


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


def golden_limbs(make_val, gp, label):
    """Little-endian u64 limbs of `floor(value * 10**gp)` — the SINGLE golden
    mantissa a band downgrades from at compile time.

    Returns `(limbs, spare_bits, prec_used)`. The value is DETERMINED by the
    shared flint/Arb oracle: it is emitted only when the enclosure pins every
    one of the `gp` retained digits, and `spare_bits` records by what factor
    the enclosure could have been wider and still decided it.

    `gp = band_hi + 1`: one guard digit above the band's top scale, so the
    `const fn` builder recovers the top scale's round bit (the most-significant
    dropped digit) from this one value. Every lower scale `s` is exact:
    `floor(value*10^s) = floor(golden / 10^(gp-s))`, and its round bit is
    `digit_at(gp-1-s) >= 5` — provable because the constants are irrational
    (no exact tie). This is what shrinks `table.rs` from ~39 MB of per-scale
    literals to ~8 KB of goldens: the per-scale array is REBUILT at compile
    time, never shipped."""
    n, spare, prec = determine_decimal_floor(make_val, gp, label)
    return limbs_le(n), spare, prec


def emit_limb_literal(prefix, name, limbs):
    """Emit `const {name}: &[u64] = &[ ... ];`, 6 limbs per line."""
    lines = [f"{prefix}const {name}: &[u64] = &["]
    for i in range(0, len(limbs), 6):
        lines.append("    " + ", ".join(f"0x{l:016x}" for l in limbs[i:i + 6]) + ",")
    lines.append("];")
    return lines


def main():
    out = []
    emit = out.append
    # One row per emitted golden: (label, digits, headroom bits, precision
    # used). Reported at the end so a tightening margin or a precision
    # escalation is visible in the run, not buried.
    determined = []

    emit("// SPDX-FileCopyrightText: 2026 John Moxley")
    emit("// SPDX-License-Identifier: MIT OR Apache-2.0")
    emit("")
    emit("//! Per-scale, oracle-sourced, width-deduplicated wide")
    emit("//! transcendental constant table.")
    emit("//!")
    emit("//! GENERATED by `scripts/gen_const_table.py` from a flint/Arb")
    emit("//! oracle: every golden mantissa below is `floor(const * 10^gp)`")
    emit("//! resolved by `ball.floor().unique_fmpz()`, which yields a value")
    emit("//! ONLY when the enclosing interval determines every retained")
    emit("//! digit — so these digits are certified, not assumed correct at")
    emit("//! some working precision. Do NOT edit by hand; re-run the script")
    emit("//! and commit its output. This file is NOT produced at build time")
    emit("//! — `build.rs` is untouched.")
    emit("//!")
    emit("//! Each constant ships ONE golden mantissa per band —")
    emit("//! `floor(const * 10^(band_hi+1))` — and a `const fn` ([`cb_build`])")
    emit("//! REBUILDS the per-scale table at compile time by dividing it down by")
    emit("//! 10 one scale at a time. So the source carries ~hundreds of bytes per")
    emit("//! constant, not the per-scale limbs, yet the compiled table (and the")
    emit("//! runtime static lookup) is byte-identical to a literal one. Each")
    emit("//! rebuilt entry is `floor(const * 10^scale)` as the narrowest-fit")
    emit("//! little-endian `u64` slice plus a `round_up` bit = 1 iff the dropped")
    emit("//! fractional tail is >= 1/2 (the most-significant dropped digit >= 5).")
    emit("//! Every constant is irrational and positive, so the tail is never an")
    emit("//! exact tie and never zero, and the six rounding modes derive exactly")
    emit("//! from `(floor, round_up)`:")
    emit("//!")
    emit("//! | mode | result |")
    emit("//! |------|--------|")
    emit("//! | Trunc / Floor | `floor` |")
    emit("//! | Ceiling / AwayFromZero | `floor + 1` |")
    emit("//! | ZeroFiveUp | `floor + 1` iff `floor`'s last digit is 0 or 5 |")
    emit("//! | HalfToEven / HalfAwayFromZero / HalfTowardZero | `floor + round_up` |")
    emit("//!")
    emit("//! The value is width-independent: an accessor zero-extends the")
    emit("//! stored limbs into the caller's work integer (the high limbs stay")
    emit("//! zero). The narrow band (`*_NARROW`, scales 0..=%d) is ALWAYS" % W_NARROW)
    emit("//! present — the public `DecimalConstants` trait on D18/D38 reads it")
    emit("//! in every build (default / no_std included). The three wider bands")
    emit("//! are feature-gated to match the tiers that can request them")
    emit("//! (mirrors `src/types/consts/wide.rs`).")
    emit("")
    # ── Compile-time table builder. The per-scale table is REBUILT from one
    # golden mantissa per band by a `const fn` divide-down — source ships the
    # golden (~hundreds of bytes), not ~39 MB of per-scale literals, while the
    # COMPILED table (and runtime: a static lookup) stays byte-identical.
    builder = r'''/// Scratch limb cap for the const-fn builders: the widest golden is
/// ln2 @ scale 5121 (~266 limbs); 280 leaves margin. A const-eval local
/// only — never in the binary.
const MAXK: usize = 280;

/// A compile-time-built per-scale constant band. `flat` holds every scale's
/// `floor(const * 10^scale)` as narrowest-fit little-endian limbs, packed
/// back-to-back; entry `i` (scale `lo + i`) is `flat[off[i] .. off[i]+len[i]]`
/// with round-up bit `bit[i]`. Built by [`cb_build`] from one golden mantissa.
pub(crate) struct ConstBand<const F: usize, const N: usize> {
    flat: [u64; F],
    off: [u32; N],
    len: [u16; N],
    bit: [u8; N],
}

/// Total narrowest-fit limb count over scales `lo..=hi`, divided down from
/// `golden = floor(const * 10^(hi+1))`. Sizes [`ConstBand`]'s `flat` exactly.
const fn cb_flat_len(golden: &[u64], lo: u32, hi: u32) -> usize {
    let mut buf = [0u64; MAXK];
    let mut blen = golden.len();
    let mut i = 0;
    while i < blen {
        buf[i] = golden[i];
        i += 1;
    }
    let mut total = 0usize;
    let mut scale = hi + 1;
    while scale > lo {
        let mut rem: u64 = 0;
        let mut j = blen;
        while j > 0 {
            j -= 1;
            let c = ((rem as u128) << 64) | (buf[j] as u128);
            buf[j] = (c / 10) as u64;
            rem = (c % 10) as u64;
        }
        while blen > 1 && buf[blen - 1] == 0 {
            blen -= 1;
        }
        total += blen;
        scale -= 1;
    }
    total
}

/// Build the band for scales `lo..=hi` by dividing `golden = floor(const *
/// 10^(hi+1))` down by 10 once per scale. Each division's remainder is the
/// most-significant dropped digit, so the round bit is `rem >= 5` — exact for
/// irrational constants (no tie); `floor(const*10^s) = floor(golden/10^(hi+1-s))`
/// is exact. Reproduces the former literal table bit-for-bit.
const fn cb_build<const F: usize, const N: usize>(
    golden: &[u64],
    lo: u32,
    hi: u32,
) -> ConstBand<F, N> {
    let mut b = ConstBand { flat: [0u64; F], off: [0u32; N], len: [0u16; N], bit: [0u8; N] };
    let mut buf = [0u64; MAXK];
    let mut blen = golden.len();
    let mut i = 0;
    while i < blen {
        buf[i] = golden[i];
        i += 1;
    }
    let mut scale = hi + 1;
    let mut cursor = 0usize;
    while scale > lo {
        let mut rem: u64 = 0;
        let mut j = blen;
        while j > 0 {
            j -= 1;
            let c = ((rem as u128) << 64) | (buf[j] as u128);
            buf[j] = (c / 10) as u64;
            rem = (c % 10) as u64;
        }
        while blen > 1 && buf[blen - 1] == 0 {
            blen -= 1;
        }
        let idx = (scale - 1 - lo) as usize;
        b.off[idx] = cursor as u32;
        b.len[idx] = blen as u16;
        b.bit[idx] = if rem >= 5 { 1 } else { 0 };
        let mut k = 0;
        while k < blen {
            b.flat[cursor + k] = buf[k];
            k += 1;
        }
        cursor += blen;
        scale -= 1;
    }
    b
}

/// The stored `(floor-limbs, round-up)` for entry `i` of a band — a `const fn`
/// so a const-scale caller folds to the one entry (and bakes through `const_n`).
/// `split_at` keeps it const-stable (range indexing is not yet const).
const fn cb_get<const F: usize, const N: usize>(
    b: &'static ConstBand<F, N>,
    i: usize,
) -> (&'static [u64], u8) {
    let o = b.off[i] as usize;
    let l = b.len[i] as usize;
    (b.flat.split_at(o).1.split_at(l).0, b.bit[i])
}
'''
    for ln in builder.splitlines():
        emit(ln)
    emit("")

    # Per constant, per band: ONE golden mantissa + the compile-time-built band.
    # The NARROW band (always present) feeds D18/D38 + the always-compiled narrow
    # kernels; BASE/XW/XXW are feature-gated to the tiers that reach them. Each
    # constant's band maxes follow its CLASS (ZIV / HOT / DEC — see CONST_CLASS).
    for name, getter in CONSTS:
        upper = name.upper()
        base_max, xw_max, xxw_max = CONST_CLASS[name]
        narrow_max = WORKING_NARROW if name in ("pi", "ln2", "ln10") else W_NARROW
        bands = [
            ("NARROW", 0, narrow_max, None),
            ("BASE", 0, base_max, BASE_CFG),
            ("XW", base_max + 1, xw_max, XW_CFG),
            ("XXW", xw_max + 1, xxw_max, XXW_CFG),
        ]
        for band, lo, hi, cfg in bands:
            n = hi - lo + 1
            golden_le, spare, prec = golden_limbs(
                getter, hi + 1, f"{name} {band}"
            )
            determined.append((f"{name} {band}", hi + 1, spare, prec))
            cfg_attr = f"#[cfg({cfg})]" if cfg is not None else None
            if cfg_attr:
                emit(cfg_attr)
            for ln in emit_limb_literal("", f"{upper}_{band}_GOLDEN", golden_le):
                emit(ln)
            if cfg_attr:
                emit(cfg_attr)
            emit(f"const {upper}_{band}_F: usize = "
                 f"cb_flat_len({upper}_{band}_GOLDEN, {lo}, {hi});")
            if cfg_attr:
                emit(cfg_attr)
            emit(f"static {upper}_{band}: ConstBand<{upper}_{band}_F, {n}> = "
                 f"cb_build({upper}_{band}_GOLDEN, {lo}, {hi});")
            emit("")

    # ── Per-constant `const fn` lookups, band-gated by `#[cfg]` on the
    # statements so a disabled band's static is never referenced. ───────
    for name, _ in CONSTS:
        upper = name.upper()
        base_max, xw_max, xxw_max = CONST_CLASS[name]
        narrow_max = WORKING_NARROW if name in ("pi", "ln2", "ln10") else W_NARROW
        # Per-band entry counts (= the static's `N`), so `entry` compares against
        # a literal instead of `.len()` and indexes the built band via `cb_get`.
        narrow_n = narrow_max + 1
        base_n = base_max + 1
        xw_n = xw_max - base_max
        xxw_n = xxw_max - xw_max
        emit("/// `floor(%s * 10^scale)` limbs (little-endian, narrowest-fit)" % name)
        emit("/// plus the round-up bit, for the const working `scale`.")
        emit("///")
        emit("/// `const fn` so a caller keyed on the const-generic SCALE folds")
        emit("/// to the single matching entry per monomorphisation — no runtime")
        emit("/// search on the hot path. The band's per-scale `(limbs, round_up)`")
        emit("/// is rebuilt at compile time from one golden mantissa ([`cb_build`]).")
        emit(f"pub(crate) const fn {name}_entry(scale: u32) -> (&'static [u64], u8) {{")
        emit("    // NARROW band (0..=%d) is always present — the public" % W_NARROW)
        emit("    // `DecimalConstants` trait on D18/D38 reads it in every build,")
        emit("    // including default / no_std (no `_wide-support`).")
        emit(f"    if (scale as usize) < {narrow_n} {{")
        emit(f"        return cb_get(&{upper}_NARROW, scale as usize);")
        emit("    }")
        emit(f"    #[cfg({BASE_CFG})]")
        emit("    {")
        emit(f"        if (scale as usize) < {base_n} {{")
        emit(f"            return cb_get(&{upper}_BASE, scale as usize);")
        emit("        }")
        emit("    }")
        emit(f"    #[cfg({XW_CFG})]")
        emit("    {")
        emit(f"        let base_lo = {base_max} + 1;")
        emit("        if scale >= base_lo {")
        emit("            let idx = (scale - base_lo) as usize;")
        emit(f"            if idx < {xw_n} {{")
        emit(f"                return cb_get(&{upper}_XW, idx);")
        emit("            }")
        emit("        }")
        emit("    }")
        emit(f"    #[cfg({XXW_CFG})]")
        emit("    {")
        emit(f"        let xw_lo = {xw_max} + 1;")
        emit("        if scale >= xw_lo {")
        emit("            let idx = (scale - xw_lo) as usize;")
        emit(f"            if idx < {xxw_n} {{")
        emit(f"                return cb_get(&{upper}_XXW, idx);")
        emit("            }")
        emit("        }")
        emit("    }")
        emit(f'    panic!("const_table: {name} scale out of generated range");')
        emit("}")
        emit("")

    # ── Width-generic accessor: zero-extend the stored limbs into W. ────
    emit("use crate::int::types::traits::BigInt;")
    emit("use crate::support::rounding::RoundingMode;")
    emit("")
    emit("/// Builds the work integer `W` holding `floor(const * 10^scale)`")
    emit("/// from a narrow little-endian `limbs` slice by DIRECT limb injection:")
    emit("/// a little-endian u64 limb slice IS the value (`Σ limbs[i]·2^(64·i)`),")
    emit("/// so the low `min(len, W::LIMBS)` limbs are copied and the rest stay")
    emit("/// zero — bit-identical to the previous high-to-low Horner fold")
    emit("/// (`acc = (acc << 64) | limb`), which kept the same low limbs but paid")
    emit("/// a full-`W`-width shift + OR + rebuild PER SOURCE LIMB (O(len · W)")
    emit("/// instead of O(len)). The fold was a measured hot frame in the wide")
    emit("/// `ln`/`exp` shells, where every `pow10`/`ln2`/`pi` lookup runs this on")
    emit("/// a multi-limb entry at a 32-limb-plus work integer.")
    emit("#[inline]")
    emit("pub(crate) fn limbs_to_w<W: BigInt>(limbs: &[u64]) -> W {")
    emit("    W::from_mag_sign_u64(limbs, false)")
    emit("}")
    emit("")
    emit("/// Applies `mode` to a `(floor-limbs, round_up)` table entry,")
    emit("/// returning the correctly-rounded constant in the work integer `W`.")
    emit("///")
    emit("/// The constants are irrational + positive, so the dropped tail is")
    emit("/// never an exact tie and never zero. Hence: Trunc / Floor keep the")
    emit("/// floor; Ceiling and AwayFromZero always bump (`+1`); ZeroFiveUp")
    emit("/// bumps only when the floor's last decimal digit is `0` or `5`; the")
    emit("/// three half-modes all reduce to round-to-nearest = `floor +")
    emit("/// round_up`. This reproduces a correct rounding of the oracle value")
    emit("/// under every mode.")
    emit("#[inline]")
    emit("fn round_entry<W: BigInt>(limbs: &[u64], round_up: u8, mode: RoundingMode) -> W {")
    emit("    let floor = limbs_to_w::<W>(limbs);")
    emit("    let bump = match mode {")
    emit("        RoundingMode::Trunc | RoundingMode::Floor => false,")
    emit("        RoundingMode::Ceiling => true,")
    emit("        RoundingMode::HalfToEven")
    emit("        | RoundingMode::HalfAwayFromZero")
    emit("        | RoundingMode::HalfTowardZero => round_up != 0,")
    emit("        // The tail is never zero, and the value is positive, so")
    emit("        // away-from-zero always lifts — as Ceiling does.")
    emit("        RoundingMode::AwayFromZero => true,")
    emit("        // `limbs` IS the floor value (zero-extended, never truncated),")
    emit("        // so its last decimal digit is the pivot.")
    emit("        RoundingMode::ZeroFiveUp => {")
    emit("            matches!(crate::support::rounding::limbs_mod_10(limbs), 0 | 5)")
    emit("        }")
    emit("    };")
    emit("    if bump { floor.wrapping_add(W::ONE) } else { floor }")
    emit("}")
    emit("")
    emit("/// Like [`round_entry`], but returns `None` when the value does")
    emit("/// not fit the SIGNED positive range of the work/storage integer")
    emit("/// `W` (i.e. it would exceed `Int::<W::LIMBS>::MAX`). Used by the")
    emit("/// PUBLIC constant accessors, where a constant requested at a")
    emit("/// scale too large for the type's storage must surface an overflow")
    emit("/// (the caller panics with an \"out of storage range\" message),")
    emit("/// not silently wrap. The constants are positive and the limbs are")
    emit("/// narrowest-fit, so the fit test is purely structural:")
    emit("///")
    emit("///   * more limbs than `W` holds            -> overflow;")
    emit("///   * exactly `W::LIMBS` limbs and the top limb has its high bit")
    emit("///     set (>= 2^63) -> the magnitude reaches into `W`'s sign bit")
    emit("///     -> overflow (the `+1` round-up bump cannot clear an already-")
    emit("///     set top bit, so no false negative);")
    emit("///   * otherwise it fits, and the rounded fold is exact.")
    emit("///")
    emit("/// The INTERNAL kernel path (`*_by_scale` / `*_by_working_scale`) does NOT use")
    emit("/// this — it folds into a wide WORK integer where the value always")
    emit("/// fits and must never panic.")
    emit("#[inline]")
    emit("fn round_entry_checked<W: BigInt>(")
    emit("    limbs: &[u64],")
    emit("    round_up: u8,")
    emit("    mode: RoundingMode,")
    emit(") -> Option<W> {")
    emit("    let n = W::LIMBS;")
    emit("    if limbs.len() > n {")
    emit("        return None;")
    emit("    }")
    emit("    if limbs.len() == n && (limbs[n - 1] & 0x8000_0000_0000_0000) != 0 {")
    emit("        return None;")
    emit("    }")
    emit("    Some(round_entry::<W>(limbs, round_up, mode))")
    emit("}")
    emit("")

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
        emit(f"/// `{name}` at the CONST working `scale`, correctly rounded")
        emit("/// under `mode`, in the work integer `W`. **The norm.**")
        emit("///")
        emit("/// The scale -> entry lookup ([`%s_entry`]) is a `const fn`; when" % name)
        emit("/// `scale` is the const-generic working scale of the")
        emit("/// monomorphisation it folds to the one matching `(&'static")
        emit("/// [u64], round_up)`, so the only runtime work is the cheap")
        emit("/// fixed-count zero-extend into `W` (which LLVM further folds")
        emit("/// against the const limbs). No runtime search, no divide. The")
        emit("/// hot (non-Ziv) path uses this.")
        emit("#[inline]")
        emit(f"pub(crate) fn {name}_by_scale<W: BigInt>(scale: u32, mode: RoundingMode) -> W {{")
        emit(f"    let (limbs, round_up) = {name}_entry(scale);")
        emit("    round_entry::<W>(limbs, round_up, mode)")
        emit("}")
        emit("")
        emit(f"/// `{name}` at a RUNTIME `working_scale`, correctly rounded")
        emit("/// under `mode`. **Rare fallback** — the Ziv-escalation path")
        emit("/// (`working_scale != SCALE + GUARD`) where the const scale is")
        emit("/// not available. Does NOT const-fold; every avoidable call is a")
        emit(f"/// const-fold miss. Prefer [`{name}_by_scale`] when a const SCALE")
        emit("/// is in hand.")
        emit("#[inline]")
        emit(f"pub(crate) fn {name}_by_working_scale<W: BigInt>(")
        emit("    working_scale: u32,")
        emit("    mode: RoundingMode,")
        emit(") -> W {")
        emit(f"    let (limbs, round_up) = {name}_entry(working_scale);")
        emit("    round_entry::<W>(limbs, round_up, mode)")
        emit("}")
        emit("")
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
            emit(f"/// `{name}` at the CONST working `scale` as in [`{name}_by_scale`],")
            emit("/// but returns `None` when the value does not fit the SIGNED")
            emit("/// storage range of `W` (see [`round_entry_checked`]). Used by the")
            emit("/// PUBLIC `DecimalConstants` impls so an over-range request panics")
            emit("/// rather than silently wrapping; NOT for the internal kernel path.")
            emit("#[inline]")
            emit(f"pub(crate) fn {name}_by_scale_checked<W: BigInt>(")
            emit("    scale: u32,")
            emit("    mode: RoundingMode,")
            emit(") -> Option<W> {")
            emit(f"    let (limbs, round_up) = {name}_entry(scale);")
            emit("    round_entry_checked::<W>(limbs, round_up, mode)")
            emit("}")
            emit("")

    # ── Strong-fold const-fn API: bake a constant into `Int<N>` at a concrete
    # `N` in a const-block — a GUARANTEED compile-time fold (not optimizer-
    # dependent like the generic `*_by_scale`). The DecimalConstants impls and
    # the trig `PI_RAW` const use these.
    emit("/// Zero-extends a narrow little-endian limb slice into `Int<N>` as a")
    emit("/// `const fn` — the strong-fold primitive. Builds `[0u64; N]` (plain")
    emit("/// const-generic `N`, no `generic_const_exprs`) + `Int::from_limbs`, so a")
    emit("/// caller can bake the value in a const-block at a concrete `N`.")
    emit("const fn limbs_to_int_n<const N: usize>(limbs: &[u64]) -> crate::int::types::Int<N> {")
    emit("    let mut arr = [0u64; N];")
    emit("    let mut i = 0;")
    emit("    while i < limbs.len() {")
    emit("        arr[i] = limbs[i];")
    emit("        i += 1;")
    emit("    }")
    emit("    crate::int::types::Int::<N>::from_limbs(arr)")
    emit("}")
    emit("")
    for name, _ in CONSTS:
        emit(f"/// `{name}` at the CONST `scale` as a compile-time-baked `Int<N>`")
        emit("/// (strong fold). Evaluate it in a const-block at a concrete-`N` leaf:")
        emit(f"/// the `{name}_entry` lookup folds, `limbs_to_int_n` bakes the zero-")
        emit("/// extend, and the `+1` bump folds when `mode` is const. Runtime / Ziv")
        emit(f"/// path: [`{name}_by_working_scale`].")
        emit(f"pub(crate) const fn {name}_const_n<const N: usize>(")
        emit("    scale: u32,")
        emit("    mode: RoundingMode,")
        emit(") -> crate::int::types::Int<N> {")
        emit(f"    let (limbs, round_up) = {name}_entry(scale);")
        emit("    let floor = limbs_to_int_n::<N>(limbs);")
        emit("    let bump = match mode {")
        emit("        RoundingMode::Trunc | RoundingMode::Floor => false,")
        emit("        RoundingMode::Ceiling => true,")
        emit("        RoundingMode::HalfToEven")
        emit("        | RoundingMode::HalfAwayFromZero")
        emit("        | RoundingMode::HalfTowardZero => round_up != 0,")
        emit("        // The tail is never zero, and the value is positive, so")
        emit("        // away-from-zero always lifts — as Ceiling does.")
        emit("        RoundingMode::AwayFromZero => true,")
        emit("        // `limbs` IS the floor value (zero-extended, never truncated),")
        emit("        // so its last decimal digit is the pivot.")
        emit("        RoundingMode::ZeroFiveUp => {")
        emit("            matches!(crate::support::rounding::limbs_mod_10(limbs), 0 | 5)")
        emit("        }")
        emit("    };")
        emit("    if bump {")
        emit("        floor.wrapping_add(crate::int::types::Int::<N>::ONE)")
        emit("    } else {")
        emit("        floor")
        emit("    }")
        emit("}")
        emit("")

    # (The old ungated single-scale raws PI_RAW_D76_S75 / LN2_RAW_D76_S75 /
    # LN10_RAW_D76_S75 are GONE: the narrow kernels now read pi / ln2 / ln10 at
    # any working scale directly from the per-scale table — its ungated NARROW
    # band covers 0..=512 for those three, so the single-value raws are no longer
    # needed in any build.)

    # ── Self-test: re-derive the six modes from (floor, round_up) and
    # assert against a handful of independently-spelled known values. ───
    emit("#[cfg(test)]")
    emit("mod tests {")
    emit("    use super::*;")
    emit("    use crate::int::types::Int;")
    emit("    use crate::support::rounding::RoundingMode::*;")
    emit("")
    emit("    /// All eight modes derive correctly from the stored (floor,")
    emit("    /// round_up) pair: Trunc = Floor = floor; Ceiling and AwayFromZero")
    emit("    /// bump (the dropped tail is never zero for an irrational);")
    emit("    /// ZeroFiveUp bumps only when the floor's last digit is 0 or 5; the")
    emit("    /// three half-modes coincide at floor + round_up (no ties for")
    emit("    /// irrationals). Checked against `pi` at two scales.")
    emit("    ///")
    emit("    /// THIS is the test that pins VALUES, so it is the one that catches")
    emit("    /// a wrong arm in [`round_entry`]. Its sibling")
    emit("    /// [`by_scale_eq_by_working_scale`] proves only that the two")
    emit("    /// accessors AGREE, and both route through that same `round_entry`,")
    emit("    /// so it holds by construction and would still pass if every arm")
    emit("    /// were identically wrong. A new `RoundingMode` variant needs an")
    emit("    /// expected value HERE; adding it to the sibling's list is not")
    emit("    /// coverage of the mode.")
    emit("    ///")
    emit("    /// The expected values below were derived with Python's `decimal`")
    emit("    /// (ROUND_DOWN / ROUND_FLOOR / ROUND_CEILING / ROUND_HALF_EVEN /")
    emit("    /// ROUND_HALF_UP / ROUND_HALF_DOWN / ROUND_UP / ROUND_05UP) — an")
    emit("    /// independent implementation of the same General Decimal")
    emit("    /// Arithmetic rounding rules — applied to a flint/Arb-determined")
    emit("    /// `pi`, rather than reasoned out from the arms they check.")
    emit("    #[test]")
    emit("    fn modes_derive_from_floor_and_roundbit() {")
    emit("        // pi = 3.14159265358979323846...; at scale 4 -> 31415.9..,")
    emit("        // floor 31415, tail .9 >= .5 -> round_up = 1.")
    emit("        let f: Int<3> = limbs_to_w(&[31415]);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, Trunc), f);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, Floor), f);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, Ceiling), f.wrapping_add(Int::<3>::ONE));")
    emit("        let up = f.wrapping_add(Int::<3>::ONE);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, HalfToEven), up);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, HalfAwayFromZero), up);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, HalfTowardZero), up);")
    emit("        // AwayFromZero and ZeroFiveUp coincide here: `decimal`")
    emit("        // ROUND_UP and ROUND_05UP both give 31416. The tail is")
    emit("        // non-zero so away-from-zero lifts, and the floor's last")
    emit("        // digit (5) IS a ZeroFiveUp pivot.")
    emit("        let away4: Int<3> = limbs_to_w(&[31416]);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, AwayFromZero), away4);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(4, ZeroFiveUp), away4);")
    emit("        // scale 5 -> 314159.26.., floor 314159, tail .26 < .5 ->")
    emit("        // round_up = 0; all three half-modes keep the floor.")
    emit("        let f5: Int<3> = limbs_to_w(&[314159]);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(5, HalfToEven), f5);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(5, Ceiling), f5.wrapping_add(Int::<3>::ONE));")
    emit("        // At scale 5 the two new modes PART COMPANY, which is what")
    emit("        // makes this pair worth pinning: `decimal` ROUND_UP gives")
    emit("        // 314160, ROUND_05UP gives 314159, because the floor's last")
    emit("        // digit (9) is not a pivot so ZeroFiveUp keeps the floor.")
    emit("        let away5: Int<3> = limbs_to_w(&[314160]);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(5, AwayFromZero), away5);")
    emit("        assert_eq!(pi_by_scale::<Int<3>>(5, ZeroFiveUp), f5);")
    emit("    }")
    emit("")
    emit("    /// `by_scale` and `by_working_scale` return identical values for the same")
    emit("    /// scale (they differ only in const-fold behaviour, not value).")
    emit("    ///")
    emit("    /// The mode list is EVERY `RoundingMode` variant, deliberately. Both")
    emit("    /// sides route through the same `*_entry` + `round_entry`, so the")
    emit("    /// equality holds for any mode by construction — a new variant needs")
    emit("    /// no new expected value here, only an extra list entry. Omitting one")
    emit("    /// would not fail the test; it would silently cover less.")
    emit("    /// Uses an `Int<16>` work integer, which only exists in a")
    emit('    /// `_wide-support` build, so the test is gated to that build (a')
    emit("    /// narrow-only build has no work integer this wide to exercise).")
    emit('    #[cfg(feature = "_wide-support")]')
    emit("    #[test]")
    emit("    fn by_scale_eq_by_working_scale() {")
    emit("        for scale in [0u32, 1, 17, 18, 19, 30, 38, 86] {")
    emit("            for mode in [")
    emit("                HalfToEven,")
    emit("                Trunc,")
    emit("                Ceiling,")
    emit("                Floor,")
    emit("                HalfAwayFromZero,")
    emit("                HalfTowardZero,")
    emit("                AwayFromZero,")
    emit("                ZeroFiveUp,")
    emit("            ] {")
    emit("                assert_eq!(")
    emit("                    pi_by_scale::<Int<16>>(scale, mode),")
    emit("                    pi_by_working_scale::<Int<16>>(scale, mode),")
    emit("                );")
    emit("                assert_eq!(")
    emit("                    ln2_by_scale::<Int<16>>(scale, mode),")
    emit("                    ln2_by_working_scale::<Int<16>>(scale, mode),")
    emit("                );")
    emit("            }")
    emit("        }")
    emit("    }")
    emit("")
    emit("    /// Width-independence: the same scale gives the same value")
    emit("    /// (zero-extended) in different work-int widths. Exercises")
    emit("    /// `Int<16>` / `Int<32>` work integers (and `resize_to` between")
    emit('    /// them), which only exist in a `_wide-support` build — so the')
    emit("    /// test is gated there. The always-present narrow band is")
    emit("    /// covered by `modes_derive_from_floor_and_roundbit` above.")
    emit('    #[cfg(feature = "_wide-support")]')
    emit("    #[test]")
    emit("    fn value_is_width_independent() {")
    emit("        for scale in [0u32, 5, 17, 18, 30, 38, 50] {")
    emit("            let narrow = pi_by_scale::<Int<16>>(scale, HalfToEven);")
    emit("            let wide = pi_by_scale::<Int<32>>(scale, HalfToEven);")
    emit("            assert_eq!(narrow, wide.resize_to::<Int<16>>());")
    emit("        }")
    emit("    }")
    emit("}")
    emit("")

    # ── POW10 lookup: exact 10^exp, narrowest-fit little-endian limbs,
    # width-generic (zero-extended into W). The wide tiers' `pow10` (the
    # no-const-table tiers D924/D1232, and the width-generic `exp_generic`)
    # read this as a static lookup instead of recomputing `10^exp` by repeated
    # squaring. EXACT (10^exp is an integer) -> no round-up bit. Bands mirror
    # ln2's (the deepest exp working-scale path); `pow10_in` falls back to a
    # runtime `TEN.pow` beyond the generated range.
    POW10_NARROW = 512
    POW10_BASE_MAX, POW10_XW_MAX, POW10_XXW_MAX = (1280, 2560, 5120)
    pow10_builder = r'''/// A compile-time-built `10^exp` band (exact — no round bit). `flat` packs every
/// exponent's narrowest-fit little-endian limbs; entry `i` (exp `lo + i`) is
/// `flat[off[i] .. off[i]+len[i]]`. Built by [`p10_build`] from `1` by ×10.
pub(crate) struct Pow10Band<const F: usize, const N: usize> {
    flat: [u64; F],
    off: [u32; N],
    len: [u16; N],
}

/// Total narrowest-fit limb count over `10^lo ..= 10^hi`, sizing the flat array.
const fn p10_flat_len(lo: u32, hi: u32) -> usize {
    let mut buf = [0u64; MAXK];
    buf[0] = 1;
    let mut blen = 1usize;
    let mut total = 0usize;
    if lo == 0 {
        total += 1;
    }
    let mut e = 1u32;
    while e <= hi {
        let mut carry: u64 = 0;
        let mut i = 0;
        while i < blen {
            let v = (buf[i] as u128) * 10 + carry as u128;
            buf[i] = v as u64;
            carry = (v >> 64) as u64;
            i += 1;
        }
        if carry > 0 {
            buf[blen] = carry;
            blen += 1;
        }
        if e >= lo {
            total += blen;
        }
        e += 1;
    }
    total
}

/// Build `10^lo ..= 10^hi` by repeated ×10 from 1. `10^exp` is exact, so there
/// is no round bit. Reproduces the former literal POW10 table bit-for-bit.
const fn p10_build<const F: usize, const N: usize>(lo: u32, hi: u32) -> Pow10Band<F, N> {
    let mut b = Pow10Band { flat: [0u64; F], off: [0u32; N], len: [0u16; N] };
    let mut buf = [0u64; MAXK];
    buf[0] = 1;
    let mut blen = 1usize;
    let mut cursor = 0usize;
    if lo == 0 {
        b.off[0] = 0;
        b.len[0] = 1;
        b.flat[0] = 1;
        cursor = 1;
    }
    let mut e = 1u32;
    while e <= hi {
        let mut carry: u64 = 0;
        let mut i = 0;
        while i < blen {
            let v = (buf[i] as u128) * 10 + carry as u128;
            buf[i] = v as u64;
            carry = (v >> 64) as u64;
            i += 1;
        }
        if carry > 0 {
            buf[blen] = carry;
            blen += 1;
        }
        if e >= lo {
            let idx = (e - lo) as usize;
            b.off[idx] = cursor as u32;
            b.len[idx] = blen as u16;
            let mut k = 0;
            while k < blen {
                b.flat[cursor + k] = buf[k];
                k += 1;
            }
            cursor += blen;
        }
        e += 1;
    }
    b
}

/// Limbs of `10^(lo+i)` for entry `i` — a `const fn` so a const `exp` folds.
const fn p10_get<const F: usize, const N: usize>(
    b: &'static Pow10Band<F, N>,
    i: usize,
) -> &'static [u64] {
    let o = b.off[i] as usize;
    let l = b.len[i] as usize;
    b.flat.split_at(o).1.split_at(l).0
}
'''
    for ln in pow10_builder.splitlines():
        emit(ln)
    emit("")
    pow10_bands = [
        ("NARROW", 0, POW10_NARROW, None),
        ("BASE", 0, POW10_BASE_MAX, BASE_CFG),
        ("XW", POW10_BASE_MAX + 1, POW10_XW_MAX, XW_CFG),
        ("XXW", POW10_XW_MAX + 1, POW10_XXW_MAX, XXW_CFG),
    ]
    for band, lo, hi, cfg in pow10_bands:
        n = hi - lo + 1
        cfg_attr = f"#[cfg({cfg})]" if cfg is not None else None
        if cfg_attr:
            emit(cfg_attr)
        emit(f"const POW10_{band}_F: usize = p10_flat_len({lo}, {hi});")
        if cfg_attr:
            emit(cfg_attr)
        emit(f"static POW10_{band}: Pow10Band<POW10_{band}_F, {n}> = p10_build({lo}, {hi});")
        emit("")
    emit("/// Limbs (little-endian) of `10^exp` if `exp` is within a generated POW10")
    emit("/// band, else `None`. Bands are feature-gated; the always-present NARROW")
    emit("/// band covers the default / no_std build. `const fn` so a const `exp`")
    emit("/// folds to the matching entry. The per-exp limbs are rebuilt at compile")
    emit("/// time ([`p10_build`]), not shipped as literals.")
    emit("#[inline]")
    emit("const fn pow10_entry(exp: u32) -> Option<&'static [u64]> {")
    emit(f"    if (exp as usize) < {POW10_NARROW + 1} {{")
    emit("        return Some(p10_get(&POW10_NARROW, exp as usize));")
    emit("    }")
    emit(f"    #[cfg({BASE_CFG})]")
    emit("    {")
    emit(f"        if (exp as usize) < {POW10_BASE_MAX + 1} {{")
    emit("            return Some(p10_get(&POW10_BASE, exp as usize));")
    emit("        }")
    emit("    }")
    emit(f"    #[cfg({XW_CFG})]")
    emit("    {")
    emit(f"        let base_lo = {POW10_BASE_MAX} + 1;")
    emit("        if exp >= base_lo {")
    emit("            let idx = (exp - base_lo) as usize;")
    emit(f"            if idx < {POW10_XW_MAX - POW10_BASE_MAX} {{")
    emit("                return Some(p10_get(&POW10_XW, idx));")
    emit("            }")
    emit("        }")
    emit("    }")
    emit(f"    #[cfg({XXW_CFG})]")
    emit("    {")
    emit(f"        let xw_lo = {POW10_XW_MAX} + 1;")
    emit("        if exp >= xw_lo {")
    emit("            let idx = (exp - xw_lo) as usize;")
    emit(f"            if idx < {POW10_XXW_MAX - POW10_XW_MAX} {{")
    emit("                return Some(p10_get(&POW10_XXW, idx));")
    emit("            }")
    emit("        }")
    emit("    }")
    emit("    None")
    emit("}")
    emit("")
    emit("/// Raw little-endian u64 limbs of `10^exp` from the generated POW10 table,")
    emit("/// or `None` beyond the (feature-gated) table range. The untyped sibling of")
    emit("/// [`pow10_in`] — for callers that need the limbs directly in a `[u64]`")
    emit("/// scratch (e.g. the Newton-reciprocal `precompute`, which otherwise rebuilds")
    emit("/// `10^scale` with an O(scale²) ×10 chain per call). `const fn` so a const")
    emit("/// `exp` folds to the entry.")
    emit("#[inline]")
    emit("pub(crate) const fn pow10_limbs(exp: u32) -> Option<&'static [u64]> {")
    emit("    pow10_entry(exp)")
    emit("}")
    emit("")
    emit("// `10^exp` in a work integer is the `pow10` POLICY now: `super::pow10::dispatch`")
    emit("// (table-first, square-and-multiply fallback). The raw-limbs Table primitive is")
    emit("// `pow10_limbs` above; `limbs_to_w` is the Table-arm zero-extend it uses.")
    emit("")

    src = "\n".join(out) + "\n"
    path = "src/consts/table.rs"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(src)
    worst_spare, worst_label, worst_digits = min(
        (spare, label, digits) for label, digits, spare, _p in determined
    )
    escalated = [
        (label, prec) for label, digits, _s, prec in determined
        if prec != decimal_prec_for(digits)
    ]
    print(f"wrote {path} ({len(src)} bytes), {datetime.date.today()}")
    print(f"goldens determined : {len(determined)}"
          "   (lower and upper bounds floor to the same integer)")
    print(f"precision raised   : {len(escalated)}")
    for label, prec in escalated:
        print(f"  RAISED PRECISION {label}: determined only at {prec} bits")
    print(f"tightest golden    : {worst_label} ({worst_digits} digits) — its "
          f"enclosure could be 2^{worst_spare:.0f} times wider and still "
          "determine every retained digit")


if __name__ == "__main__":
    main()
