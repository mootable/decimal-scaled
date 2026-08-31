#!/usr/bin/env python3
"""Push-button pre-release docs/benchmark refresh (RELEASING.md section 3).

Automates the manual artifact-download + ingest + render flow that
section 3 of RELEASING.md describes: discover the latest successful
bench/precision artifacts for a ref, refresh the committed data files
they feed, re-run the single-source renderers, and report what changed
in the working tree for human review. It never commits, never pushes —
refreshed files are left in the tree.

Steps (each independently skippable):

  1. discover   — find the newest successful runs carrying live
                  artifacts on the ref, via the gh CLI:
                    * bench-full.yml (full_matrix family)
                      -> criterion-full_matrix-D<N> artifacts
                    * bench-full.yml (lib_cmp family, --figures only)
                      -> criterion-lib_cmp-D<N> artifacts
                    * lib-cmp-precision.yml
                      -> lib-cmp-precision-results artifact
  2. download   — `gh run download` each discovered run into the work
                  dir (default tmp/release_docs_refresh/, git-ignored).
  3. timing     — `scripts/full_matrix_ingest.py --artifacts ... --fill`
                  (rebuilds docs/benchmarks.md sections 1-3 from
                  docs/benchmarks.md.draft; missing leaves render as
                  em-dashes, never fabricated).      [--skip-timing]
  4. precision  — copy the per-library *.tsv files from the
                  lib-cmp-precision-results artifact over
                  results/precision/ (the single source of truth for
                  the precision tables).             [--skip-precision]
  5. figures    — `scripts/lib_cmp_ingest.py` -> target/medians.tsv,
                  then `cargo run --release --example chart_gen` to
                  re-render docs/figures/library_comparison/*.png.
                  OPT-IN via --figures: it is a full release build of
                  the crate plus the plotters chart example, which can
                  take several minutes cold.
  6. render     — `scripts/render_docs.py` fills every GENERATED
                  region (README install snippet, width table,
                  precision tables) from the committed data.
                                                     [--skip-render]
  7. verify     — re-run the docs-drift gate's check
                  (`render_docs.py --check`) and list every refreshed
                  file pending review (`git status`).  [--skip-verify]

Exit codes:
  0 — clean success, no file changed (docs already current)
  2 — refresh succeeded and left changes in the working tree for
      human review (the expected outcome of a real refresh)
  1 — failure (no usable artifacts, a sub-step failed, gh missing...)

`--dry-run` performs discovery only and prints the plan with the
resolved run IDs.

Requirements: Python 3 stdlib + the `gh` CLI (authenticated). The
figures step additionally needs a Rust toolchain.

Typical release use (on the release branch, after the bench-all sweep
has landed its self-commits or at least finished its runs):

    python scripts/release_docs_refresh.py --figures
"""
from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Workflow files and the artifact-name prefixes that identify each
# artifact family (bench-full carries one family per run, chosen by its
# `bench_family` dispatch input — the artifact names tell them apart).
BENCH_FULL_WORKFLOW = "bench-full.yml"
PRECISION_WORKFLOW = "lib-cmp-precision.yml"
FULL_MATRIX_PREFIX = "criterion-full_matrix-D"
LIB_CMP_PREFIX = "criterion-lib_cmp-D"
PRECISION_ARTIFACT = "lib-cmp-precision-results"

# Every storage width the sweep matrices cover (see bench-full.yml).
EXPECTED_WIDTHS = 13

# The paths this refresh is allowed to change; the final report and the
# exit-code decision look only at these.
OWNED_PATHS = ["README.md", "docs", "results"]


def log(step: str, msg: str) -> None:
    print(f"[{step}] {msg}", flush=True)


def fail(msg: str) -> "NoReturn":  # noqa: F821
    print(f"[error] {msg}", file=sys.stderr, flush=True)
    raise SystemExit(1)


def run(cmd: list[str], *, capture: bool = False) -> subprocess.CompletedProcess:
    """Run a subprocess from the repo root; raise SystemExit(1) on failure."""
    try:
        return subprocess.run(
            cmd,
            cwd=ROOT,
            check=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            stdout=subprocess.PIPE if capture else None,
            stderr=subprocess.PIPE if capture else None,
        )
    except FileNotFoundError:
        fail(f"command not found: {cmd[0]} (is it installed and on PATH?)")
    except subprocess.CalledProcessError as exc:
        if capture and exc.stderr:
            print(exc.stderr, file=sys.stderr)
        fail(f"command failed (exit {exc.returncode}): {' '.join(cmd)}")


def gh_json(cmd: list[str]) -> object:
    out = run(cmd, capture=True).stdout
    try:
        return json.loads(out)
    except json.JSONDecodeError:
        fail(f"could not parse JSON from: {' '.join(cmd)}")


# --- step 1: discover ------------------------------------------------------


def list_runs(workflow: str, ref: str) -> list[dict]:
    """Newest-first successful runs of `workflow` whose head branch is `ref`."""
    return gh_json([
        "gh", "run", "list",
        "--workflow", workflow,
        "--branch", ref,
        "--status", "success",
        "--limit", "25",
        "--json", "databaseId,createdAt,headSha,displayTitle",
    ])


def run_artifacts(run_id: int) -> list[dict]:
    data = gh_json([
        "gh", "api",
        f"repos/{{owner}}/{{repo}}/actions/runs/{run_id}/artifacts?per_page=100",
    ])
    return data.get("artifacts", [])


def find_run_with_artifacts(workflow: str, ref: str, prefix: str,
                            label: str, dispatch_hint: str) -> dict:
    """Newest successful run of `workflow` on `ref` that still has live
    (non-expired) artifacts whose names start with `prefix`. Returns
    {"id", "sha", "created", "names"} or exits 1 with the hint."""
    runs = list_runs(workflow, ref)
    for r in runs:
        arts = run_artifacts(r["databaseId"])
        names = sorted(
            a["name"] for a in arts
            if a["name"].startswith(prefix) and not a.get("expired", False)
        )
        if names:
            return {
                "id": r["databaseId"],
                "sha": r["headSha"],
                "created": r["createdAt"],
                "names": names,
            }
    fail(
        f"no successful {workflow} run with live '{prefix}*' artifacts "
        f"found on ref '{ref}' ({label}). Dispatch one and wait for it:\n"
        f"    {dispatch_hint}"
    )


def discover(ref: str, want_timing: bool, want_precision: bool,
             want_figures: bool) -> dict:
    found: dict = {}
    if want_timing:
        log("discover", f"looking for full_matrix artifacts on '{ref}'...")
        found["full_matrix"] = find_run_with_artifacts(
            BENCH_FULL_WORKFLOW, ref, FULL_MATRIX_PREFIX, "timing tables",
            f"gh workflow run {BENCH_FULL_WORKFLOW} --ref {ref} "
            f"-f bench_family=full_matrix",
        )
        info = found["full_matrix"]
        log("discover", f"full_matrix: run {info['id']} "
                        f"({info['created']}, {info['sha'][:10]}, "
                        f"{len(info['names'])} width artifacts)")
        if len(info["names"]) < EXPECTED_WIDTHS:
            log("discover", f"WARNING: only {len(info['names'])}/"
                            f"{EXPECTED_WIDTHS} width artifacts present — "
                            f"missing widths will render as em-dashes")
    if want_figures:
        log("discover", f"looking for lib_cmp artifacts on '{ref}'...")
        found["lib_cmp"] = find_run_with_artifacts(
            BENCH_FULL_WORKFLOW, ref, LIB_CMP_PREFIX, "comparison figures",
            f"gh workflow run {BENCH_FULL_WORKFLOW} --ref {ref} "
            f"-f bench_family=lib_cmp",
        )
        info = found["lib_cmp"]
        log("discover", f"lib_cmp: run {info['id']} "
                        f"({info['created']}, {info['sha'][:10]}, "
                        f"{len(info['names'])} width artifacts)")
    if want_precision:
        log("discover", f"looking for precision artifacts on '{ref}'...")
        found["precision"] = find_run_with_artifacts(
            PRECISION_WORKFLOW, ref, PRECISION_ARTIFACT, "precision TSVs",
            f"gh workflow run {PRECISION_WORKFLOW} --ref {ref}",
        )
        info = found["precision"]
        log("discover", f"precision: run {info['id']} "
                        f"({info['created']}, {info['sha'][:10]})")
    return found


# --- step 2: download ------------------------------------------------------


def download(run_id: int, pattern: str, dest: Path) -> None:
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    log("download", f"run {run_id} pattern '{pattern}' -> {dest}")
    run([
        "gh", "run", "download", str(run_id),
        "--pattern", pattern,
        "--dir", str(dest),
    ])


# --- steps 3-7 --------------------------------------------------------------


def refresh_timing(artifacts_dir: Path) -> None:
    log("timing", "filling docs/benchmarks.md sections 1-3 from the "
                  "full_matrix artifacts...")
    run([sys.executable, "scripts/full_matrix_ingest.py",
         "--artifacts", str(artifacts_dir), "--fill"])
    log("timing", "done. REMINDER: update the 'Bench machine ... full_matrix "
                  "sweep' provenance note in docs/benchmarks.md by hand "
                  "(version + date).")


def refresh_precision(artifacts_dir: Path) -> None:
    src = artifacts_dir / PRECISION_ARTIFACT
    if not src.is_dir():
        fail(f"downloaded precision artifact dir not found: {src}")
    tsvs = sorted(src.glob("*.tsv"))
    if not tsvs:
        fail(f"no *.tsv files inside the precision artifact at {src}")
    dest = ROOT / "results" / "precision"
    dest.mkdir(parents=True, exist_ok=True)
    for tsv in tsvs:
        shutil.copyfile(tsv, dest / tsv.name)
        log("precision", f"refreshed results/precision/{tsv.name}")
    # REPORT.md inside the artifact is a render, not a tracked source —
    # deliberately not copied.


def refresh_figures(artifacts_dir: Path) -> None:
    medians = ROOT / "target" / "medians.tsv"
    medians.parent.mkdir(parents=True, exist_ok=True)
    if not medians.exists():
        medians.write_text("", encoding="utf-8")
    log("figures", "building target/medians.tsv from the lib_cmp artifacts...")
    run([sys.executable, "scripts/lib_cmp_ingest.py",
         "--artifacts", str(artifacts_dir),
         "--existing", str(medians),
         "--out", str(medians)])
    log("figures", "rendering docs/figures/library_comparison/*.png via "
                   "chart_gen (release build — this can take several "
                   "minutes cold)...")
    run(["cargo", "run", "--release", "--example", "chart_gen",
         "--features", "wide,x-wide,xx-wide,macros"])


def render() -> None:
    log("render", "filling the GENERATED doc regions (render_docs.py)...")
    run([sys.executable, "scripts/render_docs.py"])


def verify() -> list[str]:
    """Re-run the docs-drift gate's check and return the list of changed
    owned files awaiting review."""
    log("verify", "re-running the docs-drift check (render_docs.py --check)...")
    proc = subprocess.run(
        [sys.executable, "scripts/render_docs.py", "--check"],
        cwd=ROOT, text=True, encoding="utf-8", errors="replace",
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
    )
    print(proc.stdout, end="")
    if proc.returncode != 0:
        fail("docs-drift check failed AFTER rendering — a GENERATED "
             "region did not converge; inspect render_docs.py output above")
    out = run(["git", "status", "--porcelain", "--"] + OWNED_PATHS,
              capture=True).stdout
    changed = [line[3:].strip() for line in out.splitlines() if line.strip()]
    return changed


def write_summary(path: Path, ref: str, found: dict, steps: list[str],
                  changed: list[str] | None) -> None:
    lines = ["# Pre-release docs refresh", "",
             f"Ref: `{ref}`", "", "## Source runs", ""]
    if not found:
        lines.append("(none — all artifact-driven steps were skipped)")
    for fam, info in found.items():
        lines.append(f"- **{fam}**: run `{info['id']}` "
                     f"({info['created']}, commit `{info['sha'][:10]}`)")
    lines += ["", "## Steps executed", ""]
    lines += [f"- {s}" for s in steps]
    if changed is not None:
        lines += ["", "## Files changed (pending review)", ""]
        lines += ([f"- `{p}`" for p in changed] or ["(none)"])
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    log("summary", f"wrote {path.relative_to(ROOT)}")


# --- main --------------------------------------------------------------------


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--ref", default=None,
                    help="branch whose workflow runs to pull artifacts from "
                         "(default: the current git branch)")
    ap.add_argument("--work-dir", default="tmp/release_docs_refresh",
                    help="scratch dir for downloaded artifacts "
                         "(default: %(default)s, git-ignored)")
    ap.add_argument("--dry-run", action="store_true",
                    help="discover runs and print the plan; change nothing")
    ap.add_argument("--figures", action="store_true",
                    help="ALSO re-render the library-comparison figures "
                         "(downloads the lib_cmp artifacts and runs the "
                         "chart_gen example via cargo — a release build, "
                         "several minutes cold)")
    ap.add_argument("--skip-timing", action="store_true",
                    help="skip the docs/benchmarks.md timing-table refresh")
    ap.add_argument("--skip-precision", action="store_true",
                    help="skip the results/precision/*.tsv refresh")
    ap.add_argument("--skip-render", action="store_true",
                    help="skip the render_docs.py regeneration pass")
    ap.add_argument("--skip-verify", action="store_true",
                    help="skip the final drift check + change report")
    args = ap.parse_args()

    ref = args.ref
    if ref is None:
        ref = run(["git", "rev-parse", "--abbrev-ref", "HEAD"],
                  capture=True).stdout.strip()
    work_dir = (ROOT / args.work_dir).resolve()

    want_timing = not args.skip_timing
    want_precision = not args.skip_precision
    want_figures = args.figures

    plan = [
        ("discover + download artifacts",
         want_timing or want_precision or want_figures),
        ("timing tables (full_matrix_ingest --fill)", want_timing),
        ("precision TSVs (results/precision/*.tsv)", want_precision),
        ("comparison figures (lib_cmp_ingest + chart_gen)", want_figures),
        ("render docs (render_docs.py)", not args.skip_render),
        ("verify (drift check + change report)", not args.skip_verify),
    ]
    log("plan", f"ref = {ref}; work dir = {work_dir}")
    for name, on in plan:
        log("plan", f"{'RUN ' if on else 'SKIP'}  {name}")

    found: dict = {}
    if want_timing or want_precision or want_figures:
        found = discover(ref, want_timing, want_precision, want_figures)

    if args.dry_run:
        log("dry-run", "plan above; discovered run IDs: " + (
            ", ".join(f"{k}={v['id']}" for k, v in found.items()) or "none"))
        log("dry-run", "no files were downloaded or modified.")
        return 0

    steps_run: list[str] = []
    if "full_matrix" in found:
        download(found["full_matrix"]["id"], f"{FULL_MATRIX_PREFIX}*",
                 work_dir / "full_matrix")
        refresh_timing(work_dir / "full_matrix")
        steps_run.append(f"timing tables from run {found['full_matrix']['id']}")
    if "precision" in found:
        download(found["precision"]["id"], PRECISION_ARTIFACT,
                 work_dir / "precision")
        refresh_precision(work_dir / "precision")
        steps_run.append(f"precision TSVs from run {found['precision']['id']}")
    if "lib_cmp" in found:
        download(found["lib_cmp"]["id"], f"{LIB_CMP_PREFIX}*",
                 work_dir / "lib_cmp")
        refresh_figures(work_dir / "lib_cmp")
        steps_run.append(f"comparison figures from run {found['lib_cmp']['id']}")
    if not args.skip_render:
        render()
        steps_run.append("render_docs.py (GENERATED regions)")

    changed: list[str] | None = None
    if not args.skip_verify:
        changed = verify()
        steps_run.append("drift check (render_docs.py --check) — clean")

    write_summary(work_dir / "SUMMARY.md", ref, found, steps_run, changed)

    if changed is None:
        log("done", "refresh complete (verify skipped — inspect the tree "
                    "with `git status` yourself).")
        return 0
    if changed:
        log("done", f"refresh complete — {len(changed)} file(s) changed, "
                    f"left in the working tree for review (NOT committed):")
        for p in changed:
            log("done", f"  {p}")
        return 2
    log("done", "refresh complete — no changes; the committed docs already "
                "match the latest artifacts.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
