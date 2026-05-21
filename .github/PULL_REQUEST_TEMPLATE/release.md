# Release <version> → main

<!-- One-paragraph summary of what this release contains. -->

## Summary



## Versioning

- [ ] `Cargo.toml` version bump follows the policy (pre-1.0: breaking → minor `0.X.0`, else patch `0.x.Y`)
- [ ] `CHANGELOG.md` section added and dated

## Automatic gates (CI — must be green)

- [ ] 📐 Precision (0.5 ULP gate) — `ulp_strict_golden` delta == 0, all 6 modes / 13 widths; proptest ULP fuzz
- [ ] CodSpeed — no unexpected perf regression
- [ ] cargo-audit — clean

## Manual checks (verified before merge)

- [ ] `cargo test --features wide,x-wide,xx-wide,macros` + default-feature run green
- [ ] `cargo clippy --lib` clean
- [ ] Docs build: `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` and `mkdocs build --strict`
- [ ] **Benchmarks refreshed** from a fresh GitHub-Actions sweep (figures + `benchmarks.md` tables)
- [ ] `ROADMAP.md` — shipped items moved; next-version "incoming" section; later proposals marked `0.5+`
- [ ] `ALGORITHMS.md` — matches shipped kernels; "evaluated and not used" current; citations correct
- [ ] Comments / doc-comments accurate, no stale references
- [ ] `README.md` current — precision / LSBε table, install snippet, speed charts
- [ ] glossary / guides document only shipped behaviour; standards claims honest (recognized bodies only)
- [ ] REUSE / `LICENSES/` headers intact for any new files

## Publish (after merge — each step needs explicit authorization)

- [ ] Merge this PR into `main`
- [ ] Wait for the `main` docs run (`gh run watch`) **before** pushing the tag
- [ ] Tag `vX.Y.Z` pushed
- [ ] `cargo publish` (dry-run first)
- [ ] GitHub release notes published

---

Full process: see [`RELEASING.md`](../../RELEASING.md).
