<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import Modal from "../common/Modal.svelte";
  import { toLocalDateTimeInput } from "../../lib/time";
  import { PRODUCT_NAME } from "../../lib/constants";

  export let initialTitle = "";
  export let taskId: number | null = null;
  const dispatch = createEventDispatcher<{ close: void; save: { title: string; remindAt: number; taskId: number | null } }>();

  const initialDate = new Date(Date.now() + 60 * 60 * 1000);
  initialDate.setMinutes(Math.ceil(initialDate.getMinutes() / 5) * 5, 0, 0);
  let title = initialTitle;
  let remindAt = toLocalDateTimeInput(initialDate.getTime());
  let error = "";
  let titleInput: HTMLInputElement;

  onMount(() => setTimeout(() => titleInput?.focus(), 0));

  function submit(event: SubmitEvent): void {
    event.preventDefault();
    const timestamp = new Date(remindAt).getTime();
    if (!title.trim()) { error = "Add a reminder title."; return; }
    if (!Number.isFinite(timestamp) || timestamp <= Date.now()) { error = "Choose a time in the future."; return; }
    dispatch("save", { title: title.trim(), remindAt: timestamp, taskId });
  }
</script>

<Modal title="Set reminder" description={`${PRODUCT_NAME} will notify you locally.`} on:close={() => dispatch("close")}>
  <form onsubmit={submit}>
    <label>Reminder<input bind:this={titleInput} bind:value={title} maxlength="240" placeholder="What should you remember?" /></label>
    <label>Date and time<input type="datetime-local" bind:value={remindAt} /></label>
    {#if error}<p>{error}</p>{/if}
    <footer><button type="button" onclick={() => dispatch("close")}>Cancel</button><button class="primary" type="submit">Schedule</button></footer>
  </form>
</Modal>

<style>
  form { display: grid; gap: 11px; }
  label { display: grid; gap: 5px; color: rgba(255,255,255,.56); font-size: 10px; }
  input { box-sizing: border-box; width: 100%; padding: 9px 10px; border: 1px solid rgba(255,255,255,.08); border-radius: 9px; outline: 0; color: #f5f5f5; background: rgba(255,255,255,.055); font: inherit; font-size: 11px; }
  input:focus { border-color: rgba(174,207,181,.55); box-shadow: 0 0 0 2px rgba(126,172,136,.13); }
  p { margin: 0; color: #f0a09d; font-size: 10px; }
  footer { display: flex; justify-content: flex-end; gap: 7px; padding-top: 2px; }
  button { padding: 8px 12px; border: 0; border-radius: 8px; color: #f3f4f4; background: rgba(255,255,255,.08); font: inherit; font-size: 10px; font-weight: 630; cursor: pointer; }
  button.primary { color: #142018; background: #bfd5bf; }
</style>
