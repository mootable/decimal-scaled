# Preview the MkDocs Material documentation site locally, exactly as the `docs.yml`
# workflow builds it: fill the single-sourced generated regions (scripts/render_docs.py),
# stage the root pages the nav references under docs/ (MkDocs only reads docs/), then
# serve (live, auto-reloading) or build a static site. The staged copies are build artifacts
# and are removed again on exit, so the working tree stays clean.
#
# Usage (from bash or PowerShell, run from anywhere in the repo):
#   powershell.exe -NoProfile -File scripts/preview_docs.ps1                 # live preview at http://127.0.0.1:8000
#   powershell.exe -NoProfile -File scripts/preview_docs.ps1 -Build          # static build into ./_site (no server)
#   powershell.exe -NoProfile -File scripts/preview_docs.ps1 -Strict         # fail on broken links/nav (matches CI)
#   powershell.exe -NoProfile -File scripts/preview_docs.ps1 -- -a 127.0.0.1:9000   # pass extra args through to mkdocs
#
# -Build  : `mkdocs build --site-dir _site` instead of serving.   -Strict : add `--strict` (CI parity).
# Trailing args after `--` are forwarded to mkdocs verbatim.
# Needs (once):  pip install mkdocs-material pymdown-extensions
param(
    [switch]$Build,
    [switch]$Strict,
    [Parameter(ValueFromRemainingArguments = $true)] $MkdocsArgs
)

$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
$docs = Join-Path $repo 'docs'
# Root pages the nav pulls in; MkDocs reads only under docs/, so copy them in for the
# build. Keep in sync with the staging step in .github/workflows/docs.yml (the only
# other copy of this list).
$nav = 'ROADMAP.md', 'CHANGELOG.md', 'ALGORITHMS.md', 'CONTRIBUTORS.md', 'SECURITY.md'

# Resolve how to invoke mkdocs: prefer the console script on PATH, else fall back to
# `python -m mkdocs` (on Windows pip often installs the Scripts dir off PATH).
$mkExe = $null; $mkPre = @()
if (Get-Command mkdocs -ErrorAction SilentlyContinue) {
    $mkExe = 'mkdocs'
}
else {
    & python -c "import mkdocs" 2>$null
    if ($LASTEXITCODE -eq 0) { $mkExe = 'python'; $mkPre = @('-m', 'mkdocs') }
    else {
        Write-Host "mkdocs not found (neither on PATH nor as a python module). Install it once with:" -ForegroundColor Yellow
        Write-Host "    pip install mkdocs-material pymdown-extensions"
        exit 1
    }
}

Push-Location $repo
$copied = @()
try {
    # 1. Single-source the generated doc regions from Cargo.toml / docs/_data / results/precision.
    python scripts/render_docs.py

    # 2. Stage the root nav pages under docs/ (build artifacts; removed in `finally`).
    foreach ($f in $nav) {
        $src = Join-Path $repo $f
        if (Test-Path $src) {
            Copy-Item $src (Join-Path $docs $f) -Force
            $copied += $f
        }
    }

    # 3. Serve (live, auto-reload) or build a static site.
    $extra = @(); if ($MkdocsArgs) { $extra += $MkdocsArgs }
    if ($Build) {
        $a = @('build', '--site-dir', '_site'); if ($Strict) { $a += '--strict' }
        & $mkExe @mkPre @a @extra
        Write-Host "Built into $(Join-Path $repo '_site') - open _site\index.html." -ForegroundColor Green
    }
    else {
        $a = @('serve'); if ($Strict) { $a += '--strict' }
        Write-Host "Serving docs at http://127.0.0.1:8000  (Ctrl-C to stop)" -ForegroundColor Green
        & $mkExe @mkPre @a @extra
    }
}
finally {
    # Remove only the copies we staged, so the working tree never carries them.
    foreach ($f in $copied) {
        $dst = Join-Path $docs $f
        if (Test-Path $dst) { Remove-Item $dst -Force }
    }
    Pop-Location
}
