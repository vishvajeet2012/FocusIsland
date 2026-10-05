export type TimerMode = "countdown" | "stopwatch";
export type TimerStatus = "ready" | "running" | "paused" | "completed";

export interface TimerSnapshot {
  mode: TimerMode;
  status: TimerStatus;
  startedAt: number | null;
  pausedAt: number | null;
  accumulatedPauseMs: number;
  targetDurationSeconds: number | null;
  taskId: number | null;
  taskTitle: string | null;
  sessionStartedAt: number | null;
}

export interface TimerStartInput {
  mode: TimerMode;
  durationSeconds?: number | null;
  taskId?: number | null;
  taskTitle?: string | null;
}

export interface TimerDisplayValue {
  totalSeconds: number;
  finished: boolean;
}
