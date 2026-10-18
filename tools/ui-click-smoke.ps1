param([int]$AppPid = 0)

$ErrorActionPreference = 'Stop'
if ($AppPid -eq 0) {
    $apps = @(Get-Process -Name WidgetsRs -ErrorAction Stop)
    if ($apps.Count -ne 1) {
        throw 'Run one Widgets RS window, or pass its process ID with -AppPid.'
    }
    $AppPid = $apps[0].Id
}

function Test-UiAction {
    param([string[]]$UiArguments)
    $result = & winapp ui @UiArguments -a $AppPid 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "winapp ui $($UiArguments -join ' ') failed: $result"
    }
}

# `click` sends a real pointer click. `invoke` can bypass a hit-test overlay.
Test-UiAction @('click', 'Nav_home')
Test-UiAction @('wait-for', 'Your widget space', '-t', '3000')
Test-UiAction @('click', 'Nav_discover')
Test-UiAction @('wait-for', 'Discover widgets', '-t', '3000')
Test-UiAction @('click', 'Nav_settings')
Test-UiAction @('wait-for', 'Offline and private', '-t', '3000')
Test-UiAction @('click', 'Nav_widgets')
Test-UiAction @('wait-for', 'My widgets', '-t', '3000')
Test-UiAction @('click', 'City_0')
Test-UiAction @('wait-for', 'CloseCityEditor', '-t', '3000')
Test-UiAction @('click', 'CloseCityEditor')
Test-UiAction @('click', 'Nav_home')
Test-UiAction @('wait-for', 'Your widget space', '-t', '3000')

Write-Host 'Pointer-click smoke test passed.'
