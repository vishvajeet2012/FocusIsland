export type ThemeMode = "system" | "dark" | "light";
export type IslandSize = "compact" | "normal" | "large";
export type AnimationIntensity = "full" | "reduced" | "off";
export type MonitorMode = "active" | "primary";
export type TimeFormat = "12" | "24";

export interface AppSettings {
  theme: ThemeMode;
  workspaceBackground: string;
  taskCardColor: string;
  timerCardColor: string;
  notesCardColor: string;
  eventsCardColor: string;
  islandSize: IslandSize;
  expandOnClick: boolean;
  expandOnHover: boolean;
  hoverDelayMs: 100 | 200 | 300 | 500;
  alwaysOnTop: boolean;
  launchOnStartup: boolean;
  showTrayIcon: boolean;
  animationIntensity: AnimationIntensity;
  notificationsEnabled: boolean;
  notificationSounds: boolean;
  defaultTimerMinutes: number;
  timeFormat: TimeFormat;
  collapseOnFocusLoss: boolean;
  movable: boolean;
  monitorMode: MonitorMode;
  topGap: 0 | 6 | 12;
  firstRun: boolean;
}

export const DEFAULT_SETTINGS: AppSettings = {
  theme: "system",
  workspaceBackground: "#08090a",
  taskCardColor: "#c9dcc5",
  timerCardColor: "#d8d0ec",
  notesCardColor: "#eadcae",
  eventsCardColor: "#c8ddeb",
  islandSize: "normal",
  expandOnClick: true,
  expandOnHover: false,
  hoverDelayMs: 300,
  alwaysOnTop: true,
  launchOnStartup: false,
  showTrayIcon: true,
  animationIntensity: "full",
  notificationsEnabled: true,
  notificationSounds: false,
  defaultTimerMinutes: 25,
  timeFormat: "12",
  collapseOnFocusLoss: true,
  movable: false,
  monitorMode: "active",
  topGap: 6,
  firstRun: true,
};
