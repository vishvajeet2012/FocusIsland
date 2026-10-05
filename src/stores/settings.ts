import { writable } from "svelte/store";
import { DEFAULT_SETTINGS, type AppSettings } from "../types/settings";

export const settingsStore = writable<AppSettings>({ ...DEFAULT_SETTINGS });
