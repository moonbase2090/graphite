import { button } from "./button.js";

let dialogSequence = 0;

/** Open an accessible native confirmation dialog and remove it after close. */
export function confirmDialog(message, onConfirm, { confirmLabel = "OK", cancelLabel = "Cancel" } = {}) {
  const dialog = document.createElement("dialog");
  dialog.className = "gr-dialog";
  const text = document.createElement("p");
  text.textContent = message;
  text.id = `gr-confirm-${++dialogSequence}`;
  dialog.setAttribute("aria-labelledby", text.id);
  const actions = document.createElement("div");
  actions.className = "gr-dialog-actions";
  const cancel = button(cancelLabel, () => dialog.close("cancel"), "ghost");
  const confirm = button(confirmLabel, () => dialog.close("confirm"), "primary");
  actions.append(cancel, confirm);
  dialog.append(text, actions);
  dialog.addEventListener("close", () => {
    dialog.remove();
    if (dialog.returnValue === "confirm") onConfirm();
  }, { once: true });
  document.body.append(dialog);
  dialog.showModal();
  cancel.focus();
  return dialog;
}
