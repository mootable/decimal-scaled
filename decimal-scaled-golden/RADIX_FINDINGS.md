# Radix-aware golden — the deep dual-radix model

This branch (`feat/radix-golden`) builds radix-aware golden grading on the **settled
final model** (owner, 2026-06-13). It is a REVIEW branch — backward-compatible (a no-op
on the current untagged corpus), not yet a full tagged regen (that is the coordinator's
heavy run).

> **Supersedes** the earlier "a precision-free `2:` value is 0% / owner-decision-needed"
> framing. That conclusion was WRONG: it measured a *precision-free* `2:` value (which
> indeed carries no signal) instead of a deep value re-rounded **per subject** to that
> subject's own width. The correct, divergence-carrying numbers are below.

## The model — one deep ground truth, in BOTH radixes

We do **not** fit the golden to the libraries. The golden is the single DEEP ground
truth, stored in both radixes; we measure how close each library gets — **at its own
advertised precision** — to that golden.

1. **One deep value per line, two radixes.** Each golden output field is
   `10:<decimal>,2:<hexfloat>`:
   - `10:<decimal>` — the true value in decimal at `gen_precision` (≈1233 fraction
     digits). Today's value, unchanged.
   - `2:<hexfloat>` — the SAME true value in binary at **`bin_precision` = 2¹² + 8 =
     4104 significand bits**, stored EXACTLY as a C99 `%a`-style hex-float
     `[-]0x<hex-mantissa>p<exp>` (value = `(-1)^sign · mantissa · 2^exp`; the exponent
     carries the magnitude, so there is no int/frac split). Hex — not a decimal
     rendering — so a binary grader rounds it with ZERO conversion loss.
   - **Full grid precisions live in the file HEADER**, never per line:
     `#gen_precision=1233` (decimal) beside `#bin_precision=4104` (binary). One deep
     `2:` value serves EVERY binary subject.
   - **Backward-compat:** a field with no `:` is today's single decimal value; the
     untagged corpus is unchanged and grades identically.

2. **Grading rounds the deep golden DOWN to the SUBJECT's advertised precision, in the
   subject's radix.**
   - A **decimal** subject grades against `10:` rounded to its scale — today's path,
     unchanged.
   - A **binary** subject grades against the `2:` hex-float **rounded to its mantissa
     width, in binary** (round-half-to-even — the IEEE grid's rounding), compared in
     binary to the subject's own output parsed at that width. Each binary subject
     re-rounds the SAME deep value to its OWN width: f64 → 53, f32 → 24, g_math
     (Q128.128) → 128.

3. **This re-grades exactly the cells where binary-correct ≠ decimal-correct** — real
   divergence, NOT zero:

   | binary grid (subject) | mantissa bits | diverges from the decimal-correct verdict |
   |---|---|---|
   | f64 | 53 | **~14%** |
   | f32 | 24 | **~7%** |
   | g_math (Q128.128) | 128 | **~4%** |

   These cells are intrinsically grid-specific (f64, f32 and g_math diverge on
   different cells at different depths), which is why a single *precision-free* `2:`
   value could never carry the signal — but ONE deep value re-rounded per subject does.

4. **Precision lives on the SUBJECT.** `DecimalSubject::mantissa_bits() -> Option<u32>`
   (default `None` = a decimal subject, graded in decimal). Override: F64 → `Some(53)`,
   F32 → `Some(24)`, GMath → `Some(128)`. The runner picks the binary path when
   `storage_radix() == Binary` AND `mantissa_bits()` is `Some` AND the selected `2:`
   value parses as a hex-float — so the untagged corpus (no `2:` value) and every
   decimal subject grade in decimal, unchanged.

## What this branch builds

- **Generator (`oracle/`):** `binary_hexfloat()` in the mpmath adapter computes the
  true value at a guarded higher precision then rounds it to exactly `bin_precision`
  bits (mpmath `mpf.man`/`.exp` via `_mpf_`), formatted as the exact hex-float. The
  generator now emits `10:<dec>,2:<hexfloat>` per line and `#bin_precision=4104` in the
  header. The decimal generator/validator path is UNTOUCHED — the `2:` field is purely
  additive (a binary-compute failure falls back to a bare decimal value, never dropping
  a decimal-valid line). `revalidate` reads the `10:` value out of a tagged field.
- **Hex-float parser + binary round (`src/loader/hexfloat.rs`):** `HexFloat::parse`
  reads `[-]0x<hex>p<exp>` exactly (the ~4104-bit significand is held as `u64` limbs);
  `round_to_bits(k)` rounds it to `k` significand bits, round-half-to-even (tested hard
  at k = 24/53/128: round boundaries, exact ties, carry/renormalise, sign, exact powers
  of two). `RoundedBinary::to_f64`/`to_f32` reconstruct the exact native float.
  Dependency-free on purpose — the harness is library-AGNOSTIC (see the note below).
- **Binary grading path (`src/runner/mod.rs`):** a branch in `run_cell` that, for a
  binary subject with a mantissa width and a `2:` hex-float, rounds the deep golden to
  that width and compares. f64 (53) and f32 (24) are graded NATIVELY: the rounded deep
  value IS an `f64`/`f32`, and the subject's shortest-round-tripping decimal output
  recovers its exact float via `str::parse`, so a bit-exact comparison decides Pass /
  MisRounded.

## Demonstration (real generated data)

`demo/radix_divergence/sin.golden` (from `demo/lead/sin.lead`) carries five `sin`
cells where the correctly-rounded f64 differs from the decimal-correct value at f64's
grade depth. The runner test `binary_correct_f64_passes_2_but_fails_10` takes the
`sin(0.1665)` cell:

- true value `0.16573177318480250863…`; correctly-rounded f64 `0.1657317731848025`
  (= `round_to_53(the 2: golden)`, and what `f64::sin` yields here);
- **binary grading → Pass** (the f64 IS the deep value rounded to 53 bits);
- **decimal grading → WrongMode, NOT Pass** (the deep golden rounds half-to-even to
  …803, but the f64's shortest decimal is an exact half tie → …802, a directed-rounding
  match) — the false decimal miss that binary grading corrects.

## STOP-CLAUSE — g_math's 128-bit compare is DEFERRED

f64 + f32 are implemented FULLY (native parse → round → compare, with hard unit tests +
the worked demonstration). The g_math **128-bit** end-to-end compare is NOT shipped: it
needs a correctly-rounded **decimal→128-bit-binary** parse of g_math's decimal output
(std does this for f64/f32 via `str::parse`, but there is no native 128-bit-binary type
to lean on), which is intricate wide-mantissa numerical code. Per the branch's
stop-clause ("surface, don't bodge"), a `mantissa_bits == 128` cell currently **abstains**
(`grade_binary` returns `None`) rather than ship an unverified verdict. `GMath` still
declares `mantissa_bits() == Some(128)` so the path engages the moment that compare
lands. The shared `round_to_bits` already rounds to 128 bits correctly (tested), so the
remaining work is only the subject-output → 128-bit-binary parse.

Also deferred (follow-up, not blocking): the binary branch produces only the rounding
verdict and returns, so the `OverflowValidator` does not run on a binary cell whose
result is non-finite. On a real `2:` corpus an out-of-range binary cell would want the
overflow check via the decimal co-value (`10:`); inert today (the untagged corpus never
reaches the binary branch).

## Architecture note — the harness stays library-AGNOSTIC

The task brief suggested using `decimal_scaled::{Int,Uint}<N>` for the wide mantissa
and stated the harness "already depends on decimal_scaled". It does **not**:
`decimal-scaled-golden` is the agnostic harness (zero-dep), and the design rule
(`golden-competitors/Cargo.toml`, the `golden-subjects` skill) is that the core library
is a dependency only of the competitor crate, **never of the agnostic harness**. Adding
it here would violate that invariant. The ~4104-bit significand is therefore a small
self-contained `u64`-limb helper in `hexfloat.rs` (heap is fine — this is a dev/test
crate, off the runtime `src/**` no-heap path). Surfaced for the owner: if you would
rather take the `decimal_scaled` dependency, the limb helper is the single swap point.
