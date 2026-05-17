#!/usr/bin/env bash
# Rebuild the bench artefacts that docs/benchmarks.md depends on,
# in the right order, after a full_matrix or library_comparison run.
#
# Inputs (any subset; pass at least one):
#   $1 ... = bench-log files from `cargo bench ... > <log>` invocations.
#
# Outputs:
#   target/medians.tsv                       — keyed by criterion id, UTF-8
#   docs/figures/library_comparison/*.png    — re-rendered line charts
#
# The TSV is regenerated from the union of the supplied logs, with
# later logs winning on duplicate keys (so a re-run of the same bench
# overrides the prior). Charts are then re-rendered from the TSV via
# `examples/chart_gen.rs`; charts where no library has ≥2 data points
# are skipped automatically (no single-dot plots).
#
# Usage:
#   scripts/refresh_bench_artifacts.sh /tmp/full_matrix_bench2.log /tmp/lib_cmp_bench.log

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "usage: $0 <bench-log> [<bench-log>...]" >&2
    exit 2
fi

cd "$(dirname "$0")/.."

mkdir -p target

echo "[1/2] writing target/medians.tsv from $# log(s)..."
python scripts/bench_log_to_medians.py "$@" > target/medians.tsv
rows=$(wc -l < target/medians.tsv)
echo "      -> ${rows} rows"

echo "[2/2] rendering docs/figures/library_comparison/*.png..."
cargo run --release --quiet --example chart_gen \
    --features "wide x-wide xx-wide" \
    2>&1 | tail -1

echo "done."
