<script lang="ts">
  import { settingsStore } from "../../stores/settings";
  import { timerStore } from "../../stores/timer";
  import { showToast } from "../../stores/toasts";
  import { clamp } from "../../lib/formatting";
  import Icon from "../common/Icon.svelte";
  import TimerDisplay from "./TimerDisplay.svelte";

  export let highlighted = false;
  const presets = [15, 25, 45, 60];
  let showSetTime = false;
  let customMinutes = $settingsStore.defaultTimerMinutes;

  $: snapshot = $timerStore.snapshot;
  $: statusLabel = snapshot.status === "ready" ? "Ready" : snapshot.status === "running" ? "In focus" : snapshot.status === "paused" ? "Paused" : "Complete";
  $: selectedMinutes = Math.round((snapshot.targetDurationSeconds ?? 0) / 60);

  async function chooseMode(mode: "countdown" | "stopwatch"): Promise<void> {
    if (snapshot.status === "running" || snapshot.status === "paused") {
      showToast("Finish the active session before switching modes", "warning");
      return;
    }
    try { await timerStore.reset(mode, customMinutes * 60); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't switch timer mode", "warning"); }
  }

  async function chooseMinutes(minutes: number): Promise<void> {
    try {
      customMinutes = minutes;
      await timerStore.reset("countdown", minutes * 60);
      showSetTime = false;
    } catch (error) { showToast(error instanceof Error ? error.message : "Couldn't set timer", "warning"); }
  }

  async function primaryAction(): Promise<void> {
    try {
      if (snapshot.status === "running") await timerStore.pause();
      else if (snapshot.status === "paused") await timerStore.resume();
      else if (snapshot.status === "completed") {
        const reset = await timerStore.reset(snapshot.mode, customMinutes * 60);
        await timerStore.start({ mode: reset.mode, durationSeconds: reset.mode === "countdown" ? reset.targetDurationSeconds : null });
      }
      else await timerStore.start({ mode: snapshot.mode, durationSeconds: snapshot.mode === "countdown" ? snapshot.targetDurationSeconds ?? customMinutes * 60 : null });
    } catch (error) { showToast(error instanceof Error ? error.message : "Timer action failed", "warning"); }
  }

  async function finish(): Promise<void> {
    try { await timerStore.finish(true); showToast("Focus session completed", "success"); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't finish session", "warning"); }
  }

  function onKeydown(event: KeyboardEvent): void {
    if (event.key === " " && event.target === event.currentTarget) {
      event.preventDefault();
      primaryAction();
    }
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex a11y_no_noninteractive_element_interactions -->
<section class="card timer-card" class:highlighted tabindex="0" aria-labelledby="timer-heading" onkeydown={onKeydown}>
  <header>
    <h2 id="timer-heading">Focus</h2>
    <div class="mode-toggle">
      <button class:active={snapshot.mode === "countdown"} type="button" title="Countdown" onclick={() => chooseMode("countdown")}><Icon name="clock" size={12} /></button>
      <button class:active={snapshot.mode === "stopwatch"} type="button" title="Stopwatch" onclick={() => chooseMode("stopwatch")}><Icon name="stopwatch" size={12} /></button>
    </div>
  </header>
  <div class="timer-center">
    <span class="status"><i class:running={snapshot.status === "running"}></i>{statusLabel}</span>
    <TimerDisplay seconds={$timerStore.displaySeconds} mode={snapshot.mode} status={snapshot.status} />
    {#if snapshot.taskTitle}<span class="task-title" title={snapshot.taskTitle}>{snapshot.taskTitle}</span>{:else}<span class="task-title muted">One thing at a time</span>{/if}
  </div>
  {#if showSetTime && snapshot.mode === "countdown"}
    <div class="set-time">
      <div class="presets">
        {#each presets as minutes}<button class:active={selectedMinutes === minutes} type="button" onclick={() => chooseMinutes(minutes)}>{minutes}</button>{/each}
      </div>
      <label><input type="number" min="1" max="180" bind:value={customMinutes} /><span>min</span></label>
      <button class="apply" type="button" onclick={() => chooseMinutes(clamp(customMinutes, 1, 180))}>Set</button>
    </div>
  {:else}
    <div class="actions">
      <button class="primary" type="button" onclick={primaryAction}>
        <Icon name={snapshot.status === "running" ? "pause" : "play"} size={13} />
        {snapshot.status === "running" ? "Pause" : snapshot.status === "paused" ? "Resume" : snapshot.status === "completed" ? "Again" : "Start"}
      </button>
      {#if snapshot.status === "running" || snapshot.status === "paused"}
        <button class="secondary" type="button" onclick={finish}>Finish</button>
      {:else if snapshot.mode === "countdown"}
        <button class="secondary" type="button" onclick={() => showSetTime = true}>Set time</button>
      {/if}
    </div>
  {/if}
</section>

<style>
  .timer-card { color: #241d30; background: var(--timer-card); outline: none; transition: transform var(--motion-fast) ease, box-shadow var(--motion-normal) ease; }
  .timer-card.highlighted { box-shadow: inset 0 0 0 2px rgba(67,47,91,.22), 0 0 0 3px rgba(216,208,236,.08); }
  .timer-card:focus-visible { box-shadow: inset 0 0 0 2px rgba(59,43,79,.5); }
  header { display: flex; align-items: center; justify-content: space-between; }
  h2 { margin: 0; font-size: 13px; }
  .mode-toggle { display: flex; gap: 2px; padding: 2px; border-radius: 8px; background: rgba(43,33,55,.07); }
  .mode-toggle button { display: grid; place-items: center; width: 23px; height: 21px; padding: 0; border: 0; border-radius: 6px; color: rgba(42,32,55,.42); background: transparent; cursor: pointer; }
  .mode-toggle button.active { color: #2e243c; background: rgba(255,255,255,.44); box-shadow: 0 1px 3px rgba(51,35,67,.08); }
  .timer-center { display: grid; flex: 1; align-content: center; justify-items: center; min-height: 0; }
  .status { display: flex; align-items: center; gap: 5px; margin-bottom: 8px; color: rgba(37,28,49,.51); font-size: 9px; font-weight: 640; text-transform: uppercase; letter-spacing: .55px; }
  .status i { width: 5px; height: 5px; border-radius: 50%; background: rgba(49,36,66,.3); }
  .status i.running { background: #526e49; box-shadow: 0 0 0 3px rgba(82,110,73,.12); }
  .task-title { overflow: hidden; width: 100%; margin-top: 9px; color: rgba(38,29,51,.65); font-size: 9px; font-weight: 590; text-align: center; text-overflow: ellipsis; white-space: nowrap; }
  .task-title.muted { opacity: .65; font-weight: 450; }
  .actions { display: grid; grid-template-columns: 1fr auto; gap: 5px; }
  .actions button, .apply { display: flex; align-items: center; justify-content: center; gap: 5px; min-height: 31px; padding: 0 10px; border: 0; border-radius: 9px; color: #f5f2f8; background: #352b44; font: inherit; font-size: 10px; font-weight: 650; cursor: pointer; transition: filter var(--motion-fast) ease, transform var(--motion-fast) ease; }
  .actions button:hover, .apply:hover { filter: brightness(1.09); }
  .actions button:active, .apply:active { transform: scale(.97); }
  .actions button.secondary { color: #362c43; background: rgba(255,255,255,.35); }
  .set-time { display: grid; grid-template-columns: 1fr 50px 34px; align-items: center; gap: 4px; min-height: 31px; }
  .presets { display: flex; gap: 2px; }
  .presets button { min-width: 24px; height: 25px; padding: 0 3px; border: 0; border-radius: 6px; color: rgba(39,30,51,.58); background: rgba(255,255,255,.2); font: inherit; font-size: 8px; cursor: pointer; }
  .presets button.active { color: #f7f4fa; background: #4a3b5e; }
  label { position: relative; }
  label input { box-sizing: border-box; width: 100%; height: 25px; padding: 0 19px 0 5px; border: 1px solid rgba(47,36,60,.12); border-radius: 6px; outline: 0; color: #2e253a; background: rgba(255,255,255,.25); font: inherit; font-size: 9px; }
  label span { position: absolute; right: 4px; top: 8px; color: rgba(39,29,51,.43); font-size: 7px; }
  .apply { min-height: 25px; padding: 0 5px; border-radius: 6px; font-size: 8px; }
</style>
