# Verification

## Current pass: v8, 30 September 2026

Button icons now scale their complete native glyph to 16 DIP through a Viewbox, instead of clipping a larger SymbolIcon into an 18 DIP layout slot. Shared commands and More icons use this helper. Details and recommended-size documentation are in [the visual review](design-review-2026-09-30.md).

- Formatting, Clippy with warnings denied, release build, and **32 release UIA checks** passed.
- Actual Home/manager and isolated Refresh, Rename, and Unpin screenshots were reviewed. Real pointer checks opened/cancelled Rename, refreshed the Medium instance by clicking its icon, and opened/cancelled Unpin by clicking its icon. Both original Small/Medium pins, zones, and fallback names are retained.
- Final signed x64 MSIX: **1,731,129 bytes**, SHA-256 `59664FE08FEF3B98313F486C6052E84FA10E3D4CB76515940BB75716D2DBEB16`, staging `dist/package-layout-15a22d86`. The updated development app is running; its hash matches staging. Provider/card code was unchanged and the registered provider retains the v7 executable. This is not signed-install verification.
- Evidence: `dist/button-icons-v8-home.png`, `dist/layout-smoke-v8-icons`, and `dist/button-icon-v8-*-after.png`. This focused pass did not repeat theme, narrow-layout, board, startup, or memory acceptance. Earlier outstanding gates retain their stated limits.

## Earlier pass: v7, 30 September 2026

World Clock's background now covers its rounded board frame, using the Windows empty-header extension, full-size root minimum height, and regenerated JPEGs without top/bottom fades. The custom attribution row reserves space for the native menu. Details and primary documentation are in [the visual review](design-review-2026-09-30.md).

- **22 workspace tests**, formatting, and Clippy with warnings denied passed. The new card check includes all frame sizes and High Contrast output.
- Actual Dark board screenshots were reviewed at Small, Medium, and temporary Large. The native menu was opened with a real pointer click; its size TogglePattern worked. A real clock-card click routed to the correct instance in the existing app.
- Both original pins remain Small/Medium, with Los Angeles and UTC and no saved name/zone changes. Medium was restored after Large QA. No appearance preferences were changed. Fresh Light/High Contrast board acceptance and unrelated outstanding UI/performance checks remain pending.
- Final signed x64 MSIX: **1,731,169 bytes**, SHA-256 `4D36D2E1F157FC5A71E5C3E7295A0BE240D66D7974262DDFF966CA299920FD05`. Development executables match `dist/package-layout-5f62bd85`; signed installation still requires certificate trust.
- Evidence: `dist/board-background-before-v7.png`, `dist/board-background-v7-final-dark.png`, and the three `dist/widget-background-v7-*-dark.png` host captures. No fresh performance benchmark is claimed.

## Earlier pass: v6, 30 September 2026

The native shell now joins the title bar and page surface, with platform branding, clearer navigation spacing, lighter cards, and native city suggestions. Full details and reviewed captures are in [the visual review](design-review-2026-09-30.md).

- **21 workspace tests**, formatting, and Clippy with warnings denied passed. Picker tests cover bundled-ID validation and both native suggestion-event orders.
- **32 final wide release UIA checks** passed. **34 minimum-size Debug QA checks** passed before the final lookup/editor-reset cleanup; layout is unchanged. Search feedback, navigation, editors, and Unpin cancellation were exercised. Both original pins remain Small/Medium with Los Angeles and UTC; no name or zone changes were saved.
- Light, Dark, High Contrast, and 650 × 520 captures were visually reviewed. System appearance and the normal release startup size were restored. Active Mica, real pointer/keyboard input, title-bar drag, and native popup selection remain unverified: foreground activation and focus are rejected by Windows. The live board process has no visible window; no board restart was performed in this pass.
- Final framework-dependent x64 MSIX: **1,734,523 bytes**, SHA-256 `3C9150FA5BB12D9FF2D1A8EF3E3A54ABCCE522137591E6A7D166C39140E446C6`. Registered development executables match `dist/package-layout-3aee4849`; identity/version remain unchanged. This is not a signed-install verification.
- Final rendered Home: **115,515,392 bytes** working set. Fresh manager: **122,064,896 bytes**; after full UIA: **128,757,760 bytes**. Earlier v6 theme stress: **131,620,864 bytes**. The sustained decimal 125 MB target is not met. Final Shell-to-HWND proxy: **1,563 ms**; earlier v6 proxies **805–1,644 ms**. The 500 ms startup target is not met.
- Raw samples: `dist/measurements-v6-final.json` and referenced prior samples. Provider/card behavior was unchanged; the v5 provider/board evidence below still has its stated limits. Fresh complete board themes, disposable-pin removal, and signed installation remain pending.

Use `tools/ui-layout-smoke.ps1` for UIA callback/layout checks and `tools/ui-click-smoke.ps1` for real pointer testing. AutoSuggestBox exposes its editable child as `TextBox`; setting that value verifies search callbacks, not popup choice.

## Earlier pass: v5, 30 September 2026

The current evidence is summarized in [the 30 September visual review](design-review-2026-09-30.md). It supersedes older package sizes and "final" snapshots below.

- Signed framework-dependent x64 package: **1,750,979 bytes**. Identity/version stay `WidgetsRS.Dev` / `0.2.0.0`. Registered app/provider hashes match staging `dist/package-layout-4bcf38f1`.
- **19 workspace tests** and Clippy with warnings denied pass, including an artwork-free High Contrast card check.
- **29 final wide release UIA checks** and **30 minimum-size Debug QA checks** pass with actual Home, manager, editor, Discover, and Settings captures. Existing custom state was preserved; Unpin was cancelled in its native dialog.
- The native extended title bar, translucent surfaces, Fluent globe, instance selection indicator, quieter city search, themed portrait Earth preview, and fixed footer are running locally. Label/rename forms receive focus when opened. Catalog art keeps its natural proportions and uses a translucent clock surface. Narrow navigation uses a compact rail and a list-to-detail flow.
- Both original Small/Medium pins remain. Real-board Dark captures verify Small, Medium, and temporary Large, including JPEG backgrounds, SVG clock outlines, and the corrected bottom fade. Medium was restored without editing zones or names.
- App Light/Dark/High Contrast and 650 × 520 startup captures were reviewed. The app's live mixed-theme switch retained an unsaved rename draft, which was cancelled. Appearance and normal release size were restored. High Contrast hides decorative app artwork.
- Board Light/High Contrast acceptance remains incomplete: live programmatic switching left stale dark card content, including in stock widgets. A clean restart did not return a visible board window through the app or taskbar action. Pointer, keyboard, title-bar drag, resize gestures, active Mica, and fresh disposable-pin removal checks remain pending because foreground activation fails.
- V5 provider sample before the last card alignment: **17,108,992 bytes**. Final Home **114,987,008 bytes**; manager **128,868,352 bytes** after UIA. Earlier v5 theme/UIA stress reached **141,103,104 bytes**. Final Shell startup **1,029 ms** to a window handle. The decimal 125 MB manager and 500 ms targets remain misses; this startup proxy is not a composed-frame measurement. Raw samples: `dist/measurements-v5-final.json`.
- Revised provider exited **32,471 ms** after board close. Reopening started a new process and refreshed both original pins; their IDs and city selections were retained.
- Local development registration works. Signed MSIX installation still requires machine-level trust of the development certificate.

Use `tools/ui-layout-smoke.ps1` for reproducible UIA checks and `tools/ui-click-smoke.ps1` for the separate pointer check. UIA InvokePattern does not prove mouse hit testing.

## First milestone

Checked on Windows 11 Pro build 26200, x64, 2026-09-27.

| Check | Result |
| --- | --- |
| Rust formatting, workspace check | Passed |
| Clippy with warnings denied | Passed |
| Clock and card tests | 9 passed |
| Release MSIX build and signing | Passed, 950,497 bytes |
| Provider executable | 298,496 bytes |
| Settings executable | 1,936,384 bytes |
| Native settings window | Opened in about 220 ms from `Start-Process` to a window handle (unpackaged run) |
| Settings working set | About 74 MB at first window, 98–108 MB after rendering and UI automation |
| Provider working set | About 13.5 MB before COM activation, 30.5 MB after COM activation; about 2.5 MB and 4.9 MB private bytes respectively |
| Provider inactive exit | Exited successfully after the 30–35 second grace period |
| Loose development package registration | Passed; app starts with package identity and can query `WidgetManager` |
| Widget picker | `Widgets RS` and `World Clock` discovered; light/dark picker assets rendered |
| COM activation | Passed through Windows' packaged COM registration |
| App protocol | `widgetsrs://edit/example-id` launches the packaged settings app |
| Two pinned instances | User pinned two World Clock widgets; both appeared separately in the editor |
| Instance isolation | Reordering and adding Tokyo to the first instance left the second instance unchanged; test changes were restored |
| Custom labels | Renaming the first instance's Los Angeles row to `Home` changed only that instance; its original label was restored |
| Editor search and preview | Empty search has no unrelated suggestions and keeps the preview visible; typing `Tokyo` finds the offline city entry |
| Settings restart | The reordered first instance and untouched second instance were both retained after closing and relaunching the editor through its protocol |
| Board resizing | User confirmed resizing works |
| Board render after provider restart | Both pinned widgets appeared after the provider process was stopped and the board reopened; dark-theme small and large cards captured |
| Card Edit action | Opened the editor with the selected widget ID in the protocol URI |
| Board Customize action | Opened the editor with the other widget ID in the protocol URI |
| Medium size | Resized the small card to medium, captured it, then restored small |
| Full large card | Added five temporary zones, captured six visible clocks plus the `+1 more` cue, then removed the test zones; revised layout keeps times and day cues on separate lines without collisions |

The first COM activation attempt exposed an incorrect wait flag value: `CoWaitForMultipleObjects` returned `E_INVALIDARG` and never dispatched the class factory. The flags now use `CWMO_DISPATCH_CALLS=1` and `CWMO_DISPATCH_WINDOW_MESSAGES=2`, and an actual `CoCreateInstance` succeeded afterward.

**Remaining acceptance:** light and High Contrast screenshots at all sizes, keyboard focus review, and unpin behavior still need an interactive pass. The board did not stay in the foreground reliably for pinning automation, so the user pinned the two widgets. The signed MSIX itself was not installed because trusting the development certificate in the machine store requires an elevated PowerShell session; the loose development package was registered and tested without elevation. Screenshots of the dark board, medium-size card, full large card, and restored two-widget board are in `dist/board-after-restart.png`, `dist/board-medium.png`, `dist/board-large-seven-zones-fixed.png`, and `dist/board-final.png` (build artifacts, ignored by Git).

## Catalog and manager milestone (0.2.0.0)

Checked on the same Windows 11 x64 machine on 2026-09-28. The development package was upgraded in place from 0.1.0.0 to 0.2.0.0. Both pre-existing World Clock IDs (`6261097c-0de5-41bc-b6f7-ac51baa97567` and `90abc2f9-e75e-48b7-827f-59cf5b1e8b8b`) and their city selections remained available. The signed MSIX is a separate installable artifact; the running test used the loose development registration because the development signing root is not trusted in the machine certificate store.

| Check | Result |
| --- | --- |
| Formatting, workspace tests, Clippy with warnings denied | Passed; 13 tests, including registry/manifest agreement, v1 migration, name validation, and route decoding |
| Release x64 MSIX | Signed successfully; **1,032,765 bytes** after the pointer-input fix (target < 2 MB) |
| Editor working set | 97 MB at Home and 107 MB with a selected instance; highest observed populated view 115 MB (target < 125 MB) |
| Provider working set | 15.3 MB while the board was open (target < 35 MB); process was absent after the board became inactive, though the interval was not timed in this pass |
| Visible window from packaged Shell launch | 829, 913, and 963 ms (target < 500 ms: **missed**). In one run, process observation to visible window was 139 ms; most delay was before process creation in Shell activation |
| Home | Both pins listed; World Clock discovery card and board action visible |
| Empty Home | A temporary `WidgetsRS.EmptyTest` development identity with no pins showed `Pinned widgets · 0` and the pinning guidance; its My widgets page showed the empty state. The temporary package was removed, and the main identity still had two pins |
| Discover | `timer` search returned no released matches; selecting World Clock revealed a multi-city live preview and small/medium/large previews |
| My widgets | Two independent records with type, size, last update, editor, preview, and explicit Refresh |
| Rename | Renamed one instance to `Travel`, observed it only in the app, then cleared the name and confirmed the city-summary fallback; the other pin was unchanged |
| City editing | Added Tokyo to one pin, confirmed its row appeared, then removed it; both original zone lists restored |
| Existing-window Edit route | Protocol activation selected the requested pin in the existing window and the redirect process exited; a removed ID showed the inline message on My widgets |
| Pin and unpin | Opened the Windows picker from the app, added a disposable third pin, canceled Unpin once, then confirmed the dialog naming its size and ID; only the third pin was deleted |
| Responsive view | At a 720 px window width, selecting a pin opened a full detail page with an **All widgets** return action |
| Appearance | Dark, Light, and Windows Contrast theme screenshots captured; original system appearance restored. Focus remained visible in Contrast mode |
| Settings | Package full name and shared runtime status shown; no theme override or Updates page |

The signed MSIX install command returned `0x800B0109` because the development root certificate is absent from **LocalMachine** trust. The package remains registered in development mode and functional. An elevated `tools\build-package.ps1 -Install` run is needed to verify the signed-install path.

Screenshots captured during this milestone are ignored build artifacts in `dist/`: `home-0.2.png`, `home-empty-0.2.png`, `home-light-0.2.png`, `home-contrast-0.2.png`, `discover-selected.png`, `manage-final.png`, `narrow-detail-0.2.png`, and `unpin-confirm-0.2.png`.

### Visual correction

The first 0.2 shell implemented the page structure with plain text panels, which did not reflect the saved concepts' preview-led hierarchy. The revised Home has two live pinned-clock cards, a featured World Clock shelf, and a guidance rail. Discover has a featured surface, a compact catalog card, and a selected detail rail. My widgets now opens with a selected pin, preview cards in the instance list, and the selected preview above its editor. Navigation icons, accent actions, semantic card strokes, and a 1420 × 860 initial client size bring the composition closer to the concepts while preserving system themes and native controls.

UI Automation confirmed navigation, catalog search with a no-results state, and the populated instance list. Screenshots were inspected at wide width (`home-redesign-final.png`, `discover-redesign-final.png`, `manage-redesign-selected.png`) and at a 780 px window width (`home-narrow-redesign-fixed.png`, `manage-narrow-redesign.png`). The narrow Home initially overlapped two cards; explicit grid rows fixed it. The development identity still reports the same two original pins. The revised editor measured 106 MB working set at Home. The 500 ms startup target remains a recorded miss from the earlier 0.2 build; it was not retimed after the visual revision.

### Material and clock-editor pass

The window now uses native Acrylic with semitransparent WinUI card surfaces. Home, Discover, My widgets, and Settings were visually reviewed from actual UI Automation screenshots. The city list uses compact time rows with UTC offset and day cue; selecting a row opens its label, order, and removal controls in a separate editor. Instance naming is inside a collapsed Widget settings expander. Search results use a full-width add row. Discover's size previews stay readable in an independently scrolling catalog area, while its detail rail remains visible at wide widths. Home and Discover stack their feature content at narrow widths.

Screenshot review caught two concrete defects: Settings cards occupied the same grid cell, and the time-offset calculation read a 12-hour calendar hour as a 24-hour value, producing `UTC−31` near UTC midnight. The grid placement was corrected. Calendar arithmetic now explicitly uses `TwentyFourHour`, while visible times retain the user's regional setting; a UTC-midnight regression test verifies `UTC−7` for Los Angeles and `UTC+0` for UTC.

Dark, Light, and Windows Contrast screenshots were checked after the change. Focus outlines, city labels, times, and actions remained readable in Contrast mode; the system theme was restored. A 650 px minimum-width Discover screenshot showed the feature card without clipping. UI Automation also checked city search, editor expansion, widget settings expansion, and protocol Edit activation into the existing window. Both original pins remained intact. The final editor working set observed in the populated manager was **114,155,520 bytes**; the highest observed after the pointer fix was **117,534,720 bytes** (under the 125 MB target). The signed package is **1,032,765 bytes**. The 500 ms launch target remains unverified for this visual pass and was missed in the earlier measurement.

Representative local screenshots: `dist/home-material-v2.png`, `dist/discover-independent-bottom.png`, `dist/manage-offset-fixed-actual.png`, `dist/manage-search-final.png`, `dist/home-light-material-final.png`, `dist/manage-contrast-material-final.png`, and `dist/discover-min-width-final.png`.

### Pointer-input regression and fix

The visual review used UI Automation's `Invoke` pattern, which bypassed mouse hit testing. A root `Grid` placed an empty, full-window `TextBlock` above the `NavigationView` whenever the Unpin dialog was closed. It intercepted every pointer click. The empty overlay has been removed; the dialog layer is created only while the confirmation is open. A `winapp ui click` check reproduced the original failure before the fix and confirmed navigation, city selection, search typing, settings expansion, and Unpin cancellation afterward. `tools/ui-click-smoke.ps1` now exercises real pointer clicks across Home, Discover, Settings, My widgets, and the city editor. The final registered build passed the smoke check and still showed both original pins.

### Feature and material polish

The current window uses Mica Alt under a translucent blue content layer. Actual-window comparison showed that Acrylic picked up distracting orange and green patches from the test desktop wallpaper; Mica Alt gives the editor a steadier native surface. World Clock's feature card now has a three-city preview with aligned times and row separators. The Home card title and icon share one baseline, pinned cards have clearer clock rows, and My widgets has a bottom catalog callout and a distinct inspector surface. The catalog continues to show only the shipped World Clock definition.

Dark screenshots: `dist/home-final-polish.png`, `dist/discover-final-polish.png`, and `dist/manage-rail-polish.png`. Light screenshots: `dist/home-mica-light.png`, `dist/discover-mica-light.png`, and `dist/manage-rail-light.png`. The final inspector was also captured with a Windows Contrast theme in `dist/manage-mica-contrast.png`: text, city actions, and focus strokes remained legible. Windows Contrast was switched off and the app and system theme values were restored to Dark; `dist/home-restored-dark.png` confirms the normal window afterward.

The final x64 MSIX was signed at **1,038,173 bytes**. All **14 workspace tests** and Clippy with warnings denied passed. `tools/ui-click-smoke.ps1` passed before the final inspector tint, and a physical click on the new **Discover widgets** callout navigated correctly. A repeat after the tint was blocked by Windows refusing to foreground the window, before it sent any click; the tint only changes a background brush. The two existing pins remained present. The editor working set after visiting Home and Discover was **114.8 MB**. The previous startup-time miss (<500 ms target) was not retimed for this visual pass.

### Extended title bar

The shell now places Reactor's native `TitleBar` above `NavigationView` and uses its Tall title bar setting. Reactor registers that element with `SetExtendsContentIntoTitleBar(true)` and `SetTitleBar`, so Mica Alt reaches the top edge while Windows still owns the caption buttons and drag behavior. The duplicate pane title was removed. Centered custom branding and a native subtitle were both rejected after screenshot review; the final title stays left aligned.

### Board and catalog visual pass (29 September 2026)

Live board inspection established 300 × 148, 300 × 306, and 300 × 462 frames on this machine. The provider card now uses a prominent two-row Small layout, vertical hero zones for two-city Medium/Large, and larger dense-row text for four/six-city cards. Its transparent PNG backdrop rendered in the Windows board in Dark, Light, and Windows Contrast. A physical click on the Medium card selected that exact instance in the already-running app. The picker artwork was redrawn as a readable two-city preview.

The native app uses Mica Alt with a low-resolution translucent haze, a tinted extended title bar, a transparent Earth-limb illustration in Home and Discover, and compact instance previews. Discover hides the duplicate catalog card while only one type is released. Small and Medium size previews appear side by side, with Large below. The selected-instance inspector keeps the city editor visible and offers an actual-size preview on demand. At 820 px window width, the inspector became a full page with an **All widgets** return action.

UI Automation added four zones to a disposable third pin, resized it to Large, and captured the six-zone board card. The test pin was removed after confirmation; the original Small ID `90abc2f9-e75e-48b7-827f-59cf5b1e8b8b` and Medium ID `c8b01845-fa0b-44d7-9f75-bb7cb29bfadf` remained. During this test, opening the Reactor `ContentDialog` for Unpin crashed the packaged editor. The confirmation is now an inline native panel at the top of the selected inspector, showing the exact name, size, and ID. UI Automation verified Cancel and Confirm, then observed both original pins still present. The inspector now selects the next remaining widget after removal.

Final signed package before the board-art addition: **1,436,339 bytes** (<2 MB). Working set: **104,267,776 bytes** when the Home window first appeared, **124,776,448 bytes** in My widgets, and **128,638,976 bytes** in Discover after cross-page navigation (125 MB target missed on that path). It later reached about 139 MB after more page/theme/board activity. Shell protocol launch to a visible window: **1,042 ms** (500 ms target missed). The provider had already exited during idle board time in this pass; its earlier active-board measurement was about 15 MB. All **15 workspace tests** passed and Clippy completed with warnings denied. The nonfatal Rust incremental-directory access warning persists on this machine.

Representative reviewed captures: `dist/app-home-final-optimized.png`, `dist/app-mywidgets-final-optimized.png`, `dist/app-discover-final-optimized.png`, `dist/app-mywidgets-narrow-final.png`, `dist/widget-board-light-current.png`, `dist/widget-board-contrast-v2.png`, `dist/widget-board-large-six-zones-v2.png`, `dist/unpin-inline-top.png`, and `dist/home-earth-contrast.png`. The generated target studies are in `docs/concepts/`; screenshots and package layouts remain ignored under `dist/`.

The Medium and Large board backdrops subsequently gained low-opacity Earth artwork derived from the same transparent source. The **final signed package is 1,793,857 bytes**, still under 2 MB. The final Medium card was inspected on the running board in Dark (`dist/widget-board-earth-final-dark.png`), Light (`dist/widget-board-earth-final-light.png`), and Contrast (`dist/widget-board-earth-final-contrast.png`); text remained readable in all three. Theme changes on an already-open board required toggling the board closed and open before the host repainted text colors. After restarting the board process to clear its stale visual cache, the board showed only the two original pins. The app's host record list had already shown those two after the disposable pin was deleted. On a fresh launch of the visually identical preceding build, Home measured **117,354,496 bytes**, My widgets **125,988,864 bytes**, and Discover **128,536,576 bytes** working set in sequence. Shell launch reached a window in **1,074 ms**. The 125 MB editor and 500 ms startup targets remain misses.

The existing Medium pin was temporarily resized to Large to inspect the new background in its actual 300 × 462 host frame (`dist/widget-board-large-earth-final-dark.png`), then restored to Medium. Two temporary zones were added to that pin, with a 32-character custom London label. The four-zone Medium card (`dist/widget-board-medium-four-long-label.png`) kept its times readable and truncated the long label to one line. Both temporary zones were removed; the same Medium ID again showed only Los Angeles and UTC. No extra pin remains.

Discover's featured action now opens the Windows board when World Clock details are already visible, instead of reselecting the same type with no apparent response. UI Automation found the new label and invoked it in the final registered build. The app was left on Home.

Actual-window screenshots: `dist/home-titlebar-release.png` (final packaged build, Dark), `dist/home-titlebar-light.png` (Light), `dist/manage-titlebar-contrast.png` (Contrast), and `dist/narrow-titlebar-dark.png` (720 px window). The compact view kept its navigation, instance preview, editor, and caption buttons readable. Contrast mode was switched off afterward, and the app/system theme values were restored to Dark.

UI Automation found the extended title bar and the native 48 × 48 caption buttons. Invoking Maximize then Restore worked; physically dragging the title bar moved the window 200 × 100 px, and double-clicking it maximized the window. `tools/ui-click-smoke.ps1` passed on the registered build after the title bar change. A repeat against the final packaged executable was blocked before any click because Windows would not foreground the app. Formatting, all 14 tests, and Clippy with warnings denied passed. The signed x64 MSIX is **1,038,540 bytes**; its `WidgetsRs.exe` SHA-256 matches the final release executable. The host showed two pins during the first title-bar checks and one in the final screenshot; no Unpin action was used during this pass.

### Current x64 build after the visual follow-up

The signed development MSIX is **1,806,847 bytes** (`dist/WidgetsRS.Dev_0.2.0.0_x64.msix`). The registered development layout was updated from that package and run. Both original pins are present: Small `90abc2f9-e75e-48b7-827f-59cf5b1e8b8b` and Medium `c8b01845-fa0b-44d7-9f75-bb7cb29bfadf`, each with Los Angeles and UTC. Both cards refreshed on the live board. The current Dark captures are `dist/app-home-current-signed-layout.png`, `dist/app-mywidgets-final-faded.png`, and `dist/widget-board-final-faded.png`.

The app uses a 52 DIP native extended title bar; the real-window screenshot kept the left-aligned title and native caption buttons. The instance cards have a size badge and an edit cue within the selectable control. The Medium preview's source-art seam was removed. The board reserves a bottom strip outside the Adaptive Card background; a source-image fade now blends into it. Adding a root Adaptive Card minimum height did not change the host layout, so that experiment was removed.

All **15 workspace tests** passed and Clippy completed with warnings denied. UI Automation selected both instances, expanded and collapsed the Medium preview, opened and closed a city editor, and invoked navigation through Home, Discover, My widgets, and Settings. The pointer smoke script stopped before its first click because this desktop refused to foreground the app. The last measured fresh-launch performance remains **1,074 ms** to visible window and **117,354,496 / 125,988,864 / 128,536,576 bytes** working set at Home / My widgets / Discover on the prior visually equivalent build; the 500 ms startup and 125 MB editor targets remain misses. The signed MSIX install path still needs machine-level certificate trust; the development registration is functional.

A fresh launch of this exact final binary reached a nonzero main-window handle in **1,833 ms** on the same desktop while the board and other apps were running. One second later, Home used **116,146,176 bytes** working set. My widgets used **126,738,432 bytes**, and Discover used **130,785,280 bytes** after sequential navigation. These are observed single-run working sets, not controlled benchmark averages. The provider process was absent at the end of the run.
