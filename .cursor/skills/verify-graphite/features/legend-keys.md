# Legend keys

Legend keys pack a text run along the bottom edge when it fits. A run that does not fit is omitted entirely rather than clipped mid-label.

## Sub-features

- `legend-fit` places `Prismattyc [1]` in an 80px-wide bar.
- `legend-drop` omits that same run when the bar is only 20px wide.

## How to get to it (user POV)

- A host asks Graphite to pack legend runs into a bottom bar of a given width, bottom edge, and row count.
- The host paints the returned rectangles and strings, or paints nothing when the run was dropped.

## Driving it with verify-graphite

Preconditions:

- `doctor.sh` has printed `package=graphite-core` for this checkout.
- `target/verify-graphite/driver.pid` does not name a live process.

- **Fit.** Run `.cursor/skills/verify-graphite/scripts/drive.sh legend-keys`. Exit code 0. `target/verify-graphite/legend-keys/drive.txt` contains `fitted 1 12 26 14 22` and `fitted-text Prismattyc [1]`.
- **Drop.** The same file contains `dropped 0`.

## Gotchas

- The fitted line is count, then x, y, width, height. Width 14 is the 14-scalar label, not a clipped string.
- This drive counts one cell per scalar and ignores the font pixel size. The tabs-bar drive does not.
- An empty text list is the drop proof. Do not accept a clipped label as a substitute.
