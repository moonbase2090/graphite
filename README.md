# Graphite

Graphite is the shared design layer for Moonbase terminal tools: one set of
design tokens, the geometry of the window chrome, and the pure algorithms that
turn a host theme into readable colors. It has no renderer of its own.

| Crate | What it is |
| --- | --- |
| `graphite-core` (this package) | Tokens, color math, theme derivation, and pixel-space layouts for the tabs bar, side rail, sidebar, pane header, legend, and theme picker. |
| `graphite-tui` (`graphite-tui/`) | Ratatui widgets that draw those layouts on a terminal cell grid, colored only from tokens. |

Both crates require Rust 1.85 and are not published to crates.io yet.

## Token model

`Tokens` is a plain struct of sRGB colors (`[u8; 3]`) with one field per role,
such as `bar`, `tab_active`, `text`, `muted`, `attention`, and `hairline`.
Two sets ship with the crate: `DARK` and `LIGHT`. Consumers never hardcode a
color; they read a role from a `Tokens` value.

For a host theme that is not one of the brief themes, `derive_tokens` builds a
token set from a `ThemeSource` (chrome and default colors, ANSI palette, and
variant) and nudges text until it stays readable. `theme_tokens` returns the
brief tokens verbatim for brief themes and derives the rest. `accent` adjusts a
focus color until it has 3:1 contrast against the bar.

## Feature flags

| Flag | Default | Adds |
| --- | --- | --- |
| `motion` | off | Light-cycle easing and progress helpers (`MotionCurve`, `light_cycle_traced_samples`). |

## Quick start

Add the crate by path or git, then read colors from tokens and measure contrast:

```rust
use graphite_core::{contrast_ratio, tokens, ThemeVariant};

let t = tokens(ThemeVariant::Dark);
assert!(contrast_ratio(t.text, t.bar) >= 4.5);
```

Layouts take a `TextMetrics` implementation so the host chooses the font. This
one measures every character as one unit:

```rust
use graphite_core::{ellipsize, Face, TextMetrics};

struct OneCell;

impl TextMetrics for OneCell {
    fn width(&self, _face: Face, _px: f32, text: &str) -> f32 {
        text.chars().count() as f32
    }
}

assert_eq!(ellipsize(&OneCell, Face::Regular, 12.0, "graphite", 5.0), "grap…");
```

For terminals, use `graphite-tui`: build a layout with `tabs_bar_layout`,
`legend_layout`, or `picker_layout`, then render a widget such as `TabsBar`
into a ratatui buffer. `cargo run -p graphite-tui --example preview` prints
every widget as true-color ANSI.

## Contributing

See [CONTRIBUTING.md](https://github.com/moonbase2090/graphite/blob/main/CONTRIBUTING.md) for the workflow and
[REVIEW_POLICY.md](https://github.com/moonbase2090/graphite/blob/main/REVIEW_POLICY.md) for what needs review before merge.

## License

See the LICENSE file in the repository root.
