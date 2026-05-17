<#
.SYNOPSIS
    Release/deploy helper for the `decimal-scaled` workspace.

.DESCRIPTION
    Bumps the package version in every place it is hard-coded, runs the
    build + test gate, commits to `main`, pushes (triggering the docs
    deploy from the main branch), publishes both crates to crates.io
    in dependency order, then tags and pushes the tag.

    Tag-push ORDER MATTERS. The GitHub Pages environment is configured
    to allow deploys only from `main`; a tag-triggered docs run is
    blocked by environment protection rules. The 0.3.0 release shipped
    with the docs site stale because the tag push raced ahead of the
    main-push docs run and cancelled it via concurrency. 0.3.1 fixed
    this by pushing to main first, waiting for the docs deploy, and
    only then pushing the tag. This script enforces that ordering.

    Version is hard-coded in these files:
      * Cargo.toml            -- [package] version (line-anchored) and
                                 the `decimal_scaled_macros = { version = "..." }`
                                 dependency-table line.
      * macros/Cargo.toml     -- [package] version (line-anchored)
      * README.md             -- every `decimal-scaled = "<v>"` and
                                 `version = "<v>"` install snippet
      * docs/features.md      -- install snippets in the common-
                                 configurations and other sections
      * docs/getting-started.md
      * docs/macros.md
      * docs/rounding.md
      * docs/strict-mode.md

    NOTHING is mutated unless you pass -Execute. The default mode is a
    dry run: every file edit, git command, and publish command is
    printed but not run. Review the dry-run output, then re-run with
    -Execute.

.PARAMETER NewVersion
    The new semver version, e.g. 0.3.2. Required.

.PARAMETER Execute
    Actually perform the edits, git operations, and publish. Without
    this switch the script only prints what it *would* do.

.PARAMETER SkipPublish
    Perform the version bump + commit + push to main + tag + push tag,
    but skip the `cargo publish` steps. Useful when re-publishing the
    rustdoc / pages site without bumping the crates.io version.

.EXAMPLE
    # Dry run -- shows everything, changes nothing:
    pwsh scripts/deploy.ps1 -NewVersion 0.3.2

.EXAMPLE
    # Real run, after the dry run looked correct:
    pwsh scripts/deploy.ps1 -NewVersion 0.3.2 -Execute
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
    if (-not (Test-Path $full)) {
        Write-Warning "  $Path :: file not found, skipping"
        return
    }
    $content = Get-Content $full -Raw
    $updated = [regex]::Replace($content, $Pattern, $Replacement)
    if ($content -eq $updated) {
        Write-Host "  $Path :: $Label (no match -- already up to date or wrong file)" -ForegroundColor DarkYellow
        return
    }
    $matchCount = ([regex]::Matches($content, $Pattern)).Count
    Write-Host "  $Path :: $Label ($matchCount occurrence(s))" -ForegroundColor Green
    if (-not $DryRun) {
        # Preserve UTF-8 without BOM.
        [System.IO.File]::WriteAllText($full, $updated, (New-Object System.Text.UTF8Encoding $false))
    }
}

Write-Host "`n--- Updating version strings ---" -ForegroundColor Cyan

# Cargo.toml [package] version (line-anchored) + the macros sub-crate
# dependency-table line. Bumping the dep-table version too is required
# so the main crate resolves against the matching macros version on
# crates.io after publish.
Update-File -Path 'Cargo.toml' `
    -Pattern '(?m)^version = "[^"]+"' `
    -Replacement "version = `"$NewVersion`"" `
    -Label '[package] version'
Update-File -Path 'Cargo.toml' `
    -Pattern ('decimal_scaled_macros = \{ path = "macros", version = "' + [regex]::Escape($OldVersion) + '"') `
    -Replacement ('decimal_scaled_macros = { path = "macros", version = "' + $NewVersion + '"') `
    -Label 'macros dep version'

Update-File -Path 'macros/Cargo.toml' `
    -Pattern '(?m)^version = "[^"]+"' `
    -Replacement "version = `"$NewVersion`"" `
    -Label '[package] version'

# Every doc file with install snippets. The patterns cover the two
# shapes that appear: bare `decimal-scaled = "<v>"` and the
# `decimal-scaled = { version = "<v>", ... }` form.
$docFiles = @(
    'README.md',
    'docs/features.md',
    'docs/getting-started.md',
    'docs/macros.md',
    'docs/rounding.md',
    'docs/strict-mode.md'
)
foreach ($f in $docFiles) {
    Update-File -Path $f `
        -Pattern ('decimal-scaled = "' + [regex]::Escape($OldVersion) + '"') `
        -Replacement ('decimal-scaled = "' + $NewVersion + '"') `
        -Label 'install snippet (bare)'
    Update-File -Path $f `
        -Pattern ('(decimal-scaled = \{ version = ")' + [regex]::Escape($OldVersion) + '(")') `
        -Replacement ('${1}' + $NewVersion + '${2}') `
        -Label 'install snippet (with-features)'
}

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
# (every tier umbrella) instead, plus a separate strict-only build.
Write-Host "`n--- Build + test gate ---" -ForegroundColor Cyan
Invoke-Step 'cargo build (all tiers)' {
    cargo build --release --features wide,x-wide,xx-wide,macros
} 'cargo build --release --features wide,x-wide,xx-wide,macros'
Invoke-Step 'cargo build (strict opt-out -> fast)' {
    cargo build --release --no-default-features --features alloc,std,wide,x-wide,xx-wide,fast,macros
} 'cargo build --release --no-default-features --features alloc,std,wide,x-wide,xx-wide,fast,macros'
Invoke-Step 'cargo test (all tiers, release)' {
    cargo test --release --features wide,x-wide,xx-wide,macros --lib
} 'cargo test --release --features wide,x-wide,xx-wide,macros --lib'

# --- 5. Git: commit + PUSH-TO-MAIN-FIRST -----------------------------
# Reasoning for the order: GitHub Pages environment protection only
# allows deploys from `main`. A tag-triggered docs run is blocked.
# Push to main first, let the docs workflow build and deploy, THEN
# create + push the tag. The tag run can fail to deploy; that's
# expected and fine because the docs are already current.
$tag = "v$NewVersion"
Write-Host "`n--- Git: bump + push to main ---" -ForegroundColor Cyan
Invoke-Step 'git add' {
    git add Cargo.toml macros/Cargo.toml README.md docs/features.md docs/getting-started.md docs/macros.md docs/rounding.md docs/strict-mode.md Cargo.lock CHANGELOG.md
} 'git add <version-bumped files + CHANGELOG.md if updated>'
Invoke-Step 'git commit' {
    git commit -m "Release $tag"
} "git commit -m `"Release $tag`""
Invoke-Step 'git push origin main' {
    git push origin main
} 'git push origin main'

if (-not $DryRun) {
    Write-Host "`n  Pushed to main. The docs.yml workflow is now building." -ForegroundColor Cyan
    Write-Host "  Watch https://github.com/mootable/decimal-scaled/actions/workflows/docs.yml" -ForegroundColor Cyan
    Write-Host "  and confirm it deploys before continuing. (Press Enter to continue, Ctrl-C to abort.)" -ForegroundColor Cyan
    Read-Host '  >'
}

# --- 6. Publish to crates.io ------------------------------------------
# Order matters: macros first (the main crate depends on it), wait for
# the index sync, then the main crate. `cargo publish` auto-waits for
# the registry to show the new version since Cargo 1.75-ish, but if
# the second publish fails with "failed to select a version", give it
# another 30-60 seconds and retry.
if ($SkipPublish) {
    Write-Host "`n--- Publish skipped (-SkipPublish) ---" -ForegroundColor Cyan
} else {
    Write-Host "`n--- Publish to crates.io ---" -ForegroundColor Cyan
    Invoke-Step 'publish macros' {
        cargo publish -p decimal_scaled_macros
    } 'cargo publish -p decimal_scaled_macros'

    Invoke-Step 'publish main crate' {
        cargo publish --features wide,x-wide,xx-wide
    } 'cargo publish --features wide,x-wide,xx-wide'
}

# --- 7. Tag + push tag ------------------------------------------------
# Done LAST so the tag push only happens after main is healthy and
# both crates are on crates.io.
Write-Host "`n--- Git: tag + push tag ---" -ForegroundColor Cyan
Invoke-Step 'git tag' {
    git tag -a $tag -m "Release $tag"
} "git tag -a $tag -m `"Release $tag`""
Invoke-Step 'git push tag' {
    git push origin $tag
} "git push origin $tag"

# --- 8. (optional) GitHub Release -------------------------------------
# Not run automatically; the CHANGELOG.md section for this version is
# the natural release-notes source. If `gh` is available:
#   $notes = (Get-Content CHANGELOG.md -Raw) -split "(?m)^## \[" | Where-Object { $_ -match "^$NewVersion\]" }
#   gh release create $tag --title "$tag" --notes "$notes"

# --- 9. Done -----------------------------------------------------------
Write-Host "`n=== $mode complete ===" -ForegroundColor Cyan
if ($DryRun) {
    Write-Host "Nothing was changed. Re-run with -Execute to perform the release." -ForegroundColor Cyan
} else {
    Write-Host "Released $tag." -ForegroundColor Cyan
    Write-Host "Verify the live URLs:" -ForegroundColor Cyan
    Write-Host "  crates.io   : https://crates.io/crates/decimal-scaled/$NewVersion" -ForegroundColor Cyan
    Write-Host "  GitHub Pages: https://mootable.github.io/decimal-scaled/api/decimal_scaled/" -ForegroundColor Cyan
    Write-Host "  docs.rs     : https://docs.rs/decimal-scaled/$NewVersion (auto-build, 5-15 min)" -ForegroundColor Cyan
}
