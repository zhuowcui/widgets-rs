# Visual review: 29 September 2026

This review uses captures from the running app and the Windows widget board, not renderer simulations.

## Baseline and targets

| Surface | Baseline capture | Current iteration | Design target |
| --- | --- | --- | --- |
| Board | `dist/widget-board-baseline.png` | `dist/widget-board-final-faded.png`, `dist/widget-board-large-six-zones-v2.png` | `docs/concepts/widget-family-tall-v3.png` |
| Home | `dist/app-home-two-pins-baseline.png` | `dist/app-home-current-signed-layout.png` | `docs/concepts/home-v2.png` |
| Picker | `dist/widget-picker-baseline.png` | `dist/widget-picker-v2.png` | Use the current clearer two-zone art |
| Manager | `dist/app-mywidgets-v2.png` | `dist/app-mywidgets-final-faded.png` | Live size-accurate preview and immediately findable controls |

The first board capture showed cramped small-card rows and a tall card with over half its area empty. The old picker image squeezed four tiny clocks into its thumbnail. The new cards use right-aligned times at Small, two vertically composed hero zones at Medium and Large when only two zones are selected, a lightweight translucent geographic arc, and one full-card edit action. The picker now shows two readable clock rows. The app preview uses the same vertical size family and backdrop.

The first generated widget study (`widget-family-v2.png`) assumed a wide Medium card. The live host contradicted that assumption. The corrected target (`widget-family-tall-v3.png`) reflects the observed **300 × 148 / 300 × 306 / 300 × 462** outer frames on this test desktop. Those are observations, not platform-wide guarantees. The app concept (`home-v2.png`) is a direction for material, hierarchy, and card composition; its generated imagery is not bundled.

The final Home feature uses a separate transparent Earth-limb illustration (`docs/concepts/earth-limb-source.png`), optimized to `crates/clock_core/assets/earth-limb.png` for the native app. The full concept compositions are not bundled. Actual Dark, Light, and Windows Contrast captures showed the illustration behind the preview without hiding labels or controls. The title bar is native Reactor/WinUI with a translucent tint over Mica Alt, and its caption buttons remain Windows-owned.

The app manager defaults to a compact preview so the city editor and actions remain visible. A button expands Medium or Large to its actual board height. The Discover page presents the one released type once and places Small and Medium previews side by side; Large follows below. A live six-zone Large board card revealed that the original dense-row type was too small. Increasing time and city text sizes improved legibility without clipping in the 300 × 462 host frame. Medium and Large cards now use a restrained Earth-limb PNG blended over the existing arcs. Actual Dark, Light, and Contrast board captures show readable text over it; the host's built-in text ramp still caps time size below the generated target.

## Renderer constraints used for the design

- Windows hosts the widget frame, placement, ellipsis, and picker. The provider supplies an Adaptive Card. The host currently supports three named sizes, and our app displays 2 / 4 / 6 selected zones for Small / Medium / Large. [Widgets overview](https://learn.microsoft.com/en-us/windows/apps/design/widgets/), [widget design fundamentals](https://learn.microsoft.com/en-us/windows/apps/design/widgets/widgets-design-fundamentals)
- Adaptive Cards give us text blocks, columns, separators, minimum heights, full-card actions, and PNG/JPEG/GIF background images. The live board accepted an embedded transparent PNG data URL and rendered its arc in both current clock cards. [Adaptive Card schema](https://learn.microsoft.com/en-us/adaptive-cards/schema-explorer/adaptive-card), [background image schema](https://learn.microsoft.com/en-us/adaptive-cards/schema-explorer/background-image)
- The widget design guidance reserves a 16 px margin and a header/attribution area, and recommends calm glanceable content and a limited number of touch targets. The current payload has one full-card edit action. [Design fundamentals](https://learn.microsoft.com/en-us/windows/apps/design/widgets/widgets-design-fundamentals), [interaction guidance](https://learn.microsoft.com/en-us/windows/apps/design/widgets/widgets-interaction-design)
- The built-in Adaptive Card text size ramp tops out at ExtraLarge. The generated concept has larger numerals than the host renders from that ramp; we should judge that difference honestly in board screenshots. Moving to HTML would give more visual control, but it would depart from the original native, low-overhead goal. [Widget provider implementation](https://learn.microsoft.com/en-us/windows/apps/develop/widgets/implement-widget-provider-cs)

## Remaining visual acceptance work

1. Review pointer interactions on the exact final build. Two-zone Small/Medium/Large, four-zone Medium with a long label, and six-zone Large have now been captured on the live board. The long label truncates to one line without covering the time. Light and Windows Contrast board/app captures have been reviewed.
2. Retest physical pointer clicks on every app navigation and editor action in the exact final build. UI Automation and a physical widget-card click did select the intended instance.
3. Continue comparing Home, Discover, and Settings against `home-v2.png` at wide and narrow widths. Current visuals are materially closer, but the app still has quieter, flatter areas than the concept.
4. Improve editor memory and startup. The final signed package is 1,793,857 bytes. On the visually identical preceding build, Home measured 117,354,496 bytes working set, My widgets 125,988,864 bytes, and Discover 128,536,576 bytes after page navigation. Shell launch to a visible window took 1,074 ms. The 125 MB memory and 500 ms startup targets are still missed.

The current iteration is a substantial step from the baseline, but these checks remain before calling the visual quality target met.

## Follow-up visual pass

The extended native title bar now has a 52 DIP row. A centered custom brand slot looked out of place in a real screenshot, so the final row keeps WinUI's left-aligned title and Windows caption buttons. My widgets now uses a size badge, clearer selected-card tint, and an edit cue within the real selectable card instead of a button-shaped label that was not itself a control. The selected card and actual-size Medium preview were checked in `dist/app-mywidgets-final-faded.png`.

The Medium Earth source previously ended 26 pixels above its image boundary, creating a sharp horizontal edge in the live preview. The source now extends beyond the card edge. On this host, the Adaptive Card background still ends above a reserved strip at the bottom of Medium and Large widgets. A root `minHeight` trial did not change the host rendering and was removed. The final image alpha fades into the host surface, as reviewed in `dist/widget-board-final-faded.png`. The host's reserved strip and named Adaptive Card text sizes remain visible differences from the generated target.

The final signed MSIX in this pass is **1,806,847 bytes**. Tests and Clippy pass. The two original pins still show Los Angeles and UTC; no third pin remains. UI Automation invoked all main navigation items, instance selection, the city editor, and preview expansion on the exact registered build. A physical pointer sweep could not run because Windows refused to put the app in the foreground for injected clicks; the previous registered build passed that sweep before the latest visual changes.
