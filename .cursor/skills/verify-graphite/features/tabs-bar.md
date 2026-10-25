# Tabs bar

The tabs bar lays out the space menu, tab chips, and related controls across a window width. A hit test names the control under a pixel.

## Sub-features

- `tabs-layout` places a 1440px bar and three tabs for the space `release-lab`.
- `tabs-hit-menu` reports the space menu at the dropdown's top-left pixel.
- `tabs-hit-tab` reports the first tab, not its close control, at that tab's top-left pixel.

## How to get to it (user POV)

- A host asks Graphite to lay out the tabs bar for a space label, tab labels, and a command chord.
- The host asks which control contains a pointer position.

## Driving it with verify-graphite

Preconditions:

- `doctor.sh` has printed `package=graphite-core` for this checkout.
- `target/verify-graphite/driver.pid` does not name a live process.

- **Layout.** Run `.cursor/skills/verify-graphite/scripts/drive.sh tabs-bar`. Exit code 0. `target/verify-graphite/tabs-bar/drive.txt` contains `bar 0 0 1440 44`, `space-label release-lab`, `tabs 3`, and `first-chip 226 7 107 30`.
- **Space menu hit.** The same file contains `hit-dropdown space-menu`.
- **First tab hit.** The same file contains `hit-first tab 0`.

## Gotchas

- This drive measures text as scalar count times pixel size, at scale 1000. Legend packing uses a different metric.
- Pixel `(226, 7)` is the first chip's top-left, not its close button. A close hit would read `tab 0 close`.
- The bar height 44 is the 1x tabs-bar height. A 2x scale is a different layout.
