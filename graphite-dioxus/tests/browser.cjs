const fs = require("node:fs");
const http = require("node:http");
const path = require("node:path");
const { chromium } = require("../../web/ui/node_modules/playwright");

const root = path.resolve(process.argv[2]);
const types = { ".css": "text/css", ".html": "text/html", ".js": "text/javascript", ".wasm": "application/wasm" };
const server = http.createServer((request, response) => {
  const pathname = new URL(request.url, "http://localhost").pathname;
  const file = path.resolve(root, `.${pathname}`);
  if (!file.startsWith(`${root}${path.sep}`)) { response.writeHead(403).end(); return; }
  response.setHeader("content-type", types[path.extname(file)] || "application/octet-stream");
  fs.createReadStream(file).on("error", () => response.writeHead(404).end()).pipe(response);
});

(async () => {
  await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    const errors = [];
    page.on("pageerror", error => errors.push(error.message));
    await page.goto(`http://127.0.0.1:${server.address().port}/browser-contract.html`);
    const dialog = page.locator("#browser-confirm");
    await page.getByRole("button", { name: "Open dialog" }).click();
    await page.waitForFunction(() => {
      const dialog = document.querySelector("#browser-confirm");
      return dialog?.open && dialog.matches(":modal");
    });
    await dialog.getByRole("button", { name: "Cancel" }).click();
    await page.waitForFunction(() => !document.querySelector("#browser-confirm")?.open);

    const details = page.locator("details.gr-theme-menu");
    await details.locator("summary").click();
    const radio = details.locator('input[name="gr-theme"][value="forest-mint"]');
    await radio.focus();
    await radio.press("Escape");
    await page.waitForFunction(() => {
      const details = document.querySelector("details.gr-theme-menu");
      return !details?.open && document.activeElement === details?.querySelector("summary");
    });
    if (errors.length) throw new Error(`Dioxus browser errors: ${errors.join("; ")}`);
    console.log("Dioxus WASM dialog reactivity and theme-menu Escape behavior passed.");
  } finally {
    await browser.close();
    server.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; server.close(); });
