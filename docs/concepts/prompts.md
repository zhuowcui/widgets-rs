# Image generation prompts

## Portrait Earth production asset, 30 September (v5)

Mode: built-in `image_gen`; referenced `docs/concepts/earth-limb-source.png`; transparent background enabled. Saved output: `docs/concepts/earth-portrait-source-v5.png` (1011 × 1557). Original output: `C:\Users\neroc\.codex\generated_images\01a0e644-b6c1-7720-8d2d-957ef6ace0a7\exec-8dc3dd5c-6e60-45ce-9511-7a61eb713e08.png`.

Prompt:

> Use the attached Earth image as a photographic reference only. Create a NEW production-quality transparent raster art asset for a premium native Windows world-clock card. No UI, no text, no numerals, no clocks, no framing, no grids, no orbital rings. Portrait canvas, aspect ratio approximately 300:462. A photorealistic nighttime Earth, atmospheric electric blue rim and subtle warm city lights, natural cloud detail, natural planetary proportions, elegant and restrained. The globe is mostly off canvas to the right and below: its curved illuminated limb enters from the right edge around 35% down the canvas and bends toward the lower center near the bottom. Keep the upper third and the entire left third completely transparent for readable clock labels and times. Only show a graceful partial planet, not a complete globe. The imagery should be crisp enough for a 300 DIP wide widget without becoming busy, with a soft glow and exceptionally beautiful photographic realism. True alpha transparency everywhere outside the planet and atmospheric glow. Do not bake in any background color, stars, shadows outside the globe, gradients, or text; the app supplies theme-aware backgrounds. Preserve a consistent blue Earth identity from the reference, but fit the geometry to the portrait composition.

The full source stays outside the MSIX. `tools/generate_card_backdrops.ps1` creates small themed JPEG faces; `tools/generate_feature_art.ps1` creates proportional, cropped catalog artwork with separate Light/Dark treatments. These are production assets, while the larger generated UI concepts remain references.

Mode: built-in `image_gen`. Use case: `ui-mockup` for all three new concepts; `precise-object-edit` for the final Home refinement. All outputs are raster UX references.

## Digital manager, 30 September

Mode: built-in `image_gen`; use case: `ui-mockup`. Saved project output: `docs/concepts/manage-digital-v2.png`. Original generated output: `C:\Users\neroc\.codex\generated_images\01a0e644-b6c1-7720-8d2d-957ef6ace0a7\exec-a35dce1e-8857-4be3-bba6-3799bdae4895.png`.

Design brief used for this iteration:

> Create a high-fidelity native Windows 11 WinUI 3 management screen for Widgets RS, approximately 1420 by 860. Use Mica Alt, a restrained blue tint, Segoe UI typography, and an extended native title bar. A 210-pixel NavigationView contains Home, Discover, My widgets, and Settings. My widgets has a quiet approximately 310-pixel instance list with two World Clock pins, names, sizes, and update cues; do not repeat miniature clock faces in the list. The editor has ordered digital city rows on the left and an actual approximately 300-by-306 Windows board preview on the right. Show Rename, city search, ellipsis edit affordances, and a quiet footer with Refresh widget and Unpin. World Clock is the only available type. No analog clocks, future widget promises, macOS controls, or web UI. Use the Earth-limb artwork as restrained clock decoration.

## Discover light

> Design the Discover / Widget Catalog home screen of an extensible native Windows 11 WinUI 3 app named Widgets RS. Show a high fidelity, straight-on 16:10 desktop screenshot with a left NavigationView, search and category chips, a featured World Clock module, a grid of widget catalog cards, and a selected-widget detail pane with preview and supported sizes. World Clock is Available; Focus Timer, Calendar Agenda, and System Monitor are Coming soon. Use warm neutral light Fluent Design, Mica, Segoe UI Variable style typography, restrained blue accent, and native controls. Make it unmistakably a catalog for many widgets, not a single-widget utility. No website, macOS chrome, device frame, watermark, or giant illustration.

## My widgets dark

> Design the My widgets management screen for the same app. Show two separate World Clock instances named Home and Travel in a clean list, each with a preview, size, update time, and Edit action. Show a selected-instance inspector with ordered cities, search, reorder controls, preview, and a route to the Windows widget board. Use a denser but refined Windows 11 dark Fluent theme with Mica, strong hierarchy, accessible contrast, and a subtle catalog invitation. Make multi-widget and multi-instance support clear. No marketing site, phone, macOS chrome, watermark, or speculative metrics.

## Unified Home dark

> Explore a unified Home screen for Widgets RS: pinned-widget overview above a compact catalog shelf, plus a selected-widget detail pane. Show Home and Travel as pinned World Clock instances, World Clock as Available, and three future types as Coming soon. Use an Open widget board path for pinning, not a direct Add to board action. Use a calm dark native Windows 11 Fluent visual language, typographic digital clocks, accessible contrast, and generous spacing. No phone, macOS chrome, analog clocks, marketing layout, watermark, or speculative system metrics.

## Home refinement

> Preserve the unified Home information architecture and dark Fluent theme. Remove the contradictory No widgets pinned yet panel because two instances are shown. Replace photographic city backgrounds with typographic digital clocks on native neutral card surfaces. Keep the Open widget board pinning path and Coming soon status for future types. Do not alter the app frame, navigation, or number of pinned widgets.
