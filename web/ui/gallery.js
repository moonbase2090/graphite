import { button, confirmDialog, field, initializeTheme, roomList, sideRail, themeMenu } from "./index.js";

initializeTheme("graphite.gallery.theme", "graphite");
document.querySelector("#theme-menu").append(themeMenu({ storageKey: "graphite.gallery.theme", fallbackTheme: "graphite" }));

document.querySelector("#buttons").append(
  button("Primary", () => {}, "primary"),
  button("Ghost", () => {}, "ghost"),
  button("Danger", () => confirmDialog("Delete this item?", () => {}, { confirmLabel: "Delete" }), "danger"),
);

const email = document.createElement("input");
email.type = "email";
document.querySelector("#fields").append(
  field({ id: "email", label: "Email", control: email, help: "Enter a work address." }),
  field({ id: "invalid", label: "Work email", control: document.createElement("input"), help: "Use your company address.", error: "This address is not allowed." }),
);

document.querySelector("#rail").append(sideRail([
  { id: "overview", label: "Overview", href: "#overview" },
  { id: "members", label: "Members", href: "#members" },
], "overview"));
document.querySelector("#rooms").append(roomList([
  { id: "general", label: "# general", href: "#general" },
  { id: "design", label: "# design", href: "#design" },
], "general"));
