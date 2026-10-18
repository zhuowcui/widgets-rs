param(
    [switch]$Install
)

$ErrorActionPreference = 'Stop'
$project = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$layout = Join-Path $project ('dist\package-layout-' + [Guid]::NewGuid().ToString('N').Substring(0, 8))
$output = Join-Path $project 'dist\WidgetsRS.Dev_0.2.0.0_x64.msix'
$certificate = Join-Path $project 'dist\devcert.pfx'
$publicCertificate = Join-Path $project 'dist\devcert.cer'

Push-Location $project
try {
    & (Join-Path $PSScriptRoot 'generate_card_backdrops.ps1')
    & (Join-Path $PSScriptRoot 'generate_feature_art.ps1')
    cargo build --workspace --release
    if ($LASTEXITCODE -ne 0) { throw 'Rust release build failed.' }
    & (Join-Path $PSScriptRoot 'generate_assets.ps1')
    New-Item -ItemType Directory -Path (Join-Path $layout 'Assets') -Force | Out-Null
    Copy-Item 'packaging\Package.appxmanifest' (Join-Path $layout 'AppxManifest.xml') -Force
    Copy-Item 'packaging\Assets\*.png' (Join-Path $layout 'Assets') -Force
    Copy-Item 'target\release\widgets_settings.exe' (Join-Path $layout 'WidgetsRs.exe') -Force
    Copy-Item 'target\release\widgets_provider.exe' (Join-Path $layout 'WidgetsRsProvider.exe') -Force

    if (-not (Test-Path $certificate)) {
        winapp cert generate --manifest 'packaging\Package.appxmanifest' --output $certificate --export-cer
        if ($LASTEXITCODE -ne 0) { throw 'Development certificate generation failed.' }
    }
    winapp package $layout --output $output --cert $certificate
    if ($LASTEXITCODE -ne 0) { throw 'MSIX packaging failed.' }
    Set-Content -Path (Join-Path $project 'dist\latest-layout.txt') -Value $layout

    if ($Install) {
        $isAdmin = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
        if (-not $isAdmin.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
            throw 'MSIX installation needs an elevated PowerShell. Re-run this script as Administrator with -Install.'
        }
        winapp cert install $publicCertificate
        if ($LASTEXITCODE -ne 0) { throw 'Could not trust the development certificate.' }
        Add-AppxPackage -Path $output -ForceUpdateFromAnyVersion
        Write-Host 'Installed Widgets RS. Open the Windows widget board to pin World Clock.'
    }
    Write-Host "Package: $output"
    Write-Host "Development layout: $layout"
} finally {
    Pop-Location
}
