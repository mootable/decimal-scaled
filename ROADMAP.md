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

## Versioning intent

| target | gating work |
|--------|-------------|
| **0.3.0** (shipped) | Half-width tier ladder (D57 / D115 / D230 / D462 / D616 / D924 / D1232); the comprehensive cross-tier `widen()` / `narrow()` chain (D38.widen() now returns D57, etc.); the chain-of-÷10^38 wide-tier `mul` speedup (≥ 2× at D307<150>); strict-by-default dispatcher; per-width benchmark split; trig functions in the per-width summary chart family. |
| **0.3.1** (shipped) | Docs-site release-process fixes; rustdoc build covers x-wide + xx-wide tiers. |
| **0.3.2** (shipped) | Per-L fixed-array `mul` (1.25× on D307 exp); 17 trig fast paths; ln fast paths; adaptive halvings in atan; full profiling infrastructure (samply / flamegraph / perfetto); two benchmark add-ons (atan input-class comparison, per-L mul gate). |
| **0.3.3** (shipped) | Trait split — `Decimal` → `DecimalArithmetic + DecimalConvert`; benchmarks.md refresh from full sweep; docs workflow no longer triggers on tag pushes. |
| **0.4.0** (shipped) | Foundation release — `src/` six-bucket layout (`types/`, `algos/`, `macros/`, `wide_int/`, `consts/`, `prelude/`); type renames D56 family → D57; FromStr wide-tier fix; OpenSSF Best Practices + Scorecard + cargo-audit CI; REUSE LICENSES/. |
| **0.4.1** (shipped) | Cosmetic-only — dropped the `DecimalConsts` alias. No perf delta. |
| **0.4.2** (shipped) | Tang ln ladder 13×-34× across narrow-GUARD bands (D57<18-22> through D1232<610-620>); AGM crossover empirically located at SCALE 1000 (3× past textbook 300 digits); D18 mul/div -60% / -47%; chain-MG bit-exact half-to-even for w > 38; `limbs_mul_u64_into<L, LP1>` primitive; benchmarks.md refresh. |
| **0.4.3** (shipped) | See "0.4.3 candidates" section. Tang completion sweep (5 deeper bands: D230<115>, D307<290>, D616<590>, D924<900>, D1232<1200>); const POW10_TABLE for D38–D616; powf integer-exponent fast path (107× at D38<19> for `x.powf(2.0)`); cross-scale `_of` API + nightly `cross::*` auto-inference; precision-coverage expansion (mpmath golden tables + proptest fuzz + CI gate); parity-test tightening to ±1 LSB. |
| **0.4.4** (shipped 2026-05-21) | **Full correct-rounding completion** — every `*_strict` transcendental is 0 LSBε / ≤ 0.5 ULP under all six rounding modes, all thirteen widths, across the whole 22-function surface (directed-rounding Ziv escalation; correctly-rounded derived functions; log1p/gap reformulation for `acosh`/`atanh`; sign-stable + wider-work-int hyperbolics; exact-power pins). Strict-golden suite: 286 cells, 0 ignored, delta==0 vs an external mpmath oracle. |
| **0.5.0** (incoming) | **Integer / decimal architecture rewrite** — a unified const-generic `Int<N>` / `Uint<N>` backend (the named `IntXXXX` become aliases) plus a reusable width-matched integer-algorithm layer with method parity to the decimals; a base/std/no_std policy collapse keyed on a const-folded `(width, SCALE)` match; non-allocating stack-scratch Karatsuba with the threshold re-swept to the measured crossover; and the 0.4.4 precision corrections folded in. See the "0.5.0 architecture" section. |
| **0.5+** (proposed) | RNG surface; public `expm1` / `log1p`; GDA `round-up` / `round-05up` modes; the DB / serialisation adapter crates (incl. CBOR tag-4); the ecosystem crates (`-math`, `-finance`, the lazy/reactive expression engine); ecosystem trait impls (`approx`, `Euclid`/`Inv`/`Pow`, nalgebra/ndarray); standards-conformance evidence (`dectest` rounding vectors, I-JSON round-trip). Each lands when it earns its place; none gate 1.0. |
| **1.0.0** | The version stays pre-1.0 until either (a) the wide-tier `mul` / `div` numbers are *competitive with the best peer* at every shipped width — currently the `dashu-float` heap-arbitrary-precision baseline, which we trail by ~14× to ~100× at the wide tiers — *or* (b) the gap has a clearly-defensible structural reason (different storage shape, different precision invariant, different ULP contract) documented per row in the benchmarks. Adapter + ecosystem crates (per the sections below) ship at their own pace and do not gate the core 1.0. |

## 0.5.0 architecture (incoming)

0.5.0 is a structural release. It does **not** change the accuracy
contract — it re-lays the foundations so the integer backend and the
decimal front-ends share one vocabulary and the dispatch is provably
zero-cost.

- **Unified `Int<N>` / `Uint<N>` backend.** One const-generic wide
  integer over `[u64; N]` replaces the hand-named `IntXXXX` family (kept
  as `pub type` aliases). A reusable width-matched integer-algorithm
  layer (add / sub / mul / div / shift / `sqr` / `cube` / `root_int`)
  gives the integers full method parity with the decimals behind a
  `FixedInt` trait that mirrors `DecimalArithmetic`.
- **base / std / no_std policy.** Each function's dispatch collapses to a
  const-folded `match (width, SCALE)` (the `base` table), a `no_std`
  pointer to it, and a `std` layer carrying only the benchmarked-faster
  overrides — so the rich per-tier policy compiles to one direct call per
  monomorphisation, with no runtime dispatch.
- **Integer perf pass.** Non-allocating stack-scratch Karatsuba (no
  `Vec`, `no_std`-capable) with `KARATSUBA_THRESHOLD_U64` re-swept to the
  measured u64 crossover; it unblocks the wide-tier Burnikel–Ziegler and
  Karatsuba-sqrt work.
- **Precision corrections folded in.** The full 0.4.4 correct-rounding
  work merges forward, with the delta==0 strict-golden suite kept as a
  permanent regression gate.

The 0.5.0 architecture is documented in the Architecture overview that
ships with 0.5.0.

## Shipped recently in 0.3.x

Tactical perf and fast-path wins, with bench evidence in
`target/` and `docs/benchmarks.md`:

| change | measured effect |
|--------|-----------------|
| `limbs_mul_u64_fixed<const L, D>` wired into `widen_mul` / `wrapping_mul` / `checked_mul` | 1.22-2.73× on per-L mul; **1.25× on D307<150> `exp_strict` whole-call** (88.8 → 71.1 µs) |
| Adaptive halvings in `atan_fixed` (halve while `|y| > ~0.2`, max 8; was fixed 3) | 3-5× on atan with small / reciprocal-reduced inputs (D38<19>: `atan(0.001)` 44 → 14 µs; `atan(1e8)` 44 → 8 µs) |
| 17 trig fast paths: zero / ±1 / small-x linear band for `atan`, `sin`, `cos`, `tan`, `asin`, `acos`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `to_degrees`, `to_radians` | ~1000-10000× on the fast-path inputs (`atan(0)` 68 µs → 5 ns; `atan(1e-7)` 68 µs → 5 ns); best-in-class small-x atan vs both fastnum and g_math |
| `ln(1) = 0` + `ln(1+ε) ≈ ε` linear-band fast paths in `ln_strict` | <10 ns on fast-path inputs vs prior 1.4 µs |
| Per-width bench split: `benches/lib_cmp_d{N}.rs` for D18 through D1232 | minutes vs hours per-tier iteration; was a 0.3.x infrastructure TODO, now done |
| Profiling infrastructure: `perf-trace` feature, section spans in `exp_fixed` / `atan_fixed`, samply + perfetto example drivers + parser scripts | establishes the M2-gate discipline used through the rest of this work |
| `benches/atan_inputs.rs` — input-class atan timing (decimal-scaled vs fastnum vs g_math) | exposed two bench-validity issues (fastnum `atan(|x|>1)` = NaN, fastnum `ln(2)` = const lookup) — recorded in `docs/benchmarks.md` |

Items investigated and intentionally *not* shipped (kept as
dead code / docs for posterity; audit trail in commit messages):

- `limbs_sqr_u64` squaring fast path — 50% fewer widening MULs in theory; on this toolchain the ADC overhead equalises it, exp regressed 7%. Kernel retained in tree; not wired.
- `5^w` magic-multiply split for `÷10^w` — Granlund-Montgomery reciprocal for the `5^w` factor. Failed M2 gate 4-5× at every tier; algorithm wrong-shaped for multi-limb divisors at our ratio.
- `limbs_mul_u64_v2` dashu-style inner row (`mul_add_2carry` form) — within noise on this toolchain.
- MG 2-by-1 → 3-by-2 swap in `limbs_divmod_knuth_u64` — mixed signal across tiers; algorithm reverted, structural cleanups (n=1 short-circuit, escape-early `u_top > v_top`, hoist `j+n`) retained.
- Phase-1 LLVM-unroll batch (`add_assign_fixed`, `sub_assign_fixed`, etc.) — won per-op micro-benches but regressed whole-call D307 exp by 18% (icache / inline-decision interaction). Kernels in tree; macros wire to slice variants.
- Integer-rhs auto-detect in D38 `Mul`/`Div` — within noise at D38 (base path is already a single i128 multiply); extending to wide tiers would need a multi-limb modulo per call.
- u64-base Karatsuba mul — implemented, gated off in the dispatcher (`KARATSUBA_THRESHOLD_U64 = 256`, above our widest tier). M2 bench at L=16-96 showed schoolbook 1.07-1.92× faster everywhere; LLVM-unrolled schoolbook + the heap allocs the recursive split needs put the crossover past every shipped width. Implementation + correctness oracle retained for future use (SIMD, extra-wide tiers, or a scratch-passing rewrite that drops the Vec allocs).
- Real Burnikel-Ziegler recursive divmod — depends on Karatsuba winning for sub-products; foreclosed by the above. The existing `limbs_divmod_bz_u64` is chunked Knuth and stays.

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
textbook Karatsuba crossover is ~8 limbs and Toom-3 is ~32. In
practice the LLVM-unrolled u64 schoolbook beats both at every
length this crate emits — see the row below.

| approach | status | notes |
|---|---|---|
| Karatsuba on `Int512` / `Int1024` mul | **implemented, gated off** | `limbs_mul_karatsuba_u64` with `KARATSUBA_THRESHOLD_U64 = 256` (above our widest tier). M2 bench at L = 16-96 measured schoolbook 1.07-1.92× *faster* everywhere; the recursive split's heap allocations dominate the asymptotic win at our widths. Kept in tree for SIMD / extra-wide / scratch-passing future work. See [`ALGORITHMS.md`](ALGORITHMS.md) cross-over section. |
| Toom-3 on `Int1024` mul, gated by limb count | foreclosed | dependent on Karatsuba winning a tier below it; since Karatsuba loses to schoolbook everywhere we ship, Toom-3 is structurally further out. Reconsider if SIMD widening shifts the schoolbook constant. |
| SIMD limb multiplies (AVX-512, NEON) gated behind a `simd` feature | speculative | hardware-dependent; worth a probe bench before committing. This is the *only* lever that would change the Karatsuba / Toom-3 outcome above. |

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
| Tang table-driven `ln` / `exp` / `sin_cos` / `atan` / hyperbolic at narrow-GUARD bands | **shipped 0.4.2 + extended 0.4.3-candidate** | 3-34× over artanh / Taylor at the gated `(width, scale)` bands; full ladder D57<18-22> → D1232<610-620>. See [`ALGORITHMS.md`](ALGORITHMS.md) Tang section. |
| `*_approx(working_digits: u32)` family — same series as `*_strict` but with caller-controlled working-scale cutoff | TODO | linear cost reduction proportional to the requested digit cut |
| Document the precision cliff of `*_fast` on wide tiers more loudly | TODO | non-code; reader expectations |
| Newton-on-AGM `ln` / `exp` paths past D153 — quadratic convergence, asymptotically wins where the artanh series stalls | partial (`bench-alt`) | Crossover empirically located at SCALE 1000 (3× past textbook 300 digits) thanks to the well-tuned chain-MG artanh path. Currently exposed as the alternate path; promotion gated on AGM precision lift (queued as 0.4.3-candidate B) since the present implementation runs intermediate AGM steps at the working scale and loses precision past ~30. |

---

## More decimal widths - fill the tier ladder

Tier ladder is now complete from 64-bit storage (D18) up to 4096-bit
storage (D1232), covering every multiple-of-64 step. The half-step
tiers between each power-of-two (D57, D115, D230, D462, D924) shipped
in 0.3.0 and let callers pay only for the precision they need
without jumping a full storage doubling.

| storage bits | type | safe decimal digits | status |
|---|---|---|---|
| 64   | `D18`   | 18   | shipped |
| 128  | `D38`   | 38   | shipped |
| 192  | `D57`   | 57   | shipped (0.3.0) |
| 256  | `D76`   | 76   | shipped |
| 384  | `D115`  | 115  | shipped (0.3.0) |
| 512  | `D153`  | 153  | shipped |
| 768  | `D230`  | 230  | shipped (0.3.0) |
| 1024 | `D307`  | 307  | shipped |
| 1536 | `D462`  | 462  | shipped (0.3.0) |
| 2048 | `D616`  | 616  | shipped (0.3.0) |
| 3072 | `D924`  | 924  | shipped (0.3.0) |
| 4096 | `D1232` | 1232 | shipped (0.3.0) |

Sub-64-bit-limb tiers (the previous D14 / D28 entries at 48- and
96-bit storage) are out of scope — the wide-int kernels are
`[u64; N]`-shaped and the per-step gain over D18 or D38 doesn't
justify the limb-fragment book-keeping.

Each new tier needs its own `IntN` storage in `crate::wide_int`,
the corresponding `MAX_SCALE` plumbing, and matching wide-int +
strict transcendental kernels (the macros already generate the
per-tier code once the storage type exists). Cargo features
follow the existing `wide` / `x-wide` pattern - probably a new
`xx-wide` / `xxx-wide` gate for the additions past D307 to keep
default build times sane.

---

## Narrow-tier - already competitive

D18 / D38 arithmetic already matches or beats
`fixed::I*F*` (the only directly-comparable competitor at these
widths). D38 transcendentals are 1.47 µs `ln`, 40.5 µs `exp` at
s=19, vs `fastnum`'s 16 ns / 8.92 µs - but those are
f64-bridge for fastnum (1 ULP off) vs 0 ULP for us. No
roadmap item here unless the accuracy contract changes.

---

## Methodology / infrastructure

| approach | status | expected win |
|---|---|---|
| Split `benches/library_comparison.rs` into one bench-binary per width (`lib_cmp_d38.rs`, `lib_cmp_d76.rs`, `lib_cmp_d307.rs`, …) so `cargo bench --bench lib_cmp_d307` can iterate on a single tier without re-running the whole matrix | **DONE in 0.3.x** | minutes vs hours per iteration when tuning one tier; each file stays focused on its peer set |
| Re-bench every release on a single dedicated machine, not whatever runner happened to be available | TODO | reduces inter-release noise that currently looks like regressions |
| Track ULP deltas continuously, not one-shot at 0.2.5 | TODO | catches accuracy regressions early; cheap to run |
| Cross-platform bit-determinism CI (Linux/macOS/Windows × x86_64/aarch64) | TODO | proves the `*_strict` invariant the docs claim |

---

## Wide-tier MG magic-multiply extension

| approach | status | expected win |
|---|---|---|
| Extend the Möller–Granlund magic-multiply tables past `10^38` to cover every wide-tier SCALE (target: `10^SCALE` for `SCALE` up to the tier MAX_SCALE) so the `÷10^SCALE` step on D76 and above swaps multi-limb Knuth divide for one magic multiply + a fix-up | TODO | up to several × on wide-tier mul/div |

Signed `SCALE` (`SCALE: i32`, e.g. `D38<-3>` = "stored value × 10³")
was previously listed as a 0.4.0 / 0.5.0 target. **Deferred
indefinitely** — the const-generic infrastructure churn (every
type-bound, every `MAX_SCALE` check, every macro arm) does not pay
back the implicit-trailing-zero use case, which is already
expressible by promoting one tier and using a positive scale.

---

## Random number generation (0.5+ / proposed)

A cryptographically-secure RNG surface for sampling decimals.
Same out-of-tree-via-trait pattern as the rest of the
ecosystem: bring your own `RngCore` + `CryptoRng` from `rand`,
the crate provides the decimal-shaped sampling primitives.

| primitive | shape |
|-----------|-------|
| `gen_unit::<T, R>(rng)` | uniform `T` in `[0, 1)` — generate SCALE random decimal digits |
| `gen_range::<T, R>(rng, lo..hi)` | uniform `T` in a closed-or-half-open range; rejection-sampling at any SCALE so the distribution stays unbiased |
| `gen_storage::<T, R>(rng)` | fill the storage bits directly — useful for token-like opaque IDs |
| `gen_signed_unit::<T, R>(rng)` | uniform `T` in `(-1, 1)` with the sign bit also sampled |

Design choices:

- **No global state.** The crate doesn't ship its own `thread_rng()`
  or default RNG. Callers pass an `R: RngCore + CryptoRng`. This
  matches the `*_with(mode)` story for rounding — explicit > magic.
- **`no_std`-friendly.** Trait-bound RNG so `getrandom` /
  `OsRng` aren't required dependencies. Embedded callers can
  plug their own HRNG-backed `RngCore`.
- **Cryptographic correctness.** The rejection sampler for
  `gen_range` follows the well-trodden "draw N bytes, modulo
  by range only when below the rejection threshold" pattern;
  no modulo bias even at the widest tiers.
- **Distribution helpers in `decimal-scaled-math`.** Normal /
  log-normal / exponential / gamma / Box-Muller etc. live in
  the ecosystem math crate, not in the core. The core only
  provides the uniform primitives that everything else
  composes on top of.

## Adapter crates (in-workspace) — DB / serialisation bridges (0.5+ / proposed)

The core crate is deliberately compile-time-fixed-precision: a
runtime-variable scale would break const-fn arithmetic,
deterministic limb work, and the per-tier specialised
transcendentals. Database / serialisation ergonomics that need a
runtime scale are a better fit as thin adapter crates layered on
top.

**Layout decision: adapters live in this repo as sibling
workspace crates.** They're thin shims, they version-couple
tightly to the core, and atomic cross-crate refactors (D38.widen()
moving, SCALE going signed, etc.) land in one PR with one CI run.

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

## Ecosystem crates (separate repos under the `mootable` org) — applications of the 0-ULP core (0.5+ / proposed)

**Layout decision: ecosystem crates live in their own repos
under a shared GitHub org**, not in this workspace. They're
substantial standalone codebases with distinct contributor
audiences (finance, symbolic-math, formula-DSL), independent
release cadence, and per-domain CI policy (finance wants
regulator-driven golden-vector tests; expr might want fuzz CI;
math wants property-based tests over algebraic laws). Splitting
them out keeps each repo focused and lets specialists own
their domain without learning the wide-int internals.

The core ships *the deterministic primitive*; the interesting
applications layer above it. Three planned downstream crates that
together turn `decimal-scaled` into a complete numerical toolkit
without bloating the core:

| crate (proposed)             | what it adds | why it wants this backend |
|------------------------------|--------------|---------------------------|
| `decimal-scaled-expr`        | Dual-track expression engine: type-level builders for compile-time-known shapes that monomorphise to direct decimal ops, plus a runtime AST for spreadsheet-style string formulas | Bit-exact reproducible formula / rule engine; the *only* such engine that doesn't drift on `0.1 + 0.2`. Shared substrate for the math + finance crates below |
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

### Expression engine design notes (`decimal-scaled-expr`)

Two complementary tracks under a single `Compute` trait:

1. **Type-level expression templates.** Operator overloads on a
   small `Expr<...>` newtype build a zero-sized AST in the type
   system (`Add<X, Mul<Y, Lit>>`). `#[inline(always)]` traversal at
   `.eval()` time lets the compiler see straight through to direct
   decimal ops — when the operands are bound at the call site this
   should compile to the same machine code as hand-written
   arithmetic. Math + finance APIs use this track because their
   formulas are mostly fixed-shape at the call site (NPV is always
   `Σ cf_i / (1+r)^i`; cross-product is always `a × b - c × d`).

2. **Runtime AST.** `Box<Node>` shape for spreadsheet-style
   string-parsed formulas, evaluated by tree traversal. Required
   for the dynamic-expression use case; same `Compute` trait so
   downstream code is path-agnostic.

The "lazy decimal" idea is the bridge: the expr types behave like
decimals (impl `Add` / `Mul` / etc.) and can be passed wherever a
`Decimal` is expected. Code that materialises immediately
(`let result: D38<12> = (x + y * 2).compute();`) compiles away the
AST entirely under the type-level track. Code that defers the
materialisation gets symbolic manipulation, caching, partial
evaluation, etc., for free.

Math and finance crates compose on top by importing the trait
and writing their algorithms once — `npv(cashflows, rate)` works
identically whether `cashflows` is a `Vec<D38<12>>` (immediate
arithmetic), a `Vec<Expr<D38<12>>>` (lazy with re-evaluation), or
a `Vec<DynExpr>` (parsed from a spreadsheet cell).

**Whole-tree serialisation is a first-class requirement.**
Expressions (the entire AST, not just the materialised result)
need to round-trip through `Serialize` / `Deserialize` so they
can be:

- persisted to disk (spreadsheet save / load, business-rule
  storage, version-controlled formula libraries);
- transmitted over the network (API submission of a custom
  formula; remote evaluation; send-formula-to-server with the
  values resolved on the receiving side);
- written to audit logs for regulator-facing finance work
  (every applied formula recorded in its exact deserialisable
  form, so a re-run reproduces bit-identically — input values,
  intermediate operator tree, applied rounding mode, all stored
  as one tagged payload);
- diff-able as text (RON / JSON / S-expression for
  human-readable change review of business rules);
- equality-comparable in serialised form (two formulas that
  serialise to the same payload are structurally identical;
  useful for memoisation keys and conflict detection).

Implementation shape: the runtime AST `Box<Node>` is the
natural serialisation target — every operator node, every
literal, every variable reference, and every nested sub-tree
emits as a tagged-union element in the payload. The type-level
templates can *serialise* (visit the type-level AST, emit the
same tagged-union sequence) but *deserialisation* always
materialises a `DynExpr` because the inbound shape isn't known
at compile time. The `Compute` trait abstracts both so callers
don't care which side they got. Multiple wire formats supported
behind feature flags (`serde-json`, `serde-postcard`,
`serde-ron`) without forcing a default dependency. A schema
versioning scheme on the root node keeps long-lived persisted
formulas decodable as the AST grows new node types.
