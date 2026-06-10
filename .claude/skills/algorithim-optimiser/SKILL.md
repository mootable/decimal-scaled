
---
name: algorithim-optimiser
description: Use when adding, replacing, or speed-optimising an algorithm in the decimal_scaled crate — how to write the kernel, name it per the standard, wire it to a policy, the dec-vs-int differences, the support libraries (algos/support, int/algos/support, algo_x_support), and how to microbench two candidates at a dispatch seam.
---

# Algorithm optimiser

How to add / swap / speed-up an algorithm in `decimal_scaled` without breaking the layering, the naming, or the 0.5-ULP guarantee.

The authoritative architecture doc is `docs/ARCHITECTURE.md` → sections **"Layering direction"** and **"Policy file structure"**. This skill is the working how-to on top of it.

---

## 0. The layering law (the one rule everything else serves)

The call graph points **DOWN only**:

```
type method  →  policy::<fn>::dispatch  →  algorithm fn (algos/<fn>/)  →  kernel / leaf
```

- A type **method** (`D<Int<N>,SCALE>::sqrt_strict_with`, `Int<N>::isqrt`) is a one-line delegate to its policy `dispatch`.
- The **policy** file holds ONLY the matcher (enum / select / dispatch). No algorithm bodies.
- An **algorithm fn** does the maths, by calling **kernels / leaves** below it.
- **THE INVERSION (forbidden):** an algorithm fn must NEVER call a *dispatched method on its own operand type* that re-enters its own (or a sibling) policy. e.g. an int `sqr` kernel calling `self.wrapping_mul`, or a decimal `hypot` calling `x.sqrt_strict_with` — both route back UP through a policy. Compute via the kernel directly instead.
- Cross-tier DOWN calls are fine (see §5).

Golden is **blind** to layering (it only checks numeric output) — so layering is verified by reading diffs / grep, never by tests passing.

---

## 1. Write the algorithm

One algorithm = one **named file** under its function folder:
- decimal: `src/algos/<fn>/<name>.rs`
- int: `src/int/algos/<fn>/<name>.rs`

Even a trivial schoolbook op gets its own named file — **no exemptions**, never inlined in the policy.

### Naming standard — `<fnname>_<algorithmvariation>_<preconditions>`

- `<fnname>` — the function (or family): `exp`, `sqrt`, `add`, `atan`, `trig` (for a multi-fn shared core), `inverse` (asin/acos/atan2 group).
- `<algorithmvariation>` — the algorithm, NOT the tier: `newton`, `series` (Taylor/Maclaurin in a Fixed intermediate), `tang` (Tang lookup table), `schoolbook`, `karatsuba`, `ripple_carry`, `native`, `mg_divide`, `exp_with_ln` (composition).
- `<preconditions>` — applicability, expressed in **limbs or scale**, NEVER the `dXX` tier alias:
  - int storage width in limbs: `2limb` (Int<2> = D38), `3limb` (Int<3> = D57), `16limb` (Int<16> = D307). Tier→limb: D18=1, D38=2, D57=3, D76=4, D115=6, D153=8, D230=12, D307=16, D462=24, D616=32, D924=48, D1232=64.
  - scale band: `s44_56`, `s18_22`.
- A **generic-over-N** algorithm (one algo serving all widths) gets **NO precondition suffix** — just `<fn>_<method>`: `sqrt_newton`, `add_ripple_carry`, `mul_karatsuba`.
- Examples: `exp_series_2limb`, `atan_tang_3limb_s44_56`, `sqrt_newton`, `cbrt_newton`, `add_int_layer`, `div_widen_scale`.
- Do **not** name files after the tier (`fixed_d38`, `lookup_d57_s44_56` are the OLD anti-pattern that was renamed out).

### The body

- Operate on the lowest sensible representation (limb slices `&[u64]`, the `Fixed` work-type, `Int<N>` / `W: BigInt`).
- Call **kernels and leaves** (other algos fns, support leaves) — never a method that re-dispatches your own op.
- Keep visibility `pub(crate)`. Move logic **verbatim** when relocating (golden is blind to layering → don't change behaviour and move in the same step).

---

## 2. Wire it to a policy

`src/policy/<fn>.rs` (decimal) / `src/int/policy/<fn>.rs` (int) is **matcher-only**, the canonical shape:

```rust
// 1. the real algorithms — NAMED, no Default variant
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { Native, Newton }   // CamelCase of the kernel fn minus the `<fn>_` prefix, 1:1

// 2. the verdict
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)] ByValue(fn(&Uint<N>) -> Algorithm),   // value-dependent choice, opt-in
}

// 3. const matcher, total over the key (N / (N,M) / (N,SCALE) / (N,M,S1,S2))
const fn select<const N: usize>() -> Select<N> {
    match N { 1 | 2 => Select::ByAlgorithm(Algorithm::Native), _ => Select::ByAlgorithm(Algorithm::Newton) }
}

// 4. dispatch: fold the verdict, then EXHAUSTIVE match (no `_`, no panic)
pub(crate) fn dispatch<const N: usize>(x: Uint<N>) -> Uint<N> {
    let algo = match const { select::<N>() } { Select::ByAlgorithm(a) => a, Select::ByValue(f) => f(&x) };
    match algo {
        Algorithm::Native => isqrt_native::<N>(x),
        Algorithm::Newton => isqrt_newton::<N>(x),
    }
}
```

Rules:
- **No `Default` variant.** "Default" = the real algorithm named in `select`'s `_` arm.
- **Variant ↔ kernel fn is 1:1** (CamelCase minus the `<fn>_` prefix). Exception: a *composition* (e.g. `powf = exp∘ln`) names the composition (`ExpWithLn`) since there is no single kernel fn — document it.
- The per-`Algorithm` fns in the policy are **thin delegations** to the kernel in `algos/`, not real computation.
- **Compile-away** is the point of `const`: `const { select::<…>() }` folds per monomorphisation and the unchosen arm is dead-arm-eliminated in *release*. It is NOT a language guarantee → the acceptance gate is the **IR/asm proof** (one direct call, no runtime branch, no panic path). Debug keeps all arms (correctness-neutral).
- `ByValue` is the only arm that keeps a runtime branch — use it for value-dependent algorithm choice (placement: closure-`if` ≤2, closure-`match` 3–10, named `#[inline] fn <fn>_N<lo>_to_N<hi>` if >10 / shared / tested).
- **Policies don't leak.** The ONLY item a policy file exposes is `dispatch` (`pub(crate)`). `Algorithm` / `Select` / `select` / any threshold or tuning `const` (e.g. a Karatsuba crossover) stay **private to the file** — `dispatch` threads a threshold into the kernel as an *argument*, kernels never import it, and nothing outside the policy imports a policy internal. A `pub(crate)` threshold imported by a test is a leak — point the test at an explicit value (it should stress its own worst case anyway), and privatise the const.

The type method then delegates one line: `pub fn isqrt(self) -> Self { isqrt_dispatch(self) }`.

### Policy usage & scope — make it as wide as it's correct for

- The whole point of the matcher is **one generic algorithm serving a RANGE** of the key. Prefer *widening* a generic algorithm's applicability over adding narrow per-tier bespoke kernels (the 6.2 collapse: 60 per-tier wrappers → 6 generic `*_series`). Add a bespoke/specialised kernel **only where a microbench shows it actually wins**; otherwise route the cell to the generic one in the `_` arm.
- `select` must be **total over the full key** — every `N` / `(N,M)` / `(N,SCALE)` / `(N,M,S1,S2)` cell resolves to a *real* algorithm. The `_` arm names the widest-applicable default. No gaps, no `unreachable!` in the algorithm choice.
- Use the **right key width** — key on the full axis your algorithm's correctness/perf depends on, no less: int unary = `N`; int binary = `(N,M)`; decimal unary = `(N,SCALE)`; decimal binary = `(N,M,S1,S2)` with an `S1==S2` const fast-path. If a kernel's validity depends on `SCALE`, the key MUST include `SCALE`.
- A new algorithm's **precondition should be as wide as it is correct for**: if `<fn>_newton` is valid for all `N`, name it `<fn>_newton` (no precondition suffix) and let the `_` arm use it everywhere; only pin a `<precond>` suffix (`_2limb`, `_s44_56`) when the kernel is genuinely valid for *just* that limb/scale band.
- The type method calls dispatch with the full key (`isqrt_dispatch(self)`, `dispatch::<N, SCALE>(…)`) and **never branches on the tier itself** — tier/scale selection is the matcher's job, not the caller's.

### Work-width — keep it out of the policy (expand in LIMBS, not a work *type*)

When a decimal algorithm computes in a **wider integer than it stores** (sqrt/hypot work in ≤`2N` limbs, cbrt ≤`4N`, mul/div `2N`), stable Rust can't write the work *type* `Int<2*N>` from a generic `N` (that needs nightly `generic_const_exprs`). The chosen answer is **not** to bind a work type at all — **DO NOT** thread `W = Int<2N>` via a per-tier `*Policy` trait + `*_policy_tier!` macro (non-conformant: it pushes the policy past matcher-only AND pollutes the type-method layer with an algorithm-internal width). Earlier drafts of this skill proposed a `DoubleWidth { type W }` trait — that was **rejected and never built**; see §5.

Instead the kernel is generic over `N` **only** and does the wide work **directly in a limb scratch buffer**, calling the int layer's width-agnostic slice kernels (full mechanism + the three scratch forms in §5 "Scratch sizing — the `WorkScratch` trait"). The only trait involved binds a scratch **buffer** (`Int::<N>::work2()`/`work4()`, associated-type sized), not a work integer — and its bound stops at `dispatch`, never reaching the type method.

- Result: the policy file stays pure `Algorithm`/`Select`/`select`/`dispatch` — **`select` + `dispatch` do the work**, no per-tier trait/macro, no algorithm bodies, no `unreachable!()` panic arms, no `_ => self.*_strict_with(…)` escape (that's the inversion). `dispatch` carries no work-width parameter; it adds only `where Int<N>: WorkScratch` when it calls a limb-scratch root kernel.
- exp/ln thread a per-tier `$Core` (`WideTrigCore` impl) — at most ONE `CoreFor { type Core }` trait, and only if `$Core` can't be reached otherwise.

### Determining the arms — the algorithm-space method

Full methodology: **`research/2026_05_24_algorithm_space_mapping.md`**. An algorithm sits over a meta numeric space (axes: **N** storage limbs, **W** work width, **S** scale, **value-class**). Map TWO regions per algorithm, kept strictly separate:

- **Validity (HARD, analytical)** — where it's *correct*: the 256-bit `Fixed` ceiling (`S + GUARD ≤ 68`), work-width overflow, a kernel hard-coded to `N ≤ 2`, a series' convergence / range-reduction precondition, seed accuracy. Proven by analysis + a zero-tolerance oracle, **never a microbench** (an out-of-region algorithm can be *fast and wrong*).
- **Optimality (EMPIRICAL)** — within validity, where it's *fastest* vs the alternatives: the crossover points (e.g. `BZ_THRESHOLD = 16`). Found by benchmark; it only re-partitions *overlapping* validity regions, never extends one past a validity wall.

Recipe (also §6 of the research doc): (1) list candidates, name each `<fn>_<method>[_<precond>]`; (2) derive each validity region as inequalities on `(N,W,S)` + a value predicate; (3) confirm with a zero-tolerance golden sweep, extend `micro_golden` for any cell it misses; (4) check the validity union **tiles the whole key** — no gaps; (5) where regions overlap, find the optimum by an **N-way microbench of ALL candidate arms** (`compare_all`, §4) → single-axis sweep → bisect the crossover → record it as a named `const`, not a buried literal; (6) write `select` (`_` = widest valid generic; specific arms = validity carve-outs + optimality overrides; `ByValue` only for a genuine value split); (7) `dispatch` = `const { select() }` → exhaustive `match algo`, no `_`, no panic; (8) boundary golden at every arm edge (just-inside / just-outside, external oracle, `lsbe == 0`, all six modes).

### Testing a policy — keep it LIGHT (the ONLY layer where that is allowed)

A policy's real **correctness** test is the **golden** suite (routing-agnostic — it checks the output is correctly-rounded whichever arm ran) and its real **optimality** test is the **performance bench** (`compare_all` §4 / `bench-branch-compare`). So a policy's own unit tests must NOT pin which algorithm `select` chooses, nor re-assert function correctness — that hamstrings the very thing built to be re-tuned (thresholds re-benched, arms swapped, algorithms added).

- **OK:** a light "`dispatch` resolves to one valid algorithm and runs" check; unit-testing a custom `ByValue`/`ByShape` classifier's mapping logic (reference the threshold `const`, not a hard literal, so a re-bench doesn't break it).
- **NOT OK:** routing-pins (`len ≥ 256 → Karatsuba`), exhaustive pre/post-condition correctness duplicating golden.
- Keep KERNEL correctness + scratch-**safety** tests (e.g. "Karatsuba == schoolbook product", "fixed scratch ≥ needed" — a too-small scratch is release UB); those test the kernel/invariant, not the routing choice.

**This relaxation is `policy/`-ONLY. EVERYWHERE ELSE stays fully rigorous** — kernels, `algos/`, support leaves, `int/` primitives, types, conversions, parsing/formatting: pre/post-condition checks, exhaustive edge cases, every-test-asserts, zero-tolerance golden/accuracy. Do not let the policy relaxation leak into any other layer.

---

## 3. Support libraries — where shared leaves live

Three buckets. **Support libraries are SINGLE files of leaves, NOT split per function.**

- **`src/algos/support/`** — decimal-side cross-family leaves: `fixed.rs` (the 256-bit sign-magnitude `Fixed` work-int type), `mg_divide.rs`, `newton_reciprocal.rs`, `seed_bridge.rs` (typed-`W` seed bridge over the seed leaf), `table_cache.rs` (`decl_table_cache!` per-thread memo), `wide_trig_core.rs` (the `WideTrigCore` trait + 6 generic `*_series`).
- **`src/int/algos/support/`** — int-side leaves: `limbs.rs` (the `add_assign`/`mul_schoolbook`/`shr`/`cmp`/`bit_len`/… limb primitives).
- **`src/algo_x_support/`** — TRULY cross-tier-INDEPENDENT leaves. The hard invariant: a leaf here **calls NOTHING in-crate** — primitives + std-gated `f64` only. The **seed library** (`seed.rs`: `sqrt_seed`/`cbrt_seed`/`extract_top_u64`) lives here. One API per seed, with the std/no_std divergence encapsulated **inside** the leaf (internal `#[cfg]`), so consuming algorithms stay cfg-agnostic. Don't put anything here that imports another crate module.

If a leaf could serve both tiers but touches in-crate code, it belongs in `algos/support` or `int/algos/support`, not `algo_x_support`.

---

## 4. Microbench the candidates at the dispatch seam

The dispatch `Select` seam is the "choose + swap + **microbench**" point. **Microbench ALL algorithms available for that function at that level (N / N,S), not just two** — every registered `Algorithm` arm (the `Schoolbook` reference + every optimised variant) is a candidate; the winner becomes that `select` arm. Use the 8.1 harness.

- Support module: **`benches/support/ab_microbench.rs`**. Worked example: **`benches/micro/mul_kernel_ab.rs`**.
- API:
  ```rust
  micro_criterion()  // Criterion preset: sample 20, warm 150ms, measure 400ms — sub-60s
  // N-WAY (default): bench every candidate for the function+level at once.
  compare_all(c, "group", |inp| label_string, inputs,
              vec![("school", school_run), ("kara", kara_run), ("toom3", toom3_run)])
  // 2-candidate convenience (delegates to compare_all):
  ab_compare(c, "group", |inp| label_string, inputs, candidate_a, candidate_b)
  ab_sweep!(c => Int<16> => |c| compare_all(...), Int<32> => |c| compare_all(...))  // the type axis
  ```
- It `black_box`-guards inputs and outputs (defeat const-fold/DCE — critical here, the dispatch is *designed* to const-fold) and prints a **ranking table** (fastest→slowest, with `(N.NNx slower)` / `(~tie)` margins) plus the `A/B verdict [group]: <winner> beats <loser> by N.NNx` line (grep-stable). Add the N-th algorithm by appending one `("label", run)` tuple to the `vec!` — nothing else changes.
- Add a `[[bench]]` entry in `Cargo.toml`: `name`, `path = "benches/<folder>/<name>.rs"`, `harness = false`, and the `required-features` the example needs.

**Discipline:** validate a perf change with a **focused <60s microbench FIRST**, before the multi-hour sweeps. Run microbenches **locally**; run the full sweeps (`library_comparison`, `full_matrix`) on **GHA** (`bench-full` / `bench-history` / `bench-branch-compare` workflows) — never burn the owner's machine on a full sweep. picosecond `change:` deltas are noise; multi-hour sweep cells run 1.5–2× slower than a cold-machine microbench.

`benches/` is organised into folders: `support/` (shared incl. `ab_microbench`), `libcmp/`, `full_matrix/`, `agm/`, `backends/`, `lookup/`, `micro/`. Keep `[[bench]] name=` stable when moving files (workflows call `--bench <name>`); only `path=` changes.

## 4b. Profile the winner with samply — squeeze it, then send back

Once `compare_all` crowns a candidate, **profile it with samply** ([[reference_profilers]] — the steady-state profiler) to surface residual hotspots the relative A/B ranking hides (a 10% frame the bench averages away). This loop takes a kernel from "beats the alternatives" to "tapped out."

- **Pin the MEASUREMENT only, never the build** (same rule as `pin_run.ps1`): build the bench exe UNPINNED — `cargo build --bench <name> --no-run` runs multithreaded across all cores; a 1-core compile crawls — then `samply record` the *cached* exe pinned via Windows `ProcessorAffinity`. Never pin a `cargo build`/`check`.
- **Pin width tracks concurrency; a solo run is NOT throttled to one core.** The single-core pin only PARTITIONS concurrent agents so their measurements don't fight — give a SOLO bench/profile ~4 cores (mask `0xF00000` = cores 20–23) for cleaner, less-jittery numbers; N concurrent → disjoint groups (20–23 / 16–19 / …), one mask each, high-end cores (Windows puts system work on the low cores).
- **Profile easy AND difficult operands** — small/sparse/near-power-of-ten vs full-width-dense/max-magnitude/high-scale. The *difference* separates value-INDEPENDENT cost (the fixed-width kernel) from value-DEPENDENT cost (a decimal op's `÷10^w` rescale grows with magnitude), so "hard − easy" isolates which layer dominates and cross-checks the kernel-vs-rescale diagnosis.
- **Coordinator-parallel option:** the coordinator can samply the baseline (or the agent's worktree) on a DIFFERENT disjoint core group while the agent microbenches on its own — no interference — and `SendMessage` the hot frames back mid-flight, overlapping the profile with the optimise loop instead of waiting for return.
- **Iterate:** optimise the hot frame → re-`compare_all` (bit-identity wall) → re-samply, to diminishing returns. samply is profiling LOAD (machine-visible): the coordinator runs it, or an `algorithim-optimiser`/`policy-mapper`-charter agent may run it pinned; a research/hunter agent does not.
- **Running samply on Windows (this repo) — the `SamplyElevated` scheduled task.** ETW profiling needs elevation (a per-run UAC prompt), and the Bash tool can't drive the elevation handoff. So: a one-time owner-registered task (`schtasks /create /tn SamplyElevated /tr "powershell -File trace/samply_elevated.ps1" /sc once /st 00:00 /rl highest /f`) runs `trace/samply_elevated.ps1`, which reads the 3-line job spec `trace/samply_job.txt` (exe / criterion-filter / out-profile) and runs `samply record --save-only --unstable-presymbolicate`. The coordinator triggers it PROMPT-FREE via `powershell.exe -NoProfile -Command "schtasks /run /tn SamplyElevated"` (git-bash mangles bare `/run` `/tn` → always go through powershell.exe). Helper: `research/run_samply.sh <exe> <filter> [out]`. **CAVEAT (confirmed): `--save-only` does NOT resolve Rust symbols** — frames return `fun_<hex>` (only OS exports resolve), regardless of UAC/elevation; use the criterion A/B verdict for relative cost, and for NAMED Rust frames run `samply record -- <exe> <filter>` interactively (Firefox UI symbolicates from the PDB at view-time).

---

## 5. The dec ↔ int difference (and the cross-tier rule)

**Int algorithms** (`int/algos/<fn>/`): operate on `&[u64]` limb slices or `Int<N>`/`Uint<N>`; use the limb leaves in `int/algos/support/limbs.rs` and the optimised divide via `int::policy::div_rem::dispatch` (Knuth) — NOT the const `div_rem_mag_fixed` shift-subtract path. Policy keyed on `N`. **Audit for infinite loops:** an int algorithm must never use an operator/method (`+`, `*`, `.div_rem()`) that re-dispatches back into the same algorithm.

**Decimal algorithms** (`algos/<fn>/`): generic over storage `N` (the `Int<N>` backing the decimal). They **dispatch DOWN to the `Int<N>` layer** for their integer work — via the operator overloads (and **check the overload exists**) or by calling the int method/kernel directly. This cross-tier use is **§1a-allowed and required** — it is NOT the inversion. The inversion is decimal→decimal (calling a decimal `*_strict_with` method on your own value, which re-enters a decimal policy). The `hypot` lesson: it used to call `one_plus_sq.sqrt_strict_with(mode)` (inversion + double-rounding); fixed to form `a²+b²` and take the floor root via the int layer.

**Work-width — expand in LIMBS, never name `Int<2N>` (the limb-expansion lesson):** when a decimal kernel needs a WIDER work integer than its storage (the `sqrt` radicand spans `2N` limbs, `cbrt` `4N`, `mul`/`div` `2N`), do **NOT** thread a work *type* `W = Int<2N>`. `Int<2N>` is unnameable from a generic `N` on stable (`2*N` in type position needs nightly `generic_const_exprs`), and faking it forces the non-conformant `*Policy` trait + `*_policy_tier!` per-tier macro **and** pollutes the type-method layer with an algorithm-internal width. INSTEAD compute the wider work **directly in a fixed limb scratch buffer** (`[u64; SCRATCH]`, sized via `work_scratch(mult)` — see *Scratch sizing* below) and call the int layer's **width-agnostic slice kernels**: `int::algos::isqrt::isqrt_newton::isqrt_newton(&n, &mut out)` / `icbrt_newton` (roots), `int::algos::mul::mul_schoolbook::mul_schoolbook` (products, incl. building `mag·10^k` by iterative ×10), `int::policy::div_rem::dispatch` (divide), and `int::algos::support::limbs::{add_assign, sub_assign, cmp, cmp_cross, shl, shr, bit_len}`. The kernel is then generic over `N` **only** (no `W`), `dispatch` carries no work-width parameter, and the policy stays a pure `(N, SCALE)` matcher — no per-tier trait/macro, no pollution, and *more* honest about the §1a boundary than a phantom `W: BigInt`. (Done for `sqrt`/`cbrt`/`hypot` + the int roots.)

**Scratch sizing — the `WorkScratch` trait (in `int/types/work_scratch.rs`):** a decimal root kernel gets its limb scratch from `Int::<N>::work2()` / `work4()` (2N-family `sqrt`/`hypot`, 4N-family `cbrt`), bounding only on `where Int<N>: WorkScratch`. The buffer **size lives in the trait's associated types** (`Buf2`/`Buf4`), i.e. in the `impl` where `N` is concrete — so it is NEVER a generic const expression in the kernel. That is the whole trick: the `where Int<N>: WorkScratch` bound is discharged for free at the concrete `N` every type method dispatches at, so it stops at `dispatch` and does **not** cascade up to the macro-generated type methods. (Put a `generic_const_exprs` length directly in a kernel and you get a `where [(); …]:` bound that *does* cascade through every caller — the trap this avoids.) Add the bound to BOTH the kernel and its policy `dispatch`; the type method needs nothing. Three feature-selected forms, numerically identical (only *who pays for the slack* differs):
  - **default** (`exact-scratch`, **a default feature**): one `impl WorkScratch for Int<$n>` per concrete storage width via a macro, each sized exactly `mult·n + ceil(n/2)` from the literal — stable, smallest frames, no const generics. The owner is against per-`Int<N>` impls in general but **approved them here, gated behind the feature**.
  - **no `exact-scratch`** (`--no-default-features`): ONE blanket `impl<const N> WorkScratch for Int<N>` sized `work_scratch(mult)` = `mult·MAX_WORK_N + ceil(MAX_WORK_N/2)` (build-max — `MAX_WORK_N` is the widest *enabled* tier's limb count, 2/16/32/64). The prior behaviour, kept as the no-per-impl fallback.
  - **`exact-scratch-nightly`**: ONE blanket impl sized per-`N` via `work_scratch_n(mult, N)` under `generic_const_exprs`; the const-expr + its `where [(); …]:` obligation stay **confined to that impl block** (the associated-type indirection is what confines it), so no cascade even here.
  `work_scratch`/`work_scratch_n` still live in `int/algos/support/limbs.rs` (the int slice roots `isqrt_newton`/`icbrt_newton` have no const `N` — runtime slices — so they keep the build-max `work_scratch`). **Run golden with `exact-scratch` on (it's default) as part of the default-build gate** — a too-small per-`N` buffer is release-mode UB that only the golden will catch. A too-small scratch elsewhere is the same hazard: validate any reduced band with a golden/lib run for that feature set.

**std / no_std float policy:** `std` owns floats (inherent `f64` intrinsics — `sqrt`/`cbrt`/`sin`/…); `no_std` is integer-only, **NEVER `libm` or any external math crate**. A seed that wants a float gets a `std` (f64) and a `no_std` (pure-integer) variant encapsulated in the seed leaf (`algo_x_support::seed`), so the algorithm stays agnostic. The `fast` feature (f64 bridge) implies `std`.

---

## 6. Verification gates (run before declaring done)

- `cargo check --lib` (default features) **and** `cargo check --features wide,x-wide,xx-wide --lib` — both clean.
- **Quick validation (agents, during development):** `cargo test --features wide,x-wide,xx-wide --test micro_golden` — `tests/micro_golden.rs`, a fast curated subset of the golden oracle (~0.5s, same harness + same zero-tolerance `lsbe == 0` as the full golden). Today it covers 6 functions (sqrt/cbrt/exp/ln/sin/atan) × 3 tiers (D18/D57/D307) × the first ~20 oracle rows × all six RoundingModes. Run it freely while iterating. If you add or touch a function/tier the subset doesn't exercise, **extend the curated set in `micro_golden.rs`** so your path is covered — but it stays a *subset*; never let it grow into a second full golden.
- **Full golden** (behaviour, the correctly-rounded guarantee — **coordinator / CI / commit gate**): the `decimal-scale-test` gate (`cargo test -p decimal-scale-test --release --features wide,x-wide,xx-wide --test golden golden_default -- --ignored`), striped on CI. Agents should NOT run it (release build + slow; the coordinator/CI owns it) — verify with a small DEBUG slice via the `GOLDEN_WIDTHS`/`GOLDEN_FUNCS` filters instead.
- `cargo doc --no-deps --features wide,x-wide,xx-wide,macros --workspace` with `RUSTDOCFLAGS="-D warnings"` — the `docs (gate)` CI; broken/private intra-doc links are hard errors.
- `cargo build --all-targets` (default) must build — gate any wide-tier-using example/test with `required-features` in `Cargo.toml` so the narrow default build skips it.
- Layering: grep your algorithm fn for a call to a dispatched method on its own operand type → must be ZERO.

---

## 7. Writing a golden / accuracy test (best practice)

The 0.5-ULP guarantee is only real if every correctly-rounded op is gated by a zero-tolerance oracle test.

- **Regression capture — pin the INPUT VALUE that exposed the bug.** Whenever you find a wrong/inaccurate result, the fix is NOT complete until the **specific input value** that produced it (the exact operand(s), at that width + scale) is added to the **holistic golden suite** so it can never silently regress. Add THAT input — with its correctly-rounded expected value taken from the EXTERNAL oracle, never from the now-fixed code — to the function's `tests/lead/<fn>.lead` input file with a `// regression:` why-comment (the oracle pipeline in `decimal-scaled-golden/oracle/` regenerates the `.golden` rows, validators included), or, if the op isn't oracle-backed, to a regression test in `decimal-scale-test/tests/regressions/`. The discovery of a bug is the trigger to widen the suite with that input, every time. (The `hypot` lesson: the +8–13 ULP error survived precisely because no holistic test pinned the inputs that exposed it.)
- **External oracle, never self-validation.** Expected values come from an EXTERNAL high-precision source — the mpmath golden tables `tests/golden/<fn>_d<N>_s<S>.txt` (generated by `scripts/gen_golden_precision.py`), or for a one-off, a value computed offline with mpmath / Python `decimal` and baked in **with a source-citing comment**. NEVER assert against the crate's own output (circular).
- **Zero tolerance: `lsbe == 0`** (delta == 0 storage LSB) — bit-exact at the last place. No `diff <= 1` slack on a correctly-rounded path; that slack is exactly what let the old `hypot` ship 8–13 ULP off undetected.
- **All six RoundingModes, every width.** The harness folds the oracle's `(floor_raw, cls)` to the correctly-rounded integer for ANY mode; assert across all six modes and every supported width. Split the test per width so local iteration stays fast.
- **Cover the matcher arm boundaries.** Add explicit oracle cases at each `Select` arm's range edges — a precision cliff exactly at an arm boundary must not slip through.
- **If the oracle doesn't cover the op, add a targeted accuracy test** (the `hypot` lesson — it isn't in the golden function set): externally-correct values — exact cases where maths gives an integer (Pythagorean triples for hypot) plus mpmath-computed non-perfect cases — asserting exact (`delta == 0`) across modes/scales/widths, in its own `tests/<fn>_accuracy.rs`.
- **Every test asserts; no silent no-ops.** Name tests by the behaviour they validate. Replace any runtime `if !cond { return; }` early-out with a module-level `#[cfg(...)]` gate so a test can never pass without asserting.
- **New function → add inputs to `tests/lead/<fn>.lead`** and regenerate via the oracle pipeline (`decimal-scaled-golden/oracle/generate.py`); the gate derives every `(width, scale)` cell automatically.
- **While iterating, run `micro_golden`** (the fast curated subset) — not the full golden. The coordinator/CI runs the full `ulp_strict_golden` before merge/push.

---

## 8. Quick checklist for a new/optimised algorithm

1. Kernel file `algos/<fn>/<fn>_<variation>[_<precond>].rs`, body calls kernels/leaves only (no self-dispatch).
2. Register it (`<fn>/mod.rs`, `algos/mod.rs`).
3. Add an `Algorithm` variant (1:1 name) + wire it into `select`/`dispatch` (const, exhaustive).
4. Decimal: dispatch the integer work down to `Int<N>` (operators or direct). Int: no self-re-dispatch loop.
5. Microbench the old vs new candidate at the seam (`ab_microbench`, <60s, local). Promote the winner in `select`.
6. Gates: check (default+wide), golden 264, doc -D warnings, default --all-targets, layering grep.
