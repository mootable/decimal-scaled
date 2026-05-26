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
$a = @('bench', '--features', ('"' + $Features + '"'), '--bench', $Bench)
if ($Extra -ne "") { $a += ($Extra -split ' ') }
Write-Host "[pin_run] cargo $($a -join ' ')  (affinity mask $affinity)"

$p = Start-Process -FilePath cargo -ArgumentList $a -PassThru -NoNewWindow
# Affinity is set just after launch; the measured bench binary is spawned later (post-compile),
# so it inherits the pin. For a fully-quiet compile too, run with -Extra "--no-run" first (unpinned-ish).
$p.ProcessorAffinity = [IntPtr]$affinity
$p.WaitForExit()
if ($null -eq $p.ExitCode) { exit 0 } else { exit $p.ExitCode }
