<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Icon from "../common/Icon.svelte";
  import { protectFromFocusLoss } from "../../lib/focusGuard";

  const dispatch = createEventDispatcher<{ add: string }>();
  let title = "";
  let input: HTMLInputElement;

  export function focus(): void { input?.focus(); }

  function submit(): void {
    const value = title.trim();
    if (!value) return;
    dispatch("add", value);
    title = "";
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter") {
      event.preventDefault();
      submit();
    } else if (event.key === "Escape") {
      title = "";
      input.blur();
    }
  }
</script>

<div class="input-wrap" use:protectFromFocusLoss>
  <Icon name="plus" size={14} />
  <input bind:this={input} bind:value={title} aria-label="Add a task" placeholder="Add a task..." onkeydown={onKeydown} />
  {#if title.trim()}<button type="button" onclick={submit}>Add</button>{/if}
</div>

<style>
  .input-wrap {
    display: flex;
    align-items: center;
    gap: 7px;
    min-height: 33px;
    padding: 0 8px;
    border-radius: 10px;
    color: rgba(22,31,24,.58);
    background: rgba(255,255,255,.34);
    box-shadow: inset 0 0 0 1px rgba(255,255,255,.18);
  }
  input { min-width: 0; flex: 1; padding: 7px 0; border: 0; outline: 0; color: #172019; background: transparent; font: inherit; font-size: 11px; }
  input::placeholder { color: rgba(22,31,24,.5); }
  button { padding: 4px 7px; border: 0; border-radius: 6px; color: #e8f1e8; background: #273b2d; font: inherit; font-size: 9px; font-weight: 650; cursor: pointer; }
  button:active { transform: scale(.97); }
</style>
