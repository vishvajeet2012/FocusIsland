<script lang="ts">
  import { PRODUCT_NAME } from "../../lib/constants";
  import type { AppSettings } from "../../types/settings";
  import type { TimerStoreValue } from "../../stores/timer";
  import { formatTimer } from "../../lib/time";
  import BrandMark from "../common/BrandMark.svelte";
  import Icon from "../common/Icon.svelte";

  export let timer: TimerStoreValue;
  export let settings: AppSettings;
  export let onOpen: () => void;

  let hoverHandle: ReturnType<typeof setTimeout> | null = null;
  $: active = timer.snapshot.status === "running" || timer.snapshot.status === "paused";

  function click(): void { if (settings.expandOnClick) onOpen(); }
  function enter(): void {
    if (!settings.expandOnHover) return;
    hoverHandle = setTimeout(onOpen, settings.hoverDelayMs);
  }
  function leave(): void {
    if (hoverHandle !== null) clearTimeout(hoverHandle);
    hoverHandle = null;
  }
</script>

<button class="island-content" type="button" onclick={click} onpointerenter={enter} onpointerleave={leave} aria-label={active ? `Open ${PRODUCT_NAME}. Timer ${formatTimer(timer.displaySeconds)}` : `Open ${PRODUCT_NAME}`}>
  {#if active}
    <span class="timer-state"><Icon name={timer.snapshot.status === "paused" ? "pause" : "play"} size={13} /></span>
    {#if timer.snapshot.taskTitle}<span class="active-task">{timer.snapshot.taskTitle}</span>{/if}
    <strong>{formatTimer(timer.displaySeconds)}</strong>
  {:else}
    <span class="brand-mark"><BrandMark size={20} /></span>
    <span class="brand-name">{PRODUCT_NAME}</span>
    <span class="open-hint"><Icon name="chevron" size={12} /></span>
  {/if}
</button>

<style>
  .island-content { display: flex; align-items: center; justify-content: center; gap: 8px; width: 100%; height: 100%; padding: 0 13px; border: 0; border-radius: inherit; color: rgba(255,255,255,.9); background: transparent; font: inherit; cursor: pointer; user-select: none; }
  .island-content:focus-visible { outline: 2px solid rgba(205,231,210,.62); outline-offset: -4px; }
  .brand-mark, .timer-state { display: grid; place-items: center; flex: 0 0 auto; width: 26px; height: 26px; border-radius: 9px; }
  .brand-mark { filter: drop-shadow(0 2px 5px rgba(0,0,0,.28)); }
  .timer-state { background: rgba(255,255,255,.055); }
  .timer-state { color: #d7cfeb; }
  .brand-name { overflow: hidden; font-size: 11px; font-weight: 660; letter-spacing: -.1px; text-overflow: ellipsis; white-space: nowrap; }
  .open-hint { display: grid; place-items: center; margin-left: auto; color: rgba(255,255,255,.3); transition: transform var(--motion-fast) ease, color var(--motion-fast) ease; }
  button:hover .open-hint { color: rgba(255,255,255,.62); transform: translateX(1px); }
  .active-task { overflow: hidden; min-width: 0; max-width: 105px; color: rgba(255,255,255,.58); font-size: 9px; text-overflow: ellipsis; white-space: nowrap; }
  strong { margin-left: auto; font-family: ui-monospace, SFMono-Regular, Consolas, monospace; font-size: 15px; font-variant-numeric: tabular-nums; letter-spacing: -.6px; }
</style>
