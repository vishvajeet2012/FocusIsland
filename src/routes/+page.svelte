<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import WorkspaceShell from "../components/workspace/WorkspaceShell.svelte";
  import { PRODUCT_NAME, PRODUCT_TAGLINE } from "../lib/constants";
  import { afterPaint, motionDuration, nextFrame, waitForMotion } from "../lib/animations";
  import { bootstrap, runningInTauri, saveSettings, setWindowMode, showMainWindow } from "../lib/native";
  import { addTask, setTasks } from "../stores/tasks";
  import { notesStore } from "../stores/notes";
  import { setReminders, remindersStore } from "../stores/reminders";
  import { timerStore } from "../stores/timer";
  import { settingsStore } from "../stores/settings";
  import { appState, setActiveTab } from "../stores/app";
  import { showToast } from "../stores/toasts";
  import type { TimerSnapshot } from "../types/timer";
  import type { MotionPhase, WorkspaceTab } from "../types/app";

  let shell: { focusTaskInput: () => void };
  let transitioning = false;
  let motionPhase: MotionPhase = "idle";
  let unlisteners: UnlistenFn[] = [];

  async function openWorkspace(tab: WorkspaceTab = "workspace", highlightTimer = false): Promise<void> {
    if (transitioning) return;
    transitioning = true;
    motionPhase = "opening";
    try {
      const settings = get(settingsStore);
      await setWindowMode("expanded", settings);
      await showMainWindow();
      await afterPaint();
      appState.update((state) => ({ ...state, windowMode: "expanded", activeTab: tab, timerHighlighted: highlightTimer }));
      await waitForMotion(settings.animationIntensity, 390, 210);
      if (highlightTimer) setTimeout(() => appState.update((state) => ({ ...state, timerHighlighted: false })), 900);
    } catch (error) {
      showToast(error instanceof Error ? error.message : "Couldn't open workspace", "warning");
    } finally { motionPhase = "idle"; transitioning = false; }
  }

  async function collapse(): Promise<void> {
    if (transitioning || get(appState).interactionLocks > 0) return;
    transitioning = true;
    const settings = get(settingsStore);
    motionPhase = "closing";
    if (motionDuration(settings.animationIntensity, 1, 1) > 0) await nextFrame();
    appState.update((state) => ({ ...state, windowMode: "collapsed", timerHighlighted: false }));
    await waitForMotion(settings.animationIntensity, 285, 145);
    try { await setWindowMode("collapsed", settings); }
    catch (error) { showToast(error instanceof Error ? error.message : "Window positioning failed", "warning"); }
    finally { motionPhase = "idle"; transitioning = false; }
  }

  async function openQuickTask(): Promise<void> {
    if (transitioning) return;
    transitioning = true;
    motionPhase = "quick-opening";
    try {
      const settings = get(settingsStore);
      await setWindowMode("quick", settings);
      await showMainWindow();
      await afterPaint();
      appState.update((state) => ({ ...state, windowMode: "quick" }));
      await waitForMotion(settings.animationIntensity, 235, 125);
    } finally { motionPhase = "idle"; transitioning = false; }
  }

  async function closeQuick(): Promise<void> {
    const settings = get(settingsStore);
    transitioning = true;
    motionPhase = "quick-closing";
    if (motionDuration(settings.animationIntensity, 1, 1) > 0) await nextFrame();
    appState.update((state) => ({ ...state, windowMode: "collapsed" }));
    await waitForMotion(settings.animationIntensity, 205, 110);
    try { await setWindowMode("collapsed", settings); }
    finally { motionPhase = "idle"; transitioning = false; }
  }

  async function saveQuickTask(title: string): Promise<void> {
    try { await addTask({ title }); showToast("Task created", "success"); await closeQuick(); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't create task", "warning"); }
  }

  async function toggleTimer(): Promise<void> {
    const current = get(timerStore).snapshot;
    try {
      if (current.status === "running") await timerStore.pause();
      else if (current.status === "paused") await timerStore.resume();
      else {
        if (current.status === "completed") await timerStore.reset(current.mode, get(settingsStore).defaultTimerMinutes * 60);
        const ready = get(timerStore).snapshot;
        await timerStore.start({ mode: ready.mode, durationSeconds: ready.mode === "countdown" ? ready.targetDurationSeconds ?? get(settingsStore).defaultTimerMinutes * 60 : null, taskId: ready.taskId, taskTitle: ready.taskTitle });
      }
    } catch (error) { showToast(error instanceof Error ? error.message : "Timer action failed", "warning"); }
  }

  function onGlobalKeydown(event: KeyboardEvent): void {
    const target = event.target as HTMLElement | null;
    const typing = target?.matches("input, textarea, select, [contenteditable='true']") ?? false;
    if (event.ctrlKey && !event.altKey && event.key.toLowerCase() === "n" && !typing && get(appState).windowMode === "expanded") {
      event.preventDefault();
      setActiveTab("workspace");
      setTimeout(() => shell?.focusTaskInput(), 0);
    } else if (event.key === "Escape" && !typing && get(appState).windowMode === "expanded" && get(appState).interactionLocks === 0) {
      void collapse();
    }
  }

  function onWindowBlur(): void {
    setTimeout(() => {
      const state = get(appState);
      if (get(settingsStore).collapseOnFocusLoss && state.windowMode === "expanded" && state.interactionLocks === 0) void collapse();
    }, 80);
  }

  async function handleShortcut(action: string): Promise<void> {
    try {
      const mode = get(appState).windowMode;
      if (action === "toggle") mode === "expanded" ? await collapse() : await openWorkspace();
      else if (action === "quick-task") await openQuickTask();
      else if (action === "toggle-timer") await toggleTimer();
    } catch (error) { showToast(error instanceof Error ? error.message : "Shortcut action failed", "warning"); }
  }

  async function handleTray(action: string): Promise<void> {
    try {
      if (action === "open") await openWorkspace();
      else if (action === "new-task") await openQuickTask();
      else if (action === "settings") await openWorkspace("customize");
      else if (action === "progress") await openWorkspace("insights");
      else if (action === "start-focus") {
        const current = get(timerStore).snapshot;
        if (current.status === "running" || current.status === "paused") throw new Error("A focus session is already active");
        if (current.status === "completed" || current.mode !== "countdown" || current.targetDurationSeconds !== 1500) {
          await timerStore.reset("countdown", 1500);
        }
        await timerStore.start({ mode: "countdown", durationSeconds: 1500 });
        showToast("25 minute focus started", "success");
      } else if (action === "toggle-timer") await toggleTimer();
    } catch (error) { showToast(error instanceof Error ? error.message : "Tray action failed", "warning"); }
  }

  async function initialize(): Promise<void> {
    try {
      const payload = await bootstrap();
      setTasks(payload.tasks);
      notesStore.set({ note: payload.note, saving: false, saved: true });
      setReminders(payload.reminders);
      timerStore.setSnapshot(payload.timer);
      settingsStore.set(payload.settings);
      appState.update((state) => ({ ...state, loading: false, backendAvailable: true, shortcutWarnings: payload.shortcutWarnings }));
      await setWindowMode("collapsed", payload.settings);
      await showMainWindow();
      if (payload.settings.firstRun) {
        showToast(PRODUCT_TAGLINE, "success", 3200);
        const updated = { ...payload.settings, firstRun: false };
        settingsStore.set(updated);
        void saveSettings(updated).catch(() => showToast("First-run preference couldn't be saved", "warning"));
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : `${PRODUCT_NAME} couldn't initialize its local data.`;
      appState.update((state) => ({ ...state, loading: false, backendAvailable: false, error: message }));
      showToast("Local storage is unavailable; changes may not persist", "warning", 4500);
    }

    if (runningInTauri()) {
      unlisteners.push(await listen<string>("shortcut-action", (event) => void handleShortcut(event.payload)));
      unlisteners.push(await listen<string>("tray-action", (event) => void handleTray(event.payload)));
      unlisteners.push(await listen<TimerSnapshot>("timer-finished", (event) => { timerStore.setSnapshot(event.payload); showToast("Focus session completed", "success", 3200); }));
      unlisteners.push(await listen<{ id: number; title: string }>("reminder-fired", (event) => { remindersStore.update((items) => items.filter((item) => item.id !== event.payload.id)); showToast(event.payload.title, "success", 3200); }));
    }

  }

  onMount(() => {
    void initialize();
    return () => { for (const unlisten of unlisteners) unlisten(); timerStore.destroy(); };
  });
</script>

<svelte:window onkeydown={onGlobalKeydown} onblur={onWindowBlur} />

<div class="app-host size-{$settingsStore.islandSize}">
  <WorkspaceShell
    bind:this={shell}
    mode={$appState.windowMode}
    activeTab={$appState.activeTab}
    settings={$settingsStore}
    timer={$timerStore}
    {motionPhase}
    timerHighlighted={$appState.timerHighlighted}
    onOpen={() => void openWorkspace()}
    onCollapse={() => void collapse()}
    onTabChange={setActiveTab}
    onQuickSave={(title) => void saveQuickTask(title)}
    onQuickCancel={() => void closeQuick().catch((error) => showToast(error instanceof Error ? error.message : "Couldn't close quick task", "warning"))}
  />
</div>
