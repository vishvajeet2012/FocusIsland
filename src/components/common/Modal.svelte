<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { lockCollapse, unlockCollapse } from "../../stores/app";
  import IconButton from "./IconButton.svelte";

  export let title: string;
  export let description: string | null = null;
  export let width = 360;

  const dispatch = createEventDispatcher<{ close: void }>();
  let dialog: HTMLDivElement;

  function close(): void { dispatch("close"); }
  function onKeydown(event: KeyboardEvent): void {
    if (event.key === "Escape") close();
  }

  onMount(() => {
    lockCollapse();
    dialog?.focus();
    return unlockCollapse;
  });
</script>

<svelte:window onkeydown={onKeydown} />
<div class="backdrop" role="presentation" onmousedown={(event) => event.target === event.currentTarget && close()}>
  <div class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="-1" bind:this={dialog} style:max-width={`${width}px`}>
    <header>
      <div>
        <h2 id="modal-title">{title}</h2>
        {#if description}<p>{description}</p>{/if}
      </div>
      <IconButton icon="x" label="Close dialog" onclick={close} />
    </header>
    <div class="body"><slot /></div>
  </div>
</div>

<style>
  .backdrop {
    position: absolute;
    z-index: 80;
    inset: 0;
    display: grid;
    place-items: center;
    padding: 18px;
    border-radius: inherit;
    background: rgba(0,0,0,.5);
    animation: fade var(--modal-duration) ease-out both;
  }
  .modal {
    width: 100%;
    max-height: calc(100% - 20px);
    overflow: auto;
    padding: 15px;
    border: 1px solid rgba(255,255,255,.1);
    border-radius: 17px;
    color: #f4f5f5;
    background: #151718;
    box-shadow: 0 18px 55px rgba(0,0,0,.55);
    outline: none;
    animation: modal-in var(--modal-duration) var(--ease-out) both;
  }
  header { display: flex; align-items: flex-start; justify-content: space-between; gap: 18px; margin-bottom: 13px; }
  h2 { margin: 0; font-size: 14px; line-height: 1.3; }
  p { margin: 3px 0 0; color: rgba(255,255,255,.5); font-size: 10px; }
  .body { min-width: 0; }
  @keyframes fade { from { opacity: 0; } }
  @keyframes modal-in { from { opacity: 0; transform: translateY(5px) scale(.99); } }
</style>
