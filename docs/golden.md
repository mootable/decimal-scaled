# The golden harness — `decimal-scaled-golden`

`decimal-scaled-golden` is a small, **library-agnostic** test harness. It holds a
big set of known-correct answers (the *golden* values — <!-- BEGIN GENERATED:golden:counts -->101,809 answers across 28 functions<!-- END GENERATED:golden:counts -->) and the machinery to check
*any* decimal library against them. It depends on **nothing** from `decimal-scaled`,
so you can drop it into your own crate and grade your own number type on exactly the
same terms.

You write one small adapter for your library, point the harness at the golden
answers, and it tells you — per function, per value — whether your results are
correctly rounded, and how fast they are.

!!! info "Reference"
    API docs: [`decimal_scaled_golden` on docs.rs](https://docs.rs/decimal-scaled-golden)
    &nbsp;·&nbsp; source + design:
    [`decimal-scaled-golden/`](https://github.com/mootable/decimal-scaled/tree/main/decimal-scaled-golden)
    &nbsp;·&nbsp; worked adapters for seven real libraries:
    [`golden-competitors/src/lib.rs`](https://github.com/mootable/decimal-scaled/blob/main/golden-competitors/src/lib.rs)

## What's in the box

Three pieces:

- **The harness** (`src/`) — the `DecimalSubject` trait you implement for your
  library, plus the runners, timers, validators, and reporters that drive it. It has
  **zero dependencies** by default (the optional `bench` feature adds criterion, for
  timing).
- **The questions** (`lead/`) — one file per function listing the inputs to test:
  for `sqrt`, values like `2` and `1.5`, plus the deliberately awkward ones (near
  ties, domain edges, regressions). These are the only files you ever edit by hand;
  the oracle turns them into the answers.
- **The golden answers** (`golden/`) — one file per function (`sqrt`, `cbrt`, `exp`,
  `ln`, the trig and hyperbolic family, `add` / `sub` / `mul` / `div` / `rem`, and
  more). Each answer is stored once, to 1233 digits, and the harness rounds it down
  to whatever precision your type uses.
- **The oracle tool** (`oracle/`) — a Python program that reads the questions in
  `lead/` and *generates* the answers in `golden/`, using several independent
  high-precision maths libraries and only keeping an answer when they all agree. You
  don't need it to run the tests; it's how the answers were made.

## Where the golden answers live

The answer files are in
[`decimal-scaled-golden/golden/`](https://github.com/mootable/decimal-scaled/tree/main/decimal-scaled-golden/golden)
— one `<function>.au` file each. To test your own library, **copy that `golden/`
folder into your project** (or add it as a git submodule). Every example below reads
its location from a `GOLDEN_DIR` environment variable and falls back to a local
`golden` folder, so the same code works on your machine and in CI.

## Writing a subject

A *subject* is a small adapter that teaches the harness how to talk to your library:
how to read a number from text, do a calculation, and write the answer back out. You
implement one trait,
[`DecimalSubject`](https://docs.rs/decimal-scaled-golden/latest/decimal_scaled_golden/trait.DecimalSubject.html).

Here is a complete, working subject. It stands in for "your library" with plain
`f64` so you can run it as-is — swap `f64` for your own number type and the rest
stays the same.

```rust
use std::collections::BTreeMap;
use decimal_scaled_golden::{
    Capabilities, Computed, DecimalSubject, FnSupport, Function, Limits, Overflow, Radix,
    RoundingMode,
};

/// Your library, wrapped so the harness can test it.
struct MyLib;

impl DecimalSubject for MyLib {
    /// Your library's own number type. (We use `f64` here as a stand-in.)
    type Value = f64;

    /// A short name, used in the reports.
    fn name(&self) -> String {
        "my-lib".into()
    }

    /// Tell the harness which functions you support, how each one rounds, and what
    /// it does when a result is too big to hold. Anything you leave out simply
    /// isn't tested — so only list what you actually implement.
    fn capabilities(&self) -> Capabilities {
        let support = FnSupport {
            mode: RoundingMode::HalfToEven, // how YOUR library rounds
            overflow: Overflow::Panic,      // what it does when a result overflows
        };
        let mut functions = BTreeMap::new();
        functions.insert(Function::Sqrt, support);
        functions.insert(Function::Ln, support);
        Capabilities {
            name: "my-lib".into(),
            radix: Radix::Decimal,
            config: BTreeMap::new(), // optional notes for the report (e.g. width, scale)
            functions,
        }
    }

    /// Read a plain decimal string (like "2" or "1.5") into your number type.
    fn string_to_value(&self, s: &str) -> f64 {
        s.parse().expect("a decimal number")
    }

    /// Write your number back out as a plain decimal string (no "1e5" exponents).
    fn value_to_string(&self, v: &f64) -> String {
        format!("{v}")
    }

    /// Describe what your type can hold. `None` means "no limit on that side".
    /// `max_precision` is how many digits you keep after the decimal point.
    fn limits(&self, _value: &str) -> Limits {
        Limits {
            min_value: None,
            max_value: None,
            max_precision: 15,
            max_significant_digits: None,
        }
    }

    /// Do the actual maths. You return a small closure, so the harness can call it
    /// many times (when timing) without re-reading the inputs each time.
    fn execute(
        &self,
        function: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[f64]) -> Computed<f64> {
        move |inputs| match function {
            Function::Sqrt => Computed::Value(inputs[0].sqrt()),
            Function::Ln => Computed::Value(inputs[0].ln()),
            _ => Computed::Error("not supported".into()),
        }
    }
}
```

Three things keep the test honest:

- **A subject just does the maths — it never decides whether the answer is good.**
  Parse, compute, format. Whether a result counts as correct, or as an expected
  overflow, is the harness's job. If your code panics, the harness catches it and
  records it.
- **Only claim what you really do.** Listing a function in `capabilities()` *is* the
  claim that you support it. And `mode` must be the rounding your library actually
  uses, or you'll see a wall of "wrong rounding" failures.
- **Tell the truth in `limits()`.** If your type keeps fewer digits as numbers get
  bigger, say so — otherwise the harness will expect digits you never had and report
  them as mistakes.

The seven adapters in
[`golden-competitors`](https://github.com/mootable/decimal-scaled/blob/main/golden-competitors/src/lib.rs)
are real, worked examples for `rust_decimal`, `fastnum`, `bigdecimal`, and others —
each comment is a lesson learned the hard way.

## Examples

Three things you'll want to do, each as a small program plus the GitHub Actions
workflow to run it. They all reuse the `MyLib` subject above.

### Precision: are your answers correct?

This grades every supported function against the golden answers and **fails the
build if any answer is wrong**. Put it in `tests/precision.rs`:

```rust
use decimal_scaled_golden::{
    ConsoleReporter, FileLoader, Function, GoldenRunner, InlineReporter, OverflowValidator,
    ParallelRunner, RoundingValidator, RunCollector, RunOnce,
};

#[test]
fn my_lib_is_correctly_rounded() {
    let golden_dir = std::env::var("GOLDEN_DIR").unwrap_or_else(|_| "golden".into());

    let runner = ParallelRunner {
        threads: 8,            // run cases in parallel
        strategy: RunOnce,     // just compute once — we're checking answers, not timing
        loader: Box::new(FileLoader::new(golden_dir)),
        validators: vec![
            // Is the answer rounded correctly? (1233 = how many digits the
            // golden answers are stored to.)
            Box::new(RoundingValidator { gen_precision: 1233 }),
            // Did overflow behave the way the subject declared?
            Box::new(OverflowValidator),
        ],
    };

    // Run the functions you support and collect the results.
    let mut results = RunCollector::new();
    results.add(runner.run(&MyLib, &[Function::Sqrt, Function::Ln]));

    // Print a one-line summary and get the counts back.
    let summary = ConsoleReporter::gate()
        .report(&[results], &mut std::io::stderr())
        .unwrap();

    // A correct library has no bad answers, no unexpected crashes, and ran at
    // least one case.
    assert_eq!(summary.bad, 0, "some answers were wrong");
    assert_eq!(summary.panic, 0, "crashed on a value it should handle");
    assert!(summary.pass > 0, "nothing was actually tested");
}
```

The workflow, in `.github/workflows/precision.yml`:

```yaml
name: precision
on: [push, pull_request]
jobs:
  precision:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: check every answer is correctly rounded
        run: cargo test --release --test precision
        env:
          GOLDEN_DIR: ${{ github.workspace }}/golden
```

### Performance: how fast are your operations?

This times your operations on real values. It uses the `Timed` strategy, which calls
each operation a number of times and records the average. Put it in
`examples/timing.rs`:

```rust
use decimal_scaled_golden::{
    FileLoader, FilterLoader, Function, GoldenRunner, SequentialRunner, Timed,
};

fn main() {
    let golden_dir = std::env::var("GOLDEN_DIR").unwrap_or_else(|_| "golden".into());

    // Only time a handful of rows per function — timing doesn't need the whole set.
    let loader = FilterLoader::new(
        FileLoader::new(golden_dir),
        |_function, case| case.line <= 50,
    );

    let runner = SequentialRunner {
        // Call each case 100 times and average — one quiet thread, so the numbers
        // aren't fighting each other for the CPU.
        strategy: Timed { number_of_executions: 100 },
        loader: Box::new(loader),
        validators: vec![], // timing only — no need to grade the answers
    };

    let results = runner.run(&MyLib, &[Function::Sqrt, Function::Ln]);

    // Each tested case carries its per-call time in nanoseconds. Print the middle
    // (median) time for each function.
    for function in &results.functions {
        let mut times: Vec<u64> = function.cells.iter().filter_map(|c| c.timing).collect();
        times.sort_unstable();
        if let Some(&median) = times.get(times.len() / 2) {
            println!("{}: ~{median} ns per call", function.function.name());
        }
    }
}
```

Run it with `cargo run --release --example timing`. The workflow, in
`.github/workflows/timing.yml`:

```yaml
name: timing
on: workflow_dispatch   # run by hand from the Actions tab
jobs:
  timing:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: time the operations
        run: cargo run --release --example timing
        env:
          GOLDEN_DIR: ${{ github.workspace }}/golden
```

!!! tip "For publication-quality numbers"
    `Timed` is a quick average. For proper statistics (warmup, sampling, outlier
    handling) enable the crate's `bench` feature and swap `Timed` for
    `CriterionStrategy::new()` inside a `[[bench]]` file (`harness = false`).
    [criterion](https://docs.rs/criterion/) then writes a full report under
    `target/criterion/`. Keep the case list small — criterion spends a few hundred
    milliseconds per case.

### Comparison: how do you stack up against other libraries?

This runs several libraries over the same golden answers and prints one line of
results per library. Peers are *expected* to get some answers wrong — that contrast
is the whole point. `OtherLib` below is any second type that also implements
`DecimalSubject` (a competing library you've wrapped, or even your own type set to a
different rounding mode). Put it in `examples/comparison.rs`:

```rust
use decimal_scaled_golden::{
    ConsoleReporter, FileLoader, Function, GoldenRunner, InlineReporter, OverflowValidator,
    ParallelRunner, RoundingValidator, RunCollector, RunOnce,
};

fn main() {
    let golden_dir = std::env::var("GOLDEN_DIR").unwrap_or_else(|_| "golden".into());

    let runner = ParallelRunner {
        threads: 8,
        strategy: RunOnce,
        loader: Box::new(FileLoader::new(golden_dir)),
        validators: vec![
            Box::new(RoundingValidator { gen_precision: 1233 }),
            Box::new(OverflowValidator),
        ],
    };

    // Run each library you want to compare into one shared collection.
    let funcs = [Function::Sqrt, Function::Ln];
    let mut all = RunCollector::new();
    all.add(runner.run(&MyLib, &funcs));
    all.add(runner.run(&OtherLib, &funcs));

    // One tally line per library: pass / skip / bad / panic.
    ConsoleReporter::shootout()
        .report(&[all], &mut std::io::stderr())
        .unwrap();
}
```

Run it with `cargo run --release --example comparison`. The workflow, in
`.github/workflows/comparison.yml`:

```yaml
name: comparison
on: workflow_dispatch
jobs:
  comparison:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: compare libraries
        run: cargo run --release --example comparison
        env:
          GOLDEN_DIR: ${{ github.workspace }}/golden
```

This repo's
[`golden-competitors`](https://github.com/mootable/decimal-scaled/blob/main/golden-competitors/src/lib.rs)
is the full version: seven real libraries graded side by side, each to its own
declared precision and rounding mode.

## Testing several rounding modes

Each subject declares one rounding mode, so to prove your library is correct in
*every* mode you support, make the mode a field on your subject and run one subject
per mode. The harness rounds the single stored golden answer to whichever mode the
subject declares, so each run is judged on its own terms:

```rust
use decimal_scaled_golden::{RoundingMode, RunCollector, RunSummary};

for mode in [RoundingMode::HalfToEven, RoundingMode::Floor, RoundingMode::Ceiling] {
    let mut results = RunCollector::new();
    results.add(runner.run(&MyLib::with_mode(mode), &[Function::Sqrt]));
    let summary = RunSummary::tally(&[results]);
    println!("{mode:?}: {summary}");
}
```

(Here `MyLib::with_mode(mode)` is a constructor that stores the mode and reports it
from `capabilities()`.)

## Generating and extending the golden answers

The golden answers are *produced* by the Python oracle tool, never written by hand.
Inputs go in, correct answers come out:

```text
lead/<fn>.pb   ─▶   python -m oracle.generate   ─▶   golden/<fn>.au
  (inputs you edit)     (compute + cross-check)         (generated, committed)
```

- **New test inputs go in the `.pb` files.** A `lead/<fn>.pb` file is the answer file
  *without* the answer column: one case per line, plain decimal inputs, under a short
  `// why this case matters` comment. To add a tricky value — a near-tie, a boundary,
  a bug you just fixed — append the input line(s) to that function's `.pb` file.
  **Never edit a `.au` file by hand** — it is generated.
- **Regenerate the functions you touched.** The tool reads the `.pb` inputs, computes
  each answer with a high-precision oracle, **cross-checks it against every other
  available oracle**, and writes `golden/<fn>.au`. Any answer the oracles can't agree
  on is dropped, never silently kept.

```sh
cd decimal-scaled-golden
pip install -r oracle/requirements.txt          # the oracle's core deps (BSD)
pip install -r oracle/requirements-extra.txt    # optional: extra cross-check oracles

# regenerate just the functions you changed (inputs come from lead/):
python -m oracle.generate generate   --functions sqrt,exp,ln --out golden --precision 1233 --jobs 4

# or re-check the committed answers without regenerating them:
python -m oracle.generate revalidate --functions sqrt,exp,ln --out golden --precision 1233
```

Generating is a maintainer step, separate from running the tests — the tests only
*read* the committed `.au` files. The same inputs always produce the same answers, so
commit the regenerated `.au` file alongside your `.pb` change. (`--jobs` defaults to
about 80% of your cores; keep it modest on a shared machine — regenerating everything
is a long compute.)

## Property fuzz

Alongside the golden gate, `decimal-scale-test/tests/proptest_identities.rs` checks
identities that hold without an external oracle:

- `exp(ln(x)) ≈ x` for positive `x`
- `ln(exp(x)) ≈ x` for `x` in `[0, 30]`
- `sin² + cos² ≈ 1` over a wide real domain
- `sqrt(x)² ≈ x` for non-negative `x`
- `cbrt(x)³ ≈ x` for real `x`
- `atan(tan(x)) ≈ x` over `(-π/4, π/4)`
- `tanh(atanh(x)) ≈ x` over `(-1, 1)`
- sign symmetries: `sin(-x) = -sin(x)`, `cos(-x) = cos(x)`, `atan(-x) = -atan(x)`, `cbrt(-x) = -cbrt(x)`

Each block runs 100 cases with a deterministic seed, so a counterexample minimises the
same way every run.

## Hard-input categories

The golden set is seeded with ten classes of deliberately hard inputs, each drawn from
the correctly-rounding literature (the papers, not their test vectors):

1. **Half-ULP-tie boundaries** — inputs whose true result lands within 0.45 LSB of a
   half-tie at the storage scale.
   *Reference:* Lefèvre, Muller & Toma, "Toward correctly rounded transcendentals"
   (1998); Muller, *Elementary Functions — Algorithms and Implementation* (3rd ed.,
   2016), §10 "Table maker's dilemma".
2. **Catastrophic cancellation** — `ln(1 + ε)`, `exp(tiny)`, `cos(tiny) ≈ 1 - x²/2`,
   `sin(tiny) ≈ x`, `sqrt(1 + ε)`, `cbrt(1 + ε)`.
   *Reference:* Goldberg, "What every computer scientist should know about
   floating-point arithmetic" (1991) §3; Higham, *Accuracy and Stability of Numerical
   Algorithms* (2nd ed., 2002), §1.7.
3. **Range-reduction breakpoints** — `sin`/`cos` near k·π/2, `tan` near k·π/4, `exp`
   near k·ln 2, `atan` near 1.
   *Reference:* Payne & Hanek, "Radian reduction for trigonometric functions" (1983);
   Muller (2016), §11.
4. **Removable singularity / asymptote** — `tan` near π/2 + δ, `ln` near 0+,
   `atan(huge)`, `sqrt` near 0+.
   *Reference:* Kahan archive, "Branch cuts for complex elementary functions" (1987).
5. **Inverse-identity round-trip stress** — `sin` near π/2, `atan(tan(k·π/8))`,
   `exp(ln(small))`, `sqrt(x²)`, `cbrt(x³)`.
   *Reference:* Brent & Zimmermann, *Modern Computer Arithmetic* (2010), §4.2 "Inverse
   functions".
6. **Perfect-power ± ULP for roots** — `sqrt(n² ± 1)`, `cbrt(n³ ± 1)` for small integer
   `n`.
   *Reference:* Brent & Zimmermann (2010), §3.5 "Square root", §3.6 "k-th root".
7. **Constant edges** — inputs at named constants (π, π/2, π/4, e, ln 2, …) ± a few
   LSBs.
   *Reference:* IEEE 754-2019 standard, §9 "Recommended correctly rounded functions";
   Muller (2016), §9.
8. **Argument-halving cascade** — `atan` near `tan(0.35 · 2⁻ⁿ)` for the per-width
   halving count `n`.
   *Reference:* Muller (2016), §6 on argument reduction; the per-width cascade table in
   [Algorithms](ALGORITHMS.md).
9. **Stage-2 argument reduction edge for `exp`** — inputs near the chosen breakpoint
   `v / 2ⁿ` for `n ≈ √(precision_bits)`.
   *Reference:* Tang, "Table-driven implementation of the exponential function in IEEE
   floating-point arithmetic" (1989).
10. **Tang-lookup band edges** — `ln` and `exp` inputs at the table-index breakpoints
    `T_i = 1 + i / 2ᵏ` for `k ∈ {7, 8, 9}` and at the secondary-index breakpoints
    `j/N · ln 2` for `j = 0..N-1`, `N ∈ {32, 64, 128}`.
    *References:* Tang (1989); Gal & Bachelis, "An accurate elementary mathematical
    library for the IEEE floating-point standard" (1991).
