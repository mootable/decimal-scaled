# Changelog

All notable changes to `decimal-scaled` are documented here.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]

The half-width-tier release. The decimal ladder now goes
**D9 → D18 → D38 → D56 → D76 → D114 → D153 → D230 → D307 →
D461 → D615 → D923 → D1231** — every adjacent pair has a
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

- **`D56` (192-bit), `D114` (384-bit), `D230` (768-bit)** —
  half-width tiers between every existing power-of-two width.
  Gated behind `d56` / `d114` / `d230` individually or by the
  expanded `wide` umbrella (now `D56`–`D307` together).
- **`D461` (1536-bit), `D615` (2048-bit)** — new x-wide tier,
  gated behind `x-wide` (or `d461` / `d615` individually).
- **`D923` (3072-bit), `D1231` (4096-bit)** — new xx-wide tier,
  gated behind `xx-wide` (or `d923` / `d1231` individually).
- The naming rule: the number on every `D{N}` type is the
  highest safe `SCALE` (`MAX_SCALE`) the storage can hold, i.e.
  the number of decimal digits you can represent without
  overflow. So `D1231` means `MAX_SCALE = 1232` (∼ 1232 decimal
  digits of headroom), not "1231 bits".
- Comprehensive scale aliases per the new tiers: ≥ 16 per
  tier above the narrow range, covering 0 / common midpoints /
  the previous tier's MAX_SCALE as a cross-tier sentinel /
  the new tier's MAX_SCALE.

### Changed — breaking

- **`D38.widen()`** now returns `D56<SCALE>` instead of
  `D76<SCALE>`. Symmetrically, `D76.narrow()` → `D56`,
  `D76.widen()` → `D114`, `D153.narrow()` → `D114`,
  `D153.widen()` → `D230`, `D307.narrow()` → `D230`, and
  `D307.widen()` is new (→ `D461`, gated behind `x-wide`). The
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
  - D461<230> mul: 1.62 µs → 866 ns (1.9×)
  - D615<308> mul: 2.20 µs → 1.36 µs (1.6×)
  - D923<461> mul: 3.30 µs → 2.68 µs (1.2×)
  - D1231<616>: marginal (chain length eats the per-pass win;
    needs Barrett or wider magic tables — tracked for 0.3.x).
- **`d56`/`d114`/.../`d1231` per-tier features** can be enabled
  individually if you don't want a full umbrella.

### Added — benches + tooling

- **Per-width `lib_cmp_d{N}` benches** — 13 new bench binaries
  (`cargo bench --bench lib_cmp_d307`) replace the monolithic
  `library_comparison.rs`. Each tier runs in minutes instead
  of hours; iterating on one tier's perf doesn't need a full
  matrix sweep. Shared macros + helpers live in
  `benches/lib_cmp_common.rs`.
- **`benches/quick_div.rs`** — focused microbench for
  D307/D615/D923/D1231 div + mul. Used during the wide-tier
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

- **D56 work-integer overflow** — D56's transcendental work
  integer was Int512, which couldn't carry the squared
  intermediate at `SCALE + GUARD = 86` working scale. Bumped
  to Int1024. Caught by the in-flight bench sweep.
- **`feature = "xx-wide"`-only builds** — several macro arms
  in `src/macros/full.rs` and `src/mg_divide.rs` gated only on
  `wide` / `x-wide` and missed `xx-wide` / `d923` / `d1231`,
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
