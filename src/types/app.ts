import type { AppSettings } from "./settings";
import type { Task } from "./task";
import type { TimerSnapshot } from "./timer";

export interface DailyNote {
  id: number;
  date: string;
  content: string;
  createdAt: number;
  updatedAt: number;
}

export interface Reminder {
  id: number;
  taskId: number | null;
  title: string;
  remindAt: number;
  completed: boolean;
  createdAt: number;
}

export interface DayMetric {
  date: string;
  label: string;
  focusMinutes: number;
  completedTasks: number;
}

export interface Insights {
  todayFocusSeconds: number;
  tasksCompletedToday: number;
  sessionsToday: number;
  currentStreak: number;
  mostProductiveDay: string | null;
  averageSessionSeconds: number;
  weeklyCompletedTasks: number;
  weeklyFocusMinutes: number;
  days: DayMetric[];
}

export interface BootstrapPayload {
  tasks: Task[];
  note: DailyNote;
  reminders: Reminder[];
  timer: TimerSnapshot;
  settings: AppSettings;
  shortcutWarnings: string[];
}

export type WorkspaceTab = "workspace" | "insights" | "customize";
export type WindowMode = "collapsed" | "expanded" | "quick";
export type MotionPhase = "idle" | "opening" | "closing" | "quick-opening" | "quick-closing";
