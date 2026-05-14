<#
.SYNOPSIS
    Release/deploy helper for the `decimal-scaled` workspace.

.DESCRIPTION
    Bumps the package version in every place it is hard-coded, runs the
    build + test gate, commits, tags, pushes, and publishes via
    `cargo release`. Pushing the `v<version>` tag triggers the GitHub
    Pages docs workflow (.github/workflows/docs.yml).

    Version is hard-coded in three files:
      * Cargo.toml            -- [package] version (line-anchored)
      * macros/Cargo.toml     -- [package] version (line-anchored)
      * README.md             -- the three install-snippet occurrences
                                 (`decimal-scaled = "<v>"` and
                                 `version = "<v>"` inside `decimal-scaled`
                                 dependency lines)

    NOTHING is mutated unless you pass -Execute. The default mode is a
    dry run: every file edit, git command, and publish command is
    printed but not run. Review the dry-run output, then re-run with
    -Execute.

.PARAMETER NewVersion
    The new semver version, e.g. 0.2.0. Required.

.PARAMETER Execute
    Actually perform the edits, git operations, and publish. Without
    this switch the script only prints what it *would* do.

.PARAMETER SkipPublish
    Perform the version bump + commit + tag + push, but skip the final
    `cargo release` publish step.

.EXAMPLE
    # Dry run -- shows everything, changes nothing:
    pwsh scripts/deploy.ps1 -NewVersion 0.2.0

.EXAMPLE
    # Real run, after the dry run looked correct:
    pwsh scripts/deploy.ps1 -NewVersion 0.2.0 -Execute
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$NewVersion,

    [switch]$Execute,

    [switch]$SkipPublish
)

$ErrorActionPreference = 'Stop'

# Resolve the workspace root (this script lives in <root>/scripts).
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$DryRun = -not $Execute
$mode = if ($DryRun) { 'DRY RUN (no changes will be made)' } else { 'EXECUTE' }
Write-Host "=== decimal-scaled deploy :: $mode ===" -ForegroundColor Cyan

# --- 1. Validate the version string -----------------------------------
if ($NewVersion -notmatch '^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$') {
    throw "NewVersion '$NewVersion' is not a valid semver string."
}

# Current version is the [package] version in the root Cargo.toml: the
# first line-anchored `version = "..."` in the file.
$cargoTomlPath = Join-Path $Root 'Cargo.toml'
$cargoToml = Get-Content $cargoTomlPath -Raw
if ($cargoToml -notmatch '(?m)^version = "(?<v>[^"]+)"') {
    throw "Could not find the [package] version line in Cargo.toml."
}
$OldVersion = $Matches['v']
Write-Host "Current version : $OldVersion"
Write-Host "New version     : $NewVersion"
if ($OldVersion -eq $NewVersion) {
    throw "New version equals current version ($OldVersion); nothing to do."
}

# --- 2. Helper: apply a regex replacement to a file -------------------
function Update-File {
    param(
        [string]$Path,
        [string]$Pattern,
        [string]$Replacement,
        [string]$Label
    )
    $full = Join-Path $Root $Path
    $content = Get-Content $full -Raw
    $updated = [regex]::Replace($content, $Pattern, $Replacement)
    if ($content -eq $updated) {
        Write-Warning "  $Path :: pattern '$Label' matched nothing -- check the file."
        return
    }
    $matchCount = ([regex]::Matches($content, $Pattern)).Count
    Write-Host "  $Path :: $Label ($matchCount occurrence(s))" -ForegroundColor Green
    if (-not $DryRun) {
        # Preserve UTF-8 without BOM.
        [System.IO.File]::WriteAllText($full, $updated, (New-Object System.Text.UTF8Encoding $false))
    } else {
        # Show a unified-ish preview of the changed lines.
        $content -split "`n" | ForEach-Object {
            if ($_ -match $Pattern) { Write-Host "      - $_" -ForegroundColor DarkRed }
        }
        $updated -split "`n" | ForEach-Object {
            if ($_ -match [regex]::Escape($NewVersion)) { Write-Host "      + $_" -ForegroundColor DarkGreen }
        }
    }
}

Write-Host "`n--- Updating version strings ---" -ForegroundColor Cyan

# Root Cargo.toml -- [package] version only (line-anchored, so dependency
# `version = "..."` keys, which are never at column 0, are untouched).
Update-File -Path 'Cargo.toml' `
    -Pattern '(?m)^version = "[^"]+"' `
    -Replacement "version = `"$NewVersion`"" `
    -Label '[package] version'

# macros/Cargo.toml -- same line-anchored [package] version.
Update-File -Path 'macros/Cargo.toml' `
    -Pattern '(?m)^version = "[^"]+"' `
    -Replacement "version = `"$NewVersion`"" `
    -Label '[package] version'

# README.md -- the install snippets. Two shapes occur:
#   decimal-scaled = "<v>"
#   decimal-scaled = { version = "<v>", ... }
Update-File -Path 'README.md' `
    -Pattern ('decimal-scaled = "' + [regex]::Escape($OldVersion) + '"') `
    -Replacement ('decimal-scaled = "' + $NewVersion + '"') `
    -Label 'install snippet (bare)'

Update-File -Path 'README.md' `
    -Pattern ('(decimal-scaled = \{ version = ")' + [regex]::Escape($OldVersion) + '(")') `
    -Replacement ('${1}' + $NewVersion + '${2}') `
    -Label 'install snippet (with-features)'

# --- 3. Helper: run or echo a command ---------------------------------
function Invoke-Step {
    param([string]$Label, [scriptblock]$Action, [string]$Display)
    Write-Host "  $Label :: $Display" -ForegroundColor Yellow
    if (-not $DryRun) {
        & $Action
        if ($LASTEXITCODE -ne 0) { throw "Step failed: $Label" }
    }
}

# --- 4. Build + test gate ---------------------------------------------
# `--all-features` would pull in `experimental-floats`, which needs the
# nightly toolchain; the gate uses the widest stable-buildable set
# instead, plus a separate strict-mode build (strict and the f64 bridge
# are mutually exclusive per build).
Write-Host "`n--- Build + test gate ---" -ForegroundColor Cyan
Invoke-Step 'cargo build (wide,macros)' {
    cargo build --features wide,macros
} 'cargo build --features wide,macros'
Invoke-Step 'cargo build (strict)' {
    cargo build --no-default-features --features alloc,strict,wide,macros
} 'cargo build --no-default-features --features alloc,strict,wide,macros'
Invoke-Step 'cargo test (wide,macros)' {
    cargo test --features wide,macros
} 'cargo test --features wide,macros'

# --- 5. Git: commit, tag, push ----------------------------------------
$tag = "v$NewVersion"
Write-Host "`n--- Git ---" -ForegroundColor Cyan
Invoke-Step 'git add' {
    git add Cargo.toml macros/Cargo.toml README.md Cargo.lock
} 'git add Cargo.toml macros/Cargo.toml README.md Cargo.lock'

Invoke-Step 'git commit' {
    git commit -m "Release $tag"
} "git commit -m `"Release $tag`""

Invoke-Step 'git tag' {
    git tag -a $tag -m "Release $tag"
} "git tag -a $tag -m `"Release $tag`""

Invoke-Step 'git push' { git push } 'git push'
Invoke-Step 'git push --tags' { git push origin $tag } "git push origin $tag"

# --- 6. Publish --------------------------------------------------------
# `cargo release` publishes both workspace crates in dependency order
# (decimal_scaled_macros before decimal-scaled). It is run *after* the
# commit/tag/push above; configure `release.toml` if you want it to skip
# its own tag/commit steps. If you prefer the plain path, swap this for:
#   cargo publish -p decimal_scaled_macros
#   cargo publish -p decimal-scaled
if ($SkipPublish) {
    Write-Host "`n--- Publish skipped (-SkipPublish) ---" -ForegroundColor Cyan
} else {
    Write-Host "`n--- Publish ---" -ForegroundColor Cyan
    Invoke-Step 'cargo release' { cargo release --execute --no-confirm } 'cargo release --execute --no-confirm'
}

# --- 7. Done -----------------------------------------------------------
Write-Host "`n=== $mode complete ===" -ForegroundColor Cyan
if ($DryRun) {
    Write-Host "Nothing was changed. Re-run with -Execute to perform the release." -ForegroundColor Cyan
} else {
    Write-Host "Released $tag. Pushing the tag triggers .github/workflows/docs.yml," -ForegroundColor Cyan
    Write-Host "which builds rustdoc + the docs/ guides and deploys them to GitHub Pages." -ForegroundColor Cyan
}
