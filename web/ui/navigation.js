/** Render a compact rail from labeled navigation items. */
export function sideRail(items, currentId) {
  const nav = document.createElement("nav");
  nav.className = "gr-side-rail";
  nav.setAttribute("aria-label", "Primary");
  for (const item of items) {
    const link = document.createElement("a");
    link.className = "gr-rail-link";
    link.href = item.href;
    link.textContent = item.label;
    if (item.id === currentId) link.setAttribute("aria-current", "page");
    nav.append(link);
  }
  return nav;
}

/** Render accessible room navigation with an explicit current location. */
export function roomList(rooms, currentId, onSelect) {
  const list = document.createElement("ul");
  list.className = "gr-room-list";
  for (const room of rooms) {
    const row = document.createElement("li");
    const link = document.createElement("a");
    link.className = "gr-room-link";
    link.href = room.href || "#";
    link.textContent = room.label;
    if (room.id === currentId) link.setAttribute("aria-current", "page");
    if (onSelect) link.addEventListener("click", event => { event.preventDefault(); onSelect(room); });
    row.append(link);
    list.append(row);
  }
  return list;
}
