# `decimal-scaled-golden` — Architecture

A standalone, **library-agnostic** crate that owns everything "golden": the
correctness golden values, the oracle machinery that generates and cross-validates
them, the trait a decimal library implements to be tested, the runner that executes
the golden cases, and the multi-library comparison bench. Any decimal library — not just
`decimal-scaled` — can be validated and benchmarked against the same golden
values by implementing one trait.

This document is the design's source of truth. Where the implementation still
lags it (the code currently exposes `Subject`/`representable`; the target is
`DecimalSubject`/`limits`), the document describes the target and the code is
brought into line.

---

## 1. Purpose and boundary

### What the crate contains

1. **The golden values** — the correctness reference values themselves.
2. **Oracle generation + cross-validation** — the (Python) interface that
   *generates* golden values from a chosen oracle and *validates* them against
   alternate oracles. A separate process from execution.
3. **The `DecimalSubject` trait** — the library-facing interface a decimal
   library implements to be tested. **No `DecimalSubject` impls live in this
   crate.**
4. **The runner** — execution strategies, validators, the collector tree, and
   reporting, which produce the correctness and timing results.

### Crate boundary and dependency direction

- **No subject impls here.** The adapters that bind `decimal-scaled` and each
  competitor library live in the *libraries under test*, which depend on this
  crate — never the reverse. This crate does not depend on `decimal-scaled` or
  any subject library.
- **Consumed as a dev-dependency.** It is production-quality code that tests do a
  job with; consuming libraries pull it in under `[dev-dependencies]`.
- **Minimal Rust deps.** The crate reads committed golden files and provides the
  trait + runner. Oracles are not Rust dependencies (see §7).
- **Generation ≠ execution.** Building/refreshing the golden values from an oracle
  is a distinct process from running the tests/benches, which consume the committed
  files. They are never run together.

This crate **replaces** the old in-tree golden infrastructure: the
per-`(width, scale)` golden files, `tests/ulp_strict_golden.rs` (`decl_band!` /
`band_edges`), the band-edge wiring scripts, and the `lib_cmp_*` benches all
collapse into the single shared golden set + agnostic runner described here.

---

## 2. The subject model — pinned per cell

**A subject is one library pinned to exactly one `(width, scale)` cell.** It
parses, computes, and formats at that one cell; it never takes a width/scale
parameter at call time. The runner enumerates a surface by holding *many*
subjects:

- A **fixed-width** library (e.g. `decimal-scaled`) provides one subject per tier
  via monomorphisation — `DecimalScaledSubject<N, S>` over each `(N, S)` it
  compiles.
- A **variable-precision** library (e.g. `rust_decimal`) provides one subject per
  target scale, carrying that scale as a field; all delegate to the same native
  type.

This keeps the trait methods value-only (no cell threading) and lets the runner
treat every subject identically. The subject is **pure**: it parses, computes,
formats, and *panics* on input it cannot take — it never decides to skip and
never catches. The skip and catch policy live in the runner.

---

## 3. The `DecimalSubject` trait

```rust
pub trait DecimalSubject {
    type Value;                                   // the library's one native decimal type

    fn capabilities(&self) -> Capabilities;       // identity, radix, per-function support, report metadata
    fn string_to_value(&self, s: &str) -> Self::Value;     // parse (panics on input it can't take)
    fn value_to_string(&self, v: &Self::Value) -> String;  // format to canonical decimal text
    fn limits(&self, value: &str) -> Limits;      // the representability envelope (see §3.2)

    /// Curry func/mode/overflow into a compute-only closure over pre-parsed
    /// inputs — no parse, no format — so a timing run measures exactly the op.
    /// The closure yields `Computed<Value>` (§3.3) so a peer can report a
    /// non-value outcome (absent / non-real / error) without panicking.
    fn execute(&self, func: Function, mode: RoundingMode, overflow: Overflow)
        -> impl Fn(&[Self::Value]) -> Computed<Self::Value>;
}
```

`Value` is an associated type (one native type per library). Splitting
`string_to_value` / `execute` / `value_to_string` lets a timing run convert the
cell *once* outside the timed span and measure only `execute` (§5.2).

### 3.1 Capabilities

```rust
pub struct Capabilities {
    pub name: String,                          // library name (for the report)
    pub radix: Radix,                          // Decimal (default) | Binary  — see §6
    pub config: BTreeMap<String, String>,      // width, scale, storage_bits, tier … REPORT-ONLY
    pub functions: BTreeMap<Function, FnSupport>,  // keys = supported functions
}

pub struct FnSupport { pub mode: RoundingMode, pub overflow: Overflow }
pub enum Radix { Decimal, Binary }
```

- **`width`/`scale` are not typed fields.** They are report metadata only, so
  they live in `config`. The runner never needs them: representability comes from
  `limits` (§3.2) and grading depth from `Limits::max_precision`. A typed field
  would invite the runner to bake a storage model into itself — the exact
  coupling this design removes.
- **Function support is presence in the map.** Absence ⇒ unsupported (the runner
  records the function as unsupported, runs no cells).
- **`FnSupport` is per function** — the rounding mode the subject is tested under
  and how it behaves when that function's result overflows the cell (§4.4).

### 3.2 `Limits` — the representability envelope

The **one** judgement that depends on a subject's internals is "what can I
represent." It lives behind the trait as a described *envelope*; the runner does
the comparison. The subject never exposes a bit width and the runner never
inspects one.

```rust
pub struct Limits {
    pub min_value: Option<String>,   // most-negative representable magnitude; None = unbounded below
    pub max_value: Option<String>,   // most-positive representable magnitude; None = unbounded above
    pub max_precision: u32,          // native fractional depth (places after the point)
}
```

- **`limits` takes the value being tested, not the case.** A limit is a property
  of *a value*, not of a `(inputs, output)` pair — and for a variable-precision
  library the envelope depends on the value's magnitude. The runner calls it with
  the golden output when classifying the output, and with an input string when
  filtering inputs. Fixed-width libraries ignore the argument and return
  constants.
- **`min`/`max` are `Option` because some libraries are genuinely unbounded.**
  Arbitrary-precision peers (e.g. bigdecimal, dashu-float) have no finite
  magnitude ceiling — their mantissa grows with memory, and passing the value
  cannot conjure a finite bound. `None` means "no bound on this side" ⇒ the
  runner treats the value as always in range. Fixed libraries return `Some`.
- **`max_precision` is always concrete (never `Option`).** A library never emits
  infinite digits — it computes to a *requested* working precision — so its
  native depth is always a finite number. For a fixed-scale library it is the
  scale; for a variable-scale library it may depend on the value's integer-digit
  count (hence the value argument).

`Limits` replaces the earlier `representable`/`fits` boolean: the subject
*describes* its envelope and the runner *classifies*. The boolean conflated the
input question (exact representability) with the output question (magnitude after
rounding) — see §4.

### 3.3 `Computed` — the outcome of one execution

`execute`'s closure returns `Computed<Value>` so a subject can report *what its
library actually did* without resorting to a panic.

```rust
pub enum Computed<T> {
    Value(T),           // a finite real decimal value
    NonReal(NonReal),   // a result that isn't one
    Absent,             // the library returned no value (e.g. a checked op → None)
    Error(String),      // the library returned an explicit error value; reason → report
    Timeout(u64),       // the subject exceeded its time budget (ms) — a test failure; runner-injected
    Panic(String),      // the subject crashed (caught) — a test failure, with the message
}

pub enum NonReal { PositiveInfinity, NegativeInfinity, NaN, Imaginary }
```

`decimal-scaled` only ever returns `Value` (or panics, per its strict contract).
Peers use the other arms to stay faithful: `rust_decimal`'s checked-overflow `None`
is `Absent`, `fastnum`'s `±Inf`/`NaN` are `NonReal`, and an imaginary answer — which
a real decimal cannot hold — is `NonReal::Imaginary`. **Every arm is a faithful fact
about what the library returned** — a value, a non-real, nothing, or an error — never
a decision about the test: the subject does not skip (skipping is the runner's call,
from `Capabilities` and `Limits`, §2/§4) and does not judge its own output. The harness works only in
real numbers, so the two infinities, `NaN`, and an imaginary result gather into the
single `NonReal` enum — everything outside the reals, in one place. Two arms the closure
does **not** itself return but that are still the *subject's* failure: `Timeout` (the runner's
watchdog fired — too slow, §10) and `Panic` (the runner's `catch_unwind` caught the subject
crashing, §5.2). Both are test failures attributable to the library, so they live in `Computed`.
Only a failure of the *harness itself* — not the subject — sits outside, in `ExecutionResult` (§5.2).

The runner classifies each arm against `in_range` (§4): on an out-of-range cell, a non-`Value`
arm that **matches the declared overflow policy** (§4.4) is the expected overflow; any non-`Value`
on an in-range cell is a bug; `NaN`/`Imaginary` on a valid input is a defect. `Timeout` is
orthogonal to range — a failure whatever the cell.

---

## 4. Cell classification — the three-way decision

Per golden case at a cell, the runner sorts the cell into exactly one of three
buckets before judging anything. Getting this wrong is what produced large counts
of false "defects" (out-of-range results scored as wrong answers).

1. **Input not representable → skip.** Not a valid test for this cell.
2. **Result not representable → expect the overflow policy.** The subject cannot
   return the correctly-rounded value; under the strict contract it panics, and
   that panic is *correct*, not a defect.
3. **Both representable → judge.** Rounding and precision are scored against the
   golden.

Input and output ask **different questions of the same envelope**, which is why
one boolean could not serve both:

### 4.1 Input filter — exact representability

For each input string `s`, with `lim = subject.limits(s)`:

```
representable_input(s) =
      significant_fraction_digits(s) <= lim.max_precision     // exactness
  &&  within(s, lim.min_value, lim.max_value)                 // magnitude
```

The exactness gate is what skips e.g. `1.5` at a scale-0 cell: its magnitude
fits but it is not *exactly* representable there, and feeding it would test
`f(rounded input)` against `golden(exact input)` — a false mismatch. "Significant
fraction digits" ignores trailing zeros (`1.00` at scale 0 is exactly `1`, and is
*not* skipped). If any input is not representable, the cell is recorded
`Unsupported` and not run.

### 4.2 Output classification — magnitude after rounding

With `lim = subject.limits(golden)`:

```
grade_precision = min(lim.max_precision, oracle_limits.max_precision)    // §4.3
rounded         = round(golden, grade_precision, mode)
in_range        = within(rounded, lim.min_value, lim.max_value)
```

Precision is irrelevant to the *output* envelope — the result is rounded to the
subject's depth, so only the rounded magnitude decides `in_range`. (Applying the
input's exactness check to the long golden output — rejecting it for having more
fraction digits than the scale — was a defect: every in-range cell flagged
wrong.)

### 4.3 The oracle is the ceiling — precision clamp + flag

The golden values are stored to `gen_precision` digits (§8; ≈ 1233 = widest scale + `guard`,
with `guard = 2`). Those guard digits are spent *deciding* the rounding, so the deepest we
can actually **verify** is `gen_precision − guard` (≈ 1231 = the widest scale itself). We can
never verify a subject deeper than that, so grading depth is clamped to it:

```
grade_precision = min(subject.max_precision, oracle_limits.max_precision)  // = gen_precision − guard
oracle_limited  = subject.max_precision > oracle_limits.max_precision
```

This is one symmetric rule with two sides:

- **subject shallower than the oracle** (a fixed peer capped below the cell, e.g.
  rust_decimal at 28 digits on a 150-place cell): graded at the *subject's* own
  last digit — correct at its depth, reported as a shorter "reach", **not** failed
  for being shallow.
- **subject deeper than the oracle** (an arbitrary-precision peer emitting more
  digits than the golden value carries): graded at the *oracle's* depth. It matches
  as far as we can see ⇒ **Pass, flagged `oracle_limited`** — explicitly "correct to
  the limit of our oracle; beyond that, unverified". The verdict stays a `Pass`; the
  runner records a cell-level `oracle_limited` flag (§5.4) alongside it, so the report
  can count these honestly as oracle-limited rather than full Passes — the coverage
  caveat never becomes a separate verdict.

The clamp lives in the runner; the subject never knows the oracle's precision. That
verifiable depth is carried as `oracle_limits` — a `Limits { None, None, gen_precision − guard }`
the loader provides (§5.1), the `− guard` dropping the rounding-guard digits — so the clamp is
`min(limits.max_precision, oracle_limits.max_precision)`.

### 4.4 Overflow contract

```rust
pub enum Overflow { Panic, Error, Absent, Infinity, Saturate, Truncate, Wrap }
```

The subject *declares* (per function, in `FnSupport`) how it behaves when a result
overflows the cell, and the runner *validates* that rather than skipping. Two families:

**Signals a non-value** — the validator just matches the `Computed` arm (§3.3) against the
policy; the four are 1:1 with the non-value `Computed` arms:
- **`Panic`** (the strict default for `decimal-scaled`) — the library crashes ⇒ `Computed::Panic`.
- **`Error`** — the library returns an error value ⇒ `Computed::Error`.
- **`Absent`** — the library returns no value (e.g. a checked `None`) ⇒ `Computed::Absent`.
- **`Infinity`** — the library returns `±∞` ⇒ `Computed::NonReal::Infinity`, with the sign
  matching the overflow direction.

**Produces a value** the validator must *derive* from the envelope (`min`/`max`):
- **`Saturate`** clamp to the nearest bound · **`Truncate`** keep the low digits · **`Wrap`**
  the two's-complement reduction.

For every policy, an **in-range** result that isn't a clean `Value` is a bug — the envelope is
what distinguishes a correct overflow signal from a wrongly-firing one, which is why an
independent magnitude bound is needed and the subject's own signal can't simply be trusted.

---

## 5. The run pipeline

The runner — the **`GoldenRunner`** trait — turns one subject's golden cases into
a `SubjectCollector`. It is generic over `DecimalSubject`, so `Value` is
monomorphised at the call site and never erased. (It is named *runner*, not
*tester*: validation is only one of the things it drives — it also loads,
schedules, times, classifies, and feeds the reporters.)

```
CaseLoader → [ for each case ] → ExecutionStrategy → ExecutionCollector
                                        │
                                        └── validators score the finished cell
```

### 5.1 Loader

`CaseLoader` is the extension point that yields a function's `GoldenCase`s
(`inputs` + the raw `output` string). It is the seam between the golden data and
the run; `FileLoader` implements it by reading and parsing the on-disk format, and
an in-memory loader (parsing nothing) serves tests. **Parsing is a private detail
of each loader**, not a shared stage — a `FileLoader` knows how to turn its file
format into `GoldenCase`s; another source turns its own input into the same.

```rust
pub trait CaseLoader {
    fn load(&self, func: Function) -> Cow<'_, [GoldenCase]>;
    fn oracle_limits(&self) -> Limits;   // the golden set's reach: { None, None, gen_precision − guard }
}
```

- **Takes `Function`** — cases are loaded per function.
- **Also declares `oracle_limits`** — the loader owns the golden set, so it owns the one
  fact that defines the oracle's reach: the precision the values were generated to (§8).
  It returns it as a `Limits` (unbounded magnitude, `max_precision = gen_precision − guard` —
  the rounding-guard digits decide the round, they are not verifiable depth) that never changes,
  built once and reused by reference in every `ValidationContext` (§5.3). Both `gen_precision`
  **and** `guard` are **read from the golden-file header** (the `#` metadata lines), not hardcoded — they
  are parameters of the data, so regenerating at a different precision or guard width flows
  through automatically.
- **Returns a reusable, re-iterable *view* — never a stream.** The runner consults
  the loader once per `(subject, function)`, and many subjects share a function's
  cases, so the call must re-yield the same cases every time; a one-shot iterator
  (a file stream exhausted after the first subject) is exactly what this forbids.
  `Cow<'_, [GoldenCase]>` lets a caching loader return `Borrowed` (its already-parsed
  cases, no clone) while a fresh-parse loader returns `Owned` — either way the caller
  iterates the slice as often as it needs. It stays dyn-compatible
  (`Box<dyn CaseLoader>`), which a bare `impl Iterator` return would break — and that
  bare iterator would itself be the stream to avoid.

The format `FileLoader` reads: one test per line, the inputs followed by the
output, split on `[ \t]+` (one or more spaces/tabs — *not* `\s+`). Skipped: blank lines,
`//` line comments, and `/* … */` block comments (which may span multiple lines). `#`
lines are **metadata** in `#key=value` form (e.g. `#gen_precision=1233`, `#guard=2`) — the
loader reads `gen_precision` and `guard` (and the provenance) from them, then skips them for
case parsing. A data line whose field count ≠ `arity + 1` is skipped. Each data field is a
plain `digits.digits` decimal string (§8):

```
#gen_precision=1233
#guard=2
// one test per line: inputs then output
input1 [input2 … inputN] output
```

### 5.2 Execution strategies — timing purity

`ExecutionStrategy` is *how* one input-set runs, and it is the **typed→string
boundary**: it parses, computes, catches panics, erases the closure's `Computed<Value>`
(§3.3) to a `Value`-free `Computed<String>`, and records an **`ExecutionResult`** in the
cell — the runner-level record of attempting it:

```rust
pub enum ExecutionResult {
    Computed(Computed<String>),  // the test ran — its (erased) outcome, incl. the subject's Panic/Timeout
    HarnessError(String),        // the harness itself failed — NOT the test (bad golden data / internal fault)
    Skipped,                     // the runner did not run it (unrepresentable input / unsupported)
}
```

`Computed` is the *test*'s outcome — *including* the subject's own `Panic` and `Timeout`, which
are the library's failures, not the harness's. `ExecutionResult` adds only the non-test events:
a `Skipped` cell, and a `HarnessError` — a fault of the harness itself (bad data, internal error),
never charged against the subject.

Only `Computed::Value(v)` touches the native type — stringified via `value_to_string`
*here*, where `Value` is still concrete; every other arm carries no `Value` and erases
trivially. So a subject's `Value` never crosses into the runner, collector, or
validators, and any number of libraries with different `Value` types compose.

- **`RunOnce`** — compute once, no timing (correctness).
- **`Timed`** — parse is hoisted *out* of the timed span; the timed loop calls
  only the `execute` closure under `std::hint::black_box` (defeats dead-code
  elimination / const-folding); the stringified value comes from the first run.
  So conversions are never measured — only the operation.

A panic in the *subject* (parse, compute, format) is caught (`catch_unwind`) and recorded as
`Computed::Panic` — the library crashed, a test failure. The overflow validator (§5.3) reads it:
an out-of-range panic under a `Panic` policy is the expected overflow; an in-range panic is a
bug. (A fault in the *harness's own* code is the separate `ExecutionResult::HarnessError`.)

### 5.3 Validators — composable, disjoint, self-gating

The runner holds a `Vec<Box<dyn Validator>>` and runs each over the finished cell;
each pushes its own verdict(s). An empty `Vec` is the "validate nothing" case
(timing-only runs). Only when `validators` is non-empty, the runner builds — once per cell — a read-only
`ValidationContext`, calls `validate(&ctx)` on each validator, and writes the returned
verdicts into the cell. Validators are **pure** — context in, `Option<Outcome>` out — so
the result can be borrowed into the context without a `&mut cell` alias, and each
validator is trivially testable.

```rust
pub struct ValidationContext<'a> {
    pub function:      Function,
    pub result:        &'a ExecutionResult,   // the runner's record (§5.2): a Computed outcome (incl. its Panic)
    pub golden_value:  &'a GoldenValue,       // the oracle's value, parsed once
    pub limits:        &'a Limits,            // the LIBRARY's envelope — subject.limits(golden)
    pub oracle_limits: &'a Limits,            // the ORACLE's envelope — {None,None,~1231}, from the loader (§5.1)
    pub capabilities:  &'a Capabilities,      // mode/overflow via .function(function); radix
}
impl ValidationContext<'_> {
    pub fn grade_precision(&self) -> u32 {
        self.limits.max_precision.min(self.oracle_limits.max_precision)
    }
    pub fn in_range(&self) -> bool { /* round(golden_value, grade_precision()) within limits [min,max] */ }
}
```

It stores only the irreducible inputs and **derives the rest**. The two limitations are
both `Limits`: `limits` is the *library's* representable envelope, `oracle_limits` is the
*oracle's* — unbounded magnitude (`None`/`None`, the oracle never overflows) with the
golden set's *verifiable* depth (`gen_precision − guard`, the rounding-guard digits removed) as its
`max_precision`, supplied once by the loader and reused for every cell (§5.1). `grade_precision` is then the `min` of the two reaches, and `in_range` is the
golden rounded to that depth tested against the *library's* `min`/`max` — so they are
methods, not stored fields that could drift. `mode`/`overflow`/`radix` come from
`capabilities`; the `result` is borrowed from the cell (the verdict sink). Future shared
analysis is a new method or field here, every validator untouched. The runner sets the
cell's `oracle_limited` from the same `limits.max_precision` vs `oracle_limits.max_precision`.

The validators have **disjoint domains** so any subset composes:

- **`RoundingValidator` — decimal compliance (the verdict).** In-range values
  only: is the produced value the correctly-rounded golden at `grade_precision`
  under the declared mode? Pass; else `WrongMode { used }` if it matches another
  mode's correct rounding, else `MisRounded { delta }`. A panic / no value is
  silent here (overflow's domain). This grades **decimal** correctness for every
  subject — including a binary one, which must meet decimal terms.
- **`OverflowValidator`.** Out-of-range only (it consults `in_range`, never a bit width):
  does the result match the declared `Overflow` policy (§4.4)? In-range ⇒ silent. For a
  non-value policy it matches the `Computed` arm (`Panic`↔`Panic`, `Error`↔`Error`,
  `Absent`↔`Absent`, `Infinity`↔a correctly-signed `±∞`); for `Saturate`/`Truncate`/`Wrap`
  it compares the produced value against the one derived from `Limits`. Any non-`Value` on an
  in-range cell is a bug.
- **`PrecisionValidator` — informational.** In-range only: the ULP distance of the
  produced value from the correctly-rounded golden. A measure, not a pass/fail —
  it ranks with Pass and never dominates a real failure.
- **`BinaryComplianceValidator` — separate report (planned).** Decimal compliance
  is the primary verdict, so a `Radix::Binary` subject that lands a decimal ULP
  off at its resolution edge is reported as such. A *separate*, opt-in validator
  may grade a binary subject on **its own binary grid** for a **different**
  report; it is gated to `Radix::Binary` subjects and needs the subject's binary
  resolution. It is the home for binary-grid grading, kept out of the decimal
  verdict path entirely.

### 5.4 Collector tree

The runner fills a tree; **reporting** (§5.6) reads it, and lives *outside* the runner:

```
RunCollector ⊃ SubjectCollector ⊃ FunctionCollector ⊃ ExecutionCollector
```

`ExecutionCollector` carries one cell's inputs, expected golden, status
(`Pending`, then an `ExecutionResult` — `Computed` / `HarnessError` / `Skipped`, §5.2), optional timing, the validators'
verdicts, and an `oracle_limited` flag (set when grading was clamped to the oracle's
depth, §4.3). `SubjectCollector` carries the subject's `Capabilities`
(so reporting has the `config` width/scale/etc. for the report).

Two `GoldenRunner` impls share this pipeline: `SeriesRunner` (serial) and
`ParallelRunner` (a work-queue over the subject's executions). They differ only
in scheduling; the per-cell work is identical.

### 5.5 Result taxonomy

```rust
pub enum Outcome { Pass, MisRounded { delta }, WrongMode { used },
                   Precision { ulps }, Skipped, Error { reason }, Timeout, Panic }
```

Severity order (worst dominates a cell): `Skipped` < `Pass`/`Precision` <
`WrongMode` < `MisRounded` < `Error` < `Timeout` < `Panic`. An `oracle_limited`
result (§4.3) is **not** a separate verdict — it stays `Pass`, qualified by a
cell-level flag (§5.4) the report reads, so the coverage caveat never mutates the
verdict taxonomy.

Collation is a **reporter** concern, not the harness API. The `TsvReporter` (§5.6) flattens
the run results into one row per cell — the subject's metadata (its `config`, §3.1), the
function, mode, outcome, and optional `precision` / `detail` (the offending input) / `nanos`
— so one file serves both the precision shootout and the timing bench, filterable by library
/ function / metadata / mode. Nothing here hardcodes `width`/`scale` as fields (they aren't
universal); they ride in the metadata, and in the file it is all strings anyway. Whatever
typed shape a `TsvReporter` builds internally to analyse faster is its own private business.

### 5.6 Reporting

The `RunCollector` is the run's raw result; turning it into human- or
machine-readable output is a separate, pluggable role — the same trait-not-flag
pattern as validators. A `Reporter` consumes the collection of run results and renders
one or more named outputs.

```rust
pub trait Reporter {
    fn report(&self, runs: &[RunCollector]) -> ReportArtifact;
}

pub struct ReportArtifact { pub outputs: Vec<ReportOutput> }
pub struct ReportOutput { pub name: String, pub content: String } // name = suggested path/section
```

- **Takes a collection of runs.** Each `RunCollector` is one run (every subject), so a
  reporter sees every library at once — and a `&[RunCollector]` lets one report span several
  runs (e.g. a correctness run + a timing run, or runs across settings) without re-plumbing.
- **Returns content; never writes files.** One report run yields several outputs
  (the precision shootout emits a per-library TSV *plus* a summary table *plus*
  fidelity grades), which `Vec<ReportOutput>` models directly. Returning rendered
  text keeps a reporter **pure and testable** — a test asserts on `content`; the
  caller (a test or bench) decides whether to write each output to disk, print it,
  or assert on it. IO stays at the edge. Rendering is pure formatting, so it is
  infallible (no `Result`).
- **The TSV is just one report.** `TsvReporter` is the `Reporter` that emits the
  flattened per-cell TSV (its internal row shape is its own concern); the precision table,
  fidelity grades, timing comparison, and binary-compliance report (§5.3) are
  sibling impls.

---

## 6. Rounding radix

`decimal-scaled` and every decimal peer round on the **decimal** grid, which is
the grid the golden values and grader use. A **binary** fixed-point library (e.g. a
Q-format type) rounds on a `2^-k` grid that does not coincide with decimal
half-points. The difference is invisible where the grading scale is far coarser
than the binary grid, and reaches **up to one decimal ULP at the binary library's
own resolution edge** (a radix-grid / double-rounding effect).

**The verdict stays decimal.** This crate tests *decimal* compliance, so a binary
library is judged on decimal terms — `Radix::Binary` is a **verdict-neutral
annotation** carried in `Capabilities` purely so the report can *explain*
edge-of-resolution discrepancies (rather than burying that in a comment). Grading
a binary library on its own grid is a *separate* concern: a separate validator and
a separate report (§5.3), never a branch inside the decimal grader.

---

## 7. Oracle generation and licensing

### Process isolation is a license boundary

All oracles live **only** inside the generation/validation tool (a separate
process) which writes the `digits.digits` golden files; the Rust crate reads
**only text**. Calling a separate program (or a dynamically-linked binding in a
separate process) and reading its text output is arm's-length aggregation — it
does not make this crate a derivative of the oracle.

- **`decimal-scaled-golden`'s `Cargo.toml` lists no oracle** — zero copyleft in
  the crate's dependency tree.
- The golden data is a **maintainer/CI build artifact** (committed text), never shipped
  to end users — so LGPL's relink obligation never attaches and no oracle is
  bundled.
- Standing rule: never copy or adapt LGPL/GPL **source** — implement from the
  *paper* and cite the paper. Oracles are only ever *called* at arm's length.
- LGPL bindings (`python-flint`, `gmpy2`) are **optional pip packages the user
  installs**, imported **lazily** by our (MIT/Apache) adapter — a "work that uses
  the Library", not a derivative — never committed or bundled. The tool runs on
  `mpmath`/`sympy` without them.
- Competitor adapters that link competitor crates live in a separate,
  feature-gated comparison crate, isolating any copyleft competitor.

### Pluggable oracles — one interface, role assigned by config

A cross-platform Python tool. One `Oracle` interface; each oracle is one adapter:

```python
class Oracle:
    def name(self) -> str: ...
    def supports(self, func) -> bool: ...                  # not every oracle covers every function
    def value(self, func, inputs, precision) -> str: ...   # high-precision digits.digits
```

Execution is identical for generator and validator — an oracle does not know its
role. Config assigns roles: the **generator** oracle's output becomes the stored
golden value; each **validator** oracle independently recomputes and is compared.

```toml
generator  = "mpmath"
validators = ["flint", "mpfr"]   # 0..N independent cross-checks
```

Available oracles (each usable as generator *or* validator):

- `mpmath` — BSD; full coverage; default **generator**.
- `sympy` — BSD; symbolic cross-check **validator**.
- `flint` / Arb (`python-flint`) — LGPL; strong **validator** (separate process).
- `mpfr` (`gmpy2`) — LGPL; **validator** (separate process).

**Acceptance:** a generated value is kept only if every configured validator that
`supports` the function agrees to the required precision; a disagreement
flags/drops the value with the input logged. A second mode re-validates the
*committed* golden values against the configured oracle set.

---

## 8. Golden value representation

- **One value per `(function, input)`, for all widths and scales.** Not a
  per-`(width, scale)` table — the expected result for any tier and scale is
  *derived* by rounding the single stored value to that target.
- **Stored as a decimal string**, not a numeric — width-independent,
  human-readable, greppable, and directly re-checkable by an alternate oracle.
- **Precision `max_decimal_width + guard`** (`guard = 2`). Two digits beyond the widest
  tier's maximum scale. The widest tier is the 4096-bit `Int` (D1232, `MAX_SCALE =
  1231`), so values run to ≈ **1233** fractional digits, by standard truncation. Both
  `gen_precision` and `guard` are recorded in the golden-file `#` metadata header (§5.1), so the
  loader reads them rather than assuming them.
- **Format: plain `digits.digits`.** No sentinel, no separator. The guard
  digits beyond the widest scale carry exactly the round-half / tie / exactness
  information needed to round correctly to any tier and scale:
  - a value that **terminates before** `max + guard` is **exact** (finite);
  - a value that **runs to** `max + guard` carries a **residual** below the widest
    scale (not exact);
  - at the widest scale, a terminated `…5` is an **exact tie**, while `…5x` is
    **above half**.

The loader derives a target's expected value by checking the integer-digit count
fits the target and then shortening + correctly rounding to the target scale,
with the guard digits deciding ties at the widest scale.

---

## 9. Module layout

One folder per **extension point** — a trait and its implementations — even when a
role currently has a single impl. The boundary is the trait, not the file count:
the trait lives in its own file, each implementation in its own file, and every
folder's `mod.rs` is re-export glue only. Shared leaf types and internal helpers
stay as top-level files.

```
src/
  subject/            # DecimalSubject — the library-facing interface
    subject.rs        #   the trait
    capabilities.rs   #   Capabilities, FnSupport, Radix
    limits.rs         #   Limits
    overflow.rs       #   Overflow

  loader/             # CaseLoader — yields GoldenCases from a source
    loader.rs         #   the trait + GoldenCase (its return type)
    file.rs           #   FileLoader: reads + parses its file format (parsing is private here)
    value.rs          #   GoldenValue (+ round-to-scale derivation)

  execution/          # ExecutionStrategy — how one input-set runs
    strategy.rs       #   the trait
    run_once.rs       #   RunOnce
    timed.rs          #   Timed

  runner/             # GoldenRunner — drives a subject over the golden cases
    runner.rs         #   the trait
    series.rs         #   SeriesRunner
    parallel.rs       #   ParallelRunner

  validators/         # Validator — scores a finished cell
    validator.rs      #   the trait
    rounding.rs       #   RoundingValidator (decimal compliance)
    overflow.rs       #   OverflowValidator
    precision.rs      #   PrecisionValidator
    binary.rs         #   BinaryComplianceValidator (planned, §5.3)

  reporting/          # Reporter — renders runs into named outputs
    reporter.rs       #   the trait + ReportArtifact
    tsv.rs            #   TsvReporter — the flattened per-cell TSV
    precision.rs      #   the precision-shootout table
    fidelity.rs       #   the fidelity grades
    timing.rs         #   the timing comparison

  support/            # shared leaf types + helpers — no extension seam of their own
    collector.rs      #   the RunCollector tree — written by execution, read by reporting
    function.rs       #   Function
    rounding.rs       #   RoundingMode
    outcome.rs        #   Outcome
    string_decimal.rs #   internal decimal-string arithmetic
  lib.rs
```

Every folder is exactly one extension point (a trait + its impls); `subject/` and
`loader/` additionally carry the data types their trait owns (`Capabilities` /
`Limits`; `GoldenValue` / `GoldenCase`). The items with no extension seam of their
own — the `collector` tree (the run's result data, not a role), the leaf types
(`Function`, `RoundingMode`, `Outcome`), and the internal `string_decimal` helper —
gather under `support/`; `lib.rs` re-exports them at the crate root so their paths
stay flat (`crate::function`, `crate::collector`, …).

---

## 10. Roadmap

Planned work, not yet built. The hook for each already exists, so adding it does
not disturb the pipeline.

### Timeout guard

A pathological subject or input — an infinite loop, or a catastrophically slow
path on a deep wide tier — must not hang the whole run. The guard bounds **each
execution** with a time budget and records `Computed::Timeout(budget)` for that cell
instead of blocking, then continues.

- **Mechanism.** The `GoldenRunner` runs a cell's `execute` on a worker (the
  `ParallelRunner` already owns a worker pool) under a watchdog. If the budget
  elapses, the cell is marked `Timeout` and the run moves on. Requires
  `DecimalSubject: Sync` and the inputs/closure to be sendable to the worker.
- **Honest limitation.** Rust cannot safely *kill* a running thread, so a stuck
  worker is **abandoned, not terminated** — its result is discarded and the thread
  is reaped at process exit. Acceptable for a test/bench harness; stated so it is
  not mistaken for a hard cancel.
- **Scope.** Applies to both correctness and timing runs (a timing run especially
  wants to bound a runaway). The budget is configurable (global, or per
  function/width for the deep wide tiers).
- **Readiness.** `Computed::Timeout` (§3.3) and the `Outcome::Timeout` verdict already
  exist, so only the runner-side watchdog is new.

### Named worker threads

The `ParallelRunner` spawns its workers anonymously, so a caught panic prints
`thread '<unnamed>' panicked …` — useless for telling *which* worker (and thus
which cell) tripped it. Each worker should be spawned via
`std::thread::Builder::name(…)` with a stable identifier (e.g. its worker index,
or the `(subject, function)` it is draining) so panic output and any future
timeout diagnostics name the culprit. Purely a diagnostics improvement — no
change to scheduling or verdicts.

### Binary-grid compliance

`Radix::Binary` is carried today only as a verdict-neutral annotation (§6). A
future `BinaryComplianceValidator` (§5.3) grades a binary subject on its **own**
`2^-k` grid for a separate report — which additionally requires the subject to
declare its binary resolution `k`. Decimal compliance stays the primary verdict;
this never branches the decimal grader.
