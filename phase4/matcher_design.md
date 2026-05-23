<!--
SPDX-FileCopyrightText: 2026 John Moxley
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# 4.2 — Policy matcher design (working doc, 2026-05-23)

Companion to `0.5.0_task_list.md` Phase 4 and `2026_05_22_compile_time_dispatch_options.md`.
This is the place to capture the owner's thinking on the matcher; we discuss, then promote
the agreed design back into the task list as 4.2 subtasks.

## Cardinal rule (owner) — POLICY DISPATCH MUST COMPILE AWAY
**Zero runtime branch.** The matcher selects the algorithm purely from the const generics
(`Int<N>` width `N`, `Dxx<SCALE>` scale) at monomorphisation / const-fold. No runtime
`match` on a runtime value, no fn-pointer / `dyn` table, no injected panic path.

> **"Keep me honest" — assistant's standing job:** every choice below is checked against this
> rule. I flag ⚠️ wherever an option risks a runtime branch, a non-const path, a `dyn`/fn-ptr
> indirection, or a panic/bounds path the optimizer can't strip. Nothing is "the matcher" until
> the **#18 acceptance gate** passes: IR/asm on a representative decimal op across several
> `<width, scale>` shows **no runtime branch and no injected panic path**.

## Honesty-checks (what "compiled away" actually demands)
- ✅ `N` and `SCALE` are const generics → selection *can* be `const`. This is the whole basis.
- ⚠️ **Ranges** (`scale 18..=22 → tang`) must be expressed as `const fn select(N, SCALE) -> AlgoTag`
  evaluated to a `const SEL`, then dispatched on a **const scrutinee** (release dead-arm-elim) —
  NOT a runtime `match scale`.
- ⚠️ **Arms** must be statically dispatched (generic fns selected at compile time), never fn-pointers/`dyn`.
- ⚠️ **Completeness:** an uncovered `<width, scale>` must `compile_error!`, never silently fall to a
  runtime default (a runtime default IS a runtime branch).
- ⚠️ **Debug builds:** a const-scrutinee `match` keeps all arms compiled (no runtime branch, but code
  size). Open: acceptable, or do we need trait/assoc-const dispatch for no-arms-even-in-debug?
- ⚠️ **Value-dependent fast paths** (depend on the runtime VALUE, not width/scale) are NOT matcher
  business — they stay inside the algo. Enumerate any that exist so we don't accidentally pull them up.

## Candidate mechanisms (from the dispatch research, for reference)
- **A.** `const fn select(...) -> AlgoTag` + const-scrutinee branch — stable, great at ranges/defaults; zero-cost relies on const-prop (verify asm).
- **B.** Trait / assoc-const dispatch — truest zero-cost (no branch even in debug), but ranges need specialization/`generic_const_exprs` (poor for the SCALE-range axis; fine for the finite WIDTH axis).
- **C.** Table-macro → emits A — declarative table (ranges + mandatory default, `compile_error!` on gaps) = single source of truth; lowers to A. **Research recommendation.**
- **D.** `generic_const_exprs` — nightly → ruled out (crate is stable).

---

## OWNER'S THINKING (capture here)

### Two policy sets (one per layer)
1. **`Int<N>` policy** — keyed on **width `N`** only (1 var). Selects int-layer algorithms (mul/div/isqrt/…).
2. **`D<Int<N>, S>` policy** — keyed on **`<width N, scale S>`** (2 vars). Selects decimal-layer algorithms (the transcendental/round kernels etc.).

### Each set is subdivided into PER-FUNCTION policies
Within each layer, there's a separate matcher **per function**:
- **`Int<N>`**: per int-op (e.g. `mul`, `div`/`div_rem`, `isqrt`, …) — each keyed on `N`.
- **`D<Int<N>, S>`**: per decimal-op (e.g. `sin`, `cos`, `tan`, `exp`, `ln`, `pow`, `sqrt`, `cbrt`, …) — each keyed on `<N, S>`.

So a function's matcher = a table (ranges + default) over its layer's const generics → that function's algo.
Mirrors today's `src/policy/{cbrt,exp,ln,pow,sqrt,trig}.rs` per-function files + `policy_triplet!`.

> ⚠️ _assistant note (compile-away):_ per-function matchers are individually `const`-foldable (each is a
> `const fn select_<fn>(N[, SCALE]) -> AlgoTag`) ⇒ still ✅ zero runtime branch. **Completeness gate is
> PER FUNCTION** — each function's table must `compile_error!` on an uncovered `<width,scale>` in its
> domain (no shared/global default that could silently misroute one function).

### Each per-function policy is split into 3 TIERS (feature-flagged fallback)
1. **`non_std`** _(was "base" in earlier research)_ — the baseline implementation; always available, no_std-safe. The floor that every cell can fall back to.
2. **`non_std_wrapper`** _(was "non_std" in earlier research)_ — a thin wrapper over `non_std`.
3. **`std`** — provides its OWN dispatch; for any `<width,scale>` cell it does **not** specialise, it **falls through to `non_std`**. Faster methods that need `std` (e.g. float intrinsics / hardware) live here. **`std` mirrors `non_std`'s specialisation grid** — same `<width,scale>` cells; per cell, `std` either supplies a faster std-only impl OR const-falls-through to `non_std`'s algo for that cell. So `std` = `non_std`'s grid + a *subset* of faster overrides.
- **Feature-flagged:** with the `std` feature → the `std` dispatch (specialised cells + fall-through to `non_std`); without `std` → `non_std` (via `non_std_wrapper`).
- _Maps to today's `policy_triplet!` (`std_fn`/`no_std_fn` arms) — the rename (base→`non_std`, non_std→`non_std_wrapper`) is purely to cut confusion vs the old notes._
- **`std` and `non_std_wrapper` share the SAME public fn name** (e.g. `exp_strict`) and are **`cfg`-swapped**: `#[cfg(feature = "std")]` compiles the `std` version, `#[cfg(not(feature = "std"))]` the `non_std_wrapper` version. Callers always name the one symbol; the feature picks which impl is compiled. (`non_std` baseline carries its own internal name that both of them wrap / fall through to.)

> ⚠️ _assistant note (compile-away):_ the `cfg`-swap means only ONE of `{std, non_std_wrapper}` is ever
> compiled for a given build, so there's **no runtime branch *between* the tiers** — selection is purely
> `cfg` (compile-time). ✅ Consistent with the cardinal rule.

> ⚠️ _assistant note (compile-away) — the critical one here:_ the `std → non_std` "fall through" MUST be a
> **const decision** (std's table either has an arm for this `<width,scale>` or it doesn't — known from the
> const generics at compile time → const-select the `non_std` algo for uncovered cells). It must NOT be a
> **runtime** "call std, if it returns `None` then call `non_std`" — that's a runtime branch + an `Option`
> path the optimizer may not strip. And `std` vs `no_std` is a `cfg` (compile-time). So the whole 3-tier
> chain resolves entirely at compile time ✅ — provided the fall-through is table-level (const), not value-level.

> ⚠️ _assistant note (compile-away):_ both keys are pure const generics (`N`, and `N`+`SCALE`), so both
> matchers can be fully `const`-folded — ✅ no inherent runtime-branch risk in having two sets.
> _Clarifying Q (not deciding):_ are the two sets **independent**, or does the **decimal** policy
> **delegate down** to the `Int<N>` policy for its underlying integer ops (i.e. layered: decimal picks
> the decimal algo, which internally calls int ops that resolve via the int policy)? That changes
> whether the decimal table needs to know anything about int-algo selection or treats it as a black box.

### Walkthrough — `Int<N>` policies first (`D<Int<N>,S>` = same model + a SCALE axis)
Owner is describing the int-layer matcher in full (keyed on `N` only); the decimal layer is this exact
structure expanded with the second const-generic axis `SCALE`.

**Per-function policy shape varies by the function's requirements.** Two arities so far:
- **Unary** functions (1 operand) — e.g. `isqrt`, `neg`, `leading_zeros`, … .
- **Binary** functions (2 operands) — e.g. `mul`, `div`/`div_rem`, `add`/`sub`, `rem`, … .
Each function's policy has its own shape (signature / operands / what it needs) accordingly.

#### Unary `Int` matcher (as described)
- **Input:** the **limb count `N`** (the width).
- The matcher matches each arm on EITHER a **range** `min <= limbs <= max` OR a **single** `limbs == k`.
- It then **dispatches to the algorithm defined** for the matched arm.
- Many algos are **const-definable generic over `N`** (one impl serves a whole range of limb counts).
- The matcher's job is purely: **which algo is most EFFICIENT for that many limbs** — the table encodes the benchmark-validated choice per width / width-range.
- _A matcher is already in place_ today → Phase 4 generalises + cleans it (and feeds it from the #64 best-alg analysis).

_(continuing.)_

> ⚠️ _assistant note (compile-away):_ `limbs == N` is a const generic, so each arm's test (`min<=N<=max` or
> `N==k`) is a CONST predicate ⇒ the whole match const-folds to one arm per monomorphisation ⇒ static
> dispatch to the (const-generic) algo. ✅ This is exactly dispatch-option **A** (`const fn select(N)` over
> ranges + singletons). "Most efficient for N limbs" is a compile-time table choice (no runtime probing).

#### Binary `Int` matcher — key `(N, M)` (`Int<N>` × `Int<M>`, MIXED-WIDTH confirmed)
Binary int functions take two limb counts. A per-function policy may use any of these (all const on `N`,`M`):
- `N == M` (same width);
- `N > M` / `N < M` (ordering);
- **canonicalising swap** — send the bigger limb-count first: `if M > N { func::<M,N>(b,a) } else { func::<N,M>(a,b) }`, so the algo always gets larger-first;
- `N` and/or `M` within specific ranges (e.g. `N in 1..=2 && M in 3..=8`).

> ⚠️ _assistant note (compile-away) — **A still holds for binary**:_ `N` and `M` are both const generics, so every
> predicate (`==`, `<`, `>`, range-on-each) is a CONST test ⇒ `const fn select<const N, const M>() -> AlgoTag`
> const-folds. The canonicalising swap is a const `if M > N` at the dispatch site (folds to one orientation per
> `(N,M)`; both `func::<N,M>` and `func::<M,N>` monomorphise, DCE'd in release). So A scales unchanged in shape —
> per-function enum + a 2-var `select` (which may also return an orientation, or the dispatch applies the swap). ✅
> _Design point (owner-refined 2026-05-23):_ the canonicalising larger-first **swap is COMMUTATIVE-ONLY** — valid
> for ops where `func(a,b) == func(b,a)` (e.g. `mul`), which then halves the case-space. For **order-significant**
> ops the swap is FORBIDDEN: `div` (`a/b`), `sub` (`a−b`), and `add` (owner: original order matters) **preserve the
> original `(N,M)`** and match on the raw key. So **canonicalisation is a per-function opt-in flag (commutative
> ⇒ may swap; else preserve order)**, not a global step. _(Compile-away unaffected: swap-or-not is a const choice
> per function ✅.)_ _Flag:_ `add` is value-commutative — if its "order matters" is a result-width / operand-role
> convention rather than the value, the swap could still be safe; treat as order-preserving unless you confirm otherwise.

**Per-function signature adapts:** the policy/algo shape is function-specific — some take just `(N,M)`, some also receive `MIN`/`MAX` (e.g. `min(N,M)`/`max(N,M)`) so the algo doesn't recompute them — potentially `func(a, b, MIN, MAX)`.

> ⚠️ _assistant note (compile-away) — the `generic_const_exprs` TRAP (important):_ passing `MIN`/`MAX` is fine on
> stable **only as const VALUE arguments**: `let max = if N>M {N} else {M}; func::<N,M>(a, b, min, max)` — they're
> compile-time-known `usize`s, so they const-fold as loop bounds / iteration counts inside the algo. It is **NOT**
> fine to pass them as **computed const-GENERIC params**: `func::<N, M, {min(N,M)}, {max(N,M)}>` — a const expression
> in generic-argument position needs `generic_const_exprs` (nightly) ⇒ violates the stable+compile-away rule.
> **Corollary:** if an algo needs the work-width as a *type* (`[u64; MAX]`, an `Int<MAX>` intermediate), that *also*
> needs the computed const generic ⇒ nightly. On stable, get the wider type from the **operand itself** (the
> canonicalising swap hands the algo `Int<larger>` as a concrete type) — never compute `Int<{max(N,M)}>`. So:
> **MIN/MAX as VALUES ✅ stable; as computed const-generics / computed types ❌ nightly.** This is the line to hold.

> 🗺️ **ROADMAP (post-0.5.0, owner 2026-05-23):** the *computed-width* capability (`Int<{max(N,M)}>` intermediates,
> `[u64; max(N,M)]` array sizing, computed const-generic params) is **deferred** — gated behind the **same nightly
> feature flag as cross-scale arithmetic** (both need `generic_const_exprs`), to be lifted if/when that stabilises.
> **Not in 0.5.0.** 0.5.0 binary algos use value-args for MIN/MAX + operand-typed widths (via the commutative swap)
> only. Cross-scale *comparison* stays stable (no computed width needed). _The trap is real → deferral is correct._

> ⚠️ _assistant note (compile-away):_ arity changes the algo SIGNATURE, not the dispatch mechanism — the
> dispatch key stays the const width(s); the operands are runtime values passed THROUGH, never dispatch keys.
> So unary vs binary is ✅ no compile-away impact. _Clarifying Q — RESOLVED (owner 2026-05-23):_ binary int
> functions are **mixed-width** (`Int<N>` × `Int<M>` → key = `(N,M)`). (If a function ever needs to branch on a
> runtime operand VALUE, that stays inside the algo, not in the matcher.)

## ✅ Int<N> matcher — COVERED (owner 2026-05-23)
Unary (key `N`) and binary (key `(N,M)`) both confirmed under option **A** (per-function `AlgoTag` enum + `const fn
select(...) -> AlgoTag` + const-scrutinee `match`). Settled sub-points: range/single-value matching on the const
key(s); commutative-only larger-first swap (per-function flag; `div`/`sub`/`add` preserve order); `MIN`/`MAX` passed
as **const value-args** (fold), never computed const-generics; computed-width (`Int<{max(N,M)}>`) deferred to the
nightly roadmap bucket. **A remains PENCILLED** — locks once `D<Int<N>,S>` confirms it too (owner's lock condition). Next: **D**.

## D<Int<N>, S> matcher — unary (owner 2026-05-23): Int vocabulary × 2 OPTIONAL dimensions
Reapply the Int-unary decision set independently to **each** of the two const keys `N` and `S`, where **each axis is
OPTIONAL** (`none` = wildcard, matches all on that axis):

  **arm = [ N: single (`N==k`) | range (`lo<=N<=hi`) | none ]  ×  [ S: single (`S==k`) | range (`lo<=S<=hi`) | none ]**

- The all-`none` arm IS the default/catch-all (not a separate construct — it falls out of the optionality).
- Single-axis constraints are first-class: `any N && S==18`, or `N in 1..=2 && any S`.
- Dispatch: `const fn select<const N: usize, const S: u32>() -> AlgoTag` → const `match` → const-generic-over-`(N,S)` algo.

> ⚠️ _compile-away honesty note:_ `N` (`usize`) and `S` (`u32`) are both const generics; `none` is just an omitted
> predicate / `_` wildcard on that axis. Every arm is therefore a const test on `(N,S)` ⇒ `select<N,S>` const-folds
> to one arm ⇒ **A holds, 2-key.** ✅ No runtime branch from the scale axis or from the optionality.

### Default implementation per algorithm (owner 2026-05-23) — applies to BOTH Int and D
Every function has a **default implementation** — a generic-over-`N` (and over-`S` for `D`) algo that works for **any**
input. The matcher's single/range arms are **OVERRIDES** that pick a faster/bespoke algo where one exists; the
all-`none` catch-all arm dispatches to this default. Consequences:
- **Completeness is structural** — the `_` arm always targets the guaranteed-present default impl ⇒ the const `match`
  is **total by construction**. **No `panic!`/`unreachable!`/`compile_error!` arm needed** for "unmatched `(N[,S])`".
- Maps cleanly onto the tiers: the **non_std baseline default** = this universal default impl; `std`/`non_std_wrapper`
  layer overrides on top; an unmatched `std` lookup falls through to the non_std default. One consistent fall-through.

> ⚠️ _compile-away honesty note (this is the GOOD outcome):_ the default arm is a **real const-generic algo**, not a
> panic path — so completeness costs **zero** runtime branch and removes the `compile_error!`-completeness machinery I
> had flagged as a risk under mechanic #3. `select` still const-folds; the `_`→default arm DCEs away in release like
> any other. ✅ **Mechanic #3 resolved: default-impl fall-through, no panic/compile_error.**

## D<Int<N>, S> matcher — binary (owner 2026-05-23): int-binary rules on (N,M) × int-binary rules on (S1,S2)
Operands are `(Int<N>, S1)` and `(Int<M>, S2)` → key = **`(N, M, S1, S2)`**. Reapply the **int-binary** decision set
to the width pair AND (independently) to the scale pair, **each pair optional** (`none` = wildcard on that pair):

  **arm = [ (N,M): `N==M` | `N≷M` | range(N)/range(M) | none ]  ×  [ (S1,S2): `S1==S2` | `S1≷S2` | range(S1)/range(S2) | none ]**

- all-`none` = the default-impl catch-all (as unary). Single-pair constraints first-class (`N==M && any S`, `any N,M && S1==S2`).
- Dispatch: `const fn select<const N, const M, const S1: u32, const S2: u32>() -> AlgoTag` → const `match` → const-generic algo.

> ⚠️ _compile-away honesty note:_ all four (`N`,`M`:`usize`; `S1`,`S2`:`u32`) are const generics ⇒ every predicate
> const-folds ⇒ **A holds, 4-key.** ✅ No runtime branch.
>
> ⚠️ _flag 1 — swap moves the OPERAND, not just `N`:_ the commutative larger-first swap (still commutative-only:
> `mul` yes; `div`/`sub`/`add` preserve order) must reorder each operand **as a `(width, scale)` unit** — swapping
> `Int<N>`↔`Int<M>` carries `S1`↔`S2` with it (operand 1 is `(N,S1)`). Still a const decision ⇒ compiles away. ✅
>
> ⚠️ _flag 2 — cross-scale (`S1≠S2`) is where the nightly line can bite:_ the **matcher** stays const/stable
> regardless. But an *arithmetic* algo selected for `S1≠S2` may need to align scales (×`10^|S1−S2|`) into a **wider
> width than `N` or `M`** to avoid overflow — and a *computed* wider width is the `generic_const_exprs` trap ⇒ that
> algo rides the **nightly roadmap bucket** (consistent with the standing "cross-scale arithmetic = nightly,
> cross-scale comparison = stable" decision). 0.5.0 stable D-binary: same-scale (`S1==S2`), or cross-scale that fits
> in an operand's own width (no computed widen). The dispatch never needs the computed width — only the algo body would.

## Compile-away verdict so far (assistant, 2026-05-23) — ✅ HOLDS
The full hierarchy (2 layers → per-function → 3 tiers, `std` = grid + subset of overrides + fall-through)
**remains compile-time-resolved**, composing as nested `const fn`s on the const generics:
`select(N,SCALE) = match std_override(N,SCALE) { Some(tag)=>tag, None=>non_std_select(N,SCALE) }`,
where `std_override`/`*_select` are `const fn` → a `const SEL` → const-scrutinee `match` (release dead-arm-elim).
`std` vs `non_std_wrapper` is `cfg` (only one compiled). The `Option` is const-eval, not runtime.
**Two conditions (the line):** (1) override/fall-through is a CONST decision keyed on `(N,SCALE)` — NOT a
runtime "run std, inspect result, maybe call non_std" (⚠️ that sentinel pattern is the break); (2) arms are
STATIC dispatch (const `match` → concrete generic fns), no `dyn`/fn-ptr. **Caveat:** debug keeps all arms
(Q4.3-a); zero-runtime-branch is a release property → **#18 IR/asm gate is the mandatory proof.**

## Work needed to land this (per policy → function)
a) **Evaluate each algorithm for duplicates → consolidate** into ONE **const-based algorithm that adjusts on the limb count `N`** (or another const-provided value). Many per-tier copies should collapse to one generic-over-`N` impl; the matcher then just picks ranges of it. Fed by the per-tier duplication map in `research/2026_05_23_phase4_algo_catalog.md` (`borrow_d57`, `lookup_dXXX_sYYY[_tang]`, `wide_kernel` per-tier, `generic_wide`-vs-bespoke). **This is the 4.1 "make algos generic" lift — the prerequisite for the matcher.**
b) **Rename + refactor the consolidated algos to a paper-name-based scheme.** **[NAMING = DECIDE LATER — parked; not part of the matcher discussion. Recommendation kept below for then.]** _Owner wants recommendations; names should match the actual paper-algorithm name where possible._
   - _Assistant recommendation:_ **adopt the existing `research/2026_05_22_algo_naming_standard.md` scheme** — it already mandates exactly this: **name = `<method>[_<subfunction>]`**, where `<method>` is the algorithm's **literature/paper name**; width (`Int<N>`/work-width) + `SCALE` are const-generic params; precomputed **tables are matcher-selected data**; **no `dXXX`/`sYYY`** in any module/fn name. Extend that note's method list (which was decimal-leaning) to the **int layer** with paper names:
     - mul: `schoolbook`, `karatsuba` (Karatsuba).
     - div: `mg_divide` (Möller–Granlund), `knuth` (Knuth Algorithm D), `burnikel_ziegler` (Burnikel–Ziegler), `newton_reciprocal` (Newton).
     - roots: `newton` (Newton–Raphson), `table_seed`.
     - transcendental (decimal): `tang` (Tang table-driven reduction), `taylor`/`series` (direct fixed-point), `near_pole`.
   - Dispatch helpers (`borrow_*`/`widen_*`) move OUT of `algos/` into the policy layer (strategy, not a method) — per the note.
   - **Algo names map 1:1 to the matcher table's "method" column** (single source of truth → feeds the completeness gate).
   - _Owner leaning (2026-05-23, still PARKED):_ **`function_algname[_variation]`** — function-FIRST, then the paper algorithm name, then an optional variation. E.g. `div_knuth_d`, `div_burnikel_ziegler`, `div_mg` (Möller–Granlund), `mul_karatsuba`. (Differs from the naming-standard note's method-first `<method>[_subfn]` — function-prefix groups a function's algo variants together and reads well as the matcher table's rows. Reconcile the two when we finalize naming.)
   - _Open naming Qs (need owner / a look at the code):_ (i) the direct fixed-point kernel — `taylor` vs `series` vs `fixed_point`? (ii) what is `lookup_d57_s18_22` (the non-`tang` exp/ln kernel) actually doing → name by its real method; (iii) int division — full paper names (`burnikel_ziegler`) or abbreviations (`bz`)?

_(c… to follow.)_

> ⚠️ _assistant note:_ renaming is neutral to compile-away (names don't affect const-fold). ✅ The 1:1
> `algo-name ↔ matcher-method` mapping is what makes the matcher table auditable for the completeness gate.

> ⚠️ _assistant note:_ this directly SERVES compile-away — one const-generic algo per matcher arm, selected by the const matcher. No risk; it's the enabling work.

## Matcher mechanics — decisions
- **#1 arm→algo binding = A (`AlgoTag` enum + const `select` + const-match) — ✅ LOCKED (owner 2026-05-23).**
  - _Rationale (owner 2026-05-23):_ the enum lets the **two matchers** (`std` select + `non_std` select, the 3-tier) sit **co-located in one per-function policy file** → readable; and it's an **easy fall-back to B** (direct const-if chain) if any one function's matcher gets too complex.
  - _Per-function enum_ (e.g. `IsqrtAlgo`), co-located with that function's algos — NOT a global enum (Rust enums can't be extended cross-file — variants live with the definition). A table-macro collapses "enum variant + select arm + dispatch arm" into one declarative row.
  - **MULTI-DIMENSIONAL (owner 2026-05-23):** `select` matches on the **tuple of const keys** — `match (N[, M][, S1][, S2]) { … }` — each position a single / range (`lo..=hi`) / `_` wildcard pattern, with relational rules (`N==M`, `S1≷S2`) as `if` **guards** over the const bindings. One enum + one tuple `select` per function regardless of arity (unary = 1-tuple). The per-position patterns ARE the single/range/none vocabulary; the conjunction is the tuple. _Compile-away:_ a const-`match` on a tuple of const generics (literal/range/`_` + const guards) is fully const-eval ⇒ folds to one variant. ✅
  - _Compile-away:_ ✅ `const { select::<N>() }` → const-scrutinee match → release dead-arm-elim (#18 asm gate is the proof).
  - **LOCKED:** A confirmed across all four quadrants — unary `Int<N>` (key `N`), binary `Int` (key `N,M`), unary `D` (key `N,S`), binary `D` (key `N,M,S1,S2`) — compile-away identical in each. Escape-hatch to B per-function remains if a matcher gets gnarly.
- **#2 authoring style = (a) ALL HAND-WRITTEN, no macro — ✅ LOCKED (owner 2026-05-23).**
  - _Rationale (owner):_ a `policy_table!` DSL is overkill — the per-function policy file is simple + readable enough that a macro would only add complication (hidden codegen, cryptic errors, awkward grammar on the multi-dim/guard arms). Write the enum + `const select` + dispatch `match` by hand.
  - The macro's headline benefit (enforced completeness) is already moot: the default-impl `_` arm (#3) gives totality for free.
- **#3 completeness = default-impl `_` arm — ✅ RESOLVED (owner 2026-05-23).** Every function has a generic default impl ⇒ the `match` is total by construction; **no `panic!`/`compile_error!` arm**. Zero runtime branch (DCE'd like any arm).

## VALUE MATCHER — canonical shape (✅ LOCKED, owner 2026-05-23)
A second, OPT-IN matcher tier for decisions that depend on the runtime **value**, not just the const keys. The
cardinal compile-away rule is **scoped to the const matcher**; the value matcher is an explicitly-acknowledged
runtime layer. Three layers, cleanly separated:
1. **const policy** (`select` → `Select`) — value-free, folds away per monomorphisation;
2. **algorithm dispatcher** (`match algo { … }`) — uniform, identical shape every function, maps each named algorithm to its call;
3. **value matcher** — localised, optional, the ONLY place the runtime value enters; carried by `Select::ByValue`.

### Canonical code (unary `Int<N>`, the simplest case)
```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { Newton, Zimmermann }                     // REAL algorithms ONLY — no `Default` variant

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),                               // const-settled → use this algorithm
    ByValue(fn(&Int<N>) -> Algorithm),                    // value-dependent → carries the value-matcher (non-capturing fn / closure)
}

const fn select<const N: usize>() -> Select<N> {          // value-free, real const fn
    match N {
        0..=2 => Select::ByAlgorithm(Algorithm::Newton),
        3..=8 => Select::ByValue(|v: &Int<N>|             // inline closure (≤2 outcomes ⇒ if/else)
            if v.bit_length() <= 128 { Algorithm::Newton } else { Algorithm::Zimmermann }),
        _     => Select::ByAlgorithm(Algorithm::Zimmermann),   // `Zimmermann` chosen as the wide DEFAULT (a real algo, our pick)
    }
}

pub fn isqrt<const N: usize>(x: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {           // inline const block: folds, CAN see `N`
        Select::ByAlgorithm(a) => a,                      // value-free
        Select::ByValue(f)     => f(&x),                  // the ONE place the value enters → returns an Algorithm
    };
    match algo {                                          // EXHAUSTIVE over the real algorithms — no panic, no `_`, no unreachable
        Algorithm::Newton     => isqrt_newton::<N>(x),
        Algorithm::Zimmermann => isqrt_zimmermann::<N>(x),
    }
}
```
> ⚠️ **Use the inline `const { select::<N>() }` block, NOT a `const SEL: Select<N> = …` item** — a `const` *item* inside a generic fn cannot reference the fn's generic `N` (E0401). The inline `const { }` block can, and still forces compile-time evaluation. _(Verified both this and a non-capturing closure-in-`const fn`, with + without a `std` feature, compile on rustc 1.95.)_

### The decisions baked in
- **Two enums: `Algorithm` (real algorithms ONLY) + `Select<N>` (`ByAlgorithm(Algorithm)` | `ByValue(fn(&Int<N>) -> Algorithm)`).**
- **No `Default` variant.** Completeness is structural: `select`'s `match` is **total over the key** (every width maps to a real algorithm or a value-matcher) AND `match algo` is **exhaustive over the real `Algorithm` enum** — both compiler-enforced ⇒ no `unreachable!()`, no `_` catch-all, no panic. The "default impl" (#3) is simply **the algorithm named in `select`'s `_` arm** (here `Zimmermann`) — a real algo, chosen per function, NOT a fictional universal variant.
- **`ByValue` carries the value-matcher; non-capturing only** (it bases itself purely on the value — no captures ⇒ no `dyn`/heap ⇒ `no_std`-safe). The const layer + dispatcher stay **value-free**; only the carried matcher takes the value.
- **Value-matcher placement (taste, all prune identically):** **≤2 outcomes ⇒ inline closure `if`/`else`**; **3–10 ⇒ inline closure `match`**; **extract to a named `#[inline]` fn when it's large (>10), shared across widths, or directly unit-tested.** All three coerce to `fn(&Int<N>) -> Algorithm`.
  - **Extracted-matcher naming = `<function_name>_<applicable_preconditions>`** — the suffix names exactly the arm's applicable preconditions, and **its count/shape varies with the matcher**: a single int width ⇒ `sqrt_N5`; an int width-range ⇒ `sqrt_N5_to_N10`; a decimal arm adds scale ⇒ `sqrt_N2_S0_to_S9`. Self-documents which `select` arm it serves. _(NB: the **algorithms** stay paper-named — `Newton`/`Zimmermann`/… per the algo-naming-standard, NO width encoding; only the **matcher** fn encodes its preconditions, since they ARE its identity. Distinct rules.)_
- **Const-first / value-last:** `select` is const → `const { select::<N>() }` folds per monomorphisation; `ByAlgorithm` widths drop the value path entirely (DCE), only `ByValue` widths keep a runtime branch.
- **Value-matcher returns an `Algorithm` TAG, never a fn-ptr** — so the dispatch `match algo` is a **direct call** even on value paths. (Putting algo fns *in the enum variants* was rejected: a runtime-selected fn-ptr can't const-propagate ⇒ genuine indirect call on value paths + erases the named-algorithm audit surface. No point.)

> ⚠️ _compile-away (honest, final):_ with a **non-capturing** matcher (closure or fn item) in an inline `const { }` block, the carried
> `f` is a **compile-time-constant pointer** ⇒ `f(&x)` const-propagates to a **direct call** in release (the SAME fold
> that resolves the `ByAlgorithm` arms — NOT speculative devirtualisation of an arbitrary pointer). Debug keeps the
> fn-ptr load + indirect call, which Q4.3-a (a) already accepts. #18 IR/asm gate confirms no indirect call in release.
> Capturing closures forbidden (`dyn`/heap ⇒ breaks `no_std` + compile-away). Only genuine runtime cost = the value
> comparison itself. _(MSRV: verified on rustc 1.95; re-check if a lower MSRV is pinned.)_ ✅

### Trichotomy (reframes Q4.3-b)
- value decision that picks **between algorithms** → the **value matcher** (`ByValue`, policy layer, returns a `Leaf`);
- value-dependent **steps within** a single algorithm (Newton convergence, division normalisation) → stay in the **algo body**;
- which algorithm by **width/scale** → the **const matcher** (`select`).

## OPEN QUESTIONS (for our discussion)
- ~~Q4.2-a~~ RESOLVED → #1 = A. ~~(macro vs hand-written)~~ RESOLVED → #2 = (a) hand-written.
- ~~Q4.2-b~~ **RESOLVED → (b) (owner 2026-05-23):** source = #64 best-algorithm analysis; gate = precision golden **+ MANDATORY explicit golden coverage of every matcher arm's range boundaries** (a precision cliff at an arm edge must not slip through). Executed within the agreed/pinned **per-function 4.1→4.2 track** (`project_cross_scale_binary_d_boundary` + Phase-4 📌) — boundary cases added as each function is consolidated/wired.
- ~~Q4.3-a~~ **RESOLVED → (a) (owner 2026-05-23):** accept that debug keeps all `match` arms; zero-runtime-branch is a RELEASE property certified by the #18 IR/asm gate. Debug residue = a branch on a compile-time constant (predicted, correctness-neutral) in a non-shipped binary. Locked-A unchanged; #18 proof runs on release.
- ~~Q4.3-b~~ **RESOLVED (owner 2026-05-23) → the VALUE MATCHER tier (above).** Value decisions between *algorithms* become a first-class opt-in `ByValue(fn(&Int<N>) -> Leaf)` value matcher (runtime, returns a tag → direct dispatch); value-dependent *steps within* an algorithm stay in the algo body. Cataloguing which functions need a `*_by_value` rides the per-function 4.1→4.2 track.
