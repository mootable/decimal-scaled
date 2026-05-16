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
