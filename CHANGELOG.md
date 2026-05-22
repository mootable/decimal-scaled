# Changelog

All notable changes to `decimal-scaled` are documented here.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] — Unreleased

An architecture release. Every decimal width is now backed by the
const-generic `Int<N>` / `Uint<N>` integer layer introduced in this
cycle, unifying storage across the full tier table and eliminating the
native (primitive-integer) backend entirely. The 32-bit tier `D9` is
removed. The integer layer itself gains a parallel types / policy /
algos / limbs layout mirroring the decimal layer, plus a suite of
performance and correctness fixes that fed directly into the decimal
migration. The crate-wide default rounding mode features, docs
single-source tooling, precision harness, and pre-merge CI gates all
land alongside.

### Removed

- **`D9` (32-bit tier) removed in full.** The `D9<SCALE>` type, the
  `d9!` construction macro (both `const` and `fn` forms), the
  `wtag::D9` tag constant, the `dyn` registry entry, and all
  corresponding documentation and prose are gone. `D18` is now the
  narrowest supported width. Callers using `D9` should migrate to
  `D18`; the operator surface, rounding, and conversion APIs are
  identical.
- **Native decimal backend removed.** The non-wide macro arms that
  dispatched arithmetic, display, sign, conversions, equalities, and
  `num_traits` through primitive-integer storage are deleted. Every
  decimal tier now routes through the unified `Int<N>` wide arms.
  The dead `round_with_mode_native` helper and the `i64`-widening
  arithmetic machinery (`@native_i64_wider`, `i64` dispatch arm)
  that served only `D9` are gone.
- **`Unsigned` trait removed.** The internal marker trait carried no
  live impls after the native backend was dropped.

### Changed

- **`D18` storage changed from `i64` to `Int<1>`.** The 64-bit tier
  now stores its scaled integer in `Int<1>` (a one-limb
  const-generic integer) instead of a bare `i64`. `to_bits` and
  `from_bits` now accept and return `Int<1>` where they previously
  used `i64`. This is a breaking type change for call sites that
  stored or pattern-matched the raw bits directly.
- **`D38` storage changed from `i128` to `Int<2>`.** The 128-bit
  tier is now backed by `Int<2>` (two 64-bit limbs). `to_bits` and
  `from_bits` now use `Int<2>` instead of `i128`. Existing call
  sites that bridged through `i128` should call `as_i128()` /
  `Int::from_i128()` at the boundary. This is a breaking type change.
- **`D18` narrowing conversions tightened.** `TryFrom<i128>` and
  `TryFrom<u128>` for `D18` now reject values outside the `i64`
  storage range (previously the wide arms assumed ≥ 128-bit storage
  and silently truncated). `FromPrimitive::from_u64` on narrow
  `Int<1>` storage likewise rejects out-of-range inputs.
- **`checked_div` / `wrapping_div` / `overflowing_div` /
  `saturating_div` round to nearest like the `/` operator.** These
  four methods previously performed a truncating integer divide of
  the scaled numerator, disagreeing with the rounding `/` operator
  (e.g. `20/3` at SCALE 2 returned `666` instead of `667`). They
  now route through the same `round_with_mode_wide` step that `/`
  uses. This fixes a long-standing divergence on the wide tiers.
- **`Int<N>` / `Uint<N>` are now the public storage types.** All
  shipped widths (D18 through D1232) expose `Int<N>` as their
  `Storage` type. `Int<N>` / `Uint<N>` are re-exported from the
  crate root. The named-width aliases (`Int192`, `Int384`, …,
  `Int16384`) are now type aliases over `Int<N>`, and
  `decl_wide_int!` is removed.

### Fixed

- **`saturating_div` with a zero divisor now panics** (matching
  `std`). Previously it saturated to `MAX`, silently returning a
  nonsense value where every other integer type in Rust panics in
  debug.
- **`checked_rem` / `overflowing_rem` honour `MIN % -1` overflow.**
  `checked_rem(MIN, -1)` now returns `None`; `overflowing_rem(MIN,
  -1)` returns `(ZERO, true)`, matching the primitive integer
  contract. Both tests that covered this case were un-ignored.
- **`dyn` feature compiles after the `Int<N>` migration.** The
  `dyn_bridge` macro previously assumed primitive storage (`10 as
  $Storage`); it now constructs the rescale multiplier via
  `Int::from_i128(10).pow(shift)` and converts to the `RawStorage`
  variant primitive with `as_i128()`. `D18` fits the `i64` variant,
  `D38` fits the `i128` variant exactly.
- **`Int<N>` arithmetic sign-preserving right-shift restored.**
  `Shr` was not sign-extending correctly for negative values,
  corrupting wide-tier transcendental range-reduction. Fixed;
  confirmed against the 286-cell golden suite.
- **`Int<N>::leading_zeros` is two's-complement-faithful.** Negative
  values now correctly return `0` (all bits set under negation),
  matching `iN::leading_zeros` parity. The wide `Mul` / `Div`
  fast-path now takes magnitude via `unsigned_abs` rather than
  calling `leading_zeros` on signed values.
- **`D38` doc examples use the public `decimal_scaled::Int` path.**
  Examples that used `crate::int` failed in downstream doctests;
  corrected to the external-crate path.
- **Intra-doc link warnings resolved under `-D warnings`.** Broken
  links surfaced by the new `cargo doc --no-deps -D warnings` gate
  are fixed. `cargo doc` is now clean under `-D warnings`.
- **`D18` narrowing `TryFrom<u128>` rejects values past signed
  storage range.** Wide arms previously assumed ≥ 128-bit storage
  and truncated silently. `Int<1>` is the first narrow wide-storage
  tier; the guard is a no-op for wider tiers.

### Added

- **`Int<N>` / `Uint<N>` const-generic integer layer.** A
  const-generic fixed-width signed / unsigned integer pair
  parameterised by 64-bit limb count (`N`) backs every decimal tier.
  `Int<1>` replaces `i64`; `Int<2>` replaces `i128`; wider tiers map
  to `Int<3>` through `Int<64>`. The layer is mirrored under
  `src/int/` with types / policy / algos / limbs sub-modules
  matching the decimal layer's six-bucket layout.
- **`BigInt` trait** — a single unified trait surface for `Int<N>`,
  providing the full method coverage (`LIMBS`, `BITS`, `ZERO`, `ONE`,
  `MAX`, `MIN`, `leading_zeros`, `wrapping_add`, `wrapping_sub`,
  `wrapping_mul`, `div_rem`, `isqrt`, `widen`, `narrow`, …).
- **`Int<N>` checked primitive conversions (std-aligned).** Fills the
  gap left by the silently-truncating `from_i128` / `as_i128`:
  - `Int<N>::from_i128_checked` — value conversion, returns
    `Option<Int<N>>`, rejects out-of-range.
  - `Int<N>::from_i128_bits` — bit-reinterpretation (truncating),
    analogous to `i64::from_ne_bytes`.
  - `Int<N>::as_i128_bits` — bit-reinterpretation to `i128`.
  - `Uint<N>::from_u128_checked`, `Uint<N>::from_u128_bits` —
    same pattern for the unsigned half.
  - `From<i8..i64>` for `Int<N>`, `From<u8..u64>` for `Uint<N>`
    — infallible widening from narrow primitives.
  - `TryFrom<i128>` for `Int<N>`, `TryFrom<u128>` for `Uint<N>`
    — fallible narrowing (returns `ConvertError::Overflow`).
- **Non-allocating stack-scratch Karatsuba multiply** in
  `wide_int::widen_mul`. The non-alloc Karatsuba dispatcher is wired
  into `widen_mul`; the schoolbook path remains optimal at all
  currently-shipped widths (crossover would require limb counts beyond
  `Int<64>`), so the threshold is parked for GHA bench validation.
- **`div-by-10^19` base case for wide-int `to_string`.** The
  `to_string` implementation now peels 19-digit chunks by dividing by
  `10^19` (the largest power-of-ten below `2^64`), emitting each
  chunk with native arithmetic and calling the expensive
  full-width divide once per 19 digits. Measured speedup:
  2.3× (`Int256` / D76) to 14.9× (`Int4096` / D1232).
- **`mg_divide` clean-room rewrite from the Möller–Granlund 2011
  paper.** `MG_EXP_MAGICS` is now generated at compile time by a
  `const fn` (`mg_reciprocal`) that computes the magic constant via
  binary long division, following the paper's reciprocal formula.
  No literal table values are copied from any prior implementation.
  The function names `mul_u128_to_u256` and `divmod_pow10_2word`
  replace the prior upstream-derived identifiers. `LICENSES/THIRD-PARTY.md`
  confirms no third-party code is incorporated; `ALGORITHMS.md`
  carries the clean-room declaration and a courtesy prior-art note.
- **`no_std` + `alloc` build restored and CI-gated.** A regression
  where `alloc` and `num_traits::Float` imports were missing from the
  no-std path is fixed. A dedicated CI job now builds and tests the
  crate under `--no-default-features` to prevent future regressions.
- **`std` / `no_std` abstraction layer in policy.** `table_cache` and
  `float_seed` now live in `src/policy/` shims that select the
  `std`-backed (thread-local, memoised) or `no_std` (recompute)
  implementation at compile time, instead of having `std` / `no_std`
  conditional logic scattered across algorithm bodies.
- **`src/int/` integer layer layout.** The `wide_int/` sub-crate is
  absorbed; integer code is reorganised under `src/int/types/`,
  `src/int/policy/`, `src/int/algos/`, and `src/int/limbs/` mirroring
  the decimal six-bucket layout. The former `FixedInt` / `WideStorage` /
  `WideInt` traits are unified into the single `BigInt` trait, rooted in
  `src/int/types/traits/`.
- **Precision harness unified.** `tests/ulp_strict_golden.rs` and the
  comparative precision suite share a single `PrecisionSubject`
  harness with committable result TSV files under
  `results/precision/`. The `precision` CI workflow is extended with a
  `lib-cmp-precision` self-refresh that regenerates the TSVs after a
  clean matrix pass. Precision tables in `README.md` and
  `docs/benchmarks.md` are generated from those TSVs via
  `render_docs.py` to prevent prose drift.
- **Pre-merge CI gates.** `.github/workflows/ci.yml` runs on every
  pull request and every push to `main` / `release/*`, enforcing:
  full `cargo test` under all feature combinations (including default
  features), `cargo doc --no-deps -D warnings` (catches broken
  intra-doc links before merge), and an informational `cargo clippy
  --lib` step. A separate commit-hygiene gate rejects attribution
  trailers in commit messages.
- **`bench-full` self-refresh.** After each per-width matrix sweep,
  the `bench-full` workflow commits a refreshed `docs/benchmarks.md`
  so bench data and prose stay in lockstep without a manual step.
- **Architecture documentation.** `docs/ARCHITECTURE.md` ships a
  Mermaid layer diagram and sequence diagram showing the integer
  layer's own dispatch / algorithm tiers alongside the decimal layer.
  `src/int/` folders mirror `src/` under the diagram. `mkdocs.yml`
  enables Mermaid rendering.
- **`RELEASING.md` and release PR checklist template.** The release
  system of record (versioning policy, branch workflow, docs +
  benchmark refresh, publish steps) is codified in `RELEASING.md`
  and a PR checklist template. CodSpeed is noted as advisory (not a
  required gate).

## [0.4.4] — 2026-05-21

A correctness release. Every `*_strict` transcendental is now provably
correctly rounded — within 0.5 ULP, i.e. 0 LSB of error at the storage
scale — under **all six rounding modes**, at **all thirteen widths**,
across the **full 22-function surface**. 0.4.3 secured the primary
functions under nearest rounding; 0.4.4 extends the guarantee to the
directed rounding modes and the derived functions, with no exceptions
and no ignored cells.

### Fixed

- **Directed rounding** (`Trunc` / `Floor` / `Ceiling` /
  `HalfTowardZero` / `HalfAwayFromZero`) for the wide-tier
  transcendentals: the working-scale approximation could previously
  round one LSB the wrong way near a storage grid line. Resolved with
  residual-sign Ziv escalation; the nearest-mode fast path is unchanged.
- **Derived transcendentals** `log` / `log2` / `log10` / `exp2` /
  `sinh` / `cosh` / `tanh` / `asinh` / `acosh` / `atanh` / `powf` are
  now correctly rounded under every mode (they previously applied their
  final rounding without the directed-rounding escalation, and several
  carried cancellation or overflow-edge error at extreme inputs).
- **`acosh` near 1 and `atanh` near ±1**: removed catastrophic
  cancellation by reformulating through `log1p`, computing `1∓x` and
  `v²−1` as exact working-scale gaps.
- **`sinh` / `cosh` / `tanh` at large arguments**: corrected a
  sign-direction error that turned the relative error of `e^−|x|` into a
  large absolute error in its reciprocal; the dominant term is now
  always evaluated at `|x|`. Wide tiers near the storage-overflow edge
  now compute in a wider working integer.
- **`log_b(b^k)` and exact-power `powf`** (e.g. `powf(4, 0.5)`) return
  the exact result under directed rounding.
- **`tanh` tiny arguments**: directed rounding of the compressing
  linear band.
- **`D38` `log2` / `log10` at maximum scale**: removed an `i128`
  overflow in the exact-power detection.

### Changed

- The correct-rounding guarantee — previously documented for the
  primary functions under nearest rounding — now holds for the entire
  transcendental surface under all six rounding modes and all widths.

### Internal

- Added correctly-rounded integer `log1p` / `expm1` kernels (internal;
  a public API is planned).
- Expanded the strict-golden suite to the full 22-function × 13-width ×
  6-mode matrix, asserting `delta == 0` against an external mpmath
  oracle (286 cells, none ignored).

## [0.4.3] — 2026-05-20

A correctness-and-perf release. Closed every precision hole found by
a new external-oracle test suite, fixed two latent wide-tier overflow
bugs, and improved wide-tier `mul` while adding a cross-scale API.

### Added

- **Cross-scale `_of` operator surface** on all 13 widths (stable):
  `mul_of` / `add_of` / `sub_of` / `div_of` / `rem_of` / `max_of` /
  `min_of` / `clamp_of` (+ `_with(mode)` siblings) and comparators
  `cmp_of` / `eq_of` / `lt_of` / … accepting any-narrower-width
  any-SCALE operands via the new public `WidthLE<Target>` trait, plus
  cross-width `PartialEq` / `PartialOrd` at equal SCALE. A
  nightly-gated `cross-scale-ops` feature adds auto-inferred
  `cross::{mul,add,sub,div,rem,…}` free functions.
- **mpmath external-oracle precision suite** — `tests/ulp_strict_golden.rs`
  (golden tables generated at `mp.dps ≥ 2·SCALE + 64`),
  `tests/ulp_proptest.rs` (14 hard-input category strategies), and a
  7361-case hard-input corpus covering half-ULP ties, catastrophic
  cancellation, range-reduction breakpoints, near-pole conditioning,
  and inverse-identity round-trips. Wired into the `precision` CI gate.
- **powf integer-exponent fast path** (`|n| ≤ 64`) via `powi`,
  bit-exact and ~107× at D38<19> / ~27× at D307<150> for integer
  exponents.
- **Tang `ln` deep-band lookup slots** extending the ladder to
  D230<115>, D307<290>, D616<590>, D924<900>, D1232<1200>.
- **Const `POW10_TABLE`** for the D38–D616 tiers (`get_unchecked`
  after a bounds guard); ~22% on D76<35> `ln`, ~19% on `sin`.
- **Newton–Raphson reciprocal divide** for the `÷ 10^SCALE` step,
  dispatched at the bench-validated cells (Int2048 ≥ s200,
  Int3072 ≥ s200, Int4096 ≥ s400). Cite a textbook reciprocal
  iteration; MG magic-multiply remains canonical elsewhere.
- **AGM precision lift** — `ln_strict_agm` / `exp_strict_agm` now hold
  0.5 ULP at every wide tier (per-call working-scale guard sized to
  absorb the `sqrt` / `2^k` amplification). The dispatcher keeps
  `ln_strict` / `exp_strict` on the artanh / Tang path; lifted AGM
  stays the alternate path (it loses to artanh / Tang at every shipped
  tier × SCALE measured).
- **`bench-all` workflow** — one-click orchestrator dispatching the
  full release sweep (per-width `full_matrix` + `lib_cmp` +
  cross-version `bench-history`).

### Fixed

- **D153 / D1232 `exp` precision at large `|v|`** — the narrow-GUARD
  Tang `exp` kernel reassembled `2^k · exp(s)` at a fixed working
  scale, amplifying the `k · ln 2` reduction residual by up to `2^k`.
  `tang_exp_fixed` now lifts the working scale by
  `⌈|k|·log₁₀2⌉ + guard` and narrows back, restoring 0.5 ULP.
- **D1232 transcendental-constant truncation** — the u128 magnitude
  buffer in `mg_divide::div_wide_pow10{,_chain}_with` was sized for
  Int8192 (64 u128 limbs) and silently truncated the Int16384 work
  integer, corrupting `pi` / `ln 2` and every constant at D1232
  working scales ≳ 620. Buffer now sized to the work integer's
  `WideInt::U128_LIMBS`.
- **D115 high-scale transcendental overflow** — D115's transcendental
  work integer was `Int1024`, too small for the `t·t ≈ 1040-bit`
  intermediate in `ln 2` evaluation at SCALE 114; `exp_strict` /
  `ln_strict` panicked. Promoted D115's work integer to `Int2048`.
- **`tan` near-pole precision** (d153 / d307) — `tan = sin/cos`
  amplified the working-scale rounding by `1/cos ≈ |tan|` near odd
  multiples of π/2. The lookup sincos kernels now lift the working
  scale by `⌈log₁₀|tan|⌉ + guard` proportional to pole proximity.

### Changed

- **Width-adaptive `mg_divide` buffer** restores and improves wide
  `mul` (D76 mul ~52 ns, D307 mul ~122 ns at the history SCALEs —
  faster than v0.4.2), by sizing the `÷ 10^SCALE` stack buffer to the
  work integer instead of a flat 128-limb array.
- Parity tolerances between Tang lookup and `wide_kernel` paths
  tightened to ≤ 1 storage LSB at every audited cell.
- `ALGORITHMS.md` documents the Tang table-driven kernels and an
  "evaluated and not used" section (Comba, CORDIC, Johansson
  denominator-collection, libmpfr port, Newton reciprocal trade-offs);
  `ROADMAP.md` and `CONTRIBUTORS.md` refreshed; `docs/benchmarks.md`
  and all figures regenerated from the v0.4.3 sweep.

## [0.4.2] — 2026-05-19

A perf release. Every wide tier shipped a bespoke narrow-GUARD Tang
lookup slot for `ln` at its popular mid-storage SCALE, plus shared
kernel and infrastructure wins. Headline measured speedups at the
Tang `ln` slot (relative to v0.4.0 at the same cell): D115<57>
12.7×, D153<76> 19.4×, D307<150> 27×, D462<230> 27.6×, D616<308>
26.1×, D924<461> 30.8×, D1232<616> 33.8×.

### Added

- **Tang `ln` narrow-GUARD lookup slots** at every wide tier:
  D57<20>, D115<57>, D153<76>, D307<150>, D462<230>, D616<308>,
  D924<460>, D1232<615>. Each is M=128 with GUARD_NARROW=8 (10
  for atan/inverse). Cite Tang 1989 / 1990 (ACM TOMS 16(4)).
- **Tang `exp` surface lookup** at D57<18..=22>, D115<57>, and
  D153<70..=82>. At D307 and wider the surface-Tang `exp` loses
  to the canonical adaptive Smith r/2^n `wide_kernel::exp_fixed`
  (the Tang post-reduction Taylor needs more wide mults than the
  Smith squaring tail at Int3072+); the Tang `exp` lookup
  modules ship at D307 / D462 / D616 / D1232 but are not wired
  in `policy::exp`. The `tang_exp_fixed` helper is consumed by
  the hyperbolic kernels (sinh / cosh / tanh) at every tier
  where the trig lookup exists, in combination with the
  reciprocal-divide identity for an additional 1.2–1.31×.
- **Narrow-GUARD trig family** (sin / cos / tan / atan / sinh /
  cosh / tanh) per tier; 1.09–1.83× speedups.
- **MG-chain bit-exact half-to-even for w > 38** in
  `algos::mg_divide::div_wide_pow10_chain_with`; promoted to
  production in `macros::wide_transcendental`
  `round_to_storage_with`. Audited at 642K cross-witness inputs
  across all 6 `RoundingMode`s. Cite Möller–Granlund 2011 IEEE TC
  60(2).
- **`limbs_mul_u64_into<L, LP1>` primitive** in `wide_int/mod.rs`
  — Knuth Algorithm M specialised to n = 1. Wired into the
  wide-tier `mul_u` choke-point.
- **f64-bridge Newton seed** for `sqrt` + `cbrt` at the wide-tier
  macro core. Cite Brent–Zimmermann 2010 §5.4.
- **`benches/agm_vs_taylor_d{230,616,924,1232}.rs`** —
  reproducible AGM-vs-artanh probe at deep SCALE.

### Changed

- **`exp(-v)` → `1/exp(v)` reciprocal-divide identity** in
  sinh / cosh / tanh cuts the second `exp_fixed` call; ~2–2.4× at
  wide tier, ~1.5× at D38.
- **`Fixed::sqrt` Pythagorean joint `sin_cos` refactored to
  shared-reduction-only** at D38 — `sqrt` at the 256-bit Fixed
  costs more than a second Taylor at this width; reverted to
  shared-reduction. (Distinct from the wide-tier `sqrt_fixed`
  where the Pythagorean still pays.)
- **`mul_u` central choke-point** routes through the new
  `limbs_mul_u64_into` primitive when `n ≤ u64::MAX`; D76<35>
  `ln` −11 % (the choke-point lives in `scale_by_k` which is
  called multiple times per call).
- **D18 (i64) `mul` / `div`** for SCALE ≥ 10 now uses a two-limb
  base-2^64 schoolbook divide (Knuth Algorithm D) instead of the
  `__divti3` soft-call; D18<18> mul −60 % (9.2 → 3.5 ns), div
  −47 % (9.8 → 5.1 ns).

### Fixed

- Several `/ lit(2)` divisions in inverse-trig and hyperbolic
  paths replaced with `>> 1`; `q % lit(2)` replaced with
  `q.bit(0)`; `v / d + v % d` replaced with `v.div_rem(d)`.
  ~30 sites across the wide-transcendental macro and the
  policy / trig dispatch.

### Performance — findings

- **Tang `ln` ladder peaks at D1232<615>: 33.8× over v0.4.0** at
  the slot's exact SCALE. The ladder is monotone in tier except
  D924<461> where the larger table OOSes L2 on most x86 (still
  30.8× over v0.4.0).
- **Smith r/2^n joint sin/cos Taylor LOSES at narrow / medium
  tier.** Three-way confirmed across D38, D57, D76. Only
  theoretical wins start at D307+ (where the un-reduced Taylor
  balloons to 50+ terms). We retain the adaptive in-kernel Smith
  path in `exp_fixed`.
- **Comba diagonal-layout schoolbook DIES at every n.** Production
  row-major LLVM-unrolled `limbs_mul_u64_fixed` wins 14–92 % at
  n = 2..16.
- **Karatsuba / Toom–Cook crossovers don't fire** at any shipped
  width per `ROADMAP.md` — schoolbook + LLVM unroll plus the
  recursive split's heap allocs push the crossover past D1232.
- **Brent's AGM `ln` crossover empirically located at D1232 SCALE
  1000** — 3× past the textbook ~300-digit prediction. Our
  chain-MG + narrow-GUARD artanh kernel is well-tuned enough to
  delay the crossover. AGM not wired (precision lift required
  first; reserved for 0.5 cycle). Cite Brent 1976 JACM 23(2),
  Salamin 1976 Math Comp 30.
- **D9 (i32) genuinely exhausted** — LLVM inlines Granlund–Möller
  for `/ 10^9` already.

## [0.4.1] — 2026-05-19

### Removed

- **`DecimalConsts` type alias** — the deprecated alias for the
  `DecimalConstants` trait (deprecated since 0.3.3 with note
  "removed in 0.4") is gone. Callers should `use
  decimal_scaled::DecimalConstants;` instead. The trait surface,
  per-width impls, and all `pi()` / `tau()` / `e()` inherent
  methods are unchanged.

## [0.4.0] — 2026-05-19

The 0.4 release lands four roughly independent strands in one cycle:
a mechanical breaking rename + scale-cap so every width name equals
its storage's maximum decimal-digit count and at least one integer
digit is always representable; a wide-tier transcendental rewrite
that lifts a unified algorithm library into `src/algos/` and routes
every typed shell through per-family policy traits in `src/policy/`;
a `src/` reorganisation into six narrative buckets (`types/`,
`identity/`, `algos/`, `policy/`, `wide_int/`, `macros/`,
`support/`); and a supply-chain / governance pass (OpenSSF Best
Practices "Passing", OpenSSF Scorecard, `cargo-audit` + Dependabot
in CI, `SECURITY.md`, REUSE-compliant `LICENSES/`). Performance
work landed alongside: D38 transcendentals "borrow" the wider D57
work integer to shrink the strict path 5–43×, bespoke D57 lookup
kernels carry the SCALE 20 / 44–56 cells, and the wide-tier
`ln` / `exp` / `sin` numbers improve 34–244× versus the 0.2.5
baseline (see `docs/benchmarks.md` §3 and the History section).

### Breaking

- **Six widths renamed** to match the true digit ceiling of their
  underlying storage:

  | Old   | New   | Storage    |
  | ----- | ----- | ---------- |
  | `D56`   | `D57`   | `Int192`   |
  | `D114`  | `D115`  | `Int384`   |
  | `D461`  | `D462`  | `Int1536`  |
  | `D615`  | `D616`  | `Int2048`  |
  | `D923`  | `D924`  | `Int3072`  |
  | `D1231` | `D1232` | `Int4096`  |

  The seven widths whose name already matched (`D9`, `D18`, `D38`,
  `D76`, `D153`, `D230`, `D307`) keep their names.

- **`MAX_SCALE` capped at `name - 1` on every width.** A `Dxxx<SCALE>`
  with `SCALE == name` represents only `|x| < magic_constant` (no
  integer digit guaranteed). The cap restores the invariant that every
  representable value has at least one integer digit:

  | Type   | Old `MAX_SCALE` | New `MAX_SCALE` |
  | ------ | --------------- | --------------- |
  | `D9`   | 9    | 8    |
  | `D18`  | 18   | 17   |
  | `D38`  | 38   | 37   |
  | `D57`  (was `D56`)  | 57   | 56   |
  | `D76`  | 76   | 75   |
  | `D115` (was `D114`) | 115  | 114  |
  | `D153` | 153  | 152  |
  | `D230` | 230  | 229  |
  | `D307` | 307  | 306  |
  | `D462` (was `D461`) | 462  | 461  |
  | `D616` (was `D615`) | 616  | 615  |
  | `D924` (was `D923`) | 924  | 923  |
  | `D1232` (was `D1231`) | 1232 | 1231 |

- **Maximum-SCALE alias types removed.** Each width used to expose a
  scale alias at the old `MAX_SCALE`; those aliases are now gone:
  `D9s9`, `D18s18`, `D38s38`, `D57s57` (was `D56s57`), `D76s76`,
  `D115s115` (was `D114s115`), `D153s153`, `D230s230`, `D307s307`,
  `D462s462` (was `D461s462`), `D616s616` (was `D615s616`),
  `D924s924` (was `D923s924`), `D1232s1232` (was `D1231s1232`).
  The new ceiling is the one-less alias (`D38s37`, `D76s75`, ...).

- **Per-scale construction macros for the old MAX_SCALE removed.**
  `d9s9!`, `d18s18!`, `d38s38!`, `d76s76!`, `d153s153!`, `d307s307!`
  no longer exist. The proc-macro entry points (`d38!`, `d76!`, ...)
  now reject `scale == name` with a clearer "scale N exceeds max for
  Dxxx (max = N-1)" error.

- **Cargo feature flags renamed** for the six renamed widths:

  | Old      | New      |
  | -------- | -------- |
  | `d56`    | `d57`    |
  | `d114`   | `d115`   |
  | `d461`   | `d462`   |
  | `d615`   | `d616`   |
  | `d923`   | `d924`   |
  | `d1231`  | `d1232`  |

  The umbrella features (`wide`, `x-wide`, `xx-wide`) keep their names
  and have been updated to reference the new per-width names.

### Migration

For most users the upgrade is a mechanical search-and-replace:

- Rename type imports: `D56` → `D57`, `D114` → `D115`, `D461` → `D462`,
  `D615` → `D616`, `D923` → `D924`, `D1231` → `D1232`.
- Rename feature-flag mentions in your `Cargo.toml`: `d56` → `d57` etc.
- If you used a maximum-SCALE alias or wrote `Dxxx<MAX_SCALE>` literally,
  drop the SCALE by one: `D38<38>` → `D38<37>`, `D76<76>` → `D76<75>`,
  `D56<57>` → `D57<56>`, etc.
- The renamed per-scale macros are gone: `d38!(_, scale 38)` → write
  the value at SCALE 37 instead, or use `D38<37>::from_bits(...)`
  directly.

### Fixed

- **Wide-tier `FromStr` ceiling lifted.** Decimal strings can now be
  parsed at every legal `SCALE` on every wide-tier width — previously
  a u128 intermediate inside the parser silently capped the
  parseable scale at 38, so e.g. `"0.5".parse::<D307<150>>()` would
  fail with a precision error despite the type being well within
  storage range. The parser now widens through the storage integer
  directly, matching the round-trip contract `s.parse::<T>() ==
  Ok(T::from_str(&s).unwrap())` at every `SCALE ≤ MAX_SCALE`.

### Added

- **`SECURITY.md`** — vulnerability reporting policy, supported
  versions table, scope, and disclosure timeline. Required for
  OpenSSF Best Practices conformance.
- **`CONTRIBUTORS.md`** — contributor acknowledgement file at the
  repository root.
- **`LICENSES/` directory** — REUSE-compliant layout. The
  top-level `LICENSE-MIT` / `LICENSE-APACHE` text files were moved
  into `LICENSES/MIT.md` and `LICENSES/Apache-2.0.md`, and a new
  `LICENSES/THIRD-PARTY.md` enumerates third-party origins for the
  vendored algorithms.
- **OpenSSF Best Practices badge** — project 12895 reached the
  "Passing" tier. README badge ribbon extended with the badge.
- **OpenSSF Scorecard workflow** (`.github/workflows/scorecard.yml`)
  — weekly Scorecard run, results published as a repository badge.
- **`cargo-audit` workflow** (`.github/workflows/cargo-audit.yml`)
  — RustSec advisory check on every push to `main`, badge surfaced
  in the README. Generates `Cargo.lock` in-CI since the repo
  gitignores it.
- **Dependabot configuration** (`.github/dependabot.yml`) — daily
  updates for Cargo, GitHub Actions, and the `bench-history/`
  sub-crate.
- **CodSpeed continuous performance tracking**
  (`.github/workflows/codspeed.yml`) — wide-tier benches + a ULP
  precision gate run on every PR.
- **`bench-full` workflow** (`.github/workflows/bench-full.yml`)
  — matrix fanout across all 13 widths; each shard runs in its
  own runner so a multi-hour sweep no longer loses every completed
  width on a single VM eviction.
- **`bench-history` workflow** (`.github/workflows/bench-history.yml`)
  — cross-version harness in `bench-history/` that `cargo add`s
  each shipped release into a stub crate and benches them under
  matched Criterion settings, driving the new `docs/benchmarks.md`
  History section and the per-width `docs/figures/history/*.png`
  charts.
- **`bench-trial` workflow** (`.github/workflows/bench-trial.yml`)
  — focused single-cell trial bench for quick perf experiments
  without paying the full-matrix runtime.
- **`precision` workflow** (`.github/workflows/precision.yml`)
  — ULP precision gate against the in-tree `examples/ulp_report.rs`
  baseline; surfaced as a separate README badge.
- **`DynDecimal` object-safe façade** — runtime-polymorphic trait
  that lets callers hold a `Box<dyn DynDecimal>` across different
  widths / scales when the concrete type is only known at runtime.
  Re-exported from the crate root.
- **Unified `D<S, SCALE>` struct** — every shipped width
  (D9 / D18 / D38 / D57 / D76 / D115 / D153 / D230 / D307 / D462 /
  D616 / D924 / D1232) is now a type alias over a single generic
  `D<S, SCALE>` carrier rather than a hand-rolled struct per width.
  The alias names and their public API surface are unchanged.
- **Per-family policy traits** in `src/policy/` —
  `SqrtPolicy` / `CbrtPolicy` / `LnPolicy` / `ExpPolicy` /
  `PowPolicy` / `TrigPolicy`. The typed shell for each width
  picks its kernel by `impl`ing the matching policy trait against
  routines from `src/algos/`, instead of macro-emitting one
  open-coded copy of the kernel per width.
- **Algorithm library** in `src/algos/` — `sqrt/`, `cbrt/`, `ln/`,
  `exp/`, `pow/`, `trig/` each ship the kernel variants the
  policies route to: a `widen_to_d38` path for the narrow tier,
  a `borrow_d57` path that promotes D38 to D57 for tighter strict
  kernels, a `fixed_d38` path for the 256-bit handrolled `Fixed`,
  a `wide_kernel` for D76+, and bespoke `lookup_d57_*` tables for
  the SCALE 20 sqrt and SCALE 18–22 / 44–56 sincos / atan cells.
- **`borrow_d57` bridge** — D38 transcendentals (`ln`, `exp`,
  `pow`, `sin`, `cos`, `tan`, `atan`) gain a widen-to-D57 path
  that runs the strict kernel at the wider work integer and
  narrows back. Closes the SCALE 23+ accuracy gap and drops cost
  multiple × on the narrow tier (see Performance).
- **`docs/comparisons.md`** — moved-out home for the README's
  binary-vs-decimal-fraction explainer and the long-form
  comparison with other Rust decimal crates. Wired into the
  docs site nav alongside `widths.md`, `getting-started.md`,
  `strict-mode.md`, and `features.md`.

### Changed

- **`src/` reorganised into six narrative buckets.** Files moved:
  - `types/` — public typed shells (`D9` / `D18` / `D38` /
    `D57` / …), the unified `D<S, SCALE>` carrier, `traits/`
    (the `Decimal` / `DecimalArithmetic` / `DecimalConvert` /
    `DecimalTranscendental` / `DecimalConstants` / `DynDecimal`
    surface), `arithmetic.rs`, `rescale.rs`, `num_traits.rs`,
    `overflow_variants.rs`, `powers{,_fast}.rs`, `log_exp{,_fast}.rs`,
    `trig{,_fast}.rs`, `consts/`, and `widths.rs` (the former
    `core_type.rs`).
  - `identity/` — cross-type `PartialEq` / `PartialOrd` between
    every width and the primitive integer / float surface
    (the former top-level `equalities.rs`).
  - `algos/` — kernel library described above, plus `fixed_d38.rs`
    (256-bit `Fixed`, formerly `d_w128_kernels.rs`) and
    `mg_divide.rs` (formerly top-level).
  - `policy/` — per-family routing traits.
  - `wide_int/` — unchanged in role; the hand-rolled
    `Int192` / `Int384` / … storage family.
  - `macros/` — unchanged in role; declarative + procedural
    macros that emit the typed shells.
  - `support/` — `error.rs`, `rounding.rs`, `display.rs`,
    `serde_helpers.rs`, `bench_alt.rs`, `diagnostics.rs`.
- **Typed shells route through policy traits, not direct
  `algos::` calls.** Each `Dxxx::ln_strict` (etc.) body is now
  a single `<Self as LnPolicy>::ln_impl(self, …)` dispatch,
  emitted via the existing macro surface. The layering shim that
  let typed methods reach straight into the kernel during the
  pilot is gone.
- **README** — binary-vs-decimal-fraction explainer moved out
  to `docs/comparisons.md`; in-README copy slimmed to a one-
  paragraph pointer. Badge ribbon now carries OpenSSF Best
  Practices, OpenSSF Scorecard, and cargo-audit badges
  alongside the existing crates.io / docs.rs / MSRV / License /
  site / CodSpeed badges. The `docs.yml` GHA badge is now
  labelled "site" to disambiguate from docs.rs.
- **`AGENTS.md`** and **`.claude/skills/decimal-scaled/SKILL.md`**
  refreshed for the 0.4.0 layout: new width names, new src/
  bucket map, the six-bucket layering rule (typed shells call
  policy, policy calls algos, algos may call wide_int but never
  the reverse).
- **`docs/benchmarks.md`** — every per-tier arithmetic table
  (§1) and strict-transcendental table (§3) regenerated from
  the v0.4.0 sweep. New History section (§7) plotting
  `decimal-scaled` v0.2.5 → v0.4.0 deltas at D38 / D76 / D307
  for the same operations. New §0.4.0 raw-data dump appended
  in place of the prior 0.3.2 dump. Run-conditions preamble
  spells out the GitHub-hosted runner caveat (1.5–2× wall-
  clock variance per Criterion's standing guidance).

### Fixed

- **Wide-tier `FromStr` ceiling lifted.** Decimal strings can now be
  parsed at every legal `SCALE` on every wide-tier width — previously
  a u128 intermediate inside the parser silently capped the
  parseable scale at 38, so e.g. `"0.5".parse::<D307<150>>()` would
  fail with a precision error despite the type being well within
  storage range. The parser now widens through the storage integer
  directly, matching the round-trip contract `s.parse::<T>() ==
  Ok(T::from_str(&s).unwrap())` at every `SCALE ≤ MAX_SCALE`.
- **`D38<MAX_SCALE>.log10_strict`** no longer panics on inputs
  inside the storage range but close to the upper boundary. The
  intermediate `mul_div` step was overflowing the narrow work
  integer at the boundary; the fixed path uses the wider work
  integer that `log_strict` already uses.
- **`D57<MAX_SCALE>.cbrt_strict`** no longer overflows the work
  integer at the maximum scale. The cube-root reduction needed
  one extra guard digit at the storage ceiling.
- **Wide-tier `cbrt_strict`** no longer overflows the work
  integer at `SCALE = MAX_SCALE`. Generalises the D57 fix above to
  every wider tier.
- **Overflow panic messages** across every narrowing kernel now
  spell out which kernel + which direction + the offending
  value, instead of the prior generic "rescale: scale-up
  overflow" formatting. Surfaces the right call site immediately
  in downstream stack traces.
- **`mul_div_candidates` divide rounding** in the handrolled
  sanity-check path realigned with the production divide kernel.
  Previously the handrolled path rounded half-to-even
  off-by-one at the LSB on a small input class, masking real
  regressions in the production kernel.
- **Test gates promoted to module-level `#[cfg(...)]`.** A handful
  of test modules added in 0.3.x still carried runtime
  `if !COND { return; }` early-returns; those are now
  module-level cfg gates so a test never silently no-ops under a
  feature combination that disables its precondition.
- **Infinite recursion in `DecimalTranscendental` trait impl** —
  one trait-method body called the trait method instead of the
  underlying inherent method, producing a runtime stack overflow
  when called via the trait. Fixed.
- **`Fixed::rescale_down` argument order** is now `debug_assert`-
  ed at call sites — a defensive guard added after a near-miss
  during the policy-trait wiring.

### Removed

- **Top-level layering shims** — `src/lib.rs` no longer re-exports
  symbols that have moved into the bucketed layout for backward
  source-path compatibility. Public callers using the documented
  crate-root re-exports (`decimal_scaled::D38`, etc.) are
  unaffected; only callers that reached into the private path
  layout need to update.
- **`research/` references** — none ever shipped in source,
  docs, or commit messages; the folder remains private and
  out of the crate tarball.

### Performance

All numbers below come from `docs/benchmarks.md` (per-width
arithmetic tables in §1, strict transcendentals in §3,
cross-version History in §7). The 0.4 column reflects the
v0.4.0 GHA sweep on shared `ubuntu-latest` runners; cold-dev-box
re-runs typically measure 30–50 % faster on the wide tiers and
several × faster on the narrow tier's picosecond cells. Treat
the multiples as directional shape-of-change rather than as
drop-in benchmarks.

- **D38 strict transcendentals "borrow" the D57 work integer.**
  D38 `log_strict` / `log10_strict` / `exp2_strict` / `log_strict`
  / `powf_strict` now widen to D57 for the integer-kernel pass
  and narrow back. The D57 borrow path is also scale-gated for
  D38 `powf` (`SCALE >= 23`, tightened from the original
  `SCALE >= 25` after measurement).
- **Bespoke narrow-GUARD kernels on D57.** Hand-rolled kernels
  for the SCALE 20 sqrt (`algos::sqrt::lookup_d57_s20`), the
  SCALE 18..=22 sincos (`algos::trig::lookup_d57_s18_22_sincos`),
  the SCALE 44..=56 sincos / atan (`algos::trig::lookup_d57_s44_56_*`)
  and the SCALE 45..=56 exp (`algos::exp::lookup_d57_s45_56`)
  carry the cells where the generic wide kernel was the wrong
  shape for the scale.
- **D38 cos closes ~25 % of the gap to D38 sin** (joint
  sin / cos kernel re-tuned).
- **D38 sqrt + div u128 fast path** skips the 256-bit machinery
  when the inputs fit u128, recovering several ns per call on
  the narrow tier.
- **Wide-tier transcendentals (D76+)** — `pow10(w)` memoised
  per-tier in the wide transcendental cores. The cross-version
  delta vs the 0.2.5 baseline on the same operations, from
  `docs/benchmarks.md` §3:

  | op              | 0.2.5     | 0.4.0     | speedup   |
  |---              |---        |---        |---        |
  | D76<35> ln      | 1.37 ms   | 40.6 µs   | **34×**   |
  | D76<35> exp     | 1.27 ms   | 32.1 µs   | **40×**   |
  | D76<35> sin     | 1.08 ms   | 22.6 µs   | **48×**   |
  | D76<35> sqrt    | 20.5 µs   | 3.53 µs   |  **6×**   |
  | D153<75> ln     | 6.40 ms   | 67.5 µs   | **95×**   |
  | D153<75> exp    | 5.87 ms   | 53.8 µs   | **109×**  |
  | D153<75> sin    | 4.82 ms   | 40.8 µs   | **118×**  |
  | D153<75> sqrt   | 83.6 µs   | 4.98 µs   | **17×**   |
  | D307<150> ln    | 34.1 ms   | 161.3 µs  | **211×**  |
  | D307<150> exp   | 31.2 ms   | 131.5 µs  | **237×**  |
  | D307<150> sin   | 25.5 ms   | 104.4 µs  | **244×**  |
  | D307<150> sqrt  | 313 µs    | 7.84 µs   | **40×**   |

  These multiples are conservative since the 0.2 column is a
  cold-dev-box run and the 0.4 column is shared CI; the
  algorithm-of-record changes (u64-native limbs, MG 2-by-1
  reciprocal Knuth divide, Brent's two-stage exp argument
  reduction, multi-level sqrt halving in ln, [0, π/4] sin range
  reduction, sin_cos / sinh_cosh joint kernels, thread-local
  pi / ln2 / ln10 cache, pow10-cached mul / div per inner loop)
  carry the win.

### Internal

- Build-time math-constant scales lowered: `SCALE_REF` for D153 / D230
  / D307 dropped to 152 / 229 / 306 respectively (the others already
  matched the new `name - 1` ceiling).
- Bench file names tracking width (`benches/full_matrix_d56.rs`,
  `benches/lib_cmp_d1231.rs`, ...) renamed to match the new width
  identifiers.
- New `src/macros/dyn_bridge.rs` carries the `DynDecimal` blanket-
  impl bridge so every typed shell picks up the object-safe surface
  via one macro invocation.
- Wide transcendental macro module (`src/macros/wide_transcendental.rs`)
  consolidated to route through `src/policy/` rather than emitting
  one open-coded kernel per width.
- Layering rule codified: typed shells call policy, policy calls
  algos, algos may call `wide_int` but never the reverse.
- `Cargo.toml` dev-dependency `fastnum` bumped 0.4 → 0.7; benches
  follow the new API.

## [0.3.3] — 2026-05-18

Trait surface split + benchmarks refresh, all in one cycle.

### Changed

- **`Decimal` trait is now a marker supertrait** combining four
  narrower halves: `DecimalArithmetic` (operators, sign,
  pow / checked / wrapping / saturating / overflowing, reductions,
  plus the foundational type info `Storage`, `SCALE`, `MAX_SCALE`,
  `ZERO`, `ONE`, `MAX`, `MIN`, `multiplier()`), `DecimalConvert`
  (`from_bits` / `to_bits` / `scale`, integer bridges, the
  `std`-gated f64 / f32 bridge), `DecimalTranscendental` (the
  four-variant transcendental + root matrix), and
  `DecimalConstants` (`pi` / `tau` / `e` / …). Decimal carries a
  blanket impl so every type that implements all four halves is
  Decimal for free; every shipped width (D9..D1232) keeps the full
  surface.

  Callers using `T: Decimal` and `T::method()` syntax keep
  compiling unchanged — method resolution finds methods via the
  supertrait chain. The narrower bounds let generic code shrink
  its surface when it doesn't need everything, e.g.
  `fn dot<T: DecimalArithmetic + Copy>(a: &[T], b: &[T]) -> T`.

### Breaking

- **`<X as Decimal>::method` UFCS path callers** need to move to
  the right sub-trait: `<X as DecimalArithmetic>::ZERO` /
  `::MAX_SCALE` / `::multiplier` / `::sum` / `::is_zero` / etc.,
  and `<X as DecimalConvert>::from_bits` / `::to_int_with` /
  `::from_f64` / etc. Test files under `tests/` and the internal
  tests in `src/core_type.rs` have been updated as part of this
  release.

### Added

- **`DecimalArithmetic`** and **`DecimalConvert`** as new
  narrower trait surfaces (see Changed). Re-exported from the
  crate root.

### Benchmarks

- **`docs/benchmarks.md` refreshed from the 2026-05-18 sweep.**
  Every per-tier arithmetic table (§1) and every strict-
  transcendental table (§3) regenerated with the new numbers,
  preserving narrative around them.
- Caveats called out inline: narrow-tier (D9 / D18 / D38)
  picosecond-scale arithmetic sits near the bench machine's
  resolution floor and the cross-version deltas there are
  contention noise rather than real perf; narrow-tier
  transcendental values in this sweep also reflect contention
  drift (cold-machine micro-bench for D38<19> ln_strict measures
  ~54.8 µs vs the >800 µs in the full_matrix table — ~15× gap).
  Wide-tier numbers (D57+ arithmetic, D57+ transcendentals) are
  reliable and reflect real cumulative perf work from the 0.3.x
  cycle.
- The 326-measurement raw data dump from 0.3.2 stays appended at
  the end of the doc for anyone who wants every cell criterion
  produced.

## [0.3.2] — 2026-05-18

API-additive surface expansion across every decimal width plus
several downstream-blocking defect fixes. No breaking changes;
existing callers compile unchanged. Big themes: the four-variant
`_strict_with` / `_approx` / `_approx_with` matrix on every
transcendental, mode-aware constants, full serde parity, the
construction-macro surface extended to every shipped tier, a docs.rs
build fix, and primitive ergonomics on the wide-int and
fixed-decimal types.

Headline perf number for the new `_approx` family on the D38<19> ln
sanity bench: 1.17× faster at guard 25, 1.45× at 20, 1.80× at 15,
2.62× at 10, **3.70× at guard 6** — all vs strict's 54.8 µs. Strict
itself is at parity with 0.3.1 (52.3 µs prior → 54.8 µs now, within
bench noise). Full per-tier numbers in 0.3.3.

### Added

- **Four-variant matrix on every transcendental.** Each function
  ships `<fn>_strict`, `<fn>_strict_with(mode)`,
  `<fn>_approx(working_digits)`, and
  `<fn>_approx_with(working_digits, mode)`. The `_strict` body
  keeps a const-folded `STRICT_GUARD` so LLVM specialises one
  optimal kernel per `SCALE`; `_approx` takes the guard width at
  runtime for callers who want to trade ULP precision for latency.
  When `working_digits == STRICT_GUARD` the `_approx_with` body
  redirects to `_strict_with` so the fast path is never displaced.
  Covers ln / log / log2 / log10 / exp / exp2 / powf on D38 and
  the wide tiers, and sin / cos / tan / atan / asin / acos /
  atan2 / sinh / cosh / tanh / asinh / acosh / atanh / to_degrees
  / to_radians for the trig family.
- **`sqrt_strict_with(mode)` / `cbrt_strict_with(mode)` /
  `hypot_strict_with(mode)`** on D38 and the wide tiers. Roots
  have no working-digits parameter (the exact-integer-root path is
  precision-independent), so only the `_strict` / `_strict_with`
  pair exists. `Floor` and `Ceiling` dispatch by sign for `cbrt`.
- **`DecimalConsts::*_with(mode)` siblings.** `pi_with` /
  `tau_with` / `half_pi_with` / `quarter_pi_with` / `golden_with`
  / `e_with` rescale the embedded reference under a
  caller-supplied `RoundingMode`. `pi_with(Floor)` gives the
  largest representable value ≤ true π — useful for CAD
  tessellation that must not over-count.
- **`from_num` / `to_num` parity** across every width. The
  saturating `NumCast`-style bridge previously existed only on
  D38; extracted into `decl_decimal_num_traits_basics!` so D9 /
  D18 / D38 / D57 / D76 / D115 / D153 / D230 / D307 / D462 / D616
  / D924 / D1232 all get the same surface.
- **Serde for every wide tier.** Added `decl_wide_serde!`
  invocations for D57 / D115 / D230 / D462 / D616 / D924 / D1232
  — previously only D76 / D153 / D307 had Serialize / Deserialize.
- **`proc-macro-crate` resolution** in the construction macros
  (`d9!`, `d18!`, `d38!`, `d76!`, `d153!`, `d307!`). Lets consumer
  crates alias `decimal-scaled` freely instead of being forced to
  use the literal `decimal_scaled` import name. Falls back to
  `::decimal_scaled` when the lookup fails — same behaviour as
  the original hard-coded path.
- **Construction macros for every wide tier.** Added `d57!`,
  `d115!`, `d230!`, `d462!`, `d616!`, `d924!`, `d1232!` — every
  shipped Decimal width now has a compile-time literal
  constructor over the existing `expand_for(Width, …)`
  machinery, gated behind the matching feature.
- **`EPSILON` / `MIN_POSITIVE` for every decimal width.** Macro-
  emitted out of `decl_decimal_basics!` so D9 / D18 / D38 / D57 /
  D76 / D115 / D153 / D230 / D307 / D462 / D616 / D924 / D1232 all
  share the same surface. Closes the "literal `1` doesn't coerce
  to a wide-int storage type" trap that forced downstream callers
  to write `from_bits(<Int192>::from_u128(1))` longhand.
- **Wide-int `From<primitive>` impls.** Every signed wide int
  (`$S`) gets `From<u8/u16/u32/u64/u128/i8/i16/i32/i64/i128>`;
  every unsigned (`$U`) gets `From<u8/u16/u32/u64/u128>` plus a
  `from_u128` const-fn constructor.
- **Wide-int `From<float>` impls.** `From<f32>` and `From<f64>`
  for both `$U` and `$S`; nightly-only `From<f16>` / `From<f128>`
  gated behind the existing `experimental-floats` feature.
  Same saturate-on-overflow / NaN-maps-to-zero contract the
  decimal-tier float bridge uses.
- **Wider scale-alias coverage.** Backfilled D76 / D153 / D307
  from the previous 5-8 aliases each to ~25 per tier covering the
  standard decimal-precision breakpoints (1, 2, 3, 4, 6, 9, 12,
  15, 18, 20, 24, 28, 32, 35, 38, 50, plus tier-specific midpoints
  and the storage-max scale). Added `Dnns1` (one fractional digit)
  to every wide tier so all 13 widths have a consistent scale-1
  alias. Added `D57s20` for the 20-digit CAD-numeric precision
  moocad's Stage 5 needed.

### Fixed

- **docs.rs build for 0.3.1 failed** with
  `E0425: cannot find type D924`. `src/core_type.rs`'s
  `D616::widen` returned `D924<SCALE>` unconditionally, but `D924`
  is gated behind `xx-wide` — which the docs.rs feature set didn't
  enable. Split `D616`'s impl block in two so `widen` only exists
  when D924 does, and added `xx-wide` to the docs.rs
  `[package.metadata.docs.rs]` feature list for a complete
  rendered surface.

### Documentation

- f64-boundary warning on `DecimalConsts`: `to_f64()` is itself
  correctly rounded, but `pi() → to_f64()` at `SCALE < 15` lands
  ~466 ULPs from `std::f64::consts::PI` because the decimal value
  is intrinsically coarser than the f64 grid. Downstream code
  that uses the f64 to count, bucket, or seed an iteration loop
  should source mathematical constants from `std::f64::consts`
  directly at the boundary, or pick `SCALE ≥ 15`.
- Module-header rewrite on `log_exp_strict`, `trig_strict`,
  `powers_strict`, and the wide-tier `wide_transcendental` macro
  describing the four-variant matrix and when each form is
  appropriate.

### Benches

- Split the monolithic `full_matrix` bench into per-width binaries
  (`full_matrix_d{9,18,38,56,76,114,153,230,307,461,615,923,1231}`)
  sharing a `full_matrix_common.rs` macros module. A multi-hour
  sweep used to lose all completed widths on a power-loss / lid
  close; per-width binaries let criterion flush each width before
  starting the next.
- `scripts/bench_pinned.ps1` and `scripts/bench_sweep.ps1`: 2-lane
  parallel runner that pins each lane to a physical core (both SMT
  siblings) at High priority. Skips any bench whose log already
  shows completion, so a partial sweep resumes from where it left
  off in one command.

## [0.3.1]

Release-process patch. Library code, public API, on-wire format,
and bench numbers are byte-identical to 0.3.0.

### Fixed

- **GitHub Pages docs site (`mootable.github.io/decimal-scaled`)
  failed to refresh on the v0.3.0 tag push.** The Pages environment
  protection rule only allows deploys from `main`; the concurrent
  main-push run that *was* allowed to deploy got cancelled by the
  tag-push run that arrived right after. Tag-triggered deploys
  blocked. 0.3.1 ships as a `main`-branch push so the docs
  workflow runs to completion.

### Notes

- Future releases should land the version-bump commit, let
  `main` build + deploy the docs, *then* tag — not the other
  way round. Will codify in `scripts/deploy.ps1`.

## [0.3.0]

The half-width-tier release. The decimal ladder now goes
**D9 → D18 → D38 → D57 → D76 → D115 → D153 → D230 → D307 →
D462 → D616 → D924 → D1232** — every adjacent pair has a
lossless `From` / `widen()` plus a fallible `TryFrom` /
`narrow()`. New `x-wide` and `xx-wide` Cargo umbrellas gate the
wider ranges so default builds stay lean.

The strict / fast dispatcher rule changes too: **strict is now
the default plain dispatch in every build, and wins on tiebreak
when both `strict` and `fast` are enabled.** The only way to
land plain `sin` / `ln` / `sqrt` etc. on the f64 bridge is a
deliberate three-step opt-out (`default-features = false` + add
`fast` + add `std` + don't re-add `strict`). The named
`*_strict` and `*_fast` methods stay available regardless of
feature choice.

### Added — new decimal tiers

- **`D57` (192-bit), `D115` (384-bit), `D230` (768-bit)** —
  half-width tiers between every existing power-of-two width.
  Gated behind `d57` / `d115` / `d230` individually or by the
  expanded `wide` umbrella (now `D57`–`D307` together).
- **`D462` (1536-bit), `D616` (2048-bit)** — new x-wide tier,
  gated behind `x-wide` (or `d462` / `d616` individually).
- **`D924` (3072-bit), `D1232` (4096-bit)** — new xx-wide tier,
  gated behind `xx-wide` (or `d924` / `d1232` individually).
- The naming rule: the number on every `D{N}` type is the
  highest safe `SCALE` (`MAX_SCALE`) the storage can hold, i.e.
  the number of decimal digits you can represent without
  overflow. So `D1232` means `MAX_SCALE = 1232` (∼ 1232 decimal
  digits of headroom), not "1231 bits".
- Comprehensive scale aliases per the new tiers: ≥ 16 per
  tier above the narrow range, covering 0 / common midpoints /
  the previous tier's MAX_SCALE as a cross-tier sentinel /
  the new tier's MAX_SCALE.

### Changed — breaking

- **`D38.widen()`** now returns `D57<SCALE>` instead of
  `D76<SCALE>`. Symmetrically, `D76.narrow()` → `D57`,
  `D76.widen()` → `D115`, `D153.narrow()` → `D115`,
  `D153.widen()` → `D230`, `D307.narrow()` → `D230`, and
  `D307.widen()` is new (→ `D462`, gated behind `x-wide`). The
  legacy power-of-two-next-up semantics are gone; the
  comprehensive ladder is the new default. Callers that need
  the old jump can use `.into()` / `.try_into()` to skip rungs.
- **`strict` + `fast`** now resolves to strict. Previously
  `fast` won the tiebreak. The strict path is now fast enough
  (`ln_strict` at D38<19> is ~1.5 µs) that staying on the
  deterministic correctly-rounded path by default is the right
  call across more codepaths.
- **`(no feature)` builds** now dispatch plain transcendentals
  to `*_strict` too (previously they fell through to `*_fast`).
  Same reasoning: strict-by-default unless deliberately opted
  out.

### Added — performance

- **Chain-of-÷10^38 rescale** for wide-tier `mul` at `SCALE > 38`:
  factors `n / 10^SCALE` as a sequence of `n / 10^38` chunks,
  each riding the existing base-2^128 MG 2-by-1 magic kernel.
  Combined-remainder bookkeeping preserves HalfToEven
  correctness across chunks. Measured wins:
  - D307<150> mul: 786 ns → 434 ns (1.8× faster)
  - D462<230> mul: 1.62 µs → 866 ns (1.9×)
  - D616<308> mul: 2.20 µs → 1.36 µs (1.6×)
  - D924<461> mul: 3.30 µs → 2.68 µs (1.2×)
  - D1232<616>: marginal (chain length eats the per-pass win;
    needs Barrett or wider magic tables — tracked for 0.3.x).
- **`d57`/`d115`/.../`d1232` per-tier features** can be enabled
  individually if you don't want a full umbrella.

### Added — benches + tooling

- **Per-width `lib_cmp_d{N}` benches** — 13 new bench binaries
  (`cargo bench --bench lib_cmp_d307`) replace the monolithic
  `library_comparison.rs`. Each tier runs in minutes instead
  of hours; iterating on one tier's perf doesn't need a full
  matrix sweep. Shared macros + helpers live in
  `benches/lib_cmp_common.rs`.
- **`benches/quick_div.rs`** — focused microbench for
  D307/D616/D924/D1232 div + mul. Used during the wide-tier
  perf tuning passes.
- **Per-width summary chart family** — one PNG per power-of-two
  storage width (`docs/figures/library_comparison/summary_{N}bit.png`)
  showing every op (add / sub / neg / mul / div / rem / sqrt /
  ln / exp / sin / cos / tan / atan / sinh / cosh / tanh) on a
  log-y axis with one bar per library. Re-rendered automatically
  via the `scripts/refresh_bench_artifacts.sh` workflow.
- **`scripts/bench_log_to_medians.py`** — extracts the criterion
  medians from any number of bench-log files into
  `target/medians.tsv` so chart_gen.rs picks up the latest run.
- **chart_gen filter** tightened: only renders multi-library
  line charts where ≥ 2 libraries have ≥ 2 data points, so
  scatter-of-dots charts get dropped automatically.
- **Trig family in the bench matrix** — `cos` / `tan` / `atan` /
  `sinh` / `cosh` / `tanh` benched for every peer that ships
  them (decimal-scaled, fastnum, g_math, rust_decimal cos/tan).
- **`examples/rounding_mode_probe.rs`** — diagnostic that prints
  the candidate renderings under each rounding mode for `exp(1)`,
  `sin(1)`, `ln(2)`, `sqrt(2)`. Used to verify that the §5
  "1 ULP" entries on `fastnum` / `rust_decimal` were
  render-mode artifacts (they carry the correct value
  internally), not computation errors.
- **`examples/fast_vs_strict_ulp.rs`** — per-tier accuracy-loss
  table for `*_fast` vs `*_strict`. Drives the new §2.1 in
  `docs/benchmarks.md`.

### Fixed

- **D57 work-integer overflow** — D57's transcendental work
  integer was Int512, which couldn't carry the squared
  intermediate at `SCALE + GUARD = 86` working scale. Bumped
  to Int1024. Caught by the in-flight bench sweep.
- **`feature = "xx-wide"`-only builds** — several macro arms
  in `src/macros/full.rs` and `src/mg_divide.rs` gated only on
  `wide` / `x-wide` and missed `xx-wide` / `d924` / `d1232`,
  so an `xx-wide`-only build failed to compile. Gates extended.

### Docs

- **§5 in `docs/benchmarks.md`** rewritten as "Where each crate
  fits" — feature-matrix-first framing, per-storage-width
  summary charts, an explicit note distinguishing render-mode
  artifacts (fastnum / rust_decimal / decimal-rs) from real
  precision losses (dashu-float 4 ULP exp(1), g_math 6–46 ULP).
  Bench-doc tone shifted from "competition" to "here's where
  each crate fits, here's where ours fits".
- **`docs/widths.md`**, **`docs/getting-started.md`**,
  **`docs/strict-mode.md`**, **`docs/features.md`** all updated
  to enumerate the thirteen-tier ladder and describe the new
  strict-by-default dispatcher.
- **README** — "Two headline guarantees" lede now promotes both
  ≤ 0.5 ULP correctness AND caller-chosen rounding mode at every
  lossy operation; tier table extended to all 13 widths.
- **`ALGORITHMS.md`** — wide-int section enumerates every
  shipped storage type; atan halvings table extended to cover
  the new wide tiers; tier-listing references updated from
  the old four-tier list.
- **`ROADMAP.md`** — explicit versioning intent table (0.3.0
  done / 0.4.0 signed SCALE + RNG / 1.0.0 gated on competitive
  wide-tier mul/div or per-row documented gap); MG magic-multiply
  extension + Barrett path queued for 0.3.x; out-of-tree
  ecosystem crates (decimal-scaled-expr / -math / -finance)
  with the expression-engine dual-track + whole-tree
  serialisation design captured.

## [0.2.5]

Docs + benchmark accuracy patch. Library code, public API, and
on-wire format are byte-identical to 0.2.4.

### Fixed

- **`docs/benchmarks.md`** - every numeric cell in the
  arithmetic, fast-transcendental, strict-transcendental, and
  wide-integer-backend tables was re-measured on a single
  machine in one criterion run with the default 3 s warm-up,
  50-sample (D38-and-narrower) or 20-sample (wide tier) windows.
  The previous numbers were collected with a much shorter warm-up
  / fewer samples and several rows shipped unsubstituted
  `__LOSSY_*__` / `__STRICT_*__` template placeholders.
- **`benches/decimal_backends.rs`** - the `D128_lossy` and
  `D256_lossy` rows called the plain `*` dispatcher methods,
  which with the default `strict` Cargo feature flip to the
  `*_strict` integer kernel. The rows therefore measured the
  strict path twice instead of contrasting fast vs strict. They
  now call `*_fast` explicitly, so the fast / strict distinction
  shown in the docs is honest.

### Changed

- **`docs/benchmarks.md` §2 "Fast transcendentals"** - table
  reshaped from the unsubstituted "D9 / D18 / D38 fast"
  placeholders to the actually-benched
  "D38 `*_fast` / D76 `*_fast` / `rust_decimal`" comparison,
  with a prose note that D9 / D18 `*_fast` share the D38
  f64-bridge kernel via `to_f64` / `from_f64` and incur only a
  sub-ns round-trip on top of the listed D38 numbers.
- **`docs/benchmarks.md` methodology section** - warm-up /
  sample-size text updated to match the bench harness's actual
  configuration (3 s warm-up, auto-tuned measurement window,
  50 or 20 samples depending on tier).
- **`docs/benchmarks.md` §3 strict transcendental tables** -
  collapsed from one column per (width, scale) to one column per
  width, each cell showing only the **s = mid** measurement (the
  honest series-cost scale - s = 0 hits fast-path early returns
  and s = max sometimes shortens via Cody-Waite range reduction,
  so neither is a fair comparator). The chosen mid is listed in
  the column header (e.g. `D76 (s=35)`).
- **`docs/benchmarks.md` Time units table** - added picosecond
  row and reframed the third column as "Relative to a second"
  instead of "Relative to `ns`" for consistency across the
  table.
- **`docs/benchmarks.md` data-cell rendering** - scientific
  notation in data cells (e.g. `1.46×10⁻³ µs`) replaced with
  plain decimals (`0.00146 µs`). Scientific notation is now
  reserved for values smaller than 10⁻⁵ of the row's unit
  (none in the current tables). Time units table is unchanged
  (still uses `10⁻³ s`, `10⁻⁶ s`, etc. for second relationships).

### Added

- **`benches/library_comparison.rs`** - new bench that pits
  `decimal-scaled` against every viable peer on crates.io
  (`fastnum`, `bigdecimal`, `dashu-float`, `decimal-rs`,
  `rust_decimal`, `fixed::I*F*`, `g_math`) across all six
  decimal-scaled width tiers (32 / 64 / 128 / 256 / 512 /
  1024 bit) at three scales per tier (s=0, s=mid, s=max).
- **`examples/ulp_report.rs`** - one-shot accuracy report
  measuring ULP error for each library's
  `ln(2)` / `exp(1)` / `sin(1)` / `sqrt(2)` against a
  `D76<19>` integer-only `*_strict` baseline. Confirms
  `decimal-scaled` is 0 ULP on every transcendental tested
  and shows `g_math`'s "0 ULP transcendentals" marketing
  claim is 6–46 ULP off at the matched precision.
- **`examples/chart_gen.rs`** - pure-Rust (`plotters`) chart
  generator that reads `target/medians.tsv` and emits one
  layered line chart per (op × width) to
  `docs/figures/library_comparison/`. 60 PNGs total; the
  meaningful-variation subset is embedded in `docs/benchmarks.md`
  §5.
- **`docs/figures/library_comparison/*.png`** - 52 generated
  charts (every op × width combination that has measurements).
- **`docs/benchmarks.md` §5 Library comparison** - new
  chapter with one speed table per width tier (s=mid
  representative scale, library-by-library), an accuracy
  table at the 128-bit tier (ULP errors for every supported
  transcendental across every library), embedded charts for
  the meaningful-variation ops, and a "reading the comparison"
  buyers-guide paragraph that maps "what do you need" →
  "which crate".
- **Dev-dependencies**: `fastnum`, `bigdecimal`,
  `dashu-float`, `decimal-rs`, `scientific`, `plotters` -
  all bench/example-only, none compiled into a normal build.
- **`ROADMAP.md`** (repo root) - tracked list of throughput
  gaps surfaced by the §5 library comparison and the planned
  fix per item (Burnikel-Ziegler divide, Karatsuba/Toom-3
  mul, `*_approx(working_digits)` transcendental family).
  Cross-linked from `docs/benchmarks.md` Roadmap section.

## [0.2.4]

Agent-ecosystem additions. No library code changes - the crate's
public API, behaviour, and on-wire format are byte-identical to
0.2.3. The bump exists so the new agent-facing assets ship in the
crates.io tarball.

### Added

- `AGENTS.md` (top level) - tool-agnostic usage guide consumable by
  Cursor, Continue, Aider, Codeium and any other agent runner that
  crawls a repo for `AGENTS.md`. Covers width / scale picking, the
  strict-vs-fast dual API, rounding modes, `DecimalConsts`,
  rescaling, serde format, common anti-patterns, Cargo feature
  cheat sheet, and quick recipes.
- `.claude/skills/decimal-scaled.md` - Claude Code skill (same
  content as `AGENTS.md`) with `name` / `description` frontmatter
  so Claude Code can auto-discover and invoke the skill when a user
  prompt mentions the crate.

## [0.2.3]

Documentation patch (and matching test additions). The 0.2.2 docs
incorrectly listed `golden` among the constants that don't fit
`D38<38>`'s storage range - the code was correct (golden ≈ 1.618 is
inside the ±1.70141 storage range, so the method returns the
correctly-rounded value), but the prose and CHANGELOG copy claimed it
panicked. Fixed.

### Fixed

- Documentation in `consts.rs` (module preamble + `DecimalConsts`
  trait doc) and `CHANGELOG.md` for 0.2.2 said the
  larger-magnitude constants that overflow `D38<38>` storage were
  "pi, tau, e, **golden**". `golden ≈ 1.61803` actually fits the
  ±1.70141 storage range and the method returns the
  correctly-rounded value; only `pi` (3.14), `tau` (6.28), and
  `e` (2.72) overflow. Docs corrected.

### Added

- `tests/consts.rs` inline mod: explicit `#[should_panic]` tests
  pinning `D38<38>::pi()`, `D38<38>::tau()`, and `D38<38>::e()` to
  the storage-overflow panic. Promotes the prior single-constant
  spot test to cover all three.
- The `fitting_constants_at_scale_38_are_correctly_rounded` test now
  also asserts `D38<38>::golden()` is correctly rounded to 0.5 ULP
  (= 1 LSB) of the canonical 38-digit value.

## [0.2.2]

`DecimalConsts` 0.5-ULP contract is now uniform across every supported
scale - the 0.2.0 / 0.2.1 ≈ 5 ULP "exception at `D38<38>`" is gone.

### Changed

- **`DecimalConsts` reference precision** - every constant on D9 /
  D18 / D38 is now derived from the **75-digit `Int256` reference**
  (the same one the wide tier already used), rescaled **down**
  half-to-even to the caller's `SCALE`. The previous code used a
  37-digit `i128` reference and *rescaled upward* by 10 at
  `D38<38>`, which appended a placeholder zero and left the result
  ≈ 5 ULP off the canonical value. Every result on every supported
  scale on every width is now within **0.5 ULP** of the canonical
  decimal expansion - the precision contract holds with no
  documented exceptions.
- **`D38<38>` storage-range overflow** - at `SCALE = 38` the D38
  storage range is approximately ±1.7, so the four
  larger-magnitude constants (`pi ≈ 3.14`, `tau ≈ 6.28`,
  `e ≈ 2.72`, `golden ≈ 1.62`) genuinely cannot be represented.
  The corresponding methods previously panicked with the generic
  rescale message `D38::rescale: scale-up overflow`; they now panic
  with the explicit `D38 constant out of storage range: <name>
  cannot fit i128 at SCALE = 38`. `D38<38>::half_pi()` and
  `D38<38>::quarter_pi()` (which fit storage) are correctly rounded
  to 0.5 ULP - verified by a new test asserting `|result − truth|
  ≤ 1 LSB` at the 38-digit storage scale.

### Fixed

- The "`D38<38>` ≈ 5 ULP exception" mentioned in 0.2.1's
  `DecimalConsts` module / trait docs is removed from both the
  preamble and the trait blurb; the rewritten docs state the now-
  uniform 0.5 ULP contract.
- `docs/strict-mode.md` "Choosing the configuration" table reflowed:
  the default is `strict` (not the f64 bridge), and the `fast`
  feature row no longer claims it drops the `*_strict` surface
  (corrected in 0.2.0 elsewhere; the table row was missed).

## [0.2.1]

Documentation patch - no API changes.

### Fixed

- **docs.rs build**: the rendered crate page on https://docs.rs/decimal-scaled
  was only showing the default-feature surface, so the wide-tier types
  (`D76` / `D153` / `D307`), the `dNN!` proc-macros, and the `serde_helpers`
  module were missing. Added a `[package.metadata.docs.rs]` block that
  enables `std`, `serde`, `strict`, `macros`, `wide`, and `x-wide` for the
  docs build. The matching `docsrs` cfg is also wired so future
  `#[cfg(docsrs)]` doc-cfg badges can highlight feature-gated items.
- **`consts` module + `DecimalConsts` trait docs**: the preamble and per-
  method blurbs claimed every constant was rescaled from a single
  37-digit `i128` reference. That was true for D9/D18/D38 but ignored
  the per-tier raw references shipped for D76 (75 digits), D153 (153
  digits), and D307 (307 digits) under `consts_wide.rs`. The new docs
  include a per-tier reference table and an explicit statement of the
  precision contract: within 0.5 ULP at every supported scale on every
  width, with the documented exception of `D38<38>` (the D38 maximum,
  rescaled upward by 10 from the 37-digit reference - ≈ 5 ULP bound
  on `pi` / `tau` / `e` / `golden`).

## [0.2.0]

The 0.2 release rounds out the family the 0.1 line scaffolded: every
width ships the full method surface, the `Decimal` trait carries the
width-generic API, and the strict / fast routing is symmetric and
explicit.

### Added

- **Wide tier (D76 / D153 / D307)** - the 256 / 512 / 1024-bit decimal
  widths are now feature-complete. Each implements every surface D38
  has: arithmetic and bitwise operators, sign methods, integer
  methods, overflow variants, pow + powi + the four pow overflow
  variants, cross-type `PartialEq` against every primitive integer
  and float, the float bridge (`from_f64`, `from_f64_with`, `to_f64`,
  `to_f32`), serde round-trip, and the full strict-transcendental
  surface - every `*_strict` method plus a mode-aware
  `*_strict_with(mode)` sibling. Two AGM alternates `ln_strict_agm` /
  `exp_strict_agm` (Brent–Salamin 1976, Newton-on-AGM-ln) are exposed
  alongside the canonical artanh / Taylor paths.
- **In-tree wide-integer module** (`crate::wide_int`) - the wide tier
  is now backed by a hand-rolled `Int256` / `Int512` / `Int1024` /
  `Int2048` / `Int4096` family (plus unsigned siblings) emitted by a
  macro. No external big-integer dependency. Includes Karatsuba
  multiplication (dispatched at the 16-limb threshold), Knuth
  Algorithm D, and a Burnikel–Ziegler recursive divide wrapper.
- **`Decimal` trait - expanded surface** - the trait now carries every
  uniform method every width implements: arithmetic, bitwise, and
  shift operators as supertrait bounds; sign (`abs`, `signum`,
  `is_positive`, `is_negative`); integer methods (`div_euclid`,
  `rem_euclid`, `div_floor`, `div_ceil`, `abs_diff`, `midpoint`,
  `mul_add`); integer-shape predicates (`is_nan`, `is_infinite`,
  `is_finite`); the full pow + checked/wrapping/saturating/overflowing
  pow family; the full `checked_*`/`wrapping_*`/`saturating_*`/`overflowing_*`
  of `add`/`sub`/`mul`/`div`/`neg`/`rem`; integer conversion
  (`from_i32`, `to_int`, `to_int_with`); the float bridge gated on
  `std`; and default reductions (`is_zero`, `is_one`, `is_normal`,
  `sum`, `product`). Plus `Debug`/`Display`/`Hash` supertraits.
- **`d9!` / `d18!` / `d76!` / `d153!` / `d307!` proc-macros** -
  matching `d38!` per-width entry points, including:
  - per-scale wrappers (`d38s12!`, `d18s6!`, etc.) that pre-bake the
    scale qualifier;
  - radix prefix integers (`0xFF`, `0o755`, `0b1010`);
  - the explicit `radix N` qualifier;
  - fractional radix literals (`d76!(1.A3, radix 16, scale 12)`);
  - explicit `scale N` and `rounded` qualifiers.
- **`*_strict` and `*_fast` named methods always available** - both
  surfaces compile in every feature configuration (subject only to
  dependency gates - `*_fast` needs `feature = "std"`). The plain
  `*` form is the only thing the `strict` / `fast` features control.
- **`widen()` / `narrow()` hop methods** - promote to the next storage
  tier or demote with a fallible narrowing, without the longhand
  `From::from` / `TryFrom::try_from` syntax.
- **`rescale_with(mode)` mode-aware sibling** on every width.
- **`with_scale<TARGET>()` builder-style alias** for `rescale`.
- **`*_with(mode)` siblings throughout** - every default-rounding
  operation (`from_f64`, `to_int`, `rescale`, etc.) now has a sibling
  taking an explicit `RoundingMode`.
- **`from_num` / `to_num`** on D38 (in `src/num_traits.rs`, renamed
  from `fixed_compat`) - saturating, never-panicking constructors and
  readers that thread through `num_traits::NumCast`.
- **`hypot_strict`** on every width - integer-only, correctly-rounded
  `sqrt(a² + b²)` via the scale-trick algorithm.

### Changed

- **Type names** - public types now name themselves by *safe decimal
  digit capacity* (`D9` / `D18` / `D38` / `D76` / `D153` / `D307`)
  rather than by underlying integer bit-width. The number in the
  type name is also the type's `MAX_SCALE`.
- **Strict mode is the default** - `default = ["std", "serde", "strict"]`.
  Build without default features to get the f64-bridge path.
- **`*_fast` (formerly suffix-free)** - the f64-bridge methods are
  now named `*_fast` (`ln_fast`, `sin_fast`, …) for symmetry with
  `*_strict`. Plain `*` is the feature-driven dispatcher.
- **`fast` feature contract** - no longer drops the `*_strict`
  surface; only forces plain `*` to resolve to `*_fast`.
- **`Decimal` trait supertrait bounds** - extended with `Default`,
  `Debug`, `Display`, `Hash`, all arithmetic / `*Assign` operators,
  and the full bitwise / shift operator set.
- **`fixed_compat.rs` → `num_traits.rs`** - file renamed; module docs
  no longer reference the `fixed` crate. The `from_num` / `to_num`
  methods themselves are unchanged.
- **README, docs/, and crate-level documentation** rewritten to
  reflect the all-six-widths reality. Stale claims about
  D38-only-implements-trait, bnum-backed wide tier, "wide tier not
  yet wired", "Karatsuba is a future optimisation", and "fast drops
  the strict surface" are all corrected.

### Removed

- **The `bnum` dependency** - wide-tier storage migrated to the
  in-tree `wide_int` module. `bnum` and friends remain as
  `[dev-dependencies]` for the benchmark baselines only.
- **`_lossy` / `_fast` float-conversion suffixes** - the float
  conversion methods are now `to_f64`, `from_f64`, `to_f32`,
  `from_f64_with`. The historic `_lossy` / `_fast` suffixes were
  redundant since there is no strict counterpart for these (they
  are type conversions, not transcendentals).
- **`_lossy` / `_int` integer-conversion suffixes** dropped for the
  same reason - `from_int` / `to_int` / `to_int_with` are the only
  variants.
- **Placeholder wide-tier feature flags** (`d115`, `d230`, `d462`,
  `d616`, `d924`, `d1232`) - these were forward-planned widths that
  were never implemented. Shipping no-op feature flags would mislead
  users pinning to them. Will be re-added when the corresponding
  storage types land.
- **Dead code in `mg_divide`** - the unused `div_exp_fast_2word`
  wrapper (only the `_with_rem` variant has callers).
- **Inline test mods that ran without asserting** - the runtime
  `if !DEFAULT_IS_HALF_TO_EVEN { return; }` guard pattern was
  replaced with module-level `#![cfg(...)]` so tests never silently
  no-op under a non-default `rounding-*` feature.

### Fixed

- **Strict/fast routing defect** - pre-0.2 the `*_strict` methods
  were `cfg(not(feature = "fast"))` and the `*_fast` methods were
  `cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))`,
  so in the default-strict build there was no way to call the
  f64-bridge methods by name, and vice versa. Both surfaces now
  always compile (subject to `std` for `*_fast`).
- **Module-level doc comment staleness** - six modules contained
  D38-only narratives / "Phase N will add" / "future widths" /
  broken file-path references; rewritten to match the all-six-widths
  reality.
- **Broken intra-doc links** - `[Self::MIN]` at module scope,
  `[FromStr]` without `core::str::` prefix, `[D38::rescale]` at
  module scope, `[num_traits::Zero]` shadowed by the post-rename
  `crate::num_traits` mod - all fixed. `cargo doc --no-deps
  --document-private-items` now reports zero warnings.
- **Crate-wide warning-clean build** under every feature
  combination - `default`, `--no-default-features`, `fast`, `strict`,
  `wide`, `x-wide`, and combinations thereof.
- **Coverage hardening** - comprehensive functional tests added for
  every public surface, the wide-integer kernels, `mg_divide`, the
  guard-digit `d_w128_kernels`, and every transcendental's domain
  panic. Tests are functional (named by behaviour, not by uncovered
  line) and topic-organised in `tests/`.

### Compile-time / MSRV

- **MSRV** declared as **Rust 1.85** (lower bound for the 2024
  edition).

### Migration notes

- The `D128` (etc.) type names are gone - they were renamed to their
  digit-capacity counterparts in the 0.1 line. If you pinned to a
  pre-rename name, update to the new spelling.
- Code that called `.ln()` / `.sin()` etc. and relied on the f64
  bridge being present in the default strict build now still
  compiles, but the routing has been clarified - call `.ln_fast()`
  / `.sin_fast()` directly if you specifically want the f64 path
  regardless of the build's feature set.
- The `_lossy` / `_fast` suffixes on float conversion methods
  (`to_f64_lossy`, `from_f64_fast`, …) have been removed across two
  prior renames; the methods are now just `to_f64` / `from_f64` /
  etc. Update any leftover suffixed call sites.
- If you depended on a placeholder wide-tier feature flag (`d115`,
  `d230`, `d462`, `d616`, `d924`, `d1232`), the flag no longer
  exists. Use `wide` or `x-wide` to cover the implemented widths.

## [0.1.1] - 2025-12

Bug-fix release of the initial public 0.1 line. Refer to the git
history under tag `v0.1.1` for the full commit log; the changes
focused on the repo URL / documentation metadata.

## [0.1.0] - 2025-12

Initial public release. Established the core `D38<const SCALE: u32>`
type, the strict-vs-fast transcendental dual API, the 256-bit
Möller-Granlund magic-number divide path for mul/div, the
correctly-rounded sqrt / cbrt via exact-integer radicand, the serde
helpers, the `d128!` macro, and the docs/benchmarks scaffolding.
