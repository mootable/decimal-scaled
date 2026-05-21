# Glossary

Every acronym, shortening, and term of art used across `decimal-scaled`,
its docs, and its benchmarks — in one place.

## Precision & accuracy

| Term | Expansion | Meaning |
|------|-----------|---------|
| **LSBε** (ASCII: **LSBe**) | **Least Significant Bits in error** (the Greek *ε* denotes error) | The count of low-order bits of the stored integer that differ from the correctly-rounded result. **`0` means the value is exactly correctly rounded** (bit-for-bit). A larger number means that many of the bottom bits are wrong; bounded by the type's storage width (≤128 at `D38`, ≤1024 at `D307`, ≤4096 at `D1232`). A deliberate play on *LSB*. Written **LSBε** everywhere; **LSBe** is the plain-ASCII fallback. |
| **ULP** | [Unit in the Last Place](https://en.wikipedia.org/wiki/Unit_in_the_last_place) | The size of one step in the last representable digit. Here, 1 ULP = `10^(-SCALE)`. "Within 0.5 ULP" of the true value ⇔ correctly rounded ⇔ `0` LSBe. |
| **LSB** | Least Significant Bit | The bit of weight `2^0` (= 1) — the smallest possible change to an integer. |
| **MSB** | Most Significant Bit | The highest-weight bit. |
| **CR** | Correctly Rounded | The result equals the true real value rounded to the storage scale under the active rounding mode. The crate's strict transcendentals are CR (`0` LSBe / ≤0.5 ULP). |
| **bit-exact** | — | Identical bit pattern on every platform — a guarantee of the integer-only core (no floating point in results). |
| **guard digits** | — | Extra working-precision digits carried during a computation and discarded on the final rounding, so the rounding decision is correct. |

## The model

| Term | Meaning |
|------|---------|
| **`SCALE`** | The compile-time const generic fixing the number of fractional digits. A value is stored as `raw × 10^SCALE`. |
| **`MAX_SCALE`** | The largest `SCALE` a width supports = *N − 1* for `D{N}` (e.g. `D38` → 37), leaving ≥1 integer digit of headroom. |
| **`D9` … `D1232`** | The thirteen storage widths; the number is the type's nominal precision in decimal digits. `D9`=`i32`, `D18`=`i64`, `D38`=`i128`, `D57`+ use wide integers. |
| **limb** | One `u64` word of a wide integer. A wide value is `[u64; N]`, little-endian. |
| **wide integers** | The hand-rolled wide-integer backend for `D57`+: a value is an array of `u64` limbs, little-endian. |
| **`no_std`** | Builds without the standard library (embedded-friendly). The strict, integer-only path is `no_std`-compatible. |
| **strict / fast / approx** | `*_strict` = correctly-rounded integer path (default). `*_fast` / `*_approx` = the `f64`-bridge path (~16 digits, platform-dependent, not CR). |

## Rounding modes

| Mode | Behaviour |
|------|-----------|
| `HalfToEven` | Nearest; ties to even — the IEEE 754 default (banker's rounding). |
| `HalfAwayFromZero` | Nearest; ties away from zero. |
| `HalfTowardZero` | Nearest; ties toward zero. |
| `Ceiling` | Toward +∞. |
| `Floor` | Toward −∞. |
| `Trunc` | Toward zero (truncate). |

Every lossy operation has a `*_with(mode)` sibling; the crate-wide
default is selectable at compile time via the `rounding-*` Cargo features.

## Operation shortenings

| Short | Operation | Short | Operation |
|-------|-----------|-------|-----------|
| `mul` | multiply | `sqrt` | square root |
| `div` | divide | `cbrt` | cube root |
| `rem` | remainder (`%`) | `sqr` | square (`x²`) |
| `add` | add | `cube` | cube (`x³`) |
| `sub` | subtract | `pow` / `powf` | power (integer / decimal exponent) |
| `neg` | negate | `exp` | `e^x` |
| `abs` | absolute value | `exp2` | `2^x` |
| `ln` | natural log (base e) | `expm1` | `e^x − 1` (accurate near 0) |
| `log` | log to a given base | `log1p` | `ln(1 + x)` (accurate near 0) |
| `log2` / `log10` | log base 2 / 10 | `sin`/`cos`/`tan` | trigonometric |
| `asin`/`acos`/`atan` | inverse trig | `atan2` | two-argument arctangent |
| `sinh`/`cosh`/`tanh` | hyperbolic | `asinh`/`acosh`/`atanh` | inverse hyperbolic |
| `rescale` | change `SCALE` | `widen` / `narrow` | change storage width |
| `isqrt` | integer square root | `root_int` | integer n-th root |

Method suffixes: **`_with(mode)`** takes an explicit `RoundingMode`;
**`_strict`** / **`_fast`** select the integer vs `f64`-bridge path;
**`_of`** is the cross-scale operator family (`mul_of`, `add_of`, …)
accepting mixed-width, mixed-`SCALE` operands.

## Algorithms

| Name | Used for |
|------|----------|
| **MG** — [Möller–Granlund](https://gmplib.org/~tege/division-paper.pdf) | division by a constant (`÷ 10^SCALE`) via magic-multiply |
| **BZ** — Burnikel–Ziegler | recursive divide-and-conquer division (wide tiers) |
| **Karatsuba** / **Toom-Cook** | subquadratic multiplication |
| **NTT / FFT** | number-theoretic / fast Fourier transform multiply (evaluated; above the crate's width ceiling) |
| **Newton** ([Newton–Raphson](https://en.wikipedia.org/wiki/Newton%27s_method)) | precision-doubling iteration for roots/reciprocals |
| **Ziv** | [Ziv's strategy](https://en.wikipedia.org/wiki/Rounding#Table-maker's_dilemma): widen the working precision until the rounding is provably correct |
| **AGM** | arithmetic–geometric mean (asymptotic `ln` at extreme scales) |
| **Tang tables** | table-driven seed for `exp`/`ln` |
| **Cody–Waite** / **Payne–Hanek** | argument range reduction for trig |
| **Brent–Zimmermann** | the reference text for the integer/float algorithms used here |

## Standards

| Term | Expansion |
|------|-----------|
| **IEEE 754** | The IEEE [floating-point standard](https://en.wikipedia.org/wiki/IEEE_754). This crate is decimal fixed-point — not an IEEE 754 float type — but its `RoundingMode` set and the `HalfToEven` default mirror IEEE 754's rounding conventions. |
