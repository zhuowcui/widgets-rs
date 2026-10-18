# Widgets RS

A native Windows 11 widget catalog and manager written in Rust. **Home** shows pinned widgets, **Discover** lists the types shipped in the app, **My widgets** manages each pin, and **Settings** shows package status. The first available type, **World Clock**, shows independent, ordered sets of cities on the Windows widget board. Future widget types arrive through app updates and manifest definitions.

## Architecture

| Crate | Responsibility |
| --- | --- |
| `clock_core` | Released widget registry and generic host records; offline IANA city catalog, WinRT time conversion and regional formatting, versioned World Clock state, Adaptive Card generation, and `WidgetManager` updates |
| `widget_api` | Checked-in windows-rs bindings generated from Windows App SDK widget metadata |
| `widgets_provider` | Small local COM server implementing `IWidgetProvider` and `IWidgetProvider2`; no WinUI dependency |
| `widgets_settings` | Reactor/WinUI 3 NavigationView shell with Home, Discover, My widgets, Settings, World Clock editor, and AppInstance activation routing |

The package uses the shared Windows App Runtime 2 framework. The provider has a 30-second inactive grace period. Its existing five-second system-change wake-up also reconciles host activation state, so a missed Deactivate callback cannot keep a stale active set alive. Cached inactive COM interfaces do not defeat the grace period; explicit server locks are respected. Normal minute ticking does not keep publishing in the background. The card shows its last update time.

## Requirements

- Windows 11 x64, build 22621 or later, with the Windows widget board available
- Rust 1.95 or later with the MSVC target
- Windows App Runtime 2.4 or later, `winapp` CLI 0.6 or later, and Windows Developer Mode
- Administrator access **once** to trust the local development signing certificate for installation

This is a development identity (`WidgetsRS.Dev`, `CN=Widgets RS Development`). Choose a permanent package identity and production signing before public distribution.

## Build and install

In PowerShell at the repository root:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\build-package.ps1
```

This produces `dist\WidgetsRS.Dev_0.2.0.0_x64.msix`, a fresh development layout recorded in `dist\latest-layout.txt`, and a local development certificate under `dist`. A fresh layout lets later package builds complete while a registered development provider is running. The generated PFX uses the `winapp` development password and is gitignored. To install, run **PowerShell as Administrator**:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\build-package.ps1 -Install
```

For local development without trusting a certificate, Windows Developer Mode can register the unpacked layout for the current user:

```powershell
$layout = Get-Content dist\latest-layout.txt
Add-AppxPackage -Register (Join-Path $layout AppxManifest.xml) -ForceApplicationShutdown
```

This registers the same development identity and enables widget-board testing. Unregister that development package before switching to the signed MSIX installation:

```powershell
winapp unregister --manifest packaging\Package.appxmanifest
```

Open Widgets RS from Start. Home shows your pinned instances; Discover shows the shipped types and previews. Use **Open widget board**, select **Add widgets**, and pin **World Clock**. Pin it again to create a separate instance. In **My widgets**, select a pin to rename it inside the app, change its cities and labels, refresh it, or unpin it after confirming its name and size in the native dialog. The widget's **Customize widget** menu and **Edit cities** action route to the selected instance in the existing app window. Use **Refresh** if the board changes while the app is open. Windows continues to own pinning, size, and placement.

The app works offline after the shared runtime is installed. The bundled city index comes from IANA `zone1970.tab`; Windows' own time-zone and regional formatting APIs perform conversions. Search also recognizes a few common city aliases. Instance names live in the host's per-widget `CustomState` but appear only in the app. Existing version-1 clock settings remain readable and become version 2 on the next save. Labels are English in this milestone.

## Check and verify

```powershell
cargo fmt --all -- --check
cargo test --workspace
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

With the registered app running and at least one World Clock pinned, run `powershell -NoProfile -ExecutionPolicy Bypass -File tools\ui-click-smoke.ps1`. It verifies pointer clicks across navigation and the city editor; UI Automation `Invoke` alone does not test mouse hit targets.

The tests cover catalog and manifest agreement, version-1 state migration, name validation, route decoding, corrupted state, invalid zones, DST spring and fall transitions, Kathmandu's 45-minute offset, midnight date arithmetic, size limits, regional time formatting, and clock-face SVG outlines. Regenerate pinned Windows App SDK projections with `tools\update-bindings.ps1`, `tools\update-lifecycle-bindings.ps1`, and `tools\update-design-bindings.ps1` when changing their API surfaces.

For repeatable layout and UIA checks that preserve an existing pin, run `tools\ui-layout-smoke.ps1 -AppPid <pid> -InstanceId <widget-id>`. It checks editor visibility and appearance loading, captures Home, the manager's edit states, Discover, and Settings, and cancels the native Unpin dialog. Add `-Narrow` when the window uses the list-to-detail layout. Pointer checks remain separate because UIA patterns bypass hit testing.

See [verification results](docs/verification.md) for measurements and the current board test status.

See the [current visual review](docs/design-review-2026-09-30.md) for actual board and app captures, renderer constraints, and the next quality checks.

See [app-level UX concepts](docs/ux-concepts.md) for the visual exploration that informed the native shell. These concept images are design references, not packaged app assets.

For an installed-app acceptance pass:

1. Pin two World Clock instances. Add different cities to each; rename, reorder, and remove a city in one. Confirm the other retains its settings.
2. Resize through small, medium, and large. Confirm 2, 4, and 6 visible zones respectively, with an overflow count and a working Edit/Customize action.
3. Close and reopen the board, restart `WidgetsRsProvider.exe`, and unpin one instance. Confirm the remaining card and editor still agree.
4. Capture each size in light, dark, and High Contrast. Check readable labels, keyboard access to search and controls, and visible focus.
5. Record package bytes (`(Get-Item dist\*.msix).Length`), process working sets (`Get-Process WidgetsRs,WidgetsRsProvider`), settings startup time, and the provider's inactive exit time.

The app needs package identity to exercise `WidgetManager` and the real board. Either signed MSIX installation or the loose development registration above provides it. Launch the registered app from Start or through `widgetsrs://`; starting an `.exe` directly, even from the registered layout, can display the shell without package identity and fail to read pinned widgets.

## References

- [Windows widget providers](https://learn.microsoft.com/en-us/windows/apps/develop/widgets/widget-providers)
- [Widget provider manifest](https://learn.microsoft.com/en-us/windows/apps/develop/widgets/widget-provider-manifest)
- [Windows Reactor](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)
