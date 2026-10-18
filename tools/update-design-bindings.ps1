# Regenerate only the application resource APIs used to style native controls.
$ErrorActionPreference = 'Stop'
$project = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$runtime = Get-AppxPackage -Name 'Microsoft.WindowsAppRuntime.2' |
    Where-Object { $_.Version -eq [Version]'2.4.0.0' -and $_.Architecture -eq 'X64' } |
    Select-Object -First 1
if (-not $runtime) { throw 'Windows App Runtime 2.4.0.0 x64 metadata is required.' }
$checksums = @{
    'Microsoft.UI.Xaml.winmd' = '4C88A75617B7F3436DC130F6D2933A064955628DEDF424493516EC97905CBE01'
    'Microsoft.UI.winmd' = '49699D989516C87CC8C83CC3D6335F29A80759F91358E04F3054CC2FD09A99A5'
    'Microsoft.UI.Text.winmd' = 'C31EAFD592EA0199E2AB0CDA06D23DB533F8C62126819FFF19C46A7DB18779D7'
    'Microsoft.Foundation.winmd' = 'AFA7477691FE6470E5F1440F02F64416C70A29BCF6155A445D30CF6351B4CDD6'
    'Microsoft.Windows.ApplicationModel.Resources.winmd' = 'C719C3FFD04B343F10E81E7EEAAF2E49EF3C747BFB82C7E5D49167C5E4BA9D41'
}
foreach ($name in $checksums.Keys) {
    if ((Get-FileHash -LiteralPath (Join-Path $runtime.InstallLocation $name) -Algorithm SHA256).Hash -ne $checksums[$name]) {
        throw "Metadata checksum mismatch: $name"
    }
}
Push-Location $project
try {
    cargo run --manifest-path 'tools/bindings/Cargo.toml' --release -- design `
        (Join-Path $runtime.InstallLocation 'Microsoft.UI.Xaml.winmd') 'crates/settings/src/design_bindings.rs'
    if ($LASTEXITCODE -ne 0) { throw 'Design binding generation failed.' }
} finally { Pop-Location }
