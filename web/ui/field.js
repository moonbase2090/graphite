/** Build a labeled field and preserve the shared accessible-name contract. */
export function field({ id, label, control, help, error }) {
  if (!id || !control || !label) throw new TypeError("id, label, and control are required");
  const root = document.createElement("div");
  root.className = "gr-field";
  const labelElement = document.createElement("label");
  labelElement.htmlFor = id;
  labelElement.textContent = label;
  control.id = id;
  control.classList.add(control instanceof HTMLSelectElement ? "gr-select" : "gr-input");
  root.append(labelElement, control);
  const describedBy = [];
  if (help) {
    const helpElement = document.createElement("p");
    helpElement.id = `${id}-help`;
    helpElement.className = "gr-field-help";
    helpElement.textContent = help;
    root.append(helpElement);
    describedBy.push(helpElement.id);
  }
  if (error) {
    const errorElement = document.createElement("p");
    errorElement.id = `${id}-error`;
    errorElement.className = "gr-field-error";
    errorElement.textContent = error;
    root.append(errorElement);
    control.setAttribute("aria-invalid", "true");
    describedBy.push(errorElement.id);
  } else {
    control.removeAttribute("aria-invalid");
  }
  if (describedBy.length) control.setAttribute("aria-describedby", describedBy.join(" "));
  else control.removeAttribute("aria-describedby");
  return root;
}
