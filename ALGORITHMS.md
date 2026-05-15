# Algorithms used in `decimal-scaled`

Catalogue of the published algorithms the crate evaluates, with
academic citations and the source files where each is implemented.
This is engineering credit — it complements `LICENSE-THIRD-PARTY`
(which covers verbatim/adapted code from upstream repositories) by
giving the *idea* attributions. For the lines-of-code attributions
see `LICENSE-THIRD-PARTY`.

## Integer arithmetic

### Möller–Granlund magic-number division by an invariant

Used for the `÷ 10^SCALE` step in every `Mul` / `Div` operator and in
`rescale`. The divisor `10^SCALE` is known at compile time, so a
pre-computed magic constant and a single 128-bit multiplication plus
a one-step correction replace a generic divide instruction. The
crate ships a 39-entry table (`MG_EXP_MAGICS`, scales 0–38) and the
256/128-bit fast-2-word divide built around it.

> Möller, N. and Granlund, T. (2011). **"Improved Division by Invariant
> Integers."** *IEEE Transactions on Computers* **60(2)**, 165–175.
> DOI: [10.1109/TC.2010.143](https://doi.org/10.1109/TC.2010.143).

Earlier foundational reference for the magic-multiplier idea:

> Granlund, T. and Montgomery, P. L. (1994). **"Division by Invariant
> Integers using Multiplication."** *Proc. PLDI '94*. ACM, 61–72.
> DOI: [10.1145/178243.178249](https://doi.org/10.1145/178243.178249).

Implementation: `src/mg_divide.rs` (`mul2`, `div_exp_fast_2word`,
`div_exp_fast_2word_with_rem`, `MG_EXP_MAGICS`). The algorithm shape
was adapted from the
[`primitive_fixed_point_decimal`](https://github.com/WuBingzheng/primitive_fixed_point_decimal)
crate — see `LICENSE-THIRD-PARTY` for the verbatim attribution.

### Base-2¹²⁸ schoolbook multiplication

Standard `O(n²)` algorithm; for `n ≤ 4` limbs the constant factor is
small enough that more sophisticated algorithms (Karatsuba,
Toom-Cook) lose to it on this crate's operand sizes.
Implementation: `src/wide_int/mod.rs::limbs_mul`, with a hand-unrolled
2×2 fast path.

### Base-2⁶⁴ schoolbook long division (u64-divisor fast path)

For divisors that fit a 64-bit word, the crate uses one hardware
divide per 64-bit half-limb of the dividend. This is the standard
schoolbook long division, transcribed for `[u128]` limb storage.
Implementation: `src/wide_int/mod.rs::limbs_divmod` (fast path B);
`src/mg_divide.rs::div_long_256_by_128` (256-bit specialisation).

### Binary shift-subtract long division (fallback)

Last-resort divide for arbitrary 128+ bit divisors. One bit per
iteration, total iterations equal to the dividend's actual bit
length (precomputed via `leading_zeros`).
Implementation: `src/wide_int/mod.rs::limbs_divmod` general path;
`src/mg_divide.rs::div_long_256_by_128` general path.

## Roots

### Newton iteration for integer square root (`isqrt`)

`x_{k+1} = (x_k + N / x_k) / 2`, started from a power-of-2
overestimate so the sequence decreases monotonically. Converges
quadratically. Implementation: `src/mg_divide.rs::isqrt_256`,
`src/wide_int/mod.rs::limbs_isqrt`.

### Newton iteration for integer cube root (`icbrt`)

`x_{k+1} = (2·x_k + N / x_k²) / 3`. Same monotone-decreasing setup
as `isqrt`. Implementation: `src/mg_divide.rs::icbrt_384`,
`src/macros/wide_roots.rs` (decl_wide_roots! emits a 384/512-bit
variant per wide tier).

### Correctly-rounded sqrt / cbrt

After the integer root `q = floor(N^{1/k})`, the crate decides
"round up to `q+1`?" by an integer comparison of `N` against the
midpoint, which is an integer for sqrt (the midpoint test is
`N − q² > q`) and a multiple of `1/8` for cbrt (the test is
`8N ≥ (2q + 1)³`). For integer `N` the midpoint is never an integer
in either case, so the rounding decision is mode-independent —
every `RoundingMode` agrees with the half-to-nearest choice.
Implementation: `src/mg_divide.rs::sqrt_raw_correctly_rounded` /
`cbrt_raw_correctly_rounded`; the wide-tier counterparts in
`src/macros/wide_roots.rs`.

## Transcendentals

### `ln` via Mercator's series of `artanh`

Range-reduce `x = 2^k · m` with `m ∈ [1, 2)`, then compute
`ln(m) = 2·artanh((m − 1) / (m + 1))`. The argument `t = (m − 1) /
(m + 1)` lies in `[0, 1/3]`, so the Mercator series
`artanh(t) = t + t³/3 + t⁵/5 + …` converges as roughly `3^(-n)` —
about 22 terms per decimal digit.

Mercator's logarithm series:

> Mercator, N. (1668). *Logarithmotechnia*. (Cited via Borwein &
> Borwein, "Pi and the AGM", 1987, Wiley.)

The artanh form is a textbook identity; combined with bit-by-bit
range reduction it's sometimes called the "Cody–Waite" approach
after the influential 1980 implementation:

> Cody, W. J. and Waite, W. (1980). **"Software Manual for the
> Elementary Functions."** Prentice-Hall.

Implementation: `src/log_exp_strict.rs::ln_fixed` (D128),
`src/macros/wide_transcendental.rs::ln_fixed` (D256/D512/D1024).

### `exp` via range-reduced Taylor series

Range-reduce `x = k · ln 2 + s` with `|s| ≤ ln 2 / 2`, then
`exp(x) = 2^k · exp(s)`. The Taylor series for `exp(s)` converges
absolutely on the reduced interval. The same Cody–Waite shape.
Implementation: `src/log_exp_strict.rs::exp_fixed`,
`src/macros/wide_transcendental.rs::exp_fixed`.

### `sin` / `cos` via range-reduced Taylor

Reduce to `[0, π/4]` (or `[0, π/2]` in the wide path, slightly
slower convergence), Taylor-expand `sin`, recover `cos` from
`sin(x + π/2)`. Same Cody–Waite shape.
Implementation: `src/trig_strict.rs::sin_fixed`,
`src/macros/wide_transcendental.rs::sin_fixed` / `sin_taylor`.

### `atan` via three argument halvings + Taylor

The identity `atan(x) = 2·atan(x / (1 + √(1 + x²)))` halves the
argument; applying it three times reduces |x| by ≈ 8×, then the
Taylor series for `atan` converges in ≈ `w · log₂(10) / 3` terms
at working scale `w`. Re-multiply by `2^3 = 8` at the end.
Implementation: `src/trig_strict.rs::atan_fixed`,
`src/macros/wide_transcendental.rs::atan_fixed`.

### `π` via Machin's formula (wide tier only)

`π = 16·atan(1/5) − 4·atan(1/239)`. Each `atan` is evaluated via the
crate's Taylor implementation; with the small arguments the series
converges fast.

> Machin, J. (1706). Cited via Beckmann, P. (1971). *A History of π*.
> St. Martin's Press.

Implementation: `src/macros/wide_transcendental.rs::pi`. (D128
embeds `π` to 63 fractional digits as a literal — no series at run
time, since the constant fits the working width comfortably.)

### Hyperbolic functions

Composed from `exp`/`ln`:
- `sinh(x) = (eˣ − e⁻ˣ) / 2`
- `cosh(x) = (eˣ + e⁻ˣ) / 2`
- `tanh(x) = sinh(x) / cosh(x)`
- `asinh(x) = ln(x + √(x² + 1))` (with the `x ≥ 1` form factored as
  `ln(x) + ln(1 + √(1 + 1/x²))` to keep `x²` in the working width)
- `acosh(x) = ln(x + √(x² − 1))` (analogous factoring for `x ≥ 2`)
- `atanh(x) = ln((1 + x) / (1 − x)) / 2`

All textbook identities — no specific paper attribution.
Implementation: `src/trig_strict.rs`, `src/macros/wide_transcendental.rs`.

## Rounding

### Half-to-even (banker's rounding) and the IEEE-754 family

The crate's default rounding rule. Implementation in
`src/rounding.rs::should_bump`, dispatched per
[`RoundingMode`](src/rounding.rs) via a strategy hook.

> IEEE Std 754-2019. **"IEEE Standard for Floating-Point Arithmetic."**
> IEEE Standards Association.

## Constants

The mathematical constants in `src/consts.rs` (`pi`, `tau`,
`half_pi`, `quarter_pi`, `e`, `golden`) are stored as 37-digit raw
`i128` values at `SCALE_REF = 37` (the i128 maximum for the
largest of the six). Sources:

- `pi`, `tau`, `half_pi`, `quarter_pi`: ISO 80000-2.
- `e`: OEIS A001113.
- `golden`: OEIS A001622.

## Algorithms surveyed but not currently used

- **Karatsuba multiplication.** For 1024-bit multiplicands the
  schoolbook cross-over is at ~16+ limbs; the crate's widest fast-
  path mul is Int1024 (8 limbs of u128). Below cross-over schoolbook
  wins. Karatsuba kicks in at D2048 and above; recorded as a future
  optimisation. (Karatsuba, A. and Ofman, Yu. (1962). *Doklady Akad.
  Nauk SSSR* 145, 293–294.)
- **AGM-based ln / exp (Brent–Salamin 1976).** `ln_strict_agm`
  (D256 / D512 / D1024) uses Brent's identity
  `ln(s) ≈ π / (2 · AGM(1, 4/s))` with range reduction
  `ln(x) = ln(x · 2^m) − m·ln 2`. `exp_strict_agm` uses Newton's
  iteration on `ln_strict_agm`. Both converge quadratically — `O(log
  p)` iterations vs the artanh path's `O(p)` series terms — so they
  win asymptotically as working scale grows. Currently exposed as
  the alternate path; the canonical `ln_strict` / `exp_strict` stays
  on the artanh / Taylor implementations until a bench at the
  relevant working scale shows AGM winning by the
  `OVERRIDE_POLICY.md` margin. *Caveat:* the present
  implementation runs the AGM iteration at the same working scale
  `w` as the artanh path; at storage scales beyond ~30 the early-
  phase `sqrt(a·b)` step's truncation error amplifies and the
  output drops to ~p/2 bits of precision. Brent §3 fixes this by
  raising intermediate AGM precision; recorded as a follow-up.
  (Brent, R. P. (1976). "Fast multiple-precision evaluation of
  elementary functions." *J. ACM* 23(2), 242–251.)
- **Burnikel–Ziegler recursive division.** `limbs_divmod_bz` in
  `src/wide_int/mod.rs` is the recursive wrapper; its base case is
  the in-crate Knuth Algorithm D port (`limbs_divmod_knuth`,
  TAOCP §4.3.1 adapted to base 2^128). Both functions sit
  alongside the canonical const-fn binary `limbs_divmod` — the
  canonical path is unchanged, and `_knuth` / `_bz` are exposed for
  bench-driven promotion. Knuth's `O(m·n)` multi-limb shape beats
  the binary path's `O((m+n)·n·128)` for any multi-limb divisor;
  BZ's recursion adds value only past the threshold (currently
  `BZ_THRESHOLD = 8` limbs) and the full §3 two-by-one /
  three-by-two recursion is recorded as the next layer to add once
  a bench shows it winning at this crate's widths. (Burnikel, C.
  and Ziegler, J. (1998). "Fast recursive division." MPI-I-98-1-022;
  Knuth, D. E. (1981). *The Art of Computer Programming, Vol. 2:
  Seminumerical Algorithms*, §4.3.1.)
- **CORDIC.** Common in hardware floating-point; not competitive
  with Taylor + reduction in a software fixed-point context.

## Related external crates (benchmark baselines only)

- [`bnum`](https://github.com/isaacholt100/bnum) — fixed-width
  big-integer crate, used as a wide-int baseline in
  `benches/wide_int_backends.rs`.
- [`ruint`](https://github.com/recmo/uint) — Ethereum-flavoured
  wide-integer crate, used as a 256-bit baseline.
- [`rust_decimal`](https://github.com/paupino/rust-decimal) —
  96-bit-mantissa decimal crate, used as a decimal baseline.
- [`fixed`](https://gitlab.com/tspiteri/fixed) — binary fixed-point
  crate, used for the I64F64 baseline.

These crates are `dev-dependencies` only — they are never compiled
into a normal build.
