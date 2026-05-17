"""Load a Chrome-trace JSON (e.g. produced by tracing-chrome) into
Perfetto's trace_processor and print a per-span aggregate showing
where time is spent across the whole trace.

Usage:  python scripts/perfetto_section_breakdown.py trace/exp_perfetto.json
"""

import sys
import argparse
from perfetto.trace_processor import TraceProcessor


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("trace")
    args = p.parse_args()

    tp = TraceProcessor(trace=args.trace)
    try:
        rows = tp.query(
            """
            SELECT name,
                   COUNT(*)              AS calls,
                   SUM(dur)   / 1000.0   AS total_us,
                   AVG(dur)   / 1000.0   AS avg_us,
                   MIN(dur)   / 1000.0   AS min_us,
                   MAX(dur)   / 1000.0   AS max_us
            FROM slice
            GROUP BY name
            ORDER BY total_us DESC
            """
        )
        print(f"{'name':<35} {'calls':>8} {'total_us':>12} {'avg_us':>10} {'min_us':>10} {'max_us':>10}")
        print("-" * 90)
        for r in rows:
            print(
                f"{r.name:<35} {r.calls:>8} {r.total_us:>12.1f} {r.avg_us:>10.3f} "
                f"{r.min_us:>10.3f} {r.max_us:>10.3f}"
            )
    finally:
        tp.close()


if __name__ == "__main__":
    main()
