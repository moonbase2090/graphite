# Changelog

## [Unreleased]

- No changes since 0.1.3.

## [0.1.3] - 2026-10-28

- Add motion duration tokens and CSS easing curves to the opt-in `motion` feature.

## [0.1.2] - 2026-10-28

- Add the opt-in `pixel-buffer` feature: CPU RGBA pixel-buffer primitives (#57).

## [0.1.1] - 2026-10-27

- Fix `ellipsize` so a narrow budget returns a lone ellipsis when it fits, and an empty string when even the ellipsis does not fit. Grapheme clusters stay whole.
- Document the opt-in features in the README feature table: `series`, `states`, `serde`, and `terminal-palette`.

## [0.1.0] - 2026-10-26

- Share renderer-independent Graphite tokens, theme derivation, chrome layouts, and text measurement seams.
- Add the `graphite-tui` crate: ratatui widgets for the tabs bar, side rail, sidebar, pane chrome, legend, and theme picker, colored only from `Tokens`.
- Add `DEFAULT_ACCENT` and document the `accent()` 3:1 contract against the bar.
- Add the opt-in `motion` feature: light-cycle easing and progress helpers.
- Add the opt-in `series` feature: six categorical chart colors in `Tokens`.
- Add the opt-in `states` feature: focus, selection, disabled, and error styles at truecolor, 256-color, and 16-color depth.
- Add the opt-in `serde` feature: read and write `Tokens` as TOML theme files.
- Add the opt-in `terminal-palette` feature: deterministic 256-color and ANSI-16 fallbacks for every token.
- Add a widget gallery example for `graphite-tui`, rendering every chrome widget in dark and light themes at truecolor, 256-color, and 16-color depth.
- Add the opt-in `motion` spring and frame-rate governor: a damped spring that settles within an eighth of a cell, bounded per-call work, reduced-motion snapping, and a 60-to-30 frames-per-second governor.
