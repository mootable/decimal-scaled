#!/usr/bin/env bash
# Run samply on a criterion bench exe via the elevated `SamplyElevated`
# scheduled task — PROMPT-FREE (no per-run UAC), driven from the (non-elevated)
# Bash tool. The coordinator's headless samply runner for this repo.
#
# One-time setup (owner, elevated terminal):
#   schtasks /create /tn SamplyElevated /tr "powershell.exe -NoProfile \
#     -ExecutionPolicy Bypass -File C:\Users\jacko\RustroverProjects\decimal_scaled\trace\samply_elevated.ps1" \
#     /sc once /st 00:00 /rl highest /f
# The task (RunLevel=Highest) runs trace/samply_elevated.ps1, which reads the
# 3-line job spec trace/samply_job.txt (exe / criterion-filter / out-profile) and runs:
#   samply record --save-only --unstable-presymbolicate -o <out> -- <exe> <filter>
#
# Usage: bash scripts/run_samply.sh <exe-path> <criterion-filter> [out.json.gz]
#
# NOTE 1: ALWAYS trigger schtasks via powershell.exe — git-bash mangles bare
#         `/run` `/tn` into POSIX paths.
# NOTE 2: --save-only does NOT resolve Rust symbols; frames come back fun_<hex>
#         (only OS exports resolve). Use the printed criterion A/B verdict for
#         relative cost; for NAMED Rust frames run interactively (opens Firefox):
#           samply record -- <exe> <filter>
set -u
BASE="C:/Users/jacko/RustroverProjects/decimal_scaled"
EXE="${1:?usage: run_samply.sh <exe-path> <criterion-filter> [out.json.gz]}"
FILTER="${2:?usage: run_samply.sh <exe-path> <criterion-filter> [out.json.gz]}"
OUT="${3:-$BASE/trace/prof_samply.json.gz}"
towin() { sed 's#/#\\#g' <<<"$1"; }
printf '%s\n%s\n%s\n' "$(towin "$EXE")" "$FILTER" "$(towin "$OUT")" > "$BASE/trace/samply_job.txt"
rm -f "$BASE/trace/samply_task_out.txt" "$OUT" 2>/dev/null
powershell.exe -NoProfile -Command "schtasks /run /tn SamplyElevated" >/dev/null 2>&1
echo "triggered SamplyElevated; polling for DONE (up to ~2 min)..."
for _ in $(seq 1 60); do grep -aq "DONE exit" "$BASE/trace/samply_task_out.txt" 2>/dev/null && break; sleep 2; done
echo "=== A/B verdict + exit (task output) ==="
tr -d '\000' < "$BASE/trace/samply_task_out.txt" 2>/dev/null | grep -aE "verdict|DONE exit|ERROR" | tail -16
if [ -f "$OUT" ]; then
  echo "=== profile: $OUT (sidecar: ${OUT%.json.gz}.json.syms.json) ==="
  python3 -c "import json,sys; s=json.load(open('${OUT%.json.gz}.json.syms.json')); t=s.get('string_table',[]); f=sum(1 for x in t if x.startswith('fun_')); print(f'symbols: {len(t)-f} resolved / {f} unresolved fun_  (Rust frames do NOT resolve headless — see NOTE 2)')" 2>/dev/null
else
  echo "NO PROFILE written — check trace/samply_task_out.txt"
fi
