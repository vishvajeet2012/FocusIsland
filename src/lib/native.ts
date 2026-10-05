import { invoke } from "@tauri-apps/api/core";
import type { BootstrapPayload, DailyNote, Insights, Reminder, WindowMode } from "../types/app";
import type { AppSettings } from "../types/settings";
import type { Task, TaskInput, TaskUpdate } from "../types/task";
import type { TimerSnapshot, TimerStartInput } from "../types/timer";
import { DEFAULT_SETTINGS } from "../types/settings";
import { todayKey } from "./time";

export function runningInTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

const fallbackTimer: TimerSnapshot = {
  mode: "countdown",
  status: "ready",
  startedAt: null,
  pausedAt: null,
  accumulatedPauseMs: 0,
  targetDurationSeconds: 1500,
  taskId: null,
  taskTitle: null,
  sessionStartedAt: null,
};

const fallbackNote: DailyNote = {
  id: 0,
  date: todayKey(),
  content: "",
  createdAt: Date.now(),
  updatedAt: Date.now(),
};

let browserTasks: Task[] = [];
let browserReminders: Reminder[] = [];
let browserTimer = fallbackTimer;

async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args);
}

export async function bootstrap(): Promise<BootstrapPayload> {
  if (runningInTauri()) return call<BootstrapPayload>("bootstrap_app", { date: todayKey() });
  return {
    tasks: browserTasks,
    note: fallbackNote,
    reminders: browserReminders,
    timer: browserTimer,
    settings: DEFAULT_SETTINGS,
    shortcutWarnings: [],
  };
}

export async function createTask(input: TaskInput): Promise<Task> {
  if (runningInTauri()) return call<Task>("create_task", { input });
  const now = Date.now();
  const task: Task = {
    id: now,
    title: input.title.trim(),
    notes: input.notes ?? null,
    completed: false,
    createdAt: now,
    updatedAt: now,
    dueAt: input.dueAt ?? null,
    reminderAt: null,
    estimatedFocusMinutes: input.estimatedFocusMinutes ?? null,
    sortOrder: browserTasks.length,
    archivedAt: null,
  };
  browserTasks = [...browserTasks, task];
  return task;
}

export async function updateTask(input: TaskUpdate): Promise<Task> {
  if (runningInTauri()) return call<Task>("update_task", { input });
  const current = browserTasks.find((task) => task.id === input.id);
  if (!current) throw new Error("Task not found");
  const updated = { ...current, ...input, updatedAt: Date.now() };
  browserTasks = browserTasks.map((task) => task.id === input.id ? updated : task);
  return updated;
}

export async function setTaskCompleted(id: number, completed: boolean): Promise<Task> {
  if (runningInTauri()) return call<Task>("set_task_completed", { id, completed });
  const task = browserTasks.find((item) => item.id === id);
  if (!task) throw new Error("Task not found");
  const updated = { ...task, completed, updatedAt: Date.now() };
  browserTasks = browserTasks.map((item) => item.id === id ? updated : item);
  return updated;
}

export async function deleteTask(id: number): Promise<void> {
  if (runningInTauri()) return call<void>("delete_task", { id });
  browserTasks = browserTasks.filter((task) => task.id !== id);
}

export async function duplicateTask(id: number): Promise<Task> {
  if (runningInTauri()) return call<Task>("duplicate_task", { id });
  const source = browserTasks.find((task) => task.id === id);
  if (!source) throw new Error("Task not found");
  return createTask({ title: `${source.title} copy`, notes: source.notes });
}

export async function reorderTasks(ids: number[]): Promise<void> {
  if (runningInTauri()) return call<void>("reorder_tasks", { ids });
  const positions = new Map(ids.map((id, index) => [id, index]));
  browserTasks = browserTasks
    .map((task) => ({ ...task, sortOrder: positions.get(task.id) ?? task.sortOrder }))
    .sort((a, b) => a.sortOrder - b.sortOrder);
}

export async function archiveCompletedTasks(): Promise<number> {
  if (runningInTauri()) return call<number>("archive_completed_tasks");
  const count = browserTasks.filter((task) => task.completed).length;
  browserTasks = browserTasks.filter((task) => !task.completed);
  return count;
}

export async function saveDailyNote(date: string, content: string): Promise<DailyNote> {
  if (runningInTauri()) return call<DailyNote>("save_daily_note", { date, content });
  return { ...fallbackNote, date, content, updatedAt: Date.now() };
}

export async function createReminder(title: string, remindAt: number, taskId: number | null): Promise<Reminder> {
  if (runningInTauri()) return call<Reminder>("create_reminder", { input: { title, remindAt, taskId } });
  const reminder: Reminder = { id: Date.now(), taskId, title, remindAt, completed: false, createdAt: Date.now() };
  browserReminders = [...browserReminders, reminder];
  return reminder;
}

export async function deleteReminder(id: number): Promise<void> {
  if (runningInTauri()) return call<void>("delete_reminder", { id });
  browserReminders = browserReminders.filter((reminder) => reminder.id !== id);
}

export async function startTimer(input: TimerStartInput): Promise<TimerSnapshot> {
  if (runningInTauri()) return call<TimerSnapshot>("start_timer", { input });
  browserTimer = {
    mode: input.mode,
    status: "running",
    startedAt: Date.now(),
    pausedAt: null,
    accumulatedPauseMs: 0,
    targetDurationSeconds: input.mode === "countdown" ? input.durationSeconds ?? 1500 : null,
    taskId: input.taskId ?? null,
    taskTitle: input.taskTitle ?? null,
    sessionStartedAt: Date.now(),
  };
  return browserTimer;
}

export async function pauseTimer(): Promise<TimerSnapshot> {
  if (runningInTauri()) return call<TimerSnapshot>("pause_timer");
  browserTimer = { ...browserTimer, status: "paused", pausedAt: Date.now() };
  return browserTimer;
}

export async function resumeTimer(): Promise<TimerSnapshot> {
  if (runningInTauri()) return call<TimerSnapshot>("resume_timer");
  const now = Date.now();
  browserTimer = {
    ...browserTimer,
    status: "running",
    accumulatedPauseMs: browserTimer.accumulatedPauseMs + (now - (browserTimer.pausedAt ?? now)),
    pausedAt: null,
  };
  return browserTimer;
}

export async function finishTimer(completed = true): Promise<TimerSnapshot> {
  if (runningInTauri()) return call<TimerSnapshot>("finish_timer", { completed });
  browserTimer = { ...browserTimer, status: "completed", pausedAt: Date.now() };
  return browserTimer;
}

export async function resetTimer(mode: "countdown" | "stopwatch", durationSeconds: number): Promise<TimerSnapshot> {
  if (runningInTauri()) return call<TimerSnapshot>("reset_timer", { mode, durationSeconds });
  browserTimer = { ...fallbackTimer, mode, targetDurationSeconds: mode === "countdown" ? durationSeconds : null };
  return browserTimer;
}

export async function saveSettings(settings: AppSettings): Promise<AppSettings> {
  if (runningInTauri()) return call<AppSettings>("save_settings", { settings });
  return settings;
}

export async function getInsights(): Promise<Insights> {
  if (runningInTauri()) return call<Insights>("get_insights", { date: todayKey() });
  const now = new Date();
  const days = Array.from({ length: 7 }, (_, index) => {
    const day = new Date(now);
    day.setDate(now.getDate() - 6 + index);
    return { date: todayKey(day), label: new Intl.DateTimeFormat(undefined, { weekday: "short" }).format(day), focusMinutes: 0, completedTasks: 0 };
  });
  return { todayFocusSeconds: 0, tasksCompletedToday: browserTasks.filter((task) => task.completed).length, sessionsToday: 0, currentStreak: 0, mostProductiveDay: null, averageSessionSeconds: 0, weeklyCompletedTasks: 0, weeklyFocusMinutes: 0, days };
}

export async function setWindowMode(mode: WindowMode, settings: AppSettings): Promise<void> {
  if (!runningInTauri()) return;
  return call<void>("set_window_mode", { mode, monitorMode: settings.monitorMode, topGap: settings.topGap, islandSize: settings.islandSize });
}

export async function showMainWindow(): Promise<void> {
  if (!runningInTauri()) return;
  return call<void>("show_main_window");
}

export async function beginWindowDrag(): Promise<void> {
  if (!runningInTauri()) return;
  return call<void>("begin_window_drag");
}
