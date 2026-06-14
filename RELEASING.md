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

- **ci (pre-merge gates)** — `ci.yml`, on every PR **and** push to
  `main` / `release/*`: `tests (gate)` (the full `cargo test` for the
  root crate and `decimal-scale-test`, widest and default feature sets)
  plus the width-sharded `tests consolidated (<tier>)` matrix (one shard
  per tier, narrow…d1232 — a single full-width test binary will not
  build on a hosted runner), then `no_std`, `docs` (rustdoc
  `-D warnings`), and `msrv`. (`clippy` runs here too but is
  informational — see Manual.)
- **golden comprehensive** — `golden-comprehensive.yml`; the crate's
  **correctness gate** (it replaced the deleted 0.5 ULP precision gate).
  The full six-mode golden surface — every band-edge `(width, scale)`
  cell, every row, all six rounding modes — width-sharded × row-striped,
  0 bad / 0 panic. The precision guarantee is *enforced here, not
  assumed*: a kernel that rounds wrong turns this red.
- **docs-drift** — `docs-drift.yml`; `render_docs.py --check` on every
  PR, failing on any stale GENERATED doc region.
- **cargo-audit** — RustSec advisory check.

> **Branch protection must list the LIVE checks.** Required status checks
> on `main` live in repo settings, not these files; a required check that
> is renamed or retired will hang the PR forever on a context that never
> reports. The required set should be exactly: the `tests consolidated
> (<tier>)` shards, `tests (gate)`, `docs`, `no_std`, `msrv`, `golden
> comprehensive`, `docs-drift`, and `RustSec advisory check` — and must
> NOT list any retired job (the old `precision` and `Run benchmarks`
> contexts in particular).

> **Performance is advisory, never a gate.** Perf is tracked
> out-of-band by the `bench-branch-compare` workflow (run on demand,
> not a per-PR required context): it reports branch-vs-release shifts
> but never blocks the merge. Perf is a signal; correctness (the
> precision gate) is the release blocker.

### Manual — run / verify before merge (NOT auto-gated)

- **Clippy** — `cargo clippy --lib` clean. The `ci.yml` clippy job is
  informational only (it never fails the build), so clippy stays a
  manual check. The full `cargo test` itself is auto-gated by
  `tests (gate)`.
- **mkdocs build** — `mkdocs build --strict`. The rustdoc build is
  auto-gated pre-merge (`docs (gate)` in `ci.yml`), but the mkdocs site
  build runs only in `docs.yml` on push to `main` (post-merge), so a
  broken site build does **not** block the PR — check it locally first.
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
- The golden gate green: `golden_default` and `golden_all_modes`
  (`decimal-scale-test`) at **0 bad / 0 panic** over every band-edge
  `(width, scale)` cell, all six rounding modes — run on CI by
  `golden-comprehensive.yml` (the correctness gate that replaced the
  retired 0.5 ULP precision suite).
- **Regenerate the single-sourced docs and commit on the release branch
  as part of the PR.** Run `python scripts/render_docs.py`; it fills
  every `<!-- … GENERATED:<key> … -->` region — the install snippet
  (pinned to the new `major.minor`), the width-tier table, and the
  precision (LSBε) tables (from the committed `results/precision/*.tsv`)
  — from one source each. Commit any changes. The `docs-drift` CI gate
  runs `render_docs.py --check` on the PR and fails on any drift, so a
  stale doc region will block the merge.

## 3. Refresh benchmarks + data-driven pages — mandatory, every release

Stale numbers misrepresent the release. Every data-driven page is
regenerated from a **fresh GitHub-Actions run** (never a local sweep —
local machines cannot produce stable numbers). The pipeline is now
**self-rendering**: pushing the release branch triggers each data
workflow, and each one re-runs its bench/gate, self-commits its results
TSV under `results/`, runs `scripts/render_docs.py`, and commits the
refreshed page back to the branch in ONE `github-actions[bot]` commit
(no manual download / ingest / chart step). A `GITHUB_TOKEN` self-commit
does not re-trigger workflows, so there is no loop.

Pushing to `release/*` triggers these, each in its own serial queue
(`cancel-in-progress: false` — a queued run waits its turn; it is never
cancelled):

| Workflow | Re-runs | Self-renders |
|----------|---------|--------------|
| `golden-comprehensive.yml` | the six-mode golden surface | `golden.md`, `precision.md` (`results/golden/`) |
| `lib-perf.yml` | peer-crate timing over the golden set | `comparisons.md` + category pages (`results/lib_cmp/`) |
| `history.yml` | per-version timing across releases | `history.md` (`results/history/`) |
| `bench-branch-compare.yml` | branch-vs-latest-tag timing (advisory perf signal) | `performance.md` (`results/timing/`) |

So "refresh benchmarks" is: **push the release branch, wait for these
workflows to land their self-commits, then `git pull`.** Poll with
`gh run list --branch release/<version>`. The wide tiers are slow; the
narrow / peer runs finish sooner. If a self-commit is missing (a run
failed mid-way), re-run just that workflow — never hand-edit the page:

```sh
gh workflow run <workflow>.yml --ref release/<version>
```

The non-data GENERATED regions (the install snippet pinned to the new
`major.minor`, the width-tier table) are NOT refreshed by a workflow —
run `python scripts/render_docs.py` locally and commit any change. The
`docs-drift` gate (`render_docs.py --check`) fails the PR on any stale
region, data-driven or not.

- The **precision** tables (LSBε, ≤ 0.5 ULP per function and scale) are
  *measured*, single-sourced from the committed `results/` data, and
  rendered into the `<!-- … GENERATED:precision:* … -->` regions by
  `render_docs.py` — never hand-edited; `golden-comprehensive.yml`
  refreshes the underlying data.
- Refresh any README speed charts (absolute `raw.githubusercontent.com`
  URLs so they render on crates.io).

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
- The PR must pass the required checks (the `ci.yml` gates: `tests`,
  the `tests consolidated (<tier>)` shards, `no_std`, `docs`, `msrv`;
  plus `golden comprehensive`, `docs-drift`, and `RustSec advisory
  check`). These are the contexts branch protection must require — see
  the note under Automatic gates.
- Review for: golden (correctness) gate green, benchmarks refreshed,
  CHANGELOG and docs updated, version bumped.
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
   **Also move the minor-resolving tag** `git tag -f vX.Y && git push -f
   origin vX.Y` — `decimal-scaled-golden`'s default `UrlLoader` pins to
   `vX.Y` and fetches the golden set from the repo at that ref, so it must
   point at this release's commit ("0.5 → 0.5.max").
4. Publish the crates (dry-run each first with `--dry-run`):
   1. `cargo publish -p decimal_scaled_macros` — the proc-macro crate the
      root depends on (`version = "X.Y.Z"`), so it must reach crates.io
      before the root.
   2. `cargo publish -p decimal-scaled` — the root crate.
   3. `cargo publish -p decimal-scaled-golden` — the standalone
      library-agnostic golden-testing harness (lockstep version, its own
      release). Independent of the other two (nothing published depends on
      it), so its order is free. It ships the **harness only** — the
      ~130 MB `golden/` set is excluded via `include`; consumers fetch it
      with the `net`-feature `UrlLoader` (default ref `vX.Y`, step 3).
   The remaining members (`decimal-scaled-cells`, `decimal-scale-test`,
   `golden-competitors`) are `publish = false` dev/test crates — never
   published.
5. Publish the GitHub release notes.

## After the release

- For a fix release on an older lineage, merge its corrections
  **forward** into any in-flight feature branch (e.g. `0.4.4 → 0.5.0`),
  keeping the delta == 0 precision suite as a permanent regression gate.
- Confirm docs.rs built and the gh-pages site deployed.
