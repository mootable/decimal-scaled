# decimal-scaled-golden

A standalone, **library-agnostic** golden-validation harness for decimal
libraries, together with the committed golden data and the Python oracle tool
that generates and cross-validates it. Any decimal library — not just
`decimal-scaled` — can be validated against the same golden set by implementing
one trait.

The crate has three parts:

1. **The harness** (`src/`) — the `DecimalSubject` trait a library implements,
   plus loaders, runners, execution strategies, validators, and reporters. Zero
   dependencies by default (the optional `bench` feature pulls criterion for the
   `CriterionStrategy`).
2. **The golden data** (`golden/`) — one `.au` file per function (28
   functions: `sqrt`, `cbrt`, `exp`, `ln`, `log2`, `log10`, `exp2`, the trig /
   inverse-trig / hyperbolic / inverse-hyperbolic set, `log`, `atan2`, `powf`,
   `hypot`, and `add`/`sub`/`mul`/`div`/`rem`), each value stored once to 1233
   fractional digits and reusable at every width and scale.
3. **The oracle tool** (`oracle/`) — a Python generator/revalidator that
   computes every value with a per-function generator oracle and cross-checks it
   against every other available oracle before a line is accepted.

`decimal-scaled-golden`'s design is specified in [`ARCHITECTURE.md`](https://github.com/mootable/decimal-scaled/blob/main/decimal-scaled-golden/ARCHITECTURE.md);
this README is the practical front door.

## Getting the golden data

The published crate ships the **harness only** — the golden set is ~130 MB, far
over the crates.io package limit, so it is not bundled. Two ways to get it:

- **`UrlLoader`** (the `net` feature) fetches each function's file on demand over
  HTTPS and caches it locally — zero setup. It defaults to the matching
  `v<major>.<minor>` release tag of this repo (e.g. `v0.5`), so you always get
  your minor's latest, non-regressing corpus:

  ```toml
  decimal-scaled-golden = { version = "0.5", features = ["net"] }
  ```

  ```rust
  // Fetches from the repo at the default tag, caching under the system temp dir.
  // Override the location with the DECIMAL_SCALED_GOLDEN_CACHE env var, or pin a
  // ref with UrlLoader::from_ref("v0.5").
  let loader = decimal_scaled_golden::UrlLoader::default();
  ```

- **`FileLoader`** reads a local copy — clone the repo (or download the
  `decimal-scaled-golden/golden/` directory) and point it at the path. Best for
  offline / CI use where you vendor the data.

## The harness

```
CaseLoader → GoldenRunner → ExecutionStrategy → validators → collectors → reporters
```

- **`DecimalSubject`** (`src/subject/`) — one library pinned to one
  `(width, scale)` cell. It declares `capabilities()` (identity, radix, and a
  per-function `FnSupport { mode, overflow }` map), converts strings to and from
  its native `Value`, describes its representability envelope via
  `limits(value) -> Limits`, and curries `(function, mode, overflow)` into a
  compute-only closure via `execute` so a timing run measures exactly the
  operation. The subject is pure: it parses, computes, formats, and panics on
  input it cannot take — skip and catch policy live in the runner.
- **`CaseLoader`** (`src/loader/`) — yields a function's `GoldenCase`s.
  `FileLoader` reads the on-disk format (and the `#` metadata header);
  `FilterLoader` decorates any loader with a keep-predicate so a gate can run a
  subset of the rows. The loader also supplies `oracle_limits()` — the golden
  set's verifiable depth, `gen_precision − guard`.
- **`GoldenRunner`** (`src/runner/`) — `SequentialRunner` (serial) and
  `ParallelRunner` (a work queue over the subject's executions). Both drive an
  **`ExecutionStrategy`** (`src/execution/`): `RunOnce` for correctness, `Timed`
  for naive timing with the parse hoisted out of the timed span, and
  `CriterionStrategy` (feature `bench`) for statistically-sound timing of a
  small curated cell set. Subject panics are caught and recorded as
  `Computed::Panic` — a fact about the library, judged later against its
  declared overflow policy.
- **Validators** (`src/validators/`) — composable and disjoint.
  `RoundingValidator` grades in-range results against the correctly-rounded
  golden under the declared mode (`Pass` / `WrongMode` / `MisRounded`);
  `OverflowValidator` checks that out-of-range cells match the declared
  `Overflow` policy and that nothing non-`Value` fires in range;
  `PrecisionValidator` reports informational ULP distance.
- **Reporters** (`src/reporting/`) — `Reporter` renders runs into named text
  outputs (`TsvReporter` emits one flattened row per cell); `InlineReporter`
  streams a console summary and returns a `RunSummary` tally
  (`ConsoleReporter::gate()` lists failures for a 0-bad gate,
  `ConsoleReporter::shootout()` prints one line per subject).

## The golden format

One file per function, one case per line: the inputs followed by the expected
value, separated by spaces or tabs, every field a plain `digits.digits` decimal
string (no exponents, no sentinels). The file opens with a `#key=value`
metadata header, and each data line is preceded by a `//` provenance comment:

```
#gen_precision=1233
#guard=2
// generated by: flint, validated by: [mpmath, decimal] ; coverage
0.5 0.70710678118654752440084436210...
// generated by: flint, validated by: [mpmath, decimal(delta~1e-1233)] ; coverage
2 1.41421356237309504880168872420...
```

- **`#gen_precision` / `#guard`** — the precision the values were generated to
  (1233 fractional digits) and the guard width (2). Both are parameters of the
  data: the loader reads them from the header, never assumes them. The deepest
  verifiable depth is `gen_precision − guard` — the guard digits are spent
  deciding the rounding, so they are not gradeable depth themselves. The two have
  one job each: the **first** is the next digit after the widest scale and decides
  the round; the **second** says whether anything survives beneath it and so
  decides exactness. Two digits suffice only because the value is **truncated**
  rather than rounded — a rounded value can carry across a run of nines and
  falsify both at once.
- **Provenance** — each line records the oracle that generated it and every
  oracle that confirmed it. A validator that agrees digit-for-digit is listed by
  name; one that differs but still confirms — within one unit at the last guard
  digit — is listed as `name(delta~MAGNITUDE)`. The annotation gives the
  validator and the size of the difference and deliberately **no cause**: an
  earlier form appended the radix, and `flint(delta~1e-1233, binary)` sat on six
  wrong `exp` rows for months because "binary" pre-explained the disagreement as
  benign and every reader skipped it.
- **One value for all widths and scales.** There is no per-`(width, scale)`
  table. The expected result for any tier is derived by correctly rounding the
  single stored value to the target scale — `GoldenValue::round_to` in
  `src/loader/value.rs` does this for every rounding mode.

### Terminate vs truncate — what the stored length asserts

The stored length of a value is a claim, not an accident:

- A value whose fraction is **shorter than `gen_precision`** asserts the result
  is **exactly** that decimal — the residual below the stored digits is zero,
  so no rounding mode ever bumps it.
- A value whose fraction **runs to the full `gen_precision`** is a
  **truncation**: a nonzero residual exists below the stored digits, and the
  guard digits classify it for rounding at the widest scale. After the kept
  digits at any target scale, the remainder sorts into `Zero` / `Below` /
  `Tie` / `Above` (`classify_residual` in `src/loader/value.rs`): a remainder of
  `0…0` on a terminated value is `Zero` but on a truncated value is `Below`
  (something nonzero hides underneath); a remainder of exactly `5`/`50…0` on a
  terminated value is a true `Tie` but on a truncated one is `Above`. That
  distinction is what makes half-to-even and the directed modes gradeable at
  every scale from one stored string.

The exactness claim is **proof-based**, enforced at generation time by
`oracle/exactness.py`. A heuristic ("the next ~40 digits are all zero") is not
enough — near-zero transcendentals can carry thousands of structural zeros
before the first residual digit (`sinh(1e-1231)` resumes around `1e-3694`), so
every finite window would falsely conclude "terminated". A value therefore
stays stripped only when exactness is provable:

- **Pure-transcendental family** (`exp`, `ln`, the trig, hyperbolic, and
  inverse families, `atan2`) — irrational at every in-domain nonzero rational
  input by the Lindemann–Weierstrass theorem and its corollaries, so exact only
  at the known points (`f(0)`, `ln 1`, `acos 1`, `atan2(0, +x)`, …).
- **Algebraic / power family** (`sqrt`, `cbrt`, `hypot`, `exp2`, `log2`,
  `log10`, `log`, `powf`) — decided by an exact rational inverse-check, e.g.
  `sqrt` is exact iff `c · c == a` as `Fraction`s.
- **Arithmetic** (`add`, `sub`, `mul`, `div`, `rem`) — generated by an exact
  rational engine, so its terminate decision is itself a proof.

Anything unprovable is re-padded with zeros to the full truncated form —
identical digits, truthful claim.

## Who generates, and why only one oracle does

**A generator has to be able to prove its own answer.** That is the whole policy,
and it replaces the per-function radix table that used to live here
(`GENERATOR_POLICY`, now deleted).

| Functions | Generator | Why |
| --- | --- | --- |
| `rem` | `fraction` (`fractions.Fraction`) | exact rational arithmetic — and the flint adapter implements no `rem`, so this is a gap rather than a preference |
| everything else | `flint` (FLINT/Arb) | rigorous intervals + `unique_fmpz` *pin* the true value, so the stored truncation is proven rather than approximated |

An oracle value is the true value truncated to the stored depth, and deciding a
truncation means knowing which side of a digit boundary the value falls on.
Arb's interval can settle that; a point value cannot. `decimal` is exact base-10
but a point value behind a fixed window, and `mpmath` is a point float — so both
validate and neither generates. This is not theoretical: `decimal` generating
`exp` is exactly what produced six wrong answers, and the two "independent"
oracles that agreed on them agreed because they shared the same fixed-window
shape.

**The old radix concern is answered by the interval, not the radix.** An exact
decimal such as `0.1` has no finite binary form, so a binary oracle can bake a
spurious `…999…` or `…000…1` tail into the last digits, and a point-float oracle
can floor an exact result one unit below it. Arb pins the exact integer instead
of floating just below it, and `oracle/exactness.py` has the final say on any
exactness claim — a stored value stays short only when exactness is provable.

Every generated line is then **cross-checked by every other available oracle**
that supports the function (`mpmath`, `flint`, `mpfr`, `sympy`, `decimal`,
`fraction`, in that order). A validator that cannot compute an input abstains;
a disagreement within **1 unit at the last guard digit** *confirms* the value and
is recorded in the line's comment; anything beyond that bound is flagged for a
maintainer to investigate — never silently kept. A line no oracle could confirm
is also dropped.

The bound is one unit because it follows from the fetch contract rather than from
observed noise: every oracle floors the same true value at the same depth, so the
only honest source of difference is an internal error straddling the truncation
boundary, and that moves the floor by at most one.

## Regenerating the golden set

Generation is a maintainer step, separate from running the tests (which only
read the committed files):

```
cd decimal-scaled-golden
pip install -r oracle/requirements.txt        # mpmath (BSD)
pip install -r oracle/requirements-extra.txt  # optional: sympy (BSD), python-flint / gmpy2 (LGPL)

# regenerate (inputs harvested from the .pb files in lead/):
python -m oracle.generate generate --functions sqrt,exp,ln --out golden --precision 1233 --jobs 8

# re-check the committed set against the validator oracles:
python -m oracle.generate revalidate --functions sqrt,exp,ln --out golden --precision 1233
```

`--jobs` defaults to ~80% of the CPU cores (lines are independent, so
generation parallelises per line). `--generator` / `--validators` override the
policy for a run; `--limit N` caps the harvested inputs per function for a
quick proof set. The full all-function regeneration is a long compute.

**Licensing.** The Rust crate reads only text and lists no oracle in its
dependency tree. The `flint` and `mpfr` adapters are this project's own
(MIT/Apache) code that lazily imports the user-installed LGPL packages
`python-flint` / `gmpy2` in a separate process at arm's length — "works that
use the Library", never bundled, never linked into the crate.

## Plugging a library in

Implement `DecimalSubject` for your library (one subject per `(width, scale)`
cell — or one erased subject that dispatches internally), then point a runner
at the golden directory:

```rust
use decimal_scaled_golden::{
    FileLoader, GoldenRunner, OverflowValidator, ParallelRunner, RoundingValidator, RunOnce,
};

let runner = ParallelRunner {
    threads: 8,
    strategy: RunOnce,
    loader: Box::new(FileLoader::new("path/to/golden")),
    validators: vec![
        Box::new(RoundingValidator { gen_precision: 1233 }),
        Box::new(OverflowValidator),
    ],
};
let results = runner.run(&my_subject, &functions);
```

Two reference integrations live in this workspace:

- **`decimal-scale-test`** — the `decimal-scaled` subjects and the filterable
  full-surface gate (one erased subject over every band-edge `(width, scale)`
  cell, all six rounding modes).
- **`golden-competitors`** — third-party adapters (`rust_decimal`, `f64`,
  `bigdecimal`, `dashu-float`, `fastnum`, `decimal-rs`, `g_math`) graded
  side-by-side against the same golden set, each to its own declared precision
  and rounding mode.
