# Precision

**`decimal-scaled` is perfectly precise** — correctly rounded to within 0.5 ULP,
i.e. **0 bits of error**.

<!-- BEGIN GENERATED:precision:stats -->
We execute 101,809 specialised inputs across all 28 functions, on 92 widths and scales, under all 6 rounding modes, resulting in 56,198,568 separate checks.
<!-- END GENERATED:precision:stats -->

Each golden case was gathered from the codebase failing, from theory, from
destructive testing of our own code, and from destructive testing of other
libraries — collated into a behemoth of meanness.

The data on this page is generated and committed directly from the
[golden-comprehensive CI job](https://github.com/mootable/decimal-scaled/actions/workflows/golden-comprehensive.yml)
and auto-rendered here; there is **no manual editing**. If we ever fail our
guarantee, we want you to know.

Each `✓` is `0` bits of error — the exact correctly-rounded value — across every
function and all six rounding modes at that cell. Columns are the five sampled
scale positions per tier (`s=0` … `max`); the full per-scale surface lives in
[`results/golden/`](https://github.com/mootable/decimal-scaled/tree/main/results/golden).
A regression shows the bits of error instead.

<!-- BEGIN GENERATED:precision:surface -->
_Pending the first golden-comprehensive CI run — this renders from `results/golden/summary.tsv`._
<!-- END GENERATED:precision:surface -->
