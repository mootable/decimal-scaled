# Run a `cargo bench` PINNED to one or MORE CPU core(s), so concurrent
# microbenches don't contend and each measurement is clean. Canonical bench
# pinner -- supersedes pin_run.ps1 (kept only as a legacy alias while in-flight
# agents still reference it) and bench_pinned.ps1 (removed; it used the
# `cmd /c start /affinity` path that hangs the shell).
#
# Two-step, like pin_run: BUILD unpinned (full machine -- pinning cargo would
# single-core its rustc swarm and the compile would crawl), then RUN the cached
# bench exe PINNED. The pinned run is delegated to pin_cmd.ps1 (the general
# multi-core pinner); this script only knows the cargo-bench command.
#
# Usage (from bash):
#   powershell.exe -NoProfile -File scripts/pin_bench.ps1 -Cores 22 -Bench exp_wide_series_tang_ab
#   powershell.exe -NoProfile -File scripts/pin_bench.ps1 -Cores 22,23 -Bench mul_kernel_ab        # two cores
#   powershell.exe -NoProfile -File scripts/pin_bench.ps1 -Cores 21 -Bench rem_kernel_ab -Extra "-- s38"
#   powershell.exe -NoProfile -File scripts/pin_bench.ps1 -Cores 22 -Bench sqr_low_ab -Extra "--no-run"   # build only
#
# PARAMS
#   -Cores N[,N...] : one or more logical cores to pin the RUN to (comma list). -Mask overrides.
#   -Mask M         : explicit affinity bitmask (decimal or 0xHEX).
#   -Bench          : the [[bench]] name.  (Mandatory.)
#   -Features       : cargo feature string, SPACE-separated (default "wide x-wide xx-wide bench-alt").
#                     Converted internally to the comma form `--features=a,b,c` (one space-free arg).
#   -Extra          : extra cargo args as one string (e.g. "-- s38", "--no-run"); whitespace-split.
#
# Read the bench's own criterion "A/B verdict" line for success -- NOT the exit code.
param(
    [string]$Cores = "",
    [string]$Mask = "",
    [Parameter(Mandatory = $true)][string]$Bench,
    [string]$Features = "wide x-wide xx-wide bench-alt",
    [string]$Extra = ""
)

$ErrorActionPreference = 'Stop'

# Space-separated features -> comma form (one space-free arg cargo accepts).
$featArg = "--features=" + ((($Features.Trim()) -split '\s+') -join ',')

# Step 1: build UNPINNED (full machine). Up-to-date => near-instant no-op.
Write-Host "[pin_bench] (unpinned build) cargo bench $featArg --bench $Bench --no-run"
$bp = Start-Process -FilePath cargo -ArgumentList @('bench', $featArg, '--bench', $Bench, '--no-run') -PassThru -NoNewWindow -Wait
if ($bp.ExitCode -ne 0) { exit $bp.ExitCode }
if ($Extra -match '(^|\s)--no-run(\s|$)') { exit 0 }   # caller only wanted the compile

# Step 2: run PINNED via pin_cmd.ps1 -- pass the cargo argv as an ARRAY so nothing is space-split.
$extraTokens = if ($Extra -ne "") { $Extra -split '\s+' } else { @() }
$runArgs = @('bench', $featArg, '--bench', $Bench) + $extraTokens
$pinCmd = Join-Path $PSScriptRoot 'pin_cmd.ps1'
Write-Host "[pin_bench] (pinned run) -> cargo $($runArgs -join ' ')"
if ($Mask -ne "") { & $pinCmd -Mask $Mask -Exe cargo -ExeArgs $runArgs }
else              { & $pinCmd -Cores $Cores -Exe cargo -ExeArgs $runArgs }
exit $LASTEXITCODE
