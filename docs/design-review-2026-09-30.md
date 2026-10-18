# Visual review: 30 September 2026

This pass compares ImageGen targets with screenshots from the running native app and the real Windows widget board. Concept compositions are not bundled into the package.

## Latest refinement (v8): button icon sizing

The shared command helper constrained `SymbolIcon` to 18 × 18 DIP. Its native glyph did not scale with that layout slot, which clipped the larger symbol. Before captures exposed the cut-off Refresh and Unpin shapes.

- Standardized command and More icons at **16 DIP**, one of Microsoft's [recommended Fluent icon sizes](https://learn.microsoft.com/en-us/windows/apps/design/iconography/segoe-fluent-icons-font), with centered alignment and 8 DIP between icon and label.
- Used a native `Viewbox` around the unconstrained `SymbolIcon`. Reactor 0.100.0 does not expose `FontIcon.FontSize`; the Viewbox scales the complete glyph instead of reducing its layout slot. Native Button styling still supplies foreground colors and the larger click target.
- Reviewed the remaining native navigation and decorative icons. Their existing slots accommodate their glyphs; larger preview/header artwork keeps its intended size.

### Actual visual and input review

Reviewed running Dark Home and My widgets, plus isolated before/after Refresh, Rename, and Unpin captures. The new glyphs are complete, centered, and proportionate to the labels. The accent Open widget board button retains its contrasting icon color. Command buttons remain 42 DIP tall; Unpin is 92 DIP wide and Refresh is 100 DIP wide.

Sky foreground activation failed twice after relaunch. The purpose-built WinApp pointer click then opened Rename, and subsequent Sky clicks worked: Cancel closed Rename, clicking directly on the Refresh glyph updated the Medium instance's timestamp from 1:20 PM to 1:35 PM, and clicking directly on the Unpin glyph opened its native confirmation. Cancel retained the instance. These are real pointer checks in addition to the UIA suite.

Both original Small/Medium pins still contain Los Angeles and UTC, with their fallback names. No name or zone changes were saved, and no widget was removed. Appearance preferences and the full-frame v7 board treatment were retained. This focused pass did not repeat narrow, Light/High Contrast, board, or performance acceptance.

Evidence: `dist/button-icons-v8-home.png`, `dist/layout-smoke-v8-icons/manager.png`, `dist/button-icon-v8-refresh-before.png`, `dist/button-icon-v8-refresh-list-after.png`, and `dist/button-icon-v8-{rename,unpin}-after.png`.

### Verification

- Formatting and Clippy with warnings denied passed. **32 release UIA checks** passed, including navigation, search feedback, editors, and Unpin cancellation. No new mirrored implementation tests were added for this layout change.
- Signed MSIX: **1,731,129 bytes**, SHA-256 `59664FE08FEF3B98313F486C6052E84FA10E3D4CB76515940BB75716D2DBEB16`, staging `dist/package-layout-15a22d86`.
- The registered development app matches the rebuilt settings executable and is running locally. The provider was left on the unchanged v7 executable; this was an app-only development-layout update. Signed-install verification remains pending certificate trust.

## Earlier refinement (v7): full-frame widget artwork

The real board exposed visible horizontal bands above and below the World Clock artwork. The earlier treatment kept the default header, used a content-sized Adaptive Card, and baked matching-color fades into the JPEGs. The app preview already covered its entire frame, so it understated the board problem.

- Used the documented [empty-header extension](https://learn.microsoft.com/en-us/windows/apps/develop/widgets/widget-header-customization) (`header: null`) and a size-specific root minimum height to extend the card surface across the rounded frame.
- Retained World Clock attribution in a 40 DIP body row, with its clock icon and space reserved for Windows' More options button. High Contrast uses native text and omits imagery.
- Removed the artificial top/bottom fades from the Light/Dark JPEG faces. The background covers the frame with centered cropping; the app preview inherits the same regenerated faces. The native app shell was not redesigned.

### Actual board review

Reviewed live Dark captures at Small, Medium, and temporary Large. Medium/Large now show continuous artwork through the bottom corners; Small has an uninterrupted solid surface. City labels, times, separators, and publication timestamps remain readable. The native More options menu opened through a real pointer click. Resizing used its TogglePattern, and a real click on the Large card selected the original instance in the already-running app. The test resize was restored to Medium.

Before: `dist/board-background-before-v7.png`. Final board: `dist/board-background-v7-final-dark.png`. Individual host captures: `dist/widget-background-v7-small-dark.png`, `dist/widget-background-v7-medium-dark.png`, and `dist/widget-background-v7-large-dark.png`. The native Large app preview was also inspected. These are actual screenshots, not concept renders.

Both original IDs remain pinned at Small/Medium with Los Angeles and UTC. Refresh and resize did not save names or zones. No pin was removed, and Windows appearance preferences were not changed in this pass. Sky activation now succeeds for the app; this pass only claims the pointer flows described above. Fresh Light/High Contrast board acceptance and the broader outstanding v6 checks remain pending.

### Verification

- **22 workspace tests**, formatting, and Clippy with warnings denied passed. The new card regression check covers complete frame heights, explicit header override, attribution, reserved menu space, and normal/High Contrast output.
- Signed MSIX: **1,731,169 bytes**, SHA-256 `4D36D2E1F157FC5A71E5C3E7295A0BE240D66D7974262DDFF966CA299920FD05`. Registered development executables match staging `dist/package-layout-5f62bd85`. Identity, CLSID, definition ID, and package version are unchanged. This is a local development-layout update, not signed-install verification.
- No new startup or memory benchmark is claimed for this focused card fix. Earlier performance misses remain open.

## Earlier refinement (v6): native shell and city search

- Removed the full-window bitmap and the separate title-bar fill. Navigation content now lets the native Mica Alt backdrop show through. The rail continues behind the title bar; lighter translucent cards reduce the stacked blue-panel effect. High Contrast retains system fills and strokes.
- Added a readable platform mark and app name in the native TitleBar content area. Windows still supplies caption buttons and drag behavior. This uses the [standard TitleBar content pattern](https://raw.githubusercontent.com/microsoft/microsoft-ui-xaml/main/controls/dev/TitleBar/TitleBar.xaml), without a replacement control template.
- Widened navigation to 232 DIP and increased item spacing. Actual minimum-size captures exposed shifted compact icons; margins now differ between compact and expanded navigation, keeping the 48 DIP rail centered.
- Replaced the city search TextBox and expanding result rows with a native AutoSuggestBox. Selecting a suggestion reveals an explicit Add action. Moving through suggestions does not save a zone. Reactor exposes TextChanged without WinUI's reason; a small state model preserves the filter in either TextChanged/SuggestionChosen order. Validation resolves only bundled IDs, and normal typing avoids cloning the complete city index.
- Centralized editor reset when selecting an instance, following an Edit route, losing a removed instance, or successfully unpinning. A pending city choice, selected city, or rename form cannot carry into the next instance after removal.

### Actual visual review

Reviewed final Dark Home/manager, Light and High Contrast Home/manager, Discover, city editing, and the native Unpin dialog. The 650 × 520 Debug QA captures show the corrected compact rail, readable editor, and fixed footer. The initial narrow harness failure was a test-navigation mistake: the instance list is absent while its full-page inspector is open. The harness now returns through All widgets before reselecting.

Current captures: `dist/app-manager-v6-final-dark.png`, `dist/app-home-v6-final-dark.png`, `dist/app-manager-v6-final-light.png`, `dist/app-manager-v6-final-contrast.png`, `dist/layout-smoke-v6-final-release`, and `dist/layout-smoke-650-v6`. Light/Contrast and minimum-size captures preceded the final lookup/reset cleanup; those changes retain the reviewed layout. Windows appearance and normal 1420 × 860 release startup size were restored.

These captures remain inactive-window evidence. Sky foreground activation failed with **“failed to activate captured window”** after fresh selection and one retry. WinApp focus also returned `foreground_not_target`. Active Mica, actual pointer/keyboard input, title-bar drag, and native suggestion-popup choice are not verified. The existing live board process exposes no visible window; it was not restarted in this pass. Real-board themes and a fresh disposable-pin removal check remain open.

### Verification and measurements

- **21 workspace tests** passed. Final settings tests and Clippy with warnings denied passed after the lookup/reset cleanup. Formatting and diff checks passed. Build tools reported incremental-cache access notes; release builds completed.
- **32 final wide release UIA checks** passed, including search feedback, instance selection, rename/city editors, and native Unpin cancellation. **34 minimum-size Debug checks** passed before the cleanup. UIA ValuePattern exercised AutoSuggestBox's editable child; this does not verify popup selection or mouse hit testing.
- Final signed MSIX: **1,734,523 bytes**, SHA-256 `3C9150FA5BB12D9FF2D1A8EF3E3A54ABCCE522137591E6A7D166C39140E446C6`, staging `dist/package-layout-3aee4849`. Registered development binaries match staging. Identity, CLSID, definition ID, and both original Small/Medium pins are retained; zones and names were not saved or removed in this pass. Signed-install verification remains pending machine-level development-certificate trust.
- Final rendered Home: **115,515,392 bytes** working set. Fresh manager: **122,064,896 bytes**; after the full UIA suite: **128,757,760 bytes**. Earlier identical-layout theme stress reached **131,620,864 bytes**. Fresh use is below the decimal 125 MB target, while the full UIA/stress samples miss it.
- Final Shell launch: **1,563 ms** to a nonzero HWND, with the first process observed at **971 ms**. Earlier v6 launch proxies ranged from **805 to 1,644 ms**. The 500 ms target remains a miss; these are handle observations, not composed-frame timings. Raw samples distinguish builds in `dist/measurements-v6-final.json` and its referenced prior samples. Provider code/card output did not change; no fresh provider memory/exit measurement is claimed.

The visual-quality goal remains open. The next material/input review needs a foregroundable desktop; neutral inactive Mica captures cannot establish fidelity to the concept's active blue material.

## Earlier refinement (v5)

- Generated a dedicated portrait Earth asset. Replaced the stretched, faint planet and decorative orbit arcs in Medium/Large cards with natural proportions, atmospheric light, and a quieter area behind labels. Small stays typographic.
- Added separate Light/Dark JPEG faces. Adaptive Cards select their face with `$host.hostTheme`; the app uses Windows' color preference and `UISettings.ColorValuesChanged`. The event only redraws imagery, without record polling or resetting editor drafts. Its COM revoker stays on the startup STA.
- The first real-board screenshot showed the host cropping away the bottom fade. Bottom alignment now retains it and blends the artwork into Windows' reserved footer. [Adaptive Cards background-image cropping](https://learn.microsoft.com/en-us/adaptive-cards/schema-explorer/background-image) supports this alignment and JPEG data URLs.
- Corrected the catalog artwork's proportions and replaced the opaque clock box with a translucent native surface. The Light catalog image has its own pale blue treatment; High Contrast hides decorative imagery.
- Replaced the selected instance's complete accent outline with a narrow accent indicator, widened the list to 310 DIP, and simplified the city-search caption. Selecting an instance retains navigation/selection focus; opening label and rename forms still requests focus on their inputs.

### Evidence

The actual board rendered the new JPEG backgrounds and large outlined clocks at Small, Medium, and a temporary Large size. Reviewed `dist/board-v5-bottom-aligned-dark.png` and `dist/board-v5-small-large-dark.png` show the corrected fade and readable city/time hierarchy. The original Medium size was restored; both original IDs still have Los Angeles and UTC, with no instance rename saved.

App captures were reviewed in Light, Dark, High Contrast, and a 650 × 520 Debug startup variant. `dist/app-v5-mixed-app-light-system-dark.png` verifies a live app-Light/system-Dark switch while retaining the unsaved `Theme draft probe` name. The draft was cancelled. Home's corrected catalog surface is visible in `dist/app-home-v5-final-light.png`; the final manager is `dist/app-manager-v5-final-dark.png`.

The live board theme captures are **incomplete acceptance evidence**: when its chrome switched Light, its Adaptive Card content still used dark rendering; stock cards also retained white text. In High Contrast, the explicitly refreshed Large clock used native text and no artwork, while the unrefreshed Small clock retained its earlier face. A clean board restart did not return a visible window through the app's action or taskbar toggle. `dist/board-small-large-v5-light.png` and `dist/board-small-large-v5-contrast.png` record this problem; they are not theme passes. Windows appearance was restored to Dark with High Contrast off, and the app confirms the two pins at Small/Medium.

The desktop still rejects `sky.activate_window` with **“failed to activate captured window”**, including one retry after fresh selection. No fresh pointer, keyboard, title-bar drag, active Mica, or disposable-pin removal acceptance is claimed. UIA remains useful for layout, state, native resize menus, and callback checks.

### Verification

- 19 workspace tests, formatting, and Clippy with warnings denied passed after the card changes. The final wide release passed 29 UIA checks (`dist/layout-smoke-v5-final-release`); the minimum-size variant passed 30. The minimum-size check preceded the catalog artwork and search-focus refinements, which retain its responsive layout.
- Final signed MSIX: **1,750,979 bytes**, SHA-256 `B82D8C586FBA1C5DA6FB74B25AF06DDA3D343D32041577A9B11F205C096CA5B8`; staging layout `dist/package-layout-4bcf38f1`. The development layout is updated locally; this is not a signed-install verification.
- Latest fresh Home: **114,987,008 bytes** working set; final manager after UIA: **128,868,352 bytes**. Packaged Shell launch: **1,029 ms** to a nonzero HWND. The earlier v5 manager reached **141,103,104 bytes** after theme/UIA stress; the provider was sampled at **17,108,992 bytes** before the last card-alignment change. The 500 ms startup and decimal 125 MB manager targets remain misses. Raw samples distinguish the builds in `dist/measurements-v5-final.json`; registered executable hashes match final staging.

The visual-quality goal remains open. The new board artwork is closer to the size-correct generated target; active-window material, real input, complete board themes, removal acceptance, and performance still require work.

## Earlier refinement (v4): surfaces and editor

- Replaced repeated clock thumbnails in My widgets with a quiet instance list: name, type, size, and update time. Reduced row spacing and badge padding; High Contrast also has an explicit selected-text cue.
- Rebuilt the editor as a city workspace beside a size-aware board preview. Selecting a city replaces the city list with its label, order, and removal controls. At narrow widths this form fills the detail page, keeping its heading and Done button visible. Rename has a separate inline editor, and each input receives focus after publication.
- Kept Refresh and Unpin in a fixed footer. The first city-editor capture revealed clipped footer buttons; the revised layout keeps them visible while content scrolls.
- Matched the preview's base to a semantic solid surface. The first capture had a gray preview that differed from the darker board card.
- Added a native platform icon to the extended 52 DIP title bar through a lightweight WinUI style. The icon inherits its foreground. Caption buttons and drag behavior remain native.
- Removed empty views that added unwanted StackPanel gaps. Corrected the Discover rail width so a 300 DIP preview fits inside its padding.
- Made Home's right rail a guide to the Windows board. World Clock remains the one released catalog type.
- Replaced flat gray card fills with native, translucent gradient brushes defined separately for Light, Dark, and High Contrast. The underlying window still uses Mica Alt. No custom control templates or input overlays were introduced.
- Replaced the app's World Clock symbols with the system Fluent globe. It inherits the foreground and scales through a native Viewbox in the detail heading.
- Corrected the narrow NavigationView: a closed 48 DIP rail replaces the obstructing 210 DIP overlay, with a compact footer icon. Opening the pane restores readable labels. My widgets uses a list-to-detail flow and an All widgets return action.
- Replaced the offscreen inline Unpin prompt with a native ContentDialog naming the selected instance and size. Cancel is the default action. The original pins were only used for cancellation checks.
- Removed decorative artwork from the app in High Contrast through live theme resources. High Contrast Adaptive Cards omit their background images and use native text rather than outlined numerals.

The manager target is [manage-digital-v2.png](concepts/manage-digital-v2.png). The app now follows its composition, readable digital city rows, surface hierarchy, and globe icon. These captures were taken without the app in the foreground. [Mica falls back to a neutral color when inactive](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-mica-win32), so its active material still needs a fresh visual pass. The generated target has a stronger blue tint; visual parity is not claimed.

The styles extend native defaults where the default resource is available, following the [WinUI style guidance](https://learn.microsoft.com/en-us/windows/apps/develop/platform/xaml/xaml-styles). In this runtime, loose XamlReader could not resolve `DefaultTitleBarStyle`; adding another XamlControlsResources dictionary made stock gray fills override the app resources. The final dictionary avoids both problems. An appearance-load failure is surfaced in an InfoBar and checked by the UIA runner.

## Larger board clocks

The real host accepted embedded SVG paths. Medium and Large cards with one or two visible cities now use glyph outlines from installed Segoe UI Semibold, at nominal 48 / 56 DIP, with accessible descriptions and Light/Dark variants. No font is bundled. Windows' regional formatter inserts invisible bidi markers; an early prototype outlined them as stray strokes. Filtering them fixed the live board. Scripts needing shaping and High Contrast use native text instead.

Offset and day cues share one line above the time. Small and dense four/six-zone cards retain compact native text. One captured instant still supplies every row, and the footer identifies publication time. Windows owns chrome, placement, sizing, and its reserved bottom strip.

## Reviewed captures

These are ignored local QA artifacts, not generated illustrations.

| Surface | Capture |
| --- | --- |
| Home, Dark release | `dist/layout-smoke-v4-final-release/home.png` |
| Manager, final native surfaces and globe | `dist/app-manager-v4-final-dark.png` |
| City editor with fixed footer | `dist/layout-smoke-v4-final-release/city-editor.png` |
| Inline rename / native Unpin dialog | `dist/layout-smoke-v4-final-release/rename.png`, `dist/layout-smoke-v4-final-release/unpin-confirmation.png` |
| Discover and search | `dist/layout-smoke-v4-final-release/discover.png`, `dist/layout-smoke-v4-final-release/catalog-no-results.png` |
| Home / manager, Light release | `dist/app-home-digital-v4-light.png`, `dist/app-manager-digital-v4-light.png` |
| Home / manager, High Contrast release | `dist/app-home-digital-v4-contrast.png`, `dist/app-manager-digital-v4-contrast.png` |
| 650 × 520 minimum-size startup, Debug QA variant | `dist/layout-smoke-650-v4/manager.png`, `dist/layout-smoke-650-v4/city-editor.png`, `dist/layout-smoke-650-v4/unpin-confirmation.png` |
| Real board, Small and Medium, Dark restored | `dist/board-digital-v2-dark-restored.png` |

Light/Contrast captures include the final globe and selected-text changes. They show readable labels, times, actions, focus strokes, and no wide-layout overlap. The minimum-size form keeps its label, Done, ordering controls, and footer visible; the normal city list and preview use a vertical viewport at that height. Original Windows appearance and the normal 1420 × 860 release startup size were restored. Both original pins remain Small/Medium with Los Angeles and UTC.

## Verification and limits

- 19 workspace tests passed; formatting and Clippy with warnings denied passed. Coverage includes High Contrast cards without artwork, outline bounds, fallback, bidi filtering, cached inactive COM references, and stale activation after host termination alongside existing state, time, route, and manifest checks.
- The final wide release passed 29 UIA checks; the 650 × 520 Debug startup variant passed 30, including the return to the instance list. These exercise navigation, selection, editor visibility, catalog search, appearance loading, and native Unpin cancellation without changing existing custom state. `tools/ui-layout-smoke.ps1` makes this repeatable; use `-Narrow` for the list-to-detail layout.
- Protocol activation selected the Medium pin in the earlier layout and the Small pin in the final release, retaining the existing process and window. Foreground reconciliation preserves the current editor; a separate app activation asks Windows to restore the existing window.
- Signed x64 MSIX: **1,914,862 bytes**. SHA-256: `67B1D5BB795798C9FEE11FFAA2D9E47676986A6E6E9E132CA8C84018C4495B14`. Both executables in the registered development layout match the final staging layout. Signed installation still needs machine-level development-certificate trust.
- Earlier revised-provider working set: **18,051,072 bytes**. Latest fresh Home: **115,232,768 bytes**; manager after the final UIA suite: **129,806,336 bytes**. This manager run misses the decimal 125 MB target by about 4.81 MB. Samples and package identity are recorded in `dist/measurements-v4.json`.
- The final packaged Shell launch measured **1,004 ms** to a nonzero window handle. This launch proxy misses 500 ms and does not measure the precise composed frame.

The desktop rejects foreground activation for injected pointer input. UIA invocation bypasses hit testing, so this pass does not claim fresh pointer, keyboard, title-bar drag, or resize-gesture acceptance. Minimum-size startup captures do verify the responsive layout. The attempt to open a disposable third pin reached the picker via UIA, then the board lost visibility and its capture was black; no third pin was added and no existing pin was removed. A fresh destructive-dialog acceptance check, active-window material review, full real-board theme/size review, and the performance misses remain outstanding. The visual-quality goal remains open.

## Provider shutdown correction

Closing the board revealed a provider that remained running. The old idle gate required every COM interface to be released and trusted an activation set indefinitely. The revised policy uses the host's [WidgetContext.IsActive](https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.windows.widgets.providers.widgetcontext.isactive?view=windows-app-sdk-2.0) property to reconcile stale activation during its existing five-second wake-up. It preserves state if the query fails or a callback re-enters during the snapshot. Last-interface release clears stale activation after a host crash. An inactive cached interface no longer prevents grace expiry; explicit server locks still do. The app continues to have no background record polling.

The revised provider exited **32,471 ms** after the board's native taskbar toggle closed it (`dist/provider-inactive-v2.json`). Reopening through the app started a new provider process and refreshed both original cards. UIA found both original instance IDs with Los Angeles and UTC, and the screenshot `dist/board-digital-v2-provider-restarted.png` shows their updated times. The board retains an appearance cache after programmatic contrast testing; `board-digital-v2-dark-restored.png` is the normal Dark visual reference.
