# `decimal-scaled-golden` — Architecture

> **Living design doc** — captured as the owner dictates, section by section.
> Lives in `research/` for now; the intent is to **ship it inside the
> `decimal-scaled-golden` crate** once the design settles.
>
> **STATUS: DESIGN COMPLETE & APPROVED (2026-06-06).** Implementation plan next;
> see *Decisions (locked)* + *Build plan (phased)*.

## Purpose

A standalone crate, **`decimal-scaled-golden`**, that owns **everything
"golden"** — the correctness golden values, the oracle machinery that generates
and cross-validates them, the library-facing trait a decimal library implements
to be tested, the test that runs the whole corpus, and the multi-library
comparison bench. It pulls all golden / precision / comparison infrastructure
out of the main crate into one dedicated place behind a **library-agnostic
interface** (so any decimal library — not just `decimal-scaled` — can be
validated and benchmarked against the same golden corpus).

## What the crate contains

1. **All the golden values.** The golden corpus itself lives in this crate.
2. **Oracle generation + cross-validation.** The interface to **generate** golden
   values from a *chosen* oracle, and to **validate** them using *alternate*
   oracles.
3. **The library validation interface — the `DecimalSubject` trait.** The trait
   that libraries implement so they can be validated against the golden values.
   **No `DecimalSubject` impls live in this crate** — see *Crate boundary* below.
4. **The golden test.** The test that executes all golden values.
5. **The comparison bench.** The bench that compares between an **arbitrary
   number of libraries**.

## Golden value representation

- **Singular golden values.** A golden value is **one value per (function,
  input)** that **supports all widths and scales** — *not* a per-`(width, scale)`
  table (the current per-tier/per-scale file explosion goes away). The expected
  result for any specific tier and scale is **derived** from the single stored
  value by rounding/truncating it to that target.

- **Stored as a string, not a numeric.** Each value is a **decimal string** —
  no `Int<N>` / numeric type to store it; width-independent, human-readable,
  greppable, and directly re-checkable by an alternate oracle.

- **Precision: `max_decimal_width + 2`.** Each value is generated to **two
  digits beyond the widest tier's maximum scale**. The widest tier is the
  **4 Kb (4096-bit) `Int`** (D1232, `MAX_SCALE = 1231`), so values run to
  ≈ **1233 fractional digits**, using **standard truncation**.

- **Format: plain `digits.digits`, in all cases.** No sentinel, no separator —
  just the decimal number, truncated at `max_decimal_width + 2`. Those two guard
  digits beyond the widest scale are exactly what is needed to round correctly to
  **any** tier and scale, including round-half and true ties at the widest scale:
  - a value that **terminates before** `max + 2` is **exact** (finite);
  - a value that **runs to** `max + 2` carries a **residual** below the widest
    scale (not exact);
  - at the widest scale, `…5` that **terminates** is an **exact tie**, while
    `…5x` (a further digit) is **above half**.

> *Design trail:* this representation passed through a `:R` rounding-class
> sentinel (`0/-/5/+`) and then a `:`-separated trailing-digit form during
> design. Both were dropped — generating to `max_decimal_width + 2` with standard
> truncation makes a separate indicator unnecessary and is simpler. (The `+2`
> superseded the earlier `+1`: the 2nd guard digit lets a truncated `…5` be
> distinguished as an exact tie vs. a continuing value.)

## Golden file format

A golden file line is the **inputs followed by the output**, fields separated by
**`[ \t]+`** (one or more spaces/tabs — *not* `\s+`, which would swallow the
newlines). **Each newline separates one test (one vector) from the next.** Each
field is a `digits.digits` value (the output at `max_decimal_width + 2`
precision):

```
input1 [input2 … inputN] output
```

- **Unary** function: `input output`
- **Binary** function: `input1 input2 output`
- **N-ary**: `input1 … inputN output`

## Golden loader

The **golden loader** turns the singular high-precision values into the
correctly-rounded expected value for a target. Its job:

1. **Pick up values** for either a **library** or a **(width × scale)**.

2. **Determine whether a value fits** — based on the **number of digits above the
   decimal place** (the integer-part digit count). If the integer part is too
   large for the target's capacity, the value does not fit (n/a for that
   library / tier).

3. **Shorten and correctly round** to the target scale, using the stored digits:
   - The rounding is decided by the **digit(s) below the target scale** (standard
     truncation/round — the next-down digit decides, scanning further for ties).
   - The **two guard digits beyond the widest scale** carry the round-half /
     tie / exactness information *at the widest scale*: a value that terminated
     before `max + 2` is exact (so e.g. Ceiling does not bump); one that ran to
     `max + 2` has a residual below (Ceiling bumps); a terminated `…5` is a true
     tie.

## Runtime components (running tests / benches)

A pipeline of four components:

1. **Function tester (runner).** Runs a function's tests against a single
   implementation, or against multiple implementations (speed comparisons or
   precision collections).
   - Runs the **whole file per library**.
   - A **minor warm-up at the start** — **not per-line** (especially on benches).
   - **Queries the implementation** for its **max decimal width, precision, and
     rounding mode** for that function.
   - **Skips** a test when the parser returns inputs **not compatible** with the
     library.
   - **There are multiple tester implementations** (a tester trait / role — *not*
     one tester with a flag). Each shares the parse → run → validate → collate
     pipeline but produces its **own kind of detail**:
     - a **correctness tester** — run **and validate** every vector (per-vector
       pass/fail so failures are named);
     - a **timing tester** — **one warm-up then time the batch**, **adding extra
       detail such as timings** (no per-call validation overhead);
     - new measurement kinds are **new testers**, added without touching the
       others.

2. **Parser.** Parses the input and output values per file (the `[ \t]+`-separated
   `digits.digits` fields; one test per line).

3. **Validator.** Takes the implementation's output and compares it against the
   golden output **left to right**. If the **precision guarantee** or **rounding
   guarantee** is missed it records a **failure** — with **how much it missed
   by**, or **the rounding it actually used**. Also collects other failure types
   (e.g. **timeouts**, panics, …).

4. **Collator.** Collates all the information into **one file**. Display may need
   to **filter** the data (e.g. for a single width and/or precision), so the file
   must carry **enough information to discern** each record (library, function,
   width, scale/precision, rounding mode, …).

### Refinements (accepted)

- **Multiple tester implementations** (the tester trait above) — a *correctness*
  tester and a *timing* tester that adds timing detail, extensible to more.
  (Folded into *Function tester*.)
- **One unified result record for both precision and speed.** The collator file
  uses a single record schema with optional timing *and* optional precision
  fields, so it serves the precision shootout **and** the comparison bench (one
  collated file, filterable by library / function / width / scale / mode).
- **The "query the implementation" metadata is the library trait's capability
  surface.** The trait exposes, per function: max decimal width, precision,
  supported rounding mode(s), and `eval(inputs) -> output-string`. The tester's
  *skip-if-incompatible* uses that capability + the parsed value's digit count —
  i.e. it reuses the **loader's fit-check**.
- **A small explicit result taxonomy** for the validator/collator:
  `pass` · `mis-rounded (Δ = …)` · `wrong-rounding-mode (used = …)` ·
  `out-of-range / skipped` · `timeout` · `panic`. Makes filtering and the
  fidelity grade fall straight out of the data.

## Crate boundary, dependencies, and process separation

- **`DecimalSubject` is the confirmed trait name.**
- **No `DecimalSubject` impls live in this crate.** `decimal-scaled-golden` is
  *solely* the infrastructure: the `DecimalSubject` trait, the loader / parser /
  validator / collator / tester(s), the golden values, and the
  oracle-generation/validation interface. The `DecimalSubject` *implementations*
  (adapters) are provided by the **libraries under test** — `decimal-scaled` and
  each competitor — which depend on this crate and run the tests/benches.
- **Dependency direction.** The crate depends on **only the golden oracle
  generator(s)** plus what is **strictly necessary to do the job** — it does
  **not** depend on `decimal-scaled` or any subject library. The dependency
  points the other way: subjects depend on this crate.
- **Consumed as a dev-dependency.** It is effectively *production-quality code
  used by tests to do a job*, so consuming libraries pull it in as a
  **dev-dependency**.
- **Generation is a separate process from execution.** Golden *generation*
  (build/refresh the corpus from a chosen oracle, cross-validated with alternate
  oracles) is a **distinct process** from *executing* the tests/benches (which
  consume the committed corpus). They are not run together.
- **Post-build:** define `DecimalSubject` adapters for **`decimal-scaled` and the
  competitor libraries**, then execute.

## Decisions (locked)

1. **Full replacement, phased.** Build `decimal-scaled-golden`, migrate the
   corpus (regenerated), port decimal-scaled's `DecimalSubject`, then **retire
   the old golden infra**: `tests/ulp_strict_golden.rs` (`decl_band!` /
   `band_edges` / `wide_s30_exp`), `scripts/gen_band_edge_wiring.py`, the
   per-`(width,scale)` corpus, and the `lib_cmp_*` benches.
2. **Generation stays Python** (mpmath, with FLINT/Arb cross-validation). The
   Rust crate *consumes* the generated text corpus and provides the trait /
   loader / testers; its Rust deps stay minimal.
3. **Corpus is regenerated**, not converted, in the new singular
   `digits.digits`-to-`max+2` format; the input/vector set is seeded from the
   existing golden inputs + the adversarial-hunter inputs.
4. **Phased build order** (below); the `DecimalSubject` trait is drafted first.

## Build plan (phased)

1. **Crate skeleton** + `DecimalSubject` trait + value format + parser + loader.
2. **Tester trait** (correctness + timing) + validator + collator + unified
   result record.
3. **Generation process** (Python) + regenerate the corpus in the new format.
4. **decimal-scaled adapter** — `DecimalSubject` for decimal-scaled + wire its
   test/bench (dev-dependency on the golden crate).
5. **Competitor adapters** — `DecimalSubject` for the comparison libraries.
6. **Retire** the old golden infra (Decision 1).

> The spec is the source of truth and **moves into the crate at phase 1**. Next
> step before any code: a detailed, per-phase implementation plan for approval.

## Trait architecture (Phase 2 — final)

The Phase-1 `DecimalSubject` was a string-in/string-out sketch. The final design
(settled 2026-06-07) splits it so a runner can time **only** the operation, never
the string conversions, and stays **type-erased** over each library's native type.

### `Computed<T>` — the result of asking a library to compute

```rust
pub enum Computed<T> {
    Value(T),       // the library produced a result
    Skip,           // not applicable: out of domain / not representable at this width·scale
    Error(String),  // the library failed on an input it should have handled (reason flows to the report)
}
```

Replaces `Option` everywhere a library result is returned. `Skip` (expected
n/a) and `Error` (a real defect) are NOT conflated. A **panic** is not a
`Computed` variant — it is a crash, caught separately (`catch_unwind`) and mapped
to `Outcome::Panic`.

### The typed trait (each library's adapter implements this)

```rust
pub trait DecimalSubject {
    type Value;                                                     // the library's native decimal type
    fn capabilities(&self, func: Function) -> Capabilities;        // 1. name + max width/scale + rounding modes + supported
    fn to_text(&self, v: &Self::Value) -> String;                  // 2. value -> string
    fn from_text(&self, s: &str, width: u32, scale: u32) -> Computed<Self::Value>;          // 3. string -> value
    fn execute(&self, func: Function, inputs: &[Self::Value], width: u32, scale: u32,
               mode: RoundingMode) -> Computed<Self::Value>;       // 4. run on native types
}
```

`Value` is an **associated type** (each library has exactly one native type). The
adapter author makes the `Skip` vs `Error` judgement.

### The erased trait (the runner uses ONLY this — `Value` erased)

```rust
pub trait ErasedSubject {
    fn capabilities(&self, func: Function) -> Capabilities;
    /// correctness: per-case convert -> execute -> stringify (no timing)
    fn eval(&self, func: Function, inputs: &[&str], width: u32, scale: u32, mode: RoundingMode) -> Computed<String>;
    /// timing: convert the whole cell up front (untimed), warm up, then time the
    /// execute loop (conversions excluded, execute black-boxed). Returns total ns.
    fn time_batch(&self, func: Function, cases: &[Vec<String>], width: u32, scale: u32,
                  mode: RoundingMode, warmup: u32) -> Option<u64>;
}
```

A **blanket `impl<T: DecimalSubject> ErasedSubject for T`** captures `Value` and
erases it: `eval` does `from_text → execute → to_text`; `time_batch`
pre-converts the cell (untimed), warms up, then times one `black_box`-guarded
loop over just the `execute` calls. So:

- **Ownership:** the *caller* (a test/bench in a subject library) owns
  `Vec<Box<dyn ErasedSubject>>`; the **runner borrows `&dyn ErasedSubject`** —
  the same subjects feed both the correctness and timing runners.
- **Timing purity:** only `execute` is measured; conversions are outside the
  timed span; `execute` is `std::hint::black_box`-guarded (defeats DCE /
  const-fold, matching the `ab_microbench` discipline); timer overhead is
  amortised across the batch.

### Capabilities

```rust
pub struct Capabilities {
    pub name: String,                  // library name (carried here to keep the trait at 4 methods)
    pub supported: bool,               // does this subject expose `func` at all?
    pub max_width: u32,
    pub max_scale: u32,                // absolute max; cells a given width can't represent return Skip
    pub rounding_modes: Vec<RoundingMode>,
}
```

### Result taxonomy (mirrors `Computed` + the validator verdicts)

```rust
pub enum Outcome { Pass, MisRounded { delta }, WrongMode { used }, Skipped, Error { reason }, Timeout, Panic }
```

The tester maps each cell: `catch_unwind` trip → `Panic`; `Computed::Value` →
validate → `Pass` / `MisRounded` / `WrongMode`; `Computed::Skip` → `Skipped`;
`Computed::Error(r)` → `Error{reason:r}`. `Timeout`'s mechanism (per-cell budget
on a worker thread, needs `Subject: Sync`) is deferred to Phase 5.

