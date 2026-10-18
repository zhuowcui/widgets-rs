param(
    [Parameter(Mandatory)][int]$AppPid,
    [Parameter(Mandatory)][string]$InstanceId,
    [string]$OutputDirectory = 'dist\layout-smoke',
    [switch]$Narrow
)

$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
$checks = [Collections.Generic.List[object]]::new()

function Assert-Ui {
    param([string]$Name, [string[]]$UiArguments)
    $output = & winapp ui @UiArguments -a $AppPid --json 2>&1
    if ($LASTEXITCODE -ne 0) { throw "$Name failed: $output" }
    $checks.Add(@{ name = $Name; status = 'PASS' })
}
function Capture-Ui {
    param([string]$Name)
    $output = & winapp ui screenshot -a $AppPid -o (Join-Path $OutputDirectory "$Name.png") --json 2>&1
    if ($LASTEXITCODE -ne 0) { throw "Capture $Name failed: $output" }
}

# These checks use UIA patterns; ui-click-smoke.ps1 separately checks pointer hit targets.
# The existing pin's custom state is never changed, and Unpin is always cancelled.
try {
    Assert-Ui 'Home navigation' @('invoke', 'Nav_home')
    Assert-Ui 'Visual resources loaded' @('wait-for', "Could not load the app’s appearance:", '--gone', '-t', '1000')
    Assert-Ui 'Populated Home' @('wait-for', 'Pinned widgets', '--contains', '-t', '3000')
    Capture-Ui 'home'
    Assert-Ui 'My widgets navigation' @('invoke', 'Nav_widgets')
    Assert-Ui 'Instance selection' @('invoke', "Instance_$InstanceId")
    Assert-Ui 'City search available' @('wait-for', 'CitySearch', '-t', '3000')
    Capture-Ui 'manager'
    # AutoSuggestBox exposes its editable child as TextBox. Suggestion choice
    # and pointer/keyboard popup behavior are a separate foreground check.
    Assert-Ui 'City search text' @('set-value', 'TextBox', 'NoSuchCityForQa')
    Assert-Ui 'City search no results' @('wait-for', 'No new cities match your search.', '-t', '3000')
    Capture-Ui 'city-search-no-results'
    if ($Narrow) {
        Assert-Ui 'Return to list to reset query' @('invoke', 'BackToInstances')
    }
    Assert-Ui 'Clear city query without editing state' @('invoke', "Instance_$InstanceId")
    Assert-Ui 'Rename editor' @('invoke', 'RenameInstance')
    Assert-Ui 'Name input available' @('wait-for', 'InstanceName', '-t', '3000')
    Assert-Ui 'Name input visible' @('wait-for', 'InstanceName', '--property', 'IsOffscreen', '--value', 'False', '-t', '3000')
    Capture-Ui 'rename'
    Assert-Ui 'Cancel rename' @('invoke', 'CancelInstanceName')
    Assert-Ui 'City editor' @('invoke', 'City_0')
    Assert-Ui 'City label input available' @('wait-for', 'Label_0', '-t', '3000')
    Assert-Ui 'City label input visible' @('wait-for', 'Label_0', '--property', 'IsOffscreen', '--value', 'False', '-t', '3000')
    Assert-Ui 'City editor heading visible' @('wait-for', 'CloseCityEditor', '--property', 'IsOffscreen', '--value', 'False', '-t', '3000')
    Capture-Ui 'city-editor'
    Assert-Ui 'Close city editor' @('invoke', 'CloseCityEditor')
    Assert-Ui 'Unpin confirmation' @('invoke', 'UnpinWidget')
    Assert-Ui 'Confirmation available' @('wait-for', 'PrimaryButton', '-t', '3000')
    Assert-Ui 'Confirmation visible' @('wait-for', 'PrimaryButton', '--property', 'IsOffscreen', '--value', 'False', '-t', '3000')
    Capture-Ui 'unpin-confirmation'
    Assert-Ui 'Cancel unpin' @('invoke', 'CloseButton')
    if ($Narrow) {
        Assert-Ui 'Return to instance list' @('invoke', 'BackToInstances')
    }
    Assert-Ui 'Instance retained' @('wait-for', "Instance_$InstanceId", '-t', '3000')
    Assert-Ui 'Discover navigation' @('invoke', 'Nav_discover')
    Assert-Ui 'Catalog search' @('set-value', 'CatalogSearch', 'NoSuchWidget')
    Assert-Ui 'Catalog empty result' @('wait-for', 'No released widgets match your search.', '-t', '3000')
    Capture-Ui 'catalog-no-results'
    Assert-Ui 'Catalog search reset' @('set-value', 'CatalogSearch', 'World')
    Assert-Ui 'World Clock available' @('wait-for', 'FeaturedClock', '-t', '3000')
    Capture-Ui 'discover'
    Assert-Ui 'Settings navigation' @('invoke', 'Nav_settings')
    Assert-Ui 'Offline information' @('wait-for', 'Offline and private', '-t', '3000')
    Capture-Ui 'settings'
    Assert-Ui 'Return to manager' @('invoke', 'Nav_widgets')
    Assert-Ui 'Restore instance selection' @('invoke', "Instance_$InstanceId")
} finally {
    $checks | ConvertTo-Json | Set-Content (Join-Path $OutputDirectory 'results.json')
}
Write-Host "UIA layout smoke passed: $($checks.Count) checks."
