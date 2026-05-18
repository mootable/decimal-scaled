#requires -Version 7
# Runs a cargo bench invocation pinned to a caller-supplied CPU
# affinity mask, at High priority.
#
# Usage:
#   pwsh scripts/bench_pinned.ps1 -BenchName lib_cmp_d38 -AffinityHex 0xC00000
#
# `AffinityHex` is a hex bitmask of logical CPU indices the bench may
# run on. If omitted, defaults to the two SMT siblings of the last
# physical core (assumes 2-way SMT).
#
# Output is teed to a log file you supply via -LogFile (otherwise
# auto-named under target/bench-logs/).

param(
    [Parameter(Mandatory = $true)]
    [string] $BenchName,
    [string] $Features = "wide std",
    [string] $AffinityHex = "",
    [string] $LogFile = "",
    [string] $ExtraArgs = ""
)

$ErrorActionPreference = 'Stop'

if (-not $AffinityHex) {
    $logical = (Get-CimInstance Win32_Processor | Measure-Object -Sum NumberOfLogicalProcessors).Sum
    $lastPhys = ($logical / 2) - 1
    $mask = [int64]((1L -shl (2 * $lastPhys)) -bor (1L -shl (2 * $lastPhys + 1)))
    $AffinityHex = '0x{0:X}' -f $mask
}

if (-not $LogFile) {
    $logDir = Join-Path $PSScriptRoot '..\target\bench-logs'
    New-Item -ItemType Directory -Force -Path $logDir | Out-Null
    $ts = Get-Date -Format 'yyyyMMdd-HHmmss'
    $LogFile = Join-Path $logDir "$BenchName-$ts.log"
}

Write-Host "bench_pinned: $BenchName @ $AffinityHex -> $LogFile"

$cmd = "cargo bench --bench $BenchName --features `"$Features`" $ExtraArgs"
& cmd /c "start /wait /b /affinity $AffinityHex /high cmd /c `"$cmd 2>&1`"" 2>&1 |
    Tee-Object -FilePath $LogFile

$exit = $LASTEXITCODE
Write-Host "bench_pinned: $BenchName finished (exit $exit)"
exit $exit
