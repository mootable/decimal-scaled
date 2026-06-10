// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Division policy — the divisor-shape algorithm matcher.
//!
//! Canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"), with one twist: division is the **one policy with no
//! const-width axis**. Its operands have *independent* runtime lengths that
//! no single level const expresses — the decimal `/` divides a `2N`-limb
//! scaled numerator by an `N`-limb divisor, and the slice roots
//! (`isqrt_newton` / `icbrt_newton` / `newton_reciprocal`) divide bare
//! `&[u64]` of runtime length with no `N` in their types at all. So unlike
//! a unary (`select<N>`) or binary (`select<Nthis, Nother>`) policy,
//! [`select`] here is **non-generic**: it always returns [`Select::ByShape`],
//! delegating the whole choice to the runtime [`select_for_limbs`]. (Forcing
//! a `<N>` would make the slice roots manufacture a const they don't have —
//! the kind of caller-side specialisation the architecture forbids — and
//! the divide doesn't use `N` anyway, since its engine choice is runtime.)
//!
//! Two selectors: [`select`] (the const matcher — here a no-op `ByShape`)
//! and [`select_for_limbs`] (the runtime limb-shape decision it delegates
//! to). The engines stay pure — each takes an already-chosen algorithm.
//! This file owns the *choice*: the benched crossover threshold
//! ([`BZ_THRESHOLD`]) is policy DATA here, not a magic number in a kernel.

use crate::int::algos::div::div_burnikel_ziegler_with_knuth::div_burnikel_ziegler_with_knuth;
use crate::int::algos::div::div_knuth::div_knuth;
use crate::int::algos::div::div_knuth_u128_limb::div_knuth_u128_limb;
use crate::int::algos::div::div_rem::div_rem;
use crate::int::algos::div::div_rem_schoolbook::div_rem_schoolbook;

// ── 1. the real division engines — NAMED, no `Default` ────────────────

/// The division engines the divisor-shape matcher chooses between.
/// Variants are the CamelCase of each engine fn's name minus the `div_`
/// function prefix (`div_knuth` → `Knuth`, …) — strict 1:1 with the
/// engine fns in [`crate::int::algos::div`].
///
/// `pub(crate)` together with [`select_for_limbs`] so a concrete-`N` caller
/// that needs **exact `ComputeLimbs` scratch** (the decimal `/` and `%` int
/// layers) can read the matcher's verdict and route to the chosen engine's
/// `*_into` variant with its own buffers — rather than hardcoding one engine
/// (the matcher-bypass defect). The slice [`dispatch`] is the build-max entry
/// for callers that don't size scratch. Exposing the verdict (not a
/// `dispatch_into`, which would have to thread `N` through this slice policy)
/// keeps the division policy width-free.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Algorithm {
    /// [`div_rem`] — the `const fn` single-limb hardware fast path
    /// (`div_rem`'s Fast B is one hardware `u128 / u64` per dividend
    /// limb — already optimal for a single-limb divisor).
    Rem,
    /// [`div_knuth`] — Knuth Algorithm D at base 2⁶⁴.
    Knuth,
    /// [`div_burnikel_ziegler_with_knuth`] — Burnikel–Ziegler outer
    /// chunking, recursing to Knuth as its base case. Registered but
    /// **unrouted**: the policy-map (af3011f6) measured it slowest or
    /// near-slowest at every den_n ≥ 65 working width (u128-limb Knuth wins
    /// that region 1.68–1.78×), so `select_for_limbs` no longer returns it.
    /// Kept as a future-ready alternative — it can only win once the back-
    /// multiply turns sub-quadratic at ≤128 limbs — and reachable via its
    /// `#[cfg(test)]` differential. `#[allow(dead_code)]` suppresses the
    /// never-constructed warning.
    #[allow(dead_code)]
    BurnikelZieglerWithKnuth,
    /// [`div_knuth_u128_limb`] — Knuth Algorithm D on u128 limbs (base
    /// 2¹²⁸). The `LimbSize` axis as an engine: chosen only for the **wide
    /// (`2n`-dividend) even-`n` divisor ≥ [`U128_DIV_THRESHOLD`]** shape,
    /// where the aligned u128 carry-chain beats base-2⁶⁴ (it LOSES on the
    /// balanced shape — see the threshold doc).
    KnuthU128Limb,
    /// [`div_rem_schoolbook`] — binary shift-subtract long division,
    /// the naive reference baseline. Registered but unrouted:
    /// `select_for_limbs` never returns this variant; it exists for
    /// unit-test reachability and future routing experiments.
    /// `#[allow(dead_code)]` suppresses the compiler warning.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the verdict ────────────────────────────────────────────────────

/// A settled engine, or "the (runtime) limb shape decides". For division
/// every `N` resolves to [`Select::ByShape`] — the engine choice is
/// determined by the operands' effective limb counts, known only at run
/// time — so the `ByShape` arm delegates to [`select_for_limbs`].
/// [`Select::ByAlgorithm`] is the canonical alternative (a width-keyed
/// fixed engine); it is unused by this policy today but kept so `select<N>`
/// could pin an engine for some `N` range without changing the verdict
/// type.
#[derive(Clone, Copy)]
enum Select {
    #[allow(dead_code)]
    ByAlgorithm(Algorithm),
    /// The runtime limb shape decides: [`select_for_limbs`] applied to the
    /// raw `(num, den)` operands (it strips leading zeros and counts itself).
    ByShape(fn(&[u64], &[u64]) -> Algorithm),
}

// ── policy data: the benched crossover threshold ──────────────────────

/// Burnikel–Ziegler engagement threshold, in u64 limbs. **No longer a
/// routing threshold** — [`select_for_limbs`] does not return BZ (the
/// policy-map af3011f6 showed it losing across the whole supported surface;
/// the den_n ≥ 65 region routes to [`Algorithm::KnuthU128Limb`] instead).
/// This const is now read ONLY by [`div_burnikel_ziegler_with_knuth`]'s own
/// engagement guard (keeping BZ reachable via its `#[cfg(test)]` differential
/// + the bench seam): a divisor of at least this many effective limbs whose
/// dividend is `≥ 2·n` runs the BZ chunking, otherwise it shorts to Knuth.
/// **Policy data** — the kernels take an already-chosen engine and never
/// see this number.
///
/// **Benched optimum (Knuth-vs-BZ crossover, `div_kernel_ab`).** The
/// shipped `_with_knuth` engine is *block* division (it splits the
/// dividend into `n`-limb chunks and runs a full Knuth divide of each
/// `chunk‖carry` by the divisor), not recursive fast-division, so it has
/// no asymptotic edge over a single Knuth pass at the supported widths.
/// An A/B of Knuth vs the (forced) chunking core on the `div`-shaped
/// `wide_num` operands (`2n`-limb dividend over an `n`-limb divisor)
/// shows **Knuth wins at every width with no crossover**:
///
/// | divisor (limbs / tier) | Knuth vs BZ (wide_num) |
/// |------------------------|------------------------|
/// | 3  / D57               | Knuth 1.55× faster     |
/// | 4  / D76               | Knuth 1.85× faster     |
/// | 6  / D115              | Knuth 1.54× faster     |
/// | 8  / D153              | Knuth 1.43× faster     |
/// | 12 / D230              | Knuth 1.38× faster     |
/// | 16 / D307              | Knuth 1.27× faster     |
/// | 24 / D462              | Knuth 1.18× faster     |
/// | 32 / D616              | Knuth 1.12× faster     |
/// | 48 / D924              | Knuth 1.08× faster     |
/// | 64 / D1232             | Knuth 1.01–1.06× faster|
///
/// The margin narrows with width but never crosses (an exploratory 96-
/// and 128-limb probe still favours Knuth ~1.10×, and the curve has
/// plateaued — no crossover exists at any reachable width). The
/// `balanced` (square `rem`/`div_rem`) shape never meets the
/// `num_m ≥ 2·den_n` gate and favours Knuth ~1.4× throughout.
///
/// Therefore the optimum is to **never engage** the block engine within
/// the supported range: the widest storage tier is D1232 = 64 limbs (a
/// cross-scale dividend reaches 128 limbs), so a threshold of `65`
/// guarantees every supported divide takes the faster Knuth engine while
/// leaving the engine + gate intact for a future true recursive-BZ
/// kernel. (Lowering toward the legacy `8`/`16` would *regress* every
/// D307+ wide divide by engaging the slower block engine.)
pub(crate) const BZ_THRESHOLD: usize = 65;

/// u128-limb Knuth ([`Algorithm::KnuthU128Limb`]) engagement threshold, in
/// u64 divisor limbs. **Policy data.**
///
/// **Benched** (`div_kernel_ab`, u128 base-2¹²⁸ vs u64 base-2⁶⁴, wide `2n`/`n`
/// shape — the decimal `/` scaled-numerator shape; the limb-width win
/// materialises only on this shape). After the q̂-reciprocal was hoisted out
/// of the per-digit loop (~12–15% faster u128), the policy-map (af3011f6)
/// bisected the even-`n` crossover cleanly between 22 and 24:
///
/// | den_n | wide `2n`/`n`   |
/// |-------|----------------|
/// | 16    | u64 2.46×      |
/// | 18    | u64 1.07×      |
/// | 20    | tie (1.01×)    |
/// | 22    | u64 1.02×      |
/// | 24    | **u128 1.11×** |
/// | 26    | **u128 1.10×** |
/// | 30    | **u128 1.35×** |
/// | 32    | **u128 1.42×** |
///
/// Clean u128 win from **den_n = 24** upward (cross-checked: den_n=24 is u128
/// 1.10–1.11× across two independent runs) — lowered from `32`, since the
/// faster u128 engine now wins the 24–30 band it previously tied/lost. u128
/// is routed for an **even** divisor `≥ 24` limbs with a `≥ 2·n` dividend,
/// INCLUDING the den_n ≥ 65 working widths the decimal `÷10^w` rescale + wide
/// transcendentals reach (same map: u128 beat the old Burnikel–Ziegler
/// routing 1.68–1.78× at den_n 96/128). The balanced shape (square `rem` /
/// the `Int<N>` `/` operator) and every narrow/odd divisor stay base-2⁶⁴
/// Knuth, where u128 loses ~1.5×. The engine itself falls back to `div_knuth`
/// for odd / `< 4`-limb divisors, so the matcher gate is the perf carve-out.
const U128_DIV_THRESHOLD: usize = 24;

// ── 3. the matcher: `select` (no const axis) → `select_for_limbs` ─────

/// The top-level matcher. Division has no const-width axis (its operands'
/// lengths are independent runtime values — see the module docs), so unlike
/// a `select<N>` unary policy this is **non-generic** and always defers the
/// choice to the runtime [`select_for_limbs`]. A future limb refinement
/// (e.g. routing an even, wide divisor to a u128-limb engine) is a **runtime
/// arm inside `select_for_limbs`** — gated on the runtime `den_n`, where the
/// width information actually is — NOT a const verdict here.
const fn select() -> Select {
    Select::ByShape(select_for_limbs)
}

/// Select the division engine for an operand pair's **limb shape**. The
/// sibling of [`select`]: `select` keys on the const width, this keys on
/// the runtime effective limb counts, which it computes itself:
///
/// It works the counts out itself, and **only the ones a branch needs** —
/// passing raw slices (rather than pre-computed counts from [`dispatch`])
/// means the dividend is never walked on the paths that don't look at it:
///
/// * `den_n` — the **divisor's** effective limb count (Knuth's `n`):
///   `den.len()` with trailing zero limbs stripped. `den_n == 0` is a
///   divide-by-zero (asserted here). Always needed.
/// * the **dividend's** effective limb count (`num.len()` with top zero
///   limbs stripped) is computed **lazily**, only once the divisor reaches
///   the smaller [`U128_DIV_THRESHOLD`] gate (both wide engines want a
///   `≥ 2·n` dividend). The common cases (single-limb divisor → `Rem`; any
///   `2..U128_DIV_THRESHOLD`-limb divisor → `Knuth`) never strip the
///   dividend at all.
///
/// Routing: a single-limb divisor takes the hardware [`Algorithm::Rem`]
/// path (covers every `10^scale`, `scale ≤ 19`); a wide (`≥ 2·n` dividend)
/// EVEN divisor of at least [`U128_DIV_THRESHOLD`] limbs takes the u128-limb
/// Knuth engine; everything else takes base-2⁶⁴ Knuth.
#[inline]
pub(crate) fn select_for_limbs(num: &[u64], den: &[u64]) -> Algorithm {
    let den_n = effective_limbs(den);
    assert!(den_n > 0, "dispatch: divide by zero");
    if den_n == 1 {
        return Algorithm::Rem;
    }
    // `den_n >= 2` here. The wide u128-limb engine wants a `≥ 2·n` dividend,
    // so the dividend's effective length is computed once — and lazily: only
    // for a divisor wide enough to reach the threshold. Every common
    // `2..U128_DIV_THRESHOLD`-limb divisor returns Knuth without stripping the
    // dividend at all.
    if den_n >= U128_DIV_THRESHOLD {
        let num_m = effective_limbs(num);
        // Wide (`2n`-dividend) even divisor → the u128 limb-width engine. This
        // covers the WHOLE even-divisor wide region from `U128_DIV_THRESHOLD`
        // up, INCLUDING the den_n ≥ 65 working widths the decimal `÷10^w`
        // rescale + wide transcendentals reach: the policy-map (af3011f6)
        // measured u128 beating the old Burnikel–Ziegler routing there
        // 1.68–1.78× (96/128 limbs), so BZ is no longer routed (the balanced
        // shape stays Knuth — u128 loses it ~1.5×).
        if den_n % 2 == 0 && num_m >= 2 * den_n {
            return Algorithm::KnuthU128Limb;
        }
    }
    Algorithm::Knuth
}

/// Effective limb count of a little-endian magnitude slice: its length with
/// trailing (most-significant) zero limbs stripped — `0` for an all-zero
/// slice.
#[inline]
fn effective_limbs(limbs: &[u64]) -> usize {
    let mut n = limbs.len();
    while n > 0 && limbs[n - 1] == 0 {
        n -= 1;
    }
    n
}

// ── 4. the dispatcher: fold `select<N>`, run the selector, route ──────

/// Runtime divide dispatcher — the single entry every multi-limb divide
/// flows through. Folds the `select<N>` verdict (const per monomorphisation),
/// runs the runtime [`select_for_limbs`], and routes to the chosen engine;
/// `quot` / `rem` are written by that engine.
///
/// Slice-based (no `<N>`): the numerator and divisor have *independent*
/// runtime lengths that no single const width expresses — the decimal `/`
/// divides a `2N`-limb scaled numerator by an `N`-limb divisor, the slice
/// roots divide bare runtime-length slices. Every caller already holds its
/// operands as slices, so none has to manufacture a const to call this. The
/// build-max Knuth `u`/`v` scratch lives in the engine ([`div_knuth`] owns
/// it), not here — the matcher allocates nothing. A concrete-`N` caller that
/// can size scratch exactly (`Int<N>: ComputeLimbs`) sources its own buffer
/// family and calls the chosen engine's `*_into` variant.
pub(crate) fn dispatch(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    let algo = match const { select() } {
        Select::ByAlgorithm(fixed) => fixed,
        Select::ByShape(selector) => selector(num, den),
    };
    match algo {
        Algorithm::Rem => div_rem(num, den, quot, rem),
        Algorithm::Knuth => div_knuth(num, den, quot, rem),
        Algorithm::BurnikelZieglerWithKnuth => {
            div_burnikel_ziegler_with_knuth(num, den, quot, rem)
        }
        Algorithm::KnuthU128Limb => div_knuth_u128_limb(num, den, quot, rem),
        Algorithm::Schoolbook => div_rem_schoolbook(num, den, quot, rem),
    }
}
