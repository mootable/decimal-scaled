#requires -Version 7
# Two-lane parallel bench sweep for the 0.3.2 prep.
#
# Lanes (24-logical / 12-physical / 2-way SMT machine):
#   Lane A : logical CPUs 20+21 (mask 0x300000) — physical core 10
#   Lane B : logical CPUs 22+23 (mask 0xC00000) — physical core 11
# Both lanes run at High priority. The other 22 logicals stay
# available for foreground work; the OS won't schedule onto 20-23
# unless something explicitly asks for them.
#
# Resumability: each bench writes its log file at start. If you
# interrupt (power loss, Ctrl-C, lid close) and rerun, this script
# skips any bench whose log already contains a criterion-completion
# marker. Re-runs are cheap — criterion's data on disk per finished
# bench is preserved.
#
# Output: target/bench-logs/sweep-<timestamp>/ (created fresh on
# every run), and target/criterion/<group>/<func>/ which criterion
# itself manages and is the authoritative result store.

param(
    [string] $Features = "wide x-wide xx-wide",
    [string] $LaneAMask = "0x300000",
    [string] $LaneBMask = "0xC00000",
    [switch] $SkipBuild
)

$ErrorActionPreference = 'Stop'
$scriptDir = $PSScriptRoot
$pinned    = Join-Path $scriptDir 'bench_pinned.ps1'
$repoRoot  = Resolve-Path (Join-Path $scriptDir '..')

$ts = Get-Date -Format 'yyyyMMdd-HHmmss'
$sweepDir = Join-Path $repoRoot "target\bench-logs\sweep-$ts"
New-Item -ItemType Directory -Force -Path $sweepDir | Out-Null
$indexFile = Join-Path $sweepDir 'INDEX.md'

# All 26 per-width bench binaries. Order within each lane is
# longest-first so big benches start as the lane warms up.
$laneA = @(
    'full_matrix_d1231',
    'full_matrix_d923',
    'full_matrix_d615',
    'full_matrix_d461',
    'full_matrix_d307',
    'full_matrix_d230',
    'full_matrix_d153',
    'full_matrix_d114',
    'full_matrix_d76',
    'full_matrix_d56',
    'full_matrix_d38',
    'full_matrix_d18',
    'full_matrix_d9'
)
$laneB = @(
    'lib_cmp_d1231',
    'lib_cmp_d923',
    'lib_cmp_d615',
    'lib_cmp_d461',
    'lib_cmp_d307',
    'lib_cmp_d230',
    'lib_cmp_d153',
    'lib_cmp_d114',
    'lib_cmp_d76',
    'lib_cmp_d56',
    'lib_cmp_d38',
    'lib_cmp_d18',
    'lib_cmp_d9'
)

if (-not $SkipBuild) {
    Write-Host "Pre-building bench targets (release, features='$Features')…"
    & cargo build --release --benches --features "$Features" 2>&1 |
        Tee-Object -FilePath (Join-Path $sweepDir 'prebuild.log')
    if ($LASTEXITCODE -ne 0) { throw "prebuild failed — see prebuild.log" }
}

"# Bench sweep $ts" | Out-File $indexFile -Encoding utf8
"" | Out-File $indexFile -Encoding utf8 -Append
"Features: ``$Features``" | Out-File $indexFile -Encoding utf8 -Append
"Lane A (``$LaneAMask``): $($laneA -join ', ')" | Out-File $indexFile -Encoding utf8 -Append
"Lane B (``$LaneBMask``): $($laneB -join ', ')" | Out-File $indexFile -Encoding utf8 -Append
"" | Out-File $indexFile -Encoding utf8 -Append

$pinned = Resolve-Path $pinned
$sweepDir = Resolve-Path $sweepDir

# Each lane runs its bench queue sequentially, pinned to the lane's
# affinity mask. A bench is skipped if the lane log already records
# its completion (the per-bench log doesn't have a single canonical
# criterion "done" line, but the lane log emits one
# `bench_pinned: <name> finished (exit 0)` per successful bench from
# `bench_pinned.ps1` itself — that's the reliable signal). criterion's
# own per-bench data lives in target/criterion/ and is preserved
# across runs.
$runLane = {
    param($benches, $mask, $sweepDir, $features, $pinned, $repoRoot)
    Set-Location $repoRoot
    $laneLog = Join-Path $sweepDir "lane-$mask.log"
    foreach ($bench in $benches) {
        if (Test-Path $laneLog) {
            $lane = Get-Content $laneLog -Raw -ErrorAction SilentlyContinue
            if ($lane -and $lane -match [regex]::Escape("bench_pinned: $bench finished (exit 0)")) {
                Write-Output "skip $bench (already finished)"
                continue
            }
        }
        $logFile = Join-Path $sweepDir "$bench.log"
        Write-Output "starting $bench @ $mask"
        & pwsh -NoProfile -File $pinned `
            -BenchName $bench `
            -Features $features `
            -AffinityHex $mask `
            -LogFile $logFile *>&1 |
            Out-File -Append (Join-Path $sweepDir "lane-$mask.log")
        Write-Output "finished $bench (exit $LASTEXITCODE)"
    }
}

Write-Host "Starting lane A on $LaneAMask  ($($laneA.Count) benches)"
$jobA = Start-Job -Name "bench-laneA" -ScriptBlock $runLane `
    -ArgumentList @($laneA, $LaneAMask, $sweepDir, $Features, $pinned.Path, $repoRoot.Path)

Write-Host "Starting lane B on $LaneBMask  ($($laneB.Count) benches)"
$jobB = Start-Job -Name "bench-laneB" -ScriptBlock $runLane `
    -ArgumentList @($laneB, $LaneBMask, $sweepDir, $Features, $pinned.Path, $repoRoot.Path)

Write-Host ""
Write-Host "Lane A job id: $($jobA.Id) (bench-laneA)"
Write-Host "Lane B job id: $($jobB.Id) (bench-laneB)"
Write-Host "Sweep dir    : $sweepDir"
Write-Host ""
Write-Host "Waiting on both lanes — this dispatcher pwsh process will"
Write-Host "stay alive until both lanes finish (Start-Job children die"
Write-Host "with the parent). You can attach from another shell to peek"
Write-Host "at progress without affecting the run:"
Write-Host "    Get-Content -Wait '$sweepDir\<bench-name>.log'"
Write-Host ""
Write-Host "If you have to kill this process (Ctrl-C, lid close, power"
Write-Host "loss), the running cargo bench processes die too. Just rerun"
Write-Host "this script — it skips benches whose log shows completion."
Write-Host ""

# Stream output from both jobs as it's produced so the dispatcher's
# log file reflects live progress instead of buffering until end.
while (($jobA.State -eq 'Running') -or ($jobB.State -eq 'Running')) {
    Receive-Job -Id $jobA.Id
    Receive-Job -Id $jobB.Id
    Start-Sleep -Seconds 30
}

# Drain anything left after both jobs finished.
Receive-Job -Id $jobA.Id
Receive-Job -Id $jobB.Id

Write-Host ""
Write-Host "Both lanes finished."
Write-Host "Lane A state: $($jobA.State), Lane B state: $($jobB.State)"
Write-Host "Index file  : $indexFile"
