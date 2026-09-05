#!/usr/bin/env python3
"""Resolve a `bench-branch-compare` run's TIER FEATURE SET and WIDTH SELECTION
into a build plan, validating the two against each other BEFORE anything compiles.

Why this exists
---------------
Two independent needs, one shared consistency problem:

1. **A feature must not cost runtime.** `MAX_WORK_N`
   (`src/int/algos/support/limbs.rs`) is `#[cfg]`-selected by the width features
   — 2 with no tier feature, 16 at `wide`, 32 at `x-wide`, 64 at `xx-wide` — and
   the build-max scratch sizing derives from it. So merely ENABLING a tier a
   consumer never uses can change the speed of a tier they do use. bbc has
   always compiled every feature, so no bbc number could ever see that. Making
   the feature set a dispatch input turns "enabling a feature must not slow
   execution" into a measurable CI invariant.

   The measurement is ACROSS two runs, not within one: bbc compares branch vs
   prod at the SAME features, so both sides move together and the ratio stays
   ~1.00 whatever the features are. Dispatch the SAME ref twice with different
   `features`, then compare the `prod_ns` column of the two `bbc_medians.tsv`
   artifacts cell-by-cell. Identical code, identical runner class, one variable.

2. **Width selection is a feedback loop.** An agent working D57 should not wait
   on a 60-cell sweep. Selecting widths compiles and runs only those cells.

3. **A cross-cell ratio needs its cells on one machine.** Each bench job is its
   own runner VM, so branch-vs-prod WITHIN a cell cancels machine speed, but a
   ratio BETWEEN cells does not: on identical code that null distribution runs
   p50 1.11x, p90 1.60x, p99 2.2x, max 6.8x. Width and scale monotonicity are
   read entirely across cells, so `--group` can put a whole scale grid (or a
   whole width selection) in ONE job and make those readings valid.

Both need the same guard: a width can only be benched if the feature set can
actually BUILD it. `compare_d924.rs` names `decimal_scaled::D924`, which does
not exist in a `wide`-only build. Catching that here — in seconds, with a
message naming the missing feature — beats a compile error twenty minutes in.

Nothing here is hand-maintained
-------------------------------
* The benchable widths and their scale sets are parsed out of
  `benches/compare_d*.rs` (`width_bench!("D462", D462, D462, [0, 115, ...])`),
  so the CI matrix cannot drift from what the bench targets actually declare.
* The feature ladder (`wide` -> d57..d307, `x-wide` -> wide + d462/d616,
  `xx-wide` -> x-wide + d924/d1232) is read from the crate's own `[features]`
  table and resolved with cargo's own transitive-closure semantics, so a
  re-cut of the tiers flows through without touching this file.

Usage
-----
    bbc_plan.py --features "wide,x-wide,xx-wide" --widths all
    bbc_plan.py --features none --widths D18,D38
    bbc_plan.py --features wide --widths all      # -> D18..D307, the buildable set

`--features ""` (an unset dispatch input, e.g. a `push` event) means "the
default set"; the literal `none` is how a caller asks for a narrow-only build.
`--widths all` means every width the FEATURE SET can build, not always twelve —
so `--features wide --widths all` is the whole wide surface and not an error.
An EXPLICIT width the features cannot build is always a hard error.

Writes `k=v` lines to `$GITHUB_OUTPUT` when set, and a markdown block to
`--summary PATH` when given. stdlib only.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent

# The features every bbc build carries regardless of the tier selection. They
# are NOT accepted through `--features`: `std` carries `alloc`, and the tier
# axis is the one this script parameterises; the harness baseline is fixed.
#
# `strict` used to be listed here because it decided which kernel each op
# routed to, so dropping it would have changed what was being compared rather
# than which tiers exist. The head crate no longer has that feature — there is
# one kernel per op — but the PINNED BASELINE (`prod`, an older published
# release) still does, and the `prod` dependency must still carry it.
#
# It is NOT enough that `bench-compare/Cargo.toml` keeps `strict` on that line:
# the workflow's pin step REWRITES both dep lines wholesale, so whatever is
# committed here is discarded at run time. Removing `strict` from this tuple
# therefore stripped it from prod as well, and because the baseline's bare
# names are `#[cfg(all(feature = "strict", not(feature = "fast")))]` that did
# not slow the baseline down — it deleted 21 methods at every width and the
# build stopped compiling. The workflow now re-adds `strict` to the prod line
# only, guarded by a probe of that version's own feature list.
BASE_FEATURES = ("std",)

# The tier set the workflow has always built, and the one whose numbers are
# publishable as the tracked full-surface medians.
DEFAULT_TIERS = ("wide", "x-wide", "xx-wide")

# How the selected cells are partitioned into jobs. This is a MEASUREMENT
# choice, not a scheduling one.
#
# Every bench job is its own GitHub-hosted runner VM. Within one job branch and
# prod are measured on the same VM, so the branch/prod ratio cancels machine
# speed and that column is sound however the cells are partitioned. But a ratio
# taken BETWEEN two cells — D18<9> vs D18<13>, D462 vs D924 — compares two
# different VMs, and on identical code that null distribution runs p50 1.11x,
# p90 1.60x, p99 2.2x, max 6.8x. Any cross-cell reading needs its cells in ONE
# job:
#
#   cell   one job per (width, scale). The default and the full-sweep shape:
#          maximum fan-out, wall time = the slowest single cell. Cross-cell
#          comparison is NOT valid in this mode.
#   width  one job per width, running that width's whole scale grid in
#          sequence. Makes CROSS-SCALE ratios at a fixed width same-machine.
#   all    one job for everything selected. Also makes CROSS-WIDTH ratios
#          same-machine. Serialises every cell, so keep the width selection
#          small.
GROUPS = ("cell", "width", "all")

# Per-cell wall-clock budget for a grouped job. A cell that fans out on its own
# gets the historical 20 min; a grouped job gets this much per cell it carries,
# capped below the hosted-runner ceiling.
GROUPED_MINUTES_PER_CELL = 25
GROUPED_TIMEOUT_CAP = 340

_WIDTH_BENCH = re.compile(
    r"""width_bench!\(\s*"D(?P<width>\d+)"\s*,[^,]+,[^,]+,\s*\[(?P<scales>[^\]]*)\]""",
    re.VERBOSE,
)
_FEATURE_LINE = re.compile(r"^\s*(?P<name>[A-Za-z0-9_-]+)\s*=\s*\[(?P<deps>[^\]]*)\]")
_TIER_FEATURE = re.compile(r"^d\d+$")


class PlanError(Exception):
    """A caller-facing configuration error (bad feature, unbuildable width)."""


# ---------------------------------------------------------------- parsing


# Every benched row name, in declaration order, off the shared harness. Parsed
# rather than listed here for the same reason the widths are: a hand-kept copy
# of the op set drifts the moment a row is added, and the shard would then
# silently stop covering it.
_BENCH_ONE = re.compile(r"""bench_one!\(\s*\$c\s*,\s*"(?P<op>[A-Za-z0-9_@]+)"\s*,""")

# How many jobs each (width, scale) cell is split into along the OP axis.
#
# DEFAULT 1 — SHARDING IS OFF, AND IT SHOULD STAY OFF UNTIL ONE THING CHANGES.
#
# The machinery below works and is kept deliberately, but on the runner pool as
# it stands today sharding is a strict loss, on wall time AND on the bill.
# GitHub's documented concurrency cap for GitHub-hosted standard runners on the
# `team` plan is 60 jobs, and the default sweep is 60 cells — so the fan-out
# already fills the pool and `k` shards cannot buy `k` runners. With `B` the
# bench seconds per cell and `V` the ~22 s per-job fixed overhead (runner
# spin-up, `bbc-compiled` download, the apt install), `60k` jobs of `B/k + V`
# spread over 60 slots gives
#
#     wall  = B + k*V          billed = 60 * (B + k*V)
#
# `B` is invariant and only the overhead multiplies, so each extra shard costs
# ~22 s of wall and ~44 min of BILLED runner time per sweep for nothing
# (measured: B = 343 s, V = 22 s -> k=1 365 min/sweep, k=3 409 min/sweep).
#
# THE ONE CONDITION THAT FLIPS IT: a runner cap above 60 (it is liftable by a
# GitHub support ticket). Then the wall becomes `B/k + V` and k=3 saves ~3.8
# min. That day this is a one-input change — `op_shards: 3` on the dispatch, or
# this constant — with no other edit anywhere, which is why the code stays.
#
# THE INVARIANT, if it is ever switched on, and it is not negotiable: every row
# of one op's operand FAMILY lands in the SAME job. `ln` and `ln@hard` are
# measured in one process on one runner, so the overlaid per-op chart compares
# them on equal footing. Split a family across runners and the gap a reader sees
# between its lines carries the cross-cell machine spread instead of the operand
# difference the chart exists to show. `shard_ops` keys on the BASE op and
# `op_filter` appends an OPTIONAL `@variant` group, so co-location is
# structural — a new `@variant` row joins its base's shard automatically and
# cannot be separated by an edit here.
DEFAULT_OP_SHARDS = 1


def parse_ops(benches_dir: Path) -> list[str]:
    """Every benched row name from the shared harness, in declaration order."""
    src = benches_dir / "compare_common.rs"
    ops = [m.group("op") for m in _BENCH_ONE.finditer(src.read_text(encoding="utf-8"))]
    if not ops:
        raise PlanError(f"no `bench_one!` rows found in {src}")
    seen: dict[str, None] = {}
    for o in ops:
        seen.setdefault(o, None)
    return list(seen)


def shard_ops(ops: list[str], shards: int) -> list[list[str]]:
    """Partition the BASE ops into `shards` buckets of roughly equal ROW count.

    Row count, not op count, is the right key: criterion adapts its iteration
    count to a fixed time budget, so every row costs about the same wall time
    whatever its per-call nanoseconds. Balancing on ops would leave the bucket
    holding the 3-row families doing measurably more work.

    Greedy longest-first — each base op, heaviest family first, goes to the
    lightest bucket so far. Deterministic, and it re-balances by itself when
    rows are added instead of needing this file re-tuned."""
    if shards <= 1:
        return [sorted({o.split("@", 1)[0] for o in ops})]
    fams: dict[str, int] = {}
    for o in ops:
        fams[o.split("@", 1)[0]] = fams.get(o.split("@", 1)[0], 0) + 1
    buckets: list[list[str]] = [[] for _ in range(shards)]
    load = [0] * shards
    for base, n in sorted(fams.items(), key=lambda kv: (-kv[1], kv[0])):
        i = load.index(min(load))
        buckets[i].append(base)
        load[i] += n
    return [sorted(b) for b in buckets]


def op_filter(base_ops: list[str], scale: str) -> str:
    """A criterion name-filter selecting exactly this shard's rows at `scale`.

    Criterion treats the positional filter as a REGEX and matches it unanchored
    against `<op>_D<width>_s<scale>/<side>`, so:

      ^(alternation)(@[a-z0-9]+)?_D\\d+_s<scale>/

    `^` anchors to the row name so a bucket cannot pick up another's rows.
    The alternation is sorted LONGEST-FIRST so `log10` is not shadowed by
    `log`. `(@[a-z0-9]+)?` is what keeps a family together:
    the shard is chosen by BASE op and every variant of it comes along.
    The trailing `/` is the group/function separator, which is what stops
    `_s30/` from also matching `_s306/`.

    `scale == "all"` drops the scale pin and keeps the op pin."""
    alt = "|".join(sorted(base_ops, key=lambda s: (-len(s), s)))
    tail = r"\d+" if scale == "all" else re.escape(scale)
    return rf"^({alt})(@[a-z0-9]+)?_D\d+_s{tail}/"


def parse_widths(benches_dir: Path) -> dict[int, list[int]]:
    """`{18: [0, 4, 9, 13, 17], ...}` from the `width_bench!` invocations."""
    found: dict[int, list[int]] = {}
    for src in sorted(benches_dir.glob("compare_d*.rs")):
        m = _WIDTH_BENCH.search(src.read_text(encoding="utf-8"))
        if m is None:
            continue
        scales = [int(s) for s in m.group("scales").replace(" ", "").split(",") if s]
        found[int(m.group("width"))] = scales
    if not found:
        raise PlanError(f"no `width_bench!` targets found under {benches_dir}")
    return dict(sorted(found.items()))


def parse_features(manifest: Path) -> dict[str, list[str]]:
    """The crate's `[features]` table as `{name: [implied, ...]}`."""
    table: dict[str, list[str]] = {}
    in_section = False
    for line in manifest.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped.startswith("["):
            in_section = stripped == "[features]"
            continue
        if not in_section:
            continue
        m = _FEATURE_LINE.match(line)
        if m is None:
            continue
        deps = [d.strip().strip('"') for d in m.group("deps").split(",")]
        # `dep:serde` activates an optional dependency, never another feature.
        table[m.group("name")] = [d for d in deps if d and not d.startswith("dep:")]
    if "wide" not in table:
        raise PlanError(f"no usable `[features]` table in {manifest}")
    return table


def closure(features: dict[str, list[str]], requested: list[str]) -> frozenset[str]:
    """Cargo's transitive feature closure of `requested`."""
    seen: set[str] = set()
    stack = list(requested)
    while stack:
        f = stack.pop()
        if f in seen:
            continue
        seen.add(f)
        stack.extend(features.get(f, ()))
    return frozenset(seen)


# ---------------------------------------------------------------- resolution


def tier_features(features: dict[str, list[str]]) -> list[str]:
    """Every feature that enables at least one width tier, in manifest order.

    Derived, not listed: a feature qualifies when it is a `d<NNN>` width gate or
    its closure reaches one. That is exactly the set meaningful on the tier axis
    (`wide`, `x-wide`, `xx-wide`, `d57` .. `d1232`) and it excludes `std`,
    `strict`, `fast`, `_wide-support` and the rest without naming them.
    """
    out = []
    for name in features:
        if _TIER_FEATURE.match(name) or any(
            _TIER_FEATURE.match(f) for f in closure(features, [name])
        ):
            out.append(name)
    return out


def normalise_tiers(raw: str, features: dict[str, list[str]]) -> list[str]:
    """Validate + canonicalise the `--features` input into a tier list.

    Empty means "the default set" (a `push` event carries no inputs); the
    literal `none` is the explicit narrow-only request. The result is deduped
    and ordered by the manifest, so `xx-wide,wide` and `wide,xx-wide` are one
    configuration rather than two.
    """
    text = "".join(raw.split()).lower()
    if not text:
        return list(DEFAULT_TIERS)
    if text in ("none", "narrow"):
        return []

    allowed = tier_features(features)
    wanted = [p for p in text.split(",") if p]
    for p in wanted:
        if p in allowed:
            continue
        if p in features or p in BASE_FEATURES:
            raise PlanError(
                f"`{p}` is not a TIER feature. This input selects the width tiers "
                f"only; {' and '.join(BASE_FEATURES)} are always applied and the "
                f"rest of the crate's features are out of scope here. "
                f"Allowed: {', '.join(allowed)} (or `none` for narrow-only)."
            )
        raise PlanError(
            f"unknown feature `{p}`. Allowed: {', '.join(allowed)} "
            f"(or `none` for narrow-only)."
        )
    return [f for f in allowed if f in wanted]


def enablers(features: dict[str, list[str]], gate: str) -> list[str]:
    """Tier features that would enable `gate`, narrowest closure first."""
    cands = [f for f in tier_features(features) if gate in closure(features, [f])]
    return sorted(cands, key=lambda f: (len(closure(features, [f])), f))


def build_include(
    group: str,
    selected: list[int],
    scale_sets: dict[int, list[int]],
    shards: list[list[str]] | None = None,
) -> list[dict[str, object]]:
    """Partition the selected cells into matrix entries, one entry per job.

    Every entry carries the same fields whatever the mode — `widths` and
    `scales` as csv (`scales: all` meaning that width's whole set), an `ops`
    criterion filter (empty = every row), plus a `name` used for the job title
    and the artifact — so the bench step is ONE loop and does not branch on the
    mode.

    `shards` splits each cell along the OP axis into that many jobs, which is
    how the fan-out is widened without touching the measurement: the whole
    surface is still measured, just spread over more runners. Every row of one
    op's FAMILY stays in one job (see `DEFAULT_OP_SHARDS`). With one shard the
    `ops` filter is empty and the names come out as `D18-s0`, exactly as they
    have always been.
    """
    parts: list[tuple[str, list[str]]] = [("", [])]
    if shards and len(shards) > 1:
        parts = [(f"-g{i + 1}", ops) for i, ops in enumerate(shards)]

    def entry(name, widths, scales, cells, ops):
        return {
            "name": name,
            "widths": widths,
            "scales": scales,
            "cells": cells,
            "ops": op_filter(ops, scales) if ops else "",
        }

    if group == "cell":
        return [
            entry(f"D{w}-s{s}{sfx}", f"D{w}", str(s), 1, ops)
            for w in selected
            for s in scale_sets[w]
            for sfx, ops in parts
        ]
    if group == "width":
        return [
            entry(f"D{w}-all{sfx}", f"D{w}", "all", len(scale_sets[w]), ops)
            for w in selected
            for sfx, ops in parts
        ]
    return [
        entry(
            f"all{sfx}",
            ",".join(f"D{w}" for w in selected),
            "all",
            sum(len(scale_sets[w]) for w in selected),
            ops,
        )
        for sfx, ops in parts
    ]


def resolve(
    raw_features: str,
    raw_widths: str,
    benches_dir: Path,
    manifest: Path,
    group: str = "cell",
    op_shards: int = DEFAULT_OP_SHARDS,
) -> dict[str, object]:
    group = ("".join(group.split()) or "cell").lower()
    if group not in GROUPS:
        raise PlanError(
            f"unknown grouping `{group}`. Use one of: {', '.join(GROUPS)} "
            f"(`cell` = one job per cell, the default; `width` = one job per "
            f"width so cross-SCALE ratios are same-machine; `all` = one job for "
            f"everything selected, so cross-WIDTH ratios are too)."
        )

    if op_shards < 1:
        raise PlanError(f"op_shards must be >= 1, got {op_shards}")

    features = parse_features(manifest)
    scale_sets = parse_widths(benches_dir)
    shards = shard_ops(parse_ops(benches_dir), op_shards)

    tiers = normalise_tiers(raw_features, features)
    active = closure(features, tiers)

    def buildable(width: int) -> bool:
        # A width is gated by its own `d<NNN>` feature when the crate declares
        # one; the narrow tiers (D18/D38) have no gate and are always present.
        gate = f"d{width}"
        return gate not in features or gate in active

    known = list(scale_sets)
    text = "".join(raw_widths.split()).upper()
    if not text or text == "ALL":
        selected = [w for w in known if buildable(w)]
        if not selected:
            raise PlanError(
                "the feature set builds no benchable width at all - "
                f"resolved tiers: [{', '.join(tiers) or 'none'}]"
            )
    else:
        selected = []
        problems = []
        for part in [p for p in text.split(",") if p]:
            if not part.startswith("D") or not part[1:].isdigit():
                raise PlanError(
                    f"`{part}` is not a width. Use e.g. D57 or D18,D38,D57 "
                    f"(known: {', '.join('D%d' % w for w in known)}), or `all`."
                )
            w = int(part[1:])
            if w not in scale_sets:
                raise PlanError(
                    f"unknown width `{part}`. Known: "
                    f"{', '.join('D%d' % k for k in known)}, or `all`."
                )
            if not buildable(w):
                opts = enablers(features, f"d{w}")
                problems.append(
                    f"D{w} cannot be built with features "
                    f"[{', '.join(tiers) or 'none'}] - it needs one of: "
                    f"{', '.join(opts)}."
                )
                continue
            if w not in selected:
                selected.append(w)
        if problems:
            raise PlanError(
                "width/feature mismatch - every one of these would fail to "
                "compile:\n  "
                + "\n  ".join(problems)
                + "\nAdd the feature, or drop the width from `widths`."
            )
        if not selected:
            raise PlanError("no widths selected")
        selected.sort()

    include = build_include(group, selected, scale_sets, shards)
    all_features = list(BASE_FEATURES) + tiers

    # A grouped job runs its cells in sequence, so it needs a budget per cell
    # rather than the per-cell fan-out's flat 20 minutes.
    max_cells = max(int(e["cells"]) for e in include)
    timeout = (
        20
        if max_cells == 1
        else min(GROUPED_TIMEOUT_CAP, GROUPED_MINUTES_PER_CELL * max_cells)
    )

    # The published medians must describe the FULL default surface, measured the
    # way that surface has always been measured. A partial sweep, a reduced
    # feature set, or a different cell-to-runner partition is a different
    # measurement wearing the same filename. The feature test keys on the
    # resolved CLOSURE, not the typed string, so `xx-wide` alone — which builds
    # byte-identically to the default — still counts as default.
    is_default = (
        active == closure(features, list(DEFAULT_TIERS))
        and selected == known
        and group == "cell"
        and op_shards == DEFAULT_OP_SHARDS
    )

    return {
        "tiers": tiers,
        "group": group,
        "features_csv": ",".join(all_features),
        "features_toml": ", ".join(f'"{f}"' for f in all_features),
        "widths_csv": ",".join(f"D{w}" for w in selected),
        "bench_args": " ".join(f"--bench compare_d{w}" for w in selected),
        "matrix": json.dumps({"include": include}, separators=(",", ":")),
        "cell_count": str(sum(int(e["cells"]) for e in include)),
        "job_count": str(len(include)),
        "bench_timeout": str(timeout),
        "op_shards": str(op_shards),
        "op_shard_map": " | ".join(
            f"g{i + 1}: {', '.join(b)}" for i, b in enumerate(shards)
        ),
        "is_default": "true" if is_default else "false",
    }


# ---------------------------------------------------------------- reporting


GROUP_MEANING = {
    "cell": (
        "one job per cell. Branch-vs-prod within a cell is valid; a ratio "
        "**between** cells is not — those cells ran on different runner VMs."
    ),
    "width": (
        "one job per width, whole scale grid in sequence. Cross-SCALE ratios "
        "at a fixed width are same-machine and therefore valid."
    ),
    "all": (
        "one job for everything selected. Cross-scale AND cross-width ratios "
        "are same-machine."
    ),
}


def render_summary(plan: dict[str, object]) -> str:
    default = plan["is_default"] == "true"
    group = str(plan["group"])
    lines = [
        "### bench-branch-compare — run plan",
        "",
        "| | |",
        "|---|---|",
        f"| tier features | `{', '.join(plan['tiers']) or 'none (narrow-only)'}` |",
        f"| full feature set | `{plan['features_csv']}` (both branch AND prod) |",
        f"| widths | `{plan['widths_csv']}` |",
        f"| grouping | `{group}` — {GROUP_MEANING[group]} |",
        f"| cells / jobs | {plan['cell_count']} cells in {plan['job_count']} job(s) |",
        f"| bench job timeout | {plan['bench_timeout']} min |",
        f"| publishes medians | {'yes' if default else '**no — artifact only**'} |",
        "",
    ]
    if default:
        lines += [
            "_Full default surface: the timing medians are published as the "
            "tracked data, exactly as before._",
        ]
    else:
        lines += [
            "_PARAMETERISED RUN — the medians are **not** published; they are in "
            "the `bbc-aggregate` artifact only. These numbers describe a "
            "different build and/or a subset of the surface, so they are not "
            "comparable to the tracked full-surface medians and would corrupt "
            "the rendered Performance page if committed over them._",
            "",
            "_To test that enabling a feature costs no runtime: dispatch the "
            "SAME ref again with a different `features`, then compare the "
            "`prod_ns` column of the two `bbc_medians.tsv` artifacts. bbc's own "
            "branch/prod ratio cannot see it — both sides carry the same "
            "features and move together._",
        ]
    return "\n".join(lines) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--features", default="", help="tier features, csv; `none` = narrow-only")
    ap.add_argument("--widths", default="all", help="widths, csv (e.g. D57) or `all`")
    ap.add_argument(
        "--group",
        default="cell",
        help="cell | width | all — how cells are partitioned into runner jobs",
    )
    ap.add_argument("--benches-dir", default=str(HERE / "benches"))
    ap.add_argument("--manifest", default=str(ROOT / "Cargo.toml"))
    ap.add_argument("--summary", help="write the markdown run-plan block here")
    ap.add_argument(
        "--op-shards",
        type=int,
        default=DEFAULT_OP_SHARDS,
        help=(
            "split each cell into this many jobs along the OP axis "
            f"(default {DEFAULT_OP_SHARDS}; 1 = one job per cell as before). "
            "An op's whole operand family always stays in one job."
        ),
    )
    args = ap.parse_args()

    try:
        plan = resolve(
            args.features,
            args.widths,
            Path(args.benches_dir),
            Path(args.manifest),
            args.group,
            args.op_shards,
        )
    except PlanError as exc:
        # `::error::` renders as an annotation on the run; the plain copy keeps
        # the message readable in a local invocation and in the raw log.
        first, *rest = str(exc).splitlines()
        print(f"::error::{first}", file=sys.stderr)
        for line in rest:
            print(f"  {line}", file=sys.stderr)
        return 1

    out = os.environ.get("GITHUB_OUTPUT")
    if out:
        with open(out, "a", encoding="utf-8") as f:
            for k, v in plan.items():
                if k == "tiers":
                    continue
                f.write(f"{k}={v}\n")

    if args.summary:
        Path(args.summary).write_text(render_summary(plan), encoding="utf-8")

    print(f"tier features : {', '.join(plan['tiers']) or 'none (narrow-only)'}")
    print(f"feature set   : {plan['features_csv']}")
    print(f"widths        : {plan['widths_csv']}")
    print(f"grouping      : {plan['group']}")
    print(f"cells / jobs  : {plan['cell_count']} / {plan['job_count']}")
    print(f"bench timeout : {plan['bench_timeout']} min")
    print(f"bench args    : {plan['bench_args']}")
    print(f"is_default    : {plan['is_default']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
