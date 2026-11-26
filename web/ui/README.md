# Graphite UI

Unbundled ES modules and CSS built on Graphite's `--gr-` tokens. Import
`tokens.css` and `components.css` from the app stylesheet. Add the external
`theme-bootstrap.js` as a blocking script in `<head>` before stylesheets, with
`data-storage-key` and `data-fallback-theme`, so saved choices apply before
first paint under a CSP that disallows inline scripts. Keep markup in light DOM
and build elements with DOM APIs; this package must work with strict
`style-src 'self'` and Trusted Types policies.

## Markup contracts

- Buttons use `<button class="gr-button" data-variant="primary|ghost|danger">`.
- Fields use `.gr-field`, a `<label for>` containing only label text, a native
  control, and help/error siblings. `aria-describedby` lists visible text IDs.
- The theme menu exposes theme and color-scheme choices. The head bootstrap is
  an external classic script so the browser waits for it before first paint.
  Each app supplies its own storage key; older string-only saved themes remain
  readable.

The Dioxus package must emit the same elements, classes, data attributes, and
ARIA relationships.

## Gallery

The browser fixture at `gallery.html` exercises the shared component set. The
reviewed screenshots show the Graphite dark default and Forest Mint light:

![Graphite dark gallery](../../docs/assets/web-ui/graphite-ui-graphite-dark.png)

![Forest Mint light gallery](../../docs/assets/web-ui/graphite-ui-forest-mint-light.png)
