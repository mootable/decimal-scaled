// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! candidate: faster D57 cbrt, not wired.
//!
//! Drop-in candidate kernels for the `(D57, SCALE == 20)` cube-root
//! cell (and, where the radicand stays inside the `f64` range, the other
//! `Native`-routed mid-scale wide cells). Each is **bit-for-bit identical** to
//! [`crate::algos::cbrt::cbrt_native::cbrt_native`] /
//! [`crate::algos::cbrt::cbrt_newton::cbrt_newton`] under every
//! [`RoundingMode`] value — only the Newton *seed* (hence the iteration
//! count) differs. NOT wired into any policy.
//!
//! # Why the seed matters
//!
//! The cost of each Newton cube-root step `s ← (2s + n/s²)/3` is one wide
//! multiply plus one Knuth divide on `Int<W>` operands (`O(W²)`); the
//! iteration *count* is therefore the whole game once `W` is fixed. That
//! count is set by how close the seed lands to `∛n`.
//!
//! The shipped seed (`cbrt_native::icbrt_w_seeded` →
//! [`crate::algo_x_support::seed::cbrt_seed`], `src/algo_x_support/seed.rs:132`)
//! extracts only the **top 64 bits** of `n` and scales the cube root of
//! that window back by `2^(shift/3)`. The width-agnostic design is sound,
//! but the residue handling is coarse: for `shift % 3 == r ≠ 0` it
//! multiplies the f64 cube root by the *integer* `2^r` (`seed.rs:146`,
//! `raw * (1u64 << rem3) as f64` → ×2 for r=1, ×4 for r=2) instead of the
//! true `2^(r/3)` (×1.2599 for r=1, ×1.5874 for r=2), then adds a further
//! `+2` margin (`seed.rs:155`). So the seed over-shoots `∛n` by up to
//! ~2.52× (the worst `r=2` case: `4 / 2^(2/3) ≈ 2.52`). A 2.5× over-estimate
//! costs the monotone-decrease loop several extra `O(W²)` divides.
//!
//! # The fix
//!
//! Tighten the seed without giving up the no-overflow property the
//! top-bits design bought. Two candidates:
//!
//! * [`cbrt_native_fast_a`] — full-radicand `n.as_f64().cbrt()`
//!   on the whole radicand. Valid for D57<20> because the radicand
//!   `mag · 10^40 ≤ 10^97` is far below `f64::MAX ≈ 1.8·10^308`; the
//!   `f64`-range guard makes it safe to *offer* at the other native cells
//!   and fall back to the shipped seed where the radicand would overflow.
//!
//! * [`cbrt_native_fast_b`] — keep the width-safe top-bits extraction but
//!   replace the coarse `2^r` residue with the exact fractional
//!   `2^(r/3)` multiplier and a single `+1` margin, cutting the seed
//!   over-shoot from ~2.5× to ~1×.
//!
//! Both reuse `cbrt_native`'s exact half-step rounding tail verbatim, so the
//! result is bit-identical and only the divide count changes.
//!
//! Seed strategy citation: Hasselgren / Crandall & Pomerance 2005, "Prime
//! Numbers: A Computational Perspective" §9.2.1 (Newton integer roots from
//! an `f64` bootstrap seed). Implemented from the text; no external code.
//!
//! NOT feature-gated: this generic kernel is referenced by the
//! feature-independent `cbrt` policy, so it must compile in every build. It
//! is dead-arm-eliminated wherever its `(N, SCALE)` cells are not reached.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── shared exact rounding tail (identical to cbrt_native) ───────────────

/// The single half-step round + sign reattachment, factored out so both
/// candidates share `cbrt_native`'s exact logic verbatim. Given the floor
/// cube `root` (`⌊∛n⌋` in `Int<W>`), the `radicand`, the input sign and
/// the rounding mode, returns the rounded, signed, narrowed `Int<N>`.
#[inline]
fn round_and_narrow<const N: usize, const W: usize>(
    root: Int<W>,
    radicand: Int<W>,
    is_negative: bool,
    mode: RoundingMode,
) -> Int<N> {
    let zero = Int::<W>::ZERO;
    let one = Int::<W>::ONE;
    let eight_radicand = radicand << 3u32;
    let doubled_midpoint = root + root + one;
    let cube = doubled_midpoint * doubled_midpoint * doubled_midpoint;
    let halfway_geq = eight_radicand >= cube;
    let halfway_gt = eight_radicand > cube;
    let tie = halfway_geq && !halfway_gt;
    let two_root = root + root;
    let eight_root_cubed = if root == zero { zero } else { two_root * two_root * two_root };
    let residual_nonzero = eight_radicand > eight_root_cubed;
    // Last decimal digit of the (non-negative) root magnitude `root`.
    let root_mod_10 = (root % Int::<W>::TEN).as_i128() as u8;
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && root_mod_10 & 1 == 1),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => is_negative && residual_nonzero,
        RoundingMode::Ceiling => !is_negative && residual_nonzero,
        // `root` is the magnitude, so away-from-zero is a bump either sign.
        RoundingMode::AwayFromZero => residual_nonzero,
        RoundingMode::ZeroFiveUp => residual_nonzero && matches!(root_mod_10, 0 | 5),
    };
    let root = if bump { root + one } else { root };
    let signed_root = if is_negative { -root } else { root };
    signed_root.resize_to::<Int<N>>()
}

// ── candidate A: full-radicand f64::cbrt seed ────────

/// `⌊∛n⌋` over `Int<W>`, seeded from the **full** radicand via
/// `as_f64().cbrt()`. Caller MUST
/// guarantee the `radicand` is within the `f64` range (`bit_length ≲ 1023`);
/// the `cbrt_native_fast_a` entry guards this and falls back otherwise.
///
/// `as_f64` keeps 53 mantissa bits and `f64::cbrt` is correctly rounded, so
/// `seed` sits within ~2⁻⁵² *relative* of `∛n` — it may under- OR
/// over-shoot. One unconditional Newton pre-step lifts any positive seed to
/// `≥ ⌈∛n⌉` (AM-GM on `(x, x, n/x²)` gives `(2x + n/x²)/3 ≥ ∛n`); the
/// downward-monotone loop then settles on `⌊∛n⌋`, identical to the shipped
/// kernel's fixed point.
#[cfg(feature = "std")]
#[inline]
fn icbrt_w_f64_full<const W: usize>(radicand: Int<W>) -> Int<W> {
    let seed_f64 = crate::algo_x_support::seed::cbrt_seed_f64_full(radicand.as_f64());
    let seed = Int::<W>::from_f64(seed_f64);
    let x0 = if seed <= Int::<W>::ZERO { Int::<W>::ONE } else { seed };
    let three = Int::<W>::from_i128(3);
    // Unconditional first Newton step: lifts any positive seed to ≥ ⌈∛n⌉.
    let mut x = (x0 + x0 + radicand / (x0 * x0)) / three;
    if x <= Int::<W>::ZERO {
        x = Int::<W>::ONE;
    }
    loop {
        let y = (x + x + radicand / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// Width-agnostic fallback seed identical to the shipped
/// `cbrt_native::icbrt_w_seeded` (top-64-bits via
/// [`crate::algo_x_support::seed::cbrt_seed`]). Used by candidate A when the
/// radicand would overflow `f64`, and is the whole no_std body.
#[inline]
fn icbrt_w_shipped_seed<const W: usize>(radicand: Int<W>) -> Int<W> {
    let bits = radicand.bit_length();
    let magnitude = radicand.unsigned_abs();
    let mut seed_limbs = [0u64; W];
    crate::algo_x_support::seed::cbrt_seed(magnitude.as_limbs(), bits, &mut seed_limbs);
    let x0 = Int::<W>::from_mag_limbs(&seed_limbs, false);
    let x0 = if x0 <= Int::<W>::ZERO { Int::<W>::ONE } else { x0 };
    let three = Int::<W>::from_i128(3);
    let mut x = x0;
    loop {
        let y = (x + x + radicand / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// candidate A entry — bit-identical drop-in for `cbrt_native`, with the
/// full-radicand `f64::cbrt` seed under `std` (and an `f64`-range guard so
/// the wider native cells stay safe). `no_std` and the overflow fallback use
/// the shipped seed, so the fixed point — and the result — never change.
#[inline]
#[must_use]
pub(crate) fn cbrt_native_fast_a<const N: usize, const W: usize>(
    raw: Int<N>,
    pow10_2scale: Int<W>,
    mode: RoundingMode,
) -> Int<N> {
    if raw == Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let zero = Int::<W>::ZERO;
    let widened: Int<W> = raw.resize_to::<Int<W>>();
    let is_negative = widened < zero;
    let magnitude = if is_negative { -widened } else { widened };
    let radicand: Int<W> = magnitude * pow10_2scale;

    // `f64::cbrt` seed only when the radicand is inside the f64 range
    // (`as_f64` would otherwise saturate to ±inf → a degenerate seed). The
    // D57<20> radicand (≤ 10^97 ≈ 322 bits) always passes; the wider cells
    // pass for all but their largest magnitudes, which fall back cleanly.
    #[cfg(feature = "std")]
    let root = if radicand.bit_length() <= 1020 {
        icbrt_w_f64_full::<W>(radicand)
    } else {
        icbrt_w_shipped_seed::<W>(radicand)
    };
    #[cfg(not(feature = "std"))]
    let root = icbrt_w_shipped_seed::<W>(radicand);

    round_and_narrow::<N, W>(root, radicand, is_negative, mode)
}

// ── candidate B: width-safe top-bits seed with exact 2^(r/3) residue ────

/// `⌊∛n⌋` over `Int<W>`, seeded from the shared seed library
/// [`crate::algo_x_support::seed::cbrt_seed`] — the width-safe top-64-bits
/// bootstrap with the **exact** fractional `2^(r/3)` residue multiplier and
/// guaranteed over-estimate `+1` margin (the tight residue this candidate
/// originally pioneered now lives in the library, so all callers share it).
/// Under `std` the library bootstraps from hardware `f64::cbrt`; under
/// `no_std` it is the classical pure-integer `2^⌈bits/3⌉` — both
/// over-estimates, so the kernel stays platform-agnostic.
///
/// Correctness is unchanged: the library seed is a finite positive
/// over-estimate and the downward-monotone Newton loop self-corrects to
/// `⌊∛n⌋`. The unconditional pre-step (one redundant Newton step from a
/// guaranteed over-estimate) is retained to keep candidate B's loop shape.
#[inline]
fn icbrt_w_tight_topbits<const W: usize>(radicand: Int<W>) -> Int<W> {
    let bits = radicand.bit_length();
    let magnitude = radicand.unsigned_abs();
    let mut seed_limbs = [0u64; W];
    crate::algo_x_support::seed::cbrt_seed(magnitude.as_limbs(), bits, &mut seed_limbs);
    let x0 = Int::<W>::from_mag_limbs(&seed_limbs, false);
    let x0 = if x0 <= Int::<W>::ZERO { Int::<W>::ONE } else { x0 };
    let three = Int::<W>::from_i128(3);
    // Unconditional pre-step: AM-GM lifts any positive seed to ≥ ⌈∛n⌉, so a
    // mild under-shoot from the tighter seed is corrected before the loop.
    let mut x = (x0 + x0 + radicand / (x0 * x0)) / three;
    if x <= Int::<W>::ZERO {
        x = Int::<W>::ONE;
    }
    loop {
        let y = (x + x + radicand / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// candidate B entry — bit-identical drop-in for `cbrt_native`, width-safe at
/// every native cell (no `f64`-range guard needed), seeded from the shared
/// seed library on both `std` and `no_std` (the leaf cfg-swaps internally).
#[inline]
#[must_use]
pub(crate) fn cbrt_native_fast_b<const N: usize, const W: usize>(
    raw: Int<N>,
    pow10_2scale: Int<W>,
    mode: RoundingMode,
) -> Int<N> {
    if raw == Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let zero = Int::<W>::ZERO;
    let widened: Int<W> = raw.resize_to::<Int<W>>();
    let is_negative = widened < zero;
    let magnitude = if is_negative { -widened } else { widened };
    let radicand: Int<W> = magnitude * pow10_2scale;

    // The seed library is std/no_std-agnostic (it cfg-swaps internally), so
    // a single call covers both builds — no per-build kernel split.
    let root = icbrt_w_tight_topbits::<W>(radicand);

    round_and_narrow::<N, W>(root, radicand, is_negative, mode)
}

// ── bit-identity test (NOT run here — run by the full suite) ───────

// Same gating rationale as `cbrt_native`'s test module: these candidates run
// Newton in a wide work `Int<W>`, whose `radicand / (x·x)` build-max Knuth-divide
// scratch (`4·MAX_WORK_N + 2` u64) only covers `W` once a wide tier raises
// MAX_WORK_N to 16. Each test/case is gated to exactly the `dNN` tier whose
// storage width it instantiates; the module guard is the precise union of
// those tiers (d57..d307). Tests only — the kernels stay un-gated.
#[cfg(all(
    test,
    feature = "std",
    any(
        feature = "d57",
        feature = "d76",
        feature = "d115",
        feature = "d153",
        feature = "d230",
        feature = "d307"
    )
))]
mod tests {
    use super::{cbrt_native_fast_a, cbrt_native_fast_b};
    use crate::algos::cbrt::cbrt_native::cbrt_native;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// Both candidates must be bit-identical to the shipped `cbrt_native`
    /// (itself oracle-gated against `cbrt_newton` / `ulp_strict_golden`) for
    /// every routed cell, sign, and rounding mode. Matching it certifies the
    /// candidate seeds change only the divide count, never the result.
    fn check_cell<const N: usize, const W: usize>(scale: u32, raws: &[i128]) {
        let pow10_2scale = Int::<W>::TEN.pow(2 * scale);
        for &raw_value in raws {
            let raw = Int::<N>::from_i128(raw_value);
            for mode in ALL_MODES {
                let want = cbrt_native::<N, W>(raw, pow10_2scale, mode);
                let got_a = cbrt_native_fast_a::<N, W>(raw, pow10_2scale, mode);
                let got_b = cbrt_native_fast_b::<N, W>(raw, pow10_2scale, mode);
                assert_eq!(got_a, want, "A: N={N} W={W} scale={scale} raw={raw_value} mode={mode:?}");
                assert_eq!(got_b, want, "B: N={N} W={W} scale={scale} raw={raw_value} mode={mode:?}");
            }
        }
    }

    /// Near-storage-max radicand at each native cell (widest `mag·10^(2·SCALE)`).
    fn near_max<const N: usize>(is_negative: bool) -> Int<N> {
        let mut magnitude = [0u64; N];
        for limb in magnitude.iter_mut() {
            *limb = u64::MAX;
        }
        magnitude[N - 1] = u64::MAX >> 1;
        Int::<N>::from_mag_limbs(&magnitude, is_negative)
    }

    // D57 storage (N=3), work `Int<6>`.
    #[cfg(feature = "d57")]
    #[test]
    fn fast_candidates_match_native_d57_s20() {
        let raws: [i128; 11] = [
            0,
            1,
            100_000_000_000_000_000_000,
            150_000_000_000_000_000_000,
            -150_000_000_000_000_000_000,
            800_000_000_000_000_000_000,
            -800_000_000_000_000_000_000,
            2_700_000_000_000_000_000_000,
            12_345_678_901_234_567_890,
            (1i128 << 90) | 0xBEEF,
            (1i128 << 120) | 0x1357,
        ];
        check_cell::<3, 6>(20, &raws);
    }

    // Multi-tier sweep over D76..D307 (no D57 case); whole-fn gate is the
    // union of those tiers so a D57-only or D462+ build drops the fn whole
    // (no empty body), and each cell carries its own `dNN` gate so a
    // single-tier build runs exactly its own width.
    #[cfg(any(
        feature = "d76",
        feature = "d115",
        feature = "d153",
        feature = "d230",
        feature = "d307"
    ))]
    #[test]
    fn fast_candidates_match_native_other_cells() {
        let raws: [i128; 7] = [
            0,
            1,
            -800_000_000_000_000_000_000,
            800_000_000_000_000_000_000,
            (1i128 << 100) | 0xBEEF,
            -((1i128 << 120) | 0x1357),
            i128::MAX,
        ];
        #[cfg(feature = "d76")]
        check_cell::<4, 8>(35, &raws);
        #[cfg(feature = "d115")]
        check_cell::<6, 12>(57, &raws);
        #[cfg(feature = "d153")]
        check_cell::<8, 16>(75, &raws);
        #[cfg(feature = "d153")]
        check_cell::<8, 16>(76, &raws);
        #[cfg(feature = "d230")]
        check_cell::<12, 25>(115, &raws);
        #[cfg(feature = "d307")]
        check_cell::<16, 32>(150, &raws);
    }

    /// The `W = 3N` work widths the cbrt policy now routes the mid-wide
    /// tiers (D57/D76/D115/D153, routed by `N` at every scale) to, checked
    /// at a near-storage-max magnitude (both signs) across the tier's scale
    /// range including its max usable scale — where `mag · 10^(2·SCALE)` is
    /// widest and a too-small `W` would overflow (release-mode UB). The fast
    /// `a` arm (the production seed) must stay bit-identical to the
    /// oracle-gated `cbrt_native`.
    // 3N work widths (W = 9/12/18/24) for D57/D76/D115/D153. Whole-fn gate
    // is the union of those tiers; each `chk!` carries its own `dNN` gate so
    // a single-tier build runs exactly its width.
    #[cfg(any(
        feature = "d57",
        feature = "d76",
        feature = "d115",
        feature = "d153"
    ))]
    #[test]
    fn fast_a_routed_3n_widths_near_max() {
        for &is_negative in &[false, true] {
            for mode in ALL_MODES {
                macro_rules! chk {
                    ($n:literal, $w:literal, $($s:literal),+) => {{
                        $(
                            let pow10_2scale = Int::<$w>::TEN.pow(2 * $s);
                            let raw = near_max::<$n>(is_negative);
                            let want = cbrt_native::<$n, $w>(raw, pow10_2scale, mode);
                            assert_eq!(cbrt_native_fast_a::<$n, $w>(raw, pow10_2scale, mode), want, "A 3N N={} W={} s={} neg={is_negative} mode={mode:?}", $n, $w, $s);
                        )+
                    }};
                }
                #[cfg(feature = "d57")]
                chk!(3, 9, 0, 20, 28, 57);
                #[cfg(feature = "d76")]
                chk!(4, 12, 0, 20, 35, 76);
                #[cfg(feature = "d115")]
                chk!(6, 18, 0, 25, 57, 115);
                #[cfg(feature = "d153")]
                chk!(8, 24, 0, 25, 75, 153);
            }
        }
    }

    #[test]
    fn fast_candidates_match_native_near_max_all_cells() {
        for &is_negative in &[false, true] {
            for mode in ALL_MODES {
                macro_rules! chk {
                    ($n:literal, $w:literal, $s:literal) => {{
                        let pow10_2scale = Int::<$w>::TEN.pow(2 * $s);
                        let raw = near_max::<$n>(is_negative);
                        let want = cbrt_native::<$n, $w>(raw, pow10_2scale, mode);
                        assert_eq!(cbrt_native_fast_a::<$n, $w>(raw, pow10_2scale, mode), want, "A near_max N={} neg={is_negative} mode={mode:?}", $n);
                        assert_eq!(cbrt_native_fast_b::<$n, $w>(raw, pow10_2scale, mode), want, "B near_max N={} neg={is_negative} mode={mode:?}", $n);
                    }};
                }
                #[cfg(feature = "d57")]
                chk!(3, 6, 20);
                #[cfg(feature = "d76")]
                chk!(4, 8, 35);
                #[cfg(feature = "d115")]
                chk!(6, 12, 57);
                #[cfg(feature = "d153")]
                chk!(8, 16, 75);
                #[cfg(feature = "d153")]
                chk!(8, 16, 76);
                #[cfg(feature = "d230")]
                chk!(12, 25, 115);
                #[cfg(feature = "d307")]
                chk!(16, 32, 150);
            }
        }
    }
}
