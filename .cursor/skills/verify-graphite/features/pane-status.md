# Pane status

Pane status picks the most urgent header state and measures the width of that status label. Attention outranks mail. A quiet pane reserves no status width.

## Sub-features

- `status-decide` prefers attention over mail, and mail over quieter states.
- `status-width` measures `needs you` at a 12px font and scale 1000, and returns 0 for quiet.

## How to get to it (user POV)

- A host reports whether a pane needs attention, has mail, has unseen output, is running, or is focused.
- The host asks for the status label width before placing the pane title.

## Driving it with verify-graphite

Preconditions:

- `doctor.sh` has printed `package=graphite-core` for this checkout.
- `target/verify-graphite/driver.pid` does not name a live process.

- **Decide.** Run `.cursor/skills/verify-graphite/scripts/drive.sh pane-status`. Exit code 0. `target/verify-graphite/pane-status/drive.txt` contains `decide-attention Attention` and `decide-mail Mail(4)`.
- **Width.** The same file contains `attention-width 108` and `quiet-width 0`.

## Gotchas

- This drive measures width as scalar count times font pixel size. A one-cell metric would report a different attention width.
- Scale is 1000 (1x). A 2x scale does not produce 108.
- Quiet is the absence of a label, so its width is 0 rather than the width of the word `quiet`.
