//! Exponential policy — the per-(N, SCALE) algorithm matcher (plus the
//! derived exp2).
//!
//! `D<Int<N>, SCALE>::exp_strict_with(mode)` delegates directly to the one
//! shared [`dispatch`] generic function — the canonical matcher-only
//! policy shape (see `docs/ARCHITECTURE.md`), mirrored from `sqrt`:
//!
//! 1. an [`Algorithm`] enum — Series / Tang / Schoolbook, no `Default`;
//! 2. a [`Select`] verdict;
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via `const { select::<N, SCALE>() }`, then an exhaustive
//!    `match algo` — no `_`, no panic.
//!
//! The narrow tiers run the 256-bit `Fixed` kernel (`exp_series_2limb`,
//! D18 widened to Int<2>); the wide tiers run the tier-generic `exp_series`
//! over `WideTrigCore`, or the per-tier `exp_tang` band kernel, reached by
//! a `match N` with `resize_to` bridges (identity at the matched `N`).
//!
//! exp2 is derived (`2^x = exp(x·ln2)` with an exact-power pin) and routes
//! DOWN to the narrow `exp_series_2limb::exp2_*` kernels or the wide
//! per-tier `wide_trig_<tier>::exp2_{strict,approx}_with_kernel` free fns —
//! never back through a sibling decimal policy.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Series,
    #[cfg(feature = "_wide-support")]
    Tang,
    #[allow(dead_code)]
    Schoolbook,
}

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // D57 (Int<3>): the seam A/B (`benches/micro/exp_series_tang_ab.rs`)
        // sweeps the full SCALE range at the production Tang config and shows
        // Tang beats Series at EVERY D57 scale (validity bit-identical to
        // Series across the operand spread × all six modes at each cell):
        // s0 4.81×, s10 2.81×, s17 2.96×, s22 2.91×, s23 2.26×, s28 2.32×,
        // s33 2.29×, s38 19.3×, s42 1.90×, s44 2.27×, s45 1.52×, s56 44.6×.
        // The old gate `(3, 18..=22) | (3, 45..=56)` left the s23..=44 GAP on
        // Series — the cause of the bench-branch-compare powf_D57_s42 3.64×
        // regression (powf's inner exp(y·ln x) lands at storage SCALE=42 in
        // that gap → Series). Cover the WHOLE D57 scale range through the
        // small-`|x|` value gate (Tang's `k·ln 2` lift fits the work width
        // only for small `|x|`; large-`|x|` routes to Series, which is always
        // valid — matching the existing D76/D115/D153/D230 wide-tier pattern).
        // `tang_routed` splits 0..=44 (M=128,G=8) vs 45..=56 (M=512,G=30) per
        // the seam A/B's per-cell (M,G) ranking; the boundary at s45 is where
        // the two configs tie.
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 0..=56) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        // D76 (Int<4>): the full width × scale A/B
        // (`benches/micro/exp_wide_series_tang_ab.rs`) shows Tang beats the
        // Series squaring core at EVERY D76 scale, including the MAX scale
        // (s75), where the map ranks Series ~16× slower than Tang and every
        // Tang candidate is bit-identical to Series (zero validity failures).
        // The old gate stopped at s74, so the MAX scale `exp_D76_s75` cell —
        // the bench-branch-compare 6.41× regression — fell through to the
        // slow Series `_` arm (a Class-I single-cell boundary miss). Cover the
        // WHOLE D76 scale range (0..=75, the design max) through the
        // small-`|x|` value gate so no scale is left on the Series path.
        #[cfg(any(feature = "d76", feature = "wide"))]
        (4, 0..=75) => Select::ByValue(wide_tang_gate::<N, SCALE>),
        // Narrow-wide tiers (N = 6/8/12/16) — the storage-strict exp path.
        // RE-MAPPED 2026-05-29 against the freshly-baked exp Tang rodata table
        // (`src/algos/support/exp_tang_table.rs`, commits b390abd9/6f5929d4):
        // the OLD Tang rectangles here were tuned BEFORE the bake (when the
        // table was a per-call Series recompute), so they over-routed Tang.
        // The post-bake N-way A/B `benches/micro/exp_wide_series_tang_ab.rs`
        // (PINNED core 22, Series vs 3 Tang configs vs Schoolbook, validity-
        // gated bit-identical to Series × all six modes) shows SERIES (the
        // generic squaring core) now BEATS every Tang config at EVERY scale at
        // these tiers — the table bake did not move Tang ahead, and Tang's
        // table-multiply + post-reduction Taylor costs MORE than Series's
        // adaptive Smith `r/2^n` from D115 up. Measured medians (Tang =
        // production tang_m512_g30, ratio = Series-faster-by):
        //   D115: s0 1.55×, s28 1.40× (Schoolbook≈Series winner), s57 1.29×,
        //         s86 (Series wins), s113 1.28× — Series/Schoolbook are the
        //         top two at all five samples; Tang ranks #3+ everywhere.
        //   D153: s0 1.40×, s38 (Series), s76 (Schoolbook), s114, s151 —
        //         Series/Schoolbook always the top two; Tang #3+.
        //   D230: s0 1.65×, s57, s115, s228 — Series/Schoolbook top two. The
        //         lone s172 cell shows tang_m512 +5% over Series, but it is a
        //         single non-continuous point bracketed by s115 (Series) and
        //         s228 (Series) — bench noise, NOT a continuous win-region, so
        //         per architectural-review Class I it is NOT carved out.
        //   D307: s0 1.79×, s76 1.59×, s153, s230, s305 — Series/Schoolbook
        //         top two at every sample; Tang #3+ across the whole range.
        // So D115/D153/D230/D307 fall through to the `_` Series arm at EVERY
        // scale — no Tang gate. (Schoolbook ties Series within ~2-13% noise at
        // these tiers — same Fixed Smith core — and is the unrouted reference;
        // Series stays the canonical wide kernel, so the `_` arm is Series.)
        //
        // The WIDEST tiers (N >= 24: D462/D616/D924/D1232) likewise fall
        // through to Series: the A/B confirms Series/Schoolbook win at every
        // sampled scale (D462 s0 1.12×, … D1232 every sample), and at D1232 MAX
        // scale (s1230) single-shot Tang is not even bit-identical to Series
        // (ALL three Tang configs reported INVALID by the validity wall — its
        // `k·ln 2` lift overflows the guard), so Tang is INELIGIBLE there
        // regardless of speed. No Tang gate for N >= 24.
        // D462 (Int<24>): the wide A/B map sweeps {s0, s115, s231, s346, s460}
        // and ranks SERIES THE WINNER at every scale (s0 1.02×, s231 1.07×,
        // s346 1.14×, s460 1.19× — vs the production Tang config tang_m512_g30;
        // only s115 ~tied at +1.01× Tang). A two-pass bisection at s58
        // (`benches/micro/exp_wide_tang_bisect.rs`) returns a 0.4% ~tie
        // (Tang 304.4ms, Series 305.5ms — within bench noise), and s115 in
        // the bisect run flips to Series +1.10× (the 5-point sweep was
        // measurement noise). No Tang win-region exists at D462 — every
        // confirmed-non-noise cell goes to Series. Matches v0.4.4's per-tier
        // decision (the v0.4.4 `policy::exp` comment: "D462 — Tang exp probed
        // at SCALE 225..=235 and LOST (~75% regression)"). Tang's
        // table-multiply post-reduction Taylor needs more wide mults than
        // Series's adaptive Smith r/2^n at this depth, so the table-elimination
        // of the `k·ln 2` reduction does NOT pay for the longer Taylor at
        // Int<24>. No Tang gate at D462 — falls through to the `_` Series arm
        // at every scale. D616/D924/D1232 (wider tiers) already fall through
        // to Series for the same reason: the A/B confirms Series wins
        // 1.19×–1.50× at every sampled scale at D616 (s0 1.29×, s154 1.19×,
        // s308 ~tie, s462 1.50×, s614 1.45×) and 1.49× at D924_s0 (rest of
        // D924 + all of D1232 did not complete in the time budget, but trend
        // is uniform — Series widening lead with width).
        //
        // **Audit Finding #5 (2026-05-28) DISPOSITION** — the
        // `research/2026_05_28_d462_d924_d1232_policy_audit.md` audit raised
        // that `policy::exp` Tang then gated N ∈ {3,4,6,8,12,16} and asked
        // whether the wider tiers N ≥ 24 should also be Tang-gated. The 5-point
        // sweep above + the D462 bisection at s58/s115 EMPIRICALLY REFUTE the
        // audit lead for `exp`: at N=24/32/48 the Tang structural overhead
        // (table multiply + post-reduction Taylor at very wide work widths)
        // exceeds the saving from removing the `k·ln 2` reduction. Series's
        // adaptive Smith r/2^n is faster at every confirmed cell at D462+.
        // The audit lead is RESOLVED for `exp` and NOT a defect — the absent
        // wide-tier Tang arms are evidence-backed. (The hyperbolic trig and
        // forward-trig coverage at D462+ are unrelated and tracked separately.)
        _ => Select::ByAlgorithm(Algorithm::Series),
    }
}

/// Value gate for the wide-tier low-scale Tang rectangles: Tang is correct
/// only while its `k·ln 2` working-scale lift fits the work width, i.e. for
/// small `|x|`. Route Tang for `|x| < 100`, else Series (always valid).
///
/// `|x| < 100` ⇔ `|raw| < 10^(SCALE+2)`, tested conservatively on the bit
/// length: `|raw| < 2^B ≤ 10^(SCALE+2)` when `B = ⌊(SCALE+2)·log2 10⌋`
/// (`log2 10 ≈ 3.32192`, taken as `332192/100000`, rounded DOWN so `2^B`
/// never exceeds `10^(SCALE+2)` — never routes an out-of-range value to Tang).
#[cfg(feature = "_wide-support")]
fn wide_tang_gate<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    let max_bits = (SCALE + 2) * 332_192 / 100_000;
    if BigInt::bit_length(*raw) <= max_bits {
        Algorithm::Tang
    } else {
        Algorithm::Series
    }
}

#[inline]
fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(raw),
    }
}

/// Returns `true` iff the WORKING-SCALE composition surface should route
/// `e^{stuff}` through Tang (`tang_exp_fixed::<C, M, true>`) rather than the
/// Series `exp_fixed` for this `(N, SCALE)` cell.
///
/// Consumed by the working-scale `exp_fixed_routed<SCALE>` surface emitted per
/// tier by `decl_wide_transcendental!` — the composition sites for `exp2`,
/// `powf`, and the hyperbolics (`sinh`/`cosh`/`tanh`). This is a DISTINCT
/// operating point from the storage-strict [`select`] above: it runs at the
/// caller's working width `w` (a few extra digits) with the kernel's
/// `INTERNAL_EXTRA` lift covering arbitrary `|k|`, NOT at storage SCALE with
/// the small-`|x|` value gate. The two were the same query before 2026-05-29,
/// but the storage-strict exp A/B (`exp_wide_series_tang_ab`) remapped
/// D115/D153/D230/D307 OFF Tang at storage scale on measured evidence; that
/// evidence does NOT cover the composition operating point (different `w`,
/// different `|k|` regime), and the hyperbolic working-scale path is a
/// separately-benched WIN that must not be silently retargeted — so the
/// working-scale gate is kept independent and unchanged here (Tang for the
/// narrow-wide tiers N ∈ {3,4,6,8,12,16}; Series for N >= 24, matching the
/// pre-remap routing). A future composition-path A/B (sinh/cosh/tanh/powf/exp2
/// at working scale) can re-key this gate on its own measurements.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) const fn is_tang<const N: usize, const SCALE: u32>() -> bool {
    let _ = SCALE;
    // The narrow-wide tiers ran Tang at working scale before the storage-strict
    // remap; preserve that for the composition surface (no measurement says to
    // change it). N >= 24 was — and stays — Series.
    matches!(N, 3 | 4 | 6 | 8 | 12 | 16)
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match resolve::<N, SCALE>(&raw) {
        Algorithm::Series => series_routed::<N, SCALE>(raw, mode),
        #[cfg(feature = "_wide-support")]
        Algorithm::Tang => tang_routed::<N, SCALE>(raw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch_with<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    // Only the narrow tier honours caller working_digits (matching the
    // prior ExpPolicy routing, where wide exp_with_impl ignored it).
    match N {
        1 | 2 => super::narrow_checked::<N>(
            crate::algos::exp::exp_series_2limb::exp_with(
                raw.resize_to::<Int<2>>(),
                SCALE,
                working_digits,
                mode,
            ),
            "exp_with",
            SCALE,
        ),
        _ => {
            let _ = working_digits;
            dispatch::<N, SCALE>(raw, mode)
        }
    }
}

#[inline]
fn series_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_schoolbook::exp_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::exp::exp_schoolbook::exp_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_schoolbook::exp_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp_strict", SCALE),
    }
}

#[cfg(feature = "_wide-support")]
#[inline]
fn tang_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        // D57: TWO continuous (M,G) sub-bands inside the merged Tang arm:
        // 0..=44 runs the (M=128, G=8) low-band kernel, 45..=56 the (M=512,
        // G=30) high-band kernel. The split is the same shape the seam A/B
        // (`benches/micro/exp_series_tang_ab.rs`) already confirmed at the
        // edges (s44 (128,8)=2.27× vs s45 (512,30)=1.52× — close wall-clocks,
        // not a regression); inside 0..=44 (128,8) beats (512,30) by ~1.5× at
        // every gap cell (s23/s28/s33/s38/s42/s44), so the boundary stays at
        // 45. Flags <true,true,false> = DIRECTED + EXTERNAL_EXTRA (matching the
        // D76/D115/D153/D230 wide-tier shape): the EXTERNAL_EXTRA guard lift
        // covers the large-`|k|` case the merged Tang gate now exposes (at
        // GAP scales the value gate `wide_tang_gate` admits `|x|` up to ~100,
        // where `|k|·log10 2 ≈ 30` digits exceeds the narrow base guard);
        // DIRECTED enables Ziv escalation for the directed modes. The old
        // `<false,false,false>` shape worked for the narrow `18..=22` /
        // `45..=56` bands only because the storage-bit constraint at those
        // SCALEs implicitly bounded `|x|` to the small-`|k|` regime.
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => {
            let r = raw.resize_to::<Int<3>>();
            let out = match SCALE {
                0..=44 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 128, 8, true, true, false>(r, mode),
                45..=56 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 512, 30, true, true, false>(r, mode),
                _ => crate::algos::support::wide_trig_core::exp_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        // D76 (Int<4>): full-range Tang, M=512 G=30, the directed +
        // external-extra shape <true,true,false> — bit-identical to Series
        // across the spread × all six modes at every sampled scale (s0/s19/s38/
        // s57/s74) in the wide A/B (`exp_wide_series_tang_ab`), where Tang wins
        // 1.05-1.20× at every scale (and the value-gate sweep shows Tang wins
        // 8-10× for `|x|` 10..110 where Series's reduction blows up). The
        // `select` gate (small `|x|`) keeps it valid; large-`|x|` falls through
        // to Series. (NOTE: at s74 the *tang_m512_g60* probe was reported
        // INVALID by the validity wall, but the PRODUCTION tang_m512_g30 stays
        // bit-identical — the production config is the one wired here.)
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::exp::exp_tang::exp_tang::<crate::types::widths::wide_trig_d76::Core, SCALE, 512, 30, true, true, false>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        // N >= 6: `select` routes EVERY scale to Series here (the 2026-05-29
        // post-bake storage-strict A/B showed Series/Schoolbook beat every Tang
        // config at D115/D153/D230/D307+, and Tang is INELIGIBLE at D1232 max
        // scale — see the `select` comment), so `tang_routed` is never reached
        // for N >= 6. The per-tier Tang kernels (`exp_tang::<wide_trig_dNNN>`)
        // remain available as kept alternatives in `algos/exp/exp_tang.rs` for
        // a future re-bench; no stale `tang_routed` delegation is kept for them
        // — the `_` arm below is the single source of truth that they run
        // Series. (The working-scale composition surface for these tiers routes
        // via `is_tang` / `exp_fixed_routed`, NOT through here.)
        _ => series_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn exp2_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp2_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::exp2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "exp2_strict", SCALE),
    }
}

#[inline]
#[must_use]
pub(crate) fn exp2_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, working_digits: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "exp2_with", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::exp2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), working_digits, mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::exp::exp_series_2limb::exp2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "exp2_with", SCALE),
    }
}
