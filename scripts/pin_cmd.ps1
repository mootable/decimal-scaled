# Run ANY command/script pinned to one or MORE CPU core(s) via Windows
# ProcessorAffinity, so a heavy job (e.g. the mpmath golden generator) does not
# contend with pinned microbenches on other cores. Uses Start-Process directly
# (no `cmd`, no `start /affinity` -- those pop a Windows dialog through the
# bash->cmd boundary and block the caller's shell). Child output streams to the
# console (the caller's shell captures it).
#
# Usage (from bash):
#   # one core:
#   powershell.exe -NoProfile -File scripts/pin_cmd.ps1 -Cores 18 -Exe python -ExeArgs scripts/gen_golden_precision.py
#   # several cores (comma list -- OR'd into the affinity mask):
#   powershell.exe -NoProfile -File scripts/pin_cmd.ps1 -Cores 22,23 -Exe cargo -ExeArgs bench,--bench,mul_kernel_ab
#   # an arg that CONTAINS a space -- pass it as its OWN quoted -ExeArgs element:
#   powershell.exe -NoProfile -File scripts/pin_cmd.ps1 -Cores 18 -Exe python -ExeArgs script.py,"--label","a b"
#   # explicit mask instead of -Cores:
#   powershell.exe -NoProfile -File scripts/pin_cmd.ps1 -Mask 0xC00000 -Exe python -ExeArgs foo.py
#
# PARAMS
#   -Cores N[,N...] : one or more logical cores; affinity mask = OR of (1 shl N). Use a COMMA list.
#   -Mask M         : explicit affinity bitmask (decimal or 0xHEX) -- overrides -Cores.
#   -Exe            : the executable (python, cargo, node, ...). If its PATH contains spaces, quote it.
#   -ExeArgs        : the arguments, as an ARRAY -- each element is ONE argv passed verbatim.
#
# QUOTING / ESCAPING -- the easy thing to get wrong, now handled by the array:
#   * -ExeArgs is a STRING ARRAY: each element becomes exactly one argument to -Exe, with its
#     contents (INCLUDING spaces) preserved. From bash give a COMMA-separated list, quoting any
#     element individually:  -ExeArgs a,b,c   or   -ExeArgs script.py,"--label","value with spaces"
#   * Because each element is verbatim, there is NO space-splitting and NO need for inner
#     protective quotes. Do NOT wrap a whole multi-arg string in one quote -- that makes it ONE
#     arg. Split it into elements:  GOOD `-ExeArgs bench,--bench,foo`   BAD `-ExeArgs "bench --bench foo"`.
#   * A cargo `--features` value (which has spaces) must be the COMMA form `--features=a,b,c`
#     (one element, no spaces) -- see pin_bench.ps1, which builds it for you.
#
# Exit code is the child's exit code.
param(
    [string]$Cores = "",
    [string]$Mask = "",
    [Parameter(Mandatory = $true)][string]$Exe,
    [string[]]$ExeArgs = @()
)

# -Cores is a STRING ("18" or "18,19"), NOT [int[]]: powershell.exe -File coerces
# a bare comma list like 18,19 into the number 1819 (comma = thousands sep), so we
# take it as text and split it ourselves.
if ($Mask -ne "") {
    $affinity = if ($Mask -like "0x*") { [Convert]::ToInt64($Mask.Substring(2), 16) } else { [int64]$Mask }
} elseif ($Cores.Trim() -ne "") {
    $affinity = [int64]0
    foreach ($t in ($Cores -split ',')) {
        $n = $t.Trim()
        if ($n -ne "") { $affinity = $affinity -bor ([int64]1 -shl [int]$n) }
    }
} else {
    Write-Error "pin_cmd.ps1: supply -Cores <N[,N...]> or -Mask <M>"; exit 2
}

Write-Host "[pin_cmd] $Exe $($ExeArgs -join ' ')  (affinity mask $affinity)"
$p = if ($ExeArgs.Count -gt 0) {
    Start-Process -FilePath $Exe -ArgumentList $ExeArgs -PassThru -NoNewWindow
} else {
    Start-Process -FilePath $Exe -PassThru -NoNewWindow
}
# Set affinity. A trivially-fast child can exit before this runs; that race is
# harmless (it already ran on whatever core) so swallow the "process has exited"
# throw. Long jobs (the actual use -- mpmath regen, cargo bench) are alive here
# and get pinned.
try { $p.ProcessorAffinity = [IntPtr]$affinity }
catch { if (-not $p.HasExited) { throw } else { Write-Host "[pin_cmd] (child exited before affinity could be set -- too fast to pin; ran unpinned)" } }
$p.WaitForExit()
if ($null -eq $p.ExitCode) { exit 0 } else { exit $p.ExitCode }
