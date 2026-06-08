//! Width-generic guard-digit `exp` core (always compiled).
//!
//! The per-tier `$core` modules (emitted by
//! [`decl_wide_transcendental!`](crate::macros::wide_transcendental))
//! bind an `exp_fixed` to one work integer `W`. Near the storage-overflow
//! edge — a large `e^|x|` whose integer-digit growth, the internal
//! `2^k`-reassembly lift, AND the repeated-squaring peak together exceed
//! `W`'s decimal capacity — the value can no longer be held at the
//! precision needed to round correctly.
//!
//! This module lifts the `exp_fixed` body out to a free function generic
//! over any [`BigInt`] integer `S` (whose scratch carrier impls [`ComputeLimbs`]), so a large-result
//! cell can run it in a WIDER work integer (the wide tiers' `Wexp`, or the
//! narrow D18/D38 tier's wider work integer) where the full lift + squaring
//! peak fit, then narrow correctly-rounded back to storage. The module is
//! always compiled (NOT gated behind `_wide-support`) precisely so the
//! narrow default build reaches it for the integer-regime D38 cells whose
//! 256-bit `Fixed` intermediate cannot host the lift.

#![allow(unused)]

use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

    /// Hard cap on series iterations — a safety net; every series
    /// terminates far sooner by reaching a zero term.
    const SERIES_CAP: u128 = 20_000;

    #[inline]
    pub(crate) fn lit<S: BigInt>(n: i128) -> S {
        S::from_i128(n)
    }
    #[inline]
    pub(crate) fn zero<S: BigInt>() -> S {
        S::ZERO
    }
    #[inline]
    fn abs<S: BigInt>(v: S) -> S {
        if v < S::ZERO { -v } else { v }
    }
    #[inline]
    pub(crate) fn pow10<S: BigInt>(n: u32) -> S {
        crate::consts::pow10::dispatch::<S>(n)
    }
    #[inline]
    pub(crate) fn one<S: BigInt>(w: u32) -> S {
        pow10::<S>(w)
    }
    /// Bit length of `|v|` (0 for zero).
    pub(crate) fn bit_length<S: BigInt>(v: S) -> u32 {
        <S as BigInt>::BITS - abs(v).leading_zeros()
    }
    /// Half-to-even round of `numerator / divisor` for `S`.
    #[inline]
    pub(crate) fn round_div<S: BigInt>(n: S, d: S) -> S {
        let (q, r) = n.div_rem(d);
        if r == S::ZERO {
            return q;
        }
        let ar = abs(r);
        let comp = abs(d) - ar;
        let cmp_r = if ar < comp {
            ::core::cmp::Ordering::Less
        } else if ar > comp {
            ::core::cmp::Ordering::Greater
        } else {
            ::core::cmp::Ordering::Equal
        };
        let q_is_odd = q.bit(0);
        let result_positive = (n < S::ZERO) == (d < S::ZERO);
        if crate::support::rounding::should_bump(
            RoundingMode::HalfToEven,
            cmp_r,
            q_is_odd,
            result_positive,
        ) {
            if result_positive {
                q + S::ONE
            } else {
                q - S::ONE
            }
        } else {
            q
        }
    }
    /// Half-to-even quotient `n / 10^w`, via the MG (magic-multiply)
    /// reciprocal — the same fast divide the per-tier
    /// `decl_wide_transcendental!` core uses, here for the width-generic
    /// path the hyperbolics run through. For `1 ≤ w ≤ 38` the
    /// single-chunk base-`2^128` kernel; for `w > 38` the chain of
    /// `÷ 10^38` stages. Bit-identical to the generic `round_div(n,
    /// 10^w)` (audited in `mg_divide::tests`), but replaces the
    /// per-Taylor-term 256-limb Knuth division that dominated the wide
    /// hyperbolic/exp cost. The buffer comes from `S`'s scratch carrier ([`ComputeLimbs`]), so no
    /// const-generic limb count appears here.
    #[inline]
    pub(crate) fn round_div_pow10<S: BigInt>(n: S, w: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        if w == 0 {
            return n;
        }
        if w <= 38 {
            return crate::algos::support::mg_divide::div_wide_pow10::<S>(
                n,
                w,
                RoundingMode::HalfToEven,
            );
        }
        // `w > 38` rescale: route through the rescale MATCHER (not
        // `div_wide_pow10_chain` directly) so the wide / high-scale band gets
        // the baked-reciprocal Newton arm + the 9.24 magnitude-trim, exactly
        // as the per-tier `wide_transcendental` cores do. The matcher only
        // deviates from `MgChain` where its pick is faster, and every selected
        // kernel is bit-identical (a fixed-mode `÷10^w` has one correct
        // answer — the rescale validity wall), so this is value-neutral and
        // never slower. Single source for the wide rescale across exp/ln/the
        // generic Tang kernel.
        crate::algos::support::rescale::dispatch_wide_pow10::<S>(
            n,
            w,
            RoundingMode::HalfToEven,
        )
    }
    /// `(a · b) / 10^w`, rounded half-to-even.
    #[inline]
    pub(crate) fn mul<S: BigInt>(a: S, b: S, w: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        // u128-packed wide multiply: bit-identical to `a * b` (it IS the low
        // product) for even-limb work widths, ~1/4 the partial products;
        // falls back to the base-2^64 schoolbook for odd N. This is the hot
        // Taylor-term / squaring multiply, run at `Wexp` (up to Int<256>) for
        // exp + the hyperbolics — the fix for the ~12× wide-tier regression.
        round_div_pow10(a.wrapping_mul_low_u128(b), w)
    }
    /// Loop-friendly `mul` with a precomputed `10^w` divisor.
    #[inline]
    fn mul_cached<S: BigInt>(a: S, b: S, pow10_w: S) -> S {
        round_div(a.wrapping_mul_low_u128(b), pow10_w)
    }
    /// `(a · 10^w) / b`, rounded half-to-even (precomputed numerator
    /// factor).
    #[inline]
    pub(crate) fn div_cached<S: BigInt>(a: S, b: S, pow10_w: S) -> S {
        round_div(a.wrapping_mul_low_u128(pow10_w), b)
    }
    /// `a · n` for a small unsigned multiplier.
    #[inline]
    fn mul_u<S: BigInt>(a: S, n: u128) -> S {
        if n <= u64::MAX as u128 {
            a.checked_mul_u64(n as u64)
        } else {
            a * S::from_i128(n as i128)
        }
    }
    /// `k · c` where `k` is a signed range-reduction count.
    #[inline]
    fn scale_by_k<S: BigInt>(c: S, k: i128) -> S {
        if k >= 0 {
            mul_u(c, k as u128)
        } else {
            -mul_u(c, k.unsigned_abs())
        }
    }
    /// Rounds a working-scale value to the nearest integer (ties away
    /// from zero); used for the range-reduction quotient.
    pub(crate) fn round_to_nearest_int<S: BigInt>(v: S, w: u32) -> i128 {
        let divisor = pow10::<S>(w);
        let (q, r) = v.div_rem(divisor);
        let half = divisor >> 1;
        let qi = if abs(r) >= half {
            if v < S::ZERO { q - S::ONE } else { q + S::ONE }
        } else {
            q
        };
        crate::int::types::traits::BigInt::to_i128(qi)
    }

    /// `ln 2` at working scale `w`, sourced from the unified constant
    /// table (`consts::ln2_by_working_scale`) — a static lookup +
    /// zero-extend, NOT a recompute. Replaces the former `2·artanh(1/3)`
    /// series (~`w` terms), which dominated the wide-tier exp/hyperbolic
    /// cost; the table's `ln2` band is sized (gen_const_table.py
    /// `LN2_MAXES`) to the peak `w_ext` this path can request. Mode is
    /// half-to-even, matching the per-tier core's `ln2_cf`.
    fn ln2<S: BigInt>(w: u32) -> S {
        crate::consts::ln2_by_working_scale::<S>(w, RoundingMode::HalfToEven)
    }

    /// `√v` at working scale `w`: `√(|v| · 10^w)`, truncating. Width-generic
    /// twin of the per-tier `$core::sqrt_fixed` (the multi-level argument
    /// reduction `ln_fixed` runs); bit-identical (same seed-library bootstrap
    /// + monotone-downward Newton). `|v| · 10^w` must fit `S`.
    pub(crate) fn sqrt_fixed<S: BigInt>(v: S, w: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let av = abs::<S>(v);
        let n = av * pow10::<S>(w);
        if n <= zero::<S>() {
            return zero::<S>();
        }
        // Seed from the shared cross-algorithm seed leaf (std f64 bootstrap /
        // no_std 1-bit), both guaranteed over-estimates, so the AM-GM pre-step
        // + monotone-downward loop converge to the identical floor either way.
        let seed = crate::algos::support::seed_bridge::sqrt_seed_w::<S>(n);
        let x0 = if seed <= zero::<S>() { lit::<S>(1) } else { seed };
        let mut x = (x0 + n / x0) >> 1;
        loop {
            let y = (x + n / x) >> 1;
            if y >= x {
                return x;
            }
            x = y;
        }
    }

    /// Natural logarithm of a positive working-scale value, generic over the
    /// work integer `S`. Width-generic twin of the per-tier
    /// `$core::ln_fixed`: range-reduces `v = 2^k·m` with `m ∈ [1, 2)`, applies
    /// `sqrt_l` levels of sqrt argument reduction (Brent 1976), evaluates
    /// `ln(m) = 2^(l+1)·artanh((m−1)/(m+1))`, returns `k·ln2 + ln(m)`. `ln2_w`
    /// is `ln 2` at scale `w`, supplied by the caller (the primitive wrapper
    /// passes the const-folded `ln2_cf::<SCALE>`; a composition passes its
    /// wide-work `ln2`), so this stays free of the `SCALE` const. Bit-identical
    /// to the per-tier core for the same `(v, w, ln2_w)`.
    pub(crate) fn ln_fixed<S: BigInt>(v_w: S, w: u32, ln2_w: S) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let one_w = one::<S>(w);
        let two_w = one_w + one_w;
        let pow10_w = one_w;
        let mut k: i32 = bit_length::<S>(v_w) as i32 - bit_length::<S>(one_w) as i32;
        let mut m_w = loop {
            let m = if k >= 0 {
                v_w >> (k as u32)
            } else {
                v_w << ((-k) as u32)
            };
            if m >= two_w {
                k += 1;
            } else if m < one_w {
                k -= 1;
            } else {
                break m;
            }
        };

        // Multi-level sqrt argument reduction: `l ≈ √p_bits / 4`.
        let p_bits = w.saturating_mul(3).saturating_add(1);
        let sqrt_l: u32 = {
            let mut n: u32 = 0;
            while (n + 1) * (n + 1) <= p_bits {
                n += 1;
            }
            n / 4
        };
        let mut i = 0;
        while i < sqrt_l {
            m_w = sqrt_fixed::<S>(m_w, w);
            i += 1;
        }

        let t = div_cached::<S>(m_w - one_w, m_w + one_w, pow10_w);
        let t2 = mul::<S>(t, t, w);
        let mut sum = t;
        let mut term = t;
        let mut j: u128 = 1;
        loop {
            term = mul::<S>(term, t2, w);
            let contrib = term / lit::<S>((2 * j + 1) as i128);
            if contrib == zero::<S>() {
                break;
            }
            sum = sum + contrib;
            j += 1;
            if j > SERIES_CAP {
                break;
            }
        }
        // ln(m) = 2^(l+1)·artanh(t) = sum << (sqrt_l + 1).
        let ln_m = sum << (sqrt_l + 1);
        scale_by_k::<S>(ln2_w, k as i128) + ln_m
    }

    /// True worst-case bit-width the [`exp_fixed`] body reaches internally
    /// for a working-scale value `v_w` at scale `w`, in a work integer `S`
    /// of capacity `S::BITS` bits.
    ///
    /// Mirrors [`exp_fixed`]'s own `k` / `extra` / `w_ext` arithmetic
    /// EXACTLY (range-reduce `v = k·ln2 + s`, lift the working scale by
    /// `extra` digits, run the Taylor squarings at `w_ext`, then reassemble
    /// `2^k · exp(s)`), so the fit gate models the real squaring-reassembly
    /// PEAK — `2·w_ext` decimal digits for the symmetric `sum²` plus the
    /// `sum << k` shift — NOT just the final result magnitude. The body's
    /// `wrapping_sqr_low_u128` / `wrapping_mul_low_u128` return the low bits,
    /// so an internal peak that exceeds `S::BITS` silently TRUNCATES (an
    /// overflowed square collapses to 0) and the post-narrowing fit check —
    /// which only sees the wrapped, small result — never fires. This model
    /// lets [`exp_fixed`] reject such an argument UP FRONT instead.
    ///
    /// This is the width-generic single source for the peak estimate; the
    /// per-tier `decl_wide_transcendental!` `exp_internal_peak_bits` /
    /// `exp_fits_w` / `hyper_fits_w` gates delegate to it.
    pub(crate) fn exp_internal_peak_bits<S: BigInt>(v_w: S, w: u32) -> u64 {
        let one_w_pre = one::<S>(w);
        let l2_pre = ln2::<S>(w);
        let k = round_to_nearest_int(div_cached(v_w, l2_pre, one_w_pre), w);
        let abs_k_u128 = if k < 0 { -k } else { k } as u128;
        let extra: u32 = if abs_k_u128 == 0 {
            0
        } else {
            let digits = (abs_k_u128 * 30103).div_ceil(100_000);
            let capped = digits.min((<S as BigInt>::BITS / 4) as u128) as u32;
            capped + 12 + (capped >> 2)
        };
        let w_ext = (w + extra) as u64;
        // digits → bits: `log2(10) ≈ 3.3220 ≈ 3322/1000`.
        // Squaring peak: the symmetric `sum²` before the round-divide spans
        // `2·w_ext` decimal digits.
        let sqr_bits = 2 * w_ext * 3322 / 1000;
        // Reassembly peak: `sum << k` lifts the `w_ext`-digit Taylor sum by
        // `|k|` bits.
        let reasm_bits = w_ext * 3322 / 1000 + abs_k_u128 as u64;
        let peak = if sqr_bits > reasm_bits { sqr_bits } else { reasm_bits };
        // Small safety slack on top of the modelled peak. The model can
        // under-count the TRUE internal peak by only a few bits: `sum` can
        // reach `√2·10^w_ext` (e^(ln2/2)), so the symmetric `sum²` reaches
        // `2·10^(2·w_ext)` — `2·w_ext` digits PLUS the leading factor `2`
        // (≈ +2 bits the `2·w_ext·3322/1000` digit count omits) — plus the
        // half-LSB residue of the rounded `÷10^w_ext`. ~4 bits suffices to
        // keep `peak` an UPPER bound (so the gate never lets a genuine wrap
        // through); one u64 limb (64) is a generous, clean pad.
        //
        // The slack MUST stay small. It is a flat additive bit count, so on a
        // tier whose work integer `S` is NARROW it eats a large fraction of
        // the budget — and D76 is the sole tier whose `Wexp == W == Int<16>`
        // (1024 bits), so a value that overflows `W` cannot lift to anything
        // wider: the gate IS the last line, and an over-large slack
        // false-panics in-range band-edge cosh/sinh/exp (sqr_bits ≈ 910 at the
        // D76<0> edge x≈175). 64 clears that edge with room while every other
        // tier's wider `Wexp` absorbs it. A result that genuinely overflows
        // STORAGE but still fits `S` is NOT this gate's concern — it is caught
        // downstream by the narrowing fit check (`round_to_storage_with_g`,
        // which panics "result out of range"); this gate guards only the work
        // integer `S` itself wrapping.
        peak + 64
    }

    /// Whether [`exp_fixed`]'s internal squaring-reassembly peak for
    /// `(v_w, w)` fits the work integer `S` without wrapping. Used by the
    /// per-tier `exp_fits_w` / `hyper_fits_w` regime-routing gates.
    #[inline]
    pub(crate) fn exp_peak_fits<S: BigInt>(v_w: S, w: u32) -> bool {
        exp_internal_peak_bits::<S>(v_w, w) < <S as BigInt>::BITS as u64
    }

    /// `e^v` for a working-scale value `v`, generic over the work
    /// integer `S`. Mirrors the per-tier `$core::exp_fixed` exactly
    /// (range-reduce `v = k·ln2 + s`, extend the working scale by
    /// `extra` to absorb the `2^k` amplification, run the
    /// repeated-squaring Taylor core, reassemble `2^k · exp(s)`), but
    /// stays width-generic so the caller can run it in a wider integer
    /// for the large-result regime.
    ///
    /// # Panics
    ///
    /// Panics with the stable `"result out of range"` substring when the
    /// argument is so large that the internal squaring / `2^k`-reassembly
    /// peak would exceed the work integer `S`'s capacity. The body reduces
    /// modulo `2^BITS` (`wrapping_sqr_low_u128`), so an unchecked overflow
    /// here would silently TRUNCATE — collapsing a far-out-of-range result
    /// to a small (often zero) value that then slips through the caller's
    /// post-narrowing fit check. Failing loudly at the work integer it can
    /// no longer represent keeps the strict-transcendental overflow contract
    /// uniform: a result out of range PANICS at every tier and scale (in
    /// both debug and release), never returns a wrapped value. The caller
    /// runs this in the WIDEST work integer it can (`Wexp` / `WNarrow`); the
    /// panic fires only when even that cannot hold the peak — a genuinely
    /// unrepresentable result.
    pub(crate) fn exp_fixed<S: BigInt>(v_w: S, w: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let one_w_pre = one::<S>(w);
        let l2_pre = ln2::<S>(w);
        let pow10_w_pre = one_w_pre;
        let k = round_to_nearest_int(div_cached(v_w, l2_pre, pow10_w_pre), w);
        // Deep underflow: e^v < 10^-w, so its working value is sub-resolution. For
        // a very negative k the extra-guard range reduction below provisions
        // `extra ≈ |k|·0.3` digits, pushing `w_ext` and the `k·ln2` term past the
        // work integer S's capacity (an `Int: mul overflow`). Short-circuit to the
        // smallest positive working value, preserving the positive sub-resolution
        // so the caller rounds correctly (0 under nearest, the smallest positive
        // under Ceiling). Sufficient condition: e^v < 2^(k+1) <= 10^-w, i.e.
        // -(k+1)·log10(2) >= w  (log10(2) ≈ 30103/100000).
        if k < -1 {
            let neg = (-(k + 1)) as u128;
            if neg.saturating_mul(30103) >= (w as u128).saturating_mul(100_000) {
                return lit::<S>(1);
            }
        }
        // Overflow guard (positive results only). For `k >= 0`, `e^v >= 1` and
        // grows without bound; once the internal squaring / `2^k`-reassembly
        // peak exceeds `S::BITS` the body's `wrapping_*` arithmetic would
        // silently TRUNCATE the result (an overflowed square collapses to 0),
        // and the caller's post-narrowing fit check — seeing only the wrapped,
        // small value — would never fire, letting a far-out-of-range result
        // escape as a wrong (often zero) value. A fixed-width decimal has no
        // ∞/NaN, so there is nothing to return: PANIC, uniform across every
        // tier and scale, in both debug and release. The caller runs this in
        // the WIDEST work integer it can (`Wexp` / `WNarrow`); the panic fires
        // only when even that cannot hold the peak — a genuinely
        // unrepresentable result. (`k < 0` is the underflow direction, handled
        // by the short-circuits above and below — never panicked.)
        if k >= 0 && !exp_peak_fits::<S>(v_w, w) {
            panic!("exp_generic::exp_fixed: result out of range");
        }
        let abs_k_u128 = if k < 0 { -k } else { k } as u128;
        let extra: u32 = if abs_k_u128 == 0 {
            0
        } else {
            let digits = (abs_k_u128 * 30103).div_ceil(100_000);
            let capped = digits.min((<S as BigInt>::BITS / 4) as u128) as u32;
            capped + 12 + (capped >> 2)
        };

        let w_ext = w + extra;
        let v_ext = if extra == 0 {
            v_w
        } else {
            v_w * pow10::<S>(extra)
        };
        let one_w = one::<S>(w_ext);
        let l2 = ln2::<S>(w_ext);
        let s = v_ext - scale_by_k(l2, k);

        let p_bits = w_ext.saturating_mul(3).saturating_add(1);
        let mut n: u32 = 1;
        while (n + 1) * (n + 1) <= p_bits {
            n += 1;
        }

        let s_red = s >> n;
        let mut sum = one_w + s_red;
        let mut term = s_red;
        let mut iter: u128 = 2;
        loop {
            term = mul(term, s_red, w_ext) / lit::<S>(iter as i128);
            if term == S::ZERO {
                break;
            }
            sum = sum + term;
            iter += 1;
            if iter > SERIES_CAP {
                break;
            }
        }

        let mut squared = sum;
        let mut i = 0;
        while i < n {
            // Dedicated low-half symmetric SQUARE through the limb-width
            // matcher (`wrapping_sqr_low_u128` → `int::policy::sqr_low`): the
            // u128-packed `sqr_low_limb` on even work widths (half the limbs),
            // bit-identical to the low-`BITS` of `x²`. The squaring sibling of
            // the Taylor `mul`'s `wrapping_mul_low_u128`; feeds the same divide.
            squared = round_div_pow10(squared.wrapping_sqr_low_u128(), w_ext);
            i += 1;
        }
        let sum = squared;

        let scaled_at_w_ext = if k >= 0 {
            let shift = k as u32;
            if bit_length(sum) + shift >= <S as BigInt>::BITS {
                panic!("exp_generic::exp_fixed: result out of range");
            }
            sum << shift
        } else {
            let neg_k = -k as u128;
            if neg_k >= bit_length(sum) as u128 {
                // Deep underflow: e^x (x < 0 here, since k < 0) is strictly
                // positive but below the working resolution. Return the
                // smallest positive working value (1 = 10^-w), NOT zero, so the
                // directed narrowing preserves the sign — Ceiling rounds up to
                // 1 ULP while Floor / Trunc / nearest still give 0. Returning a
                // bare zero loses positivity and rounds Ceiling to 0 (a
                // correctly-rounded defect). Reached only by direct e^(negative)
                // — the hyperbolics call `exp_fixed` on |x| >= 0.
                return lit::<S>(1);
            }
            sum >> (neg_k as u32)
        };
        let result = if extra == 0 {
            scaled_at_w_ext
        } else {
            round_div_pow10(scaled_at_w_ext, extra)
        };
        // e^v > 0 for every finite v: a zero result here is genuine underflow
        // of `e^(negative)` below the working resolution, not a true zero.
        // Return the smallest positive value so the directed narrowing rounds
        // Ceiling up to 1 ULP (a bare zero would round Ceiling to 0 — a
        // correctly-rounded defect). Restricted to `k < 0`: for `k >= 0`,
        // `e^v >= 1`, so a 0 result would mean the working width overflowed,
        // and masking it as 1 would hide the defect rather than fix it.
        if k < 0 && result == zero::<S>() {
            lit::<S>(1)
        } else {
            result
        }
    }

    /// Narrows a `Wexp`-computed working value `v` back down to the tier's
    /// own work integer `Dst`, panicking UNIFORMLY when it does not fit.
    ///
    /// The wide `exp` / hyperbolic compositions evaluate in the wider `Wexp`
    /// (their squaring peak needs the extra width — that is why the per-tier
    /// `exp_fits_w` / `hyper_fits_w` gate lifted them there), then narrow the
    /// result back to the tier work integer `Dst`. A genuinely out-of-range
    /// result — `e^|x|` larger than the tier can represent — is correctly
    /// computed at `Wexp` but EXCEEDS `Dst` at this step. A bare
    /// [`BigInt::resize_to`] would silently TRUNCATE it to `Dst`'s low bits,
    /// yielding a small wrapped value that then slips through the downstream
    /// storage-narrowing fit check — the exact silent-overflow the strict
    /// transcendental contract forbids. Detect it here and PANIC instead
    /// ("result out of range", identical at every tier and scale, in both
    /// debug and release).
    ///
    /// This is the structural twin of [`exp_fixed`]'s own peak gate: the gate
    /// guards the *work integer wrapping* during the squaring; this guards the
    /// *narrow back to the tier width* once the (correct) result is in hand.
    /// In-range results provably fit `Dst` — the tier work integer holds any
    /// value whose storage representation is in range at the lifted working
    /// scale — so this never fires for a representable cell. When
    /// `Dst == Wexp` (the widest tier, where no narrowing happens) it is a
    /// cheap bit-length compare that always passes. The hyperbolic results are
    /// non-negative (`e^|x|`, `cosh`, `sinh(|x|)`), so the magnitude test is
    /// exact: a value needing `≥ Dst::BITS` significant bits cannot fit the
    /// signed `Dst`.
    #[inline]
    pub(crate) fn resize_or_panic<Src: BigInt, Dst: BigInt>(v: Src) -> Dst {
        if bit_length::<Src>(abs::<Src>(v)) >= <Dst as BigInt>::BITS {
            panic!("exp_generic: result out of range");
        }
        <Src as BigInt>::resize_to::<Dst>(v)
    }

    /// `(a · 10^w) / b`, rounded half-to-even (the generic sibling of
    /// the per-tier `$core::div`).
    #[inline]
    pub(crate) fn div<S: BigInt>(a: S, b: S, w: u32) -> S {
        // `(a·10^w)/b`, half-to-even. `10^w` comes from the `pow10` POLICY
        // (`pow10::dispatch`, via `pow10::<S>`), NOT a per-tier baked static;
        // the numerator product is the u128-packed truncated-low mul (the
        // macro `div`'s kernel) so routing through the policy costs no
        // multiply speed.
        round_div(a.wrapping_mul_low_u128(pow10::<S>(w)), b)
    }

    /// `sinh(|x|)` at working scale `w` for a non-negative working
    /// value `av_w` (= `|x|·10^w`), computed entirely in `S`:
    /// `(e^|x| − e^-|x|)/2`. The dominant `e^|x|` term is evaluated
    /// directly (`exp_fixed`) and the small `e^-|x|` via reciprocal, so
    /// the small term's relative error stays a small *absolute* error.
    pub(crate) fn sinh_pos<S: BigInt>(av_w: S, w: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let ex = exp_fixed::<S>(av_w, w);
        let enx = div(one::<S>(w), ex, w);
        (ex - enx) >> 1
    }

    /// `cosh(|x|) = (e^|x| + e^-|x|)/2` at working scale `w`. See
    /// [`sinh_pos`].
    pub(crate) fn cosh_pos<S: BigInt>(av_w: S, w: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let ex = exp_fixed::<S>(av_w, w);
        let enx = div(one::<S>(w), ex, w);
        (ex + enx) >> 1
    }

    /// `tanh(|x|) = (e^|x| − e^-|x|)/(e^|x| + e^-|x|)` at working scale
    /// `w`. See [`sinh_pos`].
    pub(crate) fn tanh_pos<S: BigInt>(av_w: S, w: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let one_w = one::<S>(w);
        // Large |x|: tanh(|x|) = 1 − 2·e^(−2|x|), which to scale w is all-nines
        // once 2·e^(−2|x|) < 10^−w, i.e. |x| ≳ ln(10)/2·w ≈ 1.1513·w. Computing
        // e^|x| there would overflow the work integer's internal range reduction,
        // so return the largest working value below 1 (1 − 10^−w); the caller
        // rounds it to 1 under nearest/up and to 1−ulp under Floor/Trunc.
        let thr_x = (w as i128) * 1152 / 1000 + 2;
        if av_w / one_w > lit::<S>(thr_x) {
            return one_w - lit::<S>(1);
        }
        let ex = exp_fixed::<S>(av_w, w);
        let enx = div(one_w, ex, w);
        div(ex - enx, ex + enx, w)
    }
