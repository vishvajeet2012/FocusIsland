import { writable } from "svelte/store";
import * as native from "../lib/native";
import type { Reminder } from "../types/app";

export const remindersStore = writable<Reminder[]>([]);

function ordered(reminders: Reminder[]): Reminder[] {
  return [...reminders].sort((a, b) => a.remindAt - b.remindAt);
}

export function setReminders(reminders: Reminder[]): void {
  remindersStore.set(ordered(reminders));
}

export async function addReminder(title: string, remindAt: number, taskId: number | null): Promise<Reminder> {
  const reminder = await native.createReminder(title, remindAt, taskId);
  remindersStore.update((items) => ordered([...items, reminder]));
  return reminder;
}

export async function removeReminder(id: number): Promise<void> {
  await native.deleteReminder(id);
  remindersStore.update((items) => items.filter((item) => item.id !== id));
}
