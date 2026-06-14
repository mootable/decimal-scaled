# Run a `cargo bench` pinned to one (or more) CPU core(s) via Windows ProcessorAffinity, so
# concurrent microbenches don't contend and each measurement is clean. Child output streams to
# the console (the caller's shell captures it). No `cmd`, no `start` (those pop a Windows error
# dialog through the bash->cmd boundary; this uses powershell.exe Start-Process directly).
#
# Usage (from bash):
#   powershell.exe -NoProfile -File scripts/pin_run.ps1 -Core 6 -Bench sqr_low_u128_ab
#   powershell.exe -NoProfile -File scripts/pin_run.ps1 -Core 8 -Bench exp_wide_series_tang_ab -Features "wide x-wide xx-wide bench-alt"
#   powershell.exe -NoProfile -File scripts/pin_run.ps1 -Mask 0xC0 -Bench mul_kernel_ab -Extra "--no-run"   # cores 6+7
#
# -Core N  : pin to logical core N (mask = 1 shl N).   -Mask M : explicit bitmask (decimal or 0xHEX).
# -Bench   : the [[bench]] name (e.g. sqr_low_u128_ab).  -Features : cargo feature string.  -Extra : extra cargo args.
# Read the bench's own criterion "A/B verdict" line for success — not the exit code.
param(
    [int]$Core = -1,
    [string]$Mask = "",
    [Parameter(Mandatory = $true)][string]$Bench,
    [string]$Features = "wide x-wide xx-wide bench-alt",
    [string]$Extra = ""
)

if ($Mask -ne "") {
    $affinity = if ($Mask -like "0x*") { [Convert]::ToInt64($Mask.Substring(2), 16) } else { [int64]$Mask }
} elseif ($Core -ge 0) {
    $affinity = [int64]1 -shl $Core
} else {
    Write-Error "pin_run.ps1: supply -Core <N> or -Mask <M>"; exit 2
}

# Start-Process does not auto-quote array elements that contain spaces, so the
# multi-word feature string would be split into stray args. Wrap it in literal
# double-quotes so cargo sees one --features value.
$feat = '"' + $Features + '"'

# Pinning the cargo process pins its child rustc swarm too — a one-core compile
# crawls. So compile UNPINNED (all cores), then run the cached bench exe PINNED.
# That keeps the *measurement* contention-free without serialising the build.
$wantsNoRun = ($Extra -match '(^|\s)--no-run(\s|$)')

# Step 1: build unpinned (full machine). Always safe; if up-to-date it's a no-op.
$build = @('bench', '--features', $feat, '--bench', $Bench, '--no-run')
Write-Host "[pin_run] (unpinned build) cargo $($build -join ' ')"
$bp = Start-Process -FilePath cargo -ArgumentList $build -PassThru -NoNewWindow -Wait
if ($bp.ExitCode -ne 0) { exit $bp.ExitCode }
if ($wantsNoRun) { exit 0 }   # caller only wanted the compile

# Step 2: run pinned (the cached exe inherits the single-core affinity).
$a = @('bench', '--features', $feat, '--bench', $Bench)
if ($Extra -ne "") { $a += ($Extra -split ' ') }
Write-Host "[pin_run] (pinned run) cargo $($a -join ' ')  (affinity mask $affinity)"
$p = Start-Process -FilePath cargo -ArgumentList $a -PassThru -NoNewWindow
$p.ProcessorAffinity = [IntPtr]$affinity
$p.WaitForExit()
if ($null -eq $p.ExitCode) { exit 0 } else { exit $p.ExitCode }
