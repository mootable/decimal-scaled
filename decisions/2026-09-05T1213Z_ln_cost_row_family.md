# The cost of one narrow `ln` — contradictory tables

**Date:** 2026-09-05T12:13Z
**Parties:** coordinator; Op5-Delphine
**Status:** RESOLVED — both readings correct; a retraction was itself retracted

## Raised

Two contradictory tables for "the cost of one natural log", both derived from
`results/timing/bbc_medians.tsv`.

- **Delphine** held that narrow Series `ln` is *cheaper* than wide Tang `ln` at every comparable
  scale, and on that basis **retracted** her own earlier thesis that the narrow `log` path was
  slow because it runs Series.
- **Coordinator** held that the same subtraction gave the opposite ordering, so the retraction
  was premature.

## Methods

Both parties subtracted a "hard" (non-degenerate) operand row from its plain row at the same
`(width, scale)`, calling the difference one real `ln`.

- **Coordinator:** committed `bbc_medians.tsv` at `e1dd5ae9`, column 5 (`branch_ns`), rows
  **`ln@hard − ln`**.
- **Delphine:** the same file and column, rows **`log2@hard − log2`** — *not stated at the time*.

## Argued

1. Coordinator sent a table of figures without its derivation. Withdrawn as anchoring; the
   method was supplied instead.
2. Coordinator tested four explanations for the divergence — every snapshot in history, both
   columns, raw values, no-subtraction — and eliminated all four, narrowing to "the wide figures
   come from a source not in git".
3. That narrowing produced a **false positive** the coordinator reported as a finding: an older
   snapshot appeared to "match" when two of five scales were merely close and one was 40% apart.
4. Delphine identified the actual cause in one pass: different row families, hers unnamed.
5. Delphine then produced a **better objection to her own position** than the coordinator had —
   her retraction compared two different cells, which `bench-branch-compare.yml:111` declares an
   invalid ratio under `group: cell`.
6. Delphine self-audited again after the dispute had resolved in her favour, finding her headline
   sentence false against her own numbers at the one genuinely shared scale (s28: narrow 3317 vs
   wide 2935 — narrow *more* expensive), and that several "comparable" pairs compared different
   scales.

## Agreed

- Both extractions were correct. `ln` and `log2` route different engines at the narrow tiers, so
  the two readings are complementary, not contradictory.
- **The retraction is withdrawn; the original thesis stands.** Within-cell and same-machine,
  narrow Series ÷ narrow Tang is 1.51–1.78× at s9–s37, agreeing in direction and order with the
  in-tree same-width A/B at 1.83–2.25×.
- **Neither party quotes a cross-width figure again**, including the 4.10× that opened the
  investigation. Those are cross-cell ratios the harness declares invalid.
- The `@hard − plain` subtraction is recorded as a **coarse directional cross-check at s9+**, not
  a measurement.

## Why

- **Validated:** the within-cell, same-machine comparison. Both parties can reproduce it from one
  committed file, and an independent in-tree A/B agrees.
- **Discarded — the cross-width comparison.** Not because the arithmetic was wrong but because
  the harness defines a cell as `(width, scale)` and states that ratios across cells are invalid.
  A correct subtraction on both sides still yields two numbers from different machines.
- **Discarded — the subtraction as a measurement.** It inverts at s0, where it subtracts a
  structurally different early-exit arm from real work; it is an upper bound carrying whatever
  extra walker and pin work the hard operand triggers; and the cross-op version conflates engine
  with op-specific machinery. Its own author downgraded it.
- **Discarded — the coordinator's "older snapshot" explanation.** A near-miss, not a match.

## Notes

The unstated row family was the whole of it. Had either party named its rows when presenting the
figures, this would not have been a dispute.

Left open, not part of this decision: `log` carries a ~3,900 ns narrow-specific residue at
D38 s28 that is not the `ln` core, with `log_exact_int_pin` the leading suspect.
