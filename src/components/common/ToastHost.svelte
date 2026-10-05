<script lang="ts">
  import { dismissToast, toastsStore } from "../../stores/toasts";
  import Icon from "./Icon.svelte";
</script>

<div class="toast-host" aria-live="polite" aria-atomic="false">
  {#each $toastsStore as toast (toast.id)}
    <button class:success={toast.tone === "success"} class:warning={toast.tone === "warning"} type="button" onclick={() => dismissToast(toast.id)}>
      {#if toast.tone === "success"}<Icon name="check" size={13} />{/if}
      {#if toast.tone === "warning"}<span class="dot">!</span>{/if}
      <span>{toast.text}</span>
    </button>
  {/each}
</div>

<style>
  .toast-host {
    position: absolute;
    z-index: 100;
    left: 50%;
    bottom: 14px;
    display: grid;
    gap: 6px;
    width: max-content;
    max-width: calc(100% - 28px);
    pointer-events: none;
    transform: translateX(-50%);
  }
  button {
    display: flex;
    align-items: center;
    gap: 7px;
    min-height: 31px;
    padding: 7px 11px;
    border: 1px solid rgba(255,255,255,.09);
    border-radius: 10px;
    color: rgba(255,255,255,.9);
    background: rgba(29,31,32,.97);
    box-shadow: 0 8px 25px rgba(0,0,0,.38);
    font: inherit;
    font-size: 11px;
    pointer-events: auto;
    cursor: pointer;
    animation: toast-in var(--motion-normal) var(--ease-out) both;
  }
  button.success :global(svg) { color: #9ed19b; }
  .dot { display: grid; place-items: center; width: 14px; height: 14px; border-radius: 50%; color: #1e1710; background: #e1bd78; font-weight: 700; font-size: 9px; }
  @keyframes toast-in { from { opacity: 0; transform: translateY(5px); } }
</style>
