# Precision

`decimal-scaled` is **correctly rounded** — within 0.5 ULP, i.e. `0` bits of
error — on every function, at every width and scale, under all six rounding
modes. That is the headline guarantee, and this page shows the whole surface.

The data is **generated from CI**: the *golden comprehensive* run checks every
`(width, scale)` cell, in all six rounding modes, against the oracle-validated
golden set, and commits the per-cell results to `results/golden/`, which this
page renders. Refreshed on each release PR.

<!-- The width x scale grid (each cell = our result) and the rounding-mode table
     (6/6 = all six modes correctly rounded) land here once the golden pipeline
     is wired (see research/bench_docs_pipeline.md, step 2). Until the first
     golden-comprehensive run on the release PR populates results/golden/, this
     page is intentionally a stub. -->
