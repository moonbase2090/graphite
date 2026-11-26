const http = require("node:http");
const fs = require("node:fs");
const path = require("node:path");

const { chromium } = require(process.argv[2]);
const root = path.resolve(__dirname, "../../..");
const types = { ".css": "text/css", ".js": "text/javascript", ".html": "text/html" };
const server = http.createServer((request, response) => {
  const pathname = new URL(request.url, "http://localhost").pathname;
  const file = path.resolve(root, `.${pathname}`);
  if (!file.startsWith(`${root}${path.sep}`)) { response.writeHead(403).end(); return; }
  response.setHeader("content-type", types[path.extname(file)] || "text/plain");
  fs.createReadStream(file).on("error", () => response.writeHead(404).end()).pipe(response);
});

function valueFor(selector, role) {
  const escaped = selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const match = new RegExp(`${escaped}\\s*\\{[^}]*--gr-${role}:\\s*([^;]+);`).exec(tokens);
  if (!match) throw new Error(`No --gr-${role} for ${selector}`);
  return match[1].trim();
}

const tokens = fs.readFileSync(path.join(root, "web/tokens/graphite-tokens.css"), "utf8");
const catalog = fs.readFileSync(path.join(__dirname, "../theme-catalog.js"), "utf8");
const bootstrap = fs.readFileSync(path.join(__dirname, "../theme-bootstrap.js"), "utf8");
const catalogIds = [...catalog.matchAll(/\["([a-z0-9-]+)",\s*"[^"]+"\]/g)].map(([, id]) => id);
const bootstrapIds = /const themes = (\[[^;]+\]);/.exec(bootstrap)?.[1];
if (!bootstrapIds || JSON.stringify(catalogIds) !== JSON.stringify(JSON.parse(bootstrapIds))) {
  throw new Error("theme-bootstrap.js theme IDs must stay aligned with theme-catalog.js");
}

(async () => {
  await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
  const browser = await chromium.launch({ headless: true });
  try {
    const port = server.address().port;
    let failures = 0;
    for (const system of ["light", "dark"]) {
      for (const theme of [null, "graphite", "indigo", "forest-mint"]) {
        for (const scheme of [null, "light", "dark"]) {
          const page = await browser.newPage({ colorScheme: system });
          const attrs = `${theme ? ` data-theme="${theme}"` : ""}${scheme ? ` data-scheme="${scheme}"` : ""}`;
          await page.setContent(`<html${attrs}><head><style>${tokens}</style></head><body></body></html>`);
          const actual = (await page.evaluate(() => getComputedStyle(document.documentElement).getPropertyValue("--gr-surface"))).trim();
          const effective = scheme || system;
          const selector = theme
            ? `:root[data-theme="${theme}"]${effective === "light" ? "[data-scheme=\"light\"]" : ""}`
            : `:root:not([data-theme])${effective === "light" ? "[data-scheme=\"light\"]" : ""}`;
          const expected = valueFor(selector, "surface");
          if (actual !== expected) {
            console.error(`theme=${theme || "unset"}, scheme=${scheme || "system"}, system=${system}: ${actual} != ${expected}`);
            failures++;
          }
          await page.close();
        }
      }
    }

    const delayedGallery = await browser.newPage({ colorScheme: "dark" });
    await delayedGallery.addInitScript(() => {
      localStorage.setItem("graphite.gallery.theme", JSON.stringify({ theme: "forest-mint", scheme: "light" }));
      new PerformanceObserver(entries => {
        for (const entry of entries.getEntries()) {
          if (entry.name === "first-contentful-paint") {
            window.__firstPaintTheme = document.documentElement.dataset.theme;
            window.__firstPaintScheme = document.documentElement.dataset.scheme;
          }
        }
      }).observe({ type: "paint", buffered: true });
    });
    await delayedGallery.route("**/web/ui/theme-bootstrap.js", async route => {
      await new Promise(resolve => setTimeout(resolve, 500));
      await route.continue();
    });
    await delayedGallery.route("**/web/ui/gallery.js", async route => {
      await new Promise(resolve => setTimeout(resolve, 900));
      await route.continue();
    });
    await delayedGallery.goto(`http://127.0.0.1:${port}/web/ui/gallery.html`);
    const firstPaintTheme = await delayedGallery.evaluate(() => [window.__firstPaintTheme, window.__firstPaintScheme]);
    if (firstPaintTheme[0] !== "forest-mint" || firstPaintTheme[1] !== "light") {
      console.error("gallery first paint did not use saved theme", firstPaintTheme);
      failures++;
    }
    await delayedGallery.close();

    const page = await browser.newPage();
    await page.goto(`http://127.0.0.1:${port}/web/ui/README.md`);
    await page.addStyleTag({ content: tokens });
    await page.evaluate(async () => {
      const { field } = await import("/web/ui/field.js");
      const input = document.createElement("input");
      document.body.replaceChildren(field({ id: "email", label: "Email", control: input, help: "Enter a work address." }));
    });
    const session = await page.context().newCDPSession(page);
    const tree = async () => session.send("Accessibility.getFullAXTree").then(({ nodes }) => nodes.find(node => node.role?.value === "textbox"));
    let node = await tree();
    if (node.name?.value !== "Email" || node.description?.value !== "Enter a work address.") { console.error("default field AX mismatch", node.name?.value, node.description?.value); failures++; }
    await page.evaluate(async () => {
      const { field } = await import("/web/ui/field.js");
      const input = document.createElement("input");
      document.body.replaceChildren(field({ id: "email", label: "Email", control: input, help: "Enter a work address.", error: "Use a company address." }));
    });
    node = await tree();
    if (node.name?.value !== "Email" || node.description?.value !== "Enter a work address. Use a company address.") { console.error("invalid field AX mismatch", node.name?.value, node.description?.value); failures++; }

    const behavior = await page.evaluate(async () => {
      const { confirmDialog, icon, initializeTheme, roomList, sideRail, themeMenu } = await import("/web/ui/index.js");
      const store = new Map();
      const storage = { getItem: key => store.get(key) || null, setItem: (key, value) => store.set(key, value) };
      initializeTheme("legacy-theme", "indigo", { getItem: () => "forest-mint" });
      const legacyApplied = document.documentElement.dataset.theme === "forest-mint";
      const oldMenu = themeMenu({ storageKey: "old-key", storage: { getItem: () => "mono-sand" } });
      const oldValueSelected = oldMenu.querySelector('input[name="gr-theme"][value="mono-sand"]').checked;
      initializeTheme("invalid", "bogus", { getItem: () => JSON.stringify({ theme: "not-a-theme", scheme: "light" }) });
      const invalidChoiceFallback = document.documentElement.dataset.theme === "graphite" && document.documentElement.dataset.scheme === "light";
      const menu = themeMenu({ storageKey: "app.theme", storage });
      document.body.replaceChildren(menu);
      const radio = menu.querySelector('input[value="forest-mint"]');
      radio.checked = true;
      radio.dispatchEvent(new Event("change", { bubbles: true }));
      const dark = menu.querySelector('input[name="gr-scheme"][value="dark"]');
      dark.checked = true;
      dark.dispatchEvent(new Event("change", { bubbles: true }));
      menu.open = true;
      menu.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
      const rail = sideRail([{ id: "home", label: "Home", href: "/" }], "home");
      const rooms = roomList([{ id: "general", label: "General", href: "/general" }], "general");
      const svg = icon("rail-collapse");
      const dialog = confirmDialog("Continue?", () => { window.__confirmed = true; }, { confirmLabel: "Confirm" });
      return { theme: document.documentElement.dataset.theme, scheme: document.documentElement.dataset.scheme, saved: store.get("app.theme"), menuClosed: !menu.open,
        currentRail: rail.querySelector('[aria-current="page"]')?.textContent,
        currentRoom: rooms.querySelector('[aria-current="page"]')?.textContent,
        iconHidden: svg.getAttribute("aria-hidden"), dialogLabel: dialog.getAttribute("aria-labelledby"), legacyApplied, oldValueSelected, invalidChoiceFallback };
    });
    if (behavior.theme !== "forest-mint" || behavior.scheme !== "dark" || behavior.saved !== '{"theme":"forest-mint","scheme":"dark"}' || !behavior.menuClosed ||
        behavior.currentRail !== "Home" || behavior.currentRoom !== "General" || behavior.iconHidden !== "true" || !behavior.dialogLabel || !behavior.legacyApplied || !behavior.oldValueSelected || !behavior.invalidChoiceFallback) { console.error("component behavior mismatch", behavior); failures++; }
    await page.getByRole("button", { name: "Confirm" }).click();
    await page.locator("dialog").waitFor({ state: "detached" });
    const remainingDialogs = await page.locator("dialog").count();
    const confirmed = await page.evaluate(() => window.__confirmed);
    if (remainingDialogs !== 0 || !confirmed) { console.error("dialog did not confirm and close", { remainingDialogs, confirmed, state: await page.locator("dialog").evaluate(d => ({ open: d.open, value: d.returnValue, html: d.outerHTML })) }); failures++; }

    const shotDir = process.env.GRAPHITE_GALLERY_SHOTS;
    if (shotDir) fs.mkdirSync(shotDir, { recursive: true });
    for (const [scheme, theme] of [["dark", "graphite"], ["light", "forest-mint"]]) {
      const gallery = await browser.newPage({ colorScheme: scheme, viewport: { width: 1280, height: 900 } });
      const pageErrors = [];
      gallery.on("pageerror", error => pageErrors.push(error.message));
      await gallery.addInitScript(({ theme, scheme }) => localStorage.setItem("graphite.gallery.theme", JSON.stringify({ theme, scheme })), { theme, scheme });
      await gallery.goto(`http://127.0.0.1:${port}/web/ui/gallery.html`);
      await gallery.getByRole("heading", { name: "Shared controls" }).waitFor();
      if (pageErrors.length) { console.error("gallery page errors", pageErrors); failures++; }
      if (shotDir) await gallery.screenshot({ path: path.join(shotDir, `graphite-ui-${theme}-${scheme}.png`), fullPage: true });
      await gallery.close();
    }
    await browser.close();
    if (failures) process.exitCode = 1;
    else console.log("Theme matrix (24 cases), pre-paint bootstrap, and field AX name/description passed.");
  } finally {
    server.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; server.close(); });
