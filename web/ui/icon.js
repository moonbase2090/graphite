const paths = Object.freeze({
  paperclip: ["M21.4 11.1 12.2 20.3a6 6 0 0 1-8.5-8.5l9.2-9.2a4 4 0 0 1 5.7 5.7l-9.2 9.2a2 2 0 0 1-2.8-2.8l8.5-8.5"],
  "rail-collapse": ["M4 4h16v16H4z", "M9 4v16", "M16 9l-3 3 3 3"],
  "rail-expand": ["M4 4h16v16H4z", "M9 4v16", "M13 9l3 3-3 3"],
});

/** Create a decorative inline SVG icon using DOM calls (CSP safe). */
export function icon(name) {
  const segments = paths[name];
  if (!segments) throw new TypeError(`Unknown Graphite icon: ${name}`);
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("aria-hidden", "true");
  svg.setAttribute("focusable", "false");
  svg.classList.add("gr-icon");
  for (const d of segments) {
    const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
    path.setAttribute("d", d);
    svg.append(path);
  }
  return svg;
}
