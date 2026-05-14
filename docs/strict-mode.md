# Strict mode — integer-only transcendentals

By default, transcendental functions (logarithms, exponentials, roots,
trigonometry) route through the platform `f64` intrinsics: the `D128`
value is converted to `f64`, the `f64` function is applied, and the
result is converted back. This is fast and, on mainstream platforms,
bit-deterministic in practice — but it depends on `std` and on the
platform libm.

The `strict` feature replaces that path with **integer-only**
implementations:

```toml
[dependencies]
decimal-scaled = { version = "0.1.1", features = ["strict"] }
```

Under `--features strict`:

- Every transcendental is computed with integer arithmetic only — no
  `f64`, no libm, no platform dependence. Results are bit-identical on
  every target.
- `strict` does **not** require `std`. The integer algorithms compile
  under `no_std + alloc`.
- The explicit float-conversion methods (`to_f64_lossy`,
  `from_f64_lossy`, `TryFrom<f64>`, …) remain available — they are type
  conversions, not transcendental operations.

## What strict covers

| Group | Methods |
|---|---|
| Logarithms | `ln`, `log`, `log2`, `log10` |
| Exponentials | `exp`, `exp2` |
| Roots / powers | `sqrt`, `cbrt`, `pow`, `powi`, `powf` |
| Forward trig | `sin`, `cos`, `tan` |
| Inverse trig | `asin`, `acos`, `atan`, `atan2` |
| Hyperbolic | `sinh`, `cosh`, `tanh` |
| Inverse hyperbolic | `asinh`, `acosh`, `atanh` |
| Angle conversion | `to_degrees`, `to_radians` |

```rust
# #[cfg(feature = "strict")]
# {
use decimal_scaled::D128s12;

let x = D128s12::from_bits(2_000_000_000_000);   // 2.0
let r = x.sqrt();                                 // ≈ 1.414213562373
let l = x.ln();                                   // ≈ 0.693147180560
# }
```

## Accuracy

The strict implementations use classic fixed-point techniques: range
reduction followed by a truncated Taylor / Mercator series (a tighter
Remez-polynomial implementation is tracked in
`research/strict_transcendentals_research.md`). Accuracy is within
roughly **±10 ULP** at moderate `SCALE`, degrading toward the extreme
scales where the series cap and the fixed-point intermediate run out of
headroom.

If you need correctly-rounded transcendentals, strict mode is not that —
it is the *deterministic, dependency-free* option, trading a few ULP of
accuracy for platform independence.

## Width coverage

- `D128` has hand-written strict implementations.
- `D32` / `D64` get the strict surface by widening to `D128`, computing,
  and narrowing back.
- The wide tier (`D256` / `D512` / `D1024`) does not yet have strict
  transcendentals — that work needs the higher-degree polynomials
  described in the research doc and is tracked as a follow-up. Wide-tier
  arithmetic, conversions, and rescaling are unaffected.

## Choosing the mode

| You want… | Use |
|---|---|
| Maximum speed, `std` available, mainstream platform | default (f64 bridge) |
| Bit-identical results on every platform, or `no_std` | `strict` |
| Both in one build | not supported — `strict` and the f64 bridge are mutually exclusive per build |
