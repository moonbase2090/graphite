# Migrating a terminal app to Graphite

This guide is for terminal apps that keep their own theme and chrome code and
want to read colors and geometry from Graphite instead. It covers the steps in
order. Where a step needs an opt-in feature, the feature is named.

## 1. Add the crates

```toml
[dependencies]
graphite-core = { git = "https://github.com/moonbase2090/graphite" }
graphite-tui = { git = "https://github.com/moonbase2090/graphite" }   # ratatui widgets
```

Both require Rust 1.85. `graphite-tui` uses ratatui 0.29. `graphite-core` has no
renderer, so an app on another renderer can use it alone.

Opt-in features of `graphite-core`, all off by default:

| Feature | Adds |
| --- | --- |
| `motion` | Durations, easing curves, spring and crossfade helpers. |
| `series` | `Tokens::series`, six chart colors per palette. |
| `states` | `state_style` for focus, selection, disabled, error, hover. |
| `serde` | `Tokens::to_toml` / `from_toml` for theme files. |
| `cell-design` | Cell spacing scale, border glyph sets, sub-cell glyph order. |
| `terminal-palette` | 256-color and 16-color fallback tables. |

## 2. Map your colors to token roles

Replace each named color in your theme with a `Tokens` field. Never copy hex
values; read the field so a theme switch changes everything at once.

| Your role | Token |
| --- | --- |
| window background | `ground` |
| card or pane surface | `panel` |
| hover surface | `title_hover` |
| border | `hairline` |
| title-row rule | `title_line` |
| hover outline | `hover_outline` |
| body text | `text` |
| strong text | `text_strong` |
| muted text | `muted` |
| idle or disabled dot | `idle` |
| focus fill | `title_focus` |
| focus line | `title_focus_line` |
| muted text on focus fill | `muted_focus` |
| success | `working` |
| warning | `unseen` |
| danger | `attention` (text on it: `on_attention`) |
| animated head | `cycle_head` |

Chrome colors (`bar`, `tab_active`, `field`, `key`, `status_bar`, and the rest)
are named in `Tokens`; see the rustdoc.

## 3. Choose a token set

- Built-in dark or light: `tokens(ThemeVariant::Dark)` or `DARK` / `LIGHT`.
- A host theme you already have: fill a `ThemeSource` (chrome and default
  colors, the 16 ANSI colors, and the badge colors) and call `theme_tokens`. It
  returns the built-in tokens unchanged for the brief themes and derives the
  rest. Text fields in `Tokens` are preferred inks. Resolve the ink separately
  for each actual fill with `text_ink(preferred, background)`, including dynamic
  row fills and interpolated fills during transitions. One stored ink cannot
  reach 4.5:1 on every independently conflicting surface (for example black
  chrome, white fields, and a mid-gray active tab). `text_ink` returns an
  already-passing preferred color unchanged; otherwise it picks the closest
  passing color along the black and white RGB-mix paths, by squared channel
  distance. The guarantee is for the exact sRGB pair before terminal palette
  quantization, transparency, blending, or renderer overrides.
- A user theme file: enable `serde` and call `Tokens::from_toml`. Errors name
  the key and the bad value.

## 4. Pick the focus accent

Graphite has no fixed accent per theme. Pass a focus color, usually
`DEFAULT_ACCENT` or the theme's ANSI blue, through `accent(&tokens, focus)`.
It is returned unchanged when it already has 3:1 contrast against the bar, and
otherwise moved toward black or white until it does.

## 5. Handle color depth

Detect the terminal's depth once at startup and resolve every token to a
palette then, not per cell. With `terminal-palette`, `terminal_palette` gives
the 256-color index and the ANSI-16 slot for each token. In 16-color mode emit
the ANSI slots so the user's palette picks the hues. With `states`, focus and
selection use bold and reverse video at 16 colors instead of fills.

## 6. Use the widgets or the layouts

`graphite-tui` draws the tabs bar, side rail, sidebar, pane chrome, legend, and
theme picker. It maps Graphite's pixel layouts onto 8 by 16 pixel cells and
takes colors only from `Tokens`. Rendering allocates nothing, so build layouts
and labels outside the frame loop and keep them until the data changes.

An app with its own renderer can call the `graphite-core` layout functions
(`bar_layout`, `sidebar_layout`, `legend_keys`, and others) with its own
`TextMetrics` and draw the rectangles itself.

## 7. Motion

With `motion`: use the 80, 160, and 240 ms durations and never exceed the
300 ms cap, ease out for entering and ease in for exiting, and fade between
themes with `crossfade_tokens`. Read any reduced-motion setting in your app and
snap to the end state; Graphite does not read the environment.

## 8. Check your migration

- No hex literals or raw `Color::Rgb` calls remain outside the one place that
  converts a token to your renderer's color type.
- Use `text_ink` with the final fill for every text draw, then check that pair for 4.5:1; check focus and outline edges for 3:1.
- Switching theme does not rebuild widgets, only the resolved palette.
- Rendering a frame with nothing changed performs no allocation.
