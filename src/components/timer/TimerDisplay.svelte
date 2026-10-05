<script lang="ts">
  import { formatTimer } from "../../lib/time";
  import type { TimerMode, TimerStatus } from "../../types/timer";

  export let seconds: number;
  export let mode: TimerMode;
  export let status: TimerStatus;
</script>

<div class="display" class:active={status === "running"} class:complete={status === "completed"} aria-live="off">
  {formatTimer(seconds, mode === "stopwatch")}
</div>

<style>
  .display { color: rgba(31,25,42,.84); font-family: ui-monospace, SFMono-Regular, Consolas, monospace; font-size: clamp(30px, 3.6vw, 41px); font-variant-numeric: tabular-nums; font-weight: 650; letter-spacing: -2.1px; line-height: 1; text-align: center; text-shadow: 0 1px rgba(255,255,255,.2); transition: color var(--motion-normal) ease, text-shadow var(--motion-normal) ease; }
  .display.active { color: #211b2c; text-shadow: 0 0 16px rgba(255,255,255,.27); }
  .display.complete { animation: finish-pulse 650ms ease-out both; }
  @keyframes finish-pulse { 45% { opacity: .55; } }
</style>
