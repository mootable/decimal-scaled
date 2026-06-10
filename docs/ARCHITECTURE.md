# Architecture

This page is a map of how `decimal-scaled` is put together: the two
layers (integer **backends** and decimal **front-ends**), how a method
call is routed to one specific algorithm with **zero runtime dispatch**,
how unused algorithm variants are **pruned** at compile time, and how the
correctness and performance guarantees are **enforced by testing**.

## The model in one sentence

Every value is a plain integer that encodes `real_value × 10^SCALE`. All
core arithmetic is integer arithmetic, so results are **bit-identical on
every platform**, and the transcendental functions are computed with
integer-only kernels that are **correctly rounded** — within 0.5 ULP of
the true real value at the type's last representable place.

## Two absolute invariants: no heap, no state

These two rules sit **above** everything else on this page. They are not
performance preferences or style choices — they are the load-bearing
guarantees the whole design rests on, and no benchmark, no convenience, and
no algorithm is permitted to break them.

1. **No heap — the runtime path is pure stack.** No `Vec`, `Box`, `Rc`,
   `Arc`, `alloc::*`, or any heap allocation anywhere a value is computed.
   Every working buffer is an inline `[u64; …]` / `[u128; …]` on the stack,
   sized at compile time (the `ComputeLimbs` associated-type buffers on the
   `Limbs<N>` carrier and const-init `static` data are how a width wider than
   `N` is carried — see *Work-width scratch* below). This is what lets the crate run in `no_std`
   with no allocator and keeps every call's cost on the stack where it
   const-folds. A heap allocation on the compute path is a **hard defect**,
   never a tolerated one. (The earlier heap exceptions — a `decl_table_cache!`
   `Vec` tables macro and the `newton_reciprocal` `thread_local!`+`Vec`
   cache — were removed; `_wide-support = []` is now a heap-free marker.
   Do not reintroduce any of them.)

2. **No state — thread-safety comes from being stateless, so there is NO
   cache whatsoever.** Every function is a pure function of its inputs and
   recomputes on each call. No `thread_local!`, no `static mut`, no
   interior-mutability cache slot (`RefCell`/`Cell`/`Mutex`/`OnceCell`-as-
   cache/atomics-as-cache), no memoization across calls — anywhere on the
   runtime path. The crate is `Send`/`Sync`/re-entrant **for free** because no
   call carries mutable state between invocations; introducing a cache would
   trade that guarantee for a speed-up we refuse to make.

   **The line is MUTABILITY, not whether something is global.** An immutable
   `static` is ALLOWED — it is the *sanctioned* form for compile-time-baked
   read-only data (e.g. the per-scale constant tables), and reading it at a
   *runtime* index is a lookup, not a recompute and not state. A `static mut`,
   or a `static` made mutable through interior mutability, is a singleton and
   is BANNED. Concretely:

   - ✅ `static FOO: [u64; N] = …` / `const FOO: … = …` — immutable,
     compile-time, read-only data (the per-scale constant + Tang tables).
   - ❌ `static mut FOO` — a mutable singleton.
   - ❌ any `static` whose contents change at run time — `OnceCell` /
     `OnceLock` / `Mutex` / `RwLock` / `RefCell` / `Cell` / atomics used as a
     cache or accumulator (interior mutability as a singleton / memo).
   - ❌ heap on the compute path — `Vec` / `Box` / `Rc` / `Arc` / `alloc::*`.

   **The dividing line is WHEN the value is produced, not whether it is
   stored.** *Compile-time* precomputed data is fine and encouraged: a
   `const` / `const fn` / `static` table baked into the binary by the
   compiler is immutable read-only data — it is computed once at build time,
   never written at run time, and shared with no synchronisation. What is
   forbidden is a value computed/populated **at run time on first use and
   kept for later calls** — that is memoization. That is what a
   `NewtonReciprocal` precompute cache or the `pi`/`ln2`/`ln10`/`pow10`
   `thread_local!` caches WOULD do; they were removed and must not return.
   The fix for such a site is either (a) lift the
   precompute to *compile time* (a `const`/`const fn` table) where the value
   is fixed, or (b) **recompute it each call on the stack**; it is **never**
   to relocate the runtime cache into a mutable `static`.

   **Where a runtime computation genuinely IS needed, minimise it to the cases
   that need it — const-fold the common path, never recompute on the hot path,
   and never blanket-remove the runtime path when a rare case legitimately
   needs it.** Many values are const-derivable for the *common* case yet
   runtime-varying in a *rare* one. Example: `pi`/`ln2`/`ln10` are fixed at the
   common working scale `w = SCALE + GUARD` — a compile-time `const` per
   `(width, SCALE)` const-folded off the propagated `SCALE` — but a rare
   directed-rounding / Ziv guard escalation genuinely needs them at a higher,
   runtime-varying scale. The rule: **bake the common case** (the hot path then
   never recomputes — recalculating `pi` on every call is the anti-pattern) and
   **keep the runtime recompute ONLY for the rare case that provably cannot be
   const.** A runtime input such as a working scale `w` may stay a runtime value
   where it genuinely varies — the goal is not to delete the runtime parameter
   but to stop *deriving the const-derivable value from it on the hot path*. The
   bar (enforce it in review): a runtime recomputation is retained only at sites
   where a `SCALE`/`GUARD`-derived `const` provably cannot stand in — and only
   there.

Together these mean the answer to "make this faster with a cache/buffer
pool" is always **no**: the speed comes from a better generic kernel,
const-folding, and exact-per-`N` stack scratch — never from heap or state.

## Two layers, same shape

The crate is **two layers that mirror each other** — a decimal layer on
top of an integer layer. Each has the *same three tiers*: a typed
surface, a **const-folded width dispatch**, and an **algorithm library**.
A decimal kernel expresses its math in integer operations and never
reaches into limb internals directly.

![decimal-scaled layer architecture: the decimal layer (front-ends → dispatch → kernels) composed on the integer layer (backends → dispatch → algorithms → limb primitives), each layer carrying its own width dispatch and algorithm library](figures/architecture/layers.svg)

The key point the older sketch hid: **the integer layer is not just
primitives — it has its own dispatch policy and algorithm library**,
mirroring the decimal layer. A `BigInt` method (`mul`, `div`,
`root_int`, …) routes on the compile-time width / limb count to one
matched algorithm — schoolbook below the Karatsuba threshold and the
non-allocating Karatsuba above it, Möller–Granlund divide at the narrow
tiers and Burnikel–Ziegler at the wide ones — and the compiler prunes
the rest, exactly as the decimal policy keys on `(width, SCALE)`. Both
layers compile to a single direct call per monomorphisation.

## A call through the layers

`D57<20>::sqrt_strict()` traverses **both layers' dispatch + kernels**:
the front-end dispatches on `(width, SCALE)` to one decimal algorithm, which
calls the integer layer — itself dispatching on width to a matched
algorithm down to the limb primitives — and hands back a
correctly-rounded raw value.

```mermaid
sequenceDiagram
  autonumber
  participant U as caller
  participant FE as D57 front-end
  participant DP as decimal policy
  participant DK as sqrt algorithm
  participant IP as int policy
  participant IK as int algorithm
  participant L as limb primitives
  U->>FE: sqrt_strict()
  FE->>DP: dispatch (width 192, SCALE 20)
  DP->>DK: const select → matched algorithm
  DK->>IP: root_int / isqrt on Int<3> (BigInt)
  IP->>IK: const-folded → width-matched isqrt
  IK->>L: limb ops on [u64; 3]
  L-->>IK: limbs
  IK-->>DK: integer root + residual
  DK-->>FE: correctly-rounded raw
  FE-->>U: D57 value
```

## Integer backends

The storage under a decimal type is an integer wide enough to hold
`10^MAX_SCALE`:

| Decimal tier | Storage |
|---|---|
| D18 / D38 | `Int<1>` / `Int<2>` |
| D57 … D1232 | `Int<3>` … `Int<64>` |

Every tier is built on a single const-generic pair —
`Uint<const N: usize>` and `Int<const N: usize>` — stored as `[u64; N]`
little-endian limbs (`N` = number of 64-bit limbs; bit width is `N·64`).
Choosing the **limb count** as the one type parameter sidesteps the
`LIMBS = ⌈BITS/64⌉` derivation that a bits-parameterised type cannot
express on stable Rust.

The arithmetic itself lives once, as **reusable width-matched limb
algorithms** (add/sub/mul/div/shift/compare, plus `sqr`, `cube`,
`root_int`, `isqrt`, …) operating over `&[u64]` slices. Because `N` is a
compile-time constant, the limb loops unroll per width and there is no
runtime length to carry. A `BigInt` trait exposes this surface with the
**same method names** as the decimal arithmetic trait, so the two layers
read as one vocabulary.

`src/`:

```
int/                const-generic integer layer
  types/            Int<N>/Uint<N>; the BigInt trait; the Limbs<N> scratch
                    carrier + ComputeLimbs (compute_limbs.rs)
  policy/           per-width / limb-count algorithm-selection dispatch
  algos/            reusable width-matched algorithms
    support/limbs.rs  raw slice limb primitives
```

## Decimal front-ends

Each width is a `Dxx<const SCALE: u32>` newtype around its storage
integer. The number in the name is `MAX_SCALE` (the largest `SCALE` the
storage can hold); `SCALE` is a const-generic so `D38<2>` (cents) and
`D38<18>` are distinct, zero-overhead types. Because a value has exactly
one representation at a fixed scale, `Eq`/`Ord`/`Hash` are derived
straight from the storage bits.

The cross-width API is four trait families (`src/types/traits/`):

- `DecimalArithmetic` — operators, sign, integer methods, the
  checked/wrapping/saturating/overflowing families, reductions.
- `DecimalConvert` — round-trip, integer and float bridges.
- `DecimalTranscendental` — `sqrt`/`cbrt`/`exp`/`ln`/trig/hyperbolic/`pow`.
- `DecimalConstants` — the per-type math constants (`pi`, `e`, … ).

`Decimal` is the marker supertrait combining the four. Two more traits
sit alongside them: `WidthLE` (the compile-time width-ordering relation
used by the `widen` / `narrow` hops) and, under the `dyn` feature,
`DynDecimal` (the object-safe erased view).

The typed method shells (`D57::<20>::sqrt_strict_with(mode)`) are emitted
by macros in `src/macros/` and immediately hand off to the dispatch layer.

## Work-width scratch — exact `ComputeLimbs` on the `Limbs<N>` carrier, never build-max

Many algorithms compute in a width *wider* than the value's own `N` u64
limbs: a multiply's `2N` product, a `sqrt` radicand (`2N`), a `cbrt`
radicand (`4N`), the `÷10^w` magnitude (`⌈N/2⌉` u128). Stable Rust cannot
name `[u64; 2N]` from a generic `N`, so the width lives on an
**associated-type buffer**. That buffer is **NOT** a capability of the value
integer; it is carried by a separate **zero-sized limb carrier**,
`Limbs<const N>` — the `Limb` / `Limbs<N>` / `ComputeLimbs` triad:

- **`Limb`** — the scalar element axis (`u64` / `u128`); a width-generic
  kernel's per-element type (see *Limb width* below).
- **`Limbs<N>`** — the zero-sized *sizing marker* (never instantiated), one
  per `N`-u64-limb width. It is the type the `ComputeLimbs` buffers hang off.
- **`ComputeLimbs`** — the trait carrying the per-`N` buffer associated types
  + constructors (`src/int/types/compute_limbs.rs`), implemented for
  `Limbs<N>` (NOT for `Int<N>`). It has **no supertrait** — a pure sizing
  capability, deliberately not `: BigInt`.

The value integer names its carrier through the **`BigInt::Scratch`
associated type** (`type Scratch = Limbs<N>` in `impl BigInt for Int<N>`,
`src/int/types/traits.rs`) — *the sanctioned new surface*: the value integer
merely *names* its scratch carrier, it does not *carry* the scratch. This
**severs the old `ComputeInt: BigInt` supertrait cycle**: when scratch was a
capability bounded by `ComputeInt: BigInt`, a blanket `BigInt` method (or
`Int<N>` operator) could not require `ComputeInt` without a cycle. Now
`ComputeLimbs` does not require `BigInt`, and `BigInt` only *names* its
carrier, so:

- a concrete-`N` kernel bounds on `Limbs<N>: ComputeLimbs`, calls
  `Limbs::<N>::double_buffered_u64()`, and gets an exactly-sized stack buffer
  that folds away per monomorphisation;
- a kernel generic over a *value* integer `W: BigInt` reaches scratch as
  `W::Scratch::single_u128()` under a `where W::Scratch: ComputeLimbs` bound,
  with no scratch bound on `W` itself.

`BigInt::Scratch` is declared **unbounded** (`type Scratch;`, no
`: ComputeLimbs`). `BigInt` is a blanket `impl<const N>` for every `Int<N>`,
but in the `exact-scratch` build `ComputeLimbs` is implemented only at the
listed widths; a `type Scratch: ComputeLimbs` bound would be unprovable for
an arbitrary blanket `N`. The `ComputeLimbs` requirement is therefore
discharged at the generic-helper **use sites** (`where W::Scratch:
ComputeLimbs`), where `W` is always a concrete width that has it. (`Limbs<N>`
is declared `pub` only because it is named in the `pub` `BigInt::Scratch`
type — E0446 forbids leaking a more-private type — but it lives in the
private `int` module, so it is not crate-public API, exactly like the `pub
struct Int<N>` value type beside it.)

**The scratch vocabulary is fixed clean limb-multiples — two families.** A
buffer is named by a *multiple of `N`*, never by the function that wants it
(no `LimbBufDivU128`-style per-function type). Two families, each available
in both `u64` and `u128` element form:

- **Plain `X·N` multiples** — `single` (`N`), `double` (`2N`), `quad`
  (`4N`). The exact value/product width.
- **Buffered variants** (a fixed `⌈N/2⌉` of headroom for normalisation /
  carry / packing) — `single_buffered` (`N + 2`), `double_buffered`
  (`2N + ⌈N/2⌉` ≈ 2.5N), `quad_buffered` (`4N + ⌈N/2⌉` ≈ 4.5N).

**Adding a higher multiple is expected, not exceptional** — an algorithm
that needs, say, an `8N` buffer adds one size axis the same way (a literal
in each build-form impl, an associated type per element, and the `Limb`
forwarder). There is nothing privileged about 1×/2×/4×.

**Why these are methods, not a const expression: stable Rust, no nightly.**
The clean way to write `[u64; 2N + ⌈N/2⌉]` from a generic `N` would be
`generic_const_exprs` — a nightly feature. Rather than pin the crate to
nightly, each concrete-`N` impl simply *states the exact computed size for
that width* (the per-`N` macro emits `[u64; 2 * $n + ($n + 1) / 2]`, etc.).
So the method **is** the const-expression workaround: it hands back the
exact maximum for that one width, computed at the impl, with no nightly
feature and no signature-level const. (If `generic_const_exprs` ever
stabilises, these collapse to inline array sizes — the method surface is the
stable-Rust stand-in for that, not a permanent design preference.)

**The algorithm sources its own exact scratch.** A kernel that needs a
wider width takes `where Limbs<N>: ComputeLimbs` and calls the *normal*
per-`N` method (`Limbs::<N>::double_buffered_u64()`, …); a *value*-generic
helper takes `where W::Scratch: ComputeLimbs` and calls
`W::Scratch::single_u128()`. It must **not** pass a work width as a
const-generic argument (`fn f<W, const LW: usize>` where `LW == W::U128_LIMBS`
is a defect — that const-work-width parameter is exactly the wall
`ComputeLimbs` exists to remove), and it must **not** reach for the build-max
blanket.

**The build-max blanket is the fallback of last resort — NOT for
algorithms.** `MAX_SINGLE_LIMBS` / `MAX_DOUBLE_LIMBS` / `MAX_QUADRUPLE_LIMBS`
/ `MAX_U128_LIMB` (and the `max_*_limbs()` constructors), feature-gated via
`MAX_WORK_N`, are sized to the widest tier the build enables. They exist for
the few paths that **structurally cannot** carry a concrete `N` on stable —
the **width-erased** ones, blanket over *every* `N`:

- the blanket `Int<N>` `/` / `%` operators and the width-erased blanket
  `BigInt` methods (`resize_to`). The `BigInt::Scratch` bridge no longer
  *cycles* (a blanket `BigInt` method *could* now name `Self::Scratch::*`
  under `where Self::Scratch: ComputeLimbs`) — but `resize_to` is invoked on
  a *generic* receiver `Int<N>` (the `raw.resize_to::<Int<N>>()` policy
  bridges), and a generic-`N` caller cannot discharge `Limbs<N>: ComputeLimbs`
  in the per-width `exact-scratch` build, so the bound would cascade
  unboundedly. These stay build-max.
- the **width-erased slice-divide engines** (`div_knuth`,
  `div_burnikel_ziegler_with_knuth`, `div_knuth_u128_limb`) and
  `int_fmt::fmt_into(&[u64])` — they take a bare `&[u64]` of *runtime* length
  by design; there is no `N` to size against.

The bare operators are also cold — decimal ops route through the
`ComputeLimbs` kernels, not the `Int<N>` operator — so the build-max blanket
is a *shrinking* fallback whose target is zero.

Paths that *look* like exceptions but are **not** — a concrete `N` or const
`SCALE` is in scope, so they must use `ComputeLimbs`:

- `Display` / radix formatting (`int_fmt`): `N` is the monomorphised width at
  `impl<const N> Display for Int<N>`. (The byte-output buffer it hands the
  width-erased `fmt_into(&[u64])` legitimately stays build-max; only its
  *limb* scratch, if any, would be exact.)
- `newton_reciprocal`: its reciprocal/pow buffer lengths are functions of the
  work width *and* the divide exponent, and the exponent derives from the
  const `SCALE` that the decimal policy dispatch carries (keyed on
  `(const N, const SCALE)`, the channel that should thread it). Both axes are
  const-provided, so these buffers are a const-sizing target — today frozen
  `MAX_*_U64` literals fed a runtime `scale: u32`, a Class-B defect to size
  down per `(width, SCALE)` threaded from dispatch.
- `widen_mul`, the Newton-root `seed_bridge`, every algorithm kernel:
  concrete `N` → `Limbs<N>: ComputeLimbs`; value-generic `W` →
  `W::Scratch: ComputeLimbs`.

Everywhere a concrete `N` is in scope — every algorithm kernel, every
decimal policy and decimal kernel — **use the normal `ComputeLimbs` methods,
never a `MAX_*` variant.** A `MAX_*` / `max_*_limbs()` use on a concrete-`N`
path is the build-max blanket leaking onto a tier that can size itself
exactly: that is the cross-tier size pollution the Constitution (rule 6)
forbids, and it is a defect to be migrated to `Limbs::<N>::single_u64()` /
`double_buffered_u64()` / `quad_buffered_u64()` / `single_u128()`.

## Const generics — the BigRule: pass the level's OWN, never any other

A recurring pollution: a policy or function that **invents a const generic**
beyond the ones that define its level. The rule is sharp and exhaustive.

**The policy entry point (`dispatch`) SHOULD take exactly the level's defining
const(s) — and NO OTHERS:**

| level | `dispatch` consts |
|-------|-------------------|
| int unary | `<const N>` |
| int binary | `<const Nthis, const Nother>` |
| decimal unary | `<const N, const S>` (width **and** scale) |
| decimal binary | `<const Nthis, const Sthis, const Nother, const Sother>` |

These are free to take because **the caller already holds them in its types**
— inferred at the call site, never computed (`D<Int<N>, SCALE>::sqrt` already
knows `N` and `SCALE`, so `dispatch::<N, SCALE>(…)` costs it nothing).

**These same consts propagate freely across the WHOLE layer — every policy AND
every algorithm, not just `dispatch`:** the **decimal** layer may carry
**`IntWidth` (`N`) and `SCALE`**; the **int** layer may carry **`IntWidth`
(`N`) only** (int has no scale axis). Any policy or algorithm in the layer is
free to take its layer's consts (they ride down from the type for nothing); the
rule is only that it may take **no more** than those — no derived width, no
new const knob.

> **Decimal binary is genuinely four consts.** Binary decimal ops are meant to
> work across *different* widths AND scales (`D<Int<Nthis>, Sthis> op
> D<Int<Nother>, Sother>`), so `<Nthis, Sthis, Nother, Sother>` is the real
> target — in scope, not a far-future reservation. Where the *current*
> implementation is still same-type (`D<Int<N>, S> op D<Int<N>, S>`), the four
> consts collapse to the dec-unary `<N, S>` and that is what those dispatchers
> take today; they move to the full four-const form as cross-scale/width
> binary lands. (A binary op that *needs* both operands to be the same type is
> the transitional state, not the design.)

- **No other const value is allowed.** No work-width const, no derived const,
  no `const LW`, no `{2*N}` / `{<_>::U128_LIMBS}` const-generic argument. A
  const that *encodes a width derived from another* (`fn f<W, const LW>` where
  `LW == W::U128_LIMBS`) is the canonical defect — wider-than-`N` widths come
  from `ComputeLimbs` (previous section), never a const param. That is the
  const-work-width wall the `ComputeLimbs` associated types exist to remove.
- **Inward is optional — and propagating the level's OWN const inward to
  enable const-folding is a WIN, not pollution.** Threading these consts
  *inward* (`select`, `select_for_limbs`, the kernels) is optional — a helper
  takes a const only if it uses it. Taking the level const at the `dispatch`
  entry but not inward is normal, never a defect. And the converse is equally
  fine: `SCALE`/`N` ride down from `D<Int<N>, SCALE>` for **nothing** (the type
  already carries them), so threading them into a kernel so a working-scale
  value **const-folds** — e.g. baking `pi`/`ln2`/`ln10` at `SCALE + GUARD` as a
  per-`(W, SCALE)` compile-time const, eliminating a per-call scale-down divide
  — is encouraged; the kernel simply monomorphises per consts it was always
  entitled to. **The line is: const-fold off the level's OWN `SCALE`/`N`;
  never INVENT a new const item or parameter the type doesn't already give
  you.** A *new* const knob — a derived width (`const LW`, above), or any const
  added solely to carry a value into a method — is the defect, because it bloats
  the signatures; propagating an existing level const is free.

**The caller guardrail + the metadata test.** A caller must be able to invoke
a policy with *exactly what it already has*. If calling forces the caller to
**manufacture a const, create a type, or specialise**, the *policy signature
is wrong* — not the caller.

The **metadata test** for `dispatch`: it may **freely inspect the values
passed to it** — limb lengths, magnitude, sign, any property of the operands
— to choose an algorithm. That *is* the job of a `ByValue` / `ByShape`
matcher, and it is never a violation (the divide stripping its own slices to
`den_n` is exactly this). What `dispatch` must **not** need is **external
metadata**: any const, type, or context *beyond* its level consts and the
operands themselves. If a function cannot be picked or run without extra
information threaded in from outside, the design is broken.

### Const entry + slice entry — one length/shape classifier, two doors

Most policies key on the level's own const width(s) and **const-fold to one
direct call** (the BigRule above) — that compile-away is the whole point, so it
is the **default and must be preserved for every caller that holds the const.**
But some operands have *independent runtime lengths that no const expresses*,
and the policy must still pick the best algorithm for them. The rule:

> **A policy keys its algorithm choice on a single `select_for_*(lengths/shape)`
> classifier, and may expose TWO doors over it: a const-`N` `dispatch<N>` that
> const-folds `select_for_*(N, …)` away (the hot path), and a slice
> `dispatch_slice(&[u64], …)` that runs the SAME classifier at run time — but it
> grows the slice door ONLY where a genuine runtime-slice caller needs it.** An
> `Int<N>` is just a typed `[u64; N]`, so the classifier keys on *length*, a
> property both forms share; the two doors differ only in WHEN it is evaluated.
> Never route a const-holding caller through the slice door (that throws away the
> const-fold), and never force a slice caller to manufacture a const it does not
> have (the caller-side specialisation the guardrail forbids).

**Division needs ONLY the slice door.** It looks like the int-binary
`<const Nthis, const Nother>` case but cannot be — its operands have
*independent* runtime lengths:

- the decimal `/` divides a **`2N`-limb** scaled numerator by an **`N`-limb**
  divisor — two different widths, neither the caller's single `N`;
- the slice roots (`isqrt_newton`, `icbrt_newton`, `newton_reciprocal`,
  `div_rem_mag_slice`) divide **bare `&[u64]` of runtime length** — no `N` in
  their types at all.

So `int::policy::div_rem` is **slice-only** (`dispatch(num, den, quot, rem)`),
with the shape decision in the runtime `select_for_limbs(num, den)` — its engine
choice and any limb-width refinement (e.g. a u128-limb engine for an even, wide
divisor) is a **runtime arm** there, gated on the runtime `den_n`, because that
is where the width information actually is.

**Multiply needs BOTH doors.** At its const callers (decimal mul, the
wide-transcendental work-muls — the perf-critical path) the operands *are*
width-`N`, so `int::policy::mul` keeps its const-`N` `dispatch<N>` that folds to
one call. But it ALSO has genuine slice callers — the decimal slice roots
(`cbrt_newton`, the sqrt roots) and the rescale product path hold runtime-length
`&[u64]` and no `N` — so mul gains a `dispatch_slice` over the **same**
`select_for_lengths` classifier (the length matcher already lives inside its
`ByShape(|a_len, b_len|)`), and those callers route through it instead of
reaching past the matcher to a hardcoded kernel. Exact scratch on the slice door
is **caller-supplied** (the `div_knuth_into(…, u_buf, v_buf)` pattern — the
caller usually still holds a const `N` for *sizing* even when its operand slices
are runtime-length), or build-max for a truly width-erased caller. `mul_low`,
whose operands are always width-`N`, needs only the const door. **The slice door
is added per-caller-need, never pre-emptively.**

## Limb width — the matcher's second axis (`u64` / `u128`)

A wide-tier kernel can run faster in **u128 limbs** (half the limbs and carries) than in
u64. Which limb width wins is a per-`(N, SCALE)` property, so it is a **second matcher axis**
alongside the algorithm: the policy `Select` verdict carries `(Algorithm, LimbSize)`, where
`LimbSize` (`U64` / `U128`, defined in `compute_limbs.rs`) is the *const* part of the verdict
(limb width is value-independent — never decided inside a `ByValue` closure). `dispatch`
const-folds the verdict and runs the kernel at the chosen width.

The width is delivered **by type, not by name**: a `Limb` trait (impl'd for `u64` and
`u128`, carrying the scalar primitives) parameterises ONE generic kernel
`fn k<const N: usize, L: Limb>(…) where Limbs<N>: ComputeLimbs`, dispatched by a const-folded
`match limbsize { U64 => k::<N, u64>(…), U128 => k::<N, u128>(…) }`. There is **one generic
kernel, never a per-limb-type copy** (rule 2) — the u128 path is simply the `L = u128`
monomorphisation (so a hand-written u128 variant of an algorithm is a *superseded duplicate*
once the generic exists).

Mechanics: storage stays `Int<N>` (u64); a `u128` kernel packs its `N` u64 limbs into
`⌈N/2⌉` u128 (a cheap little-endian reinterpret into the u128 `ComputeLimbs` buffer), runs,
unpacks. Packing pairs two u64 into one u128, so it is exact only for an **even** limb count
— the matcher returns `U128` only for even cells; odd/narrow tiers stay `U64`. The `L`-typed
scratch comes from `ComputeLimbs` on `Limbs<N>` (which carries both the u64 and u128 buffer
families); the `Limb` accessors route `L` to the matching family (e.g.
`L::double::<Limbs<N>>()`), so the kernel never names a build-max
size. Rolled out pilot-first, microbench-gated per cell: a cell routes `U128` only where the
benchmark shows the win.

The limb width is selected in **two stages**, and the second stage is **owned by the
algorithm**: `select` resolves the *algorithm* first (the existing `ByAlgorithm` / `ByValue`
axis); then the chosen algorithm yields its *own* limb width via a `const fn limb_size<const
N>(self) -> LimbSize` method — because the u64/u128 crossover is algorithm-dependent, it lives
on the `Algorithm` enum, not in the verdict. `dispatch` resolves the algorithm, asks it for
its limb width, and folds both in a `const { … }` block (when the algorithm is const) so the
whole thing collapses to one direct typed call. The limb width is **per-cell policy DATA**,
not a blanket: each algorithm's `limb_size` arm enumerates its benched winners, with
`LimbSize::for_packing(N)` (the even-`N` validity gate) as the default. The canonical shape
(reference instance `int/policy/mul_low.rs`, the truncated-low product):

```rust
enum Algorithm { LowLimb /* , … */ }

impl Algorithm {
    // SECOND axis — owned by the algorithm (the crossover is algorithm-dependent).
    // U128 only where a microbench wins AND it is valid (even N — for_packing gates odd → U64).
    const fn limb_size<const N: usize>(self) -> LimbSize {
        match self {
            Algorithm::LowLimb => LimbSize::for_packing(N), // ← carve a losing even cell to U64 HERE
        }
    }
}

enum Select { ByAlgorithm(Algorithm) /* + ByValue for a value-chosen algorithm */ }
const fn select() -> Select { Select::ByAlgorithm(Algorithm::LowLimb) } // <const N> if width-keyed

// dispatch — stage 1: resolve the algorithm; stage 2: ask it for its limb width.
pub(crate) fn dispatch<const N: usize>(a: &[u64; N], b: &[u64; N], out: &mut [u64; N]) {
    let (algo, limb) = const {
        let algo = match select() { Select::ByAlgorithm(a) => a };
        (algo, algo.limb_size::<N>())
    };
    match (algo, limb) {
        (Algorithm::LowLimb, LimbSize::U64)  => mul_low_limb::<N, u64>(a, b, out),
        (Algorithm::LowLimb, LimbSize::U128) => mul_low_limb::<N, u128>(a, b, out),
    }
}
```

Under a `ByValue` algorithm choice the algorithm resolves at run time; `limb_size::<N>()` is
then read inside the chosen arm (still const per `N`, still value-independent). A function
whose limb form is a *different algorithm* rather than a knob (the slice **divide**: its u128
form is base-2¹²⁸ Knuth, structurally distinct — see the const-vs-runtime note below) carries
that as its own `Algorithm` variant, so there is simply no `limb_size` knob to select.

**Const-`N` vs runtime-shape functions.** The const verdict above fits functions keyed on a
compile-time width (the truncated-low product is `<const N>`). A function whose policy is
**`ByShape`/`ByValue`** (keyed on *runtime* operand lengths — the slice divide, whose `2N`
scaled numerator has no nameable type) cannot carry a const `LimbSize`: there its limb-width
choice is a **runtime** decision and belongs as a distinct `Algorithm` engine variant the
shape classifier selects (e.g. a u128-limb Knuth engine chosen when the effective limb counts
are even and wide enough), not a const verdict. Same axis, delivered where the key allows.

## Algorithm choosing — and pruning

A single function (say `sqrt`) has several possible algorithms — a
small-width kernel, a wide generic kernel, a bespoke kernel for one scale
band, and so on. The choice is made by a **per-function policy**: a
`const fn select`, keyed on the compile-time width(s) and scale(s), that
returns which **`Algorithm`** to run. The exact file shape is in
*Policy file structure* below; the gist:

```rust
const fn select<const N: usize>() -> Select<N> {
    match N {
        0..=2 => Select::ByAlgorithm(Algorithm::Newton),     // small-width kernel
        3..=8 => Select::ByValue(/* the value decides */),   // value-dependent band
        _     => Select::ByAlgorithm(Algorithm::Zimmermann), // the chosen default
    }
}
```

The arms express the levels of choice: a **default** (the algorithm in the
`_` arm), **width/scale-range overrides** (a band picks a different
algorithm), and — where the best algorithm depends on the operand's
*value* rather than its width — a **value matcher** (`ByValue`). Top
matching arm wins.

### Pruning = dead-arm elimination

`W` and `SCALE` are *constants in every monomorphisation*. So for the
concrete type `D57<20>`, the compiler evaluates the match at compile time,
**discards every arm that doesn't match**, and inlines the one that does.
`D57<20>::sqrt` compiles to a direct call to exactly one kernel — no
branch, no table, no vtable. Every other candidate kernel is pruned out
of that type's machine code. This is what makes the rich policy table
**zero runtime cost**.

**What the `const` buys is compile-away, specifically.** `select` is
`const` and keyed only on the const generics, so the inline
`const { select::<…>() }` block evaluates at compile time and the
dispatcher *disappears* — the caller shortcuts straight to the chosen
algorithm. That is all `const` is for here; it is **not** about exposing a
`const fn` public API (a method keeps or loses its own `const`-ness on its
own merits).

**The policy/seam itself has more purposes than compile-away**, and they
are why **every function fits this shape — there is no op that cannot**,
even a single-algorithm one:

- **zero-cost dispatch** — the compile-away above;
- **one obvious place to choose and swap the algorithm** per
  `(width, scale)` cell — the algorithm choice lives in `select`, not
  scattered through the call sites, so swapping or adding a kernel is a
  one-file edit;
- **an isolated, testable dispatch** — the seam is a clean unit to test
  and to **microbenchmark**, so comparing algorithm choices for a cell is
  easy.

A single-algorithm op is therefore still worth a policy: it is a pure
`ByAlgorithm` matcher that folds to one direct kernel call today, and it
gives that op a ready seam to add/swap/bench an algorithm later.

**Layering direction.** The call graph only ever points *down*: a type
method delegates to its `policy::<fn>::dispatch`, the dispatch selects an
**algorithm fn**, and the algorithm computes via **kernels** (or another
tier's policy/method surface). An algorithm fn must **never** call a method
back on its own type (`x.cube()` from inside the cube algorithm is the
inversion — the method and the algorithm have swapped roles). The impl
lives in the algorithm/kernel; methods are thin delegators down. Each
algorithm — even a trivial schoolbook — is a named `<function>_<method>`
in its own file under `algos/<function>/` (int: `int/algos/<function>/`),
not inlined in the policy and not dismissed for being simple.

**The property holds through `ByValue` too — it is a residue, not an
exception.** Where the algorithm depends on the runtime *value*, the const
`select` has *already* pruned every other arm and folded the cell down to
the one value-matcher it picked **before** any value is examined. So the
only thing surviving into the binary is the irreducible value check
itself: one non-capturing `fn(&…) -> Algorithm` returning a tag, then the
`match` on that tag — about as cheap as a plain function call. The
selection was const; only the value-dependent choice (which genuinely
*needs* the value) remains.

### Feature-flagging a variation

A feature- or platform-specific variation lives in the policy file, gated where
it belongs:

- a `#[cfg(feature = "…")]` arm inside `select` — pick a different `Algorithm`
  for some widths when the flag is on;
- a cfg-gated value-matcher — a different runtime split under the flag;
- a cfg-gated `Algorithm` variant (plus its dispatch arm) — an algorithm that
  only exists under the flag.

```rust
// a `std`-only algorithm — variant, chosen kernel, and dispatch arm are gated
// together, so the policy stays exhaustive in BOTH configs.
enum Algorithm { Newton, Zimmermann, #[cfg(feature = "std")] StdSeeded }

#[cfg(feature = "std")]      const fn small() -> Algorithm { Algorithm::StdSeeded }  // with `std`
#[cfg(not(feature = "std"))] const fn small() -> Algorithm { Algorithm::Newton }     // core default

const fn select<const N: usize>() -> Select<N> {
    match N {
        0..=2 => Select::ByAlgorithm(small()),                 // std-swapped via the gated fn
        _     => Select::ByAlgorithm(Algorithm::Zimmermann),
    }
}
// in the dispatch, the std arm is gated to match the variant:
//   #[cfg(feature = "std")] Algorithm::StdSeeded => isqrt_std_seeded::<N>(x),
```

The unflagged policy is the default; the flag adds or overrides arms beside it,
in the same file. If a flagged variation uses `f64`, it is only ever a **seed**
to a self-correcting integer iteration — the exact integer termination pins the
unique result, so determinism holds regardless of the platform's `f64`.

### Seeds — the shared seed library (`algo_x_support::seed`)

Every initial Newton/root estimate comes from **one** place: the shared seed
library `src/algo_x_support/seed.rs` (the cross-algorithm-support leaf, the seed
analogue of `int/algos/limbs.rs`). Kernels **call it; they never hand-roll a
seed inline.** It exposes `sqrt_seed` / `cbrt_seed` (limb-slice) and
`sqrt_seed_u128` (the `u128`-scalar sibling for hot scalar isqrt call sites like
`hypot`); `isqrt`/`icbrt`/`hypot`/the decimal roots all source from it.

The library is where the **feature-based optimization** lives, written once: each
seed leaf cfg-swaps internally — under `std` it bootstraps from the inherent
hardware `f64` intrinsic of the top 64 significant bits (~53 correct bits → the
Newton loop converges in ~1–2 iterations), under `no_std` it is the classical
pure-integer one-bit estimate (`2^⌈bits/2⌉` for sqrt, `2^⌈bits/3⌉` for cbrt,
primitives only — never `libm`). **Both bodies are guaranteed _over-estimates_**
(the odd-shift `SQRT_2` correction + round-up; Hasselgren, Crandall & Pomerance
§9.2.1), so the downward-monotone Newton recurrence converges to the **identical
floor** on either build and never under-runs into the catastrophic linear
`x -= 1` floor-walk an un-corrected inline `f64` seed causes. Callers stay
`std`/`no_std`-agnostic; the result is platform-deterministic (only the
iteration count differs by build).

Consequences: a kernel that hand-rolls a pure-integer-only seed inline silently
**forfeits the `std` f64 fast-bootstrap on every build**; one that hand-rolls an
un-corrected `f64` seed risks the linear floor-walk. Either is a defect — route
the seed through the library, and if a seed needs fixing, fix it in the library
so all callers benefit.

### Policy file structure (the per-function matcher)

Each dispatched function (`sqrt`, `mul`, `exp`, …) has **one policy file** with a
fixed shape. Agents implementing or extending a policy follow this template
exactly. The dispatch key is the compile-time width(s) and scale(s) —
`N` (int unary), `(N, M)` (int binary), `(N, SCALE)` (decimal unary), or
`(N, M, S1, S2)` (decimal binary).

```rust
// 1. the real algorithms for this function — NAMED, paper-based, no `Default`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { Newton, Zimmermann }

// 2. the const verdict: a settled algorithm, or "the value decides".
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),                // width/scale settled it
    ByValue(fn(&Int<N>) -> Algorithm),     // value-dependent (non-capturing fn / closure)
}

// 3. the matcher: a `const fn`, keyed ONLY on the const generics, total over the key.
const fn select<const N: usize>() -> Select<N> {
    match N {
        0..=2 => Select::ByAlgorithm(Algorithm::Newton),
        3..=8 => Select::ByValue(|v: &Int<N>|           // value-matcher (see placement rule)
            if v.bit_length() <= 128 { Algorithm::Newton } else { Algorithm::Zimmermann }),
        _     => Select::ByAlgorithm(Algorithm::Zimmermann),   // the chosen default (a real algorithm)
    }
}

// 4. the public function: resolve the verdict, then dispatch — exhaustively, no panic.
pub fn isqrt<const N: usize>(x: Int<N>) -> Int<N> {
    let algo = match const { select::<N>() } {          // inline const block: folds at compile time, can see `N`
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f)     => f(&x),                // the ONE place a runtime value enters
    };
    match algo {                                        // exhaustive over `Algorithm` — no `_`, no `unreachable!()`
        Algorithm::Newton     => isqrt_newton::<N>(x),
        Algorithm::Zimmermann => isqrt_zimmermann::<N>(x),
    }
}
```

Rules that make this work:

- **`Algorithm` lists real algorithms only — no `Default` variant.** Completeness
  is structural: `select` is total over the key, and `match algo` is exhaustive
  over `Algorithm` (both compiler-enforced). The "default" for unspecialised
  widths is simply the real algorithm named in `select`'s `_` arm.
- **Algorithm fn/module naming = `<function>_<method>[_with_<method2>][_variant]`.**
  Prepend the function it performs (`sqrt_`, `cbrt_`, `div_`, `mul_`, `exp_`, …),
  then the literature/paper method (`newton`, `knuth`, `karatsuba`, `mg_divide`,
  `burnikel_ziegler`, `tang`, `series`, …). Hybrids keep the other method's **full
  name**: `div_tang_with_mg_divide`, `div_newton_with_mg_divide`. Append a sensible
  `_variant` only to disambiguate. **Widths are const-generic params: PREFER one
  algorithm generic over `N`, serving a *range* of limb counts via the matcher's
  range arms (`min..=max`).** A width-encoded name is a **last resort** — only when
  the algorithm genuinely cannot be generic *and* the method / `_with_` / variant
  names can't disambiguate; then suffix the **limb count** as `<N>_limb` (e.g.
  `mul_karatsuba_4_limb` for `Int<4>`), never `_int2`, bit-width, or `dXX`. The `Algorithm` *enum variant*
  = **CamelCase of the fn name minus its function prefix** (`sqrt_newton`→`Newton`,
  `sqrt_newton_with_table_seed`→`NewtonWithTableSeed`) — a strict 1:1 variant↔fn mapping.
- **`select` is `const`, called via an inline `const { … }` block, keyed only on
  the const generics.** Per monomorphisation `const { select::<N>() }` evaluates
  to a constant `Select`, so the matches fold and every unchosen arm is
  dead-arm-eliminated (the pruning above). The policy is zero runtime cost; a
  `ByValue` arm leaves only the irreducible value check (the const selection of
  *which* matcher still folds away — see "Pruning" above). _(Use the
  inline `const { }` block, not a `const SEL: Select<N>` item — a `const` item may
  not reference the function's generic `N`; the inline block can.)_
- **Value matcher** (`ByValue`) — for the rare case where the best algorithm
  depends on the operand's *value* (e.g. actual magnitude), not just its width:
  - **non-capturing**, takes the value, and **returns an `Algorithm` tag** (never
    a function pointer — the tag keeps dispatch a direct call);
  - **placement by size:** ≤2 outcomes → inline closure `if`/`else`; 3–10 →
    inline closure `match`; >10 (or shared / unit-tested) → a named `#[inline]`
    fn `<function_name>_<applicable_preconditions>`;
  - **the suffix names the arm's applicable preconditions — its count/shape
    varies with the matcher:** a single int width is `sqrt_N5`, an int width-range
    is `sqrt_N5_to_N10`, a decimal arm adds scale (e.g. `sqrt_N2_S0_to_S9`). Encode
    exactly the preconditions that apply.
- **`core`-only.** One policy per function, compiling on every platform;
  feature- or platform-specific variations are gated inside it (see
  *Feature-flagging a variation* above).
- **Acceptance gate:** the zero-runtime-branch property is a *release* property;
  it is proven per function by inspecting the release IR/asm (one direct call, no
  branch/table/vtable on the const path).

### Keeping the alternatives

**Algorithms are never deleted.** A kernel that loses at today's widths
(FFT/NTT multiplication, AGM below ~D1232, …), an **unwired candidate**
awaiting a bench/policy seam, or a **reference baseline** (the schoolbook
`Algorithm` variants) is *kept* — as a documented reference and, where the
implementation genuinely differs, as compiled-out / unrouted code — because a
future CPU/LLVM instruction, a platform-specific build, or a re-tuned
threshold can flip a today-loser into a winner. **"Unwired" or "reached only
by its own tests" is the EXPECTED state of a kept alternative, not dead code
to remove.** See `ALGORITHMS.md`.

The one thing that *is* removed is a **superseded duplicate** — the obsolete
*shape* of an algorithm that a migration replaced in place (e.g. a
`const`-work-width `fn f<W, const N>` once it became
`fn f<W: BigInt> where W::Scratch: ComputeLimbs`):
that is the same algorithm's dead skin, not an alternative, so it goes. The
test of "alternative (keep) vs superseded duplicate (remove)": does it
implement a *distinct* algorithm/shape that could win under different
conditions (keep), or is it the leftover old signature of something that now
exists in one canonical form (remove)?

## How the guarantees are enforced — by testing

The architecture's two headline promises are **platform determinism** and
**correct rounding**, and both are nailed down by tests rather than
asserted by hope.

**Determinism** falls out of the design (integer-only core, no floating
point in results) and is exercised by the cross-platform CI matrix and
bit-exact fixtures.

**Correct rounding** is the contract that the strict transcendentals
return the value within 0.5 ULP of the true real value — equivalently,
the **exact correctly-rounded value at the storage scale (0 LSB of
error)**, under *every* rounding mode and at *every* width. It is checked
by independent layers (see `precision-testing.md`):

1. **Hand-computed truth tables** at D38 — the smallest, human-audited net.
2. **Cross-witness** — compute at a tier, recompute the reference at a
   wider storage and rescale; catches storage-bit divergences.
3. **mpmath golden tables** — an external oracle (computed at working
   precision far wider than any tier) for every (function, tier); the
   kernel must match the correctly-rounded oracle **exactly (delta == 0)**
   for all six rounding modes across all widths.
4. **Property fuzz** — identities like `exp(ln x) ≈ x`, `sin²+cos² ≈ 1`,
   and sign symmetries, with deterministic seeds.

The integer backends carry their own bit-exact tests (each algorithm
checked against a schoolbook oracle across widths and edge cases). A
performance change can never silently cost accuracy: the delta == 0
precision suite is a permanent regression gate, so a faster kernel that
rounds wrong turns CI red.

## Where the rounding actually happens

The kernels compute at a wider *working scale* (`SCALE + GUARD` digits)
and then round to the storage scale. For the three nearest modes a fixed
guard is enough; for the directed modes (toward zero / ±∞) the rounding
decision needs the *sign and stickiness* of the sub-LSB residual — which
the divide already computes — and, on the rare inputs sitting within the
kernel's error of a tie, an adaptive widening step (Ziv iteration) settles
it. The result is correct rounding under all six modes with the common,
nearest-mode path paying nothing extra.

## Overflow & domain behaviour — one contract, invariant across tier and scale

**Principle (hard rule).** An operation's behaviour on overflow / out-of-range
results is IDENTICAL at every width and every scale. A behaviour chosen for one
cell holds for all of them: if a strict transcendental panics on an out-of-range
result at a wide width, it panics identically at a narrow width, and scale never
changes the contract. Tier- or scale-dependent overflow behaviour is a bug, not a
detail — the historic split (wide transcendentals `panic!`, narrow ones silently
returned a wrapped/saturated value) is exactly what this rule forbids.

**How it's enforced — detect once, apply the policy in the wrapper.** The kernel /
final narrow-to-storage step DETECTS overflow exactly once and falls through to the
decimal method with that signal (the `checked` form — `Option` / `Result` — is the
primitive). The public method variants are thin policy wrappers over that single
detection, mirroring how `i64::overflowing_add` sits under `add` / `checked_add` /
`wrapping_add` / `saturating_add`:

- `<fn>` (default) — unwrap-or-panic: **panics on overflow in both debug and
  release.** A fixed-width decimal has no ∞/NaN, and silently returning a
  wrapped or saturated value is a wrong number with no signal — unacceptable for a
  precision type, so the default fails loudly. (This follows `rust_decimal` /
  .NET `Decimal`, deliberately NOT the integer wrap-in-release default.)
- `checked_<fn>` — returns the `Option` / `Result` directly (the primitive form).
- `saturating_<fn>` — clamps to `MAX` / `MIN`; an explicit opt-in, never a default.
- `wrapping_<fn>` — modular two's-complement; arithmetic only (wrapping a
  transcendental result is meaningless, so it is not exposed there).

Because the detection lives in one place and the wrapper is the only thing that
varies, the tier/scale-invariance is structural: there is no per-tier branch left
that can drift. The exact arithmetic ops carry the full `checked_` / `wrapping_` /
`saturating_` / `overflowing_` family.

The strict transcendentals expose the default (panic) + the `checked_` form:
every `<fn>_strict` / `<fn>_strict_with` has a `checked_<fn>_strict` /
`checked_<fn>_strict_with` sibling returning `Option<Self>`, emitted once as a
single generic impl over `(N, SCALE)` (`src/types/checked_transcendentals.rs`).
`None` covers both classes of panic: domain errors (`ln` of a non-positive
value, `asin` outside `[-1, 1]`, …) are prechecked exactly in the shell at
every tier, and out-of-range results propagate the kernel's own detection
`Option` where that seam has been threaded — exactly on D18/D38 for the
`exp` / `exp2` / `ln` / `log` / `log2` / `log10` / `powf` kernels and at every
tier for `hypot`. The remaining detection points (inside the wide kernel
shells, and the D38-width trig/hyperbolic/angle kernels) still panic in the
checked form, identically to the default — never a silent wrong value — until
the seam reaches them; `research/checked_wide_shell_patch.md` carries the
completion plan, and each method's doc states its exact contract.

## Map of the source tree

```
src/
  int/        const-generic integer layer
    types/    Int<N>/Uint<N> — the [u64; N] limb integers; the BigInt trait
    policy/   per-function `select` dispatch (keyed on limb count N)
    algos/    the algorithms — limb-slice arithmetic (add/mul/divmod/…) + width-matched kernels
  types/      Dxx<SCALE> typed shells, the Decimal trait family, consts
  policy/     per-function `select` dispatch (keyed on width N, SCALE)
  algos/      the algorithms (sqrt cbrt exp ln trig pow …)
  macros/     code generation for the per-type method shells
  support/    rounding modes, errors, display, serde helpers
```
