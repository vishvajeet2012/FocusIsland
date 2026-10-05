import { writable } from "svelte/store";

export type ToastTone = "neutral" | "success" | "warning";

export interface ToastMessage {
  id: number;
  text: string;
  tone: ToastTone;
}

export const toastsStore = writable<ToastMessage[]>([]);
let nextId = 1;

export function showToast(text: string, tone: ToastTone = "neutral", duration = 2400): void {
  const id = nextId++;
  toastsStore.update((items) => [...items, { id, text, tone }]);
  setTimeout(() => dismissToast(id), duration);
}

export function dismissToast(id: number): void {
  toastsStore.update((items) => items.filter((item) => item.id !== id));
}
