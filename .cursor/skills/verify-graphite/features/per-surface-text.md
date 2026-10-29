# Per-surface text

`text_ink` resolves one preferred ink against the exact fill used by a text draw. Conflicting fills can require different inks; the truecolor result reaches 4.5:1 before terminal palette quantization or later blending.

## Sub-features

- `text-ink` preserves already-passing preferred colors and finds the closest passing result on the black and white RGB-mix paths.
- `conflicting-fills` resolves the same preferred ink independently against black, white, and mid-gray.

## How to get to it (user POV)

- A renderer selects a role color from `Tokens` and knows the final fill for its text surface.
- The renderer calls `text_ink(preferred, background)` for that pair. It repeats the call for each independent or dynamic fill.

## Driving it with verify-graphite

Preconditions:

- `doctor.sh` has printed `package=graphite-core` for this checkout.
- `target/verify-graphite/driver.pid` does not name a live process.

- **Conflicting fills.** Run `.cursor/skills/verify-graphite/scripts/drive.sh per-surface-text`. Exit code 0. `target/verify-graphite/per-surface-text/drive.txt` contains `black`, `white`, and `mid-gray` lines with their inputs and resolved RGB values. Each `contrast=` value is at least `4.5000`.
- **Independent results.** The drive asserts the black and white surfaces resolve to different inks for the same preferred input.

## Gotchas

- The returned fallback is the nearest passing candidate along the two quantized RGB interpolation paths by squared channel distance; it is not a global search of all RGB values or a perceptual distance.
- This guarantee is for the exact sRGB RGB pair. Terminal palette quantization, transparency, blending, and renderer overrides can change contrast.
