<script lang="ts">
  import { settingsStore } from "../../stores/settings";
  import { tasksStore, addTask, archiveCompleted, copyTask, editTask, moveTask, removeTask, toggleTask } from "../../stores/tasks";
  import { timerStore } from "../../stores/timer";
  import { addReminder } from "../../stores/reminders";
  import { showToast } from "../../stores/toasts";
  import { formatShortDate } from "../../lib/time";
  import type { Task, TaskInput } from "../../types/task";
  import Icon from "../common/Icon.svelte";
  import TaskInputComponent from "./TaskInput.svelte";
  import TaskRow from "./TaskRow.svelte";
  import TaskEditorModal from "./TaskEditorModal.svelte";
  import ReminderModal from "../events/ReminderModal.svelte";

  let filter: "todo" | "completed" = "todo";
  let editorTask: Task | null | undefined = undefined;
  let reminderTask: Task | null = null;
  let draggedId: number | null = null;
  let lastDragTargetId: number | null = null;
  let taskInput: TaskInputComponent;

  export function focusInput(): void { taskInput?.focus(); }

  $: pending = $tasksStore.filter((task) => !task.completed);
  $: completed = $tasksStore.filter((task) => task.completed);
  $: visibleTasks = filter === "todo" ? pending : completed;

  async function create(title: string): Promise<void> {
    try { await addTask({ title }); showToast("Task created", "success"); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't create task", "warning"); }
  }

  async function saveEditor(input: TaskInput): Promise<void> {
    try {
      if (editorTask) await editTask({ ...input, id: editorTask.id });
      else await addTask(input);
      showToast(editorTask ? "Task updated" : "Task created", "success");
      editorTask = undefined;
    } catch (error) { showToast(error instanceof Error ? error.message : "Couldn't save task", "warning"); }
  }

  async function startFocus(task: Task): Promise<void> {
    try {
      const minutes = task.estimatedFocusMinutes ?? $settingsStore.defaultTimerMinutes;
      await timerStore.start({ mode: "countdown", durationSeconds: minutes * 60, taskId: task.id, taskTitle: task.title });
      showToast(`Focusing on “${task.title}”`, "success");
    } catch (error) { showToast(error instanceof Error ? error.message : "Couldn't start focus", "warning"); }
  }

  async function scheduleReminder(event: CustomEvent<{ title: string; remindAt: number; taskId: number | null }>): Promise<void> {
    try {
      await addReminder(event.detail.title, event.detail.remindAt, event.detail.taskId);
      reminderTask = null;
      showToast("Reminder scheduled", "success");
    } catch (error) { showToast(error instanceof Error ? error.message : "Couldn't schedule reminder", "warning"); }
  }

  async function reorder(target: Task): Promise<void> {
    if (draggedId === null || draggedId === target.id || lastDragTargetId === target.id) return;
    lastDragTargetId = target.id;
    try { await moveTask(draggedId, target.id); }
    catch { showToast("Couldn't reorder tasks", "warning"); }
  }

  async function setCompleted(task: Task, completed: boolean): Promise<void> {
    try {
      await toggleTask(task.id, completed);
      showToast(completed ? "Task completed" : "Task restored", "success");
    } catch (error) { showToast(error instanceof Error ? error.message : "Couldn't update task", "warning"); }
  }

  async function deleteOne(task: Task): Promise<void> {
    try { await removeTask(task.id); showToast("Task deleted"); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't delete task", "warning"); }
  }

  async function duplicateOne(task: Task): Promise<void> {
    try { await copyTask(task.id); showToast("Task duplicated", "success"); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't duplicate task", "warning"); }
  }

  async function archiveAll(): Promise<void> {
    try {
      const count = await archiveCompleted();
      showToast(`${count} task${count === 1 ? "" : "s"} archived`);
    } catch (error) { showToast(error instanceof Error ? error.message : "Couldn't archive tasks", "warning"); }
  }
</script>

<section class="card task-card" aria-labelledby="tasks-heading">
  <header>
    <div><h2 id="tasks-heading">Today’s tasks</h2><span class="eyebrow">{formatShortDate()}</span></div>
    <button class="new-button" type="button" aria-label="Create detailed task" onclick={() => editorTask = null}><Icon name="plus" size={13} />New</button>
  </header>
  <div class="filters" aria-label="Task sections">
    <button class:active={filter === "todo"} type="button" onclick={() => filter = "todo"}>To do <b>{pending.length}</b></button>
    <button class:active={filter === "completed"} type="button" onclick={() => filter = "completed"}>Completed <b>{completed.length}</b></button>
    {#if completed.length > 0 && filter === "completed"}<button class="archive" type="button" title="Archive completed tasks" onclick={() => void archiveAll()}><Icon name="archive" size={12} /></button>{/if}
  </div>
  <div class="task-list" class:empty={visibleTasks.length === 0}>
    {#if visibleTasks.length === 0}
      <div class="empty-state">
        <span class="empty-icon"><Icon name={filter === "todo" ? "spark" : "check"} size={17} /></span>
        <p>{filter === "todo" ? "No tasks yet." : "Nothing completed yet."}</p>
        <small>{filter === "todo" ? "Add something you want to finish today." : "Small wins will appear here."}</small>
      </div>
    {:else}
      {#each visibleTasks as task (task.id)}
        <TaskRow
          {task}
          hour12={$settingsStore.timeFormat === "12"}
          dragging={draggedId === task.id}
          on:toggle={(event) => void setCompleted(event.detail.task, event.detail.completed)}
          on:edit={(event) => editorTask = event.detail}
          on:delete={(event) => void deleteOne(event.detail)}
          on:duplicate={(event) => void duplicateOne(event.detail)}
          on:reminder={(event) => reminderTask = event.detail}
          on:focus={(event) => startFocus(event.detail)}
          on:dragstart={(event) => { draggedId = event.detail.id; lastDragTargetId = null; }}
          on:dragover={(event) => reorder(event.detail)}
          on:dragend={() => { draggedId = null; lastDragTargetId = null; }}
        />
      {/each}
    {/if}
  </div>
  <TaskInputComponent bind:this={taskInput} on:add={(event) => create(event.detail)} />
</section>

{#if editorTask !== undefined}
  <TaskEditorModal task={editorTask} on:close={() => editorTask = undefined} on:save={(event) => saveEditor(event.detail)} />
{/if}
{#if reminderTask}
  <ReminderModal initialTitle={reminderTask.title} taskId={reminderTask.id} on:close={() => reminderTask = null} on:save={scheduleReminder} />
{/if}

<style>
  .task-card { color: #182019; background: var(--task-card); }
  header { display: flex; align-items: flex-start; justify-content: space-between; }
  header > div { display: flex; align-items: baseline; gap: 7px; }
  h2 { margin: 0; font-size: 13px; line-height: 1.2; letter-spacing: -.15px; }
  .eyebrow { color: rgba(24,32,25,.48); font-size: 9px; }
  .new-button { display: flex; align-items: center; gap: 3px; padding: 5px 7px; border: 0; border-radius: 7px; color: #28382c; background: rgba(255,255,255,.34); font: inherit; font-size: 9px; font-weight: 650; cursor: pointer; }
  .filters { display: flex; align-items: center; gap: 2px; width: fit-content; margin-top: 10px; padding: 2px; border-radius: 8px; background: rgba(29,43,32,.07); }
  .filters button { padding: 4px 8px; border: 0; border-radius: 6px; color: rgba(22,31,24,.51); background: transparent; font: inherit; font-size: 9px; cursor: pointer; }
  .filters button.active { color: #182019; background: rgba(255,255,255,.44); box-shadow: 0 1px 3px rgba(28,46,33,.07); }
  .filters b { margin-left: 3px; font-size: 9px; }
  .filters .archive { display: grid; place-items: center; padding: 4px 6px; }
  .task-list { min-height: 0; flex: 1; margin: 6px -3px 7px; overflow: auto; scrollbar-width: thin; scrollbar-color: rgba(26,39,29,.18) transparent; }
  .task-list.empty { display: grid; place-items: center; }
  .empty-state { display: grid; justify-items: center; max-width: 180px; text-align: center; }
  .empty-icon { display: grid; place-items: center; width: 30px; height: 30px; border-radius: 11px; color: rgba(31,52,37,.54); background: rgba(255,255,255,.26); }
  .empty-state p { margin: 7px 0 2px; font-size: 10px; font-weight: 650; }
  .empty-state small { color: rgba(24,32,25,.48); font-size: 9px; line-height: 1.35; }
</style>
