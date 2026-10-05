<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import Icon from "../common/Icon.svelte";

  const dispatch = createEventDispatcher<{ save: string; cancel: void }>();
  let title = "";
  let input: HTMLInputElement;

  function keydown(event: KeyboardEvent): void {
    if (event.key === "Escape") dispatch("cancel");
    if (event.key === "Enter" && title.trim()) dispatch("save", title.trim());
  }

  onMount(() => input?.focus());
</script>

<div class="quick-task">
  <span><Icon name="plus" size={16} /></span>
  <input bind:this={input} bind:value={title} maxlength="240" aria-label="Quick task title" placeholder="Add a task…" onkeydown={keydown} />
  <kbd>Enter</kbd>
  <button type="button" aria-label="Cancel quick task" onclick={() => dispatch("cancel")}><Icon name="x" size={14} /></button>
</div>

<style>
  .quick-task { display: flex; align-items: center; gap: 9px; width: 100%; height: 100%; padding: 0 10px 0 13px; color: rgba(255,255,255,.8); }
  .quick-task > span { display: grid; place-items: center; color: #c5dec9; }
  input { min-width: 0; flex: 1; padding: 0; border: 0; outline: 0; color: rgba(255,255,255,.92); background: transparent; font: inherit; font-size: 12px; }
  input::placeholder { color: rgba(255,255,255,.38); }
  kbd { padding: 2px 5px; border: 1px solid rgba(255,255,255,.08); border-radius: 4px; color: rgba(255,255,255,.32); background: rgba(255,255,255,.035); font: inherit; font-size: 7px; }
  button { display: grid; place-items: center; width: 27px; height: 27px; padding: 0; border: 0; border-radius: 50%; color: rgba(255,255,255,.54); background: rgba(255,255,255,.055); cursor: pointer; }
  button:hover { color: white; background: rgba(255,255,255,.09); }
</style>
