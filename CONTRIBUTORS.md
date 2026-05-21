# Contributing to `decimal-scaled`

This guide is aimed at contributors who want to **tune a specific
`(width, scale)` cell** — pick a faster kernel for a particular
`Dxx<S>` value — and then **prove the win** with the project's
existing perf infrastructure.

It assumes you have read [`README.md`](README.md) and have a working
`cargo build --features wide,x-wide,xx-wide,macros`.

The crate's correctness contract is fixed:
[≤ 0.5 ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place) for
strict transcendentals, identical bit-patterns across platforms.
Performance tuning never trades that away.

---

## 1. The algorithm library

The hot kernels live in [`src/algos/`](src/algos/), one subdirectory
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

Each family contains several **variants** — free functions that take
the raw storage integer plus runtime parameters and return a raw
storage integer. None of them know about the typed `Dxx<S>` wrapper.

Common variant names:

| Variant                       | What it does                                                      |
|-------------------------------|-------------------------------------------------------------------|
| `generic_wide`                | One function, generic over `(Storage, Wide)` via `WideStorage`    |
| `fixed_d38`                   | Hand-tuned `i128` kernel using the in-tree `Fixed` intermediate   |
| `widen_to_d38`                | D9/D18 widen into D38, run `fixed_d38`, narrow back               |
| `borrow_d57`                  | D38 widens into D57, runs the wide kernel, narrows back           |
| `wide_kernel`                 | Per-tier wide-int core emitted by `decl_wide_transcendental!`     |
| `lookup_<width>_s<lo>_<hi>[_<func>]` | Bespoke per-(width, scale-range) override slot             |

Variant names describe the algorithm, not the type. One variant may
serve many cells; one cell may have several variants competing.

### Which kernel runs where

The wiring between **a typed value** and **a kernel** lives in
[`src/policy/<family>.rs`](src/policy/). One policy trait per family
(`SqrtPolicy`, `LnPolicy`, `TrigPolicy`, `PowPolicy`, …), implemented
once per width, generic over `const SCALE: u32`. The body picks a
kernel:

```rust
impl<const SCALE: u32> SqrtPolicy for D57<SCALE> {
    fn sqrt_impl(self, mode: RoundingMode) -> Self {
        // Scale-range overrides — first match wins.
        if matches!(SCALE, 20..=20) {
            return Self(sqrt::lookup_d57_s20::sqrt(self.0, mode));
        }
        // Width default.
        Self(sqrt::generic_wide::sqrt_d57(self.0, SCALE, mode))
    }
}
```

Because `SCALE` is `const`, the `if` is dead-code-eliminated per
monomorphisation. Every concrete `D57<S>` compiles to a direct call
to one kernel — zero runtime dispatch cost.

Stable Rust does not allow trait-impl specialisation on const-generic
types, so per-(width, scale) overrides live as `if matches!(SCALE, …)`
arms inside the per-width impl rather than separate `impl … for D57<20>`
blocks. The override list is grep-able in one place per family.

### The cascade

When you call `D57<20>::sqrt_strict()`, dispatch flows:

```
typed method shell  ──►  policy trait impl  ──►  algos kernel
D57<20>::sqrt_strict     SqrtPolicy::sqrt_impl    sqrt::lookup_d57_s20::sqrt
```

Three intercept points exist, in priority order:

1. **Scale-range override** (`if matches!(SCALE, X..=Y) { … }`) — most specific
2. **Width default** (the `else` arm in the per-width impl) — moderate
3. **Family default** (the `generic_wide` kernel) — fallback

Add an override where it gives you the most leverage for the least scope.

---

## 2. Adding a per-(width, scale) override

Worked example: writing a bespoke `sin/cos` kernel for `D57<SCALE>`
at `SCALE ∈ 18..=22`. The shipped
[`src/algos/trig/lookup_d57_s44_56_sincos.rs`](src/algos/trig/lookup_d57_s44_56_sincos.rs)
is the template for the SCALE 44..=56 band; a narrower band uses the
same shape with different reduction parameters.

### Step 1 — Create the kernel file

`src/algos/trig/lookup_d57_s18_22_sincos.rs`:

```rust
//! Bespoke sin/cos slot for D57<SCALE> with SCALE in 18..=22.
//!
//! At narrower scales the wide-tier `sin_cos_fixed` runs at
//! `w = SCALE + GUARD` digits. The shared `GUARD` is sized for the
//! worst case (SCALE 57); this slot uses a tighter `GUARD` because
//! the precision target is smaller. Inner Taylor terms run shorter,
//! cutting per-call cost on the bench numbers documented in the
//! commit message that lands this kernel.

use crate::rounding::RoundingMode;
use crate::wide_int::Int192;

const GUARD: u32 = 4; // narrower than the wide-default ~25

pub(crate) fn sin_strict(raw: Int192, mode: RoundingMode) -> Int192 {
    /* … reduction + Taylor at SCALE + GUARD … */
}

pub(crate) fn cos_strict(raw: Int192, mode: RoundingMode) -> Int192 {
    /* … shares sin_cos_fixed core … */
}
```

### Step 2 — Register the module

`src/algos/trig/mod.rs`:

```rust
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_sincos;
```

### Step 3 — Wire it in the policy

`src/policy/trig.rs`, in the `impl<const SCALE: u32> TrigPolicy for D57<SCALE>`
block, add a scale-range arm above the default:

```rust
fn sin_impl(self, mode: RoundingMode) -> Self {
    if matches!(SCALE, 18..=22) {
        return Self(trig::lookup_d57_s18_22_sincos::sin_strict(self.0, mode));
    }
    if matches!(SCALE, 44..=56) {
        return Self(trig::lookup_d57_s44_56_sincos::sin_strict(self.0, mode));
    }
    Self(trig::wide_kernel::sin_strict_d57(self.0, SCALE, mode))
}
```

### Step 4 — Default rounding mode siblings

Every `*_with(mode)` method gets a default-mode sibling that
delegates to it with `HalfToEven`. The kernel-file split mirrors
this: if you write `sin_strict_with`, also write `sin_strict` that
calls it with `RoundingMode::HalfToEven`.

### Step 5 — Validate correctness BEFORE celebrating

```sh
cargo test --release --features wide,x-wide,xx-wide,macros
```

The 0.5 ULP contract has dedicated test suites. They are part of the
PR gate — if any of them fail your kernel does not land.

- [`tests/precision_strict_05_ulp.rs`](tests/precision_strict_05_ulp.rs) — D38 0.5 ULP suite. Hand-computed truth values at D38<12> for every constant and strict transcendental, asserting the result is the *correctly-rounded* value — bit-exact (`delta == 0` storage LSB) against the hand reference. Compile-gated to `HalfToEven` (the crate-default rounding mode); the bit-exact-across-every-mode proof lives in `ulp_strict_golden.rs`.
- [`tests/precision_wide_baseline.rs`](tests/precision_wide_baseline.rs) — D76 wide-tier 0.5 ULP measurement at D76<6>. Same `≤ 1 LSB` contract; constant `WIDE_TOLERANCE_LSB` makes the threshold explicit so a regression that drifts past it is loud. Gated to `HalfToEven`.
- [`tests/wide_strict_transcendentals.rs`](tests/wide_strict_transcendentals.rs) — cross-witness suite for the wide tier. Computes a value at the target's storage and scale, computes the reference at a wider storage at the same scale, rescales, and asserts bit-exact or ±1 LSB agreement. The pattern to copy when adding a new bespoke kernel.
- [`tests/narrow_strict_transcendentals.rs`](tests/narrow_strict_transcendentals.rs) — narrow tier (D9/D18/D38) inherited-method coverage.
- [`tests/d616_s308_lookup_parity.rs`](tests/d616_s308_lookup_parity.rs), [`tests/d924_s460_lookup_parity.rs`](tests/d924_s460_lookup_parity.rs), [`tests/d1232_s615_lookup_parity.rs`](tests/d1232_s615_lookup_parity.rs) — per-tier Tang-lookup-vs-wide-kernel parity at the design SCALE. Tight `≤ 1 LSB` agreement between the two implementation paths, plus `exp(ln(x))` round-trip identities. New Tang lookup bands must add a matching parity file at their target SCALE before the kernel is allowed in.
- [`tests/perf_d462_s230_correctness.rs`](tests/perf_d462_s230_correctness.rs) — composed-identity witnesses (`cosh² − sinh² = 1`, `sin² + cos² = 1`, …) for the D462 Tang slot. The same shape works for any new bespoke-kernel slot.
- [`tests/powf_integer_fastpath_parity.rs`](tests/powf_integer_fastpath_parity.rs) — bit-exact assertion of `powf_strict(D::from_i32(n)).to_bits() == powi(n).to_bits()` for the `|n| ≤ 64` fast path. Any future integer-exponent specialisation has to keep this contract.
- [`tests/ulp_strict_golden.rs`](tests/ulp_strict_golden.rs) — external-oracle suite and the crate's **definitive correctness proof**. Reads pre-computed mpmath truth tables under `tests/golden/<func>_d<N>_s<S>.txt` and asserts the kernel result is the **correctly-rounded** value — `kernel == oracle` EXACTLY (`delta == 0` storage LSB, ZERO tolerance) — for **every** `RoundingMode` (`HalfToEven`, `HalfAwayFromZero`, `HalfTowardZero`, `Trunc`, `Floor`, `Ceiling`) across **all thirteen decimal widths** at their design-target scale: D9<4>, D18<9>, D38<19>, D57<28>, D76<35>, D115<57>, D153<76>, D230<115>, D307<150>, D462<230>, D616<308>, D924<460>, D1232<615>. Each golden line stores `floor(f(x)·10^SCALE)` plus a fractional tie-class, from which the harness derives the correctly-rounded integer for each mode in-test (no per-mode tables). Catches kernel bugs that internal cross-witness paths mirror and miss. Regenerate the tables with `python scripts/gen_golden_precision.py` (requires `pip install mpmath`).
- [`tests/ulp_proptest.rs`](tests/ulp_proptest.rs) — property-based ULP fuzz at D38<19> with a D76<19> cross-tier witness. Identities (`exp(ln(x)) ≈ x`, `sin² + cos² ≈ 1`, sign symmetries, …) with deterministic seeds and 100 cases per block.

See [`docs/precision-testing.md`](docs/precision-testing.md) for the four-layer model and how to add coverage for a new tier.

If your change adds a bespoke kernel for a new `(width, scale)` cell,
**add cross-witness tests for that cell to
[`wide_strict_transcendentals.rs`](tests/wide_strict_transcendentals.rs)
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

Bespoke slot files: `lookup_<width_alias>_s<lo>_<hi>[_<func>].rs`

Examples:

- `lookup_d57_s20.rs` — single scale, single function (file scope is unambiguous)
- `lookup_d57_s44_56_atan.rs` — scale range, named function (multi-function family)
- `lookup_d57_s44_56_sincos.rs` — scale range, shared kernel emitting both sin and cos

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
in [`Cargo.toml`](Cargo.toml)) so samples resolve to function names.
Function-name resolution is on by default — you don't need to rebuild.

### Layer 3 — Local Criterion benches

The full bench harness lives under [`benches/`](benches/) and uses
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
[`bench-full`](.github/workflows/bench-full.yml) workflow:

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
[`bench-history`](.github/workflows/bench-history.yml) workflow:

```sh
gh workflow run bench-history.yml
```

Same Criterion harness against every published tag in the
`v0.2.5..v0.3.3` band plus current `HEAD`. Each cell rewrites the
[`bench-history/Cargo.toml`](bench-history/Cargo.toml) dep line to
pin a different version; only the dependency changes per cell, so
the comparison is genuinely like-for-like.

Scope is small by design (D38, D76, D307 — six functions). Expand
the harness as the API surface stabilises.

---

## 5. PR gates

Two gates run on every pull request. They protect very different
things and have very different escape hatches.

### Precision gate (hard, non-overridable)

[`.github/workflows/precision.yml`](.github/workflows/precision.yml)
runs the precision suite listed above — the four 0.5 ULP files, the
per-tier Tang-lookup parity files, and the bespoke-slot correctness
witnesses — on every pull request. **A failed precision check blocks
merge full stop.** There is no reviewer override, no label dismissal,
no waiver process. The contract is correctly-rounded to 0 storage LSB
(bit-exact at the last representable place) under every `RoundingMode`
— a kernel that drifts off the correctly-rounded result by even one
LSB does not land, regardless of how good its perf numbers look.

If you hit a precision failure: the kernel parameters need adjustment
(usually a wider `GUARD` constant, occasionally a different reduction
shape). Treat it as a numerical bug to debug, not a CI friction to
work around.

This gate is a one-job workflow specifically so it can be marked
*Required* in the repo branch-protection rules independently of the
bench / docs workflows.

### Perf gate (soft, overridable)

[`.github/workflows/codspeed.yml`](.github/workflows/codspeed.yml)
runs three Criterion-based bench harnesses through
[CodSpeed](https://codspeed.io/) — an instruction-count simulator
built on [Valgrind](https://en.wikipedia.org/wiki/Valgrind/cachegrind).
Sub-1% noise on shared CI runners, so a 5% regression is reliably
detectable instead of disappearing in the wall-time floor that a
plain Criterion-on-CI gate would suffer.

The three bench targets:

- [`benches/all_functions.rs`](benches/all_functions.rs) — 130-bench D38<12> full public-API sweep.
- [`benches/mul_div_candidates.rs`](benches/mul_div_candidates.rs) — focused mul/div algorithm comparison.
- [`benches/pr_gate.rs`](benches/pr_gate.rs) — wide-tier coverage at D38<19> / D57<20> / D307<150> across nine ops. Add cells here when you ship a kernel that the other two suites don't reach.

CodSpeed leaves a comment on the PR with per-bench `%Δ` against the
baseline and marks the check failed if any cell regressed past the
configured threshold. **Override path: a reviewer applies the
`perf-regression-acknowledged` label to the PR.** Legitimate reasons:

- A bug fix that costs perf (e.g. a missing rounding step that was
  giving false speedups).
- A correctness refactor whose perf hit will be recovered by a
  follow-up kernel.
- A new feature whose code path is intentionally slower (e.g. a new
  rounding mode).

Use the label sparingly. The whole point of the gate is to catch
unintended regressions; routinely dismissing it makes it noise.

If your PR is large enough that the bench coverage in
[`benches/pr_gate.rs`](benches/pr_gate.rs) doesn't reach your changed
cell, extend the harness in the same commit. The bench set is
deliberately small (so per-PR wall time stays low) but it is not
fixed in scope — new bespoke kernels should add a matching bench so
future PRs can see regressions against the new path.

### Repo-owner setup (one-time)

The CodSpeed gate needs:

1. The [CodSpeed GitHub App](https://github.com/marketplace/codspeed)
   installed on the repo (free for OSS).
2. The workflow's `permissions: id-token: write` grants OIDC auth —
   no separate API token secret is needed in the v4 action flow.
3. Branch-protection rules on `main` to mark **both** the `precision`
   and `CodSpeed` workflows as Required. The precision check cannot
   be dismissed by reviewers; the CodSpeed check can be dismissed
   only via the `perf-regression-acknowledged` label.

The first PR after setup establishes the baseline; subsequent PRs
compare against it.

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
