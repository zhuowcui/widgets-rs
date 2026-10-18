# Rebuild the checked-in WinRT projection from the pinned official NuGet metadata.
$ErrorActionPreference = 'Stop'
$project = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$scratch = Join-Path $env:TEMP 'widgets-rs-bindings'
$archive = Join-Path $scratch 'Microsoft.WindowsAppSDK.Widgets.2.0.5.nupkg'
$archiveZip = Join-Path $scratch 'Microsoft.WindowsAppSDK.Widgets.2.0.5.zip'
$expanded = Join-Path $scratch 'package'
New-Item -ItemType Directory -Path $scratch -Force | Out-Null
Invoke-WebRequest 'https://api.nuget.org/v3-flatcontainer/microsoft.windowsappsdk.widgets/2.0.5/microsoft.windowsappsdk.widgets.2.0.5.nupkg' -OutFile $archive
$expected = '7F1B585EF8F012784D2CB91B86E809814CECCAB74183A5979EF45DB141779CB5'
if ((Get-FileHash $archive -Algorithm SHA256).Hash -ne $expected) { throw 'Widget metadata package checksum mismatch.' }
Copy-Item $archive $archiveZip -Force
Expand-Archive -LiteralPath $archiveZip -DestinationPath $expanded -Force
$metadata = Join-Path $expanded 'metadata\Microsoft.Windows.Widgets.winmd'
Push-Location $project
try {
    cargo run --manifest-path 'tools\bindings\Cargo.toml' --release -- widgets $metadata 'crates\widget_api\src\bindings.rs'
    if ($LASTEXITCODE -ne 0) { throw 'Binding generation failed.' }
} finally {
    Pop-Location
}
