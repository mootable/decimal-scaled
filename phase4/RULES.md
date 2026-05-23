<!--
SPDX-FileCopyrightText: 2026 John Moxley
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Phase-4 implementation rules — HAND THIS TO EVERY PHASE-4 AGENT

Canonical, self-contained spec for the Phase-4 work (algorithm consolidation/migration/rewrite,
the policy matcher, the `limbs/` dissolution). An agent handed this doc + its function's migration
doc has everything it needs. **Read `docs/ARCHITECTURE.md` → "Algorithm choosing — and pruning" /
"Policy file structure" first** — this doc operationalises it and adds the hard rules.

## 0. Hard rules (non-negotiable)
- **NO AI attribution** anywhere — commits, comments, code, docs. Plain `git commit -m`.
- **Do NOT push, do NOT touch `main`, do NOT publish.** The coordinator merges, golden-gates, and pushes. You commit in your worktree only.
- **base-reset FIRST:** `git fetch origin && git reset --hard origin/release/0.5.0`. Report the tip.
- **`core`-only / `no_std`-safe by default.** No `std`/`alloc` in the algorithms unless explicitly behind a `#[cfg(feature = "std"/"alloc")]` gate. No `Vec`/heap in kernels — use fixed-size limb buffers.
- **CHECK every algorithm's `std` requirement.** A float (`f64`) **seed** computed via a **std-only intrinsic** (`f64::sqrt`/`exp`/`ln`/`cbrt`/…; these live in `std::f64`, NOT `core`) makes that algorithm std-only. Handle it with the **policy feature-flagging strategy** (`docs/ARCHITECTURE.md` → "Feature-flagging a variation"; this doc §1 cfg-in-the-policy): the std fp-seed kernel is a `#[cfg(feature = "std")]` `select` arm (or cfg-gated `Algorithm` variant); the **`core` integer-seed kernel is the default/non-std arm**. The default `no_std` build runs the core path and MUST still be correct — verify `cargo check --no-default-features` (+ `--features alloc` if needed). `f64` is only ever a *seed* to a self-correcting integer iteration whose exact integer termination pins the result.
- **`nightly` is NEVER required.** The crate must compile + work fully on stable. Where a nightly feature (`generic_const_exprs`) would help, ship a **stable workaround** (slower) and let `nightly`, if enabled, swap in the fast path. Never gate correctness/availability behind nightly. (See §2.)
- **Golden-gate every behaviour-affecting change.** The transcendental precision contract (0 ULP / correctly-rounded) must not regress.
- **Run `cargo test --no-run` (all targets compile) before declaring done** — `--lib`-only misses integration breakage.
- **Never weaken the overflow / rounding / determinism contracts.** Integer-only results.

## 1. The target shape (per function) — the policy file
Implement exactly the shape in `docs/ARCHITECTURE.md` "Policy file structure":
```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { /* real algorithms only — NO `Default` variant */ }

#[derive(Clone, Copy)]
enum Select<const N: usize> { ByAlgorithm(Algorithm), ByValue(fn(&Int<N>) -> Algorithm) }

const fn select<const N: usize>(/* keys */) -> Select<N> { match /* key tuple */ { /* arms */ } }

pub fn f<const N: usize>(x: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {        // INLINE const block — NOT `const SEL: Select<N>` (E0401)
        Select::ByAlgorithm(a) => a,
        Select::ByValue(g)     => g(&x),
    };
    match algo { /* EXHAUSTIVE over Algorithm — no `_`, no unreachable!() */ }
}
```
- **Dispatch via the inline `const { select::<N>() }` block.** A `const SEL: Select<N>` *item* can't see the fn's generic `N` (E0401).
- **Completeness is structural:** `select` total over the key + `match algo` exhaustive over the real `Algorithm` enum. The "default" is the real algorithm named in `select`'s `_` arm — **no `Default` variant, no `panic!`/`unreachable!`/`compile_error!`**.
- **Key arity:** int unary `N`; int binary `(N,M)`; decimal unary `(N,SCALE)`; decimal binary `(N,M,S1,S2)`. Per-axis `single | range | none`; relational rules (`N==M`, `S1≷S2`) as `if` guards on the const bindings. Commutative-only larger-first swap (moves the `(width,scale)` operand as a unit); `div`/`sub`/`add` preserve order.

## 2. Compile-away (the cardinal rule)
- `select` is `const`, keyed ONLY on const generics ⇒ `const { select::<N>() }` folds per monomorphisation; unchosen arms are dead-arm-eliminated in release. Zero runtime branch on the const path.
- The zero-branch property is a **release** property — proven per function by **inspecting the release IR/asm** (one direct call, no branch/table/vtable on the const path). This is the **acceptance gate** for a migrated function. Debug keeping arms is fine.
- Computed-width (`Int<{max(N,M)}>`, `[u64; max(N,M)]`) needs `generic_const_exprs` (nightly). It is **NOT a precondition** — the stable build MUST work: prefer getting the wider type from an operand (the commutative swap) and passing MIN/MAX as const VALUE args; where a genuinely wider intermediate is unavoidable, use a **fixed-max-width buffer** (over-allocate to the widest enabled tier — `core`/no_std-safe, slower). A `nightly` feature, if present, only swaps in the exact computed-width fast path. **Never gate a function's correctness/availability behind nightly.**

## 3. Value matcher (only where the best algorithm depends on the runtime VALUE)
- `Select::ByValue` carries a **non-capturing** `fn(&Int<N>) -> Algorithm` (closure or fn item) that **returns an `Algorithm` tag** (never a fn-pointer to the algo — the tag keeps dispatch a direct call). Closures must capture nothing.
- Placement by size: **≤2 outcomes → inline closure `if`/`else`; 3–10 → inline closure `match`; >10 (or shared / unit-tested) → named `#[inline]` fn**.
- Value-decision *between algorithms* → value matcher. Value-dependent *steps within* an algorithm (convergence, normalisation) → stay in the algo body.

## 4. NAMING (enforced — the consistency agent will check this)
- **Algorithm fn/module = `<function>_<method>[_with_<method2>][_variant]`.** Prepend the function (`sqrt_`, `div_`, `mul_`, `exp_`, …) + the literature/paper method (`newton`, `knuth`, `karatsuba`, `mg_divide`, `burnikel_ziegler`, `tang`, `series`, …). Hybrids keep the other method's **full** name: `div_tang_with_mg_divide`, `powf_exp_with_ln`. `_variant` only to disambiguate.
- **PREFER generic-over-`N`** (one algorithm serving a `min..=max` range of limbs). A **width-encoded name is a LAST RESORT** — only when the algorithm genuinely cannot be generic AND method/`_with_`/variant can't disambiguate; then suffix the **limb count** `<N>_limb` (e.g. `mul_karatsuba_4_limb`). NEVER `_int2`, bit-width, or `dXX`.
- **`Algorithm` enum variant = CamelCase of the algorithm fn name MINUS its function prefix** — `sqrt_newton`→`Newton`, `sqrt_mg_divide`→`MgDivide`, `sqrt_newton_with_table_seed`→`NewtonWithTableSeed`, `div_burnikel_ziegler_with_knuth`→`BurnikelZieglerWithKnuth`. A strict **1:1 variant↔fn mapping** — this is what lets it be macro-derived and consistency-checked (probably the only way the codegen works). The function is implicit from the policy file, so the variant drops the prefix.
- **Value-matcher fn = `<function_name>_<applicable_preconditions>`** (e.g. `sqrt_N5_to_N10`) — preconditions ARE its identity; their count/shape varies (single `sqrt_N5`, range `sqrt_N5_to_N10`, decimal `sqrt_N2_S0_to_S9`).
- Dispatch *strategies* (`borrow_*`, `widen_*`) carry **no** function prefix — they're not algorithms; move them to the policy layer.

## 5. Per-function workflow
1. Read this doc + the function's migration doc (`phase4/migration_{roots,explog,trig}.md`) + the dead-list (`phase4/algo_catalog.md`).
2. **4.1 (the lift):** make the surviving kernels **generic over their work-width/SCALE** so one algorithm serves a matcher range. Decide per-kernel (Q4.1b) whether it generalises or is genuinely width-bespoke — prefer generic. Drop the catalog-confirmed dead kernels.
3. **4.2 (the matcher):** write the policy file per §1, sourced from the #64 best-algorithm analysis.
4. **Boundary golden coverage (Q4.2-b):** every matcher arm's range *boundaries* get explicit golden cases — a precision cliff at an arm edge must not slip through.
5. Rename per §4. Fold the decimal alias canonicalisation (`D38`→`D<Int<N>,S>`) for files you touch.
6. **Verify (run ALL):** `cargo check` (default) + `--features wide,x-wide,xx-wide,macros`; `cargo test --features wide,x-wide,xx-wide,macros --no-run`; `--lib`; golden `--release … --test ulp_strict_golden` → **264/264**; the function's own tests. Capture the **release IR/asm proof** (§2) for the dispatch.
7. Report: base-reset, what consolidated vs stayed bespoke (+why), the names, the matcher, golden + `--no-run` lines, the IR/asm proof, anything deferred.

## 6. `limbs/` dissolution (#79) — see `phase4/migration_limbs.md`
- `Int<N>` (`[u64; N]`) stays in `int/types`. NO `limbs/` folder.
- (i) generic limb arithmetic → `int/algos/limbs.rs` (named `mul_*`/`add_*`/etc. by what they do); (ii) division engines → `int/algos/div.rs` (`div_knuth`, `div_burnikel_ziegler_with_knuth`, `div_mg`); (iii) isqrt → the roots algos; (iv) decimal formatting → `support`/display.
- **u128 legacy family → DELETE, but only after `newton_reciprocal` is rewritten onto u64 limbs** (sever the `crate::wide_int::{limbs_mul,limbs_divmod_dispatch,limbs_sub_assign}` reach + drop the `lib.rs`/`int/policy/mod.rs` re-exports). Fold the `Vec<u128>` → fixed-size u64 buffers in the same rewrite (no_std).

## 7. Pointers
- `docs/ARCHITECTURE.md` — "Algorithm choosing — and pruning" / "Policy file structure" (committed conceptual reference).
- `phase4/matcher_design.md` — full matcher design + rationale.
- `phase4/migration_{roots,explog,trig,limbs}.md` — per-function/limbs migration plans.
- `phase4/algo_catalog.md` — confirmed-dead list + duplication map.
- `research/2026_05_22_algo_naming_standard.md` — the prior naming-standard note (superseded where it conflicts with §4).

**Pilot = `sqrt`** (the canonical exemplar; every other function copies its policy-file shape). After all functions/widths/types are migrated, a consistency-review agent checks the whole surface against §1–§6.
