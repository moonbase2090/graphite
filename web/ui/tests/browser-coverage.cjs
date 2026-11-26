const fs = require("node:fs");
const path = require("node:path");
const { pathToFileURL } = require("node:url");

/**
 * Record Chromium JS coverage for every page the browser opens and, when
 * NODE_V8_COVERAGE is set (c8 sets it), write the entries for files under
 * `root` there as V8 coverage, so c8 reports the browser-run modules.
 */
function trackCoverage(browser, root) {
  const dir = process.env.NODE_V8_COVERAGE;
  if (!dir) return async () => {};
  const open = new Set();
  const entries = [];
  const collect = async page => {
    if (!open.delete(page)) return;
    entries.push(...await page.coverage.stopJSCoverage());
  };
  const newPage = browser.newPage.bind(browser);
  browser.newPage = async (...args) => {
    const page = await newPage(...args);
    await page.coverage.startJSCoverage({ resetOnNavigation: false });
    open.add(page);
    const close = page.close.bind(page);
    page.close = async (...closeArgs) => { await collect(page); return close(...closeArgs); };
    return page;
  };
  return async () => {
    for (const page of [...open]) await collect(page);
    const result = [];
    for (const { scriptId, url, functions } of entries) {
      if (!url.startsWith("http://127.0.0.1:")) continue;
      const file = path.resolve(root, `.${new URL(url).pathname}`);
      if (!file.startsWith(`${root}${path.sep}`)) continue;
      result.push({ scriptId, url: pathToFileURL(file).href, functions });
    }
    fs.mkdirSync(dir, { recursive: true });
    fs.writeFileSync(path.join(dir, `browser-${process.pid}.json`), JSON.stringify({ result }));
  };
}

module.exports = { trackCoverage };
