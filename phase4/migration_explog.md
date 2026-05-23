<!--
SPDX-FileCopyrightText: 2026 John Moxley
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Phase-4 migration — EXP/LOG family (read-only plan, 2026-05-23)

Scope: `exp`, `exp2`, `ln`, `log` (variable base), `log2`, `log10`, `powf`.
Companion to `2026_05_23_4.2_policy_matcher_design.md` (matcher shape) +
`2026_05_22_algo_naming_standard.md` (paper-names, no `dXXX`/`sYYY`) +
`2026_05_23_phase4_algo_catalog.md` (live/dead inventory). Mirrors the section
shape the `roots` doc uses. This records the TARGET state for the implementing
agents; it changes no code.

## Naming decisions taken in this doc (apply across the family)

**Naming rule:** algorithm fn name = `<function>_<method>[_with_<method2>][_variant]`.
The function is PREPENDED, so the same method reads `exp_series`/`ln_series`,
`exp_tang`/`ln_tang`. Hybrids use `_with_` (`powf_exp_with_ln`). Widths stay
const-generic params (never name-encoded); a genuinely width-bespoke kernel uses the
Int-width form `_int2` (never `dXX`) — none in this family qualify. The per-function
`Algorithm` **enum variant** keeps the short scoped method name (`Series`, `Tang`,
`ExpWithLn`, `IntSquareMultiply`); only the dispatched fn name carries the function
prefix.

The two surviving methods (per function, prefixed):

- **`series`** (`exp_series` / `ln_series`) — the direct fixed-point kernel:
  range-reduce + Taylor, with the Brent–Salamin / Smith `r/2^n`
  argument-halving-and-squaring trick. This is what `fixed_d38::exp_fixed`/`ln_fixed`
  AND `wide_kernel::*_strict_<tier>` both run — the **same method at different
  widths** (the naming-standard predicted this; confirmed by reading both). Collapse
  to one `exp_series` / `ln_series`.
  - **narrow-guard regime is NOT a distinct method:** `exp/lookup_d57_s18_22.rs`
    (the non-`_tang` one) is `series` run with a smaller `GUARD` (`GUARD_NARROW = 8`)
    for a low-SCALE band. Resolves the naming-standard's open Q "what distinguishes
    `lookup_d57_s18_22` from `_tang`": the non-tang is `series` with a narrow guard;
    the `_tang` is `tang`. The guard is a `const fn` of `SCALE` inside `series`, so
    the narrow band is just a const-folded guard value, not a separate algorithm.
- **`tang`** (`exp_tang` / `ln_tang`) — Tang (1989, ACM TOMS 16(4)) table-driven
  two-stage range reduction: `e^v = 2^k · e^(c_j) · e^δ` with a precomputed `c_j`
  table (`M` entries), short Taylor on the tiny residual `δ`. All
  `exp/ln::lookup_dXXX_sYYY_tang` kernels are this one method; `M` and the table are
  **matcher-selected data keyed on SCALE**, not part of the name. One generic
  `exp_tang` / `ln_tang` over `Int<N>`/work-width + `SCALE`.

Dispatch strategies (NOT algorithms, move OUT of `algos/` to the policy layer):

- **`borrow_wider`** — the D38→D57 widen→kernel→narrow strategy (`*/borrow_d57.rs`),
  per the naming-standard. (All exp/ln/pow `borrow_d57` entry points are already
  DEAD — see per-function sections — so for THIS family `borrow_wider` has no
  surviving callee and the files DROP entirely.)
- **`widen_to_work`** — the D18→D38 widen cascade (`*/widen_to_d38.rs`). Moves to the
  policy layer (the `impl_*_widen!` macro already lives in policy; the
  `algos/.../widen_to_d38.rs` free fns are thin shims that fold into it).

`exp2`/`log2`/`log10`/`log` are **derived** functions, not separate kernels: they
compose `series`/`tang` with a constant divide/multiply (`ln2`, `ln10`) or the
base-2 reduction. They get policy arms but reuse the `exp`/`ln` algorithm enum —
no new `Algorithm` variants for them.

---

## `exp` (natural exponential)

### Current kernels inventory
| kernel | what it does | tier / scale-band | live? |
|---|---|---|---|
| `exp/fixed_d38::exp_fixed` (+ `exp_strict`/`exp_with`) | `Fixed` 256-bit Taylor + Smith `r/2^n` halving-and-squaring | D18 (via widen), D38 | LIVE |
| `exp/widen_to_d38::exp_strict_d18`/`exp_with_d18` | D18 widen→`fixed_d38`→narrow | D18 | LIVE (strategy shim) |
| `exp/wide_kernel::exp_strict_d57.._d1232` (10 per-tier fns) | per-tier `wide_trig_<tier>::exp_fixed` (= `series` at the tier width); default wide-tier emission | D57..D1232, all SCALE not in a bespoke band | LIVE |
| `exp/lookup_d57_s18_22.rs::exp_strict` (non-tang) | `series` with `GUARD_NARROW=8` | D57, SCALE 18..=22 | DEAD (catalog §A — policy uses the `_tang` sibling) |
| `exp/lookup_d57_s18_22_tang::exp_strict` | `tang`, `M=128` | D57, SCALE 18..=22 | LIVE |
| `exp/lookup_d57_s45_56::exp_strict` | `tang`, `M=512` | D57, SCALE 45..=56 | LIVE |
| `exp/lookup_d115_s57_tang::exp_strict` | `tang` | D115, SCALE 50..=60 | LIVE |
| `exp/lookup_d153_s70_82_tang::exp_strict` | `tang` | D153, SCALE 70..=82 | LIVE |
| `exp/lookup_d307_s140_160_tang::exp_strict` | `tang` exp | D307, 140..=160 | DEAD exp (catalog §A — policy D307 uses `wide_kernel`); `tang_exp_fixed`+`GUARD_FOR_HYPER` in the file are LIVE for trig hyper — file STAYS, only its `exp_strict` drops |
| `exp/lookup_d462_s225_235_tang::exp_strict` | `tang` exp | D462, 225..=235 | DEAD (catalog — Tang lost ~75%; kept behind `cfg(test)` as a lab probe) |
| `exp/lookup_d616_s300_315_tang::exp_strict` | `tang` exp | D616, 300..=315 | DEAD exp (catalog §A — break-even); `tang_exp_fixed`/`GUARD_FOR_HYPER` only transitively dead (consumer d616 hyper is dead) |
| `exp/lookup_d1232_s610_620_tang.rs` | `tang` exp | D1232, 610..=620 | present in `mod.rs`; no `exp` policy arm wires it → DEAD as exp |
| `exp/borrow_d57::exp_strict`/`exp2_strict` | D38→D57 widen-and-back | D38 | DEAD (catalog §A) |

### Consolidation verdict (Q4.1b, per-kernel)
- `fixed_d38::exp_fixed` + `wide_kernel::exp_strict_*` (×10) → **collapse to ONE
  generic `exp_series<S, W, const SCALE>`**. Same method, different width; per-tier
  `GUARD` and `r/2^n` depth become `const fn` of `SCALE`. (Blocker noted in
  `wide_kernel` docs: the per-tier `wide_trig_<tier>` core is macro-emitted with a
  tier-specific `W`; collapsing needs the work-integer to lift to a generic `W`
  param or a `WideTrigCore` trait. This is the 4.1 genericisation lift — record as
  the prerequisite, not a blocker on the matcher.)
- `lookup_*_s*_tang::exp_strict` (the LIVE ones: D57×2, D115, D153) → **collapse to
  ONE generic `exp_tang<S, W, const SCALE>`**; `M` + the `c_j` table are
  matcher/SCALE-selected data.
- `lookup_d57_s18_22.rs` (non-tang) → **DROP** (dead; was `exp_series`-narrow-guard,
  superseded by the `_tang` sibling). The narrow-guard idea survives as a const
  guard inside `exp_series`, not a file.
- `lookup_d307/d462/d616/d1232 *_tang::exp_strict` → **DROP the `exp_strict`** (dead).
  d307/d616 FILES stay (their `tang_exp_fixed`/`GUARD_FOR_HYPER` feed trig); d462
  stays behind `cfg(test)` as the documented lab probe; d1232 `exp_strict` drops.
- `borrow_d57::exp*` → **DROP** (dead).
- `widen_to_d38` → **move to policy** (`widen_to_work` strategy).

### Proposed surviving algorithms
`exp_series`, `exp_tang` (fn names). Both generic over storage `S`, work-width `W`,
`const SCALE`. Enum variants: `Series`, `Tang`.

### Target policy (`policy/exp.rs`, key `(N, SCALE)`)
```
enum Algorithm { Series, Tang }            // variants short-scoped; fns are exp_series / exp_tang

const fn select<const N, const SCALE> -> Algorithm:
  // narrow tiers widen to the D38 work width then run Series (handled by the
  // widen_to_work strategy in policy; the algorithm chosen there is Series):
  N==1 (D18)            => Series          // via widen_to_work to D38 width
  N==2 (D38)            => Series
  N==3 (D57) & 18..=22  => Tang
  N==3 (D57) & 45..=56  => Tang
  N==6 (D115) & 50..=60 => Tang
  N==8 (D153) & 70..=82 => Tang
  _                     => Series          // every other (N,SCALE): wide series default
```
- No `ByValue` matcher for `exp`. Only one runtime fast-path exists today (`raw==0`
  short-circuit to `ONE`) — that is a value-dependent STEP inside the algo, stays
  in the algo body (trichotomy), NOT a matcher arm.
- `Default`-free: the `_`-arm names `Series`, the real wide default.
- `std` tier == `non_std` (catalog: only std machinery is the wide-kernel constant
  cache, invisible to policy) → `std` const-falls-through to `non_std` everywhere.

### Dead to drop (`exp`)
`borrow_d57.rs` (whole file); `lookup_d57_s18_22.rs` (non-tang, whole file);
`lookup_d1232_s610_620_tang::exp_strict`; the `exp_strict` of
`lookup_d307_s140_160_tang` / `lookup_d462_s225_235_tang` / `lookup_d616_s300_315_tang`
(retain the files' `tang_exp_fixed`/`GUARD_FOR_HYPER` where the trig hyper consumer
is live — d307; d462 file → `cfg(test)` probe; d616 transitively dead with its
consumer).

---

## `exp2` (base-2 exponential)

### Current kernels inventory
| kernel | what it does | tier | live? |
|---|---|---|---|
| `exp/fixed_d38::exp2_strict`/`exp2_with` | base-2 reduction → `exp_fixed` (`series`) | D38 | LIVE |
| D18 `exp2_impl` | widen→D38 `exp2_strict_with`→narrow (inline in policy macro) | D18 | LIVE (strategy) |
| wide tiers `exp2_impl` | delegate to inherent `exp2_strict_with` shell (macro core) | D57..D1232 | LIVE |
| `exp/borrow_d57::exp2_strict` | D38→D57 widen-and-back | D38 | DEAD (catalog §A) |

### Consolidation verdict
`exp2` is **derived from `exp_series`** (base-2 range reduction then the same Taylor
core). No own kernel survives — `exp2` reuses the `exp` `Algorithm::Series`/`Tang`
enum with a base-2 reduction const. The wide-tier `exp2_impl` currently shortcuts
to the inherent `exp2_strict_with` shell rather than a policy free-fn; under the new
model it routes through the same `(N,SCALE)` select as `exp`.

### Proposed surviving algorithms
None new — reuses `exp_series` (and `exp_tang` where the band kernels grow a base-2
entry). Enum variants `Series` / `Tang` shared with `exp`.

### Target policy (`policy/exp.rs`, shares the `exp` enum)
`select` identical to `exp`'s, with the base-2 reduction applied before the kernel.
Narrow tiers: `widen_to_work` → D38 `Series`. Wide tiers: `Series` default. No
`ByValue`.

### Dead to drop (`exp2`)
`borrow_d57::exp2_strict`.

---

## `ln` (natural logarithm)

### Current kernels inventory
| kernel | what it does | tier / scale-band | live? |
|---|---|---|---|
| `ln/fixed_d38::ln_fixed` (+ `ln_strict`/`ln_with`) | `Fixed` 256-bit ln (= `ln_series`) | D18 (via widen), D38 | LIVE |
| `ln/widen_to_d38::ln_strict_d18`/`ln_with_d18` | D18 widen→`fixed_d38`→narrow | D18 | LIVE (strategy shim) |
| `ln/wide_kernel::ln_strict_d57.._d1232` (10 fns) | per-tier `wide_trig_<tier>::ln_fixed` (= `ln_series` at tier width); wide default | D57..D1232 default | LIVE |
| `ln/lookup_d57_s18_22.rs` (non-tang) | `ln_series` narrow-guard | D57, 18..=22 | DEAD (catalog §A) |
| `ln/lookup_d57_s18_22_tang::ln_strict` | `tang`-ln | D57, 18..=22 | LIVE |
| `ln/lookup_d115_s57_tang` | `tang`-ln | D115, 50..=60 | LIVE |
| `ln/lookup_d153_s70_82_tang` | `tang`-ln | D153, 70..=82 | LIVE |
| `ln/lookup_d230_s110_120_tang` | `tang`-ln | D230, 110..=120 | LIVE |
| `ln/lookup_d307_s140_160_tang` | `tang`-ln | D307, 140..=160 | LIVE |
| `ln/lookup_d307_s285_295_tang` | `tang`-ln | D307, 285..=295 | LIVE |
| `ln/lookup_d462_s225_235_tang` | `tang`-ln | D462, 225..=235 | LIVE |
| `ln/lookup_d616_s300_315_tang` | `tang`-ln | D616, 300..=315 | LIVE |
| `ln/lookup_d616_s585_595_tang` | `tang`-ln | D616, 585..=595 | LIVE |
| `ln/lookup_d924_s455_465_tang` | `tang`-ln | D924, 455..=465 | LIVE |
| `ln/lookup_d924_s895_905_tang` | `tang`-ln | D924, 895..=905 | LIVE |
| `ln/lookup_d1232_s610_620_tang` | `tang`-ln | D1232, 610..=620 | LIVE |
| `ln/lookup_d1232_s1195_1205_tang` | `tang`-ln | D1232, 1195..=1205 | LIVE |
| `ln/borrow_d57::ln/log2/log10/log_strict` | D38→D57 widen-and-back | D38 | DEAD (catalog §A) |

### Consolidation verdict (Q4.1b, per-kernel)
- `fixed_d38::ln_fixed` + `wide_kernel::ln_strict_*` (×10) → **ONE generic
  `ln_series<S,W,SCALE>`** (same width-collapse + the macro-`W` blocker as `exp`).
- All `lookup_*_tang::ln_strict` (13 LIVE) → **ONE generic `ln_tang<S,W,SCALE>`**;
  `M`/`c_j` table = SCALE-selected data. Note `ln` has TWO bands per wide tier at
  the upper widths (mid + deep, e.g. D307 140..160 and 285..295) — these are the
  SAME `ln_tang`, two SCALE ranges, two table selections; no per-band kernel.
- `lookup_d57_s18_22.rs` (non-tang) → **DROP** (dead).
- `borrow_d57` → **DROP** (dead, all four entry points).
- `widen_to_d38` → **move to policy** (`widen_to_work`).

### Proposed surviving algorithms
`ln_series`, `ln_tang` (fns). Enum variants: `Series`, `Tang`.

### Target policy (`policy/ln.rs`, key `(N, SCALE)`)
```
enum Algorithm { Series, Tang }            // variants short-scoped; fns are ln_series / ln_tang

select<const N, const SCALE>:
  N==1 (D18)               => Series       // via widen_to_work to D38
  N==2 (D38)               => Series
  N==3  (D57)   & 18..=22   => Tang
  N==6  (D115)  & 50..=60   => Tang
  N==8  (D153)  & 70..=82   => Tang
  N==12 (D230)  & 110..=120 => Tang
  N==16 (D307)  & 140..=160 => Tang
  N==16 (D307)  & 285..=295 => Tang
  N==24 (D462)  & 225..=235 => Tang
  N==32 (D616)  & 300..=315 => Tang
  N==32 (D616)  & 585..=595 => Tang
  N==48 (D924)  & 455..=465 => Tang
  N==48 (D924)  & 895..=905 => Tang
  N==64 (D1232) & 610..=620 => Tang
  N==64 (D1232) & 1195..=1205 => Tang
  _                         => Series      // wide series default
```
- No `ByValue`. (`raw<=0` panics — a guard, not an algo choice; stays in algo body.)
- `_`-arm = `Series`. `std` == `non_std` (constant-cache only) → const fall-through.

### Dead to drop (`ln`)
`borrow_d57.rs` (whole file); `lookup_d57_s18_22.rs` (non-tang, whole file).

---

## `log` (variable base), `log2`, `log10`

### Current kernels inventory
| kernel | what it does | tier | live? |
|---|---|---|---|
| `ln/fixed_d38::log_strict`/`log2_strict`/`log10_strict` (+`_with`) | `ln(self)/ln(base)`, `ln/ln2`, `ln/ln10` on `Fixed` | D18 (widen), D38 | LIVE |
| D18 `log*_impl` | widen→D38→narrow (policy macro) | D18 | LIVE (strategy) |
| wide tiers `log*_impl` | delegate to inherent `log*_strict_with` shells | D57..D1232 | LIVE |
| `ln/borrow_d57::log2/log10/log_strict` | D38→D57 widen-and-back | D38 | DEAD (catalog §A) |

### Consolidation verdict
`log`/`log2`/`log10` are **derived from `ln`** — a ratio (`log`) or a constant
divide (`log2` → `÷ln2`, `log10` → `÷ln10`). No own kernel survives; they reuse the
`ln` `Algorithm` enum and the same `(N,SCALE)` select, composing `ln` with a
constant. The wide-tier `*_impl` shortcuts (currently delegating to the inherent
`*_strict_with` shells) route through the `ln` select under the new model.

### Proposed surviving algorithms
None new — reuse `ln_series`/`ln_tang` via `ln`.

### Target policy
Live in `policy/ln.rs` alongside `ln` (same enum + same `select`); `log`'s `base`
operand makes `log` a binary-ish surface but the algorithm choice is still keyed on
`(N, SCALE)` of the argument — base is reduced to the same width/scale (no second
key needed; base shares `(N,SCALE)`). No `ByValue`.

### Dead to drop
`borrow_d57::log/log2/log10_strict`.

---

## `powf` (floating-point power)

### Current kernels inventory
| kernel | what it does | tier / scale-band | live? |
|---|---|---|---|
| `pow/fixed_d38::powf_strict`/`powf_with` | `exp(y·ln x)` entirely on the `Fixed` 256-bit guard intermediate | D18 (via widen), D38 | LIVE |
| `pow/widen_to_d38::powf_strict_d18`/`powf_with_d18` | D18 widen→`fixed_d38`→narrow | D18 | LIVE (strategy shim) |
| `pow/borrow_d57::powf_strict`/`powf_with` | D38→D57 widen-and-back | D38 | DEAD (catalog §A) |
| inherent `powf_strict_with` (wide_transcendental macro shell) | composes wide-tier `*_series` exp∘ln; small-int-exponent fast path `powf_exp_as_small_int` (`|n|<=64`) | D57..D1232 | LIVE (not policy-routed) |

### Consolidation verdict (Q4.1b, per-kernel)
- `fixed_d38::powf_*` → keep as the narrow-tier realisation of the hybrid
  algorithm `powf = exp(y·ln x)`; it is `exp`∘`ln` over the `Fixed` work width.
  Rename off-tier to the hybrid `_with_` form — **`powf_exp_with_ln`** (the
  established `b^y = exp(y ln b)` identity; not a separate transcendental method, a
  composition of `exp` and `ln`). Width becomes a const-generic param; the narrow
  `Fixed`-256 path is just `powf_exp_with_ln` at the D38 work width.
- wide-tier inherent `powf_strict_with` → same `powf_exp_with_ln` (composes the
  wide-tier `*_series`). **0.5.0 gap to record:** wide `powf` is NOT policy-routed
  today (lives in the `decl_wide_transcendental!` shell); migrating it into
  `policy/pow.rs` mirrors the deferral noted on ln/exp and is the bulk of pow's
  Phase-4 lift.
- `borrow_d57::powf_*` → **DROP** (dead).
- `widen_to_d38` → **move to policy** (`widen_to_work`).

### Proposed surviving algorithms
`powf_exp_with_ln` (fn; enum variant `ExpWithLn`) — one hybrid algorithm; it
internally calls the `exp`/`ln` algorithms, in-level composition allowed by the
layering rule. Optional second algorithm `pow_int_square_multiply` (variant
`IntSquareMultiply`) — see the value matcher below.

### Target policy (`policy/pow.rs`, key `(N, SCALE)`)
```
enum Algorithm { ExpWithLn }               // fn powf_exp_with_ln; single algorithm today

select<const N, const SCALE>:
  _ => ExpWithLn                           // all (N,SCALE); narrow via widen_to_work
```
- **`ByValue` candidate (the one in this family):** the small-integer-exponent
  fast path `powf_exp_as_small_int` (`|n| <= 64` → repeated squaring instead of
  `exp∘ln`) is a value decision *between methods* (integer-power vs compose). Per
  the trichotomy this is a genuine **value matcher**: add `Algorithm::IntSquareMultiply`
  (fn `pow_int_square_multiply` — the binary square-and-multiply exponentiation
  method) and a `Select::ByValue` arm that inspects the exponent operand. Naming per
  the value-matcher rule `<function_name>_<applicable_preconditions>`:
  `powf_exp_small_int` (the precondition is "exponent is a small integer", not a
  width band, so the suffix names that precondition, not `N`/`S`). Recommend landing
  it as the family's sole `ByValue` arm; if deferred, `ExpWithLn` alone is a
  complete, total `select`.
- `_`-arm = `ExpWithLn`. `std` == `non_std`.

### Dead to drop (`powf`)
`pow/borrow_d57.rs` (whole file).

---

## Cross-family summary

- **Surviving algorithms (`<function>_<method>`, generic over `S`/`W`/`SCALE`):**
  `exp_series`/`ln_series` and `exp_tang`/`ln_tang` (exp2/log/log2/log10 derive from
  them; enum variants `Series`/`Tang`), and `powf_exp_with_ln` (variant `ExpWithLn`)
  with an optional `pow_int_square_multiply` (variant `IntSquareMultiply`)
  value-matcher branch.
- **Prerequisite 4.1 lift (shared blocker):** the `*_series`/`*_tang` width-collapse
  needs the per-tier macro-emitted `wide_trig_<tier>` core (work-integer `W`,
  constant tables) to lift to a generic `W` param or a `WideTrigCore` trait —
  flagged in `wide_kernel` module docs; this is the genericisation prerequisite,
  not a matcher blocker.
- **`borrow_*` files (exp/ln/pow) all DROP** — every entry point dead; `borrow_wider`
  as a policy strategy has no surviving callee in this family (trig inverses keep
  theirs, out of scope here).
- **`widen_to_d38` (×3) → policy `widen_to_work` strategy**, not `algos/`.
- **No `compile_error!`/`Default` arms** — each `select` `_`-arm names a real wide
  default (`Series` / `ExpWithLn`); totality is structural.
- **Tang `_tang`/non-`_tang` orphan pairs:** only `exp/lookup_d57_s18_22{,_tang}`
  and `ln/lookup_d57_s18_22{,_tang}` exist as pairs; in both the non-`_tang`
  (`*_series`-narrow-guard) is the DEAD orphan → drop; the `_tang` is LIVE → folds
  into generic `exp_tang` / `ln_tang`.
