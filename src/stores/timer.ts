import { writable } from "svelte/store";
import { timerValue } from "../lib/time";
import type { TimerSnapshot, TimerStartInput } from "../types/timer";
import * as native from "../lib/native";

export interface TimerStoreValue {
  snapshot: TimerSnapshot;
  displaySeconds: number;
}

const initialSnapshot: TimerSnapshot = {
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

function createTimerStore() {
  const store = writable<TimerStoreValue>({ snapshot: initialSnapshot, displaySeconds: 1500 });
  let current = initialSnapshot;
  let tickHandle: ReturnType<typeof setTimeout> | null = null;

  function stopTicking(): void {
    if (tickHandle !== null) clearTimeout(tickHandle);
    tickHandle = null;
  }

  function updateDisplay(): void {
    const display = timerValue(current);
    store.set({ snapshot: current, displaySeconds: display.totalSeconds });
    if (current.status === "running") {
      const delay = 1000 - (Date.now() % 1000) + 12;
      tickHandle = setTimeout(updateDisplay, delay);
    }
  }

  function apply(snapshot: TimerSnapshot): void {
    stopTicking();
    current = snapshot;
    updateDisplay();
  }

  return {
    subscribe: store.subscribe,
    setSnapshot: apply,
    refresh: updateDisplay,
    async start(input: TimerStartInput): Promise<TimerSnapshot> {
      const result = await native.startTimer(input);
      apply(result);
      return result;
    },
    async pause(): Promise<TimerSnapshot> {
      const result = await native.pauseTimer();
      apply(result);
      return result;
    },
    async resume(): Promise<TimerSnapshot> {
      const result = await native.resumeTimer();
      apply(result);
      return result;
    },
    async finish(completed = true): Promise<TimerSnapshot> {
      const result = await native.finishTimer(completed);
      apply(result);
      return result;
    },
    async reset(mode: "countdown" | "stopwatch", durationSeconds: number): Promise<TimerSnapshot> {
      const result = await native.resetTimer(mode, durationSeconds);
      apply(result);
      return result;
    },
    destroy: stopTicking,
  };
}

export const timerStore = createTimerStore();
