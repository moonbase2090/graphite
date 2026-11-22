# Graphite UI for the web: design

Status: proposal. This document plans a shared web component library built on
Graphite tokens. It adds no code. Each step in the adoption plan is its own pull
request.

## Goal

Moonbase web apps should share one look and one set of accessible controls,
with `graphite-core` as the single source of their colors, as it already is for
the terminal chrome.

The library follows the stacks the apps already use rather than adding a third:

| App | Stack today | What it gets |
| --- | --- | --- |
| Comms hosted client | Vanilla JS ES modules built with esbuild; plain CSS custom properties; a strict CSP (`style-src 'self'`, Trusted Types) | Token sheet, component CSS, ES modules |
| Waypoint web app | Dioxus 0.6 compiled to WASM with Trunk; CSS custom properties | Token sheet, component CSS, a Dioxus component crate |

Scorecard, the project sites and other plain HTML pages use the same CSS and ES
modules later. There is no React package.

## Starting point

The library does not start from a blank page. The Comms redesign, which is in
review now, already exports web tokens from `graphite-core`:

- A small Rust tool pinned to a `graphite-core` revision generates the app's
  `tokens.css`.
- A role table maps `Tokens` plus `accent` and `text_ink` to web roles:
  `--ground`, `--surface`, `--surface-raised`, `--surface-sunken`, `--line`,
  `--line-strong`, `--ink`, `--ink-muted`, `--ink-faint`, `--accent`,
  `--accent-ink`, `--on-accent`, `--verified`, `--caution`, `--fault`, two
  tints, a shadow and a backdrop.
- Each named theme (`mono-slate`, `forest-mint`, `mono-sand` and others) is a
  seed run through `theme_tokens`, in a dark and a light block.
- On light themes the solid accent fill is anchored to `--accent-ink`, so white
  button text clears 4.5:1 without visibly shifting the hue.
- Tests fail when `tokens.css` is stale against the pinned revision, or when an
  exported theme misses its contrast floors.

Graphite UI moves that tool upstream, unchanged in substance, after the
redesign merges. The role names, the mapping and the seeds stay as the
redesign defines them. The redesign is not changed to suit the library.

## Layers

```
graphite-core          tokens, theme_tokens, accent, text_ink, state_style, motion, scale
      │
graphite-css           role table + theme catalog → web/tokens/graphite-tokens.css
      │
web/tokens/graphite-tokens.css    committed, staleness- and contrast-tested
      │
web/ui/components.css             shared component CSS, hand-written, tokens only
      │
      ├── web/ui  (@moonbase2090/graphite-ui)   ES modules for behavior
      └── graphite-dioxus                       Dioxus components
```

Both packages render the same markup with the same class names over the same
CSS, so a fix in `components.css` reaches every app at its next pin.

## Theming and token export

### Generator: `graphite-css`

A new crate in this repository, holding the Comms token tool's role table and
theme seeds:

```rust
pub fn stylesheet() -> String          // the whole token sheet
pub fn themes() -> &'static [ThemeSeed]  // the theme catalog
pub fn palette(theme: &str, variant: ThemeVariant) -> Option<WebPalette>
```

A binary writes `stylesheet()` to `web/tokens/graphite-tokens.css`, which is
committed. The crate's tests assert that the committed file equals
`stylesheet()` and check the contrast floors below. `scripts/check-terminal-palettes.sh`
already does the same for `docs/terminal-palettes.md`. The generator is its own crate so
`graphite-core`'s public API and dependencies do not change.

### Names

Every web role keeps its Comms name, with a `--gr-` prefix: `--gr-surface`,
`--gr-ink`, `--gr-accent` and so on. The prefix is needed because Waypoint's
stylesheet already defines `--accent`, `--text`, `--border` and others with
different meanings, and Waypoint moves over one screen at a time while both
sets are live. For Comms, the port is a mechanical rename.

Besides colors, the sheet exports:

- `--gr-motion-micro` (80ms), `--gr-motion-standard` (160ms) and
  `--gr-motion-emphasis` (240ms), from the `motion` constants.
- `--gr-radius`, `--gr-gap` and `--gr-pad` from `scale.rs`.
- The theme picker swatches (`--gr-swatch-<theme>-a/b`).

Spacing steps, type steps and font stacks are not in core. They come from the
Comms CSS as it is (`--space-*`, `--step-*`, `--font-*`) and live in
`components.css` with the prefix.

### Theme catalog and selection

One block per theme and scheme. The theme catalog is the union of the theme
IDs that Comms and Waypoint already share (`forest-mint`, `mono-sand`,
`midnight-indigo` and others). Waypoint's extra themes join the catalog as
seeds when Waypoint adopts the library. Graphite's own `DARK` and `LIGHT` are
the theme `graphite`.

Every page names its theme. The shared sheet never infers an app's default
from a missing attribute, because a missing attribute cannot be resolved the
same way under every scheme selector. The rules are:

- `data-theme` on `<html>` picks the theme, and it is always set before first
  paint. Each app writes its default into its HTML shell
  (`<html data-theme="indigo">` for Comms, `data-theme="mono-sand"` for
  Waypoint). `theme-init.js` then swaps in a saved choice before paint. Comms
  today removes the attribute for its Indigo default. In the port, Indigo
  becomes the catalog theme `indigo` and the attribute stays.
- A page with no `data-theme` gets the `graphite` theme under every scheme.
  This is a fallback, not a way to select an app default.
- `data-scheme="light|dark"` forces a scheme. Without it the page follows the
  system. Comms follows the system only today and keeps doing so. Waypoint
  already sets `data-scheme`.
- The sheet sets `color-scheme` in each block, so native controls and
  scrollbars match.

The generated selectors, for each theme:

```css
:root[data-theme="forest-mint"] { /* dark */ }
@media (prefers-color-scheme: light) {
  :root[data-theme="forest-mint"]:not([data-scheme="dark"]) { /* light */ }
}
:root[data-theme="forest-mint"][data-scheme="light"] { /* light */ }
```

The `graphite` fallback uses the same three selectors with `:root` in place of
`:root[data-theme="…"]`. Each themed selector has one more attribute than the
matching fallback selector. So in all four combinations (system light, system
dark, explicit light, explicit dark), the themed rule that applies outranks
every fallback rule that applies. The contract tests check this in a
browser, reading the computed `--gr-surface` for each combination:
- `graphite`, a catalog theme, and an unset `data-theme`;
- an unset, light and dark `data-scheme`;
- emulated light and dark system schemes.

Bar presets (`BarColor`) and Waypoint's medium tone are not part of the first
version.

## Components

### Contract

Each component is defined once as an HTML contract (element, classes, `data-`
attributes, ARIA) in `web/ui/README.md`. Both packages implement that contract.

- Native elements first: `<button>`, `<input>`, `<select>`, `<details>`,
  `<dialog>` with `showModal()`.
- Light DOM, no shadow DOM, so app CSS, Dioxus event handlers and form
  behavior keep working.
- No inline styles and no `innerHTML`. DOM is built with `createElement`,
  which Comms's CSP requires.

Starting list:

| Component | Markup | Behavior |
| --- | --- | --- |
| Button | `<button class="gr-button" data-variant="primary\|ghost\|danger">` | Native |
| Field | `<div class="gr-field">` holding a `<label for>` with the label text only, the control, and help and error text as siblings outside the label (see below) | Native |
| Input, textarea, select | `.gr-input`, `.gr-select` | Native |
| List | `<ul class="gr-list">`; for selection, `role="listbox"` with `role="option"` rows; for navigation, links marked with `aria-current` | Arrow keys, Home/End, typeahead |
| Tabs | `role="tablist"`, `role="tab"`, `role="tabpanel"` | Roving `tabindex`, arrow keys |
| Panel, card | `<section class="gr-panel">` | None |
| Dialog, confirm | `<dialog class="gr-dialog">` | Native focus trap and Esc; focus returns to the opener |
| Menu | `<details class="gr-menu">` | Esc closes, focus returns |
| Toast | `<div class="gr-toasts" role="status" aria-live="polite">` | Queue, timeout paused on hover and focus; errors use `role="alert"` |
| Theme switcher | Radio group in a menu, with swatches | Sets `data-theme` and `data-scheme`, saves the choice |
| Icon | Inline SVG built with DOM calls, `aria-hidden` unless labeled | None |
| Layout | `.gr-stack`, `.gr-row`, `.gr-bar`, `.gr-split` | None |

The first port takes exactly what the Comms redesign ships: its buttons,
fields, confirm dialog, theme menu, icons, rail and room list. Tabs, toasts and
the listbox are added when an app first needs them. This table is a starting
list, not a commitment to build everything before anyone uses it.

### Field markup

The label holds only the label text, so the accessible name is the label and
nothing else. Help and error text sit outside it and are linked by ID:

```html
<div class="gr-field">
  <label for="email">Email</label>
  <input id="email" class="gr-input" aria-describedby="email-help">
  <p id="email-help" class="gr-field-help">Enter a work address.</p>
  <p id="email-error" class="gr-field-error" hidden></p>
</div>
```

When the value is invalid, the control gets `aria-invalid="true"`, the error
element is shown with its text, and `aria-describedby` lists both IDs
(`email-help email-error`). When the value is valid again, the error ID is
removed and the element hidden. A field with no help text omits that element
and its ID.

Both packages test the accessible name and description in a browser, through
the accessibility tree, in the default and invalid states. The expected
results:
- Default: name "Email", description "Enter a work address."
- Invalid: name "Email", description of the help text followed by the error
  text.

### JS package: `web/ui` (`@moonbase2090/graphite-ui`)

- `graphite-tokens.css` (copied from `web/tokens/`) and `components.css`.
- One ES module per behavior, each a plain function over an element, in the
  style of the Comms view helpers: `button(text, action, variant)`,
  `field(label, control)`, `confirmDialog(opts)`, `themeMenu(opts)`,
  `bindTabs(el)`, `toast(message, opts)`.
- `theme-init.js`, loaded from a file (no inline script) before first paint,
  applies the saved theme so a stored choice never flashes. Each app passes its
  own storage key, so Comms keeps `comms.theme`.
- Shipped as unbundled ES modules and CSS with no build step inside the
  package. The app's esbuild bundles the JS, and its build inlines the CSS
  `@import` lines, as Comms already does for `tokens.css`.

### Dioxus crate: `graphite-dioxus`

- Components with typed props that render the contract markup, such as
  `Button { variant: Variant::Primary, onclick, "Save" }`, `Field`,
  `Dialog { open }`, `ThemeSwitcher` and `Toasts`.
- Behavior is in Rust, with signals and event handlers. Native calls such as
  `showModal()` go through `web-sys`, which replaces Waypoint's separate
  dialog focus script.
- `STYLESHEET: &str` holds the token sheet plus `components.css`, embedded with
  `include_str!` from the same revision as the crate.
- The crate does not depend on the JS package.

## Packaging and versioning

- Everything lives in this repository and is versioned with it. A graphite
  release tag pins the tokens, the CSS and both packages together.
- `graphite-css` is a workspace member on Rust 1.85, like `graphite-tui`.
- `graphite-dioxus` is excluded from the workspace and builds in its own CI
  job, as `graphite-lunatui` does, with the Rust version Waypoint uses. This
  keeps Dioxus and its dependency tree out of the core build, the `cargo deny`
  run and the 1.85 pin.
- Waypoint depends on `graphite-dioxus` by a git tag or rev pin, never a
  sibling path.
- The JS package is released as an npm tarball attached to the graphite GitHub
  release. Comms installs it by URL, and its lockfile records the integrity
  hash. npm cannot install a package from a subdirectory of a git repository.

### Staleness

The rule everywhere: the CSS an app serves is a committed file, and a test
fails if it differs from what the pinned graphite revision generates.

- In graphite: `graphite-css` checks `web/tokens/graphite-tokens.css`, and a
  `web/ui` test checks its copy of the sheet.
- In Comms: the CSS comes from the pinned package. The existing check against
  its pinned revision moves with the tool.
- In Waypoint: Trunk serves `assets/graphite.css` through a normal
  `<link data-trunk rel="css">`. A test asserts the file equals
  `graphite_dioxus::STYLESHEET`, and a small binary rewrites it after a pin
  bump. A served file keeps the CSS cacheable and works under a strict CSP.

## Accessibility

- **Contrast is tested at generation, for every theme and scheme:**
  - `--gr-ink`, `--gr-ink-muted`, `--gr-accent-ink`, `--gr-verified`,
    `--gr-caution` and `--gr-fault` reach 4.5:1 on `--gr-surface`.
  - `--gr-on-accent` reaches 4.5:1 on `--gr-accent`.
  - `--gr-accent` reaches 3:1 on the bar.

  These are the Comms tool's checks. Graphite UI adds:
  - The accent at 3:1 against ground, surface and raised surface too, since a
    web focus ring sits on all of them. Core guarantees only the bar.
  - Ink on `--gr-surface-raised`.
  - A 3:1 control boundary (WCAG 1.4.11) for the color used on input and
    button borders.

  The last check fails today: on the light default (Graphite `LIGHT` tokens),
  `--line-strong` (`separator`) is 2.19:1 on the white surface. The fix is a derived control
  edge, mixed from the line color toward ink until it reaches 3:1. It is
  handled in the token-sheet PR with before and after screenshots, not by
  changing the redesign.
- **Focus**: `:focus-visible` gets a 2px `--gr-accent` outline with a 2px
  offset, as Comms has now. It is never removed without a replacement.
- **State is never shown by color alone.** Following `StateStyle::marker`,
  selected and current rows also get a left bar, invalid fields get error
  text, and status dots have a text label.
- **Keyboard**: each component follows its WAI-ARIA Authoring Practices
  pattern and is tested with the keyboard only.
- **Reduced motion**: under `prefers-reduced-motion: reduce`, the motion
  properties become `0ms` and no transform animation runs. No duration exceeds
  `MOTION_MAX` (300ms).
- **Forced colors**: under `forced-colors: active`, components use system
  colors and keep borders and focus visible.
- **Targets**: interactive controls are at least 24 by 24 CSS pixels
  (WCAG 2.5.8). On touch layouts they follow the Comms `--target` size.
- An automated axe check runs on the gallery page in CI. It adds to the checks
  above and does not replace them.

## Testing and proof

- **`graphite-css`**: staleness and contrast tests, run for every theme and
  scheme.
- **Contract tests**: one hand-written HTML fixture per component. Dioxus
  renders each component with `dioxus-ssr`, and the JS package renders it in a
  headless browser. Both outputs are compared to the fixture after whitespace
  is normalized. Fixtures are written and reviewed by hand, never regenerated
  from output.
- **Theme precedence**: in Chromium, the computed `--gr-surface` is checked
  for every combination of the following:
  - theme: unset, `graphite`, and a catalog theme;
  - `data-scheme`: unset, light and dark;
  - system scheme: emulated light and dark.

  Each result must be that theme's value in the effective scheme.
- **Accessible names**: both packages check each control's accessible name and
  description through the browser's accessibility tree, in the default and
  invalid states.
- **Behavior tests**: Playwright drives a gallery page for each package with
  keyboard and pointer.
- **Visual proof**: the gallery is rendered in every theme and scheme and
  captured as real browser screenshots. Every visual PR shows before and after
  screenshots from real output, and the owner sees them before it merges.

## Adoption order

1. **This document.**
2. **Phase 1: Comms.** After the Comms redesign merges:
   1. Move its token tool into `graphite-css`, with the same output under
      `--gr-` names. Add the extra contrast checks and the control edge.
   2. Port its components into `web/ui` as they are.
   3. Switch Comms to the package. The before and after screenshots should
      differ only where the control-edge fix applies.
3. **Phase 2: Waypoint.** Build `graphite-dioxus` on the same contract and
   CSS. Add Waypoint's other themes to the catalog as seeds, then move its
   controls onto the crate one screen at a time. Theme colors will shift where
   Waypoint's hand-written themes differ from their derived versions. Each
   screen goes to the owner with before and after screenshots.
4. **Phase 3: the rest of the fleet.** Scorecard, the sites and other web pages
   take the JS package or just the token sheet.

Everything ships on by default. An app that takes a new pin gets the new look
with no flag. Each visual step is its own PR, with real screenshots seen by the
owner first and a review from a different model family before merge.

## Open questions for the owner

1. **Default theme.** Should every app keep its own default (Indigo for Comms,
   `mono-sand` for Waypoint), or should the fleet share one?
2. **Theme count.** Waypoint has 20 themes and Comms 7. The catalog could take
   all 20, or the fleet could settle on a smaller shared set during phase 2.
3. **Package home.** A tarball on the GitHub release is the simplest pinned
   install. A registry is possible later if more apps need it.
