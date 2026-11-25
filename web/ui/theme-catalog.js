/** Comms dashboard theme IDs preserved by the Graphite token export. */
export const themes = Object.freeze([
  ["graphite", "Graphite"], ["indigo", "Indigo"], ["mono-slate", "Slate"], ["forest-mint", "Forest Mint"],
  ["mono-sand", "Sand"], ["analog-azure", "Azure Tide"],
  ["midnight-indigo", "Midnight Indigo"], ["comp-coral", "Mint Coral"],
]);

export const themeIds = Object.freeze(["graphite", ...themes.map(([id]) => id)]);
