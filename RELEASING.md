# Releasing decimal-scaled

This document is the **system of record** for cutting a release. The
repository's `/release` helper mirrors this file; **any change to one
must be made to the other**, and this document is canonical.

A release is never a single command. It is: a **release branch** → all
work and docs landed on it → a **mandatory benchmark refresh** → a **PR
into `main`** that must pass CI and review → a version bump → **merge,
tag, publish** in a specific order.

> **Authorization is per-action.** `cargo publish`, `git push`, tag
> pushes, and publishing a GitHub release each need an explicit,
> separate decision. "Prepare the release" or "bump the version" do
> **not** authorise publishing.

## Versioning policy

The crate is **pre-1.0**, so semver is shifted down one level until the
1.0 line is crossed.

| Change | Pre-1.0 (now) | Post-1.0 |
|--------|---------------|----------|
| **Breaking** (API removal / signature or depended-on-behaviour change) | **minor** — `0.X.0` | **major** — `X.0.0` |
| **New feature** (non-breaking addition) | **patch** — `0.x.Y` | **minor** — `x.Y.0` |
| **Fix / cosmetic** (bug fix, docs, perf with no API change) | **patch** — `0.x.Y` | **patch** — `x.y.Z`; post-1.0 these are mostly **documentation / comment / cosmetic** changes — the API and accuracy are stable, and real features go to minor |

- **Accuracy is not a versioned feature.** The crate is correctly
  rounded (0 LSBε / ≤ 0.5 ULP) on every transcendental at every tier and
  stays that way; a precision *fix* (closing a rounding hole) is a patch
  and never regresses.
- The 1.0 line is gated on the wide-tier `mul`/`div` being competitive
  with the best peer, or each gap being structurally justified per the
  ROADMAP. Until then, expect frequent minors for breaking refactors.

## Release flow (overview)

1. Cut the release branch.
2. Land all code and docs for the version on it.
3. **Refresh benchmarks** (mandatory — §3).
4. Update the narrative docs (CHANGELOG / README / ROADMAP / glossary).
5. Bump the version (only when ready to publish).
6. Open a PR `release/<version>` → `main`; pass CI and review.
7. Merge, tag, publish — in order.

## Pre-release checklist

Some checks are **automatic CI gates** (they block the PR into `main`);
the rest are **manual** and must be verified by hand before merge.

### Automatic gates (CI — fail the PR)

- **Precision (0.5 ULP gate)** — `precision.yml`, on every PR **and**
  push: the strict-ULP suites, the mpmath-oracle `ulp_strict_golden`
  golden suite (D38…D1232, **delta == 0 across all six rounding modes**),
  and the proptest ULP fuzz. The precision guarantee is *enforced here,
  not assumed* — a kernel that rounds wrong turns the PR red.
- **Run benchmarks** — `codspeed.yml`; the CodSpeed harness job. The
  benches must compile and run (this is a required context).
- **cargo-audit** — RustSec advisory check.

> **CodSpeed Performance Analysis is advisory, never a gate.** It is
> configured *Informational on failure* (PR comment *On Change*), so it
> reports perf shifts on the PR but never blocks the merge, and it is
> **not** in the branch-protection required contexts. Perf is a signal;
> correctness (the precision gate) is the release blocker.

### Manual — run / verify before merge (NOT auto-gated)

- **Tests + clippy** — `cargo test --features wide,x-wide,xx-wide,macros`
  (plus the default-feature run) and `cargo clippy --lib` clean. The
  precision gate deliberately does **not** run the full `cargo test`.
- **Docs build** — `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` and
  `mkdocs build --strict`. `docs.yml` runs only on push to `main`
  (post-merge), so a broken docs build does **not** block the PR — check
  it locally first.
- **ROADMAP.md** — shipped items moved to shipped; the next version's
  "incoming" section present; later proposals marked `0.5+`.
- **ALGORITHMS.md** — each documented kernel matches what actually ships;
  the "evaluated and not used" section is current; citations correct;
  nothing listed that isn't in the tree.
- **Comments + doc-comments** — accurate to the current code, no stale
  references.
- **CHANGELOG.md** — the version section is complete and dated.
- **README.md** — version-specific content current (the precision / LSBε
  table, install snippet, speed charts).
- **glossary / guides** — document only what is shipped; cite only
  standards genuinely followed (recognized bodies only).
- **Benchmarks refreshed** — figures and tables regenerated from a fresh
  GitHub-Actions sweep (the hard rule; see §3).
- **Version bump** correct per the versioning policy.
- **Licensing** — REUSE / `LICENSES/` headers intact for any new files.

When a new automatic gate is added (cross-platform determinism CI, a
docs-in-sync drift check, a rounding-conformance run), move its item from
Manual to Automatic above.

## 1. Cut the release branch

```sh
git checkout -b release/<MAJOR.MINOR.PATCH>   # e.g. release/0.4.4
```

- Base it on the lineage the work lives on (a fix release branches off
  the released tip; a feature/architecture release off its feature
  branch).
- One release branch per version. Commit all release prep there — never
  directly on `main`.
- **Direction is one-way:** an in-flight feature branch may receive a
  released branch's fixes (e.g. `0.4.4 → 0.5.0`), never the reverse. Do
  **not** merge unreleased feature code into a release branch.

## 2. Land all code and docs on the branch

- All code complete; tests, clippy, and the docs build green (see the
  manual checklist above).
- The precision gate green: `ulp_strict_golden` all pass, **0 ignored**,
  delta == 0 across all six rounding modes and all thirteen widths.
- **Regenerate the single-sourced docs and commit on the release branch
  as part of the PR.** Run `python scripts/render_docs.py`; it fills
  every `<!-- … GENERATED:<key> … -->` region — the install snippet
  (pinned to the new `major.minor`), the width-tier table, and the
  precision (LSBε) tables (from the committed `results/precision/*.tsv`)
  — from one source each. Commit any changes. The `docs-drift` CI gate
  runs `render_docs.py --check` on the PR and fails on any drift, so a
  stale doc region will block the merge.

## 3. Refresh benchmarks — mandatory, every release

Stale benchmark numbers misrepresent the release. Every release ships
figures and tables regenerated from a **fresh GitHub-Actions sweep**
(never a local full sweep — local machines cannot produce stable
numbers).

```sh
# Trigger the release sweep on the release branch (fans out to
# bench-full full_matrix + bench-full lib_cmp + bench-history):
gh workflow run bench-all.yml --ref release/<version>

# Then just wait. Poll with `gh run list`. The wide tiers are slow
# (~40 min); the lib_cmp run finishes sooner.
```

The sweep is self-refreshing. Once every per-width cell of a
`bench-full` run finishes, that workflow's `regenerate` job collects
the run's own Criterion artifacts and commits the refreshed docs
straight back to the swept release branch — no manual download, ingest,
or chart step:

- The **full_matrix** run regenerates `docs/benchmarks.md` §1–§3 via
  `full_matrix_ingest.py --fill` (reads `docs/benchmarks.md.draft`,
  pulls each cell's median from `*/new/estimates.json`, picks the
  per-row natural unit, bolds the row winner). A missing leaf renders
  as `—` and is listed in the job log — values are never fabricated.
- The **lib_cmp** run regenerates the §5 figures under
  `docs/figures/library_comparison/` via `lib_cmp_ingest.py` +
  `cargo run --release --example chart_gen`.

Each run pushes its own `docs(benchmarks): regenerate from <family>
sweep` commit (author `github-actions[bot]`, no attribution trailer)
to the release branch. Pull the branch after both runs land. The §4
prose and §5 precision tables are still maintained by hand as below.

If a run's regenerate job is skipped or you need a manual rebuild, the
same steps run locally against a downloaded artifact set:

```sh
gh run download <run-id> --dir bench-artifacts   # criterion-*-D*/ subdirs
python scripts/full_matrix_ingest.py \
    --artifacts bench-artifacts --fill            # criterion JSON -> docs/benchmarks.md §1–§3
cargo run --release --example chart_gen           # §5 figures -> docs/figures/library_comparison/*.png
```

- Update the "Bench machine … vX.Y.Z full_matrix sweep" provenance note
  in `benchmarks.md` to the new version and date.
- Refresh any README speed charts (absolute `raw.githubusercontent.com`
  URLs so they render on crates.io).
- The README / `benchmarks.md` **precision** tables (LSBε, ≤ 0.5 ULP per
  function and scale) are *measured*, not from the sweep. They are
  single-sourced from the committed `results/precision/*.tsv` files and
  regenerated by `python scripts/render_docs.py` (see §2) — never edit
  the `<!-- … GENERATED:precision:* … -->` regions by hand. Re-run the
  oracle sweep only if the measured TSVs themselves are out of date.

## 4. Narrative docs

See the manual checklist above — CHANGELOG, README, ROADMAP, glossary
and guides all get a pass.

## 5. Version bump + CHANGELOG — a commit *in* the PR (only when ready to publish)

The version bump and CHANGELOG date are **committed to the release branch
so they ride into `main` through the PR — never applied to `main` after
the merge.** Branch protection forbids direct pushes to `main`, so a
post-merge bump would need its own PR; the bump belongs on the release
branch before the merge.

- Bump `version` in `Cargo.toml` to `X.Y.Z`, **and** the lockstep
  `decimal_scaled_macros` dependency line + `macros/Cargo.toml` to match.
- Date the CHANGELOG section (`## [X.Y.Z] — <date>`).
- Commit and push to the release branch; CI re-runs on the open PR.
- This is the publish-authorization step — do it only with explicit
  authorization.

## 6. PR `release/<version>` → `main` + approval

```sh
git push -u origin release/<version>
gh pr create --base main --head release/<version> \
  --title "Release <version> — <headline>" \
  --body-file .github/PULL_REQUEST_TEMPLATE/release.md
# Then fill in the Summary and tick the checklist on the PR.
# (Web UI: append ?template=release.md to the compare URL.)
```

- The PR body is the release checklist
  (`.github/PULL_REQUEST_TEMPLATE/release.md`) — work through every box.
- All merges into `main` go through a **PR** (branch-protection
  practice) — never push to `main` directly.
- The PR must pass CI: the precision gate, CodSpeed, and cargo-audit.
- Review for: precision gate green, benchmarks refreshed, CHANGELOG and
  docs updated, version bumped.
- Pushing docs to the release branch during the sweep is safe — docs do
  not affect the perf run.

## 7. Merge, tag, publish — in order

Each step needs explicit authorization.

1. Merge the PR into `main`.
2. **Wait for the `main` docs run to finish before pushing the tag.**
   `git push origin main && git push --tags` cancels the in-flight main
   docs deploy *and* leaves the tag run unable to deploy (environment
   protection). Always insert `gh run watch` on the main docs run
   between the two pushes.
3. Tag: `git tag vX.Y.Z && git push origin vX.Y.Z` (after the watch).
4. `cargo publish` (dry-run first: `cargo publish --dry-run`).
5. Publish the GitHub release notes.

## After the release

- For a fix release on an older lineage, merge its corrections
  **forward** into any in-flight feature branch (e.g. `0.4.4 → 0.5.0`),
  keeping the delta == 0 precision suite as a permanent regression gate.
- Confirm docs.rs built and the gh-pages site deployed.
