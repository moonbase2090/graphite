/* Keep this blocking head script dependency-free so CSP can allow it by src. */
(() => {
  const script = document.currentScript;
  const storageKey = script?.dataset.storageKey || "graphite.theme";
  const fallback = script?.dataset.fallbackTheme || "indigo";
  const themes = ["graphite", "indigo", "mono-slate", "forest-mint", "mono-sand", "analog-azure", "midnight-indigo", "comp-coral"];
  let saved = null;
  try {
    saved = localStorage.getItem(storageKey);
  } catch { /* Use the shell defaults. */ }
  let choice = null;
  try { choice = saved ? JSON.parse(saved) : null; } catch { choice = saved; }
  const requestedTheme = typeof choice === "string" ? choice : choice?.theme;
  document.documentElement.dataset.theme = themes.includes(requestedTheme)
    ? requestedTheme
    : themes.includes(fallback) ? fallback : "graphite";
  if (choice?.scheme === "light" || choice?.scheme === "dark") {
    document.documentElement.dataset.scheme = choice.scheme;
  } else {
    delete document.documentElement.dataset.scheme;
  }
})();
