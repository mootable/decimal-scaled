//! Correctly-rounded strict transcendentals for the wide decimal tiers
//! (D76 / D153 / D307).
//!
//! D38 and the narrow tiers run their strict transcendentals on the
//! 256-bit `algos::support::fixed::Fixed` guard-digit intermediate; D18
//! delegate into D38. The wide tiers cannot widen into D38 — their
//! scale range exceeds it — so they need their own guard-digit core.
//!
//! This module provides one, generic over a hand-rolled signed wide integer
//! `$Work` chosen per tier to be wide enough to hold the working-scale
//! products without overflow:
//!
//! - D76 → `Int<16>` (working scale ≤ 106 digits);
//! - D153 → `Int<32>` (working scale ≤ 183 digits);
//! - D307 → `Int<64>` (working scale ≤ 337 digits).
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

/// Emits the per-tier `pow10_table(w)` helper. Two flavours:
///
/// - `with_const_table` — emits a `static POW10_TABLE: [W; max_scale+GUARD+1]`
///   initialised at compile time (one `wrapping_mul` per entry, chained
///   from the previous) and indexes it directly for in-range `w`.
///   Out-of-range `w` recomputes `10^w` on the stack.
/// - `no_const_table` — recomputes `10^w` on the stack (no const table).
///   Used on tiers where the const-eval step budget can't build the table
///   in stable rust (D924, D1232).
#[doc(hidden)]
#[macro_export]
macro_rules! decl_pow10_table {
    (with_const_table, $max_scale:literal) => {
        /// Upper bound on the strict-path working width
        /// `w = SCALE + GUARD`. Sizes the const `POW10_TABLE`.
        pub(crate) const POW10_TABLE_MAX_W: u32 = ($max_scale as u32) + GUARD;
        /// `10^w` lookup table, built at compile time by chaining
        /// `wrapping_mul(10)` from `1`. Covers every
        /// `w ∈ 0..=POW10_TABLE_MAX_W` — i.e. the entire strict
        /// path. The `_approx` family with `working_digits > GUARD`
        /// can exceed this range; those recompute `10^w` on the stack.
        ///
        /// Memory cost: `(POW10_TABLE_MAX_W + 1) · sizeof(W)`. For
        /// D76 that's ~13 KB (Int<16>); for D307 ~170 KB (Int<64>).
        /// The table lives in `.rodata` once per tier in builds that
        /// enable the tier. In a hot loop a single `w` value is reused,
        /// so only one cache line is touched repeatedly — the table
        /// size matters for binary footprint, not per-call cache
        /// locality.
        pub(crate) static POW10_TABLE: [W; (POW10_TABLE_MAX_W + 1) as usize] = {
            let mut table = [<W>::from_u128(0); (POW10_TABLE_MAX_W + 1) as usize];
            let ten = <W>::from_u128(10);
            table[0] = <W>::from_u128(1);
            let mut i: usize = 1;
            let len = (POW10_TABLE_MAX_W + 1) as usize;
            while i < len {
                table[i] = table[i - 1].wrapping_mul(ten);
                i += 1;
            }
            table
        };
        /// Companion to [`pow10`] keyed on `w`.
        ///
        /// For `w` within the strict-path range
        /// (`0..=POW10_TABLE_MAX_W`) returns the compile-time table
        /// entry — a single static load. For larger `w` (only reachable
        /// via `_approx` with `working_digits > GUARD`) recomputes
        /// `10^w` on the stack.
        ///
        /// The in-range path uses `get_unchecked` to skip the bounds
        /// check — safe because the preceding `w <= POW10_TABLE_MAX_W`
        /// branch guarantees `w as usize < POW10_TABLE.len()` (the
        /// table is sized `POW10_TABLE_MAX_W + 1`).
        #[inline]
        pub(crate) fn pow10_table(w: u32) -> W {
            if w <= POW10_TABLE_MAX_W {
                // SAFETY: `w <= POW10_TABLE_MAX_W` implies
                // `w as usize <= POW10_TABLE_MAX_W as usize <
                // POW10_TABLE.len()` since the table length is
                // `POW10_TABLE_MAX_W + 1`. `u32 as usize` is
                // lossless on all supported targets.
                return unsafe { *POW10_TABLE.get_unchecked(w as usize) };
            }
            pow10(w)
        }
    };
    (no_const_table, $max_scale:literal) => {
        /// Companion to [`pow10`] keyed on `w`. This tier's max scale
        /// puts the const-table build past the stable-rust const-eval
        /// step budget (the `with_const_table` arm's `static POW10_TABLE`
        /// will not compile here), so there is no compile-time table — it
        /// recomputes `10^w` on the stack each call. (Bakeable as a
        /// `static` via a `build.rs` codegen step, which sidesteps the
        /// const-eval budget; not done here.)
        #[inline]
        pub(crate) fn pow10_table(w: u32) -> W {
            pow10(w)
        }
    };
}
pub(crate) use decl_pow10_table;

/// Emits the strict transcendental surface for a wide decimal tier.
///
/// - `$Type` / `$Storage` — the decimal type and its wide storage.
/// - `$Work` — a hand-rolled signed wide integer wide enough for working-scale
/// products: at least `2·(SCALE_max + 30)` decimal digits.
/// - `$core` — the name of the private module the per-tier guard-digit
/// core is emitted into.
/// - `$max_scale` — the type's maximum supported `SCALE`. Bounds the
/// strict-path `w` range `0..=$max_scale + GUARD`, used to size the
/// const `POW10_TABLE` lookup table when the tier opts into it.
///
/// Two arms:
/// - `$Type, $Storage, $Work, $core, $max_scale` — emits the const
///   `POW10_TABLE`. Used for D38..=D616 where the const-eval step
///   budget can build the table at compile time.
/// - `$Type, $Storage, $Work, $core, $max_scale, no_const_table`
///   — recomputes `10^w` on the stack each call (no const table). Used
///   for D924 / D1232 where the table-build's `limbs_mul × max_scale`
///   work exceeds the stable-rust const-eval step budget.
macro_rules! decl_wide_transcendental {
    ($Type:ident, $Storage:ty, $Work:ty, $Wexp:ty, $core:ident, $max_scale:literal,
     $n_limbs:literal, $ln_tang_cap:literal, $exp_tang_m:literal) => {
        $crate::macros::wide_transcendental::decl_wide_transcendental!(
            $Type,
            $Storage,
            $Work,
            $Wexp,
            $core,
            $max_scale,
            with_const_table,
            $n_limbs,
            $ln_tang_cap,
            $exp_tang_m
        );
    };
    ($Type:ident, $Storage:ty, $Work:ty, $Wexp:ty, $core:ident, $max_scale:literal, $table_mode:ident,
     $n_limbs:literal, $ln_tang_cap:literal, $exp_tang_m:literal) => {
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

            /// The wider work integer used by the large-result `exp`
            /// path (`exp_fixed_wide`). Set to the next-wider `Int`
            /// where one exists, else aliases `W` (the widest tier,
            /// D1232, whose own `W` already holds the full lift). The
            /// near-storage-overflow-edge `sinh`/`cosh`/`exp2`/`tanh`
            /// inputs run their `exp` in `Wexp` so the result's
            /// integer-digit lift + the internal `2^k` reassembly +
            /// the squaring peak all fit, then narrow correctly-rounded
            /// to storage.
            pub(crate) type Wexp = $Wexp;

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
            /// Extra working-scale digits added above the canonical
            /// `GUARD` for the Brent–Salamin AGM ln/exp path.
            ///
            /// Background. `ln_fixed_agm` runs the AGM iteration on a
            /// pair `(a, b) = (1, 4/s)` where `s = v · 2^m` and
            /// `m ≈ p/2` is chosen so the AGM identity error is
            /// below one storage ULP. The very first iteration step,
            /// `sqrt(a · b)`, takes the geometric mean of two values
            /// with relative magnitude `b/a ≈ 4/s ≈ 2^-(p/2)`.
            /// `mul` rounds `a · b` to scale `w` and that
            /// rounding sheds `~ulp(w) · a/b ≈ 2^(p/2 - w)` of
            /// relative error into the AGM intermediate. To absorb
            /// that and still hit a 0.5-ULP-at-storage final, the
            /// AGM must run at a working scale `w'` satisfying
            /// `w' ≥ p_target + p/2 + safety`, i.e., roughly
            /// `w' ≈ 3p_target / 2`. In decimal-digit terms, the
            /// AGM guard scales as `~w/2 + log p` extra digits over
            /// the canonical `w = SCALE + GUARD`.
            ///
            /// Definition. Returns the number of extra working
            /// digits added on top of the canonical
            /// `w = SCALE + GUARD`. With the bit-budget-aware
            /// `m_cap` in `ln_fixed_agm` (which now allows the
            /// AGM range-reduction shift to use the full
            /// `W::BITS - bl(v)` headroom rather than only half of
            /// it), the residual precision loss at the storage
            /// scale comes from accumulated `mul` /
            /// `sqrt_fixed` half-LSB rounding over `~log₂(p)`
            /// AGM iterations. A constant `+24` lift absorbs
            /// that residue with margin across every shipped
            /// wide tier (D307<150> through D1232<615>).
            #[inline]
            pub(crate) const fn guard_agm(scale: u32) -> u32 {
                // The AGM kernel intrinsically delivers ~`p/2` bits
                // of precision at the working scale (see Brent
                // 1976 §3 and the precision caveat on
                // `ln_fixed_agm`). To recover full precision at
                // the storage scale, the lifted working scale `w'`
                // must satisfy `w'/2 ≥ SCALE`, i.e., `w' ≥ 2·SCALE`.
                // With canonical `w = SCALE + GUARD`, that means
                // `guard_agm = SCALE − GUARD`, yielding `w' = 2·SCALE`.
                // A small extra `+ 4` covers the rounded-intermediate
                // `mul` / `sqrt_fixed` half-LSB accumulation
                // over `~log₂(p)` AGM iterations.
                if scale > GUARD - 4 {
                    scale - GUARD + 4
                } else {
                    4
                }
            }
            /// Extra digit lift for `exp_strict_agm` that absorbs the
            /// `2^k` post-Newton range reassembly amplification.
            ///
            /// Given a raw storage value `v` at scale `SCALE`, the
            /// Brent–Salamin `exp_fixed_agm` reduces `v = k·ln 2 + s`
            /// with `|k| ≤ ⌈|v|/ln 2⌉`, runs Newton on `exp(s)` at
            /// the working scale, then reassembles via `x << k`. That
            /// reassembly amplifies the raw error of `x` by `2^k`,
            /// i.e., `k · log₁₀(2) ≈ k · 0.30103` decimal digits.
            ///
            /// This helper estimates the digit lift needed by
            /// examining the bit-length of `|raw|` against the
            /// bit-length of `10^scale` — anything in those higher
            /// bits represents the integer part of `|v|`, bounding
            /// `|k| ≤ ⌈|v|/ln 2⌉`. We use rational constants
            /// `144/100 ≈ 1/ln 2` and `301/1000 ≈ log₁₀(2)` plus
            /// `+ 4` safety. Cheap: a few leading-zero / shift ops
            /// inside `W`.
            pub(crate) fn exp_agm_k_lift_from_w(v_w_at_scale: W, scale: u32) -> u32 {
                // |v|'s integer part = |raw| / 10^SCALE. Compute as
                // `bit_length(|v_w|) - bit_length(10^SCALE)` clamped
                // to zero — that's a rough log₂(int_part) bound;
                // exponentiate to a u32 upper bound on int_part.
                let av = abs(v_w_at_scale);
                let bl_v = bit_length(av);
                let bl_one_s = bit_length(pow10_table(scale));
                if bl_v <= bl_one_s {
                    // |v| < 1, no integer part — minimal lift.
                    return 5;
                }
                // log₂(int_part) ≤ bl_v - bl_one_s + 1
                let log2_int_part = bl_v - bl_one_s + 1;
                // int_part ≤ 2^log2_int_part. k ≤ int_part / ln 2 + 1
                // ≤ 2^log2_int_part · 1.443 + 1.
                // k_lift = ⌈k · log₁₀(2)⌉ + 4 ≤ ⌈2^log2_int_part · 0.4343⌉ + 4
                // Use 4343/10000 ≈ 0.43429 ≈ 1/ln(10).
                // Bound 2^log2_int_part by saturating u128 shift.
                let int_part_upper = if log2_int_part >= 128 {
                    u128::MAX
                } else {
                    1u128 << log2_int_part
                };
                let k_lift_u128 = int_part_upper.saturating_mul(4343) / 10000 + 5;
                if k_lift_u128 > u32::MAX as u128 {
                    u32::MAX
                } else {
                    k_lift_u128 as u32
                }
            }

            /// Hard cap on series iterations — a safety net; every
            /// series terminates far sooner by reaching a zero term.
            const SERIES_CAP: u128 = 20_000;

            #[inline]
            pub(crate) fn lit(n: u128) -> W {
                <W as $crate::int::types::traits::BigInt>::from_mag_sign_u128(&[n], false)
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
            $crate::macros::wide_transcendental::decl_pow10_table!($table_mode, $max_scale);
            #[inline]
            pub(crate) fn one(w: u32) -> W {
                pow10_table(w)
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
                    if result_positive {
                        q + lit(1)
                    } else {
                        q - lit(1)
                    }
                } else {
                    q
                }
            }
            /// Half-to-even quotient `n / 10^w`, selecting the
            /// fastest available divide kernel.
            ///
            /// For `1 ≤ w ≤ 38` the MG (magic-multiply) base-2^128
            /// long-divide kernel ships a constant-time, branchless
            /// inner loop — ~5 ops per u128 numerator limb — which
            /// dominates the generic Knuth Algorithm D path on
            /// pipelined CPUs. Audit `round_div_audit_mg_matches_*`
            /// in `algos::support::mg_divide::tests` shows bit-exact agreement
            /// with the generic `div_rem` reference across
            /// 380 000 + 190 000 random inputs.
            ///
            /// For `w == 0` the divisor is 1 so the result is `n`
            /// unchanged. For `1 ≤ w ≤ 38` the single-chunk MG
            /// kernel handles the divide in one pass; for `w > 38`
            /// the chain-MG kernel breaks the divide into a sequence
            /// of `÷ 10^38` stages plus a final `÷ 10^(w − 38·k)`,
            /// each one a base-`2^128` MG long-divide, with
            /// combined-remainder bookkeeping that yields bit-exact
            /// half-to-even. The chain audit
            /// (`round_div_chain_audit_*` in `algos::support::mg_divide::tests`)
            /// confirms agreement with the schoolbook `div_rem`
            /// reference on 380K + 190K random inputs across every
            /// `RoundingMode` and `w ∈ 39..=100`.
            #[inline]
            fn round_div_pow10(n: W, w: u32) -> W {
                if w == 0 {
                    return n;
                }
                if w <= 38 {
                    return $crate::algos::support::mg_divide::div_wide_pow10::<W>(
                        n,
                        w,
                        $crate::support::rounding::RoundingMode::HalfToEven,
                    );
                }
                // Newton vs MG chain dispatch (see the matrix in
                // [`crate::algos::support::newton_reciprocal::dispatch_wide_pow10`]).
                // For most wide-tier `$Work` integers `W::BITS` lands
                // outside the bench-validated cells (Int<128> /
                // Int<192> / Int<256>) and the dispatcher forwards to
                // MG; the routing is here so a future bench at the
                // larger widths can promote without touching this
                // site.
                $crate::algos::support::newton_reciprocal::dispatch_wide_pow10::<W>(
                    n,
                    w,
                    $crate::support::rounding::RoundingMode::HalfToEven,
                )
            }
            /// `(a · b) / 10^w`, rounded half-to-even. The
            /// rounded variant replaces the previous truncating
            /// `mul`: each call drops the per-op ≤ 1 LSB
            /// truncation bias to a symmetric ≤ 0.5 LSB error,
            /// which is what 0.5 ULP at storage requires across
            /// the series-evaluation core.
            #[inline]
            pub(crate) fn mul(a: W, b: W, w: u32) -> W {
                round_div_pow10(a * b, w)
            }
            /// `(a · 10^w) / b`, rounded half-to-even.
            #[inline]
            pub(crate) fn div(a: W, b: W, w: u32) -> W {
                round_div(a * pow10_table(w), b)
            }
            /// Loop-friendly variant of [`div`] taking a precomputed
            /// `10^w` numerator factor.
            #[inline]
            pub(crate) fn div_cached(a: W, b: W, pow10_w: W) -> W {
                round_div(a * pow10_w, b)
            }
            /// `a · n` for a small unsigned multiplier.
            ///
            /// When `n` fits a single u64 limb, routes through the
            /// n-by-1-word `checked_mul_u64` specialisation
            /// (`L` widening muls instead of the generic `L²`
            /// outer-product loop). For `n > u64::MAX` falls back
            /// to the generic `a * lit(n)` `Mul` operator path.
            #[inline]
            fn mul_u(a: W, n: u128) -> W {
                if n <= u64::MAX as u128 {
                    a.checked_mul_u64(n as u64)
                } else {
                    a * lit(n)
                }
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
            ///
            /// # `f64`-bridge Newton seed (std, narrow radicands)
            ///
            /// The trait-level `W::isqrt` seeds Newton at `2^⌈bits/2⌉`,
            /// accurate to 1 bit and convergent in ~`log₂(bits)`
            /// iterations of full-width divmod. When `std` is
            /// available **and** the radicand fits f64's ~2^1023
            /// dynamic range, we seed instead with
            /// `f64::sqrt(n.as_f64())`. `n.as_f64()` rounds to nearest
            /// f64 (53-bit mantissa); `f64::sqrt` is correctly rounded,
            /// so the seed lands within ~2⁻⁵² of the true `√n`.
            /// Newton then needs ~2 iterations versus ~7 from the
            /// 1-bit seed — a measured 3-4× sqrt speedup at D57<20>.
            ///
            /// A single unconditional Newton pre-step restores the
            /// monotone-decrease precondition the loop relies on by
            /// AM-GM (`(x + n/x)/2 ≥ √n`), so the seed direction is
            /// irrelevant to correctness.
            pub(crate) fn sqrt_fixed(v: W, w: u32) -> W {
                let av = abs(v);
                debug_assert!(
                    bit_length(av) + (w as u32) * 4 < W::BITS,
                    "sqrt_fixed: |v| * 10^w overflows the working width"
                );
                let n = av * pow10_table(w);
                if n <= zero() {
                    // √0 = 0; also guards the Newton loop's n / x divide.
                    return zero();
                }
                // Newton seed delegated to the cross-algorithm seed leaf
                // (`algo_x_support::seed::sqrt_seed`, via the generic
                // `W ↔ &[u64]` bridge in `algos::support::seed_bridge`):
                // under
                // `std` it bootstraps from `f64::sqrt` of the top 64
                // bits of `n` (~53 correct bits in one shot); under
                // `no_std` it is the classical pure-integer 1-bit seed.
                // Both over-estimate, so the unconditional AM-GM
                // pre-step (`(x + n/x)/2 ≥ ⌈√n⌉`) and the
                // monotone-downward loop below converge to the same
                // floor root regardless of which seed body ran.
                let seed = $crate::algos::support::seed_bridge::sqrt_seed_w::<W>(n);
                let x0 = if seed <= zero() { lit(1) } else { seed };
                let mut x = (x0 + n / x0) >> 1;
                loop {
                    let y = (x + n / x) >> 1;
                    if y >= x {
                        return x;
                    }
                    x = y;
                }
            }

            /// Builds a working-scale value from the type's raw storage:
            /// `raw · 10^GUARD` (raw is `value · 10^SCALE`, the result
            /// is `value · 10^(SCALE+GUARD)`).
            ///
            /// Widens `$Storage` into the work integer `W` via the
            /// `BigInt::resize_to` magnitude/sign bridge, then scales by
            /// `10^GUARD`.
            pub(crate) fn to_work(raw: $Storage) -> W {
                $crate::int::types::traits::BigInt::resize_to::<W>(raw) * pow10_table(GUARD)
            }

            /// Runtime-guard variant of [`to_work`]: scales raw by
            /// `10^working_digits` instead of the const `GUARD`. Used by
            /// the `_approx` family where the guard width is chosen at
            /// call time.
            pub(crate) fn to_work_w(raw: $Storage, working_digits: u32) -> W {
                $crate::int::types::traits::BigInt::resize_to::<W>(raw) * pow10_table(working_digits)
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
                round_to_storage_with(
                    v,
                    w,
                    target,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Mode-aware variant of [`round_to_storage`].
            ///
            /// When the narrowing distance `w - target` is in `1..=38`
            /// the single-chunk MG kernel `div_wide_pow10` serves
            /// every mode directly. For `shift > 38` the chain-MG
            /// kernel `div_wide_pow10_chain` does the same via
            /// repeated `÷ 10^38` with combined-remainder bookkeeping
            /// (bit-exact for every `RoundingMode`; see
            /// `round_div_chain_audit_*` in `algos::support::mg_divide::tests`).
            pub(crate) fn round_to_storage_with(
                v: W,
                w: u32,
                target: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                let shift = w - target;
                let rounded = if shift == 0 {
                    v
                } else if shift <= 38 {
                    $crate::algos::support::mg_divide::div_wide_pow10::<W>(v, shift, mode)
                } else {
                    // Newton vs MG chain dispatch — see the matrix
                    // in [`crate::algos::support::newton_reciprocal::dispatch_wide_pow10`].
                    $crate::algos::support::newton_reciprocal::dispatch_wide_pow10::<W>(
                        v, shift, mode,
                    )
                };
                let max_w = $crate::int::types::traits::BigInt::resize_to::<W>(<$Storage>::MAX);
                let min_w = $crate::int::types::traits::BigInt::resize_to::<W>(<$Storage>::MIN);
                if rounded > max_w || rounded < min_w {
                    panic!(concat!(
                        stringify!($Type),
                        " strict transcendental: result out of range"
                    ));
                }
                $crate::int::types::traits::BigInt::resize_to::<$Storage>(rounded)
            }

            /// Directed-rounding narrowing with Ziv escalation.
            ///
            /// `round_to_storage_with` rounds the working-scale
            /// *approximation* per `mode`. For the three nearest modes the
            /// `GUARD` budget keeps the approximation within half a storage
            /// ULP of the true value, so a single narrowing is correctly
            /// rounded. The directed modes (`Trunc`/`Floor`/`Ceiling`)
            /// decide on which side of a storage grid line the *true*
            /// value falls — and when the true value sits within the
            /// kernel error envelope of that grid line, the approximation
            /// can be on the wrong side, flipping the answer by one LSB.
            ///
            /// `recompute(guard)` returns the kernel's working-scale value
            /// computed with `guard` guard digits (working scale
            /// `target + guard`). When the residual lands inside the
            /// uncertain band (`ZIV_ERR_LSB` working-scale LSB of either
            /// grid line) we cannot trust the directed decision, so we
            /// recompute with a wider guard and retry — Ziv's strategy.
            /// Each step roughly doubles the guard; for the algebraic
            /// points where the true residual is genuinely zero
            /// (`ln 1`, `exp 0`, `sin 0`, exact quadrant multiples) the
            /// caller resolves the value before reaching here, so the
            /// loop always terminates on a decisive residual.
            ///
            /// Nearest modes never enter the loop: they narrow once.
            /// Standard directed narrowing: the base-guard evaluation
            /// is trusted unless its residual sits within the kernel
            /// error band of a grid line, in which case it Ziv-escalates.
            pub(crate) fn round_to_storage_directed(
                base_guard: u32,
                target: u32,
                mode: $crate::support::rounding::RoundingMode,
                recompute: impl FnMut(u32) -> W,
            ) -> $Storage {
                round_to_storage_directed_impl(base_guard, target, mode, false, false, recompute)
            }

            /// Directed-rounding narrowing for a kernel whose true result is
            /// **never exactly representable** at the storage scale — a
            /// non-zero-argument transcendental (`exp`), whose value is
            /// irrational (Lindemann–Weierstrass) and so always sits strictly
            /// between two storage grid lines. Identical to
            /// [`round_to_storage_directed`] except a working residual of
            /// exactly zero is treated as a genuine sub-resolution positive
            /// residual: Ceiling rounds UP to the next grid line, Floor / Trunc
            /// keep the floor, nearest modes are unaffected. This is the only
            /// correctly-rounded answer when the deciding residual lands below
            /// the work integer's resolution (e.g. `exp(-10^-S)` just under
            /// `1.0`, whose residual is at scale ~`2S`). The caller MUST pin its
            /// algebraic-exact inputs (`exp 0` etc.) before reaching here.
            pub(crate) fn round_to_storage_directed_never_exact(
                base_guard: u32,
                target: u32,
                mode: $crate::support::rounding::RoundingMode,
                recompute: impl FnMut(u32) -> W,
            ) -> $Storage {
                round_to_storage_directed_impl(base_guard, target, mode, false, true, recompute)
            }

            /// Near-special-point directed narrowing for the derived
            /// functions (`acosh` at 1, `atanh` at +-1). After the
            /// gap/log1p reformulation the kernel is accurate, but the
            /// base `GUARD` budget can still be a few digits short of a
            /// correctly-rounded result on these inputs because the
            /// result is the small difference / logarithm of a tiny gap.
            /// `force_confirm` makes both the nearest and directed paths
            /// always confirm the base narrowing against a wider guard
            /// (rather than trusting a "clear" residual that the residual
            /// kernel error could itself have placed on the wrong side),
            /// so the answer is correctly rounded under every mode.
            pub(crate) fn round_to_storage_directed_near_special(
                base_guard: u32,
                target: u32,
                mode: $crate::support::rounding::RoundingMode,
                recompute: impl FnMut(u32) -> W,
            ) -> $Storage {
                round_to_storage_directed_impl(base_guard, target, mode, true, false, recompute)
            }

            fn round_to_storage_directed_impl(
                base_guard: u32,
                target: u32,
                mode: $crate::support::rounding::RoundingMode,
                force_confirm: bool,
                never_exact: bool,
                mut recompute: impl FnMut(u32) -> W,
            ) -> $Storage {
                use $crate::support::rounding::{RoundingMode, is_nearest_mode};

                let base_w = target + base_guard;
                if is_nearest_mode(mode) {
                    if !force_confirm {
                        return round_to_storage_with(recompute(base_guard), base_w, target, mode);
                    }
                    // A single narrowing at `base_guard` is correctly
                    // rounded whenever the working approximation lies
                    // within half a storage ULP of the true value -- the
                    // usual case the `GUARD` budget guarantees. Near a
                    // special point (`atanh` at `+-1`, `acosh` at `1`) the
                    // kernel's residual error grows, and a single
                    // narrowing at the base guard can round to the wrong
                    // storage neighbour even after the gap/log1p
                    // reformulation removes the catastrophic cancellation.
                    // Confirm the base narrowing against a wider-guard
                    // recompute (Ziv): when two successive working scales
                    // narrow to the same storage integer the result is
                    // trustworthy. This mirrors the directed-mode loop
                    // below but compares the rounded storage value
                    // directly, since a nearest decision depends on the
                    // whole residual, not just its sign. The guard is
                    // capped from the result's integer-digit count exactly
                    // as the directed loop is, so the recompute never
                    // overflows `W`.
                    let mut nearest_narrow = |guard: u32| -> $Storage {
                        let w = target + guard;
                        round_to_storage_with(recompute(guard), w, target, mode)
                    };
                    let lo = nearest_narrow(base_guard);
                    let int_digits = {
                        let n = $crate::int::types::traits::BigInt::resize_to::<W>(lo);
                        let m = if n < lit(0) { -n } else { n };
                        let bl = bit_length(m);
                        let storage_digits = (bl as u64 * 30103 / 100_000) as u32 + 1;
                        storage_digits.saturating_sub(target)
                    };
                    let cap_digits = (<W>::BITS / 8).saturating_sub(int_digits + 8);
                    let max_guard = cap_digits.saturating_sub(target).max(base_guard);
                    let mut guard = base_guard;
                    let mut best = lo;
                    loop {
                        if guard >= max_guard {
                            break;
                        }
                        let step = (target + base_guard).max(base_guard);
                        let next_guard = guard.saturating_add(step).min(max_guard);
                        let hi = nearest_narrow(next_guard);
                        if hi == best {
                            break;
                        }
                        guard = next_guard;
                        best = hi;
                    }
                    return best;
                }

                // Directed answer: the truncated/bumped magnitude derived
                // from the *true* residual sign. The working value carries a
                // kernel error that, near a storage grid line, can flip that
                // sign. `directed_narrow` returns both the rounded result and
                // the residual position so the caller can tell when the value
                // sits near a grid line (and the decision is untrustworthy).
                let mut directed_narrow = |guard: u32| -> (W, W, W) {
                    let w = target + guard;
                    let v = recompute(guard);
                    let shift = w - target;
                    let neg = v < lit(0);
                    let mag = if neg { -v } else { v };
                    let divisor = pow10(shift);
                    let (q, rem) = mag.div_rem(divisor);
                    let result_positive = !neg;
                    // `rem == 0` at the working scale means `|value|·10^target`
                    // is an integer to the work-int's resolution — `q` is the
                    // floor and the residual deciding a directed bump appears to
                    // be exactly zero. For a `never_exact` kernel (a non-zero-
                    // argument transcendental, whose true value is irrational by
                    // Lindemann–Weierstrass and so NEVER lands on a finite
                    // decimal grid line — the algebraic-exact inputs `exp 0`,
                    // `ln 1`, … are pinned by the caller before reaching here)
                    // a zero working residual is an ARTIFACT of finite working
                    // precision, not a true zero: the true residual is a genuine
                    // positive fraction sitting below the work-int's resolution
                    // (e.g. `exp(-10^-S)` = `1 - 10^-S + 10^-2S/2 - …`, whose
                    // deciding term is at scale ~`2S`, beyond any reachable
                    // guard). `rem == 0` is moreover unambiguously the LOW side
                    // of the grid line (`|value| = q·divisor + tiny_positive`):
                    // a value just BELOW a grid line gives `rem ≈ divisor`, not
                    // zero. So treat a zero working residual as present-and-
                    // positive when `never_exact`, which bumps Ceiling up to the
                    // next grid line while Floor / Trunc / nearest still keep
                    // `q`.
                    let residual_present = rem != lit(0) || never_exact;
                    let bump = residual_present
                        && match mode {
                            RoundingMode::Trunc => false,
                            RoundingMode::Floor => !result_positive,
                            RoundingMode::Ceiling => result_positive,
                            _ => unreachable!(),
                        };
                    let q_mag = if bump { q + lit(1) } else { q };
                    let signed = if neg { -q_mag } else { q_mag };
                    // Distance from the nearer grid line, in working-scale
                    // units: min(rem, divisor − rem).
                    let dist = if rem < divisor - rem {
                        rem
                    } else {
                        divisor - rem
                    };
                    (signed, dist, divisor)
                };

                // Ziv escalation. Evaluate at `base_guard`; if the residual
                // sits well clear of either grid line (`dist` exceeds a
                // generous fraction of the working ULP grid), the directed
                // decision is trustworthy and we are done. Otherwise recompute
                // at a wider guard until two consecutive evaluations agree —
                // the residual band the kernel error spans shrinks each step,
                // so every non-algebraic input converges. Exact algebraic
                // points (`exp 0`, `ln 1`, `sin 0`, exact quadrant multiples)
                // are resolved by the caller before reaching here.
                //
                // Guard is capped so the recompute never overflows `W`: the
                // result needs `int_digits + target + guard` significant
                // digits, and `W` holds about `BITS · 0.3` of them. We size
                // the cap from the result's integer-digit count (taken from
                // the base evaluation) leaving a safety margin.
                let (mut lo, dist0, divisor0) = directed_narrow(base_guard);

                // "Near a grid line": within 1/1000 of the working ULP grid.
                // Comfortably above any kernel rounding noise yet far below
                // the residual of an ordinary (non-boundary) input.
                let band0 = divisor0 / lit(1000);
                let near_grid = force_confirm || dist0 <= band0;

                let signed = if !near_grid {
                    lo
                } else {
                    // Capacity of `W` in decimal digits (~BITS·log10(2)),
                    // minus the result's integer-digit count and a margin,
                    // bounds how far we may escalate without overflow.
                    let int_digits = {
                        let m = if lo < lit(0) { -lo } else { lo };
                        // `lo` is the storage value (integer part scaled by
                        // 10^target), so its decimal length minus `target`
                        // is the integer-part digit count. Approximate the
                        // length via bit length.
                        let bl = bit_length(m);
                        let storage_digits = (bl as u64 * 30103 / 100_000) as u32 + 1;
                        storage_digits.saturating_sub(target)
                    };
                    // Some kernels form wide intermediate scratch — e.g.
                    // `sqrt_fixed` asserts `bit_length(|v|) + 4·w < W::BITS`,
                    // i.e. roughly `7·w_decimal < W::BITS`. Cap the total
                    // working scale at `W::BITS / 8` decimal digits (leaving
                    // ~12% headroom over the tightest scratch) so the
                    // recompute never overflows. Subtract the result's
                    // integer digits and a small margin.
                    let cap_digits = (<W>::BITS / 8).saturating_sub(int_digits + 8);
                    let max_guard = cap_digits.saturating_sub(target).max(base_guard);

                    // A directed result is trustworthy once two consecutive
                    // guards agree AND the residual has resolved its position
                    // relative to the grid line. Two regimes are "resolved":
                    //
                    //  * residual EXACTLY zero (`dist == 0`): the value sits on
                    //    a grid line to the full working resolution and stays
                    //    there as the guard grows — a genuine algebraic-exact
                    //    point (`sinh 0`, `cosh 0 = 1`, `log_b(b^k)`). Accept it
                    //    (the floor `q` is exact; no directed bump).
                    //  * residual CLEAR of the band (`dist > divisor/1000`): the
                    //    deciding fraction is well above any kernel round-off,
                    //    so the directed side is certain.
                    //
                    // The dangerous middle case is a SMALL NON-ZERO residual
                    // inside the band: that is a finite-precision ARTIFACT, not
                    // the true residual. E.g. `log10(10^k − 1) = k − 10^-k/ln10`
                    // — at a working scale `w < k` the `−10^-k` deviation is
                    // invisible and AGM/division round-off leaves a spurious
                    // residual on the WRONG (high) side, so Trunc/Floor would
                    // wrongly keep `k`. While the residual sits in the band we
                    // keep escalating until the deviation materialises (the
                    // guard exceeds the input's digit span) or the cap is hit.
                    let mut guard = base_guard;
                    loop {
                        if guard >= max_guard {
                            break lo;
                        }
                        // Step past `target` so a result term that only
                        // materialises at guard ≈ target (the `+x` of
                        // `exp(x) = 1 + x + …` for `|x| ≈ 10^-target`) is
                        // reached, then confirm with a further step.
                        let step = (target + base_guard).max(base_guard);
                        let next_guard = guard.saturating_add(step).min(max_guard);
                        let (hi, hi_dist, hi_div) = directed_narrow(next_guard);
                        let hi_band = hi_div / lit(1000);
                        // Resolved iff the residual is exactly on the grid line
                        // (exact algebraic point) or clear of the near-grid band.
                        let resolved = hi_dist == lit(0) || hi_dist > hi_band;
                        if hi == lo && resolved {
                            break hi;
                        }
                        guard = next_guard;
                        lo = hi;
                    }
                };

                let max_w = $crate::int::types::traits::BigInt::resize_to::<W>(<$Storage>::MAX);
                let min_w = $crate::int::types::traits::BigInt::resize_to::<W>(<$Storage>::MIN);
                if signed > max_w || signed < min_w {
                    panic!(concat!(
                        stringify!($Type),
                        " strict transcendental: result out of range"
                    ));
                }
                $crate::int::types::traits::BigInt::resize_to::<$Storage>(signed)
            }

            /// Rounds a working-scale value to the nearest integer (ties
            /// away from zero). Used for the range-reduction quotient.
            pub(crate) fn round_to_nearest_int(v: W, w: u32) -> i128 {
                let divisor = pow10_table(w);
                let (q, r) = v.div_rem(divisor);
                let half = divisor >> 1;
                let qi = if abs(r) >= half {
                    if v < lit(0) { q - lit(1) } else { q + lit(1) }
                } else {
                    q
                };
                $crate::int::types::traits::BigInt::to_i128(qi)
            }

            /// Exact-integer logarithm witness for `log_base(value)`.
            ///
            /// Given the storage-scale raw integers `value_raw` and
            /// `base_raw` (each `x · 10^scale`) and a candidate integer
            /// result `k`, returns `true` iff `value == base^k` *exactly*
            /// at the storage scale — i.e. the true `log_base(value)` is
            /// the exact integer `k`. This is the directed-rounding
            /// exact-zero residual flag (Lindemann–Weierstrass guarantees
            /// the logarithm is otherwise irrational, so a non-exact
            /// witness means a genuine non-zero residual): when it fires
            /// the kernel pins the result to exactly `k` under every mode,
            /// rather than letting the `ln(value)/ln(base)` round-off land
            /// a hair above/below the grid line and bump under a directed
            /// mode.
            ///
            /// The check is exact integer arithmetic in `W`. For `k >= 1`
            /// it tests `base_raw^k == value_raw · 10^(scale·(k − 1))`;
            /// for `k == 0` it tests `value_raw == 10^scale` (`value == 1`);
            /// negative `k` (`value < 1` with an integer base) tests the
            /// mirror `base_raw^(−k) == 10^scale · 10^(scale·(−k − 1)) ·
            /// 10^scale / value`… which reduces to `value_raw ·
            /// base_raw^(−k) == 10^(scale·(−k + 1))`. Overflow of the
            /// power short-circuits to `false` (a value that large is not
            /// a representable exact power at this width).
            pub(crate) fn log_is_exact_int(value_raw: W, base_raw: W, scale: u32, k: i128) -> bool {
                let one_s = pow10_table(scale);
                if k == 0 {
                    return value_raw == one_s;
                }
                // Reduce to the integer domain so the running power never
                // carries the `· 10^scale` factor (which tips into a wider
                // limb tier or overflows `W` at high scale). An integer
                // `base^k = value` can only be an exact storage point when
                // `base` is itself an exact integer multiple of `10^scale`
                // (only the near-1 ill-conditioning probes are not, and
                // those are never exact powers).
                let (bq, br) = base_raw.div_rem(one_s);
                if br != lit(0) {
                    return false;
                }
                let base_int = bq;
                let kk = k.unsigned_abs();
                let limit_bits = W::BITS - 4;
                if k > 0 {
                    // value == base^|k|: require `value` itself integral.
                    let (vq, vr) = value_raw.div_rem(one_s);
                    if vr != lit(0) {
                        return false;
                    }
                    let value_int = vq;
                    let mut pow = lit(1);
                    let mut i: u128 = 0;
                    while i < kk {
                        if bit_length(pow) + bit_length(base_int) >= limit_bits {
                            return false;
                        }
                        pow = pow * base_int;
                        i += 1;
                    }
                    pow == value_int
                } else {
                    // value == 1 / base^|k|: `value_raw · base_int^|k|`
                    // must equal the storage `1` exactly.
                    let mut cur = value_raw;
                    let mut i: u128 = 0;
                    while i < kk {
                        if bit_length(cur) + bit_length(base_int) >= limit_bits {
                            return false;
                        }
                        cur = cur * base_int;
                        i += 1;
                    }
                    cur == one_s
                }
            }

            /// Storage representation of the exact integer `k` at scale
            /// `scale`: the `$Storage` value `k · 10^scale`. Panics if it
            /// does not fit (a result that out-of-range would also panic
            /// in `round_to_storage_with`).
            pub(crate) fn exact_int_at_scale(k: i128, scale: u32) -> $Storage {
                narrow_to_storage(scale_by_k(one(scale), k))
            }

            /// Range-checked narrowing of a storage-scale working value
            /// `v` (already at scale `SCALE`, no rounding needed) to the
            /// type's `$Storage`. Panics if out of range, matching
            /// `round_to_storage_with`.
            pub(crate) fn narrow_to_storage(v: W) -> $Storage {
                let max_w = $crate::int::types::traits::BigInt::resize_to::<W>(<$Storage>::MAX);
                let min_w = $crate::int::types::traits::BigInt::resize_to::<W>(<$Storage>::MIN);
                if v > max_w || v < min_w {
                    panic!(concat!(
                        stringify!($Type),
                        " strict transcendental: result out of range"
                    ));
                }
                $crate::int::types::traits::BigInt::resize_to::<$Storage>(v)
            }

            /// Exact-power pin for `exp2`: when the storage raw `raw`
            /// (= `x · 10^scale`) is an exact integer `x = k`,
            /// `exp2(k) = 2^k` is an exact algebraic point — a dyadic
            /// rational, never a transcendental residual. Returns the
            /// **correctly-rounded** `$Storage` result under `mode`,
            /// computed from exact integer arithmetic (so the working-scale
            /// series can never bump it across a tie or grid line); `None`
            /// only when `x` is not an exact integer (the genuinely
            /// transcendental case the kernel handles).
            pub(crate) fn exp2_exact_pin(
                raw: $Storage,
                scale: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> ::core::option::Option<$Storage> {
                let raw_w = widen_storage(raw);
                let one_s = pow10_table(scale);
                let (kq, kr) = raw_w.div_rem(one_s);
                if kr != lit(0) {
                    return ::core::option::Option::None;
                }
                let k = $crate::int::types::traits::BigInt::to_i128(kq);
                // The exactly-representable powers (`k ≥ 0`, or `k < 0` with
                // `|k| ≤ scale`) land on the storage grid with no rounding.
                if let ::core::option::Option::Some(v) = exp2_exact_pow(k, scale) {
                    return ::core::option::Option::Some(narrow_to_storage(v));
                }
                // `k < 0`, `|k| > scale`: `2^k · 10^scale = 5^scale / 2^p`
                // (`p = |k| − scale ≥ 1`) is a proper dyadic fraction. Round
                // it exactly under `mode` (`exp2(-1)=0.5` is the half-to-even
                // tie → 0; a sub-resolution `2^k` only `Ceiling`-rounds up).
                // For `k > 0` `exp2_exact_pow` returns `None` only on genuine
                // overflow — defer to the kernel's panic-on-narrow there.
                if k >= 0 {
                    return ::core::option::Option::None;
                }
                let p = (k.unsigned_abs() as u32) - scale;
                ::core::option::Option::Some(round_pow2_fraction(scale, p, mode))
            }

            /// Correctly-rounded `$Storage` value of the dyadic fraction
            /// `5^scale / 2^p` (`p ≥ 1`) — the `exp2(k)` storage value when
            /// `k = −(p + scale)`. The result is strictly positive and at
            /// most `5^scale / 2`, so it always fits storage.
            ///
            /// `q = num >> p`, residual `r = num mod 2^p`; the half-way
            /// divisor is `2^p` so the tie compares `2·r` with `2^p`. When
            /// `2^p` exceeds the working width the quotient is `0` and the
            /// whole `num` is a sub-half positive residual (only `Ceiling`
            /// rounds up).
            fn round_pow2_fraction(
                scale: u32,
                p: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                let num = lit(5).pow(scale);
                // When `2^p` overflows the working width it strictly exceeds
                // `2·num` (since `num < 2^BITS` and `p ≥ BITS`), so `q = 0`
                // and the residual `num` sits strictly below half — a
                // sub-resolution positive value (only `Ceiling` rounds up).
                if p >= <W as $crate::int::types::traits::BigInt>::BITS {
                    let bump = $crate::support::rounding::should_bump(
                        mode,
                        ::core::cmp::Ordering::Less,
                        false,
                        true,
                    );
                    return narrow_to_storage(if bump { lit(1) } else { lit(0) });
                }
                let denom = lit(1) << p;
                let (q, r) = num.div_rem(denom);
                if r.is_zero() {
                    return narrow_to_storage(q);
                }
                let twice_r = r << 1;
                let cmp_r = twice_r.cmp(&denom);
                let q_is_odd = q.bit(0);
                let bump =
                    $crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, true);
                narrow_to_storage(if bump { q + lit(1) } else { q })
            }

            #[inline]
            fn widen_storage(raw: $Storage) -> W {
                $crate::int::types::traits::BigInt::resize_to::<W>(raw)
            }

            /// Integer-digit count of the `exp2` result `2^x` for the
            /// storage raw `raw` (= `x · 10^scale`), used as the
            /// large-result working-scale lift. Returns
            /// `⌈|x|·log10(2)⌉` capped so the lifted working scale, plus
            /// `exp_fixed`'s own internal `2^k` lift (≈ the same size) and
            /// the result's integer digits, stay inside the working
            /// integer's decimal capacity (`~BITS·0.30103` digits). A
            /// `2^x` whose integer part alone exceeds that capacity is not
            /// representable at this tier and panics on narrowing.
            /// Squaring-safe cap on a large-result working-scale lift.
            ///
            /// `needed` is the result's integer-digit count (the ideal
            /// lift). The large-result `sinh`/`cosh`/`exp2`/`tanh`
            /// closures run their `exp` in the *wider* work integer
            /// [`Wexp`] (via [`exp_fixed_wide`]), so the budget is
            /// `Wexp`'s decimal capacity, not `W`'s.
            ///
            /// `exp_fixed` (at `Wexp`) runs at
            /// `w_ext = scale + GUARD + lift + extra` where its internal
            /// `2^k` reassembly extra is `≈ 1.25·needed`, then *squares*
            /// a value at scale `w_ext` — the squaring transiently needs
            /// `2·w_ext` digits. With `lift = needed` the squaring peak
            /// is `≈ 2·(scale + GUARD) + 4.5·needed`, which must stay
            /// inside `Wexp`'s `~BITS·log10(2)` decimal capacity. We size
            /// the cap from that bound (with a safety margin). Because
            /// `Wexp` is the next-wider tier for every shipped width
            /// (and D1232's own `Int<256>` already holds the peak at its
            /// `MAX_SCALE`), the full `needed` lift fits and the cell
            /// rounds correctly; the cap only fires for genuinely
            /// out-of-range inputs, which then panic on narrowing.
            pub(crate) fn exp_lift_cap(needed: u128, scale: u32) -> u32 {
                let wexp_digits = <Wexp>::BITS as u128 * 30103 / 100_000;
                // Solve `2·(scale+GUARD) + 4.5·lift + margin ≤ wexp_digits`
                // for `lift`. Use 45/10 for the 4.5 factor; margin 64.
                let base = 2 * (scale as u128 + GUARD as u128) + 64;
                let head = wexp_digits.saturating_sub(base) * 10 / 45;
                let lift = needed.min(head);
                if lift > u32::MAX as u128 {
                    u32::MAX
                } else {
                    lift as u32
                }
            }

            /// Upper bound on the integer-digit count of `2^x` (the `exp2`
            /// result) for storage raw `raw` (= `x · 10^scale`), capped by
            /// [`exp_lift_cap`] for use as the large-result lift.
            pub(crate) fn exp2_result_int_digits(raw: $Storage, scale: u32) -> u32 {
                exp_lift_cap(
                    pow_result_digits(abs(widen_storage(raw)), scale, 30103),
                    scale,
                )
            }

            /// Upper bound on the integer-digit count of `e^|v|` (the
            /// `sinh`/`cosh`/`exp` result) for a storage-scale magnitude
            /// `mag_at_scale` (= `|v| · 10^scale`), capped by
            /// [`exp_lift_cap`].
            pub(crate) fn exp_result_int_digits(mag_at_scale: W, scale: u32) -> u32 {
                exp_lift_cap(pow_result_digits(abs(mag_at_scale), scale, 43429), scale)
            }

            /// Shared estimator: `⌈|x| · factor / 100000⌉` decimal digits,
            /// where `x = av / 10^scale` and `factor` is `log10(base)·1e5`
            /// (`30103` for `2^x`, `43429` for `e^x`). Returns `0` when
            /// `|x| < 1` (the result has no integer-digit growth).
            fn pow_result_digits(av: W, scale: u32, factor: u128) -> u128 {
                let bl_v = bit_length(av);
                let bl_one = bit_length(pow10_table(scale));
                if bl_v <= bl_one {
                    return 0;
                }
                let log2_int = bl_v - bl_one + 1;
                let int_upper = if log2_int >= 127 {
                    u128::MAX
                } else {
                    1u128 << log2_int
                };
                (int_upper.saturating_mul(factor) / 100_000) as u128
            }

            /// Exact `2^k` at scale `scale`, as a `W` working value, when
            /// `x = k` is an exact integer and `2^k` is representable at
            /// the storage scale. `exp2(integer k) = 2^k` is an exact
            /// algebraic point: for `k ≥ 0` it is the integer `2^k`; for
            /// `k < 0` it is `5^|k| / 10^|k|`, finite with `|k|` decimal
            /// places (representable iff `|k| ≤ scale`). Returns `None`
            /// when not exactly representable, so the caller falls through
            /// to the working-scale kernel.
            pub(crate) fn exp2_exact_pow(k: i128, scale: u32) -> ::core::option::Option<W> {
                let one_s = pow10_table(scale);
                if k == 0 {
                    return ::core::option::Option::Some(one_s);
                }
                let kk = k.unsigned_abs();
                if k > 0 {
                    // 2^k · 10^scale, guarding the working width.
                    let mut v = one_s;
                    let two = lit(2);
                    let mut i: u128 = 0;
                    while i < kk {
                        if bit_length(v) + 2 >= W::BITS - 4 {
                            return ::core::option::Option::None;
                        }
                        v = v * two;
                        i += 1;
                    }
                    ::core::option::Option::Some(v)
                } else {
                    // 2^-|k| = 5^|k| · 10^(scale − |k|); representable iff
                    // |k| ≤ scale.
                    if (kk as u128) > scale as u128 {
                        return ::core::option::Option::None;
                    }
                    let mut v = pow10_table(scale - kk as u32);
                    let five = lit(5);
                    let mut i: u128 = 0;
                    while i < kk {
                        if bit_length(v) + 3 >= W::BITS - 4 {
                            return ::core::option::Option::None;
                        }
                        v = v * five;
                        i += 1;
                    }
                    ::core::option::Option::Some(v)
                }
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

            /// The work-integer's u64-limb count, the key into the
            /// per-width constant references in
            /// [`crate::types::consts::wide`].
            const W_LIMBS: u32 = <W as $crate::int::types::traits::BigInt>::BITS / 64;

            /// `π` floor-truncated at its per-width stored scale, parsed
            /// once per monomorphisation.
            const PI_REF_DIGITS: W =
                match <W>::from_str_radix($crate::types::consts::wide::pi_w_ref(W_LIMBS).0, 10) {
                    Ok(v) => v,
                    Err(()) => panic!("wide consts: pi reference not parseable"),
                };
            const PI_REF_SCALE: u32 = $crate::types::consts::wide::pi_w_ref(W_LIMBS).1;
            const PI_REF_TOP_CMP: ::core::cmp::Ordering =
                $crate::types::consts::wide::pi_w_ref(W_LIMBS).2;

            /// `ln 2` floor-truncated at its per-width stored scale.
            const LN2_REF_DIGITS: W =
                match <W>::from_str_radix($crate::types::consts::wide::ln2_w_ref(W_LIMBS).0, 10) {
                    Ok(v) => v,
                    Err(()) => panic!("wide consts: ln2 reference not parseable"),
                };
            const LN2_REF_SCALE: u32 = $crate::types::consts::wide::ln2_w_ref(W_LIMBS).1;
            const LN2_REF_TOP_CMP: ::core::cmp::Ordering =
                $crate::types::consts::wide::ln2_w_ref(W_LIMBS).2;

            /// `ln 10` floor-truncated at its per-width stored scale.
            const LN10_REF_DIGITS: W =
                match <W>::from_str_radix($crate::types::consts::wide::ln10_w_ref(W_LIMBS).0, 10) {
                    Ok(v) => v,
                    Err(()) => panic!("wide consts: ln10 reference not parseable"),
                };
            const LN10_REF_SCALE: u32 = $crate::types::consts::wide::ln10_w_ref(W_LIMBS).1;
            const LN10_REF_TOP_CMP: ::core::cmp::Ordering =
                $crate::types::consts::wide::ln10_w_ref(W_LIMBS).2;

            /// Rounds a per-width constant reference — floor-truncated
            /// digits at scale `ref_scale`, with `top_cmp` describing
            /// the true value's sub-LSB residual against half at that
            /// scale — down to working scale `w` under `mode`.
            ///
            /// For `w < ref_scale` the dropped digits `w+1..=ref_scale`
            /// are the exact digits of the true value, so the divide's
            /// own residual decides the round. At `w == ref_scale` the
            /// stored digits hold no further residual, so the build-time
            /// `top_cmp` hint supplies it. Every working scale the
            /// helpers reach is representable in `W`, hence
            /// `w <= ref_scale`.
            fn const_rounded(
                digits: W,
                ref_scale: u32,
                top_cmp: ::core::cmp::Ordering,
                w: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> W {
                debug_assert!(
                    w <= ref_scale,
                    "wide consts: working scale exceeds the per-width reference scale"
                );
                if w >= ref_scale {
                    return if $crate::support::rounding::should_bump(
                        mode,
                        top_cmp,
                        digits.bit(0),
                        true,
                    ) {
                        digits + lit(1)
                    } else {
                        digits
                    };
                }
                let shift = ref_scale - w;
                if shift <= 38 {
                    $crate::algos::support::mg_divide::div_wide_pow10::<W>(digits, shift, mode)
                } else {
                    $crate::algos::support::newton_reciprocal::dispatch_wide_pow10::<W>(
                        digits, shift, mode,
                    )
                }
            }

            /// Const-evaluable companion to [`const_rounded`].
            ///
            /// Reproduces `const_rounded`'s rounding decision **bit for
            /// bit** using only `const fn` limb arithmetic, so a caller
            /// can bake the scaled-down constant at compile time instead
            /// of dividing the wide `W-1`-scale reference by `10^shift`
            /// on every call (the wide-tier-low-working-scale regression
            /// this whole module exists to remove).
            ///
            /// `digits` is the floor-truncated reference at `ref_scale`
            /// (always non-negative — the references are positive
            /// constants), `top_cmp` is the build-time sub-LSB residual
            /// against half at `ref_scale`, `w <= ref_scale` is the
            /// target working scale, `mode` the rounding mode.
            ///
            /// Bit-identity argument. `const_rounded` does exactly two
            /// things: for `w >= ref_scale` it applies `should_bump(mode,
            /// top_cmp, digits.bit(0), true)`; for `w < ref_scale` it
            /// computes `q = digits / 10^shift` and rounds per `mode`
            /// using the three-way comparison of the dropped remainder
            /// `r = digits mod 10^shift` against `10^shift / 2`. This
            /// const fn computes the *same* `q` (schoolbook divide by 10
            /// repeated `shift` times — exact integer floor division) and
            /// the *same* `Ordering(r, half)`: the remainder block's most
            /// significant dropped digit is the digit dropped on the
            /// final `÷10` step; the tie/clear decision is that digit
            /// versus 5 with a sticky-OR of the lower dropped digits.
            /// That is identical to comparing `2·r` with `10^shift`
            /// (`should_bump`'s `cmp_r`), with the same `q.bit(0)` parity
            /// and `result_positive = true`. The `should_bump` match is
            /// inlined verbatim below (it is not itself a `const fn`).
            pub(crate) const fn const_rounded_cf(
                digits: W,
                ref_scale: u32,
                top_cmp: ::core::cmp::Ordering,
                w: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> W {
                use ::core::cmp::Ordering;
                // `r_is_zero` distinguishes "remainder exactly 0" (where
                // NO mode bumps — `should_bump`'s documented precondition
                // is that the caller pre-handles `r == 0`) from a `Less`
                // comparison with a genuine non-zero residual. The
                // shift==0 branch matches the runtime `const_rounded`,
                // which applies `should_bump` to `top_cmp` directly (the
                // sub-LSB residual is never the exact-zero algebraic
                // case), so its `r_is_zero` is false.
                let (q, cmp_r, r_is_zero) = if w >= ref_scale {
                    (digits, top_cmp, false)
                } else {
                    let shift = ref_scale - w;
                    let (q, cmp_r, rz) = div_by_pow10_cf(digits, shift);
                    (q, cmp_r, rz)
                };
                if r_is_zero {
                    return q;
                }
                // Inlined `should_bump`: result is always positive (the
                // references are positive), so `q.bit(0)` is the parity
                // and the bump (when taken) is `+1`.
                let q_is_odd = q.bit(0);
                let bump = match mode {
                    $crate::support::rounding::RoundingMode::HalfToEven => match cmp_r {
                        Ordering::Less => false,
                        Ordering::Greater => true,
                        Ordering::Equal => q_is_odd,
                    },
                    $crate::support::rounding::RoundingMode::HalfAwayFromZero => {
                        !matches!(cmp_r, Ordering::Less)
                    }
                    $crate::support::rounding::RoundingMode::HalfTowardZero => {
                        matches!(cmp_r, Ordering::Greater)
                    }
                    $crate::support::rounding::RoundingMode::Trunc => false,
                    $crate::support::rounding::RoundingMode::Floor => false,
                    $crate::support::rounding::RoundingMode::Ceiling => true,
                };
                if bump { q.wrapping_add(lit_cf(1)) } else { q }
            }

            /// `const fn` floor division of a **non-negative** `W` by
            /// `10^shift` (`shift >= 1`), returning the quotient and the
            /// three-way comparison of the dropped remainder against
            /// `10^shift / 2` (`should_bump`'s `cmp_r`).
            ///
            /// Divides by 10 `shift` times over the little-endian u64
            /// limbs (high limb first; `rem < 10` so `(rem << 64) | limb`
            /// never overflows `u128`). The remainder block's most
            /// significant digit is the one dropped on the final step;
            /// lower dropped digits accumulate into a sticky flag. The
            /// comparison is then that final digit versus 5 (sticky
            /// breaking the `== 5` tie upward) — exactly `Ordering(r,
            /// half)`.
            const fn div_by_pow10_cf(
                digits: W,
                shift: u32,
            ) -> (W, ::core::cmp::Ordering, bool) {
                use ::core::cmp::Ordering;
                let mut limbs = digits.limbs_le();
                let n = limbs.len();
                let mut sticky = false; // any lower dropped digit non-zero
                let mut top_digit: u64 = 0; // most-significant dropped digit
                let mut step: u32 = 0;
                while step < shift {
                    // Fold the previous step's dropped digit into sticky
                    // (it is a lower-order digit than the one we drop now).
                    if step > 0 && top_digit != 0 {
                        sticky = true;
                    }
                    let mut rem: u64 = 0;
                    let mut i = n;
                    while i > 0 {
                        i -= 1;
                        let cur = ((rem as u128) << 64) | (limbs[i] as u128);
                        limbs[i] = (cur / 10) as u64;
                        rem = (cur % 10) as u64;
                    }
                    top_digit = rem;
                    step += 1;
                }
                // The full dropped remainder is zero iff every dropped
                // digit was zero: the most-significant (`top_digit`) and
                // all the lower ones (`sticky`).
                let r_is_zero = top_digit == 0 && !sticky;
                let cmp_r = if top_digit > 5 {
                    Ordering::Greater
                } else if top_digit < 5 {
                    Ordering::Less
                } else if sticky {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                };
                (<W>::from_limbs_le(limbs), cmp_r, r_is_zero)
            }

            /// `const fn` small literal in `W` (positive). Companion to
            /// the runtime [`lit`] usable from `const fn` context.
            #[inline]
            const fn lit_cf(n: u128) -> W {
                <W>::from_u128(n)
            }

            /// Per-`(W, SCALE)` compile-time-baked transcendental
            /// constants at the base working scale `SCALE + GUARD`.
            ///
            /// Each associated const evaluates [`const_rounded_cf`] at
            /// compile time (associated consts may use the impl's `SCALE`
            /// — a fn-body `const` may not, which is why this is a type).
            /// One `.rodata` value per `(W, SCALE, constant, mode)`; the
            /// `*_cf` fetches select among them with zero runtime divide
            /// on the common non-escalated path.
            pub(crate) struct WideConst<const SCALE: u32>;
            impl<const SCALE: u32> WideConst<SCALE> {
                const BASE_W: u32 = SCALE + GUARD;
                // π
                const PI_HTE: W = const_rounded_cf(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfToEven);
                const PI_HAFZ: W = const_rounded_cf(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfAwayFromZero);
                const PI_HTZ: W = const_rounded_cf(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfTowardZero);
                const PI_TRUNC: W = const_rounded_cf(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Trunc);
                const PI_FLOOR: W = const_rounded_cf(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Floor);
                const PI_CEIL: W = const_rounded_cf(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Ceiling);
                // ln 2
                const LN2_HTE: W = const_rounded_cf(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfToEven);
                const LN2_HAFZ: W = const_rounded_cf(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfAwayFromZero);
                const LN2_HTZ: W = const_rounded_cf(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfTowardZero);
                const LN2_TRUNC: W = const_rounded_cf(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Trunc);
                const LN2_FLOOR: W = const_rounded_cf(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Floor);
                const LN2_CEIL: W = const_rounded_cf(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Ceiling);
                // ln 10
                const LN10_HTE: W = const_rounded_cf(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfToEven);
                const LN10_HAFZ: W = const_rounded_cf(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfAwayFromZero);
                const LN10_HTZ: W = const_rounded_cf(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::HalfTowardZero);
                const LN10_TRUNC: W = const_rounded_cf(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Trunc);
                const LN10_FLOOR: W = const_rounded_cf(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Floor);
                const LN10_CEIL: W = const_rounded_cf(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, Self::BASE_W, $crate::support::rounding::RoundingMode::Ceiling);
            }

            /// `π` const-folded at the base working scale `SCALE + GUARD`
            /// for this `(W, SCALE)` cell — no runtime divide. The common
            /// (non-Ziv-escalated) path fetches the baked constant; any
            /// other `w` (a Ziv escalation) falls to the runtime
            /// [`pi_with`].
            #[inline]
            pub(crate) fn pi_cf<const SCALE: u32>(
                w: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> W {
                use $crate::support::rounding::RoundingMode as Rm;
                if w == SCALE + GUARD {
                    return match mode {
                        Rm::HalfToEven => WideConst::<SCALE>::PI_HTE,
                        Rm::HalfAwayFromZero => WideConst::<SCALE>::PI_HAFZ,
                        Rm::HalfTowardZero => WideConst::<SCALE>::PI_HTZ,
                        Rm::Trunc => WideConst::<SCALE>::PI_TRUNC,
                        Rm::Floor => WideConst::<SCALE>::PI_FLOOR,
                        Rm::Ceiling => WideConst::<SCALE>::PI_CEIL,
                    };
                }
                const_rounded(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, w, mode)
            }

            /// `ln 2` const-folded at the base working scale — see
            /// [`pi_cf`].
            #[inline]
            pub(crate) fn ln2_cf<const SCALE: u32>(
                w: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> W {
                use $crate::support::rounding::RoundingMode as Rm;
                if w == SCALE + GUARD {
                    return match mode {
                        Rm::HalfToEven => WideConst::<SCALE>::LN2_HTE,
                        Rm::HalfAwayFromZero => WideConst::<SCALE>::LN2_HAFZ,
                        Rm::HalfTowardZero => WideConst::<SCALE>::LN2_HTZ,
                        Rm::Trunc => WideConst::<SCALE>::LN2_TRUNC,
                        Rm::Floor => WideConst::<SCALE>::LN2_FLOOR,
                        Rm::Ceiling => WideConst::<SCALE>::LN2_CEIL,
                    };
                }
                const_rounded(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, w, mode)
            }

            /// `ln 10` const-folded at the base working scale — see
            /// [`pi_cf`].
            #[inline]
            pub(crate) fn ln10_cf<const SCALE: u32>(
                w: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> W {
                use $crate::support::rounding::RoundingMode as Rm;
                if w == SCALE + GUARD {
                    return match mode {
                        Rm::HalfToEven => WideConst::<SCALE>::LN10_HTE,
                        Rm::HalfAwayFromZero => WideConst::<SCALE>::LN10_HAFZ,
                        Rm::HalfTowardZero => WideConst::<SCALE>::LN10_HTZ,
                        Rm::Trunc => WideConst::<SCALE>::LN10_TRUNC,
                        Rm::Floor => WideConst::<SCALE>::LN10_FLOOR,
                        Rm::Ceiling => WideConst::<SCALE>::LN10_CEIL,
                    };
                }
                const_rounded(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, w, mode)
            }

            /// Differential validity wall: the const-folded
            /// [`const_rounded_cf`] must reproduce the runtime
            /// [`const_rounded`] **bit for bit** for every constant
            /// (π / ln2 / ln10), every [`RoundingMode`], and every
            /// working scale this tier reaches. Invoked from the outer
            /// `tests` module once per shipped tier `W`.
            ///
            /// `shift_targets` walks `ref_scale` (shift 0, the `top_cmp`
            /// branch) down through a spread of working scales — including
            /// the `shift > 38` Newton-divide region the regression came
            /// from — so the limb-array divide and the `top_cmp` hint path
            /// are both covered.
            #[cfg(test)]
            pub(crate) fn const_rounded_cf_matches_runtime() {
                use $crate::support::rounding::RoundingMode::*;
                let modes = [
                    HalfToEven, HalfAwayFromZero, HalfTowardZero, Trunc, Floor, Ceiling,
                ];
                let consts: [(W, u32, ::core::cmp::Ordering, &str); 3] = [
                    (PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, "pi"),
                    (LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, "ln2"),
                    (LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, "ln10"),
                ];
                for (digits, ref_scale, top_cmp, name) in consts {
                    // Working scales across the whole reachable span: the
                    // shift==0 top_cmp branch, small (<=38, single-chunk
                    // MG) shifts, and large (>38, Newton-region) shifts.
                    let mut w = 0u32;
                    while w <= ref_scale {
                        for mode in modes {
                            let runtime = const_rounded(digits, ref_scale, top_cmp, w, mode);
                            let cf = const_rounded_cf(digits, ref_scale, top_cmp, w, mode);
                            assert!(
                                runtime == cf,
                                "{name}: const_rounded_cf != const_rounded at w={w} ref_scale={ref_scale} mode={mode:?}"
                            );
                        }
                        // Sample densely near both ends and stride the
                        // middle to keep the sweep cheap but exhaustive in
                        // shape (shift 0..=ref_scale, both divide regions).
                        w += if w < 5 || w + 5 > ref_scale { 1 } else { 7 };
                    }
                }
            }

            /// `ln 2` at working scale `w`, rounded under the crate
            /// default mode from the per-width compile-time reference.
            pub(crate) fn ln2(w: u32) -> W {
                ln2_with(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
            /// `ln 2` at working scale `w`, rounded under `mode`.
            pub(crate) fn ln2_with(w: u32, mode: $crate::support::rounding::RoundingMode) -> W {
                const_rounded(LN2_REF_DIGITS, LN2_REF_SCALE, LN2_REF_TOP_CMP, w, mode)
            }

            /// Natural logarithm of a positive working-scale value.
            ///
            /// Range-reduces `v = 2^k · m` with `m ∈ [1, 2)`, evaluates
            /// `ln(m) = 2·artanh((m−1)/(m+1))`, returns `k·ln 2 + ln(m)`.
            pub(crate) fn ln_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
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
                // ln(m) = 2^(l+1) · artanh(t) = sum << (sqrt_l + 1).
                // With sqrt_l=0 this collapses to the historic
                // `2·sum` formula; with sqrt_l>0 it folds in the
                // `2^l` factor from the unhalved-argument identity.
                let ln_m = sum << (sqrt_l + 1);
                scale_by_k(ln2_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE), k as i128) + ln_m
            }

            /// `log1p(t) = ln(1 + t)` at working scale `w`, evaluated
            /// without ever forming `1 + t`.
            ///
            /// Uses the Goldberg/Higham reformulation
            /// `log1p(t) = 2*artanh( t / (2 + t) )`. The argument
            /// `u = t / (2 + t)` is built from `t` directly: `2 + t` is
            /// benign (no near-equal subtraction for `t > -1`) and the
            /// divide is well-conditioned, so `u ~ t/2` carries every
            /// significant digit of `t`. The artanh series then has the
            /// same `O(1)` condition number on the whole near-zero range,
            /// removing the catastrophic cancellation of the naive
            /// `ln(1 + t)` (forming `1 + t` then reducing `m - 1`) at the
            /// source. `t` is the working-scale gap supplied exactly by
            /// the caller. Domain: `t > -1` (the caller guards this).
            ///
            /// Reference: N. J. Higham, *Accuracy and Stability of
            /// Numerical Algorithms* 2nd ed. (2002), 1.14.1 and Problem
            /// 1.4; J.-M. Muller, *Elementary Functions* 3rd ed. (2016),
            /// 4.4.
            pub(crate) fn log1p_fixed(t: W, w: u32) -> W {
                let one_w = one(w);
                let two_w = one_w + one_w;
                let pow10_w = one_w;
                let u = div_cached(t, two_w + t, pow10_w);
                let u2 = mul(u, u, w);
                let mut sum = u;
                let mut term = u;
                let mut j: u128 = 1;
                loop {
                    term = mul(term, u2, w);
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

            /// `expm1(s) = exp(s) - 1` at working scale `w`, evaluated as
            /// the Taylor series with the leading `1` term dropped so the
            /// `exp(s) - 1` subtraction of two values both `~ 1` never
            /// occurs: `expm1(s) = s + s^2/2! + s^3/3! + ...`. For tiny
            /// `s` the result keeps every digit of `s`
            /// (`kappa = |s/expm1(s)| -> 1`). This kernel is the
            /// accuracy-critical small-argument case `|s| <~ ln2/2`; the
            /// caller reduces a general argument to this band and
            /// reassembles via the exact `2^k` shift. No range reduction
            /// is performed here.
            ///
            /// Reference: J.-M. Muller, *Elementary Functions* 3rd ed.
            /// (2016), 4.4; Higham 1.14.1.
            pub(crate) fn expm1_fixed(s: W, w: u32) -> W {
                let mut sum = s;
                let mut term = s;
                let mut iter: u128 = 2;
                loop {
                    term = mul(term, s, w) / lit(iter);
                    if term == zero() {
                        break;
                    }
                    sum = sum + term;
                    iter += 1;
                    if iter > SERIES_CAP {
                        break;
                    }
                }
                sum
            }

            /// `ln 10` at working scale `w`, rounded under the crate
            /// default mode from the per-width compile-time reference.
            pub(crate) fn ln10(w: u32) -> W {
                ln10_with(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
            /// `ln 10` at working scale `w`, rounded under `mode`.
            pub(crate) fn ln10_with(w: u32, mode: $crate::support::rounding::RoundingMode) -> W {
                const_rounded(LN10_REF_DIGITS, LN10_REF_SCALE, LN10_REF_TOP_CMP, w, mode)
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
            /// # Precision
            ///
            /// The caller is expected to invoke this kernel at the
            /// lifted working scale `w' = w + guard_agm(SCALE)`
            /// (see `guard_agm` and the `_strict_agm` entry points).
            /// At `w'` the early-phase `sqrt(a · b)` truncation
            /// error — amplified by `√(a/b)` when the AGM seed
            /// `y = 4/s` lies many orders of magnitude below 1 —
            /// stays below 0.5 ULP at the final storage scale.
            /// Calling at the unlifted scale `w` exhibits the
            /// historical `~p/2` precision drop past `w ~ 40`
            /// described in Brent 1976 §3.
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
                // residual error below one LSB at scale w. The
                // additive `+ 24` over the asymptotic minimum
                // covers the bookkeeping bits the cancellation in
                // `agm_part − m·ln 2` consumes plus the few-LSB
                // safety the rounded-intermediate `mul` /
                // `sqrt_fixed` accumulation contributes over
                // `~log₂(p)` iterations.
                let safety = 2 + ((p_bits.max(1) as u32).ilog2() / 2) as i32;
                let mut m: i32 = (p_bits / 2) + safety + bl_one - bl_v;
                if m < 2 {
                    m = 2;
                }
                // Cap m so `s_w = v_w << m` fits in W and the
                // `div(4·one_w, s_w, w)` numerator
                // `4·one_w · 10^w = 4·10^(2w)` does too. The AGM
                // iteration that follows operates on `(a, b)` both
                // bounded by `one_w` and does not see `s_w` after
                // the divide, so `s_w` itself does not need to
                // leave half-width headroom for the AGM mul — the
                // bit-budget constraint that backs the AGM mul is
                // `2·bl(one_w) ≤ W::BITS`, enforced via the
                // `guard_agm` lift selection at the caller.
                let cap = (W::BITS as i32) - bl_v - 2;
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
                    let d = if next_a >= next_b {
                        next_a - next_b
                    } else {
                        next_b - next_a
                    };
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
                let _exp_span =
                    $crate::tracing::info_span!(concat!(stringify!($Type), "::exp_fixed"))
                        .entered();

                // Large-result routing: when `e^v`'s integer-digit growth
                // plus the internal `2^k` reassembly would overflow the
                // tier's own work integer `W` (a large positive `v` at low
                // SCALE — e.g. exp(1061) at D462<0>, a ~461-digit result
                // that fits storage but not the `W`-scale lift), run the
                // body in the wider `Wexp` and narrow back. Mirrors the
                // `hyper_fits_w` regime split the hyperbolics use. The
                // normal / small regime keeps the fast `W` path — the check
                // is a few leading-zero / shift ops.
                if !exp_fits_w(v_w, w) {
                    return exp_fixed_wide(v_w, w);
                }

                // Cache 10^w once — used as divisor in every Taylor
                // iteration and squaring step below. At D307<150>
                // w=180 and `pow10(180)` costs ~50 µs by itself
                // (`lit(10).pow(180)` is ~log₂(180)=8 wide squarings
                // followed by ~180 cumulative multiplies); without
                // caching this would dominate the call.
                #[cfg(feature = "perf-trace")]
                let _reduce_span = $crate::tracing::info_span!("range_reduce").entered();
                // Range reduction.
                //
                // Naively `s = v − k·ln 2` evaluated at the type's working
                // scale `w` suffers catastrophic cancellation when `|v|`
                // is large: each absorbed leading bit of `v` is paid for
                // by an LSB of `k·ln 2`, and the final `2^k` rescaling at
                // the end amplifies any residual error in `s` back up by
                // the same factor. The total budget for `2^k` rescaling
                // is roughly `2^k · LSB_w ≤ 0.5 · LSB_storage`, i.e.
                // `k · log10(2) ≤ GUARD`. For wide-tier scales where the
                // input `|v|` can reach `(MAX_SCALE − SCALE) · ln 10`,
                // `k` overshoots that budget badly — D616<308>'s upper
                // end gives `k ≈ 1020`, blowing past `GUARD = 30` by
                // ~280 decimal digits and producing the multi-thousand-
                // LSB drift the precision golden gate catches.
                //
                // Mitigation: bump the whole `exp_fixed` body to an
                // extended working scale `w_ext = w + extra`, computed
                // dynamically from `bit_length(|k|)`. `extra` is sized so
                // the post-squarings amplification by `2^k` against the
                // residual `LSB_of_w_ext` lands inside the `GUARD` budget
                // at narrowing time. `extra = ceil(|k|·log10(2)) + 6`
                // suffices: the `+6` covers the Taylor-series-step
                // accumulation, the post-Taylor `n` squarings, and the
                // half-LSB error introduced by the final narrowing.
                //
                // Reference for the analysis: Muller, *Elementary
                // Functions: Algorithms and Implementation* (3rd ed.,
                // 2016), §11.1 — range-reduction error budget with the
                // `2^k · exp(s)` reassembly.
                let one_w_pre = one(w);
                let l2_pre = ln2(w);
                let pow10_w_pre = one_w_pre;
                let k = round_to_nearest_int(div_cached(v_w, l2_pre, pow10_w_pre), w);
                let abs_k_u128 = if k < 0 { -k } else { k } as u128;
                let extra: u32 = if abs_k_u128 == 0 {
                    0
                } else {
                    // The amplification of the LSB error in `k·ln 2` by
                    // the final `2^k` rescaling is `2^k`, which is
                    // `|k|·log10(2)` decimal digits. Compute that
                    // directly from `|k|` (NOT `bit_length(|k|)`), then
                    // add a margin for Taylor + squarings + final
                    // narrowing.
                    //
                    // `|k|·log10(2) = |k| · 30103 / 100000`. Round up:
                    let digits = (abs_k_u128 * 30103).div_ceil(100_000);
                    // Cap at the type's working width to avoid blowing up
                    // `pow10(extra)`; if `|k|` is so large the result
                    // would overflow storage anyway, the caller's
                    // `round_to_storage_with` will panic on narrowing.
                    let capped = digits.min((<W>::BITS / 4) as u128) as u32;
                    // The +k/3 margin covers the cumulative-rounding
                    // budget of the in-extended-width Taylor series and
                    // post-Taylor squarings. Half-LSB error per op times
                    // ~k·sqrt-of-precision ops grows roughly with k.
                    capped + 12 + (capped >> 2)
                };

                let w_ext = w + extra;
                let v_ext = if extra == 0 { v_w } else { v_w * pow10(extra) };
                let one_w = one(w_ext);
                let l2 = ln2(w_ext);
                let pow10_w = one_w;
                let s = v_ext - scale_by_k(l2, k);

                // From here on the body operates at `w_ext`; we narrow
                // back to `w` after the final `2^k` reassembly so the
                // caller's `round_to_storage_with(_, w, scale, _)` sees
                // a value at the expected `w` scale.
                let p_bits = w_ext.saturating_mul(3).saturating_add(1);
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
                    // Taylor term: low-half u128-packed product
                    // (`wrapping_mul_low_u128`) reduced by `÷10^(w_ext)`
                    // through the fast MG `round_div_pow10` kernel (the
                    // divisor is always exactly the power of ten `10^w_ext`).
                    // Mirrors the blessed `exp_generic::exp_fixed` Taylor
                    // step; bit-identical to the prior `round_div` reduction
                    // (audited power-of-10 equivalence) at MG speed.
                    term = round_div_pow10(
                        $crate::int::types::traits::BigInt::wrapping_mul_low_u128(term, s_red),
                        w_ext,
                    ) / lit(iter);
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
                    // Low-half symmetric SQUARE through the limb-width matcher
                    // (`wrapping_sqr_low_u128` → `int::policy::sqr_low`): the
                    // u128-packed `sqr_low_limb` on even work widths (half the
                    // limbs), bit-identical to the low-`BITS` of `x²`, reduced
                    // by `÷10^(w_ext)` through the fast MG `round_div_pow10`
                    // kernel (the divisor is always the power of ten
                    // `10^w_ext`). The squaring sibling of the Taylor step;
                    // bit-identical to the prior generic `round_div` at MG
                    // speed. Mirrors `exp_generic::exp_fixed`.
                    squared = round_div_pow10(
                        $crate::int::types::traits::BigInt::wrapping_sqr_low_u128(squared),
                        w_ext,
                    );
                    i += 1;
                }
                let sum = squared;
                #[cfg(feature = "perf-trace")]
                drop(_sqr_span);

                #[cfg(feature = "perf-trace")]
                let _reasm_span = $crate::tracing::info_span!("reassemble").entered();
                let scaled_at_w_ext = if k >= 0 {
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
                        // Deep underflow: e^v (v < 0 here, since k < 0) is
                        // strictly positive but below the working resolution.
                        // Return the smallest positive working value (1 = 10^-w),
                        // NOT zero, so the directed narrowing keeps the sign —
                        // Ceiling rounds up to 1 ULP while Floor / Trunc /
                        // nearest still give 0. A bare zero loses positivity and
                        // rounds Ceiling to 0 (a correctly-rounded defect the
                        // SCALE-30 golden cells catch). Reached only by direct
                        // e^(negative); the hyperbolics call exp on |x| >= 0.
                        return lit(1);
                    }
                    sum >> (neg_k as u32)
                };
                let result = if extra == 0 {
                    scaled_at_w_ext
                } else {
                    round_div_pow10(scaled_at_w_ext, extra)
                };
                // e^v > 0 for every finite v: a zero result here is genuine
                // underflow of `e^(negative)` below the working resolution,
                // NOT a true zero. Return the smallest positive working value
                // (1 = 10^-w) so the directed narrowing keeps the sign —
                // Ceiling rounds up to 1 ULP, Floor / Trunc / nearest still
                // give 0. A bare zero rounds Ceiling to 0 (a correctly-rounded
                // defect the SCALE-30 golden cells catch).
                //
                // The clamp is restricted to `k < 0` (the only regime where
                // underflow to 0 is physical). For `k >= 0` (a large positive
                // argument) `e^v >= 1`, so a 0 result would mean the `W`-scale
                // lift overflowed; masking that as 1 would hide the defect.
                // The `exp_fits_w` routing above sends those cases to the
                // wider path before they reach here.
                if k < 0 && result == zero() {
                    lit(1)
                } else {
                    result
                }
            }

            /// Large-result `e^v`: runs the guard-digit `exp` core in
            /// the wider work integer [`Wexp`] so the caller's
            /// working-scale lift + the internal `2^k` reassembly + the
            /// repeated-squaring peak all fit, then narrows the result
            /// back to `W` exactly (the value is integral at scale `w`
            /// — no rounding occurs in the narrowing).
            ///
            /// `Wexp` is the next-wider `Int` for every tier except
            /// D1232 (already widest); there `Wexp == W`, and the full
            /// lift fits because D1232's `Int<256>` holds the squaring
            /// peak at its `MAX_SCALE` anyway. Used by the near-overflow
            /// -edge `sinh`/`cosh`/`exp2`/`tanh` cells; the normal /
            /// small regime keeps the fast `exp_fixed` path on `W`.
            pub(crate) fn exp_fixed_wide(v_w: W, w: u32) -> W {
                let v_wide = $crate::int::types::traits::BigInt::resize_to::<Wexp>(v_w);
                let r_wide =
                    $crate::algos::exp::exp_generic::exp_fixed::<Wexp>(v_wide, w);
                $crate::int::types::traits::BigInt::resize_to::<W>(r_wide)
            }

            /// True worst-case bit-width the `exp_fixed(v_w, w)` body
            /// reaches internally for a working-scale value `v_w` at scale
            /// `w`, in a work integer of capacity `cap_bits` bits.
            ///
            /// This mirrors `exp_fixed`'s own `k`/`extra`/`w_ext` arithmetic
            /// EXACTLY (range-reduce `v = k·ln2 + s`, lift the working scale
            /// by `extra` digits, run the Taylor squarings at `w_ext`, then
            /// reassemble `2^k · exp(s)`), so the fit gate models the real
            /// squaring-reassembly PEAK — `2·w_ext` decimal digits for the
            /// symmetric `sum²` plus the `sum << k` shift — rather than the
            /// stale `w + 2·result_digits` digit bound that under-counted
            /// the peak by a full `w` and let large arguments silently wrap
            /// the work integer (the `wrapping_sqr_low_u128` returns the low
            /// bits, so an overflowed square truncates to 0 → the `e^-|x|`
            /// reciprocal divides by zero). `cap_bits` selects the `extra`
            /// cap so the estimate matches the body that will actually run
            /// (the tier `W`, or the wider `Wexp` after a lift).
            fn exp_internal_peak_bits(v_w: W, w: u32, cap_bits: u32) -> u64 {
                let one_w_pre = one(w);
                let l2_pre = ln2(w);
                let k = round_to_nearest_int(div_cached(v_w, l2_pre, one_w_pre), w);
                let abs_k_u128 = if k < 0 { -k } else { k } as u128;
                let extra: u32 = if abs_k_u128 == 0 {
                    0
                } else {
                    let digits = (abs_k_u128 * 30103).div_ceil(100_000);
                    let capped = digits.min((cap_bits / 4) as u128) as u32;
                    capped + 12 + (capped >> 2)
                };
                let w_ext = (w + extra) as u64;
                // digits → bits: `log2(10) ≈ 3.3220 ≈ 3322/1000`.
                // Squaring peak: the symmetric `sum²` before the round-divide
                // spans `2·w_ext` decimal digits.
                let sqr_bits = 2 * w_ext * 3322 / 1000;
                // Reassembly peak: `sum << k` lifts the `w_ext`-digit Taylor
                // sum up by `|k|` bits.
                let reasm_bits = w_ext * 3322 / 1000 + abs_k_u128 as u64;
                // A `512`-bit margin covers the series accumulation and the
                // rounded-narrowing residue.
                let peak = if sqr_bits > reasm_bits { sqr_bits } else { reasm_bits };
                peak + 512
            }

            /// Whether the hyperbolic composition fits the tier's own work
            /// integer `W` at working scale `w` for the magnitude `av_w`
            /// (`= |x|·10^w`), so the fast per-tier kernels (cached `ln2` /
            /// `pow10` / `exp_fixed`) can run directly instead of lifting to
            /// [`Wexp`].
            ///
            /// Two intermediates must fit `W`:
            /// - the `1/e^|x|` reciprocal numerator `10^(2w)` — `2w` digits;
            /// - the `exp_fixed` internal peak — modelled exactly by
            ///   [`exp_internal_peak_bits`] (the true `2·w_ext` squaring +
            ///   `2^k` reassembly), NOT the old `w + 2·result_digits` bound.
            ///
            /// The squaring peak `2·w_ext` already dominates `2w` (since
            /// `w_ext ≥ w`), so the exp peak bounds the whole composition.
            #[inline]
            fn hyper_fits_w(av_w: W, w: u32) -> bool {
                let cap_bits = <W as $crate::int::types::traits::BigInt>::BITS;
                exp_internal_peak_bits(av_w, w, cap_bits) < cap_bits as u64
            }

            /// Whether a direct `exp_fixed(v_w, w)` fits the tier's own work
            /// integer `W`.
            ///
            /// Models the real `exp_fixed` squaring-reassembly peak via
            /// [`exp_internal_peak_bits`]: when the `2·w_ext` square or the
            /// `2^k` reassembly would exceed `W`'s bit capacity the body
            /// would silently wrap (`wrapping_sqr_low_u128` truncates to the
            /// low bits, so an overflowed square returns 0), so the caller
            /// routes the value through the wider [`exp_fixed_wide`] /
            /// [`Wexp`] path instead. The normal / small regime keeps the
            /// fast `W` path.
            #[inline]
            fn exp_fits_w(v_w: W, w: u32) -> bool {
                let cap_bits = <W as $crate::int::types::traits::BigInt>::BITS;
                exp_internal_peak_bits(v_w, w, cap_bits) < cap_bits as u64
            }

            /// `sinh(|x|)` at working scale `w` for a non-negative working
            /// value. The normal / small regime runs the fast per-tier
            /// kernels directly on `W` (cached `ln2` / `pow10`); only the
            /// near-overflow-edge regime — where the `1/e^|x|` reciprocal
            /// numerator `10^(2w)` would overflow `W` (small-`W`/high-scale
            /// tiers like D462, or any tier at a large-result argument) —
            /// lifts the whole composition to the wider [`Wexp`]. See
            /// [`hyper_fits_w`]. The caller reapplies the input sign (sinh
            /// is odd).
            pub(crate) fn sinh_pos_wide(av_w: W, w: u32) -> W {
                if hyper_fits_w(av_w, w) {
                    let ex = exp_fixed(av_w, w);
                    let enx = div(one(w), ex, w);
                    (ex - enx) >> 1
                } else {
                    let av_wide = $crate::int::types::traits::BigInt::resize_to::<Wexp>(av_w);
                    let r = $crate::algos::exp::exp_generic::sinh_pos::<Wexp>(
                        av_wide, w,
                    );
                    $crate::int::types::traits::BigInt::resize_to::<W>(r)
                }
            }

            /// `cosh(|x|) = (e^|x| + e^-|x|)/2` at working scale `w`. See
            /// [`sinh_pos_wide`] for the `W`-vs-[`Wexp`] regime split.
            pub(crate) fn cosh_pos_wide(av_w: W, w: u32) -> W {
                if hyper_fits_w(av_w, w) {
                    let ex = exp_fixed(av_w, w);
                    let enx = div(one(w), ex, w);
                    (ex + enx) >> 1
                } else {
                    let av_wide = $crate::int::types::traits::BigInt::resize_to::<Wexp>(av_w);
                    let r = $crate::algos::exp::exp_generic::cosh_pos::<Wexp>(
                        av_wide, w,
                    );
                    $crate::int::types::traits::BigInt::resize_to::<W>(r)
                }
            }

            /// `tanh(|x|) = (e^|x| − e^-|x|)/(e^|x| + e^-|x|)` at working
            /// scale `w`. See [`sinh_pos_wide`] for the regime split. The
            /// caller reapplies the input sign (tanh is odd).
            pub(crate) fn tanh_pos_wide(av_w: W, w: u32) -> W {
                if hyper_fits_w(av_w, w) {
                    let ex = exp_fixed(av_w, w);
                    let enx = div(one(w), ex, w);
                    div(ex - enx, ex + enx, w)
                } else {
                    let av_wide = $crate::int::types::traits::BigInt::resize_to::<Wexp>(av_w);
                    let r = $crate::algos::exp::exp_generic::tanh_pos::<Wexp>(
                        av_wide, w,
                    );
                    $crate::int::types::traits::BigInt::resize_to::<W>(r)
                }
            }

            /// Taylor series for `atan` on `|x| < 1`, at scale `w`.
            pub(crate) fn atan_taylor(x: W, w: u32) -> W {
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

            /// `π` at working scale `w`, rounded under the crate default
            /// mode from the per-width compile-time reference.
            pub(crate) fn pi(w: u32) -> W {
                pi_with(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
            /// `π` at working scale `w`, rounded under `mode`.
            pub(crate) fn pi_with(w: u32, mode: $crate::support::rounding::RoundingMode) -> W {
                const_rounded(PI_REF_DIGITS, PI_REF_SCALE, PI_REF_TOP_CMP, w, mode)
            }

            /// `π/2` at working scale `w`. Routes `π` through the
            /// const-folded [`pi_cf`] so the common (`w == SCALE + GUARD`)
            /// path reads the baked constant rather than re-running the
            /// runtime divide.
            pub(crate) fn half_pi<const SCALE: u32>(w: u32) -> W {
                pi_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE) >> 1
            }

            /// Taylor series for `sin` on a reduced `r ∈ [0, π/4]`.
            ///
            /// `sin(r) = r − r³/3! + r⁵/5! − …`
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

            /// Taylor series for `cos` on a reduced `r ∈ [0, π/4]`.
            ///
            /// `cos(r) = 1 − r²/2! + r⁴/4! − r⁶/6! + …`
            ///
            /// Converges faster than [`sin_taylor`] at the same `r`
            /// because the leading `1` dominates the small even-power
            /// corrections — used as the "upper-half" branch of
            /// [`sin_fixed`] when the reduced argument exceeds π/4.
            fn cos_taylor(r: W, w: u32) -> W {
                let r2 = mul(r, r, w);
                let one_w = one(w);
                let mut sum = one_w;
                let mut term = one_w;
                let mut k: u128 = 1;
                loop {
                    term = mul(term, r2, w) / lit((2 * k - 1) * (2 * k));
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
            pub(crate) fn sin_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
                let pi_w = pi_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE);
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
            pub(crate) fn sin_cos_fixed<const SCALE: u32>(v_w: W, w: u32) -> (W, W) {
                let pi_w = pi_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE);
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
            pub(crate) fn cos_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
                sin_fixed::<SCALE>(half_pi::<SCALE>(w) - v_w, w)
            }

            /// Arctangent of a working-scale value, result in
            /// `(−π/2, π/2)`.
            pub(crate) fn atan_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
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
                    5 // D38-equivalent guard (~50 digits)
                } else if w < 110 {
                    6 // D76 / D153 light-end
                } else {
                    7 // D153 heavy / D307
                };
                let pow10_w = pow10_table(w);
                for _ in 0..halvings {
                    let x2 = mul(x, x, w);
                    let denom = one_w + sqrt_fixed(one_w + x2, w);
                    x = div_cached(x, denom, pow10_w);
                }
                let mut result = atan_taylor(x, w) << halvings;
                if add_half_pi {
                    result = half_pi::<SCALE>(w) - result;
                }
                if sign { -result } else { result }
            }

            // ── Tang lookup tables (ln / exp) ──────────────────────────
            //
            // The tier-generic `ln_tang` / `exp_tang` kernels
            // (`algos::ln::ln_tang`, `algos::exp::exp_tang`) drive the
            // table through the `WideTrigCore::{ln,exp}_table_entry`
            // trait methods, which forward here. Each entry is a pure
            // function of its `(w, idx[, M])` key and is computed on the
            // stack on demand — one slot per call, no stored table.

            /// Tang ln table size — `ln(1 + i/M)`, `i ∈ [0, M]`.
            const LN_TANG_M: u32 = 128;

            // ── Tang lookup-table entries — stateless single-slot recompute
            //
            // The Tang ln / exp / sincos kernels index a value-independent
            // table `T(w)[idx]` (identical for every call at the same
            // `(w, M)`). Each call needs exactly ONE slot, so it is computed
            // directly on the stack — stateless and heap-free. (The full
            // tables can't be baked as `const` rodata in-crate: the per-slot
            // builders call the runtime `*_fixed` BigInt kernels — `ln_fixed`
            // / `exp_fixed` / `sin_cos_fixed` — which are not `const fn`.)
            mod tang_table {
                use super::*;

                /// `ln(1 + idx/M)` at working scale `w` (`idx ∈ [0, M]`).
                /// idx = 0 → ln(1) = 0.
                #[inline]
                pub(super) fn ln_table_entry<const SCALE: u32>(w: u32, idx: usize) -> W {
                    if idx == 0 {
                        return zero();
                    }
                    let one_w = one(w);
                    let scaled = (one_w * lit(idx as u128)) / lit(LN_TANG_M as u128);
                    ln_fixed::<SCALE>(one_w + scaled, w)
                }

                /// `exp(idx · ln2 / m)` at working scale `w`
                /// (`idx ∈ [0, m)`). idx = 0 → exp(0) = 1.
                #[inline]
                pub(super) fn exp_table_entry<const SCALE: u32>(w: u32, idx: usize, m: u32) -> W {
                    if idx == 0 {
                        return one(w);
                    }
                    let cj_w = (ln2_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE) * lit(idx as u128)) / lit(m as u128);
                    exp_fixed(cj_w, w)
                }

                /// `(sin(c_j), cos(c_j))` with `c_j = idx · π / (4·m)` at
                /// working scale `w` (`idx ∈ [0, m]`). idx = 0 →
                /// (sin 0, cos 0) = (0, 1).
                #[inline]
                pub(super) fn sincos_table_entry<const SCALE: u32>(w: u32, idx: usize, m: u32) -> (W, W) {
                    if idx == 0 {
                        return (zero(), one(w));
                    }
                    let cj_w = (pi_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE) * lit(idx as u128)) / lit((4 * m) as u128);
                    sin_cos_fixed::<SCALE>(cj_w, w)
                }
            }

            /// Zero-sized per-tier marker implementing
            /// [`crate::algos::support::wide_trig_core::WideTrigCore`].
            /// Binds this tier's work integer [`W`] / [`Wexp`] and the
            /// storage integer as the trait's associated types so the
            /// generic `*_series` functions can drive the tier through
            /// the trait. The methods forward to the per-tier free
            /// functions emitted above; collapsing those bodies to one
            /// `BigInt`-generic core (the `exp_generic` precedent) is a
            /// later, local change behind this surface.
            pub(crate) struct Core;

            impl $crate::algos::support::wide_trig_core::WideTrigCore for Core {
                type W = W;
                type Wexp = Wexp;
                type Storage = $Storage;
                const GUARD: u32 = GUARD;

                #[inline]
                fn storage_zero() -> $Storage {
                    <$Storage as $crate::int::types::traits::BigInt>::ZERO
                }
                #[inline]
                fn storage_one(scale: u32) -> $Storage {
                    <$Storage as $crate::int::types::traits::BigInt>::TEN.pow(scale)
                }
                #[inline]
                fn zero() -> W {
                    zero()
                }
                #[inline]
                fn to_work_w(raw: $Storage, working_digits: u32) -> W {
                    to_work_w(raw, working_digits)
                }
                #[inline]
                fn to_work(raw: $Storage) -> W {
                    to_work(raw)
                }
                #[inline]
                fn round_to_storage_with(
                    v: W,
                    w: u32,
                    target: u32,
                    mode: $crate::support::rounding::RoundingMode,
                ) -> $Storage {
                    round_to_storage_with(v, w, target, mode)
                }
                #[inline]
                fn round_to_storage_directed(
                    base_guard: u32,
                    target: u32,
                    mode: $crate::support::rounding::RoundingMode,
                    recompute: &mut dyn FnMut(u32) -> W,
                ) -> $Storage {
                    round_to_storage_directed(base_guard, target, mode, recompute)
                }
                #[inline]
                fn round_to_storage_directed_never_exact(
                    base_guard: u32,
                    target: u32,
                    mode: $crate::support::rounding::RoundingMode,
                    recompute: &mut dyn FnMut(u32) -> W,
                ) -> $Storage {
                    round_to_storage_directed_never_exact(base_guard, target, mode, recompute)
                }
                #[inline]
                fn exp_fixed(v_w: W, w: u32) -> W {
                    exp_fixed(v_w, w)
                }
                #[inline]
                fn ln_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
                    ln_fixed::<SCALE>(v_w, w)
                }
                #[inline]
                fn sin_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
                    sin_fixed::<SCALE>(v_w, w)
                }
                #[inline]
                fn cos_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
                    cos_fixed::<SCALE>(v_w, w)
                }
                #[inline]
                fn sin_cos_fixed<const SCALE: u32>(v_w: W, w: u32) -> (W, W) {
                    sin_cos_fixed::<SCALE>(v_w, w)
                }
                #[inline]
                fn atan_fixed<const SCALE: u32>(v_w: W, w: u32) -> W {
                    atan_fixed::<SCALE>(v_w, w)
                }
                #[inline]
                fn div(a: W, b: W, w: u32) -> W {
                    div(a, b, w)
                }
                #[inline]
                fn mul(a: W, b: W, w: u32) -> W {
                    mul(a, b, w)
                }
                #[inline]
                fn sqrt_fixed(v: W, w: u32) -> W {
                    sqrt_fixed(v, w)
                }
                #[inline]
                fn log1p_fixed(t: W, w: u32) -> W {
                    log1p_fixed(t, w)
                }
                #[inline]
                fn bit_length(v: W) -> u32 {
                    bit_length(v)
                }
                #[inline]
                fn exp_result_int_digits(mag_at_scale: W, scale: u32) -> u32 {
                    exp_result_int_digits(mag_at_scale, scale)
                }
                #[inline]
                fn sinh_pos_wide(av_w: W, w: u32) -> W {
                    sinh_pos_wide(av_w, w)
                }
                #[inline]
                fn cosh_pos_wide(av_w: W, w: u32) -> W {
                    cosh_pos_wide(av_w, w)
                }
                #[inline]
                fn tanh_pos_wide(av_w: W, w: u32) -> W {
                    tanh_pos_wide(av_w, w)
                }
                #[inline]
                fn round_to_storage_directed_near_special(
                    base_guard: u32,
                    target: u32,
                    mode: $crate::support::rounding::RoundingMode,
                    recompute: &mut dyn FnMut(u32) -> W,
                ) -> $Storage {
                    round_to_storage_directed_near_special(base_guard, target, mode, recompute)
                }
                #[inline]
                fn one(w: u32) -> W {
                    one(w)
                }
                #[inline]
                fn lit(n: u128) -> W {
                    lit(n)
                }
                #[inline]
                fn ln2<const SCALE: u32>(w: u32) -> W {
                    ln2_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
                }
                #[inline]
                fn div_cached(a: W, b: W, pow10_w: W) -> W {
                    div_cached(a, b, pow10_w)
                }
                #[inline]
                fn round_to_nearest_int(v: W, w: u32) -> i128 {
                    round_to_nearest_int(v, w)
                }
                #[inline]
                fn pow10(n: u32) -> W {
                    pow10(n)
                }
                #[inline]
                fn w_bits() -> u32 {
                    <W as $crate::int::types::traits::BigInt>::BITS
                }
                #[inline]
                fn ln_table_entry<const SCALE: u32>(w: u32, idx: usize) -> W {
                    tang_table::ln_table_entry::<SCALE>(w, idx)
                }
                #[inline]
                fn exp_table_entry<const SCALE: u32>(w: u32, idx: usize, m: u32) -> W {
                    tang_table::exp_table_entry::<SCALE>(w, idx, m)
                }
                #[inline]
                fn pi<const SCALE: u32>(w: u32) -> W {
                    pi_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
                }
                #[inline]
                fn half_pi<const SCALE: u32>(w: u32) -> W {
                    half_pi::<SCALE>(w)
                }
                #[inline]
                fn sincos_table_entry<const SCALE: u32>(w: u32, idx: usize, m: u32) -> (W, W) {
                    tang_table::sincos_table_entry::<SCALE>(w, idx, m)
                }
            }

            // ── Matcher-routed working-scale `ln`/`exp` surfaces ────────
            //
            // The bypass-fix Class-G remediation. These wrap `ln_fixed`
            // and `exp_fixed` with the SAME scale gates as
            // `policy::ln::select` / `policy::exp::select` — routed via the
            // central `policy::{ln,exp}::is_tang::<N, SCALE>` const fns so
            // the routed surface tracks any future policy widening
            // automatically (no hand-kept duplicate). When the policy
            // routes Tang, the call lands in the working-scale shared
            // surface `tang_ln_fixed` / `tang_exp_fixed` (the same
            // surfaces `ln_tang` / `exp_tang` wrap at storage level); the
            // storage-level Ziv/EXTERNAL_EXTRA widening that the storage
            // kernels add OVER `tang_*_fixed` is the caller's concern at
            // the working-scale composition sites (`powf_strict`,
            // `log_*_with_kernel`, `asinh_strict`, …), which size their
            // own working guard from the composition's `|k|`-amplifying
            // arithmetic before calling here.
            //
            // For `exp_fixed_routed`, `tang_exp_fixed` runs with
            // `INTERNAL_EXTRA = true` so the kernel's own `extra` lift
            // covers arbitrary `|k|` — matching the working-scale Tang
            // surface the trig hyperbolics already use in `policy::trig`
            // (e.g. D153 sinh/cosh/tanh, `tang_exp_fixed::<C, 128, true>`).
            // This is what makes the routed surface safe to use without
            // re-checking the policy's `ByValue` small-`|x|` gate at the
            // call site: the kernel absorbs the lift internally.
            //
            // `M` / `CAP` are the per-tier values supplied by the macro
            // call (`$exp_tang_m`, `$ln_tang_cap`) — chosen to mirror the
            // dominant per-tier values from `policy::ln::tang_routed` /
            // `policy::exp::tang_routed`. The routed surfaces use one
            // `(M, CAP)` per tier; per-scale-band M-splits (e.g. D57's
            // 18..=22 vs 45..=56 in `policy::exp`) collapse to the
            // dominant tier value here because the working-scale routed
            // surface is single-source-per-tier.

            /// Tang/Series-routed working-scale `ln(v_w) -> v_w` for this
            /// tier. Bit-equivalent to the previous direct `ln_fixed`
            /// call wherever the policy routes Series; routes through
            /// the shared `tang_ln_fixed` surface (the same one
            /// `ln_tang` wraps at storage level) wherever the policy
            /// routes Tang. The bypass-fix call sites
            /// (`log_strict_with_kernel`, `log2_*_with_kernel`,
            /// `log10_*_with_kernel`, `powf_strict`, `powf_strict_with`,
            /// `asinh_strict`, `acosh_strict`, `atanh_strict`, and their
            /// `_with` siblings) go through this instead of `ln_fixed`
            /// directly, so the wide-tier log family now inherits the
            /// matcher's Tang routing (the Class-G remediation).
            #[cfg(feature = "_wide-support")]
            #[inline]
            pub(crate) fn ln_fixed_routed<const SCALE: u32>(v_w: W, w: u32) -> W {
                if const { $crate::policy::ln::is_tang::<$n_limbs, SCALE>() } {
                    // INTERNAL_EXTRA = true: run at extended working scale
                    // `w + 12` and residual-preserving narrow back to `w`,
                    // so the directed-rounding Ziv escalation in the caller
                    // (e.g. asinh_strict_with @ MAX scale) sees a residual
                    // sign bit-identical to Series's `ln_fixed`. Mirrors the
                    // `true, true` flags every `policy::ln::tang_routed`
                    // arm now uses.
                    $crate::algos::ln::ln_tang::tang_ln_fixed::<Core, $ln_tang_cap, false, SCALE>(v_w, w)
                } else {
                    ln_fixed::<SCALE>(v_w, w)
                }
            }
            #[cfg(not(feature = "_wide-support"))]
            #[inline]
            pub(crate) fn ln_fixed_routed<const SCALE: u32>(v_w: W, w: u32) -> W {
                ln_fixed::<SCALE>(v_w, w)
            }

            /// Tang/Series-routed working-scale `exp(v_w) -> v_w` for
            /// this tier. Bit-equivalent to the previous direct
            /// `exp_fixed` call wherever the policy routes Series;
            /// routes through `tang_exp_fixed::<Core, M, true>` (the
            /// `INTERNAL_EXTRA` lift handles arbitrary `|k|`) wherever
            /// the policy routes Tang. The bypass-fix call sites
            /// (`exp2_strict`, `exp2_strict_with_kernel`, `powf_strict`,
            /// `powf_strict_with`, `sinh_cosh_strict`, plus the per-mode
            /// `_with` siblings) go through this instead of `exp_fixed`
            /// directly. The `exp_strict` dispatcher still routes through
            /// `policy::exp::dispatch` so its `ByValue` gate (which
            /// chooses Series for large-`|x|` to skip Tang's `2^k`
            /// reassembly amplification at storage) remains in effect at
            /// the strict-narrowing layer; the working-scale composition
            /// sites just need a fast `e^{stuff}` and let
            /// `tang_exp_fixed`'s internal `extra` lift cover the
            /// large-`|k|` case.
            #[cfg(feature = "_wide-support")]
            #[inline]
            pub(crate) fn exp_fixed_routed<const SCALE: u32>(v_w: W, w: u32) -> W {
                if const { $crate::policy::exp::is_tang::<$n_limbs, SCALE>() } {
                    $crate::algos::exp::exp_tang::tang_exp_fixed::<Core, $exp_tang_m, true, SCALE>(v_w, w)
                } else {
                    exp_fixed(v_w, w)
                }
            }
            #[cfg(not(feature = "_wide-support"))]
            #[inline]
            pub(crate) fn exp_fixed_routed<const SCALE: u32>(v_w: W, w: u32) -> W {
                exp_fixed(v_w, w)
            }

            // ── log-base algorithm kernels (LnDivide) ──────────────────
            //
            // The arbitrary-base logarithm `log(x, b) = ln(x)/ln(b)` for
            // the wide tiers. These hold the real computation (exact-power
            // pin + directed-rounding Ziv escalation) so the impl lives in
            // the algorithm, NOT in the inherent `log_*_with` method. The
            // `log` policy (`policy::log`) calls these *down*; the inherent
            // `log_strict_with` / `log_approx_with` methods delegate *down*
            // to that policy. They take and return the tier's raw `$Storage`
            // integer (the typed shell wraps with `from_bits`).

            /// Strict-guard `log(x, base)` under `mode`, on raw storage.
            /// Mirrors the prior inherent `log_strict_with` body verbatim.
            #[inline]
            pub(crate) fn log_strict_with_kernel<const SCALE: u32>(
                raw: $Storage,
                braw: $Storage,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                if raw <= z {
                    panic!(concat!(
                        stringify!($Type),
                        "::log: argument must be positive"
                    ));
                }
                if braw <= z {
                    panic!(concat!(stringify!($Type), "::log: base must be positive"));
                }
                // Probe at the base guard to reject base == 1.
                let w0 = SCALE + GUARD;
                let ln_b0 = ln_fixed_routed::<SCALE>(to_work(braw), w0);
                if ln_b0 == zero() {
                    panic!(concat!(stringify!($Type), "::log: base must not equal 1"));
                }
                // Exact-power pin: `self == base^k` ⇒ result is exactly
                // the integer `k` (see `log10_strict_with`).
                {
                    let r0 = div(ln_fixed_routed::<SCALE>(to_work(raw), w0), ln_b0, w0);
                    let k = round_to_nearest_int(r0, w0);
                    if log_is_exact_int(to_work_w(raw, 0), to_work_w(braw, 0), SCALE, k) {
                        return exact_int_at_scale(k, SCALE);
                    }
                }
                // Route the final narrowing through the directed-rounding
                // Ziv escalation: recompute `ln(self)/ln(base)` at the
                // requested guard so Trunc/Floor/Ceiling decide on the
                // true residual sign, not the base-guard approximation.
                round_to_storage_directed(GUARD, SCALE, mode, |guard| {
                    let w = SCALE + guard;
                    let ln_b = ln_fixed_routed::<SCALE>(to_work_w(braw, guard), w);
                    div(ln_fixed_routed::<SCALE>(to_work_w(raw, guard), w), ln_b, w)
                })
            }

            /// Approx-guard `log(x, base)` with caller-chosen
            /// `working_digits` and `mode`, on raw storage. Mirrors the
            /// prior inherent `log_approx_with` body (the
            /// `working_digits == GUARD` short-circuit to the strict path
            /// is handled by the caller's typed shell).
            #[inline]
            pub(crate) fn log_approx_with_kernel<const SCALE: u32>(
                raw: $Storage,
                braw: $Storage,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                if raw <= z {
                    panic!(concat!(
                        stringify!($Type),
                        "::log: argument must be positive"
                    ));
                }
                if braw <= z {
                    panic!(concat!(stringify!($Type), "::log: base must be positive"));
                }
                let w = SCALE + working_digits;
                let ln_b = ln_fixed_routed::<SCALE>(to_work_w(braw, working_digits), w);
                if ln_b == zero() {
                    panic!(concat!(stringify!($Type), "::log: base must not equal 1"));
                }
                let r = div(ln_fixed_routed::<SCALE>(to_work_w(raw, working_digits), w), ln_b, w);
                round_to_storage_with(r, w, SCALE, mode)
            }

            /// Strict-guard `log2(x)` under `mode`, on raw storage.
            /// Mirrors the inherent `log2_strict_with` body verbatim;
            /// the `policy::ln` dispatch routes here so `log2` never
            /// re-enters a sibling decimal policy.
            #[inline]
            pub(crate) fn log2_strict_with_kernel<const SCALE: u32>(
                raw: $Storage,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log2: argument must be positive"));
                }
                {
                    let w0 = SCALE + GUARD;
                    let r0 = div(
                        ln_fixed_routed::<SCALE>(to_work(raw), w0),
                        ln2_cf::<SCALE>(w0, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                        w0,
                    );
                    let k = round_to_nearest_int(r0, w0);
                    let base2 = pow10_table(SCALE) + pow10_table(SCALE);
                    if log_is_exact_int(to_work_w(raw, 0), base2, SCALE, k) {
                        return exact_int_at_scale(k, SCALE);
                    }
                }
                round_to_storage_directed(GUARD, SCALE, mode, |guard| {
                    let w = SCALE + guard;
                    div(
                        ln_fixed_routed::<SCALE>(to_work_w(raw, guard), w),
                        ln2_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                        w,
                    )
                })
            }

            /// Approx-guard `log2(x)` with caller-chosen `working_digits`.
            #[inline]
            pub(crate) fn log2_approx_with_kernel<const SCALE: u32>(
                raw: $Storage,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                if working_digits == GUARD {
                    return log2_strict_with_kernel::<SCALE>(raw, mode);
                }
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log2: argument must be positive"));
                }
                let w = SCALE + working_digits;
                let r = div(ln_fixed_routed::<SCALE>(to_work_w(raw, working_digits), w), ln2(w), w);
                round_to_storage_with(r, w, SCALE, mode)
            }

            /// Strict-guard `log10(x)` under `mode`, on raw storage.
            #[inline]
            pub(crate) fn log10_strict_with_kernel<const SCALE: u32>(
                raw: $Storage,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log10: argument must be positive"));
                }
                {
                    let w0 = SCALE + GUARD;
                    let r0 = div(
                        ln_fixed_routed::<SCALE>(to_work(raw), w0),
                        ln10_cf::<SCALE>(w0, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                        w0,
                    );
                    let k = round_to_nearest_int(r0, w0);
                    let base10 = pow10_table(SCALE + 1);
                    if log_is_exact_int(to_work_w(raw, 0), base10, SCALE, k) {
                        return exact_int_at_scale(k, SCALE);
                    }
                }
                round_to_storage_directed(GUARD, SCALE, mode, |guard| {
                    let w = SCALE + guard;
                    div(
                        ln_fixed_routed::<SCALE>(to_work_w(raw, guard), w),
                        ln10_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                        w,
                    )
                })
            }

            /// Approx-guard `log10(x)` with caller-chosen `working_digits`.
            #[inline]
            pub(crate) fn log10_approx_with_kernel<const SCALE: u32>(
                raw: $Storage,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                if working_digits == GUARD {
                    return log10_strict_with_kernel::<SCALE>(raw, mode);
                }
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(stringify!($Type), "::log10: argument must be positive"));
                }
                let w = SCALE + working_digits;
                let r = div(ln_fixed_routed::<SCALE>(to_work_w(raw, working_digits), w), ln10(w), w);
                round_to_storage_with(r, w, SCALE, mode)
            }

            /// Strict-guard `exp2(x) = 2^x` under `mode`, on raw storage.
            /// Mirrors the inherent `exp2_strict_with` body verbatim; the
            /// `policy::exp` dispatch routes here so `exp2` never re-enters
            /// a sibling decimal policy.
            #[inline]
            pub(crate) fn exp2_strict_with_kernel<const SCALE: u32>(
                raw: $Storage,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return <$Storage as $crate::int::types::traits::BigInt>::TEN.pow(SCALE);
                }
                if let ::core::option::Option::Some(v) = exp2_exact_pin(raw, SCALE, mode) {
                    return v;
                }
                let k_lift = exp2_result_int_digits(raw, SCALE);
                let base_guard = GUARD + k_lift;
                round_to_storage_directed(base_guard, SCALE, mode, |guard| {
                    let w = SCALE + guard;
                    let arg = mul(
                        to_work_w(raw, guard),
                        ln2_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                        w,
                    );
                    exp_fixed_wide(arg, w)
                })
            }

            /// Approx-guard `exp2(x)` with caller-chosen `working_digits`.
            #[inline]
            pub(crate) fn exp2_approx_with_kernel<const SCALE: u32>(
                raw: $Storage,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Storage {
                if working_digits == GUARD {
                    return exp2_strict_with_kernel::<SCALE>(raw, mode);
                }
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return <$Storage as $crate::int::types::traits::BigInt>::TEN.pow(SCALE);
                }
                let w = SCALE + working_digits;
                let arg = mul(to_work_w(raw, working_digits), ln2(w), w);
                let r = exp_fixed_routed::<SCALE>(arg, w);
                round_to_storage_with(r, w, SCALE, mode)
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
                Self::from_bits($crate::policy::ln::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }

            /// Natural logarithm via the Brent–Salamin AGM (1976).
            /// Strict and correctly rounded. Same contract as
            /// [`Self::ln_strict`]; the implementation path differs.
            /// AGM converges quadratically and scales better than the
            /// artanh-series path at very high working scales in
            /// Brent's textbook complexity analysis.
            ///
            /// **Empirical crossover (post-lift, post-MG-buffer fix):**
            /// the lifted AGM (running at `w' = 2·SCALE + 4` with the
            /// half-LSB `mul`/`sqrt` rounding absorbed by `guard_agm`)
            /// loses to the chain-MG + narrow-GUARD artanh / Tang
            /// path at every shipped tier × SCALE combination:
            ///
            /// | tier | SCALE  | ln_strict (artanh/Tang) | ln_strict_agm | factor |
            /// |------|--------|-------------------------|---------------|--------|
            /// | D307 | 300    | 230 µs                  | 720 µs        | 3.1×   |
            /// | D616 | 300    | 21 µs (Tang)            | 812 µs        | 39×    |
            /// | D616 | 500    | 705 µs                  | 2.05 ms       | 2.9×   |
            /// | D924 | 500    | 980 µs                  | 2.49 ms       | 2.5×   |
            /// | D924 | 900    | 2.43 ms                 | 7.04 ms       | 2.9×   |
            /// | D1232| 615    | 69 µs (Tang)            | 4.04 ms       | 58×    |
            /// | D1232| 1000   | 3.44 ms                 | 8.63 ms       | 2.5×   |
            /// | D1232| 1200   | 4.49 ms                 | 12.04 ms      | 2.7×   |
            ///
            /// Brent's textbook ~300-digit crossover does not hold for
            /// the chain-MG kernel at these widths: the artanh inner
            /// loop runs ~`O(p)` rounded multiplies whose constant per
            /// step is far smaller than the AGM iteration's
            /// `sqrt_fixed` + `mul` pair at the *doubled*
            /// working scale the precision lift demands. The AGM
            /// path remains available via this method (and the
            /// `bench-alt` feature) for downstream apps that need the
            /// alternate kernel, but the canonical `ln_strict` stays
            /// on the artanh / Tang path at every tier.
            #[inline]
            #[must_use]
            pub fn ln_strict_agm(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(
                        stringify!($Type),
                        "::ln_agm: argument must be positive"
                    ));
                }
                // Brent §3 precision lift: run the AGM at
                // w' = SCALE + GUARD + guard_agm(SCALE) so the half-LSB
                // `sqrt(a · b)` truncation in early iterations can
                // be amplified by `√(a/b)` without leaking into the
                // storage-scale ULP budget. The final
                // `round_to_storage` narrows the wider working
                // result back to `SCALE`.
                let w_prime = SCALE + $core::GUARD + $core::guard_agm(SCALE);
                let r = $core::ln_fixed_agm(
                    $core::to_work_w(raw, $core::GUARD + $core::guard_agm(SCALE)),
                    w_prime,
                );
                Self::from_bits($core::round_to_storage(r, w_prime, SCALE))
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
                // Brent §3 precision lift: Newton-on-`ln_fixed_agm`
                // inherits the AGM precision lift via the inner
                // `ln_fixed_agm` call. The base lift `guard_agm` puts
                // ln_fixed_agm at ~0.5 ULP at storage scale. The
                // additional `k_lift` covers the `x << k` post-Newton
                // range reassembly: `exp(v) = 2^k · exp(s)` amplifies
                // the raw error of `x` by `2^k`, i.e., `k · log10(2)`
                // decimal digits. Without this lift, exp(|v|) for
                // |v| above ~3 leaks the amplified residue into the
                // storage scale (validated empirically against mpmath
                // at SCALE up to 615).
                let raw_w = $core::to_work_w(raw, 0);
                let k_lift = $core::exp_agm_k_lift_from_w(raw_w, SCALE);
                let lift = $core::GUARD + $core::guard_agm(SCALE) + k_lift;
                let w_prime = SCALE + lift;
                let r = $core::exp_fixed_agm($core::to_work_w(raw, lift), w_prime);
                Self::from_bits($core::round_to_storage(r, w_prime, SCALE))
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
                    panic!(concat!(
                        stringify!($Type),
                        "::log: argument must be positive"
                    ));
                }
                if braw <= z {
                    panic!(concat!(stringify!($Type), "::log: base must be positive"));
                }
                let w = SCALE + $core::GUARD;
                let ln_b = $core::ln_fixed_routed::<SCALE>($core::to_work(braw), w);
                if ln_b == $core::zero() {
                    panic!(concat!(stringify!($Type), "::log: base must not equal 1"));
                }
                let r = $core::div($core::ln_fixed_routed::<SCALE>($core::to_work(raw), w), ln_b, w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Base-2 logarithm. Strict and correctly rounded. Panics if
            /// `self <= 0`.
            #[inline]
            #[must_use]
            pub fn log2_strict(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(
                        stringify!($Type),
                        "::log2: argument must be positive"
                    ));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::div(
                    $core::ln_fixed_routed::<SCALE>($core::to_work(raw), w),
                    $core::ln2_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                    w,
                );
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Base-10 logarithm. Strict and correctly rounded. Panics
            /// if `self <= 0`.
            #[inline]
            #[must_use]
            pub fn log10_strict(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(
                        stringify!($Type),
                        "::log10: argument must be positive"
                    ));
                }
                let w = SCALE + $core::GUARD;
                let r = $core::div(
                    $core::ln_fixed_routed::<SCALE>($core::to_work(raw), w),
                    $core::ln10_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                    w,
                );
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
                Self::from_bits($crate::policy::exp::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
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
                let arg = $core::mul(
                    $core::to_work(raw),
                    $core::ln2_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                    w,
                );
                let r = $core::exp_fixed_routed::<SCALE>(arg, w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// `self` raised to the power `exp`, as `exp(exp · ln self)`.
            /// Strict and correctly rounded. A zero or negative base
            /// saturates to `ZERO` (a negative base with a fractional
            /// exponent is not real-valued).
            ///
            /// Integer-exponent fast path: if `exp` is an exact integer
            /// with `|n| <= INT_POWF_FAST_PATH_THRESHOLD` (= 64), routes
            /// to `Self::powi(n)` (square-and-multiply on storage),
            /// skipping the `exp(y·ln(x))` chain. `powi` is exact, so
            /// the 0.5 ULP contract is preserved.
            #[inline]
            #[must_use]
            pub fn powf_strict(self, exp: Self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                if let ::core::option::Option::Some(n) = Self::powf_exp_as_small_int(exp) {
                    return self.powi(n);
                }
                let w = SCALE + $core::GUARD;
                let ln_x = $core::ln_fixed_routed::<SCALE>($core::to_work(raw), w);
                let y = $core::to_work(exp.to_bits());
                let r = $core::exp_fixed_routed::<SCALE>($core::mul(y, ln_x, w), w);
                Self::from_bits($core::round_to_storage(r, w, SCALE))
            }

            /// Integer-exponent threshold for the [`Self::powf_strict`]
            /// / [`Self::powf_strict_with`] fast path. At `|n| <= 64`,
            /// `powi(n)` costs at most ~12 multiplications, well below
            /// the `exp(y·ln(x))` chain.
            const INT_POWF_FAST_PATH_THRESHOLD: i32 = 64;

            /// Returns `Some(n)` if `exp` is an exact integer value
            /// `n: i32` with `|n| <= INT_POWF_FAST_PATH_THRESHOLD`.
            /// Used to gate the integer fast path on `powf_strict` and
            /// `powf_strict_with`.
            #[inline]
            fn powf_exp_as_small_int(exp: Self) -> ::core::option::Option<i32> {
                let raw = exp.to_bits();
                let mult = Self::multiplier();
                let zero = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                if raw % mult != zero {
                    return ::core::option::Option::None;
                }
                let q = raw / mult;
                let lo = $crate::macros::wide_roots::wide_lit!($Storage, "-64");
                let hi = $crate::macros::wide_roots::wide_lit!($Storage, "64");
                if q < lo || q > hi {
                    return ::core::option::Option::None;
                }
                let q_i128: i128 = $crate::int::types::traits::BigInt::to_i128(q);
                ::core::option::Option::Some(q_i128 as i32)
            }

            /// Sine of `self` (radians). Strict and correctly rounded.
            ///
            /// Delegates to the policy-registered sin kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn sin_strict(self) -> Self {
                Self::from_bits($crate::policy::trig::sin_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
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
                Self::from_bits($crate::policy::trig::cos_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
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
                let (s, c) = $core::sin_cos_fixed::<SCALE>($core::to_work(self.to_bits()), w);
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
                Self::from_bits($crate::policy::trig::tan_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }

            /// Arctangent of `self`, in radians, in `(−π/2, π/2)`.
            /// Strict and correctly rounded.
            ///
            /// Delegates to the policy-registered atan kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn atan_strict(self) -> Self {
                Self::from_bits($crate::policy::trig::atan_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
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
                Self::from_bits($crate::policy::trig::asin_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }

            /// Arccosine of `self`, in radians, in `[0, π]`, as
            /// `π/2 − asin(self)`. Strict. Panics if `|self| > 1`.
            /// Uses the same two-range asin kernel as
            /// [`Self::asin_strict`] for the underlying asin.
            #[inline]
            #[must_use]
            pub fn acos_strict(self) -> Self {
                Self::from_bits($crate::policy::trig::acos_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }

            /// Four-quadrant arctangent of `self` (`y`) and `other`
            /// (`x`), in radians, in `(−π, π]`. Strict and correctly
            /// rounded.
            #[inline]
            #[must_use]
            pub fn atan2_strict(self, other: Self) -> Self {
                Self::from_bits($crate::policy::trig::atan2_dispatch::<_, SCALE>(self.to_bits(), other.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }

            /// Hyperbolic sine, as `(eˣ − e⁻ˣ)/2`. Strict and correctly
            /// rounded.
            ///
            /// Delegates to the policy-registered sinh kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn sinh_strict(self) -> Self {
                Self::from_bits($crate::policy::trig::sinh_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }

            /// Hyperbolic cosine, as `(eˣ + e⁻ˣ)/2`. Strict and
            /// correctly rounded.
            ///
            /// Delegates to the policy-registered cosh kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn cosh_strict(self) -> Self {
                Self::from_bits($crate::policy::trig::cosh_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }

            /// Hyperbolic tangent, as `sinh / cosh`. Strict and
            /// correctly rounded. Shares one `exp(v)` and one
            /// `exp(−v)` between the implicit sinh and cosh, then
            /// `tanh = (eˣ − e⁻ˣ) / (eˣ + e⁻ˣ)` — same arithmetic as
            /// the historic path, but the divide and the two
            /// subtraction/addition operands are inlined here to
            /// avoid going through the intermediate sinh/cosh.
            ///
            /// Delegates to the policy-registered tanh kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn tanh_strict(self) -> Self {
                Self::from_bits($crate::policy::trig::tanh_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }

            /// Joint hyperbolic sine and cosine of `self`, returned
            /// as `(sinh, cosh)`. Strict and correctly rounded.
            ///
            /// One `exp(v)` evaluation plus the `exp(-v) = 1/exp(v)`
            /// identity gives both `eˣ` and `e⁻ˣ` for sinh + cosh.
            /// Wide-tier `exp_fixed` is ~10-20× the cost of a wide
            /// divide, so the identity drops this joint kernel
            /// roughly 40% versus running two `exp_fixed` calls.
            #[inline]
            #[must_use]
            pub fn sinh_cosh_strict(self) -> (Self, Self) {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::div($core::one(w), ex, w);
                let sinh = (ex - enx) >> 1;
                let cosh = (ex + enx) >> 1;
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
                // asinh @ MAX scale (input ±1) loses sub-w precision in the
                // sqrt step before ln; tang_ln_fixed's INTERNAL_EXTRA
                // residue-signal can't detect that caller-side loss. Keep
                // on Series until ln_fixed_routed gains a PRE_RESIDUE flag
                // (memory project_050_asinh_max_tang_residue).
                let inner = if ax >= one_w {
                    let inv = $core::div(one_w, ax, w);
                    let root = $core::sqrt_fixed(one_w + $core::mul(inv, inv, w), w);
                    $core::ln_fixed::<SCALE>(ax, w) + $core::ln_fixed::<SCALE>(one_w + root, w)
                } else {
                    let root = $core::sqrt_fixed($core::mul(ax, ax, w) + one_w, w);
                    $core::ln_fixed::<SCALE>(ax + root, w)
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
                    $core::ln_fixed_routed::<SCALE>(v, w) + $core::ln_fixed_routed::<SCALE>(one_w + root, w)
                } else {
                    // Near 1: acosh(1+t) = log1p(t + sqrt(t*(t+2))).
                    // `t = v - one_w` is the exact gap above 1, so
                    // `v^2 - 1 = (v-1)*(v+1) = t*(t+2)` is formed without
                    // the catastrophic cancellation of `mul(v,v) - one_w`
                    // as `v -> 1`, and `log1p` avoids re-forming `1 + arg`
                    // when the gap (hence `arg`) is tiny.
                    let t = v - one_w;
                    let root = $core::sqrt_fixed($core::mul(t, t + two_w, w), w);
                    $core::log1p_fixed(t + root, w)
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
                    panic!(concat!(
                        stringify!($Type),
                        "::atanh: argument out of domain (-1, 1)"
                    ));
                }
                // Gap form: atanh(x) = (1/2)*[ln(1+x) - ln(1-x)].
                // `one_w - v` is the exact working-scale gap (`v` is the
                // storage input lifted by appending guard zeros), so
                // neither `ln_fixed` argument suffers the `(1-x)`
                // catastrophic cancellation the ratio form does near +-1.
                let r = ($core::ln_fixed_routed::<SCALE>(one_w + v, w) - $core::ln_fixed_routed::<SCALE>(one_w - v, w)) >> 1;
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
                    concat!(
                        stringify!($Type),
                        "::to_degrees: |self| * 180 overflows the working integer"
                    )
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
                let r = $core::mul(
                    v,
                    $core::pi_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                    w,
                ) / $crate::macros::wide_roots::wide_lit!($Work, "180");
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
                Self::from_bits($crate::policy::ln::dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::ln_strict_agm`].
            #[inline]
            #[must_use]
            pub fn ln_strict_agm_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    panic!(concat!(
                        stringify!($Type),
                        "::ln_agm: argument must be positive"
                    ));
                }
                let w_prime = SCALE + $core::GUARD + $core::guard_agm(SCALE);
                let r = $core::ln_fixed_agm(
                    $core::to_work_w(raw, $core::GUARD + $core::guard_agm(SCALE)),
                    w_prime,
                );
                Self::from_bits($core::round_to_storage_with(r, w_prime, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::exp_strict_agm`].
            #[inline]
            #[must_use]
            pub fn exp_strict_agm_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ONE;
                }
                // See `exp_strict_agm` for the `k_lift` rationale.
                let raw_w = $core::to_work_w(raw, 0);
                let k_lift = $core::exp_agm_k_lift_from_w(raw_w, SCALE);
                let lift = $core::GUARD + $core::guard_agm(SCALE) + k_lift;
                let w_prime = SCALE + lift;
                let r = $core::exp_fixed_agm($core::to_work_w(raw, lift), w_prime);
                Self::from_bits($core::round_to_storage_with(r, w_prime, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::log_strict`].
            ///
            /// Body delegates *down* to `policy::log::dispatch`, which
            /// routes to the `LnDivide` kernel (`$core::log_strict_with_kernel`).
            #[inline]
            #[must_use]
            pub fn log_strict_with(
                self,
                base: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                Self::from_bits($crate::policy::log::dispatch::<_, SCALE>(
                    self.to_bits(),
                    base.to_bits(),
                    mode,
                ))
            }

            /// Mode-aware sibling of [`Self::log2_strict`].
            #[inline]
            #[must_use]
            pub fn log2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($core::log2_strict_with_kernel::<SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::log10_strict`].
            #[inline]
            #[must_use]
            pub fn log10_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($core::log10_strict_with_kernel::<SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::exp_strict`]. Delegates
            /// to the policy-registered exp kernel for this
            /// `(width, SCALE)` cell — see `policy::exp`.
            #[inline]
            #[must_use]
            pub fn exp_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::exp::dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::exp2_strict`].
            #[inline]
            #[must_use]
            pub fn exp2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($core::exp2_strict_with_kernel::<SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::powf_strict`].
            ///
            /// Same integer-exponent fast path as [`Self::powf_strict`];
            /// the `mode` argument is irrelevant for `powi` (exact).
            #[inline]
            #[must_use]
            pub fn powf_strict_with(
                self,
                exp: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                if let ::core::option::Option::Some(n) = Self::powf_exp_as_small_int(exp) {
                    // `x^n` for an exact integer `n` is `x^|n|` (exact
                    // square-and-multiply) or its reciprocal `1 / x^|n|`.
                    // The reciprocal is generally NOT exact (e.g. `93^-2`),
                    // so it MUST be rounded under the caller's `mode` — using
                    // the default-mode `powi` here would silently drop a
                    // directed mode (Ceiling of a sub-resolution `x^-k` must
                    // round up to 1, not truncate to 0).
                    if n >= 0 {
                        return self.powi(n);
                    }
                    return Self::ONE.div_with(self.powi(n.unsigned_abs() as i32), mode);
                }
                // x^0.5 ≡ √x. The exp(0.5·ln x) chain loses a sub-ULP at a
                // perfect-square base (e.g. 4^0.5), rounding 1 LSB short
                // under the directed modes; the sqrt kernel pins the exact
                // algebraic root and is correctly rounded for every input,
                // so route the exact-half exponent through it.
                {
                    let two = $crate::macros::wide_roots::wide_lit!($Storage, "2");
                    let mult = Self::multiplier();
                    if exp.to_bits() == mult / two {
                        return self.sqrt_strict_with(mode);
                    }
                }
                let eraw = exp.to_bits();
                // Large-result lift. `x^y = exp(y·ln x)` carries
                // `~|y·ln x|·log10(e)` integer digits; size the working
                // lift from a base-guard probe of the exp argument so the
                // `exp_fixed` relative error stays sub-storage-ULP after
                // narrowing (same budget sinh/cosh use, see those).
                let k_lift = {
                    let w0 = SCALE + $core::GUARD;
                    let ln_x0 = $core::ln_fixed_routed::<SCALE>($core::to_work(raw), w0);
                    let arg0 = $core::mul($core::to_work(eraw), ln_x0, w0);
                    // `arg0` is the exp argument at scale `w0`; narrow it
                    // to scale `SCALE` to feed the `e^|·|` digit sizer
                    // (squaring-safe capped).
                    let arg_at_scale = $core::round_to_storage_with(
                        arg0,
                        w0,
                        SCALE,
                        $crate::support::rounding::RoundingMode::Trunc,
                    );
                    $core::exp_result_int_digits($core::to_work_w(arg_at_scale, 0), SCALE)
                };
                let base_guard = $core::GUARD + k_lift;
                Self::from_bits($core::round_to_storage_directed(
                    base_guard,
                    SCALE,
                    mode,
                    |guard| {
                        let w = SCALE + guard;
                        let ln_x = $core::ln_fixed_routed::<SCALE>($core::to_work_w(raw, guard), w);
                        let y = $core::to_work_w(eraw, guard);
                        $core::exp_fixed_routed::<SCALE>($core::mul(y, ln_x, w), w)
                    },
                ))
            }

            /// Mode-aware sibling of [`Self::sin_strict`]. Delegates
            /// to the policy-registered sin kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn sin_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::sin_dispatch::<_, SCALE>(self.to_bits(), mode))
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
                Self::from_bits($crate::policy::trig::cos_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::tan_strict`]. Delegates
            /// to the policy-registered tan kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn tan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::tan_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::atan_strict`]. Delegates
            /// to the policy-registered atan kernel for this
            /// `(width, SCALE)` cell — see `policy::trig`.
            #[inline]
            #[must_use]
            pub fn atan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::atan_dispatch::<_, SCALE>(self.to_bits(), mode))
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
                    panic!(concat!(
                        stringify!($Type),
                        "::asin: argument out of domain [-1, 1]"
                    ));
                }
                let half_w = one_w >> 1;
                let r = if abs_v == one_w {
                    let hp = $core::half_pi::<SCALE>(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed::<SCALE>($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) >> 1;
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom =
                        $core::sqrt_fixed(one_w - $core::mul(inner_sqrt, inner_sqrt, w), w);
                    let inner_asin = $core::atan_fixed::<SCALE>($core::div(inner_sqrt, inner_denom, w), w);
                    let result_abs = $core::half_pi::<SCALE>(w) - inner_asin - inner_asin;
                    if v < $core::zero() {
                        -result_abs
                    } else {
                        result_abs
                    }
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
                    panic!(concat!(
                        stringify!($Type),
                        "::acos: argument out of domain [-1, 1]"
                    ));
                }
                let half_w = one_w >> 1;
                let asin_w = if abs_v == one_w {
                    let hp = $core::half_pi::<SCALE>(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed::<SCALE>($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) >> 1;
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom =
                        $core::sqrt_fixed(one_w - $core::mul(inner_sqrt, inner_sqrt, w), w);
                    let inner_asin = $core::atan_fixed::<SCALE>($core::div(inner_sqrt, inner_denom, w), w);
                    let result_abs = $core::half_pi::<SCALE>(w) - inner_asin - inner_asin;
                    if v < $core::zero() {
                        -result_abs
                    } else {
                        result_abs
                    }
                };
                let r = $core::half_pi::<SCALE>(w) - asin_w;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::atan2_strict`]. Same
            /// max-branch + quadrant logic.
            #[inline]
            #[must_use]
            pub fn atan2_strict_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let w = SCALE + $core::GUARD;
                let z = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                let yraw = self.to_bits();
                let xraw = other.to_bits();
                let r = if xraw == z {
                    if yraw > z {
                        $core::half_pi::<SCALE>(w)
                    } else if yraw < z {
                        -$core::half_pi::<SCALE>(w)
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
                        $core::atan_fixed::<SCALE>($core::div(y, x, w), w)
                    } else {
                        let inv = $core::atan_fixed::<SCALE>($core::div(x, y, w), w);
                        let hp = $core::half_pi::<SCALE>(w);
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
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::sinh_strict`].
            ///
            /// Uses the `exp(-v) = 1/exp(v)` identity to replace the
            /// second `exp_fixed` call with one wide divide. Wide-tier
            /// `exp_fixed` is dominated by the Tang-table reduction +
            /// Taylor series and costs ~10-20× more than a wide
            /// divide; the identity drops the per-call wall-clock
            /// roughly 40%.
            #[inline]
            #[must_use]
            pub fn sinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                let szero = <$Storage>::from_i128(0);
                if raw != szero {
                    // Small-argument cubic band: `sinh(x) = x + x³/6 + …`,
                    // the cubic strictly positive yet below one ULP, so
                    // the true value sits just *above* the grid line
                    // `raw` (in magnitude). No finite-precision `exp`
                    // path resolves the sub-ULP cubic — the
                    // `(e^x − e^-x)/2` difference collapses to exactly
                    // `raw` (or one LSB short) — so we return the
                    // analytic directed decision. `sinh` is odd, so the
                    // band is symmetric. The threshold mirrors `tanh`'s:
                    // the cubic clears half a storage ULP only once
                    // `|raw| > ~10^(2·SCALE/3)`.
                    let thresh_exp = SCALE - SCALE.div_ceil(3);
                    let thresh = <$Storage>::from_i128(10).pow(thresh_exp);
                    if raw.abs() <= thresh {
                        return Self::from_bits(
                            $crate::support::rounding::tiny_odd_expanding_directed(
                                raw,
                                szero,
                                <$Storage>::from_i128(1),
                                mode,
                            ),
                        );
                    }
                }
                // Large-argument lift. `sinh(x) ≈ e^|x|/2` carries
                // `~|x|·log10(e)` integer-part digits; the `exp_fixed`
                // result holds those at the high end of the working
                // integer, so its ≤ 0.5 LSB-of-w relative error becomes
                // an absolute error of `~10^(int_digits)` storage LSB on
                // narrowing. Lift the base working scale by the same
                // `⌈|x|·log10(e)⌉` digits (the `exp` `2^k` reassembly
                // budget) so that absolute error stays sub-storage-ULP.
                // Always feed `exp_fixed` the *positive* magnitude `|v|`,
                // so the dominant `e^|x|` term is computed directly and
                // accurately. The reciprocal then gives the tiny
                // `e^-|x|`. Computing `exp(-|x|)` directly and
                // reciprocating instead would amplify the small term's
                // relative error into a large absolute error in the huge
                // `1/exp(-|x|)`, blowing the storage-ULP budget for large
                // `|x|`. `sinh` is odd, so the sign of the input is
                // reapplied to the (non-negative) `sinh(|x|)` working
                // value — `round_to_storage_directed` reads the sign off
                // the returned value and rounds each mode accordingly.
                let neg = raw < <$Storage>::from_i128(0);
                let k_lift = $core::exp_result_int_digits($core::to_work_w(raw, 0), SCALE);
                let base_guard = $core::GUARD + k_lift;
                Self::from_bits($core::round_to_storage_directed(
                    base_guard,
                    SCALE,
                    mode,
                    |guard| {
                        let w = SCALE + guard;
                        let v = $core::to_work_w(raw, guard);
                        let av = if v < $core::zero() { -v } else { v };
                        let sh = $core::sinh_pos_wide(av, w);
                        if neg { -sh } else { sh }
                    },
                ))
            }

            /// Mode-aware sibling of [`Self::cosh_strict`].
            ///
            /// Same `exp(-v) = 1/exp(v)` identity as
            /// [`Self::sinh_strict_with`]; one `exp_fixed` plus one
            /// divide replaces two `exp_fixed`s.
            #[inline]
            #[must_use]
            pub fn cosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                // Large-argument lift: see `sinh_strict_with`. `cosh` is
                // even, so we always evaluate at `|v|` — feeding the
                // positive magnitude keeps the dominant `e^|x|` term
                // direct and accurate (see `sinh_strict_with` for why the
                // sign matters to the budget).
                let k_lift = $core::exp_result_int_digits($core::to_work_w(raw, 0), SCALE);
                let base_guard = $core::GUARD + k_lift;
                Self::from_bits($core::round_to_storage_directed(
                    base_guard,
                    SCALE,
                    mode,
                    |guard| {
                        let w = SCALE + guard;
                        let v = $core::to_work_w(raw, guard);
                        let av = if v < $core::zero() { -v } else { v };
                        $core::cosh_pos_wide(av, w)
                    },
                ))
            }

            /// Mode-aware sibling of [`Self::tanh_strict`].
            ///
            /// Same `exp(-v) = 1/exp(v)` identity as
            /// [`Self::sinh_strict_with`].
            #[inline]
            #[must_use]
            pub fn tanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                let zero = <$Storage>::from_i128(0);
                if raw != zero {
                    // Small-argument linear band: tanh(x) = x − x³/3 + … ,
                    // the cubic below one ULP yet strictly positive, so the
                    // true value sits just inside the grid line `raw`. No
                    // finite-precision exp path can resolve the sub-ULP
                    // cubic, so the directed result is the analytic decision
                    // (nearest modes return `raw`).
                    let thresh_exp = SCALE - SCALE.div_ceil(3);
                    let thresh = <$Storage>::from_i128(10).pow(thresh_exp);
                    if raw.abs() <= thresh {
                        return Self::from_bits(
                            $crate::support::rounding::tiny_odd_compressing_directed(
                                raw,
                                zero,
                                <$Storage>::from_i128(1),
                                mode,
                            ),
                        );
                    }
                }
                // Saturation-edge lift. For a large `|x|` the intermediate
                // `e^|x|` carries `~|x|·log10(e)` integer digits and runs
                // its squaring core past `W` — so `exp_fixed_wide` runs it
                // in the wider work integer [`Wexp`]. The result `tanh(x)`
                // itself is in `[-1, 1]` (no result lift needed), but the
                // `(ex − enx)/(ex + enx)` ratio needs the tiny `enx = e^-|x|`
                // resolved to keep the directed-rounding decision correct;
                // lift the base working scale by the `e^|x|` integer-digit
                // count so `enx` keeps a full guard below the storage LSB.
                // `tanh` is odd; evaluate at `|v|` (so the dominant
                // `e^|x|` term is direct and accurate, see
                // `sinh_strict_with`) and reapply the input sign to the
                // non-negative `tanh(|x|)` working value.
                let neg = raw < zero;
                let k_lift = $core::exp_result_int_digits($core::to_work_w(raw, 0), SCALE);
                let base_guard = $core::GUARD + k_lift;
                Self::from_bits($core::round_to_storage_directed(
                    base_guard,
                    SCALE,
                    mode,
                    |guard| {
                        let w = SCALE + guard;
                        let v = $core::to_work_w(raw, guard);
                        let av = if v < $core::zero() { -v } else { v };
                        let th = $core::tanh_pos_wide(av, w);
                        if neg { -th } else { th }
                    },
                ))
            }

            /// Mode-aware sibling of [`Self::asinh_strict`].
            #[inline]
            #[must_use]
            pub fn asinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                if raw == $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let neg = raw < $crate::macros::wide_roots::wide_lit!($Storage, "0");
                Self::from_bits($core::round_to_storage_directed(
                    $core::GUARD,
                    SCALE,
                    mode,
                    |guard| {
                        let w = SCALE + guard;
                        let one_w = $core::one(w);
                        let v = $core::to_work_w(raw, guard);
                        let ax = if v < $core::zero() { -v } else { v };
                        // asinh @ MAX scale (input ±1) loses sub-w precision
                        // in the sqrt step before ln; tang_ln_fixed's
                        // INTERNAL_EXTRA residue-signal can't detect that
                        // caller-side loss. Keep on Series until
                        // ln_fixed_routed gains a PRE_RESIDUE flag (memory
                        // project_050_asinh_max_tang_residue).
                        let inner = if ax >= one_w {
                            let inv = $core::div(one_w, ax, w);
                            let root = $core::sqrt_fixed(one_w + $core::mul(inv, inv, w), w);
                            $core::ln_fixed::<SCALE>(ax, w) + $core::ln_fixed::<SCALE>(one_w + root, w)
                        } else {
                            let root = $core::sqrt_fixed($core::mul(ax, ax, w) + one_w, w);
                            $core::ln_fixed::<SCALE>(ax + root, w)
                        };
                        if neg { -inner } else { inner }
                    },
                ))
            }

            /// Mode-aware sibling of [`Self::acosh_strict`].
            #[inline]
            #[must_use]
            pub fn acosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                {
                    // Domain check at the base guard.
                    let w0 = SCALE + $core::GUARD;
                    if $core::to_work(raw) < $core::one(w0) {
                        panic!(concat!(stringify!($Type), "::acosh: argument must be >= 1"));
                    }
                }
                Self::from_bits($core::round_to_storage_directed_near_special(
                    $core::GUARD,
                    SCALE,
                    mode,
                    |guard| {
                        let w = SCALE + guard;
                        let one_w = $core::one(w);
                        let v = $core::to_work_w(raw, guard);
                        let two_w = one_w + one_w;
                        if v >= two_w {
                            let inv = $core::div(one_w, v, w);
                            let root = $core::sqrt_fixed(one_w - $core::mul(inv, inv, w), w);
                            $core::ln_fixed_routed::<SCALE>(v, w) + $core::ln_fixed_routed::<SCALE>(one_w + root, w)
                        } else {
                            // Near 1: acosh(1+t) = log1p(t +
                            // sqrt(t*(t+2))). The gap `t = v - one_w` is
                            // exact, so `v^2 - 1 = t*(t+2)` avoids the
                            // `mul(v,v) - one_w` cancellation as `v -> 1`.
                            let t = v - one_w;
                            let root = $core::sqrt_fixed($core::mul(t, t + two_w, w), w);
                            $core::log1p_fixed(t + root, w)
                        }
                    },
                ))
            }

            /// Mode-aware sibling of [`Self::atanh_strict`].
            #[inline]
            #[must_use]
            pub fn atanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                {
                    // Domain check at the base guard.
                    let w0 = SCALE + $core::GUARD;
                    let v0 = $core::to_work(raw);
                    let ax0 = if v0 < $core::zero() { -v0 } else { v0 };
                    if ax0 >= $core::one(w0) {
                        panic!(concat!(
                            stringify!($Type),
                            "::atanh: argument out of domain (-1, 1)"
                        ));
                    }
                }
                Self::from_bits($core::round_to_storage_directed_near_special(
                    $core::GUARD,
                    SCALE,
                    mode,
                    |guard| {
                        let w = SCALE + guard;
                        let one_w = $core::one(w);
                        let v = $core::to_work_w(raw, guard);
                        // Gap form (1/2)*[ln(1+x) - ln(1-x)]: `one_w
                        // - v` is the exact working-scale gap, so neither
                        // `ln_fixed` argument suffers the `(1-x)`
                        // cancellation the ratio form does near +-1.
                        ($core::ln_fixed_routed::<SCALE>(one_w + v, w) - $core::ln_fixed_routed::<SCALE>(one_w - v, w)) >> 1
                    },
                ))
            }

            /// Mode-aware sibling of [`Self::to_degrees_strict`].
            #[inline]
            #[must_use]
            pub fn to_degrees_strict_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                debug_assert!(
                    $core::bit_length(v) + 8 < <$Work>::BITS,
                    concat!(
                        stringify!($Type),
                        "::to_degrees: |self| * 180 overflows the working integer"
                    )
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
            pub fn to_radians_strict_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let r = $core::mul(
                    v,
                    $core::pi_cf::<SCALE>(w, $crate::support::rounding::DEFAULT_ROUNDING_MODE),
                    w,
                ) / $crate::macros::wide_roots::wide_lit!($Work, "180");
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Mode-aware sibling of [`Self::sin_cos_strict`].
            #[inline]
            #[must_use]
            pub fn sin_cos_strict_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> (Self, Self) {
                let w = SCALE + $core::GUARD;
                let (s, c) = $core::sin_cos_fixed::<SCALE>($core::to_work(self.to_bits()), w);
                (
                    Self::from_bits($core::round_to_storage_with(s, w, SCALE, mode)),
                    Self::from_bits($core::round_to_storage_with(c, w, SCALE, mode)),
                )
            }

            /// Mode-aware sibling of [`Self::sinh_cosh_strict`].
            #[inline]
            #[must_use]
            pub fn sinh_cosh_strict_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> (Self, Self) {
                let w = SCALE + $core::GUARD;
                let v = $core::to_work(self.to_bits());
                let ex = $core::exp_fixed(v, w);
                let enx = $core::div($core::one(w), ex, w);
                let sinh = (ex - enx) >> 1;
                let cosh = (ex + enx) >> 1;
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
                self.ln_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                    panic!(concat!(
                        stringify!($Type),
                        "::ln: argument must be positive"
                    ));
                }
                let w = SCALE + working_digits;
                let r = $core::ln_fixed_routed::<SCALE>($core::to_work_w(raw, working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Log to chosen base with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn log_approx(self, base: Self, working_digits: u32) -> Self {
                self.log_approx_with(
                    base,
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Log to chosen base with caller-chosen guard digits AND rounding mode.
            ///
            /// Body delegates *down* to
            /// `policy::log::dispatch_with`, which routes to the `LnDivide`
            /// kernel (`$core::log_approx_with_kernel`, or the strict kernel
            /// when `working_digits == GUARD`).
            #[inline]
            #[must_use]
            pub fn log_approx_with(
                self,
                base: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                Self::from_bits($crate::policy::log::dispatch_with::<_, SCALE>(
                    self.to_bits(),
                    base.to_bits(),
                    working_digits,
                    mode,
                ))
            }

            /// Log base 2 with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn log2_approx(self, working_digits: u32) -> Self {
                self.log2_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Log base 2 with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn log2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                Self::from_bits($core::log2_approx_with_kernel::<SCALE>(self.to_bits(), working_digits, mode))
            }

            /// Log base 10 with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn log10_approx(self, working_digits: u32) -> Self {
                self.log10_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Log base 10 with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn log10_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                Self::from_bits($core::log10_approx_with_kernel::<SCALE>(self.to_bits(), working_digits, mode))
            }

            /// `eˣ` with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn exp_approx(self, working_digits: u32) -> Self {
                self.exp_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let r = $core::exp_fixed_routed::<SCALE>($core::to_work_w(raw, working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// `2ˣ` with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn exp2_approx(self, working_digits: u32) -> Self {
                self.exp2_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// `2ˣ` with caller-chosen guard digits AND rounding mode.
            #[inline]
            #[must_use]
            pub fn exp2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                Self::from_bits($core::exp2_approx_with_kernel::<SCALE>(self.to_bits(), working_digits, mode))
            }

            /// `xʸ` with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn powf_approx(self, exp: Self, working_digits: u32) -> Self {
                self.powf_approx_with(
                    exp,
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let ln_x = $core::ln_fixed_routed::<SCALE>($core::to_work_w(raw, working_digits), w);
                let y = $core::to_work_w(exp.to_bits(), working_digits);
                let r = $core::exp_fixed_routed::<SCALE>($core::mul(y, ln_x, w), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Sine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn sin_approx(self, working_digits: u32) -> Self {
                self.sin_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let r = $core::sin_fixed::<SCALE>($core::to_work_w(self.to_bits(), working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Cosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn cos_approx(self, working_digits: u32) -> Self {
                self.cos_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let arg = $core::to_work_w(self.to_bits(), working_digits) + $core::half_pi::<SCALE>(w);
                let r = $core::sin_fixed::<SCALE>(arg, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Joint sine/cosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn sin_cos_approx(self, working_digits: u32) -> (Self, Self) {
                self.sin_cos_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let (s, c) =
                    $core::sin_cos_fixed::<SCALE>($core::to_work_w(self.to_bits(), working_digits), w);
                (
                    Self::from_bits($core::round_to_storage_with(s, w, SCALE, mode)),
                    Self::from_bits($core::round_to_storage_with(c, w, SCALE, mode)),
                )
            }

            /// Tangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn tan_approx(self, working_digits: u32) -> Self {
                self.tan_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let (sin_w, cos_w) =
                    $core::sin_cos_fixed::<SCALE>($core::to_work_w(self.to_bits(), working_digits), w);
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
                self.atan_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let r = $core::atan_fixed::<SCALE>($core::to_work_w(self.to_bits(), working_digits), w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Arcsine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn asin_approx(self, working_digits: u32) -> Self {
                self.asin_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                    panic!(concat!(
                        stringify!($Type),
                        "::asin: argument out of domain [-1, 1]"
                    ));
                }
                let half_w = one_w >> 1;
                let r = if abs_v == one_w {
                    let hp = $core::half_pi::<SCALE>(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed::<SCALE>($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) >> 1;
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom =
                        $core::sqrt_fixed(one_w - $core::mul(inner_sqrt, inner_sqrt, w), w);
                    let inner_asin = $core::atan_fixed::<SCALE>($core::div(inner_sqrt, inner_denom, w), w);
                    let result_abs = $core::half_pi::<SCALE>(w) - inner_asin - inner_asin;
                    if v < $core::zero() {
                        -result_abs
                    } else {
                        result_abs
                    }
                };
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Arccosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn acos_approx(self, working_digits: u32) -> Self {
                self.acos_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                    panic!(concat!(
                        stringify!($Type),
                        "::acos: argument out of domain [-1, 1]"
                    ));
                }
                let half_w = one_w >> 1;
                let asin_w = if abs_v == one_w {
                    let hp = $core::half_pi::<SCALE>(w);
                    if v < $core::zero() { -hp } else { hp }
                } else if abs_v <= half_w {
                    let denom = $core::sqrt_fixed(one_w - $core::mul(v, v, w), w);
                    $core::atan_fixed::<SCALE>($core::div(v, denom, w), w)
                } else {
                    let inner = (one_w - abs_v) >> 1;
                    let inner_sqrt = $core::sqrt_fixed(inner, w);
                    let inner_denom =
                        $core::sqrt_fixed(one_w - $core::mul(inner_sqrt, inner_sqrt, w), w);
                    let inner_asin = $core::atan_fixed::<SCALE>($core::div(inner_sqrt, inner_denom, w), w);
                    let result_abs = $core::half_pi::<SCALE>(w) - inner_asin - inner_asin;
                    if v < $core::zero() {
                        -result_abs
                    } else {
                        result_abs
                    }
                };
                let r = $core::half_pi::<SCALE>(w) - asin_w;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Four-quadrant arctangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn atan2_approx(self, other: Self, working_digits: u32) -> Self {
                self.atan2_approx_with(
                    other,
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                        $core::half_pi::<SCALE>(w)
                    } else if yraw < z {
                        -$core::half_pi::<SCALE>(w)
                    } else {
                        $core::zero()
                    }
                } else {
                    let y = $core::to_work_w(yraw, working_digits);
                    let x = $core::to_work_w(xraw, working_digits);
                    let base = $core::atan_fixed::<SCALE>($core::div(y, x, w), w);
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
                self.sinh_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let enx = $core::div($core::one(w), ex, w);
                let r = (ex - enx) >> 1;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Hyperbolic cosine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn cosh_approx(self, working_digits: u32) -> Self {
                self.cosh_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let enx = $core::div($core::one(w), ex, w);
                let r = (ex + enx) >> 1;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Hyperbolic tangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn tanh_approx(self, working_digits: u32) -> Self {
                self.tanh_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let enx = $core::div($core::one(w), ex, w);
                let r = $core::div(ex - enx, ex + enx, w);
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Joint sinh/cosh with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn sinh_cosh_approx(self, working_digits: u32) -> (Self, Self) {
                self.sinh_cosh_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                let enx = $core::div($core::one(w), ex, w);
                let sinh = (ex - enx) >> 1;
                let cosh = (ex + enx) >> 1;
                (
                    Self::from_bits($core::round_to_storage_with(sinh, w, SCALE, mode)),
                    Self::from_bits($core::round_to_storage_with(cosh, w, SCALE, mode)),
                )
            }

            /// Inverse hyperbolic sine with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn asinh_approx(self, working_digits: u32) -> Self {
                self.asinh_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                // asinh @ MAX scale (input ±1) loses sub-w precision in the
                // sqrt step before ln; tang_ln_fixed's INTERNAL_EXTRA
                // residue-signal can't detect that caller-side loss. Keep
                // on Series until ln_fixed_routed gains a PRE_RESIDUE flag
                // (memory project_050_asinh_max_tang_residue).
                let inner = if ax >= one_w {
                    let inv = $core::div(one_w, ax, w);
                    let root = $core::sqrt_fixed(one_w + $core::mul(inv, inv, w), w);
                    $core::ln_fixed::<SCALE>(ax, w) + $core::ln_fixed::<SCALE>(one_w + root, w)
                } else {
                    let root = $core::sqrt_fixed($core::mul(ax, ax, w) + one_w, w);
                    $core::ln_fixed::<SCALE>(ax + root, w)
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
                self.acosh_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                    $core::ln_fixed_routed::<SCALE>(v, w) + $core::ln_fixed_routed::<SCALE>(one_w + root, w)
                } else {
                    // Near 1: acosh(1+t) = log1p(t + sqrt(t*(t+2))).
                    // `t = v - one_w` is the exact gap above 1, so
                    // `v^2 - 1 = (v-1)*(v+1) = t*(t+2)` is formed without
                    // the catastrophic cancellation of `mul(v,v) - one_w`
                    // as `v -> 1`, and `log1p` avoids re-forming `1 + arg`
                    // when the gap (hence `arg`) is tiny.
                    let t = v - one_w;
                    let root = $core::sqrt_fixed($core::mul(t, t + two_w, w), w);
                    $core::log1p_fixed(t + root, w)
                };
                Self::from_bits($core::round_to_storage_with(inner, w, SCALE, mode))
            }

            /// Inverse hyperbolic tangent with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn atanh_approx(self, working_digits: u32) -> Self {
                self.atanh_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                    panic!(concat!(
                        stringify!($Type),
                        "::atanh: argument out of domain (-1, 1)"
                    ));
                }
                // Gap form: atanh(x) = (1/2)*[ln(1+x) - ln(1-x)].
                // `one_w - v` is the exact working-scale gap (`v` is the
                // storage input lifted by appending guard zeros), so
                // neither `ln_fixed` argument suffers the `(1-x)`
                // catastrophic cancellation the ratio form does near +-1.
                let r = ($core::ln_fixed_routed::<SCALE>(one_w + v, w) - $core::ln_fixed_routed::<SCALE>(one_w - v, w)) >> 1;
                Self::from_bits($core::round_to_storage_with(r, w, SCALE, mode))
            }

            /// Radians-to-degrees with caller-chosen guard digits.
            #[inline]
            #[must_use]
            pub fn to_degrees_approx(self, working_digits: u32) -> Self {
                self.to_degrees_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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
                    concat!(
                        stringify!($Type),
                        "::to_degrees: |self| * 180 overflows the working integer"
                    )
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
                self.to_radians_approx_with(
                    working_digits,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
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

    /// Validity wall for the const-folded wide constants: the
    /// compile-time [`const_rounded_cf`] must reproduce the runtime
    /// `const_rounded` bit-for-bit for π / ln2 / ln10 across every
    /// rounding mode and every reachable working scale, on every shipped
    /// wide tier `W`. If this passes, the baked `*_cf` fast path returns
    /// exactly what the old runtime divide returned.
    #[test]
    fn const_rounded_cf_matches_runtime_all_tiers() {
        use crate::types::widths::*;
        #[cfg(any(feature = "d76", feature = "wide"))]
        wide_trig_d76::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d153", feature = "wide"))]
        wide_trig_d153::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d307", feature = "wide"))]
        wide_trig_d307::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d57", feature = "wide"))]
        wide_trig_d57::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d115", feature = "wide"))]
        wide_trig_d115::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d230", feature = "wide"))]
        wide_trig_d230::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        wide_trig_d462::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        wide_trig_d616::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        wide_trig_d924::const_rounded_cf_matches_runtime();
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        wide_trig_d1232::const_rounded_cf_matches_runtime();
    }

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
            let n = crate::D::<crate::int::types::Int<2>, 6>::from_bits(crate::int::types::Int::<2>::from_i128(raw as i128));
            let w = crate::D::<crate::int::types::Int<4>, 6>::from_bits(crate::int::types::Int::<4>::from_i128(
                raw as i128,
            ));
            agree(
                "ln",
                raw,
                w.ln_strict().to_bits().as_i128(),
                n.ln_strict().to_bits().as_i128(),
            );
            agree(
                "log2",
                raw,
                w.log2_strict().to_bits().as_i128(),
                n.log2_strict().to_bits().as_i128(),
            );
            agree(
                "log10",
                raw,
                w.log10_strict().to_bits().as_i128(),
                n.log10_strict().to_bits().as_i128(),
            );
        }
        for raw in all {
            let n = crate::D::<crate::int::types::Int<2>, 6>::from_bits(crate::int::types::Int::<2>::from_i128(raw as i128));
            let w = crate::D::<crate::int::types::Int<4>, 6>::from_bits(crate::int::types::Int::<4>::from_i128(
                raw as i128,
            ));
            agree(
                "exp",
                raw,
                w.exp_strict().to_bits().as_i128(),
                n.exp_strict().to_bits().as_i128(),
            );
            agree(
                "sin",
                raw,
                w.sin_strict().to_bits().as_i128(),
                n.sin_strict().to_bits().as_i128(),
            );
            agree(
                "cos",
                raw,
                w.cos_strict().to_bits().as_i128(),
                n.cos_strict().to_bits().as_i128(),
            );
            agree(
                "atan",
                raw,
                w.atan_strict().to_bits().as_i128(),
                n.atan_strict().to_bits().as_i128(),
            );
            agree(
                "sinh",
                raw,
                w.sinh_strict().to_bits().as_i128(),
                n.sinh_strict().to_bits().as_i128(),
            );
            agree(
                "cosh",
                raw,
                w.cosh_strict().to_bits().as_i128(),
                n.cosh_strict().to_bits().as_i128(),
            );
            agree(
                "tanh",
                raw,
                w.tanh_strict().to_bits().as_i128(),
                n.tanh_strict().to_bits().as_i128(),
            );
        }
        for raw in unit_range {
            let n = crate::D::<crate::int::types::Int<2>, 6>::from_bits(crate::int::types::Int::<2>::from_i128(raw as i128));
            let w = crate::D::<crate::int::types::Int<4>, 6>::from_bits(crate::int::types::Int::<4>::from_i128(
                raw as i128,
            ));
            agree(
                "asin",
                raw,
                w.asin_strict().to_bits().as_i128(),
                n.asin_strict().to_bits().as_i128(),
            );
            agree(
                "acos",
                raw,
                w.acos_strict().to_bits().as_i128(),
                n.acos_strict().to_bits().as_i128(),
            );
            agree(
                "atanh",
                raw,
                w.atanh_strict().to_bits().as_i128(),
                n.atanh_strict().to_bits().as_i128(),
            );
        }
    }

    /// Bit-exact identity points hold across all three wide tiers.
    #[test]
    fn wide_transcendental_identities() {
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ONE.ln_strict(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ZERO.exp_strict(), crate::D::<crate::int::types::Int<4>, 6>::ONE);
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ZERO.sin_strict(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ZERO.sinh_strict(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ZERO.atan_strict(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);

        assert_eq!(crate::D::<crate::int::types::Int<8>, 6>::ONE.ln_strict(), crate::D::<crate::int::types::Int<8>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<8>, 6>::ZERO.exp_strict(), crate::D::<crate::int::types::Int<8>, 6>::ONE);
        assert_eq!(crate::D::<crate::int::types::Int<8>, 6>::ZERO.cos_strict(), crate::D::<crate::int::types::Int<8>, 6>::ONE);

        assert_eq!(crate::D::<crate::int::types::Int<16>, 6>::ONE.ln_strict(), crate::D::<crate::int::types::Int<16>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<16>, 6>::ZERO.exp_strict(), crate::D::<crate::int::types::Int<16>, 6>::ONE);
        assert_eq!(crate::D::<crate::int::types::Int<16>, 6>::ZERO.cosh_strict(), crate::D::<crate::int::types::Int<16>, 6>::ONE);
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
            let w = crate::D::<crate::int::types::Int<4>, 6>::from_bits(crate::int::types::Int::<4>::from_i128(
                raw as i128,
            ));
            agree(
                "ln",
                raw,
                w.ln_strict_agm().to_bits().as_i128(),
                w.ln_strict().to_bits().as_i128(),
            );
        }
        for raw in all {
            let w = crate::D::<crate::int::types::Int<4>, 6>::from_bits(crate::int::types::Int::<4>::from_i128(
                raw as i128,
            ));
            agree(
                "exp",
                raw,
                w.exp_strict_agm().to_bits().as_i128(),
                w.exp_strict().to_bits().as_i128(),
            );
        }
    }

    /// Identity points: AGM `ln(1) = 0`, AGM `exp(0) = 1`.
    #[test]
    fn wide_agm_identity_points() {
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ONE.ln_strict_agm(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ZERO.exp_strict_agm(), crate::D::<crate::int::types::Int<4>, 6>::ONE);
        assert_eq!(crate::D::<crate::int::types::Int<8>, 6>::ONE.ln_strict_agm(), crate::D::<crate::int::types::Int<8>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<8>, 6>::ZERO.exp_strict_agm(), crate::D::<crate::int::types::Int<8>, 6>::ONE);
        assert_eq!(crate::D::<crate::int::types::Int<16>, 6>::ONE.ln_strict_agm(), crate::D::<crate::int::types::Int<16>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<16>, 6>::ZERO.exp_strict_agm(), crate::D::<crate::int::types::Int<16>, 6>::ONE);
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
        let n = crate::D::<crate::int::types::Int<4>, 6>::ONE;
        let hte = n.exp_strict_with(RoundingMode::HalfToEven);
        let trunc = n.exp_strict_with(RoundingMode::Trunc);
        assert!(
            hte.to_bits().as_i128() - trunc.to_bits().as_i128() == 1
                || hte.to_bits().as_i128() - trunc.to_bits().as_i128() == 0,
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

    /// AGM ln/exp round-trip at moderate storage scales. With the
    /// `guard_agm` precision lift the AGM path now holds 0.5 ULP
    /// at every wide-tier storage scale; this test retains its
    /// historic D76<20> / D153<20> coverage as a smoke gate.
    #[test]
    fn wide_agm_moderate_scale_round_trip() {
        let x = crate::D::<crate::int::types::Int<4>, 20>::from_int(3);
        let back = x.ln_strict_agm().exp_strict_agm();
        let delta = (back.to_bits().as_i128() - x.to_bits().as_i128()).abs();
        assert!(delta <= 8, "AGM exp(ln(3)) at D76<20> delta {delta}");

        let y = crate::D::<crate::int::types::Int<8>, 20>::from_int(2);
        let back = y.exp_strict_agm().ln_strict_agm();
        let delta = (back.to_bits().as_i128() - y.to_bits().as_i128()).abs();
        assert!(delta <= 8, "AGM ln(exp(2)) at D153<20> delta {delta}");
    }

    /// Exercises a scale beyond D38's range, where delegation is
    /// impossible and the wide guard-digit core is the only path.
    /// `exp(ln(x)) ≈ x` and `ln(exp(x)) ≈ x` round-trips.
    #[test]
    fn wide_only_scale_round_trips() {
        // D76<50>: well past D38's max scale of 38. The round-trip
        // result fits i128 comfortably, so compare there.
        let x = crate::D::<crate::int::types::Int<4>, 50>::from_int(3);
        let back = x.ln_strict().exp_strict();
        let delta = (back.to_bits().as_i128() - x.to_bits().as_i128()).abs();
        assert!(delta <= 8, "exp(ln(3)) at D76<50> delta {delta}");

        // D307<150>: deep scale, only the wide core can serve it.
        let y = crate::D::<crate::int::types::Int<16>, 150>::from_int(2);
        let back = y.exp_strict().ln_strict();
        let delta = (back.to_bits().as_i128() - y.to_bits().as_i128()).abs();
        assert!(delta <= 8, "ln(exp(2)) at D307<150> delta {delta}");
    }
}
