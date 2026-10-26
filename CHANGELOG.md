# Changelog

## [Unreleased]

- No changes since 0.1.0.

## [0.1.0] - 2026-10-26

- Share renderer-independent Graphite tokens, theme derivation, chrome layouts, and text measurement seams.
- Add the `graphite-tui` crate: ratatui widgets for the tabs bar, side rail, sidebar, pane chrome, legend, and theme picker, colored only from `Tokens`.
- Add `DEFAULT_ACCENT` and document the `accent()` 3:1 contract against the bar.
- Add the opt-in `motion` feature: light-cycle easing and progress helpers.
- Add the opt-in `series` feature: six categorical chart colors in `Tokens`.
- Add the opt-in `states` feature: focus, selection, disabled, and error styles at truecolor, 256-color, and 16-color depth.
- Add the opt-in `serde` feature: read and write `Tokens` as TOML theme files.
- Add the opt-in `terminal-palette` feature: deterministic 256-color and ANSI-16 fallbacks for every token.
