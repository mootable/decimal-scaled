# Release <version> → main

<!-- One-paragraph summary of what this release contains. -->

## Summary



## Versioning

- [ ] `Cargo.toml` version bump follows the policy (pre-1.0: breaking → minor `0.X.0`, else patch `0.x.Y`)
- [ ] `CHANGELOG.md` section added and dated

## Automatic gates (CI — must be green)

- [ ] 📐 Precision (0.5 ULP gate) — `ci.yml` `golden (gate)` at 0 bad / 0 panic, all 6 modes, every band-edge `(width, scale)` cell; `tests (gate)` green (incl. proptest ULP fuzz)
- [ ] cargo-audit — clean

## Manual checks (verified before merge)

- [ ] `cargo clippy --lib` clean (the `ci.yml` clippy job is informational only)
- [ ] Site build: `mkdocs build --strict` (rustdoc is auto-gated pre-merge by `docs (gate)`)
- [ ] **Benchmarks refreshed** from a fresh GitHub-Actions sweep (figures + `benchmarks.md` tables)
- [ ] `ROADMAP.md` — shipped items moved; next-version "incoming" section; later proposals marked `0.5+`
- [ ] `ALGORITHMS.md` — matches shipped kernels; "evaluated and not used" current; citations correct
- [ ] Comments / doc-comments accurate, no stale references
- [ ] `README.md` current — precision / LSBε table, install snippet, speed charts
- [ ] glossary / guides document only shipped behaviour; standards claims honest (recognized bodies only)
- [ ] REUSE / `LICENSES/` headers intact for any new files

---

Full release process: [`RELEASING.md`](../../RELEASING.md).
