// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The SCALE-derived work-rung selector — shared policy-layer support.
//!
//! Wide-tier transcendental kernels compute in a working integer wider
//! than storage. The tier's `$Work` is sized for the MAX storage scale,
//! so at low scales it is heavily over-provisioned (D1232 computes a
//! scale-0 result in `Int<176>` when ~12 limbs suffice). The work-rung
//! pattern keys the work integer on the *working scale* instead: a
//! `const fn` selector picks the narrowest [`Rung`] whose Ziv-escalation
//! digit budget clears the cell's needs, and the policy's rung match
//! (`const { …_rung::<C, SCALE>() }`) monomorphises the ONE generic
//! kernel at exactly that `Int<K>` — const-folded, dead arms
//! eliminated.
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

/// A work-rung width choice — the ComputeLimbs widths the ladder can
/// span (min wide storage `Int<3>` .. max tier `$Work` floor
/// `Int<176>`). Consulted only inside a policy's rung-routing fn; never
/// part of a `Select` verdict or an `Algorithm`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::policy) enum Rung {
    W3,
    W4,
    W6,
    W8,
    W12,
    W16,
    W24,
    W32,
    W48,
    W64,
    W96,
    W128,
    W176,
}

/// The candidate rung ladder (ascending ComputeLimbs widths). Every wide
/// tier's storage width AND `$Work` floor is a member, so the walker can
/// always land on an enumerated width.
pub(in crate::policy) const AVAIL_RUNGS: [usize; 13] =
    [3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 176];

/// Smallest ladder width (limbs) in `[lo, hi]` whose digit budget
/// (`limbs · 8`, = `BITS/8` — the shared Ziv escalation's own capacity
/// rule) strictly clears `need` decimal digits. If no ladder member in
/// range clears it (the tier's max-scale extreme), `hi` is the answer —
/// reproducing the tier's full `$Work`, so those cells stay
/// bit-identical to the pre-rung routing.
pub(in crate::policy) const fn smallest_rung(need: u32, lo: usize, hi: usize) -> usize {
    let mut i = 0;
    while i < AVAIL_RUNGS.len() {
        let w = AVAIL_RUNGS[i];
        if w >= lo && w <= hi && (w as u32) * 8 > need {
            return w;
        }
        i += 1;
    }
    hi
}

/// Ladder width (limbs) → [`Rung`]. Total: every tier `$Work` is a
/// ladder member, so the `_` arm (the widest rung) is reached only by
/// `W176` itself.
pub(in crate::policy) const fn rung_of(limbs: usize) -> Rung {
    match limbs {
        3 => Rung::W3,
        4 => Rung::W4,
        6 => Rung::W6,
        8 => Rung::W8,
        12 => Rung::W12,
        16 => Rung::W16,
        24 => Rung::W24,
        32 => Rung::W32,
        48 => Rung::W48,
        64 => Rung::W64,
        96 => Rung::W96,
        128 => Rung::W128,
        _ => Rung::W176,
    }
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
pub(in crate::policy) const fn ln_rung<C: WideTrigCore, const SCALE: u32>() -> Rung {
    let storage = <C::Storage as BigInt>::LIMBS;
    let floor = <C::W as BigInt>::LIMBS;
    // Per-tier margin (measured map): wide tiers tighten to 24, narrow
    // stay safe at 51.
    let margin: u32 = if storage >= 16 { 24 } else { 51 };
    rung_of(smallest_rung(SCALE + margin, storage, floor))
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
pub(in crate::policy) const fn trig_rung<C: WideTrigCore, const SCALE: u32>() -> Rung {
    let floor = <C::W as BigInt>::LIMBS;
    rung_of(smallest_rung(SCALE + TRIG_MARGIN, AVAIL_RUNGS[0], floor))
}

/// Max decimal digits of the INTEGER part of `|x|` admitted to the trig
/// rung (the value axis of the rung's validity region — budgeted inside
/// [`TRIG_MARGIN`]). Arguments at or beyond `10^D_BUDGET` radians take
/// the tier-width path. Continuous region: every `|x| < ~10^8` at every
/// scale, not a point carve-out.
pub(in crate::policy) const D_BUDGET: u32 = 8;
