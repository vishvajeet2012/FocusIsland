import { get, writable } from "svelte/store";
import * as native from "../lib/native";
import type { Task, TaskInput, TaskUpdate } from "../types/task";

export const tasksStore = writable<Task[]>([]);

function byTaskOrder(a: Task, b: Task): number {
  if (a.completed !== b.completed) return Number(a.completed) - Number(b.completed);
  return a.sortOrder - b.sortOrder || a.createdAt - b.createdAt;
}

export function setTasks(tasks: Task[]): void {
  tasksStore.set([...tasks].sort(byTaskOrder));
}

export async function addTask(input: TaskInput): Promise<Task> {
  const task = await native.createTask(input);
  tasksStore.update((tasks) => [...tasks, task].sort(byTaskOrder));
  return task;
}

export async function editTask(input: TaskUpdate): Promise<Task> {
  const updated = await native.updateTask(input);
  tasksStore.update((tasks) => tasks.map((task) => task.id === updated.id ? updated : task).sort(byTaskOrder));
  return updated;
}

export async function toggleTask(id: number, completed: boolean): Promise<Task> {
  const updated = await native.setTaskCompleted(id, completed);
  tasksStore.update((tasks) => tasks.map((task) => task.id === id ? updated : task).sort(byTaskOrder));
  return updated;
}

export async function removeTask(id: number): Promise<void> {
  await native.deleteTask(id);
  tasksStore.update((tasks) => tasks.filter((task) => task.id !== id));
}

export async function copyTask(id: number): Promise<Task> {
  const duplicate = await native.duplicateTask(id);
  tasksStore.update((tasks) => [...tasks, duplicate].sort(byTaskOrder));
  return duplicate;
}

export async function moveTask(draggedId: number, targetId: number): Promise<void> {
  const tasks = get(tasksStore);
  const incomplete = tasks.filter((task) => !task.completed);
  const from = incomplete.findIndex((task) => task.id === draggedId);
  const to = incomplete.findIndex((task) => task.id === targetId);
  if (from < 0 || to < 0 || from === to) return;
  const next = [...incomplete];
  const [moved] = next.splice(from, 1);
  next.splice(to, 0, moved);
  const reordered = next.map((task, index) => ({ ...task, sortOrder: index }));
  const complete = tasks.filter((task) => task.completed);
  tasksStore.set([...reordered, ...complete].sort(byTaskOrder));
  try {
    await native.reorderTasks(reordered.map((task) => task.id));
  } catch (error) {
    tasksStore.set(tasks);
    throw error;
  }
}

export async function archiveCompleted(): Promise<number> {
  const count = await native.archiveCompletedTasks();
  tasksStore.update((tasks) => tasks.filter((task) => !task.completed));
  return count;
}
