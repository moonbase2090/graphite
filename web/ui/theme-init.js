import { themeIds } from "./theme-catalog.js";

/** Apply a saved theme before paint; load this module from the document head. */
export function initializeTheme(storageKey, fallbackTheme, storage) {
  const root = document.documentElement;
  let saved = null;
  try { saved = (storage ?? globalThis.localStorage)?.getItem(storageKey); } catch { /* Use the shell defaults. */ }
  let choice;
  try { choice = saved ? JSON.parse(saved) : null; } catch { choice = saved; }
  const requestedTheme = typeof choice === "string" ? choice : choice?.theme;
  const defaultTheme = themeIds.includes(fallbackTheme) ? fallbackTheme : "graphite";
  root.dataset.theme = themeIds.includes(requestedTheme) ? requestedTheme : defaultTheme;
  if (choice?.scheme === "light" || choice?.scheme === "dark") root.dataset.scheme = choice.scheme;
  else delete root.dataset.scheme;
}
