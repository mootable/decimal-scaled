# Contributing to `decimal-scaled`

This guide is aimed at contributors who want to **tune a specific
`(width, scale)` cell** — pick a faster kernel for a particular
`Dxx<S>` value — and then **prove the win** with the project's
existing perf infrastructure.

It assumes you have read [`README.md`](https://github.com/mootable/decimal-scaled/blob/main/README.md) and have a working
`cargo build --features wide,x-wide,xx-wide,macros`.

The crate's correctness contract is fixed:
[≤ 0.5 ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place) for
strict transcendentals, identical bit-patterns across platforms.
Performance tuning never trades that away.

---

## 1. The algorithm library

The hot kernels live in [`src/algos/`](https://github.com/mootable/decimal-scaled/tree/main/src/algos/), one subdirectory
per mathematical family:

```
src/algos/
├── cbrt/
├── exp/
├── ln/
├── pow/
├── sqrt/
└── trig/
```

Each function directory holds one **named file per algorithm** — a free
function that takes the raw storage integer (`Int<N>`) plus runtime
parameters and returns a raw storage integer. None of them know about
the typed `D<Int<N>, SCALE>` wrapper.

A file is named for its *algorithm*, never its tier:
`<func>_<algorithm>[_<precondition>]`. A kernel valid for all widths
carries no precondition suffix; one valid only for a specific limb-count
or scale band names that band in **limbs / scale**, never a `Dxx` alias.

| File                          | What it is                                                        |
|-------------------------------|-------------------------------------------------------------------|
| `sqrt_newton.rs`              | Newton `isqrt`, generic over `N` — the all-widths default         |
| `sqrt_mg_divide.rs`           | Hand-tuned 256-bit isqrt for `Int<2>` storage (D38, D18 widened)  |
| `exp_series_2limb.rs`         | Maclaurin series in the 256-bit `Fixed`, narrow (`Int<2>`)        |
| `sincos_tang_3limb_s18_22.rs` | Tang sin/cos on a 3-limb work integer, scale range 18..=22        |

File names describe the algorithm, not the type. One algorithm may serve
many cells; one cell may have several competing. Algorithms are **never
deleted** — an unwired kernel or a today-loser is a *kept alternative*
(`docs/ARCHITECTURE.md` → "Keeping the alternatives").

### Which kernel runs where — the policy matcher

The wiring between **a typed value** and **a kernel** lives in
[`src/policy/<func>.rs`](https://github.com/mootable/decimal-scaled/tree/main/src/policy/) (decimal) /
`src/int/policy/<func>.rs` (int). A policy file is **matcher-only** — it
holds no algorithm bodies, only the choice. The canonical shape
(`docs/ARCHITECTURE.md` → "Policy file structure"):

```rust
// 1. the real algorithms — NAMED, no `Default` variant
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { Newton, MgDivide }

// 2. the verdict: a settled algorithm, or "the value decides"
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)] ByValue(fn(&Int<N>) -> Algorithm),
}

// 3. a `const` matcher, total over the key (N / (N, SCALE))
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        (1, _) | (2, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        _               => Select::ByAlgorithm(Algorithm::Newton),
    }
}

// 4. dispatch: fold the verdict, then an EXHAUSTIVE match — no `_`, no panic
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>, mode: RoundingMode,
) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&raw),
    };
    match algo {
        Algorithm::Newton   => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
        Algorithm::MgDivide => sqrt::sqrt_mg_divide::sqrt_mg_divide(
            raw.resize_to::<Int<2>>(), SCALE, mode).resize_to::<Int<N>>(),
    }
}
```

Because `select` is `const` and keyed only on the const generics, the
`const { … }` block folds per monomorphisation and every unchosen arm is
dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>` compiles
to a direct call to one kernel — zero runtime dispatch. `ByValue` is the
one arm that keeps a runtime branch, for genuinely value-dependent
choices; most policies never return it.

There are no per-family policy *traits* and no `impl … for D57<SCALE>`
blocks — stable Rust cannot specialise a trait impl on a const-generic
type, so the whole choice is the one `const fn select` keyed on
`(N, SCALE)`, grep-able in one place per function.

### The cascade

When you call `D<Int<3>, 20>::sqrt_strict_with(mode)`, dispatch flows
**down only**:

```
typed method                 policy matcher                  algos kernel
D<Int<3>,20>::sqrt_strict_   policy::sqrt::dispatch::<3,20>   sqrt::sqrt_newton
  with                       (const-folds select → one arm)     ::sqrt_newton
```

The typed method is a one-line delegate to `dispatch`; the policy only
*chooses*; the algorithm fn does the maths by calling kernels/leaves
below it — never a method that re-enters its own policy (that inversion
is forbidden, `docs/ARCHITECTURE.md` → "Layering direction").

To tune a cell you change the **matcher** — add an `Algorithm` variant,
a `select` arm at the band, and a `dispatch` arm — never a per-tier `if`
or trait impl. The matcher is the architecture's primary lever; place an
arm across its whole continuous win-region, not a single benched point.

---

## 2. Adding a per-(width, scale) override

Worked example: writing a bespoke `sin/cos` kernel for `D57<SCALE>`
at `SCALE ∈ 18..=22`. The shipped wide-tier
[`src/algos/trig/sincos_tang.rs`](https://github.com/mootable/decimal-scaled/blob/main/src/algos/trig/sincos_tang.rs)
is the template; a narrower band uses the same shape with different
reduction parameters.

### Step 1 — Create the kernel file

`src/algos/trig/sincos_tang_3limb_s18_22.rs`:

```rust
//! Bespoke sin/cos slot for D57<SCALE> with SCALE in 18..=22.
//!
//! At narrower scales the wide-tier `sin_cos_fixed` runs at
//! `w = SCALE + GUARD` digits. The shared `GUARD` is sized for the
//! worst case (SCALE 57); this slot uses a tighter `GUARD` because
//! the precision target is smaller. Inner Taylor terms run shorter,
//! cutting per-call cost on the bench numbers documented in the
//! commit message that lands this kernel.

use crate::support::rounding::RoundingMode;
use crate::int::types::Int;

const GUARD: u32 = 4; // narrower than the wide-default ~25

pub(crate) fn sin_strict(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    /* … reduction + Taylor at SCALE + GUARD … */
}

pub(crate) fn cos_strict(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    /* … shares sin_cos_fixed core … */
}
```

### Step 2 — Register the module

`src/algos/trig/mod.rs`:

```rust
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod sincos_tang_3limb_s18_22;
```

### Step 3 — Wire it in the policy

`src/policy/trig.rs` is matcher-only. Add your kernel as a new
`Algorithm` variant, route its band in `select`, and call it from the
exhaustive `dispatch` match:

```rust
enum Algorithm { /* … existing … */ TangNarrowBand }

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // the new bespoke band — a continuous range, never a single point
        (3, 18..=22) => Select::ByAlgorithm(Algorithm::TangNarrowBand),
        _ => /* the width / family default */ Select::ByAlgorithm(Algorithm::Series),
    }
}

// …then in dispatch's exhaustive `match algo`:
Algorithm::TangNarrowBand =>
    trig::sincos_tang_3limb_s18_22::sin_strict(raw, mode),
```

Place the arm across the **continuous win-region** the bench shows, not
the single benched `(width, scale)` cell — a point-snapped arm leaves its
neighbours uncovered (`docs/ARCHITECTURE.md` → the single-cell-fit trap).

### Step 4 — Default rounding mode siblings

Every `*_with(mode)` method gets a default-mode sibling that
delegates to it with `HalfToEven`. The kernel-file split mirrors
this: if you write `sin_strict_with`, also write `sin_strict` that
calls it with `RoundingMode::HalfToEven`.

### Step 5 — Validate correctness BEFORE celebrating

```sh
# The root crate's suites (cross-witness, probes, every target builds):
cargo test --release --features wide,x-wide,xx-wide,macros

# The consolidated integration suite (api / contracts / regressions):
cargo test -p decimal-scale-test --features wide,x-wide,xx-wide,macros,serde,alloc,dyn

# The full-surface golden gate, all six rounding modes:
cargo test -p decimal-scale-test --release --features wide,x-wide,xx-wide --test golden golden_all_modes -- --ignored --nocapture
```

The 0.5 ULP contract has dedicated test suites. They are part of the
PR gate — if any of them fail your kernel does not land.

- [`decimal-scale-test/tests/golden.rs`](https://github.com/mootable/decimal-scaled/blob/main/decimal-scale-test/tests/golden.rs) — the full-surface golden gate and the crate's **definitive correctness proof**. One erased subject drives the [`decimal-scaled-golden`](https://github.com/mootable/decimal-scaled/tree/main/decimal-scaled-golden) harness over **every band-edge `(width, scale)` cell** (88 cells, `D18<0>` … `D1232<1231>`) for every strict function under **every** `RoundingMode` (`HalfToEven`, `HalfAwayFromZero`, `HalfTowardZero`, `Trunc`, `Floor`, `Ceiling`), asserting the correctly-rounded value EXACTLY (`delta == 0` storage LSB, ZERO tolerance) and validating overflow panics against each cell's envelope. Passes only at **0 bad / 0 panic**. The `golden (gate)` CI job runs it per push (row-sampled); the `golden (comprehensive)` workflow runs it unsampled on demand. See the [`decimal-scale-test` README](https://github.com/mootable/decimal-scaled/blob/main/decimal-scale-test/README.md) for the `GOLDEN_*` filter variables.
- The multi-oracle golden set (`decimal-scaled-golden/golden/<fn>.golden`, generated from `tests/lead/<fn>.lead` by the oracle pipeline) is the external truth the gate asserts against - correctly-rounded for every `RoundingMode` across all twelve widths at the band-edge cells. The legacy `ulp_strict_golden` mpmath suite is retired (its inputs were folded into the leads).
- [`tests/wide_strict_transcendentals.rs`](https://github.com/mootable/decimal-scaled/blob/main/tests/wide_strict_transcendentals.rs) — cross-witness suite for the wide tier. Computes a value at the target's storage and scale, computes the reference at a wider storage at the same scale, rescales, and asserts bit-exact or ±1 LSB agreement. The pattern to copy when adding a new bespoke kernel.
- [`tests/narrow_strict_transcendentals.rs`](https://github.com/mootable/decimal-scaled/blob/main/tests/narrow_strict_transcendentals.rs) — narrow tier (D18/D38) inherited-method coverage.
- [`decimal-scale-test/tests/regressions/ln_lookup_bands.rs`](https://github.com/mootable/decimal-scaled/blob/main/decimal-scale-test/tests/regressions/ln_lookup_bands.rs) — parity / no-panic coverage for the deep-scale Tang-lookup `ln_strict` bands, one parametrised arm per `(width, band)`. Off-grid bands (no golden cell lands inside them) keep the full `exp(ln(x))` round-trip parity plus band-edge no-panic bounds; on-grid bands (the golden gate pins the mid-band cell bit-exact) keep the band edges only. New Tang lookup bands must add a matching parity arm before the kernel is allowed in.
- [`decimal-scale-test/tests/regressions/powf_integer.rs`](https://github.com/mootable/decimal-scaled/blob/main/decimal-scale-test/tests/regressions/powf_integer.rs) — bit-exact assertion of `powf_strict(D::try_from(n).unwrap()).to_bits() == powi(n).to_bits()` for the integer-exponent fast path, plus the wide-tier exact integer-power directed-rounding pins. Any future integer-exponent specialisation has to keep this contract.
- [`decimal-scale-test/tests/proptest_identities.rs`](https://github.com/mootable/decimal-scaled/blob/main/decimal-scale-test/tests/proptest_identities.rs) — property-based ULP fuzz at D38<19> with a D76<19> cross-tier witness. Identities (`exp(ln(x)) ≈ x`, `sin² + cos² ≈ 1`, sign symmetries, …) with deterministic seeds and 100 cases per block.

See [`docs/precision-testing.md`](https://mootable.github.io/decimal-scaled/precision-testing/) for the four-layer model and how to add coverage for a new tier.

If your change adds a bespoke kernel for a new `(width, scale)` cell,
**add cross-witness tests for that cell to
[`wide_strict_transcendentals.rs`](https://github.com/mootable/decimal-scaled/blob/main/tests/wide_strict_transcendentals.rs)
(or the matching narrow-tier file) in the same commit.** Use a
wider-storage type at the same `SCALE` as the truth source — for
example, a new D57<20> kernel's witness is D76<20> at the same scale.
The pattern in the file for D57<18/20/22> sincos is the template.

The 0.5 ULP suite for D38 is comprehensive; the wide-tier suite is
less so. If you are touching D57 or a wider tier and there is no
existing dedicated 0.5 ULP test at the scale you care about, add one
— the PR gate is "we have a test that would catch this" not "the
existing tests happen to pass". A smaller perf win at strict
0.5 ULP is always preferable to a larger win that drifts past it.

---

## 3. File and naming conventions

Bespoke slot files: `<func>_<algo>_<limbcount>limb_s<lo>_<hi>.rs`

Examples:

- `atan_tang_3limb_s44_56.rs` — atan via the Tang kernel on a 3-limb work integer, scale range 44..=56
- `inverse_tang_3limb_s18_22.rs` — inverse-trig family, Tang kernel, 3 limbs, scale range 18..=22
- `sincos_tang_3limb_s18_22.rs` — shared kernel emitting both sin and cos, scale range 18..=22

Inside the file:

- Module docstring explains the *algorithmic* choice, not the policy gate. Where does the win come from?
- Document the **error budget** in LSB-of-working-scale. If a reviewer can't audit the budget, the kernel can't be reviewed.
- Cite the source — see [Sourcing algorithms](#sourcing-algorithms-licensing-and-citation) below for the licence-by-licence rules. Citation is mandatory regardless of where the technique came from.
- Use bitshifts for power-of-two arithmetic (`>> 1`, not `/ 2`).

### Sourcing algorithms — licensing and citation

This crate is dual-licensed [MIT OR Apache-2.0](https://opensource.org/licenses/MIT). Everything that lands in `src/` must be compatible with that. There are no blurred edges here — when in doubt, choose the more cautious path.

**Algorithms from [LGPL](https://www.gnu.org/licenses/lgpl-3.0.html) / [GPL](https://www.gnu.org/licenses/gpl-3.0.html) (and other copyleft) sources — e.g. [GMP](https://gmplib.org/), [MPFR](https://www.mpfr.org/), [GNU libc](https://www.gnu.org/software/libc/) `libm`:**

- An *algorithm* — a mathematical procedure — is not copyrightable. There is nothing stopping you from re-implementing one in this crate.
- A *reference implementation* is copyrighted. You may not copy its code, paraphrase its structure, lift its variable names, mirror its control flow, or translate its comments. You may not look at the source while writing the kernel — a clean-room implementation is the only defensible posture.
- The preferred sourcing path is therefore: find the underlying *paper or specification*, read that, and implement from the mathematical description directly. The reference implementation does not enter the loop.
- Cite the paper in the module docstring even when the reference implementation exists, so reviewers can audit the algorithmic choice without going near the copyleft code.

Two reasons this matters and they reinforce each other: a legal duty — copyleft licences attach viral terms to any code you copy, and this crate cannot accept those terms while staying MIT/Apache; and an ethical duty — the authors of those projects chose their licence for a reason, and circumventing it dishonours both the work and the conditions on which they shared it. Both edges of the rule sit in the same place: do not start from their code.

**Algorithms from MIT / Apache / BSD / public-domain sources — e.g. another permissively-licensed crate, a textbook, a blog post with a clear MIT licence header:**

- Re-implementation from the mathematical description is still the preferred path. A fresh implementation gives the maintainers full latitude to refactor, restructure, and adapt the code as the algorithm library evolves — which is harder with code you've inherited under an upstream's coding style and naming.
- If you do reuse code under a permissive licence, you must follow the attribution rules of that licence — and citation is *not* a substitute for compliance:
    - MIT and BSD: preserve the upstream copyright notice and licence text in the source file or in `LICENSES/THIRD-PARTY.md`.
    - Apache-2.0: preserve copyright notices, the licence text, and any `NOTICE` file; mark modifications you have made.
- Regardless of the licence, cite the source. The module docstring should name the paper or upstream and link to it, so reviewers can verify the kernel against its source.

**Citation is mandatory in every case.** A kernel without a citation cannot be reviewed because the reviewer has no way to audit the algorithmic decisions. Cite the *paper* or *specification*, not a forum post or a chat transcript — what we want is a stable, citable reference future maintainers can return to.

When in doubt about whether something counts as "copying" — particularly with copyleft sources — the answer is to step away and re-implement from the underlying mathematics. The cost of being cautious is small; the cost of a licence breach in a numerics library that may end up in fintech or CAD downstreams is large.

---

## 4. Proving the performance improvement

A perf change is unproven until measured. The project has four
layers of measurement, ordered from cheapest to most expensive.
Climb them in order; do not skip.

### Layer 1 — Focused microbench (under 60 seconds)

For any kernel change, write a tiny probe driver before and after.
Place it under `examples/` (not in the bench harness) — it's a
single-purpose, throwaway tool.

```rust
// examples/d57_sincos_s18_22_probe.rs
fn main() {
    for scale in 18..=22 {
        let a = /* deterministic input */;
        let mut samples = vec![];
        for _ in 0..3 {
            let t0 = std::time::Instant::now();
            for _ in 0..10_000 {
                let _ = std::hint::black_box(a.sin_strict());
            }
            samples.push(t0.elapsed());
        }
        samples.sort();
        println!("D57<{}> sin_strict median = {:?}", scale, samples[1]);
    }
}
```

Run it synchronously and capture to a file:

```sh
cargo run --release --example d57_sincos_s18_22_probe --features wide > probe.txt
```

Run **before** the change, then **after**, and diff. A < 60s probe
that shows the expected drop unblocks the full sweep. A probe that
doesn't show the drop means the kernel choice was wrong — iterate
before burning multi-hour sweeps.

> Delete the probe driver before committing the kernel — these are
> contributor scratch tools, not part of the public API.

### Layer 2 — Profiling

When the probe shows an unexpected pattern (regression, no-op, partial
win), profile to find why. The project uses [samply](https://github.com/mstange/samply)
for sampling-based CPU profiling — it runs headless, needs no elevated
permissions, and uploads its output to the Firefox Profiler web UI for
inspection.

The release profile already emits debug info (`[profile.release] debug = true`
in [`Cargo.toml`](https://github.com/mootable/decimal-scaled/blob/main/Cargo.toml)) so samples resolve to function names.
Function-name resolution is on by default — you don't need to rebuild.

### Layer 3 — Local Criterion benches

The full bench harness lives under [`benches/`](https://github.com/mootable/decimal-scaled/tree/main/benches/) and uses
[criterion](https://docs.rs/criterion/). Per-width benches follow the
`full_matrix_d<width>` and `library_comparison_d<width>` naming
conventions.

For a tuning change at a single (width, scale-range), run only the
matching per-width bench:

```sh
cargo bench --bench full_matrix_d57 --features wide,x-wide,xx-wide,macros
```

Criterion writes timing data to `target/criterion/`. Look at the
`new` vs `previous` comparison in the HTML report or the
`estimates.json` files. Picosecond `change:` deltas are below the
bench resolution floor — only trust ratios > ~1.5×.

### Layer 4 — Full matrix sweep on GitHub Actions

For wider regression detection, trigger the
[`bench-full`](https://github.com/mootable/decimal-scaled/blob/main/.github/workflows/bench-full.yml) workflow:

```sh
gh workflow run bench-full.yml --field bench_family=full_matrix
```

The workflow fans out per width, one runner each, with a six-hour
per-job cap. Standard runners are shared CPUs with 20-50% wall-time
variance, so absolute numbers between runs aren't comparable — but
within a single run, the matrix is internally consistent and good for
detecting "did some cell regress relative to its peers".

For library comparison (against `rust_decimal`, `bigdecimal`, etc.):

```sh
gh workflow run bench-full.yml --field bench_family=lib_cmp
```

### Layer 5 — Cross-version comparison

To check that a perf win doesn't undo a previous one, trigger the
[`bench-history`](https://github.com/mootable/decimal-scaled/blob/main/.github/workflows/bench-history.yml) workflow:

```sh
gh workflow run bench-history.yml
```

Same Criterion harness against every published tag in the
`v0.2.5..v0.3.3` band plus current `HEAD`. Each cell rewrites the
[`bench-history/Cargo.toml`](https://github.com/mootable/decimal-scaled/blob/main/bench-history/Cargo.toml) dep line to
pin a different version; only the dependency changes per cell, so
the comparison is genuinely like-for-like.

Scope is small by design (D38, D76, D307 — six functions). Expand
the harness as the API surface stabilises.

---

## 5. PR gates

One hard gate runs on every pull request — the precision gate. It is
non-overridable. Performance is tracked out-of-band (see the
benchmarking workflows above), advisory and never a merge blocker.

### Precision gate (hard, non-overridable)

The [`ci.yml`](https://github.com/mootable/decimal-scaled/blob/main/.github/workflows/ci.yml)
workflow enforces the precision suite listed above on every pull
request. Its `golden (gate)` job runs the full-surface golden gate in
release across all six rounding modes (row-sampled per push; the
unsampled deep pass is the dispatch-only `golden (comprehensive)`
workflow), and its `tests (gate)` job runs the full `cargo test` —
the cross-witness suites, the regression reproducers, and the
proptest fuzz. **A failed precision check blocks merge full stop.**
There is no reviewer override, no label dismissal, no waiver process.
The contract is correctly-rounded to 0 storage LSB (bit-exact at the
last representable place) under every `RoundingMode` — a kernel that
drifts off the correctly-rounded result by even one LSB does not
land, regardless of how good its perf numbers look.

If you hit a precision failure: the kernel parameters need adjustment
(usually a wider `GUARD` constant, occasionally a different reduction
shape). Treat it as a numerical bug to debug, not a CI friction to
work around.

The golden run is a separate job specifically so it can be marked
*Required* in the repo branch-protection rules independently of the
bench / docs workflows.

### Perf (advisory, never a gate)

Performance is not a merge gate. It is tracked out-of-band by the
benchmarking workflows described above — the
[`bench-branch-compare`](https://github.com/mootable/decimal-scaled/blob/main/.github/workflows/bench-branch-compare.yml)
workflow compares a branch against the released baseline across the
width/scale surface, and the
[`benches/micro/pr_gate.rs`](https://github.com/mootable/decimal-scaled/blob/main/benches/micro/pr_gate.rs)
micro-bench gives a small focused criterion set for local checks.
These report perf shifts but never block the merge: perf is a signal;
correctness (the precision gate) is the release blocker.

When you ship a new bespoke kernel for a `(width, scale)` cell the
existing benches don't reach, add a matching bench in the same commit
so future runs can see regressions against the new path.

---

## 6. Commit and PR conventions

- Commit subjects use a lowercase prefix matching the change kind:
  `perf:`, `fix:`, `docs:`, `ci:`, `release:`, etc. Keep them under
  ~70 characters.
- Cite the measurement that justifies a `perf:` commit in the body —
  before/after ns/op at the most relevant scale, plus the bench /
  workflow run ID if applicable.
- Do not skip pre-commit hooks (`--no-verify`) or commit signing.
- Bench output files and probe scripts stay out of the tree —
  the `.gitignore` and `.git/info/exclude` handle the common cases.

### Commit-message template and attribution policy

Point your local Git at the in-tree message template once per clone:

```sh
git config commit.template .gitmessage
```

The template ships subject-line and body guidance and a reminder of
the project's attribution policy: **commit messages must not carry
machine-attribution trailers** — no `Co-Authored-By` lines naming an
assistant, no `generated-with` notes, no robot emoji. A `commit-hygiene`
CI job scans every commit in the PR range and **fails the build** if any
message matches those patterns, so a non-compliant commit cannot land.
If the gate flags a commit, reword it (`git rebase` / `git commit
--amend`) to drop the offending line and push again.

Open a pull request against `main`. CI runs the unit-test suite and
the docs build (`RUSTDOCFLAGS=-D warnings`). Both must pass.
