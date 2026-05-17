# Roadmap

Known performance gaps and planned improvements. Tracked by tier
of the §5 Library-comparison benchmark in
[`docs/benchmarks.md`](docs/benchmarks.md). Cells where
`decimal-scaled` already wins are out of scope - these are the
loss columns and how we plan to close them.

The crate's **accuracy** invariants are not on this roadmap.
`decimal-scaled` is 0 ULP correctly-rounded on every
transcendental tested at every tier, and stays that way. The
roadmap is throughput-only - give people a way to keep the
exactness when they need it and an opt-out when they don't.

---

## Wide-tier `÷ 10^SCALE` - primary bottleneck

The Möller–Granlund magic-multiply (`mg_divide`) is the kernel
behind every wide-tier `mul` and `div` (they both end with a
`÷ 10^SCALE` step to keep the result at the right scale). At
D38 it's the right algorithm. At D76 and above the magic
constant has to be widened a tier above the storage width,
which serialises a chain of limb multiplies through a single
carry-propagating accumulator.

Concrete symptom: at 256-bit / s=35, `decimal-scaled` `div` is
~830× `fastnum`'s `div` (5.08 µs vs 6.07 ns).

| approach | status | expected win |
|---|---|---|
| Bench-pick MG vs alternatives at each width / scale point | TODO | knowing the breakeven matters for the next two |
| Burnikel–Ziegler recursive divide on top of `limbs_divmod` | TODO | the right asymptote for D153+; D307 should benefit most |
| Newton-iteration reciprocal as a `mg_divide` fast-path replacement at extreme scales | TODO | flatlines div cost across the deepest tiers if the iteration count stays bounded |

---

## Wide-tier multiplication - Karatsuba / Toom-Cook

At D76 / D153 / D307 the multiplication kernel is straight
schoolbook over `[u64; 4]` / `[u64; 8]` / `[u64; 16]`. The
crossover for Karatsuba is typically around 8 limbs, Toom-3
around 32. D153 and D307 sit squarely in Karatsuba and
Karatsuba-vs-Toom-3 territory, respectively, but neither is
implemented.

Concrete symptom: 1024-bit `mul` is 66.7 µs in `decimal-scaled`
vs 141 ns in `bigdecimal`. The crate carries the cost of
`16 × 16 = 256` limb multiplies serially.

| approach | status | expected win |
|---|---|---|
| Karatsuba on `Int512` / `Int1024` mul | TODO | ~2× at D153, ~3-4× at D307 |
| Toom-3 on `Int1024` mul, gated by limb count | TODO | further ~1.5-2× on top of Karatsuba at the very deepest scale |
| SIMD limb multiplies (AVX-512, NEON) gated behind a `simd` feature | speculative | hardware-dependent; worth a probe bench before committing |

---

## Wide-tier transcendentals - give callers an opt-out

`decimal-scaled` deliberately keeps every transcendental at
**0 ULP correctly rounded**, regardless of tier. At D76+ that
costs ~µs per call (`ln`, `exp`, `sin`); at D307 it's ~ms. For
callers that don't need 0-ULP determinism (e.g. plotting a
curve, doing approximate convergence checks) this is overkill.

`*_fast` already exists on every width, but on the wide tiers
it routes through `to_f64` / `f64::ln` / `from_f64` and the
result collapses to 16 decimal digits regardless of the storage
width - a precision cliff that's hard to communicate.

| approach | status | expected win |
|---|---|---|
| `*_approx(working_digits: u32)` family - same series as `*_strict` but with caller-controlled working-scale cutoff | TODO | linear cost reduction proportional to the requested digit cut |
| Document the precision cliff of `*_fast` on wide tiers more loudly | TODO | non-code; reader expectations |
| Newton-on-AGM `ln` / `exp` paths past D153 - quadratic convergence, asymptotically wins where the artanh series stalls | partial (`bench-alt`) | not yet promoted by the dispatcher; crossover point measured in `benches/agm_vs_taylor.rs` |

---

## More decimal widths - fill the tier ladder

Current widths cover the power-of-two storage sequence
(32 / 64 / 128 / 256 / 512 / 1024 bits). Real-world picks
often fall between these - e.g. a `D57` covers IEEE 754 binary192
mantissa precision, a `D462` covers cryptographic-class
high-precision intermediates without paying the full D616 cost.

Plan:

- **Double the top end up to 4096 bits.** D307 (1024 bit) is the
  current ceiling; add D616 (2048 bit) and D1232 (4096 bit).
- **Fill in the half-step widths between each existing pair.**

Resulting tier ladder:

| storage bits | type | safe decimal digits | status |
|---|---|---|---|
| 32   | `D9`    | 9    | shipped |
| 48   | `D14`   | 14   | TODO |
| 64   | `D18`   | 18   | shipped |
| 96   | `D28`   | 28   | TODO |
| 128  | `D38`   | 38   | shipped |
| 192  | `D57`   | 57   | TODO |
| 256  | `D76`   | 76   | shipped |
| 384  | `D115`  | 115  | TODO |
| 512  | `D153`  | 153  | shipped |
| 768  | `D230`  | 230  | TODO |
| 1024 | `D307`  | 307  | shipped |
| 1536 | `D462`  | 462  | TODO |
| 2048 | `D616`  | 616  | TODO |
| 3072 | `D924`  | 924  | TODO |
| 4096 | `D1232` | 1232 | TODO |

Each new tier needs its own `IntN` storage in `crate::wide_int`,
the corresponding `MAX_SCALE` plumbing, and matching wide-int +
strict transcendental kernels (the macros already generate the
per-tier code once the storage type exists). Cargo features
follow the existing `wide` / `x-wide` pattern - probably a new
`xx-wide` / `xxx-wide` gate for the additions past D307 to keep
default build times sane.

---

## Narrow-tier - already competitive

D9 / D18 / D38 arithmetic already matches or beats
`fixed::I*F*` (the only directly-comparable competitor at these
widths). D38 transcendentals are 1.47 µs `ln`, 40.5 µs `exp` at
s=19, vs `fastnum`'s 16 ns / 8.92 µs - but those are
f64-bridge for fastnum (1 ULP off) vs 0 ULP for us. No
roadmap item here unless the accuracy contract changes.

---

## Methodology / infrastructure

| approach | status | expected win |
|---|---|---|
| Re-bench every release on a single dedicated machine, not whatever runner happened to be available | TODO | reduces inter-release noise that currently looks like regressions |
| Track ULP deltas continuously, not one-shot at 0.2.5 | TODO | catches accuracy regressions early; cheap to run |
| Cross-platform bit-determinism CI (Linux/macOS/Windows × x86_64/aarch64) | TODO | proves the `*_strict` invariant the docs claim |

---

## Wide-tier MG magic-multiply extension + negative SCALE

| approach | status | expected win |
|---|---|---|
| Extend the Möller–Granlund magic-multiply tables past `10^38` to cover every wide-tier SCALE (target: `10^SCALE` for `SCALE` up to the tier MAX_SCALE) so the `÷10^SCALE` step on D76 and above swaps multi-limb Knuth divide for one magic multiply + a fix-up | TODO | est. 3–10× on wide-tier mul/div per the 2026-05-17 gap research |
| Make `SCALE` signed (i32) so callers can express implicit-trailing-zero magnitudes (D38<-3> = "stored value × 10³"); orthogonal to the magic-multiply work but shares the per-tier `10^k` constant tables — a single tables-rewrite covers both | TODO | enables values up to `±i128::MAX × 10^(-SCALE)` without burning storage on zero-padding; common in actuarial / national-accounts work |

See `research/2026_05_17_wide_mul_div_gap.md` for the gap
analysis and `research/2026_05_17_mg_magic_extension_eval.md` for
the design eval combining both items.

---

## Out-of-tree adapter crates

The core crate is deliberately compile-time-fixed-precision: a
runtime-variable scale would break const-fn arithmetic,
deterministic limb work, and the per-tier specialised
transcendentals. Database / serialisation ergonomics that need a
runtime scale are a better fit as thin adapter crates layered on
top.

| crate (proposed)              | what it bridges                                                                                                       | status |
|-------------------------------|-----------------------------------------------------------------------------------------------------------------------|--------|
| `decimal-scaled-sqlx`         | Map SQL `NUMERIC(p, s)` columns to a caller-chosen `D{N}<SCALE>`; handle string-form fallback for non-matching scale  | TODO   |
| `decimal-scaled-diesel`       | Same shape for Diesel's `Numeric` SQL type                                                                            | TODO   |
| `decimal-scaled-arrow`        | Arrow `Decimal128` / `Decimal256` column round-trip                                                                   | TODO   |
| `decimal-scaled-protobuf`     | A protobuf `Decimal` message round-trip helper                                                                        | TODO   |

These intentionally live outside the core crate so the core
stays `no_std`, has no DB / serialisation drivers as deps, and
keeps a small public surface. Each adapter owns its own
runtime-scale negotiation and converts at the boundary into the
caller's compile-time-fixed tier.

---

## Out-of-tree ecosystem crates — applications of the 0-ULP core

The core ships *the deterministic primitive*; the interesting
applications layer above it. Three planned downstream crates that
together turn `decimal-scaled` into a complete numerical toolkit
without bloating the core:

| crate (proposed)             | what it adds | why it wants this backend |
|------------------------------|--------------|---------------------------|
| `decimal-scaled-expr`        | Runtime-parsed expression DSL (formulas-as-strings, spreadsheet-style); AST → eval against `D{N}<SCALE>` values; UDF hook | Bit-exact reproducible spreadsheet / rule-engine evaluation; the *only* such engine that doesn't drift on `0.1 + 0.2` |
| `decimal-scaled-math`        | Extended types: complex, rationals, vectors, matrices (small + sparse), statistical distributions, interval arithmetic, error propagation | 0-ULP determinism propagates through every algebra; lets you compose linear-algebra pipelines that are bit-identical across machines |
| `decimal-scaled-finance`     | Time-value-of-money (NPV, IRR, PV, FV), amortisation schedules, day-count conventions (ACT/360, 30/360, ACT/ACT, ACT/365), bond pricing, Black-Scholes, FX with caller-chosen rounding | Finance is the original deterministic-decimal use case; every regulator-facing calc must reproduce exactly across re-runs and across counterparties |

Same out-of-tree principle as the adapters: each lives in its own
crate, depends on `decimal-scaled` only, opts into the tier (`d76`,
`d307`, etc.) it actually needs, and exposes its surface generically
over `Decimal`-trait-implementing types so the caller picks the
storage tier. The finance crate in particular benefits from
`*_with(mode)` propagation: every regulator has its own
last-digit-rounding rule (HALFUP, HALFDOWN, HALFEVEN), and we can
honour each per-call without forking the engine.
