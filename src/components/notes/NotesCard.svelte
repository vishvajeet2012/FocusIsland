<script lang="ts">
  import { onDestroy } from "svelte";
  import { NOTE_SAVE_DEBOUNCE_MS } from "../../lib/constants";
  import { saveDailyNote } from "../../lib/native";
  import { formatShortDate } from "../../lib/time";
  import { wordCount } from "../../lib/formatting";
  import { protectFromFocusLoss } from "../../lib/focusGuard";
  import { notesStore } from "../../stores/notes";
  import { addTask } from "../../stores/tasks";
  import { showToast } from "../../stores/toasts";
  import Icon from "../common/Icon.svelte";

  let editor: HTMLTextAreaElement;
  let saveHandle: ReturnType<typeof setTimeout> | null = null;
  let currentContent = "";
  let initializedDate = "";

  $: if ($notesStore.note.date && initializedDate !== $notesStore.note.date) {
    initializedDate = $notesStore.note.date;
    currentContent = $notesStore.note.content;
  }
  $: words = wordCount(currentContent);

  function queueSave(): void {
    notesStore.update((state) => ({ ...state, note: { ...state.note, content: currentContent }, saving: true, saved: false }));
    if (saveHandle !== null) clearTimeout(saveHandle);
    saveHandle = setTimeout(flushSave, NOTE_SAVE_DEBOUNCE_MS);
  }

  async function flushSave(): Promise<void> {
    if (saveHandle !== null) clearTimeout(saveHandle);
    saveHandle = null;
    const date = $notesStore.note.date;
    const content = currentContent;
    if (!date) return;
    try {
      const saved = await saveDailyNote(date, content);
      if (content === currentContent) notesStore.set({ note: saved, saving: false, saved: true });
    } catch (error) {
      notesStore.update((state) => ({ ...state, saving: false, saved: false }));
      showToast(error instanceof Error ? error.message : "Note couldn't be saved", "warning");
    }
  }

  async function taskFromCurrentLine(): Promise<void> {
    const start = currentContent.lastIndexOf("\n", Math.max(0, editor.selectionStart - 1)) + 1;
    const nextBreak = currentContent.indexOf("\n", editor.selectionStart);
    const end = nextBreak < 0 ? currentContent.length : nextBreak;
    const title = currentContent.slice(start, end).replace(/^\s*[-*•]\s*/u, "").trim();
    if (!title) { showToast("Place the cursor on a line with text", "warning"); return; }
    try { await addTask({ title }); showToast("Task created", "success"); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't create task", "warning"); }
  }

  function keydown(event: KeyboardEvent): void {
    if (event.ctrlKey && event.key === "Enter") {
      event.preventDefault();
      taskFromCurrentLine();
    }
  }

  onDestroy(() => { if (saveHandle !== null) void flushSave(); });
</script>

<section class="card notes-card" aria-labelledby="notes-heading" use:protectFromFocusLoss>
  <header>
    <div><h2 id="notes-heading">Notepad</h2><span>{formatShortDate()}</span></div>
    <span class="save-state">{$notesStore.saving ? "Saving…" : $notesStore.saved ? "Saved" : "Unsaved"}</span>
  </header>
  <textarea bind:this={editor} bind:value={currentContent} oninput={queueSave} onkeydown={keydown} aria-label="Daily note" spellcheck="true" placeholder="Write anything here…"></textarea>
  <footer>
    <span>{words} {words === 1 ? "word" : "words"}</span>
    <span class="hint"><kbd>Ctrl</kbd><b>+</b><kbd>Enter</kbd> to task</span>
    <Icon name="edit" size={12} />
  </footer>
</section>

<style>
  .notes-card { color: #272313; background: var(--notes-card); }
  header { display: flex; align-items: flex-start; justify-content: space-between; }
  header > div { display: grid; gap: 2px; }
  h2 { margin: 0; font-size: 13px; }
  header span { color: rgba(42,36,18,.47); font-size: 9px; }
  .save-state { opacity: .76; }
  textarea { box-sizing: border-box; min-width: 0; min-height: 0; width: calc(100% + 2px); flex: 1; margin: 9px -1px 6px; padding: 8px; border: 0; border-radius: 9px; outline: 0; color: #272313; background: rgba(255,255,255,.16); font: inherit; font-size: 10.5px; line-height: 1.55; resize: none; scrollbar-width: thin; scrollbar-color: rgba(58,50,25,.16) transparent; }
  textarea:focus { background: rgba(255,255,255,.23); box-shadow: inset 0 0 0 1px rgba(70,60,28,.12); }
  textarea::placeholder { color: rgba(42,36,18,.42); }
  footer { display: flex; align-items: center; gap: 4px; color: rgba(42,36,18,.47); font-size: 8.5px; }
  footer > :first-child { margin-right: auto; }
  .hint { display: flex; align-items: center; gap: 2px; opacity: 0; transition: opacity var(--motion-fast) ease; }
  .notes-card:focus-within .hint { opacity: 1; }
  kbd { padding: 1px 3px; border: 1px solid rgba(52,44,21,.13); border-radius: 3px; background: rgba(255,255,255,.2); font: inherit; font-size: 7px; }
  .hint b { font-weight: 400; }
</style>
