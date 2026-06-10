---
name: golden-subjects
description: Use when writing or maintaining a DecimalSubject adapter or any golden-harness integration — adding a subject for a new decimal library to golden-competitors, fixing an existing adapter's limits/rounding/overflow declarations, or wiring a library against the decimal-scaled-golden golden set. Covers the typed-vs-erased choice, the Capabilities/FnSupport contract, value-aware limits, round-trip rules, panic capture, the gotchas hit building the seven competitor adapters, and how to validate a new subject.
---

# Golden subjects — adding a decimal library to the harness

The recipe for binding a NEW decimal library to the `decimal-scaled-golden`
harness as a `DecimalSubject`. The working references are
`golden-competitors/src/lib.rs` (seven third-party adapters, each comment a
lesson paid for) and `decimal-scale-test/src/lib.rs` (the erased decimal-scaled
subject). The trait and pipeline are specified in
`decimal-scaled-golden/ARCHITECTURE.md`.

A subject is **pure**: it parses, computes, formats, and describes its
envelope. It never skips, never catches, never judges its own output — the
runner owns skip/catch/verdict. Every `Computed` arm you return must be a
faithful fact about what the library did.

---

## 1. Typed or erased?

- **Typed adapter** (`type Value = TheLibrary'sType`) — the default. One struct
  per library; for a variable-precision library one instance covers everything;
  for a fixed-width library you get one subject per `(width, scale)` cell. All
  seven competitor adapters are typed. The harness pipeline (runner →
  collectors → validators) monomorphises once **per distinct `Value` type** —
  fine for a handful of types.
- **Erased adapter** (`type Value = String`) — when one library contributes
  MANY concrete monomorphic types (decimal-scaled: 88 band-edge `D<W><S>`
  cells). A typed adapter per cell would monomorphise the whole pipeline 88
  times; the erased `DsSubject` carries `(width, scale, mode)` as runtime
  fields, keeps the string as the carried value, and fans out to the concrete
  type only inside `execute`/`limits` via a `(width, scale)` match (the
  `cells!` macro). Cost: parse moves inside the (timed) `execute` closure, so
  an erased subject is for correctness gates, not pure op-timing.

## 2. Declare `Capabilities` honestly

`capabilities()` returns the per-function support map; **presence in the map IS
the support claim** — the runner runs nothing for an absent function.

- **Only declare what the library genuinely provides.** rust_decimal has no
  cbrt/exp2/log2/inverse-trig/hyperbolics — those are simply not in its map.
- **Omit functions that cannot terminate on this data.** An
  arbitrary-precision library asked for `exp` of a huge golden input computes a
  number of unbounded size and never returns — bigdecimal and dashu-float
  deliberately do NOT declare the exp family. Declaring it would hang the gate,
  not fail it.
- **`FnSupport.mode` is the library's real rounding mode**, per function. Read
  the library's kernels/docs, do not assume: rust_decimal is half-to-even
  (banker's) uniformly; fastnum, decimal-rs, and dashu-float (`DBig` =
  `HalfAway`) are half-away-from-zero; g_math's decimal compute path is
  half-away-from-zero. A wrong declaration shows up as a wall of `WrongMode`.
- **`FnSupport.overflow` is what the library actually does out of range**:
  `Absent` for checked-`None` (rust_decimal, decimal-rs), `Infinity` for ±inf
  signalling (f64), `Panic` where it genuinely crashes. fastnum's default
  context TRAPS exceptional conditions, so under a debug `cargo test` overflow
  panics rather than yielding ±infinity — `Panic` is the faithful policy there.
- `config` is report-only metadata (width/scale/mode strings); the runner never
  reads it.

## 3. `limits()` — the envelope, sized per VALUE

`limits(value: &str) -> Limits` is called with the specific value being
classified (an input when filtering, the golden output when range-checking), so
the envelope can — and for float-backed/fixed-significant libraries MUST —
depend on the value.

- **`min_value`/`max_value`**: the magnitude bounds as decimal strings; `None`
  = genuinely unbounded on that side (bigdecimal, dashu-float). decimal-rs's
  bound is derived from its representation: 38 nines followed by 126 zeros.
- **`max_precision` (fractional grading depth) must be value-aware for
  fixed-significant libraries.** A library with S significant digits holds only
  `S − int_digits(value)` fractional places once the integer part grows. A flat
  cap over-claims on large-magnitude results and manufactures FALSE
  `MisRounded`s — this bit rust_decimal (28), f64 (16, capped at 15
  fractional), fastnum D512 (`min(50, 154 − int_digits)`), and decimal-rs (38).
  Use the `int_digits()` helper pattern: significant integer digits of the
  value string, sign and leading zeros stripped.
- **`max_significant_digits`**: the total-figure ingestion cap — an input
  literal spanning more figures than the mantissa holds cannot be parsed
  exactly, so the runner must skip it (`Some(28)` rust_decimal, `Some(16)` f64,
  `Some(154)` fastnum, `Some(38)` decimal-rs). `None` for arbitrary-precision
  subjects and for fixed-scale subjects whose magnitude envelope + scale
  already bound everything exactly (the decimal-scaled subject).

## 4. String round-trip rules

The grader parses only plain `[-]digits[.digits]` text.

- **`value_to_string` must emit plain decimal.** Several `Display` impls emit
  scientific notation for small/large magnitudes (fastnum, dashu-float) — wrap
  the output in an `expand_scientific` conversion (see
  `golden-competitors/src/lib.rs`; it is a no-op on plain strings, so it is
  safe to apply to any library that *might* emit exponent form).
- Trailing/leading zeros are irrelevant (the grader truncates at the graded
  depth), so `normalize()`/`reduce()` style canonicalisation is fine.
- **`string_to_value` panics on what it cannot parse** — with the offending
  literal in the message. The runner pre-filters unrepresentable inputs via
  `limits`; anything that still fails to parse is a real defect and the panic
  is the right signal.
- **Working precision is set at parse time where the library needs it.**
  dashu-float rounds each binary op to `max(lhs, rhs)` precision, so inputs
  are lifted to a high working precision (1280 significant digits) in
  `string_to_value`; a 34-digit working precision silently truncated every
  large-magnitude result into a false `MisRounded`.

## 5. `execute` — the compute closure

`execute(func, mode, overflow)` curries everything into a
`Fn(&[Value]) -> Computed<Value>` closure over pre-parsed inputs — no parse, no
format inside (timing purity). Map outcomes faithfully:

- A real finite result → `Computed::Value`.
- A checked op's `None` → `Computed::Absent`.
- NaN / ±infinity / an imaginary answer (sqrt of a negative) →
  `Computed::NonReal(..)` — classify, do not panic. Guard domain edges the
  library would panic on if a peer maps them to NonReal (dashu's `ln(x <= 0)`
  is guarded to `NegativeInfinity` for parity with the other adapters).
- A function you did not declare can defensively return
  `Computed::Error("lib: unsupported ..")` in the match arm — it never runs.
- **Panics are caught by the harness** (`catch_unwind` in the execution
  strategy) and recorded as `Computed::Panic` — then judged against the cell's
  range and the declared overflow policy. So a strict library's overflow panic
  needs no special handling in the adapter.
- **Compute precision is the adapter's job where defaults are too shallow.**
  bigdecimal's DEFAULT context is 100 significant digits — far short once a
  result carries hundreds of integer digits — so sqrt/cbrt run under an
  explicit high-precision `Context` (1320 sig digits), and its `/` operator
  (pinned to the default context) is replaced by multiply-by-
  `inverse_with_context`.
- A lazy-evaluation library (g_math's `LazyExpr`) builds the op in `execute`
  and realises it in `value_to_string`; an evaluation failure there panics and
  is still caught as `Computed::Panic` — which must then match the declared
  overflow policy.

## 6. Validating a new subject

Use the **proof-test pattern** (`golden-competitors/tests/competitors_proof.rs`),
NOT the 0-bad gate: a third-party library is *expected* to mis-round values —
that is the comparison. Run the subject through a `ParallelRunner` with
`RoundingValidator { gen_precision }` + `OverflowValidator` over its declared
functions, tally pass/skip/bad per cell, print the split, and assert only that
`pass > 0` (it computes a meaningful share correctly).

Read the split critically before trusting it:

- **bad ≈ everything** → a broken envelope or mode declaration (flat
  `max_precision` over-claiming, wrong rounding mode, scientific-notation
  output the grader cannot parse) — an adapter defect, not the library's.
- **skip ≈ everything** → `limits` too tight, or `max_significant_digits`
  rejecting inputs the library could take.
- **hangs** → you declared an exp-family function on an unbounded library.

The 0-bad gate (`decimal-scale-test/tests/golden.rs`, `RunSummary` asserts
`bad == 0 && panic == 0 && pass > 0`) is reserved for the library whose
correctness contract is under test.
