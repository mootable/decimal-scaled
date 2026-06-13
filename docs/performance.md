# Performance

How fast each operation is, by storage width and scale.

The numbers on this page are **generated from CI**: the `bench-branch-compare`
run measures every `(operation, width, scale)` on a GitHub-hosted runner,
compares it against the previous release, and commits the medians to
`results/timing/`, which this page renders. They are refreshed on each release
PR.

> Absolute timings are machine-dependent — the *ratios* between operations and
> widths, measured in the same run, are what to read. Operands are `black_box`-ed
> so the optimiser can't fold the work away.

<!-- The generated timing tables land here once the timing pipeline is wired
     (see research/bench_docs_pipeline.md, step 3). Until the first
     bench-branch-compare run on the release PR populates results/timing/, this
     page is intentionally a stub. -->
