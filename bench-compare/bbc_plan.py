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
# are NOT accepted through `--features`: `strict` decides which kernel each op
# routes to, so dropping it would silently change what is being compared rather
# than which tiers exist, and `std` carries `alloc`/`exact-scratch`. The tier
# axis is the one this script parameterises; the harness baseline is fixed.
BASE_FEATURES = ("std", "strict")

# The tier set the workflow has always built, and the one whose numbers are
# publishable as the tracked full-surface medians.
DEFAULT_TIERS = ("wide", "x-wide", "xx-wide")

_WIDTH_BENCH = re.compile(
    r"""width_bench!\(\s*"D(?P<width>\d+)"\s*,[^,]+,[^,]+,\s*\[(?P<scales>[^\]]*)\]""",
    re.VERBOSE,
)
_FEATURE_LINE = re.compile(r"^\s*(?P<name>[A-Za-z0-9_-]+)\s*=\s*\[(?P<deps>[^\]]*)\]")
_TIER_FEATURE = re.compile(r"^d\d+$")


class PlanError(Exception):
    """A caller-facing configuration error (bad feature, unbuildable width)."""


# ---------------------------------------------------------------- parsing


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


def resolve(
    raw_features: str,
    raw_widths: str,
    benches_dir: Path,
    manifest: Path,
) -> dict[str, object]:
    features = parse_features(manifest)
    scale_sets = parse_widths(benches_dir)

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

    include = [{"width": f"D{w}", "scale": s} for w in selected for s in scale_sets[w]]
    all_features = list(BASE_FEATURES) + tiers

    # The published medians must describe the FULL default surface: a partial or
    # reduced-feature sweep is a different measurement wearing the same filename.
    # Keyed on the resolved CLOSURE, not the typed string, so `xx-wide` alone —
    # which builds byte-identically to the default — still counts as default.
    is_default = active == closure(features, list(DEFAULT_TIERS)) and selected == known

    return {
        "tiers": tiers,
        "features_csv": ",".join(all_features),
        "features_toml": ", ".join(f'"{f}"' for f in all_features),
        "widths_csv": ",".join(f"D{w}" for w in selected),
        "bench_args": " ".join(f"--bench compare_d{w}" for w in selected),
        "matrix": json.dumps({"include": include}, separators=(",", ":")),
        "cell_count": str(len(include)),
        "is_default": "true" if is_default else "false",
    }


# ---------------------------------------------------------------- reporting


def render_summary(plan: dict[str, object]) -> str:
    default = plan["is_default"] == "true"
    lines = [
        "### bench-branch-compare — run plan",
        "",
        "| | |",
        "|---|---|",
        f"| tier features | `{', '.join(plan['tiers']) or 'none (narrow-only)'}` |",
        f"| full feature set | `{plan['features_csv']}` (both branch AND prod) |",
        f"| widths | `{plan['widths_csv']}` |",
        f"| cells | {plan['cell_count']} |",
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
    ap.add_argument("--benches-dir", default=str(HERE / "benches"))
    ap.add_argument("--manifest", default=str(ROOT / "Cargo.toml"))
    ap.add_argument("--summary", help="write the markdown run-plan block here")
    args = ap.parse_args()

    try:
        plan = resolve(
            args.features,
            args.widths,
            Path(args.benches_dir),
            Path(args.manifest),
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
    print(f"cells         : {plan['cell_count']}")
    print(f"bench args    : {plan['bench_args']}")
    print(f"is_default    : {plan['is_default']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
