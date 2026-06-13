# Radix-aware golden — status & the decision this branch surfaces

This branch (`feat/radix-golden`) builds the **harness** for radix-aware golden grading and
surfaces an empirical finding that needs an owner decision before a real tagged corpus can
be generated. It is a REVIEW branch — backward-compatible (a no-op on the current untagged
corpus), not intended to merge as-is.

## What is built and gated (the harness — sound, reviewable)

- `Radix::{tag, from_tag}` — base tags `10` / `2` only (no precision, no fixed/float kind).
- `DecimalSubject::storage_radix()` (drives value selection) + `rounding_radix()` (defaults
  to storage; override hook).
- `loader/radix.rs::select_radix_output(output_raw, radix)` — the value-chooser: a no-`:`
  field is today's single value (returned verbatim → the untagged corpus grades identically);
  a `10:v,2:v` field selects by the subject's storage radix, falling back to the untagged
  catch-all. Unit-tested (no-tag, tagged, catch-all, malformed).
- `runner::run_cell` — select-then-parse via `subject.storage_radix()`; classifies the
  **chosen** value in `limits()` (regression-tested). Validators unchanged.
- Subjects: f64 / g_math → `Binary`; new **F32** subject (in the precision shootout per
  owner Q-C); radix-aware `limits()` reach (g_math 30→38 digits, f32 ≈7).

All gated: `cargo check` clean; `cargo test -p decimal-scaled-golden radix` → 7 pass;
`competitors_proof` f32 → pass>0. The runtime crate (`src/`) is untouched.

## The finding (why no corpus is generated yet)

An mpmath measurement over 140 (function, input) cells — does `f(x)` rounded to a binary
grid, rendered to that grid's reach, differ from the decimal-correct value there?

| binary grid (subject) | reach | diverged from decimal |
|---|---|---|
| 53-bit (f64) | 16 digits | **14%** |
| 24-bit (f32) | 7 digits | **7%** |
| 128-bit (g_math) | 38 digits | **4%** |
| a single deep, precision-free `2:` value | 38 digits | **0%** |

**A precision-free `2:` value carries no signal (0%).** A deep binary value rendered to any
subject's grade depth is byte-identical to the decimal value there, so the literal "one
binary value, no precision" grammar yields an **all-untagged corpus** — the feature would
change zero verdicts. The real divergence is intrinsically **grid-specific**: f64, f32, and
g_math diverge on different cells at different depths, so **no single `2:` value is correct
for more than one binary subject at a time**. (Corollary: only ~4% of g_math's cells diverge
by radix at 38 digits — most of its shootout mis-rounds are a compute-accuracy gap, not a
radix artefact.)

## The decision (owner)

To generate a meaningful tagged corpus the generator must compute each `2:` value at a
**specific binary grid**, which IS a precision:

- **(a) minimal precision tag — RECOMMENDED:** `2/53:` (f64), `2/24:` (f32), `2/128:`
  (g_math). The only fully-correct option; tiny grammar extension (the chooser already
  splits on `:`), and the harness already grades each subject at its own reach. Reverses the
  "no precision" simplification — which is why it is the owner's call.
- **(b) one canonical grid behind a bare `2:`:** honors "no precision" but mis-grades the
  non-matching binary subjects. A compromise, not correct.
- **(c) drop it:** with no precision the corpus is all-untagged and radix-awareness does
  nothing; keep the harness dormant.

I held off generating any corpus because (b)/(c) bake in a wrong or pointless result and (a)
reverses a decision only the owner should reverse.
