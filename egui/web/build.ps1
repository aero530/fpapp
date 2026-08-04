<#
.SYNOPSIS
    Builds the browser version of the Financial Planner into egui/web/dist.

.DESCRIPTION
    Compiles the egui app to wasm32-unknown-unknown, generates the JavaScript
    glue with wasm-bindgen, and assembles a directory that can be copied
    straight into an Apache document root.  Everything in dist is static: no
    CGI, no modules, no server-side anything.

    wasm-bindgen's CLI has to match the wasm-bindgen crate version exactly, so
    the matching CLI is installed automatically when it is missing.

.PARAMETER DebugBuild
    Build without optimisations (much faster to compile, much larger and slower
    to load).

.PARAMETER OutDir
    Where to assemble the site.  Defaults to egui/web/dist.

.EXAMPLE
    .\egui\web\build.ps1
    # then copy egui\web\dist\* to the Apache document root
#>
[CmdletBinding()]
param(
    [switch]$DebugBuild,
    [string]$OutDir
)

$ErrorActionPreference = 'Stop'

# cargo, rustup and wasm-bindgen all report progress on stderr, which Windows
# PowerShell turns into a terminating error while $ErrorActionPreference is
# 'Stop'.  Run them with that relaxed and judge them by their exit code, which
# is what actually says whether they worked.
function Invoke-Tool {
    param(
        [Parameter(Mandatory)][string]$What,
        [Parameter(Mandatory)][scriptblock]$Command
    )
    $previous = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try { & $Command } finally { $ErrorActionPreference = $previous }
    if ($LASTEXITCODE -ne 0) { throw "$What failed (exit code $LASTEXITCODE)" }
}

$webDir = $PSScriptRoot
$repoRoot = Resolve-Path (Join-Path $webDir '..\..')
if (-not $OutDir) { $OutDir = Join-Path $webDir 'dist' }

$profileName = 'release'
if ($DebugBuild) { $profileName = 'debug' }

Write-Host "Repository:  $repoRoot"
Write-Host "Output:      $OutDir"
Write-Host "Profile:     $profileName"
Write-Host ''

# --- toolchain ---------------------------------------------------------------

$targets = Invoke-Tool 'rustup target list' { rustup target list --installed }
if ($targets -notcontains 'wasm32-unknown-unknown') {
    Write-Host 'Adding the wasm32-unknown-unknown target...'
    Invoke-Tool 'rustup target add' { rustup target add wasm32-unknown-unknown }
}

# The generated glue and the compiled-in ABI must come from the same
# wasm-bindgen release; a mismatch fails the build with a schema error.
$lock = Get-Content (Join-Path $repoRoot 'Cargo.lock') -Raw
$match = [regex]::Match($lock, '(?m)^name = "wasm-bindgen"\r?\nversion = "([^"]+)"')
if (-not $match.Success) { throw 'Could not find the wasm-bindgen version in Cargo.lock' }
$wantVersion = $match.Groups[1].Value

$haveVersion = ''
$cli = Get-Command wasm-bindgen -ErrorAction SilentlyContinue
if ($cli) {
    $reported = Invoke-Tool 'wasm-bindgen --version' { & wasm-bindgen --version }
    $haveVersion = ($reported -split ' ')[-1]
}

if ($haveVersion -ne $wantVersion) {
    if ($haveVersion) {
        Write-Host "wasm-bindgen CLI is $haveVersion but the crate is $wantVersion; installing the matching CLI..."
    } else {
        Write-Host "Installing the wasm-bindgen CLI ($wantVersion)..."
    }
    # cargo-binstall grabs a prebuilt binary in seconds; cargo install builds
    # it from source, which takes a few minutes but needs nothing extra.
    $binstall = Get-Command cargo-binstall -ErrorAction SilentlyContinue
    if ($binstall) {
        Invoke-Tool 'cargo binstall' { cargo binstall --no-confirm "wasm-bindgen-cli@$wantVersion" }
    } else {
        Invoke-Tool 'cargo install' { cargo install --locked wasm-bindgen-cli --version $wantVersion }
    }
}

# --- compile -----------------------------------------------------------------

Write-Host ''
Write-Host 'Compiling to WebAssembly...'
Push-Location $repoRoot
try {
    $cargoArgs = @('build', '--target', 'wasm32-unknown-unknown', '--package', 'fpapp-egui', '--bin', 'fpapp-egui')
    if (-not $DebugBuild) { $cargoArgs += '--release' }
    Invoke-Tool 'cargo build' { cargo @cargoArgs }
} finally {
    Pop-Location
}

$wasmIn = Join-Path $repoRoot "target\wasm32-unknown-unknown\$profileName\fpapp-egui.wasm"
if (-not (Test-Path $wasmIn)) { throw "Expected $wasmIn to exist" }

# --- assemble ----------------------------------------------------------------

# Start clean so stale snippets from an earlier build cannot linger
if (Test-Path $OutDir) { Remove-Item $OutDir -Recurse -Force }
New-Item -ItemType Directory -Path $OutDir | Out-Null

Write-Host 'Generating JavaScript bindings...'
Invoke-Tool 'wasm-bindgen' {
    wasm-bindgen --target web --no-typescript --out-name fpapp --out-dir $OutDir $wasmIn
}

$wasmOut = Join-Path $OutDir 'fpapp_bg.wasm'

# wasm-opt is optional (it ships with binaryen).  It typically takes another
# 15-25% off the module; if the installed version chokes on a newer wasm
# feature, the unoptimised module is perfectly usable.
if (-not $DebugBuild) {
    $wasmOpt = Get-Command wasm-opt -ErrorAction SilentlyContinue
    if ($wasmOpt) {
        Write-Host 'Optimising with wasm-opt...'
        $tmp = "$wasmOut.opt"
        $previous = $ErrorActionPreference
        $ErrorActionPreference = 'Continue'
        try { wasm-opt -Oz --output $tmp $wasmOut } finally { $ErrorActionPreference = $previous }
        if ($LASTEXITCODE -eq 0) {
            Move-Item $tmp $wasmOut -Force
        } else {
            Write-Warning 'wasm-opt failed; keeping the unoptimised module'
            if (Test-Path $tmp) { Remove-Item $tmp -Force }
        }
    } else {
        Write-Host 'wasm-opt not found (optional) - skipping size optimisation'
    }
}

Copy-Item (Join-Path $webDir 'index.html') $OutDir
Copy-Item (Join-Path $webDir 'htaccess') (Join-Path $OutDir '.htaccess')
Copy-Item (Join-Path $repoRoot 'egui\assets\icon-256.png') $OutDir

# --- report ------------------------------------------------------------------

Write-Host ''
Write-Host "Built $OutDir" -ForegroundColor Green
Get-ChildItem $OutDir -Recurse -File -Force |
    Sort-Object Length -Descending |
    ForEach-Object {
        $size = '{0,8:N0} KB' -f ($_.Length / 1KB)
        $name = $_.FullName.Substring($OutDir.Length + 1)
        Write-Host "  $size  $name"
    }
Write-Host ''
Write-Host 'Copy the contents of that directory into your Apache document root.'
Write-Host 'To try it locally first:  python -m http.server 8080 --directory' $OutDir
