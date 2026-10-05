import { writable } from "svelte/store";
import type { WindowMode, WorkspaceTab } from "../types/app";

export interface AppState {
  windowMode: WindowMode;
  activeTab: WorkspaceTab;
  loading: boolean;
  backendAvailable: boolean;
  error: string | null;
  interactionLocks: number;
  timerHighlighted: boolean;
  shortcutWarnings: string[];
}

export const appState = writable<AppState>({
  windowMode: "collapsed",
  activeTab: "workspace",
  loading: true,
  backendAvailable: true,
  error: null,
  interactionLocks: 0,
  timerHighlighted: false,
  shortcutWarnings: [],
});

export function setActiveTab(activeTab: WorkspaceTab): void {
  appState.update((state) => ({ ...state, activeTab }));
}

export function lockCollapse(): void {
  appState.update((state) => ({ ...state, interactionLocks: state.interactionLocks + 1 }));
}

export function unlockCollapse(): void {
  appState.update((state) => ({ ...state, interactionLocks: Math.max(0, state.interactionLocks - 1) }));
}

export function setAppError(message: string | null): void {
  appState.update((state) => ({ ...state, error: message }));
}
