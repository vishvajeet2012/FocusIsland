export const PRODUCT_NAME = "FocusIsland";
export const BRAND_ICON_PATH = "/app-icon.svg";
export const PRODUCT_TAGLINE = "Your day, one glance away.";

export const WINDOW_DIMENSIONS = {
  collapsed: { width: 228, height: 52 },
  compact: { width: 202, height: 46 },
  large: { width: 254, height: 58 },
  expanded: { width: 948, height: 420 },
  quick: { width: 430, height: 64 },
} as const;

export const SHORTCUTS = {
  toggleWorkspace: "Ctrl + Alt + Space",
  quickTask: "Ctrl + Alt + T",
  toggleTimer: "Ctrl + Alt + P",
} as const;

export const NOTE_SAVE_DEBOUNCE_MS = 450;
export const SETTINGS_SAVE_DEBOUNCE_MS = 250;
