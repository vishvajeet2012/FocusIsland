import type { TimerDisplayValue, TimerSnapshot } from "../types/timer";

export function todayKey(date = new Date()): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

export function timerValue(snapshot: TimerSnapshot, now = Date.now()): TimerDisplayValue {
  if (snapshot.status === "ready") {
    return {
      totalSeconds: snapshot.mode === "countdown" ? snapshot.targetDurationSeconds ?? 1500 : 0,
      finished: false,
    };
  }

  const end = snapshot.status === "paused" ? snapshot.pausedAt ?? now : now;
  const elapsedMs = snapshot.startedAt === null
    ? 0
    : Math.max(0, end - snapshot.startedAt - snapshot.accumulatedPauseMs);
  const elapsedSeconds = Math.floor(elapsedMs / 1000);

  if (snapshot.mode === "stopwatch") {
    return { totalSeconds: elapsedSeconds, finished: false };
  }

  const remaining = Math.max(0, (snapshot.targetDurationSeconds ?? 0) - elapsedSeconds);
  return {
    totalSeconds: snapshot.status === "completed" ? 0 : remaining,
    finished: remaining === 0 && snapshot.status === "running",
  };
}

export function formatTimer(totalSeconds: number, includeHours = false): string {
  const safe = Math.max(0, Math.floor(totalSeconds));
  const hours = Math.floor(safe / 3600);
  const minutes = Math.floor((safe % 3600) / 60);
  const seconds = safe % 60;
  if (includeHours || hours > 0) {
    return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
  }
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}

export function formatDuration(totalSeconds: number): string {
  const minutes = Math.round(totalSeconds / 60);
  if (minutes < 60) return `${minutes}m`;
  const hours = Math.floor(minutes / 60);
  const remainder = minutes % 60;
  return remainder > 0 ? `${hours}h ${remainder}m` : `${hours}h`;
}

export function formatShortDate(timestamp = Date.now()): string {
  return new Intl.DateTimeFormat(undefined, {
    weekday: "short",
    day: "numeric",
    month: "short",
  }).format(timestamp);
}

export function formatReminderTime(timestamp: number, hour12: boolean): string {
  const date = new Date(timestamp);
  const day = todayKey(date);
  const today = todayKey();
  const tomorrowDate = new Date();
  tomorrowDate.setDate(tomorrowDate.getDate() + 1);
  const prefix = day === today ? "Today" : day === todayKey(tomorrowDate) ? "Tomorrow" :
    new Intl.DateTimeFormat(undefined, { day: "numeric", month: "short" }).format(date);
  const time = new Intl.DateTimeFormat(undefined, {
    hour: "numeric",
    minute: "2-digit",
    hour12,
  }).format(date);
  return `${prefix} at ${time}`;
}

export function toLocalDateTimeInput(timestamp: number): string {
  const date = new Date(timestamp - new Date(timestamp).getTimezoneOffset() * 60_000);
  return date.toISOString().slice(0, 16);
}
