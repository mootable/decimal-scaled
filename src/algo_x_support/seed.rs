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
//! **fixed-point** over-estimate `seed_int` of `√top · 2^F` (of `√(2·top) · 2^F`
//! when `shift` is odd), with `F = min(h, SEED_FRAC_BITS)` fractional bits and
//! `h = ⌊shift/2⌋`, via `⌊f64::sqrt(top) · 2^F⌋ + [frac≠0] + 1` — note the
//! **always-`+1`** — and returns `seed_int · 2^{h−F}`. (`F = 0` recovers the
//! plain integer form; the fractional bits exist so a big `2^h` placement does
//! not balloon the integer-quantisation margin to a relative `2^{-31}` — with
//! them the seed is accurate to a relative `~2^{-49}`, which is what keeps the
//! consumers' Newton divide count at ~1 for roots up to ~98 bits.)
//!
//! **Even `shift = 2h`.** `√n ≤ √((top+1)·2^{2h}) = √(top+1)·2^h`, so it
//! suffices that `seed_int ≥ √(top+1)·2^F`. The body gives
//! `seed_int ≥ √top·2^F − d + 1` where `d` is the f64 deficit (next paragraph,
//! `d < 0.4`), and the gap to cover is
//! `(√(top+1) − √top)·2^F = 2^F/(√(top+1)+√top) ≤ 2^{F−32.5} ≤ 2^{−15.5}`
//! (`top ≥ 2^{63}` whenever `shift ≥ 1`; for `shift = 0` the window is exact,
//! `F = h = 0`, and the original integer-lemma `(√t+1)² ≥ t+1` applies).
//! Since `1 − d > 0.6 > 2^{−15.5}`, `seed = seed_int·2^{h−F} ≥ √n`. ∎
//! The always-`+1` is **essential**: it closes the perfect-square-`top` case
//! (`⌊√top⌋ = √top` exactly), where the low bits `r` push `√n` strictly above
//! `√top·2^h`. Without it the seed under-estimates there — a failure of
//! density `2^{-32}` that uniform-random tests practically never hit.
//!
//! **Odd `shift = 2h+1`.** `√n ≤ √(2(top+1))·2^h`; the body over-estimates
//! `√2·√top·2^F = √(2·top)·2^F`, the gap `(√(2(top+1))−√(2·top))·2^F ≤ 2^{−15}`
//! and the deficit bound below still leave the `+1` dominant; hence
//! `seed ≥ √n`. ∎
//!
//! **f64 rounding (the deficit `d`).** `top ≤ 2^64` rounds to the nearest f64
//! (relative `≤ 2^{−53}`), `f64::sqrt` is correctly rounded (`≤ 2^{−53}`), the
//! odd-shift `SQRT_2` constant + multiply add `≤ 2·2^{−53}`, and the `· 2^F`
//! scale is EXACT (power of two). Total relative deficit `≤ ~2^{−51}` on a
//! magnitude `√(2·top)·2^F ≤ 2^{32.5+F} ≤ 2^{49.5}` (with
//! `SEED_FRAC_BITS = 17`), i.e. `d < 0.4` absolute — dominated by the
//! always-`+1`. This bound is why `SEED_FRAC_BITS` must stay `≤ 17`: at higher
//! `F` the deficit approaches/exceeds 1 and the `+1` no longer covers it.
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

/// Fractional fixed-point bits the `std` sqrt seed bodies keep below the
/// integer part of the unscaled window root (capped by the placement shift
/// `h`). The cap is part of the over-estimate PROOF (module doc, "f64
/// rounding"): at `F ≤ 17` the f64 deficit stays `< 0.4` absolute so the
/// always-`+1` margin still strictly covers it — do NOT raise this without
/// re-deriving that bound.
#[cfg(feature = "std")]
const SEED_FRAC_BITS: u32 = 17;

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
        // Keep `frac_bits` of the f64 mantissa's FRACTIONAL bits in fixed
        // point before placing: quantising `seed_f64` to a bare integer first
        // costs the seed everything below 1 ulp of the
        // ~2^32-magnitude window root, so after the `· 2^half_shift`
        // placement the margin balloons to a relative ~2^-31 and every
        // big-shift Newton consumer pays ~2 extra divide iterations to win
        // those bits back. Scaling by `2^frac_bits` (an EXACT power-of-two
        // f64 multiply) keeps the seed accurate to a relative ~2^-49 instead.
        // The over-estimate proof carries over (module doc): with
        // `frac_bits <= 17` the scaled magnitude is `<= 2^49.5`, so the f64
        // path's total relative deficit (~2^-51) is `< 0.4` absolute and the
        // always-`+1` still strictly dominates it plus the perfect-square-top
        // gap `(sqrt(top+1)-sqrt(top))·2^frac_bits <= 2^-15`.
        let frac_bits = half_shift.min(SEED_FRAC_BITS);
        let scaled = seed_f64 * (1u64 << frac_bits) as f64;
        let truncated = scaled as u128;
        let frac_nonzero = (truncated as f64) != scaled;
        let seed_int: u128 = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        place_seed(seed_int, half_shift - frac_bits, out, work);
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
        // Fixed-point fractional bits, exactly as in `sqrt_seed`'s std body
        // (see the comment there): an EXACT power-of-two scale keeps the
        // placed seed accurate to a relative ~2^-49 instead of ~2^-31, saving
        // the Newton consumers ~2 divide iterations at big shifts. Same
        // over-estimate margin (+1 fraction round-up, +1 strict).
        let frac_bits = half_shift.min(SEED_FRAC_BITS);
        let scaled = seed_f64 * (1u64 << frac_bits) as f64;
        let truncated = scaled as u128;
        let frac_nonzero = (truncated as f64) != scaled;
        let seed_int = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        return seed_int << (half_shift - frac_bits);
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

/// Full-radicand `f64` cube-root seed: `value.cbrt()`, where `value` is the
/// radicand's `f64` approximation (`Int::<W>::as_f64()`). This is the tight
/// seed — the cube root of the WHOLE radicand, not the top-bits
/// window [`cbrt_seed`] extracts — correctly rounded to ~53 bits, so it sits
/// within ~`2⁻⁵²` *relative* of `∛n`. The caller bridges the result back to
/// `Int<W>` (`from_f64`) and lifts it above `⌈∛n⌉` with one Newton pre-step
/// before the downward-monotone loop, so the floor cube root is reached
/// identically regardless of the seed's tightness.
///
/// **Std-only — no `no_std` arm.** Unlike [`cbrt_seed`], the full-radicand
/// recipe has no pure-integer analogue: it is the inherent `f64::cbrt`
/// intrinsic applied to the whole magnitude. A `no_std` cube-root kernel
/// seeds through [`cbrt_seed`] (the top-bits window) instead, so this leaf is
/// gated to `std` exactly like its sole caller
/// (`cbrt_native_fast::icbrt_w_f64_full`). The caller MUST guarantee `value`
/// is finite and in range (`as_f64` would otherwise saturate to `±inf`).
#[cfg(feature = "std")]
#[inline]
pub(crate) fn cbrt_seed_f64_full(value: f64) -> f64 {
    value.cbrt()
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
