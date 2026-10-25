# Theme tokens

Theme tokens hand a host the dark or light Graphite palette. Text on the dark ground stays at least WCAG AA contrast, and black on white is the reference ratio of 21.

## Sub-features

- `tokens-dark` returns the dark ground color.
- `tokens-light` returns the light primary text color.
- `tokens-contrast` reports black-on-white and dark text-on-ground contrast.

## How to get to it (user POV)

- A host asks for the dark palette or the light palette before painting chrome.
- The host reads the ground and text bytes, then the contrast of text against ground.

## Driving it with verify-graphite

Preconditions:

- `doctor.sh` has printed `package=graphite-core` for this checkout.
- `target/verify-graphite/driver.pid` does not name a live process.

- **Dark ground.** Run `.cursor/skills/verify-graphite/scripts/drive.sh theme-tokens`. Exit code 0. `target/verify-graphite/theme-tokens/drive.txt` contains `dark-ground 16 18 22` and `dark-variant Dark`.
- **Light text.** The same file contains `light-text 31 35 41` and `light-variant Light`.
- **Contrast.** The same file contains `contrast-black-white 21.0000` and a `contrast-text-ground` line whose number is at least 4.5.

## Gotchas

- Bytes are decimal, matching the sRGB values `#101216` and `#1f2329`.
- Contrast is printed to four decimal places. Do not require more precision than the evidence line shows.
- This drive does not open a window. The proof is the evidence file.
