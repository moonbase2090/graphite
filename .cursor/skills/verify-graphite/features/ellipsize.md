# Ellipsize

Ellipsize shortens a label that does not fit a caller's width budget and appends an ellipsis. A label that already fits is returned unchanged.

## Sub-features

- `ellipsize-fit` returns a short label unchanged.
- `ellipsize-cut` keeps a scalar prefix and appends `…` when the label is wider than the budget.

## How to get to it (user POV)

- A host asks Graphite to fit a pane or tab label into a measured width.
- The host reads the string Graphite returns and draws that string.

## Driving it with verify-graphite

Preconditions:

- `doctor.sh` has printed `package=graphite-core` for this checkout.
- `target/verify-graphite/driver.pid` does not name a live process.

- **Fit.** Drive the feature. Run `.cursor/skills/verify-graphite/scripts/drive.sh ellipsize`. Exit code 0. `target/verify-graphite/ellipsize/drive.txt` contains `feature ellipsize` and `unchanged input=short budget=8 result=short`.
- **Cut.** The same drive measures `abcdef` with a 4-cell budget. The evidence file contains `cut input=abcdef budget=4 result=abc…`.

## Gotchas

- The driver counts Unicode scalars, not font pixels. A different host metric changes the cut point.
- The ellipsis character is `…` (U+2026), not three ASCII periods.
- Exit code 0 without the input, the budget, and the result on those lines is not proof.
