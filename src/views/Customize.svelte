<script lang="ts">
  import { get } from "svelte/store";
  import { DEFAULT_SETTINGS, type AppSettings } from "../types/settings";
  import { PRODUCT_NAME, SETTINGS_SAVE_DEBOUNCE_MS, SHORTCUTS } from "../lib/constants";
  import { saveSettings, setWindowMode } from "../lib/native";
  import { settingsStore } from "../stores/settings";
  import { appState } from "../stores/app";
  import { showToast } from "../stores/toasts";
  import { protectFromFocusLoss } from "../lib/focusGuard";
  import Icon from "../components/common/Icon.svelte";

  let saving = false;
  let saved = true;
  let saveHandle: ReturnType<typeof setTimeout> | null = null;

  function patch(changes: Partial<AppSettings>): void {
    settingsStore.update((settings) => ({ ...settings, ...changes }));
    saved = false;
    if (saveHandle !== null) clearTimeout(saveHandle);
    saveHandle = setTimeout(persist, SETTINGS_SAVE_DEBOUNCE_MS);
  }

  async function persist(): Promise<void> {
    if (saveHandle !== null) clearTimeout(saveHandle);
    saveHandle = null;
    saving = true;
    try {
      const stored = await saveSettings(get(settingsStore));
      settingsStore.set(stored);
      saved = true;
      if ($appState.windowMode === "expanded") await setWindowMode("expanded", stored);
    } catch (error) { showToast(error instanceof Error ? error.message : "Settings couldn't be saved", "warning"); }
    finally { saving = false; }
  }

  function resetAppearance(): void {
    patch({
      theme: DEFAULT_SETTINGS.theme,
      workspaceBackground: DEFAULT_SETTINGS.workspaceBackground,
      taskCardColor: DEFAULT_SETTINGS.taskCardColor,
      timerCardColor: DEFAULT_SETTINGS.timerCardColor,
      notesCardColor: DEFAULT_SETTINGS.notesCardColor,
      eventsCardColor: DEFAULT_SETTINGS.eventsCardColor,
      islandSize: DEFAULT_SETTINGS.islandSize,
      animationIntensity: DEFAULT_SETTINGS.animationIntensity,
      topGap: DEFAULT_SETTINGS.topGap,
    });
    showToast("Appearance reset", "success");
  }

  const bool = (event: Event): boolean => (event.currentTarget as HTMLInputElement).checked;
  const value = (event: Event): string => (event.currentTarget as HTMLInputElement | HTMLSelectElement).value;
</script>

<div class="customize-view" use:protectFromFocusLoss>
  <aside>
    <span class="section-icon"><Icon name="settings" size={17} /></span>
    <h2>Make it yours</h2>
    <p>A calm workspace that stays out of the way.</p>
    <div class="preview" style={`--preview-bg:${$settingsStore.workspaceBackground};--preview-task:${$settingsStore.taskCardColor};--preview-timer:${$settingsStore.timerCardColor};--preview-notes:${$settingsStore.notesCardColor};--preview-events:${$settingsStore.eventsCardColor}`}>
      <div class="preview-pill"></div><div class="preview-cards"><i></i><i></i><i></i><i></i></div>
    </div>
    <span class="save-state">{saving ? "Saving changes…" : saved ? "All changes saved" : "Changes pending"}</span>
    <button class="reset" type="button" onclick={resetAppearance}>Reset appearance</button>
  </aside>
  <div class="settings-scroll">
    {#if $appState.shortcutWarnings.length > 0}
      <section class="warning"><strong>Shortcut warning</strong><p>{$appState.shortcutWarnings.join(" ")}</p></section>
    {/if}
    <section>
      <h3>Appearance</h3>
      <div class="setting"><div><b>Theme</b><span>Follow Windows or choose a mode</span></div><select value={$settingsStore.theme} onchange={(event) => patch({ theme: value(event) as AppSettings["theme"] })}><option value="system">System</option><option value="dark">Dark</option><option value="light">Light</option></select></div>
      <div class="setting color-setting"><div><b>Workspace</b><span>Shell background</span></div><label><input type="color" value={$settingsStore.workspaceBackground} oninput={(event) => patch({ workspaceBackground: value(event) })} /><code>{$settingsStore.workspaceBackground}</code></label></div>
      <div class="color-grid">
        <label>Tasks<input type="color" value={$settingsStore.taskCardColor} oninput={(event) => patch({ taskCardColor: value(event) })} /></label>
        <label>Timer<input type="color" value={$settingsStore.timerCardColor} oninput={(event) => patch({ timerCardColor: value(event) })} /></label>
        <label>Notes<input type="color" value={$settingsStore.notesCardColor} oninput={(event) => patch({ notesCardColor: value(event) })} /></label>
        <label>Events<input type="color" value={$settingsStore.eventsCardColor} oninput={(event) => patch({ eventsCardColor: value(event) })} /></label>
      </div>
      <div class="setting"><div><b>Island size</b><span>Collapsed footprint</span></div><select value={$settingsStore.islandSize} onchange={(event) => patch({ islandSize: value(event) as AppSettings["islandSize"] })}><option value="compact">Compact</option><option value="normal">Normal</option><option value="large">Large</option></select></div>
      <div class="setting"><div><b>Animation intensity</b><span>Also respects Windows motion settings</span></div><select value={$settingsStore.animationIntensity} onchange={(event) => patch({ animationIntensity: value(event) as AppSettings["animationIntensity"] })}><option value="full">Full</option><option value="reduced">Reduced</option><option value="off">Off</option></select></div>
    </section>
    <section>
      <h3>Island behavior</h3>
      <div class="setting"><div><b>Expand on click</b><span>Open the workspace with a click</span></div><input class="switch" type="checkbox" checked={$settingsStore.expandOnClick} onchange={(event) => patch({ expandOnClick: bool(event) })} /></div>
      <div class="setting"><div><b>Expand on hover</b><span>Wait before opening to prevent accidents</span></div><input class="switch" type="checkbox" checked={$settingsStore.expandOnHover} onchange={(event) => patch({ expandOnHover: bool(event) })} /></div>
      {#if $settingsStore.expandOnHover}<div class="setting"><div><b>Hover delay</b><span>Milliseconds</span></div><select value={$settingsStore.hoverDelayMs} onchange={(event) => patch({ hoverDelayMs: Number(value(event)) as AppSettings["hoverDelayMs"] })}><option value="100">100 ms</option><option value="200">200 ms</option><option value="300">300 ms</option><option value="500">500 ms</option></select></div>{/if}
      <div class="setting"><div><b>Collapse on focus loss</b><span>Stay open while menus and editors are active</span></div><input class="switch" type="checkbox" checked={$settingsStore.collapseOnFocusLoss} onchange={(event) => patch({ collapseOnFocusLoss: bool(event) })} /></div>
      <div class="setting"><div><b>Movable workspace</b><span>Drag the header when enabled</span></div><input class="switch" type="checkbox" checked={$settingsStore.movable} onchange={(event) => patch({ movable: bool(event) })} /></div>
      <div class="setting"><div><b>Monitor</b><span>Where shortcuts open the island</span></div><select value={$settingsStore.monitorMode} onchange={(event) => patch({ monitorMode: value(event) as AppSettings["monitorMode"] })}><option value="active">Mouse monitor</option><option value="primary">Primary monitor</option></select></div>
      <div class="setting"><div><b>Top gap</b><span>Distance from screen edge</span></div><select value={$settingsStore.topGap} onchange={(event) => patch({ topGap: Number(value(event)) as AppSettings["topGap"] })}><option value="0">0 px</option><option value="6">6 px</option><option value="12">12 px</option></select></div>
    </section>
    <section>
      <h3>Windows</h3>
      <div class="setting"><div><b>Always on top</b><span>Keep {PRODUCT_NAME} above other windows</span></div><input class="switch" type="checkbox" checked={$settingsStore.alwaysOnTop} onchange={(event) => patch({ alwaysOnTop: bool(event) })} /></div>
      <div class="setting"><div><b>Launch on startup</b><span>Start after you sign in to Windows</span></div><input class="switch" type="checkbox" checked={$settingsStore.launchOnStartup} onchange={(event) => patch({ launchOnStartup: bool(event) })} /></div>
      <div class="setting"><div><b>Show tray icon</b><span>Keep quick actions in the notification area</span></div><input class="switch" type="checkbox" checked={$settingsStore.showTrayIcon} onchange={(event) => patch({ showTrayIcon: bool(event) })} /></div>
      <div class="setting"><div><b>Notifications</b><span>Timer completions and reminders</span></div><input class="switch" type="checkbox" checked={$settingsStore.notificationsEnabled} onchange={(event) => patch({ notificationsEnabled: bool(event) })} /></div>
      <div class="setting"><div><b>Notification sounds</b><span>Off by default for quiet focus</span></div><input class="switch" type="checkbox" checked={$settingsStore.notificationSounds} onchange={(event) => patch({ notificationSounds: bool(event) })} /></div>
    </section>
    <section>
      <h3>Focus & time</h3>
      <div class="setting"><div><b>Default timer</b><span>1–180 minutes</span></div><label class="number"><input type="number" min="1" max="180" value={$settingsStore.defaultTimerMinutes} onchange={(event) => patch({ defaultTimerMinutes: Math.min(180, Math.max(1, Number(value(event)))) })} /><span>min</span></label></div>
      <div class="setting"><div><b>Time format</b><span>Used for tasks and reminders</span></div><select value={$settingsStore.timeFormat} onchange={(event) => patch({ timeFormat: value(event) as AppSettings["timeFormat"] })}><option value="12">12 hour</option><option value="24">24 hour</option></select></div>
      <div class="shortcuts">
        <div><span>Toggle workspace</span><kbd>{SHORTCUTS.toggleWorkspace}</kbd></div>
        <div><span>Quick task</span><kbd>{SHORTCUTS.quickTask}</kbd></div>
        <div><span>Start / pause timer</span><kbd>{SHORTCUTS.toggleTimer}</kbd></div>
      </div>
    </section>
  </div>
</div>

<style>
  .customize-view { display: grid; grid-template-columns: 220px 1fr; gap: 8px; height: 100%; color: var(--shell-text); }
  aside, section { border: 1px solid var(--surface-border); border-radius: 18px; background: var(--surface-bg); }
  aside { display: flex; flex-direction: column; padding: 15px; }
  .section-icon { display: grid; place-items: center; width: 32px; height: 32px; border-radius: 11px; color: #c8d7ca; background: rgba(200,215,202,.1); }
  aside h2 { margin: 12px 0 3px; font-size: 14px; }
  aside p { margin: 0; color: var(--shell-subtle); font-size: 9px; line-height: 1.4; }
  .preview { display: grid; align-content: center; gap: 5px; height: 90px; margin: 17px 0 11px; padding: 9px; border: 1px solid rgba(255,255,255,.08); border-radius: 13px; background: var(--preview-bg); box-shadow: inset 0 1px rgba(255,255,255,.04); }
  .preview-pill { width: 38px; height: 5px; border-radius: 4px; background: rgba(255,255,255,.15); }
  .preview-cards { display: grid; grid-template-columns: 1.7fr .8fr 1fr 1fr; gap: 3px; height: 47px; }
  .preview-cards i { border-radius: 5px; background: var(--preview-task); }.preview-cards i:nth-child(2) { background: var(--preview-timer); }.preview-cards i:nth-child(3) { background: var(--preview-notes); }.preview-cards i:nth-child(4) { background: var(--preview-events); }
  .save-state { margin-top: auto; color: var(--shell-subtle); font-size: 8px; }
  .reset { margin-top: 8px; padding: 7px; border: 1px solid var(--surface-border); border-radius: 8px; color: var(--shell-muted); background: var(--surface-control); font: inherit; font-size: 9px; cursor: pointer; }
  .settings-scroll { display: grid; gap: 7px; min-height: 0; padding-right: 3px; overflow-y: auto; scrollbar-width: thin; scrollbar-color: rgba(255,255,255,.12) transparent; }
  section { padding: 12px 13px; }
  section h3 { margin: 0 0 5px; color: var(--shell-muted); font-size: 9px; letter-spacing: .55px; text-transform: uppercase; }
  .setting { display: flex; align-items: center; justify-content: space-between; gap: 20px; min-height: 42px; border-bottom: 1px solid var(--surface-line); }
  .setting:last-child { border: 0; }
  .setting > div { display: grid; gap: 2px; min-width: 0; }
  .setting b { font-size: 10px; font-weight: 600; }.setting span { color: var(--shell-subtle); font-size: 8px; }
  select, .number input { height: 27px; padding: 0 7px; border: 1px solid var(--surface-border); border-radius: 7px; outline: 0; color: var(--shell-text); background: var(--field-bg); font: inherit; font-size: 9px; }
  select:focus, .number input:focus { border-color: rgba(190,216,195,.45); }
  .switch { appearance: none; flex: 0 0 auto; width: 31px; height: 18px; padding: 2px; border: 0; border-radius: 99px; background: rgba(255,255,255,.12); cursor: pointer; transition: background var(--motion-normal) ease; }
  .switch::before { content: ""; display: block; width: 14px; height: 14px; border-radius: 50%; background: rgba(255,255,255,.72); transition: transform var(--motion-normal) var(--ease-standard), background var(--motion-normal) ease; }
  .switch:checked { background: #66886f; }.switch:checked::before { background: white; transform: translateX(13px); }
  .color-setting label { display: flex; align-items: center; gap: 6px; }.color-setting code { color: rgba(255,255,255,.46); font-size: 8px; }
  input[type="color"] { width: 26px; height: 26px; padding: 2px; border: 1px solid rgba(255,255,255,.1); border-radius: 7px; background: #1d1f20; cursor: pointer; }
  input[type="color"]::-webkit-color-swatch { border: 0; border-radius: 4px; }
  .color-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 5px; padding: 8px 0; }
  .color-grid label { display: flex; align-items: center; justify-content: space-between; gap: 4px; padding: 6px; border-radius: 8px; color: var(--shell-muted); background: var(--surface-control); font-size: 8px; }
  .number { position: relative; }.number input { box-sizing: border-box; width: 62px; padding-right: 23px; }.number span { position: absolute; right: 6px; top: 9px; font-size: 7px; }
  .shortcuts { display: grid; grid-template-columns: repeat(3, 1fr); gap: 5px; padding-top: 9px; }
  .shortcuts div { display: grid; gap: 5px; padding: 7px; border-radius: 8px; background: var(--surface-control); }.shortcuts span { color: var(--shell-subtle); font-size: 8px; }.shortcuts kbd { width: fit-content; padding: 2px 5px; border: 1px solid var(--surface-border); border-radius: 4px; color: var(--shell-muted); background: var(--surface-control); font: inherit; font-size: 7px; }
  section.warning { padding: 9px 12px; border-color: rgba(219,178,100,.18); background: rgba(112,83,35,.18); }.warning strong { color: #e0c18b; font-size: 9px; }.warning p { margin: 3px 0 0; color: rgba(255,255,255,.48); font-size: 8px; }
</style>
