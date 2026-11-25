/** Create a native Graphite button without HTML string parsing. */
export function button(text, action, variant = "primary") {
  if (!["primary", "ghost", "danger"].includes(variant)) {
    throw new TypeError(`Unknown button variant: ${variant}`);
  }
  const element = document.createElement("button");
  element.type = "button";
  element.className = "gr-button";
  element.dataset.variant = variant;
  element.textContent = text;
  if (action) element.addEventListener("click", action);
  return element;
}
