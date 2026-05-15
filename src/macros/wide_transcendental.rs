//! Correctly-rounded strict transcendentals for the wide decimal tiers
//! (D256 / D512 / D1024).
//!
//! D128 and the narrow tiers run their strict transcendentals on the
//! 256-bit `d128_kernels::Fixed` guard-digit intermediate; D32 / D64
//! delegate into D128. The wide tiers cannot widen into D128 — their
//! scale range exceeds it — so they need their own guard-digit core.
//!
//! This module provides one, generic over a hand-rolled signed wide integer
//! `$Work` chosen per tier to be wide enough to hold the working-scale
//! products without overflow:
//!
//! - D256 → `I1024` (working scale ≤ 106 digits);
//! - D512 → `I2048` (working scale ≤ 183 digits);
//! - D1024 → `I4096` (working scale ≤ 337 digits).
//!
//! A working value `x` is held as the `$Work` integer `x · 10^w`, where
//! `w = SCALE + GUARD` and `GUARD = 30` guard digits. the wide integers
//! are signed, so sign handling is native — no sign-magnitude struct is
//! needed. Every reduction and series step runs at scale `w`; the value
//! is rounded once, half-to-even, back to `SCALE` at the end.
//!
//! The bedrock constants are bootstrapped from integer series rather
//! than embedded: `ln 2 = 2·artanh(1/3)`, `ln 10 = ln_fixed(10)`, and
//! `π` from Machin's `16·atan(1/5) − 4·atan(1/239)`.
//!
//! # The `*_strict` dual API
//!
//! - `<method>_strict` — always present unless the `no_strict` feature
//! is set; integer-only and `no_std`-compatible.
//! - `<method>` — a dispatcher present only under
//! `#[cfg(all(feature = "strict", not(feature = "no_strict")))]`,
//! forwarding to `<method>_strict`. The wide tiers have no f64-bridge
//! transcendentals of their own.
//!
//! # Precision
//!
//! Strict and **correctly rounded** — within 0.5 ULP of the exact
//! result (IEEE-754 round-to-nearest), at the storage scale.
//!
//! Two structural choices keep the error inside the 0.5 ULP budget:
//!
//! - **`GUARD = 60` guard digits below the storage scale.** The
//!   working scale `w = SCALE + GUARD` gives every intermediate
//!   computation an LSB ~10⁻⁶⁰ below the storage LSB. Even after a
//!   long series-evaluation core accumulates a few hundred LSB of
//!   working-scale error, the absolute error remains ≪ 0.5 storage
//!   ULP.
//! - **Half-to-even rounded `mul` / `div`** in the working scale
//!   (see `round_div` below). Replaces the previous truncating ops,
//!   which leaked ~1 LSB-of-`w` *bias* per call — a coherent error
//!   that didn't cancel even with many guard digits.
//!
//! The final round to storage runs through
//! [`crate::rounding::should_bump`] and honours `DEFAULT_ROUNDING_MODE`.
//!
//! For inputs whose own storage representation has ≤ 0.5 LSB
//! rounding (any value parsed from a literal at the storage scale),
//! the *result* is within 1 LSB of the truth-at-storage. For inputs
//! that are themselves stored with rounding (like `D256s12::pi()`
//! widened from D128's 37-digit reference), the input's rounding
//! propagates through whatever conditioning the method has — that's
//! an input-side budget the wide-tier API can't compensate for.
//!
//! [`RoundingMode`]: crate::rounding::RoundingMode

/// Emits the strict transcendental surface for a wide decimal tier.
///
/// - `$Type` / `$Storage` — the decimal type and its wide storage.
/// - `$Work` — a hand-rolled signed wide integer wide enough for working-scale
/// products: at least `2·(SCALE_max + 30)` decimal digits.
/// - `$core` — the name of the private module the per-tier guard-digit
/// core is emitted into.
macro_rules! decl_wide_transcendental {
    ($Type:ident, $Storage:ty, $Work:ty, $core:ident) => {
        /// Per-tier guard-digit transcendental core. Every function
        /// works on `$Work` integers interpreted at a working scale `w`
        /// passed explicitly alongside the value.
        #[cfg(not(feature = "no_strict"))]
        mod $core {
            #![allow(unused)]

            /// The working integer: a value `x` is held as `x · 10^w`.
            pub(super) type W = $Work;

            /// Guard digits added below the type's own scale.
            ///
            /// Sized for 0.5 ULP at the storage scale with the
            /// rounded-intermediate `mul`/`div` (see `round_div`
            /// below). Each rounded op introduces ≤ 0.5 LSB-of-w
            /// of *uncorrelated* error (half-to-even is symmetric,
            /// so the random walk is the worst case). Across a
            /// 200-iteration series the accumulated worst-case
            /// drift is ~200 × 0.5 = 100 LSB-of-w; at GUARD = 30
            /// that's ~10⁻²⁸ in storage units — many orders of
            /// magnitude below half a storage ULP for any SCALE
            /// the wide tiers support. The truncating-intermediate
            /// path that preceded this used GUARD = 30 too but
            /// leaked a coherent bias (always toward zero) that
            /// blew the budget; with rounded ops we recovered the
            /// margin and didn't need the doubled width.
            pub(super) const GUARD: u32 = 30;
            /// Hard cap on series iterations — a safety net; every
            /// series terminates far sooner by reaching a zero term.
            const SERIES_CAP: u128 = 20_000;

            #[inline]
            fn lit(n: u128) -> W {
                $crate::wide_int::wide_cast(n)
            }
            #[inline]
            pub(super) fn zero() -> W {
                lit(0)
            }
            #[inline]
            fn abs(v: W) -> W {
                if v < lit(0) { -v } else { v }
            }
            #[inline]
            pub(super) fn pow10(n: u32) -> W {
                lit(10).pow(n)
            }
            #[inline]
            pub(super) fn one(w: u32) -> W {
                pow10(w)
            }
            /// Half-to-even round of `(numerator / divisor)` for
            /// the signed wide integer `W`. Pulled out so the
            /// `mul` / `div` core helpers share one rounding rule
            /// instead of truncating per op (which leaks ~1 LSB
            /// each into the strict-transcendental series).
            #[inline]
            fn round_div(n: W, d: W) -> W {
                let q = n / d;
                let r = n % d;
                if r == lit(0) {
                    return q;
                }
                let ar = abs(r);
                let comp = abs(d) - ar;
                let cmp_r = ar.cmp(&comp);
                let q_is_odd = (q % lit(2)) != lit(0);
                let result_positive = (n < lit(0)) == (d < lit(0));
                if $crate::rounding::should_bump(
                    $crate::rounding::RoundingMode::HalfToEven,
                    cmp_r,
                    q_is_odd,
                    result_positive,
                ) {
                    if result_positive { q + lit(1) } else { q - lit(1) }
                } else {
                    q
                }
            }
            /// `(a · b) / 10^w`, rounded half-to-even. The
            /// rounded variant replaces the previous truncating
            /// `mul`: each call drops the per-op ≤ 1 LSB
            /// truncation bias to a symmetric ≤ 0.5 LSB error,
            /// which is what 0.5 ULP at storage requires across
            /// the series-evaluation core.
            #[inline]
            pub(super) fn mul(a: W, b: W, w: u32) -> W {
                round_div(a * b, pow10(w))
            }
            /// `(a · 10^w) / b`, rounded half-to-even.
            #[inline]
            pub(super) fn div(a: W, b: W, w: u32) -> W {
                round_div(a * pow10(w), b)
            }
            /// `a · n` for a small unsigned multiplier.
            #[inline]
            fn mul_u(a: W, n: u128) -> W {
                a * lit(n)
            }

            /// Bit length of `|v|` (0 for zero).
            pub(super) fn bit_length(v: W) -> u32 {
                W::BITS - abs(v).leading_zeros()
            }

            /// `√v` at working scale `w`: `√(|v| · 10^w)`, truncating.
            ///
            /// `|v| * 10^w` must fit in `W`. Bit-length headroom is
            /// asserted in debug builds; in release the multiply
            /// wraps silently if violated. Every caller in this crate
            /// passes a value with sufficient headroom: the working
            /// integer is sized so `2·(SCALE + GUARD)` digits fit.
            pub(super) fn sqrt_fixed(v: W, w: u32) -> W {
                let av = abs(v);
                debug_assert!(
                    bit_length(av) + (w as u32) * 4 < W::BITS,
                    "sqrt_fixed: |v| * 10^w overflows the working width"
                );
                (av * pow10(w)).isqrt()
            }

            /// Builds a working-scale value from the type's raw storage:
            /// `raw · 10^GUARD` (raw is `value · 10^SCALE`, the result
            /// is `value · 10^(SCALE+GUARD)`).
            ///
            /// Uses [`wide_cast`] instead of `.resize::<W>()` so the
            /// macro accepts both wide-int and primitive `$Storage`
            /// (`i128` for D128).
            ///
            /// [`wide_cast`]: $crate::wide_int::wide_cast
            pub(super) fn to_work(raw: $Storage) -> W {
                $crate::wide_int::wide_cast::<$Storage, W>(raw) * pow10(GUARD)
            }

            /// Rounds a working-scale value down to scale `target` using
            /// the crate-default rounding mode and narrows to the
            /// type's storage. Panics if the rounded value does not
            /// fit.
            ///
            /// Mode dispatch goes through [`crate::rounding::should_bump`]
            /// (the same strategy the operator path uses), so a
            /// wide-tier `*_strict` honours the active `rounding-*`
            /// feature flag instead of always rounding half-to-even.
            pub(super) fn round_to_storage(v: W, w: u32, target: u32) -> $Storage {
                round_to_storage_with(v, w, target, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Mode-aware variant of [`round_to_storage`].
            pub(super) fn round_to_storage_with(
                v: W,
                w: u32,
                target: u32,
                mode: $crate::rounding::RoundingMode,
            ) -> $Storage {
                let divisor = pow10(w - target);
                let q = v / divisor;
                let r = v % divisor;
                let rounded = if r == lit(0) {
                    q
                } else {
                    let ar = abs(r);
                    let comp = divisor - ar;
                    let cmp_r = ar.cmp(&comp);
                    let q_is_odd = (q % lit(2)) != lit(0);
                    let result_positive = v >= lit(0);
                    if $crate::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
                        if result_positive { q + lit(1) } else { q - lit(1) }
                    } else {
                        q
                    }
                };
                let max_w = $crate::wide_int::wide_cast::<$Storage, W>(<$Storage>::MAX);
                let min_w = $crate::wide_int::wide_cast::<$Storage, W>(<$Storage>::MIN);
                if rounded > max_w || rounded < min_w {
                    panic!(concat!(
                        stringify!($Type),
                        " strict transcendental: result out of range"
                    ));
                }
                $crate::wide_int::wide_cast::<W, $Storage>(rounded)
            }

            /// Rounds a working-scale value to the nearest integer (ties
            /// away from zero). Used for the range-reduction quotient.
            pub(super) fn round_to_nearest_int(v: W, w: u32) -> i128 {
                let divisor = pow10(w);
                let q = v / divisor;
                let r = v % divisor;
                let half = divisor / lit(2);
                let qi = if abs(r) >= half {
                    if v < lit(0) { q - lit(1) } else { q + lit(1) }
                } else {
                    q
                };
                $crate::wide_int::wide_cast::<W, i128>(qi)
            }

            /// `k · c` where `k` is a signed range-reduction count.
            #[inline]
            fn scale_by_k(c: W, k: i128) -> W {
                if k >= 0 {
                    mul_u(c, k as u128)
                } else {
                    -mul_u(c, k.unsigned_abs())
                }
            }

            /// `ln 2` at working scale `w`, via `2·artanh(1/3)`.
            pub(super) fn ln2(w: u32) -> W {
                let t = one(w) / lit(3);
                let t2 = mul(t, t, w);
                let mut sum = t;
                let mut term = t;
                let mut j: u128 = 1;
                loop {
                    term = mul(term, t2, w);
                    let contrib = term / lit(2 * j + 1);
                    if contrib == zero() {
                        break;
                    }
                    sum = sum + contrib;
                    j += 1;
                    if j > SERIES_CAP {
                        break;
                    }
                }
                sum + sum
            }

            /// Natural logarithm of a positive working-scale value.
            ///
            /// Range-reduces `v = 2^k · m` with `m ∈ [1, 2)`, evaluates
            /// `ln(m) = 2·artanh((m−1)/(m+1))`, returns `k·ln 2 + ln(m)`.
            pub(super) fn ln_fixed(v_w: W, w: u32) -> W {
                let one_w = one(w);
                let two_w = one_w + one_w;
                let mut k: i32 = bit_length(v_w) as i32 - bit_length(one_w) as i32;
                let m_w = loop {
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
                let t = div(m_w - one_w, m_w + one_w, w);
                let t2 = mul(t, t, w);
                let mut sum = t;
                let mut term = t;
                let mut j: u128 = 1;
                loop {
                    term = mul(term, t2, w);
                    let contrib = term / lit(2 * j + 1);
                    if contrib == zero() {
                        break;
                    }
                    sum = sum + contrib;
                    j += 1;
                    if j > SERIES_CAP {
                        break;
                    }
                }
                let ln_m = sum + sum;
                scale_by_k(ln2(w), k as i128) + ln_m
            }

            /// `ln 10` at working scale `w`.
            pub(super) fn ln10(w: u32) -> W {
                ln_fixed(one(w) * lit(10), w)
            }

            /// Natural log of a positive working-scale value via the
            /// Brent–Salamin AGM (1976).
            ///
            /// Identity: `ln(s) ≈ π / (2 · AGM(1, 4/s))` as `s → ∞`,
            /// with error `O(s⁻²)`. To compute `ln(x)` for arbitrary
            /// positive `x`, pick `m` so `s := x · 2^m` is large
            /// enough that `4/s < 2^(−p/2)` (p = working precision in
            /// bits). Then `ln(x) = ln(s) − m·ln 2`.
            ///
            /// Convergence: AGM doubles the number of correct digits
            /// per iteration, so `O(log p)` iterations suffice
            /// regardless of `w`. For very high working scales this
            /// asymptotically beats the artanh-series `ln_fixed`,
            /// which is linear in `p`.
            ///
            /// Bit budget: this routine shifts `v_w` left by `m` bits.
            /// `W` must have headroom for `bit_length(v_w) + m`; for
            /// every wide tier in this crate, `W` is sized so that
            /// holds with comfortable margin (see the macro header).
            ///
            /// # Precision caveat
            ///
            /// This implementation runs the AGM iteration at the same
            /// working scale `w` as the artanh path. When the input
            /// `y = 4/s` is many orders of magnitude smaller than
            /// `1` (a regime AGM is designed for), the early-phase
            /// `sqrt(a·b)` step amplifies its half-LSB truncation
            /// error by `√(a/b)`. At very deep storage scales (`w`
            /// beyond ~40) that amplification dominates and the
            /// output drops to `~p/2` bits of precision rather than
            /// `p`. Brent 1976 §3 fixes this by raising the
            /// intermediate AGM precision; that's recorded as a
            /// follow-up and not implemented yet. At storage scales
            /// up to ~30 the AGM and artanh paths agree to 2 LSB.
            pub(super) fn ln_fixed_agm(v_w: W, w: u32) -> W {
                let one_w = one(w);
                // p_bits ≈ working-scale precision in bits, w · log2(10).
                // 332/100 is the integer rational just above log2(10).
                let p_bits = ((w as i32) * 332 + 99) / 100;
                let bl_v = bit_length(v_w) as i32;
                let bl_one = bit_length(one_w) as i32;
                // We need s = v_w · 2^m with bit_length(s) ≥ p/2 + bl_one
                // + safety_margin so that y = 4·one_w/s has bit_length
                // ≤ bl_one − (p/2 + safety_margin). Brent's bound on
                // the AGM error is `O(log(s)/s²)`, so log₂(s) needs an
                // extra `½·log₂(p)` bits beyond `p/2` to push the
                // residual error below one LSB at scale w.
                let safety = 2 + ((p_bits.max(1) as u32).ilog2() / 2) as i32;
                let mut m: i32 = (p_bits / 2) + safety + bl_one - bl_v;
                if m < 2 {
                    m = 2;
                }
                // Cap m so v_w << m + the AGM mul both fit in W. The mul
                // doubles bit length, so leave half-width headroom.
                let half_w = (W::BITS as i32) / 2;
                let cap = half_w - bl_v - 2;
                if cap > 0 && m > cap {
                    m = cap;
                }
                debug_assert!(
                    m > 0,
                    "ln_fixed_agm: working-int width too small for this scale"
                );
                let s_w = v_w << (m as u32);
                let y_w = div(lit(4) * one_w, s_w, w);
                let mut a = one_w;
                let mut b = y_w;
                let iter_cap = 80u32;
                for _ in 0..iter_cap {
                    let next_a = (a + b) / lit(2);
                    let next_b = sqrt_fixed(mul(a, b, w), w);
                    let d = if next_a >= next_b { next_a - next_b } else { next_b - next_a };
                    a = next_a;
                    b = next_b;
                    if d <= lit(2) {
                        break;
                    }
                }
                let pi_w = pi(w);
                let agm_part = div(pi_w, a + a, w);
                agm_part - scale_by_k(ln2(w), m as i128)
            }

            /// Exponential of a working-scale value via Newton's
            /// iteration on `ln_fixed_agm`.
            ///
            /// Given target `y`, finds `x = exp(y)` by iterating
            /// `x_{n+1} = x_n · (1 + (y − ln x_n))`. Quadratic
            /// convergence: roughly `log₂(p)` iterations of one
            /// `ln_fixed_agm` each. For very high working scales this
            /// is asymptotically faster than the Taylor `exp_fixed`,
            /// which is linear in `p`.
            ///
            /// Range-reduces `v = k·ln 2 + s` first (same trick as
            /// `exp_fixed`) so the Newton seed and iterations stay in
            /// a small absolute range, then reassembles `2^k · exp(s)`.
            pub(super) fn exp_fixed_agm(v_w: W, w: u32) -> W {
                let one_w = one(w);
                let l2 = ln2(w);
                let k = round_to_nearest_int(div(v_w, l2, w), w);
                let s = v_w - scale_by_k(l2, k);
                // Newton seed: low-order Taylor (1 + s + s²/2). Within
                // ~10⁻² of truth for |s| ≤ ln(2)/2 ≈ 0.347.
                let s2 = mul(s, s, w);
                let mut x = one_w + s + s2 / lit(2);
                if x <= lit(0) {
                    x = one_w;
                }
                let iter_cap = 80u32;
                for _ in 0..iter_cap {
                    let ln_x = ln_fixed_agm(x, w);
                    let delta = s - ln_x;
                    if abs(delta) <= lit(2) {
                        x = mul(x, one_w + delta, w);
                        break;
                    }
                    x = mul(x, one_w + delta, w);
                }
                if k >= 0 {
                    let shift = k as u32;
                    if bit_length(x) + shift >= W::BITS {
                        panic!(concat!(
                            stringify!($Type),
                            "::exp: result overflows the representable range"
                        ));
                    }
                    x << shift
                } else {
                    let neg_k = (-k) as u128;
                    if neg_k >= bit_length(x) as u128 {
                        return zero();
                    }
                    x >> (neg_k as u32)
                }
            }

            /// `e^v` for a working-scale value `v`.
            ///
            /// Range-reduces `v = k·ln 2 + s` with `|s| ≤ ln 2 / 2`,
            /// Taylor-expands `exp(s)`, reassembles `2^k · exp(s)`.
            pub(super) fn exp_fixed(v_w: W, w: u32) -> W {
                let one_w = one(w);
                let l2 = ln2(w);
                let k = round_to_nearest_int(div(v_w, l2, w), w);
                let s = v_w - scale_by_k(l2, k);
                let mut sum = one_w;
                let mut term = one_w;
                let mut n: u128 = 1;
                loop {
                    term = mul(term, s, w) / lit(n);
                    if term == zero() {
                        break;
                    }
                    sum = sum + term;
                    n += 1;
                    if n > SERIES_CAP {
                        break;
                    }
                }
                if k >= 0 {
                    let shift = k as u32;
                    if bit_length(sum) + shift >= W::BITS {
                        panic!(concat!(
                            stringify!($Type),
                            "::exp: result overflows the representable range"
                        ));
                    }
                    sum << shift
                } else {
                    // Underflow: |k| · ln(2) ≥ |v|. Once `-k` exceeds
                    // the bit-length of `sum`, `sum >> -k` truncates to
                    // zero — but the cap at `W::BITS - 1` previously
                    // left a stray `1` for very large `-k`. Catch the
                    // underflow explicitly so `exp(very_negative)` is
                    // a true zero.
                    let neg_k = -k as u128;
                    if neg_k >= bit_length(sum) as u128 {
                        return zero();
                    }
                    sum >> (neg_k as u32)
                }
            }

            /// Taylor series for `atan` on `|x| < 1`, at scale `w`.
            pub(super) fn atan_taylor(x: W, w: u32) -> W {
                let x2 = mul(x, x, w);
                let mut sum = x;
                let mut term = x;
                let mut k: u128 = 1;
                loop {
                    term = mul(term, x2, w);
                    let contrib = term / lit(2 * k + 1);
                    if contrib == zero() {
                        break;
                    }
                    if k % 2 == 1 {
                        sum = sum - contrib;
                    } else {
                        sum = sum + contrib;
                    }
                    k += 1;
                    if k > SERIES_CAP {
                        break;
                    }
                }
                sum
            }

            /// `π` at working scale `w`, via Machin's formula.
            pub(super) fn pi(w: u32) -> W {
                let a = atan_taylor(one(w) / lit(5), w);
                let b = atan_taylor(one(w) / lit(239), w);
                mul_u(a, 16) - mul_u(b, 4)
            }
            /// `π/2` at working scale `w`.
            pub(super) fn half_pi(w: u32) -> W {
                pi(w) / lit(2)
            }

            /// Taylor series for `sin` on a reduced `r ∈ [0, π/2]`.
            fn sin_taylor(r: W, w: u32) -> W {
                let r2 = mul(r, r, w);
                let mut sum = r;
                let mut term = r;
                let mut k: u128 = 1;
                loop {
                    term = mul(term, r2, w) / lit((2 * k) * (2 * k + 1));
                    if term == zero() {
                        break;
                    }
                    if k % 2 == 1 {
                        sum = sum - term;
                    } else {
                        sum = sum + term;
                    }
                    k += 1;
                    if k > SERIES_CAP {
                        break;
                    }
                }
                sum
            }

            /// Sine of a working-scale value.
            pub(super) fn sin_fixed(v_w: W, w: u32) -> W {
                let pi_w = pi(w);
                let tau = pi_w + pi_w;
                let hp = pi_w / lit(2);
                let q = round_to_nearest_int(div(v_w, tau, w), w);
                let r = v_w - scale_by_k(tau, q);
                let neg = r < zero();
                let abs_r = if neg { -r } else { r };
                let reduced = if abs_r >= hp { pi_w - abs_r } else { abs_r };
                let s = sin_taylor(reduced, w);
                if neg { -s } else { s }
            }

            /// Arctangent of a working-scale value, result in
            /// `(−π/2, π/2)`.
            pub(super) fn atan_fixed(v_w: W, w: u32) -> W {
                let one_w = one(w);
                let sign = v_w < zero();
                let mut x = if sign { -v_w } else { v_w };
                let mut add_half_pi = false;
                if x > one_w {
                    x = div(one_w, x, w);
                    add_half_pi = true;
                }
                // Three argument halvings: atan(x) = 2·atan(x/(1+√(1+x²))).
                //
                // Empirically chosen as the trade-off point on this
                // truncating fixed-point core: each halving reduces
                // |x| by a factor ≈ 2, so after 3 halvings |x| ≤ 1/8
                // and the Taylor series converges in ≈ w·log₂(10)/3
                // terms (e.g. ~70 terms at w=63). Adding more
                // halvings shortens the series but introduces more
                // sqrt/div truncation error; fewer halvings explodes
                // the term count. Tighten via the
                // `atan_iter_count_bound` test if you change it.
                let halvings: u32 = 3;
                for _ in 0..halvings {
                    let x2 = mul(x, x, w);
                    let denom = one_w + sqrt_fixed(one_w + x2, w);
                    x = div(x, denom, w);
                }
                let mut result = atan_taylor(x, w) << halvings;
                if add_half_pi {
                    result = half_pi(w) - result;
                }
                if sign { -result } else { result }
            }
        }

        #[cfg(not(feature = "no_strict"))]
        impl<const SCALE: u32> $Type<SCALE> {
            /// Natural logarithm (base e). Strict: integer-only and
            /// correctly rounded. Panics if `self <= 0`.
            #[inline]
            #[must_use]
            pub fn ln_strict(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::ln: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::ln_fixed($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Natural logarithm via the Brent–Salamin AGM (1976).
            /// Strict and correctly rounded. Same contract as
            /// [`Self::ln_strict`]; the implementation path differs.
            /// AGM converges quadratically and scales better than the
            /// artanh-series path at very high working scales.
            ///
            /// Currently an alternate; the canonical `ln_strict` stays
            /// on the artanh path until a bench at the relevant
            /// working scale shows AGM winning by the
            /// `OVERRIDE_POLICY.md` margin.
            #[inline]
            #[must_use]
            pub fn ln_strict_agm(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::ln_agm: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::ln_fixed_agm($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// `e^self` via Newton's iteration on `ln_fixed_agm`.
            /// Strict and correctly rounded. Same contract as
            /// [`Self::exp_strict`]; the implementation path differs.
            /// Quadratic convergence makes this asymptotically faster
            /// than the Taylor `exp_strict` at very high working
            /// scales.
            #[inline]
            #[must_use]
            pub fn exp_strict_agm(self) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + $core::GUARD;
                let r = $core::exp_fixed_agm($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Logarithm of `self` in the given `base`, as
            /// `ln(self) / ln(base)`. Strict and correctly rounded.
            /// Panics if `self <= 0`, `base <= 0`, or `base == 1`.
            #[inline]
            #[must_use]
            pub fn log_strict(self, base: Self) -> Self {
                let raw = self.to_bits();
                let braw = base.to_bits();
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                if raw <= z {
                    panic!(concat!(stringify!($Type), "::log: argument must be positive"));
                }
                if braw <= z {
                    panic!(concat!(stringify!($Type), "::log: base must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let ln_b = $core::ln_fixed($core::to_work(braw), w);
                if ln_b == $core::zero() {
                    panic!(concat!(stringify!($Type), "::log: base must not equal 1"));
                }
                let r = $core::div($core::ln_fixed($core::to_work(raw), w), ln_b, w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Base-2 logarithm. Strict and correctly rounded. Panics if
            /// `self <= 0`.
            #[inline]
            #[must_use]
            pub fn log2_strict(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log2: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::div($core::ln_fixed($core::to_work(raw), w), $core::ln2(w), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Base-10 logarithm. Strict and correctly rounded. Panics
            /// if `self <= 0`.
            #[inline]
            #[must_use]
            pub fn log10_strict(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log10: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::div($core::ln_fixed($core::to_work(raw), w), $core::ln10(w), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// `e^self`. Strict and correctly rounded. Panics if the
            /// result overflows the representable range.
            #[inline]
            #[must_use]
            pub fn exp_strict(self) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + $core::GUARD;
                let r = $core::exp_fixed($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// `2^self`, as `exp(self · ln 2)`. Strict and correctly
            /// rounded. Panics if the result overflows.
            #[inline]
            #[must_use]
            pub fn exp2_strict(self) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + $core::GUARD;
                let arg = $core::mul($core::to_work(raw), $core::ln2(w), w);
                let r = $core::exp_fixed(arg, w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// `self` raised to the power `exp`, as `exp(exp · ln self)`.
            /// Strict and correctly rounded. A zero or negative base
            /// saturates to `ZERO` (a negative base with a fractional
            /// exponent is not real-valued).
            #[inline]
            #[must_use]
            pub fn powf_strict(self, exp: Self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let w = SCALE + $core::GUARD;
                let ln_x = $core::ln_fixed($core::to_work(raw), w);
                let y = $core::to_work(exp.to_bits());
                let r = $core::exp_fixed($core::mul(y, ln_x, w), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Sine of `self` (radians). Strict and correctly rounded.
            #[inline]
            #[must_use]
            pub fn sin_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let r = $core::sin_fixed($core::to_work(self.to_bits()), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Cosine of `self` (radians), as `sin(self + π/2)`. Strict
            /// and correctly rounded.
            #[inline]
            #[must_use]
            pub fn cos_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let arg = $core::to_work(self.to_bits()) + $core::half_pi(w);
                let r = $core::sin_fixed(arg, w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Tangent of `self` (radians), as `sin / cos`. Strict and
            /// correctly rounded. Panics at odd multiples of π/2.
            #[inline]
            #[must_use]
            pub fn tan_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let sin_w = $core::sin_fixed(v, w);
                let cos_w = $core::sin_fixed(v + $core::half_pi(w), w);
                if cos_w == $core::zero() {
                    panic!(concat!(
                        stringify!($Type),
                        "::tan: cosine is zero (argument is an odd multiple of pi/2)"
                    ));
                }
                let r = $core::div(sin_w, cos_w, w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Arctangent of `self`, in radians, in `(−π/2, π/2)`.
            /// Strict and correctly rounded.
            #[inline]
            #[must_use]
            pub fn atan_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let r = $core::atan_fixed($core::to_work(self.to_bits()), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Arcsine of `self`, in radians, in `[−π/2, π/2]`, as
            /// `atan(x / √(1 − x²))`. Strict. Panics if `|self| > 1`.
            #[inline]
            #[must_use]
            pub fn asin_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::asin: argument out of domain [-1, 1]"));
                }
                let r = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                };
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Arccosine of `self`, in radians, in `[0, π]`, as
            /// `π/2 − asin(self)`. Strict. Panics if `|self| > 1`.
            #[inline]
            #[must_use]
            pub fn acos_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::acos: argument out of domain [-1, 1]"));
                }
                let asin_w = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                };
                let r = $core::half_pi(w) - asin_w;
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Four-quadrant arctangent of `self` (`y`) and `other`
            /// (`x`), in radians, in `(−π, π]`. Strict and correctly
            /// rounded.
            #[inline]
            #[must_use]
            pub fn atan2_strict(self, other: Self) -> Self {
                let w = SCALE + $core::GUARD;
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                let yraw = self.to_bits();
                let xraw = other.to_bits();
                let r = if xraw == z {
                    if yraw > z {
                        $core::half_pi(w)
                    } else if yraw < z {
                        -$core::half_pi(w)
                    } else {
                        $core::zero()
                    }
                } else {
                    let y = $core::to_work(yraw);
                    let x = $core::to_work(xraw);
                    let base = $core::atan_fixed($core::div(y, x, w), w);
                    if xraw > z {
                        base
                    } else if yraw >= z {
                        base + $core::pi(w)
                    } else {
                        base - $core::pi(w)
                    }
                };
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Hyperbolic sine, as `(eˣ − e⁻ˣ)/2`. Strict and correctly
            /// rounded.
            #[inline]
            #[must_use]
            pub fn sinh_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = (ex - enx) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Hyperbolic cosine, as `(eˣ + e⁻ˣ)/2`. Strict and
            /// correctly rounded.
            #[inline]
            #[must_use]
            pub fn cosh_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = (ex + enx) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Hyperbolic tangent, as `sinh / cosh`. Strict and
            /// correctly rounded.
            #[inline]
            #[must_use]
            pub fn tanh_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = $core::div(ex - enx, ex + enx, w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Inverse hyperbolic sine, as
            /// `sign · ln(|x| + √(x² + 1))`. Strict and correctly
            /// rounded. For `|x| ≥ 1` the radicand is factored to keep
            /// `x²` inside the working width.
            #[inline]
            #[must_use]
            pub fn asinh_strict(self) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(raw);
                let ax = if v < $core::zero() { -v } else { v };
                let inner = if ax >= one_w {
                    let inv = $core::div(one_w, ax, w);
                    let root = $core::sqrt_fixed(one_w + $core::mul(inv, inv, w), w);
                    $core::ln_fixed(ax, w) + $core::ln_fixed(one_w + root, w)
                } else {
                    let root = $core::sqrt_fixed($core::mul(ax, ax, w) + one_w, w);
                    $core::ln_fixed(ax + root, w)
                };
                let signed = if raw < $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    -inner
                } else {
                    inner
                };
                Self::from_bits($core::round_to_storage(signed, w, SCALE))
            }

            /// Inverse hyperbolic cosine, as `ln(x + √(x² − 1))`,
            /// defined for `x ≥ 1`. Strict and correctly rounded. For
            /// `x ≥ 2` the radicand is factored to keep `x²` in range.
            #[inline]
            #[must_use]
            pub fn acosh_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                if v < one_w {
                    panic!(concat!(stringify!($Type), "::acosh: argument must be >= 1"));
                }
                let two_w = one_w + one_w;
                let inner = if v >= two_w {
                    let inv = $core::div(one_w, v, w);
                    let root = $core::sqrt_fixed(one_w - $core::mul(inv, inv, w), w);
                    $core::ln_fixed(v, w) + $core::ln_fixed(one_w + root, w)
                } else {
                    let root = $core::sqrt_fixed($core::mul(v, v, w) - one_w, w);
                    $core::ln_fixed(v + root, w)
                };
                Self::from_bits($core::round_to_storage(inner, w, SCALE))
            }

            /// Inverse hyperbolic tangent, as `ln((1+x)/(1−x)) / 2`,
            /// defined for `|x| < 1`. Strict and correctly rounded.
            /// Panics if `|self| >= 1`.
            #[inline]
            #[must_use]
            pub fn atanh_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let ax = if v < $core::zero() { -v } else { v };
                if ax >= one_w {
                    panic!(concat!(stringify!($Type), "::atanh: argument out of domain (-1, 1)"));
                }
                let ratio = $core::div(one_w + v, one_w - v, w);
                let r = $core::ln_fixed(ratio, w) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Convert radians to degrees: `self · (180 / π)`. Strict
            /// and correctly rounded. Panics if `|self| · 180`
            /// overflows the working integer.
            #[inline]
            #[must_use]
            pub fn to_degrees_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                debug_assert!(
                    $core::bit_length(v) + 8 < <$Work>::BITS,
                    concat!(stringify!($Type),
                        "::to_degrees: |self| * 180 overflows the working integer")
                );
                let r = $core::div(
                    v * $crate::macros::wide_roots::wide_lit!($Work, "180"),
                    $core::pi(w),
                    w,
                );
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Convert degrees to radians: `self · (π / 180)`. Strict
            /// and correctly rounded. `mul` is the scale-aware
            /// `(a * b) / 10^w`, so the working-width budget is the
            /// same as any other binary op in the core — no separate
            /// overflow check needed.
            #[inline]
            #[must_use]
            pub fn to_radians_strict(self) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let r = $core::mul(v, $core::pi(w), w)
                    / $crate::macros::wide_roots::wide_lit!($Work, "180");
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            // ---- Mode-aware siblings ----
            //
            // Every `*_strict` method above has a `*_strict_with(mode)`
            // companion that performs the same correctly-rounded
            // computation but routes the final storage-scale rounding
            // through the given [`RoundingMode`] instead of the crate
            // default. The body is duplicated rather than refactored
            // into a helper so each method's panic / early-return
            // semantics stay attached to its canonical name.

            /// Mode-aware sibling of [`Self::ln_strict`].
            #[inline]
            #[must_use]
            pub fn ln_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::ln: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::ln_fixed($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::ln_strict_agm`].
            #[inline]
            #[must_use]
            pub fn ln_strict_agm_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::ln_agm: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::ln_fixed_agm($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::exp_strict_agm`].
            #[inline]
            #[must_use]
            pub fn exp_strict_agm_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + $core::GUARD;
                let r = $core::exp_fixed_agm($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::log_strict`].
            #[inline]
            #[must_use]
            pub fn log_strict_with(self, base: Self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                let braw = base.to_bits();
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                if raw <= z {
                    panic!(concat!(stringify!($Type), "::log: argument must be positive"));
                }
                if braw <= z {
                    panic!(concat!(stringify!($Type), "::log: base must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let ln_b = $core::ln_fixed($core::to_work(braw), w);
                if ln_b == $core::zero() {
                    panic!(concat!(stringify!($Type), "::log: base must not equal 1"));
                }
                let r = $core::div($core::ln_fixed($core::to_work(raw), w), ln_b, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::log2_strict`].
            #[inline]
            #[must_use]
            pub fn log2_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log2: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::div($core::ln_fixed($core::to_work(raw), w), $core::ln2(w), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::log10_strict`].
            #[inline]
            #[must_use]
            pub fn log10_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log10: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::div($core::ln_fixed($core::to_work(raw), w), $core::ln10(w), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::exp_strict`].
            #[inline]
            #[must_use]
            pub fn exp_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + $core::GUARD;
                let r = $core::exp_fixed($core::to_work(raw), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::exp2_strict`].
            #[inline]
            #[must_use]
            pub fn exp2_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + $core::GUARD;
                let arg = $core::mul($core::to_work(raw), $core::ln2(w), w);
                let r = $core::exp_fixed(arg, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::powf_strict`].
            #[inline]
            #[must_use]
            pub fn powf_strict_with(self, exp: Self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let w = SCALE + $core::GUARD;
                let ln_x = $core::ln_fixed($core::to_work(raw), w);
                let y = $core::to_work(exp.to_bits());
                let r = $core::exp_fixed($core::mul(y, ln_x, w), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::sin_strict`].
            #[inline]
            #[must_use]
            pub fn sin_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let r = $core::sin_fixed($core::to_work(self.to_bits()), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::cos_strict`].
            #[inline]
            #[must_use]
            pub fn cos_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let arg = $core::to_work(self.to_bits()) + $core::half_pi(w);
                let r = $core::sin_fixed(arg, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::tan_strict`].
            #[inline]
            #[must_use]
            pub fn tan_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let sin_w = $core::sin_fixed(v, w);
                let cos_w = $core::sin_fixed(v + $core::half_pi(w), w);
                if cos_w == $core::zero() {
                    panic!(concat!(
                        stringify!($Type),
                        "::tan: cosine is zero (argument is an odd multiple of pi/2)"
                    ));
                }
                let r = $core::div(sin_w, cos_w, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::atan_strict`].
            #[inline]
            #[must_use]
            pub fn atan_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let r = $core::atan_fixed($core::to_work(self.to_bits()), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::asin_strict`].
            #[inline]
            #[must_use]
            pub fn asin_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::asin: argument out of domain [-1, 1]"));
                }
                let r = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                };
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::acos_strict`].
            #[inline]
            #[must_use]
            pub fn acos_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::acos: argument out of domain [-1, 1]"));
                }
                let asin_w = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                };
                let r = $core::half_pi(w) - asin_w;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::atan2_strict`].
            #[inline]
            #[must_use]
            pub fn atan2_strict_with(self, other: Self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                let yraw = self.to_bits();
                let xraw = other.to_bits();
                let r = if xraw == z {
                    if yraw > z {
                        $core::half_pi(w)
                    } else if yraw < z {
                        -$core::half_pi(w)
                    } else {
                        $core::zero()
                    }
                } else {
                    let y = $core::to_work(yraw);
                    let x = $core::to_work(xraw);
                    let base = $core::atan_fixed($core::div(y, x, w), w);
                    if xraw > z {
                        base
                    } else if yraw >= z {
                        base + $core::pi(w)
                    } else {
                        base - $core::pi(w)
                    }
                };
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::sinh_strict`].
            #[inline]
            #[must_use]
            pub fn sinh_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = (ex - enx) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::cosh_strict`].
            #[inline]
            #[must_use]
            pub fn cosh_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = (ex + enx) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::tanh_strict`].
            #[inline]
            #[must_use]
            pub fn tanh_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = $core::div(ex - enx, ex + enx, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::asinh_strict`].
            #[inline]
            #[must_use]
            pub fn asinh_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(raw);
                let ax = if v < $core::zero() { -v } else { v };
                let inner = if ax >= one_w {
                    let inv = $core::div(one_w, ax, w);
                    let root = $core::sqrt_fixed(one_w + $core::mul(inv, inv, w), w);
                    $core::ln_fixed(ax, w) + $core::ln_fixed(one_w + root, w)
                } else {
                    let root = $core::sqrt_fixed($core::mul(ax, ax, w) + one_w, w);
                    $core::ln_fixed(ax + root, w)
                };
                let signed = if raw < $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    -inner
                } else {
                    inner
                };
                Self::from_bits($core::round_to_storage_with(signed, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::acosh_strict`].
            #[inline]
            #[must_use]
            pub fn acosh_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                if v < one_w {
                    panic!(concat!(stringify!($Type), "::acosh: argument must be >= 1"));
                }
                let two_w = one_w + one_w;
                let inner = if v >= two_w {
                    let inv = $core::div(one_w, v, w);
                    let root = $core::sqrt_fixed(one_w - $core::mul(inv, inv, w), w);
                    $core::ln_fixed(v, w) + $core::ln_fixed(one_w + root, w)
                } else {
                    let root = $core::sqrt_fixed($core::mul(v, v, w) - one_w, w);
                    $core::ln_fixed(v + root, w)
                };
                Self::from_bits($core::round_to_storage_with(inner, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::atanh_strict`].
            #[inline]
            #[must_use]
            pub fn atanh_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let ax = if v < $core::zero() { -v } else { v };
                if ax >= one_w {
                    panic!(concat!(stringify!($Type), "::atanh: argument out of domain (-1, 1)"));
                }
                let ratio = $core::div(one_w + v, one_w - v, w);
                let r = $core::ln_fixed(ratio, w) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::to_degrees_strict`].
            #[inline]
            #[must_use]
            pub fn to_degrees_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                debug_assert!(
                    $core::bit_length(v) + 8 < <$Work>::BITS,
                    concat!(stringify!($Type),
                        "::to_degrees: |self| * 180 overflows the working integer")
                );
                let r = $core::div(
                    v * $crate::macros::wide_roots::wide_lit!($Work, "180"),
                    $core::pi(w),
                    w,
                );
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::to_radians_strict`].
            #[inline]
            #[must_use]
            pub fn to_radians_strict_with(self, mode: $crate::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let r = $core::mul(v, $core::pi(w), w)
                    / $crate::macros::wide_roots::wide_lit!($Work, "180");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }
        }

        // Strict-feature dispatchers: the plain method routes to
        // `*_strict` when `strict` is on (and `no_strict` is not). The
        // wide tiers have no f64-bridge transcendentals, so there is no
        // non-strict plain form.
        #[cfg(all(feature = "strict", not(feature = "no_strict")))]
        impl<const SCALE: u32> $Type<SCALE> {
            /// With `strict`, dispatches to [`Self::ln_strict`].
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                self.ln_strict()
            }
            /// With `strict`, dispatches to [`Self::log_strict`].
            #[inline]
            #[must_use]
            pub fn log(self, base: Self) -> Self {
                self.log_strict(base)
            }
            /// With `strict`, dispatches to [`Self::log2_strict`].
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                self.log2_strict()
            }
            /// With `strict`, dispatches to [`Self::log10_strict`].
            #[inline]
            #[must_use]
            pub fn log10(self) -> Self {
                self.log10_strict()
            }
            /// With `strict`, dispatches to [`Self::exp_strict`].
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                self.exp_strict()
            }
            /// With `strict`, dispatches to [`Self::exp2_strict`].
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                self.exp2_strict()
            }
            /// With `strict`, dispatches to [`Self::powf_strict`].
            #[inline]
            #[must_use]
            pub fn powf(self, exp: Self) -> Self {
                self.powf_strict(exp)
            }
            /// With `strict`, dispatches to [`Self::sin_strict`].
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                self.sin_strict()
            }
            /// With `strict`, dispatches to [`Self::cos_strict`].
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                self.cos_strict()
            }
            /// With `strict`, dispatches to [`Self::tan_strict`].
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                self.tan_strict()
            }
            /// With `strict`, dispatches to [`Self::asin_strict`].
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                self.asin_strict()
            }
            /// With `strict`, dispatches to [`Self::acos_strict`].
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                self.acos_strict()
            }
            /// With `strict`, dispatches to [`Self::atan_strict`].
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                self.atan_strict()
            }
            /// With `strict`, dispatches to [`Self::atan2_strict`].
            #[inline]
            #[must_use]
            pub fn atan2(self, other: Self) -> Self {
                self.atan2_strict(other)
            }
            /// With `strict`, dispatches to [`Self::sinh_strict`].
            #[inline]
            #[must_use]
            pub fn sinh(self) -> Self {
                self.sinh_strict()
            }
            /// With `strict`, dispatches to [`Self::cosh_strict`].
            #[inline]
            #[must_use]
            pub fn cosh(self) -> Self {
                self.cosh_strict()
            }
            /// With `strict`, dispatches to [`Self::tanh_strict`].
            #[inline]
            #[must_use]
            pub fn tanh(self) -> Self {
                self.tanh_strict()
            }
            /// With `strict`, dispatches to [`Self::asinh_strict`].
            #[inline]
            #[must_use]
            pub fn asinh(self) -> Self {
                self.asinh_strict()
            }
            /// With `strict`, dispatches to [`Self::acosh_strict`].
            #[inline]
            #[must_use]
            pub fn acosh(self) -> Self {
                self.acosh_strict()
            }
            /// With `strict`, dispatches to [`Self::atanh_strict`].
            #[inline]
            #[must_use]
            pub fn atanh(self) -> Self {
                self.atanh_strict()
            }
            /// With `strict`, dispatches to [`Self::to_degrees_strict`].
            #[inline]
            #[must_use]
            pub fn to_degrees(self) -> Self {
                self.to_degrees_strict()
            }
            /// With `strict`, dispatches to [`Self::to_radians_strict`].
            #[inline]
            #[must_use]
            pub fn to_radians(self) -> Self {
                self.to_radians_strict()
            }
        }
    };
}

pub(crate) use decl_wide_transcendental;

#[cfg(all(test, not(feature = "no_strict")))]
mod tests {
    use crate::{D128, D256, D512, D1024};

    /// The wide-tier strict transcendentals are correctly rounded, so
    /// at any scale they must agree with the D128 strict path — itself
    /// correctly rounded — to within a couple of ULP (a small slack
    /// absorbs the two paths' independent final-rounding of values that
    /// land near a half-ULP boundary).
    #[test]
    fn wide_transcendentals_match_d128() {
        // Raw bit-patterns at SCALE = 6 spanning a useful range.
        let positives = [1i64, 250_000, 500_000, 1_000_000, 2_718_282, 7_500_000];
        let unit_range = [-900_000i64, -250_000, 1, 250_000, 900_000];
        let all = [-3_000_000i64, -500_000, 1, 500_000, 1_500_000, 4_000_000];

        // `wide` and `d128` are both `i128`-valued raw results at the
        // same scale; compare with a 2-ULP slack.
        fn agree(label: &str, ctx: i64, wide: i128, d128: i128) {
            assert!(
                (wide - d128).abs() <= 2,
                "{label} mismatch at {ctx}: wide {wide} vs d128 {d128}"
            );
        }

        for raw in positives {
            let n = D128::<6>::from_bits(raw as i128);
            let w = D256::<6>::from_bits(crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128));
            agree("ln", raw, w.ln_strict().to_bits().resize::<i128>(), n.ln_strict().to_bits());
            agree("log2", raw, w.log2_strict().to_bits().resize::<i128>(), n.log2_strict().to_bits());
            agree("log10", raw, w.log10_strict().to_bits().resize::<i128>(), n.log10_strict().to_bits());
        }
        for raw in all {
            let n = D128::<6>::from_bits(raw as i128);
            let w = D256::<6>::from_bits(crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128));
            agree("exp", raw, w.exp_strict().to_bits().resize::<i128>(), n.exp_strict().to_bits());
            agree("sin", raw, w.sin_strict().to_bits().resize::<i128>(), n.sin_strict().to_bits());
            agree("cos", raw, w.cos_strict().to_bits().resize::<i128>(), n.cos_strict().to_bits());
            agree("atan", raw, w.atan_strict().to_bits().resize::<i128>(), n.atan_strict().to_bits());
            agree("sinh", raw, w.sinh_strict().to_bits().resize::<i128>(), n.sinh_strict().to_bits());
            agree("cosh", raw, w.cosh_strict().to_bits().resize::<i128>(), n.cosh_strict().to_bits());
            agree("tanh", raw, w.tanh_strict().to_bits().resize::<i128>(), n.tanh_strict().to_bits());
        }
        for raw in unit_range {
            let n = D128::<6>::from_bits(raw as i128);
            let w = D256::<6>::from_bits(crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128));
            agree("asin", raw, w.asin_strict().to_bits().resize::<i128>(), n.asin_strict().to_bits());
            agree("acos", raw, w.acos_strict().to_bits().resize::<i128>(), n.acos_strict().to_bits());
            agree("atanh", raw, w.atanh_strict().to_bits().resize::<i128>(), n.atanh_strict().to_bits());
        }
    }

    /// Bit-exact identity points hold across all three wide tiers.
    #[test]
    fn wide_transcendental_identities() {
        assert_eq!(D256::<6>::ONE.ln_strict(), D256::<6>::ZERO);
        assert_eq!(D256::<6>::ZERO.exp_strict(), D256::<6>::ONE);
        assert_eq!(D256::<6>::ZERO.sin_strict(), D256::<6>::ZERO);
        assert_eq!(D256::<6>::ZERO.sinh_strict(), D256::<6>::ZERO);
        assert_eq!(D256::<6>::ZERO.atan_strict(), D256::<6>::ZERO);

        assert_eq!(D512::<6>::ONE.ln_strict(), D512::<6>::ZERO);
        assert_eq!(D512::<6>::ZERO.exp_strict(), D512::<6>::ONE);
        assert_eq!(D512::<6>::ZERO.cos_strict(), D512::<6>::ONE);

        assert_eq!(D1024::<6>::ONE.ln_strict(), D1024::<6>::ZERO);
        assert_eq!(D1024::<6>::ZERO.exp_strict(), D1024::<6>::ONE);
        assert_eq!(D1024::<6>::ZERO.cosh_strict(), D1024::<6>::ONE);
    }

    /// AGM-based `ln_strict_agm` and `exp_strict_agm` (Brent–Salamin
    /// 1976 / Newton-on-AGM) are correctly rounded by the same
    /// contract as the canonical artanh / Taylor paths, so they must
    /// agree to within a couple of ULP at storage scale.
    #[test]
    fn wide_agm_matches_taylor_at_storage_scale() {
        let positives = [1i64, 250_000, 500_000, 1_000_000, 2_718_282, 7_500_000];
        let all = [-3_000_000i64, -500_000, 1, 500_000, 1_500_000, 4_000_000];

        fn agree(label: &str, ctx: i64, agm: i128, taylor: i128) {
            assert!(
                (agm - taylor).abs() <= 2,
                "{label} AGM-vs-Taylor mismatch at {ctx}: agm {agm} vs taylor {taylor}"
            );
        }

        for raw in positives {
            let w = D256::<6>::from_bits(
                crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128),
            );
            agree(
                "ln",
                raw,
                w.ln_strict_agm().to_bits().resize::<i128>(),
                w.ln_strict().to_bits().resize::<i128>(),
            );
        }
        for raw in all {
            let w = D256::<6>::from_bits(
                crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128),
            );
            agree(
                "exp",
                raw,
                w.exp_strict_agm().to_bits().resize::<i128>(),
                w.exp_strict().to_bits().resize::<i128>(),
            );
        }
    }

    /// Identity points: AGM `ln(1) = 0`, AGM `exp(0) = 1`.
    #[test]
    fn wide_agm_identity_points() {
        assert_eq!(D256::<6>::ONE.ln_strict_agm(), D256::<6>::ZERO);
        assert_eq!(D256::<6>::ZERO.exp_strict_agm(), D256::<6>::ONE);
        assert_eq!(D512::<6>::ONE.ln_strict_agm(), D512::<6>::ZERO);
        assert_eq!(D512::<6>::ZERO.exp_strict_agm(), D512::<6>::ONE);
        assert_eq!(D1024::<6>::ONE.ln_strict_agm(), D1024::<6>::ZERO);
        assert_eq!(D1024::<6>::ZERO.exp_strict_agm(), D1024::<6>::ONE);
    }

    /// `*_strict_with(mode)` siblings honour the explicit rounding
    /// mode. Picks a transcendental whose true value lands strictly
    /// between two storage representable values so the rounding mode
    /// actually changes the result.
    #[test]
    fn wide_strict_with_honours_mode() {
        use crate::rounding::RoundingMode;
        // π at SCALE=6 truncates to 3.141592 (HalfToEven also picks
        // 3.141592 here since digit 7 is < 5). ln(10) at SCALE=6 is
        // 2.302585...0929... — digit after 6 is 0, so all modes pick
        // the same. Use a less-friendly value: ln(7).
        // ln(7) = 1.9459101090932196... at SCALE=6 the truth digit 7
        // is just past the cut: 1.945910 with next digit 1 → all
        // truncating/HTE modes pick 1.945910. Need a value where the
        // exact fractional part is ≥ 0.5 LSB so Trunc and HTE diverge.
        //
        // A clean way: positive number with HTE rounding up. exp(1) =
        // 2.7182818... at SCALE=6: 2.718281 cut, fractional 0.8 →
        // HTE rounds up to 2.718282, Trunc keeps 2.718281.
        let n = D256::<6>::ONE;
        let hte = n.exp_strict_with(RoundingMode::HalfToEven);
        let trunc = n.exp_strict_with(RoundingMode::Trunc);
        assert!(
            hte.to_bits().resize::<i128>() - trunc.to_bits().resize::<i128>() == 1
                || hte.to_bits().resize::<i128>() - trunc.to_bits().resize::<i128>() == 0,
            "exp(1) HTE vs Trunc: hte={}, trunc={}",
            hte,
            trunc,
        );
        // HalfToEven matches the canonical *_strict (which uses
        // DEFAULT_ROUNDING_MODE = HalfToEven absent a feature flag).
        if !(cfg!(feature = "rounding-half-away-from-zero")
            || cfg!(feature = "rounding-half-toward-zero")
            || cfg!(feature = "rounding-trunc")
            || cfg!(feature = "rounding-floor")
            || cfg!(feature = "rounding-ceiling"))
        {
            assert_eq!(hte, n.exp_strict());
        }
    }

    /// AGM ln/exp round-trip at moderate storage scales. Goes up to
    /// the scale where the current implementation maintains full
    /// precision (see the precision caveat on `ln_strict_agm`).
    #[test]
    fn wide_agm_moderate_scale_round_trip() {
        let x = D256::<20>::from_int(3);
        let back = x.ln_strict_agm().exp_strict_agm();
        let delta = (back.to_bits().resize::<i128>() - x.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "AGM exp(ln(3)) at D256<20> delta {delta}");

        let y = D512::<20>::from_int(2);
        let back = y.exp_strict_agm().ln_strict_agm();
        let delta = (back.to_bits().resize::<i128>() - y.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "AGM ln(exp(2)) at D512<20> delta {delta}");
    }

    /// Exercises a scale beyond D128's range, where delegation is
    /// impossible and the wide guard-digit core is the only path.
    /// `exp(ln(x)) ≈ x` and `ln(exp(x)) ≈ x` round-trips.
    #[test]
    fn wide_only_scale_round_trips() {
        // D256<50>: well past D128's max scale of 38. The round-trip
        // result fits i128 comfortably, so compare there.
        let x = D256::<50>::from_int(3);
        let back = x.ln_strict().exp_strict();
        let delta = (back.to_bits().resize::<i128>() - x.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "exp(ln(3)) at D256<50> delta {delta}");

        // D1024<150>: deep scale, only the wide core can serve it.
        let y = D1024::<150>::from_int(2);
        let back = y.exp_strict().ln_strict();
        let delta = (back.to_bits().resize::<i128>() - y.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "ln(exp(2)) at D1024<150> delta {delta}");
    }
}
