// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Seed leaves — initial Newton estimates for the integer roots (and,
//! later, the decimal `sqrt_fixed` work-width path).
//!
//! A single support-library file (the leaf analogue of
//! `int/algos/limbs.rs`): all the seed leaves plus their shared
//! bit-extraction helper live here together, so the std/no_std scaffolding
//! and `extract_top_u64` are written once.
//!
//! Each seed leaf exposes ONE API whose `std`/`no_std` behaviour is
//! cfg-swapped *internally*, so callers (the isqrt/icbrt Newton kernels)
//! stay agnostic:
//!
//! - under `std`, the seed comes from the **inherent** `f64` intrinsic
//!   (`(top as f64).sqrt()` / `.cbrt()`) of the top 64 significant bits,
//!   scaled back to the operand's magnitude — ~53 correct bits in one shot;
//! - under `no_std`, the seed is the classical **pure-integer** one-bit
//!   estimate (`2^⌈bits/2⌉` for sqrt, `2^⌈bits/3⌉` for cbrt) — built only
//!   from Rust primitives, never `libm` / `num_traits::Float`.
//!
//! Both seeds are safe over-estimates: the downward-monotone Newton loop in
//! the kernel self-corrects to the exact floor root regardless of which
//! seed body ran, so the *result* is identical on `std` and `no_std`; only
//! the iteration count differs.
//!
//! All leaves are width-agnostic — a `&[u64]` / `u32` interface — so the
//! decimal-side consumers can reuse them unchanged.
//!
//! # Over-estimate guarantee (the load-bearing invariant — PROOF)
//!
//! Every `sqrt_seed*` leaf returns a value `≥ √n` (the *real* square root,
//! hence `≥ floor(√n)`). This is what makes the caller's downward-monotone
//! Newton recurrence `x ← (x + n/x)/2` valid: from an over-estimate it
//! decreases monotonically and lands on `floor(√n)` or `floor(√n)+1`, which a
//! single final `if x > n/x { x -= 1 }` correction pins exactly. From an
//! UNDER-estimate the recurrence steps *up* on the first iteration, the
//! "stop when it stops decreasing" guard fires immediately, and the routine
//! returns the under-estimate — a WRONG floor. So the over-estimate property
//! is correctness-critical, not a mere performance hint.
//!
//! **Decomposition.** For `n` with bit-length `bits`, write
//! `shift = bits − min(64, bits)` and `top = ⌊n / 2^shift⌋` — the top `≤64`
//! significant bits (leading 1 at bit 63 when `bits ≥ 64`). Then
//! `n = top·2^shift + r` with `0 ≤ r < 2^shift`. The `std` body forms a strict
//! integer over-estimate `seed_int` of `√top` (of `√(2·top)` when `shift` is
//! odd) via `⌊f64::sqrt(top)⌋ + [frac≠0] + 1` — note the **always-`+1`** — and
//! returns `seed_int · 2^h` with `h = ⌊shift/2⌋`.
//!
//! **Even `shift = 2h`.** `√n ≤ √((top+1)·2^{2h}) = √(top+1)·2^h`. The
//! always-`+1` gives `seed_int ≥ √top + 1`, and the elementary inequality
//! `(√t + 1)² = t + 2√t + 1 ≥ t + 1` yields `√top + 1 ≥ √(top+1)`. Hence
//! `seed = seed_int·2^h ≥ √(top+1)·2^h ≥ √n`. ∎
//! The always-`+1` is **essential**: it closes the perfect-square-`top` case
//! (`⌊√top⌋ = √top` exactly), where the low bits `r` push `√n` strictly above
//! `√top·2^h`. Without it the seed under-estimates there — a failure of
//! density `2^{-32}` that uniform-random tests practically never hit.
//!
//! **Odd `shift = 2h+1`.** `√n ≤ √(2(top+1))·2^h`. The body first over-estimates
//! `√2·√top = √(2·top)`, so `seed_int ≥ √(2·top) + 1 ≥ √(2(top+1))` by the same
//! lemma; hence `seed ≥ √n`. ∎
//!
//! **f64 rounding.** `top ≤ 2^64` rounds to the nearest f64 (`< 2^11`
//! absolute for the largest `top`) and `f64::sqrt` is correctly rounded, so the
//! deficit in `⌊f64::sqrt(top)⌋` is `< 1` — dominated by the always-`+1`. The
//! `core::f64::consts::SQRT_2` constant's `~2^{-52}` relative error is `≪ 1`
//! absolute at these magnitudes, likewise covered.
//!
//! **no_std / tiny-`n` path.** Returns `2^⌈bits/2⌉`; since `n < 2^bits`,
//! `√n < 2^{bits/2} ≤ 2^⌈bits/2⌉`. ∎
//!
//! **Consequence for wider-than-u128 consumers.** The proof holds for
//! ARBITRARY `bits` via the `top`/`shift` decomposition — it is not special to
//! `n` that fit a `u128`. So the `&[u64]` slice leaf [`sqrt_seed`] is a valid
//! over-estimate seed for a radicand of ANY width: a width-`N` isqrt kernel
//! (e.g. the `u256` hypot root) MUST seed through this leaf rather than
//! re-deriving the `· 2^h` scaling by hand. A hand-rolled re-derivation that
//! drops the always-`+1` silently reintroduces the perfect-square-`top` bug
//! that random tests cannot detect — reuse the proven leaf.
//!
//! Hasselgren's trick / seed strategy — Crandall & Pomerance 2005, "Prime
//! Numbers: A Computational Perspective" §9.2.1.

/// Extract the top (most significant) 64 significant bits of the
/// little-endian `u64` magnitude `n`, given its bit length `bits` (the seed
/// callers guard `bits >= 2` before calling).
///
/// Returns `(top, shift)` where `shift = bits - min(64, bits)` is the bit
/// position of the start of the extracted window (`n ≈ top << shift`, exact
/// when `bits <= 64`), and `top` holds those bits right-aligned into a
/// `u64` (leading 1 at bit 63 when `bits >= 64`). The 64-bit window fits
/// the f64 mantissa with 11 bits of headroom, which is what makes the `f64`
/// seed bootstrap exact to ~53 bits. Pure bit manipulation — primitives
/// only.
#[inline]
pub(crate) const fn extract_top_u64(n: &[u64], bits: u32) -> (u64, u32) {
    let shift = bits - if bits < 64 { bits } else { 64 };
    let limb_idx = (shift / 64) as usize;
    let bit_off = shift % 64;
    let top: u64 = if bit_off == 0 {
        n[limb_idx]
    } else {
        let lo = n[limb_idx] >> bit_off;
        let hi = if limb_idx + 1 < n.len() {
            // `bit_off != 0` here, so `64 - bit_off` is in `1..=63`; the
            // `checked_shl` keeps the kernel's original defensive form.
            match n[limb_idx + 1].checked_shl(64 - bit_off) {
                Some(v) => v,
                None => 0,
            }
        } else {
            0
        };
        lo | hi
    };
    (top, shift)
}

/// Write a safe over-estimate seed for `floor(sqrt(n))` into `out`.
///
/// `n` is the little-endian `u64` magnitude, `bits = bit_len(n)` (caller
/// guarantees `bits >= 2`), and `out` is the **zeroed** work slice; the
/// seed is OR-ed in, then a minimal non-zero fallback is forced if the
/// placement landed entirely outside the slice — preserving the Newton
/// kernel's pre-loop invariant `x >= 1`.
///
/// `std`: hardware `f64::sqrt` bootstrap (~53 correct bits) with the
/// odd-shift `SQRT_2` correction and round-up. `no_std`: classical
/// pure-integer 1-bit seed `2^ceil(bits/2)`. Both are over-estimates, so
/// the downward-monotone Newton loop converges to the identical floor root.
///
/// Hasselgren's trick — see Crandall & Pomerance 2005, "Prime Numbers: A
/// Computational Perspective" §9.2.1.
#[inline]
pub(crate) fn sqrt_seed(n: &[u64], bits: u32, out: &mut [u64]) {
    let work = out.len();

    // Under `std`, the f64 bootstrap needs the top-64-bit window to carry
    // enough magnitude to be worth it; for tiny `n` (`bits < 8`) it falls
    // back to the same classical 1-bit seed the no_std path always uses.
    #[cfg(feature = "std")]
    if bits >= 8 {
        let (top_u64, shift) = extract_top_u64(n, bits);
        let seed_f64 = (top_u64 as f64).sqrt();
        let (seed_f64, half_shift) = if (shift & 1) == 1 {
            (seed_f64 * core::f64::consts::SQRT_2, (shift - 1) / 2)
        } else {
            (seed_f64, shift / 2)
        };
        let truncated = seed_f64 as u128;
        let frac_nonzero = (truncated as f64) != seed_f64;
        let seed_int: u128 = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        place_seed(seed_int, half_shift, out, work);
        return;
    }

    // Pure-integer 1-bit seed `2^ceil(bits/2)` — the no_std path always,
    // and the `std` tiny-`n` fallback. `n`-derived top bits unused here.
    let _ = n;
    let e = bits.div_ceil(2);
    let idx = (e / 64) as usize;
    if idx < work {
        out[idx] |= 1u64 << (e % 64);
    } else if work > 0 {
        out[0] |= 1;
    }
}

/// Over-estimate seed for `floor(sqrt(n))` of a single `u128` scalar `n >= 1`
/// (`bits = 128 - n.leading_zeros()`). The SCALAR sibling of [`sqrt_seed`] for
/// the hot u128 isqrt call sites (hypot's `isqrt_u128`, …) that work in one
/// `u128` rather than a limb slice.
///
/// Same feature-based bootstrap and convergence guarantee as [`sqrt_seed`]:
/// under `std` the hardware `f64::sqrt` of the top 64 significant bits with
/// the odd-shift `SQRT_2` correction + round-up (~53 correct bits in one shot
/// → the downward Newton loop converges in ~1-2 iterations); under `no_std`
/// the classical pure-integer 1-bit `2^ceil(bits/2)`. BOTH are guaranteed
/// over-estimates, so the caller's downward-monotone Newton recurrence never
/// under-runs into a linear floor-walk and lands on the identical floor root
/// on either build. Returns a value `>= floor(sqrt(n))`.
#[inline]
pub(crate) fn sqrt_seed_u128(n: u128, bits: u32) -> u128 {
    #[cfg(feature = "std")]
    if bits >= 8 {
        // Top 64 significant bits + their bit position (mirrors
        // `extract_top_u64` for the scalar case): `shift = bits - min(64,bits)`.
        let shift = bits - if bits < 64 { bits } else { 64 };
        let top_u64 = (n >> shift) as u64;
        let seed_f64 = (top_u64 as f64).sqrt();
        let (seed_f64, half_shift) = if (shift & 1) == 1 {
            (seed_f64 * core::f64::consts::SQRT_2, (shift - 1) / 2)
        } else {
            (seed_f64, shift / 2)
        };
        let truncated = seed_f64 as u128;
        let frac_nonzero = (truncated as f64) != seed_f64;
        // +1 for any truncated fraction, +1 for a strict over-estimate
        // (mirrors `sqrt_seed`'s std body).
        let seed_int = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        return seed_int << half_shift;
    }
    // no_std (and `std` tiny-`n`): classical 1-bit over-estimate `2^ceil(bits/2)`.
    1u128 << bits.div_ceil(2)
}

/// Write a safe over-estimate seed for `floor(cbrt(n))` into `out`.
///
/// Same contract and convergence guarantee as [`sqrt_seed`].
///
/// `std`: hardware `f64::cbrt` bootstrap. `cbrt(top · 2^shift) =
/// cbrt(top) · 2^(shift/3)`; the `shift % 3` residue is folded into the
/// **exact** fractional `2^(r/3)` multiplier — `2^(1/3) ≈ 1.2599` for
/// `r=1`, `2^(2/3) ≈ 1.5874` for `r=2` — then rounded up to a strict
/// over-estimate. (The earlier coarse `2^r` integer multiplier — `×2` and
/// `×4` — over-shot the true cbrt by up to ~2.52× and cost the Newton loop
/// several extra `O(W²)` divides per call; the exact fractional multiplier
/// keeps the seed within ~`1 + 2⁻⁵²` of `∛n`, which is the whole tightness
/// win.) `no_std`: classical pure-integer 1-bit seed `2^ceil(bits/3)` (the
/// root has at most `ceil(bits/3)` bits, so this over-estimates). The
/// Brent–Zimmermann downward Newton iteration self-corrects to the floor
/// cube root from any positive over-estimate, so the convergence guarantee
/// is unchanged — only the iteration *count* improves.
///
/// Hasselgren seed strategy — Crandall & Pomerance 2005 §9.2.1.
#[inline]
pub(crate) fn cbrt_seed(n: &[u64], bits: u32, out: &mut [u64]) {
    let work = out.len();

    #[cfg(feature = "std")]
    if bits >= 9 {
        let (top_u64, shift) = extract_top_u64(n, bits);
        let rem3 = shift % 3;
        // Exact fractional residue: cbrt(top · 2^shift) =
        // cbrt(top) · 2^(shift/3); the integral `shift/3` is absorbed into
        // `half_shift` below, the fractional residue stays here as an
        // exact f64 multiplier (vs the earlier coarse `1u64 << rem3` ≡
        // `×2` / `×4`).
        let factor = match rem3 {
            1 => 1.259_921_049_894_873_2_f64, // 2^(1/3)
            2 => 1.587_401_051_968_199_5_f64, // 2^(2/3)
            _ => 1.0_f64,
        };
        let seed_f64 = (top_u64 as f64).cbrt() * factor;
        // half_shift = shift / 3 (the fractional third absorbed above).
        let half_shift = shift / 3;
        let truncated = seed_f64 as u128;
        let frac_nonzero = (truncated as f64) != seed_f64;
        // +1 for any truncated fraction, +1 for a strict over-estimate
        // (the f64 cbrt + multiply each round-rounded to ≤ 1 ULP, well
        // within this `+1`). Mirrors the `sqrt_seed` margin.
        let seed_int: u128 = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        place_seed(seed_int, half_shift, out, work);
        return;
    }

    // Pure-integer 1-bit seed `2^ceil(bits/3)` — the no_std path always,
    // and the `std` tiny-`n` fallback.
    let _ = n;
    let e = bits.div_ceil(3);
    let idx = (e / 64) as usize;
    if idx < work {
        out[idx] |= 1u64 << (e % 64);
    } else if work > 0 {
        out[0] |= 1;
    }
}

/// Place `seed_int << half_shift` into `out` (OR-ed), then guarantee a
/// non-zero seed. Pure primitive limb writes — part of the leaf, shared by
/// both `std` seed bodies.
#[cfg(feature = "std")]
#[inline]
fn place_seed(seed_int: u128, half_shift: u32, out: &mut [u64], work: usize) {
    let seed_limb_idx = (half_shift / 64) as usize;
    let seed_bit_off = half_shift % 64;
    let shifted: u128 = seed_int << seed_bit_off;
    let seed_lo = shifted as u64;
    let seed_hi = (shifted >> 64) as u64;
    if seed_limb_idx < work {
        out[seed_limb_idx] |= seed_lo;
    }
    if seed_limb_idx + 1 < work {
        out[seed_limb_idx + 1] |= seed_hi;
    }
    // If the placement landed entirely outside the work slice, force a
    // minimal non-zero seed so the Newton loop's `x >= 1` invariant holds.
    let mut all_zero = true;
    let mut i = 0;
    while i < work {
        if out[i] != 0 {
            all_zero = false;
            break;
        }
        i += 1;
    }
    if all_zero && work > 0 {
        out[0] = 1;
    }
}
