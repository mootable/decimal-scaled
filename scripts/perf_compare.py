#!/usr/bin/env python3
"""Side-by-side performance comparison: a feature branch against the published docs.

Renders the SAME charts the docs site renders -- it imports `_perf_svg` and
`_perf_series` from `scripts/render_docs.py` rather than approximating them -- so
what you see here is directly comparable to
https://mootable.github.io/decimal-scaled/performance/ .

Reads `results/timing/bbc_medians.tsv` straight out of git, so it needs no bench
run, no cargo, and no network beyond a fetch. Both refs must have had a bbc run.

    python scripts/perf_compare.py                    # main vs perf/monotonicity
    python scripts/perf_compare.py --branch my/branch
    python scripts/perf_compare.py --watch            # refresh as sweeps land
    python scripts/perf_compare.py --no-fetch         # offline, use local refs

Output: research/perf_compare.html -- open it in a browser. `research/` is
git-ignored, so the generated page stays local while this generator is tracked.

THE RULE THIS PAGE EXISTS TO CHECK (owner): less work must not cost more.
A narrower width must not be slower than a wider one at the same scale, and a
lower scale must not be slower than a higher one at the same width. Every such
inversion is a DEFECT, and the fix target is the NARROW side. The counts at the
top of the page are the score.

CAVEAT built into the page, do not remove it: bbc runs each (width, scale) cell
on its OWN GitHub runner, so a cross-cell ratio carries a machine-to-machine
floor (p90 1.60x, p99 2.2x on identical code). That governs whether an inversion
is VISIBLE, never whether it is acceptable. And scale-0 cells are degenerate for
trig/hyperbolic ops -- `op_str!` substitutes the integer form, so the small
operand `0.1` becomes literally "0". Scale 0 is excluded by default; --with-zero
puts it back.
"""
from __future__ import annotations

import argparse
import datetime as _dt
import hashlib
import math as _math
import html
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

import render_docs as rd  # noqa: E402  (needs the path above)

CATEGORIES = {
    "Arithmetic": ["add", "sub", "mul", "div", "rem", "neg", "abs", "quantize"],
    "Roots and exponents": ["sqrt", "cbrt", "exp", "expm1", "ln", "log1p", "log",
                            "log2", "log10", "exp2", "powf", "powi", "hypot"],
    "Trigonometry": ["sin", "cos", "tan", "asin", "acos", "atan", "atan2",
                     "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
                     "to_radians", "to_degrees"],
}


def sh(*args: str) -> str:
    return subprocess.run(args, cwd=ROOT, capture_output=True, text=True,
                          check=True).stdout


def tsv_at(ref: str) -> str:
    return sh("git", "show", f"{ref}:results/timing/bbc_medians.tsv")




def parse(text: str) -> dict[tuple[str, int, int], float]:
    """`(op, width, scale) -> branch_ns`, the column the docs page renders."""
    out = {}
    for line in text.splitlines()[1:]:
        c = line.split("\t")
        if len(c) >= 5:
            w = c[1].lstrip("D")
            if w.isdigit() and c[2].lstrip("-").isdigit():
                try:
                    out[(c[0], int(w), int(c[2]))] = float(c[4])
                except ValueError:
                    pass
    return out


def parse_pair(text: str) -> dict[tuple[str, int, int], tuple[float, float]]:
    """`(op, width, scale) -> (prod_ns, branch_ns)` from ONE run.

    This is the trustworthy comparison and the reason these two columns exist:
    both numbers are measured in the SAME job on the SAME runner, so machine
    speed cancels exactly. Comparing a cell across two runs (the charts above)
    spans two runner VMs and carries a p90 1.60x / p99 2.2x floor; this does not.
    """
    out = {}
    for line in text.splitlines()[1:]:
        c = line.split("\t")
        if len(c) >= 5:
            w = c[1].lstrip("D")
            if w.isdigit() and c[2].lstrip("-").isdigit():
                try:
                    out[(c[0], int(w), int(c[2]))] = (float(c[3]), float(c[4]))
                except ValueError:
                    pass
    return out


# The measured cross-cell null on IDENTICAL code: two cells come off two runner
# VMs, so a ratio between them carries this much spread before any code differs.
# p90 1.60x, p99 2.2x, max 6.8x over 1620 cells. An inversion below p99 is not
# dismissed -- the owner's rule is that an inversion is a defect at any magnitude
# -- but it cannot be DISTINGUISHED from runner jitter in a single cell-mode
# sweep, so the panel reports the count at both bands rather than one number that
# is mostly coin-flips on 2 ns ops.
NULL_P50, NULL_P90, NULL_P99 = 1.11, 1.60, 2.2


# RELATIVE ONLY: a move counts when it is more than 1% of the shipped time.
#
# The repo's 8.4 filter also requires |delta| > 10 ns, and that absolute half is
# deliberately NOT applied here (owner's call: some noise is acceptable, a blind
# spot is not). Every narrow-tier cell is single-digit nanoseconds, so a 10 ns
# floor censors D18 and D38 almost entirely — precisely where this campaign is
# working. It would discard a 3 ns -> 6 ns doubling as unassessable while
# reporting a 1% wobble on a microsecond cell. The 1% test scales with the cell
# and keeps the narrow tiers visible; the cost is some noise in the counts, which
# is why the mean and max columns are there to size what the counts contain.
FLOOR_PCT = 0.01


def assessable(prod: float, branch: float) -> bool:
    """One predicate, used by every count on this page, so they cannot diverge."""
    if prod <= 0 or branch <= 0:
        return False
    return abs(prod - branch) / prod > FLOOR_PCT


def movers(pairs: dict, min_scale: int, n: int) -> tuple[list, list]:
    """Top `n` improvements and regressions, branch vs the shipped release."""
    rows = []
    for (op, w, s), (prod, branch) in pairs.items():
        if s < min_scale or not assessable(prod, branch):
            continue
        rows.append((prod / branch, op, w, s, prod, branch))
    rows.sort(reverse=True)
    gains = [r for r in rows if r[0] > 1][:n]
    losses = sorted([r for r in rows if r[0] < 1])[:n]
    return gains, losses


def mover_table(rows: list, gain: bool) -> str:
    if not rows:
        return ("<p class='ok'>None above the 1% assessability floor.</p>"
                if not gain else "<p class='none'>none</p>")
    head = ("<table><thead><tr><th>op</th><th>cell</th>"
            "<th>shipped release</th><th>branch</th><th>change</th>"
            "<th>absolute</th></tr></thead><tbody>")
    body = []
    for ratio, op, w, s, prod, branch in rows:
        factor = ratio if gain else 1 / ratio
        body.append(
            f"<tr><td><code>{html.escape(op)}</code></td>"
            f"<td>D{w}&lt;{s}&gt;</td><td>{fmt(prod)}</td><td>{fmt(branch)}</td>"
            f"<td class='{'win' if gain else 'loss'}'>{factor:.2f}x "
            f"{'faster' if gain else 'slower'}</td>"
            f"<td>{fmt(abs(prod - branch))}</td></tr>")
    return head + "".join(body) + "</tbody></table>"


def history(branch: str, min_scale: int, limit: int = 12) -> list[tuple]:
    """Campaign progress, free: every full sweep already self-commits the
    medians, so the snapshots are sitting in git history.

    Each point trends `branch vs the shipped release` WITHIN one snapshot. That
    is the only comparison worth trending: both columns of a snapshot come from
    the same job on the same runner, so machine speed cancels. Trending absolute
    nanoseconds across snapshots would be comparing different runner VMs and
    would mostly plot GitHub's fleet.
    """
    try:
        log = sh("git", "log", f"--format=%H\t%ad\t%s", "--date=format:%m-%d %H:%M",
                 branch, "--", "results/timing/bbc_medians.tsv")
    except subprocess.CalledProcessError:
        return []

    # ONE BASELINE ONLY. bbc pins `prod` to the latest PUBLISHED release at run
    # time, so the baseline moves the moment a release ships. Trending across a
    # release boundary makes shipping look like a regression: the 0.5.0 release
    # snapshot measured against 0.4.4 and showed 1,079 cells faster, while
    # post-0.5.1 snapshots measure against 0.5.1 and show 577 -- the code did not
    # get slower, the bar moved. Walk back only as far as the newest release
    # commit and stop, so every point in the series shares a baseline.
    lines = []
    for line in log.splitlines():
        if line.split("\t")[-1].startswith("Release "):
            break
        lines.append(line)

    out = []
    for line in lines[:limit]:
        sha, _, rest = line.partition("\t")
        when = rest.split("\t")[0]
        try:
            text = sh("git", "show", f"{sha}:results/timing/bbc_medians.tsv")
        except subprocess.CalledProcessError:
            continue
        pairs = parse_pair(text)
        ups, dns, logs = [], [], []
        for (_op, _w, s), (prod, branch_ns) in pairs.items():
            if s < min_scale or prod <= 0 or branch_ns <= 0:
                continue
            # The overall figure spans EVERY cell, not just the assessable ones:
            # dropping sub-1% moves would bias it away from 1.0 by keeping only
            # the large changes. This is the whole surface's average effect.
            logs.append(_math.log(prod / branch_ns))
            if not assessable(prod, branch_ns):
                continue
            # Both directions expressed as a factor >= 1, so "mean" and "max"
            # read the same way on each row and cannot be confused for each
            # other's reciprocal.
            (ups if branch_ns < prod else dns).append(
                prod / branch_ns if branch_ns < prod else branch_ns / prod)
        wi, si = inversions(parse(text), min_scale)
        band = lambda rows, f: sum(1 for r in rows if r[5] >= f)
        mean = lambda v: sum(v) / len(v) if v else 1.0

        def pct(v, q):
            """Percentile of a ratio distribution. Median and p90 say what the
            typical and near-worst case actually are; a mean is dragged around by
            a single 36x outlier and describes no cell in particular."""
            if not v:
                return 1.0
            s = sorted(v)
            return s[min(len(s) - 1, int(q * (len(s) - 1) + 0.5))]
        gmean = _math.exp(sum(logs) / len(logs)) if logs else 1.0
        out.append((sha[:8], when, len(ups), len(dns), len(wi), len(si),
                    mean(ups), max(ups, default=1.0),
                    mean(dns), max(dns, default=1.0),
                    gmean, len(ups) - len(dns),
                    band(wi, NULL_P50), band(wi, NULL_P90), band(wi, NULL_P99),
                    band(si, NULL_P50), band(si, NULL_P90), band(si, NULL_P99),
                    pct(ups, 0.5), pct(ups, 0.9), pct(dns, 0.5), pct(dns, 0.9),
                    pct(ups, 0.10), pct(ups, 0.25), pct(ups, 0.75),
                    pct(dns, 0.10), pct(dns, 0.25), pct(dns, 0.75),
                    # The inversion RATIO distributions: how severe a typical
                    # inversion is, which the band counts cannot say.
                    *(pct([r[5] for r in wi], q)
                      for q in (0.10, 0.25, 0.50, 0.75, 0.90)),
                    max((r[5] for r in wi), default=1.0),
                    *(pct([r[5] for r in si], q)
                      for q in (0.10, 0.25, 0.50, 0.75, 0.90)),
                    max((r[5] for r in si), default=1.0)))
    out = list(reversed(out))  # oldest first

    # POINT ZERO: the shipped release itself, where branch == shipped, so
    # improvements and regressions are zero BY DEFINITION. Without it the series
    # starts at the first snapshot, which already contains the campaign's first
    # win -- the narrow-tier divide fix is baked into point 1 and invisible.
    #
    # Its inversion counts come from the `prod_ns` column of the NEWEST snapshot.
    # That column is the shipped release measured in that same job, so it needs
    # no extra run and carries no cross-runner error: it is literally the
    # baseline as that machine saw it.
    if out:
        try:
            newest = sh("git", "show",
                        f"{lines[0].split(chr(9))[0]}:results/timing/bbc_medians.tsv")
            shipped = {k: p for k, (p, _b) in parse_pair(newest).items() if p > 0}
            wi0, si0 = inversions(shipped, min_scale)
            b0 = lambda rows, f: sum(1 for r in rows if r[5] >= f)

            def pct0(v, q):
                if not v:
                    return 1.0
                s = sorted(v)
                return s[min(len(s) - 1, int(q * (len(s) - 1) + 0.5))]
            out.insert(0, ("shipped", "0.5.1", 0, 0, len(wi0), len(si0),
                           1.0, 1.0, 1.0, 1.0, 1.0, 0,
                           b0(wi0, NULL_P50), b0(wi0, NULL_P90), b0(wi0, NULL_P99),
                           b0(si0, NULL_P50), b0(si0, NULL_P90), b0(si0, NULL_P99),
                           1.0, 1.0, 1.0, 1.0,
                           1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                           *(pct0([r[5] for r in wi0], q)
                             for q in (0.10, 0.25, 0.50, 0.75, 0.90)),
                           max((r[5] for r in wi0), default=1.0),
                           *(pct0([r[5] for r in si0], q)
                             for q in (0.10, 0.25, 0.50, 0.75, 0.90)),
                           max((r[5] for r in si0), default=1.0)))
        except subprocess.CalledProcessError:
            pass
    return out


def spark(vals: list[int], good_low: bool) -> str:
    """A bare inline sparkline. No library, no external request."""
    if len(vals) < 2:
        return ""
    lo, hi = min(vals), max(vals)
    span = (hi - lo) or 1
    w, h = 132, 26
    pts = " ".join(
        f"{w * i / (len(vals) - 1):.1f},{h - 3 - (h - 6) * (v - lo) / span:.1f}"
        for i, v in enumerate(vals))
    end_good = (vals[-1] <= vals[0]) == good_low
    colour = "var(--win)" if end_good else "var(--loss)"
    return (f'<svg viewBox="0 0 {w} {h}" width="{w}" height="{h}" '
            f'style="vertical-align:middle">'
            f'<polyline points="{pts}" fill="none" stroke="{colour}" '
            f'stroke-width="1.6"/></svg>')


def history_panel(pts: list[tuple]) -> str:
    if len(pts) < 2:
        return ("<p class='none'>Needs at least two committed full sweeps on this "
                "branch — only the full default sweep self-commits medians.</p>")
    n = lambda v: f"{v:,}"
    x = lambda v: f"{v:.2f}x"
    cols = [("OVERALL — geometric mean, whole surface", 10, False, x),
            ("net cells (faster − slower)", 11, False,
             lambda v: f"{v:+,}"),
            ("cells faster than shipped", 2, False, n),
            ("  mean improvement", 6, False, x),
            ("  median (p50) improvement", 18, False, x),
            ("  p75 improvement", 24, False, x),
            ("  p90 improvement", 19, False, x),
            ("  biggest improvement", 7, False, x),
            ("cells slower", 3, True, n),
            ("  mean regression", 8, True, x),
            ("  median (p50) regression", 20, True, x),
            ("  p75 regression", 27, True, x),
            ("  p90 regression", 21, True, x),
            ("  worst regression", 9, True, x),
            ("width inversions — all", 4, True, n),
            ("  p10 severity", 28, True, x),
            ("  p25 severity", 29, True, x),
            ("  p50 severity", 30, True, x),
            ("  p75 severity", 31, True, x),
            ("  p90 severity", 32, True, x),
            ("  worst", 33, True, x),
            ("  count above 1.60x (p90 of the runner null)", 13, True, n),
            ("  count above 2.20x (p99)", 14, True, n),
            ("scale inversions — all", 5, True, n),
            ("  p10 severity", 34, True, x),
            ("  p25 severity", 35, True, x),
            ("  p50 severity", 36, True, x),
            ("  p75 severity", 37, True, x),
            ("  p90 severity", 38, True, x),
            ("  worst", 39, True, x),
            ("  count above 1.60x (p90 of the runner null)", 16, True, n),
            ("  count above 2.20x (p99)", 17, True, n)]
    head = ("<table><thead><tr><th>metric</th><th>trend</th><th>first</th>"
            "<th>now</th><th>change</th></tr></thead><tbody>")
    body = []
    for label, idx, good_low, fmt_v in cols:
        vals = [p[idx] for p in pts]
        d = vals[-1] - vals[0]
        good = (d <= 0) == good_low
        cls = "win" if d and good else ("loss" if d else "flat")
        # Delta formatting follows the ROW's own kind, not the identity of a
        # shared helper: a count row deltas as a count, a ratio row as a ratio.
        delta = f"{d:+.2f}x" if fmt_v is x else f"{d:+,}"
        indent = " style='padding-left:1.6rem;color:var(--dim)'" if label.startswith("  ") else ""
        body.append(
            f"<tr><td{indent}>{label.strip()}</td><td>{spark(vals, good_low)}</td>"
            f"<td>{fmt_v(vals[0])}</td><td><strong>{fmt_v(vals[-1])}</strong></td>"
            f"<td class='{cls}'>{delta}</td></tr>")
    sweeps = sum(1 for p in pts if p[0] != "shipped")
    span = f"{pts[0][1]} &rarr; {pts[-1][1]}"
    return (head + "".join(body) + "</tbody></table>"
            f"<div class='cap'>Baseline plus {sweeps} committed full "
            f"{'sweep' if sweeps == 1 else 'sweeps'}, {span}. The baseline is the "
            f"shipped release against itself, so it is zero improvements by "
            f"definition; its inversion counts come from the `prod_ns` column of "
            f"the newest sweep, which is that release measured on the same "
            f"machine. Every point compares branch against shipped WITHIN one "
            f"job, so runner speed cancels.</div>")


def svg_for(rows: dict, op: str) -> str:
    """The docs site's own chart, for one op."""
    op_rows = [(o, w, s, ns) for (o, w, s), ns in rows.items() if o == op]
    if not op_rows:
        return "<p class='none'>no data</p>"
    widths, P, series = rd._perf_series(op_rows)
    return rd._perf_svg(widths, P, series) or "<p class='none'>not enough data</p>"


def inversions(rows: dict, min_scale: int) -> tuple[list, list]:
    """Every cell where cost fails to track work.

    width: same op and scale, a NARROWER width slower than a wider one.
    scale: same op and width, a LOWER scale slower than a higher one.
    Adjacent pairs only -- a full cross product would report the same defect
    many times over and drown the real count.
    """
    cells = {k: v for k, v in rows.items() if k[2] >= min_scale}
    by_op_scale, by_op_width = {}, {}
    for (op, w, s), ns in cells.items():
        by_op_scale.setdefault((op, s), {})[w] = ns
        by_op_width.setdefault((op, w), {})[s] = ns

    width_inv = []
    for (op, s), d in sorted(by_op_scale.items()):
        ws = sorted(d)
        for a, b in zip(ws, ws[1:]):
            if d[a] > d[b]:
                width_inv.append((op, f"D{a} vs D{b}", f"s{s}", d[a], d[b], d[a] / d[b]))

    scale_inv = []
    for (op, w), d in sorted(by_op_width.items()):
        ss = sorted(d)
        for a, b in zip(ss, ss[1:]):
            if d[a] > d[b]:
                scale_inv.append((op, f"D{w}", f"s{a} vs s{b}", d[a], d[b], d[a] / d[b]))

    width_inv.sort(key=lambda r: -r[5])
    scale_inv.sort(key=lambda r: -r[5])
    return width_inv, scale_inv


def fmt(ns: float) -> str:
    unit, power = rd._unit_of(ns)
    return f"{ns / 10 ** power:.3g} {unit}"


def inv_table(rows: list, kind: str, limit: int) -> str:
    if not rows:
        return f"<p class='ok'>None. Cost tracks work on every adjacent {kind} pair.</p>"
    # Name the two time columns after WHICH CELL they hold, not "slower"/"faster".
    # A row only exists when the inequality is violated, so the first cell named
    # in `pair` is always the offending one -- but the reader should not have to
    # infer that from the ordering.
    if kind == "width":
        c1, c2 = "narrower tier (should be cheaper)", "wider tier"
    else:
        c1, c2 = "lower scale (should be cheaper)", "higher scale"
    head = (f"<table><thead><tr><th>op</th><th>pair</th><th>at</th>"
            f"<th>{c1}</th><th>{c2}</th><th>ratio</th></tr></thead><tbody>")
    body = "".join(
        f"<tr><td><code>{html.escape(o)}</code></td><td>{html.escape(p)}</td>"
        f"<td>{html.escape(a)}</td><td>{fmt(hi)}</td><td>{fmt(lo)}</td>"
        f"<td class='{'big' if r >= 2.2 else 'small'}'>{r:.2f}x</td></tr>"
        for o, p, a, hi, lo, r in rows[:limit])
    more = (f"<tr><td colspan='6' class='none'>... and {len(rows) - limit} more</td></tr>"
            if len(rows) > limit else "")
    return head + body + more + "</tbody></table>"


def delta_table(base: dict, br: dict, op: str) -> str:
    keys = sorted({k for k in set(base) | set(br) if k[0] == op}, key=lambda k: (k[1], k[2]))
    if not keys:
        return ""
    out = ["<table class='delta'><thead><tr><th>cell</th><th>published</th>"
           "<th>branch</th><th>change</th></tr></thead><tbody>"]
    for k in keys:
        b, n = base.get(k), br.get(k)
        if b is None or n is None:
            continue
        r = n / b
        cls = "win" if r < 0.95 else ("loss" if r > 1.05 else "flat")
        arrow = "faster" if r < 1 else ("slower" if r > 1 else "same")
        out.append(f"<tr><td>D{k[1]}&lt;{k[2]}&gt;</td><td>{fmt(b)}</td><td>{fmt(n)}</td>"
                   f"<td class='{cls}'>{1 / r:.2f}x {arrow}</td></tr>"
                   if r < 1 else
                   f"<tr><td>D{k[1]}&lt;{k[2]}&gt;</td><td>{fmt(b)}</td><td>{fmt(n)}</td>"
                   f"<td class='{cls}'>{r:.2f}x {arrow}</td></tr>")
    return "".join(out) + "</tbody></table>"


CSS = """
:root{
  --bg:#fbfbfa; --fg:#1a1a18; --dim:#6b6b66; --line:#e2e2dd; --card:#fff;
  --win:#1a7f4b; --loss:#b3261e; --warn:#8a6100;
  /* the docs site's SVGs reference mkdocs Material variables; supply them or
     every chart renders invisible */
  --md-primary-fg-color:#4051b5; --md-default-fg-color--light:#6b6b66;
}
@media (prefers-color-scheme:dark){:root:not([data-theme="light"]){
  --bg:#16161a; --fg:#e8e8e3; --dim:#9a9a93; --line:#2e2e34; --card:#1e1e23;
  --win:#4ade80; --loss:#f87171; --warn:#fbbf24;
  --md-primary-fg-color:#8b9bff; --md-default-fg-color--light:#9a9a93;
}}
:root[data-theme="dark"]{
  --bg:#16161a; --fg:#e8e8e3; --dim:#9a9a93; --line:#2e2e34; --card:#1e1e23;
  --win:#4ade80; --loss:#f87171; --warn:#fbbf24;
  --md-primary-fg-color:#8b9bff; --md-default-fg-color--light:#9a9a93;
}
body{background:var(--bg);color:var(--fg);font:14px/1.55 -apple-system,
  BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;margin:0;padding:2rem 1.5rem 5rem}
.wrap{max-width:none;margin:0}
h1{font-size:1.6rem;margin:0 0 .3rem} h2{font-size:1.15rem;margin:2.4rem 0 .8rem;
  padding-bottom:.35rem;border-bottom:1px solid var(--line)}
h3{font-size:.95rem;margin:1.6rem 0 .5rem;font-family:ui-monospace,monospace}
.sub{color:var(--dim);margin:0 0 1.6rem}
.score{display:flex;gap:1rem;flex-wrap:wrap;margin:1.2rem 0 2rem}
.box{flex:1;min-width:190px;background:var(--card);border:1px solid var(--line);
  border-radius:8px;padding:.9rem 1rem}
.box .n{font-size:1.9rem;font-weight:600;line-height:1.1}
.box .l{color:var(--dim);font-size:.8rem;margin-top:.2rem}
.pair{display:grid;grid-template-columns:1fr 1fr;gap:1rem;align-items:start}
@media(max-width:820px){.pair{grid-template-columns:1fr}}
.chart{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:.7rem}
.chart .cap{color:var(--dim);font-size:.78rem;margin-bottom:.3rem}
table{border-collapse:collapse;width:100%;font-size:.82rem;margin:.4rem 0 1rem}
th,td{text-align:left;padding:.32rem .55rem;border-bottom:1px solid var(--line)}
th{color:var(--dim);font-weight:500} td:nth-child(n+4){text-align:right;
  font-variant-numeric:tabular-nums}
.win{color:var(--win)} .loss{color:var(--loss)} .flat{color:var(--dim)}
.big{color:var(--loss);font-weight:600} .small{color:var(--warn)}
.ok{color:var(--win)} .none{color:var(--dim)}
.note{background:var(--card);border-left:3px solid var(--warn);padding:.8rem 1rem;
  border-radius:0 6px 6px 0;margin:1.4rem 0;font-size:.86rem}
.scroll{overflow-x:auto} details{margin:.5rem 0} summary{cursor:pointer;color:var(--dim)}
.panel{display:grid;grid-template-columns:1fr 1fr;gap:1rem;margin:1rem 0 2rem}
@media(max-width:1100px){.panel{grid-template-columns:1fr}}
.panel section{background:var(--card);border:1px solid var(--line);border-radius:8px;
  padding:.8rem 1rem;min-width:0}
.panel section h3{margin:.1rem 0 .2rem;font-family:inherit;font-size:.92rem}
.panel section .cap{color:var(--dim);font-size:.76rem;margin-bottom:.4rem}
.panel table{font-size:.76rem} .panel th,.panel td{padding:.24rem .4rem}
.hist{background:var(--card);border:1px solid var(--line);border-radius:8px;
  padding:.8rem 1rem;margin:.6rem 0 2rem}
.hist table{font-size:.84rem;margin:0} .hist td:nth-child(2){text-align:left}
.hist .cap{color:var(--dim);font-size:.76rem;margin-top:.5rem}
"""


def build(base: dict, br: dict, pairs: dict, hist: list, base_ref: str,
          br_ref: str, min_scale: int, limit: int,
          source: str = "committed medians", refresh: int = 0) -> str:
    bw, bs = inversions(base, min_scale)
    nw, ns_ = inversions(br, min_scale)
    gains, losses = movers(pairs, min_scale, limit)

    def d(a, b):
        if b == a:
            return "<span class='flat'>no change</span>"
        cls = "win" if b < a else "loss"
        return f"<span class='{cls}'>{b - a:+d}</span>"

    ops = sorted({k[0] for k in set(base) | set(br)})
    body = []
    for cat, names in CATEGORIES.items():
        present = [o for o in names if o in ops]
        if not present:
            continue
        body.append(f"<h2>{cat}</h2>")
        for op in present:
            body.append(f"<h3>{html.escape(op)}</h3><div class='pair'>"
                        f"<div class='chart'><div class='cap'>published &mdash; {html.escape(base_ref)}</div>"
                        f"{svg_for(base, op)}</div>"
                        f"<div class='chart'><div class='cap'>branch &mdash; {html.escape(br_ref)}</div>"
                        f"{svg_for(br, op)}</div></div>"
                        f"<details><summary>per-cell change</summary>"
                        f"<div class='scroll'>{delta_table(base, br, op)}</div></details>")
    leftover = [o for o in ops if not any(o in v for v in CATEGORIES.values())]
    if leftover:
        body.append("<h2>Other</h2>")
        for op in leftover:
            body.append(f"<h3>{html.escape(op)}</h3><div class='pair'>"
                        f"<div class='chart'><div class='cap'>published</div>{svg_for(base, op)}</div>"
                        f"<div class='chart'><div class='cap'>branch</div>{svg_for(br, op)}</div></div>")

    stamp = _dt.datetime.now().strftime("%Y-%m-%d %H:%M")
    zero = "excluded" if min_scale > 0 else "INCLUDED"
    meta_refresh = (f'<meta http-equiv="refresh" content="{refresh}">'
                    if refresh else "")
    return f"""<!doctype html><html><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">{meta_refresh}
<title>Performance compare</title><style>{CSS}</style></head><body><div class="wrap">
<h1>Performance: branch vs published</h1>
<p class="sub">published <code>{html.escape(base_ref)}</code> &nbsp;vs&nbsp;
branch <code>{html.escape(br_ref)}</code> &middot; generated {stamp} &middot;
scale 0 {zero} &middot; branch data: {html.escape(source)}</p>

<div class="score">
  <div class="box"><div class="n">{len(nw)}</div>
    <div class="l">width inversions &mdash; a narrower tier slower than a wider one<br>
    was {len(bw)} &nbsp;({d(len(bw), len(nw))})</div></div>
  <div class="box"><div class="n">{len(ns_)}</div>
    <div class="l">scale inversions &mdash; a lower scale slower than a higher one<br>
    was {len(bs)} &nbsp;({d(len(bs), len(ns_))})</div></div>
</div>

<div class="note"><strong>An inversion is a defect, not a measurement.</strong>
If D18 is slower than D57, D18 is too slow &mdash; it does strictly less work, and
the fix target is the narrow side. Two things govern whether you can <em>see</em> one,
never whether it is acceptable: bbc runs each (width, scale) cell on its own runner,
so a cross-cell ratio carries a machine floor (p90 1.60x, p99 2.2x on identical code)
&mdash; ratios below that are shown in amber and want replicating before you hunt.
And several ops have degenerate bench operands: <code>ln</code> runs on
<code>2.0</code>, a power of two, which short-circuits its range reduction entirely.
Check the operand before trusting any single cell.</div>

<h2>Campaign progress</h2>
<div class="hist">{history_panel(hist)}</div>

<div class="panel">
  <section><h3>Width inversions &mdash; branch</h3>
    <div class="cap">a narrower tier slower than a wider one, same op and scale.
      Top {limit} by ratio, of {len(nw)}.</div>
    <div class="scroll">{inv_table(nw, "width", limit)}</div></section>
  <section><h3>Scale inversions &mdash; branch</h3>
    <div class="cap">a lower scale slower than a higher one, same op and width.
      Top {limit} by ratio, of {len(ns_)}.</div>
    <div class="scroll">{inv_table(ns_, "scale", limit)}</div></section>
  <section><h3>Biggest improvements</h3>
    <div class="cap">branch vs the shipped release, measured in the SAME job on
      the same runner &mdash; machine speed cancels, so this is clean at any
      magnitude. Moves under {FLOOR_PCT:.0%} of the shipped time excluded; the 10 ns absolute floor is deliberately NOT applied, so the narrow tiers stay visible.</div>
    <div class="scroll">{mover_table(gains, True)}</div></section>
  <section><h3>Biggest regressions</h3>
    <div class="cap">same comparison, other direction. Anything here is a real
      cost the branch added, not runner noise.</div>
    <div class="scroll">{mover_table(losses, False)}</div></section>
</div>
{''.join(body)}
</div></body></html>"""


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--base", default="origin/main", help="published side (default origin/main)")
    p.add_argument("--branch", default="origin/perf/monotonicity", help="feature branch")
    p.add_argument("--out", default=str(ROOT / "research" / "perf_compare.html"))
    p.add_argument("--with-zero", action="store_true",
                   help="include scale 0 (degenerate operands for trig/hyperbolic)")
    p.add_argument("--limit", type=int, default=20, help="rows per summary table")
    p.add_argument("--no-fetch", action="store_true")
    p.add_argument("--watch", action="store_true",
                   help="keep regenerating; picks up every completed bbc run")
    p.add_argument("--interval", type=int, default=60,
                   help="seconds between checks in --watch (default 60)")
    p.add_argument("--refresh", type=int, default=60,
                   help="page auto-reload seconds; 0 disables")
    a = p.parse_args()

    out = Path(a.out)
    out.parent.mkdir(parents=True, exist_ok=True)

    def render() -> tuple[int, str]:
        """Render once. Returns (exit_code, signature) — signature changes only
        when the underlying DATA changed, so the watch loop can stay quiet."""
        if not a.no_fetch:
            for ref in (a.base, a.branch):
                if ref.startswith("origin/"):
                    try:
                        sh("git", "fetch", "origin", ref.split("/", 1)[1], "-q")
                    except subprocess.CalledProcessError:
                        print(f"  (fetch of {ref} failed, using local)",
                              file=sys.stderr)

        try:
            base_text = tsv_at(a.base)
        except subprocess.CalledProcessError as e:
            print(f"could not read {a.base} medians: {e.stderr.strip()}",
                  file=sys.stderr)
            return 1, ""

        # Prefer the newest COMPLETED bbc run's artifact over the committed TSV:
        # parameterised runs never self-commit, so the committed file can be
        # many runs stale while the interesting measurements sit in artifacts.
        source, br_text = "committed medians (full default sweep)", None
        branch_name = a.branch.split("/", 1)[1] if a.branch.startswith("origin/") \
            else a.branch
        # FULL SWEEP vs FULL SWEEP, and git alone gives exactly that.
        #
        # Only the full default sweep self-commits
        # `results/timing/bbc_medians.tsv` -- the workflow gates the commit on
        # `!inputs.ref && is_default`, so a narrowed or reduced-feature run is
        # artifact-only by construction and can never reach the committed file.
        # The committed medians are therefore ALREADY the full-vs-full data set,
        # and reading the two refs is the whole job. Micro-mode runs never
        # produce medians at all, so they cannot appear here either.
        #
        # An earlier version walked run artifacts to pick up parameterised runs
        # sooner. That was solving the wrong problem: a narrowed run measures a
        # different surface, so putting it opposite a full baseline silently
        # changes what every count on this page means.
        if br_text is None:
            try:
                br_text = tsv_at(a.branch)
            except subprocess.CalledProcessError as e:
                print(f"could not read {a.branch} medians: {e.stderr.strip()}",
                      file=sys.stderr)
                return 1, ""

        base, br = parse(base_text), parse(br_text)
        if not base or not br:
            print("one side has no timing rows -- has bbc run on it?",
                  file=sys.stderr)
            return 1, ""

        out.write_text(
            build(base, br, parse_pair(br_text),
                  history(a.branch, 0 if a.with_zero else 1),
                  a.base, a.branch, 0 if a.with_zero else 1, a.limit,
                  source, a.refresh),
            encoding="utf-8")
        sig = hashlib.sha256((base_text + br_text).encode("utf-8")).hexdigest()
        print(f"{_dt.datetime.now():%H:%M:%S}  {len(base)} published / "
              f"{len(br)} branch cells · {source} -> {out}")
        return 0, sig

    code, sig = render()
    if not a.watch:
        return code

    print(f"watching every {a.interval}s — Ctrl+C to stop")
    while True:
        try:
            time.sleep(a.interval)
            _, new = render()
            if new and new != sig:
                print("  ^ data changed")
                sig = new
        except KeyboardInterrupt:
            print("\nstopped")
            return 0
        except Exception as e:  # a transient gh/git failure must not kill the watch
            print(f"  (skipped a tick: {e})", file=sys.stderr)


if __name__ == "__main__":
    raise SystemExit(main())
