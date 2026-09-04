// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The SCALE-derived work-rung selector — shared policy-layer support.
//!
//! Wide-tier transcendental kernels compute in a working integer wider
//! than storage. The tier's `$Work` is sized for the MAX storage scale,
//! so at low scales it is heavily over-provisioned (D1232 computes a
//! scale-0 result in `Int<176>` when ~12 limbs suffice). The work-rung
//! pattern keys the work integer on the *working scale* instead: a
//! `const fn` selector picks the narrowest ladder width (in limbs) whose
//! Ziv-escalation digit budget clears the cell's needs, and the policy's
//! rung match (`const { …_rung::<C, SCALE>() }`, a plain `usize`)
//! monomorphises the ONE generic kernel at exactly that `Int<K>`. The
//! switch is on an integer constant, so the monomorphisation collector
//! prunes the unchosen arms before codegen — a `match` on an enum
//! verdict switches on a discriminant read, which it cannot fold.
//!
//! This is the matcher's width axis (the `LimbSize`-axis spirit of
//! `docs/ARCHITECTURE.md` → "Limb width — the matcher's second axis"):
//! the selector only *chooses* a width; the kernels stay single-source
//! generic, and the rung never appears in any `dispatch` signature (the
//! BigRule — it enters as a type parameter via the policy-internal rung
//! match, exactly like `policy::ln`'s Tang rung). Shared here (one
//! private policy-support module, `pub(in crate::policy)`, the
//! `policy::narrow_fit` precedent) so `ln` and the forward trig use a
//! single ladder + walker instead of per-policy copies.
//!
//! The `limbs · 8` digit budget mirrors the shared Ziv escalation's own
//! capacity rule (`wide_trig_core`: `cap_digits = BITS/8 − …` =
//! `limbs · 8 − …`): a rung passing `8·K > need` digits gives the
//! escalation the same headroom accounting it self-caps by, with the
//! ~2.4× bits-per-digit slack (a u64 limb holds ~19.2 digits) covering
//! every kernel intermediate.

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::traits::BigInt;

/// The candidate rung ladder (ascending ComputeLimbs widths, in limbs —
/// min wide storage `Int<3>` .. max tier `$Work` floor `Int<176>`). Every
/// wide tier's storage width AND `$Work` floor is a member, so the walker
/// can always land on an enumerated width. A rung is consulted only
/// inside a policy's rung-routing fn; never part of a `Select` verdict
/// or an `Algorithm`.
pub(in crate::policy) const AVAIL_RUNGS: [usize; 13] =
    [3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 176];

/// Smallest ladder width (limbs) in `[lo, hi]` whose digit budget
/// (`limbs · 8`, = `BITS/8` — the shared Ziv escalation's own capacity
/// rule) strictly clears `needed_digits` decimal digits. If no ladder
/// member in range clears it (the tier's max-scale extreme), `hi` is the
/// answer — reproducing the tier's full `$Work`, so those cells stay
/// bit-identical to the pre-rung routing.
pub(in crate::policy) const fn smallest_rung(needed_digits: u32, lo: usize, hi: usize) -> usize {
    let mut i = 0;
    while i < AVAIL_RUNGS.len() {
        let limbs = AVAIL_RUNGS[i];
        if limbs >= lo && limbs <= hi && (limbs as u32) * 8 > needed_digits {
            return limbs;
        }
        i += 1;
    }
    hi
}

/// Resolve the `ln` Tang work rung for tier `C` at `SCALE` — derives
/// `[storage, floor]` from `C`'s own associated types (`C::Storage`,
/// `C::W` = the tier's `$Work`), so ONE generic selector serves every
/// wide tier (no per-tier ladder, no extra const knob — the BigRule's
/// "inspect your own types" allowance). The rung is clamped at the
/// STORAGE width from below because `ln`'s argument spans the full
/// storage range at every scale (`ln(10^1232)` is a legal scale-0 call).
///
/// `MARGIN` is the directed-Ziv escalation headroom above the working
/// scale. Wide tiers (storage >= 16 limbs) use `MARGIN = 24`: their
/// near-grid-line validity is monotone, so the tighter margin lands the
/// narrowest valid rung. Narrow tiers (storage < 16) keep `MARGIN = 51`:
/// their validity is non-monotone near the grid line, so no single
/// tighter margin is safe — `51` is never too aggressive, at the cost of
/// some missed narrowing. Each tier carries only its own width (rule 6);
/// the golden gate is the correctness wall.
pub(in crate::policy) const fn ln_rung<C: WideTrigCore, const SCALE: u32>() -> usize {
    let storage = <C::Storage as BigInt>::LIMBS;
    let floor = <C::W as BigInt>::LIMBS;
    // Per-tier margin (measured map): wide tiers tighten to 24, narrow
    // stay safe at 51.
    let margin: u32 = if storage >= 16 { 24 } else { 51 };
    smallest_rung(SCALE + margin, storage, floor)
}

/// Digit reserve the forward-trig rung budgets above `SCALE`:
/// the tier `GUARD` (30) + ≥ 30 digits of Ziv-escalation probing beyond
/// the base guard (one full escalation step at low scales; the
/// escalation self-clamps its probes to the rung's `BITS/8` cap, so a
/// deeper-than-reach tie falls back to the clean base narrowing exactly
/// as the tier width does past ITS cap) + the trig `D_BUDGET` argument
/// integer digits (the mod-τ reduction eats one guard digit per integer
/// digit of `|x|` — see `trig_generic::sin_fixed`) + the escalation
/// formula's own `int_digits + 8` headroom. Analytic, continuous in
/// `SCALE`; the golden gate is the correctness wall.
const TRIG_MARGIN: u32 = 76;

/// Resolve the forward-trig (sin / cos / tan) work rung for tier `C` at
/// `SCALE`. Unlike [`ln_rung`] the lower clamp is the ladder minimum,
/// not the storage width: the policy's runtime magnitude gate
/// (`policy::trig`, `|x| < 10^D_BUDGET`) bounds the admitted VALUE, so a
/// rung narrower than storage still holds the lifted argument exactly
/// (the storage→rung resize is magnitude/sign-based and the magnitude
/// provably fits). Out-of-budget arguments never reach the rung — the
/// gate routes them to the tier-width kernel, bit-identical to the
/// pre-rung routing.
pub(in crate::policy) const fn trig_rung<C: WideTrigCore, const SCALE: u32>() -> usize {
    let floor = <C::W as BigInt>::LIMBS;
    smallest_rung(SCALE + TRIG_MARGIN, AVAIL_RUNGS[0], floor)
}

/// Max decimal digits of the INTEGER part of `|x|` admitted to the trig
/// rung (the value axis of the rung's validity region — budgeted inside
/// [`TRIG_MARGIN`]). Arguments at or beyond `10^D_BUDGET` radians take
/// the tier-width path. Continuous region: every `|x| < ~10^8` at every
/// scale, not a point carve-out.
pub(in crate::policy) const D_BUDGET: u32 = 8;

/// Resolve the work rung for a NEAR-SPECIAL-POINT directed kernel
/// (`acosh` at 1, `atanh` at ±1 — the `round_to_storage_directed_
/// near_special` walkers, which `force_confirm` EVERY call with at
/// least one escalated probe at `w₂ ≈ 2·(SCALE + GUARD)`). The budget
/// is therefore keyed on `2·SCALE` (+ the shared [`TRIG_MARGIN`]) so
/// the confirm probe is always REACHABLE inside the rung's escalation
/// cap (`cap = 8·K − int_digits − 8` digits must clear
/// `2·SCALE + 2·GUARD + int_digits`; `TRIG_MARGIN`'s 76 covers the
/// `2·GUARD = 60` plus the small result-digit terms with room), and
/// the rung's true bit capacity (~2.4× the budget digits) holds the
/// ln kernel's `2·w₂` intermediates at that probe. Deeper unstable
/// confirms beyond the rung cap fall back exactly as the tier does
/// past ITS cap; the golden gate is the correctness wall.
pub(in crate::policy) const fn near_special_rung<C: WideTrigCore, const SCALE: u32>() -> usize {
    let floor = <C::W as BigInt>::LIMBS;
    smallest_rung(2 * SCALE + TRIG_MARGIN, AVAIL_RUNGS[0], floor)
}

/// `true` iff `|x| < ~10^BUDGET` — a rung's admitted magnitude region.
/// Conservative bit-length test (`332_192/100_000 < log2(10)`): never
/// admits a value at or beyond `10^(SCALE + BUDGET)` raw units, so an
/// admitted argument's integer digits provably fit the rung's budget;
/// the sliver it under-admits just below the boundary takes the
/// (correct, slower) tier path. One compare against a compile-time
/// constant. Shared by the forward / inverse / hyperbolic / exp rung
/// gates (each passes its family's budget).
#[inline]
pub(in crate::policy) fn in_budget<St: BigInt, const SCALE: u32, const BUDGET: u32>(
    raw: &St,
) -> bool {
    let raw_bit_length = crate::algos::exp::exp_generic::bit_length::<St>(*raw) as u64;
    raw_bit_length * 100_000 <= ((SCALE + BUDGET) as u64) * 332_192
}

/// The const-folded work-rung match: one macro emits the 13-arm ladder
/// match per kernel call so the ladder stays single-source (the
/// `policy::ln::tang_at_rung` shape). `$sel` is the policy-internal
/// rung selector (`trig_rung`, …) resolved in the caller's scope;
/// `$kernel` the rung-generic kernel (imported by the calling module).
///
/// The arms are the [`AVAIL_RUNGS`] widths; the `_` arm is `Int<176>`.
/// `smallest_rung` returns a ladder member or its `hi` bound, which is
/// itself a ladder member (every tier `$Work` is one), so `_` is reached
/// only by 176 — it is the widest rung, not a catch-all for an unknown
/// width.
macro_rules! rung_match {
    ($sel:ident, $C:ty, $SCALE:ident, $kernel:ident, [$($k:tt)*], $($arg:expr),+ $(,)?) => {
        match const { $sel::<$C, $SCALE>() } {
            3 => $kernel::<$C, crate::int::types::Int<3>, $($k)*>($($arg),+),
            4 => $kernel::<$C, crate::int::types::Int<4>, $($k)*>($($arg),+),
            6 => $kernel::<$C, crate::int::types::Int<6>, $($k)*>($($arg),+),
            8 => $kernel::<$C, crate::int::types::Int<8>, $($k)*>($($arg),+),
            12 => $kernel::<$C, crate::int::types::Int<12>, $($k)*>($($arg),+),
            16 => $kernel::<$C, crate::int::types::Int<16>, $($k)*>($($arg),+),
            24 => $kernel::<$C, crate::int::types::Int<24>, $($k)*>($($arg),+),
            32 => $kernel::<$C, crate::int::types::Int<32>, $($k)*>($($arg),+),
            48 => $kernel::<$C, crate::int::types::Int<48>, $($k)*>($($arg),+),
            64 => $kernel::<$C, crate::int::types::Int<64>, $($k)*>($($arg),+),
            96 => $kernel::<$C, crate::int::types::Int<96>, $($k)*>($($arg),+),
            128 => $kernel::<$C, crate::int::types::Int<128>, $($k)*>($($arg),+),
            _ => $kernel::<$C, crate::int::types::Int<176>, $($k)*>($($arg),+),
        }
    };
}
pub(in crate::policy) use rung_match;

/// Max integer digits of `|x|` admitted to the exp / hyperbolic rungs —
/// the RESULT-MAGNITUDE axis (`e^|x|` grows with `x`, so the rung is
/// valid only where the result's integer-digit lift and the exp
/// kernel's internal `2^k` extension provably fit). `|x| < 10` bounds
/// the result lift to `exp_result_int_digits ≤ ~8` digits and the
/// internal `extra` to ≤ ~18 digits, which [`TRIG_MARGIN`]'s budget
/// clears at every scale (the base-probe peak `2·(SCALE + GUARD +
/// k_lift + extra)` digits against the rung's `~2.4 × 8·K` true digit
/// capacity, plus the per-probe `exp_peak_fits` belt in the kernel
/// itself). Larger `|x|` takes the tier-width path. Continuous region —
/// the everyday hyperbolic/exp argument band — not a point carve-out;
/// an A/B re-bench may widen it later.
pub(in crate::policy) const EXP_ARG_BUDGET: u32 = 1;
