# Documentation single source of truth

Facts that are repeated across `README.md` and `docs/` — the crate
version, the storage-width tier family, the `MAX_SCALE = N − 1` rule,
and the precision (LSBε) shootout tables — live in exactly one place
each and are rendered into the docs by
[`scripts/render_docs.py`](../../scripts/render_docs.py).

## How it works

Each rendered fact occupies a region in a doc bounded by a pair of HTML
comments:

```text
<!-- BEGIN GENERATED:<key> -->
…generated body — do not edit by hand…
<!-- END GENERATED:<key> -->
```

`render_docs.py` replaces the body between the markers from a single
data source. Editing inside a region by hand is pointless: the next
`python scripts/render_docs.py` (and the `docs-drift` CI gate)
overwrites it. The markers themselves are preserved.

Run:

```sh
python scripts/render_docs.py          # rewrite the docs in place
python scripts/render_docs.py --check  # exit 1 if any doc is stale (CI)
```

`render_docs.py` is the ONE entry point: a single invocation
regenerates every snippet region — both the prose facts below and the
precision tables — from their sources. The
[`.github/workflows/docs-drift.yml`](../../.github/workflows/docs-drift.yml)
gate runs the renderer on every pull request (and on push to `main`)
and fails via `git diff --exit-code` if the committed docs no longer
match their sources.

## Sources and regions today

### Prose facts

| Source | Region key | Target | What it fills |
|---|---|---|---|
| `Cargo.toml` `[package] version` | `install:dependency` | `README.md` | the install `[dependencies]` snippet, version pinned to the current `major.minor` |
| `docs/_data/tiers.json` | `widths:table` | `docs/widths.md` | the full thirteen-row tier table; `MAX_SCALE` is derived as `digits − 1`, so it can never drift from the type name |
| `docs/_data/tiers.json` | `widths:count` | `docs/widths.md` | the spelled-out tier count in the lede (`thirteen storage widths`) |

### Precision (LSBε) tables

These regions are filled by
[`scripts/render_precision_table.py`](../../scripts/render_precision_table.py)
from the committed `results/precision/*.tsv` shootout result files, so
every cell traces back to exactly one measured TSV row and the findings
prose can never drift from the data. The region body is the markdown
table only (header + separator + one row per library); the surrounding
heading and legend are ordinary prose.

| Source | Region key | Target | What it fills |
|---|---|---|---|
| `results/precision/*.tsv` | `precision:D38:readme` | `README.md` | the representative `D38<19>` nine-method slice |
| `results/precision/*.tsv` | `precision:D38` | `docs/benchmarks.md` | the full 22-function `D38<19>` table |
| `results/precision/*.tsv` | `precision:D76` | `docs/benchmarks.md` | the eight-method `D76<35>` wide-tier subset |
| `results/precision/*.tsv` | `precision:D307` | `docs/benchmarks.md` | the eight-method `D307<150>` deep-scale subset |

The benchmark **timing** tables in `docs/benchmarks.md` are deliberately
NOT covered here — they need a GHA bench sweep to regenerate and are
owned by the bench self-refresh workflows (`bench-full.yml`,
`bench-history.yml`), not the deterministic PR-time drift gate.

## Adding a new single-sourced fact

1. Put the datum in a `docs/_data/*.json` file (or read it from
   `Cargo.toml`, like the version; or, for a precision table, point at
   the relevant `results/precision/*.tsv`).
2. Add a `render_<key>()` builder in `scripts/render_docs.py` returning
   the region body (no marker lines).
3. Register it in the `REGIONS` map with its target file.
4. Wrap the target text in the doc with the matching
   `<!-- BEGIN GENERATED:<key> -->` / `<!-- END GENERATED:<key> -->`
   markers.
5. Run `python scripts/render_docs.py` and commit. The drift gate now
   guards it.
