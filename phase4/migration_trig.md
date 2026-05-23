<!--
SPDX-FileCopyrightText: 2026 John Moxley
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Phase-4 migration plan — TRIG family (read-only analysis, 2026-05-23)

Companion to `research/2026_05_23_phase4_algo_catalog.md` (B.b duplication map),
`research/2026_05_23_4.2_policy_matcher_design.md` (the const-matcher / value-matcher
shape), `research/2026_05_22_algo_naming_standard.md` (paper-name scheme), and
`docs/ARCHITECTURE.md` → "Policy file structure". Mirrors the per-function section
format intended for the `roots` doc.

This records WHAT future implementing agents do; it changes no code. Scope: `sin`,
`cos`, `tan`, `atan`, `asin`, `acos`, `atan2`, plus the hyperbolics `sinh`/`cosh`/
`tanh` (and their `*_with` working-digits siblings).

## Naming rules applied here
- **Algorithm fn/module name** = `<function>_<method>[_with_<method2>][_variant]`
  (owner correction 2026-05-23):
  - PREPEND the function performed (`sin_`, `cos_`, `atan_`, `sinh_`, …). NO bare
    `tang`/`series`/`near_pole` — always `sin_tang`, `atan_series`, `tan_near_pole`.
  - Hybrids (the common case) join methods with `_with_`: `sin_tang_with_taylor`.
  - `_variant` only where it genuinely distinguishes.
  - Widths stay const-generic params, NEVER name-encoded. A genuinely width-bespoke
    kernel may suffix the Int-width form (`_int2`), never `dXX`.
  - Method = literature/paper name; `SCALE`/work-width are const-generic; lookup
    tables are matcher-selected DATA, not part of the name.
- **Per-function `Algorithm` enum variant** keeps the short scoped method name
  (`Tang`, `Series`, `NearPole`) — it is already namespaced by the function.
- **Value-matcher fns** = `<function_name>_<applicable_preconditions>` (e.g.
  `tan_n3_s44_to_s56`), per the 4.2 value-matcher rule.

## Cross-cutting picture (read before the per-function sections)

Today's trig kernels fall into FIVE METHOD families once tier/scale encoding is
stripped. Below, the **method** is the bare concept; the actual fn/module names are
function-prefixed per the naming rule (e.g. method `series` for sin → `sin_series`).

1. **method `series`** — the macro-emitted `sin_fixed` / `cos_fixed` / `atan_fixed`
   cores (Taylor / Euler-accelerated on a range-reduced residue). Today this is
   `wide_kernel::*_strict_dXXX` per tier, the D38 `fixed_d38` kernels, AND the
   **narrow-GUARD lookup slots** (`lookup_d57_s18_22_*`, `lookup_d153_s70_82_atan`,
   …) which are NOT a different algorithm — they call the SAME cores with a smaller
   `GUARD` constant. The GUARD value is a `const fn(SCALE)`, so the narrow-GUARD
   slots collapse INTO the series method as a const-table guard choice.
   → names: `sin_series`, `cos_series`, `tan_series`, `atan_series`,
   `asin_series` / `acos_series`, `atan2_series`. (Maclaurin/Taylor is the method;
   the Euler-accelerated atan variant is `atan_series` — same family, no `_with_`
   needed since acceleration is an internal series detail, not a second method.)
2. **method `tang`** — Tang table-driven argument reduction (Tang 1991, ACM TOMS
   17(4)): table lookup of `sin/cos(c_j)` THEN a short Taylor on the residual `δ`.
   That residual recombination makes it a genuine HYBRID → `_with_taylor`. Today:
   `lookup_d57_s44_56_sincos`, `lookup_d153_s70_82_sincos`,
   `lookup_d307_s140_160_sincos`, `lookup_d462_s225_235_sincos`, and the matching
   `*_atan` Tang slots. A genuinely DISTINCT algorithm → survives. The shared `M`
   (table size) and `compute_table` machinery is duplicated per slot and must de-dup
   to one generic core parameterised by `const M` + work width.
   → names: `sin_tang_with_taylor`, `cos_tang_with_taylor`, `tan_tang_with_taylor`,
   `atan_tang_with_taylor`.
3. **method `near_pole` (tan only)** — `near_pole_tan` is not a standalone kernel; it
   is the guard-digit SIZING helper for `tan = sin/cos` near ±π/2. It is a
   value-dependent working-width lift. Per 4.2 trichotomy this is a **value-dependent
   step WITHIN the `tan` algorithm**, NOT a matcher arm — it stays inside the `tan`
   algo body. → helper name `tan_near_pole` (a sizing helper, not a matcher
   `Algorithm`).
4. **method `hyper`** — sinh/cosh/tanh via the `(eˣ, e⁻ˣ)` identity, reusing `exp`'s
   kernels (incl. `tang_exp_fixed`). Today the `lookup_dXXX_sYYY_hyper` slots are
   narrow-GUARD / Tang-routed overrides; the generic fall-through is the
   macro-emitted inherent `*_strict_with`. Collapses to one hyper method over `exp`.
   → names: `sinh_hyper`, `cosh_hyper`, `tanh_hyper` (where the override routes the
   inner exp through the Tang exp kernel, that is `sinh_hyper_with_tang` etc. — a
   hybrid only when the inner `exp` kernel is genuinely Tang, see the sinh/cosh/tanh
   section).
5. **`borrow` / `widen`** — DISPATCH STRATEGIES, not algorithms (no function prefix;
   they are not in any `Algorithm` enum). `borrow_d57` (D38→D57→narrow) and
   `widen_to_d38` (D18→D38→narrow) move OUT of `algos/` into the policy layer (per the
   naming standard's "dispatch helpers are not algorithms").

**Shared de-dup target (Q4.1b):** the `compute_table` / `const M` / `GUARD_NARROW` /
quadrant-permutation machinery is copy-pasted across every Tang and narrow-GUARD
slot. Consolidation pulls this into ONE generic core per method (`*_series`,
`*_tang_with_taylor`, `*_hyper`) parameterised on work width + `const M` + a
`const fn guard(SCALE)`.

**`_tang`/non-`_tang` orphans:** `lookup_d57_s18_22_sincos` (non-tang, LIVE narrow-
GUARD `*_series`) and `lookup_d57_s18_22_sincos_tang` (Tang, DEAD) are the orphan pair —
the policy wires the non-tang one; the tang one is dead. Drop the dead orphan.

**`_with` (working-digits) siblings:** the wide tiers ship NO runtime-`working_digits`
variant of the cores; every wide `*_with_impl` ignores `wd` and delegates to strict.
This is a per-method property, not a matcher axis — fold into each method (the `_with`
entry is the strict entry on wide tiers; only D38/D18 honour `wd`).

---

## `sin`

### Current kernels inventory
- `wide_kernel::sin_strict_d{57,76,115,153,230,307,462,616,924,1232}` — per-tier
  free fns, all delegate to the macro-emitted `sin_fixed` core (Taylor on reduced
  residue). **LIVE** (the `(wtag, _)` default arm for every wide tier). `series`.
- `fixed_d38::sin_strict` / `sin_with` — D38 256-bit `Fixed` kernel. **LIVE** (D38
  forward path, both cfg branches). `series` (narrow, bespoke width).
- `widen_to_d38::sin_strict_d18` / `sin_with_d18` — D18 widen→D38→narrow. **LIVE**
  (D18 policy). DISPATCH STRATEGY (`widen`).
- `borrow_d57::sin_strict` — D38→D57→narrow. **DEAD** (catalog A: D38 uses
  `fixed_d38::sin_strict`, not borrow). Drop.
- `lookup_d57_s18_22_sincos::sin_strict` — narrow-GUARD slot, SCALE 18..=22.
  **LIVE** (D57 band arm). Folds into `series` (GUARD = const fn of SCALE).
- `lookup_d57_s44_56_sincos::sin_strict` — Tang table-driven, SCALE 44..=56.
  **LIVE** (D57 band arm). `tang`.
- `lookup_d153_s70_82_sincos::sin_strict` (SCALE 70..=82, D153, **LIVE**),
  `lookup_d307_s140_160_sincos::sin_strict` (140..=160, D307, **LIVE**),
  `lookup_d462_s225_235_sincos::sin_strict` (225..=235, D462, **LIVE**) — all Tang.
  `tang`.
- `lookup_d57_s18_22_sincos_tang::sin_strict` — Tang slot, SCALE 18..=22. **DEAD**
  (catalog A: policy uses the non-tang `lookup_d57_s18_22_sincos`). Drop.

### Consolidation verdict per kernel (Q4.1b)
- `wide_kernel::sin_strict_d*` → **collapse to generic-over-`N`/`SCALE` `series`**.
  The per-tier wrappers add four lines each over the same `sin_fixed` core; lift the
  work width `W` and pi tables to const-generic / matcher-selected data. (Counter-note
  in `wide_kernel.rs`: the macro emits four cores per tier against a tier-specific `W`
  + pi table; collapsing needs a `WideTrigCore`-style generic core or a const-`W`
  parameterisation. This IS the 4.1 lift — do it.)
- `fixed_d38::sin_strict` → **stays width-bespoke** (the 256-bit `Fixed`/`i128` narrow
  kernel is a genuinely different work representation; keep as a narrow series variant
  named `sin_series_int2`). Candidate to fold into generic `sin_series` only if `Int<2>`
  work width reproduces its perf — verify before merging.
- narrow-GUARD `lookup_d57_s18_22_sincos` → **collapse INTO `sin_series`** (GUARD is a
  `const fn(SCALE)`; not a separate algorithm).
- `lookup_d{57_s44_56,153,307,462}_sincos` → **collapse to ONE generic
  `sin_tang_with_taylor`** (de-dup `M` + `compute_table` + quadrant permutation; work
  width + `const M` as params; tables matcher-selected by SCALE band).
- `widen_to_d38::sin_*_d18`, `borrow_d57::sin_strict` → **DROP from `algos/`**: borrow
  is DEAD (delete); widen is a strategy → move to policy layer.
- `lookup_d57_s18_22_sincos_tang::sin_strict` → **DROP** (dead).

### Proposed names
- `sin_series` — direct Taylor on reduced residue (generic over `N`/`SCALE`).
- `sin_series_int2` — width-bespoke D38/`Int<2>` `Fixed` variant.
- `sin_tang_with_taylor` — Tang table lookup + residual Taylor (shares the `sincos`
  table core with `cos_tang_with_taylor`).

### Target policy (`D<Int<N>, S>` unary, key `(N, SCALE)`)
```
enum Algorithm { Series, Tang }          // sin: real algorithms only
const fn select<N, SCALE>() -> Select {
    match (N, SCALE) {
        (2, _)        => Series,         // D38 narrow Fixed (bespoke series variant)
        (3, 44..=56)  => Tang,           // D57 Tang band
        (8, 70..=82)  => Tang,           // D153 Tang band
        (16, 140..=160) => Tang,         // D307 Tang band
        (24, 225..=235) => Tang,         // D462 Tang band
        _             => Series,         // DEFAULT (wide_kernel series; also D57 18..=22 narrow-GUARD)
    }
}
```
- Value-matcher: none for `sin`.
- `_with` (working digits): D18/D38 honour `wd`; wide tiers ignore `wd` → `Series`
  strict (a per-method property recorded in the `Series` body, not a matcher arm).
- D18 routes by widening to N=2 (handled in policy `widen` strategy, not a `sin` algo).

### Dead to drop
- `borrow_d57::sin_strict`, `lookup_d57_s18_22_sincos_tang::sin_strict`.

---

## `cos`

### Current kernels inventory
- `wide_kernel::cos_strict_d*` — per-tier; uses dedicated `cos_fixed` (cofunction
  identity `cos(x)=sin(π/2−x)`, one `sin_fixed`, no sqrt). **LIVE** (wide default).
  `series`.
- `fixed_d38::cos_strict` / `cos_with` — D38 Fixed kernel. **LIVE**. `series`.
- `widen_to_d38::cos_strict_d18` / `cos_with_d18` — **LIVE**. `widen` strategy.
- `borrow_d57::cos_strict` — **DEAD** (D38 uses `fixed_d38`). Drop.
- `lookup_d57_s18_22_sincos::cos_strict` (18..=22, **LIVE**, narrow-GUARD `series`),
  `lookup_d57_s44_56_sincos::cos_strict` (44..=56, **LIVE**, `tang`),
  `lookup_d153_s70_82_sincos::cos_strict` (70..=82, **LIVE**, `tang`),
  `lookup_d307_s140_160_sincos::cos_strict` (140..=160, **LIVE**, `tang`),
  `lookup_d462_s225_235_sincos::cos_strict` (225..=235, **LIVE**, `tang`).
- `lookup_d57_s18_22_sincos_tang::cos_strict` — **DEAD**. Drop.

### Consolidation verdict per kernel (Q4.1b)
Identical to `sin`: `wide_kernel::cos_strict_d*` → generic `series`; the four Tang
slots → one generic `tang` (sub-function `cos`, shares the `sincos` table/M machinery
with `sin` — same `compute_table`); `fixed_d38::cos_strict` stays width-bespoke;
narrow-GUARD 18..=22 folds into `series`; `borrow`/`widen` leave `algos/`;
`_sincos_tang` dropped.

### Proposed names
- `cos_series` — cofunction-identity (`cos(x)=sin(π/2−x)`, one `sin_fixed`, no sqrt).
- `cos_series_int2` — width-bespoke D38/`Int<2>` `Fixed` variant.
- `cos_tang_with_taylor` — Tang lookup + residual Taylor (shares the `sincos` table
  core with `sin_tang_with_taylor`).

### Target policy
Same `(N, SCALE)` table as `sin` — `Series` default, `Tang` on the D57 44..=56 /
D153 70..=82 / D307 140..=160 / D462 225..=235 bands, `Series` for D38 (N=2). No
value-matcher. Wide `_with` ignores `wd` → strict `Series`.

### Dead to drop
- `borrow_d57::cos_strict`, `lookup_d57_s18_22_sincos_tang::cos_strict`.

---

## `tan`

### Current kernels inventory
- `wide_kernel::tan_strict_d*` — per-tier; `tan = sin/cos` on the reduced residue.
  **LIVE** (wide default). `series`.
- `fixed_d38::tan_strict` / `tan_with` — D38 Fixed; panics at `tan(±π/2)`. **LIVE**.
  `series`.
- `widen_to_d38::tan_strict_d18` / `tan_with_d18` — **LIVE**. `widen` strategy.
- `borrow_d57::tan_strict` — **DEAD** (D38 uses `fixed_d38`). Drop.
- `lookup_d57_s18_22_sincos::tan_strict` (18..=22, **LIVE**, narrow-GUARD `series`),
  `lookup_d57_s44_56_sincos` — note: D57 tan has NO 44..=56 band (policy wires only
  sin/cos to `lookup_d57_s44_56_sincos`; tan at 44..=56 falls to `wide_kernel`).
- `lookup_d153_s70_82_sincos::tan_strict` (70..=82, **LIVE**, `tang`),
  `lookup_d307_s140_160_sincos::tan_strict` (140..=160, **LIVE**, `tang`),
  `lookup_d462_s225_235_sincos::tan_strict` (225..=235, **LIVE**, `tang`).
- `lookup_d57_s18_22_sincos_tang::tan_strict` — **DEAD**. Drop.
- `near_pole_tan::*` — guard-digit SIZING helper (Muller §11.1). **LIVE**. NOT an
  algorithm and NOT a matcher arm.

### Consolidation verdict per kernel (Q4.1b)
- `wide_kernel::tan_strict_d*` → generic `series`.
- Tang `tan` slots (D153/D307/D462; note the D57 tan-band GAP at 44..=56) → fold into
  the one generic `tang` (sub-function `tan`).
- narrow-GUARD `lookup_d57_s18_22_sincos::tan_strict` → into `series`.
- `fixed_d38::tan_strict` → stays width-bespoke.
- `near_pole_tan` → **stays as a value-dependent step INSIDE the `tan` algo** (4.2
  trichotomy: value-dependent working-width lift, not a between-algorithm choice). Do
  NOT promote to a matcher arm. Keep as a shared helper `tan_near_pole` called by both
  the `tan_series` and `tan_tang_with_taylor` paths.
- `borrow`/`widen`/`_sincos_tang` as for sin/cos.
- **Orphan note:** the D57 44..=56 tan band is an asymmetry — sin/cos get Tang there,
  tan does not. When consolidating, decide whether tan joins the 44..=56 Tang band or
  documents the gap. Record as a boundary case for the golden gate.

### Proposed names
- `tan_series` — `sin_fixed`/`cos_fixed` quotient + the `tan_near_pole` guard lift.
- `tan_series_int2` — width-bespoke D38/`Int<2>` `Fixed` variant.
- `tan_tang_with_taylor` — Tang lookup + residual Taylor.
- `tan_near_pole` — guard-digit sizing helper (NOT an `Algorithm` variant; called
  inside the `tan_*` bodies).

### Target policy
```
enum Algorithm { Series, Tang }
match (N, SCALE) {
    (2, _)          => Series,           // D38
    (8, 70..=82)    => Tang,             // D153
    (16, 140..=160) => Tang,             // D307
    (24, 225..=235) => Tang,             // D462
    _               => Series,           // wide default + D57 (incl. 18..=22, 44..=56)
}
```
- `near_pole` guard-digit lift lives inside the algo bodies, value-dependent.
- No `ByValue` matcher arm (the near-pole decision is intra-algorithm, not a
  cross-algorithm pick).

### Dead to drop
- `borrow_d57::tan_strict`, `lookup_d57_s18_22_sincos_tang::tan_strict`.

---

## `atan`

### Current kernels inventory
- `wide_kernel::atan_strict_d*` — per-tier; `atan_fixed` (Taylor / Euler-accelerated,
  half-angle pre-reduction for `|x|>1`). **LIVE** (wide default). `series`.
- `fixed_d38::atan_strict` / `atan_with` — D38 Fixed adaptive-halvings path. **LIVE
  only in the no-d57 cfg branch**; when D57 is present D38 atan routes through
  `borrow_d57::atan_strict` (the wide_kernel atan is ~2× faster — see policy comment
  L424-432).
- `borrow_d57::atan_strict` — D38→D57→narrow. **LIVE** (D38 atan path under d57/wide).
  `borrow` DISPATCH STRATEGY (not an algorithm) — but its survival is the reason the
  D38 atan/asin/acos/atan2 siblings are NOT dead (contrast the sin/cos/tan borrow,
  which IS dead).
- `widen_to_d38::atan_strict_d18` / `atan_with_d18` — **LIVE**. `widen` strategy.
- `lookup_d57_s18_22_atan::atan_strict` (18..=22, **LIVE**, narrow-GUARD `series`),
  `lookup_d57_s44_56_atan::atan_strict` (44..=56, **LIVE**, `tang`),
  `lookup_d153_s70_82_atan` (70..=82, **LIVE**, `tang`),
  `lookup_d307_s140_160_atan` (140..=160, **LIVE**, `tang`),
  `lookup_d462_s225_235_atan` (225..=235, **LIVE**, `tang`).

### Consolidation verdict per kernel (Q4.1b)
- `wide_kernel::atan_strict_d*` → generic `atan_series`.
- narrow-GUARD `lookup_d57_s18_22_atan` / `lookup_d153_s70_82_atan` → into `atan_series`
  (GUARD = const fn of SCALE).
- Tang atan slots (`lookup_d57_s44_56_atan`, `lookup_d{307,462}_*_atan`) → one generic
  `atan_tang_with_taylor`. NOTE the atan Tang machinery differs from sincos Tang — it
  is a separate `compute_table` (atan-specific table). De-dup the GUARD/M scaffold but
  keep the atan table distinct from the sincos table.
- `fixed_d38::atan_strict` → **stays width-bespoke** (`atan_series_int2`) but is the
  SLOWER D38 path; the bespoke choice is encoded by `borrow` strategy in policy.
- `borrow_d57::atan_strict` → **move to policy layer** as a `borrow` strategy. The D38
  atan policy arm should be expressed as "compute via the N=3 (D57) `atan_series` /
  `atan_tang_with_taylor` algo, narrow back" — a strategy, not a `fixed_d38` algorithm
  pick. This is the one trig function whose D38 borrow is LIVE.
- `widen_to_d38::atan_*_d18` → policy `widen` strategy.

### Proposed names
- `atan_series` — Taylor/Euler-accelerated on the reduced argument (acceleration is an
  internal series detail, so no `_with_`).
- `atan_series_int2` — width-bespoke D38/`Int<2>` `Fixed` adaptive-halvings variant.
- `atan_tang_with_taylor` — Tang lookup (atan-specific table) + residual Taylor.
- (`borrow`/`widen` = policy strategies, not algo names.)

### Target policy
```
enum Algorithm { Series, Tang }
match (N, SCALE) {
    (2, _)          => Series,   // D38 — via borrow-to-N3 strategy when d57 present;
                                 //       fixed_d38 series in the no-d57 build
    (3, 44..=56)    => Tang,     // D57
    (16, 140..=160) => Tang,     // D307
    (24, 225..=235) => Tang,     // D462
    (8, 70..=82)    => Tang,     // D153
    _               => Series,   // wide default + narrow-GUARD bands
}
```
- D38 `(2,_)`: cfg-gated — with `d57`, the policy applies the `borrow`-to-D57 strategy
  around the D57 `select` result; without `d57`, runs the bespoke `fixed_d38` series.
  Record this as a policy-level strategy decision (consistent with ARCHITECTURE
  feature-flag-a-variation), NOT two `Algorithm` variants.
- Wide `_with` ignores `wd` → strict.

### Dead to drop
- None unique to `atan` (borrow atan is LIVE — relocate, don't delete).

---

## `asin` / `acos`

### Current kernels inventory
- `fixed_d38::asin_strict` / `acos_strict` (+ `_with`) — D38 Fixed, two-range
  half-angle reduction. **LIVE only in the no-d57 cfg branch**.
- `borrow_d57::asin_strict` / `acos_strict` — D38→D57→narrow (D57 inherent
  `*_strict_with`). **LIVE** (D38 asin/acos under d57/wide).
- `widen_to_d38::asin_strict_d18` / `acos_strict_d18` (+ `_with`) — **LIVE** (D18).
  `widen` strategy.
- `lookup_d57_s18_22_inverse::asin_strict` / `acos_strict` — narrow-GUARD slot, SCALE
  18..=22; `asin(x)=atan(x/√(1−x²))` w/ half-angle reduction, `acos=π/2−asin`.
  **LIVE** (D57 band arm). Folds into `series` (inverse sub-function).
- Wide tiers (D76, D115, D153, D230, D307, D462, D616, D924, D1232): asin/acos
  **delegate to the macro-emitted inherent `asin_strict_with`/`acos_strict_with`**
  (no `algos/` kernel) — these compose `atan_fixed` + `sqrt_fixed` + half-pi. **LIVE**
  but currently NOT a free-fn kernel.

### Consolidation verdict per kernel (Q4.1b)
- The inherent `*_strict_with` composition + the `lookup_d57_s18_22_inverse` slot are
  the SAME method (atan-of-ratio with half-angle reduction) at different GUARD →
  **collapse to one generic `asin_atan_with_sqrt` / `acos_atan_with_sqrt`**, lifted out
  of the inherent macro into an `algos/` free fn so the policy can name it (today it is
  the one trig family with no free-fn kernel for the wide tiers — the 4.1 lift extracts
  it). The `_with_sqrt` reflects the genuine hybrid composition (`atan(x/√(1−x²))`).
- narrow-GUARD `lookup_d57_s18_22_inverse` → into those generic fns as the band GUARD
  choice.
- `fixed_d38::asin/acos_strict` → stays width-bespoke (`asin_atan_with_sqrt_int2` /
  `acos_atan_with_sqrt_int2`, no-d57 build).
- `borrow_d57::asin/acos_strict` → policy `borrow` strategy (D38 under d57).
- `widen_to_d38::asin/acos_*_d18` → policy `widen` strategy.

### Proposed names
- `asin_atan_with_sqrt` / `acos_atan_with_sqrt` — atan-of-ratio (`atan(x/√(1−x²))`) +
  half-angle reduction; `acos = π/2 − asin` shares the body. (No Tang variant exists
  for the inverse family today.)
- `asin_atan_with_sqrt_int2` / `acos_atan_with_sqrt_int2` — width-bespoke D38 variants.

### Target policy
```
enum Algorithm { Atan }              // single method today (atan-of-ratio); variant named for the method
match (N, SCALE) { _ => Atan }       // total; D38 borrow + D18 widen are policy strategies
```
- Action item: extract the wide-tier inherent `asin_strict_with`/`acos_strict_with`
  into the named `asin_atan_with_sqrt`/`acos_atan_with_sqrt` free fns so the matcher
  table has a real method to point at (closes the "macro-emitted, no algos kernel" gap
  flagged in the policy docs L578).

### Dead to drop
- None (`lookup_d57_s18_22_inverse` is LIVE; the dead `borrow_d57` is sin/cos/tan only,
  not asin/acos).

---

## `atan2`

### Current kernels inventory
- `fixed_d38::atan2_strict` / `atan2_with` — D38 `atan2_kernel` on `Fixed`. **LIVE only
  in the no-d57 cfg branch**.
- `borrow_d57::atan2_strict` — D38→D57→narrow. **LIVE** (D38 atan2 under d57/wide).
- `widen_to_d38::atan2_strict_d18` / `atan2_with_d18` — **LIVE** (D18). `widen`.
- `lookup_d57_s18_22_inverse::atan2_strict` — narrow-GUARD slot, SCALE 18..=22.
  **LIVE** (D57 band arm). Folds into `series`.
- Wide tiers: atan2 **delegates to inherent `atan2_strict_with`** (composes `atan` +
  quadrant logic). **LIVE**, no free-fn kernel today.

### Consolidation verdict per kernel (Q4.1b)
- inherent `atan2_strict_with` + `lookup_d57_s18_22_inverse::atan2_strict` = one method
  (quadrant dispatch around `atan`) → **collapse to one generic `atan2_atan`** (the
  `_atan` reflects that atan2 is built ON the atan algorithm + quadrant logic),
  extracted out of the inherent macro into a named free fn.
- `fixed_d38::atan2_strict` stays width-bespoke (`atan2_atan_int2`, no-d57).
  `borrow_d57::atan2_strict` → policy `borrow`. `widen_to_d38::atan2_*` → policy `widen`.

### Proposed names
- `atan2_atan` — quadrant dispatch around the `atan_*` algorithm.
- `atan2_atan_int2` — width-bespoke D38 `atan2_kernel` variant.

### Target policy (`D` binary, key `(N, M, S1, S2)` — but same-type so `N==M`,
`S1==S2`)
```
enum Algorithm { Atan }              // one method (quadrant dispatch around atan)
match (N, SCALE) { _ => Atan }       // atan2 is same-type (N==M, S1==S2); no mixed-width
```
- atan2 takes two operands of the SAME `D` type → the binary key degenerates to
  `(N, SCALE)`; no canonicalising swap (atan2 is order-significant: `atan2(y, x)`).
- Extract inherent `atan2_strict_with` into the named `atan2_atan` free fn (as for
  asin/acos).

### Dead to drop
- None.

---

## `sinh` / `cosh` / `tanh`

### Current kernels inventory
- `fixed_d38::sinh_strict` / `cosh_strict` / `tanh_strict` (+ `_with`) — D38 Fixed via
  `exp_fixed` + the `(eˣ,e⁻ˣ)` identity. **LIVE** (D38, both cfg branches — the
  `d38_hyperbolic_and_angle!` macro). `hyper`.
- Narrow D18: macro-emitted shells widen → D38 → narrow (`impl_narrow_trig`
  hyperbolic tail). **LIVE**. `widen` strategy.
- `lookup_d57_s18_22_hyper::{sinh,cosh,tanh}_strict` (18..=22, D57, **LIVE**,
  narrow-GUARD `hyper`),
  `lookup_d115_s57_hyper` (50..=60, D115, **LIVE**),
  `lookup_d153_s70_82_hyper` (70..=82, D153, **LIVE**),
  `lookup_d307_s140_160_hyper` (140..=160, D307, **LIVE**) — all the `(eˣ,e⁻ˣ)`
  identity with a single `exp_fixed` (or `tang_exp_fixed`) + a wide divide.
- `lookup_d616_s300_315_hyper` — **DEAD** (catalog A: entire module dead; hyperbolics
  route through the macro core). Drop.
- `lookup_d1232_s610_620_hyper` — present module; per catalog the d616/d1232 hyper
  slots are the dead Tang-routed ones. **DEAD-or-suspect**: not wired in any policy arm
  (D924/D1232 use `wide_trig_delegating_tail!` → inherent shells; no hyper band arm).
  Treat as DEAD pending the free-fn-reference check; drop with the d616 sibling.
- Wide tiers without a hyper band (D76, D230, D462, D616, D924, D1232): delegate to
  inherent `sinh_strict_with` etc. **LIVE** via inherent macro.

### Consolidation verdict per kernel (Q4.1b)
- All the LIVE `lookup_*_hyper` slots + the inherent `*_strict_with` + `fixed_d38`
  hyperbolics are the SAME method (`(eˣ,e⁻ˣ)` identity over `exp`) at different GUARD /
  exp-kernel choice → **collapse to generic `sinh_exp_identity` / `cosh_exp_identity` /
  `tanh_exp_identity`** that call the (already-consolidated) `exp` algo. The per-band
  override is a GUARD + exp-kernel (`tang_exp_fixed` vs `exp_fixed`) choice =
  const-of-SCALE, NOT a separate algorithm. Where the inner exp is genuinely the Tang
  exp kernel and it benches as a distinct path, name that hybrid
  `sinh_exp_identity_with_tang` (etc.) and add a second `Algorithm` variant.
- `lookup_d616_s300_315_hyper`, `lookup_d1232_s610_620_hyper` → **DROP** (dead).
- `fixed_d38` hyperbolics → stay width-bespoke (or fold into generic `hyper` over the
  D38 `exp` if perf holds).
- The dependency on `exp::lookup_d307_s140_160_tang::tang_exp_fixed` + `GUARD_FOR_HYPER`
  is the cross-family LIVE link (catalog A): the trig consolidation must keep that exp
  symbol alive — coordinate with the `exp` migration doc so the hyper kernel's
  `tang_exp_fixed` consumer is not orphaned.

### Proposed names
- `sinh_exp_identity` / `cosh_exp_identity` / `tanh_exp_identity` — `(eˣ,e⁻ˣ)`
  identity over the `exp` algorithm.
- `sinh_exp_identity_with_tang` / … — hybrid where the inner exp is the Tang exp
  kernel (only if it benches as a distinct path; else fold the Tang choice as a
  const-of-SCALE inside the base name).

### Target policy (`D` unary, key `(N, SCALE)`) — shown for `sinh`; `cosh`/`tanh` same
```
enum Algorithm { ExpIdentity }       // one method; GUARD + exp-kernel are const-of-SCALE inside it
match (N, SCALE) { _ => ExpIdentity }
```
- The SCALE bands (D57 18..=22, D115 50..=60, D153 70..=82, D307 140..=160) become
  internal const guard/exp-kernel choices inside `ExpIdentity`, not matcher arms —
  UNLESS a band uses a qualitatively different exp kernel that benches as a distinct
  algorithm, in which case add an `ExpIdentityTang` variant (fn
  `sinh_exp_identity_with_tang`). Verify per band against #64.
- D18 widen + D38 route handled as policy strategy / bespoke as elsewhere.

### Dead to drop
- `lookup_d616_s300_315_hyper` (whole module), `lookup_d1232_s610_620_hyper` (whole
  module, pending the confirming free-fn-reference check — no policy arm references it).

---

## asinh / acosh / atanh, to_degrees / to_radians (noted, out of primary scope)
These delegate everywhere to inherent `*_strict_with` (and D18 widen / D38 `fixed_d38`).
No bespoke per-band kernels exist. They consolidate trivially via inherent extraction
into function-prefixed names (`asinh_series`, `acosh_series`, `atanh_series`,
`to_degrees_scale`, `to_radians_scale`) with an all-`_`→single-algorithm policy; flagged
here for completeness so the trig policy file stays exhaustive over the `TrigPolicy`
trait surface.

## Consolidation summary (the de-dup targets, one place)
Names below are the consolidated `<function>_<method>` fns; per-function `Algorithm`
enum variants keep the short scoped form (`Series`, `Tang`, `ExpIdentity`, `Atan`).
1. **`*_series`** — collapse `wide_kernel::*_strict_d*` (all 10 tiers × sin/cos/tan/atan)
   + the narrow-GUARD lookup slots into ONE generic-over-`N`/`SCALE` series core per
   function: `sin_series`, `cos_series`, `tan_series`, `atan_series`; GUARD =
   `const fn(SCALE)`. Width-bespoke D38 variants suffix `_int2`.
2. **`*_tang_with_taylor`** — collapse the Tang sincos slots (D57 44..=56, D153, D307,
   D462) into one generic `sincos` Tang core feeding `sin_tang_with_taylor` /
   `cos_tang_with_taylor` / `tan_tang_with_taylor`, and the Tang atan slots into
   `atan_tang_with_taylor`; share the `M`/`compute_table`/quadrant scaffold, keep the
   sincos vs atan tables distinct.
3. **`*_exp_identity`** — collapse the LIVE `lookup_*_hyper` slots + inherent shells into
   `sinh_exp_identity` / `cosh_exp_identity` / `tanh_exp_identity` over `exp` (Tang-exp
   hybrids `_with_tang` only if they bench distinct).
4. **inverse**: `asin_atan_with_sqrt` / `acos_atan_with_sqrt` / `atan2_atan` — extract
   the inherent `*_strict_with` compositions into named free fns.
5. **`tan_near_pole`** — keep as an intra-`tan`-algo value-dependent helper (NOT a
   matcher arm, NOT an `Algorithm` variant).
6. **`borrow_d57` / `widen_to_d38`** — move OUT of `algos/` into the policy layer as
   `borrow` / `widen` strategies (delete the DEAD sin/cos/tan borrow fns; keep the LIVE
   atan/asin/acos/atan2 borrow as a relocated strategy). No function prefix — not algos.

## Dead-to-drop master list (from catalog A + this analysis)
- `borrow_d57::{sin,cos,tan}_strict` (DEAD; atan/asin/acos/atan2 siblings LIVE → relocate).
- `lookup_d57_s18_22_sincos_tang` (whole module, DEAD).
- `lookup_d616_s300_315_hyper` (whole module, DEAD).
- `lookup_d1232_s610_620_hyper` (whole module, DEAD-pending-confirm; no policy arm refs it).
