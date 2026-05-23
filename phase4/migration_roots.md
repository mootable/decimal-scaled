<!--
SPDX-FileCopyrightText: 2026 John Moxley
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Phase-4 migration plan — ROOT functions (`sqrt`, `cbrt`, integer `isqrt`)

Read-only analysis + migration recipe (2026-05-23). No code changed by this doc.

This is the canonical exemplar migration doc for Phase 4. `sqrt` is worked first
and in full detail; `cbrt` and `isqrt` follow the **same section structure** and
future per-function docs (B/C) mirror the `sqrt` layout exactly.

**Sources cross-referenced:** `docs/ARCHITECTURE.md` ("Policy file structure"),
`research/2026_05_23_4.2_policy_matcher_design.md` (matcher + value-matcher),
`research/2026_05_23_phase4_algo_catalog.md` (dead list + duplication map),
`research/2026_05_22_algo_naming_standard.md` (paper-name scheme).

**Naming rules applied** (owner correction 2026-05-23):
- ALGORITHM fn/module name = `<function>_<method>[_with_<method2>][_variant]` —
  PREPEND the function performed: `sqrt_newton`, `cbrt_newton`, `isqrt_newton`,
  `sqrt_mg_divide`, never bare `newton`. Hybrids use `_with_` (e.g.
  `div_newton_with_mg`). `_variant` only where it genuinely distinguishes.
- Widths stay **const-generic params, never name-encoded**. Only a genuinely
  width-bespoke kernel may suffix, in the Int-width form (`_int2`), never `dXX`.
- The per-function `Algorithm` ENUM VARIANT keeps the **short scoped** method name
  (`Newton`, `MgDivide`) — it is namespaced by the function/file.
- VALUE-MATCHER fns = `<function>_<applicable_preconditions>` (e.g. `sqrt_N5_to_N10`).
- tables are matcher-selected data, no `sYYY` band encoding.

---

## Target shape recap (per ARCHITECTURE "Policy file structure")

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { /* paper-named, real algorithms ONLY, no Default */ }

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    ByValue(fn(&Int<N>) -> Algorithm),
}

const fn select<const N: usize>(/* + S for decimal */) -> Select<N> { match … }

pub fn f<const N: usize>(x: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(g)     => g(&x),
    };
    match algo { /* exhaustive over Algorithm — no _, no panic */ }
}
```

The decimal-layer matchers key on `(N, SCALE)`; the int-layer matcher keys on `N`.

---

# 1. `sqrt` (canonical exemplar)

## 1.1 Current kernels inventory

Dispatch entry: `D*<SCALE>::sqrt_strict_with(mode)` → `policy::sqrt::SqrtPolicy::sqrt_impl`
(per-width trait impls in `src/policy/sqrt.rs`) → one kernel in `src/algos/sqrt/*`.

| Kernel file / fn | What it does | Tier / scale-band served | Live? |
|---|---|---|---|
| `algos/sqrt/generic_wide.rs::sqrt<S,W>` | `isqrt(raw·10^SCALE)` in a wide work int `W` (next-up width), one round step; correct under all 6 modes | **width default** for D57…D1232; per-tier shims `sqrt_d57`…`sqrt_d1232` pick `W` (`Int<6>`,`Int<8>`,…`Int<128>`) | LIVE (the `_` arm of every wide tier) |
| `algos/sqrt/generic_wide.rs::sqrt_dNN` shims | thin per-tier wrappers fixing `(S,W)` and forwarding to `sqrt<S,W>` | one per wide tier | LIVE but mechanical — these are the tier-encoding to remove |
| `algos/sqrt/mg_divide_d38.rs::sqrt` / `sqrt_raw` | bridges `Int<2>` → `i128`, calls `mg_divide::sqrt_raw_with` (hand-tuned 256-bit isqrt over `i128`) | **D38 width override** | LIVE |
| `algos/sqrt/widen_to_d38.rs::sqrt_d18` | widen D18→D38 (same SCALE), call `mg_divide_d38::sqrt`, narrow back | **D18 width override** | LIVE |
| `algos/sqrt/lookup_d57_s20.rs::sqrt` | bespoke `(D57,20)` cell: radicand fits `Int<4>` not `Int<6>`; f64-seeded Newton via `policy::float_seed::isqrt` | **(D57, SCALE==20) scale-band override** | LIVE (named by both `base` and `std` arms of D57 triplet) |
| `algos/mg_divide.rs::sqrt_raw_with` / `sqrt_raw_correctly_rounded` | the actual D38 256-bit math: u128 fast path + `mul_u128_to_u256`/`isqrt_256` widening path; all 6 modes | D38 (via `mg_divide_d38`) | LIVE |
| `algos/mg_divide.rs::isqrt_256` | `floor(sqrt(N))` for 256-bit `N` (Newton, `div_long_256_by_128`) | D38 internal | LIVE |

Underlying integer isqrt the kernels lean on:
- `generic_wide` uses `W::isqrt()` (the `BigInt::isqrt` trait method → `Int<N>::isqrt`
  → `int/algos/div.rs::isqrt_mag_fixed` → native u64/u128 or `limbs_isqrt_u64`).
- `lookup_d57_s20` uses `policy::float_seed::isqrt::<Int<4>>` (std f64-seed / no_std fallback).
- `mg_divide_d38` uses its **own** `isqrt_256` (does not touch the int layer).

## 1.2 Consolidation verdict per kernel (Q4.1b)

| Kernel | Verdict | Justification |
|---|---|---|
| `generic_wide::sqrt<S,W>` | **COLLAPSE to ONE generic algorithm** → `sqrt_newton` | Already generic over `(S,W)`; the only per-tier variation is `W`. The 10 `sqrt_dNN` shims are pure tier-encoding and DROP (the matcher selects `W` from `N`). |
| `sqrt_dNN` shims (×10) | **DROP** (dead tier-encoding) | They carry no math; their job (pick `W`) becomes a matcher concern. |
| `mg_divide_d38::sqrt` + `mg_divide::sqrt_raw_with`/`isqrt_256` | **STAYS genuinely width-bespoke** → `sqrt_mg_divide` | i128/u128-specific 256-bit path with a hardware-`u128::isqrt` fast arm; strictly faster than widening `Int<2>→Int<4>` through the generic path. Width-bespoke for `N==2` (suffix `_int2` if a width tag is wanted). |
| `widen_to_d38::sqrt_d18` | **MOVE to policy layer** (not an algorithm) | It is a *dispatch strategy* ("delegate to the next wider tier"), per naming-standard §"Dispatch/cascade helpers are NOT algorithms". Becomes a matcher arm `N==1 → MgDivide` operating on the widened operand, OR a thin policy-level widen wrapper. Not a kernel in `algos/`. |
| `lookup_d57_s20::sqrt` | **KEEP as bespoke** → `sqrt_newton_with_table_seed` | Genuinely distinct: smaller work width (`Int<4>` vs `Int<6>`) for the `(D57,20)` cell + f64 seed. Could fold into `sqrt_newton` IF it learns to pick its work width and seed from a const policy of `(N,SCALE)` — see note below. Conservative verdict: keep it as a named variant for 0.5.0. |

**Merge-vs-keep note for `lookup_d57_s20`:** the bespoke kernel is the generic
Newton root with (a) a tighter work width and (b) the f64 seed. Both are
`(N,SCALE)`-const choices. The cleanest end state is a single `newton` whose work
width and seed source are const-selected; then `table_seed` disappears. For 0.5.0
the safer, lower-risk path is to keep `table_seed` as a distinct `Algorithm`
variant (golden gate covers the `(D57,20)` cell). Flag for owner: collapse-or-keep.

## 1.3 Proposed names (surviving algorithms)

Fn/module names (function-prefixed); enum variants in parentheses are the short
scoped form used inside `policy/sqrt.rs`.

- `sqrt_newton` (variant `Newton`) — Newton–Raphson integer square root over a
  const-selected work width (today's `generic_wide`). Generic over `N` and `SCALE`.
- `sqrt_mg_divide` (variant `MgDivide`) — Möller–Granlund-backed 256-bit isqrt for
  the `N==2` (D38) storage (today's `mg_divide_d38` + `mg_divide::sqrt_raw_with`/
  `isqrt_256`). Width-bespoke; if a width suffix is ever wanted, `sqrt_mg_divide_int2`.
- `sqrt_newton_with_table_seed` (variant `TableSeed`) — f64-seeded narrow-work
  bespoke for the `(D57,20)` cell (today's `lookup_d57_s20`); a Newton root with a
  table/float seed, hence the `_with_table_seed` hybrid form. _(May collapse into
  `sqrt_newton` — see 1.2 note.)_

`widen_to_d38` is **not** an algorithm name (moves to policy).

## 1.4 Target policy (`src/policy/sqrt.rs`)

Key = `(N, SCALE)` (decimal unary). `N` is the storage limb count
(`D18`→`Int<1>`/`N==1`, `D38`→`Int<2>`/`N==2`, `D57`→`Int<3>`/`N==3`, …, `D1232`→`N==64`).

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { Newton, MgDivide, TableSeed }   // real algorithms only

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    ByValue(fn(&Int<N>) -> Algorithm),            // unused for sqrt today (no value split)
}

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // D18 — widen to D38 storage and run MgDivide there (widen is a
        // policy-level operand transform; the *algorithm* is MgDivide).
        (1, _)  => Select::ByAlgorithm(Algorithm::MgDivide),
        // D38 — hand-tuned 256-bit isqrt.
        (2, _)  => Select::ByAlgorithm(Algorithm::MgDivide),
        // D57, SCALE == 20 — bespoke narrow-work + f64 seed.
        (3, 20) => Select::ByAlgorithm(Algorithm::TableSeed),
        // Everything else (all wide tiers, all other scales) — generic Newton.
        _       => Select::ByAlgorithm(Algorithm::Newton),
    }
}
```

- **`_`-arm default algorithm:** `Newton` (the universal generic-over-`(N,SCALE)` root).
- **Width/scale-range overrides:** `N==1` and `N==2` → `MgDivide`; `(N==3, SCALE==20)` → `TableSeed`.
- **`ByValue` value-matcher:** **NONE for sqrt** — the choice is fully determined by
  `(N,SCALE)`; no operand-value split exists today. (`Select::ByValue` stays in the
  enum for uniformity but is unused; the dispatch still folds.) No
  `sqrt_<preconditions>` matcher fn is needed.
- **Dispatch** is exhaustive over `Algorithm`, no `_`, no panic, and maps each
  short variant to its function-prefixed kernel:

  ```rust
  match algo {
      Algorithm::Newton    => sqrt_newton::<N, SCALE>(x),
      Algorithm::MgDivide  => sqrt_mg_divide(x /* widened to Int<2> for N==1 */, SCALE, mode),
      Algorithm::TableSeed => sqrt_newton_with_table_seed(x, mode),  // (D57,20)
  }
  ```

  `N==1`'s `MgDivide` arm runs against the operand widened to `Int<2>` (the policy
  applies the widen before calling the kernel — preserving the current
  `widen_to_d38` behaviour without an `algos/` helper).

**std/no_std tiering:** the f64 seed inside `TableSeed`/`Newton` already lives in
`policy::float_seed::{isqrt}` and is `cfg`-swapped there; the matcher itself is
`core`-only and identical in both configs (consistent with the 3-tier model — the
seed is the only `std` override and it is bit-identical).

## 1.5 Limb-cleanup touchpoints (#79)

`sqrt` reaches the limb layer only through the int-isqrt path:
- `sqrt_newton` → `BigInt::isqrt` → `Int<N>::isqrt` → `int/algos/div.rs::isqrt_mag_fixed`
  → `limbs_isqrt_u64` (for `N>=3`). **Destination: (iii) isqrt → root algos** — see
  the limbs doc; `limbs_isqrt_u64` folds into the int-layer `isqrt` algorithm.
- `sqrt_mg_divide` uses its own `isqrt_256` (self-contained in `algos/mg_divide.rs`,
  u128 arithmetic) — **does not touch `limbs/`**; stays put with the D38 kernel.
- The u128 `limbs_isqrt` (legacy family) is **not** on the sqrt path — it is dead-as-u128
  (catalog A) and DROPs with the u128 family.

## 1.6 Dead to drop (for sqrt)

From the catalog:
- The 10 `generic_wide::sqrt_dNN` tier shims (mechanical tier-encoding) — fold into the matcher.
- `widen_to_d38.rs` as an `algos/` module — relocate to policy (not a delete of behaviour, a relocation).
- u128 `limbs_isqrt` (legacy) — drops with the u128 family (#79), not a sqrt-specific kernel.

---

# 2. `cbrt`

## 2.1 Current kernels inventory

Dispatch: `D*<SCALE>::cbrt_strict_with` → `policy::cbrt::CbrtPolicy::cbrt_impl`
(`src/policy/cbrt.rs`, mirrors sqrt) → `src/algos/cbrt/*`.

| Kernel | What it does | Tier / band | Live? |
|---|---|---|---|
| `algos/cbrt/generic_wide.rs::cbrt<S,W>` | Newton cube root on `mag·10^(2·SCALE)` in `W`; inline f64-cbrt seed (top-64-bit bridge) gated on `FLOAT_SEED_AVAILABLE`; sign-preserving; all 6 modes | **width default** D57…D1232; shims `cbrt_d57`…`cbrt_d1232` with **double-bumped** `W` (`Int<12>`…`Int<256>`) | LIVE |
| `cbrt_dNN` shims (×10) | per-tier `(S,W)` fixers | one per wide tier | LIVE, mechanical |
| `algos/cbrt/mg_divide_d38.rs::cbrt`/`cbrt_raw` | `Int<2>`→`i128`, sign split, `mg_divide::cbrt_raw_with_signed` (hand-tuned 384-bit) | **D38 override** | LIVE |
| `algos/cbrt/widen_to_d38.rs::cbrt_d18` | widen D18→D38, cbrt, narrow | **D18 override** | LIVE |
| `algos/cbrt/lookup_d57_s20.rs::cbrt` | bespoke `(D57,20)`: work in `Int<6>` not `Int<12>`; f64 seed via `policy::float_seed::icbrt` | **(D57, SCALE==20)** | LIVE |
| `algos/mg_divide.rs::cbrt_raw_with_signed` / `icbrt_384` / `mul_u128_by_256` / `div_384_by_256` / `floor_div3` / `ge_384`/`gt_384`/… | the 384-bit cube-root math | D38 internal | LIVE |

**Seed asymmetry to note:** `generic_wide::cbrt` inlines its own f64 seed (does NOT
call `float_seed::icbrt`), while `lookup_d57_s20::cbrt` DOES call `float_seed::icbrt`.
Consolidating cbrt should route both through `float_seed::icbrt` so the kernel body
is cfg-free (matches the sqrt pattern).

## 2.2 Consolidation verdict per kernel

| Kernel | Verdict | Justification |
|---|---|---|
| `generic_wide::cbrt<S,W>` | **COLLAPSE to ONE generic** → `cbrt_newton` | Generic over `(S,W)`; only `W` varies. Fold the inline seed onto `float_seed::icbrt` while consolidating. |
| `cbrt_dNN` shims (×10) | **DROP** | Tier-encoding; `W` becomes a matcher concern. NOTE: cbrt's `W` is **double-bumped** (one width step beyond next-up because `10^(2·SCALE)`); the matcher's width selection must reproduce this per-tier mapping (it is a const fn of `N`). |
| `mg_divide_d38::cbrt` + `mg_divide::cbrt_raw_with_signed` + 384-bit helpers | **STAYS width-bespoke** → `cbrt_mg_divide` | u128-based 384-bit path tailored to `N==2` (suffix `_int2` if a width tag is wanted). |
| `widen_to_d38::cbrt_d18` | **MOVE to policy** | Dispatch strategy, not a method (same as sqrt). |
| `lookup_d57_s20::cbrt` | **KEEP bespoke** → `cbrt_newton_with_table_seed` | Narrower work width for `(D57,20)`; may collapse into `cbrt_newton` later (same caveat as sqrt). |

## 2.3 Proposed names

- `cbrt_newton` (variant `Newton`) — today's `generic_wide`.
- `cbrt_mg_divide` (variant `MgDivide`) — today's `mg_divide_d38` + the 384-bit
  `mg_divide` helpers.
- `cbrt_newton_with_table_seed` (variant `TableSeed`) — today's `lookup_d57_s20`.
- `widen_to_d38` → policy (not an algorithm).

## 2.4 Target policy (`src/policy/cbrt.rs`)

Identical shape and key `(N, SCALE)` to sqrt:

```rust
enum Algorithm { Newton, MgDivide, TableSeed }

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        (1, _)  => Select::ByAlgorithm(Algorithm::MgDivide),   // D18 widened to D38
        (2, _)  => Select::ByAlgorithm(Algorithm::MgDivide),   // D38
        (3, 20) => Select::ByAlgorithm(Algorithm::TableSeed),  // (D57,20) bespoke
        _       => Select::ByAlgorithm(Algorithm::Newton),     // default
    }
}
```

- **`_`-arm default:** `Newton`. **Overrides:** `N∈{1,2}`→`MgDivide`, `(3,20)`→`TableSeed`.
- **`ByValue`:** NONE — fully `(N,SCALE)`-determined. No matcher fn.
- The `Newton` arm's work-width selection (`N → W`, double-bumped) is a `const fn`
  helper in the policy/algorithm, not a runtime branch.

## 2.5 Limb-cleanup touchpoints (#79)

- `cbrt_newton` does its arithmetic via `BigInt` ops on `W` (`mul`, `div`, `pow`,
  `leading_zeros`, `<<`) → those route to `int/algos/{mul,div,shift}` → u64 limb
  primitives. No direct `limbs_*` call from the cbrt kernel.
- `cbrt_mg_divide` is self-contained u128 math in `algos/mg_divide.rs` — no `limbs/` reach.
- The cbrt Newton loop's `n / (x*x)` divide rides the int-layer divmod (Knuth/BZ);
  see the limbs doc (ii) division engines.

## 2.6 Dead to drop (for cbrt)

- 10 `cbrt_dNN` shims — fold into matcher.
- `cbrt/widen_to_d38.rs` as `algos/` — relocate to policy.
- (No cbrt-specific entries in catalog A; cbrt has no DEAD kernels of its own.)

---

# 3. integer `isqrt` (int layer)

## 3.1 Current kernels inventory

This is the **int-layer** `isqrt` (key = `N` only), distinct from the decimal sqrt above.

| Item | What it does | Width served | Live? |
|---|---|---|---|
| `Int<N>::isqrt` (`int/types/mod.rs:370`, Uint) | allocates `out`, calls `isqrt_mag_fixed::<N>` | all N | LIVE (public + used by `BigInt::isqrt`) |
| `Int<N>::isqrt` (`int/types/mod.rs:1566`, signed) | `floor(sqrt(|self|))` via the unsigned sibling | all N | LIVE |
| `BigInt::isqrt` (`int/types/traits.rs:338`) | trait method, the kernel-facing surface used by `generic_wide` | all N | LIVE |
| `int/algos/div.rs::isqrt_mag_fixed::<N>` | const-`N` fast-arm: `N==1`→`u64::isqrt`, `N==2`→`u128::isqrt`, `N>=3`→`limbs_isqrt_u64` | all N | LIVE (the real dispatcher) |
| `limbs/mod.rs::limbs_isqrt_u64` (2477) | Newton + hardware-f64 seed over u64 limbs | N>=3 | LIVE |
| `limbs/mod.rs::limbs_isqrt` (1006, **u128**) | legacy u128 Newton isqrt | — | DEAD-as-u128 (catalog A) |
| `algos/mg_divide.rs::isqrt_256` | D38 256-bit isqrt (decimal-sqrt's `MgDivide`) | — | LIVE but belongs to D38 sqrt, not the int `isqrt` surface |
| `algos/fixed_d38.rs::isqrt_u512` (815) | part of the dead D38 "Fixed" kernel | — | dead/legacy (`fixed_d38` is being removed) |

`isqrt_mag_fixed` is **already** the matcher in spirit: a const-`N` ladder. Phase 4
formalises it into the `Algorithm`-enum shape.

## 3.2 Consolidation verdict per item

| Item | Verdict | Justification |
|---|---|---|
| `isqrt_mag_fixed::<N>` ladder | **BECOMES the matcher** (3 arms) | Already a const-`N` dead-arm-eliminated ladder. Reshape into `Algorithm` + `select`. |
| `N==1` arm (`u64::isqrt`) | **STAYS width-bespoke** → `isqrt_native` | Hardware single-instruction path. |
| `N==2` arm (`u128::isqrt`) | **STAYS width-bespoke** → `isqrt_native` (same fn, const-split inside) | Compiler 128-bit path. |
| `N>=3` arm (`limbs_isqrt_u64`) | **COLLAPSE to ONE generic** → `isqrt_newton` | Width-agnostic Newton-with-f64-seed over u64 limbs; one impl for all `N>=3`. |
| u128 `limbs_isqrt` | **DROP** | Dead-as-u128 (#79). |
| `isqrt_256` / `isqrt_u512` | **not part of int `isqrt`** | `isqrt_256` stays with D38 `mg_divide`; `isqrt_u512` drops with `fixed_d38`. |

## 3.3 Proposed names

- `isqrt_native` (variant `Native`) — the hardware `u64::isqrt`/`u128::isqrt` arms
  (`N∈{1,2}`), const-split inside the one fn.
- `isqrt_newton` (variant `Newton`) — Newton iteration with hardware-f64 seed over
  u64 limbs (`N>=3`, today's `limbs_isqrt_u64`).

(`isqrt_native` is a defensible name for "use the primitive intrinsic"; an
alternative is to treat `N∈{1,2}` as just early `Newton` arms. Owner pick —
`isqrt_native` reads clearer as a distinct path.)

## 3.4 Target policy (`src/int/policy/isqrt.rs`, new)

Key = `N` (int unary):

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { Native, Newton }

const fn select<const N: usize>() -> Select<N> {
    match N {
        1 | 2 => Select::ByAlgorithm(Algorithm::Native),  // hardware u64/u128 isqrt
        _     => Select::ByAlgorithm(Algorithm::Newton),  // limb Newton + f64 seed
    }
}

pub fn isqrt<const N: usize>(x: Uint<N>) -> Uint<N> {
    let algo = match const { select::<N>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f)     => f(&x),
    };
    match algo {
        Algorithm::Native => isqrt_native::<N>(x),   // N==1/N==2 const-split inside
        Algorithm::Newton => isqrt_newton::<N>(x),   // = today's limbs_isqrt_u64
    }
}
```

- **`_`-arm default:** `Newton`. **Override:** `N∈{1,2}`→`Native`.
- **`ByValue`:** NONE.
- `Int<N>::isqrt` / `Uint<N>::isqrt` / `BigInt::isqrt` become thin shells over
  `int::policy::isqrt::isqrt`.

## 3.5 Limb-cleanup touchpoints (#79)

- `isqrt_newton` arm = today's `limbs_isqrt_u64` — **destination (iii): isqrt moves
  into the int-layer `isqrt` algorithm** (`int/algos/`), out of `limbs/mod.rs`. The
  u64 isqrt is the body of `isqrt_newton`.
- u128 `limbs_isqrt` DROPs with the u128 family.

## 3.6 Dead to drop (for int isqrt)

- u128 `limbs_isqrt` (legacy family).
- `fixed_d38::isqrt_u512` (drops with the whole `fixed_d38` "Fixed" kernel removal).

---

## Cross-function summary

- **`sqrt`** → `Algorithm { Newton, MgDivide, TableSeed }`, key `(N,SCALE)`: default
  `Newton`; `N∈{1,2}`→`MgDivide` (D18 widened); `(3,20)`→`TableSeed`. No value matcher.
- **`cbrt`** → identical shape to sqrt (same three variants, same arms). Note the
  double-bumped work width for `Newton` is a const fn of `N`.
- **int `isqrt`** → `Algorithm { Native, Newton }`, key `N`: default `Newton`;
  `N∈{1,2}`→`Native`. No value matcher.
- **Headline consolidation:** the ~20 per-tier `sqrt_dNN`/`cbrt_dNN` shims collapse
  into `sqrt_newton`/`cbrt_newton` (generic over const `N`); D38 keeps its bespoke
  `sqrt_mg_divide`/`cbrt_mg_divide` (256/384-bit u128); the `(D57,20)` cells keep
  `*_newton_with_table_seed` (candidate to later fold into `*_newton`);
  `widen_to_d38` leaves `algos/` for the policy layer.
- **#79 link:** the only root touchpoint into `limbs/` is the int `isqrt` Newton arm
  (`limbs_isqrt_u64`), which relocates into `int/algos/` as `isqrt_newton`; the u128
  `limbs_isqrt` is dead and drops.
