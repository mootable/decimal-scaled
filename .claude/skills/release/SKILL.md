---
name: release
description: Cut a decimal-scaled release. Points to RELEASING.md (the system of record) for the full process — versioning policy, release branch, PR-into-main + approval, mandatory docs + benchmark refresh, and publish steps. Use when preparing or performing any release.
---

# Releasing decimal-scaled

The **canonical release process is `RELEASING.md`** at the repo root —
the *system of record*. Read it and follow it end-to-end when preparing
or performing any release.

**Keep in sync:** this skill and `RELEASING.md` must stay aligned — if
you change one, change the other. `RELEASING.md` is canonical; keep this
file a thin pointer so they cannot drift.

Three things load-bearing enough to repeat here:

- **Per-action authorization.** `cargo publish`, `git push`, tag pushes,
  and publishing a GitHub release each need an explicit, separate go.
  "Prepare the release" / "bump the version" do **not** authorise
  publishing.
- **Benchmarks are refreshed every release** (hard rule), and the
  **0.5 ULP precision gate is CI-enforced** — it blocks the PR. The
  single-sourced docs are regenerated too: run
  `python scripts/render_docs.py` and commit on the release branch as
  part of the PR; the `docs-drift` gate fails the PR on any drift.
- **Release branch → PR into `main`** (never push to `main` directly).
  Open the PR with the release checklist template:
  `gh pr create --base main --head release/<version> --body-file .github/PULL_REQUEST_TEMPLATE/release.md`
  (web UI: append `?template=release.md` to the compare URL), then tick
  the checklist.

Everything else — the versioning policy, the full release flow, and the
automatic-vs-manual pre-release checklist — is in `RELEASING.md`.
