// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! candidate: faster D57 cbrt, not wired.
//!
//! Drop-in candidate kernels for the regressed `(D57, SCALE == 20)` cube-root
//! cell (and, where the radicand stays inside the `f64` range, the other
//! `Native`-routed mid-scale wide cells). Each is **bit-for-bit identical** to
//! [`crate::algos::cbrt::cbrt_native::cbrt_native`] /
//! [`crate::algos::cbrt::cbrt_newton::cbrt_newton`] under all six
//! [`RoundingMode`] values — only the Newton *seed* (hence the iteration
//! count) differs. NOT wired into any policy.
//!
//! # Diagnosis — why the shipped `cbrt_native` regressed ~2× vs 0.4.4
//!
//! The cost of each Newton cube-root step `s ← (2s + n/s²)/3` is one wide
//! multiply plus one Knuth divide on `Int<W>` operands (`O(W²)`); the
//! iteration *count* is therefore the whole game once `W` is fixed. That
//! count is set by how close the seed lands to `∛n`.
//!
//! * **0.4.4** (`lookup_d57_s20::icbrt_f64_seeded`, tag `v0.4.4`) seeded
//!   straight off the **full** ~230-bit radicand: `n.as_f64().cbrt()` then
//!   `from_f64`. `as_f64` keeps 53 mantissa bits and `f64::cbrt` is
//!   correctly rounded, so the seed sat within ~2⁻⁵² *relative* of `∛n`.
//!   One unconditional pre-step lifted it above `⌈∛n⌉` and ~2 monotone
//!   steps finished — roughly 3 divides total.
//!
//! * **0.5.0** (`cbrt_native::icbrt_w_seeded` →
//!   [`crate::algo_x_support::seed::cbrt_seed`], `src/algo_x_support/seed.rs:132`)
//!   extracts only the **top 64 bits** of `n` and scales the cube root of
//!   that window back by `2^(shift/3)`. The width-agnostic design is sound,
//!   but the residue handling is coarse: for `shift % 3 == r ≠ 0` it
//!   multiplies the f64 cube root by the *integer* `2^r` (`seed.rs:146`,
//!   `raw * (1u64 << rem3) as f64` → ×2 for r=1, ×4 for r=2) instead of the
//!   true `2^(r/3)` (×1.2599 for r=1, ×1.5874 for r=2), then adds a further
//!   `+2` margin (`seed.rs:155`). So the seed over-shoots `∛n` by up to
//!   ~2.52× (the worst `r=2` case: `4 / 2^(2/3) ≈ 2.52`), versus 0.4.4's
//!   ~1+2⁻⁵². A 2.5× over-estimate costs the monotone-decrease loop several
//!   extra `O(W²)` divides — the measured ~2× regression at D57<20>.
//!
//! # The fix
//!
//! Recover 0.4.4's tight seed without giving up the no-overflow property the
//! top-bits design bought. Two candidates:
//!
//! * [`cbrt_native_fast_a`] — the literal 0.4.4 recipe: `n.as_f64().cbrt()`
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

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::algo_x_support::seed::extract_top_u64;
use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── shared exact rounding tail (identical to cbrt_native) ───────────────

/// The single half-step round + sign reattachment, factored out so both
/// candidates share `cbrt_native`'s exact logic verbatim. Given the floor
/// cube root `q = ⌊∛n⌋` (in `Int<W>`), the radicand `n`, the input sign and
/// the rounding mode, returns the rounded, signed, narrowed `Int<N>`.
#[inline]
fn round_and_narrow<const N: usize, const W: usize>(
    q: Int<W>,
    n: Int<W>,
    negative: bool,
    mode: RoundingMode,
) -> Int<N> {
    let zero = Int::<W>::ZERO;
    let one = Int::<W>::ONE;
    let eight_n = n << 3u32;
    let t = q + q + one;
    let cube = t * t * t;
    let halfway_geq = eight_n >= cube;
    let halfway_gt = eight_n > cube;
    let tie = halfway_geq && !halfway_gt;
    let two_q = q + q;
    let eight_q_cubed = if q == zero { zero } else { two_q * two_q * two_q };
    let residual_nonzero = eight_n > eight_q_cubed;
    let q_is_odd = (q % (one + one)) != zero;
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && q_is_odd),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => negative && residual_nonzero,
        RoundingMode::Ceiling => !negative && residual_nonzero,
    };
    let q = if bump { q + one } else { q };
    let signed = if negative { -q } else { q };
    signed.resize_to::<Int<N>>()
}

// ── candidate A: full-radicand f64::cbrt seed (the 0.4.4 recipe) ────────

/// `⌊∛n⌋` over `Int<W>`, seeded from the **full** radicand via
/// `n.as_f64().cbrt()` (the 0.4.4 `icbrt_f64_seeded` recipe). Caller MUST
/// guarantee `n` is within the `f64` range (`bit_length ≲ 1023`); the
/// `cbrt_native_fast_a` entry guards this and falls back otherwise.
///
/// `as_f64` keeps 53 mantissa bits and `f64::cbrt` is correctly rounded, so
/// `seed` sits within ~2⁻⁵² *relative* of `∛n` — it may under- OR
/// over-shoot. One unconditional Newton pre-step lifts any positive seed to
/// `≥ ⌈∛n⌉` (AM-GM on `(x, x, n/x²)` gives `(2x + n/x²)/3 ≥ ∛n`); the
/// downward-monotone loop then settles on `⌊∛n⌋`, identical to the shipped
/// kernel's fixed point.
#[cfg(feature = "std")]
#[inline]
fn icbrt_w_f64_full<const W: usize>(n: Int<W>) -> Int<W> {
    let seed_f64 = n.as_f64().cbrt();
    let seed = Int::<W>::from_f64(seed_f64);
    let x0 = if seed <= Int::<W>::ZERO { Int::<W>::ONE } else { seed };
    let three = Int::<W>::from_i128(3);
    // Unconditional first Newton step: lifts any positive seed to ≥ ⌈∛n⌉.
    let mut x = (x0 + x0 + n / (x0 * x0)) / three;
    if x <= Int::<W>::ZERO {
        x = Int::<W>::ONE;
    }
    loop {
        let y = (x + x + n / (x * x)) / three;
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
fn icbrt_w_shipped_seed<const W: usize>(n: Int<W>) -> Int<W> {
    let bits = n.bit_length();
    let mag = n.unsigned_abs();
    let mut seed_limbs = [0u64; W];
    crate::algo_x_support::seed::cbrt_seed(mag.as_limbs(), bits, &mut seed_limbs);
    let x0 = Int::<W>::from_mag_limbs(&seed_limbs, false);
    let x0 = if x0 <= Int::<W>::ZERO { Int::<W>::ONE } else { x0 };
    let three = Int::<W>::from_i128(3);
    let mut x = x0;
    loop {
        let y = (x + x + n / (x * x)) / three;
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
    let negative = widened < zero;
    let mag = if negative { -widened } else { widened };
    let n: Int<W> = mag * pow10_2scale;

    // `f64::cbrt` seed only when the radicand is inside the f64 range
    // (`as_f64` would otherwise saturate to ±inf → a degenerate seed). The
    // D57<20> radicand (≤ 10^97 ≈ 322 bits) always passes; the wider cells
    // pass for all but their largest magnitudes, which fall back cleanly.
    #[cfg(feature = "std")]
    let q = if n.bit_length() <= 1020 {
        icbrt_w_f64_full::<W>(n)
    } else {
        icbrt_w_shipped_seed::<W>(n)
    };
    #[cfg(not(feature = "std"))]
    let q = icbrt_w_shipped_seed::<W>(n);

    round_and_narrow::<N, W>(q, n, negative, mode)
}

// ── candidate B: width-safe top-bits seed with exact 2^(r/3) residue ────

/// `⌊∛n⌋` over `Int<W>`, seeded from the top 64 bits like the shipped path
/// but with the **exact** fractional `2^(r/3)` residue multiplier (vs the
/// shipped coarse `2^r`) and a single `+1` margin (vs `+2`). This keeps the
/// no-overflow property at any width while cutting the seed over-shoot from
/// ~2.5× to ~1×, so the monotone loop needs fewer divides.
///
/// Correctness is unchanged: the seed is still a finite positive value and
/// the downward-monotone Newton loop self-corrects to `⌊∛n⌋` from *any*
/// positive start. The unconditional pre-step guarantees we begin at or
/// above `⌈∛n⌉` even if the tighter seed slightly under-shoots.
#[cfg(feature = "std")]
#[inline]
fn icbrt_w_tight_topbits<const W: usize>(n: Int<W>) -> Int<W> {
    let bits = n.bit_length();
    let mag = n.unsigned_abs();
    // Mirror cbrt_seed's path for the tiny-n fallback band.
    let x0 = if bits >= 9 {
        let (top_u64, shift) = extract_top_u64(mag.as_limbs(), bits);
        let rem3 = shift % 3;
        // Exact fractional residue: cbrt(top) · 2^(rem3/3).
        let factor = match rem3 {
            1 => 1.259_921_049_894_873_2_f64, // 2^(1/3)
            2 => 1.587_401_051_968_199_5_f64,  // 2^(2/3)
            _ => 1.0_f64,
        };
        let seed_f64 = (top_u64 as f64).cbrt() * factor;
        let half_shift = shift / 3;
        let truncated = seed_f64 as u128;
        let frac_nonzero = (truncated as f64) != seed_f64;
        let seed_int: u128 = truncated.saturating_add(if frac_nonzero { 1 } else { 0 });
        // Place seed_int << half_shift into an Int<W>.
        let lo = seed_int as u64;
        let hi = (seed_int >> 64) as u64;
        let mut limbs = [0u64; W];
        limbs[0] = lo;
        if W > 1 {
            limbs[1] = hi;
        }
        let base = Int::<W>::from_mag_limbs(&limbs, false);
        base << half_shift
    } else {
        // 2^ceil(bits/3) classical 1-bit seed.
        let e = bits.div_ceil(3);
        Int::<W>::ONE << e
    };
    let x0 = if x0 <= Int::<W>::ZERO { Int::<W>::ONE } else { x0 };
    let three = Int::<W>::from_i128(3);
    // Unconditional pre-step: AM-GM lifts any positive seed to ≥ ⌈∛n⌉, so a
    // mild under-shoot from the tighter seed is corrected before the loop.
    let mut x = (x0 + x0 + n / (x0 * x0)) / three;
    if x <= Int::<W>::ZERO {
        x = Int::<W>::ONE;
    }
    loop {
        let y = (x + x + n / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// candidate B entry — bit-identical drop-in for `cbrt_native`, width-safe at
/// every native cell (no `f64`-range guard needed), with a tighter top-bits
/// seed. `no_std` delegates to the shipped seed.
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
    let negative = widened < zero;
    let mag = if negative { -widened } else { widened };
    let n: Int<W> = mag * pow10_2scale;

    #[cfg(feature = "std")]
    let q = icbrt_w_tight_topbits::<W>(n);
    #[cfg(not(feature = "std"))]
    let q = icbrt_w_shipped_seed::<W>(n);

    round_and_narrow::<N, W>(q, n, negative, mode)
}

// ── bit-identity test (NOT run here — coordinator runs the suite) ───────

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::{cbrt_native_fast_a, cbrt_native_fast_b};
    use crate::algos::cbrt::cbrt_native::cbrt_native;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    /// Both candidates must be bit-identical to the shipped `cbrt_native`
    /// (itself oracle-gated against `cbrt_newton` / `ulp_strict_golden`) for
    /// every routed cell, sign, and rounding mode. Matching it certifies the
    /// candidate seeds change only the divide count, never the result.
    fn check_cell<const N: usize, const W: usize>(scale: u32, raws: &[i128]) {
        let pow = Int::<W>::TEN.pow(2 * scale);
        for &r in raws {
            let raw = Int::<N>::from_i128(r);
            for mode in ALL_MODES {
                let want = cbrt_native::<N, W>(raw, pow, mode);
                let got_a = cbrt_native_fast_a::<N, W>(raw, pow, mode);
                let got_b = cbrt_native_fast_b::<N, W>(raw, pow, mode);
                assert_eq!(got_a, want, "A: N={N} W={W} scale={scale} raw={r} mode={mode:?}");
                assert_eq!(got_b, want, "B: N={N} W={W} scale={scale} raw={r} mode={mode:?}");
            }
        }
    }

    /// Near-storage-max radicand at each native cell (widest `mag·10^(2·SCALE)`).
    fn near_max<const N: usize>(neg: bool) -> Int<N> {
        let mut mag = [0u64; N];
        for m in mag.iter_mut() {
            *m = u64::MAX;
        }
        mag[N - 1] = u64::MAX >> 1;
        Int::<N>::from_mag_limbs(&mag, neg)
    }

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
        check_cell::<4, 8>(35, &raws);
        check_cell::<6, 12>(57, &raws);
        check_cell::<8, 16>(75, &raws);
        check_cell::<8, 16>(76, &raws);
        check_cell::<12, 25>(115, &raws);
        check_cell::<16, 32>(150, &raws);
    }

    #[test]
    fn fast_candidates_match_native_near_max_all_cells() {
        for &neg in &[false, true] {
            for mode in ALL_MODES {
                macro_rules! chk {
                    ($n:literal, $w:literal, $s:literal) => {{
                        let pow = Int::<$w>::TEN.pow(2 * $s);
                        let raw = near_max::<$n>(neg);
                        let want = cbrt_native::<$n, $w>(raw, pow, mode);
                        assert_eq!(cbrt_native_fast_a::<$n, $w>(raw, pow, mode), want, "A near_max N={} neg={neg} mode={mode:?}", $n);
                        assert_eq!(cbrt_native_fast_b::<$n, $w>(raw, pow, mode), want, "B near_max N={} neg={neg} mode={mode:?}", $n);
                    }};
                }
                chk!(3, 6, 20);
                chk!(4, 8, 35);
                chk!(6, 12, 57);
                chk!(8, 16, 75);
                chk!(8, 16, 76);
                chk!(12, 25, 115);
                chk!(16, 32, 150);
            }
        }
    }
}
