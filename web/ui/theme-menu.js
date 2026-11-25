import { themes, themeIds } from "./theme-catalog.js";

function savedTheme(storage, key, fallback) {
  try {
    const value = storage?.getItem(key);
    if (!value) return fallback;
    try { return JSON.parse(value).theme || fallback; } catch { return value; }
  } catch { return fallback; }
}

/** Build a theme chooser that follows system light/dark unless scheme is set. */
export function themeMenu({ storageKey, fallbackTheme = "indigo", storage } = {}) {
  if (!storageKey) throw new TypeError("storageKey is required");
  let storageArea = storage;
  if (storageArea === undefined) {
    try { storageArea = globalThis.localStorage; } catch { storageArea = null; }
  }
  const menu = document.createElement("details");
  menu.className = "gr-menu gr-theme-menu";
  const summary = document.createElement("summary");
  summary.className = "gr-theme-menu-button";
  const chip = document.createElement("span");
  chip.className = "gr-theme-chip";
  chip.setAttribute("aria-hidden", "true");
  const label = document.createElement("span");
  label.textContent = "Theme";
  summary.append(chip, label);
  const panel = document.createElement("fieldset");
  panel.className = "gr-theme-panel";
  const legend = document.createElement("legend");
  legend.textContent = "Theme";
  panel.append(legend);
  const defaultTheme = themeIds.includes(fallbackTheme) ? fallbackTheme : "graphite";
  const saved = savedTheme(storageArea, storageKey, defaultTheme);
  let currentTheme = themeIds.includes(saved) ? saved : defaultTheme;
  let currentScheme = null;
  try {
    const stored = JSON.parse(storageArea?.getItem(storageKey) || "null");
    if (stored?.scheme === "light" || stored?.scheme === "dark") currentScheme = stored.scheme;
  } catch { /* Old theme-only values follow the system scheme. */ }
  const save = () => {
    try { storageArea?.setItem(storageKey, JSON.stringify({ theme: currentTheme, scheme: currentScheme })); } catch { /* Session-only choice. */ }
  };
  for (const [id, name] of themes) {
    const option = document.createElement("label");
    option.className = "gr-theme-option";
    const radio = document.createElement("input");
    radio.type = "radio";
    radio.name = "gr-theme";
    radio.value = id;
    radio.checked = id === currentTheme;
    radio.addEventListener("change", () => {
      currentTheme = id;
      document.documentElement.dataset.theme = id;
      save();
    });
    const swatch = document.createElement("span");
    swatch.className = "gr-theme-swatch";
    swatch.dataset.swatch = id;
    swatch.setAttribute("aria-hidden", "true");
    const text = document.createElement("span");
    text.textContent = id === defaultTheme ? `${name} (default)` : name;
    option.append(radio, swatch, text);
    panel.append(option);
  }
  const schemes = document.createElement("fieldset");
  schemes.className = "gr-theme-schemes";
  const schemeLegend = document.createElement("legend");
  schemeLegend.textContent = "Color scheme";
  schemes.append(schemeLegend);
  for (const [id, label] of [["system", "System"], ["light", "Light"], ["dark", "Dark"]]) {
    const option = document.createElement("label");
    option.className = "gr-theme-option";
    const radio = document.createElement("input");
    radio.type = "radio";
    radio.name = "gr-scheme";
    radio.value = id;
    radio.checked = id === (currentScheme || "system");
    radio.addEventListener("change", () => {
      currentScheme = id === "system" ? null : id;
      if (currentScheme) document.documentElement.dataset.scheme = currentScheme;
      else delete document.documentElement.dataset.scheme;
      save();
    });
    option.append(radio, document.createTextNode(label));
    schemes.append(option);
  }
  panel.append(schemes);
  const note = document.createElement("p");
  note.className = "gr-theme-note";
  note.textContent = "Light and dark follow your device.";
  panel.append(note);
  menu.append(summary, panel);
  const closeOnOutsideClick = event => {
    if (!menu.isConnected) document.removeEventListener("click", closeOnOutsideClick);
    else if (menu.open && !menu.contains(event.target)) menu.open = false;
  };
  document.addEventListener("click", closeOnOutsideClick);
  menu.addEventListener("keydown", event => {
    if (event.key === "Escape" && menu.open) { menu.open = false; summary.focus(); }
  });
  return menu;
}
