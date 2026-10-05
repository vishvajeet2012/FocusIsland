<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import type { Task, TaskInput } from "../../types/task";
  import Modal from "../common/Modal.svelte";
  import { toLocalDateTimeInput } from "../../lib/time";

  export let task: Task | null = null;
  const dispatch = createEventDispatcher<{ close: void; save: TaskInput }>();

  let title = task?.title ?? "";
  let notes = task?.notes ?? "";
  let dueAt = task?.dueAt ? toLocalDateTimeInput(task.dueAt) : "";
  let estimatedMinutes = task?.estimatedFocusMinutes ?? 25;
  let error = "";
  let titleInput: HTMLInputElement;

  onMount(() => setTimeout(() => titleInput?.focus(), 0));

  function submit(event: SubmitEvent): void {
    event.preventDefault();
    const clean = title.trim();
    if (!clean) { error = "Give the task a short title."; return; }
    dispatch("save", {
      title: clean,
      notes: notes.trim() || null,
      dueAt: dueAt ? new Date(dueAt).getTime() : null,
      estimatedFocusMinutes: estimatedMinutes,
    });
  }
</script>

<Modal title={task ? "Edit task" : "New task"} description="Keep it concrete and finishable." on:close={() => dispatch("close")}>
  <form onsubmit={submit}>
    <label>Task title<input bind:this={titleInput} bind:value={title} maxlength="240" placeholder="What needs doing?" /></label>
    <label>Notes<textarea bind:value={notes} maxlength="4000" rows="3" placeholder="Optional details"></textarea></label>
    <div class="row">
      <label>Due<input type="datetime-local" bind:value={dueAt} /></label>
      <label>Focus estimate<input type="number" min="1" max="180" bind:value={estimatedMinutes} /><span>minutes</span></label>
    </div>
    {#if error}<p class="error">{error}</p>{/if}
    <footer><button class="secondary" type="button" onclick={() => dispatch("close")}>Cancel</button><button class="primary" type="submit">{task ? "Save changes" : "Create task"}</button></footer>
  </form>
</Modal>

<style>
  form { display: grid; gap: 11px; }
  label { position: relative; display: grid; gap: 5px; color: rgba(255,255,255,.56); font-size: 10px; }
  input, textarea { box-sizing: border-box; width: 100%; padding: 9px 10px; border: 1px solid rgba(255,255,255,.08); border-radius: 9px; outline: 0; color: #f5f5f5; background: rgba(255,255,255,.055); font: inherit; font-size: 11px; resize: none; }
  input:focus, textarea:focus { border-color: rgba(174,207,181,.55); box-shadow: 0 0 0 2px rgba(126,172,136,.13); }
  .row { display: grid; grid-template-columns: 1.5fr 1fr; gap: 9px; }
  label span { position: absolute; right: 8px; bottom: 10px; color: rgba(255,255,255,.34); font-size: 9px; pointer-events: none; }
  input[type="number"] { padding-right: 47px; }
  .error { margin: 0; color: #f0a09d; font-size: 10px; }
  footer { display: flex; justify-content: flex-end; gap: 7px; padding-top: 2px; }
  button { padding: 8px 12px; border: 0; border-radius: 8px; color: #f3f4f4; background: rgba(255,255,255,.08); font: inherit; font-size: 10px; font-weight: 630; cursor: pointer; }
  button.primary { color: #142018; background: #bfd5bf; }
  button:active { transform: scale(.98); }
</style>
