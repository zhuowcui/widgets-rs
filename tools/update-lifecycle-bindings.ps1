# Regenerate the checked-in AppInstance projection from the installed, pinned runtime metadata.
$ErrorActionPreference = 'Stop'
$project = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$runtime = Get-AppxPackage -Name 'Microsoft.WindowsAppRuntime.2' |
    Where-Object { $_.Version -eq [Version]'2.4.0.0' -and $_.Architecture -eq 'X64' } |
    Select-Object -First 1
if (-not $runtime) { throw 'Windows App Runtime 2.4.0.0 x64 is required to regenerate lifecycle bindings.' }
$metadata = Join-Path $runtime.InstallLocation 'Microsoft.Windows.AppLifecycle.winmd'
$expected = '1AA30FC55CEBE7ED2B574DF879C37799A8A289E1239DCEF3A6C4D51F369C70F0'
if ((Get-FileHash $metadata -Algorithm SHA256).Hash -ne $expected) {
    throw 'AppLifecycle metadata checksum mismatch.'
}
Push-Location $project
try {
    cargo run --manifest-path 'tools\bindings\Cargo.toml' --release -- lifecycle $metadata 'crates\settings\src\lifecycle_bindings.rs'
    if ($LASTEXITCODE -ne 0) { throw 'Binding generation failed.' }
} finally {
    Pop-Location
}
