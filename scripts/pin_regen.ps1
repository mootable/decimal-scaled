# Regenerate the mpmath golden corpus (tests/golden/*.txt) PINNED to one or
# MORE quiet CPU core(s), so the long generation does not contend with pinned
# microbenches on other cores. Thin wrapper: builds the
# `python scripts/gen_golden_precision.py ...` argv and hands it to pin_cmd.ps1
# (the general multi-core pinner) -- affinity lives there, this only knows the
# regen command. Runs from the REPO ROOT so the generator resolves tests/golden/.
#
# Usage (from bash):
#   # full corpus regen, pinned to the default core 18:
#   powershell.exe -NoProfile -File scripts/pin_regen.ps1
#   # several cores:
#   powershell.exe -NoProfile -File scripts/pin_regen.ps1 -Cores 18,19
#   # scoped regen (generator filters via -GenArgs, whitespace-split into argv):
#   powershell.exe -NoProfile -File scripts/pin_regen.ps1 -GenArgs "--only-func=exp --only-alias=d76"
#
# PARAMS
#   -Cores N[,N...] : logical core(s) to pin to (comma list; default 18 -- high, away from the
#                     low cores Windows favours and the bench cores 21-23). -Mask overrides.
#   -Mask M         : explicit affinity bitmask (decimal or 0xHEX).
#   -GenArgs        : extra args for gen_golden_precision.py as one string, whitespace-split into
#                     argv. Filters: --only-func=exp,ln  --only-alias=d307,d462  --only-scale=38
#                     Omit for the FULL corpus (default). Each filter is one space-free token.
#
# Exit code is the generator's. The COORDINATOR then reconciles the harness scale anchors,
# golden-gates (ulp_strict_golden), and commits the regenerated tables.
param(
    [string]$Cores = "18",
    [string]$Mask = "",
    [string]$GenArgs = ""
)

$ErrorActionPreference = 'Stop'
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..')
Push-Location $repoRoot
try {
    $genTokens = if ($GenArgs.Trim() -ne "") { $GenArgs.Trim() -split '\s+' } else { @() }
    $genArgv = @('scripts/gen_golden_precision.py') + $genTokens
    $pinCmd = Join-Path $PSScriptRoot 'pin_cmd.ps1'
    Write-Host "[pin_regen] golden regen -> python $($genArgv -join ' ')"
    if ($Mask -ne "") { & $pinCmd -Mask $Mask -Exe python -ExeArgs $genArgv }
    else              { & $pinCmd -Cores $Cores -Exe python -ExeArgs $genArgv }
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
