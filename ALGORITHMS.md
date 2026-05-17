# Algorithms used in `decimal-scaled`

Catalogue of the published algorithms the crate evaluates, with
academic citations and the source files where each is implemented.
This is engineering credit - it complements `LICENSE-THIRD-PARTY`
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
crate - see `LICENSE-THIRD-PARTY` for the verbatim attribution.

Further reading:

- Wikipedia - [Division algorithm § Division by a constant](https://en.wikipedia.org/wiki/Division_algorithm#Division_by_a_constant)
- Wolfram MathWorld - [Division](https://mathworld.wolfram.com/Division.html)
- Niels Möller's homepage: <https://www.lysator.liu.se/~nisse/>
- Torbjörn Granlund's homepage (GMP project): <https://gmplib.org/~tege/>

### Limb storage shape — `[u64; 2·N]`

The wide-integer types (`Int256` through `Int4096`) are stored as
little-endian `[u64; 2·N]` arrays — twice the `u128`-equivalent limb
count quoted in their names. The choice exposes native `u64 × u64 →
u128` and `u128 / u64` hardware instructions directly (Zen 4 / Intel
Golden Cove issue one widening 64×64 mul per cycle in steady state)
where the historical `[u128; N]` layout had to soft-emulate every
mul as four `u64 × u64` sub-products plus a nested carry chain. The
public `from_limbs_le([u128; N])` / `limbs_le() -> [u128; N]` API
preserves its u128-shaped signatures via a 4-line const-fn boundary
conversion, so the wire format and downstream pattern-matching stay
bit-stable.

Implementation: every `limbs_*_u64` routine in
`src/wide_int/mod.rs`; the `$U` / `$S` storage in
`src/wide_int/macros/mod.rs`.

### Base-2⁶⁴ schoolbook multiplication

Standard `O(n²)` schoolbook on u64 limbs, using the native
`u64 × u64 → u128` widening multiply for each sub-product.
Karatsuba was implemented (and tested) but lost to schoolbook at
every tier this crate emits because the per-cycle widening mul
throughput beats Karatsuba's `3·(n/2)² + add/sub` overhead until
`n > ~64` limbs (beyond our widest tier). The Karatsuba code is
retained in `src/wide_int/mod.rs` (u128-base) as a reference / future
SIMD baseline.
Implementation: `src/wide_int/mod.rs::limbs_mul_u64`.

Further reading:

- Wikipedia - [Multiplication algorithm](https://en.wikipedia.org/wiki/Multiplication_algorithm)
- Wolfram MathWorld - [Multiplication](https://mathworld.wolfram.com/Multiplication.html)

### Möller–Granlund 2-by-1 invariant reciprocal in Knuth's q̂ loop

Inside [`Knuth Algorithm D`](#knuth-algorithm-d-multi-limb-divide),
each quotient limb's q̂ estimate would otherwise cost a 64-iteration
bit-recovery loop. Precomputing the divisor's 2-by-1 reciprocal
(Möller-Granlund Algorithm 4) once per call collapses the q̂ step
to ~2 multiplies plus a constant fix-up. Setup cost is one hardware
`u128 / u64` divide; amortised across the `m + 1` quotient-limb
estimations the per-limb work drops by ~30×.

> Möller, N. and Granlund, T. (2011). **"Improved Division by
> Invariant Integers."** *IEEE Transactions on Computers* **60(2)**,
> 165–175.

Implementation: `src/wide_int/mod.rs::MG2by1U64`.

A 3-by-2 sibling (`MG3by2U64`, also implemented per MG Algorithm 5
with GMP's `invert_pi1` reciprocal refinement) is kept available for
arbitrary-divisor use cases. It was *not* faster than 2-by-1 +
refinement loop on decimal divisors because the refinement loop
almost never fires for our (well-conditioned) divisors, so the
3-by-2's per-call extra multiply costs more than it saves.

### Knuth Algorithm D — multi-limb divide

Textbook Algorithm D (Knuth, TAOCP Vol. 2, §4.3.1) adapted to base
2⁶⁴. Normalise the divisor so its top limb has the high bit set,
then for each quotient limb estimate q̂ from the top two limbs of
the running dividend (via the MG 2-by-1 reciprocal above), refine
once against the second-from-top divisor limb, multiply-subtract,
and add-back on the rare miss. Complexity `O(m·n)` limb-ops vs the
shift-subtract fallback's `O((m+n)·n·64)`.

Implementation: `src/wide_int/mod.rs::limbs_divmod_knuth_u64`,
fronted by `limbs_divmod_dispatch_u64`.

### Base-2⁶⁴ schoolbook long division (u64-divisor fast path)

For divisors that fit a single u64 word, the crate uses one hardware
`u128 / u64` divide per dividend limb — every `10^scale` with
`scale ≤ 19` lands here. This is the standard schoolbook long
division, now riding the native hardware instruction directly
(previously this path had to split each u128 limb into 64-bit halves
and do two divides per limb).
Implementation: `src/wide_int/mod.rs::limbs_divmod_u64` (fast path B);
`src/mg_divide.rs::div_long_256_by_128` (256-bit specialisation for
D38 transcendentals, still u128-typed by design).

Further reading:

- Wikipedia - [Long division](https://en.wikipedia.org/wiki/Long_division)
- Wolfram MathWorld - [Long division](https://mathworld.wolfram.com/LongDivision.html)

### Binary shift-subtract long division (const fallback)

Last-resort divide for arbitrary multi-limb divisors *in const
context*. Runtime callers route through the
[`limbs_divmod_dispatch_u64`](#knuth-algorithm-d-multi-limb-divide)
dispatcher to Knuth instead; this path stays as the `const fn`
backstop for `wrapping_div` / `wrapping_rem` and as the setup path
for the MG reciprocal computation.
Implementation: `src/wide_int/mod.rs::limbs_divmod_u64` general path.

Further reading:

- Wikipedia - [Division algorithm § Restoring division](https://en.wikipedia.org/wiki/Division_algorithm#Restoring_division)
- Wolfram MathWorld - [Division](https://mathworld.wolfram.com/Division.html)

## Roots

### Newton iteration for integer square root (`isqrt`)

`x_{k+1} = (x_k + N / x_k) / 2`, started from a power-of-2
overestimate so the sequence decreases monotonically. Converges
quadratically.

At wide tiers the per-iteration divide routes through
[`limbs_divmod_dispatch_u64`](#knuth-algorithm-d-multi-limb-divide),
i.e. Knuth Algorithm D with the MG 2-by-1 q̂ reciprocal. Earlier
versions used the const-context `limbs_divmod_u64` shift-subtract
path here, which made wide-tier sqrt 24–92× slower than necessary;
swapping to the runtime dispatcher closes that gap completely. D38
keeps its hand-tuned `isqrt_256` because the 256-bit specialisation
out-paces the generic path at single-limb scales.

Implementation: `src/mg_divide.rs::isqrt_256`,
`src/wide_int/mod.rs::limbs_isqrt_u64`.

Further reading:

- Wikipedia - [Methods of computing square roots § Heron's method](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method)
- Wikipedia - [Newton's method § Description](https://en.wikipedia.org/wiki/Newton%27s_method#Description) (the parent recurrence)
- Wolfram MathWorld - [Square Root](https://mathworld.wolfram.com/SquareRoot.html), [Newton's Method](https://mathworld.wolfram.com/NewtonsMethod.html)

### Newton iteration for integer cube root (`icbrt`)

`x_{k+1} = (2·x_k + N / x_k²) / 3`. Same monotone-decreasing setup
as `isqrt`. Implementation: `src/mg_divide.rs::icbrt_384`,
`src/macros/wide_roots.rs` (decl_wide_roots! emits a 384/512-bit
variant per wide tier).

Further reading:

- Wikipedia - [Cube root § Numerical methods](https://en.wikipedia.org/wiki/Cube_root#Numerical_methods)
- Wolfram MathWorld - [Cube Root](https://mathworld.wolfram.com/CubeRoot.html)

### Correctly-rounded sqrt / cbrt

After the integer root `q = floor(N^{1/k})`, the crate decides
"round up to `q+1`?" by an integer comparison of `N` against the
midpoint, which is an integer for sqrt (the midpoint test is
`N − q² > q`) and a multiple of `1/8` for cbrt (the test is
`8N ≥ (2q + 1)³`). For integer `N` the midpoint is never an integer
in either case, so the rounding decision is mode-independent -
every `RoundingMode` agrees with the half-to-nearest choice.
Implementation: `src/mg_divide.rs::sqrt_raw_correctly_rounded` /
`cbrt_raw_correctly_rounded`; the wide-tier counterparts in
`src/macros/wide_roots.rs`.

Further reading:

- Wikipedia - [IEEE 754 § Roundings to nearest](https://en.wikipedia.org/wiki/IEEE_754#Roundings_to_nearest) (the "correctly rounded" contract the crate emulates at the storage scale)

## Transcendentals

### `ln` via Mercator's series of `artanh`

Range-reduce `x = 2^k · m` with `m ∈ [1, 2)`, then compute
`ln(m) = 2·artanh((m − 1) / (m + 1))`. The argument `t = (m − 1) /
(m + 1)` lies in `[0, 1/3]`, so the Mercator series
`artanh(t) = t + t³/3 + t⁵/5 + …` converges as roughly `3^(-n)` -
about 22 terms per decimal digit.

Mercator's logarithm series:

> Mercator, N. (1668). *Logarithmotechnia*. (Cited via Borwein &
> Borwein, "Pi and the AGM", 1987, Wiley.)

The artanh form is a textbook identity; combined with bit-by-bit
range reduction it's sometimes called the "Cody–Waite" approach
after the influential 1980 implementation:

> Cody, W. J. and Waite, W. (1980). **"Software Manual for the
> Elementary Functions."** Prentice-Hall.

Implementation: `src/log_exp_strict.rs::ln_fixed` (D38),
`src/macros/wide_transcendental.rs::ln_fixed` (D76/D153/D307).

Further reading:

- Wikipedia - [Mercator series](https://en.wikipedia.org/wiki/Mercator_series) (the `ln(1+x)` expansion at the top)
- Wikipedia - [Inverse hyperbolic functions § Series expansions](https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions#Series_expansions) (the `artanh` series the crate evaluates)
- Wolfram MathWorld - [Mercator Series](https://mathworld.wolfram.com/MercatorSeries.html), [Inverse Hyperbolic Tangent](https://mathworld.wolfram.com/InverseHyperbolicTangent.html)

### `exp` via two-stage argument reduction + Taylor

Wide-tier exp uses Brent's two-stage argument reduction (dashu's
`exp_internal` pattern, traced to Brent 1976 §3):

1. **Stage 1 — modular:** `k = round(v/ln 2)`; `s = v − k·ln 2`,
   giving `|s| ≤ ln 2 / 2 ≈ 0.347`.
2. **Stage 2 — multiplicative:** `s ← s / 2^n` with
   `n ≈ √(precision_bits)`. After both reductions the Taylor
   argument satisfies `|r| < 2⁻ⁿ ≈ 2⁻√ᵖ`, so the series converges
   in `O(√p)` terms instead of `O(p)`. n is chosen via integer
   `sqrt(3w + 1)` (using `w·log₂(10) ≈ 3.32w` as the bit estimate).
3. **Taylor** on the reduced argument.
4. **Reassembly:** square `n` times to undo stage 2, then bit-shift
   by `k` to undo stage 1.

The squaring step replaces `≈ 60` Taylor mul+div pairs with
`n` plain wide multiplies, which is the dominant saving (a divide
is more expensive than a multiply at our widths even after the
u64 storage migration).

> Brent, R. P. (1976). **"Fast multiple-precision evaluation of
> elementary functions."** *Journal of the ACM* **23(2)**, 242–251.
> DOI: [10.1145/321941.321944](https://doi.org/10.1145/321941.321944).

Implementation: `src/macros/wide_transcendental.rs::exp_fixed`;
narrow tier in `src/log_exp_strict.rs`.

Further: `dashu-float::exp::Context::exp_internal` is the modern
reference implementation we cross-checked against.

### Cached `10^w` divisor in Taylor / AGM / Newton inner loops

Every `mul(a, b, w)` / `div(a, b, w)` in `wide_transcendental` does
a `round_div` against `10^w`. Computing `lit(10).pow(w)` from
scratch on each call costs ~10–50 µs at D307<150> (`w=180`, ~8 wide
squarings followed by ~180 cumulative multiplies). The cached
variants `mul_cached(a, b, pow10_w)` / `div_cached(...)` accept
a precomputed `10^w` and skip the recomputation; the inner Taylor
/ AGM / Newton loops hoist `let pow10_w = pow10(w);` once and pass
it down, saving ~20 % on every wide transcendental.

Implementation: `src/macros/wide_transcendental.rs::mul_cached`,
`div_cached`; consumed by `exp_fixed`, `ln_fixed`, `sin_taylor`,
`atan_taylor`.

### `exp` via plain Taylor (legacy reference)

Range-reduce `x = k · ln 2 + s` with `|s| ≤ ln 2 / 2`, then
`exp(x) = 2^k · exp(s)`. The Taylor series for `exp(s)` converges
absolutely on the reduced interval. The same Cody–Waite shape.
Implementation: `src/log_exp_strict.rs::exp_fixed`,
`src/macros/wide_transcendental.rs::exp_fixed`.

Further reading:

- Wikipedia - [Exponential function § Computation](https://en.wikipedia.org/wiki/Exponential_function#Computation) (the Taylor series and the `2^k · exp(s)` reduction)
- Wikipedia - [Taylor series § Exponential function](https://en.wikipedia.org/wiki/Taylor_series#Exponential_function)
- Wolfram MathWorld - [Exponential Function](https://mathworld.wolfram.com/ExponentialFunction.html), [Maclaurin Series](https://mathworld.wolfram.com/MaclaurinSeries.html)

### `sin` / `cos` via range-reduced Taylor

Reduce to `[0, π/4]` (or `[0, π/2]` in the wide path, slightly
slower convergence), Taylor-expand `sin`, recover `cos` from
`sin(x + π/2)`. Same Cody–Waite shape.
Implementation: `src/trig_strict.rs::sin_fixed`,
`src/macros/wide_transcendental.rs::sin_fixed` / `sin_taylor`.

Further reading:

- Wikipedia - [Taylor series § Trigonometric functions](https://en.wikipedia.org/wiki/Taylor_series#Trigonometric_functions) (the `sin x = x − x³/3! + …` and `cos x = 1 − x²/2! + …` series)
- Wolfram MathWorld - [Sine](https://mathworld.wolfram.com/Sine.html), [Cosine](https://mathworld.wolfram.com/Cosine.html), [Maclaurin Series](https://mathworld.wolfram.com/MaclaurinSeries.html)

### `atan` via three argument halvings + Taylor

The identity `atan(x) = 2·atan(x / (1 + √(1 + x²)))` halves the
argument; applying it three times reduces |x| by ≈ 8×, then the
Taylor series for `atan` converges in ≈ `w · log₂(10) / 3` terms
at working scale `w`. Re-multiply by `2^3 = 8` at the end.
Implementation: `src/trig_strict.rs::atan_fixed`,
`src/macros/wide_transcendental.rs::atan_fixed`.

Further reading:

- Wikipedia - [Inverse trigonometric functions § Infinite series](https://en.wikipedia.org/wiki/Inverse_trigonometric_functions#Infinite_series) (the `atan` Taylor series)
- Wikipedia - [Inverse trigonometric functions § Argument halving](https://en.wikipedia.org/wiki/Inverse_trigonometric_functions) (the halving identity)
- Wolfram MathWorld - [Inverse Tangent](https://mathworld.wolfram.com/InverseTangent.html)

### `π` via Machin's formula (wide tier only)

`π = 16·atan(1/5) − 4·atan(1/239)`. Each `atan` is evaluated via the
crate's Taylor implementation; with the small arguments the series
converges fast.

> Machin, J. (1706). Cited via Beckmann, P. (1971). *A History of π*.
> St. Martin's Press.

Implementation: `src/macros/wide_transcendental.rs::pi`. (D38
embeds `π` to 63 fractional digits as a literal - no series at run
time, since the constant fits the working width comfortably.)

Further reading:

- Wikipedia - [Machin-like formula](https://en.wikipedia.org/wiki/Machin-like_formula) (the `π = 16 atan(1/5) − 4 atan(1/239)` equation at the top)
- Wolfram MathWorld - [Machin's Formula](https://mathworld.wolfram.com/MachinsFormula.html), [Pi Formulas](https://mathworld.wolfram.com/PiFormulas.html)

### Hyperbolic functions

Composed from `exp`/`ln`:
- `sinh(x) = (eˣ − e⁻ˣ) / 2`
- `cosh(x) = (eˣ + e⁻ˣ) / 2`
- `tanh(x) = sinh(x) / cosh(x)`
- `asinh(x) = ln(x + √(x² + 1))` (with the `x ≥ 1` form factored as
  `ln(x) + ln(1 + √(1 + 1/x²))` to keep `x²` in the working width)
- `acosh(x) = ln(x + √(x² − 1))` (analogous factoring for `x ≥ 2`)
- `atanh(x) = ln((1 + x) / (1 − x)) / 2`

All textbook identities - no specific paper attribution.
Implementation: `src/trig_strict.rs`, `src/macros/wide_transcendental.rs`.

Further reading:

- Wikipedia - [Hyperbolic functions § Definitions in terms of the exponential function](https://en.wikipedia.org/wiki/Hyperbolic_functions#Definitions) (the `sinh`/`cosh`/`tanh` identities)
- Wikipedia - [Inverse hyperbolic functions § Logarithmic forms](https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions#Logarithmic_representation) (the `asinh`/`acosh`/`atanh` log-forms)
- Wolfram MathWorld - [Hyperbolic Functions](https://mathworld.wolfram.com/HyperbolicFunctions.html), [Inverse Hyperbolic Functions](https://mathworld.wolfram.com/InverseHyperbolicFunctions.html)

## Rounding

### Half-to-even (banker's rounding) and the IEEE-754 family

The crate's default rounding rule. Implementation in
`src/rounding.rs::should_bump`, dispatched per
[`RoundingMode`](src/rounding.rs) via a strategy hook.

> IEEE Std 754-2019. **"IEEE Standard for Floating-Point Arithmetic."**
> IEEE Standards Association.

Further reading:

- Wikipedia - [Rounding § Round half to even](https://en.wikipedia.org/wiki/Rounding#Round_half_to_even) (the tie-breaking rule the crate uses by default)
- Wikipedia - [IEEE 754 § Roundings to nearest](https://en.wikipedia.org/wiki/IEEE_754#Roundings_to_nearest)

## Constants

The mathematical constants in `src/consts.rs` (`pi`, `tau`,
`half_pi`, `quarter_pi`, `e`, `golden`) are stored as 37-digit raw
`i128` values at `SCALE_REF = 37` (the i128 maximum for the
largest of the six). Sources:

- `pi`, `tau`, `half_pi`, `quarter_pi`: ISO 80000-2.
- `e`: OEIS A001113.
- `golden`: OEIS A001622.

## Cross-over algorithms

- **Karatsuba multiplication.** Implemented in
  `wide_int::limbs_mul_karatsuba` and dispatched to by
  `wide_int::limbs_mul_fast` when both operands are equal-length and at
  least `KARATSUBA_MIN = 16` limbs. Below that threshold, schoolbook
  wins outright; at or above it, the recursive
  `(a₁b₁, (a₁+a₀)(b₁+b₀) − a₁b₁ − a₀b₀, a₀b₀)` decomposition reduces
  the asymptotic cost. In practice the threshold means storage tiers
  through Int1024 (8 limbs) use schoolbook; the 2048-/4096-bit work
  integers behind the wide-tier strict transcendentals (Int2048 = 16
  limbs, Int4096 = 32 limbs) hit the Karatsuba path. (Karatsuba, A. and
  Ofman, Yu. (1962). *Doklady Akad. Nauk SSSR* 145, 293–294.) Anatoly
  Karatsuba (1937–2008) and Yuri Ofman are both deceased; see the
  Wikipedia biography links below. Further reading:
  [Karatsuba algorithm](https://en.wikipedia.org/wiki/Karatsuba_algorithm),
  [Anatoly Karatsuba bio](https://en.wikipedia.org/wiki/Anatoly_Karatsuba),
  [Yuri Ofman bio](https://en.wikipedia.org/wiki/Yuri_Ofman),
  [MathWorld - Karatsuba Algorithm](https://mathworld.wolfram.com/KaratsubaAlgorithm.html).
- **AGM-based ln / exp (Brent–Salamin 1976).** `ln_strict_agm`
  (D76 / D153 / D307) uses Brent's identity
  `ln(s) ≈ π / (2 · AGM(1, 4/s))` with range reduction
  `ln(x) = ln(x · 2^m) − m·ln 2`. `exp_strict_agm` uses Newton's
  iteration on `ln_strict_agm`. Both converge quadratically - `O(log
  p)` iterations vs the artanh path's `O(p)` series terms - so they
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
  elementary functions." *J. ACM* 23(2), 242–251.) Richard Brent
  is at ANU - homepage: <https://maths-people.anu.edu.au/~brent/>.
  Further reading:
  [Arithmetic–geometric mean](https://en.wikipedia.org/wiki/Arithmetic%E2%80%93geometric_mean#Definition)
  (the `aₙ₊₁ = (aₙ+bₙ)/2`, `bₙ₊₁ = √(aₙ bₙ)` recurrence),
  [Gauss–Legendre algorithm](https://en.wikipedia.org/wiki/Gauss%E2%80%93Legendre_algorithm)
  (the same AGM iteration applied to π),
  [MathWorld - Arithmetic-Geometric Mean](https://mathworld.wolfram.com/Arithmetic-GeometricMean.html).
- **Burnikel–Ziegler recursive division.** `limbs_divmod_bz` in
  `src/wide_int/mod.rs` is the recursive wrapper; its base case is
  the in-crate Knuth Algorithm D port (`limbs_divmod_knuth`,
  TAOCP §4.3.1 adapted to base 2^128). Both functions sit
  alongside the canonical const-fn binary `limbs_divmod` - the
  canonical path is unchanged, and `_knuth` / `_bz` are exposed for
  bench-driven promotion. Knuth's `O(m·n)` multi-limb shape beats
  the binary path's `O((m+n)·n·128)` for any multi-limb divisor;
  BZ's recursion adds value only past the threshold (currently
  `BZ_THRESHOLD = 8` limbs) and the full §3 two-by-one /
  three-by-two recursion is recorded as the next layer to add once
  a bench shows it winning at this crate's widths. (Burnikel, C.
  and Ziegler, J. (1998). "Fast recursive division." MPI-I-98-1-022;
  Knuth, D. E. (1981). *The Art of Computer Programming, Vol. 2:
  Seminumerical Algorithms*, §4.3.1.) Donald Knuth's homepage:
  <https://www-cs-faculty.stanford.edu/~knuth/>. Further reading:
  [Division algorithm § Newton–Raphson division and recursive
   division](https://en.wikipedia.org/wiki/Division_algorithm)
  (no dedicated BZ article, but the parent page lists the
   recursive-division family),
  [MathWorld - Long Division](https://mathworld.wolfram.com/LongDivision.html).
  The Burnikel–Ziegler tech report is the canonical algorithm
  reference: [MPI-I-98-1-022](https://pure.mpg.de/rest/items/item_1819444_4/component/file_2599480/content).
- **CORDIC.** Common in hardware floating-point; not competitive
  with Taylor + reduction in a software fixed-point context.
  Further reading: [CORDIC](https://en.wikipedia.org/wiki/CORDIC)
  (the rotation-mode and vectoring-mode iterations are the central
  equations there), [MathWorld - CORDIC](https://mathworld.wolfram.com/CORDIC.html).

## Related external crates (benchmark baselines only)

- [`bnum`](https://github.com/isaacholt100/bnum) - fixed-width
  big-integer crate, used as a wide-int baseline in
  `benches/wide_int_backends.rs`.
- [`ruint`](https://github.com/recmo/uint) - Ethereum-flavoured
  wide-integer crate, used as a 256-bit baseline.
- [`rust_decimal`](https://github.com/paupino/rust-decimal) -
  96-bit-mantissa decimal crate, used as a decimal baseline.
- [`fixed`](https://gitlab.com/tspiteri/fixed) - binary fixed-point
  crate, used for the I64F64 baseline.

These crates are `dev-dependencies` only - they are never compiled
into a normal build.
