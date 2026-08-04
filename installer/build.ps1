<#
.SYNOPSIS
    Build the Financial Planner Windows installer (MSI).

.DESCRIPTION
    Compiles a release binary of the egui app and packages it into a per-user MSI.

    The WiX toolset is provisioned automatically into `target/installer-tools/`: a pinned,
    checksummed copy of the official WiX 3.14 binaries archive. That keeps the build
    reproducible, needs no administrator rights, installs nothing system-wide, and behaves the
    same on a developer machine as in CI. A WiX already on PATH is used in preference.

    Everything the installer shows -- the application icon, the licence page and the two wizard
    bitmaps -- is generated at build time from `egui/assets/icon-256.png` and `LICENSE`. Those
    two files are the only sources of that artwork, so a committed copy cannot drift from them.

.PARAMETER SkipBuild
    Package whatever is already in target/release instead of rebuilding.

.PARAMETER Version
    Override the version. Defaults to the version in egui/Cargo.toml.

.PARAMETER OutDir
    Where to write the MSI. Defaults to target/installer.

.PARAMETER NoDownload
    Fail rather than downloading the WiX toolset. Use when the toolchain must come from
    somewhere controlled.

.PARAMETER PinnedWix
    Ignore any WiX on PATH or in $env:WIX and use only the pinned, checksummed copy. Release
    builds should set this so the output does not depend on whichever version a build machine
    happens to have installed.

.PARAMETER CertThumbprint
    Sign the MSI with the certificate in the user's store matching this thumbprint. Requires
    signtool.exe on PATH. Unsigned installers raise a SmartScreen warning on first download.

.EXAMPLE
    .\installer\build.ps1
    Build a release binary and produce target/installer/FinancialPlanner-5.1.0-x64.msi

.EXAMPLE
    .\installer\build.ps1 -SkipBuild -Version 5.2.0
#>
[CmdletBinding()]
param(
    [switch]$SkipBuild,
    [string]$Version,
    [string]$OutDir,
    [switch]$NoDownload,
    [switch]$PinnedWix,
    [string]$CertThumbprint
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

# Pinned toolchain. Bumping these three lines together is the only supported way to change it.
$WixVersion = '3.14.1'
$WixUrl = 'https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip'
$WixSha256 = '6AC824E1642D6F7277D0ED7EA09411A508F6116BA6FAE0AA5F2C7DAA2FF43D31'

# The cargo target is `fpapp-egui`; the installed application is "Financial Planner", which is
# also the window title. MSI renames the file on the way in, so nothing in the crate has to
# carry the friendlier name.
$ProductName = 'Financial Planner'
$CargoBin = 'fpapp-egui.exe'
$InstalledExe = 'FinancialPlanner.exe'

$RepoDir = Split-Path -Parent $PSScriptRoot
$InstallerDir = $PSScriptRoot
$TargetDir = Join-Path $RepoDir 'target'
$BinDir = Join-Path $TargetDir 'release'
if (-not $OutDir) { $OutDir = Join-Path $TargetDir 'installer' }

function Write-Step($message) { Write-Host "==> $message" -ForegroundColor Cyan }
function Write-Note($message) { Write-Host "    $message" -ForegroundColor DarkGray }

<#
    Run an external program, failing on a non-zero exit code.

    Needed because Windows PowerShell wraps anything a native program writes to stderr in an
    ErrorRecord, and with $ErrorActionPreference = 'Stop' that aborts the script even when the
    program succeeded. cargo reports build progress on stderr, so this is not hypothetical.
    Exit codes are the only reliable success signal.
#>
function Invoke-Tool {
    param(
        [Parameter(Mandatory)][string]$What,
        [Parameter(Mandatory)][string]$Exe,
        [string[]]$Arguments = @()
    )
    $previous = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        # Merge stderr into the output stream and print everything as plain host text. Without
        # this, each stderr line surfaces as an ErrorRecord, which litters CI logs with
        # NativeCommandError blocks that look like failures but are not.
        & $Exe @Arguments 2>&1 | ForEach-Object {
            if ($_ -is [System.Management.Automation.ErrorRecord]) {
                Write-Host $_.ToString()
            } else {
                Write-Host $_
            }
        }
        if ($LASTEXITCODE -ne 0) { throw "$What failed with exit code $LASTEXITCODE" }
    } finally {
        $ErrorActionPreference = $previous
    }
}

# ---------------------------------------------------------------------------------------
# Version
# ---------------------------------------------------------------------------------------

<#
    The version comes from egui/Cargo.toml, not the repository root.

    The root manifest is a virtual workspace -- it has a [workspace] table and no [package] --
    so it carries no version at all. egui/Cargo.toml is the manifest of the binary being
    shipped, which makes it the right place to read.
#>
function Get-CargoVersion {
    $manifest = Join-Path $RepoDir 'egui\Cargo.toml'
    foreach ($line in Get-Content $manifest) {
        # Stop at the first table after [package] so a dependency's version is never picked up.
        if ($line -match '^\s*\[' -and $line -notmatch '^\s*\[package\]') { break }
        if ($line -match '^\s*version\s*=\s*"([^"]+)"') { return $Matches[1] }
    }
    throw "Could not find a version in $manifest"
}

<#
    Convert a Cargo version into one an MSI accepts.

    MSI ProductVersion is `major.minor.build`, with major and minor at most 255 and build at most
    65535, and it has no concept of a pre-release suffix. Windows Installer also only compares
    those three fields when deciding whether an upgrade applies, so a fourth field would be
    silently ignored.
#>
function ConvertTo-MsiVersion($cargoVersion) {
    $core = ($cargoVersion -split '[-+]')[0]
    if ($core -ne $cargoVersion) {
        Write-Note "Cargo version '$cargoVersion' has a pre-release suffix; MSI does not support one, using '$core'."
    }
    $parts = $core -split '\.'
    if ($parts.Count -lt 3) { throw "Version '$cargoVersion' is not major.minor.patch" }
    $major, $minor, $build = [int]$parts[0], [int]$parts[1], [int]$parts[2]
    if ($major -gt 255 -or $minor -gt 255) { throw "MSI allows at most 255 for major and minor; got $major.$minor" }
    if ($build -gt 65535) { throw "MSI allows at most 65535 for the build field; got $build" }
    return "$major.$minor.$build"
}

# ---------------------------------------------------------------------------------------
# WiX toolset
# ---------------------------------------------------------------------------------------

function Resolve-WixBin {
    # Prefer whatever the environment already provides, unless a reproducible build was asked
    # for. Build machines often ship their own WiX, and silently using it would make the output
    # depend on the machine.
    if (-not $PinnedWix) {
        $candle = Get-Command 'candle.exe' -ErrorAction SilentlyContinue
        if ($candle) {
            Write-Note "Using WiX already on PATH: $(Split-Path -Parent $candle.Source)"
            return Split-Path -Parent $candle.Source
        }
        if ($env:WIX -and (Test-Path (Join-Path $env:WIX 'bin\candle.exe'))) {
            Write-Note "Using WiX from `$env:WIX"
            return (Join-Path $env:WIX 'bin')
        }
    }

    $localRoot = Join-Path $TargetDir "installer-tools\wix-$WixVersion"
    $localCandle = Join-Path $localRoot 'candle.exe'
    if (Test-Path $localCandle) {
        Write-Note "Using provisioned WiX at $localRoot"
        return $localRoot
    }

    if ($NoDownload) {
        throw @"
WiX $WixVersion is not available and -NoDownload was given.

Provide it one of these ways:
  * put candle.exe and light.exe on PATH, or
  * set `$env:WIX to a WiX installation, or
  * extract $WixUrl into $localRoot
"@
    }

    Write-Step "Provisioning WiX $WixVersion (once) into target/installer-tools"
    Write-Note "Nothing is installed system-wide and no administrator rights are needed."
    New-Item -ItemType Directory -Force -Path $localRoot | Out-Null
    $zip = Join-Path $TargetDir "installer-tools\wix-$WixVersion.zip"
    if (-not (Test-Path $zip)) {
        Write-Note "Downloading $WixUrl"
        Invoke-WebRequest -Uri $WixUrl -OutFile $zip -UseBasicParsing -TimeoutSec 900
    }

    $actual = (Get-FileHash $zip -Algorithm SHA256).Hash
    if ($actual -ne $WixSha256) {
        Remove-Item $zip -Force
        throw "WiX archive checksum mismatch.`n  expected $WixSha256`n  actual   $actual`nThe download was discarded."
    }
    Write-Note "Checksum verified."
    Expand-Archive -Path $zip -DestinationPath $localRoot -Force
    if (-not (Test-Path $localCandle)) { throw "candle.exe not found after extracting $zip" }
    return $localRoot
}

# ---------------------------------------------------------------------------------------
# Generated artwork and licence page
# ---------------------------------------------------------------------------------------

<#
    Render the repository's LICENSE into the RTF that the installer's license page shows.

    WixUI's LicenseAgreementDlg takes its text from the WixUILicenseRtf variable, and when that
    is not set WiX quietly substitutes the placeholder License.rtf bundled in WixUIExtension --
    which is Lorem ipsum. An installer that builds, validates and installs perfectly well while
    asking people to accept filler text is the failure mode this avoids.

    Generated at build time instead of committing a second copy: an RTF checked in beside
    LICENSE is a duplicate that drifts, and the whole point is that the dialog shows the
    licence the project is actually under.

    Monospace, and one \par per source line, because the Apache text is laid out with leading
    spaces -- a centred title and indented clauses -- which a proportional font would ragged.
#>
function New-LicenseRtf {
    param([Parameter(Mandatory)][string] $TextPath,
          [Parameter(Mandatory)][string] $RtfPath)

    if (-not (Test-Path -LiteralPath $TextPath)) { throw "licence text not found: $TextPath" }

    $sb = [System.Text.StringBuilder]::new()
    [void]$sb.Append('{\rtf1\ansi\ansicpg1252\deff0{\fonttbl{\f0\fmodern\fcharset0 Courier New;}}')
    [void]$sb.Append("`r`n" + '\viewkind4\uc1\pard\f0\fs16 ')

    foreach ($line in (Get-Content -LiteralPath $TextPath)) {
        # Backslash and braces are RTF syntax; anything above ASCII needs a \u escape.
        $escaped = $line.Replace('\', '\\').Replace('{', '\{').Replace('}', '\}')
        $out = [System.Text.StringBuilder]::new()
        foreach ($ch in $escaped.ToCharArray()) {
            if ([int]$ch -gt 127) { [void]$out.Append('\u' + [int]$ch + '?') }
            else { [void]$out.Append($ch) }
        }
        # The newline after \par is what terminates the control word, so a following line's
        # leading spaces survive as literal text rather than being eaten as a delimiter.
        [void]$sb.Append($out.ToString() + '\par' + "`r`n")
    }
    [void]$sb.Append('}')

    # ASCII: every byte above 127 has already been escaped, and a BOM would be shown verbatim.
    [System.IO.File]::WriteAllText($RtfPath, $sb.ToString(), [System.Text.ASCIIEncoding]::new())
    Write-Note "Licence page generated from LICENSE ($((Get-Item $RtfPath).Length) bytes of RTF)"
}

<#
    Pack a bitmap into the DIB form an .ico entry uses.

    Not the same layout as a .bmp file: there is no BITMAPFILEHEADER, the height in the
    info header is doubled because a colour bitmap and a 1-bit AND mask are stacked in one
    stream, and the rows run bottom-up. The mask is left all zeroes -- for a 32-bit entry the
    alpha channel is what Windows honours, and a stale mask would punch holes in the icon.
#>
function ConvertTo-IcoDib {
    param([Parameter(Mandatory)][System.Drawing.Bitmap] $Bitmap)

    $width = $Bitmap.Width
    $height = $Bitmap.Height
    $rect = New-Object System.Drawing.Rectangle(0, 0, $width, $height)
    $locked = $Bitmap.LockBits($rect, [System.Drawing.Imaging.ImageLockMode]::ReadOnly,
                               [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $colour = New-Object byte[] ($width * $height * 4)
    try {
        $row = New-Object byte[] ($width * 4)
        for ($y = 0; $y -lt $height; $y++) {
            # Bottom-up: the last source row is written first.
            $source = [IntPtr]::Add($locked.Scan0, $locked.Stride * ($height - 1 - $y))
            [System.Runtime.InteropServices.Marshal]::Copy($source, $row, 0, $row.Length)
            [Array]::Copy($row, 0, $colour, $y * $row.Length, $row.Length)
        }
    } finally { $Bitmap.UnlockBits($locked) }

    # Each mask row is padded up to a 4-byte boundary, as every DIB row is.
    $maskStride = [int][math]::Floor(($width + 31) / 32) * 4
    $mask = New-Object byte[] ($maskStride * $height)

    $stream = New-Object System.IO.MemoryStream
    $writer = New-Object System.IO.BinaryWriter($stream)
    try {
        $writer.Write([uint32]40)               # BITMAPINFOHEADER size
        $writer.Write([int32]$width)
        $writer.Write([int32]($height * 2))     # colour bitmap plus AND mask
        $writer.Write([uint16]1)                # planes
        $writer.Write([uint16]32)               # bits per pixel
        $writer.Write([uint32]0)                # BI_RGB, uncompressed
        $writer.Write([uint32]$colour.Length)
        $writer.Write([int32]0)                 # horizontal resolution, unused
        $writer.Write([int32]0)                 # vertical resolution, unused
        $writer.Write([uint32]0)                # palette entries used
        $writer.Write([uint32]0)                # palette entries required
        $writer.Write($colour)
        $writer.Write($mask)
        $writer.Flush()
        return $stream.ToArray()
    } finally { $writer.Dispose(); $stream.Dispose() }
}

<#
    Build the multi-resolution .ico the MSI needs from the app's PNG icon.

    An .ico is required in two places -- ARPPRODUCTICON in Add/Remove Programs and the Start
    Menu shortcut -- and the repository only has egui/assets/icon-256.png, which the app itself
    embeds for its window icon. Generating the .ico here keeps that PNG the single source of the
    artwork rather than adding a second binary to git that can drift from it.

    Several sizes are emitted because Windows picks the nearest one and scales it: with only a
    256x256 entry, a 16x16 shortcut icon would be a blurry downscale. The 256 entry is the
    original PNG byte for byte -- .ico has allowed PNG-compressed entries since Vista, and
    re-encoding it would only lose fidelity. Smaller entries are resampled DIBs, which is the
    form every shell version reads without question.
#>
function New-AppIcon {
    param([Parameter(Mandatory)][string] $PngPath,
          [Parameter(Mandatory)][string] $IcoPath)

    if (-not (Test-Path -LiteralPath $PngPath)) { throw "icon not found: $PngPath" }
    Add-Type -AssemblyName System.Drawing

    $resolved = (Resolve-Path -LiteralPath $PngPath).Path
    $source = [System.Drawing.Image]::FromFile($resolved)
    $entries = @()
    try {
        foreach ($size in 16, 24, 32, 48, 64, 128) {
            $scaled = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
            $g = [System.Drawing.Graphics]::FromImage($scaled)
            try {
                $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
                $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
                $g.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality
                $g.Clear([System.Drawing.Color]::Transparent)
                $g.DrawImage($source, (New-Object System.Drawing.Rectangle(0, 0, $size, $size)))
            } finally { $g.Dispose() }
            # The [byte[]] cast is not decoration. `return` in PowerShell writes an array to the
            # pipeline element by element, so the caller gets an Object[] of boxed bytes; handed
            # to BinaryWriter.Write that binds to the single-byte overload and silently writes
            # one byte per entry, producing a well-formed directory pointing at no image data.
            $entries += [pscustomobject]@{ Size = $size; Data = [byte[]](ConvertTo-IcoDib $scaled) }
            $scaled.Dispose()
        }
        if ($source.Width -ne 256 -or $source.Height -ne 256) {
            throw "$PngPath is $($source.Width)x$($source.Height); the 256x256 icon entry is taken from it verbatim"
        }
    } finally { $source.Dispose() }
    $entries += [pscustomobject]@{ Size = 256; Data = [byte[]][System.IO.File]::ReadAllBytes($resolved) }

    $stream = New-Object System.IO.MemoryStream
    $writer = New-Object System.IO.BinaryWriter($stream)
    try {
        $writer.Write([uint16]0)                  # reserved
        $writer.Write([uint16]1)                  # resource type: icon
        $writer.Write([uint16]$entries.Count)
        # Image data follows the directory, so the first offset is past all of its entries.
        $offset = 6 + 16 * $entries.Count
        foreach ($entry in $entries) {
            # A dimension of 256 does not fit in a byte and is encoded as zero.
            $dimension = if ($entry.Size -ge 256) { 0 } else { $entry.Size }
            $writer.Write([byte]$dimension)       # width
            $writer.Write([byte]$dimension)       # height
            $writer.Write([byte]0)                # palette size; 0 for true colour
            $writer.Write([byte]0)                # reserved
            $writer.Write([uint16]1)              # planes
            $writer.Write([uint16]32)             # bits per pixel
            $writer.Write([uint32]$entry.Data.Length)
            $writer.Write([uint32]$offset)
            $offset += $entry.Data.Length
        }
        foreach ($entry in $entries) { $writer.Write($entry.Data) }
        $writer.Flush()
        [System.IO.File]::WriteAllBytes($IcoPath, $stream.ToArray())
    } finally { $writer.Dispose(); $stream.Dispose() }

    # An .ico with a correct directory and truncated image data is still a valid file as far as
    # every tool in this pipeline is concerned: candle embeds it, light links it, and the icon
    # merely renders blank. The size is the one cheap check that catches it, so it is asserted
    # rather than assumed.
    $expected = 6 + 16 * $entries.Count
    foreach ($entry in $entries) { $expected += $entry.Data.Length }
    $actual = (Get-Item $IcoPath).Length
    if ($actual -ne $expected) {
        throw "$IcoPath is $actual bytes; the directory it contains accounts for $expected. Image data was lost on the way in."
    }

    Write-Note "Icon generated from $(Split-Path -Leaf $PngPath) ($(($entries.Size) -join ', ') px, $actual bytes)"
}

<#
    Render the installer's two WixUI bitmaps from the app icon.

    Without these, WiX uses the bitmaps bundled in WixUIExtension, which carry its own artwork --
    a red graphic in the top-right of every page that has nothing to do with this application.

    The sizes are fixed by WixUI, not chosen here: the banner across the top of most pages is
    493x58, and the panel behind the Welcome and Finish pages is 493x312. A bitmap of any other
    size is not rejected, it just renders wrong, which is why the dimensions are asserted below.

    Layout is dictated by where WixUI puts its text. The page title is drawn over the left of the
    banner and the welcome text over the right two-thirds of the panel, both in dark ink -- so
    those regions stay white and the artwork keeps to the opposite side.
#>
function New-UiBitmaps {
    param([Parameter(Mandatory)][string] $IconPath,
          [Parameter(Mandatory)][string] $BannerPath,
          [Parameter(Mandatory)][string] $DialogPath)

    if (-not (Test-Path -LiteralPath $IconPath)) { throw "icon not found: $IconPath" }
    Add-Type -AssemblyName System.Drawing

    $icon = [System.Drawing.Image]::FromFile((Resolve-Path -LiteralPath $IconPath).Path)
    try {
        # --- banner: white, icon on the right, clear of the page title on the left ---
        $banner = New-Object System.Drawing.Bitmap(493, 58, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
        $g = [System.Drawing.Graphics]::FromImage($banner)
        try {
            $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
            $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
            $g.Clear([System.Drawing.Color]::White)
            $g.DrawImage($icon, (New-Object System.Drawing.Rectangle(433, 5, 48, 48)))
        } finally { $g.Dispose() }
        $banner.Save($BannerPath, [System.Drawing.Imaging.ImageFormat]::Bmp)

        # --- panel: dark band down the left with the icon on it, white where the text lands ---
        $dialog = New-Object System.Drawing.Bitmap(493, 312, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
        $g = [System.Drawing.Graphics]::FromImage($dialog)
        try {
            $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
            $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
            $g.Clear([System.Drawing.Color]::White)
            # egui's dark theme paints its panels in this near-black, so the installer looks
            # like the thing it installs. The icon is wine with white lettering and reads
            # cleanly on it, which it would not on a band of its own colour.
            $band = New-Object System.Drawing.Rectangle(0, 0, 161, 312)
            $top = [System.Drawing.Color]::FromArgb(27, 27, 27)
            $bottom = [System.Drawing.Color]::FromArgb(12, 12, 12)
            $brush = New-Object System.Drawing.Drawing2D.LinearGradientBrush($band, $top, $bottom, 90.0)
            try { $g.FillRectangle($brush, $band) } finally { $brush.Dispose() }
            # A rule in the icon's own wine, separating the band from the text area.
            $accent = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(160, 27, 82))
            try { $g.FillRectangle($accent, (New-Object System.Drawing.Rectangle(161, 0, 3, 312))) }
            finally { $accent.Dispose() }
            $g.DrawImage($icon, (New-Object System.Drawing.Rectangle(25, 100, 112, 112)))
        } finally { $g.Dispose() }
        $dialog.Save($DialogPath, [System.Drawing.Imaging.ImageFormat]::Bmp)

        foreach ($check in @(@{ Path = $BannerPath; W = 493; H = 58 },
                             @{ Path = $DialogPath; W = 493; H = 312 })) {
            $img = [System.Drawing.Image]::FromFile((Resolve-Path -LiteralPath $check.Path).Path)
            try {
                if ($img.Width -ne $check.W -or $img.Height -ne $check.H) {
                    throw "$($check.Path) is $($img.Width)x$($img.Height), WixUI requires $($check.W)x$($check.H)"
                }
            } finally { $img.Dispose() }
        }
        $banner.Dispose(); $dialog.Dispose()
    } finally { $icon.Dispose() }

    Write-Note "Installer artwork generated from $(Split-Path -Leaf $IconPath) (493x58 banner, 493x312 panel)"
}

# ---------------------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------------------

$cargoVersion = if ($Version) { $Version } else { Get-CargoVersion }
$msiVersion = ConvertTo-MsiVersion $cargoVersion
Write-Step "$ProductName $cargoVersion (MSI version $msiVersion)"

if (-not $SkipBuild) {
    Write-Step 'Building release binary'
    Push-Location $RepoDir
    try {
        # Named explicitly: the workspace also contains the `accounts` library, and the
        # installer only ships the application.
        Invoke-Tool -What 'cargo build' -Exe 'cargo' -Arguments @(
            'build', '--release', '--package', 'fpapp-egui', '--bin', 'fpapp-egui')
    } finally { Pop-Location }
} else {
    Write-Note 'Skipping cargo build (-SkipBuild).'
}

$exe = Join-Path $BinDir $CargoBin
if (-not (Test-Path $exe)) {
    throw "$exe not found. Run without -SkipBuild, or build it first with 'cargo build --release'."
}

$wixBin = Resolve-WixBin
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$objDir = Join-Path $TargetDir 'installer-obj'
New-Item -ItemType Directory -Force -Path $objDir | Out-Null

$wixObj = Join-Path $objDir 'fpapp.wixobj'
$msi = Join-Path $OutDir "FinancialPlanner-$cargoVersion-x64.msi"

$licenseRtf = Join-Path $objDir 'license.rtf'
New-LicenseRtf -TextPath (Join-Path $RepoDir 'LICENSE') -RtfPath $licenseRtf

$iconPng = Join-Path $RepoDir 'egui\assets\icon-256.png'
$appIco = Join-Path $objDir 'app.ico'
New-AppIcon -PngPath $iconPng -IcoPath $appIco

$bannerBmp = Join-Path $objDir 'banner.bmp'
$dialogBmp = Join-Path $objDir 'dialog.bmp'
New-UiBitmaps -IconPath $iconPng -BannerPath $bannerBmp -DialogPath $dialogBmp

Write-Step 'Compiling installer authoring (candle)'
Invoke-Tool -What 'candle' -Exe (Join-Path $wixBin 'candle.exe') -Arguments @(
    '-nologo'
    '-arch', 'x64'
    "-dVersion=$msiVersion"
    "-dBinDir=$BinDir"
    "-dRepoDir=$RepoDir"
    "-dCargoBin=$CargoBin"
    "-dInstalledExe=$InstalledExe"
    "-dLicenseRtf=$licenseRtf"
    "-dAppIco=$appIco"
    "-dBannerBmp=$bannerBmp"
    "-dDialogBmp=$dialogBmp"
    '-ext', 'WixUIExtension'
    '-ext', 'WixUtilExtension'
    '-out', $wixObj
    (Join-Path $InstallerDir 'fpapp.wxs')
)

Write-Step 'Linking MSI (light)'
# Two validation checks are suppressed, both deliberately:
#   ICE61 - AllowSameVersionUpgrades lets a rebuild of the same version replace itself, which is
#           what you want while developing.
#   ICE91 - warns that files land in a per-user directory that does not vary with ALLUSERS. That
#           is the entire point of a per-user install.
Invoke-Tool -What 'light' -Exe (Join-Path $wixBin 'light.exe') -Arguments @(
    '-nologo'
    '-ext', 'WixUIExtension'
    '-ext', 'WixUtilExtension'
    '-cultures:en-us'
    '-sice:ICE61'
    '-sice:ICE91'
    '-spdb'
    '-out', $msi
    $wixObj
)

if ($CertThumbprint) {
    Write-Step 'Signing'
    $signtool = Get-Command 'signtool.exe' -ErrorAction SilentlyContinue
    if (-not $signtool) { throw 'signtool.exe is not on PATH. It ships with the Windows SDK.' }
    Invoke-Tool -What 'signtool' -Exe $signtool.Source -Arguments @(
        'sign'
        '/sha1', $CertThumbprint
        '/fd', 'SHA256'
        '/tr', 'http://timestamp.digicert.com'
        '/td', 'SHA256'
        $msi
    )
} else {
    Write-Note 'Not signed. Windows SmartScreen will warn on first download; pass -CertThumbprint to sign.'
}

$size = [math]::Round((Get-Item $msi).Length / 1MB, 1)
Write-Host ''
Write-Step "Built $msi ($size MB)"
Write-Host ''
Write-Host '    Install interactively:  ' -NoNewline -ForegroundColor DarkGray
Write-Host (Split-Path -Leaf $msi)
Write-Host '    Install silently:       ' -NoNewline -ForegroundColor DarkGray
Write-Host "msiexec /i `"$(Split-Path -Leaf $msi)`" /qn"
Write-Host '    Uninstall:              ' -NoNewline -ForegroundColor DarkGray
Write-Host 'Settings > Apps, or msiexec /x with the same file'
Write-Host ''
Write-Note "Installs to %LOCALAPPDATA%\Programs\$ProductName for the current user; no UAC prompt."
