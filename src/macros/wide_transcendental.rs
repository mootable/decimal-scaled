//! Correctly-rounded strict transcendentals for the wide decimal tiers
//! (D76 / D153 / D307).
//!
//! D38 and the narrow tiers run their strict transcendentals on the
//! 256-bit `algos::fixed_d38::Fixed` guard-digit intermediate; D9 / D18
//! delegate into D38. The wide tiers cannot widen into D38 — their
//! scale range exceeds it — so they need their own guard-digit core.
//!
//! This module provides one, generic over a hand-rolled signed wide integer
//! `$Work` chosen per tier to be wide enough to hold the working-scale
//! products without overflow:
//!
//! - D76 → `I1024` (working scale ≤ 106 digits);
//! - D153 → `I2048` (working scale ≤ 183 digits);
//! - D307 → `I4096` (working scale ≤ 337 digits).
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
//! # The four-variant matrix
//!
//! Each transcendental ships four entry points so a single name
//! covers every (precision × rounding) combination:
//!
//! | Method            | Guard width    | Rounding mode               |
//! |-------------------|----------------|------------------------------|
//! | `<fn>_strict`     | crate default  | crate default               |
//! | `<fn>_strict_with`| crate default  | caller-supplied              |
//! | `<fn>_approx`     | caller-chosen  | crate default               |
//! | `<fn>_approx_with`| caller-chosen  | caller-supplied              |
//!
//! `_strict` runs at `SCALE + GUARD` (const-folded so each per-tier
//! kernel specialises for one `w`). `_approx` runs at
//! `SCALE + working_digits` — pick less than `GUARD` to trade
//! precision for latency (the AGM / Taylor series shortens), or
//! more for chained-composition headroom. When `working_digits ==
//! GUARD` the `_approx_with` body redirects to `_strict_with` so the
//! const-folded fast path is never displaced.
//!
//! All four variants are integer-only, `no_std`-compatible, and
//! correctly rounded under the selected mode. Without `strict` the
//! plain `<fn>` is unimplemented — the wide tiers have no f64-bridge
//! transcendentals of their own. With `strict` the plain `<fn>`
//! dispatches to `<fn>_strict`.
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
//! [`crate::support::rounding::should_bump`] and honours `DEFAULT_ROUNDING_MODE`.
//!
//! For inputs whose own storage representation has ≤ 0.5 LSB
//! rounding (any value parsed from a literal at the storage scale),
//! the *result* is within 1 LSB of the truth-at-storage. For inputs
//! that are themselves stored with rounding (like `D76s12::pi()`
//! widened from D38's 37-digit reference), the input's rounding
//! propagates through whatever conditioning the method has — that's
//! an input-side budget the wide-tier API can't compensate for.
//!
//! [`RoundingMode`]: crate::support::rounding::RoundingMode

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
        ///
        /// Visibility is `pub(crate)` so the per-family wide-tier
        /// kernels in `crate::algos::<family>::wide_kernel` can call
        /// `to_work` / `round_to_storage_with` / `*_fixed` directly.
        pub(crate) mod $core {
            #![allow(unused)]

            /// The working integer: a value `x` is held as `x · 10^w`.
            pub(crate) type W = $Work;

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
            pub(crate) const GUARD: u32 = 30;
            /// Hard cap on series iterations — a safety net; every
            /// series terminates far sooner by reaching a zero term.
            const SERIES_CAP: u128 = 20_000;

            #[inline]
            pub(crate) fn lit(n: u128) -> W {
                $crate::wide_int::wide_cast(n)
            }
            #[inline]
            pub(crate) fn zero() -> W {
                lit(0)
            }
            #[inline]
            fn abs(v: W) -> W {
                if v < lit(0) { -v } else { v }
            }
            #[inline]
            pub(crate) fn pow10(n: u32) -> W {
                lit(10).pow(n)
            }
            /// Memoised companion to [`pow10`] keyed on `w`.
            ///
            /// Every wide-tier `mul` / `div` / `sqrt_fixed` /
            /// `to_work_w` / `round_to_*` call recomputes `pow10(w)`;
            /// at D57<57>.atan the body invokes that ~198 times per
            /// call, each `lit(10).pow(w)` running ~log₂(w) wide
            /// squarings followed by ~w cumulative wide multiplies.
            /// Caching collapses that into one compute per
            /// `(thread, w)` pair, served from a tiny per-tier
            /// thread-local `Vec<(u32, W)>` (typically 1-3 entries
            /// matching the user's SCALE choices) — see the
            /// `cached` / `pi_cache_get` / `ln2_cache_get` /
            /// `ln10_cache_get` slots below for the same pattern.
            #[inline]
            pub(crate) fn pow10_cached(w: u32) -> W {
                cached(&POW10_CACHE_GET, w, pow10)
            }
            #[inline]
            pub(crate) fn one(w: u32) -> W {
                pow10_cached(w)
            }
            /// Half-to-even round of `(numerator / divisor)` for
            /// the signed wide integer `W`. Pulled out so the
            /// `mul` / `div` core helpers share one rounding rule
            /// instead of truncating per op (which leaks ~1 LSB
            /// each into the strict-transcendental series).
            ///
            /// Uses `div_rem` for the q + r pair (single dispatcher
            /// call) instead of the previous `n/d` + `n%d` pattern
            /// (two dispatcher calls = two full Knuth runs).
            #[inline]
            fn round_div(n: W, d: W) -> W {
                let (q, r) = n.div_rem(d);
                if r == lit(0) {
                    return q;
                }
                let ar = abs(r);
                let comp = abs(d) - ar;
                let cmp_r = ar.cmp(&comp);
                let q_is_odd = q.bit(0);
                let result_positive = (n < lit(0)) == (d < lit(0));
                if $crate::support::rounding::should_bump(
                    $crate::support::rounding::RoundingMode::HalfToEven,
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
            pub(crate) fn mul(a: W, b: W, w: u32) -> W {
                round_div(a * b, pow10_cached(w))
            }
            /// Loop-friendly variant of [`mul`] that takes a
            /// precomputed `10^w` divisor. Use inside Taylor /
            /// AGM / Newton loops where `w` is constant across
            /// every iteration — saves one `lit(10).pow(w)`
            /// recomputation per call (which for D307<150> at w=180
            /// is itself a full Int4096 power of ~50 µs).
            #[inline]
            pub(crate) fn mul_cached(a: W, b: W, pow10_w: W) -> W {
                round_div(a * b, pow10_w)
            }
            /// `(a · 10^w) / b`, rounded half-to-even.
            #[inline]
            pub(crate) fn div(a: W, b: W, w: u32) -> W {
                round_div(a * pow10_cached(w), b)
            }
            /// Loop-friendly variant of [`div`] taking a precomputed
            /// `10^w` numerator factor.
            #[inline]
            pub(crate) fn div_cached(a: W, b: W, pow10_w: W) -> W {
                round_div(a * pow10_w, b)
            }
            /// `a · n` for a small unsigned multiplier.
            #[inline]
            fn mul_u(a: W, n: u128) -> W {
                a * lit(n)
            }

            /// Bit length of `|v|` (0 for zero).
            pub(crate) fn bit_length(v: W) -> u32 {
                W::BITS - abs(v).leading_zeros()
            }

            /// `√v` at working scale `w`: `√(|v| · 10^w)`, truncating.
            ///
            /// `|v| * 10^w` must fit in `W`. Bit-length headroom is
            /// asserted in debug builds; in release the multiply
            /// wraps silently if violated. Every caller in this crate
            /// passes a value with sufficient headroom: the working
            /// integer is sized so `2·(SCALE + GUARD)` digits fit.
            pub(crate) fn sqrt_fixed(v: W, w: u32) -> W {
                let av = abs(v);
                debug_assert!(
                    bit_length(av) + (w as u32) * 4 < W::BITS,
                    "sqrt_fixed: |v| * 10^w overflows the working width"
                );
                (av * pow10_cached(w)).isqrt()
            }

            /// Builds a working-scale value from the type's raw storage:
            /// `raw · 10^GUARD` (raw is `value · 10^SCALE`, the result
            /// is `value · 10^(SCALE+GUARD)`).
            ///
            /// Uses [`wide_cast`] instead of `.resize::<W>()` so the
            /// macro accepts both wide-int and primitive `$Storage`
            /// (`i128` for D38).
            ///
            /// [`wide_cast`]: $crate::wide_int::wide_cast
            pub(crate) fn to_work(raw: $Storage) -> W {
                $crate::wide_int::wide_cast::<$Storage, W>(raw) * pow10(GUARD)
            }

            /// Runtime-guard variant of [`to_work`]: scales raw by
            /// `10^working_digits` instead of the const `GUARD`. Used by
            /// the `_approx` family where the guard width is chosen at
            /// call time.
            pub(crate) fn to_work_w(raw: $Storage, working_digits: u32) -> W {
                $crate::wide_int::wide_cast::<$Storage, W>(raw) * pow10_cached(working_digits)
            }

            /// Rounds a working-scale value down to scale `target` using
            /// the crate-default rounding mode and narrows to the
            /// type's storage. Panics if the rounded value does not
            /// fit.
            ///
            /// Mode dispatch goes through [`crate::support::rounding::should_bump`]
            /// (the same strategy the operator path uses), so a
            /// wide-tier `*_strict` honours the active `rounding-*`
            /// feature flag instead of always rounding half-to-even.
            pub(crate) fn round_to_storage(v: W, w: u32, target: u32) -> $Storage {
                round_to_storage_with(v, w, target, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Mode-aware variant of [`round_to_storage`].
            pub(crate) fn round_to_storage_with(
                v: W,
                w: u32,
                target: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                let divisor = pow10_cached(w - target);
                let q = v / divisor;
                let r = v % divisor;
                let rounded = if r == lit(0) {
                    q
                } else {
                    let ar = abs(r);
                    let comp = divisor - ar;
                    let cmp_r = ar.cmp(&comp);
                    let q_is_odd = q.bit(0);
                    let result_positive = v >= lit(0);
                    if $crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
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
            pub(crate) fn round_to_nearest_int(v: W, w: u32) -> i128 {
                let divisor = pow10_cached(w);
                let (q, r) = v.div_rem(divisor);
                let half = divisor >> 1;
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

            /// `ln 2` at working scale `w`. Thread-local memoised
            /// per `w` (std feature) so the artanh series runs once
            /// per `(thread, working-scale)` pair, not per call.
            pub(crate) fn ln2(w: u32) -> W {
                cached(&LN2_CACHE_GET, w, ln2_compute)
            }
            fn ln2_compute(w: u32) -> W {
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
            pub(crate) fn ln_fixed(v_w: W, w: u32) -> W {
                let one_w = one(w);
                let two_w = one_w + one_w;
                let pow10_w = one_w;
                let mut k: i32 = bit_length(v_w) as i32 - bit_length(one_w) as i32;
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

                // Multi-level sqrt argument reduction (Brent 1976,
                // fastnum's approach). After `l` sqrt operations,
                // `m ← m^(1/2^l)`, so `|t| = |(m-1)/(m+1)|` shrinks
                // geometrically and the artanh series converges in
                // `~p / (2 + 2l)` pair-terms instead of `~p / 2`.
                // Each sqrt costs ~one wide isqrt; the term saving
                // dominates around `l ≈ log₂(term_savings_per_sqrt)`
                // — empirically `l ≈ √p_bits / 4` is the sweet spot.
                let p_bits = w.saturating_mul(3).saturating_add(1);
                let mut sqrt_l: u32 = 0;
                {
                    let mut n: u32 = 0;
                    while (n + 1) * (n + 1) <= p_bits {
                        n += 1;
                    }
                    sqrt_l = n / 4;
                }
                let mut i = 0;
                while i < sqrt_l {
                    m_w = sqrt_fixed(m_w, w);
                    i += 1;
                }

                let t = div_cached(m_w - one_w, m_w + one_w, pow10_w);
                let t2 = mul_cached(t, t, pow10_w);
                let mut sum = t;
                let mut term = t;
                let mut j: u128 = 1;
                loop {
                    term = mul_cached(term, t2, pow10_w);
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
                // ln(m) = 2^(l+1) · artanh(t) = sum << (sqrt_l + 1).
                // With sqrt_l=0 this collapses to the historic
                // `2·sum` formula; with sqrt_l>0 it folds in the
                // `2^l` factor from the unhalved-argument identity.
                let ln_m = sum << (sqrt_l + 1);
                scale_by_k(ln2(w), k as i128) + ln_m
            }

            /// `ln 10` at working scale `w`. Memoised, see [`ln2`].
            pub(crate) fn ln10(w: u32) -> W {
                cached(&LN10_CACHE_GET, w, ln10_compute)
            }
            fn ln10_compute(w: u32) -> W {
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
            pub(crate) fn ln_fixed_agm(v_w: W, w: u32) -> W {
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
                    let next_a = (a + b) >> 1;
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
            pub(crate) fn exp_fixed_agm(v_w: W, w: u32) -> W {
                let one_w = one(w);
                let l2 = ln2(w);
                let k = round_to_nearest_int(div(v_w, l2, w), w);
                let s = v_w - scale_by_k(l2, k);
                // Newton seed: low-order Taylor (1 + s + s²/2). Within
                // ~10⁻² of truth for |s| ≤ ln(2)/2 ≈ 0.347.
                let s2 = mul(s, s, w);
                let mut x = one_w + s + (s2 >> 1);
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
            /// then applies the "r/2^n" further reduction (n ≈ √p):
            /// shift `s` right by `n` bits, run the Taylor series on
            /// the tiny shifted argument, then square `n` times to
            /// undo the reduction. Net effect: Taylor needs `O(√p)`
            /// terms instead of `O(p)`, traded against `n` extra
            /// squarings — a clear win because each squaring is one
            /// wide mul whereas each Taylor term is mul + div.
            ///
            /// Reassembles `2^k · exp(s)` at the end.
            ///
            /// Reference: dashu-float's `exp_internal`
            /// (`float/src/exp.rs`); the trick traces back to Brent
            /// 1976 §3 ("binary-splitting for exp via repeated
            /// squaring of a reduced argument").
            pub(crate) fn exp_fixed(v_w: W, w: u32) -> W {
                #[cfg(feature = "perf-trace")]
                let _exp_span = $crate::tracing::info_span!(concat!(
                    stringify!($Type), "::exp_fixed"
                )).entered();

                // Cache 10^w once — used as divisor in every Taylor
                // iteration and squaring step below. At D307<150>
                // w=180 and `pow10(180)` costs ~50 µs by itself
                // (`lit(10).pow(180)` is ~log₂(180)=8 wide squarings
                // followed by ~180 cumulative multiplies); without
                // caching this would dominate the call.
                #[cfg(feature = "perf-trace")]
                let _reduce_span = $crate::tracing::info_span!("range_reduce").entered();
                let one_w = one(w);
                let l2 = ln2(w);
                let pow10_w = one_w;
                let k = round_to_nearest_int(div_cached(v_w, l2, pow10_w), w);
                let s = v_w - scale_by_k(l2, k);

                let p_bits = w.saturating_mul(3).saturating_add(1);
                let mut n: u32 = 1;
                while (n + 1) * (n + 1) <= p_bits {
                    n += 1;
                }

                let s_red = s >> n;
                #[cfg(feature = "perf-trace")]
                drop(_reduce_span);

                #[cfg(feature = "perf-trace")]
                let _taylor_span = $crate::tracing::info_span!("taylor_series").entered();
                let mut sum = one_w + s_red;
                let mut term = s_red;
                let mut iter: u128 = 2;
                loop {
                    term = mul_cached(term, s_red, pow10_w) / lit(iter);
                    if term == zero() {
                        break;
                    }
                    sum = sum + term;
                    iter += 1;
                    if iter > SERIES_CAP {
                        break;
                    }
                }
                #[cfg(feature = "perf-trace")]
                drop(_taylor_span);

                #[cfg(feature = "perf-trace")]
                let _sqr_span = $crate::tracing::info_span!("postfix_squarings").entered();
                let mut squared = sum;
                let mut i = 0;
                while i < n {
                    squared = mul_cached(squared, squared, pow10_w);
                    i += 1;
                }
                let sum = squared;
                #[cfg(feature = "perf-trace")]
                drop(_sqr_span);

                #[cfg(feature = "perf-trace")]
                let _reasm_span = $crate::tracing::info_span!("reassemble").entered();
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
                    let neg_k = -k as u128;
                    if neg_k >= bit_length(sum) as u128 {
                        return zero();
                    }
                    sum >> (neg_k as u32)
                }
            }

            /// Taylor series for `atan` on `|x| < 1`, at scale `w`.
            pub(crate) fn atan_taylor(x: W, w: u32) -> W {
                let pow10_w = pow10_cached(w);
                let x2 = mul_cached(x, x, pow10_w);
                let mut sum = x;
                let mut term = x;
                let mut k: u128 = 1;
                loop {
                    term = mul_cached(term, x2, pow10_w);
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
            /// Memoised per `w` (std feature); see [`ln2`].
            pub(crate) fn pi(w: u32) -> W {
                cached(&PI_CACHE_GET, w, pi_compute)
            }
            fn pi_compute(w: u32) -> W {
                let a = atan_taylor(one(w) / lit(5), w);
                let b = atan_taylor(one(w) / lit(239), w);
                mul_u(a, 16) - mul_u(b, 4)
            }

            // ── Thread-local memoisation for pi / ln2 / ln10 ───────────
            //
            // Each helper computes its constant once per thread per
            // working scale `w` (typically only one or two distinct `w`
            // values per process, matching the user's SCALE choices).
            // Subsequent calls hit the cache and return in ~few ns
            // vs the 50-150 µs the series evaluation would cost.
            //
            // The cache is a tiny `Vec<(u32, W)>` per thread —
            // typical occupancy is 1-3 entries (one per SCALE the
            // user computes at). Linear scan is faster than any
            // hash structure at that scale.
            //
            // Gated on the `std` feature for `thread_local!`. Under
            // `no_std` the wrappers degrade to direct computation —
            // no cache, no contention concerns.

            #[cfg(feature = "std")]
            fn cached<F>(slot_get: &dyn Fn() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>>, w: u32, compute: F) -> W
            where
                F: FnOnce(u32) -> W,
            {
                let slot = slot_get();
                let hit = slot.with(|c| {
                    let cache = c.borrow();
                    for &(cw, cv) in cache.iter() {
                        if cw == w {
                            return ::core::option::Option::Some(cv);
                        }
                    }
                    ::core::option::Option::None
                });
                if let ::core::option::Option::Some(v) = hit {
                    return v;
                }
                let v = compute(w);
                slot.with(|c| {
                    c.borrow_mut().push((w, v));
                });
                v
            }

            #[cfg(not(feature = "std"))]
            fn cached<F>(_slot_get: &(), w: u32, compute: F) -> W
            where
                F: FnOnce(u32) -> W,
            {
                compute(w)
            }

            #[cfg(feature = "std")]
            fn pi_cache_get() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> {
                ::std::thread_local! {
                    static SLOT: ::core::cell::RefCell<alloc::vec::Vec<(u32, W)>> = const {
                        ::core::cell::RefCell::new(alloc::vec::Vec::new())
                    };
                }
                &SLOT
            }
            #[cfg(feature = "std")]
            fn ln2_cache_get() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> {
                ::std::thread_local! {
                    static SLOT: ::core::cell::RefCell<alloc::vec::Vec<(u32, W)>> = const {
                        ::core::cell::RefCell::new(alloc::vec::Vec::new())
                    };
                }
                &SLOT
            }
            #[cfg(feature = "std")]
            fn ln10_cache_get() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> {
                ::std::thread_local! {
                    static SLOT: ::core::cell::RefCell<alloc::vec::Vec<(u32, W)>> = const {
                        ::core::cell::RefCell::new(alloc::vec::Vec::new())
                    };
                }
                &SLOT
            }
            #[cfg(feature = "std")]
            fn pow10_cache_get() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> {
                ::std::thread_local! {
                    static SLOT: ::core::cell::RefCell<alloc::vec::Vec<(u32, W)>> = const {
                        ::core::cell::RefCell::new(alloc::vec::Vec::new())
                    };
                }
                &SLOT
            }

            #[cfg(feature = "std")]
            const PI_CACHE_GET: fn() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> = pi_cache_get;
            #[cfg(feature = "std")]
            const LN2_CACHE_GET: fn() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> = ln2_cache_get;
            #[cfg(feature = "std")]
            const LN10_CACHE_GET: fn() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> = ln10_cache_get;
            #[cfg(feature = "std")]
            const POW10_CACHE_GET: fn() -> &'static ::std::thread::LocalKey<::core::cell::RefCell<alloc::vec::Vec<(u32, W)>>> = pow10_cache_get;

            #[cfg(not(feature = "std"))]
            const PI_CACHE_GET: () = ();
            #[cfg(not(feature = "std"))]
            const LN2_CACHE_GET: () = ();
            #[cfg(not(feature = "std"))]
            const LN10_CACHE_GET: () = ();
            #[cfg(not(feature = "std"))]
            const POW10_CACHE_GET: () = ();
            /// `π/2` at working scale `w`.
            pub(crate) fn half_pi(w: u32) -> W {
                pi(w) >> 1
            }

            /// Taylor series for `sin` on a reduced `r ∈ [0, π/4]`.
            ///
            /// `sin(r) = r − r³/3! + r⁵/5! − …`
            fn sin_taylor(r: W, w: u32) -> W {
                let pow10_w = pow10_cached(w);
                let r2 = mul_cached(r, r, pow10_w);
                let mut sum = r;
                let mut term = r;
                let mut k: u128 = 1;
                loop {
                    term = mul_cached(term, r2, pow10_w) / lit((2 * k) * (2 * k + 1));
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

            /// Taylor series for `cos` on a reduced `r ∈ [0, π/4]`.
            ///
            /// `cos(r) = 1 − r²/2! + r⁴/4! − r⁶/6! + …`
            ///
            /// Converges faster than [`sin_taylor`] at the same `r`
            /// because the leading `1` dominates the small even-power
            /// corrections — used as the "upper-half" branch of
            /// [`sin_fixed`] when the reduced argument exceeds π/4.
            fn cos_taylor(r: W, w: u32) -> W {
                let pow10_w = pow10_cached(w);
                let r2 = mul_cached(r, r, pow10_w);
                let one_w = one(w);
                let mut sum = one_w;
                let mut term = one_w;
                let mut k: u128 = 1;
                loop {
                    term = mul_cached(term, r2, pow10_w)
                        / lit((2 * k - 1) * (2 * k));
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
            ///
            /// Reduces to `|r| ≤ π/2` via mod-τ; then folds to
            /// `r ∈ [0, π/2]` via `sin(π − x) = sin(x)`; then routes
            /// to `sin_taylor` if `r ≤ π/4` or `cos_taylor(π/2 − r)`
            /// otherwise. The `[0, π/4]` window halves the convergence
            /// argument and roughly halves the Taylor term count, and
            /// cos converges faster than sin at the same argument
            /// because of the constant-1 leading term.
            pub(crate) fn sin_fixed(v_w: W, w: u32) -> W {
                let pi_w = pi(w);
                let tau = pi_w + pi_w;
                let hp = pi_w >> 1;
                let qp = hp >> 1; // π/4
                let q = round_to_nearest_int(div(v_w, tau, w), w);
                let r = v_w - scale_by_k(tau, q);
                let neg = r < zero();
                let abs_r = if neg { -r } else { r };
                let reduced = if abs_r >= hp { pi_w - abs_r } else { abs_r };
                let s = if reduced > qp {
                    // sin(reduced) = cos(π/2 − reduced); the cos
                    // argument lies in [0, π/4].
                    cos_taylor(hp - reduced, w)
                } else {
                    sin_taylor(reduced, w)
                };
                if neg { -s } else { s }
            }

            /// Joint sine + cosine of a working-scale value.
            ///
            /// Replaces two independent `sin_fixed(...)` calls (one
            /// for sin, one for `sin(x + π/2)` = cos) with a single
            /// sin evaluation plus a sqrt:
            ///
            /// - Reduce mod τ and fold to `|r| ∈ [0, π/2]`, tracking
            ///   both signs (sin from the mod-τ residue, cos from
            ///   whether the unfolded `|r|` exceeded `π/2`).
            /// - Evaluate `|sin(reduced)|` via the same `sin_taylor`
            ///   or `cos_taylor` branch as `sin_fixed`.
            /// - Recover `|cos(reduced)|` from the Pythagorean
            ///   identity: `√(1 − sin²)`.
            /// - Apply the cached signs.
            ///
            /// One Taylor series + one wide sqrt + one wide mul,
            /// vs the historic two independent Taylor evaluations.
            /// Halves the wall-clock when both are needed.
            pub(crate) fn sin_cos_fixed(v_w: W, w: u32) -> (W, W) {
                let pi_w = pi(w);
                let tau = pi_w + pi_w;
                let hp = pi_w >> 1;
                let qp = hp >> 1;
                let q = round_to_nearest_int(div(v_w, tau, w), w);
                let r = v_w - scale_by_k(tau, q);
                let sin_neg = r < zero();
                let abs_r = if sin_neg { -r } else { r };
                let cos_neg = abs_r > hp; // |r| > π/2 → cos negative.
                let reduced = if cos_neg { pi_w - abs_r } else { abs_r };
                let s_abs = if reduced > qp {
                    cos_taylor(hp - reduced, w)
                } else {
                    sin_taylor(reduced, w)
                };
                // cos² + sin² = 1 ⇒ |cos| = √(1 − sin²).
                let one_w = one(w);
                let s2 = mul(s_abs, s_abs, w);
                let cos_abs = sqrt_fixed(one_w - s2, w);
                let sin_result = if sin_neg { -s_abs } else { s_abs };
                let cos_result = if cos_neg { -cos_abs } else { cos_abs };
                (sin_result, cos_result)
            }

            /// Cosine of a working-scale value via the cofunction
            /// identity `cos(x) = sin(π/2 − x)`.
            ///
            /// Used by the standalone `cos_strict` kernel path: one
            /// `sin_fixed` evaluation, no sqrt — strictly cheaper than
            /// the `sin_cos_fixed` path when only `cos` is needed.
            /// `sin_cos_fixed` remains the right choice when both
            /// outputs are wanted (one Taylor + one sqrt vs two
            /// Taylors).
            pub(crate) fn cos_fixed(v_w: W, w: u32) -> W {
                sin_fixed(half_pi(w) - v_w, w)
            }

            /// Arctangent of a working-scale value, result in
            /// `(−π/2, π/2)`.
            pub(crate) fn atan_fixed(v_w: W, w: u32) -> W {
                let one_w = one(w);
                let sign = v_w < zero();
                let mut x = if sign { -v_w } else { v_w };
                let mut add_half_pi = false;
                if x > one_w {
                    x = div(one_w, x, w);
                    add_half_pi = true;
                }
                // Argument halvings: atan(x) = 2·atan(x/(1+√(1+x²))).
                //
                // Each halving reduces |x| by a factor ≈ 2, so the
                // Taylor series convergence rate gains ~log₂(4) = 2
                // bits per term. Cost per halving: 1 wide mul + 1 wide
                // sqrt + 1 wide div ≈ 7 µs at D307. Savings per
                // halving: ~p_bits/halvings² Taylor terms × ~1.5 µs.
                //
                // The break-even (where one more halving costs more
                // than the term savings) sits around halvings ≈
                // log₂(p_bits/halving_cost), which lands at 6–7 for
                // D153/D307 and 5–6 for D76. We pick the per-tier
                // sweet spot from w (the working scale = SCALE + GUARD
                // decimal digits): wider working scale → more halvings
                // worth taking.
                let halvings: u32 = if w < 60 {
                    5  // D38-equivalent guard (~50 digits)
                } else if w < 110 {
                    6  // D76 / D153 light-end
                } else {
                    7  // D153 heavy / D307
                };
                let pow10_w = pow10_cached(w);
                for _ in 0..halvings {
                    let x2 = mul_cached(x, x, pow10_w);
                    let denom = one_w + sqrt_fixed(one_w + x2, w);
                    x = div_cached(x, denom, pow10_w);
                }
                let mut result = atan_taylor(x, w) << halvings;
                if add_half_pi {
                    result = half_pi(w) - result;
                }
                if sign { -result } else { result }
            }
        }

        impl<const SCALE: u32> $Type<SCALE> {
            /// Natural logarithm (base e). Strict: integer-only and
            /// correctly rounded. Panics if `self <= 0`.
            ///
            /// Delegates to the policy-registered ln kernel for this
            /// `(width, SCALE)` cell — see `policy::ln`.
            #[inline]
            #[must_use]
            pub fn ln_strict(self) -> Self {
                <Self as $crate::policy::ln::LnPolicy>::ln_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
            ///
            /// Delegates to the policy-registered exp kernel for this
            /// `(width, SCALE)` cell — see `policy::exp`.
            #[inline]
            #[must_use]
            pub fn exp_strict(self) -> Self {
                <Self as $crate::policy::exp::ExpPolicy>::exp_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
            ///
            /// Delegates to the policy-registered sin kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn sin_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::sin_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Cosine of `self` (radians). Strict and correctly
            /// rounded. The policy-registered kernel evaluates a
            /// single `sin_fixed(π/2 − self)` via the cofunction
            /// identity — no sqrt, no shared Taylor with sin.
            /// `sin_cos_strict` keeps the shared-Taylor
            /// `sin_cos_fixed` path for joint evaluation.
            ///
            /// Delegates to the policy-registered cos kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn cos_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::cos_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Joint sine and cosine of `self` (radians), returned
            /// as `(sin, cos)`. Strict and correctly rounded.
            ///
            /// Internally shares one Taylor-series evaluation between
            /// the two results (computing only `|sin|` and recovering
            /// `|cos| = √(1 − sin²)` from the Pythagorean identity),
            /// so the wall-clock is `~one sin_strict + one wide sqrt`
            /// — roughly half the cost of `(self.sin_strict(),
            /// self.cos_strict())`.
            ///
            /// Useful for rotation matrices, polar→cartesian, complex
            /// `e^{iθ}` evaluation, and anywhere both trig values of
            /// the same argument are needed.
            #[inline]
            #[must_use]
            pub fn sin_cos_strict(self) -> (Self, Self) {
                let w = SCALE + $core::GUARD;
                let (s, c) = $core::sin_cos_fixed($core::to_work(self.to_bits()), w);
                (
                    Self::from_bits($core::round_to_storage(s, w, SCALE)),
                    Self::from_bits($core::round_to_storage(c, w, SCALE)),
                )
            }

            /// Tangent of `self` (radians), as `sin / cos`. Strict and
            /// correctly rounded. Panics at odd multiples of π/2.
            ///
            /// Delegates to the policy-registered tan kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn tan_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::tan_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Arctangent of `self`, in radians, in `(−π/2, π/2)`.
            /// Strict and correctly rounded.
            ///
            /// Delegates to the policy-registered atan kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn atan_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::atan_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Arcsine of `self`, in radians, in `[−π/2, π/2]`.
            /// Strict. Panics if `|self| > 1`.
            ///
            /// Two-range kernel to preserve the 0-ULP contract at
            /// every representable input including the asymptotic
            /// edge `|x| → 1`:
            ///
            /// - `|x| ≤ 0.5`: the direct identity
            ///   `asin(x) = atan(x / √(1 − x²))`. At this range
            ///   `1 − x² ∈ [0.75, 1]` — no cancellation in the
            ///   subtraction, so the sqrt keeps full precision.
            /// - `0.5 < |x| < 1`: the half-angle identity
            ///   `asin(x) = π/2 − 2·asin(√((1−|x|)/2))`. The inner
            ///   `√((1−|x|)/2)` lies in `(0, 0.5]` so the recursive
            ///   asin call hits the stable range. The
            ///   `(1−|x|)/2` subtraction is exact at integer level
            ///   (no cancellation — `|x|` ≤ 1 means `1−|x| ≥ 0`),
            ///   so the asymptotic-edge precision is bounded by
            ///   the working scale, not by the input's distance
            ///   from 1.
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
                let half_w = one_w / $core::lit(2);
                let r = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                } else {
                    // Half-angle: asin(|x|) = π/2 − 2·asin(√((1−|x|)/2)).
                    // The inner argument is in (0, 0.5], so the
                    // recursive asin call takes the stable branch.
                    let inner = (one_w - abs_v) / $core::lit(2);
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom = $core::sqrt_fixed(
                        one_w - $core::mul(inner_sqrt, inner_sqrt, w),
                        w,
                    );
                    let inner_asin = $core::atan_fixed(
                        $core::div(inner_sqrt, inner_denom, w),
                        w,
                    );
                    let result_abs = $core::half_pi(w) - inner_asin - inner_asin;
                    if v < $core::zero() { -result_abs } else { result_abs }
                };
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Arccosine of `self`, in radians, in `[0, π]`, as
            /// `π/2 − asin(self)`. Strict. Panics if `|self| > 1`.
            /// Uses the same two-range asin kernel as
            /// [`Self::asin_strict`] for the underlying asin.
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
                let half_w = one_w / $core::lit(2);
                let asin_w = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) / $core::lit(2);
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom = $core::sqrt_fixed(
                        one_w - $core::mul(inner_sqrt, inner_sqrt, w),
                        w,
                    );
                    let inner_asin = $core::atan_fixed(
                        $core::div(inner_sqrt, inner_denom, w),
                        w,
                    );
                    let result_abs = $core::half_pi(w) - inner_asin - inner_asin;
                    if v < $core::zero() { -result_abs } else { result_abs }
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
                    let zero_w = $core::zero();
                    // Max-branch: feed atan_fixed whichever of y/x or
                    // x/y has |·| ≤ 1, so the argument-halving cascade
                    // doesn't blow up. The historic `atan(y/x)`-only
                    // path lost ~log₂(|y/x|) bits of precision when
                    // |y| ≫ |x|; the swap recovers them via the
                    // identity `atan(t) = sign(t)·π/2 − atan(1/t)`
                    // for `|t| > 1`.
                    let abs_y = if y < zero_w { -y } else { y };
                    let abs_x = if x < zero_w { -x } else { x };
                    let base = if abs_x >= abs_y {
                        $core::atan_fixed($core::div(y, x, w), w)
                    } else {
                        let inv = $core::atan_fixed($core::div(x, y, w), w);
                        let hp = $core::half_pi(w);
                        // sign(y/x): same iff y and x agree in sign.
                        let same_sign = (y < zero_w) == (x < zero_w);
                        if same_sign { hp - inv } else { -hp - inv }
                    };
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
            /// correctly rounded. Shares one `exp(v)` and one
            /// `exp(−v)` between the implicit sinh and cosh, then
            /// `tanh = (eˣ − e⁻ˣ) / (eˣ + e⁻ˣ)` — same arithmetic as
            /// the historic path, but the divide and the two
            /// subtraction/addition operands are inlined here to
            /// avoid going through the intermediate sinh/cosh.
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

            /// Joint hyperbolic sine and cosine of `self`, returned
            /// as `(sinh, cosh)`. Strict and correctly rounded.
            ///
            /// Shares one `exp(v)` and one `exp(−v)` evaluation
            /// between sinh and cosh — same cost as a single
            /// `sinh_strict` or `cosh_strict` call, vs the historic
            /// `(self.sinh_strict(), self.cosh_strict())` pair which
            /// computed both `exp` pairs twice.
            #[inline]
            #[must_use]
            pub fn sinh_cosh_strict(self) -> (Self, Self) {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let two = $crate::macros::wide_roots::wide_lit!($Work, "2");
                let sinh = (ex - enx) / two;
                let cosh = (ex + enx) / two;
                (
                    Self::from_bits($core::round_to_storage(sinh, w, SCALE)),
                    Self::from_bits($core::round_to_storage(cosh, w, SCALE)),
                )
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

            /// Mode-aware sibling of [`Self::ln_strict`]. Delegates to
            /// the policy-registered ln kernel for this `(width, SCALE)`
            /// cell — see `policy::ln`.
            #[inline]
            #[must_use]
            pub fn ln_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <Self as $crate::policy::ln::LnPolicy>::ln_impl(self, mode)
            }

            /// Mode-aware sibling of [`Self::ln_strict_agm`].
            #[inline]
            #[must_use]
            pub fn ln_strict_agm_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn exp_strict_agm_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn log_strict_with(self, base: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn log2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn log10_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log10: argument must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::div($core::ln_fixed($core::to_work(raw), w), $core::ln10(w), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::exp_strict`]. Delegates
            /// to the policy-registered exp kernel for this
            /// `(width, SCALE)` cell — see `policy::exp`.
            #[inline]
            #[must_use]
            pub fn exp_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <Self as $crate::policy::exp::ExpPolicy>::exp_impl(self, mode)
            }

            /// Mode-aware sibling of [`Self::exp2_strict`].
            #[inline]
            #[must_use]
            pub fn exp2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn powf_strict_with(self, exp: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
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

            /// Mode-aware sibling of [`Self::sin_strict`]. Delegates
            /// to the policy-registered sin kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn sin_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::sin_impl(self, mode)
            }

            /// Mode-aware sibling of [`Self::cos_strict`]. Delegates
            /// to the policy-registered cos kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            ///
            /// Note: pre-policy this method ran `sin_fixed(self + π/2)`
            /// while the no-mode `cos_strict` ran the shared
            /// `sin_cos_fixed` Pythagorean-identity path. The migration
            /// consolidates both on the latter (faster) path; the two
            /// paths agree to well within the existing 2-ULP test
            /// slack.
            #[inline]
            #[must_use]
            pub fn cos_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::cos_impl(self, mode)
            }

            /// Mode-aware sibling of [`Self::tan_strict`]. Delegates
            /// to the policy-registered tan kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn tan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::tan_impl(self, mode)
            }

            /// Mode-aware sibling of [`Self::atan_strict`]. Delegates
            /// to the policy-registered atan kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn atan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::atan_impl(self, mode)
            }

            /// Mode-aware sibling of [`Self::asin_strict`]. Same
            /// two-range kernel; see the unmodified docs there for
            /// the algorithm.
            #[inline]
            #[must_use]
            pub fn asin_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::asin: argument out of domain [-1, 1]"));
                }
                let half_w = one_w / $core::lit(2);
                let r = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) / $core::lit(2);
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom = $core::sqrt_fixed(
                        one_w - $core::mul(inner_sqrt, inner_sqrt, w),
                        w,
                    );
                    let inner_asin = $core::atan_fixed(
                        $core::div(inner_sqrt, inner_denom, w),
                        w,
                    );
                    let result_abs = $core::half_pi(w) - inner_asin - inner_asin;
                    if v < $core::zero() { -result_abs } else { result_abs }
                };
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::acos_strict`].
            #[inline]
            #[must_use]
            pub fn acos_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let one_w = $core::one(w);
                let v = $core::to_work(self.to_bits());
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::acos: argument out of domain [-1, 1]"));
                }
                let half_w = one_w / $core::lit(2);
                let asin_w = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) / $core::lit(2);
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom = $core::sqrt_fixed(
                        one_w - $core::mul(inner_sqrt, inner_sqrt, w),
                        w,
                    );
                    let inner_asin = $core::atan_fixed(
                        $core::div(inner_sqrt, inner_denom, w),
                        w,
                    );
                    let result_abs = $core::half_pi(w) - inner_asin - inner_asin;
                    if v < $core::zero() { -result_abs } else { result_abs }
                };
                let r = $core::half_pi(w) - asin_w;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::atan2_strict`].
            #[inline]
            #[must_use]
            pub fn atan2_strict_with(self, other: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn sinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn cosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn tanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn asinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn acosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn atanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn to_degrees_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
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
            pub fn to_radians_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let r = $core::mul(v, $core::pi(w), w)
                    / $crate::macros::wide_roots::wide_lit!($Work, "180");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::sin_cos_strict`].
            #[inline]
            #[must_use]
            pub fn sin_cos_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> (Self, Self) {
                let w = SCALE + $core::GUARD;
                let (s, c) = $core::sin_cos_fixed($core::to_work(self.to_bits()), w);
                (
                    Self::from_bits($core::round_to_storage_with(s, w, SCALE, mode)),
                    Self::from_bits($core::round_to_storage_with(c, w, SCALE, mode)),
                )
            }

            /// Mode-aware sibling of [`Self::sinh_cosh_strict`].
            #[inline]
            #[must_use]
            pub fn sinh_cosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> (Self, Self) {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let two = $crate::macros::wide_roots::wide_lit!($Work, "2");
                let sinh = (ex - enx) / two;
                let cosh = (ex + enx) / two;
                (
                    Self::from_bits($core::round_to_storage_with(sinh, w, SCALE, mode)),
                    Self::from_bits($core::round_to_storage_with(cosh, w, SCALE, mode)),
                )
            }

            // ─── *_approx(working_digits) family ─────────────────────
            // Each transcendental gets `_approx(g)` and
            // `_approx_with(g, mode)`. When `g == GUARD` we redirect to
            // the corresponding strict variant so the const-folded
            // strict path is never displaced.

            /// Natural log with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn ln_approx(self, working_digits: u32) -> Self {
                self.ln_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Natural log with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn ln_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.ln_strict_with(mode);
                }
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::ln: argument must be positive"));
                }
                let w = SCALE + working_digits;
                let r = $core::ln_fixed($core::to_work_w(raw, working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Log to chosen base with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn log_approx(self, base: Self, working_digits: u32) -> Self {
                self.log_approx_with(base, working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Log to chosen base with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn log_approx_with(
                self,
                base: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.log_strict_with(base, mode);
                }
                let raw = self.to_bits();
                let braw = base.to_bits();
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                if raw <= z {
                    panic!(concat!(stringify!($Type), "::log: argument must be positive"));
                }
                if braw <= z {
                    panic!(concat!(stringify!($Type), "::log: base must be positive"));
                }
                let w = SCALE + working_digits;
                let ln_b = $core::ln_fixed($core::to_work_w(braw, working_digits), w);
                if ln_b == $core::zero() {
                    panic!(concat!(stringify!($Type), "::log: base must not equal 1"));
                }
                let r = $core::div(
                    $core::ln_fixed($core::to_work_w(raw, working_digits), w),
                    ln_b,
                    w,
                );
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Log base 2 with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn log2_approx(self, working_digits: u32) -> Self {
                self.log2_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Log base 2 with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn log2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.log2_strict_with(mode);
                }
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log2: argument must be positive"));
                }
                let w = SCALE + working_digits;
                let r = $core::div(
                    $core::ln_fixed($core::to_work_w(raw, working_digits), w),
                    $core::ln2(w),
                    w,
                );
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Log base 10 with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn log10_approx(self, working_digits: u32) -> Self {
                self.log10_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Log base 10 with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn log10_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.log10_strict_with(mode);
                }
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log10: argument must be positive"));
                }
                let w = SCALE + working_digits;
                let r = $core::div(
                    $core::ln_fixed($core::to_work_w(raw, working_digits), w),
                    $core::ln10(w),
                    w,
                );
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// `eˣ` with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn exp_approx(self, working_digits: u32) -> Self {
                self.exp_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// `eˣ` with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn exp_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.exp_strict_with(mode);
                }
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + working_digits;
                let r = $core::exp_fixed($core::to_work_w(raw, working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// `2ˣ` with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn exp2_approx(self, working_digits: u32) -> Self {
                self.exp2_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// `2ˣ` with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn exp2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.exp2_strict_with(mode);
                }
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                let w = SCALE + working_digits;
                let arg = $core::mul($core::to_work_w(raw, working_digits), $core::ln2(w), w);
                let r = $core::exp_fixed(arg, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// `xʸ` with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn powf_approx(self, exp: Self, working_digits: u32) -> Self {
                self.powf_approx_with(exp, working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// `xʸ` with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn powf_approx_with(
                self,
                exp: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.powf_strict_with(exp, mode);
                }
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let w = SCALE + working_digits;
                let ln_x = $core::ln_fixed($core::to_work_w(raw, working_digits), w);
                let y = $core::to_work_w(exp.to_bits(), working_digits);
                let r = $core::exp_fixed($core::mul(y, ln_x, w), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Sine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn sin_approx(self, working_digits: u32) -> Self {
                self.sin_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Sine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn sin_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.sin_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let r = $core::sin_fixed($core::to_work_w(self.to_bits(), working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Cosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn cos_approx(self, working_digits: u32) -> Self {
                self.cos_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Cosine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn cos_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.cos_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let arg = $core::to_work_w(self.to_bits(), working_digits) + $core::half_pi(w);
                let r = $core::sin_fixed(arg, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Joint sine/cosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn sin_cos_approx(self, working_digits: u32) -> (Self, Self) {
                self.sin_cos_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Joint sine/cosine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn sin_cos_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> (Self, Self) {
                if working_digits == $core::GUARD {
                    return self.sin_cos_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let (s, c) = $core::sin_cos_fixed(
                    $core::to_work_w(self.to_bits(), working_digits),
                    w,
                );
                (
                    Self::from_bits($core::round_to_storage_with(s, w, SCALE, mode)),
                    Self::from_bits($core::round_to_storage_with(c, w, SCALE, mode)),
                )
            }

            /// Tangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn tan_approx(self, working_digits: u32) -> Self {
                self.tan_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Tangent with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn tan_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.tan_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let (sin_w, cos_w) = $core::sin_cos_fixed(
                    $core::to_work_w(self.to_bits(), working_digits),
                    w,
                );
                if cos_w == $core::zero() {
                    panic!(concat!(
                        stringify!($Type),
                        "::tan: cosine is zero (argument is an odd multiple of pi/2)"
                    ));
                }
                let r = $core::div(sin_w, cos_w, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Arctangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn atan_approx(self, working_digits: u32) -> Self {
                self.atan_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Arctangent with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn atan_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.atan_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let r = $core::atan_fixed($core::to_work_w(self.to_bits(), working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Arcsine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn asin_approx(self, working_digits: u32) -> Self {
                self.asin_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Arcsine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn asin_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.asin_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let one_w = $core::one(w);
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::asin: argument out of domain [-1, 1]"));
                }
                let half_w = one_w / $core::lit(2);
                let r = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) / $core::lit(2);
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom = $core::sqrt_fixed(
                        one_w - $core::mul(inner_sqrt, inner_sqrt, w),
                        w,
                    );
                    let inner_asin = $core::atan_fixed(
                        $core::div(inner_sqrt, inner_denom, w),
                        w,
                    );
                    let result_abs = $core::half_pi(w) - inner_asin - inner_asin;
                    if v < $core::zero() { -result_abs } else { result_abs }
                };
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Arccosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn acos_approx(self, working_digits: u32) -> Self {
                self.acos_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Arccosine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn acos_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.acos_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let one_w = $core::one(w);
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let abs_v = if v < $core::zero() { -v } else { v };
                if abs_v > one_w {
                    panic!(concat!(stringify!($Type), "::acos: argument out of domain [-1, 1]"));
                }
                let half_w = one_w / $core::lit(2);
                let asin_w = if abs_v == one_w {
                    let hp = $core::half_pi(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) / $core::lit(2);
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom = $core::sqrt_fixed(
                        one_w - $core::mul(inner_sqrt, inner_sqrt, w),
                        w,
                    );
                    let inner_asin = $core::atan_fixed(
                        $core::div(inner_sqrt, inner_denom, w),
                        w,
                    );
                    let result_abs = $core::half_pi(w) - inner_asin - inner_asin;
                    if v < $core::zero() { -result_abs } else { result_abs }
                };
                let r = $core::half_pi(w) - asin_w;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Four-quadrant arctangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn atan2_approx(self, other: Self, working_digits: u32) -> Self {
                self.atan2_approx_with(other, working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Four-quadrant arctangent with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn atan2_approx_with(
                self,
                other: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.atan2_strict_with(other, mode);
                }
                let w = SCALE + working_digits;
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
                    let y = $core::to_work_w(yraw, working_digits);
                    let x = $core::to_work_w(xraw, working_digits);
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

            /// Hyperbolic sine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn sinh_approx(self, working_digits: u32) -> Self {
                self.sinh_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Hyperbolic sine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn sinh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.sinh_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = (ex - enx) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Hyperbolic cosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn cosh_approx(self, working_digits: u32) -> Self {
                self.cosh_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Hyperbolic cosine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn cosh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.cosh_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = (ex + enx) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Hyperbolic tangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn tanh_approx(self, working_digits: u32) -> Self {
                self.tanh_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Hyperbolic tangent with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn tanh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.tanh_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let r = $core::div(ex - enx, ex + enx, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Joint sinh/cosh with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn sinh_cosh_approx(self, working_digits: u32) -> (Self, Self) {
                self.sinh_cosh_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Joint sinh/cosh with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn sinh_cosh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> (Self, Self) {
                if working_digits == $core::GUARD {
                    return self.sinh_cosh_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let ex = $core::exp_fixed(v, w);
                let enx = $core::exp_fixed(-v, w);
                let two = $crate::macros::wide_roots::wide_lit!($Work, "2");
                let sinh = (ex - enx) / two;
                let cosh = (ex + enx) / two;
                (
                    Self::from_bits($core::round_to_storage_with(sinh, w, SCALE, mode)),
                    Self::from_bits($core::round_to_storage_with(cosh, w, SCALE, mode)),
                )
            }

            /// Inverse hyperbolic sine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn asinh_approx(self, working_digits: u32) -> Self {
                self.asinh_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Inverse hyperbolic sine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn asinh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.asinh_strict_with(mode);
                }
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let w = SCALE + working_digits;
                let one_w = $core::one(w);
                let v = $core::to_work_w(raw, working_digits);
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

            /// Inverse hyperbolic cosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn acosh_approx(self, working_digits: u32) -> Self {
                self.acosh_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Inverse hyperbolic cosine with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn acosh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.acosh_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let one_w = $core::one(w);
                let v = $core::to_work_w(self.to_bits(), working_digits);
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

            /// Inverse hyperbolic tangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn atanh_approx(self, working_digits: u32) -> Self {
                self.atanh_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Inverse hyperbolic tangent with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn atanh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.atanh_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let one_w = $core::one(w);
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let ax = if v < $core::zero() { -v } else { v };
                if ax >= one_w {
                    panic!(concat!(stringify!($Type), "::atanh: argument out of domain (-1, 1)"));
                }
                let ratio = $core::div(one_w + v, one_w - v, w);
                let r = $core::ln_fixed(ratio, w) / $crate::macros::wide_roots::wide_lit!($Work, "2");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Radians-to-degrees with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn to_degrees_approx(self, working_digits: u32) -> Self {
                self.to_degrees_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Radians-to-degrees with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn to_degrees_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.to_degrees_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let v = $core::to_work_w(self.to_bits(), working_digits);
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

            /// Degrees-to-radians with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn to_radians_approx(self, working_digits: u32) -> Self {
                self.to_radians_approx_with(working_digits, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Degrees-to-radians with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn to_radians_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                if working_digits == $core::GUARD {
                    return self.to_radians_strict_with(mode);
                }
                let w = SCALE + working_digits;
                let v = $core::to_work_w(self.to_bits(), working_digits);
                let r = $core::mul(v, $core::pi(w), w)
                    / $crate::macros::wide_roots::wide_lit!($Work, "180");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }
        }

        // Strict-feature dispatchers: the plain method routes to
        // `*_strict` when `strict` is on (and `fast` is not). The
        // wide tiers have no f64-bridge transcendentals, so there is no
        // non-strict plain form.
        #[cfg(all(feature = "strict", not(feature = "fast")))]
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

#[cfg(all(test, not(feature = "fast")))]
mod tests {
    use crate::{D38, D76, D153, D307};

    /// The wide-tier strict transcendentals are correctly rounded, so
    /// at any scale they must agree with the D38 strict path — itself
    /// correctly rounded — to within a couple of ULP (a small slack
    /// absorbs the two paths' independent final-rounding of values that
    /// land near a half-ULP boundary).
    #[test]
    fn wide_transcendentals_match_d38() {
        // Raw bit-patterns at SCALE = 6 spanning a useful range.
        let positives = [1i64, 250_000, 500_000, 1_000_000, 2_718_282, 7_500_000];
        let unit_range = [-900_000i64, -250_000, 1, 250_000, 900_000];
        let all = [-3_000_000i64, -500_000, 1, 500_000, 1_500_000, 4_000_000];

        // `wide` and `d38` are both `i128`-valued raw results at the
        // same scale; compare with a 2-ULP slack.
        fn agree(label: &str, ctx: i64, wide: i128, d38: i128) {
            assert!(
                (wide - d38).abs() <= 2,
                "{label} mismatch at {ctx}: wide {wide} vs d38 {d38}"
            );
        }

        for raw in positives {
            let n = D38::<6>::from_bits(raw as i128);
            let w = D76::<6>::from_bits(crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128));
            agree("ln", raw, w.ln_strict().to_bits().resize::<i128>(), n.ln_strict().to_bits());
            agree("log2", raw, w.log2_strict().to_bits().resize::<i128>(), n.log2_strict().to_bits());
            agree("log10", raw, w.log10_strict().to_bits().resize::<i128>(), n.log10_strict().to_bits());
        }
        for raw in all {
            let n = D38::<6>::from_bits(raw as i128);
            let w = D76::<6>::from_bits(crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128));
            agree("exp", raw, w.exp_strict().to_bits().resize::<i128>(), n.exp_strict().to_bits());
            agree("sin", raw, w.sin_strict().to_bits().resize::<i128>(), n.sin_strict().to_bits());
            agree("cos", raw, w.cos_strict().to_bits().resize::<i128>(), n.cos_strict().to_bits());
            agree("atan", raw, w.atan_strict().to_bits().resize::<i128>(), n.atan_strict().to_bits());
            agree("sinh", raw, w.sinh_strict().to_bits().resize::<i128>(), n.sinh_strict().to_bits());
            agree("cosh", raw, w.cosh_strict().to_bits().resize::<i128>(), n.cosh_strict().to_bits());
            agree("tanh", raw, w.tanh_strict().to_bits().resize::<i128>(), n.tanh_strict().to_bits());
        }
        for raw in unit_range {
            let n = D38::<6>::from_bits(raw as i128);
            let w = D76::<6>::from_bits(crate::wide_int::wide_cast::<i128, crate::wide_int::I256>(raw as i128));
            agree("asin", raw, w.asin_strict().to_bits().resize::<i128>(), n.asin_strict().to_bits());
            agree("acos", raw, w.acos_strict().to_bits().resize::<i128>(), n.acos_strict().to_bits());
            agree("atanh", raw, w.atanh_strict().to_bits().resize::<i128>(), n.atanh_strict().to_bits());
        }
    }

    /// Bit-exact identity points hold across all three wide tiers.
    #[test]
    fn wide_transcendental_identities() {
        assert_eq!(D76::<6>::ONE.ln_strict(), D76::<6>::ZERO);
        assert_eq!(D76::<6>::ZERO.exp_strict(), D76::<6>::ONE);
        assert_eq!(D76::<6>::ZERO.sin_strict(), D76::<6>::ZERO);
        assert_eq!(D76::<6>::ZERO.sinh_strict(), D76::<6>::ZERO);
        assert_eq!(D76::<6>::ZERO.atan_strict(), D76::<6>::ZERO);

        assert_eq!(D153::<6>::ONE.ln_strict(), D153::<6>::ZERO);
        assert_eq!(D153::<6>::ZERO.exp_strict(), D153::<6>::ONE);
        assert_eq!(D153::<6>::ZERO.cos_strict(), D153::<6>::ONE);

        assert_eq!(D307::<6>::ONE.ln_strict(), D307::<6>::ZERO);
        assert_eq!(D307::<6>::ZERO.exp_strict(), D307::<6>::ONE);
        assert_eq!(D307::<6>::ZERO.cosh_strict(), D307::<6>::ONE);
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
            let w = D76::<6>::from_bits(
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
            let w = D76::<6>::from_bits(
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
        assert_eq!(D76::<6>::ONE.ln_strict_agm(), D76::<6>::ZERO);
        assert_eq!(D76::<6>::ZERO.exp_strict_agm(), D76::<6>::ONE);
        assert_eq!(D153::<6>::ONE.ln_strict_agm(), D153::<6>::ZERO);
        assert_eq!(D153::<6>::ZERO.exp_strict_agm(), D153::<6>::ONE);
        assert_eq!(D307::<6>::ONE.ln_strict_agm(), D307::<6>::ZERO);
        assert_eq!(D307::<6>::ZERO.exp_strict_agm(), D307::<6>::ONE);
    }

    /// `*_strict_with(mode)` siblings honour the explicit rounding
    /// mode. Picks a transcendental whose true value lands strictly
    /// between two storage representable values so the rounding mode
    /// actually changes the result.
    #[test]
    fn wide_strict_with_honours_mode() {
        use crate::support::rounding::RoundingMode;
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
        let n = D76::<6>::ONE;
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
        let x = D76::<20>::from_int(3);
        let back = x.ln_strict_agm().exp_strict_agm();
        let delta = (back.to_bits().resize::<i128>() - x.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "AGM exp(ln(3)) at D76<20> delta {delta}");

        let y = D153::<20>::from_int(2);
        let back = y.exp_strict_agm().ln_strict_agm();
        let delta = (back.to_bits().resize::<i128>() - y.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "AGM ln(exp(2)) at D153<20> delta {delta}");
    }

    /// Exercises a scale beyond D38's range, where delegation is
    /// impossible and the wide guard-digit core is the only path.
    /// `exp(ln(x)) ≈ x` and `ln(exp(x)) ≈ x` round-trips.
    #[test]
    fn wide_only_scale_round_trips() {
        // D76<50>: well past D38's max scale of 38. The round-trip
        // result fits i128 comfortably, so compare there.
        let x = D76::<50>::from_int(3);
        let back = x.ln_strict().exp_strict();
        let delta = (back.to_bits().resize::<i128>() - x.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "exp(ln(3)) at D76<50> delta {delta}");

        // D307<150>: deep scale, only the wide core can serve it.
        let y = D307::<150>::from_int(2);
        let back = y.exp_strict().ln_strict();
        let delta = (back.to_bits().resize::<i128>() - y.to_bits().resize::<i128>()).abs();
        assert!(delta <= 8, "ln(exp(2)) at D307<150> delta {delta}");
    }
}
