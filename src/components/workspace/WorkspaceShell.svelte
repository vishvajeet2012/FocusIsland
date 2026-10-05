<script lang="ts">
  import type { MotionPhase, WindowMode, WorkspaceTab } from "../../types/app";
  import type { AppSettings } from "../../types/settings";
  import { DEFAULT_SETTINGS } from "../../types/settings";
  import type { TimerStoreValue } from "../../stores/timer";
  import Island from "../island/Island.svelte";
  import QuickTask from "../island/QuickTask.svelte";
  import WorkspaceHeader from "./WorkspaceHeader.svelte";
  import Workspace from "../../views/Workspace.svelte";
  import Insights from "../../views/Insights.svelte";
  import Customize from "../../views/Customize.svelte";
  import ToastHost from "../common/ToastHost.svelte";

  export let mode: WindowMode;
  export let activeTab: WorkspaceTab;
  export let settings: AppSettings;
  export let timer: TimerStoreValue;
  export let motionPhase: MotionPhase = "idle";
  export let timerHighlighted = false;
  export let onOpen: () => void;
  export let onCollapse: () => void;
  export let onTabChange: (tab: WorkspaceTab) => void;
  export let onQuickSave: (title: string) => void;
  export let onQuickCancel: () => void;

  let workspace: { focusTaskInput: () => void };
  export function focusTaskInput(): void { workspace?.focusTaskInput(); }

  $: isExpanded = mode === "expanded";
  $: isCollapsed = mode === "collapsed";
  $: isQuick = mode === "quick";
</script>

<main
  class="shell"
  class:expanded={isExpanded}
  class:collapsed={isCollapsed}
  class:quick={isQuick}
  class:opening={motionPhase === "opening"}
  class:closing={motionPhase === "closing"}
  class:quick-opening={motionPhase === "quick-opening"}
  class:quick-closing={motionPhase === "quick-closing"}
  class:light={settings.theme === "light"}
  class:default-background={settings.workspaceBackground === DEFAULT_SETTINGS.workspaceBackground}
  data-theme={settings.theme}
  data-motion={settings.animationIntensity}
  style={`--shell-bg:${settings.workspaceBackground};--task-card:${settings.taskCardColor};--timer-card:${settings.timerCardColor};--notes-card:${settings.notesCardColor};--events-card:${settings.eventsCardColor}`}
>
  <div class="collapsed-layer" aria-hidden={!isCollapsed}>
    <Island {timer} {settings} {onOpen} />
  </div>
  {#if isQuick || motionPhase === "quick-opening" || motionPhase === "quick-closing"}
    <div class="quick-layer" aria-hidden={!isQuick}>
      <QuickTask on:save={(event) => onQuickSave(event.detail)} on:cancel={onQuickCancel} />
    </div>
  {/if}
  {#if isExpanded || motionPhase === "opening" || motionPhase === "closing"}
    <div class="expanded-layer" aria-hidden={!isExpanded}>
      <WorkspaceHeader {activeTab} {onTabChange} {onCollapse} movable={settings.movable} />
      <div class="view-stack">
        {#key activeTab}
          <div class="view-content">
            {#if activeTab === "workspace"}
              <Workspace bind:this={workspace} {timerHighlighted} />
            {:else if activeTab === "insights"}
              <Insights />
            {:else}
              <Customize />
            {/if}
          </div>
        {/key}
      </div>
    </div>
  {/if}
  <ToastHost />
</main>

<style>
  .shell {
    --shell-text: rgba(255,255,255,.92);
    --shell-muted: rgba(255,255,255,.5);
    --shell-subtle: rgba(255,255,255,.38);
    --surface-bg: #121415;
    --surface-border: rgba(255,255,255,.055);
    --surface-control: rgba(255,255,255,.055);
    --surface-selected: rgba(255,255,255,.09);
    --surface-line: rgba(255,255,255,.06);
    --field-bg: #1e2021;
    --metric-focus: #26352b;
    --metric-tasks: #303046;
    --metric-streak: #3c3325;
    position: relative;
    width: var(--collapsed-width);
    height: var(--collapsed-height);
    overflow: hidden;
    border: 1px solid rgba(255,255,255,.065);
    border-radius: 19px;
    color: white;
    background: var(--shell-bg, #08090a);
    box-shadow: 0 8px 30px rgba(0,0,0,.32), inset 0 1px rgba(255,255,255,.035);
    transform-origin: top center;
    backface-visibility: hidden;
    contain: layout paint style;
  }
  .shell.opening, .shell.closing, .shell.quick-opening, .shell.quick-closing { will-change: width, height, border-radius, box-shadow; }
  .shell.opening {
    transition-property: width, height, border-radius, box-shadow;
    transition-duration: var(--island-open-width), var(--island-open-height), var(--island-open-radius), var(--island-open-shadow);
    transition-delay: 0ms, var(--island-open-height-delay), var(--island-open-radius-delay), 0ms;
    transition-timing-function: var(--ease-island), var(--ease-island), var(--ease-island), var(--ease-standard);
  }
  .shell.closing {
    transition-property: width, height, border-radius, box-shadow;
    transition-duration: var(--island-close-width), var(--island-close-height), var(--island-close-radius), var(--island-close-shadow);
    transition-delay: var(--island-close-width-delay), 0ms, var(--island-close-radius-delay), 0ms;
    transition-timing-function: var(--ease-island), var(--ease-island), var(--ease-standard), var(--ease-standard);
  }
  .shell.quick-opening {
    transition: width var(--quick-open-width) var(--ease-island), height var(--quick-open-height) var(--quick-open-height-delay) var(--ease-island), border-radius var(--quick-open-radius) var(--ease-standard), box-shadow var(--quick-open-shadow) var(--ease-standard);
  }
  .shell.quick-closing {
    transition: width var(--quick-close-width) var(--quick-close-width-delay) var(--ease-island), height var(--quick-close-height) var(--ease-island), border-radius var(--quick-close-radius) var(--ease-standard), box-shadow var(--quick-close-shadow) var(--ease-standard);
  }
  .shell.expanded { width: min(948px, 100vw); height: min(420px, 100vh); border-radius: 25px; box-shadow: 0 20px 62px rgba(0,0,0,.44), inset 0 1px rgba(255,255,255,.045); }
  .shell.quick { width: min(430px, 100vw); height: min(64px, 100vh); border-radius: 18px; }
  .collapsed-layer, .quick-layer, .expanded-layer { position: absolute; inset: 0; transition: opacity var(--motion-normal) ease, transform var(--motion-normal) var(--ease-island), visibility 0s linear var(--motion-normal); }
  .collapsed-layer { opacity: 0; visibility: hidden; transform: translateY(-2px); pointer-events: none; }
  .collapsed .collapsed-layer { opacity: 1; visibility: visible; transform: none; pointer-events: auto; transition-delay: 70ms, 70ms, 0s; }
  .closing.collapsed .collapsed-layer { transition-delay: 125ms, 125ms, 0s; }
  .quick-layer { opacity: 0; visibility: hidden; transform: translateY(-2px); pointer-events: none; }
  .quick .quick-layer { opacity: 1; visibility: visible; transform: none; pointer-events: auto; transition-delay: 40ms, 40ms, 0s; }
  .expanded-layer { display: grid; grid-template-rows: auto minmax(0, 1fr); padding: 10px; opacity: 0; visibility: hidden; transform: translateY(-3px); pointer-events: none; }
  .expanded .expanded-layer { opacity: 1; visibility: visible; transform: none; pointer-events: auto; transition-delay: 88ms, 88ms, 0s; }
  .view-stack, .view-content { min-height: 0; height: 100%; }
  .shell.expanded .view-content { animation: view-in 185ms var(--ease-island) backwards; }
  .shell.expanded :global(.workspace-grid > .card) { animation: workspace-card-in var(--card-duration) var(--ease-island) backwards; }
  .shell.expanded :global(.workspace-grid > .card:nth-child(1)) { animation-delay: var(--card-delay-1); }
  .shell.expanded :global(.workspace-grid > .card:nth-child(2)) { animation-delay: var(--card-delay-2); }
  .shell.expanded :global(.workspace-grid > .card:nth-child(3)) { animation-delay: var(--card-delay-3); }
  .shell.expanded :global(.workspace-grid > .card:nth-child(4)) { animation-delay: var(--card-delay-4); }
  .light {
    --shell-text: #1b201d;
    --shell-muted: rgba(24,31,27,.58);
    --shell-subtle: rgba(24,31,27,.43);
    --surface-bg: #fbfcfa;
    --surface-border: rgba(18,28,22,.09);
    --surface-control: rgba(18,28,22,.055);
    --surface-selected: rgba(18,28,22,.1);
    --surface-line: rgba(18,28,22,.08);
    --field-bg: #eef1ee;
    --metric-focus: #dce9de;
    --metric-tasks: #e4e1ef;
    --metric-streak: #eee4d4;
    border-color: rgba(0,0,0,.08);
    color: var(--shell-text);
    color-scheme: light;
    box-shadow: 0 16px 50px rgba(0,0,0,.24), inset 0 1px rgba(255,255,255,.32);
  }
  .light.default-background { background: #f1f3f0; }
  @media (prefers-color-scheme: light) {
    .shell[data-theme="system"] {
      --shell-text: #1b201d;
      --shell-muted: rgba(24,31,27,.58);
      --shell-subtle: rgba(24,31,27,.43);
      --surface-bg: #fbfcfa;
      --surface-border: rgba(18,28,22,.09);
      --surface-control: rgba(18,28,22,.055);
      --surface-selected: rgba(18,28,22,.1);
      --surface-line: rgba(18,28,22,.08);
      --field-bg: #eef1ee;
      --metric-focus: #dce9de;
      --metric-tasks: #e4e1ef;
      --metric-streak: #eee4d4;
      color: var(--shell-text);
      color-scheme: light;
    }
    .shell[data-theme="system"].default-background { background: #f1f3f0; }
  }
  @keyframes view-in { from { opacity: 0; transform: translateY(3px); } }
  @keyframes workspace-card-in { from { opacity: 0; transform: translateY(5px); } }
</style>
