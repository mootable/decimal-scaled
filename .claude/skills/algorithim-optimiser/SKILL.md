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

The type method then delegates one line: `pub fn isqrt(self) -> Self { isqrt_dispatch(self) }`.

### Policy usage & scope — make it as wide as it's correct for

- The whole point of the matcher is **one generic algorithm serving a RANGE** of the key. Prefer *widening* a generic algorithm's applicability over adding narrow per-tier bespoke kernels (the 6.2 collapse: 60 per-tier wrappers → 6 generic `*_series`). Add a bespoke/specialised kernel **only where a microbench shows it actually wins**; otherwise route the cell to the generic one in the `_` arm.
- `select` must be **total over the full key** — every `N` / `(N,M)` / `(N,SCALE)` / `(N,M,S1,S2)` cell resolves to a *real* algorithm. The `_` arm names the widest-applicable default. No gaps, no `unreachable!` in the algorithm choice.
- Use the **right key width** — key on the full axis your algorithm's correctness/perf depends on, no less: int unary = `N`; int binary = `(N,M)`; decimal unary = `(N,SCALE)`; decimal binary = `(N,M,S1,S2)` with an `S1==S2` const fast-path. If a kernel's validity depends on `SCALE`, the key MUST include `SCALE`.
- A new algorithm's **precondition should be as wide as it is correct for**: if `<fn>_newton` is valid for all `N`, name it `<fn>_newton` (no precondition suffix) and let the `_` arm use it everywhere; only pin a `<precond>` suffix (`_2limb`, `_s44_56`) when the kernel is genuinely valid for *just* that limb/scale band.
- The type method calls dispatch with the full key (`isqrt_dispatch(self)`, `dispatch::<N, SCALE>(…)`) and **never branches on the tier itself** — tier/scale selection is the matcher's job, not the caller's.

---

## 3. Support libraries — where shared leaves live

Three buckets. **Support libraries are SINGLE files of leaves, NOT split per function.**

- **`src/algos/support/`** — decimal-side cross-family leaves: `fixed.rs` (the 256-bit sign-magnitude `Fixed` work-int type), `mg_divide.rs`, `newton_reciprocal.rs`, `seed_bridge.rs` (typed-`W` seed bridge over the seed leaf), `table_cache.rs` (`decl_table_cache!` per-thread memo), `wide_trig_core.rs` (the `WideTrigCore` trait + 6 generic `*_series`).
- **`src/int/algos/support/`** — int-side leaves: `limbs.rs` (the `add_assign`/`mul_schoolbook`/`shr`/`cmp`/`bit_len`/… limb primitives).
- **`src/algo_x_support/`** — TRULY cross-tier-INDEPENDENT leaves. The hard invariant: a leaf here **calls NOTHING in-crate** — primitives + std-gated `f64` only. The **seed library** (`seed.rs`: `sqrt_seed`/`cbrt_seed`/`extract_top_u64`) lives here. One API per seed, with the std/no_std divergence encapsulated **inside** the leaf (internal `#[cfg]`), so consuming algorithms stay cfg-agnostic. Don't put anything here that imports another crate module.

If a leaf could serve both tiers but touches in-crate code, it belongs in `algos/support` or `int/algos/support`, not `algo_x_support`.

---

## 4. Microbench two candidates at the dispatch seam

The dispatch `Select` seam is the "choose + swap + **microbench**" point. Use the 8.1 harness.

- Support module: **`benches/support/ab_microbench.rs`**. Worked example: **`benches/micro/mul_kernel_ab.rs`**.
- API:
  ```rust
  Candidate::new("label", |input| run(input))      // two fns of identical signature
  micro_criterion()                                 // Criterion preset: sample 20, warm 150ms, measure 400ms — sub-60s
  ab_compare(c, "group", |inp| label_string, inputs, candidate_a, candidate_b)  // black_box-guards in AND out
  ab_sweep!(c => Int<16> => |c| ab_compare(...), Int<32> => |c| ab_compare(...))  // the "two types" axis
  ```
- It `black_box`-guards inputs and outputs (defeat const-fold/DCE — critical here, the dispatch is *designed* to const-fold) and prints `A/B verdict [group]: <winner> beats <loser> by N.NNx`.
- Add a `[[bench]]` entry in `Cargo.toml`: `name`, `path = "benches/<folder>/<name>.rs"`, `harness = false`, and the `required-features` the example needs.

**Discipline:** validate a perf change with a **focused <60s microbench FIRST**, before the multi-hour sweeps. Run microbenches **locally**; run the full sweeps (`library_comparison`, `full_matrix`) on **GHA** (`bench-full` / `bench-history` / `bench-branch-compare` workflows) — never burn the owner's machine on a full sweep. picosecond `change:` deltas are noise; multi-hour sweep cells run 1.5–2× slower than a cold-machine microbench.

`benches/` is organised into folders: `support/` (shared incl. `ab_microbench`), `libcmp/`, `full_matrix/`, `agm/`, `backends/`, `lookup/`, `micro/`. Keep `[[bench]] name=` stable when moving files (workflows call `--bench <name>`); only `path=` changes.

---

## 5. The dec ↔ int difference (and the cross-tier rule)

**Int algorithms** (`int/algos/<fn>/`): operate on `&[u64]` limb slices or `Int<N>`/`Uint<N>`; use the limb leaves in `int/algos/support/limbs.rs` and the optimised divide via `int::policy::div_rem::dispatch` (Knuth) — NOT the const `div_rem_mag_fixed` shift-subtract path. Policy keyed on `N`. **Audit for infinite loops:** an int algorithm must never use an operator/method (`+`, `*`, `.div_rem()`) that re-dispatches back into the same algorithm.

**Decimal algorithms** (`algos/<fn>/`): generic over storage `S` + work width `W` (both `: BigInt`); the policy binds the concrete `(S,W)` per tier. They must **dispatch DOWN to the `Int<N>` layer** for their integer work — via the operator overloads (and **check the overload exists** for the op you need) or by calling the int method/kernel directly:
- `W::isqrt()`, `W::icbrt()` (the floor roots — routes to the int policy + `div_rem_dispatch`),
- `raw.resize_to::<W>()`, `W::TEN.pow(scale)`, `w.div_rem(d)`, `w.bit_length()`.
- This cross-tier use is **§1a-allowed and required** — it is NOT the inversion. The inversion is decimal→decimal (calling a decimal `*_strict_with` method on your own value, which re-enters a decimal policy). The `hypot` lesson: it used to call `one_plus_sq.sqrt_strict_with(mode)` (inversion + double-rounding error); fixed to form `a²+b²` in `W` and take `W::isqrt` — the root now dispatches to int and is correctly-rounded.

**std / no_std float policy:** `std` owns floats (inherent `f64` intrinsics — `sqrt`/`cbrt`/`sin`/…); `no_std` is integer-only, **NEVER `libm` or any external math crate**. A seed that wants a float gets a `std` (f64) and a `no_std` (pure-integer) variant encapsulated in the seed leaf (`algo_x_support::seed`), so the algorithm stays agnostic. The `fast` feature (f64 bridge) implies `std`.

---

## 6. Verification gates (run before declaring done)

- `cargo check --lib` (default features) **and** `cargo check --features wide,x-wide,xx-wide --lib` — both clean.
- **Quick validation (agents, during development):** `cargo test --features wide,x-wide,xx-wide --test micro_golden` — `tests/micro_golden.rs`, a fast curated subset of the golden oracle (~0.5s, same harness + same zero-tolerance `lsbe == 0` as the full golden). Today it covers 6 functions (sqrt/cbrt/exp/ln/sin/atan) × 3 tiers (D18/D57/D307) × the first ~20 oracle rows × all six RoundingModes. Run it freely while iterating. If you add or touch a function/tier the subset doesn't exercise, **extend the curated set in `micro_golden.rs`** so your path is covered — but it stays a *subset*; never let it grow into a second full golden.
- **Full golden** (behaviour, the 0.5-ULP guarantee — **coordinator / CI / commit gate, ~285s**): `cargo test --features wide,x-wide,xx-wide --test ulp_strict_golden` → `264 passed`. Agents should NOT run the full golden (it's slow + the coordinator/commit owns it) — use `micro_golden` instead; the coordinator runs the full one before merge/push. NOTE: golden does **not** cover `hypot` — add a targeted accuracy test with externally-validated values (mpmath / Pythagorean exacts) for anything golden misses.
- `cargo doc --no-deps --features wide,x-wide,xx-wide,macros --workspace` with `RUSTDOCFLAGS="-D warnings"` — the `docs (gate)` CI; broken/private intra-doc links are hard errors.
- `cargo build --all-targets` (default) must build — gate any wide-tier-using example/test with `required-features` in `Cargo.toml` so the narrow default build skips it.
- Layering: grep your algorithm fn for a call to a dispatched method on its own operand type → must be ZERO.

---

## 7. Writing a golden / accuracy test (best practice)

The 0.5-ULP guarantee is only real if every correctly-rounded op is gated by a zero-tolerance oracle test.

- **Regression capture — pin the INPUT VALUE that exposed the bug.** Whenever you find a wrong/inaccurate result, the fix is NOT complete until the **specific input value** that produced it (the exact operand(s), at that width + scale) is added to the **holistic golden suite** so it can never silently regress. Add THAT input — with its correctly-rounded expected value taken from the EXTERNAL oracle, never from the now-fixed code — to the function's `tests/golden/<fn>_d<N>_s<S>.txt` table (regenerate via `scripts/gen_golden_precision.py` / mpmath so `ulp_strict_golden` covers it), or, if the op isn't oracle-backed, to its `tests/<fn>_accuracy.rs`. The discovery of a bug is the trigger to widen the suite with that input, every time. (The `hypot` lesson: the +8–13 ULP error survived precisely because no holistic test pinned the inputs that exposed it.)
- **External oracle, never self-validation.** Expected values come from an EXTERNAL high-precision source — the mpmath golden tables `tests/golden/<fn>_d<N>_s<S>.txt` (generated by `scripts/gen_golden_precision.py`), or for a one-off, a value computed offline with mpmath / Python `decimal` and baked in **with a source-citing comment**. NEVER assert against the crate's own output (circular).
- **Zero tolerance: `lsbe == 0`** (delta == 0 storage LSB) — bit-exact at the last place. No `diff <= 1` slack on a correctly-rounded path; that slack is exactly what let the old `hypot` ship 8–13 ULP off undetected.
- **All six RoundingModes, every width.** The harness folds the oracle's `(floor_raw, cls)` to the correctly-rounded integer for ANY mode; assert across all six modes and every supported width. Split the test per width so local iteration stays fast.
- **Cover the matcher arm boundaries.** Add explicit oracle cases at each `Select` arm's range edges — a precision cliff exactly at an arm boundary must not slip through.
- **If the oracle doesn't cover the op, add a targeted accuracy test** (the `hypot` lesson — it isn't in the golden function set): externally-correct values — exact cases where maths gives an integer (Pythagorean triples for hypot) plus mpmath-computed non-perfect cases — asserting exact (`delta == 0`) across modes/scales/widths, in its own `tests/<fn>_accuracy.rs`.
- **Every test asserts; no silent no-ops.** Name tests by the behaviour they validate. Replace any runtime `if !cond { return; }` early-out with a module-level `#[cfg(...)]` gate so a test can never pass without asserting.
- **New function/width → regenerate the oracle** with `scripts/gen_golden_precision.py` (mpmath) and commit `tests/golden/<fn>_d<N>_s<S>.txt`; the full `ulp_strict_golden` picks it up.
- **While iterating, run `micro_golden`** (the fast curated subset) — not the full golden. The coordinator/CI runs the full `ulp_strict_golden` before merge/push.

---

## 8. Quick checklist for a new/optimised algorithm

1. Kernel file `algos/<fn>/<fn>_<variation>[_<precond>].rs`, body calls kernels/leaves only (no self-dispatch).
2. Register it (`<fn>/mod.rs`, `algos/mod.rs`).
3. Add an `Algorithm` variant (1:1 name) + wire it into `select`/`dispatch` (const, exhaustive).
4. Decimal: dispatch the integer work down to `Int<N>` (operators or direct). Int: no self-re-dispatch loop.
5. Microbench the old vs new candidate at the seam (`ab_microbench`, <60s, local). Promote the winner in `select`.
6. Gates: check (default+wide), golden 264, doc -D warnings, default --all-targets, layering grep.
