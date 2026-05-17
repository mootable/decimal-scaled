#!/usr/bin/env python3
"""Parse criterion stdout from one or more bench-log files into the
`target/medians.tsv` shape that `examples/chart_gen.rs` consumes.

Each criterion bench line looks like:

    lib_cmp/192bit_s28/decimal-scaled/mul   time:   [102.06 ns 103.51 ns 105.00 ns]
    arith/D9_s0/add        time:   [422.34 ps 425.18 ps 428.12 ps]

The middle number in the bracket is criterion's median estimate; we
emit one TSV row per bench:

    <source>\t<id>\t<median_value>\t<unit>

`source` is the bench-binary stem (e.g. `library_comparison`,
`full_matrix`), inferred from the log filename. `id` is the
slash-form name (preserving the structure chart_gen.rs expects).

Usage:
    python scripts/bench_log_to_medians.py \
        /tmp/full_matrix_bench2.log /tmp/lib_cmp_bench.log \
        > target/medians.tsv
"""
import os
import re
import sys

# Bench lines we care about: either inline ("<name>  time: [...]") or
# split across two lines ("<name>\n<spaces>time: [...]"). The
# bracket payload always has the same shape: low-bound median up-bound,
# each with its own unit suffix.
TIME_RE = re.compile(
    r"\[\s*([0-9.]+)\s*\S+\s+([0-9.]+)\s*(ps|ns|µs|us|ms|s)\s+([0-9.]+)\s*\S+\s*\]"
)

# Bench-id lines look like "arith/D9_s0/add" or
# "lib_cmp/192bit_s28/decimal-scaled/mul". They start at column 0
# and are made of [A-Za-z0-9_/-].
NAME_RE = re.compile(r"^([A-Za-z][A-Za-z0-9_/.-]+)")

def stem(path):
    base = os.path.basename(path)
    # Drop trailing ".log" and the optional "_bench" / "_bench2" suffix
    # so two re-runs of the same bench collide nicely.
    base = base[:-4] if base.endswith(".log") else base
    for suf in ("_bench2", "_bench"):
        if base.endswith(suf):
            base = base[: -len(suf)]
            break
    return base or "unknown"


def parse(path):
    out = []
    src = stem(path)
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    # Strip ANSI escapes (criterion colours).
    text = re.sub(r"\x1b\[[0-9;]*m", "", text)

    pending_name = None
    for raw in text.splitlines():
        line = raw.rstrip()
        # Inline form: "<name>  time:   [...]".
        if " time:" in line:
            parts = line.split(" time:", 1)
            name_part = parts[0].strip()
            time_part = parts[1]
            name_match = NAME_RE.match(name_part)
            if not name_match:
                # Continuation line (the name was on the previous line).
                if pending_name is not None and TIME_RE.search(time_part):
                    m = TIME_RE.search(time_part)
                    out.append((src, pending_name, m.group(2), m.group(3)))
                    pending_name = None
                continue
            name = name_match.group(1)
            m = TIME_RE.search(time_part)
            if m:
                out.append((src, name, m.group(2), m.group(3)))
                pending_name = None
            else:
                pending_name = name
            continue
        # Possibly a bare "<name>" awaiting its time line.
        bare = NAME_RE.match(line.lstrip())
        if bare and line.lstrip().endswith(bare.group(1)):
            pending_name = bare.group(1)
    return out


def main():
    if len(sys.argv) < 2:
        sys.exit("usage: bench_log_to_medians.py <log> [<log>...]")
    rows = []
    for path in sys.argv[1:]:
        rows.extend(parse(path))
    # De-dup keeping latest occurrence (so re-runs of the same bench in
    # later logs overwrite earlier ones).
    seen = {}
    for src, name, value, unit in rows:
        seen[(src, name)] = (value, unit)
    # Force UTF-8 + LF on stdout: chart_gen.rs reads the file expecting
    # the literal "µs" bytes, and Python's default Windows code page
    # (cp1252) would mangle them.
    out = open(sys.stdout.fileno(), "w", encoding="utf-8", newline="\n", closefd=False)
    for (src, name), (value, unit) in seen.items():
        out.write(f"{src}\t{name}\t{value}\t{unit}\n")
    out.flush()


if __name__ == "__main__":
    main()
